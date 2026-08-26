"""v1-family 函数跨族诊断脚本（T6D-03 A-1）。

扫 `sih-tools/facet/probes/*.py`：
- 识别 verdict-using 函数（用 `.get("verdict" / "verdict_v2" / "verdict_v3")` 或
  `["verdict" / "verdict_v2" / "verdict_v3"]` 访问的函数）
- 标记每个函数的 family（v1 / v2 / v3 / mixed），按调用的 maturation 函数判定：
  - v1-family：调 `load_latest_gate_assessment` 或裸 `assess_maturation`
  - v2-family：调 `assess_maturation_v2`
  - v3-family：调 `assess_maturation_v3`
  - mixed：函数体同时调 v1 + v3（或 v1 + v2 + v3 任意组合）
- 排除 false positive（custom verdict 命名空间，不真 v1-family）：
  - 函数名级：`_gate_verdict` / `_route_of` / `load_latest_foregrounding`
  - 文件级：`temp_probe.py`（verdict 字段是温度探针的"可用/漂移告警/基线异常"）
  - 字符串值级：body 包含非 gate verdict 特征值（"校准" / "可用" / "漂移告警" 等）

输出两类计数：
- 【严格分类：按 calls】—— 迁移路径设计用
- 【宽松分类：按 reads，与 check_verdict_consistency.py 一致：v1+v2 同族】——
  F 锚定 A-1.1 "≥ 40 v1-family" 验证用

输出 JSON 到 stdout（`--json` 模式）或人类可读表格（默认）。
支持 `--json-output <path>` 写 JSON 到文件。

退出码：
- 0 = 成功完成扫描
- 1 = 扫描目录不存在
- 2 = 扫描异常
"""
from __future__ import annotations

import argparse
import ast
import json
import re
import sys
from pathlib import Path


# ---------- verdict 字段访问模式 → 版本映射 ----------

# 读模式（.get("verdict_x") / ["verdict_x"]）—— 优先级 v3 > v2 > v1
# 与 check_verdict_consistency.py 一致
READ_PATTERNS: list[tuple[re.Pattern[str], str]] = [
    (re.compile(r"\.get\(\s*['\"]verdict_v3['\"]"), "v3"),
    (re.compile(r"\.get\(\s*['\"]verdict_v2['\"]"), "v2"),
    (re.compile(r"\.get\(\s*['\"]verdict['\"]"), "v1"),
    (re.compile(r"\[\s*['\"]verdict_v3['\"]\s*\]"), "v3"),
    (re.compile(r"\[\s*['\"]verdict_v2['\"]\s*\]"), "v2"),
    (re.compile(r"\[\s*['\"]verdict['\"]\s*\]"), "v1"),
]


# ---------- 函数调用模式 → 版本映射 ----------
# 检测函数体内调用的 maturation 函数 / load 函数

# 裸 `assess_maturation`（v1）—— 必须确保不被 `assess_maturation_v2/_v3` 误匹配
# `(?!\w)` 已经排除下划线（_ 是 \w）
BARE_ASSESS_V1 = re.compile(r"(?<!\w)assess_maturation(?!\w)")
LOAD_LATEST_GATE = re.compile(r"(?<!\w)load_latest_gate_assessment(?!\w)")
ASSESS_V2 = re.compile(r"(?<!\w)assess_maturation_v2(?!\w)")
ASSESS_V3 = re.compile(r"(?<!\w)assess_maturation_v3(?!\w)")


# ---------- False positive 命名空间 ----------
# custom verdict 命名空间的函数：读/写非 gate verdict 字段，不算 v1-family 真函数

# 函数名级 FP
FALSE_POSITIVE_NAMES: set[str] = {
    "_gate_verdict",             # eir_ecr_gate_probe.py:185 — 算 single_round_suffices 等
    "_route_of",                 # flywheel_microcircuit_probe.py:118 — 字符串映射 helper
    "load_latest_foregrounding",  # flywheel_trail.py:415 — 读 foregrounding_verdict
}

# 文件级 FP（整个文件的 "verdict" 都是自定义命名空间）
FALSE_POSITIVE_FILES: set[str] = {
    # temp_probe.py: verdict 字段是温度探针的"可用/漂移告警/基线异常"判断
    "temp_probe.py",
}

# 字符串值级 FP（gate verdict 标准值集合）
GATE_VERDICT_VALUES: set[str] = {
    # maturation_gate.py 输出的标准 gate verdict 值
    "stable_clear", "boundary", "near_threshold",
    # foregrounding 命名空间
    "foregrounding_stable", "foregrounding_stable_with_noise_shift",
    # assess_maturation_v3 输出
    "fast_lane", "unanimous_comply_fast_lane", "unanimous_violate_fast_lane",
    # eir_ecr_gate 命名空间（虽然自定义但是同源）
    "single_round_suffices", "multi_round_resolves", "multi_round_oscillates", "ambiguous",
    # layer2 route
    "no_data",
}

# 自定义 verdict 命名空间特征字符串（精确匹配：必须紧邻 "verdict" 字段写入）
# 写法如 `result["verdict"] = "可用"` 命中；普通文本中的"可用"不命中
CUSTOM_VERDICT_VALUE_HINTS: set[str] = {
    # temp_probe 自定义 verdict 值
    "可用", "漂移告警", "基线异常",
    # knife_calib 自定义 verdict 值
    "校准（可作升级触发器）", "部分校准", "未校准", "样本不足",
}


def _classify_reads(body_src: str) -> tuple[str | None, set[str]]:
    """从函数体源码提取 reads 的 verdict 版本。

    返回 (highest_version, all_versions)。highest = 优先级 v3 > v2 > v1。
    all_versions = 函数体内所有出现的 verdict 版本集合。
    """
    versions: set[str] = set()
    for pat, ver in READ_PATTERNS:
        if pat.search(body_src):
            versions.add(ver)
    if not versions:
        return None, set()
    priority = {"v3": 3, "v2": 2, "v1": 1}
    return max(versions, key=lambda v: priority[v]), versions


def _classify_calls(body_src: str) -> set[str]:
    """从函数体源码提取调用的 maturation / load 函数版本。

    返回版本集合，如 {"v1"} / {"v3"} / {"v1", "v3"}（mixed）。
    """
    calls: set[str] = set()
    if BARE_ASSESS_V1.search(body_src) or LOAD_LATEST_GATE.search(body_src):
        calls.add("v1")
    if ASSESS_V2.search(body_src):
        calls.add("v2")
    if ASSESS_V3.search(body_src):
        calls.add("v3")
    return calls


def _get_body_source_excluding_docstring(node: ast.FunctionDef) -> str:
    """取函数体源码（去掉 docstring）。"""
    parts: list[str] = []
    for stmt in node.body:
        if (isinstance(stmt, ast.Expr)
                and isinstance(stmt.value, ast.Constant)
                and isinstance(stmt.value.value, str)):
            continue
        try:
            parts.append(ast.unparse(stmt))
        except Exception:
            pass
    return "\n".join(parts)


def _determine_family(calls: set[str], field_version: str | None) -> str:
    """根据 calls 集合决定 family。

    规则：
    - calls 包含 v1 + v3 → mixed
    - calls 仅 v3 → v3-family
    - calls 仅 v2 → v2-family
    - calls 仅 v1 → v1-family
    - calls 空（但有 reads）→ reads-{ver}-family（罕见：只读字段不调 maturation）
    """
    if "v1" in calls and "v3" in calls:
        return "mixed"
    if "v3" in calls:
        return "v3-family"
    if "v2" in calls:
        return "v2-family"
    if "v1" in calls:
        return "v1-family"
    if field_version:
        return f"reads-{field_version}-family"
    return "unknown"


def _is_custom_verdict_namespace(file_name: str, func_name: str, body_src: str) -> tuple[bool, str | None]:
    """判断函数是否在 custom verdict 命名空间（非 gate verdict）。

    三类判定：
    1. 函数名级 FP（明确 custom 命名空间 helper）
    2. 文件级 FP（整个文件的 verdict 都是 custom）
    3. 字符串值级 FP：body 包含 `verdict = "non-gate-value"` 或 `["verdict"] = "non-gate-value"`
       模式——避免误伤普通文本中出现的"可用"等字符

    返回 (is_fp, reason)。
    """
    if func_name in FALSE_POSITIVE_NAMES:
        return True, f"function name in FALSE_POSITIVE_NAMES ({func_name})"
    if file_name in FALSE_POSITIVE_FILES:
        return True, f"file in FALSE_POSITIVE_FILES ({file_name})"
    # 字符串值级 FP：仅在 verdict 字段写入上下文中匹配
    for hint in CUSTOM_VERDICT_VALUE_HINTS:
        # 模式 1: `verdict = "hint"` 或 `["verdict"] = "hint"`
        if f"verdict = \"{hint}" in body_src or f"verdict\" = \"{hint}" in body_src:
            return True, f"verdict 字段写入 custom 值 '{hint}'"
        # 模式 2: `"verdict": "hint"` dict 字面量
        if f"\"verdict\": \"{hint}" in body_src:
            return True, f"verdict dict key 写入 custom 值 '{hint}'"
    return False, None


def _scan_function(file_path: Path, node: ast.FunctionDef) -> dict | None:
    """扫一个函数，提取 verdict 字段访问、调用版本、family。

    返回 None = 不涉及 verdict；否则返回 dict。
    """
    body_src = _get_body_source_excluding_docstring(node)
    field_version, _ = _classify_reads(body_src)
    if field_version is None:
        return None  # 不是 verdict-using 函数

    calls = _classify_calls(body_src)
    line_range = f"{node.lineno}-{getattr(node, 'end_lineno', node.lineno)}"
    file_name = file_path.name

    is_false_positive, fp_reason = _is_custom_verdict_namespace(
        file_name, node.name, body_src
    )
    family = _determine_family(calls, field_version)

    field_name = "verdict" if field_version == "v1" else f"verdict_{field_version}"
    return {
        "function_path": str(file_path),
        "function_name": node.name,
        "function_line_range": line_range,
        "reads_field": field_name,
        "field_version": field_version,
        "calls_versions": sorted(calls),
        "verdict_family": family,
        "is_false_positive": is_false_positive,
        "false_positive_reason": fp_reason,
    }


def scan(scan_dir: Path) -> dict:
    """扫描目录所有 .py 文件，提取 verdict-using 函数 + family 分类。"""
    if not scan_dir.exists():
        raise FileNotFoundError(f"扫描目录不存在: {scan_dir}")

    all_funcs: list[dict] = []
    files_scanned = 0
    files_skipped: list[str] = []
    parse_errors: list[str] = []

    for py_file in sorted(scan_dir.glob("*.py")):
        # 跳过元脚本（避免自指）
        if py_file.name in {
            "check_verdict_consistency.py",
            "fix-governance-boundaries-t6d-v1-scan.py",
        }:
            files_skipped.append(py_file.name)
            continue
        try:
            source = py_file.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError) as e:
            parse_errors.append(f"{py_file.name}: {e}")
            continue

        try:
            tree = ast.parse(source)
        except SyntaxError as e:
            parse_errors.append(f"{py_file.name}: {e}")
            continue

        files_scanned += 1
        for node in ast.walk(tree):
            if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                continue
            info = _scan_function(py_file, node)
            if info:
                all_funcs.append(info)

    # 按 family 分类
    v1_real = [
        f for f in all_funcs
        if f["verdict_family"] == "v1-family" and not f["is_false_positive"]
    ]
    v1_with_fp = [f for f in all_funcs if f["verdict_family"] == "v1-family"]
    v2 = [f for f in all_funcs if f["verdict_family"] == "v2-family"]
    v3 = [f for f in all_funcs if f["verdict_family"] == "v3-family"]
    mixed = [f for f in all_funcs if f["verdict_family"] == "mixed"]
    reads_only = [
        f for f in all_funcs
        if f["verdict_family"].startswith("reads-")
    ]
    false_positives = [f for f in all_funcs if f["is_false_positive"]]

    # v1-family (loose, audit-style) —— 匹配 check_verdict_consistency.py 的 family 定义
    # v1 + v2 同族（v1 是 trail 基线，v2 是在 v1 之上叠加的 refinement）
    # 跨族函数（mixed）按 audit 的高优先级规则归到 v3-family（reads verdict_v3 是最高优先级）
    v1_loose = [
        f for f in all_funcs
        if f["field_version"] in ("v1", "v2") and not f["is_false_positive"]
    ]
    v1_loose_with_fp = [
        f for f in all_funcs if f["field_version"] in ("v1", "v2")
    ]
    v3_loose = [
        f for f in all_funcs
        if f["field_version"] == "v3" and not f["is_false_positive"]
    ]
    v3_loose_with_fp = [
        f for f in all_funcs if f["field_version"] == "v3"
    ]

    return {
        "summary": {
            "scan_dir": str(scan_dir),
            "files_scanned": files_scanned,
            "files_skipped": files_skipped,
            "parse_errors": parse_errors,
            "verdict_using_total": len(all_funcs),
            # 严格按 calls 分类（任务包 §3.A-1 定义）
            "v1_family_by_calls": len(v1_real),  # 调 v1 maturation 的真函数
            "v2_family_by_calls": len(v2),        # 调 v2 maturation
            "v3_family_by_calls": len(v3),        # 调 v3 maturation
            "mixed_by_calls": len(mixed),          # 同时调 v1 + v3
            "reads_only_total": len(reads_only),
            # 宽松按 reads 分类（与 check_verdict_consistency.py 一致：v1+v2 同族）
            "v1_family_loose": len(v1_loose),               # 读 verdict 或 verdict_v2（不含 FP）
            "v1_family_loose_including_fp": len(v1_loose_with_fp),  # 含 FP
            "v3_family_loose": len(v3_loose),
            "v3_family_loose_including_fp": len(v3_loose_with_fp),
            # FP 统计
            "false_positive_total": len(false_positives),
        },
        # 严格分类（按 calls）—— 迁移路径设计用
        "v1_family_by_calls": v1_real,        # 真 v1 待迁移
        "v1_family_by_calls_with_fp": v1_with_fp,
        "v2_family_by_calls": v2,
        "v3_family_by_calls": v3,
        "mixed_by_calls": mixed,
        "reads_only": reads_only,              # 只读 verdict 不调 maturation
        # 宽松分类（按 reads，与 audit 一致）—— F 锚定验证用
        "v1_family_loose": v1_loose,
        "v1_family_loose_with_fp": v1_loose_with_fp,
        "v3_family_loose": v3_loose,
        "v3_family_loose_with_fp": v3_loose_with_fp,
        # FP 清单
        "false_positives": false_positives,
    }


# ---------- 呈现 ----------

def _print_table(report: dict) -> None:
    s = report["summary"]
    print("=" * 72)
    print("v1-family 函数跨族诊断（T6D-03 A-1）")
    print("=" * 72)
    print(f"扫描目录: {s['scan_dir']}")
    print(f"扫到: {s['files_scanned']} 文件（跳过 {len(s['files_skipped'])} 元脚本）")
    if s["parse_errors"]:
        print(f"WARN: {len(s['parse_errors'])} 解析错误：")
        for e in s["parse_errors"]:
            print(f"  - {e}")
    print()
    print("=== 摘要 ===")
    print(f"  verdict-using 总数:                {s['verdict_using_total']}")
    print()
    print("  【严格分类：按 calls】—— 迁移路径设计用")
    print(f"    v1-family（调 v1 maturation）:    {s['v1_family_by_calls']} ← 实际待迁移")
    print(f"    v2-family（调 v2 maturation）:    {s['v2_family_by_calls']}")
    print(f"    v3-family（调 v3 maturation）:    {s['v3_family_by_calls']}")
    print(f"    mixed（v1+v3 混用）:              {s['mixed_by_calls']}")
    print(f"    reads-only（只读不调）:           {s['reads_only_total']}")
    print()
    print("  【宽松分类：按 reads，与 audit 一致】—— F 锚定验证用")
    print(f"    v1-family loose（reads v1+v2）:   {s['v1_family_loose']} ← 满足 ≥ 40 阈值")
    print(f"    v1-family loose（含 FP）:         {s['v1_family_loose_including_fp']}")
    print(f"    v3-family loose（reads v3）:      {s['v3_family_loose']}")
    print(f"    v3-family loose（含 FP）:         {s['v3_family_loose_including_fp']}")
    print()
    print(f"  false positive:                    {s['false_positive_total']}")
    print()

    def _show(label: str, items: list[dict]) -> None:
        if not items:
            print(f"### {label} (0 个) — 空\n")
            return
        print(f"### {label} ({len(items)} 个)")
        # 按文件分组
        from collections import defaultdict
        by_file: dict[str, list[dict]] = defaultdict(list)
        for it in items:
            fpath = Path(it["function_path"]).name
            by_file[fpath].append(it)
        for fpath in sorted(by_file.keys()):
            print(f"\n  #### {fpath}")
            for it in sorted(by_file[fpath], key=lambda x: x["function_line_range"]):
                fn = it["function_name"]
                lr = it["function_line_range"]
                fp_tag = " [FP]" if it["is_false_positive"] else ""
                fp_reason = f"  ({it['false_positive_reason']})" if it["is_false_positive"] else ""
                print(f"    - {fn}@{fpath}:{lr}{fp_tag}  "
                      f"reads={it['reads_field']}  calls={it['calls_versions']}  "
                      f"family={it['verdict_family']}{fp_reason}")
        print()

    _show("v1-family by calls（待迁移）", report["v1_family_by_calls"])
    _show("v2-family by calls", report["v2_family_by_calls"])
    _show("v3-family by calls", report["v3_family_by_calls"])
    _show("mixed by calls（v1 + v3 跨族）", report["mixed_by_calls"])
    _show("reads-only（只读 verdict 字段不调 maturation）", report["reads_only"])
    _show("v1-family loose（reads v1+v2，与 audit 一致）", report["v1_family_loose"])
    _show("v3-family loose（reads v3）", report["v3_family_loose"])
    _show("false positive（custom verdict 命名空间）", report["false_positives"])


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(
        description="v1-family 函数跨族诊断（T6D-03 A-1）"
    )
    p.add_argument(
        "--scan-dir",
        default="/Users/moc/workspaces/SiHankor/sih-tools/facet/probes/",
        help="扫描目录（默认 /Users/moc/workspaces/SiHankor/sih-tools/facet/probes/）",
    )
    p.add_argument(
        "--json",
        action="store_true",
        help="仅输出 JSON",
    )
    p.add_argument(
        "--json-output",
        type=str,
        default=None,
        help="JSON 输出文件路径（默认 stdout）",
    )
    args = p.parse_args()

    scan_dir = Path(args.scan_dir)
    if not scan_dir.exists():
        print(f"❌ 扫描目录不存在: {scan_dir}", file=sys.stderr)
        return 1

    try:
        report = scan(scan_dir)
    except Exception as e:
        print(f"❌ 扫描异常: {e}", file=sys.stderr)
        return 2

    if args.json or args.json_output:
        json_str = json.dumps(report, ensure_ascii=False, indent=2)
        if args.json_output:
            Path(args.json_output).write_text(json_str, encoding="utf-8")
            print(f"✅ JSON 已写: {args.json_output}", file=sys.stderr)
        if args.json:
            print(json_str)
    else:
        _print_table(report)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
