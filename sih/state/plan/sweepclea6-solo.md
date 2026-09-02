# sweepclea6-solo：六小件裁行泊流水批

> task-packages 治理任务
> 承接：用户 2026-09-02 全部批准一次修完令、去人类节点实验流批量实战令、predspec2 与 predsplitAB 先例
> 队形：单线形 solo——委外代理亲写零子代理，六命题流水过闸
> 日期：2026-09-02

## 一、问题陈述 {#problem}

六件工程收尾小件经用户全部批准，按裁行泊三态流水执行：每件拟命题过得一裁，过即当场执行，不过入泊界。另含 assettwave-a 残局收拾（非命题件，欠账直接执行）。

## 二、关键设计 {#design}

六命题五执行件。件一 assettwave-a 残局（欠账执行不出裁）：lease status 核会话 ede05ac82f712700 的 close_failed 态，close --force 或按台账语义补 revoked 行，fmt-registry 与 nom-registry 两笔回执经引擎 scribe append 补链（载体在 scribe/reports 在档），主树 assetwave 系 43 件 untracked 逐件盘点入册或确认属 identity 除外类，assettwave-a-solo 工地拆净。件二 BATCH-FACE 两坑：meter --quiet 位形坑（旗标须置子命令前）加 open 撞陈旧会话先 close 坑，两行入坑位速查表。件三 U+U+2026 双前缀双侧同步修：工具件 scrutinator Python 侧消息格式源（先定位双前缀生成位）与引擎侧 rule.rs 注释与断言金向量三处同步修，修后重录受影响金向量并 cargo test 全绿加双跑 cmp IDENTICAL（同参形）。件四 viewrider 两陈旧会话清理：72e2e40cd689 与 4b5ff07f7db4 零锁在持，close --force 补台账 revoked 行，msh/viewrider-solo 残支删。件五 散件回灌：sealwin3-solo-materials 三 dispatch、deyimerge-switch materials 内 postmerge 两件、predicate 与 autoflow 各批在途散件逐件入册（identity 除外）。件六 listzero CONTRACT 补笔：tally CONTRACT.md 修订记录补五修（签署印守卫）一笔，源即 5bfb95b6 与 listzero 批 diff。

流水形：件一先行（欠账）；件二至六各拟命题（单锚 baseline_4，gid sweep-2 至 sweep-6，九发独立重采，全流程引擎件，对己不利声明前置——件三修双前缀即修本席位曾照抄的瑕疵），过即执行不过即入泊 pk-042 续号。命题写法承 SPEC-016 即刚生效规范首用。

## 三、工作清单 {#work}

- [ ] 件一 assettwave-a 残局收拾
- [ ] 件二至六五命题流水过闸
- [ ] 过件当场执行与验证
- [ ] 双仓收口

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 件一 | 链上治理 | 会话 revoked 行在、两笔回执补链、43 件盘点处置清、工地拆净 |
| **F-2** 流水 | 链上治理 | 五命题各全流程引擎件出裁，gate 如实，过签不过泊两态必居其一 |
| **F-3** 执行 | 工程治理 | 过件逐件执行即 BATCH-FACE 两行、双前缀三处同步修加重录加双跑 IDENTICAL、两会话 revoked 加残支删、散件入册、CONTRACT 补笔 |
| **F-4** 收口 | 链上治理 | 双仓 settle 归并、reconcile 四零、链 valid、全量入册、inputlog 逐字 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/spec/SPEC-016-attractor-proposition-drafting.md 即命题写法规范首用
- 必读 2：sih-engine/sih/event/plan/assetwave-a-solo-results.md 即件一欠账底册
- 必读 3：sih-tools/BATCH-FACE.md 即机械链正典

## 六、约束 {#constraints}

1. 全流程引擎件出裁，九发独立重采（红线）
2. 不过不硬签入泊 pk-042 续号（红线）
3. 引擎 scrutinator 改动限注释与测试与金向量数据，规则语义零变（红线）
4. 守卫在位严禁直提、撞锁显式 --session 即撞即停批、链尾对表

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

件三金向量重录面即消息文本变更波及全部含 U+ 字符发现的金向量，防御即先扫受影响金向量清单再改，逐件重录围堰基准。件一 43 件盘点误收编风险，防御即 identity 与存量除外清单逐件对。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 全部批准一次修完令
- 链件：随批意图入当日链
- 关联：SPEC-016、assettwave-a 结果档、pk-040 泾例、SPEC-013 修订四

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[流水]: 消解 即大白话直述即逐件过闸，非登记术语
叩问处置[残局]: 消解 即大白话直述即未竟收尾，非登记术语
叩问处置[双前缀]: 消解 即前批已消解
叩问处置[回执]: 消解 即大白话直述即认证记录，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/sweepclea6-solo.md
- sih-engine/sih/event/plan/sweepclea6-solo-results.md
- sih-engine/sih/event/plan/sweepclea6-solo-materials/
- sih-tools/BATCH-FACE.md
- sih-tools/scrutinator/src/scrutinator/
- sih-tools/scrutinator/tests/
- sih-engine/src/scrutinator/rule.rs
- sih-engine/src/scrutinator/fixtures/golden/
- sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md
- sih-engine/src/scrutinator/tests.rs
- sih-engine/sih/state/plan/assetwave-a-solo.md
- sih-engine/sih/state/plan/assetwave-b-solo.md
- sih-engine/sih/state/plan/assetwave-c-solo.md
- sih-engine/sih/state/plan/assetwave-d-solo.md
- sih-engine/sih/event/plan/assettwave-a-solo-results.md
- sih-engine/sih/event/plan/sealwin3-solo-materials/
- sih-engine/sih/event/plan/deyimerge-switch-solo-materials/
- sih-engine/sih/event/plan/predmerge-guard-solo-materials/
- sih-engine/sih/event/plan/scrutmerge-switch-solo-materials/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/lease/ledger/
- sih-tools/lease/CALL-LOG.md
- sih-tools/facet/probes/calibration/ledger.jsonl
- sih-tools/scribe/reports/
- sih-tools/tally/CONTRACT.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/facet/facet_task_packages/sweep-2/
- sih-tools/facet/facet_task_packages/sweep-3/
- sih-tools/facet/facet_task_packages/sweep-4/
- sih-tools/facet/facet_task_packages/sweep-5/
- sih-tools/facet/facet_task_packages/sweep-6/
- sih-tools/proposition/DES/sweep-2/
- sih-tools/proposition/DES/sweep-3/
- sih-tools/proposition/DES/sweep-4/
- sih-tools/proposition/DES/sweep-5/
- sih-tools/proposition/DES/sweep-6/
- sih-tools/facet/CALL-LOG.md
- sih-tools/tally/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
