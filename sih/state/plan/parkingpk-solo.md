# parkingpk-solo：三件入全态泊界批

> task-packages 治理任务
> 承接：用户 2026-09-02 令「数学侧我们负责，其它问题入泊界」
> 队形：单线形 solo（主会话手跑）
> 日期：2026-09-02

## 一、问题陈述 {#problem}

三件未决事项入全态泊界续号：pk-036 golden_des001_gov003 金向量重录、pk-037 retriever 词面匹配语义升级评估、pk-038 listzero 与 legacytwo 任务书正身拷贝随批债。进泊经 scribe park 事件三笔（六字段齐），PARKING-v1 在泊名录投影随批刷新，出泊唯人节点本批只进不出。

## 二、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 三笔进泊 | 治理 | 链上 parking_entered 三事件 entry_id 恰 pk-036/037/038 六字段齐，重入门未触发 |
| F-2 投影刷新 | 治理 | PARKING-v1 名录含三新住户且出泊条件与链一致，三门过即化格核阅检词 |
| F-3 只进不出 | 治理 | 在泊 pk-013/016 未动，无 parking_exited 事件 |
| F-4 lease 链 | 治理 | engine 仓全程，verify 零，reconcile 较批前零新增，快照末笔后 |

## 三、请求写入 {#requested-writes}

- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/sih/state/plan/parkingpk-solo.md
- sih-engine/sih/event/plan/parkingpk-solo-results.md
- sih-engine/sih/event/plan/parkingpk-solo-materials/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- sih-tools/meter/counts/
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md

## 四、约束 {#constraints}

只进不出；不动在泊与历史住户记录；数学侧不泊；上链遇锁即等待；链快照在末笔后。

## 五、风险 {#risks}

PARKING-v1 改行过 des-001 若触发规则即以化格先行纠形；park 事件六字段缺一即机械门拒如实改后再进。
