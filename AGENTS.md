# sih-engine AGENTS.md

本文件是 sih-engine 工程仓的入口。代理进入 sih-engine 后首先读取本文件。本文件承载工程仓的身份定位、组件索引、术语映射。

本文件不重复根 AGENTS.md 的内容。根 AGENTS.md 承载工作区整体的项目身份、哲学仓地位、工程基线与禁止条款。本文件只承载 sih-engine 工程仓内部的入口信息。

## 概览 {#overview}

- 本仓身份：司衡引擎 sih-engine，司衡哲学在工程层面的本轮实现::[身份定位](#identity)
- 第一阶段组件索引指向各组件协议文件::[组件索引](#component-index)
- 术语映射承载哲学命题到工程组件的对应关系::[术语映射](#term-mapping)
- 文档索引指向本仓各类型文档::[文档索引](#doc-index)

## 身份定位 {#identity}

司衡引擎即 sih-engine 是工程产物，本轮基础设施。哲学身份是司衡即 SiHankor，治理实体，承接哲学仓全部命题与铁律。哲学权威归属于 sih-philosophy/，工程层所有操作受其约束。

旧工程仓 sihankor/ 的实现已被废弃。失败设计不继承，失败经验已提炼为禁止条款，详见根 AGENTS.md 工程基线与禁止条款，以及本仓 GOV-001 失败推导的禁止条款与设计约束。

本仓当前处于手动阶段，协助人类决策者模拟司衡引擎的治理动作。确定性程序是治理操作的唯一执行者，LLM 只生成符号材料。

## 组件索引 {#component-index}

第一阶段最小闭合治理环含四个组件。组件协议详见 DES-003 第一阶段组件清单与协议。

意图锚定组件
: 承载人类意图的结构化声明，是参验的唯一基准。接收人类意图，结构化为意图声明，写入知识包。不生成意图，LLM 不可代替人类声明意图。协议定位 DES-003 § 意图锚定组件。

符号材料生成
: LLM 的唯一职能。LLM 消费知识包与意图声明，产出代码或文档或方案作为符号材料。不直接写入治理对象，产出仅作为待校验材料。协议定位 DES-003 § 符号材料生成。

参验
: 以意图声明为唯一基准，校验符号材料是否对齐意图，一次穿透同时完成验证与度量。不投射偏好，不创建新意图，只反映事实。协议定位 DES-003 § 参验。

事件流
: 不可篡改的留痕载体，承接所有治理操作的记录。追加写入治理操作事件，提供检索与回放。不改写历史行，不删除已写入事件。协议定位 DES-003 § 事件流。

第二阶段组件预登记。视图组件负责信号聚合与异常告警，承接 GOV-001 FM-07 与 FM-10 设计约束。微积分引擎负责趋势指标产出。这两个组件在第二阶段引入，第一阶段不启用。

## 术语映射 {#term-mapping}

术语映射承载哲学命题到工程组件的对应关系，标注来源类型。来源类型区分流衍命题与 convergence 层对照命题。

意图锚定
: 工程组件：意图锚定组件。哲学命题：PRO-03 道二意图先于代码，流衍命题。convergence 层对照：P1.3 意图保真路径、R8 意图锚定原则，定位 KNOW-001。

映照
: 工程组件：参验。哲学命题：PRO-07 鉴层映照义，用心若镜反映而不投射，流衍命题。convergence 层对照：P1.2 偏差隐蔽性、差异分析冲突 1 一次穿透要求，定位 KNOW-001。

留痕
: 工程组件：事件流。哲学命题：PRO-08 应而不藏，留痕是应鉴循环的构成性条件，流衍命题。convergence 层对照：P4.2 责任归属界定、P4.3 决策留痕审计，定位 KNOW-001。

确定性程序执行
: 工程组件：事件流写入器、参验校验器。哲学相邻命题：PRO-07 鉴，检验由可重复程序承载。convergence 层对照：D003 提议权对主对话关闭、D004 治理变更的治理，定位 KNOW-001。来源类型：工程基线。

人类只看异常
: 工程组件：视图组件即第二阶段。convergence 层对照：P3.1.3 软退化先兆信号、R7 交付度量原则，定位 KNOW-001。来源类型：工程基线。

符号材料
: 工程组件：符号材料生成。哲学命题：PRO-03 道二意图先于代码，因果方向不可逆。convergence 层对照：P1.1 翻译链损失、P1.3 意图保真路径，定位 KNOW-001。来源类型：工程基线。

事件流
: 工程组件：事件流。convergence 层对照：P3.2 外化管理三性质即持久性与版本化与可审计性，定位 KNOW-001。来源类型：工程实证经验，待哲学锚定。

## 文档索引 {#doc-index}

本仓文档按 DEC-001 七大类型组织。第一阶段已建四个类型。

决策文档
: DEC-000 文档格式原则。DEC-001 仓库结构。定位 doc/decision/。

设计文档
: DES-001 文档格式设计集群。DES-002 哲学仓消费路径设计。DES-003 第一阶段组件清单与协议。DES-004 仓库结构第一阶段扩展。定位 doc/design/。

规约文档
: GOV-001 失败推导的禁止条款与设计约束。定位 doc/governance/。

知识库文档
: KNOW-001 convergence 层检索路径。定位 doc/knowledge/。

过程性产出
: T-001 启动任务包、old-repo-failure-analysis 旧仓失败复盘。定位 tmp/，不进治理链条。

## 内部工具 bin 索引 {#bin-index}

src/bin/ 下 36 个 bin 逐件登记，三列即 bin 名、行数、职能，职能取各文件头注 //! 首行语义压缩。lease 含 src/bin/lease/ 七子模块。

- acceptor：596 行，引擎侧空腹判定包执行机命令行面，lease-mergeleg6-parallel 簇J 移植件
- ask3repeater：81 行，三问确定性外壳的校验腿，承接 SPEC-005#deterministic-shell
- askroute：147 行，判定路由命令行面，gap-askroute-port 引擎位移植件
- attnanchor：483 行，回锚引擎件，五行锚读数组装器，注入式回锚 v1 的引擎 bin 形
- attractor：337 行，得一机械核对腿的六子命令入口
- basemgr：820 行，引擎侧基线向量管理工具命令行面，lease-mergeleg6-parallel 簇I 移植件
- calllogtool：500 行，引擎侧行式账本命令行面，lease-mergeleg23-parallel 簇F 移植件
- cascade：377 行，引擎侧级联命令行面，lease-mergeleg23-parallel 簇E 移植件
- checker：831 行，引擎侧检查器命令行面，lease-mergeleg6-parallel 簇I 移植件
- confledger：796 行，引擎侧冲突账本命令行面，lease-mergeleg6-parallel 簇I 移植件
- critsweep：1102 行，判据扫引擎件，对话框内治理态回算器，v1.2.0 的引擎 bin 形
- elicit：578 行，引擎侧叩问命令行面，lease-mergeleg6-parallel 簇J 移植件
- formatter：627 行，引擎侧化格命令行面，lease-mergeall-parallel 簇B 移植件
- gauge：839 行，秤星引擎件，治理态读数计算核，ga-2 的引擎 bin 形
- identity：962 行，引擎侧正身命令行面，lease-mergeleg23-parallel 簇E 移植件
- incubation：1186 行，孵化回路契约校验器引擎 bin，lease-mergeleg6-parallel 簇H 移植件
- latextool：1495 行，引擎侧 LaTeX 书写辅助命令行面，lease-mergeleg6-parallel 簇G 移植件
- lease：729 行，租约引擎件，锁核腿 fixture 对等实装，SPEC-024 腿一；含 src/bin/lease/ 七子模块：attachments、calllogface、closegate、commitlaw、guardlaw、sddgate、sweepcore
- locator：2872 行，引擎侧寻址命令行面，lease-mergeleg23-parallel 簇D 移植件
- locksview：746 行，引擎侧锁视图工具命令行面，lease-mergeleg6-parallel 簇I 移植件
- meter：696 行，引擎侧计量命令行面，lease-mergeleg23-parallel 簇F 移植件
- nomenclator：930 行，引擎侧检词命令行面，lease-mergeall-parallel 簇B 移植件
- parser：3012 行，引擎侧句读命令行面，lease-mergeleg23-parallel 簇D 移植件
- pendline：74 行，候裁处置范式编排件的五子命令入口，pl-04
- projsnap：786 行，引擎侧派生快照投影器命令行面，lease-mergeleg6-parallel 簇J 移植件
- registrydemo：125 行，腿五插件槽位演示 bin，SPEC-025 插件槽位协议，lease-mergeall-parallel 簇C
- retriever：168 行，温故宿主命令面，承接 SPEC-007#interface-signature 与 SPEC-008#boundary 与修订六
- scribe：750 行，书简融回命令行面，本名回滚承 DEC-017 修订二，承接 SPEC-006#boundary 与 T6
- scrutinator：232 行，引擎侧核阅组件命令行面，承接 DEC-007#decision-component 与 SPEC-013
- selector：1097 行，引擎侧路择命令行面，lease-mergeleg23-parallel 簇E 移植件
- sih：288 行，sih 命令薄壳，sih init 开域单步窄口，adoptface 批
- sihmcp：151 行，sihmcp 二进制入口，MCP 线 Rust 载体，sihmcp-solo 批
- tally：1366 行，引擎侧执契命令行面，lease-mergeleg23-parallel 簇F 移植件
- viewer：221 行，视图组件命令行面，承接 DEC-007#decision-component 聚合输出组件
- watchcheck：670 行，引擎侧稽命令行面，lease-mergeleg23-parallel 簇F 移植件
- wikirecall：703 行，三通道确定性召回引擎 bin，lease-mergeleg6-parallel 簇H 移植件

## lib 模块清单 {#lib-modules}

src/lib.rs 现役 12 个 pub mod 逐件登记。

- ask3repeater：三问组件本体，承接 DEC-006 七项决策、DES-013 组件设计、SPEC-005 接口规格
- askroute：判定路由组件库，gap-askroute-port 引擎位移植件；此前未列入，O2 收口
- attractor：得一，判定器席实例的机械核对腿，融回自 sih-tools facet 与 tally
- cascade_registry：级联建册核心库件，pk-055 cascadeclose-solo 批自 src/bin/cascade.rs 原位提取；此前未列入，O2 收口
- event_stream：事件流模块入口，承接 DES-007#module-organization 与 SPEC-004#interface-signature
- exitenvelope：引擎共享退出封套基座，CLI bin 侧错误报文发射与终退出的唯一实现位；此前未列入，O2 收口
- mcpserver：MCP 线 Rust 载体，rmcp SDK，sihmcp-solo 批
- retriever：项目记忆组件库面，承接 SPEC-007 冻结契约与 SPEC-008 落差规格
- snapline：墨斗，引擎共享谓词求值基座，纯求值原语零治理语义；此前未列入，O2 收口
- scrutinator：引擎侧核阅组件库，承接 SPEC-013
- tools_registry：腿五插件槽位架构骨架，SPEC-025 插件槽位协议；此前未列入，O2 收口
- view：视图组件纯函数簇，承接 DEC-007#decision-component 聚合输出组件边界

## 关联 {#relation}

- 根 AGENTS.md：工作区整体入口，承载项目身份、哲学仓地位、工程基线与禁止条款
- 哲学仓：sih-philosophy/，唯一权威指导源
- 哲学仓检索：sih-philosophy/llm-friendly-build/，流衍命题查 mapping.md
- convergence 检索：KNOW-001，convergence 层工程命题查本仓知识库
- 静态审计：sihankor/tools/doclint/target/release/sih-doclint，暂借旧仓二进制

## 自检 {#self-check}

### 形式合规自检 {#formal-self-check}

- 一级标题无锚点，仅一个
- 二级及以上标题均带锚点
- 首个二级标题命名为概览
- 无破折号、无装饰符号、无 Unicode Emoji
- 全角括号仅用于单一治理编号
- RANTS.md 风格豁免（DEC-033）

### 内容自检 {#content-self-check}

- 身份定位不重复根 AGENTS.md，只承载工程仓内部入口
- 组件索引四个第一阶段组件均有最小语义定义与协议定位
- 第二阶段组件预登记，标注第一阶段不启用
- 术语映射标注来源类型，区分流衍命题与 convergence 层对照命题
- 文档索引按 DEC-001 七大类型组织，过程性产单列

### 自反性结论 {#reflexive-conclusion}

本文件是 sih-engine 工程仓入口。本文件不重复根 AGENTS.md，只承载工程仓内部的组件索引与术语映射。本文件本身经过自我审视，未发现违反 DES-001 格式规范的形态。
