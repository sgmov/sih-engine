# AGENTS 约束模版 v1：下游项目代理约束的最小可配形

本件是模版不是正典本体。把第二节可复制骨架整段落到你项目的 AGENTS.md，替换占位符后按裁剪指引收窄。接入动作全序见 doc/guide/adoption-guide-v1.md。

## 概览 {#overview}

- 占位符两形与裁剪原则与冲突序::[模版使用说明](#howto)
- 可整段复制的约束文件骨架三节::[可复制骨架](#skeleton)
- 本地可信档与自动立会与进阶纪律::[裁剪指引](#tailoring)

## 模版使用说明 {#howto}

- 占位符两形。`{{PROJECT_ROOT}}` 即被治理项目根，域自举后即域根；`{{ENGINE_WORKSPACE}}` 即引擎工作区根，其下应有 sih-engine 仓
- 裁剪原则。最小可配形照裁即用，三节即 MCP 注册、sih 强制触发协议、纪律与会话启动；节内条目可按项目实际收窄，链上留痕与退出码两条不得删
- 冲突序。引擎报文与链在先，本模版次之，代理即兴解释最后

## 可复制骨架 {#skeleton}

以下自「骨架开始」至「骨架结束」整段复制，骨架首行即一级标题，照抄此形：`# {{PROJECT_ROOT}} AGENTS.md`

<!-- ===== 骨架开始 ===== -->

本项目的代理约束入口。sih 治理面已接入本域，一切治理动作走 sih 工具面与引擎 bin 位。

## sih 治理面注册 {#mcp}

客户端 MCP 配置里 mcp.servers 下的 sih 条目，缺省 stdio 单域形，可复制单行形：

`{"sih":{"type":"stdio","command":"{{ENGINE_WORKSPACE}}/sih-engine/target/debug/sihmcp","env":{"SIH_ROOT":"{{PROJECT_ROOT}}","SIH_MCPLINE_CODE_ROOT":"{{ENGINE_WORKSPACE}}"},"enabled":true,"timeoutMs":60000}}`

- 配置于新会话读取，既有会话不重载，重开新会话方见 sih 工具面
- 在役验证，新会话中 locks_read 心跳可调即配置在役
- 读面九具，即 chain_query、chain_verify、critsweep、heartbeat、locks_read、retriever_recall、naming_guide、nomenclator_query、nomenclator_check；写面缺省十具，本地可信档 SIH_MCPLINE_AGENT_CLASS 置 local 时写面全集十二具
- 冷启动体检三具，即 locks_read 与 critsweep 与 heartbeat

## sih 强制触发协议 {#trigger}

- 用户输入以 sih 开头即强制治理触发器。你是单一入口，自主解析意图、拆解动作、编排 MCP 工具链并立即执行
- 惯例缩写词义不走即兴解释。先调 askroute 机械判定，命令形 {{ENGINE_WORKSPACE}}/sih-engine/target/debug/askroute route "<缩写>"，缺省内嵌 intents-v0 判定包，可用 --pack 换包；命中按返回链执行，未命中走三步兜底，即查册 retriever_recall 与 naming_guide 与 nomenclator_query、语料召回、问人
- 被拒是教学。读错误载荷 reason_code 与 suggested_action 改做法，勿原样重试
- 治理事实以链为准，记忆与转述不作数，读数用工具

## 纪律与会话启动 {#discipline}

链上留痕。sih/event/trail 每日一链文件，只经书简写位追加，手改链文件即验链红。认证之后的任何写使认证作废。

写面五步链。lease_open 加 lease_lock 加 lease_commit 加 lease_unlock 加 lease_close，意图笔与认证笔先于收约；一连接恰好一会话，未立会话一切写拒；写面绕行旗标不透传。

产出管线。治理文档类产出先经化格加核阅加检词三步，序固定，全走引擎 bin 位。退出码三值即法律，零合规、一违规、二工具自身异常。

零直写。施工在隔离副本收约归并，主根不直接改。不确定的事实如实申报，禁伪造。

会话启动五件，每会话开场按序。一 产出前自检，判断与方案与决策类产出从已立约束出发被支配生成。二 回锚，可选件，引擎工作区有锚面惯例时裸跑 {{ENGINE_WORKSPACE}}/sih-engine/target/debug/attnanchor 得五行锚，无则跳过。三 冷启动体检三具，locks_read 与 critsweep 与 heartbeat。四 判据扫，critsweep 读当日现态，沉底项转呈人节点。五 链验，chain_verify 读当日链判词 valid。

<!-- ===== 骨架结束 ===== -->

## 裁剪指引 {#tailoring}

- 本地单人可信档，客户端配置 env 加 SIH_MCPLINE_AGENT_CLASS 值 local，写面补 record_direct 与 lease_unclaim 两具
- lease_open 的 repo 参数传你自己的仓相对路径，缺省 sih-engine 是引擎自用形
- 连接级自动立会话形，客户端配置 env 加 SIH_MCPLINE_PACKAGE 与 SIH_MCPLINE_INTENT 两键，批名与意图件齐备即连接建立时自动 lease_open；缺省未配走显式开
- 判据登记册与例行读数与泊界心跳三件属进阶纪律，候域主按引擎仓现行文档自布
