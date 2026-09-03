import json
import sys
from pathlib import Path


def active_locks(events):
    held = {}
    for event in events:
        if event.get("event") == "acquired":
            held[event["path"]] = event["session_id"]
    for event in events:
        if event.get("event") == "released":
            if held.get(event["path"]) == event.get("session_id"):
                held.pop(event["path"], None)
    return held


def replay(case):
    events = []
    steps = []
    for op in case["ops"]:
        kind = op["kind"]
        path = op["path"]
        sid = op["session"]
        if kind == "lock":
            held = active_locks(events)
            holder = held.get(path)
            if holder is not None and holder != sid:
                steps.append({"op": "lock", "path": path, "session": sid,
                              "decision": "locked_elsewhere",
                              "holder": holder, "verdict_after": dict(active_locks(events))})
                continue
            events.append({"event": "acquired", "path": path, "session_id": sid})
            steps.append({"op": "lock", "path": path, "session": sid,
                          "decision": "granted", "holder": None,
                          "verdict_after": dict(active_locks(events))})
        elif kind == "unlock":
            held = active_locks(events)
            if path not in held:
                steps.append({"op": "unlock", "path": path, "session": sid,
                              "decision": "not_locked",
                              "verdict_after": dict(active_locks(events))})
                continue
            if held[path] != sid:
                steps.append({"op": "unlock", "path": path, "session": sid,
                              "decision": "not_holder", "holder": held[path],
                              "verdict_after": dict(active_locks(events))})
                continue
            events.append({"event": "released", "path": path, "session_id": sid})
            steps.append({"op": "unlock", "path": path, "session": sid,
                          "decision": "released",
                          "verdict_after": dict(active_locks(events))})
    return {"case": case["name"], "steps": steps, "final_held": dict(active_locks(events))}


def main():
    cases_path = Path(sys.argv[1])
    cases = json.loads(cases_path.read_text(encoding="utf-8"))
    payload = {"formula_version": "ga-1", "carrier": "ORD-020",
               "cases": [replay(c) for c in cases]}
    print(json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()