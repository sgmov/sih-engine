#!/usr/bin/env python3
"""mathquote-solo 构建/引文补写脚本：程序切片逐字节子串断言后入文。
对四子仓缺锚条目，把命题对照的 PRO/EPI 依据对应原文「引文」锚追加到 形式化 bullet。
操作对象为租约工地 worktrees/sih-math/mathquote-solo。"""
import re, sys
from pathlib import Path

EMAN = Path("/Users/moc/workspaces/SiHankor/sih-philosophy/emanation")
WT   = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/mathquote-solo")

# source 文件 -> (文件名显示, 路径)
SOURCES = {
  "PRO-01": ("01-ontology-of-names", "proodos/01-ontology-of-names.md", 6, "名字不是标签，名字是本体性的"),
  "PRO-02": ("02-on-first-tao",      "proodos/02-on-first-tao.md",      89, "司衡之道一：发散自-然，收敛必-为。"),
  "PRO-03": ("03-on-second-tao",     "proodos/03-on-second-tao.md",     15, "道二：意图先于代码。"),
  "PRO-04": ("04-on-third-tao",      "proodos/04-on-third-tao.md",      15, "道三：代码自晦，意图必复。"),
  "PRO-07": ("07-on-assay",          "proodos/07-on-assay.md",          61, "镜的特点是纯粹映照"),
  "PRO-08": ("08-on-settle",         "proodos/08-on-settle.md",        108, "司衡之应：应而不藏，应辨当下，应几未来。"),
  "PRO-11": ("11-on-recursion-termination", "proodos/11-on-recursion-termination.md", 39, "元递归在此终止：自然之上无更高"),
  "EPI-12": ("12-on-epistrophe",     "epistrophe/12-on-epistrophe.md",  91, "Epistrophe 是 Emanation 的镜像"),
  "EPI-13": ("13-on-grounding",      "epistrophe/13-on-grounding.md",    3, "每个 Emanation 步的命题是否被后续引用或修正"),
  "EPI-14": ("14-on-reducibility",   "epistrophe/14-on-reducibility.md", 3, "每个 Emanation 步的核心命题是否可归约回前步命题"),
  "EPI-15": ("15-on-falsifiability", "epistrophe/15-on-falsifiability.md", 3, "每个 Emanation 步的核心命题是否具备明确的证伪条件"),
}

# 条目 stem -> [PRO/EPI 编号]（按该条目桥接实际引用的命题顺序）
MAPPING = {
 "topology": {
  "TOP-001-banach-fixed-point":["PRO-08","PRO-07"],
  "TOP-002-brouwer-fixed-point":["PRO-07","PRO-02"],
  "TOP-003-knaster-tarski-mirror":["PRO-02"],
  "TOP-004-metric-space":["PRO-02","PRO-07"],
  "TOP-005-complete-metric-space":["PRO-02","PRO-08"],
  "TOP-006-compact-set":["PRO-02","PRO-08"],
  "TOP-007-continuous-mapping":["PRO-08","PRO-02"],
 },
 "probability": {
  "PROB-001-law-of-large-numbers":["PRO-07"],
  "PROB-002-strong-law-of-large-numbers":["PRO-07"],
  "PROB-003-central-limit-theorem":["PRO-07"],
  "PROB-004-large-deviation-principle":["PRO-08"],
  "PROB-005-bayesian-updating":["PRO-07"],
  "PROB-006-probability-measure":["PRO-07"],
  "PROB-007-expectation":["PRO-07"],
  "PROB-008-information-content-and-falsifiability":["EPI-15"],
  "PROB-009-fano-inequality-and-recovery-bound":["PRO-04"],
 },
 "order": {
  "ORD-001-partially-ordered-set":["PRO-02"],
  "ORD-002-complete-lattice":["PRO-02"],
  "ORD-003-knaster-tarski-fixed-point":["PRO-02"],
  "ORD-004-monotone-operator":["PRO-02"],
  "ORD-005-chains-and-antichains":["PRO-02"],
  "ORD-006-closure-and-consequence-operator":["EPI-14"],
  "ORD-007-derivation-soundness-and-mirror":["EPI-12"],
  "ORD-008-reference-reachability-and-dangling":["EPI-13"],
  "ORD-009-commitment-closure-and-naming":["PRO-01"],
  "ORD-010-falsification-and-counterexample":["EPI-15"],
  "ORD-011-well-founded-recursion-termination":["PRO-11"],
  "ORD-016-well-founded-relation-and-backward-termination":["EPI-13"],
  "ORD-017-definability-and-definition-extension":["EPI-14"],
  "ORD-018-coupled-fixed-point-and-mutual-construction":["PRO-01"],
 },
 "algebra": {
  "ALG-001-matrix-and-eigenvalue":["PRO-07","PRO-02"],
  "ALG-008-kernel-and-fiber":["PRO-03"],
  "ALG-009-rank-nullity-and-section":["PRO-04"],
  "ALG-010-factorization-and-invariance":["PRO-03"],
 },
}

def verify_quotes():
    problems=[]
    for pid,(disp,f,ln,q) in SOURCES.items():
        full=(EMAN/f).read_text(encoding="utf-8")
        lines=full.split("\n")
        if q not in full: problems.append(f"{pid}: quote not whole-substring")
        if not (q in lines[ln-1]): problems.append(f"{pid}: quote not at line {ln}")
    return problems

def build(subset=None, dry_run=False):
    errors=[]; done=[]
    for sub, items in MAPPING.items():
        if subset and sub not in subset: continue
        for stem, pids in items.items():
            f=WT/sub/"entries"/f"{stem}.md"
            if not f.exists(): errors.append(f"MISS {f}"); continue
            txt=f.read_text(encoding="utf-8")
            # confirm bridge present and no existing anchor for these specific quotes
            # build anchor clauses
            clauses=[]
            for pid in pids:
                disp,fname,ln,q=SOURCES[pid]
                clauses.append(f"原文「{q}」见 {fname} 正本 L{ln}")
            anchor=("；".join(clauses))+"。"
            # find 形式化 bullet
            m=re.search(r"(?m)^(\s*-?\s*形式化[:：][^\n]*(?:\n(?!\s*-|\s*#+)[^\n]*)*)", txt)
            if not m:
                errors.append(f"{stem}: no 形式化 bullet"); continue
            block=m.group(0)
            # avoid double-append: skip if block already carries any full anchor
            if "原文「" in block and re.search(r"见\s*\S+\s*正本\s+L\d+", block):
                errors.append(f"{stem}: already has anchor"); continue
            # append on last line of the bullet block
            newblock=block.rstrip("\n")
            if not re.search(r"[。；；！？]$", newblock.rstrip()):
                newblock=newblock.rstrip()+"。"
            newblock=newblock.rstrip()+anchor
            newtxt=txt.replace(block, newblock, 1)
            if dry_run:
                print(f"[dry] {sub}/{stem} -> append {len(clauses)} anchor(s)")
            else:
                f.write_text(newtxt,encoding="utf-8")
                done.append((sub,stem,len(clauses)))
    print("\n".join(errors) if errors else f"NO ERRORS (dry_run={dry_run})")
    print("DONE:",len(done))
    return errors

if __name__=="__main__":
    mode = sys.argv[1] if len(sys.argv)>1 else "dry"
    subj = sys.argv[2] if len(sys.argv)>2 else None
    vp=verify_quotes()
    print("QUOTE SOURCE VERIFY:", "ALL OK" if not vp else vp)
    if not vp:
        errors=build(None, dry_run=(mode=="dry"))
        sys.exit(1 if errors else 0)
    sys.exit(2)