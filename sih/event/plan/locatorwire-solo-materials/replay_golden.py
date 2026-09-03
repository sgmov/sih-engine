# locatorwire-solo 金向量机械重放：locator 寻址派生与陈旧判定两场景双跑。
# 场景一 addressing_derivation_determinism：三载体语料建索引双跑逐字节一致（快照复现，推导档 §3.1/§3.3），
#   并以 identity.stable_id 对首个条目独立重算登记号与索引条目 id 对表（一致命名）。
# 场景二 stale_detection_version_comparability：文件级三态对表（stale 一件 missing 两件，推导档 §3.2）
#   加头部包版本判定（reason=pack 且文件级零 stale，推导档 §3.3）。
# 重放寻径约定（V6 教训条款）：locator 源路径经 --locator-src 参数传入（工地或主树已提交树均可复现），
#   语料与包定义内联于本 cases 件，cases 与冻结向量零绝对路径。
# 用法：python3 replay_golden.py golden_cases.json --locator-src <locator/src 路径> \
#           [--frozen locatorwire-solo-golden-vector.json] [--out 计算载荷落盘路径]
# 判据：载荷双跑逐字节一致（承 SPEC-015 双跑同参形条款）；给 --frozen 时与冻结金向量比对。
import argparse
import hashlib
import json
import shutil
import sys
import tempfile
from pathlib import Path


def canon(obj):
    return json.dumps(obj, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def payload_from(cases_path, locator_src):
    sys.path.insert(0, str(locator_src))
    from locator.builder import build
    from locator.identity import stable_id
    from locator.stale import run_stale

    cases = json.loads(Path(cases_path).read_text(encoding="utf-8"))
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        corpus = tmp / "corpus"
        corpus.mkdir()
        for rel, content in cases["corpus"].items():
            target = corpus / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(content, encoding="utf-8")
        pack_dir = tmp / "pack"
        pack_dir.mkdir()
        (pack_dir / "pack.json").write_text(
            json.dumps(cases["pack"], ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
        idx1 = tmp / "i1.ndjson"
        idx2 = tmp / "i2.ndjson"
        summary1 = build(pack_dir, corpus, idx1)
        build(pack_dir, corpus, idx2)
        idx1_bytes = idx1.read_bytes()
        idx2_bytes = idx2.read_bytes()
        double_run = idx1_bytes == idx2_bytes

        first_entry = None
        for ln in idx1.read_text(encoding="utf-8").splitlines():
            rec = json.loads(ln)
            if rec.get("type") == "entry":
                first_entry = rec
                break
        recomputed = stable_id(
            first_entry["path"], first_entry["carrier"], first_entry["kind"],
            first_entry["seq"], first_entry["content_hash"])

        work = tmp / "work"
        shutil.copytree(corpus, work)
        for rel, content in cases["mutations"]["mutate"].items():
            (work / rel).write_text(content, encoding="utf-8")
        for rel in cases["mutations"]["delete"]:
            (work / rel).unlink()
        for rel, content in cases["mutations"]["add"].items():
            (work / rel).write_text(content, encoding="utf-8")
        rep_file = run_stale(pack_dir, work, idx1)

        pack_v2_dir = tmp / "pack_v2"
        pack_v2_dir.mkdir()
        (pack_v2_dir / "pack.json").write_text(
            json.dumps(cases["pack_v2"], ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
        rep_head = run_stale(pack_v2_dir, corpus, idx1)

        exp = cases["expectations"]["scenario_2_stale_detection_version_comparability"]
        payload = {
            "build_summary": summary1,
            "carrier": "ORD-019",
            "double_run_byte_identical": double_run,
            "formula_version": "lw-1",
            "index_sha256": hashlib.sha256(idx1_bytes).hexdigest(),
            "scenario_1_addressing_derivation_determinism": {
                "build_files": summary1["files"],
                "double_run_byte_identical": double_run,
                "entry_kind": first_entry["kind"],
                "entry_path": first_entry["path"],
                "entry_seq": first_entry["seq"],
                "first_entry_id": first_entry["id"],
                "id_matches_independent_recomputation": recomputed == first_entry["id"],
                "parse_errors_empty": summary1["parse_errors"] == [],
                "recomputed_id": recomputed,
            },
            "scenario_2_stale_detection_version_comparability": {
                "file_level": {
                    "fresh": sorted(rep_file["files"]["fresh"]),
                    "missing": sorted(rep_file["files"]["missing"]),
                    "over_threshold": rep_file["over_threshold"],
                    "reason": rep_file["reason"],
                    "rebuild_required": rep_file["rebuild_required"],
                    "stale": sorted(rep_file["files"]["stale"]),
                },
                "head_version_level": {
                    "fresh": sorted(rep_head["files"]["fresh"]),
                    "missing": sorted(rep_head["files"]["missing"]),
                    "over_threshold": rep_head["over_threshold"],
                    "reason": rep_head["reason"],
                    "rebuild_required": rep_head["rebuild_required"],
                    "stale": sorted(rep_head["files"]["stale"]),
                },
                "matches_expectations": (
                    sorted(rep_file["files"]["stale"]) == sorted(exp["file_level"]["stale"])
                    and sorted(rep_file["files"]["missing"]) == sorted(exp["file_level"]["missing"])
                    and sorted(rep_file["files"]["fresh"]) == sorted(exp["file_level"]["fresh"])
                    and rep_file["rebuild_required"] == exp["file_level"]["rebuild_required"]
                    and rep_file["over_threshold"] == exp["file_level"]["over_threshold"]
                    and rep_file["reason"] == exp["file_level"]["reason"]
                    and sorted(rep_head["files"]["stale"]) == []
                    and sorted(rep_head["files"]["missing"]) == []
                    and rep_head["rebuild_required"] == exp["head_version_level"]["rebuild_required"]
                    and rep_head["reason"] == exp["head_version_level"]["reason"]
                ),
            },
        }
    return payload


def main():
    parser = argparse.ArgumentParser(description="locatorwire-solo golden replay")
    parser.add_argument("cases")
    parser.add_argument("--locator-src", required=True)
    parser.add_argument("--frozen")
    parser.add_argument("--out")
    args = parser.parse_args()

    payload1 = payload_from(args.cases, args.locator_src)
    payload2 = payload_from(args.cases, args.locator_src)
    text1 = canon(payload1)
    text2 = canon(payload2)
    double_run_identical = text1 == text2

    scenarios_ok = (
        payload1["double_run_byte_identical"] is True
        and payload1["scenario_1_addressing_derivation_determinism"]["double_run_byte_identical"] is True
        and payload1["scenario_1_addressing_derivation_determinism"]["id_matches_independent_recomputation"] is True
        and payload1["scenario_1_addressing_derivation_determinism"]["parse_errors_empty"] is True
        and payload1["scenario_2_stale_detection_version_comparability"]["matches_expectations"] is True
    )

    verdict = None
    frozen_ok = None
    if args.frozen:
        frozen = json.loads(Path(args.frozen).read_text(encoding="utf-8"))
        frozen_text = json.dumps(frozen["payload"], ensure_ascii=False, sort_keys=True, separators=(",", ":"))
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
