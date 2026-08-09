#!/usr/bin/env python3
"""Phase 2 full-scale aggregation.

Supports two modes:
1. Single type: python3 phase2-full-aggregate.py governance
2. All types:   python3 phase2-full-aggregate.py all

For each check item, computes:
- 4/0 unanimous, 3/1 strong majority, 2/2 tie, other
- auto-decide rate, tie rate
- anomaly targets (2/2 ties with systematic split)
- comparison with Phase 1 same type
"""

import json
import sys
from collections import defaultdict, Counter
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor/sih-engine/sih/event/experiment/phase2-full")
PHASE1_ROOT = Path("/Users/moc/workspaces/SiHankor/sih-engine/sih/event/experiment/phase1-round1")

TYPE_ORDER = ["governance", "research", "draft", "knowledge", "spec", "decision", "proposal", "design"]
CHECK_ITEMS = ["承接语义对应性", "论证自洽性", "归位充分性"]


def load_checklist_targets(type_name):
    """Load expected targets from checklists. Returns set of (doc_id, check_item, target)."""
    CL_BASE = ROOT / "checklists"
    prefix = type_name + "__"
    valid_keys = set()
    for cl_file in CL_BASE.glob(f"{prefix}*.json"):
        cl = json.loads(cl_file.read_text())
        doc_id = cl["doc_id"]
        for item_name in CHECK_ITEMS:
            for item in cl.get(item_name, []):
                valid_keys.add((doc_id, item_name, item["target"]))
    return valid_keys


def load_type_data(type_name):
    """Load all jsonl for a type. Returns dict[(doc_id, check_item, target)] -> {seq: judgment}.

    Only aggregates targets present in the checklist (discards extra targets
    that subagents may have produced beyond the fixed checklist).
    """
    type_dir = ROOT / type_name
    if not type_dir.exists():
        return {}

    valid_keys = load_checklist_targets(type_name)
    dropped = 0

    by_key = defaultdict(dict)
    for doc_dir in sorted(type_dir.iterdir()):
        if not doc_dir.is_dir():
            continue
        doc_id = doc_dir.name
        for seq in range(1, 6):  # support seq5 tie-break
            seq_files = list(doc_dir.glob(f"*-seq{seq}.jsonl"))
            if not seq_files:
                continue
            try:
                for line in open(seq_files[0]):
                    line = line.strip()
                    if not line:
                        continue
                    rec = json.loads(line)
                    key = (doc_id, rec["check_item"], rec["target"])
                    # 只聚合清单中存在的 target，丢弃子代理多产出的项
                    if key not in valid_keys:
                        dropped += 1
                        continue
                    by_key[key][seq] = rec["judgment"]
            except (json.JSONDecodeError, KeyError) as e:
                print(f"WARN: parse error in {seq_files[0]}: {e}")
    if dropped:
        print(f"  (已丢弃 {dropped} 条不在清单中的记录)")
    return by_key


def classify_agreement(judgments_by_seq):
    """Classify agreement pattern. Returns category string."""
    vals = [judgments_by_seq.get(s) for s in sorted(judgments_by_seq) if judgments_by_seq.get(s)]
    n = len(vals)
    if n == 0:
        return "empty"
    counts = Counter(vals).most_common(2)
    if len(counts) == 1:
        return "4/0"
    if counts[0][1] >= 3:
        return "3/1"
    if n == 4 and counts[0][1] == 2 and counts[1][1] == 2:
        # check systematic split
        seqs = sorted(judgments_by_seq.keys())
        v = judgments_by_seq
        if len(seqs) == 4:
            if v[seqs[0]] == v[seqs[2]] and v[seqs[1]] == v[seqs[3]] and v[seqs[0]] != v[seqs[1]]:
                return "2/2-systematic"
        return "2/2"
    if counts[0][1] == 2:
        return "2/1/1"
    return "other"


def aggregate_type(type_name, by_key):
    """Aggregate one type, return stats dict."""
    stats = {
        "type": type_name,
        "total_targets": 0,
        "per_item": {},
        "anomaly_targets": [],
    }

    for item in CHECK_ITEMS:
        item_keys = [k for k in by_key if k[1] == item]
        if not item_keys:
            stats["per_item"][item] = {"total": 0}
            continue

        cats = Counter()
        for k in item_keys:
            cat = classify_agreement(by_key[k])
            cats[cat] += 1
            stats["total_targets"] += 1
            if cat in ("2/2", "2/2-systematic"):
                stats["anomaly_targets"].append({
                    "doc_id": k[0],
                    "target": k[2],
                    "judgments": {str(s): v for s, v in by_key[k].items()},
                    "pattern": cat,
                })

        total = len(item_keys)
        u4 = cats.get("4/0", 0)
        u31 = cats.get("3/1", 0)
        u22 = cats.get("2/2", 0) + cats.get("2/2-systematic", 0)
        systematic = cats.get("2/2-systematic", 0)
        other = total - u4 - u31 - u22

        stats["per_item"][item] = {
            "total": total,
            "4/0": u4,
            "3/1": u31,
            "2/2_tie": u22,
            "2/2_systematic": systematic,
            "other": other,
            "tie_rate": round(u22 / total * 100, 1) if total else 0,
            "auto_decide_rate": round((u4 + u31) / total * 100, 1) if total else 0,
        }

    return stats


def load_phase1_type(type_name):
    """Load Phase 1 data for comparison."""
    type_dir = PHASE1_ROOT / type_name
    if not type_dir.exists():
        return None

    by_key = defaultdict(dict)
    for doc_dir in sorted(type_dir.iterdir()):
        if not doc_dir.is_dir():
            continue
        doc_id = doc_dir.name
        for seq in range(1, 6):
            seq_files = list(doc_dir.glob(f"*-seq{seq}.jsonl"))
            if not seq_files:
                continue
            for line in open(seq_files[0]):
                line = line.strip()
                if not line:
                    continue
                try:
                    rec = json.loads(line)
                    key = (doc_id, rec["check_item"], rec["target"])
                    by_key[key][seq] = rec["judgment"]
                except:
                    pass

    if not by_key:
        return None

    total = len(by_key)
    u4 = u31 = u22 = 0
    for k, sjs in by_key.items():
        cat = classify_agreement(sjs)
        if cat == "4/0":
            u4 += 1
        elif cat == "3/1":
            u31 += 1
        elif cat in ("2/2", "2/2-systematic"):
            u22 += 1
    return {
        "total": total,
        "tie_rate": round(u22 / total * 100, 1) if total else 0,
        "auto_decide_rate": round((u4 + u31) / total * 100, 1) if total else 0,
    }


def print_type_report(stats, phase1_data=None):
    """Print formatted report for one type."""
    type_name = stats["type"]
    print(f"\n{'='*70}")
    print(f"Phase 2 Full: {type_name}")
    print(f"{'='*70}")

    for item in CHECK_ITEMS:
        s = stats["per_item"].get(item, {})
        total = s.get("total", 0)
        if total == 0:
            print(f"\n  {item}: (无数据)")
            continue
        print(f"\n  {item}:")
        print(f"    total targets:    {total}")
        print(f"    4/0 unanimous:    {s['4/0']:4d} ({s['4/0']/total*100:5.1f}%)")
        print(f"    3/1 strong:       {s['3/1']:4d} ({s['3/1']/total*100:5.1f}%)")
        print(f"    2/2 tie:          {s['2/2_tie']:4d} ({s['tie_rate']:5.1f}%)")
        if s.get("2/2_systematic", 0) > 0:
            print(f"      其中系统性分裂:  {s['2/2_systematic']}")
        print(f"    other:            {s['other']:4d}")
        print(f"    auto-decide rate: {s['auto_decide_rate']:.1f}%")

    # Phase 1 对比
    if phase1_data:
        print(f"\n  --- Phase 1 对照 (结构查询层) ---")
        print(f"    Phase 1 total:     {phase1_data['total']}")
        print(f"    Phase 1 tie rate:  {phase1_data['tie_rate']:.1f}%")
        print(f"    Phase 1 auto-dec:  {phase1_data['auto_decide_rate']:.1f}%")

    # 异常 targets
    anomalies = stats["anomaly_targets"]
    if anomalies:
        print(f"\n  --- 异常 targets (2/2 tie, 共 {len(anomalies)} 个) ---")
        for a in anomalies[:15]:
            print(f"    {a['doc_id'][:40]:<40} {a['target'][:20]:<20} {a['pattern']}")
            print(f"      {a['judgments']}")
        if len(anomalies) > 15:
            print(f"    ... 还有 {len(anomalies)-15} 个")


def main():
    if len(sys.argv) < 2:
        print("用法: python3 phase2-full-aggregate.py <type_name|all>")
        print(f"可选类型: {', '.join(TYPE_ORDER)}, all")
        sys.exit(1)

    target = sys.argv[1]

    if target == "all":
        all_stats = []
        for t in TYPE_ORDER:
            by_key = load_type_data(t)
            if not by_key:
                print(f"\n{t}: (无数据，可能尚未执行)")
                continue
            stats = aggregate_type(t, by_key)
            p1 = load_phase1_type(t)
            all_stats.append(stats)
            print_type_report(stats, p1)

        # 汇总
        if all_stats:
            print(f"\n{'='*70}")
            print(f"Phase 2 Full 全量汇总")
            print(f"{'='*70}")
            print(f"\n{'类型':<14}", end="")
            for item in CHECK_ITEMS:
                print(f"  {item[:8]:>10} tie%  auto%", end="")
            print()
            print("-" * 80)

            for s in all_stats:
                print(f"{s['type']:<14}", end="")
                for item in CHECK_ITEMS:
                    si = s["per_item"].get(item, {})
                    if si.get("total", 0) == 0:
                        print(f"  {'n/a':>10}  {'':>4}  {'':>5}", end="")
                    else:
                        print(f"  {si['total']:>10}  {si['tie_rate']:>4.1f}  {si['auto_decide_rate']:>5.1f}", end="")
                print()

    else:
        if target not in TYPE_ORDER:
            print(f"未知类型: {target}")
            sys.exit(1)
        by_key = load_type_data(target)
        if not by_key:
            print(f"{target}: 无数据，可能尚未执行或输出目录不存在")
            sys.exit(1)
        stats = aggregate_type(target, by_key)
        p1 = load_phase1_type(target)
        print_type_report(stats, p1)


if __name__ == "__main__":
    main()
