#!/usr/bin/env python3
"""Phase 1 Round 1 aggregation: N=4 majority vote + tie detection.

For each doc, 4 sessions per check item. Majority vote:
- 4/0 or 3/1: auto-decided
- 2/2: tie, needs tie-break (5th session) or human

Computes per-type and overall:
- auto-decide rate
- tie rate
- agreement distribution
- human intervention estimate
"""

import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

CHECK_ITEMS = [
    "承接关系有效性",
    "可证伪条件存在性",
    "认识论标签合法性",
    "术语一致性",
]


def load_doc(doc_dir):
    """Load all seq files for a doc. Returns {check_item: {target: {seq: judgment}}}."""
    data = defaultdict(lambda: defaultdict(dict))
    for f in sorted(doc_dir.glob("*.jsonl")):
        content = f.read_text(encoding="utf-8").strip()
        if not content:
            continue
        for line in content.split("\n"):
            line = line.strip()
            if not line:
                continue
            r = json.loads(line)
            ci = r.get("check_item")
            tgt = r.get("target", "(empty)")
            seq = r.get("sample_seq")
            j = r.get("judgment")
            data[ci][tgt][seq] = j
    return data


def analyze_doc(doc_dir):
    """Analyze one doc. Returns per-check-item stats."""
    data = load_doc(doc_dir)
    result = {}
    for ci in CHECK_ITEMS:
        targets = data.get(ci, {})
        total = 0
        auto = 0  # 3/1 or 4/0
        tie = 0   # 2/2
        unanimous = 0  # 4/0
        low_confidence = 0  # 3/1
        disagree_targets = []

        for tgt, seq_j in targets.items():
            if len(seq_j) < 2:
                continue
            total += 1
            votes = list(seq_j.values())
            counts = Counter(votes)
            top_val, top_cnt = counts.most_common(1)[0]

            if top_cnt == 4:
                unanimous += 1
                auto += 1
            elif top_cnt == 3:
                low_confidence += 1
                auto += 1
            elif top_cnt == 2 and len(counts) == 2:
                tie += 1
            else:
                # shouldn't happen with 4 sessions
                auto += 1

            if len(counts) > 1:
                disagree_targets.append({
                    "target": tgt[:60],
                    "votes": dict(counts),
                })

        result[ci] = {
            "total": total,
            "auto": auto,
            "unanimous": unanimous,
            "low_confidence": low_confidence,
            "tie": tie,
            "disagree_targets": disagree_targets,
        }
    return result


def main():
    base = Path(sys.argv[1])
    types = ["governance", "research", "draft", "spec", "decision", "knowledge"]
    if len(sys.argv) > 2:
        types = sys.argv[2].split(",")

    report = []
    report.append("# Phase 1 Round 1 聚合分析（N=4 多数投票）\n")
    report.append(f"分析范围: {', '.join(types)}\n")

    # Overall stats
    overall = {
        ci: {"total": 0, "auto": 0, "unanimous": 0, "low_confidence": 0, "tie": 0, "disagree": 0}
        for ci in CHECK_ITEMS
    }

    for t in types:
        t_dir = base / t
        if not t_dir.exists():
            continue

        docs = sorted([d for d in t_dir.iterdir() if d.is_dir()])
        type_stats = {
            ci: {"total": 0, "auto": 0, "unanimous": 0, "low_confidence": 0, "tie": 0, "disagree": 0}
            for ci in CHECK_ITEMS
        }
        type_ties = []

        for doc_dir in docs:
            doc_result = analyze_doc(doc_dir)
            for ci in CHECK_ITEMS:
                d = doc_result.get(ci, {})
                for k in ["total", "auto", "unanimous", "low_confidence", "tie"]:
                    type_stats[ci][k] += d.get(k, 0)
                if d.get("disagree_targets"):
                    type_stats[ci]["disagree"] += len(d["disagree_targets"])
                    for dt in d["disagree_targets"]:
                        if dt["votes"].get(max(dt["votes"], key=dt["votes"].get), 0) == 2:  # tie
                            type_ties.append((doc_dir.name, ci, dt["target"], dt["votes"]))

        # Print type summary
        report.append(f"## {t}（{len(docs)} 份文档）\n")
        report.append("| 检查项 | 对象数 | 全一致4/0 | 低置信3/1 | 平局2/2 | 自动率 | 平局率 |")
        report.append("|---|---|---|---|---|---|---|")
        for ci in CHECK_ITEMS:
            s = type_stats[ci]
            if s["total"] == 0:
                report.append(f"| {ci} | 0 | — | — | — | — | — |")
                continue
            auto_rate = s["auto"] / s["total"] * 100
            tie_rate = s["tie"] / s["total"] * 100
            report.append(
                f"| {ci} | {s['total']} | {s['unanimous']} | {s['low_confidence']} | {s['tie']} | {auto_rate:.1f}% | {tie_rate:.1f}% |"
            )
            # Add to overall
            for k in ["total", "auto", "unanimous", "low_confidence", "tie", "disagree"]:
                overall[ci][k] += s[k]
        report.append("")

        # Print tie details
        if type_ties:
            report.append(f"### {t} 平局对象明细\n")
            report.append("| 文档 | 检查项 | 对象 | 票数 |")
            report.append("|---|---|---|---|")
            for doc_name, ci, tgt, votes in type_ties[:20]:  # limit
                vote_str = " / ".join(f"{k}×{v}" for k, v in sorted(votes.items(), key=lambda x: -x[1]))
                report.append(f"| {doc_name[:30]} | {ci} | {tgt[:40]} | {vote_str} |")
            if len(type_ties) > 20:
                report.append(f"... 共 {len(type_ties)} 个平局对象")
            report.append("")

    # Overall summary
    report.append("## 总体汇总\n")
    report.append("| 检查项 | 对象数 | 全一致4/0 | 低置信3/1 | 平局2/2 | 自动率 | 平局率 |")
    report.append("|---|---|---|---|---|---|---|")
    grand_total = 0
    grand_auto = 0
    grand_tie = 0
    for ci in CHECK_ITEMS:
        s = overall[ci]
        if s["total"] == 0:
            report.append(f"| {ci} | 0 | — | — | — | — | — |")
            continue
        auto_rate = s["auto"] / s["total"] * 100
        tie_rate = s["tie"] / s["total"] * 100
        report.append(
            f"| {ci} | {s['total']} | {s['unanimous']} | {s['low_confidence']} | {s['tie']} | {auto_rate:.1f}% | {tie_rate:.1f}% |"
        )
        grand_total += s["total"]
        grand_auto += s["auto"]
        grand_tie += s["tie"]

    report.append(f"| **合计** | **{grand_total}** | — | — | **{grand_tie}** | **{grand_auto/grand_total*100:.1f}%** | **{grand_tie/grand_total*100:.1f}%** |")
    report.append("")

    # SPEC-002 criteria check
    report.append("## SPEC-002 判据检查\n")
    auto_coverage = grand_auto / grand_total * 100
    human_rate = grand_tie / grand_total * 100
    report.append(f"自动判定覆盖率: {auto_coverage:.1f}%（SPEC-002 判据: >95% 通过）")
    report.append(f"人类介入率: {human_rate:.1f}%（SPEC-002 判据: <5% 通过）")
    report.append("")
    if auto_coverage > 95 and human_rate < 5:
        report.append("结论: 拖底策略生产可行")
    elif auto_coverage > 80 and human_rate < 15:
        report.append("结论: 部分可行，需按检查项分别评估")
    else:
        report.append("结论: 不可行")

    print("\n".join(report))
    print("\n---\n聚合完成。")


if __name__ == "__main__":
    main()
