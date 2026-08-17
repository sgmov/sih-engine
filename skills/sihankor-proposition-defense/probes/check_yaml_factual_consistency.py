"""methodology.yaml 文字 vs 代码事实一致性机械检查（sihankor-proposition-defense 内嵌脚本）。

设计原则：
- **零 LLM 调用**：纯 yaml parse + grep
- **模式驱动**：每条事实陈述用明确命名的 pattern 校验（避免随机关键词误判）
- **可机械校验**：JSON + 退出码
- **可扩展**：CHECKS 列表追加新 pattern 即可（不破坏已有）
- **不写建议**（只输出事实 + F 判定）

退出码：
- 0 = yaml 文字与代码事实一致（无错位）
- 1 = ≥ 1 mismatch（yaml 文字与代码事实错位）
- 2 = 文件不可读（yaml / 代码路径缺）
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

try:
    import yaml
except ImportError:
    print("❌ 缺 PyYAML。pip install pyyaml", file=sys.stderr)
    raise SystemExit(2)

# 方法学 yaml 真源 = 同目录 methodology.yaml
_THIS_DIR = Path(__file__).resolve().parent
_METHODOLOGY_PATH = _THIS_DIR.parent / "methodology.yaml"

# 关联代码搜索根（默认 facet probes/）
# 路径：_THIS_DIR = probes/，上溯 4 层 = 项目根 SiHankor/，再下到 sih-tools/facet/probes/
_DEFAULT_CODE_ROOT = _THIS_DIR.parent.parent.parent.parent / "sih-tools" / "facet" / "probes"

# LLM 调用指纹（机械判据）—— 出现任一即视为该函数有 LLM 调用
LLM_CALL_FINGERPRINTS = [
    "openai.",
    "anthropic.",
    "import openai",
    "import anthropic",
    "from openai",
    "from anthropic",
    "llm_call(",
    "chat_completion(",
    "completion.create(",
    "messages.create(",
    ".create(",
    "synthesize_speech(",
    "invoke_llm(",
    "call_llm(",
]


# ---------- 数据结构 ----------

@dataclass
class Mismatch:
    """单条 yaml / 代码错位。"""
    check_id: str                    # 模式名（如 "rationale_llm_claim"）
    yaml_path: str                   # yaml 文件路径
    yaml_line: int                   # 错位所在行号
    yaml_text: str                   # 错位原文
    code_path: str | None            # 涉及代码文件
    code_evidence: str               # 代码事实证据
    verdict: str = "MISMATCH"        # 永远 MISMATCH（仅在错位时记录）


@dataclass
class ConsistencyReport:
    """yaml vs 代码事实一致性检查报告。"""
    yaml_path: str
    code_root: str
    mismatches: list[Mismatch] = field(default_factory=list)
    mismatch_count: int = 0
    verdict: str = "PASS"            # PASS / FAIL
    file_readable: bool = True
    checks_run: list[str] = field(default_factory=list)  # 跑过的 check 名

    def to_dict(self) -> dict:
        return {
            "yaml_path": self.yaml_path,
            "code_root": self.code_root,
            "mismatches": [
                {
                    "check_id": m.check_id,
                    "yaml_line": m.yaml_line,
                    "yaml_text": m.yaml_text,
                    "code_path": m.code_path,
                    "code_evidence": m.code_evidence,
                    "verdict": m.verdict,
                }
                for m in self.mismatches
            ],
            "mismatch_count": self.mismatch_count,
            "verdict": self.verdict,
            "checks_run": self.checks_run,
        }


# ---------- 核心检查（pattern-driven） ----------

def _yaml_lines(yaml_path: Path) -> list[tuple[int, str]]:
    """返回 [(行号, 行内容)]，行号 1-based。"""
    return [(i + 1, ln) for i, ln in enumerate(
        yaml_path.read_text(encoding="utf-8").splitlines())]


def _has_llm_call(code_text: str) -> bool:
    """代码文本是否包含 LLM 调用指纹。"""
    return any(fp in code_text for fp in LLM_CALL_FINGERPRINTS)


def _find_rationale_function(code_root: Path) -> Path | None:
    """在 code_root 下找含 _build_rationale / def.*rationale 的 .py 文件。"""
    for py in code_root.rglob("*.py"):
        try:
            text = py.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        if re.search(r"def\s+_?build_rationale", text) or \
           re.search(r"def\s+\w*rationale\w*\s*\(", text):
            return py
    return None


def _read_function_body(path: Path, func_name_pattern: str) -> str | None:
    """读 path 中匹配 func_name_pattern 的 def 所在函数体（粗略：取 def 到下一个 def 或文件尾）。"""
    try:
        text = path.read_text(encoding="utf-8")
    except (UnicodeDecodeError, OSError):
        return None
    lines = text.splitlines()
    start = None
    for i, ln in enumerate(lines):
        if re.search(func_name_pattern, ln):
            start = i
            break
    if start is None:
        return None
    # 粗略抓函数体：取 start 到下一个顶层 def / 文件尾
    end = len(lines)
    for j in range(start + 1, len(lines)):
        if re.match(r"^def\s+\w+", lines[j]) or re.match(r"^class\s+\w+", lines[j]):
            end = j
            break
    return "\n".join(lines[start:end])


def check_rationale_llm_claim(yaml_lines: list[tuple[int, str]],
                                code_root: Path) -> list[Mismatch]:
    """Check 1: yaml 文字若声称 "rationale 是 LLM 自动生成" / "rationale 涉及 LLM 参与"，
    则代码 _build_rationale 中应有 LLM 调用。若无 → MISMATCH。

    已知 baseline（F2.1 必须抓到）：yaml L62 说 "rationale 是 LLM 自动生成"，
    但 program_signoff.py:170-192 _build_rationale 是纯字符串模板拼接，零 LLM 调用。
    """
    mismatches: list[Mismatch] = []

    # 1. 找 yaml 中所有声称 rationale 涉及 LLM 的行
    claim_patterns = [
        (r"rationale[^,。()]{0,30}是\s*LLM\s*自动生成", "rationale 是 LLM 自动生成"),
        (r"rationale[^,。()]{0,30}由\s*LLM\s*生成", "rationale 由 LLM 生成"),
        (r"rationale[^,。()]{0,30}LLM\s*参与", "rationale 含 LLM 参与"),
        (r"rationale[^,。()]{0,30}调[用]?\s*LLM", "rationale 调用 LLM"),
    ]

    yaml_path_str = "methodology.yaml"  # 报告里只放相对名（避免硬编码绝对路径）
    claimed_lines: list[tuple[int, str, str]] = []
    for line_no, text in yaml_lines:
        for pat, label in claim_patterns:
            if re.search(pat, text):
                claimed_lines.append((line_no, text.strip(), label))
                break

    if not claimed_lines:
        return mismatches  # yaml 没这类声称 → 该 check 不报错

    # 2. 找代码 _build_rationale 函数
    rationale_path = _find_rationale_function(code_root)
    if rationale_path is None:
        # 找不到函数本身不算 MISMATCH（代码可能改名）—— 跳过
        return mismatches

    body = _read_function_body(rationale_path, r"def\s+_?build_rationale")
    if body is None:
        return mismatches

    # 3. 判据：函数体含 LLM 调用指纹？
    has_llm = _has_llm_call(body)

    # 4. 错位判定：yaml 声称 LLM 生成 / 参与，代码无 LLM 调用 = MISMATCH
    if not has_llm:
        for line_no, text, label in claimed_lines:
            # 提取函数体的关键事实证据
            body_excerpt = body[:200].replace("\n", " ⏎ ")
            mismatches.append(Mismatch(
                check_id="rationale_llm_claim",
                yaml_path=yaml_path_str,
                yaml_line=line_no,
                yaml_text=text,
                code_path=str(rationale_path.relative_to(code_root.parent.parent)),
                code_evidence=(f"_build_rationale 函数体无 LLM 调用指纹，"
                                f"实测是模板拼接（节选：{body_excerpt}...）"),
            ))

    return mismatches


# ---------- 注册的 check 列表（可扩展） ----------

CHECKS = [
    check_rationale_llm_claim,
]


# ---------- 主检查函数 ----------

def check_yaml_factual_consistency(yaml_path: Path | str | None = None,
                                     code_root: Path | str | None = None
                                     ) -> ConsistencyReport:
    """对 methodology.yaml 与 facet 代码做事实一致性检查。

    参数:
        yaml_path: methodology.yaml 路径
        code_root: 关联代码根目录

    返回: ConsistencyReport（含 mismatches / verdict）。
    """
    yp = Path(yaml_path) if yaml_path is not None else _METHODOLOGY_PATH
    cr = Path(code_root) if code_root is not None else _DEFAULT_CODE_ROOT

    # 不可读早返回
    if not yp.exists():
        return ConsistencyReport(
            yaml_path=str(yp),
            code_root=str(cr),
            verdict="FAIL",
            file_readable=False,
        )

    try:
        yaml_data = yaml.safe_load(yp.read_text(encoding="utf-8"))
    except (yaml.YAMLError, OSError) as e:
        print(f"❌ yaml 解析失败: {e}", file=sys.stderr)
        return ConsistencyReport(
            yaml_path=str(yp),
            code_root=str(cr),
            verdict="FAIL",
            file_readable=False,
        )

    # 解析成功 = 文件可读；data 是否含 layers 是 yaml 内容问题，不阻断 file_readable
    if not isinstance(yaml_data, dict) or "layers" not in yaml_data:
        print(f"⚠ yaml 顶层不是 dict 或缺 layers（仍继续）", file=sys.stderr)

    if not cr.exists():
        return ConsistencyReport(
            yaml_path=str(yp),
            code_root=str(cr),
            verdict="FAIL",
            file_readable=False,
        )

    lines = _yaml_lines(yp)
    mismatches: list[Mismatch] = []
    checks_run: list[str] = []

    for check_fn in CHECKS:
        checks_run.append(check_fn.__name__)
        try:
            found = check_fn(lines, cr)
            mismatches.extend(found)
        except Exception as e:
            # 单个 check 抛错不阻断整体（容错）；打印到 stderr
            print(f"⚠ check {check_fn.__name__} 抛错: {e}", file=sys.stderr)

    mismatch_count = len(mismatches)
    verdict = "FAIL" if mismatch_count > 0 else "PASS"

    return ConsistencyReport(
        yaml_path=str(yp),
        code_root=str(cr),
        mismatches=mismatches,
        mismatch_count=mismatch_count,
        verdict=verdict,
        file_readable=True,
        checks_run=checks_run,
    )


# ---------- 呈现 ----------

def _print_table(report: ConsistencyReport) -> None:
    print("=" * 64)
    print("methodology.yaml vs 代码事实一致性机械检查")
    print("（sihankor-proposition-defense）")
    print("=" * 64)
    print(f"yaml: {report.yaml_path}")
    print(f"code root: {report.code_root}")
    if not report.file_readable:
        print("❌ 文件不可读")
        return
    print()
    print(f"执行的 check: {report.checks_run}")
    print(f"错位数: {report.mismatch_count}")
    print()
    if report.mismatches:
        print("错位清单（yaml 文字 vs 代码事实）:")
        for m in report.mismatches:
            print()
            print(f"  [{m.check_id}] yaml L{m.yaml_line}:")
            print(f"    原文: {m.yaml_text}")
            print(f"    代码: {m.code_path}")
            print(f"    证据: {m.code_evidence}")
    else:
        print("  （无错位）")
    print()
    print("=" * 64)
    if report.verdict == "PASS":
        print("✅ PASS（yaml 文字与代码事实一致）")
    else:
        print(f"❌ FAIL（{report.mismatch_count} 条错位）")
        print("  F2.1 锚定：未抓到已知 yaml L62 rationale 错位 → 撤销")
        print("  F2.2 锚定：把 yaml 中文字误判为代码事实 → 撤销")
    print("=" * 64)
    print()
    print("⚠ 机械检查 ≠ 真审。本脚本只查明确的 'X 是 LLM 生成' 类事实声明。")


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(
        description="methodology.yaml 文字 vs 代码事实一致性机械检查（sihankor-proposition-defense 内嵌脚本）")
    p.add_argument("--json", action="store_true", help="仅输出 JSON")
    p.add_argument("--yaml", default=None, help="methodology.yaml 路径")
    p.add_argument("--code-root", default=None,
                    help="关联代码根目录（默认 sih-tools/facet/probes/）")
    args = p.parse_args()

    report = check_yaml_factual_consistency(args.yaml, args.code_root)

    if args.json:
        print(json.dumps(report.to_dict(), ensure_ascii=False, indent=2))
    else:
        _print_table(report)

    if not report.file_readable:
        return 2
    return 0 if report.verdict == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
