# 盘点档：locks（lease 锁面）

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表）；线索：疑已随后续批有载体（lease ORD-020）

## 机制职责一句话

租约锁面：同一路径至多一 exclusive 持锁会话的互斥语义，append 型共享追加兼容，撞锁拒绝等待不绕行，放锁落 released 行即让路，归一化路径即全序资源标识（lease/src/lease/lockcore.py 锁核心）。

## 线索核验（源码引用位实查）

**已坐实有载体。** lockcore.py 四处 ORD-020 载体引用位（本批 2026-09-04 grep 实查）：

| 引用位 | 位置 | 承接语义 |
|---|---|---|
| 互斥语义 | lease/src/lease/lockcore.py:179 | ORD-020 互斥语义在锁映射（同一 path 至多一 exclusive 持锁会话） |
| 撞锁拒绝 | lease/src/lease/lockcore.py:206 | ORD-020 互斥+死锁不自由：撞锁所拒 locked_elsewhere 即等待不绕行 |
| 放锁让路 | lease/src/lease/lockcore.py:240 | ORD-020 等待终止性让步良基：放锁落 released 行即让路 |
| 全序资源标识 | lease/src/lease/lockcore.py:289 | ORD-020 全序资源标识统一：归一化路径即资源集上一致标识 |

接线批次：ordwire-lease-solo（2026-09-03，sessions 台账 issued 2ad4395d8f06335d 在案，ORD-020 终签承 sih-math/docs/asset-anchor-registry-2026-09-02.md 登记面）。

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| 互斥判据 | lease/src/lease/lockcore.py:179 注释锚定位 | 同 path 至多一 exclusive 持锁会话 | 判定性（已承 ORD-020） |
| 撞锁判据 | lease/src/lease/lockcore.py:206 注释锚定位 | locked_elsewhere 拒绝不绕行 | 判定性（已承 ORD-020） |
| 让路判据 | lease/src/lease/lockcore.py:240 注释锚定位 | released 行记入台账即等待方获放行 | 判定性（已承 ORD-020） |
| 资源标识 | lease/src/lease/lockcore.py:289 注释锚定位 | 归一化路径全序一致标识 | 判定性（已承 ORD-020） |
| 四态处置 | lease/src/lease/commitcore.py:341（rev1 原判待确认） | unset/already_absent 等状态枚举 | 待确认（提交面状态机，非锁面判定位） |

## 既有归属判

判定性。锁面四判据全属判定性，且已由 ordwire 批接线 ORD-020 载体。

## 载体候选分析

- 线索核实结论：rev1「无可指认载体」已被 ordwire-lease-solo 批（2026-09-03）清账——ORD-020 全序资源分配与死锁自由（mapping.md:206，已建实存）四判据位注释锚点接线在源码。
- 本批零补施工：账清，无后续接线批输入。

## 证据行

- rev1：summary-rev1.md:11 无可指认载体十件含 locks（账本时点读数，被后续批推进）。
- 对挂核验：summary-rev1.md:28 对挂核验表 lease ORD-021 行……lease ORD-020 行 SIH✓ 磁盘✓ 实存（rev1 已自证 SIH 侧实存，当时源码引用位未接线）。
