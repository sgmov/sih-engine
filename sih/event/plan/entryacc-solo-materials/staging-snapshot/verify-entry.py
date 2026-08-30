#!/usr/bin/env python3
"""条目草稿五件机械验收。用法：python3 verify-entry.py  退出码 0=全绿 1=有失败

五件核心即标题前缀、首节名、括号零、破折号零、每条引文 grep -F 逐字节命中且行号复核。
附核即 front matter 可解析、围栏代码块零、引文无全角括号与破折号、结构八节在场。
只报不判，零写路径，同参双跑逐字节一致。
"""
import re, sys, pathlib, yaml

ROOT = pathlib.Path(__file__).resolve().parents[2]
STAGING = pathlib.Path(__file__).resolve().parent
ID_PREFIXES = ('TOP-', 'PROB-', 'ORD-')
# 全角括号违例：（内为中文内容）；纯英文数字内容括号可留。同桥接波 proven 规则。
PAREN_RULE = re.compile(r'（(?!(?:[A-Za-z0-9 ./&+=,-]*|[A-Za-z][A-Za-z0-9 ./&+-]*，以下简写为：[A-Za-z0-9]+)）)[^）]*）')
REQUIRED_SECTIONS = [
    '## 定义 {#definition}',
    '## 公理条件 {#axioms}',
    '## 哲学桥接 {#philosophy-bridge}',
    '## 在 facet 的应用 {#facet-application}',
    '## 与其他概念的关系 {#relations}',
    '## 历史脉络 {#history}',
    '## 工程注意事项 {#engineering-notes}',
    '## 参考文献 {#references}',
]

def main():
    files = sorted(STAGING.glob('ENTRY-*.md'))
    if not files:
        print('ENTRY-*.md 零件，无草稿可验'); sys.exit(1)
    fails, tot_anchor, ok_anchor, n = [], 0, 0, 0
    for f in files:
        n += 1
        txt = f.read_text(encoding='utf-8')
        m = re.match(r'---\n(.*?)\n---\n(.*)', txt, re.S)
        if not m:
            fails.append((f.name, '无 front matter')); continue
        try:
            fm = yaml.safe_load(m.group(1))
        except Exception as e:
            fails.append((f.name, f'front matter 不可解析: {e}')); continue
        if not isinstance(fm, dict):
            fails.append((f.name, 'front matter 非映射')); continue
        for key in ('entry', 'agent', 'anchors', 'selfcheck'):
            if key not in fm:
                fails.append((f.name, f'front matter 缺字段 {key}'))
        body = m.group(2)

        # 一 标题前缀：正文首个非空行须为 H1 且 ID 前缀受控
        first = next((ln for ln in body.splitlines() if ln.strip()), '')
        h1m = re.match(r'^#\s+(\S+)\s+\S+', first)
        if not h1m:
            fails.append((f.name, f'H1 缺失或形不符: {first[:40]}'))
        else:
            idtok = h1m.group(1)
            if not idtok.startswith(ID_PREFIXES):
                fails.append((f.name, f'H1 ID 前缀违规: {idtok}'))

        # 二 首节名：首个二级标题须为 定义
        h2s = [ln.strip() for ln in body.splitlines() if ln.strip().startswith('## ')]
        if not h2s or h2s[0] != '## 定义 {#definition}':
            fails.append((f.name, f'首个二级标题违规: {h2s[0] if h2s else "(无)"}'))

        # 三 括号零：全文零全角括号中文内容
        pv = PAREN_RULE.findall(body)
        if pv:
            fails.append((f.name, f'全角括号中文违例 {len(pv)} 处，首处: {pv[0][:30]}'))

        # 四 破折号零
        if '——' in body:
            fails.append((f.name, f'正文含禁止破折号 {body.count("——")} 处'))

        # 附 围栏代码块零
        if re.search(r'```', body):
            fails.append((f.name, '正文含围栏代码块'))

        # 附 结构八节在场
        for sec in REQUIRED_SECTIONS:
            if sec not in body:
                fails.append((f.name, f'结构节缺失: {sec}'))

        # 五 引文逐字节命中且行号复核
        anchors = fm.get('anchors', []) or []
        if not (1 <= len(anchors) <= 2):
            fails.append((f.name, f'锚点数 {len(anchors)} 超出 1 至 2'))
        for a in anchors:
            tot_anchor += 1
            q = a.get('quote', '')
            srcfull = a.get('source', '')
            if ':' not in srcfull:
                fails.append((f.name, f"{a.get('pro','?')[:12]} source 无行号: {srcfull}")); continue
            src, ln = srcfull.rsplit(':', 1)
            p = ROOT / src
            if not p.is_file():
                fails.append((f.name, f"{a.get('pro','?')[:12]} 源不存在: {src}")); continue
            lines = p.read_text(encoding='utf-8').splitlines()
            if not ln.isdigit() or not (1 <= int(ln) <= len(lines)):
                fails.append((f.name, f"{a.get('pro','?')[:12]} 行号越界: {src.split('/')[-1]}:{ln}")); continue
            line = lines[int(ln) - 1]
            if '（' in q or '——' in q:
                fails.append((f.name, f"{a.get('pro','?')[:12]} 引文含全角括号或破折号")); continue
            # grep -F 等价：逐字节子串命中该指定行
            if q and q in line:
                ok_anchor += 1
            else:
                fails.append((f.name, f"{a.get('pro','?')[:12]} 引文与 {src.split('/')[-1]}:{ln} 逐字节不符"))
    print(f'草稿 {n} 件 | 锚 {tot_anchor} 条 | 逐字验过 {ok_anchor} 条')
    if fails:
        for x in fails:
            print('FAIL', x[0], x[1])
        sys.exit(1)
    print('五件全绿零失败')
    sys.exit(0)

if __name__ == '__main__':
    main()
