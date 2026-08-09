#!/usr/bin/env python3
"""Anomaly session detection algorithm validation.

Tests multiple detection strategies on v3 data to see if they
accurately identify seq3 (the systematically biased session in DES-058).

Strategies:
1. Minority frequency: session is anomaly if it's the minority vote
   in more than X% of disagreed targets
2. Agreement deficit: session's average pairwise agreement with other
   sessions is below threshold
3. Outlier judgment rate: session disagrees with majority more than
   X standard deviations from the mean

For each strategy, compute:
- True positive: correctly flags seq3 as anomaly in DES-058
- False positive: incorrectly flags a normal session
- Across all 6 docs: does it flag anomalies that don't exist?

Usage:
    python3 phase0-anomaly-detection.py <phase0-v3-dir>
"""

import json
import sys
from collections import Counter, defaultdict
from pathlib import Path
from itertools import combinations

DOCS = ["DES-058", "DES-017", "DEC-008", "DEC-004", "sprint-plan", "INTENT-DRIFT"]
CHECK_ITEMS = [
    "承接关系有效性",
    "可证伪条件存在性",
    "认识论标签合法性",
    "术语一致性",
]


def load_doc_sessions(doc_dir):
    sessions = {}
    for seq in range(1, 11):
        f = doc_dir / f"{doc_dir.name}-seq{seq}.jsonl"
        if not f.exists():
            continue
        recs = [json.loads(l) for l in open(f) if l.strip()]
        for r in recs:
            ci = r.get("check_item")
            tgt = r.get("target", "(empty)")
            j = r.get("judgment")
            if ci not in sessions:
                sessions[ci] = defaultdict(dict)
            sessions[ci][tgt][seq] = j
    return sessions


def analyze_doc(doc_dir):
    """For one doc, compute per-session anomaly metrics across all check items."""
    sessions = load_doc_sessions(doc_dir)
    if not sessions:
        return None

    # Strategy 1: minority frequency
    minority_count = Counter()  # seq -> count of times it's the minority
    total_disagree = 0

    for ci in CHECK_ITEMS:
        if ci not in sessions:
            continue
        ci_data = sessions[ci]
        targets = set()
        for s in ci_data.values():
            targets.update(s.keys())

        for tgt in targets:
            votes = {seq: ci_data[seq][tgt] for seq in ci_data if tgt in ci_data[seq]}
            if len(votes) < 2:
                continue
            counts = Counter(votes.values())
            if len(counts) == 1:
                continue  # no disagreement
            total_disagree += 1
            majority_val = counts.most_common(1)[0][0]
            for seq, v in votes.items():
                if v != majority_val:
                    minority_count[seq] += 1

    # Strategy 2: pairwise agreement
    # For each pair of sessions, compute agreement rate over all common targets
    seq_agreement_sum = defaultdict(float)
    seq_agreement_count = defaultdict(int)

    for ci in CHECK_ITEMS:
        if ci not in sessions:
            continue
        ci_data = sessions[ci]
        targets = set()
        for s in ci_data.values():
            targets.update(s.keys())

        for seq_a, seq_b in combinations(sorted(ci_data.keys()), 2):
            common = [t for t in targets if t in ci_data[seq_a] and t in ci_data[seq_b]]
            if not common:
                continue
            agreements = sum(1 for t in common if ci_data[seq_a][t] == ci_data[seq_b][t])
            rate = agreements / len(common)
            seq_agreement_sum[seq_a] += rate
            seq_agreement_sum[seq_b] += rate
            seq_agreement_count[seq_a] += 1
            seq_agreement_count[seq_b] += 1

    avg_agreement = {}
    for seq in sorted(seq_agreement_sum.keys()):
        if seq_agreement_count[seq] > 0:
            avg_agreement[seq] = round(seq_agreement_sum[seq] / seq_agreement_count[seq], 4)
        else:
            avg_agreement[seq] = None

    return {
        "minority_count": dict(minority_count),
        "total_disagree": total_disagree,
        "avg_agreement": avg_agreement,
        "n_sessions": len(avg_agreement),
    }


def main():
    base = Path(sys.argv[1])
    report = []
    report.append("# 异常 session 检测算法验证\n")

    all_analysis = {}
    for doc in DOCS:
        doc_dir = base / doc
        if not doc_dir.exists():
            continue
        all_analysis[doc] = analyze_doc(doc_dir)

    # Strategy 1 results: minority frequency
    report.append("## 策略一：少数派频次法\n")
    report.append("原理：在所有出现分歧的对象中，某 session 是少数派的次数。")
    report.append("阈值：少数派次数超过分歧总数 30% 即为异常 session。\n")

    report.append("| 文档 | 分歧对象数 | 异常 session（>30%） | 各 session 少数派次数 |")
    report.append("|---|---|---|---|")
    for doc in DOCS:
        a = all_analysis.get(doc)
        if not a or a["total_disagree"] == 0:
            report.append(f"| {doc} | {a['total_disagree'] if a else 0} | 无分歧 | — |")
            continue
        threshold = a["total_disagree"] * 0.3
        anomalies = [s for s, c in a["minority_count"].items() if c > threshold]
        anomaly_str = ", ".join(f"seq{s}" for s in sorted(anomalies)) if anomalies else "无"
        all_seqs = sorted(set(list(a["minority_count"].keys()) + list(a["avg_agreement"].keys())))
        counts_str = " ".join(f"seq{s}:{a['minority_count'].get(s,0)}" for s in all_seqs)
        report.append(f"| {doc} | {a['total_disagree']} | {anomaly_str} | {counts_str} |")
    report.append("")

    # Strategy 2 results: pairwise agreement deficit
    report.append("## 策略二：成对一致率法\n")
    report.append("原理：每个 session 与其他所有 session 的平均一致率。")
    report.append("阈值：一致率低于所有 session 平均值减 1 个标准差即为异常。\n")

    report.append("| 文档 | 各 session 平均一致率 | 异常 session |")
    report.append("|---|---|---|")
    for doc in DOCS:
        a = all_analysis.get(doc)
        if not a or not a["avg_agreement"]:
            report.append(f"| {doc} | 无数据 | — |")
            continue
        rates = [r for r in a["avg_agreement"].values() if r is not None]
        if not rates:
            report.append(f"| {doc} | 无数据 | — |")
            continue
        mean_r = sum(rates) / len(rates)
        variance = sum((r - mean_r) ** 2 for r in rates) / len(rates)
        std_r = variance ** 0.5
        threshold = mean_r - std_r
        anomalies = [s for s, r in a["avg_agreement"].items() if r is not None and r < threshold]
        anomaly_str = ", ".join(f"seq{s}" for s in sorted(anomalies)) if anomalies else "无"
        rates_str = " ".join(f"seq{s}:{a['avg_agreement'][s]:.3f}" for s in sorted(a["avg_agreement"].keys()))
        report.append(f"| {doc} | {rates_str} | {anomaly_str} |")
    report.append("")

    # Combined assessment
    report.append("## 综合评估\n")
    report.append("已知真值：DES-058 的 seq3 是系统性偏差 session（诊断阶段验证，14/18 分歧对象的少数派）。\n")

    # Check strategy 1 on DES-058
    a58 = all_analysis.get("DES-058", {})
    if a58:
        threshold = a58["total_disagree"] * 0.3
        s1_anomalies = [s for s, c in a58["minority_count"].items() if c > threshold]
        report.append(f"策略一在 DES-058 上的表现：")
        report.append(f"  分歧对象 {a58['total_disagree']} 个，30% 阈值 = {threshold:.1f}")
        report.append(f"  检出异常 session：{sorted(s1_anomalies)}")
        report.append(f"  seq3 少数派次数：{a58['minority_count'].get(3, 0)}")
        report.append(f"  结果：{'检出 seq3（真阳性）' if 3 in s1_anomalies else '漏检 seq3（假阴性）'}")
        fp = [s for s in s1_anomalies if s != 3]
        report.append(f"  误报：{sorted(fp) if fp else '无（无假阳性）'}")
        report.append("")

    # Check false positives across all docs
    report.append("策略一全文档假阳性检查：")
    for doc in DOCS:
        a = all_analysis.get(doc, {})
        if not a or a["total_disagree"] == 0:
            continue
        threshold = a["total_disagree"] * 0.3
        anomalies = [s for s, c in a["minority_count"].items() if c > threshold]
        # Flag docs where >1 session is anomaly (indicates noise not real anomaly)
        if len(anomalies) > 1:
            report.append(f"  {doc}：检出 {len(anomalies)} 个异常 session {sorted(anomalies)}，可能误报")
    report.append("")

    # Threshold sensitivity analysis for strategy 1
    report.append("## 策略一阈值敏感性（DES-058）\n")
    report.append("| 阈值 | 检出 session | seq3 是否检出 | 误报数 |")
    report.append("|---|---|---|---|")
    for pct in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6]:
        threshold = a58["total_disagree"] * pct
        detected = [s for s, c in a58["minority_count"].items() if c > threshold]
        has_seq3 = 3 in detected
        fp = len([s for s in detected if s != 3])
        report.append(f"| >{int(pct*100)}% | {sorted(detected)} | {'是' if has_seq3 else '否'} | {fp} |")
    report.append("")

    print("\n".join(report))
    print("---")
    print("验证完成。")


if __name__ == "__main__":
    main()
