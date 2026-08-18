"""crosslink_writer 单测: ndjson 写 + facet 回写 + 端到端 process_baseline.

承接: DES-011 baseline_checker DEC §cross-link 协议 + T6D-13 Cluster 4 X2.
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import pytest

# 允许 probe 文件 import
PROBES_DIR = Path(__file__).resolve().parent.parent
if str(PROBES_DIR) not in sys.path:
    sys.path.insert(0, str(PROBES_DIR))

from baseline_checker import check_baseline  # noqa: E402
from crosslink_writer import (  # noqa: E402
    compute_event_id,
    process_baseline,
    write_back_cross_link_verified,
    write_crosscheck_completed,
)


# ---------- 测试 fixture ----------

def _good_report() -> dict:
    """一个合法签核报告: stable_clear + promote + 衍生原则 + 充分 rationale + signed_by."""
    return {
        "guidance_id": "test-DES-001",
        "event_id": "facet-evt-abc123",
        "layer1_verdict_v3": "stable_clear",
        "decision": "promote",
        "tier": "衍生原则",
        "regulation_anchor": "PRO-007 签核",
        "signed_by": "moc",
        "rationale": "stable_clear verified, family_temperature v3, baseline_v1, rationale ≥10 字",
    }


def _good_result() -> dict:
    """check_baseline 输出的 PASS 结果 (供直接传入, 不调 check_baseline)."""
    return {
        "verdict": "PASS",
        "verdict_v3": "stable_clear",
        "decision": "promote",
        "tier": "衍生原则",
        "signed_by": "moc",
        "rationale_len": 60,
        "regulation_anchor": "PRO-007 签核",
        "guards_passed": 4,
        "guards_failed": 0,
        "guard_results": [
            {"name": "PRO-07", "passed": True, "error": None},
            {"name": "PRO-10-c", "passed": True, "error": None},
            {"name": "irreversible", "passed": True, "error": None},
            {"name": "PRO-08", "passed": True, "error": None},
        ],
        "timestamp": "2026-08-18T14:08:25+00:00",
    }


def _write_facet_report(path: Path, payload: dict | None = None) -> dict:
    """写一个 facet report JSON 文件, 返回 payload."""
    if payload is None:
        payload = {
            "guidance_id": "test-DES-001",
            "event_id": "facet-evt-abc123",
            "verdict": "stable_clear",
            "decision": "promote",
            "tier": "衍生原则",
            "signed_by": "moc",
            "rationale": "facet 端生成的 rationale",
        }
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(payload, f, ensure_ascii=False, indent=2)
    return payload


# ---------- compute_event_id 单测 ----------

class TestComputeEventId:
    def test_returns_16_char_hex(self):
        eid = compute_event_id(_good_result(), _good_report())
        assert len(eid) == 16
        assert re.fullmatch(r"[0-9a-f]{16}", eid), f"event_id 非 hex: {eid}"

    def test_deterministic_same_input(self):
        # 同 result + report → 同 event_id (幂等)
        result = _good_result()
        report = _good_report()
        eid1 = compute_event_id(result, report)
        eid2 = compute_event_id(result, report)
        assert eid1 == eid2

    def test_different_verdict_different_id(self):
        r1 = _good_result()
        r2 = _good_result()
        r2["verdict"] = "FAIL"
        eid1 = compute_event_id(r1, _good_report())
        eid2 = compute_event_id(r2, _good_report())
        assert eid1 != eid2

    def test_different_guidance_id_different_id(self):
        rep1 = _good_report()
        rep2 = _good_report()
        rep2["guidance_id"] = "test-DES-002"
        eid1 = compute_event_id(_good_result(), rep1)
        eid2 = compute_event_id(_good_result(), rep2)
        assert eid1 != eid2


# ---------- write_crosscheck_completed 单测 ----------

class TestWriteCrosscheckCompleted:
    def test_writes_ndjson_line(self, tmp_path: Path):
        trail_path = tmp_path / "trail"
        event = write_crosscheck_completed(
            _good_result(), _good_report(), trail_path,
        )

        today_match = re.search(r"trail/\d{4}-\d{2}-\d{2}\.ndjson", event["trail_file"])
        assert today_match is not None
        assert Path(event["trail_file"]).exists()

        with open(event["trail_file"], "r", encoding="utf-8") as f:
            line = f.readline().strip()
        assert line, "ndjson 行不应为空"
        parsed = json.loads(line)
        assert parsed["event_type"] == "crosscheck_completed"
        assert parsed["verdict"] == "PASS"
        assert parsed["sih_engine_event_id"] == parsed["event_id"]
        assert parsed["facet_event_id"] == "facet-evt-abc123"
        assert parsed["guidance_id"] == "test-DES-001"
        assert parsed["guards_passed"] == 4
        assert parsed["guards_failed"] == 0
        assert parsed["verdict_v3"] == "stable_clear"
        assert parsed["decision"] == "promote"
        assert parsed["tier"] == "衍生原则"
        assert parsed["signed_by"] == "moc"
        assert parsed["regulation_anchor"] == "PRO-007 签核"

    def test_event_dict_returned(self, tmp_path: Path):
        trail_path = tmp_path / "trail"
        event = write_crosscheck_completed(
            _good_result(), _good_report(), trail_path,
        )
        assert "event_id" in event
        assert "event_type" in event
        assert event["event_type"] == "crosscheck_completed"
        assert "guard_results" in event
        assert len(event["guard_results"]) == 4
        assert "trail_file" in event
        assert "timestamp" in event

    def test_appends_multiple_events(self, tmp_path: Path):
        # 同 ndjson 文件追加多事件
        trail_path = tmp_path / "trail"
        e1 = write_crosscheck_completed(_good_result(), _good_report(), trail_path)

        rep2 = _good_report()
        rep2["guidance_id"] = "test-DES-002"
        e2 = write_crosscheck_completed(_good_result(), rep2, trail_path)

        assert e1["trail_file"] == e2["trail_file"]  # 同一文件

        with open(e1["trail_file"], "r", encoding="utf-8") as f:
            lines = f.readlines()
        assert len(lines) == 2
        # 每行是合法 JSON
        for line in lines:
            assert json.loads(line)

    def test_trail_file_under_correct_date(self, tmp_path: Path):
        trail_path = tmp_path / "trail"
        event = write_crosscheck_completed(
            _good_result(), _good_report(), trail_path,
        )
        # 文件名是 YYYY-MM-DD.ndjson
        filename = Path(event["trail_file"]).name
        assert re.fullmatch(r"\d{4}-\d{2}-\d{2}\.ndjson", filename)

    def test_creates_parent_directory(self, tmp_path: Path):
        # trail_path 父目录不存在, 应自动创建
        trail_path = tmp_path / "deep" / "nested" / "trail"
        assert not trail_path.exists()
        event = write_crosscheck_completed(
            _good_result(), _good_report(), trail_path,
        )
        assert Path(event["trail_file"]).exists()

    def test_no_facet_event_id_handled(self, tmp_path: Path):
        # report 无 event_id 字段 → facet_event_id 应为 None
        report = _good_report()
        report.pop("event_id")
        event = write_crosscheck_completed(
            _good_result(), report, tmp_path / "trail",
        )
        assert event["facet_event_id"] is None


# ---------- write_back_cross_link_verified 单测 ----------

class TestWriteBackCrossLinkVerified:
    def test_returns_false_when_file_missing(self, tmp_path: Path):
        nonexistent = tmp_path / "missing.json"
        event = write_crosscheck_completed(
            _good_result(), _good_report(), tmp_path / "trail",
        )
        result = write_back_cross_link_verified(event, nonexistent)
        assert result is False

    def test_writes_three_fields(self, tmp_path: Path):
        # 写一个 facet report
        facet_path = tmp_path / "DES" / "test-001" / "report.json"
        _write_facet_report(facet_path)

        event = write_crosscheck_completed(
            _good_result(), _good_report(), tmp_path / "trail",
        )
        ok = write_back_cross_link_verified(event, facet_path)
        assert ok is True

        with open(facet_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        assert data["cross_link_verified"] is True
        assert data["sih_engine_event_id"] == event["sih_engine_event_id"]
        assert data["cross_link_timestamp"] == event["timestamp"]

    def test_preserves_existing_fields(self, tmp_path: Path):
        # 回写后, 原 JSON 其他字段不变
        facet_path = tmp_path / "report.json"
        original = {
            "guidance_id": "test-001",
            "verdict": "stable_clear",
            "rationale": "原 rationale 不动",
            "nested": {"key": "value", "list": [1, 2, 3]},
        }
        _write_facet_report(facet_path, original)

        event = write_crosscheck_completed(
            _good_result(), _good_report(), tmp_path / "trail",
        )
        write_back_cross_link_verified(event, facet_path)

        with open(facet_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        # 原字段保留
        assert data["guidance_id"] == "test-001"
        assert data["verdict"] == "stable_clear"
        assert data["rationale"] == "原 rationale 不动"
        assert data["nested"] == {"key": "value", "list": [1, 2, 3]}
        # 新增 3 字段
        assert data["cross_link_verified"] is True
        assert data["sih_engine_event_id"] == event["sih_engine_event_id"]
        assert data["cross_link_timestamp"] == event["timestamp"]

    def test_overwrite_previous_cross_link(self, tmp_path: Path):
        # 二次回写覆盖前次字段
        facet_path = tmp_path / "report.json"
        _write_facet_report(facet_path)

        e1 = write_crosscheck_completed(_good_result(), _good_report(), tmp_path / "trail")
        write_back_cross_link_verified(e1, facet_path)

        e2 = write_crosscheck_completed(_good_result(), _good_report(), tmp_path / "trail2")
        write_back_cross_link_verified(e2, facet_path)

        with open(facet_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        assert data["sih_engine_event_id"] == e2["sih_engine_event_id"]


# ---------- process_baseline 端到端单测 ----------

class TestProcessBaselineHappy:
    def test_full_pipeline_pass(self, tmp_path: Path):
        # 端到端: report + result + trail + facet 都齐
        trail_path = tmp_path / "trail"
        facet_path = tmp_path / "DES" / "test-001" / "report.json"
        _write_facet_report(facet_path)

        out = process_baseline(
            report=_good_report(),
            trail_path=trail_path,
            facet_report_path=facet_path,
        )

        # result 校验
        assert out["result"]["verdict"] == "PASS"
        assert out["result"]["guards_passed"] == 4

        # event 校验
        assert out["event"] is not None
        assert out["event"]["event_type"] == "crosscheck_completed"
        assert out["event"]["sih_engine_event_id"] == out["event"]["event_id"]

        # facet 回写校验
        assert out["facet_written"] is True
        with open(facet_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        assert data["cross_link_verified"] is True
        assert data["sih_engine_event_id"] == out["event"]["sih_engine_event_id"]
        assert data["cross_link_timestamp"] == out["event"]["timestamp"]

        # ndjson 写校验
        trail_file = Path(out["event"]["trail_file"])
        assert trail_file.exists()
        with open(trail_file, "r", encoding="utf-8") as f:
            line = f.readline().strip()
        parsed = json.loads(line)
        assert parsed["event_type"] == "crosscheck_completed"

    def test_without_trail_path(self, tmp_path: Path):
        # trail_path = None → 跳过 ndjson 写, 仅返回 result
        facet_path = tmp_path / "report.json"
        _write_facet_report(facet_path)

        out = process_baseline(
            report=_good_report(),
            trail_path=None,
            facet_report_path=facet_path,
        )

        assert out["event"] is None
        assert out["result"]["verdict"] == "PASS"
        # facet 也不会回写 (因 event is None)
        assert out["facet_written"] is False

    def test_without_facet_path(self, tmp_path: Path):
        # facet_report_path = None → 跳过回写, 仍写 ndjson
        out = process_baseline(
            report=_good_report(),
            trail_path=tmp_path / "trail",
            facet_report_path=None,
        )

        assert out["event"] is not None
        assert out["facet_written"] is False
        assert out["facet_path"] is None
        # ndjson 仍写
        assert Path(out["event"]["trail_file"]).exists()

    def test_minimal_call(self, tmp_path: Path):
        # 最简调用: 只传 report → 自动 check_baseline + 写 ndjson (无 facet)
        out = process_baseline(
            report=_good_report(),
            trail_path=tmp_path / "trail",
        )

        assert out["result"]["verdict"] == "PASS"
        assert out["event"] is not None
        assert out["facet_written"] is False


class TestProcessBaselineVerdictBranch:
    def test_boundary_promote(self, tmp_path: Path):
        # boundary + promote → PRO-07 守卫失败 → verdict=BOUNDARY
        report = _good_report()
        report["layer1_verdict_v3"] = "boundary"
        report["decision"] = "promote"

        out = process_baseline(
            report=report,
            trail_path=tmp_path / "trail",
        )

        assert out["result"]["verdict"] == "BOUNDARY"
        assert out["result"]["guards_failed"] == 1
        failed = [g for g in out["result"]["guard_results"] if not g["passed"]]
        assert failed[0]["name"] == "PRO-07"
        # ndjson 仍写 (crosscheck_completed 不管 verdict 都写)
        assert out["event"] is not None
        assert out["event"]["verdict"] == "BOUNDARY"

    def test_fail_promote_core_axiom(self, tmp_path: Path):
        # promote + 核心公理 → PRO-10-c 守卫失败 → verdict=FAIL
        report = _good_report()
        report["tier"] = "核心公理"

        out = process_baseline(
            report=report,
            trail_path=tmp_path / "trail",
        )

        assert out["result"]["verdict"] == "FAIL"
        assert out["result"]["guards_failed"] == 1
        failed = [g for g in out["result"]["guard_results"] if not g["passed"]]
        assert failed[0]["name"] == "PRO-10-c"


class TestProcessBaselineCrossLinkProtocol:
    def test_unidirectional_reference(self, tmp_path: Path):
        # 单向引用: facet → sih-engine (facet.sih_engine_event_id = sih-engine event_id)
        facet_path = tmp_path / "report.json"
        _write_facet_report(facet_path)

        out = process_baseline(
            report=_good_report(),
            trail_path=tmp_path / "trail",
            facet_report_path=facet_path,
        )

        with open(facet_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        # 单向引用: facet 指向 sih-engine
        assert data["sih_engine_event_id"] == out["event"]["event_id"]
        assert data["sih_engine_event_id"] == out["event"]["sih_engine_event_id"]

    def test_bidirectional_confirmation(self, tmp_path: Path):
        # 双向确认: facet.cross_link_verified=True 表示已核对
        facet_path = tmp_path / "report.json"
        _write_facet_report(facet_path)
        # 初始: cross_link_verified 字段不存在
        with open(facet_path, "r", encoding="utf-8") as f:
            initial = json.load(f)
        assert "cross_link_verified" not in initial

        process_baseline(
            report=_good_report(),
            trail_path=tmp_path / "trail",
            facet_report_path=facet_path,
        )

        with open(facet_path, "r", encoding="utf-8") as f:
            after = json.load(f)
        # 双向确认: 置 True
        assert after["cross_link_verified"] is True

    def test_event_includes_facet_event_id(self, tmp_path: Path):
        # 事件含 facet_event_id 字段 (反向引用)
        out = process_baseline(
            report=_good_report(),
            trail_path=tmp_path / "trail",
        )
        assert out["event"]["facet_event_id"] == "facet-evt-abc123"
        # 含 guidance_id (供反向追踪)
        assert out["event"]["guidance_id"] == "test-DES-001"

    def test_no_llm_invocation(self, tmp_path: Path):
        # 约束 1: 0 LLM 调用 - process_baseline 全程机械
        # 此测试通过 import 检查: 不引入 llm-call 相关模块
        import crosslink_writer  # noqa: F401
        module_source = Path(crosslink_writer.__file__).read_text(encoding="utf-8")
        # 不应 import openai / anthropic / requests 等 LLM SDK
        forbidden_imports = ["import openai", "import anthropic", "from openai", "from anthropic"]
        for forbidden in forbidden_imports:
            assert forbidden not in module_source, (
                f"crosslink_writer 含 LLM SDK import: {forbidden}"
            )
        # 跑一遍确认无 LLM 调用
        out = process_baseline(
            report=_good_report(),
            trail_path=tmp_path / "trail",
        )
        assert out["event"] is not None


class TestProcessBaselinePreservesResult:
    def test_uses_provided_result_without_recheck(self, tmp_path: Path):
        # 传 result + call_check_baseline=False → 不用 check_baseline
        provided_result = {
            "verdict": "PASS",
            "verdict_v3": "stable_clear",
            "decision": "promote",
            "tier": "衍生原则",
            "signed_by": "moc",
            "rationale_len": 60,
            "regulation_anchor": "PRO-007 签核",
            "guards_passed": 4,
            "guards_failed": 0,
            "guard_results": [
                {"name": "PRO-07", "passed": True, "error": None},
                {"name": "PRO-10-c", "passed": True, "error": None},
                {"name": "irreversible", "passed": True, "error": None},
                {"name": "PRO-08", "passed": True, "error": None},
            ],
            "timestamp": "2026-08-18T14:08:25+00:00",
        }
        out = process_baseline(
            report=_good_report(),
            result=provided_result,
            trail_path=tmp_path / "trail",
            call_check_baseline=False,
        )

        # 用的就是传入的 result, 不会重新 check_baseline
        assert out["result"] is provided_result
        # ndjson 仍写
        assert out["event"] is not None


# ---------- 集成测试 (真实场景) ----------

class TestEndToEndScenario:
    def test_realistic_pipeline(self, tmp_path: Path):
        """真实场景模拟: 完整 DES / 衍生原则 / 程序签核 / 核对通过."""
        trail_path = tmp_path / "sih-engine" / "trail"
        facet_path = tmp_path / "sih-tools" / "proposition" / "DES" / "mt3-real-test" / "report.json"

        # 模拟 facet 端: 程序签已签 (cross_link_verified=False 初始)
        _write_facet_report(facet_path, {
            "guidance_id": "mt3-real-test",
            "event_id": "facet-evt-real001",
            "verdict": "stable_clear",
            "decision": "promote",
            "tier": "衍生原则",
            "regulation_anchor": "PRO-007 签核",
            "signed_by": "facet-program",
            "rationale": "stable_clear verified, family_temperature v3.0.1, baseline_v1",
            "signoff_kind": "program",
        })

        # 跑 baseline_checker 端到端
        out = process_baseline(
            report={
                "guidance_id": "mt3-real-test",
                "event_id": "facet-evt-real001",
                "layer1_verdict_v3": "stable_clear",
                "decision": "promote",
                "tier": "衍生原则",
                "regulation_anchor": "PRO-007 签核",
                "signed_by": "facet-program",
                "rationale": "stable_clear verified, family_temperature v3.0.1, baseline_v1",
            },
            trail_path=trail_path,
            facet_report_path=facet_path,
        )

        # 验 4 守卫全通过
        assert out["result"]["verdict"] == "PASS"
        assert out["result"]["guards_passed"] == 4
        assert out["result"]["guards_failed"] == 0

        # 验 ndjson 写 (sih-engine/trail/YYYY-MM-DD.ndjson)
        trail_file = Path(out["event"]["trail_file"])
        assert trail_file.exists()
        assert trail_file.parent == trail_path
        assert re.fullmatch(r"\d{4}-\d{2}-\d{2}\.ndjson", trail_file.name)

        # 验 facet 回写
        assert out["facet_written"] is True
        with open(facet_path, "r", encoding="utf-8") as f:
            facet = json.load(f)
        assert facet["cross_link_verified"] is True
        assert facet["sih_engine_event_id"] == out["event"]["event_id"]
        # 原字段保留
        assert facet["guidance_id"] == "mt3-real-test"
        assert facet["signoff_kind"] == "program"
        assert facet["signed_by"] == "facet-program"

        # 验 ndjson 事件内容
        with open(trail_file, "r", encoding="utf-8") as f:
            event_line = json.loads(f.readline().strip())
        assert event_line["event_type"] == "crosscheck_completed"
        assert event_line["verdict"] == "PASS"
        assert event_line["facet_event_id"] == "facet-evt-real001"
        assert event_line["guidance_id"] == "mt3-real-test"
        assert event_line["sih_engine_event_id"] == event_line["event_id"]
        # 4 守卫结果全在
        assert len(event_line["guard_results"]) == 4
        assert all(g["passed"] for g in event_line["guard_results"])
