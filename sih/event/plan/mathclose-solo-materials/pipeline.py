#!/usr/bin/env python3
# mathclose-solo 三步管线驱动：化格→核阅→检词，逐件读数入报告
import json, os, subprocess, sys

ROOT = "/Users/moc/workspaces/SiHankor"
WT = f"{ROOT}/worktrees/sih-math/mathclose-solo"
ENUM = f"{ROOT}/sih-engine/sih/event/plan/mathclose-solo-materials/2026-09-03-mathclose-enumerate.json"
OUT = f"{ROOT}/sih-engine/sih/event/plan/mathclose-solo-materials/2026-09-03-mathclose-pipeline.json"

def run(cmd, cwd):
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, env=dict(os.environ, **{"PYTHONHOME":"","PYTHONPATH":""}))

files = sorted(json.load(open(ENUM))["entries"].keys())
report = {"total": len(files), "fmt": {}, "scr": {}, "nomen": {}}

for rel in files:
    p = f"{WT}/{rel}"
    # 1 化格
    f = run(["uv","run","--project",".","formatter","--pack","packs/general-v1","--write",p], f"{ROOT}/sih-tools/formatter")
    report["fmt"][rel] = {"code": f.returncode, "out": (f.stdout or "")[-400:]}
    # 2 核阅 des-001-mathe
    s = run([f"{ROOT}/sih-engine/target/debug/scrutinator","--pack","des-001-mathe",p], f"{ROOT}/sih-engine")
    out = ""
    try:
        j = json.loads(s.stdout)
        out = {"findings": len(j.get("findings",[])), "domain_mismatches": len(j.get("domain_mismatches",[])), "summary": j.get("summary")}
    except Exception:
        out = (s.stdout or s.stderr)[-300:]
    report["scr"][rel] = {"code": s.returncode, "res": out}
    # 3 检词 core
    n = run(["uv","run","--project",".","nomenclator","check","--pack","packs/core",p], f"{ROOT}/sih-tools/nomenclator")
    report["nomen"][rel] = {"code": n.returncode, "out": (n.stdout or "")[-300:]}

json.dump(report, open(OUT,"w"), ensure_ascii=False, indent=2)

# 汇总
def agg(stage):
    codes = [v["code"] for v in report[stage].values()]
    from collections import Counter
    return Counter(codes)
print("files:", len(files))
print("formatter exit dist:", dict(agg("fmt")))
print("scrutinator exit dist:", dict(agg("scr")))
print("nomenclator exit dist:", dict(agg("nomen")))
# 列出任何非零/异常
for stage,label in [("fmt","formatter"),("scr","scrutinator"),("nomen","nomenclator")]:
    bad = [rel for rel,v in report[stage].items() if v["code"]!=0]
    if bad:
        print(f"[{label}] non-zero count:", len(bad))
        for rel in bad: print("   ", rel, "code=", report[stage][rel]["code"])
    else:
        print(f"[{label}] all exit 0")