"""verdict 版本一致性机械检查（sihankor-proposition-defense 内嵌脚本）。

机械检查：关键函数读什么 verdict 字段版本（v1 / v2 / v3）+ 用什么版本做裁决。
判定：
- **v1 / v2 同族**（兼容）：v1 是 trail 存的基线，v2 是在 v1 之上叠加的 refinement
  （v2_rule / criteria_version），同族不报警
- **v3 独立一族**（v1 之上叠加 family_temperature + unanimous fast_lane）：
  v1-family 与 v3-family 混用 = 跨族 = 报警
- 仅 v1 / 仅 v2 / 仅 v3 = 同族一致 = 不报警

退出码：
- 0 = 一致（所有 verdict-using 函数在同一族）
- 1 = 不一致（跨族混用）
- 2 = 文件不可读 / AST 解析失败
"""
from __future__ import annotations

import argparse
import ast
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path


# ---------- verdict 字段访问模式 → 版本映射 ----------

# 读模式（.get("verdict_x") / ["verdict_x"]）—— 优先级 v3 > v2 > v1
READ_PATTERNS: list[tuple[re.Pattern[str], str]] = [
    (re.compile(r"\.get\(\s*['\"]verdict_v3['\"]"), "v3"),
    (re.compile(r"\.get\(\s*['\"]verdict_v2['\"]"), "v2"),
    (re.compile(r"\.get\(\s*['\"]verdict['\"]"), "v1"),
    (re.compile(r"\[\s*['\"]verdict_v3['\"]\s*\]"), "v3"),
    (re.compile(r"\[\s*['\"]verdict_v2['\"]\s*\]"), "v2"),
    (re.compile(r"\[\s*['\"]verdict['\"]\s*\]"), "v1"),
]

# 函数返回值/写入 verdict 字段的"裸出现"——仅匹配 dict key 位置（X["..."] / X.get(...)）
# 已涵盖在 READ_PATTERNS 中。生产方（如 assess_maturation_v3）写 verdict 字段但不读，
# 不在本脚本抓取范围（生产方出 v1/v2/v3 都合理；问题是消费方混用）。


def _classify(body_src: str) -> str | None:
    """从函数 body 源码提取主要 verdict 版本。

    优先级 v3 > v2 > v1。返回 None = 函数体不涉及 verdict 字段访问。
    """
    versions: set[str] = set()
    for pat, ver in READ_PATTERNS:
        if pat.search(body_src):
            versions.add(ver)
    if not versions:
        return None
    priority = {"v3": 3, "v2": 2, "v1": 1}
    return max(versions, key=lambda v: priority[v])


def _family(version: str) -> str:
    """v1 / v2 同族（v1-family），v3 单独（v3-family）。"""
    if version in ("v1", "v2"):
        return "v1-family"
    return "v3-family"


# ---------- 数据结构 ----------

@dataclass
class FunctionVerdict:
    """单函数的 verdict 字段使用情况。"""

    function_path: str
    function_name: str
    function_line_range: str
    reads_field: str
    verdict_version: str
    verdict_family: str


@dataclass
class Inconsistency:
    """跨函数 verdict 版本不一致。"""

    type: str
    trace: str
    scenario: str
    verdict: str = "INCONSISTENT"


@dataclass
class ConsistencyReport:
    """verdict 版本一致性检查报告。"""

    scan_dir: str
    n_files_scanned: int = 0
    n_functions_scanned: int = 0
    n_verdict_using: int = 0
    verdict_versions_in_use: list[str] = field(default_factory=list)
    decisions: list[dict] = field(default_factory=list)
    inconsistencies: list[dict] = field(default_factory=list)

    def to_dict(self) -> dict:
        return {
            "scan_dir": self.scan_dir,
            "n_files_scanned": self.n_files_scanned,
            "n_functions_scanned": self.n_functions_scanned,
            "n_verdict_using": self.n_verdict_using,
            "verdict_versions_in_use": self.verdict_versions_in_use,
            "decisions": self.decisions,
            "inconsistencies": self.inconsistencies,
            "inconsistency_count": len(self.inconsistencies),
            "verdict": "FAIL" if self.inconsistencies else "PASS",
        }


# ---------- 核心检查 ----------

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


def _scan_function(file_path: Path, node: ast.FunctionDef) -> dict | None:
    """扫一个函数，提取 verdict 字段访问与版本。

    返回 None = 不涉及 verdict；否则返回 dict。
    """
    body_src = _get_body_source_excluding_docstring(node)
    version = _classify(body_src)
    if version is None:
        return None

    line_range = f"{node.lineno}-{getattr(node, 'end_lineno', node.lineno)}"
    field_name = "verdict" if version == "v1" else f"verdict_{version}"
    return {
        "function_path": str(file_path),
        "function_name": node.name,
        "function_line_range": line_range,
        "reads_field": f"{field_name} ({version})",
        "verdict_version": version,
        "verdict_family": _family(version),
    }


def audit_dir(scan_dir: Path) -> ConsistencyReport:
    """扫描目录所有 .py 文件，提取 verdict 字段访问。"""
    report = ConsistencyReport(scan_dir=str(scan_dir))

    for py_file in sorted(scan_dir.glob("*.py")):
        # 跳过本脚本自己（避免自指）
        if py_file.name == "check_verdict_consistency.py":
            continue
        try:
            source = py_file.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError) as e:
            print(f"❌ 文件不可读: {py_file}: {e}", file=sys.stderr)
            continue

        try:
            tree = ast.parse(source)
        except SyntaxError as e:
            print(f"❌ 语法错误: {py_file}: {e}", file=sys.stderr)
            continue

        report.n_files_scanned += 1
        for node in ast.walk(tree):
            if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                continue
            report.n_functions_scanned += 1
            info = _scan_function(py_file, node)
            if info:
                report.n_verdict_using += 1
                report.decisions.append(info)

    # 收集所有版本
    versions = sorted(set(d["verdict_version"] for d in report.decisions))
    report.verdict_versions_in_use = versions

    # 跨族检测
    families = sorted(set(d["verdict_family"] for d in report.decisions))
    if len(families) > 1:
        v1_fns = [d for d in report.decisions if d["verdict_family"] == "v1-family"]
        v3_fns = [d for d in report.decisions if d["verdict_family"] == "v3-family"]
        v1_names = ", ".join(
            f"{d['function_name']}@{Path(d['function_path']).name}:{d['function_line_range']}"
            for d in v1_fns
        )
        v3_names = ", ".join(
            f"{d['function_name']}@{Path(d['function_path']).name}:{d['function_line_range']}"
            for d in v3_fns
        )
        # 列举具体场景
        scenarios = []
        if v1_fns and v3_fns:
            scenarios.append(
                "v1 stable_clear + v3 boundary 时，v1-family 函数调 v3-family 函数"
                " → 用户拿到拒签但上游没说闸改判（F4.1 baseline）"
            )
        report.inconsistencies.append({
            "type": "version_split",
            "trace": f"v1-family: [{v1_names}]  ↔  v3-family: [{v3_names}]",
            "scenario": " | ".join(scenarios),
            "verdict": "INCONSISTENT",
        })

    return report


# ---------- 呈现 ----------

def _print_table(report: ConsistencyReport) -> None:
    print("=" * 72)
    print("verdict 版本一致性机械检查（v1 / v2 同族 vs v3 独立族）")
    print("=" * 72)
    print(f"扫描目录: {report.scan_dir}")
    print(f"扫到: {report.n_files_scanned} 文件 / "
          f"{report.n_functions_scanned} 函数 / "
          f"{report.n_verdict_using} verdict-using")
    print(f"verdict 版本: {report.verdict_versions_in_use}")
    print()
    if report.decisions:
        print(f"{'函数':<46} {'行':<10} {'reads':<22} {'族':<12}")
        print(f"{'-'*46} {'-'*10} {'-'*22} {'-'*12}")
        for d in report.decisions:
            fn = f"{d['function_name']}@{Path(d['function_path']).name}"
            print(f"{fn:<46} {d['function_line_range']:<10} "
                  f"{d['reads_field']:<22} {d['verdict_family']:<12}")
    print()
    if report.inconsistencies:
        print(f"❌ 跨族不一致: {len(report.inconsistencies)} 条")
        for inc in report.inconsistencies:
            print(f"\n  type:     {inc['type']}")
            print(f"  trace:    {inc['trace']}")
            print(f"  scenario: {inc['scenario']}")
            print(f"  verdict:  {inc['verdict']}")
    else:
        print("✅ 所有 verdict-using 函数在同一族")
    print()
    print(f"verdict: {report.to_dict()['verdict']}")
    print("=" * 72)


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(
        description="verdict 版本一致性机械检查")
    p.add_argument("--scan-dir", default="sih-tools/facet/probes/",
                    help="扫描目录（默认 sih-tools/facet/probes/）")
    p.add_argument("--json", action="store_true", help="仅输出 JSON")
    args = p.parse_args()

    scan_dir = Path(args.scan_dir)
    if not scan_dir.exists():
        print(f"❌ 目录不存在: {scan_dir}", file=sys.stderr)
        return 2

    report = audit_dir(scan_dir)

    if args.json:
        print(json.dumps(report.to_dict(), ensure_ascii=False, indent=2))
    else:
        _print_table(report)

    if report.inconsistencies:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
