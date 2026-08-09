#!/usr/bin/env python3
"""Phase 0 v3 aggregation: fixed checklist, judgment-only mode.

Since checklists are pre-extracted and identical across sessions,
all 10 sessions judge the exact same targets. Alignment is trivial.

Computes per-document per-check-item:
- per-target agreement (all 10 sessions judge the same target)
- judgment distribution per target
- disagreement targets

Usage:
    python3 phase0-v3-aggregate.py <phase0-v3-dir> <summary.json>
"""

import json
import sys
from collections import defaultdict, Counter
from pathlib import Path

CHECK_ITEMS = [
    "承接关系有效性",
    "可证伪条件存在性",
    "认识论标签合法性",
    "术语一致性",
]
DOCS = [
    "DES-058",
    "DES-017",
    "DEC-008",
    "DEC-004",
    "sprint-plan",
    "INTENT-DRIFT",
]


def load_records(path):
    records = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            records.append(json.loads(line))
    return records


def aggregate_doc(doc_dir):
    """Aggregate one document's 10 seq files."""
    # Load each seq file
    session_data = {}  # seq -> {check_item: {target: judgment}}
    for seq in range(1, 11):
        seq_file = doc_dir / f"{doc_dir.name}-seq{seq}.jsonl"
        if not seq_file.exists():
            continue
        recs = load_records(seq_file)
        by_ci = defaultdict(dict)
        for r in recs:
            ci = r.get("check_item")
            tgt = r.get("target", "(empty)")
            j = r.get("judgment")
            by_ci[ci][tgt] = j
        session_data[seq] = by_ci

    if not session_data:
        return None

    result = {}
    for ci in CHECK_ITEMS:
        # Align by target: {target: {seq: judgment}}
        target_judgments = defaultdict(dict)
        for seq, by_ci in session_data.items():
            for tgt, j in by_ci.get(ci, {}).items():
                target_judgments[tgt][seq] = j

        total_targets = len(target_judgments)
        all10 = [t for t, sjs in target_judgments.items() if len(sjs) == 10]
        disagree = [
            t for t, sjs in target_judgments.items()
            if len(sjs) >= 2 and len(set(sjs.values())) > 1
        ]

        per_target = []
        agreements_10 = []
        for tgt, sjs in sorted(target_judgments.items()):
            judgments = list(sjs.values())
            counts = Counter(judgments)
            top_val, top_cnt = counts.most_common(1)[0]
            agree = round(top_cnt / len(sjs), 3)
            per_target.append({
                "target": tgt,
                "n_sessions": len(sjs),
                "judgments": dict(counts),
                "agreement": agree,
                "fully_agree": len(counts) == 1,
            })
            if len(sjs) == 10:
                agreements_10.append(agree)

        avg_agree = round(sum(agreements_10) / len(agreements_10), 3) if agreements_10 else None
        min_agree = round(min(agreements_10), 3) if agreements_10 else None

        result[ci] = {
            "sessions_present": len(session_data),
            "total_targets": total_targets,
            "targets_all10": len(all10),
            "targets_disagree": len(disagree),
            "avg_agreement": avg_agree,
            "min_agreement": min_agree,
            "per_target": sorted(per_target, key=lambda x: x["agreement"]),
        }
    return result


def main():
    if len(sys.argv) != 3:
        print("Usage: phase0-v3-aggregate.py <phase0-v3-dir> <summary.json>", file=sys.stderr)
        sys.exit(2)
    base_dir = Path(sys.argv[1])
    summary_path = sys.argv[2]

    all_results = {}
    report = []

    report.append("# Phase 0 v3 统计聚合（固定清单，仅判断）\n")
    report.append(f"输入目录 {base_dir}")
    report.append(f"明细 JSON {summary_path}\n")

    for doc in DOCS:
        doc_dir = base_dir / doc
        if not doc_dir.exists():
            report.append(f"## {doc}\n目录不存在\n")
            continue
        agg = aggregate_doc(doc_dir)
        if agg is None:
            report.append(f"## {doc}\n无数据\n")
            continue
        all_results[doc] = agg

        report.append(f"## {doc}\n")
        report.append("| 检查项 | 对象数 | 全10对齐 | 分歧对象 | 平均一致率 | 最低一致率 |")
        report.append("|---|---|---|---|---|---|")
        for ci in CHECK_ITEMS:
            d = agg.get(ci, {})
            tt = d.get("total_targets", 0)
            a10 = d.get("targets_all10", 0)
            dis = d.get("targets_disagree", 0)
            aa = d.get("avg_agreement")
            ma = d.get("min_agreement")
            report.append(
                f"| {ci} | {tt} | {a10} | {dis} | {aa if aa else 'N/A'} | {ma if ma else 'N/A'} |"
            )
        report.append("")

        for ci in CHECK_ITEMS:
            d = agg.get(ci, {})
            disagree_list = [t for t in d.get("per_target", []) if not t["fully_agree"] and t["n_sessions"] >= 2]
            if disagree_list:
                report.append(f"### {doc} {ci} 分歧明细\n")
                report.append("| 对象 | 各判断票数 | 一致率 |")
                report.append("|---|---|---|")
                for t in disagree_list:
                    tgt_short = t["target"][:60]
                    votes = " / ".join(f"{k}×{v}" for k, v in sorted(t["judgments"].items(), key=lambda x: -x[1]))
                    report.append(f"| {tgt_short} | {votes} | {t['agreement']} |")
                report.append("")

    with open(summary_path, "w", encoding="utf-8") as out:
        json.dump(all_results, out, ensure_ascii=False, indent=2)

    print("\n".join(report))
    print("---")
    print("聚合完成。")


if __name__ == "__main__":
    main()
