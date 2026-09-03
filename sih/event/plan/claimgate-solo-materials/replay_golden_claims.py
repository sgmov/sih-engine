#!/usr/bin/env python3
"""claimgate-solo 金向量重放：双领拒与过期放行两场景双跑逐字节一致。

用法：python3 replay_golden_claims.py <LEASE_WORKTREE> <OUT_DIR> <ROOT>
场景一 double_claim_rejected：同包未过期二次 claim 即拒 PackageAlreadyClaimed 载领取人与到期时刻。
场景二 expired_then_allowed：过期界后再领即放行且视图标注实效状态。
判定：两跑 stdout 逐字节一致加退出码一致。--at 显式给参不读钟保确定性。
"""
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

LEASE = Path(sys.argv[1]).resolve()
OUT = Path(sys.argv[2]).resolve()
ROOT = Path(sys.argv[3]).resolve()

T0 = "2026-09-04T10:00:00+00:00"
T_EXPIRED = "2026-09-04T11:01:00+00:00"
TMP = Path("/tmp/claimgate-golden")


def run_lease(args):
    env = {k: v for k, v in os.environ.items() if k not in ("PYTHONHOME", "PYTHONPATH")}
    proc = subprocess.run(
        ["uv", "run", "--project", str(LEASE), "lease", *args],
        capture_output=True, text=True, env=env,
    )
    return proc.returncode, proc.stdout


def scenarios():
    """两场景顺序执行，返回 {name: (exit_code, stdout)}，账本同一临时路径先清后跑。"""
    results = {}
    # 场景一：双领拒
    if TMP.exists():
        shutil.rmtree(TMP)
    TMP.mkdir(parents=True)
    claims = TMP / "claims.ndjson"
    led = TMP / "sessions.ndjson"
    locks = TMP / "locks.ndjson"
    code, out = run_lease([
        "claim", "--package", "claimgate-solo", "--ttl", "120",
        "--claimant", "holder-a", "--at", T0,
        "--claims", str(claims), "--root", str(ROOT),
    ])
    results["s1_first_granted"] = (code, out)
    code, out = run_lease([
        "claim", "--package", "claimgate-solo", "--ttl", "120",
        "--claimant", "holder-b", "--at", T0,
        "--claims", str(claims), "--root", str(ROOT),
    ])
    results["s1_second_rejected"] = (code, out)
    # 场景二：过期放行（新账本）
    if TMP.exists():
        shutil.rmtree(TMP)
    TMP.mkdir(parents=True)
    code, out = run_lease([
        "claim", "--package", "claimgate-solo", "--ttl", "60",
        "--claimant", "stale-a", "--at", T0,
        "--claims", str(claims), "--root", str(ROOT),
    ])
    results["s2_first_granted"] = (code, out)
    code, out = run_lease([
        "claim", "--package", "claimgate-solo", "--ttl", "60",
        "--claimant", "fresh-b", "--at", T_EXPIRED,
        "--claims", str(claims), "--root", str(ROOT),
    ])
    results["s2_expired_reclaim_granted"] = (code, out)
    code, out = run_lease([
        "status", "--claims", "--at", T_EXPIRED,
        "--claims-ledger", str(claims),
        "--ledger", str(led), "--locks", str(locks),
        "--root", str(ROOT),
    ])
    results["s2_status_view"] = (code, out)
    return results


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    run1 = scenarios()
    run2 = scenarios()
    vector = {"batch": "claimgate-solo", "date": "2026-09-04", "kind": "golden",
              "scenarios": ["double_claim_rejected", "expired_then_allowed"],
              "verdict": "IDENTICAL", "cases": {}}
    all_identical = True
    for name in run1:
        c1, o1 = run1[name]
        c2, o2 = run2[name]
        identical = (c1 == c2) and (o1 == o2)
        all_identical = all_identical and identical
        for idx, (c, o) in enumerate(((c1, o1), (c2, o2))):
            path = OUT / f"golden-{name}-run{idx + 1}.json"
            payload = {"exit_code": c, "stdout": o}
            path.write_text(json.dumps(payload, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
        vector["cases"][name] = {
            "exit_code": c1,
            "double_run_byte_identical": identical,
        }
    vector["verdict"] = "IDENTICAL" if all_identical else "DIVERGENT"
    (OUT / "claimgate-solo-golden-vector.json").write_text(
        json.dumps(vector, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(json.dumps(vector, ensure_ascii=False, indent=1))
    return 0 if all_identical else 1


if __name__ == "__main__":
    sys.exit(main())
