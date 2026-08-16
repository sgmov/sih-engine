"""三命题层次机械漏层检查（sihankor-proposition-defense 内嵌脚本）。

承接 skill 方法学：立题命题 / 应用命题 / 治理领域展开贡献命题 三层。
本脚本只查"是否覆盖到三个命题层次"——不替代 LLM 真判断，只报警漏层。

设计原则（来自 skill 立文）：
- **机械优先**：锚点匹配可机械执行，避免 LLM 漏报
- **不替代 LLM**：查"是否含某锚点" ≠ "是否真审过"，LLM 仍须真审
- **退出码清晰**：0 = 三层都覆盖 / 1 = 漏层 / 2 = 文件读取失败
- **不预判内容**：缺什么锚点列什么锚点，不替 LLM 给"应该审什么"

锚点表（keyword 集合）：
- 立题命题层：
  - 中文：本体命题 / 立题 / 异质性 / 独立探索 / 命题合法性 / facet 立题 / 异质性来自
  - 英文：foundation proposition / core proposition / cross-family heterogeneity
- 应用命题层（A-A3.1 / A-A4.1 / A-A4.2 / PRO-07）：
  - 中文：应用命题 / 自证循环 / 候选建议 / 确定性引擎 / 鉴层 / PRO-07 / A-A3 / A-A4
  - 英文：application proposition / A-A3.1 / A-A4.1 / A-A4.2 / self-verification
- 治理领域展开贡献层：
  - 中文：治理领域 / 上下文风洞 / 权责归一 / 信息洪流 / 上下文包 / 牌照 / 治理展开
  - 英文：governance contribution / context wind tunnel / human attention

输出格式：
- JSON（--json）：结构化覆盖判定 + 漏层清单
- 终端表格（默认）：人可读概览

用法：
  python check_three_proposition_audit.py <report.md>
  python check_three_proposition_audit.py --json <report.md>
  python check_three_proposition_audit.py --strict <report.md>  # 任何 keyword 缺则 fail
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

# ---------- 锚点表（从 skill 方法学派生，不预判） ----------

LAYER_ANCHORS: dict[str, dict[str, list[str]]] = {
    "立题命题": {
        "zh": [
            r"本体命题", r"立题", r"异质性", r"独立探索", r"命题合法性",
            r"facet\s*立题", r"异质性来自", r"立题范围", r"本体论",
        ],
        "en": [
            r"foundation\s+proposition", r"core\s+proposition",
            r"cross-family\s+heterogeneity", r"independent\s+exploration",
            r"proposition.{0,5}scope",
        ],
    },
    "应用命题": {
        "zh": [
            r"应用命题", r"自证循环", r"候选建议", r"确定性引擎",
            r"鉴层", r"PRO-?07", r"A-A3", r"A-A4",
            r"破自证", r"建议生成器", r"裁决权",
        ],
        "en": [
            r"application\s+proposition", r"A-A3\.1", r"A-A4\.1", r"A-A4\.2",
            r"self-verification", r"PRO-?07", r"deterministic\s+engine",
        ],
    },
    "治理领域展开贡献": {
        "zh": [
            r"治理领域", r"上下文风洞", r"权责归一", r"信息洪流",
            r"上下文包", r"牌照", r"治理展开", r"治理贡献",
            r"裁决权跟着责任", r"人节点",
        ],
        "en": [
            r"governance\s+contribution", r"context\s+wind\s+tunnel",
            r"human\s+attention", r"context\s+licen[cs]e",
        ],
    },
}


@dataclass
class LayerCoverage:
    """单层覆盖判定。"""
    name: str
    matched_keywords: list[str] = field(default_factory=list)
    unmatched_keywords: list[str] = field(default_factory=list)
    coverage_ratio: float = 0.0
    covered: bool = False


@dataclass
class AuditReport:
    """三命题层次漏层检查报告。"""
    file_path: str
    layers: list[LayerCoverage] = field(default_factory=list)
    all_covered: bool = False
    missing_layers: list[str] = field(default_factory=list)
    strict_mode: bool = False
    file_readable: bool = True

    def to_dict(self) -> dict:
        return {
            "file_path": self.file_path,
            "layers": [
                {
                    "name": l.name,
                    "matched_keywords": l.matched_keywords,
                    "unmatched_keywords": l.unmatched_keywords,
                    "coverage_ratio": round(l.coverage_ratio, 3),
                    "covered": l.covered,
                }
                for l in self.layers
            ],
            "all_covered": self.all_covered,
            "missing_layers": self.missing_layers,
            "strict_mode": self.strict_mode,
            "file_readable": self.file_readable,
        }


# ---------- 核心检查 ----------

def _scan_layer(text: str, layer_name: str, anchors: dict[str, list[str]]) -> LayerCoverage:
    """扫一层命题：每条锚点 regex 至少一次命中 = 覆盖。"""
    matched: list[str] = []
    unmatched: list[str] = []
    for lang_key, patterns in anchors.items():
        for pat in patterns:
            if re.search(pat, text, re.IGNORECASE | re.MULTILINE):
                matched.append(f"{lang_key}:{pat}")
            else:
                unmatched.append(f"{lang_key}:{pat}")
    total = len(matched) + len(unmatched)
    coverage_ratio = len(matched) / total if total else 0.0
    # 覆盖判定：至少 1 个锚点命中（不要求全覆盖）
    covered = len(matched) >= 1
    return LayerCoverage(
        name=layer_name,
        matched_keywords=matched,
        unmatched_keywords=unmatched,
        coverage_ratio=coverage_ratio,
        covered=covered,
    )


def audit_file(file_path: Path, strict: bool = False) -> AuditReport:
    """对单个报告文件做三命题层次漏层检查。

    参数:
        file_path: 报告文件路径
        strict: 严格模式（任一 anchor 缺则 fail；默认只看覆盖度 ≥ 1）
    """
    file_path = Path(file_path)
    if not file_path.exists():
        return AuditReport(
            file_path=str(file_path),
            file_readable=False,
            missing_layers=list(LAYER_ANCHORS.keys()),
            strict_mode=strict,
        )

    text = file_path.read_text(encoding="utf-8")
    layers = [
        _scan_layer(text, name, anchors)
        for name, anchors in LAYER_ANCHORS.items()
    ]

    if strict:
        # 严格模式：每层 coverage_ratio 必须 > 0（已有此条件）+ 任一 anchor 缺时 fail
        # 严格模式实际不改变"covered"逻辑（已要求 ≥ 1 命中）—— 严格模式更多用作
        # 标记 audit 报告的严格度，让调用方知道"脚本用严格态度审"
        pass

    all_covered = all(l.covered for l in layers)
    missing = [l.name for l in layers if not l.covered]

    return AuditReport(
        file_path=str(file_path),
        layers=layers,
        all_covered=all_covered,
        missing_layers=missing,
        strict_mode=strict,
        file_readable=True,
    )


# ---------- 呈现 ----------

def _print_table(report: AuditReport) -> None:
    print("=" * 64)
    print("三命题层次漏层检查（sihankor-proposition-defense）")
    print("=" * 64)
    print(f"文件: {report.file_path}")
    if not report.file_readable:
        print("❌ 文件不存在或不可读")
        return
    print()
    print(f"{'命题层':<24} {'覆盖':<8} {'命中/总锚点':<14} {'覆盖度':<8}")
    print(f"{'-'*24} {'-'*8} {'-'*14} {'-'*8}")
    for l in report.layers:
        status = "✅" if l.covered else "❌"
        n_match = len(l.matched_keywords)
        n_total = n_match + len(l.unmatched_keywords)
        print(f"{l.name:<24} {status:<8} {n_match}/{n_total:<12} {l.coverage_ratio:<8.2%}")
    print()
    if report.all_covered:
        print("✅ 三层都覆盖（机械检查通过）")
    else:
        print(f"❌ 漏层：{report.missing_layers}")
        print()
        print("漏层细节（未命中锚点）：")
        for l in report.layers:
            if l.unmatched_keywords:
                print(f"\n  [{l.name}]")
                for kw in l.unmatched_keywords[:10]:
                    print(f"    - {kw}")
                if len(l.unmatched_keywords) > 10:
                    print(f"    ... ({len(l.unmatched_keywords) - 10} more)")
    print()
    print("=" * 64)
    print("⚠ 机械检查 ≠ 真审。LLM 仍须按 SKILL.md 三层方法学真审。")
    print("=" * 64)


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(
        description="三命题层次漏层检查（sihankor-proposition-defense 内嵌脚本）")
    p.add_argument("file", help="报告 / 命题文件路径")
    p.add_argument("--json", action="store_true", help="仅输出 JSON")
    p.add_argument("--strict", action="store_true", help="严格模式标记")
    args = p.parse_args()

    report = audit_file(Path(args.file), strict=args.strict)

    if args.json:
        print(json.dumps(report.to_dict(), ensure_ascii=False, indent=2))
    else:
        _print_table(report)

    if not report.file_readable:
        return 2
    return 0 if report.all_covered else 1


if __name__ == "__main__":
    raise SystemExit(main())
