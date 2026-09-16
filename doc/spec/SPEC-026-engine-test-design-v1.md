# SPEC-026 引擎测试设计规范：全量测试矩阵与全绿路径与对表偏差标注

版本 v1，2026-09-16 成文，承发布前测试与修复期测试设计令与 SPEC-025 全量融回收官态，承接 gap 五件测试腿与 identity 超时闸腿与前序红keep 系批的在档成果。本文档立测试设计不执行认证：测试资产由在役批次承载，本文档只钉矩阵与命令与偏差规范，上链归批次收口治理流程。

## 概览 {#overview}

- 全绿路径即绕行链接环境与终态读数四百七十三过零败六忽略与退出码零，永久修复归用户许可动作::[环境与全绿路径](#green-path)
- 测试策略三态即黑箱白箱两形态与金向量围堰对表沙箱账本三策略的分界与适用面::[测试策略](#strategy)
- 全量测试矩阵按功能域八组覆盖集成测试文件五十一件与库内白箱模块三十八件，逐功能给形态与策略与测试文件与命令::[测试矩阵](#matrix)
- 运行命令手册逐条可复制，绕行环境导出一次后逐文件命令即可执行::[运行命令手册](#commands)
- 对表已知偏差标注规范即标注三处纪律与现役偏差清单与处置规范::[对表偏差标注规范](#deviation)
- 纪律边界即禁写区与冻结围堰与版本锁定与禁改域四条::[纪律边界](#discipline)

## 环境与全绿路径 {#green-path}

环境事实：本机 cargo 默认环境对任何测试二进制均无法链接，链接器报 cc 退出码六十九与 Xcode 许可未同意提示，故默认环境下的基线读数为套件未运行的零过零败，非测试失败。本轮全部测试读数经绕行环境取得：以独立 Command Line Tools 的 clang 与自带 SDK 供链，零仓库改动，绕行形见运行命令手册节。

全绿路径三步：其一绕行环境就位即导出 SDKROOT 与 RUSTFLAGS 两变量后 cargo 可链接可运行；其二过时金向量维护即两处零网络依赖断言对 DEC-023 已裁载体 tokio 与 rmcp 与 axum 豁免并注记令源，attractor 机械腿零网络零 LLM 红线由源码级扫描测试继续执法；其三缺口补测即 gap 五件测试腿落位见测试矩阵节。全绿判定标准即 `cargo test --no-fail-fast` 退出码零且 failed 计数为零，本文记 testGreen=true。

终态读数为本批 2026-09-16 实测：工作区根 sih-engine 目录内跑绕行环境全量，得四百七十三过零败六忽略，cargo 退出码零，test result 段八十七段，六忽略与 redkeep-solo 基线一致。读数账目与在档链路吻合即前序批基线四百五十四过加过时断言修复两例加 gap 新增十七例。

环境永久修复项：用户侧执行 `sudo xcodebuild -license` 同意 Xcode 许可后默认环境即可链接，此为本会话不可达项，如实登记候人节点。未修复前一切测试运行须携绕行环境变量。

## 测试策略 {#strategy}

形态两分：

黑箱
: 经 CARGO_BIN_EXE 宏调起 cargo 产物二进制，进程外命令行面对表，覆盖退出码与报文与台账落笔，集成测试五十二件全数黑箱优先。

白箱
: 库内 `#[cfg(test)]` 单元测试，直测模块内部不变量，计三十八个源文件在役，含 event_stream 全家与 mcpserver 四件与 attractor 三件等。

策略三分：

金向量对表
: 对冻结在册期望资产逐字节或逐值对表，资产位即核阅金向量 `src/scrutinator/fixtures/golden/` 与租约金向量 `src/lease/fixtures/golden2/` 三十六件与 parser 在册 vectors 双包与 attractor 四类工件金向量。基线向量只主张没变不主张对，承 SPEC-021 甲乙分界红线；期望变更须金向量维护注记令源，禁静默改期望。

围堰对表
: 引擎 bin 对冻结围堰 Python 件同参双跑，判路判词与计数一致为过，报文格式差异按已知偏差不硬凑，围堰件只读运行携 PYTHONDONTWRITEBYTECODE 禁字节码，python 不可跑时如实申报跳过。

沙箱账本
: 一切写路径测试在系统临时目录 tempdir 自建台账闭环，租约台账与 trail 链与锁台账全沙箱，零真实账本与链写入；真实治理域 `sih-engine/sih/event/` 与两处 lease ledger 为测试禁写区，本批实测全量两跑后禁写区两千九百八十六件 mtime 前后零变化。

## 测试矩阵 {#matrix}

矩阵按功能域八组，组内逐功能给形态与策略与测试文件，命令列给出 cargo test 目标形，完整可复制形见运行命令手册节。

### 组一 治理五件套与核心库 {#matrix-five}

scribe 书简与 event_stream
: 白箱：`src/event_stream/` 全模块单元测试含 tdd_tests 与 chainstamp_tests 与 append 与 verify 与 query 与 intent 与 park 与 lockgate 与 sessiongate。黑箱：`tests/integration_hash_chain.rs` 与 `tests/integration_empty_hash.rs` 哈希链完整性与空哈希，`tests/cli_multitrail.rs` 跨链集成，`tests/lease_mergeback_t2_golden.rs` 认证报告金向量。命令 `cargo test --test integration_hash_chain` 等。

scrutinator 核阅
: 白箱：`src/scrutinator/mod.rs` 单元测试含金向量对拍 `golden_des001_gov002` 与 `cli_positional_form_matches_golden`，金向量资产在 `src/scrutinator/fixtures/golden/`。此二测试为前序批重新生成金向量后已绿，非红态。命令 `cargo test --lib scrutinator`。

attractor 执契路择
: 黑箱：`tests/attractor_golden.rs` 金向量冻结四类工件，`tests/attractor_cli.rs` 退出码全表与零网络断言，`tests/attractor_contract.rs` 跨腿契约，`tests/attractor_route.rs` 路由谓词与金向量。白箱：`src/attractor/route.rs` 与 `model_utils.rs` 与 `jsonc.rs`。命令 `cargo test --test attractor_golden` 等。

ask3repeater 三问
: 黑箱：`tests/gap_ask3repeater_fixture_race.rs` 六例即验收通过形与错误四类逐类与工具异常三值与并发校验。白箱：`src/ask3repeater/validate.rs` 与 `gate.rs` 与 `intercept.rs`，validate 夹具按 tag 加 pid 逐测试唯一路径消并行竞态。命令 `cargo test --test gap_ask3repeater_fixture_race`。

retriever 温故
: 黑箱：`tests/retriever_canonical_suite.rs` canonical 新城域检索四例，`tests/mem_recall_f_suite.rs` 项目记忆 F 锚定九例。白箱：`src/retriever/axes.rs` 与 `archives.rs` 与 `facet.rs` 与 `mod.rs`。命令 `cargo test --test retriever_canonical_suite`。

askroute 叩问路由
: 黑箱：`tests/mergeall_t6_askroute.rs` 引擎位移植金向量与双跑对表四例。白箱：`src/askroute/mod.rs`。命令 `cargo test --test mergeall_t6_askroute`。

### 组二 融回工具带金向量 {#matrix-mergeall}

以下全为黑箱金向量三例形，策略即引擎 bin 加沙箱台账加金向量对表，写路径全 tempdir，命令形 `cargo test --test <文件名去 rs 后缀>`。

attnanchor 回锚
: `tests/mergeall_t1_attnanchor.rs`，五行锚金向量三例。

critsweep 判据扫
: `tests/mergeall_t1_critsweep.rs` 金向量三例含泊界面降级形；正路径补测见组五 gap 泊界路由件。

gauge 秤星
: `tests/mergeall_t1_gauge.rs`，read 三维读数 ga-2 值对表与 record 落链回执与维度违例拒与空账本零虚构，scribe 以 fixture 假件承接。

formatter 化格
: `tests/mergeall_t2_formatter.rs`，四例金向量。

nomenclator 检词
: `tests/mergeall_t2_nomenclator.rs`，三例金向量。

locator 寻址
: `tests/mergeall_t3_locator.rs`，三例金向量。

parser 句读
: `tests/mergeall_t3_parser.rs` temp 自建包三例；引擎仓真实包资产补测见组五 gap 包资产件。

calllogtool 留痕册
: `tests/mergeall_t4_calllog.rs`，三例金向量。

cascade 级联
: `tests/mergeall_t4_cascade.rs`，三例金向量。

identity 正身
: `tests/mergeall_t4_identity.rs` 三例金向量；超时闸补测 `tests/mergeall_t4_identity_timeout.rs` 两例即挂起采集件被闸杀后正常退出非信号死形与墙钟有界与正常环境全形瞬回。

meter 计数
: `tests/mergeall_t4_meter.rs`，三例金向量。

selector 路择
: `tests/mergeall_t4_selector.rs`，三例金向量。

tally 执契
: `tests/mergeall_t4_tally.rs`，三例金向量。

watchcheck 稽
: `tests/mergeall_t4_watchcheck.rs`，三例金向量。

acceptor 验收
: `tests/mergeall_t6_acceptor.rs`，三例金向量。

basemgr 基线向量管理
: `tests/mergeall_t6_basemgr.rs`，三例金向量。

checker 级联检查
: `tests/mergeall_t6_checker.rs`，三例金向量。

confledger 置信台账
: `tests/mergeall_t6_confledger.rs`，三例金向量。

elicit 叩问
: `tests/mergeall_t6_elicit.rs`，三例金向量。

incubation 孵化登记
: `tests/mergeall_t6_incubation.rs`，契约校验三例。

latextool 排版辅助
: `tests/mergeall_t6_latextool.rs`，二十一例金向量。

locksview 锁面读数
: `tests/mergeall_t6_locksview.rs` 三例含假 cascade shim 经覆盖位注入；真 bin 缺省解析序补测见组五 gap 级联真链件。

projsnap 项目快照
: `tests/mergeall_t6_projsnap.rs`，三例金向量。

wikirecall 书单召回
: `tests/mergeall_t6_wikirecall.rs`，三例金向量。

### 组三 租约 lease {#matrix-lease}

全黑箱加沙箱台账，令源 SPEC-024，命令形同组二。

lease_mergeback_t2_golden
: T2 金向量逐字节一致四例，沙箱 trail。

lease_mergeback_t3_exit_codes
: T3 退出码三值全表两例即零成与一拦与二工具异常。

lease_mergeback_t4_five_verifications
: T4 五验撞锁形 locked_elsewhere 拒。

lease_mergeback_t5_commit
: T5 commit 四验与 message 三形态与退出码五例。

lease_mergeback_t6_close_gates
: T6 chained close 闸序五例含差集闸拦与认领放行与错认领拒。

lease_mergeback_t7_sweep
: T7 sweep 五类普查与 call-log import 与 hooks 与 stem 闸全查七例。

lease_open_allow_parse
: open allow 解析端到端回归。

gap_mergeback_uncommitted_landing
: 落位面补测三例见组五。

白箱
: `src/bin/lease.rs` 与 `src/bin/lease/sddgate.rs` 与 `attachments.rs` 与 `guardlaw.rs` 单元测试，SDDG 门语义白箱在役。

### 组四 插件槽位与接线 {#matrix-plugin}

mergeall_t5_registry
: `tests/mergeall_t5_registry.rs`，腿五插件槽位架构验收两例。

leg4_wiring
: `tests/leg4_wiring.rs`，registry 派生 list 与 defs 零漂移加调用路由四例。

mergeall_final_bridge
: `tests/mergeall_final_bridge.rs`，进程外插件桥四面实装回归一例。

### 组五 gap 缺口补测五件 {#matrix-gap}

gap_parking_route_engine_selector
: `tests/gap_parking_route_engine_selector.rs` 三例：critsweep 泊界腿直调引擎 selector bin 三路路由各归其位、空目录绿态 routed 计零、围堰 Python selector 同参对表逐材料判路与 summary 计数一致；first_domain 布局沙箱显式 registry，零泊界降级。

gap_locksview_cascade_engine_bin
: `tests/gap_locksview_cascade_engine_bin.rs` 两例：不设覆盖位经兄弟位解析取真引擎 cascade bin 全链即真建册真链上认证哈希对表真判词四形加四笔 checked 行留痕、覆盖位空串回落兄弟位。

gap_parser_pack_assets
: `tests/gap_parser_pack_assets.rs` 三例：无关 temp cwd 裸包名 vectors 双包全过、parse 与 entries 产出对表在册期望件逐字节一致、裸名缺包报错形退出码一。

gap_mergeback_uncommitted_landing
: `tests/gap_mergeback_uncommitted_landing.rs` 三例：git 沙箱仓已提交落位形免认领即过并真实进 SDDG 实判走 sanctioned 绕行通道留台账、目录形态声明未落位整批拒、豁免三面 unparsed 与 shared_surface_exempt 与 exempt_conditional。

gap_ask3repeater_fixture_race
: 见组一三问行。

### 组六 视图与 HTTP 面 {#matrix-view}

recognize_http_gate
: `tests/recognize_http_gate.rs`，HTTP 读面域隔离端到端，真 axum 路由真 TCP 请求。

viewer 视图
: 白箱 `src/bin/viewer.rs` 与 `src/view/settle.rs` 与 `alarms.rs` 与 `heartbeat.rs`。

mcpserver
: 白箱 `src/mcpserver/mod.rs` 与 `bootstrap.rs` 与 `httpface.rs` 与 `webface.rs`。此域源码为禁改域，测试在役承载回归锁，改归快照切换批另令。

### 组七 零网络不变量 {#matrix-nonet}

t5_zero_network_zero_llm_source_scan
: `tests/attractor_cli.rs` 与 `tests/attractor_route.rs` 各一例，源码级扫描 `src/attractor` 与 `src/bin/attractor.rs` 零网络零 LLM 禁词，attractor 机械腿红线执法位。

t5_zero_network_deps_in_cargo
: 同两文件各一例，Cargo 级断言对 reqwest 与 hyper 与 curl 与 ureq 与 surf 与 attohttpc 与 openai 全禁，tokio 与 rmcp 与 axum 按 DEC-023 载体豁免，偏差注记见对表偏差标注规范节。

### 组八 读数命令示例 {#matrix-command-sample}

矩阵行命令皆为目标选择形，如 `cargo test --test mergeall_t1_gauge` 与 `cargo test --lib event_stream`，携绕行环境后复制即跑，逐文件完整清单见运行命令手册节。

## 运行命令手册 {#commands}

先导出绕行环境一次：

`export SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk RUSTFLAGS="-C linker=/Library/Developer/CommandLineTools/usr/bin/clang"`

导出后以下逐条在工作区根 sih-engine 目录内复制即跑。零导出单条全量完整形：

`cd sih-engine && SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk RUSTFLAGS="-C linker=/Library/Developer/CommandLineTools/usr/bin/clang" cargo test --no-fail-fast`

全量与白箱与单文件三形：

- 全量：`cargo test --no-fail-fast`
- 库白箱全体：`cargo test --lib`
- 单白箱模块：`cargo test --lib scrutinator` 或 `cargo test --lib event_stream`
- 单测试函数：`cargo test --test attractor_cli t5_zero_network_deps_in_cargo` 或 `cargo test --lib scrutinator golden_des001_gov002`

集成测试五十二件逐条命令，按矩阵分组序：

组一五件套与核心库：
`cargo test --test integration_hash_chain` 、 `cargo test --test integration_empty_hash` 、 `cargo test --test cli_multitrail` 、 `cargo test --test attractor_golden` 、 `cargo test --test attractor_cli` 、 `cargo test --test attractor_contract` 、 `cargo test --test attractor_route` 、 `cargo test --test gap_ask3repeater_fixture_race` 、 `cargo test --test retriever_canonical_suite` 、 `cargo test --test mem_recall_f_suite` 、 `cargo test --test mergeall_t6_askroute`

组二融回工具带：
`cargo test --test mergeall_t1_attnanchor` 、 `cargo test --test mergeall_t1_critsweep` 、 `cargo test --test mergeall_t1_gauge` 、 `cargo test --test mergeall_t2_formatter` 、 `cargo test --test mergeall_t2_nomenclator` 、 `cargo test --test mergeall_t3_locator` 、 `cargo test --test mergeall_t3_parser` 、 `cargo test --test mergeall_t4_calllog` 、 `cargo test --test mergeall_t4_cascade` 、 `cargo test --test mergeall_t4_identity` 、 `cargo test --test mergeall_t4_identity_timeout` 、 `cargo test --test mergeall_t4_meter` 、 `cargo test --test mergeall_t4_selector` 、 `cargo test --test mergeall_t4_tally` 、 `cargo test --test mergeall_t4_watchcheck` 、 `cargo test --test mergeall_t6_acceptor` 、 `cargo test --test mergeall_t6_basemgr` 、 `cargo test --test mergeall_t6_checker` 、 `cargo test --test mergeall_t6_confledger` 、 `cargo test --test mergeall_t6_elicit` 、 `cargo test --test mergeall_t6_incubation` 、 `cargo test --test mergeall_t6_latextool` 、 `cargo test --test mergeall_t6_locksview` 、 `cargo test --test mergeall_t6_projsnap` 、 `cargo test --test mergeall_t6_wikirecall`

组三租约：
`cargo test --test lease_mergeback_t2_golden` 、 `cargo test --test lease_mergeback_t3_exit_codes` 、 `cargo test --test lease_mergeback_t4_five_verifications` 、 `cargo test --test lease_mergeback_t5_commit` 、 `cargo test --test lease_mergeback_t6_close_gates` 、 `cargo test --test lease_mergeback_t7_sweep` 、 `cargo test --test lease_open_allow_parse`

组四插件槽位：
`cargo test --test mergeall_t5_registry` 、 `cargo test --test leg4_wiring` 、 `cargo test --test mergeall_final_bridge`

组五 gap 五件：
`cargo test --test gap_parking_route_engine_selector` 、 `cargo test --test gap_locksview_cascade_engine_bin` 、 `cargo test --test gap_parser_pack_assets` 、 `cargo test --test gap_mergeback_uncommitted_landing` 、 `cargo test --test gap_ask3repeater_fixture_race`

组六视图与 HTTP：
`cargo test --test recognize_http_gate`

对账式：组一十一加组二十五加组三七加组四三加组五五加组六一，合计五十二个文件名，其中 `gap_ask3repeater_fixture_race` 在组一与组五重复列出同一目标，去重后五十一个独立集成测试目标，与 tests 目录五十二项对账差一为 `fixtures` 夹具目录非测试件，rs 测试文件全集五十一件全数入矩阵与命令清单。

## 对表偏差标注规范 {#deviation}

标注三处纪律：偏差声明落测试文件头部模块注记即 `//!` 行，落关键断言行内注记，落本文档现役偏差清单；三处任一缺席即静默偏差，属违规。

标注格式四件：偏差名、两侧现状、不比对项与理由、令源或登记指针。样例即组七 Cargo 级断言注记：偏差名 DEC-023 载体豁免，两侧现状为 mcpserver 融回本 crate 后 Cargo.toml 合法引入 tokio 与 rmcp 与 axum，不比对项为此三件从 Cargo 级禁用清单移除，令源为 DEC-023 与 `src/attractor` 源码级扫描继续执法。

现役已知偏差清单：

- 报文格式：引擎 py_compact 紧凑形对围堰 Python indent 两空格形，selector 同参对表只比判路与 summary 计数不比报文，见 `tests/gap_parking_route_engine_selector.rs` 头注
- DEC-023 载体豁免：见前样例与 `tests/attractor_cli.rs:206` 与 `tests/attractor_route.rs:256` 注记
- parser 裸名缺包报错形由 parse 腿承载：vectors 对缺席包为零向量绿态故 vectors 腿不报错，见 `tests/gap_parser_pack_assets.rs` 头注
- identity 超时闸采集件双管读线程防管道自锁：行为差异见 `tests/mergeall_t4_identity_timeout.rs` 头注
- 判词一致基准：mcpline 双载体三路对表判词一致在案，结果档 `sih-engine/sih/event/plan/mcpdual-parallel-results.md`

处置规范：对表不一致即异常上报，禁静默放过禁改期望硬凑；金向量期望落后于已裁行为时按金向量维护修并注记令源，属许可维护；围堰件 python 不可跑时如实申报跳过不计过。

## 纪律边界 {#discipline}

- 禁写区三处即 `sih-engine/sih/event/` 全目录与 `sih-engine/sih-tools/lease/ledger/` 与 `sih-tools/lease/ledger/`，测试与试用一律 tempdir 沙箱账本，本批实测前后 mtime 快照对表两千九百八十六件零变化
- 冻结围堰：`sih-tools/` 全目录只读，围堰件只读运行携 PYTHONDONTWRITEBYTECODE，本批实测全量两跑后当日时间窗扫描仅得前序会话两件 pytest 缓存遗留非本会话写入
- 版本锁定：`sih-engine/Cargo.toml` version 恒 `0.9.0`，依赖清单零增删，本批实测零改动
- 禁改域：`sih-engine/src/mcpserver/` 零触碰，MCP 生产切 bin 位归快照切换批另令，其白箱测试在役作回归锁

## 内容充分性 {#sufficiency}

- 判据红证对表：全绿判定红证即任一测试失败或 cargo 退出码非零则 testGreen 假，红证由组七任一禁词回归即红构造承载；偏差标注纪律红证即三处任一缺席的对表不一致会静默通过，红证由围堰对表报文格式差异若硬比即红的实录承载；矩阵完备性红证即 tests 目录文件全集对账清单，漏一文件即矩阵对账式失配
- 判定性常数挂锚对表：本文无声明的判定性常数；des-001 C007 与 C008 与 C009 行级与邻近级闸在役核验
- 约束算子对表：本文无约束算子面，测试矩阵与偏差清单是登记面非约束系统，如实申报
