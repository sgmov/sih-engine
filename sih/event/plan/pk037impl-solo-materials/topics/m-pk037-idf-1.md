---
title: wikirecall 语义层词元加权的逆文档频率与自信息承载（新载体立项）
authored: pk037impl-solo 载体补立（2026-09-04，实查 PROB 族无 TF-IDF 族在仓载体后走 newcarr 同款立项）；承接用户 2026-09-04 裁 pk-037 升级与路线材料 m-pk037-route-1/2；改写链第 0 次
ng: medium
n: 9
gid: m-pk037-idf-1
---

# 待裁命题

wikirecall 语义层的词元加权判定可由概率子仓新条目「逆文档频率与自信息加权」（PROB-017）承载：词元 t 的文档频率 df(t) 计含 t 的文档数，逆文档频率 idf(t)=log2(N/df(t)) 即「经验文档分布下随机取一篇文档含 t」这一事件的自信息，常见词 df=N 时 idf=0 即零鉴别力退化（承 PROB-014 定理一零增益判据），词元权重取 tf×idf 即局部计数（PROB-016 计数测度的单点赋值）乘语料级稀有度信息量；该条目无哲学新桥（工程实证语义），定义先行、可证伪节与双答结构在场（M-4），落位 probability 子仓，工程接线位 semantic.py 的 idf 与 query_vector 函数已实现并过确定性双跑。

## anchors

- path: wikirecall/semantic.py
  range: 72-79
  note: idf 函数即 log2(N/df) 实现，df=0 或 df=N 即 0.0 退化侧
- path: wikirecall/semantic.py
  range: 81-99
  note: query_vector 语料外词元忽略即无 idf 不加权
- path: probability/entries/PROB-016-counting-measure-and-additivity.md
  range: 3-10
  note: tf 局部计数是计数测度的单点赋值，既有在仓载体
- path: probability/entries/PROB-014-expected-information-gain.md
  range: 3-10
  note: 零增益判退化判据源，idf 零鉴别力边界同构
