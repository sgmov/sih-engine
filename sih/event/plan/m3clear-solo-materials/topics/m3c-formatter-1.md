---
title: formatter 包驱动改写幂等语义的重写系统承载（新载体候选）
authored: m3clear-solo M-3 不可译清账（2026-09-04，盘点档 inventory-formatter.md）
ng: medium
n: 9
gid: m3c-formatter-1
---

# 待裁命题

formatter 的包驱动改写幂等判定语义可由确定性重写系统的合流与幂等性质承载：按包声明序逐操作对文本改写，重跑落零（exit 0 无需改）即不动点，未知操作种类即拒绝；该性质在数学仓暂无在册条目（mapping.md 全表零命中如实记），属新载体候选条目（重写系统确定性幂等），无哲学新桥，不代建只出候选行。

## anchors

- path: formatter/src/formatter/format.py
  range: 54-66
  note: apply_ops 核——按包声明序逐操作改写，未知操作种类拒绝
- path: formatter/src/formatter/format.py
  range: 13-15
  note: apply_line_ops 行操作核——围栏感知的确定性行改写
