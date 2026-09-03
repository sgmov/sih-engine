#!/usr/bin/env python3
"""mathquote-calc-solo 补写脚本：程序切片逐字节子串断言后入文。
对 calculus 缺锚条目，按其桥接节命题对照 PRO 编号取 canonical 引文，
在桥接节最后一个 哲学命题 bullet 行尾后缀追加标准锚
「原文「q」见 <file> 正本 L<line>」。禁 LLM 手打，切片源只读。"""
import re, sys
from pathlib import Path

EMAN = Path("/Users/moc/workspaces/SiHankor/sih-philosophy/emanation/proodos")
WT   = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/mathquote-calc-solo/calculus/llm-friendly-build/entries")

# canonical PRO -> (显示名, 相对路径, 行号, 引文)
SOURCES = {
  "PRO-01": ("01-ontology-of-names",           "01-ontology-of-names.md",           6,   "名字不是标签，名字是本体性的"),
  "PRO-02": ("02-on-first-tao",                "02-on-first-tao.md",                89,  "司衡之道一：发散自-然，收敛必-为。"),
  "PRO-03": ("03-on-second-tao",               "03-on-second-tao.md",               15,  "道二：意图先于代码。"),
  "PRO-04": ("04-on-third-tao",                "04-on-third-tao.md",                15,  "道三：代码自晦，意图必复。"),
  "PRO-05": ("05-on-fourth-tao",               "05-on-fourth-tao.md",               15,  "道四：规约与实现必有间隙。"),
  "PRO-06": ("06-on-canon",                    "06-on-canon.md",                    57,  "法二：有度。收敛恰到好处，不过度收紧也不放任发散。"),
  "PRO-07": ("07-on-assay",                    "07-on-assay.md",                    61,  "镜的特点是纯粹映照"),
  "PRO-08": ("08-on-settle",                   "08-on-settle.md",                   108, "司衡之应：应而不藏，应辨当下，应几未来。"),
  "PRO-09": ("09-on-arche",                    "09-on-arche.md",                    84,  "司衡之元：治理治理的治理。任何治理框架的变更必须自带防御机制，防止变更被治理对象反向利用。"),
  "PRO-10": ("10-on-philosophy-compendium",    "10-on-philosophy-compendium.md",    3,   "司衡（SiHankor）：以道-法-鉴-应-元为骨架的治理哲学体系，代码工程为其首要应用领域。"),
}

BRIDGE = re.compile(r"(?m)^##\s*哲学桥接\s*\{#philosophy-bridge\}")
BULLET = re.compile(r"(?m)^(\s*)[-*]\s+")           # any bullet start
ZHEXUE = re.compile(r"^(\s*)[-*]\s*哲学命题[:：]")
PRO_RE = re.compile(r"PRO[-－](0?\d{1,2})")
STD_ANCHOR = re.compile(r"原文「.+?」\s*[,，]?\s*见\s+\S+\s+正本\s+L\d+")

def verify_canonical():
    probs=[]
    for pid,(disp,fn,ln,q) in SOURCES.items():
        full=(EMAN/fn).read_text(encoding="utf-8")
        lines=full.split("\n")
        if q not in full: probs.append(f"{pid}: not whole-substring")
        elif not (q in lines[ln-1]): probs.append(f"{pid}: not at line {ln}")
    return probs

def bridge_block(txt):
    m=BRIDGE.search(txt)
    if not m: return None
    a=txt[m.end():]
    n=re.search(r"(?m)^##\s",a)
    return a[:n.start()] if n else a

def last_zhexue_block_end(bridge):
    lines=bridge.split("\n")
    zx=[i for i,l in enumerate(lines) if ZHEXUE.match(l)]
    if not zx: return None
    start=zx[-1]
    end=len(lines)
    for j in range(start+1, len(lines)):
        l=lines[j]
        if re.match(r"^\s*$", l) or BULLET.match(l) or l.startswith("##"):
            end=j; break
    return start, end

def build(dry=True):
    errs=[]; done=[]
    for f in sorted(WT.glob("*.md")):
        txt=f.read_text(encoding="utf-8")
        if STD_ANCHOR.search(txt):
            continue
        bridge=bridge_block(txt)
        if bridge is None:
            errs.append(f"{f.name}: no bridge"); continue
        pros=[]
        for m in PRO_RE.finditer(bridge):
            p="PRO-%02d"%int(m.group(1))
            if p not in pros: pros.append(p)
        if not pros:
            errs.append(f"{f.name}: no PRO ref in bridge"); continue
        se=last_zhexue_block_end(bridge)
        if se is None:
            errs.append(f"{f.name}: no 哲学命题 bullet"); continue
        st,en=se
        lines=bridge.split("\n")
        clauses=[]
        for p in pros:
            disp,fn,ln,q=SOURCES[p]
            clauses.append(f"原文「{q}」见 {fn} 正本 L{ln}")
        anchor=("；".join(clauses))+"。"
        last=lines[en-1].rstrip("\n").rstrip()
        if not re.search(r"[。！？；:]$", last):
            last=last+"。"
        lines[en-1]=last+anchor
        newbridge="\n".join(lines)
        newtxt=txt.replace(bridge,newbridge,1)
        if not dry:
            f.write_text(newtxt,encoding="utf-8")
            done.append((f.name,len(clauses)))
        else:
            print(f"[dry] {f.name}: +{len(clauses)} anchors -> {','.join(pros)}")
    print("ERRORS:" if errs else f"NO ERRORS (dry={dry})")
    print("\n".join(errs))
    print("DONE:",len(done))
    return errs

if __name__=="__main__":
    vp=verify_canonical()
    print("CANONICAL VERIFY:", "ALL OK" if not vp else vp)
    if vp:
        sys.exit(2)
    dry=(len(sys.argv)<2 or sys.argv[1]=="dry")
    errs=build(dry=dry)
    sys.exit(1 if errs else 0)