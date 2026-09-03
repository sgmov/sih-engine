# caswire2-solo：cascade 载体接线重发批（承 caswire-solo 锁竞态停滞处置）

> task-packages 治理任务
> 承接：caswire-solo 原任务包（sih-engine/sih/state/plan/caswire-solo.md）全 scope 不变即四件套接线 ORD-016；原批 2026-09-03 二波锁竞态停滞经用户停手主会处置，施工已抢救即 materials/salvage-tools-wiring-2026-09-03.diff（1880 行 CONTRACT 加 core.py 加词条）与 materials/caswire-cascade-derivation-2026-09-03.md 推导档原件
> locksplit-solo 已落链：追加态锁与路径同一化与包含判定在役，本批共享追加面一律 --mode append 即机械承载用户插队裁定
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械机械链全绿即放行
> 队形：单线形 solo，与 scriwire2-solo 故意同跑实测追加态（pk-045 参与者）
> 日期：2026-09-03
> 温故检索：materials/recall-caswire2.json 双档零命中如实记

## 一、问题陈述 {#problem}

同原包：cascade 判定面（上游洁净判定与乐观基线比较）载体接线 ORD-016。原批施工过半被锁竞态憋停，抢救件在档，本批合法落链。

## 二、关键设计 {#design}

1. 施工源二选一自报：甲即抢救 diff 与推导档为盘点源复核续作（apply 后逐件机械复验）；乙即从头重做。读数不继承原批。
2. 四件套形制承 ordwire 先例：载体引用 mapping.md:190、推导档（原件复核或重做）、CONTRACT 载体节加源码锚点、金向量两场景（洁净链全过与污染链拒）双跑。
3. 行为零变更：只加注释与契约节零改判定谓词，cascade 既有测试全绿。
4. 锁纪律机器位：施工面（cascade 与 sih-math/docs 与批文件）exclusive；共享追加面（trail 文件、scribe/reports、CALL-LOG、meter/counts、ledger）--mode append 短持取放；core 包 exclusive。

## 三、工作清单与 F 表 {#work}

| F | 判据 |
|---|---|
| F-1 四件套 | 引用与推导与接线与金向量齐，ORD-016 实存 |
| F-2 零行为变更 | 既有测试全绿，wiring_only_no_behavior_change |
| F-3 金向量 | 两场景双跑逐字节一致 |
| F-4 锁纪律 | 追加态实测读数在档（与 scriwire2 并发即插队实录） |
| F-5 写入仅 allow | 请求写入节所列 |

其余工作清单承原包三节。

## 四、请求写入 {#requested-writes}

- sih-math/docs/
- sih-tools/cascade/
- sih-tools/nomenclator/packs/core/
- sih-engine/sih/state/plan/caswire2-solo.md
- sih-engine/sih/event/plan/caswire2-solo-results.md
- sih-engine/sih/event/plan/caswire2-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/

## 五、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 冲突样本节含与 scriwire2 并发追加态实录
- [ ] 认证入链双仓结算收约对表读数在档
