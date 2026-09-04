---
title: wikirecall 语义层相似度判定的向量空间与余弦相似度承载（新载体立项）
authored: pk037impl-solo 载体补立（2026-09-04，实查 algebra 族余弦相似度无已建在仓载体、ALG-011 向量空间为已登记待建概念后走 newcarr 同款立项）；承接用户 2026-09-04 裁 pk-037 升级与路线材料 m-pk037-route-1/2；改写链第 0 次
ng: medium
n: 9
gid: m-pk037-cos-1
---

# 待裁命题

wikirecall 语义层的相似度判定可由代数子仓已登记待建概念「向量空间」（ALG-011）建条承载：词与文档表为实内积空间中的 tf-idf 加权向量，相似度取夹角余弦即内积除以两范数之积，Cauchy-Schwarz 不等式给余弦落 [-1,1] 即得分有界，零向量侧余弦取零即无共有词元两向量正交不命中，检索排序按得分降序加条目 id 升序决胜即全序裁决确定；该条目无哲学新桥（工程实证语义），定义先行、可证伪节与双答结构在场（M-4），落位 algebra 子仓并把 INDEX 既有待建行升级为已建，工程接线位 semantic.py 的 cosine 与 semantic_channel 函数已实现并过确定性双跑。

## anchors

- path: wikirecall/semantic.py
  range: 101-111
  note: cosine 函数即内积除两范数之积，零向量侧取 0.0
- path: wikirecall/semantic.py
  range: 114-136
  note: semantic_channel 全量暴力排序，序决胜为得分降序加条目 id 升序
- path: algebra/INDEX.md
  range: 39-39
  note: ALG-011 向量空间已登记待建行，本条目落地后升级已建
