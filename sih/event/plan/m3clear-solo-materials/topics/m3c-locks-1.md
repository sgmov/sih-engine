---
title: locks 互斥让路判定语义的 ORD-020 承载（清账复核）
authored: m3clear-solo M-3 不可译清账（2026-09-04，盘点档 inventory-locks.md）
ng: medium
n: 9
gid: m3c-locks-1
---

# 待裁命题

locks 的互斥与撞锁等待与放锁让路判定语义可由 ORD-020 全序资源分配与死锁自由承载：同一归一化路径至多一 exclusive 持锁会话即互斥语义在锁映射，撞锁所拒 locked_elsewhere 即等待不绕行，放锁落 released 行即让步良基，且已由 ordwire-lease-solo 批在 lockcore.py 四判据位（179/206/240/289）注释锚点实例化（mapping.md:206 已建实存），本命题为清账复核非新接线，复核通过即账清零补施工。

## anchors

- path: lease/src/lease/lockcore.py
  range: 175-210
  note: 互斥与撞锁拒绝判据位——ORD-020 注释锚点在 179 与 206
- path: lease/src/lease/lockcore.py
  range: 236-292
  note: 放锁让路与全序资源标识判据位——ORD-020 注释锚点在 240 与 289
