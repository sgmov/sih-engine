#!/usr/bin/env python3
# mathclose-solo 件一枚举：四波提交逐文件归因，产出改写条目清单
# 预期 147 件 = calculus 113 + 四子仓 34，APP-011 零触碰
import json, subprocess, os, sys

MATH = "/Users/moc/workspaces/SiHankor/sih-math"
WAVES = [
    ("a4372c1", "mathrefmt"),
    ("e4d5724", "mathquote2"),
    ("bd60afd", "mathquote-calc"),
    ("56e90c8", "mathrefmt2"),
]
# 漫画 wav 顺序即 git log 提交顺序（按时间序），并入括注用加连接

def wave_files(commit):
    r = subprocess.run(
        ["git", "diff", "--name-only", f"{commit}^", commit],
        cwd=MATH, capture_output=True, text=True, check=True)
    return [l for l in r.stdout.splitlines() if l.strip()]

allw = []
for commit, wave in WAVES:
    files = wave_files(commit)
    # 仅保留 entry 目录 .md 文件，其余(INDEX等)不属条目改写
    entries = [f for f in files if f.endswith(".md")
               and ("/entries/" in f or f.startswith("entries/"))]
    allw.append((commit, wave, entries))

# 汇总每文件被哪些波改写（保持波序）
from collections import OrderedDict
mapping = OrderedDict()
for commit, wave, entries in allw:
    for e in entries:
        mapping.setdefault(e, []).append(wave)

# 统计
calc = [f for f in mapping if f.startswith("calculus/")]
others = [f for f in mapping if not f.startswith("calculus/")]
app011 = [f for f in mapping if "APP-011" in f]

result = {
    "total": len(mapping),
    "calculus": len(calc),
    "others": len(others),
    "app011_touched": app011,
    "waves_summary": {w: len(e) for (_, w, e) in allw},
    "entries": {p: mapping[p] for p in mapping},
}
print(json.dumps(result, ensure_ascii=False, indent=2))