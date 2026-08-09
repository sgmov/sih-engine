#!/usr/bin/env python3
"""Phase 0 diagnostic aggregation: true session independence test.

Reads 10 per-seq files from phase0-diag/, aligns by (check_item, target),
computes per-object cross-session agreement.

Key outputs:
- For each (check_item, target): how many of the 10 sessions judged it,
  what judgments they gave, and whether they agree.
- Extraction count variance per session (how many objects each session found).
- Cross-session convergence: majority ratio per check_item.

Usage:
    python3 phase0-diag-aggregate.py <diag_dir> <summary.json>
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


def load_records(path):
    records = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            records.append(json.loads(line))
    return records


def normalize_target(t):
    """Loose normalization for matching targets across sessions:
    strip whitespace, lowercase ascii, keep first 40 chars."""
    if not t:
        return "(empty)"
    t = t.strip()
    # Keep original; full normalization is too error-prone, we group by raw target
    return t


def main():
    if len(sys.argv) != 3:
        print("Usage: phase0-diag-aggregate.py <diag_dir> <summary.json>", file=sys.stderr)
        sys.exit(2)
    diag_dir = Path(sys.argv[1])
    summary_path = sys.argv[2]

    files = sorted(diag_dir.glob("DES-058-seq*.jsonl"))
    if not files:
        print(f"No DES-058-seq*.jsonl in {diag_dir}", file=sys.stderr)
        sys.exit(2)

    # Load all
    session_data = {}  # seq -> {check_item -> [records]}
    per_seq_counts = {}  # seq -> total records
    for f in files:
        recs = load_records(f)
        by_ci = defaultdict(list)
        for r in recs:
            by_ci[r.get("check_item")].append(r)
        # Extract seq from filename or records
        seq = int(f.stem.split("-seq")[1].split(".")[0])
        session_data[seq] = by_ci
        per_seq_counts[seq] = len(recs)

    # For each check item, align by target string
    # target_to_judgments: {check_item: {target: {seq: judgment}}}
    alignment = defaultdict(lambda: defaultdict(dict))
    for seq, by_ci in session_data.items():
        for ci, recs in by_ci.items():
            for r in recs:
                tgt = normalize_target(r.get("target"))
                # If same seq has duplicate targets (shouldn't), last wins
                alignment[ci][tgt][seq] = r.get("judgment")

    # Compute per-check-item, per-target agreement
    summary = {}
    report_lines = []

    report_lines.append("# Phase 0 独立性诊断聚合\n")
    report_lines.append(f"输入目录 {diag_dir}")
    report_lines.append(f"session 数 {len(files)}")
    report_lines.append(f"明细 JSON {summary_path}\n")

    # Per-session extraction counts
    report_lines.append("## 各 session 抽取量\n")
    report_lines.append("| session | 承接关系 | 可证伪 | 标签 | 术语 | 合计 |")
    report_lines.append("|---|---|---|---|---|---|")
    for seq in sorted(session_data.keys()):
        by_ci = session_data[seq]
        c1 = len(by_ci.get("承接关系有效性", []))
        c2 = len(by_ci.get("可证伪条件存在性", []))
        c3 = len(by_ci.get("认识论标签合法性", []))
        c4 = len(by_ci.get("术语一致性", []))
        report_lines.append(
            f"| seq{seq} | {c1} | {c2} | {c3} | {c4} | {c1+c2+c3+c4} |"
        )
    report_lines.append("")

    for ci in CHECK_ITEMS:
        targets = alignment.get(ci, {})
        if not targets:
            summary[ci] = {"status": "missing"}
            continue

        total_targets = len(targets)
        # For each target: how many sessions judged it, agreement
        per_target = []
        for tgt, seq_judgments in targets.items():
            n_sessions = len(seq_judgments)
            judgments = list(seq_judgments.values())
            counts = Counter(judgments)
            n_distinct = len(counts)
            top_val, top_cnt = counts.most_common(1)[0]
            agreement = top_cnt / n_sessions if n_sessions > 0 else 0
            per_target.append({
                "target": tgt,
                "n_sessions": n_sessions,
                "judgments": dict(counts),
                "n_distinct": n_distinct,
                "majority": top_val,
                "agreement": round(agreement, 3),
                "fully_agree": n_distinct == 1,
            })

        # Targets judged by all 10 sessions
        all10 = [t for t in per_target if t["n_sessions"] == 10]
        # Targets with disagreement (judged by >=2 sessions, n_distinct > 1)
        disagree = [t for t in per_target if t["n_sessions"] >= 2 and not t["fully_agree"]]
        # Cross-session convergence: for targets judged by all 10, avg agreement
        if all10:
            avg_agree = sum(t["agreement"] for t in all10) / len(all10)
            min_agree = min(t["agreement"] for t in all10)
        else:
            avg_agree = None
            min_agree = None

        summary[ci] = {
            "total_distinct_targets": total_targets,
            "targets_judged_by_all_10": len(all10),
            "targets_with_disagreement": len(disagree),
            "avg_agreement_all10": round(avg_agree, 3) if avg_agree else None,
            "min_agreement_all10": round(min_agree, 3) if min_agree else None,
            "per_target": per_target,
        }

        report_lines.append(f"## {ci}\n")
        report_lines.append(f"不同对象总数（所有 session 并集）: {total_targets}")
        report_lines.append(f"被全部 10 session 判断的对象: {len(all10)}")
        report_lines.append(f"出现分歧的对象（>=2 session 且判断不同）: {len(disagree)}")
        if avg_agree:
            report_lines.append(f"全 10 session 对象的平均一致率: {round(avg_agree,3)}")
            report_lines.append(f"最低一致率: {round(min_agree,3)}")
        report_lines.append("")

        if disagree:
            report_lines.append(f"### {ci} 分歧对象明细\n")
            report_lines.append("| 对象 | 各判断票数 | 一致率 |")
            report_lines.append("|---|---|---|")
            for t in sorted(disagree, key=lambda x: x["agreement"]):
                tgt_short = t["target"][:50]
                votes = " / ".join(f"{k}×{v}" for k, v in t["judgments"].items())
                report_lines.append(
                    f"| {tgt_short} | {votes} | {t['agreement']} |"
                )
            report_lines.append("")

    # Write summary JSON
    with open(summary_path, "w", encoding="utf-8") as out:
        json.dump(summary, out, ensure_ascii=False, indent=2)

    print("\n".join(report_lines))
    print("---")
    print("诊断聚合完成。")


if __name__ == "__main__":
    main()
