"""test docstring/name vs 实际测什么 机械检查（sihankor-proposition-defense 内嵌脚本）。

方法学真源 = 本脚本内嵌 deep-keyword 集合（暂未沉淀为 yaml；本仓当前只有
methodology.yaml 承载"立题/应用/治理"三层锚定，test coverage 维度独立）。
设计原则：
- **零 LLM 调用**：纯 AST 解析 + keyword 匹配 + 正则
- **可机械校验**：JSON 输出 + 退出码
- **深检查关键词驱动**：name / docstring 命中"复现 / 去重 / 幂等 / 指纹"等
  暗示深度文件级检查的关键词时，body 必须含文件读取操作
- **不替代真审**：declared vs actual 偏差只是机械提示，LLM 仍须真审
- **不预判内容**：列错位不替人给"应该改什么"

退出码：
- 0 = 全部对应（无错位）
- 1 = 有错位（declared intent 暗示深检查，body 实际只验一半）
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


# ---------- 深检查关键词（name / docstring 命中 = body 应含文件级检查） ----------

# 设计：只保留"直接指向文件级状态/跨调用一致性"的关键词。
# 排除 "stable_clear" / "boundary" / "knife_edge"——这些只是 verdict 值，
# 测 verdict 值 ≠ 测文件级状态。F3.2 要求"对简单测试零误报"。

# name 关键词：暗示"应验证 trail 文件状态 / 跨调用幂等"
DEEP_NAME_KEYWORDS = [
    "reproduc",       # reproducibility / reproducible / reproduce
    "fingerprint",    # fingerprint
    "idempot",        # idempotent / idempotence
    "deduplic",       # deduplicate / deduplication
    "dedup",          # dedup
    "not_duplic",     # not_duplicate / not_duplicated
    "no_duplic",      # no_duplicate
    "no_double",      # no_double_write
]

# docstring 关键词：暗示"应验证文件级状态"
DEEP_DOCSTRING_KEYWORDS = [
    "完全一致", "复现", "重现", "不重复",
    "idempot", "reproducib", "fingerprint",
    "不写入", "幂等", "去重", "不写重复",
    "trail 文件", "trail不被", "文件不被",
]

# 文件级操作 pattern（body 命中 = 真的检查了文件状态）
FILE_OP_PATTERNS = [
    r"\.read_text\(",
    r"\.read_bytes\(",
    r"\.iterdir\(",
    r"\.listdir\(",
    r"\blistdir\(",
    r"\.glob\(",
    r"\.rglob\(",
    r"\bopen\(",
    r"\.stat\(",
    r"\bos\.stat\(",
    r"\bfile_count\b",
    r"len\([^)]*iterdir",
    r"len\([^)]*listdir",
    r"sorted\([^)]*iterdir",
    r"sorted\([^)]*listdir",
    r"Path\([^)]*\)\.exists\(",
    r"\.load_latest_",
    r"\.load_trail\(",
    r"\.load_supers",
    r"\.read_records\(",
    r"\.iter_records\(",
    r"json\.loads\([^)]*read",
    r"json\.load\(",
]


# ---------- 数据结构 ----------

@dataclass
class TestMismatch:
    """单 test 错位记录。"""
    test_path: str
    test_name: str
    test_line_range: str
    declared_intent: str
    actual_assertion: str
    missing_coverage: str
    verdict: str  # PARTIAL / OK


@dataclass
class CoverageReport:
    """test coverage 检查报告。"""
    tests_dir: str
    n_tests_scanned: int = 0
    n_deep_tests: int = 0
    mismatches: list[dict] = field(default_factory=list)
    matched: list[dict] = field(default_factory=list)

    def to_dict(self) -> dict:
        return {
            "tests_dir": self.tests_dir,
            "n_tests_scanned": self.n_tests_scanned,
            "n_deep_tests": self.n_deep_tests,
            "mismatches": self.mismatches,
            "matched_count": len(self.matched),
            "mismatch_count": len(self.mismatches),
            "verdict": "FAIL" if self.mismatches else "PASS",
        }


# ---------- 核心检查 ----------

def _extract_docstring(node: ast.FunctionDef) -> str:
    """提取函数 docstring（标准位置：body[0]）。"""
    return ast.get_docstring(node) or ""


def _extract_asserts(node: ast.FunctionDef) -> list[str]:
    """提取函数体中的 assert 语句（unparse 形式）。"""
    asserts: list[str] = []
    for n in ast.walk(node):
        if isinstance(n, ast.Assert):
            try:
                asserts.append(ast.unparse(n.test))
            except Exception:
                asserts.append("<unparseable>")
    return asserts


def _get_body_source_excluding_docstring(node: ast.FunctionDef) -> str:
    """取函数体源码（去掉 docstring 那一行）。"""
    parts: list[str] = []
    for stmt in node.body:
        # 跳过 docstring
        if (isinstance(stmt, ast.Expr)
                and isinstance(stmt.value, ast.Constant)
                and isinstance(stmt.value.value, str)):
            continue
        try:
            parts.append(ast.unparse(stmt))
        except Exception:
            pass
    return "\n".join(parts)


def _has_file_op(body_src: str) -> bool:
    """body 源码是否含文件级操作。"""
    for pat in FILE_OP_PATTERNS:
        if re.search(pat, body_src):
            return True
    return False


def _detect_deep(name: str, docstring: str) -> tuple[bool, str, str]:
    """检测 (是否 deep, 来源=name|docstring|none, 命中关键词)。"""
    name_lower = name.lower()
    doc_lower = docstring.lower()
    for kw in DEEP_NAME_KEYWORDS:
        if kw in name_lower:
            return True, "name", kw
    for kw in DEEP_DOCSTRING_KEYWORDS:
        if kw.lower() in doc_lower:
            return True, "docstring", kw
    return False, "none", ""


def _analyze_test(test_path: Path, node: ast.FunctionDef) -> dict | None:
    """分析单个 test 函数。

    返回:
        None = 不是 deep test 或 body 有文件检查（PASS/不在报告范围）
        dict = mismatch 记录
    """
    name = node.name
    docstring = _extract_docstring(node)
    is_deep, source, kw = _detect_deep(name, docstring)

    if not is_deep:
        return None  # 非 deep test，不查

    body_src = _get_body_source_excluding_docstring(node)
    asserts = _extract_asserts(node)
    actual_assertion = "; ".join(asserts) if asserts else "<no asserts>"

    if _has_file_op(body_src):
        return None  # deep + body 含文件检查 = 一致，不入错位清单

    # 错位
    line_range = f"{node.lineno}-{getattr(node, 'end_lineno', node.lineno)}"
    declared_intent = (docstring[:80] if docstring
                       else f"name={name}（无 docstring）")
    actual_short = actual_assertion[:200]
    if source == "name":
        missing = (f"未验 trail 文件不被重复写（name 含 '{kw}' 暗示深度文件级检查，"
                   f"body 仅 in-memory dict 比对 / 无文件读取）")
    else:
        missing = (f"未验 trail 文件状态（docstring 含 '{kw}' 暗示文件级检查，"
                   f"body 无任何文件读取 / iterdir / read_text）")

    return {
        "test_path": str(test_path),
        "test_name": name,
        "test_line_range": line_range,
        "declared_intent": declared_intent,
        "actual_assertion": actual_short,
        "missing_coverage": missing,
        "deep_signal_source": source,
        "deep_keyword": kw,
        "verdict": "PARTIAL",
    }


def audit_tests_dir(tests_dir: Path) -> CoverageReport:
    """扫描 tests 目录所有 test_*.py 的 test_* 函数。"""
    report = CoverageReport(tests_dir=str(tests_dir))

    for test_file in sorted(tests_dir.glob("test_*.py")):
        try:
            source = test_file.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError) as e:
            print(f"❌ 文件不可读: {test_file}: {e}", file=sys.stderr)
            continue

        try:
            tree = ast.parse(source)
        except SyntaxError as e:
            print(f"❌ 语法错误: {test_file}: {e}", file=sys.stderr)
            continue

        for node in ast.walk(tree):
            if not isinstance(node, ast.FunctionDef):
                continue
            if not node.name.startswith("test_"):
                continue
            report.n_tests_scanned += 1
            is_deep, _, _ = _detect_deep(node.name, _extract_docstring(node))
            if is_deep:
                report.n_deep_tests += 1
            mismatch = _analyze_test(test_file, node)
            if mismatch:
                report.mismatches.append(mismatch)
            elif is_deep:
                line_range = (f"{node.lineno}-"
                              f"{getattr(node, 'end_lineno', node.lineno)}")
                report.matched.append({
                    "test_path": str(test_file),
                    "test_name": node.name,
                    "test_line_range": line_range,
                    "verdict": "OK",
                })

    return report


# ---------- 呈现 ----------

def _print_table(report: CoverageReport) -> None:
    print("=" * 64)
    print("test docstring/name vs 实际测什么 机械检查")
    print("=" * 64)
    print(f"tests 目录: {report.tests_dir}")
    print(f"扫描 test 函数: 总 {report.n_tests_scanned} / "
          f"deep {report.n_deep_tests}")
    print()
    if report.mismatches:
        print(f"❌ 错位清单（declared intent vs actual coverage）: "
              f"{len(report.mismatches)} 条")
        for m in report.mismatches:
            print(f"\n  [{m['test_path']}:{m['test_line_range']}] "
                  f"{m['test_name']}")
            print(f"    declared_intent:    {m['declared_intent']}")
            print(f"    actual_assertion:   {m['actual_assertion']}")
            print(f"    missing_coverage:   {m['missing_coverage']}")
            print(f"    deep_signal:        {m['deep_signal_source']}"
                  f"='{m['deep_keyword']}'")
            print(f"    verdict:            {m['verdict']}")
    else:
        print("✅ 所有 deep test 都有文件级检查")
    print()
    print(f"verdict: {report.to_dict()['verdict']}")
    print("=" * 64)
    print("⚠ 机械检查 ≠ 真审。LLM 仍须按 declared_intent 真审 test 覆盖度。")
    print("=" * 64)


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(
        description="test docstring/name vs 实际测什么 机械检查")
    p.add_argument("--tests-dir", default="sih-tools/facet/tests/",
                    help="tests 目录（默认 sih-tools/facet/tests/）")
    p.add_argument("--json", action="store_true", help="仅输出 JSON")
    args = p.parse_args()

    tests_dir = Path(args.tests_dir)
    if not tests_dir.exists():
        print(f"❌ tests 目录不存在: {tests_dir}", file=sys.stderr)
        return 2

    report = audit_tests_dir(tests_dir)

    if args.json:
        print(json.dumps(report.to_dict(), ensure_ascii=False, indent=2))
    else:
        _print_table(report)

    if report.mismatches:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
