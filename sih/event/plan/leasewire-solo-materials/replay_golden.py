"""leasewire-solo 金向量机械重放：ORD-020 全序获取序与死锁自由反例与台账配对三场景。

重放寻径约定：两工具源路径经 --lease-src 与 --locks-src 参数传入，
cases 零绝对路径，root 与台账为重放时新建临时夹具，判定走工具真实
acquire/release/lock_status 代码路径，同参双跑逐字节一致。
"""

import argparse
import json
import shutil
import sys
import tempfile
from pathlib import Path

AT = "2026-09-04T08:00:00+00:00"
HOST = "LWHOST"
USER = "lwuser"
BOOT = "1788000000"


def import_tool(src, pkg, mod):
    src = str(Path(src).resolve())
    if src not in sys.path:
        sys.path.insert(0, src)
    for name in list(sys.modules):
        if name == pkg or name.startswith(pkg + "."):
            del sys.modules[name]
    return __import__(f"{pkg}.{mod}", fromlist=[mod])


def make_identity(path):
    path.write_text(
        json.dumps(
            {
                "anomalies": [],
                "identity": {"hash": "c" * 64},
                "observed": {"boottime": BOOT, "hostname": HOST, "user": USER},
            },
            sort_keys=True,
        ),
        encoding="utf-8",
    )
    return str(path)


def make_session(sessions_path, wts_root, sid, allow):
    line = {
        "allow": allow,
        "event": "issued",
        "issued_at": AT,
        "package": "golden",
        "repos": [],
        "session_id": sid,
    }
    with open(sessions_path, "a", encoding="utf-8") as handle:
        handle.write(json.dumps(line, sort_keys=True, separators=(",", ":")) + "\n")
    binding = wts_root / ".bindings" / f"{sid}.json"
    binding.parent.mkdir(parents=True, exist_ok=True)
    binding.write_text(
        json.dumps(
            {"boottime": BOOT, "hostname": HOST, "session_id": sid, "user": USER},
            sort_keys=True,
        ),
        encoding="utf-8",
    )


def held_view(events, normalizer):
    view = {}
    for event in events:
        if event.get("event") == "acquired":
            key = normalizer(event["path"])
            view.setdefault(key, []).append(
                [event["session_id"], event.get("mode", "exclusive")]
            )
        elif event.get("event") == "released":
            key = normalizer(event["path"])
            view[key] = [
                h for h in view.get(key, []) if h[0] != event.get("session_id")
            ]
            if not view[key]:
                view.pop(key, None)
    return {k: sorted(v) for k, v in sorted(view.items())}


def run_case(case, lease_lockcore, locks_core, tmp):
    tool_name = case.get("tool", "lease")
    cname = case["name"]
    sessions = {}
    identity = make_identity(tmp / f"{cname}-identity.json")
    ledger = {}
    for spec in case["sessions"]:
        tool = tool_name if tool_name != "both" else "lease"
        ledger.setdefault(tool, tmp / f"{cname}-sessions-{tool}.ndjson")
        make_session(ledger[tool], tmp / "worktrees", spec["session"], spec["allow"])
    if tool_name == "both":
        ledger.setdefault("locks", tmp / f"{cname}-sessions-default.ndjson")
        for spec in case["sessions"]:
            make_session(ledger["locks"], tmp / "worktrees", spec["session"], spec["allow"])
    events_path = {
        "lease": tmp / f"{cname}-locks-lease.ndjson",
        "locks": tmp / f"{cname}-locks-lockstool.ndjson",
    }
    steps = []
    for op in case["ops"]:
        tool = op.get("tool", "lease" if tool_name != "both" else "lease")
        if tool_name == "both":
            tool = op["tool"]
        elif tool_name == "locks":
            tool = "locks"
        core = lease_lockcore if tool == "lease" else locks_core
        locks_ledger = str(events_path[tool])
        worktree_ledger = str(ledger[tool])
        try:
            if op["op"] == "lock":
                kwargs = {"at": AT}
                if tool == "lease":
                    kwargs["mode"] = op.get("mode", "exclusive")
                core.acquire(
                    op["path"], identity, op["session"], str(tmp),
                    locks_ledger, worktree_ledger, **kwargs,
                )
                steps.append({
                    "decision": "granted", "op": "lock", "path": op["path"],
                    "session": op["session"], "tool": tool,
                    "held_after": held_view(read_events(locks_ledger), core.normalize_path),
                })
            else:
                core.release(
                    op["path"], identity, op["session"], str(tmp),
                    locks_ledger, worktree_ledger, at=AT,
                )
                steps.append({
                    "decision": "released", "op": "unlock", "path": op["path"],
                    "session": op["session"], "tool": tool,
                    "held_after": held_view(read_events(locks_ledger), core.normalize_path),
                })
        except core.LockBlocked as exc:
            steps.append({
                "decision": exc.reason, "detail": _jsonable(exc.detail),
                "op": op["op"], "path": op["path"], "session": op["session"],
                "tool": tool,
                "held_after": held_view(read_events(locks_ledger), core.normalize_path),
            })
    result = {"case": case["name"], "steps": steps}
    if case.get("status"):
        status_tool = "locks" if tool_name in ("locks",) else "lease"
        core = locks_core if status_tool == "locks" else lease_lockcore
        report = core.lock_status(str(events_path[status_tool]))
        result["status_held"] = [
            {k: line[k] for k in sorted(line) if k != "tool"}
            for line in report["held"]
        ]
    return result


def read_events(path):
    out = []
    p = Path(path)
    if p.exists():
        for line in p.read_text(encoding="utf-8").splitlines():
            if line.strip():
                out.append(json.loads(line))
    return out


def _jsonable(detail):
    return {k: str(v) for k, v in sorted((detail or {}).items())}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cases", required=True)
    parser.add_argument("--lease-src", required=True)
    parser.add_argument("--locks-src", required=True)
    args = parser.parse_args()

    lease_lockcore = import_tool(args.lease_src, "lease", "lockcore")
    locks_core = import_tool(args.locks_src, "locks", "core")

    cases = json.loads(Path(args.cases).read_text(encoding="utf-8"))
    tmp = Path(tempfile.mkdtemp(prefix="lw-golden-"))
    try:
        payload = {
            "batch": "leasewire-solo",
            "carrier": "ORD-020",
            "cases": [run_case(c, lease_lockcore, locks_core, tmp) for c in cases],
            "formula_version": "lw-1",
            "at": AT,
        }
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
    print(json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
