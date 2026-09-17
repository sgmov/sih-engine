# retrieverline 批（rl-04）同参双跑对表工具壳侧 harness。
# 正典指针：retrieverline 批立项包（sih-engine/sih/state/plan/retrieverline.md）。
#
# 读同目录 engine-results.json（引擎侧读数，tests/retrline_dualrun.rs 产出），
# 以 sih-tools/wikirecall/semantic.semantic_channel（冻结只读，import 零写）对同一
# 语料（corpus/ 十二件，整档文本为文档面）与同一查询同参跑工具壳侧读数，逐查询
# 对表出 dualrun-summary.json。判词口径机械可查：top-k 集合全等且逐位得分差
# ≤ 1e-9 方判一致；不一致如实入清单，禁美化。
#
# 用法：python3 run_semantic_side.py（本目录内跑，零参数零写 wikirecall）

import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, "/Users/moc/workspaces/SiHankor/sih-tools/wikirecall")
import semantic  # noqa: E402  冻结只读 import

K = 3
SCORE_TOL = 1e-9


def load_index():
    index = {}
    for f in sorted((HERE / "corpus").glob("*.md")):
        text = f.read_text(encoding="utf-8")
        # 通道对价对齐：wikirecall hay = title + "\n" + triggers；此处 title=整档
        # 文本 triggers=[]，hay 与引擎侧文档面（整档文本）词元集全同。
        index[f.stem] = {"title": text, "triggers": []}
    return index


def main():
    engine = json.loads((HERE / "engine-results.json").read_text(encoding="utf-8"))
    assert engine["schema"] == "retrline-dualrun-engine/1", "引擎侧读数档形不符"
    k = engine["k"]
    assert k == K, f"K 不对表 engine={k}"
    index = load_index()
    rows = []
    disagreements = []
    score_max_delta = 0.0
    for row in engine["queries"]:
        q = row["query"]
        hits, scores, _stats = semantic.semantic_channel(index, [q], k)
        engine_hits = [(h["id"], h["score"]) for h in row["hits"]]
        tool_hits = [(hid, scores[hid]) for hid in hits]
        ids_equal = [i for i, _ in engine_hits] == [i for i, _ in tool_hits]
        delta = 0.0
        if ids_equal and len(engine_hits) == len(tool_hits):
            for (eid, es), (_tid, ts) in zip(engine_hits, tool_hits):
                delta = max(delta, abs(es - ts))
        score_max_delta = max(score_max_delta, delta)
        agreed = ids_equal and delta <= SCORE_TOL
        if not agreed:
            disagreements.append(
                {
                    "query": q,
                    "engine": engine_hits,
                    "tool": tool_hits,
                    "ids_equal": ids_equal,
                    "score_delta": delta,
                }
            )
        rows.append(
            {
                "query": q,
                "engine_ids": [i for i, _ in engine_hits],
                "tool_ids": [i for i, _ in tool_hits],
                "ids_equal": ids_equal,
                "score_max_delta_this_query": delta,
                "verdict": "agree" if agreed else "disagree",
                "hit_type": row.get("hit_type", ""),
            }
        )
    total = len(rows)
    agreed_n = sum(1 for r in rows if r["verdict"] == "agree")
    summary = {
        "schema": "retrline-dualrun/1",
        "date": "2026-09-18",
        "engine_side": engine["engine_side"],
        "tool_side": "sih-tools/wikirecall semantic.semantic_channel (冻结只读 import)",
        "corpus": "sih/event/plan/retrline-materials/dualrun/corpus (十二件)",
        "criterion": "top-k 集合全等且逐位得分差 <= 1e-9",
        "k": K,
        "total_queries": total,
        "agreed": agreed_n,
        "disagreement_count": len(disagreements),
        "disagreements": disagreements,
        "score_max_delta": score_max_delta,
        "verdict": "consistent" if not disagreements else "inconsistent",
        "per_query": rows,
    }
    out = HERE / "dualrun-summary.json"
    out.write_text(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
                   encoding="utf-8")
    print(json.dumps({k2: summary[k2] for k2 in (
        "total_queries", "agreed", "disagreement_count", "score_max_delta", "verdict")},
        ensure_ascii=False))
    return 0 if not disagreements else 1


if __name__ == "__main__":
    sys.exit(main())
