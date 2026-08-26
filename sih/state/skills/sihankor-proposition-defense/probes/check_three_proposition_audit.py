"""三命题层次机械漏层检查（sihankor-proposition-defense 内嵌脚本）。

方法学真源 = methodology.yaml，本脚本只读取不内嵌。
设计原则：
- **方法学真源唯一**：锚点表 / 必查问题 / 典型错位模式都从 yaml 读
- **改一处生效一处**：改 methodology.yaml，SKILL.md 渲染 + 本脚本都同步
- **机械优先**：锚点匹配可机械执行，避免 LLM 漏报
- **不替代 LLM**：查"是否含某锚点" ≠ "是否真审过"，LLM 仍须真审
- **不预判内容**：缺什么锚点列什么锚点，不替 LLM 给"应该审什么"

退出码：
- 0 = 三层都覆盖（机械层未漏）
- 1 = 漏层（须 LLM 补审）
- 2 = 文件不可读
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

# 方法学真源 = 同目录 methodology.yaml
_THIS_DIR = Path(__file__).resolve().parent
_METHODOLOGY_PATH = _THIS_DIR.parent / "methodology.yaml"


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
    methodology_version: int = 0
    layers: list[LayerCoverage] = field(default_factory=list)
    all_covered: bool = False
    missing_layers: list[str] = field(default_factory=list)
    strict_mode: bool = False
    file_readable: bool = True

    def to_dict(self) -> dict:
        return {
            "file_path": self.file_path,
            "methodology_version": self.methodology_version,
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


# ---------- 方法学加载（真源 = yaml） ----------

def _load_methodology(path: Path | None = None) -> tuple[int, dict[str, dict[str, list[str]]]]:
    """从 methodology.yaml 加载方法学。

    返回 (version, {layer_name: {lang: [patterns]}})
    异常 = yaml 不存在 / 解析失败 / 缺关键字段。
    """
    p = path or _METHODOLOGY_PATH
    if not p.exists():
        raise FileNotFoundError(f"方法学真源不存在: {p}")
    data = yaml.safe_load(p.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValueError("methodology.yaml 顶层必须是 dict")
    version = data.get("version", 0)
    layers = data.get("layers", [])
    if not isinstance(layers, list) or not layers:
        raise ValueError("methodology.yaml 缺 layers 列表")

    anchors: dict[str, dict[str, list[str]]] = {}
    for layer in layers:
        name = layer.get("name_zh") or layer.get("id")
        if not name:
            raise ValueError(f"methodology.yaml 层缺 name_zh: {layer}")
        layer_anchors = layer.get("anchors", {})
        if not layer_anchors:
            raise ValueError(f"methodology.yaml 层缺 anchors: {name}")
        anchors[name] = layer_anchors
    return version, anchors


# ---------- 核心检查 ----------

def _scan_layer(text: str, layer_name: str, anchors: dict[str, list[str]]) -> LayerCoverage:
    """扫一层命题：每条锚点 regex 至少一次命中 = 覆盖。"""
    matched: list[str] = []
    unmatched: list[str] = []
    for lang_key, patterns in anchors.items():
        for pat in patterns:
            try:
                if re.search(pat, text, re.IGNORECASE | re.MULTILINE):
                    matched.append(f"{lang_key}:{pat}")
                else:
                    unmatched.append(f"{lang_key}:{pat}")
            except re.error as e:
                unmatched.append(f"{lang_key}:{pat} [INVALID: {e}]")
    total = len(matched) + len(unmatched)
    coverage_ratio = len(matched) / total if total else 0.0
    covered = len(matched) >= 1
    return LayerCoverage(
        name=layer_name,
        matched_keywords=matched,
        unmatched_keywords=unmatched,
        coverage_ratio=coverage_ratio,
        covered=covered,
    )


def audit_file(file_path: Path,
                methodology_path: Path | None = None,
                strict: bool = False) -> AuditReport:
    """对单个报告文件做三命题层次漏层检查。

    参数:
        file_path: 报告文件路径
        methodology_path: 方法学真源路径（默认 methodology.yaml）
        strict: 严格模式标记
    """
    file_path = Path(file_path)
    if not file_path.exists():
        return AuditReport(
            file_path=str(file_path),
            file_readable=False,
            missing_layers=[],
            strict_mode=strict,
        )

    try:
        version, anchors = _load_methodology(methodology_path)
    except (FileNotFoundError, ValueError) as e:
        print(f"❌ 方法学加载失败: {e}", file=sys.stderr)
        return AuditReport(
            file_path=str(file_path),
            file_readable=False,
            missing_layers=[],
            strict_mode=strict,
        )

    text = file_path.read_text(encoding="utf-8")
    layers = [
        _scan_layer(text, name, layer_anchors)
        for name, layer_anchors in anchors.items()
    ]

    all_covered = all(l.covered for l in layers)
    missing = [l.name for l in layers if not l.covered]

    return AuditReport(
        file_path=str(file_path),
        methodology_version=version,
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
    print(f"方法学版本: v{report.methodology_version}")
    if not report.file_readable:
        print("❌ 文件或方法学不可读")
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
    print("⚠ 机械检查 ≠ 真审。LLM 仍须按 methodology.yaml 三层方法学真审。")
    print("=" * 64)


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(
        description="三命题层次漏层检查（sihankor-proposition-defense 内嵌脚本）")
    p.add_argument("file", help="报告 / 命题文件路径")
    p.add_argument("--json", action="store_true", help="仅输出 JSON")
    p.add_argument("--strict", action="store_true", help="严格模式标记")
    p.add_argument("--methodology", default=None,
                    help="方法学真源路径（默认 methodology.yaml）")
    args = p.parse_args()

    methodology_path = Path(args.methodology) if args.methodology else None
    report = audit_file(Path(args.file), methodology_path=methodology_path,
                          strict=args.strict)

    if args.json:
        print(json.dumps(report.to_dict(), ensure_ascii=False, indent=2))
    else:
        _print_table(report)

    if not report.file_readable:
        return 2
    return 0 if report.all_covered else 1


if __name__ == "__main__":
    raise SystemExit(main())
