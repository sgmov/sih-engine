---
title: nomenclator 词面违例判定的串跨度匹配承载（新载体候选）
authored: m3clear-solo M-3 不可译清账（2026-09-04，盘点档 inventory-nomenclator.md）
ng: medium
n: 9
gid: m3c-nomenclator-1
---

# 待裁命题

nomenclator 的词面违例判定语义可由已登记术语集上的子串跨度匹配结构承载：登记面 terms.json 为判定基准集，iter_spans 在目标文本枚举已登记词的出现跨度即违例定位，字符类别判定（CJK/ASCII）决定跨度切分；该结构在数学仓暂无在册条目（mapping.md 全表零命中如实记），属新载体候选条目（基准集上的串跨度匹配判定），无哲学新桥，不代建只出候选行。

## anchors

- path: nomenclator/src/nomenclator/matching.py
  range: 6-18
  note: 词跨度匹配核——字符类别判定与 iter_spans 出现跨度枚举
- path: nomenclator/src/nomenclator/matching.py
  range: 41-52
  note: 违例定位输出核——locate 行列定位与摘录截断资源位
