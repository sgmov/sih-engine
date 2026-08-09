#!/usr/bin/env python3
"""Phase 0 v2 statistical aggregation: isolated session mode.

Reads per-document subdirectories under phase0-v2/, each containing
seq1.jsonl to seq10.jsonl. Aligns by (check_item, target) across the
10 isolated sessions per document.

Computes per-document per-check-item:
- extraction count variance (how many objects each session found)
- cross-session target alignment (how many targets judged by all 10)
- per-target agreement (majority ratio across sessions that judged it)
- disagreement objects (>=2 sessions, different judgments)

Outputs markdown report + JSON summary.

Usage:
    python3 phase0-v2-aggregate.py <phase0-v2-dir> <summary.json>
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
    """Aggregate one document's 10 seq files.

    Returns {check_item: {...}} with alignment data.
    """
    # Load each seq file
    session_data = {}  # seq -> {check_item -> [records]}
    for seq in range(1, 11):
        seq_file = doc_dir / f"{doc_dir.name}-seq{seq}.jsonl"
        if not seq_file.exists():
            continue
        recs = load_records(seq_file)
        by_ci = defaultdict(list)
        for r in recs:
            by_ci[r.get("check_item")].append(r)
        session_data[seq] = by_ci

    if not session_data:
        return None

    result = {}
    for ci in CHECK_ITEMS:
        # Align by target across sessions
        target_to_judgments = defaultdict(dict)  # target -> {seq: judgment}
        per_seq_counts = {}  # seq -> count
        for seq, by_ci in session_data.items():
            recs = by_ci.get(ci, [])
            per_seq_counts[seq] = len(recs)
            for r in recs:
                tgt = r.get("target", "(empty)")
                target_to_judgments[tgt][seq] = r.get("judgment")

        total_targets = len(target_to_judgments)
        all10 = [t for t, sjs in target_to_judgments.items() if len(sjs) == 10]
        disagree = [
            t for t, sjs in target_to_judgments.items()
            if len(sjs) >= 2 and len(set(sjs.values())) > 1
        ]

        # Per-target detail
        per_target = []
        for tgt, sjs in target_to_judgments.items():
            judgments = list(sjs.values())
            counts = Counter(judgments)
            top_val, top_cnt = counts.most_common(1)[0]
            per_target.append({
                "target": tgt,
                "n_sessions": len(sjs),
                "judgments": dict(counts),
                "agreement": round(top_cnt / len(sjs), 3),
                "fully_agree": len(counts) == 1,
            })

        # Stats for targets judged by all 10
        if all10:
            agreements = []
            for tgt in all10:
                sjs = target_to_judgments[tgt]
                counts = Counter(sjs.values())
                top_cnt = counts.most_common(1)[0][1]
                agreements.append(top_cnt / len(sjs))
            avg_agree = round(sum(agreements) / len(agreements), 3)
            min_agree = round(min(agreements), 3)
        else:
            avg_agree = None
            min_agree = None

        result[ci] = {
            "sessions_present": len(per_seq_counts),
            "per_seq_extraction_counts": dict(sorted(per_seq_counts.items())),
            "extraction_min": min(per_seq_counts.values()) if per_seq_counts else 0,
            "extraction_max": max(per_seq_counts.values()) if per_seq_counts else 0,
            "total_distinct_targets": total_targets,
            "targets_judged_by_all_10": len(all10),
            "targets_with_disagreement": len(disagree),
            "avg_agreement_all10": avg_agree,
            "min_agreement_all10": min_agree,
            "per_target": sorted(per_target, key=lambda x: x["agreement"]),
        }
    return result


def main():
    if len(sys.argv) != 3:
        print("Usage: phase0-v2-aggregate.py <phase0-v2-dir> <summary.json>", file=sys.stderr)
        sys.exit(2)
    base_dir = Path(sys.argv[1])
    summary_path = sys.argv[2]

    all_results = {}
    report_lines = []

    report_lines.append("# Phase 0 v2 统计聚合\n")
    report_lines.append(f"输入目录 {base_dir}")
    report_lines.append(f"明细 JSON {summary_path}\n")

    for doc in DOCS:
        doc_dir = base_dir / doc
        if not doc_dir.exists():
            report_lines.append(f"## {doc}\n目录不存在，跳过\n")
            continue
        agg = aggregate_doc(doc_dir)
        if agg is None:
            report_lines.append(f"## {doc}\n无数据\n")
            continue
        all_results[doc] = agg

        report_lines.append(f"## {doc}\n")
        report_lines.append("| 检查项 | session数 | 抽取min-max | 并集对象 | 全10对齐 | 分歧对象 | 全10平均一致率 |")
        report_lines.append("|---|---|---|---|---|---|---|")
        for ci in CHECK_ITEMS:
            d = agg.get(ci, {})
            sp = d.get("sessions_present", 0)
            emin = d.get("extraction_min", 0)
            emax = d.get("extraction_max", 0)
            tt = d.get("total_distinct_targets", 0)
            a10 = d.get("targets_judged_by_all_10", 0)
            dis = d.get("targets_with_disagreement", 0)
            aa = d.get("avg_agreement_all10")
            aa_s = f"{aa}" if aa is not None else "N/A"
            report_lines.append(
                f"| {ci} | {sp} | {emin}-{emax} | {tt} | {a10} | {dis} | {aa_s} |"
            )
        report_lines.append("")

        # Disagreement detail per check item
        for ci in CHECK_ITEMS:
            d = agg.get(ci, {})
            per_target = d.get("per_target", [])
            disagree_list = [t for t in per_target if not t["fully_agree"] and t["n_sessions"] >= 2]
            if disagree_list:
                report_lines.append(f"### {doc} {ci} 分歧明细\n")
                report_lines.append("| 对象 | 各判断票数 | 一致率 |")
                report_lines.append("|---|---|---|")
                for t in disagree_list:
                    tgt_short = t["target"][:50]
                    votes = " / ".join(f"{k}×{v}" for k, v in t["judgments"].items())
                    report_lines.append(f"| {tgt_short} | {votes} | {t['agreement']} |")
                report_lines.append("")

    # Write JSON summary
    with open(summary_path, "w", encoding="utf-8") as out:
        json.dump(all_results, out, ensure_ascii=False, indent=2)

    print("\n".join(report_lines))
    print("---")
    print("聚合完成。")


if __name__ == "__main__":
    main()
