"""baseline_checker: sih-engine 治理层确定性程序.

消费 facet 报告 (sih-tools/proposition/DES/<guidance_id>/*.jsonl)
验 PRO-07 / PRO-10-c / 不可逆 / PRO-08 四守卫
返回结构化 verdict (PASS/FAIL/BOUNDARY), 供 X2 写 ndjson + 回写 cross_link_verified.

承接: OQ-22 + DES-011 baseline_checker DEC.
"""
from __future__ import annotations

import argparse
import json
import sys
from datetime import datetime, timezone


# ---------- 4 守卫 ----------

def check_pro07_route(verdict: str, decision: str) -> tuple[bool, str | None]:
    """PRO-07 路由守卫: boundary 不进 Layer 2 (签核路径).

    Args:
        verdict: v3 verdict (stable_clear/near_threshold/boundary)
        decision: signoff decision (promote/hold/reject/refine)

    Returns:
        (passed, error_message) tuple
    """
    if verdict == "boundary" and decision == "promote":
        return False, "PRO-07: boundary 不进 Layer 2 签核 (boundary → refine 循环)"
    return True, None


def check_pro10c_tier(decision: str, tier: str) -> tuple[bool, str | None]:
    """PRO-10-c 守卫: 核心公理 facet 不碰 (升格核心公理归哲学仓自身极高门槛流程)."""
    if decision == "promote" and tier == "核心公理":
        return False, "PRO-10-c: 核心公理 facet 不碰 (升格核心公理归哲学仓自身极高门槛流程)"
    return True, None


def check_irreversible_rationale(decision: str, rationale: str) -> tuple[bool, str | None]:
    """不可逆守卫: promote 须充分 rationale (≥10 字)."""
    if decision == "promote" and (not rationale or len(rationale) < 10):
        actual_len = len(rationale) if rationale else 0
        return False, (
            f"promote 不可逆 - 须充分 rationale (≥10 字, 实际 {actual_len} 字)"
        )
    return True, None


def check_pro08_signed_by(signed_by: str) -> tuple[bool, str | None]:
    """PRO-08 守卫: 签核须留痕归属 (无匿名签核)."""
    if not signed_by:
        return False, "PRO-08: 须 signed_by (签核是应层动作, 无匿名签核)"
    return True, None


# ---------- 4 守卫主入口 ----------

def check_baseline(
    report: dict,
    baseline_pro07: bool = True,
    baseline_pro10c: bool = True,
    baseline_irreversible: bool = True,
    baseline_pro08: bool = True,
) -> dict:
    """baseline_checker 主入口: 消费 facet 报告 + 验 4 守卫 + 返回结构化 verdict.

    Args:
        report: facet program_signoff 报告 dict (含 decision / tier / regulation_anchor / signed_by / rationale / layer1_verdict_v3 / ...)
        baseline_pro07/10c/irreversible/pro08: 4 守卫启用开关 (默认全启用)

    Returns:
        {
            "verdict": "PASS" / "FAIL" / "BOUNDARY",
            "verdict_v3": str,  # 原始 v3 verdict (供 trail 留痕)
            "decision": str,
            "tier": str,
            "signed_by": str,
            "rationale_len": int,
            "regulation_anchor": str,
            "guards_passed": int,
            "guards_failed": int,
            "guard_results": [
                {"name": "PRO-07", "passed": bool, "error": str | None},
                {"name": "PRO-10-c", "passed": bool, "error": str | None},
                {"name": "irreversible", "passed": bool, "error": str | None},
                {"name": "PRO-08", "passed": bool, "error": str | None},
            ],
            "timestamp": str,  # ISO 8601
        }

    关键边界:
    - 不写 ndjson (X2 写)
    - 不回写 cross_link_verified (X2 写)
    - 不调 LLM (机械)
    - 不改 facet 报告 (只读消费)
    """
    verdict_v3 = report.get("layer1_verdict_v3") or report.get("verdict")
    decision = report.get("decision")
    tier = report.get("tier")
    signed_by = report.get("signed_by")
    rationale = report.get("rationale", "")
    regulation_anchor = report.get("regulation_anchor", "")

    guard_results = []
    if baseline_pro07:
        passed, error = check_pro07_route(verdict_v3, decision)
        guard_results.append({"name": "PRO-07", "passed": passed, "error": error})
    if baseline_pro10c:
        passed, error = check_pro10c_tier(decision, tier)
        guard_results.append({"name": "PRO-10-c", "passed": passed, "error": error})
    if baseline_irreversible:
        passed, error = check_irreversible_rationale(decision, rationale)
        guard_results.append({"name": "irreversible", "passed": passed, "error": error})
    if baseline_pro08:
        passed, error = check_pro08_signed_by(signed_by)
        guard_results.append({"name": "PRO-08", "passed": passed, "error": error})

    guards_passed = sum(1 for g in guard_results if g["passed"])
    guards_failed = sum(1 for g in guard_results if not g["passed"])

    # 总体 verdict
    if guards_failed == 0:
        verdict = "PASS"
    elif any("boundary" in (g["error"] or "") for g in guard_results if not g["passed"]):
        verdict = "BOUNDARY"
    else:
        verdict = "FAIL"

    return {
        "verdict": verdict,
        "verdict_v3": verdict_v3,
        "decision": decision,
        "tier": tier,
        "signed_by": signed_by,
        "rationale_len": len(rationale) if rationale else 0,
        "regulation_anchor": regulation_anchor,
        "guards_passed": guards_passed,
        "guards_failed": guards_failed,
        "guard_results": guard_results,
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }


# ---------- CLI ----------

def _cmd_check(args: argparse.Namespace) -> int:
    report = json.loads(args.report)
    result = check_baseline(report)
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0 if result["verdict"] == "PASS" else 1


def main() -> int:
    parser = argparse.ArgumentParser(description="baseline_checker: 消费 facet 报告 + 验 4 守卫")
    parser.add_argument("--report", required=True, help="facet report JSON 字符串")
    args = parser.parse_args()
    return _cmd_check(args)


if __name__ == "__main__":
    sys.exit(main())
