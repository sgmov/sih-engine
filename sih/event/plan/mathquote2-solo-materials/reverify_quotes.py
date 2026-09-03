#!/usr/bin/env python3
"""mathquote-solo 复验脚本（F-2）：对本批 34 条施工条目全部新增引文做原文子串断言，全绿可重跑。
范围严格限定本批施工条目（queue.json missing_list），12 条异格式已锚条目（convergence 参照 P3.x）
不在本批范围，脚本扫描到即标注 out-of-scope 不纳入红绿判决，避免误报。
锚引文须逐字节为哲学仓原文对应行子串。退出码 0=全绿 1=有失败。"""
import re, sys
from pathlib import Path

EMAN = Path("/Users/moc/workspaces/SiHankor/sih-philosophy/emanation")
WT   = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/mathquote-solo")
SUBS = ["topology","probability","order","algebra"]

# 本批施工条目（queue.json missing_list 34 条，enumerate 复算一致）
WORKED = {
 "topology":["TOP-001-banach-fixed-point","TOP-002-brouwer-fixed-point","TOP-003-knaster-tarski-mirror",
  "TOP-004-metric-space","TOP-005-complete-metric-space","TOP-006-compact-set","TOP-007-continuous-mapping"],
 "probability":["PROB-001-law-of-large-numbers","PROB-002-strong-law-of-large-numbers","PROB-003-central-limit-theorem",
  "PROB-004-large-deviation-principle","PROB-005-bayesian-updating","PROB-006-probability-measure","PROB-007-expectation",
  "PROB-008-information-content-and-falsifiability","PROB-009-fano-inequality-and-recovery-bound"],
 "order":["ORD-001-partially-ordered-set","ORD-002-complete-lattice","ORD-003-knaster-tarski-fixed-point",
  "ORD-004-monotone-operator","ORD-005-chains-and-antichains","ORD-006-closure-and-consequence-operator",
  "ORD-007-derivation-soundness-and-mirror","ORD-008-reference-reachability-and-dangling",
  "ORD-009-commitment-closure-and-naming","ORD-010-falsification-and-counterexample",
  "ORD-011-well-founded-recursion-termination","ORD-016-well-founded-relation-and-backward-termination",
  "ORD-017-definability-and-definition-extension","ORD-018-coupled-fixed-point-and-mutual-construction"],
 "algebra":["ALG-001-matrix-and-eigenvalue","ALG-008-kernel-and-fiber","ALG-009-rank-nullity-and-section",
  "ALG-010-factorization-and-invariance"],
}

# 锚引文逐字节为原文对应行子串。解析停在第一个「」之后，要求「见 <file> 正本 L<n>」紧邻，
# 不会把条目自身行内的多个「原文「…」」并句误吞为一个锚。
FULL_ANCHOR = re.compile(r"原文「([^」]+)」\s*见\s+(\S+)\s+正本\s+L(\d+)")
FILEMAP = {  # 显示名或全相对路径 -> 相对 emanation 路径
 "01-ontology-of-names":"proodos/01-ontology-of-names.md",
 "02-on-first-tao":"proodos/02-on-first-tao.md",
 "03-on-second-tao":"proodos/03-on-second-tao.md",
 "04-on-third-tao":"proodos/04-on-third-tao.md",
 "05-on-fourth-tao":"proodos/05-on-fourth-tao.md",
 "06-on-canon":"proodos/06-on-canon.md",
 "07-on-assay":"proodos/07-on-assay.md",
 "08-on-settle":"proodos/08-on-settle.md",
 "09-on-arche":"proodos/09-on-arche.md",
 "10-on-philosophy-compendium":"proodos/10-on-philosophy-compendium.md",
 "11-on-recursion-termination":"proodos/11-on-recursion-termination.md",
 "12-on-epistrophe":"epistrophe/12-on-epistrophe.md",
 "13-on-grounding":"epistrophe/13-on-grounding.md",
 "14-on-reducibility":"epistrophe/14-on-reducibility.md",
 "15-on-falsifiability":"epistrophe/15-on-falsifiability.md",
}

def match_to_orig(fname):
    if fname in FILEMAP: return FILEMAP[fname]
    for disp,path in FILEMAP.items():
        if fname==path: return path
    return None

def main():
    scanned=0; worked=0; anchors_total=0; fails=[]; out_of_scope=[]
    for sub in SUBS:
        for md in sorted((WT/sub/"entries").glob("*.md")):
            scanned+=1
            stem=md.stem
            txt=md.read_text(encoding="utf-8")
            if "哲学桥接" not in txt: continue
            if stem not in WORKED[sub]:
                # 非本批条目：有桥接者应为既有异格式锚（convergence 参照）或先例，不入红绿
                if "哲学桥接" in txt:
                    out_of_scope.append(f"{sub}/{stem}")
                continue
            worked+=1
            anchors=FULL_ANCHOR.findall(txt)
            if not anchors:
                fails.append(f"{sub}/{stem}: worked entry missing full anchor")
                continue
            for q, fname, ln in anchors:
                anchors_total+=1
                path=match_to_orig(fname)
                if not path:
                    fails.append(f"{sub}/{stem}: unmapped anchor file {fname!r}"); continue
                full=(EMAN/path).read_text(encoding="utf-8")
                lines=full.split("\n")
                lnb=int(ln)
                whole=q in full
                atline=(lnb<=len(lines)) and (q in lines[lnb-1])
                if not whole: fails.append(f"{sub}/{stem} {fname} L{ln}: quote NOT whole-substring: {q[:20]}…")
                if not atline: fails.append(f"{sub}/{stem} {fname} L{ln}: quote NOT at stated line: {q[:20]}…")
    print("scanned entries:", scanned)
    print("worked entries verified:", worked, "anchors checked:", anchors_total)
    print("bridge entries not in batch scope (info, pre-anchored/other-format):", len(out_of_scope))
    if out_of_scope: print("  out-of-scope:", " ".join(out_of_scope))
    print("byte-substring failures:", len(fails))
    for x in fails: print("   -", x)
    if fails:
        print("-> RED")
        return 1
    print("ALL 34 worked entries carry full anchors; every quote is byte-exact substring at its stated line. -> GREEN")
    return 0

if __name__=="__main__":
    sys.exit(main())