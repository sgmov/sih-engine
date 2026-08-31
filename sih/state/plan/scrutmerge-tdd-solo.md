# scrutmerge-tdd-solo：核阅融回 TDD 实现批

> task-packages 治理任务
> 承接：用户 2026-08-31「核阅开」 + 批一 SPEC-013 主会验收实质通过 + crosscheck-m-mbgate-scrut 终签 a0bbcb40
> 队形：单线形 solo——执行代理亲写零子代理、主会验收位
> 日期：2026-08-31

## 一、问题陈述 {#problem}

批一 SPEC-013 已主会验收实质通过。本批按规格用 Rust 实现引擎侧核阅：src/核阅/ 库模块加 src/bin/scrutinator.rs 命令行面，Cargo.toml 加 toml 依赖。TDD 六组逐判据先红后绿，红态证据存档，绿态以金向量对表与真实狗粮逐字节一致。实现批完成后停下等切换放行令，不得自行进切换批。

## 二、关键设计 {#design}

六件。一 T1 金向量冻结先行即工具件对三包各二件真实目标生成期望输出，落 src/核阅/fixtures/golden/，承规格 § 验收判据 A1 零豁免面（engine.version 字段不要求逐字节一致）。二规格三处修正走本批管线即 mathe 包规则重数与承继重声明关系写清、两同名「验收判据」节并节或改名消歧义、规则包装载方式定案（include_str! 编译期内嵌 vs 运行时路径）。三 T2 逐字节一致即同包同目标下引擎件与工具件 JSON 报告逐字段对表除 engine.version 外。 四 T3 退出码五场景即合规 / 空载 / 单违规 / 域外 / 缺包 五场景同输入同退出码。五 T4 多包加载与归因即 des-001 加 des-001-mathe 双包同载，发现逐条携带正确 pack_name。六 T5 不变量即运行全程无网络调用无目标仓写入，进程监视机械验证。

## 三、工作清单 {#work}

- [ ] 任务包与三处规格修正入档
- [ ] T1 金向量冻结三包各二件
- [ ] T2 同包同目标逐字节一致
- [ ] T3 退出码五场景
- [ ] T4 多包加载与归因
- [ ] T5 不变量
- [ ] T6 报告 content_hashes 兼容
- [ ] 真实狗粮对表逐字节
- [ ] 管线化格+核阅+检词
- [ ] 认证入链+结算+收约+对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** T1 金向量冻结 | 工程 | 工具件对三包各二件真实目标生成期望输出，落 fixtures/golden/，三包六件在档 |
| **F-2** T2 逐字节一致 | 工程 | 同包同目标下引擎件与工具件 JSON 报告除 engine.version 外逐字段一致 |
| **F-3** T3 退出码五场景 | 工程 | 合规零 / 空载零 / 单违规一 / 域外二 / 缺包二 五场景同输入同退出码 |
| **F-4** T4 多包归因 | 工程 | des-001 加 des-001-mathe 双包同载，发现逐条携带正确 pack_name |
| **F-5** T5 不变量 | 工程 | 运行全程无网络调用无目标仓写入，进程监视机械验证 |
| **F-6** T6 content_hashes 兼容 | 工程 | 报告 content_hashes 字段含目标 SHA-256 且与 scribe append 八项负载兼容 |
| **F-7** 真实狗粮 | 工程 | doc/governance 一件 + ask3 记录件各跑引擎件对工具件 diff 逐字节一致（除 engine.version） |
| **F-8** 收口 | 链上治理 | 全程租约零直写零裸 commit、双仓 routed、链 verify valid、unrouted 净增零、工具件零改、scrutmerge-sdd 段1 65d3b4d 与 viewfix 段1 60a6149 与 viewrider 段3 1a778f1 零触碰 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/spec/SPEC-013-核阅-mergeback-gap.md
- 必读 2：sih-tools/scrutinator/CONTRACT.md
- 必读 3：sih-tools/scrutinator/packs/{des-001,des-001-mathe,ask3}/ 规则包
- 必读 4：sih-tools/scrutinator/src/scrutinator/ Python 实现
- 必读 5：sih-engine/Cargo.toml（必加 toml 依赖）
- 必读 6：sih-engine/src/lib.rs（库模块导出）

## 六、约束 {#constraints}

1. 工具件 Python 实现零改动即零字修改，归属于人节点处置由切换批执行
2. 上链前必须等绿、findings 亲读、禁管道掩退出码
3. 范围闸若拦即零提交收约改包重开
4. 全程租约零直写零裸 commit，主树只是草稿位
5. 工地与收约铁律：所有待提交文件先复制进 worktrees/ 工地，工地内 git add 后 lease commit 提交
6. 与并行批撞锁即报不绕行

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-8 全过即主会验收位复跑

## 八、风险点 {#risks}

- include_str! 编译期内嵌 vs 运行时路径定案需在规格中写明取舍理由
- mathe 包 20 件规则（12 件 des-001 承继 + 1 M008 + 7 M010 系列）需在规格中重数与承继关系写清
- 两同名「验收判据」节（acceptance 五条 A1-A5 与 acceptance-criteria 六条 A1-A6）需并节或改名消歧义
- TDD 全程必须先红后绿，红态证据存档
- 工地提交铁律 — 本工作区已五犯，本批必须严格工地提交，禁止主树直写
- 与 mtreeaudit 批在追认层可能撞链

## 九、队形声明 {#formation}

单线形即执行代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31「核阅开」+ 批一 SPEC-013
- 链件：sih/event/trail/2026-08-31.ndjson
- 关联：DEC-013 融回门三步、SPEC-006 书简融回先例、scrutmerge-sdd-solo 段1、mtreeaudit-solo 追认批

## 十一、请求写入 {#requested-writes}

- sih-engine/src/核阅/
- sih-engine/src/bin/scrutinator.rs
- sih-engine/src/lib.rs
- sih-engine/Cargo.toml
- sih-engine/Cargo.lock
- sih-engine/doc/spec/SPEC-013-核阅-mergeback-gap.md
- sih-engine/src/核阅/fixtures/golden/
- sih-engine/tests/
- sih-engine/sih/state/plan/scrutmerge-tdd-solo.md
- sih-engine/sih/event/plan/scrutmerge-tdd-solo-results.md
- sih-engine/sih/event/trail/2026-08-31.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

- 叩问处置[scrutmerge-tdd-solo]: 消解 即本批名 scrutmerge-tdd-solo 核阅融回 TDD 实现
- 叩问处置[核阅实现]: 消解 即工作名直述即用 Rust 重写核阅引擎侧 src/核阅/ 与 src/bin/scrutinator.rs
- 叩问处置[TDD 先红后绿]: 消解 即工作名直述即测试驱动开发的红态先存绿态后转
- 叩问处置[金向量冻结]: 消解 即工作名直述即对拍基线冻结的工程实践
- 叩问处置[规则包装载]: 消解 即工作名直述即 include_str! 编译期内嵌 vs 运行时路径的取舍
- 叩问处置[mathe 包承继]: 消解 即工作名直述即 des-001-mathe 12 件承继 des-001 加 8 件 mathe 新增共 20 件
- 叩问处置[并节消歧义]: 消解 即工作名直述即两「验收判据」节并节或改名消歧义
- 叩问处置[stop-the-world]: 消解 即 TDD 全绿后停下等切换放行令
- 叩问处置[工地提交铁律]: 消解 即工作名直述即工地内 git add 后 lease commit 的批纪律
- 叩问处置[unrouted 净增必零]: 消解 即工作名直述即本批新增 commit 必须带 session 号入 routed
- 叩问处置[工具件零改]: 消解 即工作名直述即 Python 工具件 src/scrutinator/ 零字修改
- 叩问处置[三笔历史零触碰]: 消解 即工作名直述即 65d3b4d / 60a6149 / 1a778f1 零字改动
- 叩问处置[TDD实现]: 消解 即工作名直述即 TDD 测试驱动开发的批模式
- 叩问处置[核阅融回]: 消解 即工作名直述即核阅从围堰外切态融回引擎侧
- 叩问处置[规则包迁移]: 消解 即工作名直述即 des-001/des-001-mathe/ask3 三包随件迁引擎侧
- 叩问处置[Rust重写]: 消解 即工作名直述即 Rust 引擎侧重写核阅组件
- 叩问处置[TDD先红后绿]: 消解 即工作名直述即测试先红态后转绿态
- 叩问处置[include_str]: 消解 即工作名直述即 Rust 编译期内嵌规则包文件
- 叩问处置[mathe包承继]: 消解 即工作名直述即 des-001-mathe 12 件承继 des-001 加 8 件 mathe 新增共 20 件
- 叩问处置[卸载点]: 消解 即工作名直述即 TDD 全绿后停下等切换放行令的批纪律
- 叩问处置[范围闸]: 消解 即工作名直述即 lease 范围四验拦即收约改包重开
- 叩问处置[规则包装载方式]: 消解 即工作名直述即 include_str! 编译期内嵌 vs 运行时路径的取舍
- 叩问处置[运行时路径]: 消解 即工作名直述即运行时从路径加载规则包文件
- 叩问处置[验收判据并节]: 消解 即工作名直述即两「验收判据」节并节或改名消歧义
