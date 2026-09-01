# deyimerge-sdd-solo：得一融回规格批

> task-packages 治理任务
> 承接：用户 2026-09-02 得一裁融回令即开门、DEC-013 融回门机制、GOV-002 退出标准判据一判定器席位、贡献度证据即 adisp-guard-1 与 m-p3xcarr 两笔非自证 stable_clear 在链
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-02

## 一、问题陈述 {#problem}

判定器席实例得一（facet 上层采样观察 + tally 下层确定性核对终签）仍在围堰即 sih-tools/facet 与 sih-tools/tally，GOV-002 退出标准判据一判定器席缺位，判据二至五全挂其后。用户开门，照 DEC-013 机制与核阅融回先例走三步曲第一步即规格先行。

## 二、关键设计 {#design}

本批只产 SPEC-014 得一融回落差规格，零实现。规格必备七节：

一即家位与模块形：库模块 src/attractor/ 与二进制同名（代码标识符 attractor 承 DEC-020，治理名得一不变），采样观察腿与机械核对腿分层即 llm_client 与采样 runner 留围堰双模不融回（组件层零 LLM 纪律），纯机械层即 contract_mode 与 facet_stats 判定函数与 tally 的 R1-R7 与 sign 与 crosscheck 落 src/attractor/。二即接口契约对表：facet CONTRACT-MODE-SPEC 的 emit-contract/answer/score 两段式与 tally CONTRACT 的 assemble/check/verify/sign/watch 与 crosscheck 事件十五字段负载逐条对表零语义漂移。三即双模并存条款：facet 单独发布能力保留即 CLI 围堰原位，引擎侧模块接口可被组件层调用即判据四；采样腿在围堰经进程边界调用，模块边界显式。四即验收判据：金向量逐字节一致（含发现材料的合同哈希与响应哈希与终签哈希）、退出码对齐、多包加载（des-011 判定规约包随迁）、不变量即机械腿零网络零 LLM、跨腿契约即 score 报告字段与 tally check 输入逐字段兼容。五即金向量脏目标条款承 SPEC-013 修订四教训：判定材料金向量须含挂起与失败与告警非净目标。六即回迁债：req 与 llm_client 依赖切分清单、跨仓子进程调用形态、DEC-001 归位映射行。七即测试计划 T1 至 T6 先红后绿。

## 三、工作清单 {#work}

- [ ] 查档与接口盘点即两 CONTRACT 与 SPEC 全读
- [ ] SPEC-014 落档走管线
- [ ] 收口

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 规格 | 工程治理 | SPEC-014 七节全、两工具契约逐条对表、家位 attractor、双模条款在场 |
| **F-2** 教训承接 | 工程治理 | 金向量脏目标条款与同参形条款显式在场 |
| **F-3** 管线 | 链上治理 | 化格核阅检词即域内零违规、认证入链 |
| **F-4** 收口 | 链上治理 | 双仓 settle 归并、reconcile 双零、链 valid、包档入册 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/facet/docs/CONTRACT-MODE-SPEC.md 即采样腿契约
- 必读 2：sih-tools/tally/CONTRACT.md 即机械腿契约
- 必读 3：sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md 即规格体例先例

## 六、约束 {#constraints}

1. 零实现即本批只落规格（红线）
2. 采样腿留围堰即引擎组件零 LLM 纪律（红线）
3. 两工具源码零改动（红线）
4. 上链前必须等绿、findings 亲读、禁管道掩退出码

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

采样腿与机械腿边界划错即融回后违零 LLM 纪律，防御即规格显式列腿切分清单且 TDD 批按清单逐条验。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 得一裁融回令
- 链件：随批意图入当日链
- 关联：DEC-013、DEC-020、SPEC-013 先例、adisp-guard-1 与 m-p3xcarr 终签

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[采样腿]: 消解 即大白话直述即 LLM 采样观察层，非登记术语
叩问处置[机械腿]: 消解 即大白话直述即确定性核对层，非登记术语
叩问处置[腿切分]: 消解 即大白话直述即两层边界清单，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/spec/SPEC-014-attractor-mergeback-gap.md
- sih-engine/sih/state/plan/deyimerge-sdd-solo.md
- sih-engine/sih/event/plan/deyimerge-sdd-solo-results.md
- sih-engine/sih/event/plan/deyimerge-sdd-solo-materials/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
