# scriwire-solo：scribe 事件链载体接线批（mathpipe-full 程序批四起）

> task-packages 治理任务
> 承接：mathpipe-full-program-v1.md 批四节即 scribe 与事件链对挂 ORD-019 版本偏序与外化状态存储；批一账本 rev1 可指认未实例化件；四件套形制全承 ordwire-lease-solo 先例
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械即机械链全绿即放行
> 用户裁定 2026-09-03 冲突行照录「链上冲突，可以插队，并不存在太大的冲突点」——链追加面插队合法，收约 trail 冲突取并集超集机械解，严禁非书简通道改写链文件
> 队形：单线形 solo
> 日期：2026-09-03
> 温故检索：materials/recall-scriwire.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，与并行批共享链/台账/调用册面

## 一、问题陈述 {#problem}

scribe 事件链的判定面即 prev_hash 链接校验与 append-only 外化存储与 verify 全链复算，当前裸奔无载体引用。ORD-019 承载版本偏序与外化状态存储三性质即持久性版本化可审计，正是事件链的数学形态。本批将链校验语义与 ORD-019 载体接线。接线对象为引擎件 sih-engine/src/event_stream/（工具侧 scribe 已退役兼容只读零碰）。

## 二、关键设计（四件套，形制承 ordwire-lease-solo） {#design}

1. 载体引用：mapping.md:196「上下文外化三性质」→ sih-math/order/entries/ORD-019-version-order-and-externalized-state-store.md 磁盘实存核验。
2. 推导档：sih-math/docs/scriwire-scribe-derivation-<实日>.md，承载链校验语义形式化即事件序构成版本偏序（prev_hash 链接即覆盖关系）、append-only 即偏序只增不改写、verify 即全链自最小元起的传递性复算，随批入版控。
3. 接线：引擎 event_stream 源码判定位注释锚点（append 链接位、verify 复算位、park 重放面位），若有 CONTRACT 或 SPEC-004 相关节则增载体引用节；SPEC-006 已有修订史零冲突。
4. 金向量：两场景机械重放逐字节一致即合法追加链 verify 过与断裂链（prev_hash 不衔接）verify 拒。

## 三、工作清单 {#work}

- [ ] 载体四件套齐
- [ ] 引擎 cargo test 全绿零行为变更
- [ ] 金向量双跑逐字节一致
- [ ] 三步管线读数
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 载体四件套** | 工程 | 引用与推导与接线与金向量四件齐，ORD-019 锚点磁盘实存 |
| **F-2 行为零变更** | 工程 | 引擎 cargo test 全绿，只加注释与文档节零改判定逻辑 |
| **F-3 金向量重放** | 工程 | 两场景机械重放逐字节一致 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列，工具侧 scribe 零碰 |

## 五、必读文件 {#read}

- 必读 1：sih-math/order/entries/ORD-019-version-order-and-externalized-state-store.md
- 必读 2：sih-engine/sih/event/plan/ordwire-lease-solo-results.md 四件套形制先例
- 必读 3：sih-engine/src/event_stream/ 与 doc/spec/SPEC-004-event-stream.md
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 只加引用注释与文档节，禁改链写入与校验逻辑（护栏在役禁自拆）
2. 认证先落主树活链即插队合法，链 settle 前一次性拷工地，严禁工地链副本追加
3. 收约 trail 冲突取并集超集且 scribe verify 过为唯一放行形
4. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-math/docs/
- sih-engine/src/event_stream/
- sih-engine/doc/spec/SPEC-004-event-stream.md
- sih-engine/sih/state/plan/scriwire-solo.md
- sih-engine/sih/event/plan/scriwire-solo-results.md
- sih-engine/sih/event/plan/scriwire-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 冲突样本节在结果档（含插队实测读数）
- [ ] 认证入链双仓结算收约对表读数在档
