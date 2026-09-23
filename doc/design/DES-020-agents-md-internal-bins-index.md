# DES-020 AGENTS.md 内部工具索引设计

本设计补齐 AGENTS.md 对工程实体的索引覆盖。背景即评估报告文档维度 W4 与观察 O2：W4 即 AGENTS.md 覆盖偏薄，组件索引仅列 4 个第一阶段组件加 2 个第二阶段预登记件，src/bin 实况 36 个内部工具 bin 与 lib 面模块均未索引，新贡献者只读 AGENTS.md 会误判仓体只有 4 个组件；O2 即 src/lib.rs 实测 12 个 pub mod，其中 cascade_registry 与 exitenvelope 与 tools_registry 与 snapline 与 askroute 五件未在 AGENTS.md 出现。判词：AGENTS.md 是仓的门面索引，索引缺件即门面失真；索引以机械对表为纪律，不靠记忆维护。

## 概览 {#overview}

- 两段结构即阶段组件段保留 4 加 2 不动加新增内部工具 bin 索引段::[两段结构](#structure)
- 索引条目载三信息即 bin 名加行数加职能一句话，在 AGENTS.md 内一行一件::[索引条目形](#entry-form)
- 初始索引快照 36 件与 lib 模块清单 12 件在案，含观察 O2 五件收口::[初始索引快照](#snapshot)
- AGENTS.md 全文总行数不超 200::[行数约束](#line-budget)
- 新 bin 入场须同步索引行，漂移由评审对表::[更新纪律](#update-discipline)

## 两段结构 {#structure}

第一段阶段组件
: 保留现有组件索引节 4 加 2 不动：4 个第一阶段组件即意图锚定组件与符号材料生成与参验与事件流，2 个第二阶段预登记件即视图组件与微积分引擎。本设计不改此段一字。

第二段内部工具 bin 索引
: 新增二级标题节，位次紧随组件索引节之后，节内一行一件覆盖 src/bin 全部 bin。lease 目录七子模块是 lease bin 的内部模块，Cargo.toml 无显式 bin 段，bin 由自动发现产出，子模块不计独立 bin。

## 索引条目形 {#entry-form}

条目形
: 每件一行，载三信息即 bin 名加行数加职能一句话。职能取各件头注首行，去移植批注尾段；行数取 wc -l 读数，随批更新。行形即 bin 名接行数接职能，单行内以空格分隔。

## 初始索引快照 {#snapshot}

快照声明
: 以下 36 件为 libwave 批时点读数，源即 src/bin 各件头注首行，作为索引初始内容与后续对表基线。

acceptor
: 596 行，引擎侧空腹判定包执行机命令行面。

ask3repeater
: 81 行，三问确定性外壳的校验腿，承接 SPEC-005#deterministic-shell。

askroute
: 147 行，判定路由命令行面。

attnanchor
: 483 行，回锚引擎件，五行锚读数组装器，注入式回锚 v1 的引擎 bin 形。

attractor
: 337 行，得一机械核对腿的六子命令入口。

basemgr
: 820 行，引擎侧基线向量管理工具命令行面。

calllogtool
: 500 行，引擎侧行式账本命令行面。

cascade
: 377 行，引擎侧级联命令行面。

checker
: 831 行，引擎侧检查器命令行面。

confledger
: 796 行，引擎侧冲突账本命令行面。

critsweep
: 1102 行，判据扫引擎件，对话框内治理态回算器，v1.2.0 的引擎 bin 形。

elicit
: 578 行，引擎侧叩问命令行面。

formatter
: 627 行，引擎侧化格命令行面。

gauge
: 839 行，秤星引擎件，治理态读数计算核，ga-2 的引擎 bin 形。

identity
: 962 行，引擎侧正身命令行面。

incubation
: 1186 行，孵化回路契约校验器引擎 bin。

latextool
: 1495 行，引擎侧 LaTeX 书写辅助命令行面。

lease
: 729 行，租约引擎件，锁核腿 fixture 对等实装。

locator
: 2872 行，引擎侧寻址命令行面。

locksview
: 746 行，引擎侧锁视图工具命令行面。

meter
: 696 行，引擎侧计量命令行面。

nomenclator
: 930 行，引擎侧检词命令行面。

parser
: 3012 行，引擎侧句读命令行面。

pendline
: 74 行，候裁处置范式编排件的五子命令入口。

projsnap
: 786 行，引擎侧派生快照投影器命令行面。

registrydemo
: 125 行，腿五插件槽位演示 bin。

retriever
: 168 行，温故宿主命令面，承接 SPEC-007#interface-signature 与 SPEC-008#boundary 与修订六。

scribe
: 750 行，书简融回命令行面，本名回滚承 DEC-017 修订二，承接 SPEC-006#boundary 与 T6。

scrutinator
: 232 行，引擎侧核阅组件命令行面，承接 DEC-007#decision-component 与 SPEC-013。

selector
: 1097 行，引擎侧路择命令行面。

sih
: 288 行，sih 命令薄壳，sih init 初始化单步窄口。

sihmcp
: 151 行，sihmcp 二进制入口，MCP 线 Rust 载体。

tally
: 1366 行，引擎侧执契命令行面。

viewer
: 221 行，视图组件命令行面，聚合输出组件。

watchcheck
: 670 行，引擎侧稽命令行面。

wikirecall
: 703 行，三通道确定性召回引擎 bin。

## lib 模块清单 {#lib-modules}

清单
: src/lib.rs pub mod 实况 12 件即 ask3repeater 与 askroute 与 attractor 与 cascade_registry 与 event_stream 与 exitenvelope 与 mcpserver 与 retriever 与 snapline 与 scrutinator 与 tools_registry 与 view。

O2 收口
: 其中 cascade_registry 与 exitenvelope 与 tools_registry 与 snapline 与 askroute 五件为此前未在 AGENTS.md 出现的模块，随第二段入索引即评估观察 O2 收口。lib 模块清单与 bin 索引同节承载，同适用更新纪律。

## 行数约束 {#line-budget}

预算
: AGENTS.md 全文总行数不超 200。现状 112 行，第二段增量即节头与引导约 3 行加 36 索引行加 lib 清单与注记约 8 行，合计增量约 47 行，改后全文约 159 行，预算内。超限即压缩职能句，不减件。

## 更新纪律 {#update-discipline}

同步义务
: 新 bin 入场须同步索引行：新 bin 落笔批在索引段增行，行数与职能照条目形；lib 新模块入清单同理。

漂移对表
: 漂移由评审对表：批评审或例行评审对表 src/bin 实况与 src/lib.rs pub mod 实况与 AGENTS.md 索引，缺行与行数漂移与职能失真即批内修复。

## 关联 {#relation}

评估实证即 doc/assessment/2026-09-22-sih-engine-evaluation-report.md 文档维度 W4 与 doc/assessment/2026-09-22-sih-engine-iteration-recommendations.md 观察 O2；结构基线即 AGENTS.md 组件索引节与 DES-003 第一阶段组件清单；受载对象即 AGENTS.md；本设计载体即 libwave 批。
