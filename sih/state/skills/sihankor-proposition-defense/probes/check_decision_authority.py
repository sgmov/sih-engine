"""裁决权威路径分析（sihankor-proposition-defense 内嵌脚本，零 LLM 调用）。

承接任务包 T6D-01 子任务 A5 + T6D-03 子任务 B-1：分析 facet / sih-engine 的
裁决路径是否构成"唯一权威 / 双权威 / 冲突 / 缺位 / 路径分裂"，并实际读 DEC
文档 + schema 字段 + trail 事件，动态计算 cross-link gap 数量与 verdict。

设计原则：
- **零 LLM 调用**：纯 parse / 静态分析 + 文件读取
- **路径图**：输出所有裁决路径（写什么 trail + 什么事件 + 何时触发）
- **冲突检测**：路径对同一 guidance 是否冲突
- **cross-link 检测（动态）**：
  - facet 侧 schema 是否含 `sih_engine_event_id` + `cross_link_verified`
  - DES-011 DEC 文档是否定义 cross-link 协议
  - 实际 trail 事件中 `program_signoff` 是否含 `sih_engine_event_id` 字段
- **可证伪**：F5.x 机械判定

B-1 修订（2026-08-17，承接 T6D-03 fix-governance-boundaries-t6d §B-1）：
- 修前：硬编码 4 路径 + 硬编码"3 cross-link gap" → 永远判 UNIQUE_BUT_BLIND
- 修后：实际读 DEC 文档 + flywheel_trail.py schema + 生产 trail 事件，
  动态计算 gap 数量与 verdict
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

# ---------- 默认路径（与 T6D-01 §5 / T6D-03 §B-1 一致）----------

# 路径 3 的设计文档落地点（sih-engine 侧）
_DES011_PATH = Path("/Users/moc/workspaces/SiHankor/sih-engine/doc/design/DES-011-adjudication-baseline-check.md")
# cross-link 协议 DEC 文档（T6D-02 A5 立文，T6D-03 B-1 读取锚定）
_DEC_PATH = Path("/Users/moc/workspaces/SiHankor/sih-engine/doc/design/DES-011-baseline-checker-DEC.md")
# record_program_signoff 所在文件（T6D-03 B-1 schema 检测锚定）
_FLYWHEEL_TRAIL_PATH = Path("/Users/moc/workspaces/SiHankor/sih-tools/facet/probes/flywheel_trail.py")
# 默认生产 trail 根（与 audit_pipeline.py 一致）
_DEFAULT_TRAIL_ROOT = Path("/Users/moc/workspaces/SiHankor/sih-tools/proposition/DES")


# ---------- 数据类 ----------

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
    verdict: str = "UNKNOWN"  # UNIQUE / UNIQUE_BUT_BLIND / DUAL / SPLIT / CONFLICT / MISSING
    file_readable: bool = True
    # B-1 新增字段：cross-link 三层检测结果
    protocol_defined: bool = False           # DEC 文档是否定义 cross-link 协议
    schema_has_cross_link_fields: bool = False  # flywheel_trail.py record_program_signoff 是否含 2 字段
    events_with_cross_link: int = 0          # 含 sih_engine_event_id 的 program_signoff 事件数
    events_without_cross_link: int = 0       # 不含或 None 的 program_signoff 事件数
    total_program_signoff_events: int = 0    # 扫到的 program_signoff 事件总数
    dec_doc_path: str = ""                   # DEC 文档路径（用于回溯）
    schema_source_path: str = ""             # schema 来源路径
    trail_root_scanned: str = ""             # 实际扫描的 trail 根

    def to_dict(self) -> dict:
        return {
            "file_path": self.dec_doc_path or str(_DES011_PATH),  # 兼容旧字段
            "protocol_defined": self.protocol_defined,
            "schema_has_cross_link_fields": self.schema_has_cross_link_fields,
            "events_with_cross_link": self.events_with_cross_link,
            "events_without_cross_link": self.events_without_cross_link,
            "total_program_signoff_events": self.total_program_signoff_events,
            "verdict": self.verdict,
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
            "dec_doc_path": self.dec_doc_path,
            "schema_source_path": self.schema_source_path,
            "trail_root_scanned": self.trail_root_scanned,
            "file_readable": self.file_readable,
        }


# ---------- 路径提取（静态 hardcoded——承载代码层设计）----------

def _extract_paths() -> list[AuthorityPath]:
    """从代码 + 设计文档静态提取所有裁决路径。

    路径来源（T6D-01 §5 登记 + T6D-02 修订）：
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

    # 路径 3: sih-engine DES-011 baseline_checker（基线核对器）
    paths.append(AuthorityPath(
        name="sih-engine-baseline-checker",
        owner="sih-engine",
        file_path="sih-engine/doc/design/DES-011-adjudication-baseline-check.md",
        function="baseline_checker",
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


# ---------- B-1 新增：DEC 协议检测 ----------

def _check_dec_protocol() -> bool:
    """检查 DES-011 DEC 文档是否定义 cross-link 协议。

    三件套（须同时满足）：
    1. DEC 文件存在
    2. 含 cross-link 协议节（或等价段名）
    3. 协议节内显式提及 `sih_engine_event_id` + `cross_link_verified` 两字段

    Returns:
        bool: 协议是否在 DEC 文档中明确
    """
    if not _DEC_PATH.exists():
        return False
    try:
        text = _DEC_PATH.read_text(encoding="utf-8")
    except OSError:
        return False
    # 三件套文本标记
    has_protocol_section = bool(re.search(r"cross[-_ ]link.*协议|cross[-_ ]link\s*协议", text, re.IGNORECASE))
    has_event_id_field = "sih_engine_event_id" in text
    has_verified_field = "cross_link_verified" in text
    return has_protocol_section and has_event_id_field and has_verified_field


# ---------- B-1 新增：schema 字段检测 ----------

def _check_schema_fields() -> bool:
    """检查 flywheel_trail.py:record_program_signoff 函数签名是否含 cross-link 两字段。

    解析策略：regex 匹配 def record_program_signoff(...) -> dict 段，
    检查段内是否同时含 `sih_engine_event_id` 与 `cross_link_verified`。

    Returns:
        bool: schema 是否真写两字段
    """
    if not _FLYWHEEL_TRAIL_PATH.exists():
        return False
    try:
        text = _FLYWHEEL_TRAIL_PATH.read_text(encoding="utf-8")
    except OSError:
        return False
    # 匹配 def record_program_signoff( ... ) -> dict 段（DOTALL 含换行）
    match = re.search(
        r"def\s+record_program_signoff\s*\((.*?)\)\s*->\s*dict\s*:",
        text,
        re.DOTALL,
    )
    if not match:
        return False
    sig = match.group(1)
    has_event_id = "sih_engine_event_id" in sig
    has_verified = "cross_link_verified" in sig
    return has_event_id and has_verified


# ---------- B-1 新增：trail 实际事件扫描 ----------

def _scan_program_signoff_events(trail_root: Path) -> tuple[int, int, int]:
    """扫描 trail_root 下所有 flywheel-trail.jsonl 文件，统计 program_signoff 事件。

    判据：
    - `sih_engine_event_id` 字段存在且值非 None 且非空字符串 → 视为已 cross-link
    - 其余 program_signoff 事件（字段缺失 / None / 空串）→ 视为未 cross-link

    Returns:
        (total, with_link, without_link) 三元组
    """
    total = with_link = without_link = 0
    if not trail_root.exists():
        return total, with_link, without_link
    for trail_file in trail_root.rglob("flywheel-trail.jsonl"):
        try:
            with trail_file.open(encoding="utf-8") as fp:
                for line in fp:
                    line = line.strip()
                    if not line:
                        continue
                    try:
                        rec = json.loads(line)
                    except json.JSONDecodeError:
                        continue
                    if rec.get("trail_type") != "program_signoff":
                        continue
                    total += 1
                    event_id = rec.get("sih_engine_event_id")
                    if event_id is not None and event_id != "":
                        with_link += 1
                    else:
                        without_link += 1
        except OSError:
            continue
    return total, with_link, without_link


# ---------- 冲突 / 双权威检测（保留原行为）----------

def _detect_dual_authority(paths: list[AuthorityPath]) -> list[str]:
    """检测同一 guidance 是否有 ≥2 路径都可裁决。"""
    issues = []
    if len(paths) >= 2:
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


# ---------- B-1 修订：cross-link gap 动态检测 ----------

def _detect_cross_link_gaps(
    protocol_defined: bool,
    schema_has_fields: bool,
    total_events: int,
    events_with_link: int,
) -> list[str]:
    """动态计算 cross-link 缺失项（不再硬编码 3 个）。

    规则：
    - 协议未定义 → gap 1
    - schema 缺字段 → gap 2
    - 有 program_signoff 事件但 0 个含 cross-link → gap 3（实际事件层断）
    - 有 program_signoff 事件且 ≥1 个含 cross-link → 0 gap
    - 无 program_signoff 事件 → 1 gap（"等真实流量触发"）

    Returns:
        动态计算的 gap 列表（可能为空）
    """
    gaps: list[str] = []
    if not protocol_defined:
        gaps.append(
            f"DES-011 baseline_checker DEC 文档未定义 cross-link 协议（路径：{_DEC_PATH}）"
        )
    if not schema_has_fields:
        gaps.append(
            f"flywheel_trail.py:record_program_signoff schema 缺 cross-link 字段 "
            f"（{_FLYWHEEL_TRAIL_PATH} 未含 sih_engine_event_id + cross_link_verified 两参数）"
        )
    if total_events == 0:
        # baseline 状态：协议层 OK 但 0 事件触发
        if protocol_defined and schema_has_fields:
            gaps.append(
                "0 个 program_signoff 事件已写入——cross-link 协议层 OK，"
                "实际生效层待真实流量触发（baseline_checker 消费后回写 cross_link_verified）"
            )
    else:
        if events_with_link == 0:
            gaps.append(
                f"已有 {total_events} 个 program_signoff 事件，但 0 个含 cross-link 字段 "
                f"（{total_events} 个待 baseline_checker 消费并回写）"
            )
        # else: events_with_link > 0 → 实际生效层 OK，不加 gap
    return gaps


# ---------- 核心分析 ----------

def analyze(
    trail_root: Path | None = None,
    with_trail_scan: bool = True,
) -> AuthorityReport:
    """分析裁决权威路径。

    Args:
        trail_root: 扫描的 trail 根目录，默认生产路径
        with_trail_scan: 是否扫实际 trail 数据（关掉则只看协议 + schema）

    Returns:
        AuthorityReport 含 verdict + 4 路径 + 动态 cross-link gap + 三层检测结果
    """
    if not _DES011_PATH.parent.exists():
        return AuthorityReport(
            file_readable=False,
            verdict="MISSING",
            dec_doc_path=str(_DEC_PATH),
            schema_source_path=str(_FLYWHEEL_TRAIL_PATH),
            trail_root_scanned=str(trail_root or _DEFAULT_TRAIL_ROOT),
        )

    if trail_root is None:
        trail_root = _DEFAULT_TRAIL_ROOT

    paths = _extract_paths()
    dual = _detect_dual_authority(paths)

    # B-1 三层检测：协议 / schema / 实际事件
    protocol_defined = _check_dec_protocol()
    schema_has_fields = _check_schema_fields()
    if with_trail_scan:
        total_events, events_with_link, events_without_link = _scan_program_signoff_events(trail_root)
    else:
        total_events = events_with_link = events_without_link = 0

    # 动态 gap（不再硬编码 3）
    cross_link = _detect_cross_link_gaps(
        protocol_defined=protocol_defined,
        schema_has_fields=schema_has_fields,
        total_events=total_events,
        events_with_link=events_with_link,
    )

    # 判定逻辑（B-1 修订）：
    # - 协议未定义 或 schema 缺字段 → 协议层不 OK，回退到原行为（按 dual + cross_link 形态）
    # - 协议层 OK（protocol_defined AND schema_has_fields）：
    #   - events_with_link > 0 → 实际生效层 OK → UNIQUE
    #   - events_with_link == 0 → 实际生效层待触发 → UNIQUE_BUT_BLIND
    if not protocol_defined or not schema_has_fields:
        # 协议层未建立，按 dual + gap 形态判定
        if dual and cross_link:
            verdict = "SPLIT"
        elif dual and not cross_link:
            verdict = "DUAL"
        elif not dual and cross_link:
            verdict = "UNIQUE_BUT_BLIND"
        elif not dual and not cross_link:
            verdict = "UNIQUE"
        else:
            verdict = "UNKNOWN"
    else:
        # 协议层已建立，看实际生效层
        if events_with_link > 0:
            verdict = "UNIQUE"
        else:
            # 协议层 OK 但 0 事件已 cross-link = 基线状态（盲权威）
            verdict = "UNIQUE_BUT_BLIND"

    return AuthorityReport(
        paths=paths,
        dual_authority_points=dual,
        cross_link_gaps=cross_link,
        verdict=verdict,
        file_readable=True,
        protocol_defined=protocol_defined,
        schema_has_cross_link_fields=schema_has_fields,
        events_with_cross_link=events_with_link,
        events_without_cross_link=events_without_link,
        total_program_signoff_events=total_events,
        dec_doc_path=str(_DEC_PATH),
        schema_source_path=str(_FLYWHEEL_TRAIL_PATH),
        trail_root_scanned=str(trail_root),
    )


# ---------- 呈现 ----------

def _print_table(report: AuthorityReport) -> None:
    print("=" * 64)
    print("裁决权威路径分析（sihankor-proposition-defense，B-1 修订）")
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
    print(f"检测：cross-link 三层（动态，非硬编码）")
    print(f"  DEC 协议层:       {'OK' if report.protocol_defined else 'MISSING'}")
    print(f"  schema 字段层:    {'OK' if report.schema_has_cross_link_fields else 'MISSING'}")
    print(f"  实际生效层:       {report.events_with_cross_link}/{report.total_program_signoff_events} 事件已 cross-link")
    print()
    print(f"检测：cross-link 缺失（动态计算）")
    if report.cross_link_gaps:
        for i, g in enumerate(report.cross_link_gaps, 1):
            print(f"  [{i}] {g}")
    else:
        print("  无")
    print()
    print("=" * 64)
    verdict_zh = {
        "UNIQUE": "唯一权威（OK，协议 + 实际都建立）",
        "UNIQUE_BUT_BLIND": "唯一但单方不知对面（盲权威，协议层 OK 但 0 事件已 cross-link）",
        "DUAL": "双权威（需协调）",
        "SPLIT": "路径分裂（协议未建立 + cross-link 缺失）",
        "MISSING": "缺位（无裁决路径）",
        "UNKNOWN": "未知",
    }.get(report.verdict, report.verdict)
    print(f"综合判定: {report.verdict} — {verdict_zh}")
    print("=" * 64)
    print()
    print("⚠ 静态分析 ≠ 真裁决。LLM 仍须按 methodology.yaml 治理领域展开层真审。")
    print("⚠ baseline UNIQUE_BUT_BLIND = 协议层 OK 但 0 真实事件触发（待 baseline_checker 实际接流量）。")


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(description="裁决权威路径分析（T6D-01 A5 + T6D-03 B-1，零 LLM）")
    p.add_argument("--json", action="store_true", help="仅输出 JSON")
    p.add_argument("--trail-root", default=str(_DEFAULT_TRAIL_ROOT),
                    help=f"扫描的 trail 根目录（默认 {_DEFAULT_TRAIL_ROOT}）")
    p.add_argument("--no-trail-scan", action="store_true",
                    help="跳过实际 trail 扫描（只看 DEC + schema 两层）")
    args = p.parse_args()

    trail_root = Path(args.trail_root)
    report = analyze(
        trail_root=trail_root,
        with_trail_scan=not args.no_trail_scan,
    )
    if args.json:
        print(json.dumps(report.to_dict(), ensure_ascii=False, indent=2))
    else:
        _print_table(report)

    if not report.file_readable:
        return 2
    # 退出码：B-1 修订
    # 0 = UNIQUE（协议层 + 实际生效层都 OK）
    # 1 = UNIQUE_BUT_BLIND / DUAL / SPLIT / MISSING
    return 0 if report.verdict == "UNIQUE" else 1


if __name__ == "__main__":
    raise SystemExit(main())
