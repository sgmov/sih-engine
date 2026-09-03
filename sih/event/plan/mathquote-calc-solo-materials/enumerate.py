#!/usr/bin/env python3
"""mathquote-calc-solo 枚举：宽形口径扫 calculus 缺锚，提取桥接节 PRO 引用序。
宽形口径：严正则（标准锚「原文「…」见 文件 正本 Ldigits」）加嵌套引号与可选逗号宽形，
承 mathquote2 枚举口径注记。零写。"""
import re, json
from pathlib import Path

ENTRIES = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/mathquote-calc-solo/calculus/llm-friendly-build/entries")

ANCHOR_BROAD = re.compile(r"原文「.+?」\s*[,，]?\s*见\s+\S+\s+正本\s+L\d+")
BRIDGE = re.compile(r"(?m)^##\s*哲学桥接\s*\{#philosophy-bridge\}")
FORMAL = re.compile(r"(?m)^\s*[-*]?\s*形式化[:：]")
PRO_RE = re.compile(r"PRO[-－](0?\d{1,2})")

def scan():
    files = sorted(ENTRIES.glob("*.md"))
    missing=[]; anchored=[]; no_bridge=[]; no_formal=[]
    for f in files:
        txt=f.read_text(encoding="utf-8")
        stem=f.name
        if ANCHOR_BROAD.search(txt):
            anchored.append(stem); continue
        bm=BRIDGE.search(txt)
        if not bm:
            no_bridge.append(stem); continue
        after=txt[bm.end():]
        nxt=re.search(r"(?m)^##\s", after)
        bridge_block = after[:nxt.start()] if nxt else after
        if not FORMAL.search(bridge_block):
            no_formal.append(stem)
        pros=[]
        for m in PRO_RE.finditer(bridge_block):
            p="PRO-%02d"%int(m.group(1))
            if p not in pros: pros.append(p)
        missing.append({"file":stem,"pros":pros})
    return {"files":len(files),"anchored":anchored,"missing":missing,
            "no_bridge":no_bridge,"no_formal":no_formal}

if __name__=="__main__":
    r=scan()
    print("TOTAL_FILES", r["files"])
    print("ANCHORED", len(r["anchored"]), r["anchored"])
    print("MISSING", len(r["missing"]))
    print("NO_BRIDGE", r["no_bridge"])
    print("NO_FORMAL", r["no_formal"])
    from collections import Counter
    c=Counter(f["file"].split("-")[0] for f in r["missing"])
    print("MISSING_PREFIX_COUNTS", dict(sorted(c.items())))
    pc=Counter()
    for f in r["missing"]:
        for p in f["pros"]: pc[p]+=1
    print("PRO_REF_COUNTS", dict(sorted(pc.items())))
    out=Path("/Users/moc/workspaces/SiHankor/sih-engine/sih/event/plan/mathquote-calc-solo-materials/queue.json")
    out.write_text(json.dumps({"anchored":r["anchored"],"missing":r["missing"],
        "no_bridge":r["no_bridge"],"no_formal":r["no_formal"],
        "files":r["files"]},ensure_ascii=False,indent=1),encoding="utf-8")
    print("WROTE queue.json len", len(r["missing"]))