#!/usr/bin/env python3
"""桥接草稿四项机械验收。用法：python3 verify.py  退出码 0=全绿 1=有失败"""
import re, sys, pathlib, yaml

ROOT = pathlib.Path(__file__).resolve().parents[2]
STAGING = pathlib.Path(__file__).resolve().parent
ENTRIES = ROOT/'sih-math/calculus/llm-friendly-build/entries'
RULE = re.compile(r'（(?!(?:[A-Za-z0-9 ./&+=,-]*|[A-Za-z][A-Za-z0-9 ./&+-]*，以下简写为：[A-Za-z0-9]+)）)[^）]*）')

def main():
    fails, tot, ok, n = [], 0, 0, 0
    for f in sorted(STAGING.glob('BRIDGE-*.md')):
        n += 1
        txt = f.read_text(encoding='utf-8')
        m = re.match(r'---\n(.*?)\n---\n(.*)', txt, re.S)
        if not m:
            fails.append((f.name, '无front matter')); continue
        try:
            fm = yaml.safe_load(m.group(1))
        except Exception as e:
            fails.append((f.name, f'front matter不可解析: {e}')); continue
        body = m.group(2)
        ent = ENTRIES/fm['entry']
        if not (ent.is_file() and '## 哲学桥接' not in ent.read_text(encoding='utf-8')):
            fails.append((f.name, '条目不存在或已有桥接节'))
        if '## 哲学桥接 {#philosophy-bridge}' not in body:
            fails.append((f.name, '节头缺失'))
        if '——' in body:
            fails.append((f.name, f"正文含禁止破折号 {body.count('——')} 处"))
        if re.search(r'```', body):
            fails.append((f.name, '正文含围栏代码块'))
        if RULE.findall(body):
            fails.append((f.name, f"正文全角括号违例{len(RULE.findall(body))}"))
        for a in fm.get('anchors', []):
            tot += 1
            q = a['quote']
            src, ln = a['source'].rsplit(':', 1)
            p = ROOT/src
            if not p.is_file():
                fails.append((f.name, f"{a['pro'][:10]} 源不存在")); continue
            line = p.read_text(encoding='utf-8').splitlines()[int(ln)-1]
            if '（' in q:
                fails.append((f.name, f"{a['pro'][:10]} 引文含全角括号")); continue
            if q in line:
                ok += 1
            else:
                fails.append((f.name, f"{a['pro'][:10]} 引文与 {src.split('/')[-1]}:{ln} 不符"))
    print(f"草稿 {n} 件 | 锚 {tot} 条 | 逐字验过 {ok} 条")
    if fails:
        for x in fails: print('FAIL', x[0], x[1])
        sys.exit(1)
    print('四项全绿零失败')
    sys.exit(0)

if __name__ == '__main__':
    main()
