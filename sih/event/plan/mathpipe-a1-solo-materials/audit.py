#!/usr/bin/env python3
"""mathpipe-a1 覆盖审计四步：机制枚举、常数枚举、判定性资源性分类、载体匹配。

零 LLM 零网络，时间显式给参（RUN_DATE），输出全排序可复算。
"""
import json
import os
import re
import sys

ROOT = "/Users/moc/workspaces/SiHankor"
TOOLS = os.path.join(ROOT, "sih-tools")
ENGINE = os.path.join(ROOT, "sih-engine")
MATH = os.path.join(ROOT, "sih-math")
RUN_DATE = "2026-09-03"
OUT = sys.argv[1]

FAMILY = re.compile(
    r"(THRESHOLD|MAX|MIN|BUDGET|CAP|LIMIT|TTL|SKEW|TIMEOUT|ALPHA|BETA|EPS|"
    r"WINDOW|QUORUM|MAJORITY|RATIO|PVALUE|P_VALUE|KAPPA|LAMBDA|WEIGHT|BOUNDARY)")
RESOURCE_NAME = re.compile(r"(TIMEOUT|TTL|RETRY|BUFFER|CHUNK|DEPTH|SIZE|LEN|PORT)")
DECISION_NAME = re.compile(
    r"(THRESHOLD|QUORUM|RATIO|ALPHA|BETA|EPS|SKEW|PVALUE|P_VALUE|KAPPA|"
    r"BOUNDARY|WEIGHT|MAJORITY|CONFIDENCE)")
MATH_ID = re.compile(r"\b(PROB|ORD|TOP|ALG|LIM|DIFF|INT|SPEC|MUL|SER|HIS|NS|APP)-\d{3}\b")
TRIVIAL = {"-1", "0", "1", "2"}

PY_CONST = re.compile(r"^([A-Z][A-Z0-9_]{2,})\s*=\s*(-?\d+(?:\.\d+)?)\s*(?:#.*)?$", re.M)
RS_CONST = re.compile(r"const\s+([A-Z][A-Z0-9_]{2,})[^=\n]*=\s*(-?\d+(?:\.\d+)?)")
CMP = re.compile(r"([<>]=?|==|!=)\s*(-?\d+(?:\.\d+)?)(?![\.\d])")


def source_files(root, exts, skip_tests=True):
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = sorted(d for d in dirnames if d not in (".git", "target", "node_modules", "__pycache__", ".venv"))
        if skip_tests and ("/tests/" in dirpath.replace(os.sep, "/") + "/" or os.path.basename(dirpath) == "tests"):
            continue
        for f in sorted(filenames):
            if f.endswith(exts):
                yield os.path.join(dirpath, f)


def zh_aliases():
    p = os.path.join(TOOLS, "nomenclator", "packs", "core", "terms.json")
    try:
        d = json.load(open(p, encoding="utf-8"))
    except Exception:
        return {}
    out = {}
    for t in d.get("terms", []):
        en = (t.get("en") or "").strip()
        code = (t.get("code") or "").strip()
        key = (code or en).lower()
        if key:
            out.setdefault(key, set()).add(t.get("zh", ""))
    return out


def step_mechanisms():
    rows = []
    for entry in sorted(os.listdir(TOOLS)):
        pp = os.path.join(TOOLS, entry, "pyproject.toml")
        if not os.path.isfile(pp):
            continue
        text = open(pp, encoding="utf-8").read()
        m = re.search(r"\[project\.scripts\](.*?)(\[|$)", text, re.S)
        scripts = sorted(re.findall(r"^([A-Za-z0-9_-]+)\s*=", m.group(1), re.M)) if m else []
        drivers = sorted(f for f in os.listdir(os.path.join(TOOLS, entry))
                         if f.endswith(".py") and os.path.isfile(os.path.join(TOOLS, entry, f)))
        has_pkg = os.path.isdir(os.path.join(TOOLS, entry, "src"))
        if not (scripts or drivers or has_pkg):
            continue
        subs = set()
        for path in source_files(os.path.join(TOOLS, entry), (".py",)):
            t = open(path, encoding="utf-8", errors="replace").read()
            subs.update(re.findall(r'add_parser\(\s*"([a-z0-9_-]+)"', t))
        rows.append({"domain": "sih-tools", "name": entry,
                     "command_face": sorted(set(scripts + drivers)), "decision_face": sorted(subs)})
    binp = os.path.join(ENGINE, "src", "bin")
    for f in sorted(os.listdir(binp)):
        if not f.endswith(".rs"):
            continue
        t = open(os.path.join(binp, f), encoding="utf-8").read()
        subs = sorted(set(re.findall(r'^\s+"([a-z_]+)"\s*=>', t, re.M)))
        rows.append({"domain": "sih-engine", "name": f[:-3],
                     "command_face": [f[:-3]], "decision_face": subs})
    return rows


def course_v2_crosscheck(mechanisms):
    text = open(os.path.join(TOOLS, "COURSE-v2.md"), encoding="utf-8").read()
    aliases = zh_aliases()
    tools_dirs = {e for e in os.listdir(TOOLS)
                  if os.path.isfile(os.path.join(TOOLS, e, "pyproject.toml"))}
    mentioned = set()
    for d in sorted(tools_dirs):
        keys = {d} | {a for a in aliases.get(d.lower(), set()) if a}
        pats = [re.escape(k) if re.search(r"[^\x00-\x7f]", k)
                else r"\b" + re.escape(k) + r"\b" for k in keys]
        if any(re.search(p, text) for p in pats):
            mentioned.add(d)
    enumerated = {r["name"] for r in mechanisms if r["domain"] == "sih-tools"}
    return {"course_v2_mentioned_tools": sorted(mentioned),
            "enumerated_tools": sorted(enumerated),
            "mentioned_not_enumerated": sorted(mentioned - enumerated),
            "enumerated_not_mentioned": sorted(enumerated - mentioned)}


def step_constants():
    rows = []
    for path in source_files(TOOLS, (".py",)):
        t = open(path, encoding="utf-8", errors="replace").read()
        for m in PY_CONST.finditer(t):
            rows.append(named_row(path, t.count("\n", 0, m.start()) + 1, m.group(1), m.group(2), m.group(0)))
    for path in source_files(ENGINE, (".rs",)):
        t = open(path, encoding="utf-8", errors="replace").read()
        for m in RS_CONST.finditer(t):
            rows.append(named_row(path, t.count("\n", 0, m.start()) + 1, m.group(1), m.group(2), m.group(0)))
    return rows


def named_row(path, line, name, value, ctx):
    kind = "named"
    if RESOURCE_NAME.search(name) and not DECISION_NAME.search(name):
        cls = "资源性"
    elif DECISION_NAME.search(name) or FAMILY.search(name):
        cls = "判定性"
    else:
        cls = "待确认"
    return {"file": rel(path), "line": line, "kind": kind, "name": name,
            "value": value, "class": cls, "context": ctx[:160]}


def rel(path):
    return os.path.relpath(path, ROOT)


def literal_rows():
    rows = []
    for base in (TOOLS, ENGINE):
        for path in source_files(base, (".py", ".rs")):
            t = open(path, encoding="utf-8", errors="replace").read()
            id_refs = sorted(set(m.group(0) for m in MATH_ID.finditer(t)))
            for i, line in enumerate(t.splitlines(), 1):
                stripped = line.strip()
                if stripped.startswith(("#", "//", "*")):
                    continue
                for m in CMP.finditer(line):
                    v = m.group(2)
                    if v in TRIVIAL or v.lstrip("-").startswith("0x"):
                        continue
                    frac = 0 < abs(float(v)) < 1
                    cls = "判定性" if frac else "待确认"
                    rows.append({"file": rel(path), "line": i, "kind": "literal",
                                 "name": None, "value": v, "class": cls,
                                 "context": stripped[:160], "file_math_ids": id_refs})
    return rows


def load_math_faces():
    faces = {}
    mp = os.path.join(MATH, "llm-friendly-build", "mapping.md")
    faces["sih-math/llm-friendly-build/mapping.md"] = open(mp, encoding="utf-8").read()
    for sub in ("calculus", "topology", "probability", "order", "algebra"):
        p = os.path.join(MATH, sub, "INDEX.md")
        if os.path.isfile(p):
            faces["sih-math/%s/INDEX.md" % sub] = open(p, encoding="utf-8").read()
    return faces


def step_matching(mechanisms, faces):
    out = []
    for mech in mechanisms:
        name = mech["name"].lower()
        anchors = []
        for face, text in sorted(faces.items()):
            for i, line in enumerate(text.splitlines(), 1):
                if re.search(r"\b" + re.escape(name) + r"\b", line.lower()):
                    anchors.append({"face": face, "line": i, "excerpt": line.strip()[:120]})
        ids_in_src = set()
        base = TOOLS if mech["domain"] == "sih-tools" else ENGINE
        src = os.path.join(base, mech["name"]) if mech["domain"] == "sih-tools" else os.path.join(ENGINE, "src")
        if os.path.isdir(src):
            for path in source_files(src, (".py", ".rs")):
                t = open(path, encoding="utf-8", errors="replace").read()
                ids_in_src.update(m.group(0) for m in MATH_ID.finditer(t))
        ids_in_src = sorted(ids_in_src)
        if ids_in_src:
            state = "已实例化"
        elif anchors:
            state = "可指认未实例化"
        else:
            state = "无可指认载体"
        out.append({"name": mech["name"], "domain": mech["domain"],
                    "carrier_state": state, "source_math_ids": ids_in_src,
                    "face_anchors": anchors[:8]})
    return out


def main():
    os.makedirs(OUT, exist_ok=True)
    mechanisms = step_mechanisms()
    cross = course_v2_crosscheck(mechanisms)
    named = step_constants()
    literals = literal_rows()
    faces = load_math_faces()
    matched = step_matching(mechanisms, faces)
    pending = [r for r in named + literals if r["class"] == "待确认"]
    env_rows = [r for r in named if r["class"] == "资源性"]
    ledger = {
        "run_date": RUN_DATE,
        "root": ROOT,
        "enumeration": {
            "mechanisms": mechanisms,
            "course_v2_crosscheck": cross,
            "named_constants": named,
            "comparison_literals": [{k: v for k, v in r.items()} for r in literals],
        },
        "classification": {
            "counts": {
                "named_total": len(named),
                "named_decision": sum(1 for r in named if r["class"] == "判定性"),
                "named_resource": len(env_rows),
                "named_pending": sum(1 for r in named if r["class"] == "待确认"),
                "literal_total": len(literals),
                "literal_decision": sum(1 for r in literals if r["class"] == "判定性"),
                "literal_pending": sum(1 for r in literals if r["class"] == "待确认"),
            },
            "trivial_values_skipped": sorted(TRIVIAL),
        },
        "carrier_matching": matched,
        "carrier_counts": {
            "已实例化": sum(1 for m in matched if m["carrier_state"] == "已实例化"),
            "可指认未实例化": sum(1 for m in matched if m["carrier_state"] == "可指认未实例化"),
            "无可指认载体": sum(1 for m in matched if m["carrier_state"] == "无可指认载体"),
        },
        "pending_confirmation": pending,
    }
    with open(os.path.join(OUT, "ledger.json"), "w", encoding="utf-8") as f:
        json.dump(ledger, f, ensure_ascii=False, indent=1, sort_keys=True)
        f.write("\n")
    with open(os.path.join(OUT, "env-params.json"), "w", encoding="utf-8") as f:
        json.dump({"run_date": RUN_DATE, "resource_params": env_rows}, f,
                  ensure_ascii=False, indent=1, sort_keys=True)
        f.write("\n")
    write_summary(OUT, ledger, mechanisms, matched, pending)
    print(json.dumps({"mechanisms": len(mechanisms), "named": len(named),
                      "literals": len(literals), "pending": len(pending),
                      "carriers": ledger["carrier_counts"]}, ensure_ascii=False))


def write_summary(out, ledger, mechanisms, matched, pending):
    c = ledger["classification"]["counts"]
    cc = ledger["carrier_counts"]
    cross = ledger["enumeration"]["course_v2_crosscheck"]
    lines = []
    a = lines.append
    a("# 数学管线全量载体覆盖审计汇总")
    a("")
    a("## 概览 {#overview}")
    a("")
    a("- 运行日期 2026-09-03，批 mathpipe-a1-solo，四步纯脚本零 LLM 零网络，时间显式给参")
    a("- 枚举口径即 sih-tools 全 uv 编组加 sih-engine src/bin 清单，机制行 %d 即工具 %d 加引擎 %d" % (
        len(mechanisms),
        sum(1 for m in mechanisms if m["domain"] == "sih-tools"),
        sum(1 for m in mechanisms if m["domain"] == "sih-engine")))
    a("- 常数面即命名常数 %d 与比较位字面量 %d，分类语法启发式主分类，边界件入待确认清单不静默归类" % (
        c["named_total"], c["literal_total"]))
    a("- 载体三态即已实例化 %d 与可指认未实例化 %d 与无可指认载体 %d，锚点磁盘实存零虚构" % (
        cc["已实例化"], cc["可指认未实例化"], cc["无可指认载体"]))
    a("- 账本即 ledger.json 与 env-params.json 与本文件，脚本重跑逐字节一致为验收线")
    a("")
    a("## 分类分布 {#classification}")
    a("")
    a("| 面 | 判定性 | 资源性 | 待确认 | 合计 |")
    a("|---|---|---|---|---|")
    a("| 命名常数 | %d | %d | %d | %d |" % (
        c["named_decision"], c["named_resource"], c["named_pending"], c["named_total"]))
    a("| 比较位字面量 | %d | 0 | %d | %d |" % (
        c["literal_decision"], c["literal_pending"], c["literal_total"]))
    a("")
    a("## 编组对表 {#crosscheck}")
    a("")
    a("COURSE-v2 提及且枚举到的工具 %d 件；提及未枚举 %s；枚举未提及 %s。逐件清单在 ledger 即 course_v2_crosscheck 节。" % (
        len(cross["course_v2_mentioned_tools"]),
        "与".join(cross["mentioned_not_enumerated"]) or "零",
        "与".join(cross["enumerated_not_mentioned"]) or "零"))
    a("")
    a("## 载体三态 {#carriers}")
    a("")
    a("| 状态 | 数量 | 机制 |")
    a("|---|---|---|")
    for state in ("已实例化", "可指认未实例化", "无可指认载体"):
        names = "与".join(m["name"] for m in matched if m["carrier_state"] == state)
        a("| %s | %d | %s |" % (state, cc[state], names))
    a("")
    a("已实例化即该机制源码出现数学概念 ID 引用；可指认未实例化为零即工具名与数学检索面词汇不重叠，后续匹配须走概念 ID 不走工具名。")
    a("")
    a("## 发现与申报 {#findings}")
    a("")
    a("1. 待确认清单 %d 件即比较位字面量的非分数整数值为主，语法启发式无法单点判定其判定性，如实标待裁定不静默归类，其规模即 M-1 后续清账工作量的第一信号。" % len(pending))
    a("2. 无可指认载体 %d 件为零命中如实申报，按 M-3 二选一处置属后续批。" % cc["无可指认载体"])
    a("3. 命名常数判定性 %d 件为批二起的优先序列输入，逐件载体推导与金向量属后续批。" % c["named_decision"])
    a("4. 资源性 %d 件入环境参数登记面初版即 env-params.json，变更走工程基线裁定不走载体推导。" % c["named_resource"])
    a("")
    a("## 边界 {#boundary}")
    a("")
    a("待确认清单只呈报不代裁；新载体不立；源码只读零改动；本审计不预设任何后续批的批序。")
    a("")
    with open(os.path.join(out, "summary.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines))


if __name__ == "__main__":
    main()
