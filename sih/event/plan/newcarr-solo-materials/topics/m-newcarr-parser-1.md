---
title: parser 空腹文法解析判定语义的派生唯一性承载（新载体立项）
authored: newcarr-solo 新数学载体立项（2026-09-04，m3clear-solo 处置清单候选行）
ng: medium
n: 9
gid: m-newcarr-parser-1
---

# 待裁命题

parser 的空腹文法解析判定语义可由代数子仓新条目「形式文法与解析确定性」（ALG-012）承载：有序选择即给候选式集一个优先全序，同类先断即回溯自由，解析确定性即派生唯一——每个可解析串恰有一棵派生树，这是自由项代数的唯一构造性质（初始代数语义），组合子 seq 与 choice 与 repeat 构成项上的代数算子；词法层代理对合并的码点算术是构造性正确的字符归一（0x10000 加高低半差移位合成的可交换验证）；该条目无哲学新桥（工程实证语义），定义先行、可证伪节与双答结构在场（M-4），落位 algebra 子仓，工程接线归后续逐工具批零接线。

## anchors

- path: parser/src/parser/peg.py
  range: 94-102
  note: choice 有序选择——候选式按声明序尝试，首个成功即定，回溯自由
- path: parser/src/parser/lexer.py
  range: 72-77
  note: 代理对合并码点算术——0x10000 加高低半差移位合成，构造性正确
