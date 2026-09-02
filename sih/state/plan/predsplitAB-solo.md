# predsplitAB-solo：谓词融回双路重裁批

> task-packages 治理任务
> 承接：用户 2026-09-02 双令即两条重测路各走一遍并进、predmerge-guard boundary 打回重作、用户命题规范意向即命题选题写法规范将入向界（另批承载）
> 队形：单线形 solo——委外代理亲写零子代理（两路各为独立裁决，串行执行防锁冲突）
> 日期：2026-09-02

## 一、问题陈述 {#problem}

predmerge-guard-1 九发全 comply 但依据三散即 baseline_5×4 与 baseline_1×3 与 baseline_4×2，闸判 boundary 打回重作。归因即三事捆绑命题过宽且各事裁决基线不同。用户令两路并走：路 A 拆三子命题各裁各签；路 B 法层 refine 归约单一基线重投。两路结论对照即命题写法规范的实测输入。

## 二、关键设计 {#design}

两路全流程引擎件出裁，串行执行。

路 A（拆分三裁）：三子命题各走 emit-contract 加九发加 score 加 check，stable_clear 即 sign 终签（gid 各异即 predsplit-a1 路择谓词融判定器、predsplit-a2 截流谓词族融三问、predsplit-a3 过程件归零），各子命题单锚单基线：a1 锚 baseline_4 与 baseline_5 双述即路择谓词纯数据迁移的性质自明；a2 锚 baseline_5 即截流融三问减少 LLM 参与；a3 锚 baseline_4 即过程件归零可验证性收口。子命题写法承单基线纪律即一命题一裁决依据族。

路 B（法层 refine）：原命题归约重写为单一判据锚即「路择谓词件与截流谓词族与过程件归零三事应经可插拔机制融回引擎侧（GOV-002 判据二三五整体闭项）」锚死 baseline_5 单基线重裁，gid predmerge-refine-b1。refine 只动命题锚不定结论，九发重采禁复用旧响应。

两路对己不利声明与禁钓样本与 seat 基线对表（当日基线 predmerge-guard-1 标定件同席同哈希可复用，R5 对表）承先例。两路结论四象限如实申报即双过、双不过、A 过 B 不过、B 过 A 不过，各象限处置建议入结果档供用户与规范批取数。

## 三、工作清单 {#work}

- [x] 路 A 三子命题起草与三场裁决
- [x] 路 B refine 命题重写与一场裁决
- [x] 终签入链与四象限对照档

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 路 A | 工程治理 | 三子命题各全流程引擎件出裁，各 gate_verdict 与 disposition 如实，stable_clear 即签即 crosscheck 事件各异 gid |
| **F-2** 路 B | 工程治理 | refine 命题单基线重投全流程，结论如实 |
| **F-3** 对照 | 工程治理 | 四象限对照表入结果档，各象限处置建议在场 |
| **F-4** 纪律 | 链上治理 | 禁钓样本、对己不利声明、seat 基线对表、链尾对表、reconcile 四零 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/facet/facet_task_packages/predmerge-guard-1/ 与 sih-tools/proposition/DES/predmerge-guard-1/ 即前裁材料
- 必读 2：sih-engine/doc/governance/GOV-002-mainline-lock-v1.md 即判据原文
- 必读 3：sih-engine/sih/event/plan/predmerge-guard-solo-results.md 即 boundary 归因

## 六、约束 {#constraints}

1. 全流程引擎件出裁（红线）
2. 通过即签不过即停即不立项不写规格（红线）
3. selector 与 facet 与 tally 与引擎源码零改动（红线）
4. 守卫在位严禁直提、撞锁显式 --session 即撞即停批
5. 禁钓样本禁改答即每场九发独立重采

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

a2 截流融三问可能独立 boundary 即其接口设计确实需先行规格——该结论本身即有价值如实报。四发并列终态可能即双路同判，对照表如实载不硬凑差异。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理，两路串行。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 双路并走令
- 链件：随批意图入当日链
- 关联：predmerge-guard-solo boundary 前裁、命题写法规范批（候开）、GOV-002 判据二三五

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[单基线纪律]: 消解 即大白话直述即一命题一裁决依据族，非登记术语
叩问处置[四象限]: 消解 即大白话直述即双路结论对照形，非登记术语
叩问处置[法层 refine]: 消解 即大白话直述即闸路由法层对命题锚的归约重写形，非登记术语
叩问处置[子命题]: 消解 即大白话直述即捆绑命题拆分后的各裁各签单元，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/predsplitAB-solo.md
- sih-engine/sih/event/plan/predsplitAB-solo-results.md
- sih-engine/sih/event/plan/predsplitAB-solo-materials/
- sih-tools/facet/facet_task_packages/predsplit-a1/
- sih-tools/facet/facet_task_packages/predsplit-a2/
- sih-tools/facet/facet_task_packages/predsplit-a3/
- sih-tools/facet/facet_task_packages/predmerge-refine-b1/
- sih-tools/proposition/DES/predsplit-a1/
- sih-tools/proposition/DES/predsplit-a2/
- sih-tools/proposition/DES/predsplit-a3/
- sih-tools/proposition/DES/predmerge-refine-b1/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/facet/CALL-LOG.md
- sih-tools/tally/CALL-LOG.md
- sih-tools/meter/counts/
