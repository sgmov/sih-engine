# scriwire2-solo：scribe 事件链载体接线重发批（承 scriwire-solo 锁竞态停滞处置）

> task-packages 治理任务
> 承接：scriwire-solo 原任务包（sih-engine/sih/state/plan/scriwire-solo.md）全 scope 不变即四件套接线 ORD-019；原批 2026-09-03 二波锁竞态停滞经用户停手主会处置，施工已抢救即 materials/salvage-engine-wiring-2026-09-03.diff（73 行 SPEC-004 加 append.rs 加 park.rs 锚点）与 materials/scriwire-scribe-derivation-2026-09-03.md 推导档原件与金向量双跑件四份
> locksplit-solo 已落链：追加态锁与路径同一化与包含判定在役，本批共享追加面一律 --mode append
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械机械链全绿即放行
> 队形：单线形 solo，与 caswire2-solo 故意同跑实测追加态（pk-045 参与者）
> 日期：2026-09-03
> 温故检索：materials/recall-scriwire2.json 双档零命中如实记

## 一、问题陈述 {#problem}

同原包：引擎事件链判定面（prev_hash 链接、append-only、verify 复算）载体接线 ORD-019。原批施工过半被锁竞态憋停，抢救件在档，本批合法落链。

## 二、关键设计 {#design}

1. 施工源二选一自报：甲即抢救 diff 与推导档与金向量双跑件为盘点源复核续作；乙即重做。读数不继承原批。
2. 四件套承 ordwire 先例：载体引用 mapping.md:196、推导档、SPEC-004 载体节加引擎 src/event_stream/ 注释锚点、金向量两场景（合法链 verify 过与断裂链拒）双跑。
3. 行为零变更：只加注释与文档节禁改链写入与校验逻辑（护栏在役禁自拆），引擎 cargo test --lib 全绿（现况 144 过 0 败即基线）。
4. 锁纪律机器位：施工面（src/event_stream、SPEC-004、sih-math/docs、批文件）exclusive；共享追加面 --mode append 短持取放。

## 三、F 表 {#work}

| F | 判据 |
|---|---|
| F-1 四件套 | 引用与推导与接线与金向量齐，ORD-019 实存 |
| F-2 零行为变更 | 引擎 144 测基线全绿，wiring_only_no_behavior_change |
| F-3 金向量 | 两场景双跑逐字节一致 |
| F-4 锁纪律 | 追加态实测读数在档（与 caswire2 并发即插队实录） |
| F-5 写入仅 allow | 请求写入节所列，工具侧 scribe 零碰 |

## 四、请求写入 {#requested-writes}

- sih-math/docs/
- sih-engine/src/event_stream/
- sih-engine/doc/spec/SPEC-004-event-stream.md
- sih-engine/sih/state/plan/scriwire2-solo.md
- sih-engine/sih/event/plan/scriwire2-solo-results.md
- sih-engine/sih/event/plan/scriwire2-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/

## 五、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 冲突样本节含与 caswire2 并发追加态实录
- [ ] 认证入链双仓结算收约对表读数在档
