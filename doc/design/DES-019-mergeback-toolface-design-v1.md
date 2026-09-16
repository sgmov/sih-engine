# DES-019 融回工具面软件设计

## 概览 {#overview}

- 本档是 SPEC-025 全量工具融回在工具面的软件设计文档，规格先行承 DEC-013「SDD 即规格先行」，规格物位承 DEC-024 SDDG-1，逐节事实给证据路径::[选型理由](#selection)
- bin 层三十四 bin 源件实测即三十三工具 bin 加一 MCP 载体 bin，Cargo 自动发现全数成 bin::[bin 层](#bin-layer)
- 模块层十 lib 模块单源承载共享逻辑，租约执法面以 bin 私有子模块族七件封闭承载::[模块层](#module-layer)
- 数据面三件即每日哈希链 trail 与租约三账本与判据回算 registry::[数据面](#data-plane)
- 逐工具职责与边界三十四节以 ls 实测为准，每具载职责、输入输出、依赖、围堰来源四项::[逐工具职责与边界](#tools)
- 融回四正典约束逐条承接即 DEC-013 三步曲三查、SPEC-025 六腿判据、DEC-023 载体位、DEC-024 四判据::[融回决策承接](#decisions)
- 已知偏差与候批项如实列表零粉饰::[已知偏差与候批项](#deviations)

## 技术设计 {#technical-design}

### 选型理由 {#selection}

单仓单二进制族
: 融回发布形是引擎单一 Rust 仓内一族 bin，全量融回令源与「完整的 rmcp 工具」要求见 SPEC-025 令源节；单二进制分发理据承旧仓 audit-040 经 DEC-023 继承，见 sih-engine/doc/decision/023-mcp-rust-carrier.md 选型继承节。bin 自足为默认形：移植件逻辑落本 bin 源文件，跨工具共享逻辑只在引擎既有 lib 模块处单源，承 SPEC-025 边界「腿内工具间共享逻辑按引擎既有模块单源化，零复制粘贴」条款，不为共享新建抽象层。

包资产引擎位
: 格式包与谓词包与判据 registry 落引擎位 sih-engine/packs/ 与 sih-engine/critsweep/registry.json，ls 实测 packs/formatter、packs/nomenclator、packs/parser、packs/selector 四族与 critsweep/registry.json 在档。--pack 相对形解析以 exe 派生引擎仓根为首位候选，见 src/bin/formatter.rs:383 与 :397，消对围堰的运行时依赖，缺口申报号 gap-packs-assets。

零新增依赖
: 融回面零新增外部 crate，承 SPEC-025 判据 A6；引擎依赖面实测十六件加开发件 tempfile 一件，见 sih-engine/Cargo.toml:7-23 与 :26，版本 0.9.0 见 sih-engine/Cargo.toml:3。sqlite 派生腿因 rusqlite 不入许可依赖集而以落差申报形承载，见 src/bin/calllogtool.rs 头注落差一。

### 模块划分与依赖 {#modules}

依赖谁
: 工具 bin 依赖 std 与 Cargo 既有依赖面，见 sih-engine/Cargo.toml:7-23；共享逻辑消费 lib 十模块，见 src/lib.rs:8-17；包资产消费引擎位 packs/ 与 critsweep/registry.json；bin 间兄弟位直调三处，即回锚与判据扫的泊界路由腿直调 selector bin，见 src/bin/attnanchor.rs 头注落差三与 src/bin/critsweep.rs 头注落差四，锁视图乐观级联腿直调 cascade bin，见 src/bin/locksview.rs 头注落差六。

不依赖谁
: 工具 bin 运行时零 sih-tools 子进程 spawn、零网络、零 LLM，承 SPEC-025 判据 A9 红线，各 bin 头注零网络红线核对段在档；唯一例外是 MCP 载体面现行仍 spawn 围堰件，见偏差节 D9 与候批项 P1；写位封闭在各具声明的数据面位置，不触其他账本与链。

### bin 层 {#bin-layer}

src/bin 计三十四个 .rs 源件，ls src/bin/*.rs 实测三十四，时点 2026-09-16；sih-engine/Cargo.toml 全文无 [[bin]] 显式段，cat 实测，即按 Cargo 惯例自动发现全数成 bin。构成即三十三工具 bin 加一 MCP 载体 bin sihmcp；与回锚锚线读数「引擎 bin 33，sihmcp 19 具指纹对基线一致」自洽，读数出参来自 sih-engine/target/debug/attnanchor 2026-09-16 实跑。收尾批 lease-cutover-parallel 已把生产调用面切引擎 bin 位，归并 c115d27 在 git 历史可复现，git log 实测；工作区 AGENTS.md 工具层静态审计节围堰退役宣告段在档。

### 模块层 {#module-layer}

src/lib.rs 导出十模块，见 src/lib.rs:8-17 实测：ask3repeater、askroute、attractor、event_stream、mcpserver、retriever、snapline、scrutinator、tools_registry、view。分工如下。

- event_stream：链本体十五源件，ls src/event_stream 实测；scribe bin 六子命令与 MCP alpha 面 chain_query 与 chain_verify 进程内库调消费，见 src/mcpserver/alpha.rs:55、:66、:119、:130。
- attractor：得一机械核对腿，子模块十件，见 src/lib.rs:20 导出清单实测；attractor bin 消费。
- scrutinator：核阅规则引擎，规则包编译期内嵌三包名 des-001、des-001-mathe、ask3，见 src/scrutinator/asset.rs:28。
- tools_registry：腿五插件槽位骨架，ToolProvider trait 加 ToolRegistry 加 PluginManifest 解析加 ExternalPluginBridge 签名，见 src/tools_registry.rs 头注。
- mcpserver：MCP 载体本体十五源件，ls src/mcpserver 实测；sihmcp bin 消费；本档只记录零触碰，改划归快照切换批另令。
- lease 不入 lib：以 bin 私有子模块族七件承载，见 src/bin/lease.rs:8-21 模块声明与 ls src/bin/lease 实测；镜像理由见 tools_registry 选形申报，bin 私有子模块形把能力锁进单一 bin 私域，registry 须全域可达故落 lib，租约执法面封闭私域故不落 lib，见 src/tools_registry.rs 头注选形申报段。

### 数据面 {#data-plane}

#### 每日哈希链 trail {#data-trail}

- 家位 sih-engine/sih/event/trail/，一日一件 YYYY-MM-DD.ndjson，ls 实测至 2026-09-15.ndjson。
- 链式：首笔 prev_hash 取 GENESIS_PREV_HASH 六十四零常量，见 src/event_stream/hash.rs:9；逐笔哈希 compute_event_hash，见 src/event_stream/hash.rs:17；整链校验 verify_chain，见 src/event_stream/mod.rs:39 导出实测。
- 命令行写读位即 scribe bin 六子命令 append、verify、query、intent、park、record 另携 vectors 冻结向量集，见 src/bin/scribe.rs:3-4；record 即秤星读数落链入口，七字段守卫承 SPEC-011，见 src/bin/scribe.rs:4。
- 认证绑定内容哈希即 certify 腿与 chainstamp 查询形，ls src/event_stream 实测 certify.rs 与 chainstamp_tests.rs 在档。

#### 租约三账本 {#data-ledgers}

- 三账本主面即会话账本 sessions.ndjson、锁账本 locks.ndjson、绕行台账 bypass.ndjson；「sessions 与 locks 与 bypass 台账行格式零变更」承围堰契约修订四十八明文并列，见 sih-tools/lease/CONTRACT.md:187，只读引用。
- 附面即认领台账 claims.ndjson 五字段 append-only，见 sih-tools/lease/CONTRACT.md:59；账单 lockface-bills.ndjson 与收据 receipts/ 与检查 checks/，ls sih-tools/lease/ledger 实测；引擎侧骨架创建位见 src/bin/lease.rs:251-257。
- 引擎 bin 对等写位即工作区根下 sih-tools/lease/ledger/，对表围堰形，见 src/bin/lease.rs:251 注记；根判别即 sih-engine 与 sih-tools 双仓标记，见 src/bin/lease/attachments.rs:242。

#### 判据回算 {#data-critsweep}

- 判据单源是 registry.json 文件非内嵌拷贝：critsweep 定位序为 --registry 显式参、env、cwd 上溯引擎位 sih-engine/critsweep/registry.json、围堰兼容候选、root 下两形，见 src/bin/critsweep.rs 头注落差一；引擎位文件在档，ls sih-engine/critsweep 实测。
- 回算面即 GOV-002 判据三态加泊界路由加两账本在飞，只读零裁决，失效只降级不拦截且降级可见，见 src/bin/critsweep.rs 头注；判据正典 sih-engine/doc/governance/GOV-002-mainline-lock-v1.md。
- 例行快照写链由 gauge record 算并落链经 scribe 承载，read 与 record 两动作共用算半，见 src/bin/gauge.rs 头注行为链段。

## 逐工具职责与边界 {#tools}

以 ls src/bin 实测为准逐具一小节。各具完整落差申报单源在其源件头注，本节是索引不是复制；退出码三值约定全 bin 统一，即零成功、一违规或业务拒、二工具自身异常。

### acceptor {#tool-acceptor}

职责：引擎侧空腹判定包执行机，四查操作封闭词汇表与三态失败定位，零项目知识零命令字面。输入输出：acceptor --pack 判定包.json 加可选 --root，报告 JSON 落 stdout，退出码三值。依赖：std 自足，被检命令子进程超时六百秒。围堰来源：sih-tools/acceptor 0.2.0，lease-mergeleg6-parallel 簇J 移植。证据：src/bin/acceptor.rs 头注。

### ask3repeater {#tool-ask3repeater}

职责：三问确定性外壳的校验腿，不执行诘察、不生成记录、不落盘，写入腿归书简。输入输出：记录校验，退出码三值。依赖：引擎 lib ask3repeater 模块，见 src/lib.rs:8。围堰来源：引擎原生，承 SPEC-005 确定性外壳。证据：src/bin/ask3repeater.rs 头注。

### askroute {#tool-askroute}

职责：强制触发协议的引擎侧承载位，缩写到意图到动作链解析全部由包数据机械承载，表外缩写唯一出口是 unknown_action 三步兜底，禁即兴解释。输入输出：askroute route 缩写加可选 --pack，与 askroute check 加可选 --pack；缺省包为编译期内嵌引擎位 intents-v0。依赖：src/askroute 包，内嵌 intents-v0 与围堰包逐字节随迁。围堰来源：sih-tools/askroute 纯数据包形，gap-askroute-port 移植件。证据：src/bin/askroute.rs 头注。

### attnanchor {#tool-attnanchor}

职责：五行锚读数组装器即任务锚、在飞、泊界、链、纪律令，注入式回锚，退出码恒零失效降级可见。输入输出：出参严格 JSON 单键 additionalContext，新增 --root 与 --at 显式参。依赖：引擎 selector bin 泊界路由腿直调、两账本读、当日 trail 末笔、git 在 PATH。围堰来源：sih-tools/attnanchor anchor.py，lease-mergeall-parallel 簇A 移植。证据：src/bin/attnanchor.rs 头注。

### attractor {#tool-attractor}

职责：得一机械核对腿六子命令入口即 emit-contract、score、check、verify、sign、watch，全程零网络零 LLM 零目标仓写入。输入输出：六子命令，退出码零合规一违规二用法或环境，sign 三态。依赖：src/attractor 模块，见 src/lib.rs:20。围堰来源：引擎原生承 SPEC-014，围堰 tally 与 facet 已退役兼容只读，见工作区 AGENTS.md 文件索引执契行。证据：src/bin/attractor.rs 头注。

### basemgr {#tool-basemgr}

职责：基线向量与规约向量双种管理，freeze、replay、refreeze 三操作，重放只读零写入。输入输出：三子命令带 --vector 与 --command 等旗标，退出码三值。依赖：std 自足，被检命令子进程。围堰来源：sih-tools/basemgr 0.1.0，lease-mergeleg6-parallel 簇I 移植。证据：src/bin/basemgr.rs 头注。

### calllogtool {#tool-calllogtool}

职责：行式调用账本，append 三腿齐落与 render 投影再生与 reconcile 对账；选名避让引擎 src/lease 的 calllog 模块名。输入输出：calllogtool --root 根 加 append、render、reconcile、rebuild、import 子命令，退出码零成、二用法错或未实装位。依赖：ndjson 权威腿 flock 加投影腿；sqlite 索引腿未实装，见偏差节 D2。围堰来源：sih-tools/calllog 0.1.0，lease-mergeleg23-parallel 簇F 移植。证据：src/bin/calllogtool.rs 头注。

### cascade {#tool-cascade}

职责：级联上游洁净检查，路径即 id 不改名即删除新建，当前内容哈希对链上最近认证哈希。输入输出：build、check、orphans 三子命令，退出码三值。依赖：册文件与 trail ndjson；代码载体扫描腿未移植，见偏差节 D7；乐观级联被 locksview 直调。围堰来源：sih-tools/cascade 0.4.0，lease-mergeleg23-parallel 簇E 移植。证据：src/bin/cascade.rs 头注。

### checker {#tool-checker}

职责：空腹判定机，规则全部来自格式包 check 机器字段，封闭操作词汇表通用求值零文档形状知识。输入输出：checker --pack 包.json 目标.md 加可选 --reference，退出码三值。依赖：std 自足与 regex crate。围堰来源：sih-tools/checker 0.1.0，lease-mergeleg6-parallel 簇I 移植。证据：src/bin/checker.rs 头注。

### confledger {#tool-confledger}

职责：冲突账本，规格基准 confmodel-derivation v3，余额守恒与支付 fail-closed 与人席位无强制算子，休眠门零触发。输入输出：open、activate、mint、pay、balance、verify、replay、bills、preempt 九子命令，退出码三值。依赖：std 自足。围堰来源：sih-tools/confledger，lease-mergeleg6-parallel 簇I 移植。证据：src/bin/confledger.rs 头注。

### critsweep {#tool-critsweep}

职责：对话框内治理态回算器，判据面加泊界面加在飞面加散文对照面加裁判面封印，只读零裁决。输入输出：critsweep --at 日 --root 根，严格 JSON 单对象，退出码零回算成一裁判拒、二用法错。依赖：引擎位 registry.json 判据单源、引擎 selector bin 泊界路由直调、两账本读。围堰来源：sih-tools/critsweep sweep.py v1.2.0，lease-mergeall-parallel 簇A 移植。证据：src/bin/critsweep.rs 头注。

### elicit {#tool-elicit}

职责：轻信号叩问，三扳机即未登记词与召回零命中与矛盾候选，digest 消化闸验契约逐信号处置，载体语义承引用图可达与孤悬判定。输入输出：check、digest、suspend 三子命令，退出码三值。依赖：std 自足零第三方。围堰来源：sih-tools/elicit 0.2.0，lease-mergeleg6-parallel 簇J 移植。证据：src/bin/elicit.rs 头注。

### formatter {#tool-formatter}

职责：化格，格式归一，幂等构成性即同输入二遍格式化输出恒同。输入输出：formatter --pack 格式包目录 可重复加 --write 加目标列表，退出码零无需改、一已有改、二工具异常。依赖：引擎位包根候选序解析，见 src/bin/formatter.rs:383；被核阅在 T6 管线序前置，笔在核前。围堰来源：sih-tools/formatter 0.2.0，lease-mergeall-parallel 簇B 移植，域判定借围堰 scrutinator domain.py。证据：src/bin/formatter.rs 头注。

### gauge {#tool-gauge}

职责：秤星治理态读数计算核，read 与 record 两动作共用算半，公式 ga-2 三维，只报不判退出码仅指自身。输入输出：read 与 record 两子命令，record 经 scribe 落链。依赖：trail 读、两账本读、scribe bin 落链。围堰来源：sih-tools/gauge cli.py，lease-mergeall-parallel 簇A 移植。证据：src/bin/gauge.rs 头注；缺席面见偏差节 D1。

### identity {#tool-identity}

职责：零信任正身验证，组件十二件采集、身份串 v3 规范形、盐哈希与声明对表、human seat 五闸验真，留痕不签。输入输出：identity verify 加旗标，报告 json sort_keys 缩进二，退出码三值。依赖：sysctl、ps、ifconfig、curl 子进程采集，超时闸两秒与三秒对表围堰。围堰来源：sih-tools/identity 0.5.0，lease-mergeleg23-parallel 簇E 移植。证据：src/bin/identity.rs 头注。

### incubation {#tool-incubation}

职责：孵化回路契约校验器，包数据驱动谓词求值零语义裁量，锁步无状态同参双跑逐字节一致，只读零写入。输入输出：incubation --pack 包 加可选 --reference 与受检目标列表，退出码三值。依赖：std 自足与 regex crate。围堰来源：sih-tools/incubation checker，lease-mergeleg6-parallel 簇H 移植。证据：src/bin/incubation.rs 头注。

### latextool {#tool-latextool}

职责：LaTeX 书写辅助五子命令即 validate、autofix、block-create、suggest、knowledge，选名避让 LaTeX 控制词命名空间。输入输出：五子命令，退出码三值。依赖：本地知识库词条八十四条；compute 子命令未移植，见源件头注落差一。围堰来源：sih-tools/latex-helper 0.1.0，lease-mergeleg6-parallel 簇G 移植。证据：src/bin/latextool.rs 头注。

### lease {#tool-lease}

职责：租约治理写入引擎件，锁核腿 fixture 对等加收约执法面，覆盖域十二子命令即 open、lock、unlock、close、status、commit、bypass、reconcile、sweep、call-log、install-hooks、uninstall-hooks，见 src/bin/lease.rs:520-532 实测；收约闸序十一步含 SDDG 四判据门。输入输出：十二子命令，退出码三值。依赖：src/bin/lease/ 私有子模块族七件即 attachments、calllogface、closegate、commitlaw、guardlaw、sddgate、sweepcore，ls src/bin/lease 实测；scribe bin 落据外调。围堰来源：sih-tools/lease 1.46.0，SPEC-024 腿一 lease-lockcore-solo 与腿二 lease-commitlaw-parallel 批。证据：src/bin/lease.rs 头注与各子模块头注。

### locator {#tool-locator}

职责：确定性寻址，多载体结构化解析派生稳定标识，代码载体内嵌句读核心紧凑副本。输入输出：build、query、stale 三子命令，退出码三值。依赖：内嵌句读算法与 parser bin 同源同算法；语言包定位环境变量加相对候选，见源件头注落差五。围堰来源：sih-tools/locator 0.1.0，lease-mergeleg23-parallel 簇D 移植。证据：src/bin/locator.rs 头注。

### locksview {#tool-locksview}

职责：独立锁视图，acquire、release、status、check 四子命令，悲观五验加乐观级联；选名避让引擎 lease 侧 locks 语义位。输入输出：四子命令，退出码三值。依赖：锁台账 ndjson 与引擎 cascade bin 级联腿直调，CASCADE_BIN_OVERRIDE 测试缝。围堰来源：sih-tools/locks 0.2.0，lease-mergeleg6-parallel 簇I 移植。证据：src/bin/locksview.rs 头注。

### meter {#tool-meter}

职责：计量包裹，run 包裹执行被包裹命令并按日落计数册，count 三方对表汇总，crosscheck 链上对册面漏计检出，归因为启发式如实声明。输入输出：run、count、crosscheck 三子命令，退出码三值。依赖：被包裹命令子进程与 trail 读。围堰来源：sih-tools/meter 0.2.0，lease-mergeleg23-parallel 簇F 移植。证据：src/bin/meter.rs 头注。

### nomenclator {#tool-nomenclator}

职责：检词，术语登记核查，check、query、map 三子命令空腹只报不改。输入输出：三子命令带 --pack，退出码零零违例或查询出、一有违例、二工具异常。依赖：引擎位术语包 sih-engine/packs/nomenclator/core，ls 实测；register 写面未实装，见偏差节 D3。围堰来源：sih-tools/nomenclator 0.3.0，lease-mergeall-parallel 簇B 移植。证据：src/bin/nomenclator.rs 头注。

### parser {#tool-parser}

职责：句读解析，空腹 PEG 与词法回溯 VM 加条目投影，纯数据零引擎改动。输入输出：parse、entries、lint、vectors 四子命令，退出码三值。依赖：语言包引擎位 sih-engine/packs/parser 三包，ls 实测；词法模式引擎 std 自足回溯 VM。围堰来源：sih-tools/parser 0.1.0，lease-mergeleg23-parallel 簇D 移植。证据：src/bin/parser.rs 头注。

### projsnap {#tool-projsnap}

职责：派生快照投影器，纪律三件即零 LLM、同参双跑逐字节一致、投影与源不符以源为准，投影写面仅限派生节标记对之间。输入输出：build、check、diff 三子命令，缺省目标工作区根 AGENTS.md，退出码三值。依赖：difflib SequenceMatcher 算法逐字对表移植。围堰来源：sih-tools/projsnap 0.1.0，lease-mergeleg6-parallel 簇J 移植，承 DES-018。证据：src/bin/projsnap.rs 头注。

### registrydemo {#tool-registrydemo}

职责：腿五插件槽位演示 bin，真件 HeartbeatProvider 加 mock echo 加 mock fail 加 manifest 扫描读数，演示 ToolRegistry 统一注册形；在役 MCP 分派面接线候收口批，见候批项 P2。输入输出：运行即演示出参，零 LLM 零网络。依赖：sih_engine::tools_registry 与 mcpserver alpha heartbeat 只读投影。围堰来源：非移植新件，承 SPEC-025 插件槽位协议节，lease-mergeall-parallel 簇C。证据：src/bin/registrydemo.rs 头注。

### retriever {#tool-retriever}

职责：温故宿主命令面，recall 单子命令四轴检索，只报不判，根定位两形即双仓目录与 canonical。输入输出：recall 参数十件，退出码三值，不读系统钟。依赖：src/retriever 模块，见 src/lib.rs:13。围堰来源：引擎原生，承 SPEC-007 与 SPEC-008 修订六。证据：src/bin/retriever.rs 头注。

### scribe {#tool-scribe}

职责：书简命令行面，本名回滚，链面唯一命令行写读位。输入输出：append、verify、query、intent、park、record 六子命令加 vectors 冻结向量集，见 src/bin/scribe.rs:3-4，退出码三值。依赖：src/event_stream 模块，见 src/lib.rs:11。围堰来源：引擎原生承 SPEC-006，工具侧 scribe 已退役兼容只读，见工作区 AGENTS.md 工具层静态审计节。证据：src/bin/scribe.rs 头注。

### scrutinator {#tool-scrutinator}

职责：核阅组件命令行面，视图零写零 LLM，规则包编译期内嵌默认三包。输入输出：位置参数形与 --target 旗标形双形并存，退出码三值，空载形零发现退出零。依赖：src/scrutinator 模块与内嵌三包，见 src/scrutinator/asset.rs:28。围堰来源：围堰 scrutinator 融回承 SPEC-013，第一件融回先例，DEC-013 修订一实录。证据：src/bin/scrutinator.rs 头注。

### selector {#tool-selector}

职责：路择三路路由，谓词包驱动按声明序逐件机械求值首败定路，批级告警只记不改路，参照时间显式给参不读系统钟缺参判败 fail-closed。输入输出：selector route --pack 谓词包目录 加可选 --reference-time 与材料列表，输出 json 报告，退出码三值。依赖：引擎位谓词包 sih-engine/packs/selector 三包，ls 实测；被 attnanchor 与 critsweep 泊界腿直调。围堰来源：sih-tools/selector 0.4.0，lease-mergeleg23-parallel 簇E 移植。证据：src/bin/selector.rs 头注。

### sihmcp {#tool-sihmcp}

职责：MCP 线 Rust 载体入口，stdio 与 streamable HTTP 双传输，进程始即连接生，stdio 管道关闭即断开收约。输入输出：SIH_TRANSPORT 环境变量择传输，HTTP 绑 127.0.0.1:8765，SIH_HTTP_BIND 与 SIH_HTTP_ENDPOINT 可覆写，见 src/bin/sihmcp.rs 头注；会话内在役 alpha 相九读数加 beta 相十写具，见工作区 AGENTS.md MCP Tool 调用义务节。依赖：sih_engine::mcpserver 十五源件，ls src/mcpserver 实测。围堰来源：非围堰移植，承 DEC-023 直接替换决策，行为对等基准旧 Python 载体 mcpline 现冻结兼容。证据：src/bin/sihmcp.rs 头注。

### tally {#tool-tally}

职责：执契，check 对材料做 R1 至 R9 逐条核对与三态映射四值处置、verify 同输入重放逐字节比对、sign 仅裁决通过时外调 scribe 落据、watch 批量重放浮异常视图、assemble 确定性装配。输入输出：check、verify、assemble、watch、sign 五子命令，退出码三值。依赖：scribe bin sign 落据外调。围堰来源：sih-tools/tally 1.0.0，lease-mergeleg23-parallel 簇F 移植。证据：src/bin/tally.rs 头注。

### viewer {#tool-viewer}

职责：聚合输出组件，alarms 异常视图、heartbeat 心跳视图、settle 结算视图，同参双跑逐字节一致是构成性纪律。输入输出：三子命令，--trail 可重复多链装载，退出码三值。依赖：src/view 模块，见 src/lib.rs:17，与 trail 读。围堰来源：引擎原生承 DEC-007 聚合输出组件。证据：src/bin/viewer.rs 头注。

### watchcheck {#tool-watchcheck}

职责：稽，主树活写对表哨，脏文件集减锁面加链笔声明面加豁免面的无主清单判定，只呈报不代裁。输入输出：watchcheck check --at 日 --root 根，退出码三值。依赖：git status 双仓解析、锁面镜像 ndjson、当日 trail。围堰来源：sih-tools/watchcheck 0.2.0，lease-mergeleg23-parallel 簇F 移植。证据：src/bin/watchcheck.rs 头注。

### wikirecall {#tool-wikirecall}

职责：三通道确定性召回即词面触发集、骨架全读清单、关系图一跳加语义通道，并集出应读书单，零模型零网络。输入输出：--repo 与 --query 等旗标，缺省语义层 K 取三，退出码零成功、二用法错或异常。依赖：数学仓语料本地检索；checkcite 消费面未移植，见偏差节 D5。围堰来源：sih-tools/wikirecall，lease-mergeleg6-parallel 簇H 移植。证据：src/bin/wikirecall.rs 头注。

## 工程实现 {#engineering}

### 关键接口 {#key-interfaces}

- ToolProvider trait 四面即 name、description、input_schema、call，async 承载；ExternalPluginBridge 四面签名缺省体 BridgeUnsupported，见 src/tools_registry.rs 头注三件套段与 A4 申报段。
- 链笔 Event 与 GENESIS_PREV_HASH 与 compute_event_hash，见 src/event_stream/hash.rs:9 与 :17。
- 判定包信封三件族 envelope_version 整数一严格判，见 src/scrutinator/asset.rs:44-47 信封硬切换实测；formatter 与 nomenclator 头注落差六同形申报。
- 退出码三值约定全 bin 统一，各 bin 头注在档。

### 关键算法 {#key-algorithms}

- 每日链哈希与整链校验，见 src/event_stream/hash.rs。
- difflib SequenceMatcher 逐字对表移植含 autojunk 门，见 src/bin/projsnap.rs 头注落差四。
- 词法回溯 VM 加 PEG 加条目投影，parser 与 locator 同源同算法，见 src/bin/locator.rs 头注第三段与 src/bin/parser.rs 头注落差一。
- TF-IDF 统计加权与余弦相似度全量暴力排序前 K，见 src/bin/wikirecall.rs 头注。
- Neumaier 补偿求和对表围堰求和形，见 src/bin/wikirecall.rs 头注落差一。

### 测试与对表形 {#tests-form}

- tests/ 集成测试五十一件，ls tests/*.rs 实测五十一，时点 2026-09-16；判据面以 mergeall_t1 至 t6、lease_mergeback_t2 至 t7、leg4_wiring、gap_ 前缀族命名承载，ls tests 实测。
- 对表方法即 SPEC-024 同参双跑形条款：围堰件与引擎件同参双跑逐字节比对加退出码对齐。
- 如实申报：本档未重跑全量双跑对表，对表一致性声明承各移植件头注落差申报与上述测试件在档事实；本档撰写批的验证面是三审管线与本节证据路径可复查性。

## 融回决策承接 {#decisions}

### DEC-013 三步曲与三查 {#dec-dec013}

- 第一步引擎开发：SDD 规格先行即本档；TDD 失败测试先行先红后绿，判据件在 tests/，ls tests 实测。
- 第二步双模并存切换：T6 管线认证位改指引擎件，并存期以双跑一致为切换判据；2026-09-14 收尾批 lease-cutover-parallel 完成切换面引擎位切换并宣告围堰退役，git log c115d27 归并实测；工作区 AGENTS.md 工具层静态审计节围堰退役宣告段在档。
- 第三步工具侧退役：围堰转冻结只读兼容零改动保留作对表基准，物理退役候终裁，见工作区 AGENTS.md 同段与候批项 P7。
- 三查：验收判据全过即 SPEC-025 判据逐条现状见下节与偏差节；接口契约未变即围堰 CONTRACT 退役加注不删接口先例，sih-tools/lease/CONTRACT.md 只读在档；回迁债已评估即本档偏差节逐条承载词形，不默写。

### SPEC-025 六腿与判据现状 {#dec-spec025}

- 腿一治理例行三件、腿二 T6 管线四件、腿三治理机械七件、腿四 MCP 面收编、腿六长尾收口：bin 位全数在役，见本档逐工具节 ls 实测；腿五插件槽位 trait 加 manifest 加演示 bin 在役，进程外桥实装缺席显式申报。
- A1 行为对表：同参双跑判据承 tests/ 判据件与各移植件头注落差申报。
- A2 rmcp 单面：缺口如实申报，MCP alpha 读数面 critsweep 与 gauge 腿仍 spawn 围堰 Python，见 src/mcpserver/alpha.rs:168 与 :219-256 实测；beta lease 写面仍 spawn 围堰 uv run，见 src/mcpserver/passthrough.rs:58 实测；切引擎 bin 位归快照切换批另令，见候批项 P1。
- A3 registry 统一：演示位成立，见 registrydemo 与 mergeall_t5_registry 判据件；在役 MCP 分派面接线缺席，见 src/bin/registrydemo.rs 头注。
- A4 插件接口预留：签名在役实装缺席显式申报，见 src/tools_registry.rs 头注 A4 申报段，合 SPEC-025 A4 允许形。
- A5 退役序：围堰冻结只读兼容已宣告，见工作区 AGENTS.md 2026-09-14 段。
- A6 依赖零新增：成立，见 sih-engine/Cargo.toml:7-23 实测十六件，融回批未增。
- 不迁裁断三件：parking 纯数据零码、estcore 与 counter 并入面或消亡，见 SPEC-025 腿切分节。

### DEC-023 MCP 载体位 {#dec-dec023}

- 载体三件位即引擎根 Cargo 增 rmcp、axum、tokio、async-trait、anyhow 五依赖，见 sih-engine/Cargo.toml:18-22 实测；lib 模块 src/mcpserver；bin src/bin/sihmcp.rs；与 DEC-023 载体与结构位节一致。
- 四段实施序：段一 alpha 九具 stdio、段二 beta 十写具、段三 HTTP 面加台面、段四 parity 加切换加冻结；段一至段三落笔在案承 DEC-023 与工作区 AGENTS.md MCP 节，双通道与管理台全量实测在案，识别写面扩量见 recognize-solo 归并，git log 894faca 实测。
- 本档对 src/mcpserver 零触碰只记录；MCP 载体壳与工具本体 Rust 化是两条线不混，承 DEC-023 工具本体边界节；serverInfo 名称变更是有意变更，承 DEC-023 甲表认领节。

### DEC-024 SDDG 四判据 {#dec-dec024}

- SDDG-1 典在码前：本档即规格物，DES 路径文件。
- SDDG-2 规格束齐备：逐 bin 源件头注正典指针在档，本档逐工具节证据路径逐条可复查。
- SDDG-3 偏差有承载：本档偏差节逐条承载词形，候批项带批次指向。
- SDDG-4 测试同批：tests/ 判据件五十一件在档。
- B 层两道门执法位即引擎收约闸，见 src/bin/lease/sddgate.rs 头注，判据定义单源在 DEC-024 本模块只执法；C 层扫描位 critsweep GOV2-C6 在 registry 判据面，见 src/bin/critsweep.rs 头注；D 层度量位 gauge sddgate 维度缺席如实申报，见偏差节 D1。

## 已知偏差与候批项 {#deviations}

偏差，对表落差，单源在各源件头注，本节是索引：

- D1 gauge 三只读子命令 gchart、gqueue、contrib 与 sddgate 第四维未移植，见 src/bin/gauge.rs 头注落差一、二；影响面即 DEC-013 门开判据的引擎读数位与 DEC-024 D 层读数位缺席。
- D2 calllogtool sqlite 索引腿未实装，rebuild 与 import 占位退出码二，见 src/bin/calllogtool.rs 头注落差一、二。
- D3 nomenclator register 写面未实装，调用即错误 JSON 退出码二，见 src/bin/nomenclator.rs 头注落差一。
- D4 latextool compute 子命令未移植，保留参数面调用即未移植申报，见 src/bin/latextool.rs 头注落差一。
- D5 wikirecall checkcite、metrics、ab_run 未移植，见 src/bin/wikirecall.rs 头注未移植面段。
- D6 locator vectors 未移植、markdown 载体 lite 形、yaml 行位恒 null、toml 键序字典序，见 src/bin/locator.rs 头注落差一至四。
- D7 cascade 代码载体扫描未移植，build 产册 code 节恒空，见 src/bin/cascade.rs 头注落差一。
- D8 locksview 级联子进程超时未实装，见 src/bin/locksview.rs 头注落差二。
- D9 lease bin 对等域十二子命令，围堰契约十三子命令中 claim、unclaim、check 三子命令未入对等域，见 src/bin/lease.rs:520-532 与 sih-tools/lease/CONTRACT.md:53 对照实测；MCP lease_claim 现行 spawn 围堰承载，见 src/mcpserver/passthrough.rs lease_base。
- D10 lease chained workspace 门族归腿一后继批，见 src/bin/lease.rs 头注覆盖域段。
- D11 移植共性边缘形差逐件头注申报在案，即正则方言差见 checker 与 incubation、行界差多具、JSON 浮点最短表示差多具、argparse 报文形不对齐多具，退出码与判定主形均对齐。

候批项：

- P1 快照切换批：MCP 生产面切引擎 bin 位，消解 alpha 与 passthrough 的围堰 spawn 位，调用形改写收口，见工作区 AGENTS.md 工具层静态审计引擎融回位段；本任务纪律四另令。
- P2 腿五收口批：MCP 分派面 defs 与 dispatch 改走 ToolRegistry，ExternalPluginBridge 进程外桥实装，见 src/tools_registry.rs 头注在役面改造段与 src/bin/registrydemo.rs 头注。
- P3 gauge 补齐批：contrib 与 sddgate 维度与 gchart、gqueue 融回，为 DEC-013 门开判据引擎读数位前置件，偏差 D1 承载。
- P4 lease 对等域补齐批：claim、unclaim、check 三子命令引擎位，偏差 D9 承载。
- P5 cascade 代码载体腿补齐批，偏差 D7 承载。
- P6 写面与依赖裁决批：nomenclator register 写面、calllogtool sqlite 腿、latextool compute 三件归并一议，偏差 D2、D3、D4 承载；涉新增依赖者须单独申报，承 SPEC-025 A6。
- P7 围堰物理退役与目录清理候终裁，见工作区 AGENTS.md 围堰退役宣告段。

## 边界 {#boundary}

- 本档只记录不改造：src/mcpserver 零触碰、围堰零改动、真实账本与链零写入，三限时点即本档撰写批纪律。
- 证据路径约定：引擎文件用引擎仓相对路径，围堰文件用工作区相对路径；ls 实测、cat 实测、grep 实测指本档撰写批 2026-09-16 在本机工作区执行。
- 本档不承载逐工具落差全文，落差单源在各 bin 源件头注，本档偏差节是索引；两处冲突时以源件头注为准并回修本档。
- 命名面：calllogtool、locksview、latextool 三处避让选名申报在各自头注，语义忠实终裁候立名程序人节点，承 DEC-017 程序。
- 测试与试用一律用系统临时目录沙箱账本，本档不附真实账本数据。

## 跨领域引用 {#relation}

- sih-engine/doc/decision/013-mergeback-gate.md，融回门机制与三步曲三查
- sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md，全量融回规格与六腿判据
- sih-engine/doc/decision/023-mcp-rust-carrier.md，MCP 载体直接替换
- sih-engine/doc/decision/024-sdd-completeness-gates.md，SDDG 四判据与两道门
- sih-engine/doc/spec/SPEC-024-lease-mergeback-gap-v1.md，租约融回落差规格与同参双跑形条款
- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md，判据正典
- sih-engine/doc/design/DES-018-derived-snapshot-design-v1.md，projsnap 上游设计
- 工作区 AGENTS.md，围堰退役宣告与引擎融回位与 MCP 节
- sih-tools/lease/CONTRACT.md，围堰契约，只读对表基准

## 设计修订记录 {#revisions}

v1 即 2026-09-16 初版：bin 层三十四源件、模块层十 lib 模块、数据面三件、逐工具三十四节、四正典承接、偏差十一条候批七条。
