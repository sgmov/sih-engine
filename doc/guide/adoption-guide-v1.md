# 下游采纳指南 v1：把司衡引擎接入你自己的项目

写给要把司衡引擎接入自己项目的人。不要求读哲学仓，不要求会 Rust 内部。读完你能完成域自举、客户端注册、代理约束落位，并亲自验算第一笔治理留痕。本件与 AGENTS 模版与决策档构成接入动线三件，指针见文末正典节。

## 概览 {#overview}

- 三条承诺与三方角色，接入前的心智对表::[心智模型](#mindset)
- 缺省路径六步，单项目单域一块配置::[单项目接入](#default)
- 多域集中治理与令牌管理台，显式选用档::[进阶 HTTP 面](#http)
- 常见疑问十二问::[问答](#faq)
- 本文引用的决策档设计档与规格档::[正典指针](#canon)

## 心智模型 {#mindset}

三条承诺先立住，全部接入动作都服务它们。

其一，注意力只投异常。确定性程序处理绝大多数操作，你只在告警与裁决点出现。

其二，一切可复算。每个治理动作落哈希链，链文件每日一件，任何时候一条命令验证整条链完整未篡改。

其三，材料同权。人的直觉与 LLM 的建议在裁决前是同等地位的参考材料，谁都不能直接当裁决用。写入权在确定性程序与租约闸，LLM 产出只是待校验材料。

接入后的三方角色：LLM 只生成符号材料；确定性程序执法，即租约闸与哈希链与判据扫；你作为人节点只做四类事，即下令、裁决、看回显、异常介入。

缺省形态一句话：单项目即单域，单域即一块 stdio 客户端配置，无后台服务、无端口、无令牌。HTTP 多域与管理台是显式选用的进阶档，见第三节。缺省面裁定见 DEC-026。

占位符约定，全文两形。`{{ENGINE_WORKSPACE}}` 即引擎工作区根，其下应有 sih-engine 仓；`{{PROJECT_ROOT}}` 即被治理项目根，域自举后成为域根。

## 单项目接入缺省路径 {#default}

### 前置 {#default-prereq}

- Rust 工具链，引擎 edition 2021，`cargo build` 一律
- 引擎工作区，含 sih-engine 仓。推荐 sih-tools 仓并列在位，两处既有依赖会用到它，即写面正身签发的 identity 件与域自举落地的任务包模板源，详见问答第七问
- uv 包管理器，仅写面需要
- 被治理项目是一个 git 仓。新项目先补齐身份两件，域自举前置检查会自动判：`cd {{PROJECT_ROOT}} && git init && git config user.name "你的名字" && git config user.email "你@example.com"`

### 第一步 构建引擎 bin {#default-build}

`cd {{ENGINE_WORKSPACE}}/sih-engine && cargo build`

产物在 target/debug 下。常用五件即 scribe 书简、critsweep 判据扫、sihmcp MCP 服务器、lease 租约、retriever 温故。生产调用面唯一形态即引擎 bin 位。

### 第二步 sih init 域自举 {#default-init}

域自举把 {{PROJECT_ROOT}} 落成正典形域。sih init 是域自举单步窄口，零签发零镜像零客户端注册，签发与镜像与客户端注册归 sihmcp bootstrap 全链，各守各位。窄口两动作：

其一，一次性签发本域标识牌。中央登记册本域根尚无 active 行时直接 init 即拒，拒语会指引到签发位。签发走管理台两步确认：按第三节起服，浏览器开 /tokens 签发表单，填标识词形与本域根与档位 domain_write，确认落笔即停服。标识牌是明文短标识防呆锚点非安全凭据，一次性动作，签完即回缺省档。

其二，跑窄口：`cd {{ENGINE_WORKSPACE}}/sih-engine && target/debug/sih init --root {{PROJECT_ROOT}}`

域根缺省取当前目录，可用 --root 显式传。前置检查六项自动逐项判词，任何一项不过即拒并给处置语，全链零写入可修复重跑：一 域根存在且为目录。二 域根下 .git 在位。三 中央登记册本域 active 标识牌行在档且同路。四 域根不等于中央根，司衡工作区自身是第一域映射形，不走本步。五 sih/domain.json 缺席，域已在册即拒，幂等守卫明确拒非静默，无 reinit 旗标。六 半态拒，sih 目录存在且非空但声明文件缺席时位人节点处置。

出参严格 JSON 单对象，precheck 六项逐项 pass 或 fail 或 unverified，domain 字段含域名与域自举时与首笔哈希与链验判词。落地五步即建目录四件、写域声明文件、落模板三件、写域自述、域登记首笔落链，落毕自动验域，scribe verify 判词 valid 才算成。

落地后 {{PROJECT_ROOT}} 下新增 sih 树：

- sih/domain.json 域声明文件七字段
- sih/README.md 域自述卡
- sih/event/trail/ 哈希链，域登记首笔已落
- sih/ledger/ 域内登记册与锁册
- sih/state/plan/ 与 sih/state/parking/materials/ 任务包与停泊，含三份模板

隔离一句：init 成功时自动把 `sih/` 一行幂等写入域根 `.git/info/exclude` 并以 `git check-ignore` 机械验证，治理飞轮不进你的 git，项目可随意公开。

两形提示。运行 init 时不要把 SIH_ROOT 环境变量指到 {{PROJECT_ROOT}}，中央登记册落中央根，指错会落进项目内，详见问答第六问。

版本提示。sih init 是本批新增薄壳位，usage 即 sih init [--root <域根>]，--help 出用法。若你构建出的 bin 集尚无 sih，说明引擎版本早于本批，先升版本，或走第三节管理台两步域自举全链。

### 第三步 MCP 注册 {#default-mcp}

在你客户端的 MCP 配置里加一块，下形为通用形，路径换成你机器上的绝对路径，可复制单行形：

`{"mcp":{"servers":{"sih":{"type":"stdio","command":"{{ENGINE_WORKSPACE}}/sih-engine/target/debug/sihmcp","env":{"SIH_ROOT":"{{PROJECT_ROOT}}","SIH_MCPLINE_CODE_ROOT":"{{ENGINE_WORKSPACE}}"},"enabled":true,"timeoutMs":60000}}}}`

字段释义五条：

- SIH_ROOT 显式指域根最稳，缺省自当前目录向上找标记
- SIH_MCPLINE_CODE_ROOT 指引擎工作区，写面的 scribe 与 lease 二进制位和正身签发都从这里解析；引擎与治理根同源布局时可省，缺省与数据根同源
- 客户端配置于新会话读取，既有会话不重载，重开新会话方见 sih 工具面
- 在役验证，新会话中 MCP 面 locks_read 心跳可调即配置在役
- 本地单人可信档可在 env 加 SIH_MCPLINE_AGENT_CLASS 值 local，写面全集十二具在役；缺省 external 分级在役十具，record_direct 与 lease_unclaim 两具不透传

在役工具面：读面九具，即 chain_query、chain_verify、critsweep、heartbeat、locks_read、retriever_recall、naming_guide、nomenclator_query、nomenclator_check；写面缺省十具，即 lease_open、record_intent、record_append、record_park、lease_lock、lease_unlock、lease_wait_turn、lease_claim、lease_commit、lease_close。

### 第四步 AGENTS.md 约束落位 {#default-agents}

把 doc/guide/agents-template-v1.md 的可复制骨架落到 {{PROJECT_ROOT}}/AGENTS.md，替换两个占位符。骨架三节即 MCP 注册、sih 强制触发协议、纪律与会话启动。已有 AGENTS.md 的项目按节合并，冲突时以引擎报文与链为准。

### 第五步 第一次治理动作走查 {#default-walkthrough}

只读走查三件，全部不写零风险。其一验链：`cd {{PROJECT_ROOT}} && latest=$(find sih/event/trail -name '*.ndjson' | sort | tail -1) && {{ENGINE_WORKSPACE}}/sih-engine/target/debug/scribe verify --trail "$latest"`

其二判据扫：`cd {{PROJECT_ROOT}} && {{ENGINE_WORKSPACE}}/sih-engine/target/debug/critsweep --at $(date +%F) --root .`

验链 status valid 即域登记首笔在案且整链未篡改。判据扫出严格 JSON 单对象，achieved 达成、in_flight 在飞、sunk 沉底三态；新城判据登记未布时以降级注记如实呈现，不是错误。判据扫是你的常态告警面，某条主线判据超过阈值无程序活动痕迹即沉底，那是唯一需要你主动过问的常态信号。

MCP 面走查，新会话按序调三具即冷启动体检，locks_read 与 critsweep 与 heartbeat，再加 chain_verify 验当日链。

首笔写走查，可选，走租约五步链，意图笔与认证笔先于收约。一 lease_open 立会话，package 给任务包路径，intent 给意图件路径，即域自举落地的 sih/state/plan/ 下两份模板填后传参，repo 传你自己的仓相对路径。二 lease_lock 锁定要写的路径。三 施工改件。四 record_intent 追加意图笔加 record_append 追加认证笔。五 lease_unlock 放锁加 lease_commit 机械落笔加 lease_close 收约。一连接恰好一会话，未立会话一切写拒。

### 第六步 验证清单 {#default-checklist}

- cargo build 零错，target/debug 下 scribe 与 critsweep 与 sihmcp 在位
- sih init 退出码零，出参 precheck 六项 pass，sih/domain.json 与 sih/README.md 与四目录在位
- scribe verify 判词 valid
- critsweep 出严格 JSON 单对象，降级注记如实
- 新会话 tools 列表见 sih 工具面，locks_read 可调
- 未立会话直调 record_append 被拒且载荷含 reason_code，即教学面在役
- AGENTS.md 三节在位，占位符零残留

## 进阶：HTTP 多域与管理台 {#http}

何时才需要：一个引擎实例集中服务多个项目域；客户端走 HTTP 而非 stdio，比如其他语言或远机形态；需要令牌生命周期管理台。单项目缺省档日常用不到本节，唯一交叉点即第二节的一次性签发。

起服与面览。SIH_TRANSPORT 置 http 起服，缺省绑 127.0.0.1:8765，SIH_HTTP_BIND 与 SIH_HTTP_ENDPOINT 可覆写：`cd {{ENGINE_WORKSPACE}}/sih-engine && SIH_TRANSPORT=http target/debug/sihmcp`

同进程单端口四面：/ 视图面板入口页；/tokens 令牌管理台，即列表加签发加撤销加域自举三表单；/manual AI 使用说明书直出；/mcp streamable HTTP 识别写面。

令牌与绑域。标识牌是明文短标识防呆锚点非安全凭据。台账行六字段即 token_id 与 domain_root 与 scope 与 status 与 issued_at 与 issued_by，scope 三档即 domain_write 域写档、readonly 只读档、custom 零缺省放行。签发与撤销与域自举三个动作都是两步确认防手滑，确认前零写入。域自举两步确认直调全链同一核心，台面路径不含客户端注册。

HTTP 客户端形。域自举全链 --client-config 给参时写的是 HTTP 形 payload，type http 加 url http://127.0.0.1:8765/mcp 加 Authorization Bearer 头加 enabled 真值加 timeoutMs 60000。连接头携 Bearer 标识牌，写操作只落所绑域；HTTP 面恒 external 最小分级。此形与第二节 stdio 形不可混写。

登记册两形。中央册在 {{ENGINE_WORKSPACE}}/sih-tools/mcpline/ledger/tokens.ndjson，域内镜像在 {{PROJECT_ROOT}}/sih/ledger/tokens.ndjson，域自举时逐字段恒等镜像落位。

安全边界如实申报：管理台是本机回环面零防护形，台面路由零鉴权，只在本机回环使用，勿暴露到外网。

## 问答 {#faq}

问：要联网吗。答：不要，构建之外全程本地。

问：要开端口吗。答：缺省档日常零端口零后台服务，stdio 是客户端拉起的子进程；仅域自举前的一次性签发会短暂起一次管理台，回环即停。进阶档绑 127.0.0.1:8765 回环，可用 SIH_HTTP_BIND 换址。

问：单项目要令牌吗。答：stdio 日常零 Bearer 令牌。域自举前需一次性签发域标识牌，那是明文短标识防呆锚点，不是安全凭据，签发走管理台两步确认，签完即回缺省档。

问：为什么 init 前要先签发。答：sih init 是域自举单步窄口，零签发零镜像零客户端注册，它从中央登记册取本域 active 标识牌行落域；签发位与镜像位与客户端注册位归域自举全链，各守各位。

问：项目还不是 git 仓。答：先 git init 加 git config user.name 与 user.email，前置检查第二项的拒教文案同此指路。

问：sih 树会进我的 git 提交吗。答：不会，init 成功时自动把 sih/ 一行写入域根 .git/info/exclude 并机械验证，治理飞轮不进你的 git，项目可随意公开。

问：重复跑 init 会怎样。答：幂等守卫明确拒非静默，sih/domain.json 在位即域已在册，无 reinit 旗标。重开候人节点先处置既有域状态。

问：引擎装项目里还是项目外。答：两形都可。项目外分置时用 SIH_MCPLINE_CODE_ROOT 指引擎工作区；引擎在治理根内的同源形可省此键，缺省与数据根同源。运行 init 与起服时 SIH_ROOT 保持指中央根，指错会把中央登记册落进项目内。

问：为什么写面要装 uv 与 sih-tools。答：写面正身签发经 identity 件采集，现役承载即 uv run 加 sih-tools/identity；域自举任务包模板源也在 sih-tools。未装时读面全功能，写面 lease_open 会在正身签发位拒。两件依赖融回引擎后此答会收窄，以仓内现行文档为准。

问：写操作被拒。答：被拒是教学。读错误载荷 reason_code 与 suggested_action 改做法，勿原样重试。

问：链 verify 破了。答：红证文化，链文件留笔不删，人节点对表后重跑 verify 复核。

问：多项目怎么接。答：缺省形一域一块配置，多项目即多块 stdio 配置，各自 SIH_ROOT 指各域根；或进阶档一个 HTTP 实例多域绑牌。

问：支持什么平台。答：本文命令形按类 Unix shell 写就；Windows 未经验证，如实申报。

问：退出码怎么读。答：三值即法律，零合规、一违规、二工具自身异常。管道掩退出码是禁手。

## 正典指针 {#canon}

- DEC-026：doc/decision/026-adoption-default-stdio-v1.md，缺省面裁 stdio，HTTP 面转显式选用，只裁缺省面不裁通道存废，本批在途
- DEC-023：doc/decision/023-mcp-rust-carrier.md，MCP 载体 Rust 形
- DES-014：doc/design/DES-014-mcp-beta-security-model-v1.md，写面安全模型与分级矩阵
- DES-015：doc/design/DES-015-mcp-http-multitenant-auth-v1.md，多域识别与域自举全链
- SPEC-023：doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md，读面契约
- 配套模版：doc/guide/agents-template-v1.md，AGENTS.md 最小可配形
- AI 使用说明书：sih-tools/mcpline/AI-MANUAL.md，HTTP 面 /manual 直出
