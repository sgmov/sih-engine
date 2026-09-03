---
title: parser 文法驱动解析判定语义的形式文法承载（新载体候选）
authored: m3clear-solo M-3 不可译清账（2026-09-04，盘点档 inventory-parser.md）
ng: medium
n: 9
gid: m3c-parser-1
---

# 待裁命题

parser 的文法驱动解析判定语义可由形式文法解析的确定性结构承载：grammar 驱动的 tokenize+parse 对同输入产出同 parse 树，代理对到码点的解码算术（0x10000 位移组合式）构造性正确判定字符归一，lint 样例覆盖下限八条是判定阈值；该结构在数学仓暂无在册条目（mapping.md 全表零命中如实记），属新载体候选条目（形式文法解析确定性），无哲学新桥，不代建只出候选行。

## anchors

- path: parser/src/parser/lexer.py
  range: 68-75
  note: 代理对解码算术核——构造性正确的码点组合式
- path: parser/src/parser/lint.py
  range: 128-134
  note: lint 覆盖阈值核——样例少于八条即失败
