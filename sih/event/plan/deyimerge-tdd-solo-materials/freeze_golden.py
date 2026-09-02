#!/usr/bin/env python3
"""freeze_golden.py — deyimerge-tdd-solo 金向量冻结驱动件。

以围堰 Python 原件为唯一基准（红线）：全部期望输出由 facet/contract_mode.py、
tally/cli.py、probes/maturation_gate.py 的原件跑出，本驱动零自造期望。

归一形（唯一放宽，双侧同参）：绝对路径位以 @ROOT@（工作区根）、@MATERIAL@
（材料路径实参串）、@TOPIC_ENTRY@（合同 meta.measurement_entry）三 token
落盘；引擎测试运行时以自身运行时路径反填后逐字节比对。活体双跑 cmp
（零归一零差）由 deyimerge-tdd-solo 沉降前证据脚本另行执行。

只读围堰：本脚本对 sih-tools 零写入。
"""
from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
FACET = ROOT / "sih-tools/facet"
WT = ROOT / "worktrees/sih-engine/deyimerge-tdd-solo"
GOLDEN = WT / "src/attractor/fixtures/golden"
ARENA = Path("/tmp/deyimerge-tdd-golden-arena")

sys.path.insert(0, str(FACET))
sys.path.insert(0, str(FACET / "src"))
sys.path.insert(0, str(FACET / "probes"))

import contract_mode  # noqa: E402  围堰原件
from lightweight_mode_probe import NG_STRENGTHS  # noqa: E402  围堰原件
from maturation_gate import assess_maturation_v3  # noqa: E402  围堰原件（只冻结用，不融回）


def sha256_file(p: Path) -> str:
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def tokenize(text: str, ws_root: Path) -> str:
    return text.replace(str(ws_root), "@ROOT@")


def write_expected(rel: str, data: bytes):
    out = GOLDEN / rel
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_bytes(data)
    print(f"frozen {rel} ({len(data)} bytes) sha256={hashlib.sha256(data).hexdigest()[:16]}")


def build_ws(name: str) -> Path:
    ws = ARENA / name
    if ws.exists():
        shutil.rmtree(ws)
    (ws / "input").mkdir(parents=True)
    return ws


def seed_input(ws: Path, src_dir: Path, mutate=None):
    for f in src_dir.iterdir():
        if f.name == "material.json":
            continue  # 材料由 base_material 现造，种子不带（防陈旧覆盖）
        shutil.copy(f, ws / "input" / f.name)
    if mutate:
        mutate(ws / "input")


def write_material(ws: Path, material: dict) -> Path:
    mp = ws / "material.json"
    mp.write_text(json.dumps(material, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
                  encoding="utf-8")
    return mp


def base_material(input_dir: Path, gate_verdict="stable_clear") -> dict:
    """从净目标输入目录拼 tally-check-input（同构自 adisp 原件，路径指冻结副本）。"""
    topic = input_dir / "topic.md"
    contract = input_dir / "contract.json"
    responses = input_dir / "responses.jsonl"
    trail = input_dir / "flywheel-trail.jsonl"
    baseline = input_dir / "seat-baseline.json"
    # dc_fingerprint 复算承 tally._load_dc_list + _fingerprint 原件逻辑
    dc_list = []
    for line in trail.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        r = json.loads(line)
        if r.get("trail_type") == "flywheel_run":
            dc = r.get("decision_convergence")
            if isinstance(dc, dict) and "per_actor" in dc:
                dc_list.append(dc)
    payload = json.dumps(dc_list, ensure_ascii=False, sort_keys=True).encode()
    fp = hashlib.sha256(payload).hexdigest()[:16]
    gid = json.loads(contract.read_text(encoding="utf-8"))["proposition"]["gid"]
    return {
        "kind": "tally-check-input", "gid": gid, "date": "2026-09-02",
        "topic_path": str(topic), "topic_sha256": sha256_file(topic),
        "trail_path": str(trail), "dc_fingerprint": fp,
        "gate_verdict": gate_verdict, "criteria_version": "v3",
        "contract_path": str(contract), "contract_sha256": sha256_file(contract),
        "responses_path": str(responses), "responses_sha256": sha256_file(responses),
        "n_shots": len([l for l in responses.read_text(encoding="utf-8").splitlines() if l.strip()]),
        "voids": [],
        "identity_hash": json.loads(baseline.read_text(encoding="utf-8"))["identity_hash"],
        "seat_baseline_path": str(baseline),
        "rules_version": "des-011-r1",
    }


def run_tally_check(material: Path):
    """围堰 tally CLI 原件跑 check，返回 (stdout_bytes, exit_code)。"""
    proc = subprocess.run(
        ["uv", "run", "tally", "check", "--material", str(material)],
        cwd=ROOT / "sih-tools/tally", capture_output=True, check=False)
    assert proc.returncode in (0, 1), f"tally check 异常退出 {proc.returncode}: {proc.stderr.decode()}"
    return proc.stdout, proc.returncode


def freeze_check_scenario(name: str, input_dir: Path, mutate=None):
    ws = build_ws(name)
    seed_input(ws, input_dir, mutate)
    material = base_material(ws / "input")
    if name == "dirty-alarm-r7":
        # 改写披露与链：topic 前言区补 rewrite_of，材料带四元 rewrite_chain 触发超三次告警
        material["rewrite_of"] = "m-earlier"
        material["rewrite_chain"] = ["m-a", "m-b", "m-c", "m-d"]
        t = ws / "input/topic.md"
        t.write_text(t.read_text(encoding="utf-8").replace(
            "gid: ", "rewrite_of: m-earlier\ngid: ", 1), encoding="utf-8")
        material["topic_sha256"] = sha256_file(t)
    if name == "dirty-return-r2":
        # 篡改 topic 内容不重算哈希 → R2 失败 → 材料退回
        t = ws / "input/topic.md"
        t.write_text(t.read_text(encoding="utf-8") + "\n被篡改行\n", encoding="utf-8")
    if name == "dirty-suspend-near":
        material["gate_verdict"] = "near_threshold"
    mp = write_material(ws, material)
    out, code = run_tally_check(mp)
    # signcheck 与 check 报告同字节（canonical + 尾换行）
    write_expected(f"{name}/expected/check-report.json", tokenize(out.decode("utf-8"), ws).encode("utf-8"))
    write_expected(f"{name}/expected/signcheck.json", tokenize(out.decode("utf-8"), ws).encode("utf-8"))
    (GOLDEN / name / "expected").mkdir(parents=True, exist_ok=True)
    (GOLDEN / name / "expected/check-exit.txt").write_text(str(code) + "\n", encoding="utf-8")
    # 冻结带 token 的材料与输入种子（测试重建工作区用）
    write_expected(f"{name}/input/material.json", tokenize(mp.read_text(encoding="utf-8"), ws).encode("utf-8"))
    for f in sorted((ws / "input").iterdir()):
        if f.name == "material.json":
            continue  # 材料已由带 token 的期望件承载，种子位不回拷
        shutil.copy(f, GOLDEN / name / "input" / f.name)
    report = json.loads(out)
    print(f"  {name}: exit={code} disposition={report['disposition']} verdict={report['verdict']} alarms={len(report['alarms'])}")
    return ws, material


def freeze_reject(name: str, mutate):
    """合同类拒收基线：围堰 contract_mode.load_responses 原件拒收消息冻结。"""
    ws = build_ws(name)
    src = GOLDEN / "adisp-net" / "input"
    shutil.copy(src / "contract.json", ws / "input" / "contract.json")
    shutil.copy(src / "responses.jsonl", ws / "input" / "responses.jsonl")
    mutate(ws / "input")
    responses_path = ws / "input" / "responses.jsonl"
    contract = contract_mode.load_contract(ws / "input" / "contract.json")
    try:
        contract_mode.load_responses(responses_path, contract)
        raise SystemExit(f"{name}: 围堰未拒收，构造失效")
    except ValueError as exc:
        envelope = json.dumps({"error": f"ValueError: {exc}"}, ensure_ascii=False) + "\n"
        write_expected(f"{name}/expected/reject.json", envelope.encode("utf-8"))
    # 冻结构造后的响应与合同种子
    (GOLDEN / name / "input").mkdir(parents=True, exist_ok=True)
    for f in sorted((ws / "input").iterdir()):
        shutil.copy(f, GOLDEN / name / "input" / f.name)
    print(f"  {name}: rejected message frozen")


def freeze_score(gid: str, input_dir: Path):
    """计分材料金向量：新鲜 trail 工作区跑围堰计分管线（闸 v3 由围堰原件判，仅冻结用）。"""
    ws = build_ws(f"score-{gid}")
    shutil.copy(input_dir / "contract.json", ws / "contract.json")
    shutil.copy(input_dir / "responses.jsonl", ws / "responses.jsonl")
    contract_path = ws / "contract.json"
    responses_path = ws / "responses.jsonl"
    trail_path = ws / "flywheel-trail.jsonl"
    contract = contract_mode.load_contract(contract_path)
    responses = contract_mode.load_responses(responses_path, contract)
    per_actor, voids = contract_mode.dc_from_responses(responses, contract["seat"])
    written = contract_mode.append_contract_runs(
        trail_path, gid, responses, per_actor, contract["seat"],
        contract["pack"]["ng_label"])
    dc_list = []
    for line in trail_path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        r = json.loads(line)
        if r.get("trail_type") == "flywheel_run":
            dc = r.get("decision_convergence")
            if isinstance(dc, dict) and "per_actor" in dc:
                dc_list.append(dc)
    v3 = assess_maturation_v3(dc_list)
    material_path = ws / "score-material.json"
    contract_mode.write_score_material(
        material_path, contract_path=contract_path, responses_path=responses_path,
        gid=gid, n_shots=len(responses), voids=voids,
        identity_hash=json.loads((GOLDEN / gid if False else input_dir / "seat-baseline.json").read_text(encoding="utf-8"))["identity_hash"],
        extra={"trail_path": str(trail_path), "runs_written": len(written),
               "gate_verdict": v3["verdict_v3"]})
    write_expected(f"{gid}-net/expected/score-material.json",
                   tokenize(material_path.read_text(encoding="utf-8"), ws).encode("utf-8"))
    print(f"  score-{gid}: gate_verdict={v3['verdict_v3']} runs_written={len(written)} voids={len(voids)}")


def freeze_contract_emit():
    """合同 emit 金向量：围堰原件重 emit 与真实存量合同逐字节比对后冻结真实字节。"""
    ws = build_ws("emit-adisp")
    src = GOLDEN / "adisp-net" / "input"
    topic_src = src / "topic.md"
    ng_seen = contract_mode.scheme_clipped_ng(NG_STRENGTHS["medium"])
    prompts = contract_mode.build_integrator_prompts(topic_src.read_text(encoding="utf-8"), ng_seen)
    shot_list = contract_mode.make_shots(prompts, "adisp-guard-1", 9)
    out_path = ws / "re-emit-contract.json"
    contract_mode.emit_contract(
        out_path,
        seat={"framework": "ZCode", "model_id": "GLM-5.3-Flash", "version": "self-reported"},
        pack={"paradigm_id": "normative_convergence", "atom": "integrator",
              "direction": "judge", "ng_label": "medium",
              "ng_text_sha256": contract_mode.sha256_bytes(ng_seen.encode("utf-8"))},
        proposition={"gid": "adisp-guard-1",
                     "title": "拒直提守卫命题：lease 应增 pre-commit 守卫拦无模板 plain git commit，--no-verify 显式留痕放行",
                     "topic_sha256": contract_mode.sha256_file(topic_src)},
        shots=shot_list,
        meta={"measurement_entry": "facet_task_packages/adisp-guard-1/topic.md", "n_declared": 9})
    real = (src / "contract.json").read_bytes()
    reemitted = out_path.read_bytes()
    assert real == reemitted, (
        "围堰重 emit 与真实存量合同不逐字节一致：\n"
        f"real sha={hashlib.sha256(real).hexdigest()[:16]} re sha={hashlib.sha256(reemitted).hexdigest()[:16]}")
    print(f"  emit-adisp: 围堰重 emit 与真实合同逐字节一致 ({len(real)} bytes)")
    write_expected("adisp-net/expected/contract.json",
                   real.replace(b"facet_task_packages/adisp-guard-1/topic.md", b"@TOPIC_ENTRY@"))


def freeze_stats():
    """统计面参照值：围堰原件跑固定输入电池，浮点文本形豁免判据（结论等值+容差比对）。"""
    import facet_stats as fs
    battery = {
        "hellinger": [
            {"p": [0.25, 0.75], "q": [0.25, 0.75]},
            {"p": [1, 0], "q": [0, 1]},
            {"p": [60, 0, 0], "q": [58, 2, 0]},
            {"p": [3, 7], "q": [4, 6]},
        ],
        "tv": [
            {"p": [3, 7], "q": [3, 7]},
            {"p": [1, 0], "q": [0, 1]},
            {"p": [60, 0, 0], "q": [58, 2, 0]},
            {"p": [1, 0], "q": [0.5, 0.5]},
        ],
        "js": [
            {"p": [0.2, 0.8], "q": [0.2, 0.8]},
            {"p": [1, 0], "q": [0, 1]},
            {"p": [1, 0], "q": [0.5, 0.5]},
            {"p": [10, 20, 30], "q": [30, 20, 10]},
        ],
        "jaccard": [
            {"p": [3, 5, 7], "q": [3, 5, 7]},
            {"p": [3, 0], "q": [0, 5]},
            {"p": [60, 0, 0], "q": [58, 2, 0]},
            {"p": [1, 1], "q": [1, 0]},
        ],
        "binom": [
            {"k": 60, "n": 60, "p": 0.5}, {"k": 0, "n": 60, "p": 0.5},
            {"k": 3, "n": 10, "p": 0.5}, {"k": 2, "n": 100, "p": 0.01},
        ],
        "bonferroni": [{"ps": [0.01, 0.04], "alpha": 0.05}, {"ps": [0.7, 0.8], "alpha": 0.05}],
        "bh": [{"ps": [0.001, 0.008, 0.039, 0.041, 0.042]}, {"ps": [0.4, 0.01]}],
        "power": [
            {"n": 60, "p0": 0.5, "p1": 0.8, "alpha": 0.05},
            {"n": 10, "p0": 0.5, "p1": 0.9, "alpha": 0.05},
        ],
        "anova": [{"data": {
            ("a1", "b1"): [1.0, 2.0, 3.0], ("a1", "b2"): [4.0, 5.0, 6.0],
            ("a2", "b1"): [7.0, 8.0, 9.0], ("a2", "b2"): [10.0, 11.0, 12.0]},
            "names": ["factor_a", "factor_b"]}],
    }
    values = {
        "hellinger": [fs.hellinger_distance(**c) for c in battery["hellinger"]],
        "tv": [fs.tv_distance(**c) for c in battery["tv"]],
        "js": [fs.js_divergence(**c) for c in battery["js"]],
        "jaccard": [fs.generalized_jaccard(**c) for c in battery["jaccard"]],
        "binom": [fs.binomial_test_pvalue(c["k"], c["n"], c["p"]) for c in battery["binom"]],
        "bonferroni": [fs.bonferroni_correction(c["ps"], c["alpha"]) for c in battery["bonferroni"]],
        "bh": [fs.bh_fdr(c["ps"]) for c in battery["bh"]],
        "power": [fs.statistical_power(c["n"], c["p0"], c["p1"], c["alpha"]) for c in battery["power"]],
        "anova": [fs.mixed_effects_anova(c["data"], tuple(c["names"])) for c in battery["anova"]],
        "conclusion": {
            "perm_strong_overlap_p_lt_0.01": None,
        },
    }
    import facet_stats_conv as conv
    factories = {f"f{i}": {f"p{j}" for j in range(8)} for i in range(3)}
    universe = [f"p{j}" for j in range(10)]
    p = conv.permutation_test_shared_convergence(factories, universe=universe, n_perm=2000, seed=7)
    values["conclusion"]["perm_strong_overlap_p_lt_0.01"] = p
    p2 = conv.permutation_test_shared_convergence({"a": {"x"}}, universe=["x", "y"], n_perm=500, seed=1)
    values["conclusion"]["perm_zero_intersect_p"] = p2
    out = GOLDEN / "stats-values.json"
    out.write_text(json.dumps(values, ensure_ascii=False, sort_keys=True, indent=2) + "\n", encoding="utf-8")
    print(f"frozen stats-values.json")


def main():
    ARENA.mkdir(parents=True, exist_ok=True)
    print("== 净目标 ==")
    freeze_check_scenario("adisp-net", GOLDEN / "adisp-net" / "input")
    freeze_check_scenario("p3xcarr-net", GOLDEN / "p3xcarr-net" / "input")
    print("== 脏目标三形 ==")
    # 脏形派生自 adisp 净目标种子（先种净种子再变异）
    freeze_check_scenario("dirty-suspend-near", GOLDEN / "adisp-net" / "input")
    freeze_check_scenario("dirty-return-r2", GOLDEN / "adisp-net" / "input")
    freeze_check_scenario("dirty-alarm-r7", GOLDEN / "adisp-net" / "input")
    print("== 合同类拒收基线四形 ==")
    freeze_reject("reject-missing-shot",
                  lambda d: (lambda lines: d.joinpath("responses.jsonl").write_text(
                      "".join(lines[:-1]), encoding="utf-8"))(
                      d.joinpath("responses.jsonl").read_text(encoding="utf-8").splitlines(keepends=True)))
    def _key_mismatch(d):
        p = d / "responses.jsonl"
        lines = p.read_text(encoding="utf-8").splitlines(keepends=True)
        obj = json.loads(lines[4]); obj["key"] = "bogus#r5"
        lines[4] = json.dumps(obj, ensure_ascii=False) + "\n"
        p.write_text("".join(lines), encoding="utf-8")
    freeze_reject("reject-key-mismatch", _key_mismatch)
    def _shot_misalign(d):
        p = d / "responses.jsonl"
        lines = p.read_text(encoding="utf-8").splitlines(keepends=True)
        obj = json.loads(lines[2]); obj["shot"] = 4
        lines[2] = json.dumps(obj, ensure_ascii=False) + "\n"
        p.write_text("".join(lines), encoding="utf-8")
    freeze_reject("reject-shot-misalign", _shot_misalign)
    def _raw_empty(d):
        p = d / "responses.jsonl"
        lines = p.read_text(encoding="utf-8").splitlines(keepends=True)
        obj = json.loads(lines[1]); obj["raw"] = ""
        lines[1] = json.dumps(obj, ensure_ascii=False) + "\n"
        p.write_text("".join(lines), encoding="utf-8")
    freeze_reject("reject-raw-empty", _raw_empty)
    print("== 计分材料金向量 ==")
    freeze_score("adisp-guard-1", GOLDEN / "adisp-net" / "input")
    freeze_score("m-p3xcarr", GOLDEN / "p3xcarr-net" / "input")
    print("== 合同 emit 金向量 ==")
    freeze_contract_emit()
    print("== 统计面参照值 ==")
    freeze_stats()
    print("DONE")


if __name__ == "__main__":
    main()
