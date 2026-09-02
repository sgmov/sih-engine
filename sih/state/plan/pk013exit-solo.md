# pk013exit-solo：pk-013 里程碑修订窗出泊批

> task-packages 治理任务
> 承接：用户 2026-09-02 措辞裁令即删数量为二留新增须过边界命题同款测量、pk-013 出泊条件即主线里程碑结算时用户裁、主线 v1 已结算
> 队形：单线形 solo——主会亲写零子代理
> 日期：2026-09-02

## 一、问题陈述 {#problem}

pk-013 出泊条件已触发即主线 v1 结算完成，用户裁冻结清单首项删「数量为二即 facet 与路择」保留「新增须过边界命题同款测量」。执行修订与出泊记账与金向量随冻。

## 二、关键设计 {#design}

三件。一即 GOV-002 修订：冻结清单首项改「新增信息洪流工具须过边界命题同款测量」，版本节追记 v1.7 即用户裁决承载；二即金向量随冻：des-001-gov002.json 以引擎件实跑新快照重录，断言逻辑零动，双跑复验 IDENTICAL；三即出泊记账：pk-013-exit.json 落档、出泊事件经书简 park 通道入链载裁决文即用户原话、PARKING-v1 名册转历史住户。

## 三、工作清单 {#work}

- [ ] GOV-002 修订与金向量重录
- [ ] 出泊记账与名册更新
- [ ] 管线认证与收口

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 修订 | 工程治理 | 冻结清单首项新文在、v1.7 句在、其余条款零碰 |
| **F-2** 金向量 | 工程治理 | des-001-gov002.json 重录后 golden 测试转绿、双跑 cmp IDENTICAL |
| **F-3** 出泊 | 链上治理 | 出泊事件入链载裁决文、名册更新、exit json 在档 |
| **F-4** 收口 | 链上治理 | 双仓 settle 归并、reconcile 四零、链 valid、全量入版控 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/governance/GOV-002-mainline-lock-v1.md 即修订对象
- 必读 2：sih-tools/parking/records/ 即 pk-013 进泊事件指针

## 六、约束 {#constraints}

1. 其余冻结条款与范畴排除零碰（红线）
2. 断言逻辑零动即只重录快照数据（红线）
3. 守卫在位严禁直提、链尾对表、撞锁显式 --session

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

低。金向量重录遗漏即测试红暴露自证。

## 九、队形声明 {#formation}

单线形即主会亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 措辞裁令
- 链件：随批意图入当日链
- 关联：SETTLEMENT-V1、pk-013 泊档、SPEC-013 修订四金向量条款

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[随冻]: 消解 即大白话直述即同批重录快照，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md
- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/src/scrutinator/fixtures/golden/des-001-gov002.json
- sih-tools/parking/materials/pk-013-exit.json
- sih-engine/sih/state/plan/pk013exit-solo.md
- sih-engine/sih/event/plan/pk013exit-solo-results.md
- sih-engine/sih/event/plan/pk013exit-solo-materials/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
