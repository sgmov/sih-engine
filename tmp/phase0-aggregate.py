#!/usr/bin/env python3
"""Phase 0 statistical aggregation.

Reads all phase0 output jsonl files, computes per-document per-check-item:
- judgment distribution (count per judgment value)
- stability (whether all 10 samples agree)
- cumulative convergence curve (majority ratio at N=1..10)
- target-object-level stability (per target string, do 10 samples agree?)

Outputs a markdown report to stdout and a JSON summary to a file.

Usage:
    python3 phase0-aggregate.py <output_dir> <summary.json>
"""

import json
import os
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
        for lineno, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            records.append(rec)
    return records


def group_by_check_and_seq(records):
    """Returns {check_item: {sample_seq: [records]}}."""
    g = defaultdict(lambda: defaultdict(list))
    for r in records:
        ci = r.get("check_item")
        ss = int(r.get("sample_seq"))
        g[ci][ss].append(r)
    return g


def majority_ratio_at_n(records_by_seq, n):
    """Compute majority ratio over first n samples.

    For each sample seq in 1..n, flatten all records (multiple targets per sample).
    Aggregate all judgments across those samples, find majority value ratio.
    """
    all_judgments = []
    for ss in range(1, n + 1):
        for r in records_by_seq.get(ss, []):
            all_judgments.append(r.get("judgment"))
    if not all_judgments:
        return 0.0, None
    counts = Counter(all_judgments)
    top_val, top_cnt = counts.most_common(1)[0]
    return top_cnt / len(all_judgments), top_val


def convergence_curve(records_by_seq):
    """Compute majority ratio at N=1..10."""
    curve = []
    for n in range(1, 11):
        ratio, val = majority_ratio_at_n(records_by_seq, n)
        curve.append((n, round(ratio, 3), val))
    return curve


def per_target_stability(records_by_seq):
    """For each distinct target string, do all 10 samples give the same judgment?

    Returns dict {target: {judgments: Counter, stable: bool}}.
    """
    target_judgments = defaultdict(list)
    for ss in range(1, 11):
        for r in records_by_seq.get(ss, []):
            target_judgments[r.get("target")].append(r.get("judgment"))
    result = {}
    for target, judgments in target_judgments.items():
        counts = Counter(judgments)
        stable = len(counts) == 1
        result[target] = {"judgments": dict(counts), "stable": stable, "n": len(judgments)}
    return result


def aggregate_file(path):
    records = load_records(path)
    grouped = group_by_check_and_seq(records)
    result = {}
    for ci in CHECK_ITEMS:
        if ci not in grouped:
            result[ci] = {"status": "missing"}
            continue
        by_seq = grouped[ci]
        total_records = sum(len(v) for v in by_seq.values())
        # Overall distribution
        all_j = [r.get("judgment") for ss in range(1, 11) for r in by_seq.get(ss, [])]
        dist = dict(Counter(all_j))
        # Convergence curve
        curve = convergence_curve(by_seq)
        # Per-target stability
        tgt = per_target_stability(by_seq)
        n_targets = len(tgt)
        n_stable_targets = sum(1 for t in tgt.values() if t["stable"])
        result[ci] = {
            "total_records": total_records,
            "distribution": dist,
            "convergence_curve": curve,
            "n_targets": n_targets,
            "n_stable_targets": n_stable_targets,
            "target_stability": tgt,
        }
    return result


def fmt_curve(curve):
    parts = []
    for n, ratio, _ in curve:
        parts.append(f"N{n}={ratio}")
    return "  ".join(parts)


def main():
    if len(sys.argv) != 3:
        print("Usage: phase0-aggregate.py <output_dir> <summary.json>", file=sys.stderr)
        sys.exit(2)
    out_dir = Path(sys.argv[1])
    summary_path = sys.argv[2]

    files = sorted(out_dir.glob("*-phase0.jsonl"))
    if not files:
        print(f"No *-phase0.jsonl files in {out_dir}", file=sys.stderr)
        sys.exit(2)

    all_results = {}
    for f in files:
        doc_id = f.stem.replace("-phase0", "")
        all_results[doc_id] = aggregate_file(f)

    # Write JSON summary
    with open(summary_path, "w", encoding="utf-8") as out:
        json.dump(all_results, out, ensure_ascii=False, indent=2)

    # Print markdown report
    print(f"# Phase 0 统计聚合\n")
    print(f"输入目录 {out_dir}")
    print(f"文件数 {len(files)}")
    print(f"明细 JSON {summary_path}\n")

    for doc_id, doc_result in all_results.items():
        print(f"## {doc_id}\n")
        for ci in CHECK_ITEMS:
            ci_data = doc_result.get(ci, {})
            if ci_data.get("status") == "missing":
                print(f"### {ci}\n缺失\n")
                continue
            dist = ci_data["distribution"]
            curve = ci_data["convergence_curve"]
            nt = ci_data["n_targets"]
            ns = ci_data["n_stable_targets"]
            print(f"### {ci}")
            print(f"判断分布 {dist}")
            print(f"检查对象 {nt} 个，稳定 {ns} 个，不稳定 {nt - ns} 个")
            print(f"收敛曲线 {fmt_curve(curve)}")
            # Show unstable targets
            tgt = ci_data["target_stability"]
            unstable = {k: v for k, v in tgt.items() if not v["stable"]}
            if unstable:
                print(f"不稳定对象 {len(unstable)} 个")
                for k, v in unstable.items():
                    print(f"  {k[:40]}: {v['judgments']}")
            print()
    print("---")
    print("聚合完成。")


if __name__ == "__main__":
    main()
