# deyimerge-tdd-solo：得一融回实装批

> task-packages 治理任务
> 承接：用户 2026-09-02 批准令、SPEC-014 得一融回落差规格（deyimerge-sdd-solo 交付）、DEC-013 三步曲第二步、核阅 TDD 先例即金向量逐字节与先红后绿
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-02

## 一、问题陈述 {#problem}

SPEC-014 已立即家位 src/attractor/ 与腿切分清单二十三行与验收判据 A1-A5 与测试计划 T1-T6。本批实装：融回十一行功能重写为 Rust 落 src/attractor/ 与 src/bin/attractor.rs，金向量按脏目标条款冻结，des-011 判定规约包随迁，留堰十二行零碰。

## 二、关键设计 {#design}

五件。一即模块形承 SPEC-014 家位节：src/attractor/ 子模块即 contract_mode 合同内核加 stats 统计三族加 compiler 聚合加 validators 加 anchors 加 model_utils 加 paradigm_loader 加 tally 机械核对五子命令，src/bin/attractor.rs 命令行面承 ask3repeater 先例即子命令 score 加 check 加 verify 加 sign 加 watch 加 emit-contract。二即金向量冻结：以围堰原件为基准对真实判定材料生成期望输出冻结 src/attractor/fixtures/golden/，按脏目标条款含净一加脏三即挂起形与退回形与告警形加合同类拒收基线，含合同哈希与响应哈希与终签锚哈希逐字节。三即 des-011 判定规约包随迁即 manifest 加规则纯数据编译期内嵌承核阅先例。四即测试 T1-T6 先红后绿：T1 金向量冻结、T2 逐字节一致、T3 退出码对齐围堰、T4 多包归因、T5 不变量即机械腿零网络零 LLM、T6 跨腿契约 score 报告与 tally check 输入逐字段兼容。五即浮点文本形豁免承 SPEC-014 即 facet_stats 数值面字节级判据不适用于浮点文本形，结构面逐字节。

## 三、工作清单 {#work}

- [x] src/attractor/ 实装与 src/bin/attractor.rs
- [x] 金向量冻结含脏目标
- [x] des-011 包随迁
- [x] T1-T6 先红后绿
- [x] 双仓收口

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 实装 | 工程治理 | 融回十一行全落 src/attractor/，腿切分清单逐条验即融回件零依赖留堰件，留堰十二行与两工具源码零改动 |
| **F-2** 金向量 | 工程治理 | 金向量含净一加脏三加合同拒收基线，围堰件对同输入输出与引擎件逐字节（浮点文本形豁免外），证据入材料件 |
| **F-3** 测试 | 工程治理 | cargo test 全绿 T1-T6，红转绿迹入结果档 |
| **F-4** 收口 | 链上治理 | 双仓 settle 归并、reconcile 四类双零、链 valid、包档归档、构建探针即 cargo build 后实跑 attractor 一行验证 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/spec/SPEC-014-attractor-mergeback-gap.md 即全规格
- 必读 2：sih-tools/facet/src/{contract_mode,facet_stats*,compiler,validators,anchors,model_utils,paradigm_loader}.py 即融回基准
- 必读 3：sih-tools/tally/src/tally/cli.py 即机械腿基准

## 六、约束 {#constraints}

1. 留堰十二行与 facet 与 tally 全部源码零改动（红线）
2. 融回件零依赖 llm_client 与 req 与 env_loader（红线）
3. 金向量冻结后零漂移（红线）
4. 上链前必须等绿、findings 亲读、禁管道掩退出码
5. 守卫在位严禁直提、链尾对表、撞锁显式 --session 即撞即停批

## 七、验收标准 {#acceptance}

- [x] F-1 至 F-4 全过

## 八、风险点 {#risks}

Python 语义到 Rust 重写的边界形漂移即金向量判负，防御即以围堰输出为唯一基准禁自造。浮点豁免滥用即真漂移漏网，防御即豁免仅限浮点文本形且结构面逐字节。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 批准令
- 链件：随批意图入当日链
- 关联：SPEC-014、deyimerge-sdd-solo、核阅 TDD 先例、adisp-guard-1 终签材料

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[腿切分]: 消解 即 deyimerge-sdd 批已消解
叩问处置[随迁]: 消解 即大白话直述即随融回迁移，非登记术语
叩问处置[重写]: 消解 即大白话直述即同语义换语言实现，非登记术语
叩问处置[金向量]: 消解 即围堰原件跑出的期望输出冻结件，SPEC-014 已定义在案承核阅先例，非登记术语
叩问处置[归因]: 消解 即多包同载按 gid 独立累计谱系与预算互不串扰，承 tally CONTRACT 语义，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/src/attractor/
- sih-engine/src/bin/attractor.rs
- sih-engine/src/lib.rs
- sih-engine/Cargo.toml
- sih-engine/Cargo.lock
- sih-engine/tests/
- sih-engine/doc/spec/SPEC-014-attractor-mergeback-gap.md
- sih-engine/sih/state/plan/deyimerge-tdd-solo.md
- sih-engine/sih/event/plan/deyimerge-tdd-solo-results.md
- sih-engine/sih/event/plan/deyimerge-tdd-solo-materials/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
