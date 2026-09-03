---
title: meter 逐调用计数语义的计数测度承载（新载体候选）
authored: m3clear-solo M-3 不可译清账（2026-09-04，盘点档 inventory-meter.md）
ng: medium
n: 9
gid: m3c-meter-1
---

# 待裁命题

meter 的逐调用计数判定语义可由计数测度的可加性结构承载：一次包裹调用恰一记录（追加不可改），日计数即逐记录累加，crosscheck 对账即记录序列对链事件序列的差集核对；该结构在数学仓暂无在册条目（mapping.md 全表零命中如实记，PROB-006 概率测度为归一化测度不覆盖计数测度，不硬挂），属新载体候选条目（计数测度可加性），无哲学新桥，不代建只出候选行。

## anchors

- path: meter/src/meter/cli.py
  range: 41-46
  note: 逐调用一记录核——按日 ndjson 追加恰一行
- path: meter/src/meter/cli.py
  range: 130-135
  note: crosscheck 对账核——counts 记录对链事件差集判定
