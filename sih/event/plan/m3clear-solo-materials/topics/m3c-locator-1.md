---
title: locator 稳定标识派生与陈旧检测的 ORD-019 承载
authored: m3clear-solo M-3 不可译清账（2026-09-04，盘点档 inventory-locator.md）
ng: medium
n: 9
gid: m3c-locator-1
---

# 待裁命题

locator 的稳定标识派生与陈旧检测判定语义可由 ORD-019 版本偏序与外化状态存储承载：路径加载体种类加序号加内容哈希四元组 sha256 派生即外化载体的一致标识，stale 面存储哈希对现算哈希对表即外化存储态与现算态的版本一致性判定（ORD-019 mapping.md:196 已建实存，承 P3.2 外化管理持久性与版本化与可审计性三性质），实例化形态即 identity.py 与 stale.py 判定位注释锚点接线。

## anchors

- path: locator/src/locator/identity.py
  range: 7-13
  note: stable_id 四元组派生核——内容哈希加结构化四元组的确定性寻址
- path: locator/src/locator/stale.py
  range: 18-22
  note: 陈旧检测核——存储哈希与现算哈希对表
