"""basefix-solo 金向量机械重放：locks 互斥不变式与资源同一化修复后形态。

重放寻径约定（承 2026-09-04 坑位勘误，冻结态携带）：locks 工具源路径经
--locks-src 参数传入，cases 零绝对路径，root 与台账为重放时新建临时夹具，
判定走 locks 真实 acquire/release/lock_status 代码路径，同参双跑逐字节一致。
as-is 偏差形旧向量（l4 两遍配对 granted 形）留档于
sih-engine/sih/event/plan/leasewire-solo-materials/ 不删，本件重冻修复后形态。
"""

import argparse
import json
import shutil
import sys
import tempfile
from pathlib import Path

AT = "2026-09-04T12:00:00+00:00"
HOST = "BFHOST"
USER = "bfuser"
BOOT = "1789000000"


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
                "identity": {"hash": "d" * 64},
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
            view[key] = event["session_id"]
        elif event.get("event") == "released":
            key = normalizer(event["path"])
            if view.get(key) == event.get("session_id"):
                view.pop(key, None)
    return dict(sorted(view.items()))


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


def run_case(case, locks_core, tmp):
    cname = case["name"]
    identity = make_identity(tmp / f"{cname}-identity.json")
    sessions = tmp / f"{cname}-sessions.ndjson"
    for spec in case["sessions"]:
        make_session(sessions, tmp / "worktrees", spec["session"], spec["allow"])
    events_path = str(tmp / f"{cname}-locks.ndjson")
    steps = []
    for op in case["ops"]:
        path = op["path"]
        if path.startswith("ABS:"):
            # ABS: 占位形展开为 root 绝对路径，cases 零绝对路径红线承载
            path = str(tmp / path[len("ABS:"):])
        try:
            if op["op"] == "lock":
                locks_core.acquire(
                    path, identity, op["session"], str(tmp), events_path, str(sessions), at=AT
                )
                decision = "granted"
            else:
                locks_core.release(
                    path, identity, op["session"], str(tmp), events_path, str(sessions), at=AT
                )
                decision = "released"
        except locks_core.LockBlocked as exc:
            decision = exc.reason
        steps.append({
            "decision": decision,
            "op": op["op"],
            "path": locks_core.normalize_path(path, str(tmp)),
            "session": op["session"],
            "held_after": held_view(read_events(events_path), lambda p: locks_core.normalize_path(p, str(tmp))),
        })
    result = {"case": case["name"], "steps": steps}
    if case.get("status"):
        report = locks_core.lock_status(events_path, root=str(tmp))
        result["status_held"] = [
            {k: line[k] for k in sorted(line) if k != "tool"}
            for line in report["held"]
        ]
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cases", required=True)
    parser.add_argument("--locks-src", required=True)
    args = parser.parse_args()

    locks_core = import_tool(args.locks_src, "locks", "core")

    cases = json.loads(Path(args.cases).read_text(encoding="utf-8"))
    tmp = Path(tempfile.mkdtemp(prefix="bf-golden-"))
    try:
        payload = {
            "batch": "basefix-solo",
            "carrier": "ORD-020",
            "cases": [run_case(c, locks_core, tmp) for c in cases],
            "formula_version": "bf-1",
            "at": AT,
        }
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
    print(json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
