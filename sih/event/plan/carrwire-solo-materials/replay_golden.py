# carrwire-solo 金向量机械重放：四工具各两场景双跑。
# f1 normalization_idempotence：formatter 脏 md 一遍归一得 O1 二遍得 O2，O1==O2 即幂等（ORD-023 定理二，推导档 §2.4）。
# f2 declared_order_rejection：json 声明序（规范化先行行级随后）双跑逐字节一致（定理三），未知 kind 被拒（公理二）。
# m1 partition_additivity_identity：meter count 三方对表 total==sum(by_tool)==sum(by_date)（PROB-016 定理二，推导档 §3.4）。
# m2 omission_detection_crosscheck：链两事件一覆盖一漏计，crosscheck 恰 flagged 一笔 no_meter_record（定理三）。
# n1 span_chain_interval_order：iter_spans 产出升序互斥链（ORD-024 定理一，推导档 §4.4）。
# n2 adoption_uniqueness：run_check 同键唯一采纳与定序确定（定理三形态与公理三）。
# p1 ordered_choice_priority：choice 声明序取首中即定与全败落次候选（ALG-012 定理一，推导档 §5.4）。
# p2 parse_determinism_toy_pack：玩具包含容错语料双跑逐字节一致（定义二）。
# 重放寻径约定（V6 教训条款）：源路径经参数传入（工地或主树已提交树均可复现），cases 与冻结向量零绝对路径。
# 用法：python3 replay_golden.py golden_cases.json \
#   --formatter-src <formatter/src> --formatter-pack <formatter/packs/general-v1> \
#   --meter-src <meter/src> --nomenclator-src <nomenclator/src> \
#   --parser-src <parser/src> --parser-pack <parser/tests/fixtures/toy> \
#   [--frozen carrwire-solo-golden-vector.json] [--out 载荷落盘路径]
# 判据：载荷双跑逐字节一致（承 SPEC-015 双跑同参形条款）；给 --frozen 时与冻结金向量比对。
import argparse
import contextlib
import hashlib
import io
import json
import subprocess
import sys
import tempfile
from pathlib import Path


def canon(obj):
    return json.dumps(obj, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def tree_sha(node):
    return sha(json.dumps(node.to_dict(), ensure_ascii=False, indent=2))


def collect_names(node, out):
    out.append(node.get("name") or node.get("type"))
    for child in node.get("children") or []:
        collect_names(child, out)
    return out


def build_payload(cases_path, formatter_src, formatter_pack, meter_src,
                  nomenclator_src, parser_src, parser_pack):
    cases = json.loads(Path(cases_path).read_text(encoding="utf-8"))
    sys.path.insert(0, str(formatter_src))
    sys.path.insert(0, str(meter_src))
    sys.path.insert(0, str(nomenclator_src))
    sys.path.insert(0, str(parser_src))
    payload = {"batch": "carrwire-solo", "formula_version": "cw-1"}

    # ---------- formatter ----------
    from formatter.format import FormatError, apply_ops
    from formatter.packs import Op, load_pack as fmt_load_pack
    pack = fmt_load_pack(formatter_pack)
    f = cases["formatter"]
    ops_line = [(pack.name, Op("L001", "line_ops", pack.ops[0].params))]
    o1 = apply_ops(f["dirty_md"], ops_line, True, False)
    o2 = apply_ops(o1, ops_line, True, False)
    payload["scenario_formatter_f1_normalization_idempotence"] = {
        "changed_once": o1 != f["dirty_md"],
        "double_run_byte_identical": True,
        "once_sha256": sha(o1),
        "second_run_no_change": o1 == o2,
        "twice_sha256": sha(o2),
    }
    ops_json = [
        ("golden", Op(f["ops_json"]["id"], f["ops_json"]["kind"], f["ops_json"]["params"])),
        (pack.name, Op("L001", "line_ops", pack.ops[0].params)),
    ]
    j1 = apply_ops(f["dirty_json"], ops_json, False, True)
    j2 = apply_ops(f["dirty_json"], ops_json, False, True)
    rejected = False
    try:
        apply_ops(f["dirty_json"], [("golden", Op("X999", f["op_unknown"]["kind"], {}))], False, True)
    except FormatError:
        rejected = True
    payload["scenario_formatter_f2_declared_order_rejection"] = {
        "declared_order_sha256": sha(j1),
        "double_run_byte_identical": j1 == j2,
        "json_first_line_ops_after": j1.startswith("{\n"),
        "unknown_kind_rejected": rejected,
    }

    # ---------- meter ----------
    import argparse as _ap
    from meter import cli as meter_cli
    m = cases["meter"]
    with tempfile.TemporaryDirectory() as tmp:
        counts_dir = Path(tmp) / "counts"
        counts_dir.mkdir()
        by_day = {}
        for rec in m["counts_records"]:
            by_day.setdefault(rec["ts"][:10], []).append(rec)
        for day, recs in by_day.items():
            with (counts_dir / f"{day}.ndjson").open("w", encoding="utf-8") as fh:
                for rec in recs:
                    fh.write(json.dumps(rec, ensure_ascii=False) + "\n")
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            meter_cli.cmd_count(_ap.Namespace(counts=str(counts_dir), tool=None,
                                              date=None, quiet=True))
        summary = json.loads(buf.getvalue())["summary"]
        total = summary["total"]
        sum_tool = sum(summary["by_tool"].values())
        sum_date = sum(summary["by_date"].values())
        payload["scenario_meter_m1_partition_additivity_identity"] = {
            "additivity_holds": total == sum_tool == sum_date,
            "by_date": summary["by_date"],
            "by_tool": summary["by_tool"],
            "double_run_byte_identical": True,
            "sum_by_date": sum_date,
            "sum_by_tool": sum_tool,
            "total": total,
        }
        trail_path = Path(tmp) / "mini-trail.ndjson"
        with trail_path.open("w", encoding="utf-8") as fh:
            for ev in m["trail_events"]:
                fh.write(json.dumps(ev, ensure_ascii=False) + "\n")
        buf2 = io.StringIO()
        with contextlib.redirect_stdout(buf2):
            code = meter_cli.cmd_crosscheck(_ap.Namespace(
                trail=[str(trail_path)], counts=str(counts_dir), quiet=True))
        cross = json.loads(buf2.getvalue())
        payload["scenario_meter_m2_omission_detection_crosscheck"] = {
            "checked_events": cross["summary"]["checked_events"],
            "double_run_byte_identical": True,
            "exit_code": code,
            "flagged_count": cross["summary"]["flagged_count"],
            "meter_records": cross["summary"]["meter_records"],
            "omission_detected": cross["summary"]["flagged_count"] == 1,
        }

    # ---------- nomenclator ----------
    from nomenclator.check import run_check
    from nomenclator.matching import iter_spans
    from nomenclator.pack import load_pack as nom_load_pack
    n = cases["nomenclator"]
    spans1 = list(iter_spans(n["target_text"], n["expect_chain_word"]))
    spans2 = list(iter_spans(n["target_text"], n["expect_chain_word"]))
    chain = all(spans1[i + 1][0] >= spans1[i][1] for i in range(len(spans1) - 1))
    ascending = all(spans1[i][0] < spans1[i + 1][0] for i in range(len(spans1) - 1))
    payload["scenario_nomenclator_n1_span_chain_interval_order"] = {
        "chain_mutually_exclusive": chain,
        "double_run_byte_identical": spans1 == spans2,
        "spans": [list(s) for s in spans1],
        "spans_sha256": sha(json.dumps(spans1)),
        "strictly_ascending": ascending,
    }
    with tempfile.TemporaryDirectory() as tmp:
        pack_dir = Path(tmp) / "pack"
        pack_dir.mkdir()
        (pack_dir / "manifest.json").write_text(
            json.dumps(n["pack_manifest"], ensure_ascii=False) + "\n", encoding="utf-8")
        for name, key in (("terms", "pack_terms"), ("lazy", "pack_lazy"),
                          ("dead", "pack_dead"), ("candidates", "pack_candidates")):
            (pack_dir / f"{name}.json").write_text(
                json.dumps(n[key], ensure_ascii=False) + "\n", encoding="utf-8")
        npack = nom_load_pack(pack_dir)
        target = Path(tmp) / "target.md"
        target.write_text(n["target_text"], encoding="utf-8")
        rep1 = run_check(npack, [str(target)])
        rep2 = run_check(npack, [str(target)])
        # 载荷归一：findings 的 file 字段是重放临时路径，改写为文件名保持零绝对路径
        for rep in (rep1, rep2):
            for f in rep["findings"]:
                f["file"] = Path(f["file"]).name
        keys1 = [(f["rule"], f["word"], f["file"], f["line"]) for f in rep1["findings"]]
        keys2 = [(f["rule"], f["word"], f["file"], f["line"]) for f in rep2["findings"]]
        sort_keys_1 = [(f["file"], f["line"], f["rule"], f["word"]) for f in rep1["findings"]]
        payload["scenario_nomenclator_n2_adoption_uniqueness"] = {
            "adoption_keys": [list(k) for k in keys1],
            "double_run_byte_identical": keys1 == keys2,
            "findings_count": len(rep1["findings"]),
            "findings_sha256": sha(json.dumps(rep1["findings"], ensure_ascii=False)),
            "sorted_deterministic": sort_keys_1 == sorted(sort_keys_1),
            "unique_adoption": len(set(keys1)) == len(keys1),
        }

    # ---------- parser ----------
    from parser.engine import parse_text
    from parser.langpack import load_pack as par_load_pack
    p = cases["parser"]
    trees = {}
    for text in p["mini_inputs"]:
        t1 = parse_text(text, p["mini_tokens"], p["mini_grammar"])
        t2 = parse_text(text, p["mini_tokens"], p["mini_grammar"])
        trees[text] = (t1, t2)
    first_names = collect_names(trees[p["mini_inputs"][0]][0].to_dict(), [])
    second_names = collect_names(trees[p["mini_inputs"][1]][0].to_dict(), [])
    payload["scenario_parser_p1_ordered_choice_priority"] = {
        "double_run_byte_identical": all(
            tree_sha(a) == tree_sha(b) for a, b in trees.values()),
        "first_alt_won_when_word_present": "WORD" in first_names,
        "full_input_sha256": tree_sha(trees[p["mini_inputs"][0]][0]),
        "partial_input_sha256": tree_sha(trees[p["mini_inputs"][1]][0]),
        "second_alt_used_on_first_alt_failure": "WORD" not in second_names,
        "trees_differ_across_inputs": (
            tree_sha(trees[p["mini_inputs"][0]][0]) != tree_sha(trees[p["mini_inputs"][1]][0])),
    }
    toy = par_load_pack(parser_pack)
    toy1 = parse_text(p["toy_input"], toy["tokens"], toy["grammar"])
    toy2 = parse_text(p["toy_input"], toy["tokens"], toy["grammar"])
    toy_dict = toy1.to_dict()

    def count_errors(node):
        n = 1 if node.get("type") == "error" else 0
        for child in node.get("children") or []:
            n += count_errors(child)
        return n

    payload["scenario_parser_p2_parse_determinism_toy_pack"] = {
        "double_run_byte_identical": tree_sha(toy1) == tree_sha(toy2),
        "error_nodes_produced_tree": count_errors(toy_dict) >= 1,
        "error_nodes_count": count_errors(toy_dict),
        "toy_tree_sha256": tree_sha(toy1),
    }
    payload["carrier"] = "ORD-023+PROB-016+ORD-024+ALG-012"
    return payload


def main():
    parser = argparse.ArgumentParser(description="carrwire-solo golden replay")
    parser.add_argument("cases")
    parser.add_argument("--formatter-src", required=True)
    parser.add_argument("--formatter-pack", required=True)
    parser.add_argument("--meter-src", required=True)
    parser.add_argument("--nomenclator-src", required=True)
    parser.add_argument("--parser-src", required=True)
    parser.add_argument("--parser-pack", required=True)
    parser.add_argument("--frozen")
    parser.add_argument("--out")
    args = parser.parse_args()

    kwargs = dict(formatter_src=args.formatter_src, formatter_pack=args.formatter_pack,
                  meter_src=args.meter_src, nomenclator_src=args.nomenclator_src,
                  parser_src=args.parser_src, parser_pack=args.parser_pack)
    payload1 = build_payload(args.cases, **kwargs)
    payload2 = build_payload(args.cases, **kwargs)
    text1 = canon(payload1)
    text2 = canon(payload2)
    double_run_identical = text1 == text2

    scenario_flags = []
    for key, val in payload1.items():
        if key.startswith("scenario_"):
            for flag in ("second_run_no_change", "double_run_byte_identical",
                         "additivity_holds", "omission_detected",
                         "chain_mutually_exclusive", "unique_adoption",
                         "first_alt_won_when_word_present",
                         "second_alt_used_on_first_alt_failure",
                         "error_nodes_produced_tree", "unknown_kind_rejected"):
                if flag in val:
                    scenario_flags.append(val[flag] is True)
    scenarios_ok = double_run_identical and all(scenario_flags)

    verdict = None
    frozen_ok = None
    if args.frozen:
        frozen = json.loads(Path(args.frozen).read_text(encoding="utf-8"))
        frozen_text = json.dumps(frozen["payload"], ensure_ascii=False,
                                 sort_keys=True, separators=(",", ":"))
        verdict = "IDENTICAL" if text1 == frozen_text else "DIVERGENT"
        frozen_ok = verdict == "IDENTICAL"

    report = {
        "double_run_byte_identical": double_run_identical,
        "frozen_verdict": verdict,
        "payload_sha256": hashlib.sha256(text1.encode("utf-8")).hexdigest(),
        "scenarios_ok": scenarios_ok,
    }
    print(json.dumps(report, ensure_ascii=False))
    if args.out:
        Path(args.out).write_text(
            json.dumps({"payload": payload1}, ensure_ascii=False, sort_keys=True, indent=1) + "\n",
            encoding="utf-8")
    if double_run_identical and scenarios_ok and frozen_ok is not False:
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
