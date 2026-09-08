# critsweep registry 派生依据（critsweep-solo 批 2026-09-08）

判据文本源：sih-engine/doc/governance/GOV-002-mainline-lock-v1.md §退出标准（v2.4）五行程序切片，逐字节内嵌于 registry.json（构建器 make_registry.py 可重放切片）。派生原则：令牌宁窄勿宽，窄报沉底不谎报在飞；活动扫描按批名命名空间字段（record_path/report_path/package）禁全文散文匹配。

## 逐条派生与链面核实读数（核实日 2026-09-08）

| 判据 | status_kind | token_scope | 派生依据 | 链面核实 |
|---|---|---|---|---|
| GOV2-C1-leaseopt | evidence | leaseopt | 达成态承 GOV-002 v2.4 追记与结算单 SETTLEMENT-LEASEOPT-2026-09-05.md（在档已核）；链证据谓词 parking_exited entry_id=pk-045 disposition=promoted；令牌取线族批名共根 | 扫描全在盘 trail：pk-045 promoted 出泊事件在链；命名空间 leaseopt 命中 39 笔 last_hit 2026-09-05 |
| GOV2-C2-viewline | activity | viewline | 线程序包 viewline-line-v1.md（在档）与 viewline-solo 批名共根；viewer 一词泛化过宽（散文易撞）不收 | 命名空间命中 14 笔全部落 2026-09-05，last_hit 2026-09-05，gap 3 不大于阈值三即在飞；散文对照 2 件不计 |
| GOV2-C3-measure-poly | activity | measure-poly, measurepoly | 程序源 measure-poly-rev1-progdoc.md 立约 2026-09-04 即 registered_at；令牌取程序名连字与无连字两形；pk-044 出泊裁定（2026-09-07）明引 measure-poly-rev1 §7 属设计引用非程序活动，全文匹配会误计活动，故设散文对照面 | 命名空间零批记录；散文对照 2 件即 2026-09-05 pk-054 进泊与 2026-09-07 pk-044 出泊裁定；gap 自 registered_at 起算 4 > 3 报沉底 |
| GOV2-C4-math-attribution | activity | pk-053, mathclose, mathreg | 判据本体挂 pk-053 冻结件清账（泊件在档）；令牌自链面核实取三件即泊位号与清账批名 mathclose-solo 共根与账本批名 mathreg-solo 共根；泛数学词 math 前缀族过宽不收 | pk-053 命名空间零命中；mathclose 命中 8 笔落 2026-09-03；mathreg 命中 15 笔落 2026-08-30；last_hit 2026-09-03，gap 5 > 3 按令牌实报沉底 |
| GOV2-C5-lease-line | evidence | leaseup | 达成态承 GOV-002 v2.4 追记与 SETTLEMENT-LEASEUP-2026-09-07.md（在档）与 lease CONTRACT.md 修订四十一至四十三文本指纹（1.28.0 至 1.30.0 在役）；链证据谓词 parking_exited pk-072 promoted（leaseupclose-solo 补落在链）；令牌 leaseup 与 leaseopt 互斥不重叠 | pk-072 promoted 出泊事件在链；两文件在场且三指纹全命中；命名空间 leaseup 命中 7 笔 last_hit 2026-09-06 |

## 阈值语义

沉底阈值缺省三日：gap 严格大于三日即沉底，恰等于三日仍算在飞（--threshold 可覆调）。边界实读：viewline gap 3 报在飞、measure-poly gap 4 报沉底、C4 gap 5 报沉底，三例同日可复算互证。零批记录自 registered_at 起计 gap（C3 形）。

## 金向量（2026-09-08 主树条件首跑冻结）

C1 achieved（证据面文件在场加 pk-045 链事件在）、C2 in_flight（gap 3）、C3 sunk（零批 gap 4 自 registered_at）、C4 sunk（last_hit 2026-09-03 gap 5）、C5 achieved（证据面全过）；parking engine mainline 47 侧 2 废 9 告警一（siding_surplus 已知配置性误报）、tools mainline 23 侧 1 废 0 告警零；inflight 随批内时点浮动不作金向量项；双跑逐字节 IDENTICAL（first-run-2026-09-08/run1.json 与 run2.json cmp 相等）。
