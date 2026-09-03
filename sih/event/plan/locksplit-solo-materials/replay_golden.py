"""locksplit-solo 金向量机械重放：按新锁判定语义（路径同一化 + 锁型 + 包含）回放四场景。

重放判据：同参形双跑逐字节一致，输出与 locksplit-solo-golden-vector.json 冻结一致。
"""

import json
import sys
from pathlib import Path

ROOT = "/abs/root"


def normalize_path(path):
    s = str(path)
    root_s = ROOT.rstrip("/")
    if s.startswith(root_s + "/"):
        s = s[len(root_s) + 1:]
    if s.startswith("./"):
        s = s[2:]
    if s.endswith("/") and s != "/":
        s = s.rstrip("/")
    return s


def is_ancestor(ancestor, descendant):
    a = ancestor.rstrip("/")
    d = descendant.rstrip("/")
    return d.startswith(a + "/")


def conflicts(held_path, held_mode, new_path, new_mode):
    if held_path == new_path:
        return not (held_mode == "append" and new_mode == "append")
    if is_ancestor(held_path, new_path) or is_ancestor(new_path, held_path):
        return not (held_mode == "append" and new_mode == "append")
    return False


def active_locks(events):
    held = {}
    for event in events:
        if event.get("event") == "acquired":
            key = normalize_path(event["path"])
            sid = event["session_id"]
            mode = event.get("mode", "exclusive")
            holders = held.setdefault(key, [])
            if not any(s == sid for s, _ in holders):
                holders.append((sid, mode))
        elif event.get("event") == "released":
            key = normalize_path(event["path"])
            holders = held.get(key)
            if holders:
                remaining = [(s, m) for (s, m) in holders if s != event.get("session_id")]
                if remaining:
                    held[key] = remaining
                else:
                    held.pop(key, None)
    return held


def held_view(events):
    """持位视图即 path → [[session, mode], ...] 列表，承载双 append 共存。"""
    view = {}
    for key, holders in active_locks(events).items():
        view[key] = [[s, m] for s, m in holders]
    return view


def replay(case):
    events = []
    steps = []
    for op in case["ops"]:
        kind = op["kind"]
        path = op["path"]
        sid = op["session"]
        mode = op.get("mode", "exclusive")
        if kind == "lock":
            held = active_locks(events)
            blocked = None
            for held_path, holders in held.items():
                for holder, held_mode in holders:
                    if holder == sid and held_path == normalize_path(path):
                        continue
                    if conflicts(held_path, held_mode, normalize_path(path), mode):
                        blocked = {"holder": holder, "held_mode": held_mode}
                        break
                if blocked:
                    break
            if blocked:
                steps.append({
                    "op": "lock", "path": path, "session": sid, "mode": mode,
                    "decision": "locked_elsewhere", "holder": blocked["holder"],
                    "held_mode": blocked["held_mode"],
                    "verdict_after": held_view(events),
                })
                continue
            events.append({"event": "acquired", "path": normalize_path(path),
                           "session_id": sid, "mode": mode})
            steps.append({
                "op": "lock", "path": path, "session": sid, "mode": mode,
                "decision": "granted", "holder": None,
                "verdict_after": held_view(events),
            })
        elif kind == "unlock":
            held = active_locks(events)
            key = normalize_path(path)
            if key not in held:
                steps.append({
                    "op": "unlock", "path": path, "session": sid,
                    "decision": "not_locked", "verdict_after": held_view(events),
                })
                continue
            if not any(s == sid for s, _ in held[key]):
                steps.append({
                    "op": "unlock", "path": path, "session": sid,
                    "decision": "not_holder", "holder": held[key][0][0],
                    "verdict_after": held_view(events),
                })
                continue
            events.append({"event": "released", "path": key, "session_id": sid})
            steps.append({
                "op": "unlock", "path": path, "session": sid,
                "decision": "released", "verdict_after": held_view(events),
            })
    return {"case": case["name"], "steps": steps, "final_held": held_view(events)}


def main():
    cases_path = Path(sys.argv[1])
    cases = json.loads(cases_path.read_text(encoding="utf-8"))
    payload = {
        "formula_version": "ga-1", "carrier": "ORD-020", "batch": "locksplit-solo",
        "cases": [replay(c) for c in cases],
    }
    print(json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
