# goldlim-refreeze-solo：引擎金向量 golden_des001mathe_lim001 随冻重录

> task-packages 治理任务
> 承接：mathrefmt-solo 段1（sih-math a4372c1）合法改写 LIM-001 引发引擎金向量期望漂移即 golden_des001mathe_lim001 冻结哈希 f609116b… 对盘上实哈希 a83b5ba8…，findings 两边皆零纯期望过期；主会验收发现（mathrefmt 结果档未申报此漂移，现登记）；pk-036 与 deyimerge-switch-solo 与 pendsweep-solo 随冻先例
> 用户裁定 2026-09-03 原话照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——本件纯机械即机械链全绿即放行不扰人节点
> 队形：单线形 solo
> 日期：2026-09-03
> 温故检索：materials/recall-goldlim.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，结果档须载冲突样本节（零撞锁如实记零）

## 一、问题陈述 {#problem}

cargo test --lib 现况 139 过 2 败：golden_des001mathe_lim001 败因 LIM-001 内容经 mathrefmt 合法变更（概览块移除加话题改定义节）后冻结期望 content_hashes 过期；test_evidence_file_path_form 单跑绿全量跑偶发即顺序依赖抖动另案观察。金向量语义即期望值须随合法内容变更重冻，消费逻辑零改。

## 二、关键设计 {#design}

1. 只刷 sih-engine/src/scrutinator/tests.rs 中 golden_des001mathe_lim001 期望对象的 content_hash 为盘上实哈希，断言与消费逻辑零改，承 pendsweep 随冻先例。
2. live 实跑对表：cargo test --lib 全绿（141 计数对表）；同参双跑一致。
3. evidence 测试抖动观察：全量跑复现即如实申报读数（单跑绿证据留档），处置另批不在本批。

## 三、工作清单 {#work}

- [x] 期望哈希重冻单点改
- [x] cargo test --lib 全绿读数加双跑一致
- [x] evidence 抖动观察读数（不改代码）
- [ ] 认证上链结算收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 单点改** | 工程 | tests.rs diff 仅期望哈希一处，消费逻辑零改 |
| **F-2 全绿** | 工程 | cargo test --lib 全绿含全部金向量，双跑一致 |
| **F-3 抖动如实** | 工程 | evidence 测试全量跑读数如实记，复现即申报不掩饰 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/src/scrutinator/tests.rs 金向量节
- 必读 2：sih-engine/sih/event/plan/pendsweep-solo-results.md 随冻先例（若在档）
- 必读 3：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 只刷期望值，禁改判定逻辑与测试结构
2. 认证先落主树活链，链 settle 前一次性拷工地
3. 撞锁有限重试如实计数
4. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-engine/src/scrutinator/tests.rs
- sih-engine/sih/state/plan/goldlim-refreeze-solo.md
- sih-engine/sih/event/plan/goldlim-refreeze-solo-results.md
- sih-engine/sih/event/plan/goldlim-refreeze-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 冲突样本节在结果档（零撞锁如实记零）
- [ ] 认证入链结算收约对表读数在档
