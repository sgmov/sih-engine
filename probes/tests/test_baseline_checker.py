"""baseline_checker 单测: 4 守卫 + check_baseline 主入口 + 边界 cases.

承接: DES-011 baseline_checker DEC + T6D-13 Cluster 4 X1.
"""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

import pytest

# 允许 probe 文件 import
PROBES_DIR = Path(__file__).resolve().parent.parent
if str(PROBES_DIR) not in sys.path:
    sys.path.insert(0, str(PROBES_DIR))

from baseline_checker import (  # noqa: E402
    check_baseline,
    check_pro07_route,
    check_pro08_signed_by,
    check_irreversible_rationale,
    check_pro10c_tier,
)


# ---------- 测试 fixture ----------

def _good_report() -> dict:
    """一个合法签核报告: stable_clear + promote + 衍生原则 + 充分 rationale + signed_by."""
    return {
        "layer1_verdict_v3": "stable_clear",
        "decision": "promote",
        "tier": "衍生原则",
        "regulation_anchor": "PRO-007 签核",
        "signed_by": "moc",
        "rationale": "stable_clear verified, family_temperature v3, baseline_v1, rationale ≥10 字",
    }


# ---------- PRO-07 守卫单测 ----------

class TestPro07Route:
    def test_boundary_promote_rejected(self):
        passed, err = check_pro07_route("boundary", "promote")
        assert passed is False
        assert "PRO-07" in (err or "")
        assert "boundary" in (err or "").lower()

    def test_boundary_hold_allowed(self):
        # boundary + hold 不在 PRO-07 路由触发范围内（只 promote 触发）
        passed, err = check_pro07_route("boundary", "hold")
        assert passed is True
        assert err is None

    def test_boundary_reject_allowed(self):
        passed, err = check_pro07_route("boundary", "reject")
        assert passed is True
        assert err is None

    def test_boundary_refine_allowed(self):
        # boundary + refine = 路由进 refine 循环（合规）
        passed, err = check_pro07_route("boundary", "refine")
        assert passed is True
        assert err is None

    def test_stable_clear_promote_allowed(self):
        passed, err = check_pro07_route("stable_clear", "promote")
        assert passed is True
        assert err is None

    def test_near_threshold_promote_allowed(self):
        passed, err = check_pro07_route("near_threshold", "promote")
        assert passed is True
        assert err is None


# ---------- PRO-10-c 守卫单测 ----------

class TestPro10cTier:
    def test_promote_核心公理_rejected(self):
        passed, err = check_pro10c_tier("promote", "核心公理")
        assert passed is False
        assert "PRO-10-c" in (err or "")

    def test_promote_衍生原则_allowed(self):
        passed, err = check_pro10c_tier("promote", "衍生原则")
        assert passed is True
        assert err is None

    def test_promote_工程规范_allowed(self):
        passed, err = check_pro10c_tier("promote", "工程规范")
        assert passed is True
        assert err is None

    def test_hold_核心公理_allowed(self):
        # hold 不触发（只 promote 触发升格检查）
        passed, err = check_pro10c_tier("hold", "核心公理")
        assert passed is True
        assert err is None

    def test_reject_核心公理_allowed(self):
        passed, err = check_pro10c_tier("reject", "核心公理")
        assert passed is True
        assert err is None


# ---------- 不可逆守卫单测 ----------

class TestIrreversibleRationale:
    def test_promote_no_rationale_rejected(self):
        passed, err = check_irreversible_rationale("promote", "")
        assert passed is False
        assert "不可逆" in (err or "")
        assert "0" in (err or "")

    def test_promote_short_rationale_rejected(self):
        passed, err = check_irreversible_rationale("promote", "太短")
        assert passed is False
        assert "≥10" in (err or "") or "10" in (err or "")

    def test_promote_9_chars_rejected(self):
        # 边界 = 9 字 = 不足
        passed, err = check_irreversible_rationale("promote", "一二三四五六七八九")
        assert passed is False

    def test_promote_10_chars_allowed(self):
        # 边界 = 10 字 = 刚好
        passed, err = check_irreversible_rationale("promote", "一二三四五六七八九十")
        assert passed is True
        assert err is None

    def test_promote_long_rationale_allowed(self):
        passed, err = check_irreversible_rationale(
            "promote",
            "stable_clear verified, rationale 充分充分充分充分充分充分",
        )
        assert passed is True
        assert err is None

    def test_hold_no_rationale_allowed(self):
        # hold 可逆, 不要求 rationale
        passed, err = check_irreversible_rationale("hold", "")
        assert passed is True
        assert err is None

    def test_reject_no_rationale_allowed(self):
        passed, err = check_irreversible_rationale("reject", "")
        assert passed is True
        assert err is None

    def test_refine_no_rationale_allowed(self):
        passed, err = check_irreversible_rationale("refine", "")
        assert passed is True
        assert err is None


# ---------- PRO-08 守卫单测 ----------

class TestPro08SignedBy:
    def test_no_signed_by_rejected(self):
        passed, err = check_pro08_signed_by("")
        assert passed is False
        assert "PRO-08" in (err or "")

    def test_none_signed_by_rejected(self):
        passed, err = check_pro08_signed_by(None)
        assert passed is False
        assert "PRO-08" in (err or "")

    def test_signed_by_present_allowed(self):
        passed, err = check_pro08_signed_by("moc")
        assert passed is True
        assert err is None

    def test_signed_by_long_string_allowed(self):
        passed, err = check_pro08_signed_by("sih-engine actor moc 2026-08-18")
        assert passed is True
        assert err is None


# ---------- check_baseline 主入口测试 ----------

class TestCheckBaselineHappy:
    def test_legal_promote_pass(self):
        result = check_baseline(_good_report())
        assert result["verdict"] == "PASS"
        assert result["guards_passed"] == 4
        assert result["guards_failed"] == 0
        assert len(result["guard_results"]) == 4
        for g in result["guard_results"]:
            assert g["passed"] is True
            assert g["error"] is None

    def test_result_structure(self):
        result = check_baseline(_good_report())
        assert "verdict" in result
        assert "verdict_v3" in result
        assert "decision" in result
        assert "tier" in result
        assert "signed_by" in result
        assert "rationale_len" in result
        assert "regulation_anchor" in result
        assert "guards_passed" in result
        assert "guards_failed" in result
        assert "guard_results" in result
        assert "timestamp" in result

    def test_result_timestamp_iso8601(self):
        result = check_baseline(_good_report())
        # 2026-08-18T14:08:25+00:00 格式
        assert "T" in result["timestamp"]
        ts = result["timestamp"]
        # 至少含日期部分
        assert len(ts) >= 10

    def test_rationale_len_recorded(self):
        report = _good_report()
        report["rationale"] = "1234567890"  # 10 字
        result = check_baseline(report)
        assert result["rationale_len"] == 10


class TestCheckBaselineBoundary:
    def test_boundary_promote_returns_BOUNDARY(self):
        report = _good_report()
        report["layer1_verdict_v3"] = "boundary"
        result = check_baseline(report)
        assert result["verdict"] == "BOUNDARY"
        assert result["guards_failed"] == 1
        # PRO-07 守卫失败, 其他通过
        failed = [g for g in result["guard_results"] if not g["passed"]]
        assert len(failed) == 1
        assert failed[0]["name"] == "PRO-07"

    def test_boundary_refine_passes(self):
        report = _good_report()
        report["layer1_verdict_v3"] = "boundary"
        report["decision"] = "refine"  # boundary 路由 refine = 合规
        result = check_baseline(report)
        assert result["verdict"] == "PASS"
        assert result["guards_passed"] == 4


class TestCheckBaselineFail:
    def test_核心公理_promote_returns_FAIL(self):
        report = _good_report()
        report["tier"] = "核心公理"
        result = check_baseline(report)
        assert result["verdict"] == "FAIL"
        assert result["guards_failed"] == 1
        failed = [g for g in result["guard_results"] if not g["passed"]]
        assert failed[0]["name"] == "PRO-10-c"

    def test_no_signed_by_returns_FAIL(self):
        report = _good_report()
        report["signed_by"] = ""
        result = check_baseline(report)
        assert result["verdict"] == "FAIL"
        failed = [g for g in result["guard_results"] if not g["passed"]]
        assert failed[0]["name"] == "PRO-08"

    def test_short_rationale_returns_FAIL(self):
        report = _good_report()
        report["rationale"] = "短"
        result = check_baseline(report)
        assert result["verdict"] == "FAIL"
        failed = [g for g in result["guard_results"] if not g["passed"]]
        assert failed[0]["name"] == "irreversible"

    def test_multiple_guards_failed_priority_to_BOUNDARY(self):
        # PRO-07 boundary 失败 + PRO-08 也失败 = BOUNDARY 优先
        report = _good_report()
        report["layer1_verdict_v3"] = "boundary"
        report["signed_by"] = ""
        result = check_baseline(report)
        # 2 个失败, PRO-07 含 "boundary" 关键词, 应判 BOUNDARY
        assert result["verdict"] == "BOUNDARY"
        assert result["guards_failed"] == 2

    def test_multiple_guards_failed_no_BOUNDARY_is_FAIL(self):
        # PRO-10-c 失败 + PRO-08 失败 (无 boundary 触发) = FAIL
        report = _good_report()
        report["tier"] = "核心公理"
        report["signed_by"] = ""
        result = check_baseline(report)
        assert result["verdict"] == "FAIL"
        assert result["guards_failed"] == 2


class TestCheckBaselineGuardToggles:
    def test_baseline_pro07_disabled_skips(self):
        report = _good_report()
        report["layer1_verdict_v3"] = "boundary"
        report["decision"] = "promote"
        # 关闭 PRO-07 守卫
        result = check_baseline(report, baseline_pro07=False)
        assert result["guards_passed"] == 3
        assert result["guards_failed"] == 0
        # guard_results 长度 3 (PRO-07 不在)
        names = [g["name"] for g in result["guard_results"]]
        assert "PRO-07" not in names
        assert result["verdict"] == "PASS"

    def test_all_baselines_disabled_always_pass(self):
        report = _good_report()
        report["layer1_verdict_v3"] = "boundary"
        report["tier"] = "核心公理"
        report["signed_by"] = ""
        report["rationale"] = ""
        result = check_baseline(
            report,
            baseline_pro07=False,
            baseline_pro10c=False,
            baseline_irreversible=False,
            baseline_pro08=False,
        )
        assert result["guards_passed"] == 0
        assert result["guards_failed"] == 0
        assert result["verdict"] == "PASS"
        assert result["guard_results"] == []


class TestCheckBaselineFieldFallback:
    def test_verdict_field_fallback(self):
        # 旧 report 用 "verdict" 字段 (v1) 而非 "layer1_verdict_v3"
        report = _good_report()
        report.pop("layer1_verdict_v3")
        report["verdict"] = "stable_clear"
        result = check_baseline(report)
        assert result["verdict_v3"] == "stable_clear"
        assert result["verdict"] == "PASS"

    def test_no_verdict_v3_field(self):
        # 无 verdict 字段
        report = _good_report()
        report.pop("layer1_verdict_v3")
        result = check_baseline(report)
        assert result["verdict_v3"] is None
        # 缺 verdict + promote 不触发 PRO-07 (因为 verdict 不是 boundary)
        assert result["verdict"] == "PASS"

    def test_no_decision(self):
        report = _good_report()
        report.pop("decision")
        result = check_baseline(report)
        assert result["decision"] is None
        # 无 decision → PRO-07 不触发 (因为 promote 才触发)
        # PRO-10-c 不触发 (promote 才触发)
        # 不可逆不触发 (promote 才触发)
        assert result["verdict"] == "PASS"


# ---------- CLI 测试 ----------

class TestCLI:
    def test_cli_pass_returns_0(self):
        report = _good_report()
        proc = subprocess.run(
            [sys.executable, str(PROBES_DIR / "baseline_checker.py"), "--report", json.dumps(report)],
            capture_output=True,
            text=True,
        )
        assert proc.returncode == 0, f"stderr={proc.stderr}"
        # 输出含 verdict
        result = json.loads(proc.stdout)
        assert result["verdict"] == "PASS"
        assert result["guards_passed"] == 4

    def test_cli_fail_returns_1(self):
        report = _good_report()
        report["signed_by"] = ""  # PRO-08 失败
        proc = subprocess.run(
            [sys.executable, str(PROBES_DIR / "baseline_checker.py"), "--report", json.dumps(report)],
            capture_output=True,
            text=True,
        )
        assert proc.returncode == 1
        result = json.loads(proc.stdout)
        assert result["verdict"] == "FAIL"

    def test_cli_boundary_returns_1(self):
        report = _good_report()
        report["layer1_verdict_v3"] = "boundary"
        report["decision"] = "promote"
        proc = subprocess.run(
            [sys.executable, str(PROBES_DIR / "baseline_checker.py"), "--report", json.dumps(report)],
            capture_output=True,
            text=True,
        )
        assert proc.returncode == 1
        result = json.loads(proc.stdout)
        assert result["verdict"] == "BOUNDARY"

    def test_cli_missing_report_flag(self):
        proc = subprocess.run(
            [sys.executable, str(PROBES_DIR / "baseline_checker.py")],
            capture_output=True,
            text=True,
        )
        assert proc.returncode != 0

    def test_cli_invalid_json(self):
        proc = subprocess.run(
            [sys.executable, str(PROBES_DIR / "baseline_checker.py"), "--report", "not json"],
            capture_output=True,
            text=True,
        )
        assert proc.returncode != 0
