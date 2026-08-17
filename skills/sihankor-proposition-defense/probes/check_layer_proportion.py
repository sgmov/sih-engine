"""N+C 各 layer 贡献占比机械检查（sihankor-proposition-defense 内嵌脚本）。

权重真源 = sih-tools/facet/probes/contribution_metric.py LAYER_WEIGHTS，本脚本只引用不内嵌。
设计原则：
- **权重真源唯一**：LAYER_WEIGHTS 从 contribution_metric 直接 import，facet 改权重本脚本自动跟随
- **纯读 trail**（无副作用、无 LLM 调用）
- **可机械校验**（JSON + 退出码，下游 verifier 可断言）
- **预登记阈值**（默认 0.60，CLI 可覆盖；F1.1 锚定：实测 ≥ 60% = 撤销）
- **不写建议**（只输出事实 + F 判定，不给"应该调阈值"）

退出码：
- 0 = flywheel_run 占比 ≤ 阈值（balanced）
- 1 = flywheel_run 占比 > 阈值（生成器层过度占比，FAIL）
- 2 = 文件不可读（trail root 缺 / yaml 缺）
"""
from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, field
from pathlib import Path

# 权重真源 = facet 仓 contribution_metric。本脚本跨仓引用之。
# 路径：当前文件 = sih-engine/skills/sihankor-proposition-defense/probes/check_layer_proportion.py
# 上溯 5 层 = 项目根 /Users/moc/workspaces/SiHankor/，再下到 sih-tools/facet/probes/
_FACET_PROBES_DIR = Path(__file__).resolve().parent.parent.parent.parent.parent / "sih-tools" / "facet" / "probes"

# 跨仓 import：先把 facet probes 目录加 sys.path
if _FACET_PROBES_DIR.exists() and str(_FACET_PROBES_DIR) not in sys.path:
    sys.path.insert(0, str(_FACET_PROBES_DIR))

try:
    from contribution_metric import LAYER_WEIGHTS  # type: ignore[import-not-found]  # noqa: E402
    _WEIGHT_SOURCE = "contribution_metric.LAYER_WEIGHTS"
except (ImportError, ModuleNotFoundError):
    # 跨仓路径不可用时降级到内嵌权重——但脚本会标 warn（仍能跑）
    LAYER_WEIGHTS = {
        "flywheel_run": 0.5,
        "gate_assessment": 1.0,
        "layer2_signoff": 2.0,
        "program_signoff": 2.0,
        "knife_edge_risk": 2.0,
        "supersession": 3.0,
        "escalation_review": 3.0,
        "escalation_drill_marker": 3.0,
    }
    print("⚠ 跨仓 import 失败，使用内嵌权重（应与 contribution_metric.py 一致）", file=sys.stderr)


# ---------- 默认 trail root（与 contribution_metric.py _TRAIL_ROOT 一致） ----------

_DEFAULT_TRAIL_ROOT = _FACET_PROBES_DIR.parent.parent / "proposition" / "DES"

# 阈值预登记（**不事后调**——F1.1 锚定"实测 ≥ 60% = 撤销"）
DEFAULT_THRESHOLD = 0.60


# ---------- 数据结构 ----------

@dataclass
class LayerContribution:
    """单 layer 贡献明细。"""
    type: str
    count: int
    weight: float
    contribution: float
    ratio: float  # 占总贡献度的比例


@dataclass
class LayerProportionReport:
    """N+C layer 占比检查报告。"""
    file_path: str                   # 检查的 trail root（事实陈述，不变）
    n_guidances: int = 0
    n_events: int = 0
    contribution: float = 0.0
    layer_contributions: list[LayerContribution] = field(default_factory=list)
    flywheel_run_ratio: float = 0.0
    threshold: float = DEFAULT_THRESHOLD
    balanced: bool = False
    verdict: str = "PASS"            # PASS / FAIL
    file_readable: bool = True
    weight_source: str = ""  # 权重来源事实（构造时显式传入）

    def to_dict(self) -> dict:
        return {
            "file_path": self.file_path,
            "n_guidances": self.n_guidances,
            "n_events": self.n_events,
            "contribution": round(self.contribution, 2),
            "layer_contributions": {
                lc.type: {
                    "count": lc.count,
                    "weight": lc.weight,
                    "contribution": round(lc.contribution, 2),
                    "ratio": round(lc.ratio, 4),
                }
                for lc in self.layer_contributions
            },
            "flywheel_run_ratio": round(self.flywheel_run_ratio, 4),
            "threshold": self.threshold,
            "balanced": self.balanced,
            "verdict": self.verdict,
            "weight_source": self.weight_source,
        }


# ---------- 核心扫描 ----------

def _scan_trail(trail_root: Path) -> tuple[int, dict[str, int]]:
    """扫所有 flywheel-trail.jsonl，统计 event_type（trail_type）。

    返回 (n_guidances, event_type_counts)。
    """
    n_guidances = 0
    counts: dict[str, int] = {}

    for trail_file in trail_root.rglob("flywheel-trail.jsonl"):
        n_guidances += 1
        for line in trail_file.read_text(encoding="utf-8").splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                rec = json.loads(line)
            except json.JSONDecodeError:
                continue
            t = rec.get("trail_type", "?")
            counts[t] = counts.get(t, 0) + 1

    return n_guidances, counts


def check_layer_proportion(trail_root: Path | str | None = None,
                            threshold: float = DEFAULT_THRESHOLD) -> LayerProportionReport:
    """对 trail 数据做 layer 占比检查（纯读，无副作用）。

    参数:
        trail_root: trail 根目录，默认 sih-tools/proposition/DES/
        threshold: flywheel_run 占比上限（超过 = FAIL），默认 0.60

    返回: LayerProportionReport（含 flywheel_run_ratio / balanced / verdict）。
    """
    root = Path(trail_root) if trail_root is not None else _DEFAULT_TRAIL_ROOT

    if not root.exists():
        return LayerProportionReport(
            file_path=str(root),
            threshold=threshold,
            verdict="FAIL",
            file_readable=False,
            weight_source=_WEIGHT_SOURCE,
        )

    n_guidances, counts = _scan_trail(root)

    # 总贡献度 C = ∑ w(t) × n(t)，未知 type 权重 0
    n_events = sum(counts.values())
    total_contribution = sum(LAYER_WEIGHTS.get(t, 0.0) * n
                              for t, n in counts.items())

    # 各 layer 占比
    layer_contributions: list[LayerContribution] = []
    for t, n in sorted(counts.items(), key=lambda x: -x[1]):
        w = LAYER_WEIGHTS.get(t, 0.0)
        c = w * n
        ratio = (c / total_contribution) if total_contribution > 0 else 0.0
        layer_contributions.append(LayerContribution(
            type=t, count=n, weight=w, contribution=c, ratio=ratio,
        ))

    # flywheel_run 占比
    flywheel_count = counts.get("flywheel_run", 0)
    flywheel_contrib = LAYER_WEIGHTS.get("flywheel_run", 0.0) * flywheel_count
    flywheel_ratio = (flywheel_contrib / total_contribution) if total_contribution > 0 else 0.0

    balanced = flywheel_ratio <= threshold
    verdict = "PASS" if balanced else "FAIL"

    return LayerProportionReport(
        file_path=str(root),
        n_guidances=n_guidances,
        n_events=n_events,
        contribution=total_contribution,
        layer_contributions=layer_contributions,
        flywheel_run_ratio=flywheel_ratio,
        threshold=threshold,
        balanced=balanced,
        verdict=verdict,
        file_readable=True,
        weight_source=_WEIGHT_SOURCE,
    )


# ---------- 呈现 ----------

def _print_table(report: LayerProportionReport) -> None:
    print("=" * 64)
    print("N+C layer 占比机械检查（sihankor-proposition-defense）")
    print("=" * 64)
    print(f"trail root: {report.file_path}")
    print(f"权重真源: {report.weight_source}")
    if not report.file_readable:
        print("❌ 文件不可读")
        return
    print()
    print(f"guidances with trail: {report.n_guidances}")
    print(f"样本数 N = {report.n_events}")
    print(f"贡献度 C = {report.contribution:.1f}")
    print()
    print("各 layer 贡献度（按 contribution 降序）:")
    print(f"  {'type':<28} {'count':>8} {'weight':>8} {'贡献':>10} {'占比':>8}")
    print(f"  {'-'*28} {'-'*8} {'-'*8} {'-'*10} {'-'*8}")
    for lc in report.layer_contributions:
        print(f"  {lc.type:<28} {lc.count:>8} {lc.weight:>8.1f} "
              f"{lc.contribution:>10.1f} {lc.ratio:>8.2%}")
    print()
    print(f"flywheel_run 占比 = {report.flywheel_run_ratio:.2%}"
          f"（阈值 ≤ {report.threshold:.0%}）")
    print()
    print("=" * 64)
    if report.balanced:
        print(f"✅ PASS（flywheel_run 占比 {report.flywheel_run_ratio:.2%} ≤ "
              f"阈值 {report.threshold:.0%}，生成器层未过度占比）")
    else:
        print(f"❌ FAIL（flywheel_run 占比 {report.flywheel_run_ratio:.2%} > "
              f"阈值 {report.threshold:.0%}，生成器层过度占比）")
        print("  F1.1 锚定：实测 flywheel_run 占比 ≥ 60% → 撤销（生成器层过度占比）")
    print("=" * 64)
    print()
    print("⚠ 机械检查 ≠ 真审。本脚本只查占比失衡，不查治理展开质量。")


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(
        description="N+C 各 layer 贡献占比机械检查（sihankor-proposition-defense 内嵌脚本）")
    p.add_argument("--json", action="store_true", help="仅输出 JSON")
    p.add_argument("--trail-root", default=None,
                    help="trail 根目录（默认 sih-tools/proposition/DES/）")
    p.add_argument("--threshold", type=float, default=DEFAULT_THRESHOLD,
                    help=f"flywheel_run 占比上限（默认 {DEFAULT_THRESHOLD}，F1.1 锚定）")
    args = p.parse_args()

    report = check_layer_proportion(args.trail_root, args.threshold)

    if args.json:
        print(json.dumps(report.to_dict(), ensure_ascii=False, indent=2))
    else:
        _print_table(report)

    if not report.file_readable:
        return 2
    return 0 if report.balanced else 1


if __name__ == "__main__":
    raise SystemExit(main())
