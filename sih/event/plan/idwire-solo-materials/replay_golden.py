"""idwire-solo 金向量机械重放：换序同串（ALG-002）与变点定位（PROB-013）。

零行为重放：从 identity core 导入 identity_string / identity_hash /
compare_claims 三判定函数，逐场景机械求值，产出金向量 JSON。
同参形双跑逐字节一致即金向量冻结（F-3）。
"""

import json
import sys
from pathlib import Path


def main():
    cases_path = Path(sys.argv[1])
    cases = json.loads(cases_path.read_text(encoding="utf-8"))

    payload = {
        "formula_version": "ga-1",
        "carrier": ["ALG-002", "PROB-013"],
        "cases": [],
    }

    for case in cases:
        name = case["name"]
        salt = case["salt"]
        if name == "shuffle_invariance_same_string":
            a = case["components_a"]
            b = case["components_b"]
            sa = IDENTITY_STRING(a)
            sb = IDENTITY_STRING(b)
            ha = IDENTITY_HASH(salt, a)
            hb = IDENTITY_HASH(salt, b)
            payload["cases"].append({
                "name": name,
                "scenario": "同组件换序同串",
                "identity_string_a": sa,
                "identity_string_b": sb,
                "string_identical": sa == sb,
                "identity_hash_a": ha,
                "identity_hash_b": hb,
                "hash_identical": ha == hb,
                "verdict": "canonical_rep_stable" if sa == sb and ha == hb else "canonical_rep_diverged",
            })
        elif name == "drift_change_point_located":
            baseline = case["baseline"]
            drifted = case["drifted"]
            s0 = IDENTITY_STRING(baseline)
            s1 = IDENTITY_STRING(drifted)
            # compare_claims 键域限可比十一件（timestamp 不入对表域）
            claims = {k: baseline[k] for k in COMPARABLE}
            cmps = COMPARE_CLAIMS(claims, drifted)
            mismatch_keys = [k for k, v in cmps.items() if v["result"] == "mismatch"]
            payload["cases"].append({
                "name": name,
                "scenario": "异组件异串且漂移位指认",
                "string_baseline": s0,
                "string_drifted": s1,
                "string_differ": s0 != s1,
                "compare": {k: v["result"] for k, v in cmps.items()},
                "mismatch_keys": mismatch_keys,
                "verdict": "change_point_located" if s0 != s1 and mismatch_keys == ["hostname"] else "change_point_not_located",
            })
        else:
            raise SystemExit(f"unknown case: {name}")

    print(json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2))


def _load_core():
    # 判定函数从已被载体锚点注释的 core.py 导入，同参重放不改判定行为。
    # 工具侧在 sih-tools 工地 idwire-solo worktree（与 engine 工地同置 worktrees/ 下）。
    worktrees = Path(__file__).resolve().parents[6]
    tools_src = worktrees / "sih-tools" / "idwire-solo" / "identity" / "src"
    sys.path.insert(0, str(tools_src))
    from identity import core  # noqa: E402
    return core


CORE = _load_core()
IDENTITY_STRING = CORE.identity_string
IDENTITY_HASH = CORE.identity_hash
COMPARE_CLAIMS = CORE.compare_claims
COMPARABLE = CORE.COMPARABLE

if __name__ == "__main__":
    main()