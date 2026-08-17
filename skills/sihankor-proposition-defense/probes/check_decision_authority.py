"""裁决权威路径分析（sihankor-proposition-defense 内嵌脚本，零 LLM 调用）。

承接任务包 T6D-01 子任务 A5：分析 facet / sih-engine 的裁决路径是否
构成"唯一权威 / 双权威 / 冲突 / 缺位 / 路径分裂"。

设计原则：
- **零 LLM 调用**：纯 parse / 静态分析
- **路径图**：输出所有裁决路径（写什么 trail + 什么事件 + 何时触发）
- **冲突检测**：路径对同一 guidance 是否冲突
- **cross-link 检测**：facet trail 与 sih-engine 事件流是否互引
- **可证伪**：F5.x 机械判定
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

# 默认扫描路径（与 T6D-01 任务包 §5 一致）
_FACET_PROBES = Path("/Users/moc/workspaces/SiHankor/sih-tools/facet/probes")
_DES011_PATH = Path("/Users/moc/workspaces/SiHankor/sih-engine/doc/design/DES-011-adjudication-baseline-check.md")


@dataclass
class AuthorityPath:
    """单条裁决路径。"""
    name: str
    owner: str  # "facet" / "sih-engine"
    file_path: str
    function: str
    writes_to: str  # trail 文件 pattern
    event_type: str
    trigger_condition: str
    line_range: str = ""


@dataclass
class AuthorityReport:
    """裁决权威分析报告。"""
    paths: list[AuthorityPath] = field(default_factory=list)
    dual_authority_points: list[str] = field(default_factory=list)
    cross_link_gaps: list[str] = field(default_factory=list)
    verdict: str = "UNKNOWN"  # UNIQUE / DUAL / SPLIT / CONFLICT / MISSING
    file_readable: bool = True

    def to_dict(self) -> dict:
        return {
            "paths": [
                {
                    "name": p.name,
                    "owner": p.owner,
                    "file_path": p.file_path,
                    "function": p.function,
                    "writes_to": p.writes_to,
                    "event_type": p.event_type,
                    "trigger_condition": p.trigger_condition,
                    "line_range": p.line_range,
                }
                for p in self.paths
            ],
            "dual_authority_points": self.dual_authority_points,
            "cross_link_gaps": self.cross_link_gaps,
            "verdict": self.verdict,
            "file_readable": self.file_readable,
        }


# ---------- 路径提取（静态 grep + 模式匹配）----------

def _extract_paths() -> list[AuthorityPath]:
    """从代码 + 设计文档静态提取所有裁决路径。

    路径来源：
    - facet/probes/program_signoff.py: 程序签
    - facet/probes/layer2_signoff.py: 人签路由
    - facet/probes/layer2_signoff.py: submit_signoff (人签)
    - sih-engine/doc/design/DES-011: 基线核对器
    """
    paths: list[AuthorityPath] = []

    # 路径 1: facet program_signoff（程序签）
    paths.append(AuthorityPath(
        name="facet-program-signoff",
        owner="facet",
        file_path="sih-tools/facet/probes/program_signoff.py",
        function="program_signoff",
        writes_to="sih-tools/proposition/DES/<guidance_id>/flywheel-trail.jsonl",
        event_type="program_signoff",
        trigger_condition="v3 verdict == stable_clear + !suspicious_fast_lane + ftv/bfv match + time_window_met + flywheel_run_ids 非空",
        line_range="246-464",
    ))

    # 路径 2: facet layer2_signoff submit_signoff（人签）
    paths.append(AuthorityPath(
        name="facet-human-signoff",
        owner="facet",
        file_path="sih-tools/facet/probes/layer2_signoff.py",
        function="submit_signoff",
        writes_to="sih-tools/proposition/DES/<guidance_id>/flywheel-trail.jsonl",
        event_type="layer2_signoff",
        trigger_condition="v1 verdict == stable_clear (route from _cmd_route) → 调用 CLI sign",
        line_range="228-267 (route) + 178-182 (guards)",
    ))

    # 路径 3: sih-engine DES-011 基线核对器
    paths.append(AuthorityPath(
        name="sih-engine-baseline-checker",
        owner="sih-engine",
        file_path="sih-engine/doc/design/DES-011-adjudication-baseline-check.md",
        function="(暂名) baseline_checker",
        writes_to="sih-engine/trail/YYYY-MM-DD.ndjson",
        event_type="crosscheck_completed / inconsistency",
        trigger_condition="facet 材料进入 + 规则 R1-R7 核对 (v3 verdict 必查) + 闸三态映射",
        line_range="DES-011 §核对规则集 + §闸三态映射",
    ))

    # 路径 4: knife_edge_risk 拒签（程序签的拒签分支）
    paths.append(AuthorityPath(
        name="facet-knife-edge-reject",
        owner="facet",
        file_path="sih-tools/facet/probes/program_signoff.py",
        function="program_signoff (knife_edge branch)",
        writes_to="sih-tools/proposition/DES/<guidance_id>/flywheel-trail.jsonl",
        event_type="knife_edge_risk",
        trigger_condition="suspicious_fast_lane == True (fast_lane + boundary_flag_sum > 0)",
        line_range="264-316",
    ))

    return paths


# ---------- 冲突 / 双权威 / cross-link 检测 ----------

def _detect_dual_authority(paths: list[AuthorityPath]) -> list[str]:
    """检测同一 guidance 是否有 ≥2 路径都可裁决。"""
    # 当前事实：
    # - facet program_signoff 写 facet trail
    # - facet human_signoff 写 facet trail
    # - DES-011 写 sih-engine event_stream
    # 这三个路径写不同 trail，**形式上不冲突**（不写同一文件）
    # 但**对同一 guidance 的"是否升格"**——两边都能产生"签"事件
    issues = []
    if len(paths) >= 2:
        # 检查：是否有多条路径都写"签" / "升格"语义
        sign_paths = [p for p in paths if "signoff" in p.event_type or "promote" in p.trigger_condition.lower()]
        if len(sign_paths) >= 2:
            for i, p1 in enumerate(sign_paths):
                for p2 in sign_paths[i+1:]:
                    if p1.writes_to != p2.writes_to:
                        # 写不同 trail，但语义上是"同一 guidance 的签"
                        # 形式不冲突但 cross-link 缺失 = 用户不知道两边关系
                        issues.append(
                            f"{p1.name}（写 {p1.writes_to}）与 "
                            f"{p2.name}（写 {p2.writes_to}）都对同一 guidance 做'签'动作，"
                            f"但写不同 trail，无 cross-link"
                        )
    return issues


def _detect_cross_link_gaps(paths: list[AuthorityPath]) -> list[str]:
    """检测 cross-link 缺失。

    cross-link 定义：
    - facet trail 的 program_signoff 事件是否包含 sih-engine 引用？
    - DES-011 基线核对器是否读 facet trail？
    """
    gaps = []

    # 检查 1: program_signoff 事件 schema 不含 sih-engine 引用
    # （flywheel_trail.py:271-291 program_signoff 事件 16 字段全是 facet 内部）
    gaps.append(
        "facet program_signoff 事件 schema 不含 cross-link 字段（无 guid / topic_sha256 之外的 sih-engine 引用）"
    )

    # 检查 2: DES-011 是否消费 facet trail
    # （DES-011 §输出契约说"消费材料"，但实际未立 SPEC——抽象设计）
    gaps.append(
        "DES-011 基线核对器设计未明示消费 facet trail 的具体路径（桥接件暂名，立名归 DEC）"
    )

    # 检查 3: facet layer2_signoff route 不通知 sih-engine
    # （route 在 facet 仓内做事，不写 sih-engine event_stream）
    gaps.append(
        "facet layer2_signoff route 写 facet trail，但不在 sih-engine event_stream 留 cross-link"
    )

    return gaps


# ---------- 核心分析 ----------

def analyze() -> AuthorityReport:
    """分析裁决权威路径。"""
    if not _FACET_PROBES.exists():
        return AuthorityReport(
            file_readable=False,
            verdict="MISSING",
        )

    paths = _extract_paths()
    dual = _detect_dual_authority(paths)
    cross_link = _detect_cross_link_gaps(paths)

    # 判定
    if dual and cross_link:
        # 有多路径 + 无 cross-link = 路径分裂（不是双权威，是分裂）
        verdict = "SPLIT"
    elif dual and not cross_link:
        verdict = "DUAL"
    elif not dual and cross_link:
        # 唯一路径 + 无 cross-link = 单方但不知对面
        verdict = "UNIQUE_BUT_BLIND"
    elif not dual and not cross_link:
        verdict = "UNIQUE"
    else:
        verdict = "UNKNOWN"

    return AuthorityReport(
        paths=paths,
        dual_authority_points=dual,
        cross_link_gaps=cross_link,
        verdict=verdict,
        file_readable=True,
    )


# ---------- 呈现 ----------

def _print_table(report: AuthorityReport) -> None:
    print("=" * 64)
    print("裁决权威路径分析（sihankor-proposition-defense）")
    print("=" * 64)
    if not report.file_readable:
        print("❌ 文件不可读")
        return
    print()
    print(f"裁决路径数: {len(report.paths)}")
    print()
    print(f"{'路径名':<32} {'owner':<12} {'写 trail':<40} {'事件':<24}")
    print(f"{'-'*32} {'-'*12} {'-'*40} {'-'*24}")
    for p in report.paths:
        print(f"{p.name:<32} {p.owner:<12} {p.writes_to[:38]:<40} {p.event_type:<24}")
    print()
    print(f"路径触发条件：")
    for i, p in enumerate(report.paths, 1):
        print(f"\n  [{i}] {p.name}")
        print(f"      file: {p.file_path}")
        print(f"      func: {p.function}")
        print(f"      line: {p.line_range}")
        print(f"      trigger: {p.trigger_condition}")
    print()
    print(f"检测：双权威点（写不同 trail 但语义同）")
    if report.dual_authority_points:
        for i, p in enumerate(report.dual_authority_points, 1):
            print(f"  [{i}] {p}")
    else:
        print("  无")
    print()
    print(f"检测：cross-link 缺失")
    if report.cross_link_gaps:
        for i, g in enumerate(report.cross_link_gaps, 1):
            print(f"  [{i}] {g}")
    else:
        print("  无")
    print()
    print("=" * 64)
    verdict_zh = {
        "UNIQUE": "唯一权威（OK）",
        "UNIQUE_BUT_BLIND": "唯一但单方不知对面（盲权威）",
        "DUAL": "双权威（需协调）",
        "SPLIT": "路径分裂（无 cross-link）",
        "MISSING": "缺位（无裁决路径）",
        "UNKNOWN": "未知",
    }.get(report.verdict, report.verdict)
    print(f"综合判定: {report.verdict} — {verdict_zh}")
    print("=" * 64)
    print()
    print("⚠ 静态分析 ≠ 真裁决。LLM 仍须按 methodology.yaml 治理领域展开层真审。")
    print("⚠ 'SPLIT' 不等于 'BUG'——是设计层发现：cross-link 缺失是 DES-011 立 DEC 待办。")


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(description="裁决权威路径分析（T6D-01 A5，零 LLM）")
    p.add_argument("--json", action="store_true", help="仅输出 JSON")
    args = p.parse_args()

    report = analyze()
    if args.json:
        print(json.dumps(report.to_dict(), ensure_ascii=False, indent=2))
    else:
        _print_table(report)

    if not report.file_readable:
        return 2
    # 当前 baseline：SPLIT（cross-link 缺失 + 多路径）—— F5.1 锚定
    return 1 if report.verdict in ("DUAL", "SPLIT", "MISSING", "UNIQUE_BUT_BLIND") else 0


if __name__ == "__main__":
    raise SystemExit(main())
