import json
import sys
from pathlib import Path


def state_of(up, cert_hashes, disk_hashes):
    """三态判定：无认证史 unverified，当前等于链上基线 clean，偏离 dirty（推导档 3.2）。"""
    recorded = cert_hashes.get(up)
    current = disk_hashes.get(up)
    if recorded is None:
        return "unverified"
    if recorded == current:
        return "clean"
    return "dirty"


def check_targets(edges, isolated, cert_hashes, disk_hashes):
    """逐上游判定聚合出 verdict：任一脏上游拒下游 blocked，否则 writable（推导档 3.3）。"""
    targets = {}
    for rel in sorted(edges):
        upstream_states = {up: state_of(up, cert_hashes, disk_hashes) for up in edges[rel]}
        dirty = sorted(u for u, s in upstream_states.items() if s == "dirty")
        targets[rel] = {
            "dirty": dirty,
            "unverified": sorted(u for u, s in upstream_states.items() if s == "unverified"),
            "upstreams": upstream_states,
            "verdict": "blocked" if dirty else "writable",
        }
    for rel in sorted(isolated):
        targets[rel] = {
            "dirty": [],
            "unverified": [],
            "upstreams": {},
            "verdict": "writable",
        }
    return targets


def replay(case):
    targets = check_targets(case["edges"], case.get("isolated", []),
                            case["cert_hashes"], case["disk_hashes"])
    verdicts = {rel: t["verdict"] for rel, t in targets.items()}
    blocked = sum(1 for t in targets.values() if t["verdict"] == "blocked")
    exit_code = 1 if blocked else 0
    return {
        "case": case["name"],
        "targets": targets,
        "verdicts": verdicts,
        "exit": exit_code,
        "expect": case["expect"],
    }


def main():
    cases_path = Path(sys.argv[1])
    cases = json.loads(cases_path.read_text(encoding="utf-8"))
    payload = {"formula_version": "ga-1", "carrier": "ORD-016",
               "cases": [replay(c) for c in cases]}
    print(json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
