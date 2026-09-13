# SPEC-025 全量工具融回与插件槽位规格

令源：用户 2026-09-14 goal 令「所有的工具要全部融回到 rust，sih-engine 发布是一个完整的 rmcp 工具，而不是带了一个 sih-tools 的东西。但是融回要预留 tools 工具的槽位，未来要支持插件的接入。多子代理并行运行。」

## 概览 {#overview}

sih-tools 全部生产工具融回引擎 Rust；发布形即 sih-engine 单一 rmcp server 即 sihmcp 承载全部工具面，运行时零 sih-tools spawn 依赖；融回预留 tools 插件槽位即 ToolProvider trait registry 与外部插件协议接口，插件接入为后向兼容扩展位。融回承接 SPEC-024 三腿先例即行为对表加 TDD 先红后绿加落差如实申报，施工形按批并联多子代理。

## 腿切分 {#legs}

六腿按耦合与在役频率切分，行数为围堰 Python 现行实测即 2026-09-14 摸底：

- 腿一治理例行三件约 2820 行：attnanchor 410 加 critsweep 1333 加 gauge 1078，即会话启动回锚与判据扫与三维读数，最高频在役面。
- 腿二 T6 管线四件约 3120 行：formatter 329 加 nomenclator 613 加 parser 1426 加 locator 756，即化格核阅链的格式归一与检词与句读与寻址。
- 腿三治理机械七件约 3470 行：selector 800 加 cascade 497 加 identity 503 加 tally 527 加 meter 272 加 watchcheck 380 加 calllog 488，即路由与级联与正身与执契与计量与守卫与调用账本。
- 腿四 MCP 面收编：mcpline 全行为对表入 sihmcp，引擎 rmcp 面已承载十九具读数与识别写面承 SPEC-023 加 DES-014 加 DES-015，收编余量即 HTTP 8765 面与 web 管理台与 tokens 管理与 bootstrap/init 全链与 writeface 行为对表。
- 腿五插件槽位架构：ToolProvider trait registry 统一注册形加外部插件 manifest 协议接口预留。
- 腿六长尾收口约 8000 行：elicit 220 加 askroute 67 加 acceptor 228 加 basemgr 301 加 confledger 553 加 projsnap 263 加 checker 348 加 incubation 1110 加 locks 481 加 latex-helper 2539 加 wikirecall 1562 加 proposition 1788。
- 不迁裁断三件：parking 即纯数据目录零码、estcore 与 counter 零码并入面或已消亡，切换批逐件核销。

## 插件槽位协议 {#plugin}

- 内置统一形：全部工具实现 ToolProvider trait即 name 加 description 加 input_schema 加 call 四面，sihmcp 经 ToolRegistry 动态注册承载 MCP tools 面，废弃散点注册。
- 外部插件预留：插件清单形 plugin manifest，目录扫描 sih/plugins 逐件 plugin.json 载 name 加 transport 加 command 加 tools 四键，进程外 stdio JSON-RPC 桥协议接口声明；本规格只立 trait 与 manifest 解析与接口签名，进程外桥实装候后批申报。
- 零 LLM 零网络红线对表 A9 承袭：插件协议不放宽引擎执法面红线。

## 验收判据 {#criteria}

- A1 行为对表：每件工具 CLI 出参与退出码三值与金向量对表围堰现行文承 SPEC-024 同参双跑形条款，落差如实申报。
- A2 rmcp 单面：MCP tools 面全数经 sihmcp 承载，会话内零 sih-tools 子进程 spawn。
- A3 registry 统一：内置工具全数 ToolProvider 注册，零散点注册残留。
- A4 插件接口预留：manifest 解析与外部协议 trait 在役，进程外实装缺席显式申报。
- A5 退役序：切换批后 sih-tools 生产工具转冻结只读兼容，生产写位与 spawn 主用位全数引擎。
- A6 依赖零新增：融回面零新增外部 crate腿四 HTTP 面承既有 axum，插件桥实装批如须新增须单独申报裁决。

## 测试计划 {#tdd}

逐工具先红后绿：红即引擎侧模块缺席或金向量缺席，绿即行为对表测试通过。每腿一批或数批，批名 lease-mergelegN 形加形后缀，金向量捕获承 SPEC-024 T1 形。腿四 MCP 行为对表承 mcpdual-parallel 双跑对表先例。

## 边界 {#boundary}

- 围堰源码零改动红线全程维持至切换批。
- 引擎 Cargo 依赖面零新增（A6）；sqlite 面维持 ndjson 事件序推导落差申报形。
- 腿内工具间共享逻辑如 identity 之于 lease 与 calllog 之于多工具按引擎既有模块单源化，零复制粘贴。
- 上链前必须等绿；读 findings 不只看退出码；锁被他在持即报不绕行。
- 多子代理并行施工时子代理只写工地源码与测试，git 暂存与提交与结算归主线。
