# scrutmerge-sdd-solo：核阅融回规格先行批

> task-packages 治理任务
> 承接：用户 2026-08-31 令"核阅开" + crosscheck-m-mbgate-scrut 终签在链（事件哈希 a0bbcb40cc79b51c4d663167af033422977b7562f5cf5e6885d1a809158dea66）+ tally m-mbgate-scrut 报告 R1-R7 全过 + DEC-013 融回门三步机制
> 队形：单线 solo — 执行代理亲写零子代理、主会话守验收位
> 日期：2026-08-31

## 一、问题陈述 {#problem}

核阅（治理名）作为治理文档格式规约核验位已久，但实装位 sih-tools/scrutinator 是围堰外切状态，工具契约 CONTRACT.md 第 49-51 行钉死"融回门三查"机制：验收判据全过、接口契约未变、回迁债已评估。现状即 S6 验收会于 2026-08-20 用户签收五判据对表全过，工程接入 DES-012 瘦引用已注册但引擎侧 src/ 仍为零实装。本批承 DEC-013 融回门三步"引擎侧开发非移植"路径，规格先行出 SPEC-013，钉死家位与接口对表与规则包家位与验收判据与回迁债五件，为 TDD 实现批铺路。

## 二、关键设计 {#design}

五件。一家位即引擎侧 src/scrutinator/ 库模块加 src/bin/scrutinator.rs 命令行面，承 SPEC-006 书简融回先例的双形态结构。二接口对表即工具 CONTRACT.md 第 17-33 行六件机器形态与五判据与融回门三查逐条对表，引擎规格不新立接口只搬实现。三规则包家位即 manifest.toml 加 rules.toml 纯数据随件迁引擎侧，三包即 des-001 0.1.0 加 des-001-mathe 0.3.0 加 ask3 0.1.0 全量迁，工具侧保留不删作兼容只读。四验收即同包同目标下引擎件与工具件输出 JSON 报告逐字节一致（冻结工具输出为金向量）、退出码三值一致、报告 content_hashes 字段与 scribe append 认证位兼容。五回迁债即 TOML 解析依赖（引擎侧引 toml crate，引擎不受句读零第三方约束）、双跑对表基线、DEC-001 围堰物理对应归位映射的核阅行（DEC-001 第 107-141 节）。

## 三、工作清单 {#work}

- [ ] 任务包落 sih/state/plan/scrutmerge-sdd-solo.md
- [ ] 三问记录 + 三门 + 意图入链 + 叩问消化闸 + 正身
- [ ] 立约双仓 + 锁位请求写入
- [ ] 写 SPEC-013-scutiny-mergeback-gap.md 落 doc/spec/
- [ ] 化格 + 核阅 des-001 + 检词（报告落 sih-tools/scribe/reports/）
- [ ] 认证入链 + 结算 + 收约
- [ ] 双仓 reconcile unrouted 零 + 验链 valid

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 家位与模块形 | 规格 | 引擎侧 src/scrutinator/ 库模块加 src/bin/scrutinator.rs 命令行面，承 DEC-007 组件层归属 |
| **F-2** 接口契约对表 | 规格 | 工具 CONTRACT.md 六件机器形态加五判据加融回门三查逐条对表，零语义漂移 |
| **F-3** 规则包家位 | 规格 | manifest.toml 加 rules.toml 纯数据随件迁引擎侧，三包全量迁，工具侧保留不删 |
| **F-4** 验收判据 | 规格 | 同包同目标下引擎件与工具件输出 JSON 报告逐字节一致 + 退出码三值 + content_hashes 与 scribe append 认证位兼容 |
| **F-5** 回迁债 | 规格 | TOML 解析依赖列示加 Cargo.toml 必加 toml 依赖 + 双跑对表基线 + DEC-001 围堰归位映射核阅行 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/scrutinator/CONTRACT.md（工具契约，6 件机器形态 + 5 判据 + 融回门三查）
- 必读 2：sih-engine/doc/spec/SPEC-006-scribe-mergeback-gap.md（书简融回落差规格先例）
- 必读 3：sih-engine/doc/decision/001-repository-structure.md 第 107-141 节（围堰物理对应归位映射）
- 必读 4：sih-tools/scrutinator/packs/{des-001,des-001-mathe,ask3}/ 规则包
- 必读 5：sih-tools/tally/reports/m-mbgate-scrut-check-input.json + signcheck（终签材料）
- 必读 6：sih-engine/doc/decision/013-mergeback-implementation.md（如有，融回机制；DEC-013）

## 六、约束 {#constraints}

- 引擎侧是开发非移植：不逐行翻译 Python 实现，按 SPEC-013 判据用 Rust 重写
- 工具件不删不改语义，工具侧退役只标注与转兼容只读
- 上链前必须等绿；读 findings 不只看退出码
- 锁被他在持即报不绕行
- 范围闸：请求写入、锁、staged、提交四面对齐
- 全程留痕：四工具 CALL-LOG 加人类输入日结（链外逐字）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过即主会话验收复跑
- [ ] 双仓 reconcile unrouted 零（红线）
- [ ] 链 verify status=valid（红线）
- [ ] 管线化格 + 核阅 des-001 + 检词 0 violations（红线）
- [ ] SPEC-013 在 doc/spec 即 des-001 域必过核阅

## 八、风险点 {#risks}

- Python 工具侧 6 类 kind 含 5 类 json 材料谓词 + 1 类 text 谓词；Rust 重写时 json_field / json_array_schema / json_number_range / json_field_compare 词表与 Python 逐条对表是机械工作量大点
- TOML 解析须 toml crate，Cargo.toml 必加；引擎侧原本零第三方依赖（仅 std + sha2 + uuid + chrono + serde + serde_json）
- 金向量冻结：先跑工具件对三包在真实目标上生成期望输出，引擎件逐字节对表；任何键序、缩进、空值形漂移即判负返工
- DEC-001 围堰归位映射核阅行：spec/spec-013.md 自身必须在核阅域内且零违规

## 九、队形声明 {#formation}

单线 solo — 执行代理亲写零子代理，主会话守验收位不代写。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 令"核阅开" + 终签事件
- 链件：sih/event/trail/2026-08-31.ndjson 即本批意图与认证
- 关联：DEC-013 融回门机制、SPEC-006 书简融回先例、AGENTS.md 工具层静态审计节

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/scrutmerge-sdd-solo.md（本任务包）
- sih-engine/doc/spec/SPEC-013-scutiny-mergeback-gap.md（核心规格产出）
- sih-engine/sih/event/plan/scrutmerge-sdd-solo-results.md（结果档）
- sih-engine/sih/event/trail/2026-08-31.ndjson（意图与认证）
- AGENTS.md（如需改静态审计节；锁冲突延期如实记）
- sih-tools/scribe/reports/（管线报告 + recall + ask3 + 正身）
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/lease/ledger/sessions.ndjson
- sih-tools/lease/ledger/locks.ndjson

## 十二、叩问处置 {#elicit-dispositions}

- 叩问处置[核阅融回]: 消解 即工作名直述即核阅工具从围堰外切态融回引擎侧
- 叩问处置[SDD 规格先行]: 消解 即工作名直述即 SDD 即规格先行的批模式
- 叩问处置[TDD 先红后绿]: 消解 即工作名直述即 TDD 即测试驱动先写失败测试的批模式
- 叩问处置[双模并存]: 消解 即工作名直述即切换批期引擎件与工具件并存的过渡形态
- 叩问处置[金向量冻结]: 消解 即工作名直述即金向量即对拍基线冻结的工程实践
- 叩问处置[规则包纯数据随件迁]: 消解 即工作名直述即规则包随件迁而非运行时再加载的工程选择
- 叩问处置[TOML 解析]: 消解 即工作名直述即 Rust 解析 manifest 与 rules 的依赖项
- 叩问处置[回迁债]: 消解 即工作名直述即回迁债即迁移负担的评估项
- 叩问处置[围堰归位映射]: 消解 即工作名直述即 DEC-001 第 107-141 节围堰物理对应归位映射
- 叩问处置[代码标识符承本名]: 消解 即工作名直述即代码标识符承 DEC-006 本名回滚即非治理名
- 叩问处置[治理名核阅不变]: 消解 即工作名直述即治理名核阅不变、代码标识符 scrutinator 不另起
- 叩问处置[席位参验]: 消解 即工作名直述即 DEC-007 子决策三核阅承接鉴的反映职能的治理位
- 叩问处置[SDD]: 消解 即工作名直述即规格先行的批模式简称
- 叩问处置[scrutmerge-sdd-solo]: 消解 即本批名 scrutmerge-sdd 核阅融回规格先行
- 叩问处置[双模并存]: 消解 即工作名直述即切换批期引擎件与工具件并存的过渡形态
- 叩问处置[核阅融回]: 消解 即工作名直述即核阅从围堰外切态融回引擎侧
- 叩问处置[规格先行]: 消解 即工作名直述即 SDD 即规格先行的批模式
- 叩问处置[SPEC-013]: 消解 即本批规格编号 SPEC-013-scutiny-mergeback-gap
- 叩问处置[TOML解析]: 消解 即工作名直述即 Rust 解析 manifest 与 rules 的依赖项
- 叩问处置[scrutmerge]: 消解 即本批主题 scrutmerge 核阅融回简称
- 叩问处置[围堰归位映射]: 消解 即工作名直述即 DEC-001 第 107-141 节围堰物理对应归位映射
- 叩问处置[规则包]: 消解 即工作名直述即三包 des-001/des-001-mathe/ask3 的统称
- 叩问处置[金向量]: 消解 即工作名直述即金向量即对拍基线冻结的工程实践
