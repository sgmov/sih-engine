#!/usr/bin/env python3
"""Minimum sample count analysis: N=2, 3, 5 vs ground truth (N=10).

Uses v3 data to simulate: if we only ran N sessions instead of 10,
how often would the majority vote match the 10-session "truth"?

Also analyzes:
- Undecided rate (ties, e.g. 1/1 with N=2)
- Anomaly detection capability at each N
- Cost-benefit: accuracy vs session count

Usage:
    python3 phase0-min-samples-analysis.py <phase0-v3-dir>
"""

import json
import random
import sys
from collections import Counter, defaultdict
from pathlib import Path

DOCS = ["DES-058", "DES-017", "DEC-008", "DEC-004", "sprint-plan", "INTENT-DRIFT"]
CHECK_ITEMS = [
    "承接关系有效性",
    "可证伪条件存在性",
    "认识论标签合法性",
    "术语一致性",
]
SAMPLE_SIZES = [2, 3, 5]
N_TRIALS = 1000  # Monte Carlo trials per (target, N)


def load_doc_sessions(doc_dir):
    """Load all 10 sessions for a doc. Returns {check_item: {target: {seq: judgment}}}."""
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


def majority_vote(judgments):
    """Return majority judgment, or None if tie."""
    counts = Counter(judgments)
    top = counts.most_common(2)
    if len(top) == 1:
        return top[0][0]
    if top[0][1] == top[1][1]:
        return None  # tie
    return top[0][0]


def simulate_n(all_judgments_by_seq, n, n_trials=1000):
    """Simulate sampling n sessions, compare to full-10 majority.

    all_judgments_by_seq: {seq: judgment} for one target (10 sessions)
    Returns: {correct, wrong, undecided} counts over n_trials.
    """
    seqs = list(all_judgments_by_seq.keys())
    full_judgments = list(all_judgments_by_seq.values())
    truth = majority_vote(full_judgments)

    results = {"correct": 0, "wrong": 0, "undecided": 0}
    for _ in range(n_trials):
        sampled = random.sample(seqs, min(n, len(seqs)))
        sampled_j = [all_judgments_by_seq[s] for s in sampled]
        vote = majority_vote(sampled_j)
        if vote is None:
            results["undecided"] += 1
        elif vote == truth:
            results["correct"] += 1
        else:
            results["wrong"] += 1
    return results


def main():
    base = Path(sys.argv[1])
    random.seed(42)

    # Collect all targets across all docs and check items
    # {doc: {check_item: {target: {seq: judgment}}}}
    all_data = {}
    for doc in DOCS:
        doc_dir = base / doc
        if doc_dir.exists():
            all_data[doc] = load_doc_sessions(doc_dir)

    report = []
    report.append("# 最小采样次数分析\n")
    report.append(f"方法：对每个检查对象，从 10 session 数据中随机抽取 N 个 session，")
    report.append(f"用多数投票得出判断，与 10 session 的多数投票真值比较。")
    report.append(f"每个对象每个 N 模拟 {N_TRIALS} 次。\n")

    for n in SAMPLE_SIZES:
        total_correct = 0
        total_wrong = 0
        total_undecided = 0
        total_trials = 0
        n_targets = 0

        for doc in DOCS:
            if doc not in all_data:
                continue
            for ci in CHECK_ITEMS:
                if ci not in all_data[doc]:
                    continue
                for tgt, seq_judgments in all_data[doc][ci].items():
                    if len(seq_judgments) < n:
                        continue
                    n_targets += 1
                    r = simulate_n(seq_judgments, n, N_TRIALS)
                    total_correct += r["correct"]
                    total_wrong += r["wrong"]
                    total_undecided += r["undecided"]
                    total_trials += N_TRIALS

        report.append(f"## N={n}（{n_targets} 个检查对象）\n")
        report.append(f"| 指标 | 值 |")
        report.append(f"|---|---|")
        report.append(f"| 与真值一致 | {total_correct/total_trials*100:.1f}% |")
        report.append(f"| 与真值不一致 | {total_wrong/total_trials*100:.1f}% |")
        report.append(f"| 平局无法判定 | {total_undecided/total_trials*100:.1f}% |")
        report.append(f"| 有效判定率 | {(total_correct+total_wrong)/total_trials*100:.1f}% |")
        report.append(f"| 判定正确率 | {total_correct/(total_correct+total_wrong)*100:.2f}% |")
        report.append("")

    # Breakdown by disagreement type for 承接关系有效性 (DES-058, richest signal)
    report.append("## 按分歧类型细分（DES-058 承接关系有效性）\n")
    doc_data = all_data.get("DES-058", {}).get("承接关系有效性", {})
    report.append("| 对象 | 10session票型 | N=2正确率 | N=2平局率 | N=3正确率 | N=5正确率 |")
    report.append("|---|---|---|---|---|---|")
    for tgt in sorted(doc_data.keys()):
        seq_j = doc_data[tgt]
        if len(seq_j) < 10:
            continue
        votes = list(seq_j.values())
        counts = Counter(votes)
        vote_str = " / ".join(f"{k}×{v}" for k, v in counts.most_common())
        truth = majority_vote(votes)

        row_parts = [tgt, vote_str]
        for n in [2, 3, 5]:
            r = simulate_n(seq_j, n, N_TRIALS)
            eff = r["correct"] + r["wrong"]
            corr_rate = f"{r['correct']/N_TRIALS*100:.0f}%"
            if n == 2:
                tie_rate = f"{r['undecided']/N_TRIALS*100:.0f}%"
                row_parts.extend([corr_rate, tie_rate])
            else:
                row_parts.append(corr_rate)
        report.append("| " + " | ".join(str(p) for p in row_parts) + " |")
    report.append("")

    # Cost analysis
    report.append("## 成本-收益分析\n")
    report.append("| N | session数/文档 | 总session(6文档) | 输入tokens估算 | 正确率 | 平局率 |")
    report.append("|---|---|---|---|---|---|")
    # v3 total input was 35.17M for 60 sessions
    per_session_input = 35.17e6 / 60
    for n in [2, 3, 5, 10]:
        total_sessions = n * 6
        est_input = per_session_input * total_sessions / 1e6
        if n in [2, 3, 5]:
            # recompute summary for this N
            tc = tw = tu = tt = 0
            for doc in DOCS:
                if doc not in all_data:
                    continue
                for ci in CHECK_ITEMS:
                    if ci not in all_data[doc]:
                        continue
                    for tgt, sj in all_data[doc][ci].items():
                        if len(sj) < n:
                            continue
                        r = simulate_n(sj, n, N_TRIALS)
                        tc += r["correct"]
                        tw += r["wrong"]
                        tu += r["undecided"]
                        tt += N_TRIALS
            corr = f"{tc/tt*100:.1f}%"
            tie = f"{tu/tt*100:.1f}%"
        else:
            corr = "100.0% (真值)"
            tie = "0.0%"
        report.append(f"| {n} | {n} | {total_sessions} | {est_input:.1f}M | {corr} | {tie} |")
    report.append("")

    print("\n".join(report))
    print("---")
    print("分析完成。")


if __name__ == "__main__":
    main()
