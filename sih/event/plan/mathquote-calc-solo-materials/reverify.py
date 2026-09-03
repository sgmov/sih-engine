#!/usr/bin/env python3
"""mathquote-calc-solo 复验脚本：全部新增标准锚「原文「q」见 <file> 正本 L<n>」
断言 q 为 file 在行 L 的逐字字节子串（整文亦含）。可重跑，零写。"""
import re, sys
from pathlib import Path

EMAN=Path("/Users/moc/workspaces/SiHankor/sih-philosophy/emanation/proodos")
WT  =Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/mathquote-calc-solo/calculus/llm-friendly-build/entries")

ANCHOR=re.compile(r"原文「(.+?)」\s*[,，]?\s*见\s+([\w\-./]+\.md)\s+正本\s+L(\d+)")

def main():
    total=0; checked_lines=0; fails=[]
    files=0; anch=0
    for f in sorted(WT.glob("*.md")):
        files+=1
        txt=f.read_text(encoding="utf-8")
        for m in ANCHOR.finditer(txt):
            q,src,ln=m.group(1),m.group(2),int(m.group(3))
            anch+=1
            fp=EMAN/src
            if not fp.exists():
                fails.append(f"{f.name}: source not found {src}"); continue
            full=fp.read_text(encoding="utf-8")
            lines=full.split("\n")
            if q not in full:
                fails.append(f"{f.name}: '{q[:22]}' not whole-substring in {src}")
            elif ln>len(lines) or q not in lines[ln-1]:
                fails.append(f"{f.name}: quote '{q[:22]}' not at {src} line {ln}")
            else:
                checked_lines+=1
            total+=1
    print(f"scanned entries: {files}")
    print(f"anchors checked: {anch} (total) / line-exact: {checked_lines}")
    print(f"failures: {len(fails)}")
    for x in fails: print("FAIL",x)
    return 1 if fails else 0

if __name__=="__main__":
    sys.exit(main())