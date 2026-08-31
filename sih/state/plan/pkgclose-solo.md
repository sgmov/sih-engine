# pkgclose-solo：任务包 007 至 013 对账收口批

> task-packages 治理任务
> 承接：用户 2026-08-31 令即先做 007-013 对账收口、按代理编组范式由执行代理完成、主会话验收
> 队形：单线形 solo——执行代理亲写零子代理、主会话守验收位
> 日期：2026-08-31

## 一、问题陈述 {#problem}

GOV-002 退出标准第一条要求任务包 007 至 013 执行完毕并关闭。七包即 doclint 集成、分词层、语义树、判定器、规则、NPC 编排、分歧度量，为八阶段路线早期形态。其实体多已被组件架构承接交付即核阅承 doclint、句读承分词与语义树、得一承判定与规则、秤星承分歧度量、代理编组承多代理编排，但包级账面仍挂活跃态于 sih/state/plan/tasks/，账实落差使退出标准第一条不可机械判定，主线 v1 结算的判据账本缺一块。

## 二、关键设计 {#design}

四件。一逐包对账四列即原目标、现承接位、证据、处置，证据机械可回查即文件在场或链上事件号或已签令源，处置三值即承接闭项、部分承接闭项、废弃有据，证据缺席即如实写废弃待立不硬凑。二两态归位即每包头部加关闭裁定块后 git mv 至 sih/event/plan/tasks/，名随源不改承 DEC-001 修订四两态判据即消费完成度。三GOV-003 编排节结算追加升 v1.4 即记退出标准第一条账面进度与七包处置概览，GOV-002 判据文本零改。四结果档上链即对账表与完成度表与队形验证一行。

## 三、工作清单 {#work}

- [ ] 七包逐包查证即原目标复读、承接位定位、证据落列
- [ ] 每包头部关闭裁定块与两态归位迁移
- [ ] GOV-003 v1.4 结算追加
- [ ] 结果档与管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 对账表 | 工程治理 | 七包四列齐、证据逐条可机械回查即文件在场或事件号在链或令源已签、验收方抽两条复跑一致 |
| **F-2** 关闭裁定块 | 工程治理 | 每包头部块含裁定、承接位、证据引用、批号引用、日期、原文主体零改动 |
| **F-3** 两态归位 | 工程治理 | 七包移至 sih/event/plan/tasks/ 且名不改、state/plan/tasks 不再含 007 至 013 |
| **F-4** 记账落链 | 工程治理 | GOV-003 升 v1.4 结算追加、结果档在档、意图与认证入 08-31 链、结算 commit 在册、链 verify valid |
| **F-5** 验收门 | 工程治理 | 化格与核阅与检词全绿即 GOV-003 在 des-001 域必过核阅、reconcile unrouted 零、findings 逐件读过不只看退出码 |

## 五、必读文件 {#read}

- 必读 1：sih/state/plan/tasks/ 七包本体即 task-007 至 task-013
- 必读 2：doc/governance/GOV-002-mainline-lock-v1.md 即退出标准第一条原文
- 必读 3：doc/governance/GOV-003-fullstate-course-v1.md 即编排节与结算追加纪律
- 必读 4：doc/decision/001-repository-structure.md 修订四即两态判据与归位规则
- 必读 5：doc/decision/016-parser-initiation.md 与 019-tally-naming.md 与 020-deyi-component-naming.md 即承接位令源
- 必读 6：sih-tools/SETTLEMENT-001.md 与 sih/event/plan/judouimpl 与 tallyimpl 等结果档即承接证据取材

## 六、约束 {#constraints}

1. 七包原文主体零改动即只加头部关闭裁定块
2. 不虚报承接即证据缺席如实写废弃待立、废弃处置须引已签令源
3. GOV-002 判据文本零改即只在对账档与 GOV-003 结算追加记账
4. task-001 至 006 不动即超出本批范围、结果档观察节如实记
5. AGENTS.md 零改动即并行会话可能仍持锁
6. 上链前必须等绿

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过即主会话验收复跑

## 八、风险点 {#risks}

承接判定是把实体去向映射到既有已签令源，不是新裁决；映射不成立的包如实写废弃待立留人节点，宁缺毋滥。GOV-003 属治理文档即核阅 des-001 域，格式违规即返工不跳过。

## 九、队形声明 {#formation}

单线形即执行代理亲写零子代理，主会话守验收位不代写。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 令
- 链件：sih/event/trail/2026-08-31.ndjson 即本批意图与认证
- 关联：GOV-002 退出标准、DEC-001 修订四、DEC-016 与 DEC-019 与 DEC-020、SETTLEMENT-001

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/tasks/
- sih-engine/sih/event/plan/tasks/
- sih-engine/sih/state/plan/pkgclose-solo.md
- sih-engine/sih/event/plan/pkgclose-solo-results.md
- sih-engine/doc/governance/GOV-003-fullstate-course-v1.md
- sih-engine/sih/event/trail/2026-08-31.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[对账收口]: 消解 即工作名直述即七包账实对齐与归档、不做登记
叩问处置[对账]: 消解 即 recall 面系承接加 GOV-003 加任务包加归档加结算六主题面即二百三十六命中、对账主题真零命中如实记、对账语义锚由 GOV-002 退出标准第一条原文与任务包第二节四列设计直载、不做登记
叩问处置[两态归位]: 做承不做立 即 DEC-001 修订四已立判据并承 AGENTS.md 文件索引行、术语包未登属包侧覆盖缺口、本批承已立名不登记
叩问处置[关闭裁定]: 消解 即工作名直述即七包关闭裁定的判定动作、不做登记
叩问处置[关闭裁定块]: 消解 即工作名直述即本批新增头部块、不做登记
叩问处置[四列对账表]: 消解 即工作名直述即任务包第二节设计结构、不做登记
叩问处置[废弃待立]: 消解 即工作名直述即任务包第二节设计处置三值之证据缺席值、不做登记
叩问处置[承接闭项]: 消解 即工作名直述即任务包第二节设计处置三值之一、不做登记
叩问处置[现承接位]: 消解 即工作名直述即任务包第二节设计四列之二、不做登记
叩问处置[结算追加]: 做承不做立 即 GOV-003 增长纪律已立即每段结算后追加本向界、本批承该纪律不登记
叩问处置[账实落差]: 消解 即工作名直述即任务包第一节问题陈述、不做登记
叩问处置[部分承接闭项]: 消解 即工作名直述即任务包第二节设计处置三值之二、不做登记
