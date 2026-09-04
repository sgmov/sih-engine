"""pk050sw-solo 确定性七项核对跑件：逐项机械取证据，全过出 verdict pass。

零模型零网络零 LLM。同输入逐字节同输出（S4 双跑为其中一项判定）。
用法：
  python3 switch_gate.py --wikirecall <施工面 wikirecall 目录> \
    --tools-wt <sih-tools 工地根> --math <sih-math 主树路径> \
    --trail <当日链 ndjson> --prereg <ab-criteria-preregistration.json> \
    --readings <ab-readings.json> --out <清单输出.json>

七项：
  S1 预登记 A/B pass——读数件 verdict==pass 且三判据全真，双件 sha256 在案
  S2 semantic.py 仅 stdlib 零 LLM——AST import 集 ⊆ stdlib 白名单，禁词扫描零命中
  S3 零新增依赖——施工面 wikirecall diff 文件集 ⊆ 允许集，无依赖清单新增
  S4 双跑逐字节一致——实仓两查询缺省计划双跑 canonical sha256 相等
  S5 载体终签在链——crosscheck 6b3723e8（PROB-017）与 70da4959（ALG-011）在链
  S6 判变登记在链——parking_entered 0b3da7ec doc_id pk-050 在链
  S7 词面回退位设计在位——--word 对切前 HEAD 版缺省计划夹具逐字节一致，
     且 CLI 无旗标缺省出语义通道
"""

import argparse
import ast
import hashlib
import json
import pathlib
import subprocess
import sys
import tempfile

STDLIB_OK = {"argparse", "json", "pathlib", "re", "sys", "math", "semantic"}
FORBIDDEN_TOKENS = [
    "openai", "anthropic", "requests", "urllib", "socket", "subprocess",
    "embedding", "torch", "numpy", "sklearn", "faiss",
]
DIFF_ALLOWED_PREFIX = ("wikirecall/recall.py", "wikirecall/semantic_selftest.py",
                       "wikirecall/fixtures/golden/", "wikirecall/CALL-LOG.md")


def sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def canonical(obj):
    return json.dumps(obj, ensure_ascii=False, sort_keys=True)


def s1_ab_pass(prereg_path, readings_path):
    prereg = json.loads(pathlib.Path(prereg_path).read_text(encoding="utf-8"))
    readings = json.loads(pathlib.Path(readings_path).read_text(encoding="utf-8"))
    ok = (
        prereg.get("kind") == "ab-gate-criteria-preregistration"
        and readings.get("verdict") == "pass"
        and all(readings.get("criteria", {}).values())
        and readings.get("k") == prereg.get("paths", {}).get("K")
    )
    return ok, {
        "prereg_sha256": sha256_bytes(pathlib.Path(prereg_path).read_bytes()),
        "readings_sha256": sha256_bytes(pathlib.Path(readings_path).read_bytes()),
        "verdict": readings.get("verdict"),
        "criteria": readings.get("criteria"),
        "k": readings.get("k"),
        "delta_ig_bits": readings.get("aggregate", {}).get("delta_ig_bits"),
    }


def s2_stdlib_only(wikirecall_dir):
    ev = {"files": {}, "forbidden_hits": []}
    ok = True
    for name in ("semantic.py", "recall.py"):
        src = (wikirecall_dir / name).read_text(encoding="utf-8")
        tree = ast.parse(src)
        imports = set()
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                imports.update(a.name.split(".")[0] for a in node.names)
            elif isinstance(node, ast.ImportFrom) and node.module:
                imports.add(node.module.split(".")[0])
        bad = imports - STDLIB_OK
        ev["files"][name] = {"imports": sorted(imports), "out_of_allow": sorted(bad)}
        if bad:
            ok = False
        low = src.lower()
        for tok in FORBIDDEN_TOKENS:
            if tok in low:
                ev["forbidden_hits"].append({"file": name, "token": tok})
                ok = False
    return ok, ev


def s3_no_new_deps(tools_wt):
    out = subprocess.run(
        ["git", "-C", str(tools_wt), "diff", "--name-only", "HEAD", "--", "wikirecall"],
        capture_output=True, check=True,
    ).stdout.decode("utf-8")
    files = [ln for ln in out.splitlines() if ln.strip()]
    outside = [f for f in files if not f.startswith(DIFF_ALLOWED_PREFIX)]
    untracked = subprocess.run(
        ["git", "-C", str(tools_wt), "ls-files", "--others", "--exclude-standard", "--", "wikirecall"],
        capture_output=True, check=True,
    ).stdout.decode("utf-8")
    unt = [ln for ln in untracked.splitlines() if ln.strip()]
    unt_outside = [f for f in unt
                   if f.startswith("wikirecall/") and not f.startswith(DIFF_ALLOWED_PREFIX)]
    manifests = [f for f in files + unt
                 if f.endswith(("pyproject.toml", "requirements.txt", "setup.py", "setup.cfg",
                                "Pipfile", "poetry.lock", "uv.lock"))]
    ok = not outside and not unt_outside and not manifests
    return ok, {"diff_files": files, "new_files": unt,
                "outside_allow": outside + unt_outside, "dep_manifests": manifests}


def _load_recall(wikirecall_dir):
    sys.path.insert(0, str(wikirecall_dir))
    for mod in ("semantic", "recall"):
        sys.modules.pop(mod, None)
    import recall
    return recall


def s4_double_run(wikirecall_dir, math_repo):
    recall = _load_recall(wikirecall_dir)
    ev = {"queries": {}}
    ok = True
    for q in ("置信区间", "不动点"):
        h1 = sha256_bytes(canonical(recall.recall(math_repo, [q], semantic_k=recall.DEFAULT_SEMANTIC_K)).encode("utf-8"))
        h2 = sha256_bytes(canonical(recall.recall(math_repo, [q], semantic_k=recall.DEFAULT_SEMANTIC_K)).encode("utf-8"))
        same = h1 == h2
        ev["queries"][q] = {"sha_run1": h1, "sha_run2": h2, "equal": same}
        if not same:
            ok = False
    return ok, ev


def _trail_events(trail_path):
    events = []
    for line in pathlib.Path(trail_path).read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            e = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(e, dict):
            events.append(e)
    return events


def s5_carriers_on_chain(trail_path):
    events = _trail_events(trail_path)
    found = {}
    for e in events:
        if e.get("event_type") == "crosscheck_completed":
            h = e.get("event_hash", "")
            if h.startswith("6b3723e8"):
                found["PROB-017"] = h
            if h.startswith("70da4959"):
                found["ALG-011"] = h
    ok = set(found) == {"PROB-017", "ALG-011"}
    return ok, {"found": found, "doc_ids": {k: f"crosscheck-m-pk037-{'idf' if v.startswith('6b3723e8') else 'cos'}-1"
                                             for k, v in found.items()}}


def s6_verdict_registered(trail_path):
    events = _trail_events(trail_path)
    hit = None
    for e in events:
        if (e.get("event_type") == "parking_entered"
                and e.get("doc_id") == "pk-050"
                and e.get("event_hash", "").startswith("0b3da7ec")):
            hit = e.get("event_hash")
    return hit is not None, {"parking_entered_hash": hit, "doc_id": "pk-050"}


def s7_fallback_design(wikirecall_dir, tools_wt):
    recall = _load_recall(wikirecall_dir)
    head_src = subprocess.run(
        ["git", "-C", str(tools_wt), "show", "HEAD:wikirecall/recall.py"],
        capture_output=True, check=True,
    ).stdout
    ev = {}
    with tempfile.TemporaryDirectory() as td:
        tdp = pathlib.Path(td)
        (tdp / "legacy").mkdir()
        (tdp / "legacy" / "recall.py").write_bytes(head_src)
        repo = tdp / "repo"
        (repo / "topology" / "entries").mkdir(parents=True)
        (repo / "order" / "entries").mkdir(parents=True)
        (repo / "llm-friendly-build").mkdir(parents=True)
        (repo / "order" / "INDEX.md").write_text("| ORD-905 | t | y |\n", encoding="utf-8")
        (repo / "topology" / "INDEX.md").write_text("| TOP-901 | t | y |\n", encoding="utf-8")
        (repo / "llm-friendly-build" / "mapping.md").write_text("# m\n", encoding="utf-8")
        fix_a = ("# TOP-901 测试不动点\n\n状态：夹具。\n\n## 触发问题 {#triggers}\n\n"
                 "- iteration 什么时候可以停下来？\n- 迭代收敛怎么验证？\n- fixed point 的条件是什么？\n"
                 "- 这个算子稳不稳定？\n- 收敛判据有哪些？\n\n## 哲学桥接 {#philosophy-bridge}\n\n- 关系：见 TOP-902\n")
        fix_b = ("# TOP-902 测试邻域\n\n状态：夹具。\n\n## 触发问题 {#triggers}\n\n"
                 "- separation 的工程含义是什么？\n- 隔离怎么做才够？\n- neighborhood 半径怎么定？\n"
                 "- 两态能不能判明？\n- 距离怎么选？\n")
        (repo / "topology" / "entries" / "TOP-901-test-a.md").write_text(fix_a, encoding="utf-8")
        (repo / "topology" / "entries" / "TOP-902-test-b.md").write_text(fix_b, encoding="utf-8")
        spec = __import__("importlib.util", fromlist=["util"]).spec_from_file_location(
            "legacy_recall_gate", str(tdp / "legacy" / "recall.py"))
        import importlib.util
        spec = importlib.util.spec_from_file_location("legacy_recall_gate", str(tdp / "legacy" / "recall.py"))
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        legacy_plan = mod.recall(str(repo), ["迭代收敛"])
        word_plan = recall.recall(str(repo), ["迭代收敛"], semantic_k=None)
        a = canonical(word_plan)
        b = canonical(legacy_plan)
        ev["word_vs_preswitch_default_identical"] = a == b
        default_plan = recall.recall(str(repo), ["迭代什么时候能停"], semantic_k=recall.DEFAULT_SEMANTIC_K)
        ev["default_has_semantic_channel"] = bool(default_plan["channels"].get("semantic"))
        ev["word_plan_has_no_semantic_key"] = "semantic" not in word_plan["channels"]
        ev["fixture_corpus_entries"] = len(mod.build_index(str(repo)))
    ok = all([ev["word_vs_preswitch_default_identical"], ev["default_has_semantic_channel"],
              ev["word_plan_has_no_semantic_key"]])
    return ok, ev


ITEMS = [
    ("S1", "预登记 A/B pass", lambda a: s1_ab_pass(a.prereg, a.readings)),
    ("S2", "semantic.py 仅 stdlib 零 LLM", lambda a: s2_stdlib_only(pathlib.Path(a.wikirecall))),
    ("S3", "零新增依赖", lambda a: s3_no_new_deps(a.tools_wt)),
    ("S4", "双跑逐字节一致", lambda a: s4_double_run(pathlib.Path(a.wikirecall), a.math)),
    ("S5", "载体 PROB-017/ALG-011 终签在链", lambda a: s5_carriers_on_chain(a.trail)),
    ("S6", "判变 pk-050 登记在链", lambda a: s6_verdict_registered(a.trail)),
    ("S7", "词面回退位设计在位", lambda a: s7_fallback_design(pathlib.Path(a.wikirecall), a.tools_wt)),
]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--wikirecall", required=True)
    ap.add_argument("--tools-wt", required=True)
    ap.add_argument("--math", required=True)
    ap.add_argument("--trail", required=True)
    ap.add_argument("--prereg", required=True)
    ap.add_argument("--readings", required=True)
    ap.add_argument("--out", default=None)
    a = ap.parse_args()
    items = []
    all_pass = True
    for iid, name, fn in ITEMS:
        try:
            ok, ev = fn(a)
        except Exception as exc:  # 判定失败也是如实记录，不静默
            ok, ev = False, {"error": type(exc).__name__ + ": " + str(exc)[:200]}
        if not ok:
            all_pass = False
        items.append({"id": iid, "name": name, "pass": ok, "evidence": ev})
    out = {
        "schema": "switch-gate-checklist/1",
        "gid": "m-pk050-switch-1",
        "batch": "pk050sw-solo",
        "date": "2026-09-04",
        "items": items,
        "verdict": "pass" if all_pass else "fail",
    }
    text = json.dumps(out, ensure_ascii=False, sort_keys=True, indent=1) + "\n"
    if a.out:
        pathlib.Path(a.out).write_text(text, encoding="utf-8")
    print(text, end="")
    return 0 if all_pass else 1


if __name__ == "__main__":
    sys.exit(main())
