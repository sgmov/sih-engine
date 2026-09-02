# predmerge-guard-solo：谓词融回得一裁批

> task-packages 治理任务
> 承接：用户 2026-09-02 得意裁过了就融令、GOV-002 判据二三五待闭、deyimerge-switch 判据一与四已闭
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-02

## 一、问题陈述 {#problem}

GOV-002 退出标准剩判据二（路择谓词融判定器）、判据三（截流谓词族融三问）、判据五（过程件归零）。用户令先过得一裁：以引擎件 attractor 出裁，过了即融，不过即停。

## 二、关键设计 {#design}

两段。一段即得一裁：命题「路择谓词件与按轮截流谓词族应经可插拔机制融回引擎侧判定器与三问模块，过程件归零」——用引擎件 attractor 全流程出裁即 emit-contract 出合同加席位九发作答加 score 计分加 tally 核对加机器终签 crosscheck 入链，gid 用 predmerge-guard-1，seat 基线先行（引擎件当日 seat 标定或承 adisp 当日基线对表）。基线材料即 GOV-002 五判据文本与 selector 谓词包形态（core 加 parking 两包 routes.toml 七谓词）与 engine.py 依赖倒挂实况。二段即裁后分流：stable_clear 加裁决通过即出 SDD 批（SPEC-015 谓词融回落差规格）进入融回三步曲；挂起或归人即停批报告不融，裁决材料留链。

## 三、工作清单 {#work}

- [ ] 命题起草与基线先行
- [ ] 引擎件 attractor 全流程出裁
- [ ] 终签入链与分流

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 裁决流程 | 工程治理 | 全流程经引擎件 attractor 即合同与计分与核对皆引擎件产出，禁用围堰件出裁 |
| **F-2** 终签 | 链上治理 | crosscheck 事件入链载 predmerge-guard-1、gate_verdict 与 disposition 如实即通过或挂起 |
| **F-3** 基线 | 工程治理 | seat 基线先行有对照、无钓样本、对己不利声明在案 |
| **F-4** 分流 | 链上治理 | 通过即 SDD 批立项声明入结果档；不通过即停批报告 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/governance/GOV-002-mainline-lock-v1.md 即退出标准原文
- 必读 2：sih-tools/selector/packs/core/routes.toml 与 packs/parking/routes.toml 即谓词形态
- 必读 3：sih-tools/facet/facet_task_packages/adisp-guard-1/topic.md 即命题形制先例

## 六、约束 {#constraints}

1. 裁决全流程引擎件（红线）即本裁本身是 attractor 融回后的首次实战自证
2. 不过即停即融回不因判据全绿强推（红线）
3. selector 与 facet 与 tally 源码零改动（红线）
4. 守卫在位严禁直提、撞锁显式 --session、链尾对表

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

命题即待裁事项的利益方含本席位即对己不利声明必须前置，adisp 先例照抄结构。引擎件 attractor 首次实战出裁或遇边界缺陷即如实挂起归人，缺陷转候补批不掩饰。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 得意裁过了就融令
- 链件：随批意图入当日链
- 关联：GOV-002 判据二三五、SPEC-014、adisp-guard-1 先例、selector 谓词包

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[可插拔]: 消解 即 GOV-002 既有词非新造
叩问处置[谓词融回]: 消解 即大白话直述即判定逻辑数据化迁移，非登记术语
叩问处置[对己不利声明]: 消解 即大白话直述即谱系披露节利益申报形制名，非登记术语
叩问处置[首战自证]: 消解 即大白话直述即引擎件融回后首次实战出裁自证，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/predmerge-guard-solo.md
- sih-engine/sih/event/plan/predmerge-guard-solo-results.md
- sih-engine/sih/event/plan/predmerge-guard-solo-materials/
- sih-tools/facet/facet_task_packages/predmerge-guard-1/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/facet/CALL-LOG.md
- sih-tools/tally/CALL-LOG.md
- sih-tools/meter/counts/
