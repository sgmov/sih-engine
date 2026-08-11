# sih-engine 开放问题清单

本文件是工作记忆,非治理文档。记录当前会话中已推导但待决策或待执行的治理问题。每项一个问题,一个已推导结论,一个下一步动作。逐个消化,消化完成后该项标记 CLOSED 并迁移到对应的 DEC/DES。若本清单自身需要正式化,走 DEC 流程登记。

## 待办项 {#backlog}

### OQ-01 trail 断裂修复 [CLOSED]

- 问题。两阶段任务包执行完成缺 trail 留痕,08-06 末尾只有起草行,且该行带 mismatch 标记。延伸发现:08-03 全部 7 行与 08-06 前 3 行的哈希字段全空,哈希链从未建立
- 已推导结论。引擎事件钩子不存在是根因,手动阶段所有留痕是降级复现。更深根因:写 trail 的组件没有走立名→提案→决策路线,被事件流名整体包办,导致手动构造 trail 行时无独立契约可对照
- 解决路径。PRO-002 界定关系 → DEC-004 立名书简/Scribe + 二层划分 → DEC-002 修正参验英文对 crosscheck → DES-003/SPEC-004 拆分两层 → 确定性程序 rebuild_hash_chain 重建 08-03 与 08-06 哈希链
- 最终状态。08-03(7行)与 08-06(4行)哈希链已重建并经 verify_chain 校验完整。全量 29 测试通过。治理链条闭环:立名→提案→决策→设计→规格→代码→重建
- 哲学依据。PRO-08 应而不藏第 114 行留痕是构成性条件,PRO-09 元层第 84 行治理框架变更须自带防御机制

### OQ-02 state 层新子节点命名 [CLOSED]

- 问题。sih/state/task-package 是 DEC-001 state 层未登记的新节点,需命名后写入 DEC-001
- 已推导结论。task 承诺单数与实体运作复数错位,task-package 的 package 语义不准。plan 承载待执行意图的当前编排投影,与 state 层当前投影语义一致,与 registry/graph/view 命名模式同构
- 解决。目录 task-package 迁移为 plan,DEC-001 节点树与状态层小节已补入 plan 节点声明,sih-doclint 通过
- 哲学依据。PRO-01-name-ontology 名字是本体,PRO-01-naming-failure 名字承诺与实体运作不可错位

### OQ-03 数据飞轮存储位置 [CLOSED]

- 问题。sih/event/experiment/ 的 Phase 2 实验数据位置错误,event 层四个子节点 trail/feedback/challenge/report 都不匹配
- 已推导结论修正。此前判定"非治理产物,不应进 sih/"经范畴重判推翻。实验数据的本体是参验机制的标定基准,是治理产出的重大结果。九视角核验后,校准基准与演化基线两个视角最贴切实验意图。实验设计受微积分与概率论与数理统计启发,用 N 次隔离采样标定语义判断的收敛基准
- 解决。experiment 重命名为 calibration,从 sih/event/ 迁移到 sih/state/calibration/。calibration 作为 state 层新子节点,语义是参验机制的标定基准集。DEC-001 节点树与状态层小节已补入,sih-doclint 通过
- 哲学依据。PRO-05 道四间隙量化,实验标定的基准值是间隙量化的度量地基

### OQ-04 record_trail 是否独立组件 [CLOSED]

- 问题。留痕机制在 DES-003 被描述为事件流的隐含功能,但实际有独立执行入口 record_trail,没走立名路线
- 解决。record_trail 是旧仓遗产,新仓从未实现。DEC-004 已立名留痕操作组件为书简/Scribe,MCP Tool 承接组件名 Scribe。Tool 不是独立组件,是书简组件的执行入口。具体 Tool 拆分即单 Tool 或多 Tool 归 DES 层 MCP 协议设计,不影响命名
- 哲学依据。PRO-01 第 14 行有实体无名是立名失败,PRO-08 留痕是构成性条件

### OQ-05 git commit 治理边界

- 问题。你要求 commit 分梯度 + message 承载任务包,这把 git 操作的资格控制纳入治理范畴
- 已推导结论。锁机制是工程基础设施(SETSP 已定义),commit 资格控制是治理领域(PRO-09-acknowledge)。SETSP 已有 pre-commit hook 检查 trailer 设计,但有外加锁 vs 内在自限的张力待处理
- 下一步。是否现在立项 DEC,还是等 worktree/lock 组件实现时一并做。SETSP 已有完整设计可承接
- 阻塞。无,是未来组件的设计输入
- 哲学依据。PRO-09-acknowledge 资格准入,PRO-09-infuse 约束内在化否定外加锁

### OQ-06 knowledge 与 view 边界定义

- 问题。sih-engine 的 knowledge 区和 view 子节点当前都是空的,边界未定义
- 已推导结论。knowledge 回答怎么用(开发者操作经验),view 回答现在是什么状态(从事件流合成)。两者不重叠
- 下一步。不单独做。在第一个 knowledge 文档或第一个 view 产物产生时,顺带在对应 DES 里定义边界
- 阻塞。无,当前无内容要承载
- 哲学依据。PRO-03 道二方向性,PRO-04 道三意图恢复

### OQ-07 DEC-001 治理语义节点命名补走立名

- 问题。DEC-001 声明的 24 个节点命名无一走过完整 DEC 立名流程。5 个在旧仓 audit-017 跑过立名失败检测(event/state/trail/registry/view)但 audit 非决策文档,其余 19 个连立名失败检测都没跑
- 已推导结论。节点分两类。工程接口节点(src/test/tools)用工程惯例直接命名,不需立名。治理语义节点(doc/ 下 7 个文档类型 + sih/ 下 11 个治理节点)承载治理语义,按 PRO-01 立名本体论应走立名流程
- 待定。立名优先级与批次。治理语义节点有 18 个,逐一走 DEC 立名成本过高,需判定哪些是名实错位风险高的优先处理,哪些可批量声明沿用
- 阻塞。无,当前节点已在运作,立名是补正不是前置阻塞
- 哲学依据。PRO-01 立名本体论,名字是本体参与构成被命名者

### OQ-08 免费模型直连 API 替代借脑机制

- 问题。司衡引擎需要 LLM 做语义判定时，当前只有借脑机制（通过 MCP tool 响应让主 agent 顺带做）。如果有免费 API（GLM-4.7-Flash、MiniMax-M2），司衡可直连做判定，不用借主 agent 的脑。这是否是更优路径
- 已推导结论。借脑机制的最早意图是避免产生账单（成本控制），不是架构必然。免费模型消解了成本根因。直连免费 API 在隔离性（判定独立于生成）、可控性（主动调用不依赖 agent 配合度）、延迟（不等待 agent 顺带做）上优于借脑。借脑的已知局限前三项（配合度不保证、冷启动盲区、协议层免疫假设）都被直连消除
- 合规约束推导（承接 OQ-13）。司衡自持免费 key 直连第三方 API，与代用户调用第三方 key 是同类合规风险，只是 key 归属不同。开源项目不持有用户 key 调用第三方服务，是合规边界。司衡自持 key 调用第三方服务须自行承担 ToS 合规责任，不能通过免责声明转移。免费 key 持续性不保证且 ToS 对第三方代理调用的容忍度不可控。结论：直连免费 API 的跨族隔离路径在开源合规下不可行，OQ-08 的原始定位须重新审视。直连免费 API 若保留，只能用于司衡自建自部署场景，不作开源默认路径
- 待定。第一，与工程基线第五条「治理延伸是减少 LLM 参与」的关系——直连是减少主 LLM 负担还是增加 LLM 调用总量，需判定。第二，免费模型持续性不保证（GLM-4.7-Flash 现在免费不代表永远），需设计降级路径：免费 API 为主路径，借脑为降级路径。第三，需新仓 DEC 确立直连 API 的地位，因为它改变了旧仓「零部署不持 API key」的设计预设，虽然该预设的根因是账单已被免费消解
- 关联。旧仓 PRO-009/DES-017 的借脑机制设计。GOV-001 FM-11 把 T6 集群并发登记为失败模式。新仓尚无 DES-017 设计文档
- 阻塞。无，借脑机制当前未实现（新仓在手动阶段）
- 哲学依据。PRO-07 鉴层多主体独立判定。AGENTS.md 工程基线第五条需判定适用边界

### OQ-09 doclint 迁移至新仓

- 问题。doclint 当前暂借旧仓二进制 sihankor/tools/doclint/target/release/sih-doclint，不在新仓。旧仓 doclint 的 specs 规则集与新仓 DES-001 通用格式规范不同步，例如刚新增的标题内中文不插入空格规则在旧仓 specs 无实现。继续用旧仓二进制等于用规则源不权威的工具校验新仓文档
- 已推导结论。doclint 在新仓组件体系有定位。DES-006 第 13 行明确 sih-doclint 是参验路径一的实例候选。doclint 不是外部工具，是参验组件的确定性程序实现之一。但当前参验组件协议尚未代码实现（SPEC-003 是规格，src/ 只有 event_stream），迁移 doclint 会变成游离工具不承接组件协议
- 待定。迁移时机。参验组件协议确立后 doclint 作为路径一实例迁入，还是先迁移为独立工具后补承接关系。迁移时需同步把旧仓 specs 规则集更新为新仓 DES-001 当前规则
- 关联。DES-006 参验基准预置。SPEC-003 参验规约。GOV-001 FM 分类
- 阻塞。参验组件代码实现（当前 src/ 只有 event_stream）。参验组件实现前须先修订 SPEC-003，路径二 N 等于 4 隔离采样设计前提已失效，见 OQ-13
- 哲学依据。工程基线第一条确定性程序是治理操作的唯一执行者。PRO-07 鉴检验由可重复程序承载

### OQ-10 旧仓 MCP server 卸载

- 问题。旧仓 MCP server 运行在 127.0.0.1:9741，被 .trae/mcp.json 与 .vscode/mcp.json 引用，当前会话的 mcp__sihankor__* 工具全部来自旧仓。旧仓 MCP 操作的 trail、state、knowledge 结构是旧仓的，与新仓 DEC-001 节点树不完全一致，例如旧仓 meta-index/audit/project-state 在新仓不存在
- 已推导结论。旧仓 MCP 的结构与新仓不匹配，继续运行等于让结构不匹配的 server 操作新仓治理留痕。但当前新仓无 MCP server 实现（src/ 只有 event_stream），旧仓 MCP 是 sih 触发协议与留痕校验的唯一执行入口
- 待定。卸载时机。新仓 MCP server 覆盖 record_trail 与 validate_sihmd 的等价职能后卸载。或判定手动阶段不再需要 MCP 执行入口，改用确定性脚本加手动操作替代
- 关联。AGENTS.md sih 触发协议与隐式映射。DEC-004 书简组件。SPEC-004 事件流规约
- 阻塞。新仓 MCP server 实现（当前不存在）
- 哲学依据。工程基线第一条确定性程序是治理操作的唯一执行者。PRO-08 应而不藏留痕是构成性条件

### OQ-12 agent 会话内信息生成行为的治理

- 问题。agent 自身在会话中高速生成推导与 OQ 登记本身就是信息洪流的制造源，与工程基线第二条注意力稀缺与第五条减少 LLM 参与直接冲突
- 阻塞。无
- 哲学依据。工程基线第二条与第五条
- 应用实例。PRO-003 起草采用分阶段产出即 PRO 先登记方向不一次性产出完整 DES，是本 OQ 的第一个内建防护实践

### OQ-13 参验语义校验的判定路径与适用边界

- 问题。SPEC-003 路径二原定义 N 等于 4 隔离采样加多数投票为参验语义校验方法。经多轮推导，多族群隔离采样在司衡开源合规约束下不可行，须重新定位语义校验路径。三处待定。第一，单模型 NPC 专家团带引导态的语义判定有效性是否够用，业界 Cost of Consensus 论文只测了无引导辩论不如隔离自纠正，带引导态专家团是空白，须本仓对照补登。第二，确定性规则路径即 sih-doclint 的覆盖边界在哪里，哪些语义层问题它查不了须专家团补。第三，五法即有度顺因损补知止顺势的哲学源在哲学仓 06-on-canon 已立，新仓工程映射只有有度与知止与顺因三法被 DES-001 局部承接，损补与顺势未承接，五法作为统一判定框架未在新仓定义，旧仓 verify_decision 不作权威
- 已推导结论。多族群隔离采样在司衡开源合规约束下不可行，推导链如下。费用超标是结构性约束，不需实验佐证。业界有效隔离须跨族多模型组合，单 key 单模型是司衡默认部署前提，同模型多次是假隔离不消解偏差。司衡自持多 key 形态把轻量工具变成有运维负担的服务，用户配置意愿不足。司衡代用户调用第三方 key 有封停风险且不可控，不能通过免责声明把合规审查转移给用户，违反工程基线第一条精神与 PRO-09 元层防御要求。借脑模式手工多 agent 切换转嫁人类注意力成本违反基线二与基线三。结论是多族群隔离采样在开源合规下无解，不进路径。可用路径收窄为确定性规则路径全量兜底加单模型 NPC 专家团补语义层。五法工程映射须新仓独立 DEC 确立
- 待定。NPC 专家团带引导态的对照补登实验设计归 SPEC-001。确定性规则路径覆盖边界的判定。五法工程映射归新仓 DEC
- 关联。SPEC-003 路径二语义校验待重新定位。DES-005 预算置信度权衡曲线在多族群路径废弃后定位待重新审视。DES-006 基准清单降级形态。SPEC-001 语义计算实验。PRO-003 待定问题。哲学仓 06-on-canon 五法
- 阻塞。NPC 专家团对照补登归 SPEC-001 实验设计补充。五法工程映射归新仓 DEC 立名。SPEC-003 路径二须按本 OQ 结论修订，N 等于 4 设计前提已失效
- 哲学依据。PRO-07 鉴层破自证循环。工程基线第一条确定性程序是治理操作唯一执行者。工程基线第二条注意力稀缺。PRO-09 元层治理框架须自带防御。06-on-canon 五法

### OQ-11 OQ 机制自身的定位与正式化

- 问题。OQ 机制已在实际工作中被依赖，但当前自述为工作记忆非治理文档，不在 sih-doclint 校验范围，不走 DEC 流程。OQ 的实际形态已超出简单待办清单：与项目级记忆同构，跨 todo 与流程编排，部分 OQ 有阻塞或依赖关系（如 OQ-09 阻塞于参验组件、OQ-10 阻塞于新仓 MCP server、OQ-01 曾阻塞 OQ-04）。旧仓同类机制在失败时被信息洪流淹没，条目堆积不消化变成遗忘清单
- 已推导结论。OQ 承接四层。PRO-08 应而不藏的外化留痕载体职能，防止会话上下文丢失。工程基线第二条注意力预算的结构性保护，散点外化成可扫描列表防止稀释。08-on-settle 应几职能，微兆预判登记等数据成熟度升高后转应辨处理。信息洪流分流消解，会话中现在不用处理但不能丢的内容从主上下文分流到 OQ 暂存，当前是手动阶段降级消解（消解待决策问题暂存这一类），全量消解需司衡引擎全组件落地后的完全态（参验确定性校验加书简自动留痕加视图异常聚合加确定性程序自动处理）。当前形态已包含阻塞依赖关系，与简单 todo 不同构
- 待定。第一，OQ 的正式化触发条件——什么时候从工作记忆升级为治理文档。第二，OQ 间的阻塞依赖关系是否需显式建模（当前在正文里用文字描述，无结构化依赖图）。第三，OQ 与项目级记忆的关系——是否同构为同一机制的不同投影。第四，消化速率的健康指标——旧仓失败于条目堆积不消化，需判据区分活跃清单与遗忘清单
- 关联。旧仓 OPEN-QUESTIONS 失败模式。AGENTS.md 工程基线第二条与第四条。sih-doclint 校验范围边界
- 阻塞。无，当前 OQ 机制在运作
- 哲学依据。PRO-08 应而不藏。工程基线第二条注意力预算。08-on-settle 应几

### OQ-14 确定性度量计算层的定位与命名

- 问题。counter 的收敛算法对比实验中，4 个算法（Chao1、Clench、BOIN、Baseline）采用统一函数签名（list[dict] 到 dict），自然解耦为框架层（序列加载、评估判据、报告生成）与算法层（可插拔算法函数）。进一步审视发现，采样饱和度只是治理域展开后确定性度量计算的第一类需求，鉴层收敛率、参验隔离采样 N 值、注意力 OQ 堆积率、解网拓扑排序、视图异常聚合阈值等计算需求共享同一组特征：都是确定性程序承接工程基线第一条、都从事件流或状态数据派生度量承接第四条、输出都是人类决策输入承接第三条、算法可替换度量需求稳定。本 OQ 登记的问题是：这一类跨治理域的确定性度量计算是否应抽象为 sih-engine 的一个基础组件（而非 sih-tools 的独立工具），以及该组件的命名与边界。
- 已推导结论。确定性度量计算层这一方向的识别来自两次定位修正：最初将计算器框定为采样饱和度度量框架，后修正为确定性度量计算层。两次修正的认识论意义是，算法数量不决定解耦价值，问题域的不确定性决定解耦价值，一个算法在某批数据上被弃用是数据相关结论不是算法能力结论。
- 待定。第一，确定性度量计算层的命名，须承接 PRO-001 立名本体论，已在 counter proposition/GOV/naming-ontology-for-unblocker/ 登记命题待 bps2 审阅，命名方向未定。第二，该层的边界，是否涵盖解网拓扑排序（OQ-11 第二项阻塞依赖建模的工程落地）与鉴层度量，还是仅限采样饱和度与同质度量。第三，该层是 sih-engine 组件还是 sih-tools 独立工具，当前倾向 sih-engine 基础组件因承接跨域确定性计算职能，但需 DEC 流程裁定。第四，算法注册与插拔机制的设计，当前 4 算法统一签名是自然起点，但需验证能否覆盖未来新增算法（Christen-Nakamura、ACE、DivE 等）。第五，计算框架与算法层的解耦时机，当前对比实验正在跑，解耦会改 convergence_calculator.py，M3 正在读此文件，存在执行冲突。
- 关联。OQ-11 OQ 机制正式化第二项（OQ 间阻塞依赖关系显式建模，解网是同类计算需求的工程落地）。counter ROADMAP P1 确定性裁决与 P3 反哺 sih-engine。counter audit/convergence-algorithm-survey.md 15 候选方法调研。counter proposition/GOV/naming-ontology-for-unblocker/ 命名命题。counter proposition/GOV/topology-for-dependency-graph/ 拓扑学命题
- 阻塞。OQ-11 OQ 机制正式化（组件的治理地位须先于组件设计）。counter bps2 命名审阅结果（命名方向须先于组件立名）。counter P0-exp 对比实验结果（算法保留集须先于解耦工程化）
- 哲学依据。PRO-001 立名本体论（名字参与构成实体，命名先于实现）。工程基线第一条确定性程序是治理操作唯一执行者。工程基线第三条人类注意力只投向异常信号。工程基线第四条可验证性约束

### OQ-15 facet runner 硬编码 atom_name 架构债务 [CLOSED]

- 问题。facet P1 第三批验收通过的 runner.py 第 174 行硬编码了 `atoms.get("facetor_independent", {})`，第 186 行硬编码 `atom_name="facetor_independent"`。PRO-001 核心命题是"加范式不改代码"，runner 读 atom-chain.yaml 编排配置机械执行，不应知道具体 atom 名。当前实现只对 single_round_explore（P1 唯一范式）有效，加新范式（P2 endogenous_tree / P4 ban_pick）时 runner 要改成对应 atom_name，违反 PRO-001。
- 解决。runner.py 从 atom-chain.yaml 的 composition 段读 atom_name（parallel.atom / sequential.atom），替换两处硬编码。同时修复 validator_params 从 direction 读（原硬编码 {"strict": True}）。验证：hermes-philosophy-relation scheme 重跑通过，runner.py grep facetor_independent 零结果。P2 endogenous_tree 可不改 runner 直接加范式。
- 关联。PRO-001 范式原语定义（加范式不改代码的核心命题）。facet ROADMAP P2 endogenous_tree（硬编码修复的前置触发点）。
- 哲学依据。PRO-001 范式配置与固定程序分离的核心命题。工程基线第一条确定性程序是治理操作唯一执行者。

### OQ-16 Hermes Agent 与 SiHankor 的关系调研后续

- 问题。人类决策者观察到 Hermes Agent（Nous Research）的哲学与司衡哲学可能存在同源关系。初步概览对照发现工程方法学层多处强对应（确定性护栏 / 前置约束 / 持久化不可变性 / 关注点分离），但概览级判断存在过早下结论的风险——四个"SiHankor 独有"命题（道一 / 立名不撤回 / 治理流程 / 治理实体）尚未验证是否是同一命题在不同接入层的不同表达。调研目的有二：厘清接入层次边界（SiHankor 哲学层/治理引擎层 vs Hermes 通用 agent 框架/工具层，不构成竞争但需定边界），借鉴飞轮数据（Hermes 47K stars / 42 天社区提供大量实践数据）。
- 已推导结论。第一，底稿采集任务包已写（ai-ex/HERMES-RESEARCH-BASELINE-COLLECTION.md），三路并行采集（官方文档 / 源码 / 社区数据）。第二，人类决策者调整了后续策略：参照/提炼/借鉴三个渐进式任务不作为独立任务包跑，而是把 Hermes 材料作为 facet csnx 实验的真实 topic 素材，让 facet 多 facetor 审阅 Hermes 材料，人类读 facet 产出后自己做参照/提炼/借鉴判断。理由：避免单独跑调研成为脱离真实用途的虚假数据飞轮，同时验证 facet 的异质性发现能力。
- 待定。第一，csnx 实验的 topic.md 设计——Hermes 的哪些材料作为锚点（架构原则 / 约束执行三层模型 / 记忆冻结快照 / skill 系统），命题怎么定（"SiHankor 与 Hermes 接入层次关系"还是更具体的命题）。第二，参照/提炼/借鉴三个判断的输出载体——是写进 ai-ex 还是由人类决策者直接在 facet 产出基础上做推理。第三，底稿采集任务包是否仍按原计划跑（作为 topic 锚点材料来源），还是直接用已读过的官方架构文档 + 约束执行 issue 作为锚点省略底稿采集。
- 关联。ai-ex/HERMES-RESEARCH-BASELINE-COLLECTION.md（底稿采集任务包，定位已调整为 topic 锚点材料来源）。Hermes 官方架构文档（设计原则 / 可观测执行 / 提示词稳定性 / 松耦合 / 配置隔离）。Hermes GitHub Issue #29652（三层约束执行模型：prompt / tool / architecture 层的可靠性频谱）。Hermes GitHub Issue #476（Mode System 被评最高价值架构变更）。OQ-15 facet runner 硬编码（csnx 实验是否需要修完硬编码才能跑——不需要，single_round_explore 不受影响）。
- 阻塞。无强阻塞。csnx 实验可在 P1 现状直接跑（single_round_explore + Hermes 材料 topic）。底稿采集可并行跑也可省略。
- 哲学依据。无直接哲学命题约束。观察动机是人类决策者对"独立系统得出相同命题"这一现象的验证需求，属于工程实证积累。参照/提炼/借鉴三个判断本身承接 PRO-007 鉴层（多视角交叉审阅打破自证循环）。

### OQ-17 facet severity 合法值定档

- 问题。facet 的 severity 字段当前暂定 critical / major / minor / info 四档（比 counter 多一档 info 信息级）。定档依据不足——没有实际跑出来的 severity 分布数据支撑四档还是三档更合理。过早定档会在 csnx 实验数据积累后发现分档不合适，回头改 compiler + 历史数据的 severity 字段。
- 已推导结论。第一，info 档是 facet 新增的猜测性分档，counter 没有这一档，info 的语义边界（什么程度的发现算 info 不算 minor）未定义。第二，暂不定档不阻塞 P1——compiler 的 compute_severity_distribution 已经支持四档（KNOWN_SEVERITIES 含 info），未声明的 severity 归 (unknown) 桶，改档只改 KNOWN_SEVERITIES 常量。第三，ROADMAP P3 已登记此待定项。
- 待定。第一，csnx 实验跑出实际 severity 分布后定档——看 info 是否有实际产出、info 与 minor 的分布是否可区分。第二，定档后的历史数据处理——已有的 single_round_explore 实验数据如果含 info 标注，定档时是否回溯重分类。第三，info 的语义定义——是"信息性发现不构成问题"还是"低优先级观察"。
- 关联。facet ROADMAP P3 数据与分析（severity 定档已登记）。facet src/compiler.py KNOWN_SEVERITIES 常量（定档改这里）。facet src/engine.py _extract_facets（severity 字段从 LLM 产出解析）。
- 阻塞。csnx 消息压缩采样实验（实验跑出来才有分布数据可看）。无其他阻塞。
- 哲学依据。工程基线第五条治理延伸是减少 LLM 参与（severity 定档是确定性程序的度量定义，不应靠 LLM 自行判断 severity 归属，应由人类基于分布数据定档后机械应用）。

## 已发现的治理债务 {#debts}

以下是与上述问题关联但尚未处理的工程层缺陷,记录备查。

### OD-01 sih-engine 零 commit

- 现象。git log 返回 your current branch master does not have any commits yet,所有产出 untracked
- 判定修正。此前判定违反工程基线第四条已撤回。git 是工程基础设施层,0 commits 是工程易失性风险,不是治理基线违规
- 关联。OQ-05 commit 治理边界确定后,首次 commit 走相应流程

### OD-02 experiment 目录归属错误 [CLOSED]

- 现象。sih/event/experiment/ 含 Phase 2 实验数据,event 层四个子节点都不匹配
- 解决。experiment 重命名为 calibration,迁移到 sih/state/calibration/。承接 OQ-03

## 执行顺序建议 {#order}

DEC-004 已完成 trail 治理链条的立名环节(书简/Scribe 命名 + 载体与组件二层划分 + MCP Tool 承接)。DEC-002 子决策三同步完成参验英文对修正(crosscheck)。下一步路径:修订 DES-003 与 SPEC-004 拆分两层,事件流小节拆为 trail 载体描述与书简组件协议,最后重建 08-03 与 08-06 哈希链。OQ-02 阻塞 DEC-001 修复且只需你拍板命名。OQ-03 阻塞 experiment 迁移且只需你选方案。OQ-05 与 OQ-06 是较大的设计决策,可延后。

当前建议:OQ-01 trail 治理链条重建 CLOSED。OQ-02 plan 命名 CLOSED。OQ-03 calibration 命名 CLOSED。OQ-04 record_trail CLOSED(书简/Scribe 已立名,record_trail 是旧仓遗产不存在)。剩余活跃项:OQ-05 git commit 治理边界(未来组件设计输入)、OQ-06 knowledge/view 边界(等第一个产物触发)、OQ-07 DEC-001 治理语义节点补走立名(18 个节点的优先级与批次待定)、OQ-08 免费模型直连 API 替代借脑(待 DEC)、OQ-09 doclint 迁移(阻塞于参验组件实现)、OQ-10 旧仓 MCP 卸载(阻塞于新仓 MCP server 实现)、OQ-11 OQ 机制自身定位与正式化(当前在运作,正式化触发条件待定)。
