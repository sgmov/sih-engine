# 盘点档：elicit（叩问）

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表）

## 机制职责一句话

缺口信号检测与消化闸：词对登记面集合的隶属判定出信号，契约对信号逐条的处置标记覆盖判定放行 digest，suspend 落挂起记录（elicit/src/elicit/cli.py:4 模块docstring）。

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| 未登记词判定 | elicit/src/elicit/cli.py:66 `if word not in known` | 词不在登记面集合即出信号 | 判定性 |
| 消化覆盖判定 | elicit/src/elicit/cli.py:116 `f"叩问处置[{s['subject']}]" not in text` | 契约文本含逐信号处置标记即覆盖，缺即 blocked | 判定性 |
| 退出码枚举 | elicit/src/elicit/cli.py（check 1/0，digest 0/1/2） | 三态退出码即闸门通拒信号 | 判定性 |
| 挂起状态常量 | elicit/src/elicit/cli.py:140 `"state": "suspended"` | 挂起记录态标记 | 判定性 |

## 既有归属判

判定性。信号判定与覆盖判定直接决定叩问闸通拒，无资源性参数（零超时零并发零缓冲常数）。

## 载体候选分析

- 未登记词判定是登记面全集上的集合隶属判定。
- 消化覆盖判定是信号节点到契约处置标记的可达判定：每信号须在契约文本有出边（处置标记），缺即孤悬信号。
- 已建条目对照：ORD-008 引用图可达与孤悬判定（mapping.md:184，sih-math/order/entries/ 实存）与出边存在性判据同构（gatecap 批 C008 先例即「@ 双指针记号即出边存在性的机械判据」）。
- 源码概念引用位实测：sih-tools/elicit/src 零命中（本批 grep 实查 2026-09-04）。

## 证据行

- rev1：summary-rev1.md:11 无可指认载体十件含 elicit；ledger-rev1.json carrier_matching elicit 条目 source_math_ids 空与 face_refs 三面空。
- 温故：recall-m3clear.json 零命中如实记。
