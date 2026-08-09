#!/usr/bin/env python3
"""
Phase 2 calibration analysis.

Reads 24 jsonl files (6 docs × 4 seq), aggregates per check item:
  - judgment distribution per target (N=4 majority vote)
  - tie rate (no majority) per check item per doc
  - agreement rate per check item overall
  - discriminative power assessment
"""

import json
from collections import defaultdict, Counter
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor/sih-engine/sih/event/experiment/phase2-calibration")
DOCS = ["DES-058", "DES-017", "DEC-008", "DEC-004", "sprint-plan", "INTENT-DRIFT"]
CHECK_ITEMS = ["承接语义对应性", "论证自洽性", "归位充分性"]


def load_jsonl(path):
    """Load one jsonl file, return list of dicts."""
    records = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                records.append(json.loads(line))
            except json.JSONDecodeError as e:
                print(f"WARN: parse error in {path}: {e}")
    return records


def load_all():
    """Return dict[(doc_id, check_item, target)] -> {seq: judgment}."""
    by_key = defaultdict(dict)
    for doc in DOCS:
        for seq in range(1, 5):
            p = ROOT / doc / f"{doc}-seq{seq}.jsonl"
            if not p.exists():
                print(f"WARN missing: {p}")
                continue
            for rec in load_jsonl(p):
                key = (rec["doc_id"], rec["check_item"], rec["target"])
                by_key[key][seq] = rec["judgment"]
    return by_key


def majority_vote(judgments_by_seq):
    """N=4 majority. Returns (verdict, is_tie)."""
    seqs = sorted(judgments_by_seq.keys())
    counts = Counter(judgments_by_seq[s] for s in seqs)
    top = counts.most_common()
    if len(top) == 1:
        return top[0][0], False
    if top[0][1] > top[1][1]:
        return top[0][0], False
    return None, True  # tie


def analyze():
    by_key = load_all()

    # ---- Per check-item summary ----
    print("=" * 80)
    print("Phase 2 Calibration Analysis")
    print("=" * 80)

    for item in CHECK_ITEMS:
        print(f"\n### Check Item: {item}")
        print("-" * 60)

        # collect all keys for this check item
        item_keys = [k for k in by_key if k[1] == item]

        # group by doc
        per_doc = defaultdict(list)
        for k in item_keys:
            per_doc[k[0]].append(k)

        overall_total = 0
        overall_ties = 0
        overall_dist = Counter()

        for doc in DOCS:
            keys = per_doc.get(doc, [])
            if not keys:
                print(f"  [{doc}] NO DATA")
                continue

            doc_total = len(keys)
            doc_ties = 0
            doc_dist = Counter()
            non_unanimous = []

            for k in keys:
                seq_j = by_key[k]
                verdict, is_tie = majority_vote(seq_j)
                if is_tie:
                    doc_ties += 1
                    non_unanimous.append((k[2], dict(seq_j)))
                elif verdict is not None:
                    doc_dist[verdict] += 1
                # for tie, no verdict, count separately
                for s in seq_j:
                    overall_dist[seq_j[s]] += 1

            overall_total += doc_total
            overall_ties += doc_ties

            tie_rate = doc_ties / doc_total * 100 if doc_total else 0
            print(f"\n  [{doc}] targets={doc_total}, ties={doc_ties} ({tie_rate:.1f}%)")
            print(f"    verdict distribution: {dict(doc_dist)}")
            if non_unanimous:
                print(f"    non-unanimous/tie targets ({len(non_unanimous)}):")
                for tgt, sj in non_unanimous[:8]:
                    print(f"      {tgt}: {sj}")
                if len(non_unanimous) > 8:
                    print(f"      ... and {len(non_unanimous)-8} more")

        # overall for this check item
        overall_tie_rate = overall_ties / overall_total * 100 if overall_total else 0
        print(f"\n  >>> {item} OVERALL: targets={overall_total}, ties={overall_ties} ({overall_tie_rate:.1f}%)")
        print(f"  >>> judgment distribution (raw across all seq): {dict(overall_dist)}")

    # ---- Cross-item tie matrix per doc ----
    print("\n" + "=" * 80)
    print("Tie Rate Matrix (doc × check item)")
    print("=" * 80)
    header = f"{'doc':<16}" + "".join(f"{c:<22}" for c in CHECK_ITEMS) + "total"
    print(header)
    for doc in DOCS:
        row = f"{doc:<16}"
        doc_total_ties = 0
        doc_total_targets = 0
        for item in CHECK_ITEMS:
            keys = [k for k in by_key if k[0] == doc and k[1] == item]
            if not keys:
                row += f"{'n/a':<22}"
                continue
            ties = sum(1 for k in keys if majority_vote(by_key[k])[1])
            rate = ties / len(keys) * 100
            row += f"{ties}/{len(keys)} ({rate:.0f}%){'':<6}".ljust(22)
            doc_total_ties += ties
            doc_total_targets += len(keys)
        overall = doc_total_ties / doc_total_targets * 100 if doc_total_targets else 0
        row += f"  {doc_total_ties}/{doc_total_targets} ({overall:.0f}%)"
        print(row)

    # ---- Unanimity analysis (4/0 and 3/1) ----
    print("\n" + "=" * 80)
    print("Unanimity Distribution (per check item)")
    print("=" * 80)
    for item in CHECK_ITEMS:
        item_keys = [k for k in by_key if k[1] == item]
        unanimous = 0
        three_one = 0
        two_two = 0
        other = 0
        for k in item_keys:
            seq_j = by_key[k]
            vals = list(seq_j.values())
            uc = Counter(vals)
            top = uc.most_common(2)
            if len(top) == 1:
                unanimous += 1
            elif top[0][1] == 3 and top[1][1] == 1:
                three_one += 1
            elif top[0][1] == 2 and top[1][1] == 2:
                two_two += 1
            else:
                other += 1
        total = len(item_keys)
        print(f"  {item}: total={total}, 4/0={unanimous} ({unanimous/total*100:.1f}%), "
              f"3/1={three_one} ({three_one/total*100:.1f}%), "
              f"2/2={two_two} ({two_two/total*100:.1f}%)")

    # ---- Calibration verdict ----
    print("\n" + "=" * 80)
    print("Calibration Verdict")
    print("=" * 80)
    print("Pass criteria: tie rate 5-20% on anchor docs")
    print("  <2% = too easy (no discriminative power)")
    print("  >30% = too vague (ambiguous criteria)")
    for item in CHECK_ITEMS:
        item_keys = [k for k in by_key if k[1] == item]
        ties = sum(1 for k in item_keys if majority_vote(by_key[k])[1])
        rate = ties / len(item_keys) * 100 if item_keys else 0
        if rate < 2:
            verdict = "TOO EASY (<2% tie) — may lack discriminative power"
        elif rate <= 20:
            verdict = "PASS (5-20% tie) — has discriminative power"
        elif rate <= 30:
            verdict = "BORDERLINE (20-30% tie) — review for clarity"
        else:
            verdict = "TOO VAGUE (>30% tie) — criteria may be ambiguous"
        print(f"  {item}: tie rate {rate:.1f}% → {verdict}")


if __name__ == "__main__":
    analyze()
