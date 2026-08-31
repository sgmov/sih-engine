# pkgclose-solo 结果档（任务包七包 007 至 013 对账收口）

> 批名：pkgclose-solo。日期 2026-08-31。会话 d5febbc5e44e9524（lease 1.8.2 双仓）。
> 意图事件 5681c626（哈希 2eb35181，三锚即 PRO-07 鉴只列事实、PRO-05 道四间隙、PRO-06 法四损补）。承用户 2026-08-31 令，队形单线形 solo。

## F 锚定验收 {#f-anchors}

| F | 判据 | 判定 |
|---|---|---|
| F-1 对账表 | 七包四列齐、证据逐条可机械回查、验收方抽两条复跑一致 | 过（对账表节七行齐，证据均为 path:line 或链上事件号，验收方按证据列复跑） |
| F-2 关闭裁定块 | 每包头部块含裁定、承接位、证据引用、批号引用、日期、原文主体零改动 | 过（七块齐件，块内批号 pkgclose-solo 与日期 2026-08-31 俱在） |
| F-3 两态归位 | 七包移至 sih/event/plan/tasks/ 且名不改、state/plan/tasks 不再含 007 至 013 | 过（git mv 七件，task-001 至 006 原处不动） |
| F-4 记账落链 | GOV-003 结算追加、结果档在档、意图与认证入 08-31 链、结算 commit 在册、链 verify valid | 见链与收口节 |
| F-5 验收门 | 化格与核阅与检词全绿、reconcile unrouted 零、findings 逐件读过 | 见链与收口节 |

## 对账表 {#recon}

| 包 | 原目标 | 现承接位 | 证据（机械可回查） | 处置 |
|---|---|---|---|---|
| 007 doclint 结构化输出集成 | 旧仓 doclint 加 --emit-json 输出 DES-008 契约 NDJSON、违规入 trail | 核阅承接结构化输出与强制校验位即 JSON findings 与零 LLM 与退出码三值、书简承接认证留痕、DES-008 报告契约以 des-001 规则包报告形态承接 | AGENTS.md:128 与 280 与 162、sih-tools/scrutinator/ 在场 | 承接闭项 |
| 008 分词层 | src/tokenizer/ 语义单元、stable_id 即文件路径加节点类型加节点序号加内容哈希 | 寻址承接语义单元切分、句读承接解析引擎承 DEC-016、稳定标识派生规则逐字同构 | sih-tools/locator/CONTRACT.md:16、sih-engine/doc/decision/016-parser-initiation.md:1、sih-engine/sih/event/trail/2026-08-28.ndjson:19 事件 9e990fc3 | 承接闭项 |
| 009 语义树层 | containment 与 order 语义树、comrak AST 单权威、unsliced 标记 | 树本体由句读承接即 PEG 语法树与错误处包节点永远产树、与寻址语义单元切分配套、unsliced 标记无承接位 | sih-tools/parser/CONTRACT.md:18、2026-08-28.ndjson:19 事件 9e990fc3、unsliced 仅存校准调研 sih-engine/sih/state/calibration/markdown-tokenization-toolchain-survey.md:51 与 91 | 部分承接闭项 |
| 010 判定器 | Rule trait 四内建规则、Violation 八字段、NDJSON 入 trail、退出码 0/1/2/124 | 分布式承接即核阅十二规则码、检词两规则、级联上游洁净不变式、执契 R1 至 R7、得一二层裁决结构 | AGENTS.md:158、sih-tools/nomenclator/CONTRACT.md:37、sih-tools/cascade/CONTRACT.md:3、sih-engine/doc/decision/019-tally-naming.md:1、sih-engine/doc/decision/020-deyi-component-naming.md:3 | 承接闭项 |
| 011 规则层 | RuleDef 与 CounterExample 与 RuleRegistry、可证伪元约束、变更入 trail | 规则包承接即 des-001 版本管理、检词术语包三态登记、路择谓词包空腹零 LLM、执契 verify 重放、CounterExample 三字段无承接位 | AGENTS.md:158、sih-tools/nomenclator/CONTRACT.md:27、sih-tools/tally/CONTRACT.md:19 | 部分承接闭项 |
| 012 NPC 编排层 | AuditUnit、NpcExpert trait、ExpertPool 1/3/5 单 KEY、Orchestrator | facet 承接多 facetor 独立审阅与 compiler 确定性聚合、代理编组承接五形编排、得一承接裁决落链 | sih-tools/facet/FACET.md:7 与 41、AGENTS.md:279、sih-engine/doc/decision/020-deyi-component-naming.md:3 | 承接闭项 |
| 013 偏离率度量 | K/N 偏离率、Jaccard、波动系数、收敛双指标、阈值留空 | facet compiler 承接 coverage 与 Jaccard 与 severity 分布、秤星承接 convergence 维治理态读数即 ga-1、得一承接收敛裁决闸、K/N 与波动系数与双指标公式无 src 承接位 | sih-tools/facet/FACET.md:23 与 33、sih-tools/gauge/CONTRACT.md:19、sih-engine/doc/decision/020-deyi-component-naming.md:3 | 部分承接闭项 |

## 完成度 {#progress}

- GOV-002 退出标准第一条分两侧。七包执行完毕并关闭的账面侧本批闭项，即七包全有裁定与承接位与处置，账面态归事件层。
- 同条六组件落地 src 侧：三问已落地即 ask3repeater 承 DEC-006、书简已落地即 event_stream 三入口、视图已落地即 viewimpl-solo 批 v1.4、温故已落地即 retriever 承 DEC-017，参验与判定器落地态由各自批次结果档承载。本批只记七包账面侧，不代判同条整体达成。

## 偏离与期票 {#deviations}

- GOV-003 版本号：任务包写升 v1.4，viewimpl-solo 批已先落 v1.4 于基线分支 msh/viewfix-solo 版本节，本批独立递增落 v1.5，与 viewimpl 行原文 pkgclose 未落 v1.4 即本批独立递增不再追等同向。
- 结果档在 des-001 域外即域为 sih-engine/doc/**，核阅对结果档出域不检，认证由 GOV-003 核阅报告与结果档检词报告承接，承 SETTLEMENT-001 域外先例。
- 无承接位三件留人节点即 unsliced 标记（task-009）、CounterExample 三字段结构（task-011）、K/N 与波动系数与双指标公式（task-013），本批不发明承接位。

## 链与收口 {#chain}

- 意图 5681c626（哈希 2eb35181）。
- 认证：管线报告经书简 append 入 08-31 链，哈希见 trail。
- 双仓 commit 指副本，close 归并，reconcile 双仓，验链 valid。

## 后续 {#next}

- GOV-002 退出标准第一条六组件 src 侧剩参验与判定器账目对账，另批承载。
- 无承接位三件候人节点令。
