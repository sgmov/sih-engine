# autoflow2-solo：去人类节点实验流两件批

> task-packages 治理任务
> 承接：用户 2026-09-02 令即两件全过得一裁一过一执行一未过进泊界——去人类节点实验工作流首次全流程实测、predsplitAB 四枚 stable_clear 终签在链
> 队形：单线形 solo——委外代理亲写零子代理，两件串行
> 日期：2026-09-02

## 一、问题陈述 {#problem}

用户实验令：件一谓词融回与件二命题写法规范，各经得一裁，裁过即执行，未过进泊界。这是治理回路首次以「裁-行-泊」三态自动化分流替代人类节点逐件放行。四枚 pred 系终签已在链即件一过裁凭据成立。

## 二、关键设计 {#design}

件一（凭据在手直执行）：谓词融回实装连跑——SPEC-015 路择谓词融回落差规格（SDD）→ 实装（TDD）→ 切换，三步照 SPEC-014 与核阅融回形制。落点三件：路择 core 与 parking 两谓词包纯数据随迁 src/attractor/packs/，attractor 增 route 谓词装载与判定入口（可插拔机制即规则包形态承 scrutinator 先例）；截流谓词族融三问模块即 src/ask3repeater 增截流谓词装配位（按轮判定接口，形式承 GOV-002 判据三文本与 selector predicates.py 形）；过程件归零即 f-anchors-x11-t6d.md 归档入 sih/event/plan/ 过程件档。金向量脏目标条款与同参形条款全程适用，围堰 selector 为唯一基准，留堰件零改动。

件二（先裁后行）：命题「得一命题选题写法规范应立为引擎向界层规范文件即 SPEC-016」先过得一裁（gid predspec-guard-1，全流程引擎件，单锚 baseline_4，九发独立重采，对己不利声明前置即规范将约束本席位后续命题起草），stable_clear 即执行落 SPEC-016-attractor-proposition-drafting.md（内容骨架即四场实测：一命题一裁决依据族、禁跨主题捆绑、双述不罚、frontmatter 必载项、对己不利声明模板、身份漂移观察附录），boundary 或不过即进泊界 pk-039 停泊（exit_condition 即用户裁规范另立载体或重拟）。

## 三、工作清单 {#work}

- [x] 件一 SPEC-015 落档走管线
- [x] 件一 TDD 实装与金向量
- [x] 件一 切换与 GOV-002 判据二三五闭项与 GOV-003 v1.8
- [x] 件二 得一裁 predspec-guard-1
- [x] 件二 分流：过即 SPEC-016，不过即 pk-039 进泊（实态 pk-039 被占改 pk-040 进泊，偏差申报）
- [x] 双仓收口全链

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 件一 SDD | 工程治理 | SPEC-015 七节全承 014 体例、管线域内零违规 |
| **F-2** 件一 TDD | 工程治理 | 路由谓词与截流谓词实装、金向量含脏目标、围堰 selector 零改、cargo test 全绿 |
| **F-3** 件一 切换 | 链上治理 | 完成档三查、GOV-003 v1.8 判据二三五闭项表述、过程件归零实证 |
| **F-4** 件二 裁决 | 链上治理 | predspec-guard-1 全流程引擎件、结论如实 |
| **F-5** 件二 分流 | 链上治理 | 过即 SPEC-016 落档走管线，不过即 pk-039 停泊事件入链，两态必居其一 |
| **F-6** 收口 | 链上治理 | 双仓 settle 归并、reconcile 四零、链 valid、全量入版控 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/spec/SPEC-014-attractor-mergeback-gap.md 即规格体例
- 必读 2：sih-tools/selector/src/selector/{predicates,route,pack}.py 与 packs/{core,parking}/ 即融回基准
- 必读 3：sih-tools/facet/facet_task_packages/predsplit-a2/topic.md 即单锚命题形制

## 六、约束 {#constraints}

1. 四枚 pred 终签是件一唯一开工凭据，件二无凭据必先裁（红线）
2. 围堰 selector 与 facet 与 tally 源码零改动（红线）
3. 金向量冻结零漂移、围堰输出唯一基准（红线）
4. 守卫在位严禁直提、撞锁显式 --session 即撞即停批、链尾对表
5. 件二禁钓样本、不过不硬签（红线）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过

## 八、风险点 {#risks}

截流谓词融三问的接口形态无先例即 SPEC-015 需按 GOV-002 判据三文本与 predicates.py 形式自钉，规格不清即宁窄勿宽留泊。件二命题自身涉规范即元层自指，对己不利声明须双倍显式。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理，两件串行即件一三步毕接件二。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 去人类节点实验流令
- 链件：随批意图入当日链
- 关联：predsplitAB 四终签、SPEC-014 体例、GOV-002 判据二三五、PARKING 名册

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[裁行泊]: 消解 即大白话直述即裁决执行停泊三态分流，非登记术语
叩问处置[谓词包]: 消解 即大白话直述即 routes.toml 加 manifest 成对的谓词纯数据目录，GOV-002 既有词可插拔机制的承载形，非登记术语
叩问处置[截流谓词]: 消解 即大白话直述即按轮判定截流判定所用的谓词，GOV-002 判据三既有表述「截流谓词族」，非登记术语
叩问处置[命题写法规范]: 消解 即大白话直述即命题起草时选题与写法的成文规矩即本批件二对象，非登记术语
叩问处置[可插拔]: 消解 即 GOV-002 既有词
叩问处置[元层自指]: 消解 即大白话直述即规范约束自身起草，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/spec/SPEC-015-predicate-mergeback-gap.md
- sih-engine/src/attractor/
- sih-engine/src/attractor/packs/
- sih-engine/src/bin/attractor.rs
- sih-engine/src/ask3repeater/
- sih-engine/src/lib.rs
- sih-engine/Cargo.toml
- sih-engine/Cargo.lock
- sih-engine/tests/
- sih-engine/sih/event/mergeback/mergeback-predicate-completion-2026-09-02.md
- sih-engine/doc/governance/GOV-003-fullstate-course-v1.md
- sih-engine/doc/spec/SPEC-016-attractor-proposition-drafting.md
- sih-engine/sih/state/plan/autoflow2-solo.md
- sih-engine/sih/event/plan/autoflow2-solo-results.md
- sih-engine/sih/event/plan/autoflow2-solo-materials/
- sih-engine/task-packages/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/selector/CALL-LOG.md
- sih-tools/BATCH-FACE.md
- sih-tools/parking/materials/
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/facet/CALL-LOG.md
- sih-tools/tally/CALL-LOG.md
- sih-tools/meter/counts/
