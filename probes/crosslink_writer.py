"""baseline_checker cross-link 写端: 写 ndjson + 回写 facet cross_link_verified.

承接: DES-011 baseline_checker DEC §cross-link 协议 + T6D-13 Cluster 4 X2.

工作流:
1. write_crosscheck_completed() - 写 ndjson 事件 (sih-engine/trail/YYYY-MM-DD.ndjson)
2. write_back_cross_link_verified() - 回写 facet 报告 (sih-tools/proposition/DES/<guidance_id>/*)
3. process_baseline() - 端到端入口 (check_baseline + 写 ndjson + 回写 facet)

cross-link 协议:
- 单向引用: facet report 加 sih_engine_event_id 字段 (facet → sih-engine)
- 双向确认: sih-engine 写完 crosscheck_completed 后回写 facet cross_link_verified=True
- 失败态: 不回写, 保持 cross_link_verified=False 作为"待核对"标记
"""
from __future__ import annotations

import hashlib
import json
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

# 让 crosslink_writer 既能被 `from probes.crosslink_writer import ...` 找到,
# 又能 import 同目录的 baseline_checker. 当以 `python3 -c 'from probes.crosslink_writer import ...'`
# 从 sih-engine/ 跑时, sys.path 包含 sih-engine, 不含 probes/, baseline_checker 直接
# import 失败, 须手动注入 probes/ 自身到 sys.path.
_PROBES_DIR = Path(__file__).resolve().parent
if str(_PROBES_DIR) not in sys.path:
    sys.path.insert(0, str(_PROBES_DIR))

from baseline_checker import check_baseline  # noqa: E402


# ---------- event_id 计算 ----------

def compute_event_id(result: dict, report: dict) -> str:
    """计算 crosscheck_completed 事件 ID (SHA-256 of verdict+timestamp, 取前 16 字符).

    不可变字段: verdict + guards_passed + guards_failed + timestamp + guidance_id
    同一 (verdict, guards_*, timestamp, guidance_id) 组合必产生同一 event_id,保证幂等.
    """
    payload = json.dumps({
        "verdict": result["verdict"],
        "guards_passed": result["guards_passed"],
        "guards_failed": result["guards_failed"],
        "timestamp": result["timestamp"],
        "guidance_id": report.get("guidance_id", ""),
    }, sort_keys=True, ensure_ascii=False)
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()[:16]


# ---------- ndjson 写入 ----------

def write_crosscheck_completed(
    result: dict,
    report: dict,
    trail_path: Path,
) -> dict:
    """写 crosscheck_completed ndjson 事件 (sih-engine/trail/YYYY-MM-DD.ndjson).

    Args:
        result: check_baseline 返回的 dict (含 verdict/guards_*/guard_results/...)
        report: 原始 facet report dict (含 guidance_id/event_id)
        trail_path: ndjson 文件根目录 (例如 Path("sih-engine/trail"))

    Returns:
        event dict (含 event_id/event_type/verdict/...)

    边界:
    - 不调 LLM
    - 不读 facet 报告 (只接收 report 参数)
    - ndjson 追加 (一行一事件, 末尾换行符)
    - 文件不存在时自动创建 (含父目录)
    """
    event_id = compute_event_id(result, report)
    today = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    trail_file = trail_path / f"{today}.ndjson"
    trail_path.mkdir(parents=True, exist_ok=True)

    event: dict[str, Any] = {
        "event_id": event_id,
        "event_type": "crosscheck_completed",
        "verdict": result["verdict"],
        "guards_passed": result["guards_passed"],
        "guards_failed": result["guards_failed"],
        "guard_results": result["guard_results"],
        "verdict_v3": result["verdict_v3"],
        "decision": result["decision"],
        "tier": result["tier"],
        "signed_by": result["signed_by"],
        "rationale_len": result["rationale_len"],
        "regulation_anchor": result["regulation_anchor"],
        "sih_engine_event_id": event_id,
        "facet_event_id": report.get("event_id"),
        "guidance_id": report.get("guidance_id"),
        "timestamp": result["timestamp"],
        "trail_file": str(trail_file),
    }

    with open(trail_file, "a", encoding="utf-8") as f:
        f.write(json.dumps(event, ensure_ascii=False) + "\n")

    return event


# ---------- facet 回写 ----------

def write_back_cross_link_verified(
    event: dict,
    facet_report_path: Path,
) -> bool:
    """回写 facet cross_link_verified 字段 (sih-tools/proposition/DES/<guidance_id>/*).

    Args:
        event: write_crosscheck_completed 返回的 event dict
        facet_report_path: facet report json 文件绝对路径

    Returns:
        True = 成功回写, False = 文件不存在

    边界:
    - 文件不存在时返 False, 不抛异常
    - 保留原 JSON 其他字段不变
    - 新增 3 字段: cross_link_verified=True, sih_engine_event_id, cross_link_timestamp
    """
    if not facet_report_path.exists():
        return False

    with open(facet_report_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    data["cross_link_verified"] = True
    data["sih_engine_event_id"] = event["sih_engine_event_id"]
    data["cross_link_timestamp"] = event["timestamp"]

    with open(facet_report_path, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)

    return True


# ---------- 端到端入口 ----------

def process_baseline(
    report: dict,
    result: dict | None = None,
    trail_path: Path | None = None,
    facet_report_path: Path | None = None,
    *,
    call_check_baseline: bool = True,
    **check_kwargs: Any,
) -> dict:
    """baseline_checker 端到端入口: 写 ndjson + 回写 facet.

    Args:
        report: facet program_signoff 报告 dict
        result: 预计算 check_baseline 结果 (None = 自动调用 check_baseline)
        trail_path: ndjson 写入根目录 (None = 跳过 ndjson 写)
        facet_report_path: facet report 文件绝对路径 (None = 跳过回写)
        call_check_baseline: 是否自动调 check_baseline (False = 用传入的 result)
        **check_kwargs: 透传给 check_baseline 的参数 (baseline_pro07/...)

    Returns:
        {
            "event": {...} | None,  # ndjson event (trail_path None 时为 None)
            "result": {...},  # check_baseline 结果
            "facet_written": bool,  # 是否回写 facet 成功
            "facet_path": str | None,  # facet_report_path 字符串
        }

    边界:
    - 不调 LLM
    - check_baseline 由 baseline_checker X1 提供
    - ndjson 写 + facet 回写 各自独立 (一个失败不影响另一个)
    - trail_path=None 时跳过 ndjson 写, 仍返回 result
    - facet_report_path=None 时跳过回写
    """
    if call_check_baseline or result is None:
        result = check_baseline(report, **check_kwargs)

    event: dict | None = None
    if trail_path is not None:
        event = write_crosscheck_completed(result, report, trail_path)

    facet_written = False
    if event is not None and facet_report_path is not None:
        facet_written = write_back_cross_link_verified(event, facet_report_path)

    return {
        "event": event,
        "result": result,
        "facet_written": facet_written,
        "facet_path": str(facet_report_path) if facet_report_path else None,
    }
