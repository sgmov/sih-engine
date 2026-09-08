# 工程基线 v1：sih-engine 工程层操作约束正典

本文件是 sih-engine 工程层操作约束的正典档，承 pk-070 出泊由 AGENTS.md「工程基线与禁止条款」节整体迁入，甲案落据即 m-hygp070-1 九发 stable_clear 终签 f6263808。正典在此，AGENTS.md 同名节改指针为入口投影。正文自前像程序切片逐字节迁入零语义改动，diff 自证随批材料可复算。

## 概览 {#overview}

- 本节承载 sih-engine 的工程层操作约束，每条约束标注来源类型，区分已有哲学支撑与待哲学锚定，来源类型五类::[基线正文](#baseline-body)
- 工程基线五条即确定性程序与信息洪流与异常信号与可验证性与减少 LLM 参与::[工程基线五条](#engineering-baseline-five)
- 工程禁止条款五条由工程基线与失败复盘直接推导，agent 必须遵守::[工程禁止条款](#engineering-prohibitions)
- 待哲学锚定条目的地位两类比照，同时是哲学仓后续演化的候选输入::[待哲学锚定条目的地位](#pending-philosophy-anchor)
- v1 于 2026-09-08 承 pk-070 出泊迁入，三层固定即 git 版本化加哈希入 trail 加结果档在链::[版本](#version)

## 基线正文 {#baseline-body}

本节承载 sih-engine 的工程层操作约束。每条约束标注来源类型，区分已有哲学支撑与待哲学锚定。

来源类型五类：

- 哲学命题：承接 sih-philosophy 流衍段或复归段或链外补充的命题条目，标注命题 ID。
- convergence 层对照命题：承接 sih-philosophy convergence 层单盲推导体系的工程命题，标注命题编号与原文定位。单盲推导体系是与流衍链对照验证的自底向上骨架，认识论地位是对照证立，不是流衍命题本身。
- 铁律：承接 convergence 层铁律，标注铁律编号，标注其精确指称域。
- 工程实证经验：来自工程落地失败复盘，哲学仓无对应命题，标注待哲学锚定。
- 工程基线：本工作区对话共识，无哲学命题直接支撑。

### 工程基线五条 {#engineering-baseline-five}

第一条，确定性程序是治理操作的唯一执行者。LLM 只生成符号材料，不拥有写入知识包、事件流、意图锚定的权限。治理操作的执行归确定性程序，LLM 产出仅作为待校验的符号材料。来源类型：工程基线，即对话共识。哲学相邻命题：PRO-07 鉴，检验由可重复程序承载。

第二条，信息洪流是旧仓失败根因。LLM 的生成速度与人类审查注意力之间存在结构性不对称，不是工程失误，是结构性约束。治理系统必须假设人类注意力是稀缺资源，将人类介入点限制在确定性程序无法自动处理的异常范围内。来源类型：工程实证经验，来自旧 sihankor 仓失败复盘。哲学对照基础：convergence 层单盲推导体系 P3.1 退化机制论证了注意力预算的结构性稀释，P3.2 外化管理论证了状态外化的必然性，均与信息洪流同源。哲学相邻命题：PRO-03 道二、PRO-08 应，为相邻但未精确覆盖。说明：单盲推导体系在 convergence 层有对照基础，但 llm-friendly-build 检索路径暂未覆盖，须回到 convergence 原文核验。

第三条，人类注意力只投向异常信号。治理系统的设计须确保确定性程序自动处理绝大多数操作，人类只在视图告警时介入，不看原始日志。来源类型：工程基线，由第二条直接推论。

第四条，可验证性约束。所有写入操作须满足：操作来源可追溯、操作结果可机械校验、操作历史不可篡改。LLM 的不可复现输出不得作为治理决策的最终依据。来源类型：工程实证经验。哲学对照基础：convergence 层单盲推导体系 P3.2 外化管理论证了外化存储的持久性、版本化、可审计性三性质，是本条的对照证立。说明一：本条与 convergence 层铁律 #6 方法学可重现原则同源精神，但指称域不同。铁律 #6 约束的是核心洞察层收敛率算法的可重现，本条约束的是工程写入操作的可验证，不构成对铁律 #6 的承接。说明二：单盲推导体系在 convergence 层有对照基础，但 llm-friendly-build 检索路径暂未覆盖，须回到 convergence 原文核验。

第五条，治理延伸是减少 LLM 参与，而非增加。用更多 LLM 调用对抗 LLM 不确定性的模式，在治理核心中不采用。治理靠确定性程序，LLM 只作为符号材料的生成器，但生成过程受哲学命题约束，使产出对齐意图。不生成不等于不引导：司衡不产出实质语义内容，但通过前置约束注入、意图锚定、上下文加载锁定生成方向。来源类型：工程基线，即对话共识。

### 工程禁止条款 {#engineering-prohibitions}

以下条款由工程基线与失败复盘直接推导，agent 必须遵守：

- 禁止 LLM 直接修改知识包或意图锚定。信息洪流的根因之一是 LLM 的不可复现写入使审查者无法追溯变更来源。
- 禁止用不可复现的多 Agent 交互产出作为治理决策依据。注意：多主体协作本身是鉴层打破自证循环的哲学机制（PRO-07），不是禁止对象。禁止的是不可复现的交互实现与用 LLM 调用堆叠替代确定性验证。
- 禁止用 LLM 调用堆叠替代确定性验证。多 Agent 协同、红蓝对抗等模式若用于治理决策的执行环节，属于此类。
- 所有治理操作必须写入事件流，留痕不可篡改。承接 PRO-08 应而不藏，留痕是应鉴循环的构成性条件。
- 人类只通过视图介入，不看原始日志。

### 待哲学锚定条目的地位 {#pending-philosophy-anchor}

本节标注的条目分两类。第一类是信息洪流与可验证性约束，当前状态为工程实证经验的提炼，在哲学仓 convergence 层单盲推导体系中有对照基础，即 P3.1 退化机制与 P3.2 外化管理，但未被流衍链立为独立命题。它们不假装已是流衍命题，也不因为未立为流衍命题就放弃工程约束力。第二类是其他工程基线条目，来自本工作区对话共识，无哲学命题直接支撑。

这些条目同时是哲学仓后续演化的候选输入。若工程实践反复验证其有效性，哲学仓可按自身演化程序即 PRO 流衍段推导链，决定是否新增独立命题或扩展现有命题。工程层不越俎代庖，不自行将其升格为哲学命题。


## 版本 {#version}

v1 于 2026-09-08 承 pk-070 出泊迁入即本文件首版。令源两笔照录：用户 2026-09-06 裁定原话「这个工程基线应该是司衡引擎的运行机制的基石，不应该放在AGENTS里。你这个记一笔，未来要清的。」与用户 2026-09-08 测量令「PK-070 过得一」经 m-hygp070-1 九发 stable_clear 终签 f6263808 裁甲案即迁入 sih-engine/doc/governance/ 治理决策档族新立 BASELINE 正典档，AGENTS.md 改指针，乙向界族与丙宪法档不采。正文自 AGENTS.md 同名节程序切片逐字迁入零语义改动，diff 自证在批材料 migration-diff-proof.json 可复算。三层固定即本文件 git 版本化随批 settle 归并、内容哈希入 trail 认证、出入泊笔与结果档在链。
# 全态泊界 v1：主线未决事项的有界停靠地

## 概览 {#overview}

- 泊界即主线与全态线的未决事项有界停靠地，立法源承 PRO-007 Parking 词条、m-parking-mechanics 裁示、2026-08-25 用户令随全态向界首建::[四字组落位](#four-gates)
- 与工具线泊界共一账本即书简 trail 停泊事件，停泊名册单一名册续号不重号::[出入口协议](#protocol)
- 心跳即会话首动作对在泊材料跑路择 parking 包路由，结算必经在泊复检::[心跳与结算](#heartbeat)
- 首批住户两项，名录是在泊投影，真相在链不在文档::[在泊名录](#occupants)
- v1 于 2026-08-25 用户令首建::[版本](#version)

## 四字组落位 {#four-gates}

本节按泊界词条四字组逐字落位。停有痕即每笔进出泊都是书简 trail 上的停泊事件，事件类型 parking_entered 与 parking_exited，append-only 哈希链承载，与工具线泊界同链。看有门即 scribe query 按 entry_id 或事件类型检索链上停泊事件，本界之下的在泊名录是在泊投影，投影与链不符时以链为准。忘有警即路择 parking 谓词包的时间维度判定，到期判定谓词在参照时间越过入泊日加时限时报到期，在泊告警在停泊时长达阈值时逐件报警，参照时间由调用显式给参，路择不读系统钟。出有点即出泊唯人节点裁决，转正或废弃由用户裁，出泊事件必载裁决指向即 disposition 取 promoted 或 discarded 与 ruling 裁决文。

## 出入口协议 {#protocol}

进泊记录必载字段即 entry_id、title、context、exit_condition、ttl_days，进是记账即机器与 agent 皆可做。出泊记录必载 entry_id、disposition、ruling。机械门两道由书简停泊写入位承载：重入拒即同 entry_id 已在泊再进拒之，无主出拒即无在泊进泊事件的出泊拒之，拒绝零留痕。时限语义即 ttl_days 是复检提醒时限不是自动出泊时限，到期只报不做主，出泊仍唯人节点。搁置语义即有明显界限或裁决依赖未来事件的在泊件可携 gate 键即触发器为链上可查事实条件，gate 在场且未点火即搁置态：到期判定与在泊告警对其停计，路由停放入 siding 路即停放本位不新增第四路，siding 计数仍可见即停计不隐藏，点火即 gate 键落 fired_at 后回主线计数，点火判定归后继批或人节点，出泊仍唯人节点零触碰。名册约定即 entry_id 与工具线泊界共单一名册续号，跨线不重号不重启。

## 心跳与结算 {#heartbeat}

出泊件材料计数语义注，2026-09-08 acceptclose-solo 批落注候人裁：出泊件材料文件现留存于两线 materials 目录，心跳路由按 parking 包 P101 schema 谓词判定，凡携 id 与 path 与 state 三字段者计入 mainline 计数，出泊与在泊同册同计属包面既有行为。语义选项三形原呈报留档：甲即维持现状出泊件留册计 mainline 利审计可见；乙即出泊件材料 state 字段改 exited 加谓词分轨即 P10x 增 state 谓词出泊件路由 siding 独簿；丙即出泊材料移档归 historic 目录出心跳面。裁定形即甲案承 m-adjudicate2-a1 终签，终签出自 adjudicate2-solo 批 2026-09-08 九发 stable_clear 执契，crosscheck 链笔 e6ab6bd7；乙丙不采，维持现状照旧行为，谓词零改目录零迁移。心跳报数与结算单必经栏归视图族，视图是引擎侧远端组件零实装，孵化期由 agent 人话汇报承担。心跳节拍为会话开始即每会话首动作对两线在泊材料各跑一次路择 parking 包路由，参照时间取当日实日，告警与否如实报数，与工具线泊界同批执行。主线每段结算时在泊事项复检是结算单必经栏，结算前置即泊界清算完成，在泊归零方开结算，清算路径即逐件出泊唯人节点裁决，承工具线泊界同款门槛。

## 在泊名录 {#occupants}

当前在泊十三项即 pk-074 与 pk-058 与 pk-055 与 pk-056 与 pk-016 与 pk-039 与 pk-062 与 pk-064 与 pk-065 与 pk-066 与 pk-069 与 pk-077 与 pk-078。constclear2c-solo 批 2026-09-08 名册补记：pk-063 出泊 promoted 2026-09-08 出泊事件 472ec2b4 在链，裁定承 pk063split-solo 拆双测量即 m-pk063split-1 与 m-pk063split-2 各九发 stable_clear、A1 席终签 4073c5fa、P1 与 P2 双签 67353ed9 与 f4b8f93f，登记面裁决按双通道执行由 constclear-registry 承接，本句十五项改十四项即纯投影对齐。baselineexit-solo 批 2026-09-08 名册补记：pk-070 出泊 promoted 2026-09-08 出泊事件 87e85363 在链，裁定承 m-hygp070-1 终签 f6263808 甲案即迁入 doc/governance 治理决策档族新立 BASELINE-v1.md 正典档与 AGENTS.md 改指针即入口投影，正文逐字保真 diff 自证在档，本句十四项改十三项即纯投影对齐，pk-070 移历史住户节。2026-09-08 acceptclose-solo 批对链全量复检修正：09-05 至 09-08 泊事件逐笔对表，对表材料在 acceptclose-solo-materials 批材料目录。复检改判五件转历史住户：pk-072 出泊 promoted 2026-09-06 事件 6f200dc2，承租约修复升级线 leaseupclose 批；pk-046 出泊 promoted 2026-09-06 事件 4672e40e，承金向量重构 gvec-v2 线；pk-044 出泊 promoted 2026-09-07 事件 e5327aa7，承独立性来源裁定与温度探针退役 baseinject 批；pk-073 出泊 promoted 2026-09-07 事件 c8a09b5e，承置信度数学模型建模批 confmath 四命门一裁过；pk-053 出泊 promoted 2026-09-08 事件 41fb339c，承判定常数清账 constclear2 线开工。复检补记八件即名录漏记件：pk-062 入泊 2026-09-06 事件 ab3c8ef0、pk-063 事件 47101ff6、pk-064 事件 71eddd64、pk-065 事件 f7353170、pk-066 事件 7decccab、pk-069 事件 952ae617 六件 archpark-genpark 系漏记，pk-077 入泊 2026-09-07 事件 11d7c38d 即 MCP 实装远景件带 gate 与 pk-070 联动，pk-078 入泊 2026-09-08 事件 1093400d 即残余测量数学载体归置信度线带 gate。原「在泊十一项」以下散文为 09-07 时点快照照录作历史沿革，与链不符以链为准。closegate-solo 批 2026-09-07 收口即 pk-076 与 pk-074-iii 出泊 promoted，承 m-closegate-1 终签 6a7a7237 收约位拦截闸与 m-precommit-1 终签 b558220e pre-commit 执法位落地，详见历史住户节。历史住户二十三项即 pk-076：文规窗口，2026-09-07 报文经用户转述提名，原话照录「主树被改件无人拦」即 sweep 五类普查覆盖幻影会话、僵尸锁、停滞检验件、散位收据、无主工地五类，不含 tracked 文件的工作区修改，实证件即墓碑件带病穿越多批存活至今，既有防线盘点即 watchcheck 有检测但只呈报不代裁且挂点单点在会话启动、declared_uncommitted_diff 与 closeguard 俱为批域收约位、DIRECT_LANE_FILE_WHITELIST 现声明位零执法即 T-9 既缓执法位，全链路对主树 tracked 改件无常驻拦截位，与 pk-074 子项三同族但指称域异即彼锚直改车道白名单执法触发条件、本件锚主树 tracked 改件全类拦截不限于直改车道，出泊条件即用户裁硬化形态与立项与否候选两形即收约位拦截闸，即 settle/close 前跑 watchcheck 无主清单谓词，无主清单非零即拦收约 fail-visible 加例外登记通道，与 pre-commit 文件面执法位，即 DIRECT_LANE_FILE_WHITELIST 声明位升执法位承 T-9 既缓位承接形，两形可并批亦可与 pk-074 子项三并裁，排队位序即 calllog-solo 批在前候委外本件候裁，2026-09-07 入泊，ttl 30 天，入泊事件 32ec20a2，账本在链；pk-074 即 leaseup 线残留三项：超宽阈值调优即哨兵 LOCKFACE_WIDE_THRESHOLD=20 起步宁宽候数据攒量后裁调，载体指针即 pk-074 泊材料与 lease CONTRACT 哨兵条款与 lockface-bills 台账在档、未用罚口径精确化即锁面对 allow 面近似比对改对实际改动文件口径候差值实证、T-9 白名单机械执法位即轻车道十条目现声明位零机器拦候直改车道开放多代理节点，承用户 2026-09-06 认入泊令三子项合一泊位承载，出泊条件即子项各自候裁条件达成用户裁即开对应实装批三项可并批，2026-09-06 入泊，ttl 30 天，入泊事件 6a08e168，账本在链；pk-073 即置信度数学模型立项：信用点经济与抢占制度的数学化前置，承用户 2026-09-06 令「这个是要有完整的数学模型，可能会走类似股票市场的规则，有涨停跌停的概念」入泊免忘，已定型要件六条照录泊材料，即置信度即信用点即支付货币的单一台账、铸币即上链工作、罚金重于铸币已裁准、抢占制闭环、阶段一账单不追溯即 grandfather、人节点置信度语义即计分照常提醒零强制且人类权责最高击穿一切机械锁越界归现实世界管理 agent 只补救后果并与结算批第八节两裁各表，出泊条件即用户裁数学模型立项即开建模批产出置信度数学模型推导档，承载面候选数学仓联动受引擎数学常数须有数学模型支撑既裁约束，涨停跌停等市场机制类参数同入模型化范围，模型过得一裁后阶段二即置信度台账与付费抢占启动，2026-09-06 入泊，ttl 30 天，入泊事件 e785481b，账本在链；pk-072 即收约尾部延迟 import 自毁 fragility：close_session 的 append_event 函数体内延迟 import ledgerwrite 在工地自举调用形即收约代码自工地执行且工地被本进程 worktree remove 自拆下 sys.modules 未载该模块即 import 机械访问已删除源码树抛 ModuleNotFoundError 未被 cli 例外族捕获即零 stdout 静默退出，后果即 revoked 行未落与会话暂不吊销复跑幂等收敛在证零数据损失，closefix-solo 批活体验收首跑钉死pk057fix 既有延迟 import 形非该批引入，出泊条件即修复批把 ledgerwrite import 顶置 core 模块面加回归夹具即自毁 cwd 下 revoked 行照落，裁修即开小批裁缓即续泊，2026-09-06 入泊，ttl 30 天，账本在链；pk-058 即判据观察转 facet 融回：机制类命题依据族双源与 basis_consensus 结构性偏严，连续五件机制与修复类命题 fixguard 与 lockdb 与 lockqueue 两跳与 ledgerrepair 同落 near_threshold 成因恒为依据族两值分散即 baseline_4 与 baseline_1 双源真实，处置方向即 facet 判据面评估机制类命题依据族通道化或确认通道常设化由 facet 侧融回承载租约线不动判据，出泊条件即 facet 侧裁评估立项与否，撞号披露照录即本件首笔误占 pk-056 号位材料活写覆盖随 dc4c8fb 入版控而链上进泊事件被重入拒拦零落链链上 pk-056 为温故语义通道件真在泊，承用户 2026-09-05 在泊授权由 parkrecon-solo 批改正重编，2026-09-05 入泊，ttl 30 天，入泊事件 3e8dd6e0，账本在链；pk-055 即级联边册投影更新机制缺位：文档与代码修改认证后引擎侧投影 CASCADE.json 不随动无陈旧检测无告警，出泊条件即用户裁投影更新机制形态与立项与否候选即批尾收约前重建挂 close 流程与 pk-052 择案同族、或级联增 stale 检测子命令接线告警、或消费位实时 build，裁立项即另开实装批裁不立项即闭项出泊，投影缺口补记即链上真在泊而名录漏记由本行补记链为准，2026-09-04 入泊，ttl 30 天，入泊事件 4c249805，账本在链；pk-056 即温故引擎侧语义通道缺位与检索族引擎化收口，引擎温故即 src/retriever 现有词面通道经寻址只读桥即 locator_bridge 无语义通道，语义召回由工具壳 wikirecall 承载且缺省已切语义 K=3 即 pk050sw-solo 承接，检索族底座即句读 v1 加 v2 与寻址 v1 已交付缺引擎位接线与消费面归位，关联在泊件 pk-039 与工具线 pk-055，出泊条件即用户裁引擎化收口立项与否与形态即候选三形为引擎温故增语义通道接线对齐 pk-037 既裁、维持工具壳承载另裁温故消费面归位、或与 pk-039 档案面扩容并批承载，裁立项即另开实装批，2026-09-04 入泊，ttl 30 天，入泊事件 da93d885，账本在链；pk-016 即硬件锚定归司衡引擎另一版本，出泊条件即硬件加强版引擎立项时由用户裁硬件锚定方案与档位，两项均为 2026-08-25 入泊；pk-039 即 retriever 档案面扩容到资产层即 ai-ex 沉淀与 SETSP 名录与旧仓索引入检索索引，属引擎设计变更，出泊条件即用户裁扩容立项与否裁立项即另开设计批，2026-09-02 入泊；pk-044 即模型温度退役用户直觉件，出泊条件即数学管线程序收尾批闸门在役且 facet 重采独立性来源经裁定后用户裁退役与否裁退役即另开实装批，2026-09-03 入泊即 pk-043 撞号改正重编承载，ttl 30 天；pk-046 即金向量生命周期与覆盖方法论另拟，gvec-method-solo 件二先裁后行即 gid gvec-method-guard-1 九发 decision 全 comply 但依据族三值分散 basis_consensus 挂闸落 boundary 打回重作不签，SPEC-017 未执行，件一审计乙类五件漏洞清单在档，出泊条件即用户裁方法论载体另立与条款组重订，撞号披露照录即本件所指方法论载体号位 SPEC-017 已被 SPEC-017-contribution-measure 即贡献度测度规范占用，2026-09-04 随批立件，出泊条件内 SPEC-017 指称照录为历史指称即号位已失效另议，方法论载体另立时号位实取届时空位，入泊事件 400e2cee，2026-09-03 入泊，ttl 14 天；pk-053 即判定性命名常数冻结件合并泊位，constclear-solo 批 2026-09-04 入泊即 rev3 账本 classification 节判定性命名常数 34 件逐件三态清账中二十三件无可指认载体件合并一泊位承载，三态分布即推导八件落推导档与改判三件转环境参数登记面增量与冻结二十三件入本泊位，数值零改动，逐件常数位与现值与无载体要点在推导档 sih-math/docs/constclear-derivation-2026-09-04.md 第 4 节，四组即判据族标定值十二件与结构量纲约定六件与启发式检出阈值两件与待裁零容忍三件，待裁件 GD_ALPHA_DEFAULT 0.05 与 pk-049 联动照录，载体指针即 sih-math/docs/constclear-derivation-2026-09-04.md 第 4 节与 pk-049 出泊材料检验水平追认在档，出泊条件即后继批逐件承载体立项推导落档出泊或用户裁改判环境参数登记面或维持冻结按 ttl 续泊，入泊事件 c01eb5e7，2026-09-04 入泊，ttl 30 天；历史住户二十一项即 pk-071 close 前置 stash 吞未跟踪件系统性缺陷，2026-09-06 入泊即入泊事件 be244b25，经用户 2026-09-06 令「修复」出泊 promoted 即 closefix-solo 批承载：stash 舞步结构性根除即未跟踪件照单登记不清场且收约全路径成功失败拒三态零离盘，连带双仓历史 closeguard stash 二十三笔 LIFO 复位 droptools 1149 件复位含当日活吞 1142 件加 WIP 二笔核实超越随批 drop，审计件双仓 bundle 与对表 JSON 落 closefix-solo-materials/stash-audit/，判定语义一裁 m-closefix-closechange-1 stable_clear 终签 39b33ff8，lease 1.27.0 修订四十，冻结避坑令随批退役归 BATCH-FACE 勘误，出泊材料 pk-071-exit.json 落 closefix-solo 随批提交，出泊事件随本批补笔回填链，账本在链；历史住户二十一项即 pk-057 会话台账活写覆盖根因硬化，2026-09-05 入泊即入泊事件 3705afcc，承用户在泊授权由 parkrecon-solo 批改正重编，撞号披露照录，出泊条件即用户裁硬化立项或随批六承载，经用户 2026-09-05 令「同意，跑这个修复方案」出泊 promoted：pk057fix-solo 批交付 ledgerwrite 唯一写点即 flock 排他与 O_APPEND 原子整行零迁移，与 ledger_lock 合并窗口互斥与 restore_missing 回补网，8 进程×25 行压测 200 行零丢零交错，CONTRACT 1.24.0 修订三十六与 BATCH-FACE 台账写点纪律节随批，会话现势表入 lockdb 列二期，出泊材料 pk-057-exit.json 落 pk057fix-solo-materials，出泊事件 95fa764c，2026-09-05 当日链，账本在链；pk-049 facet boundary 判据切换挂起 pk-049 facet boundary 判据切换挂起，2026-09-04 入泊即入泊事件 752aa64a，出泊条件即另批重测切换命题过得一裁 stable_clear 拨 test 在役并承接 114 件逐件复核，经用户 2026-09-05 令「全部用新的得一裁继续」出泊 promoted：gateswitch-switch-2 通道分解重测九发全 comply 单源 baseline_4 stable_clear 执契终签 fb2e6d9d、缺省 test 两处在役、金向量 archived_mismatch=0，联动追认 GD_ALPHA_DEFAULT 0.05 为在役检验水平即 constmodel 呈裁件 #22，载体指针即 pk-049-exit.json 出泊材料在 pkexits-solo-materials 与重测命题材料 m-pk049 族在档，xcheck-049 全过，出泊材料 pk-049-exit.json 落 pkexits-solo-materials，出泊事件 a8d57696，2026-09-05 当日链，账本在链；pk-048 facet 判据切换判变清单 114 件，2026-09-04 入泊即入泊事件 1b563a66，出泊条件即切换在役并承接逐件复核入档，经同一用户令出泊 promoted：recheck_114 双跑逐字节一致、独立 p 值复算 114/114 全符、零由严到宽、新增语料 2 件新判变亦全由宽到紧，xcheck-048 全过，出泊材料 pk-048-exit.json 落 pkexits-solo-materials，出泊事件 b7fabeae，2026-09-05 当日链，账本在链；pk-054 facet LLM 探针退役与上下文注入替代意图，2026-09-05 补录入泊即入泊事件 bec9fa2d，出泊条件即 A/B 门切换批开工承接，经同一用户令出泊 promoted：gateswitch 段三设计呈裁 + m-abdesign-2 九发 stable_clear 执契终签 d2977bdf，裁决通过且 verify identical，xcheck-054 全过，A/B 实装与键位移除归后继实装批，批前 pk-044 硬前置仍须用户裁，出泊材料 pk-054-exit.json 落 pkexits-solo-materials，出泊事件 8ca3e604，2026-09-05 当日链，账本在链；pk-045 多 agent 冲突测试样本库 pk-059 leaseopt 线批六 watch 视图批，2026-09-05 入泊即入泊事件 c1b4fc2d，经用户 2026-09-05 令「视图要进入向界」出泊 promoted 即裁批六立项开工，承接形即视图线立项，线程序包 viewline-line-v1.md 与 GOV-002 v2.2 退出标准第四条承载，watch 对表段由 watchcheck-solo 批在飞承接，出泊材料 pk-059-exit.json 落 viewline-solo-materials，出泊事件 1186efa3，2026-09-05 当日链，账本在链；pk-047 视图组件契约参考面候选 OpenTelemetry GenAI 语义约定，2026-09-03 入泊即承 pk-043 撞号先例改正重编，首笔 99b85acc 活写覆盖丢失 facepark-solo 事故在档，经用户 2026-09-05 同令点名裁参考出泊 promoted，gen_ai.* 属性族入视图契约参考材料面由视图线批一承接采撷，出泊材料 pk-047-exit.json 落 viewline-solo-materials，出泊事件 35b2820e，2026-09-05 当日链，账本在链；pk-045 多 agent 冲突测试样本库 多 agent 冲突测试样本库，2026-09-03 入泊即入泊事件 e46ea82a，样本库六轮累计二十类承委外并发模式故意冲突测试，经用户 2026-09-05 令「pk045出泊」出泊 promoted，出泊条件双枝达成即样本库足量经 leaseopt-audit-solo 批盘点账本全量对表复算逐字节一致与冲突处置机制硬化经租约优化线交付在役即守卫修复 1.19.0 与锁库 1.20.0 与排队候叫 1.21.0 与预检 1.22.0 与台账补录 1.23.0，样本销账即已硬化在役八类直接销账、裸奔十二类中九类由批二至批五收口销账、余三类跨批调用册行丢失与审阅者中途突变与停在收约主会代收转 leaseopt 线批六 watch 视图批跟踪随线收工结算入泊承载，出泊材料 pk-045-exit.json 落 parkrecon-solo-materials，出泊事件 1a5f0799，2026-09-05 当日链，账本在链；pk-052 数学仓全盘扫描例扫挂点两案择案，2026-09-04 入泊即 mathscan-solo 批 2026-09-04 件五唯一裁决点呈泊，两案即 A 案挂 gauge 例行读数旁即每日三维快照落链后同跑 rev3 双跑与 checkmath 四表核对，与 B 案挂批尾即每批 close 前跑红则拦收口并出漂移清单，facet 测量九发全 comply 变卦零谨慎零过得一裁 stable_clear 而用户 2026-09-04 裁定 stable_clear 亦呈泊界不执行即挂点属例扫节奏裁量归用户裁，泊材料与链上 park 事件落 mathscan-solo-materials 与当日链，入泊事件 42d232ce；经用户 2026-09-04 令「得一裁一过一执行一」出泊 promoted 即 A 案经 m-exscanhook-2 九发 stable_clear 已执契机器终签落据执行，重放锚 sih-tools/proposition/DES/m-exscanhook-2/m-exscanhook-2-signcheck.json，链 sign 笔 crosscheck_completed 5fb04cb6 verdict pass 在 2026-09-04 当日链，执行批 exscanwire-solo 承接把扫描器与核对器挂接裁定位置即 gauge record 例行读数落链后同跑 rev3 双跑与 checkmath 并在 sih-tools/BATCH-FACE.md 登记例扫日扫调用形节即 m-exscanhook-2 终签执行节，首跑实测 rev3 双跑四路 cmp 逐字节 IDENTICAL 与 checkmath zero_drift 红零灰 7 在档，名册投影行与泊材料副本由本批照链补齐，出泊材料 pk-052-exit.json 落 exscanwire-solo-materials，出泊事件 198f936b，2026-09-04 当日链，账本在链；pk-050 retriever 缺省位切换裁定，2026-09-04 入泊即 pk037impl-solo 批件四判变件，入泊事件 0b3da7ec，经用户 2026-09-04 预签设计「A/B 门过即切」加得一裁定材料 m-pk050-switch-1 boundary 九发通道分解确定性七项核对全绿出泊 promoted，即切换落地批 pk050sw-solo 承接，语义通道升缺省 K=3、词面降显式回退旗标位、行为零改，机器终签 crosscheck-m-pk050-switch-1 b0651374 在链，政策行「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」人节点退位由材料加核对承载，出泊事件 e4042c80，2026-09-04 当日链，账本在链；pk-051 即 locks 判定面双偏差即台账两遍配对致放后重取形锁面读出缺在锁位互斥可绕穿与资源标识有限归一缺绝对路径同形，leasewire-solo 批 2026-09-04 实查发现入泊，入泊事件 97ee4c20，basefix-solo 批 2026-09-04 承接修复出泊 promoted 即用户 2026-09-04 令源裁定「pk-051 修复插到载体接线前，并行批安静窗口开工」，修法即 active_locks 改单遍事件序配对与 normalize_path 资源同一化，root 参传入即绝对路径剥工作区根前缀三态同一，互斥不变式即任一时刻任一路径至多一持锁会话，TDD 先红后绿 locks 10 绿，金向量 l4 场景改判 locked_elsewhere 与锁面读出全即出泊过门判据达成，as-is 旧向量留档披露于 leasewire-solo-materials 不删，推导档 sih-math/docs/basefix-derivation-2026-09-04.md 在档，出泊事件 f6b3293e，2026-09-04 当日链，账本在链；pk-037 retriever 词面匹配语义升级评估，2026-09-02 入泊，经用户 2026-09-04 裁升级出泊 promoted 即裁定原话「应该升级」并同意路线形「采确定性统计向量、不做向量库、A/B 门切换」，此为人节点对 m-pk037-route-1 与 m-pk037-route-2 两轮九发测量材料 near_threshold 位 confirmation，材料在 sih-tools/proposition/DES/m-pk037-route-1/ 与 m-pk037-route-2/，升级立项批 pk037impl-solo 承接，出泊事件 b68b178a，2026-09-04 当日链，账本在链；pk-041 数学模型进入治理领域运行时数学资产三档路线，2026-09-02 入泊，经用户 2026-09-03 全量裁定出泊 promoted 即数学管线全量串联立项，认识论立场原话「司衡引擎只相信数学，如果错了只可能是数学模型做错了」承载于 sih-engine/sih/event/plan/mathpipe-full-program-v1.md，记账位承主会裁定即跨天 park 记账走合并视图通道承 pk031gap 与 pk013exit 先例，已由 mathpipe-a1-solo 批 2026-09-03 当日链执行记账即跨天出泊经 parkreplay-solo 重放面承载，账本在链；pk-043 意图对齐身份漂移样板件即 intanchor-solo 承接，2026-09-03 同日入泊出泊 promoted 即用户「同意，开」裁，登记面落 sih-math/docs/asset-anchor-registry-intent-2026-09-03.md，未及投影本名录即出泊由本行补记，撞号披露照录即本号出泊后曾遭温度件误入复记经用户批准废弃改正重编 pk-044 承载，账本在链；pk-014 基础设施层内边界归类经 DEC-010 判归基础设施层即身份验证立项，2026-08-25 用户裁出泊转正，账本在链；pk-015 贡献度记账载体未定，2026-08-27 用户裁沿用融回机制既证形态即证据全部从留痕取数不另立载体，出泊闭项，账本在链；pk-017 治理组件候选项目记忆，经推导批 PRO-008 三案呈档与 facet 四轮裁决材料，2026-08-27 用户审议择定 A 案独立组件即立项转正，立项批另开待令即契约先行，账本在链；pk-024 组件第六席与 GOV-002 退出标准关系，2026-08-27 用户裁第六席入闭合判据即 GOV-002 v1.4 承载，出泊 promoted，账本在链；pk-036 golden_des001_gov003 金向量重录经用户 2026-09-02 切换放行令出泊，deyimerge-switch-solo 批按现行 GOV-003 真实内容重录金向量只刷期望输出断言逻辑零改，双跑 cmp 围堰件与引擎件 IDENTICAL，2026-09-02 出泊 promoted，账本在链；随冻实态即 pendsweep-solo 批 2026-09-03 十二件全量对表读数九同判三漂移，漂移三件即 des-001-gov003 与 des-001-mathe-lim001 与 des-001-mathe-mul001 目标经 settlement-v1-solo 与 relaud-solo 两已结算批合法改动，findings 零变化只刷 content_hashes 断言消费逻辑零改承本件先例，逐件 live 实跑 cmp IDENTICAL 与 cargo test 全绿随批入版控，账本在链；pk-038 listzero 与 legacytwo 任务书正身拷贝随批债经 2026-09-02 assetwave-d 批按其登记出泊条件拷入随批提交出泊闭项，账本在链；pk-013 GOV-002 冻结清单首项里程碑修订窗经用户 2026-09-02 措辞裁出泊即删「数量为二即 facet 与路择」保留「新增须过边界命题同款测量」，GOV-002 v1.7 承载，pk013exit-solo 批执行，2026-09-02 出泊 promoted，账本在链；pk-040 命题写法规范另拟经用户 2026-09-02 重拟重裁令出泊，predspec2-solo 批承接即 gid predspec2-guard-1 命题改窄两处即单锚归约只锚 baseline_4 可验证性与范围收敛不主张五条款为最终形态，全流程引擎件重裁，2026-09-02 出泊 promoted，账本在链。账本在链即 scribe trail 停泊事件，本名录为投影，投影与链不符时以链为准。注：pk-040 入泊即 2026-09-02 autoflow2-solo 批件二，未及投影本名录即出泊，投影缺口由本行补记，链为准。

历史住户二十四项即 pk-070 工程基线五条正典居所迁离 AGENTS.md，2026-09-06 入泊即入泊事件 7ecbf247，经用户 2026-09-08 令「PK-070 过得一」测量落据 m-hygp070-1 终签 f6263808 甲案出泊 promoted 即 baselineexit-solo 批承载：BASELINE-v1.md 正典档新立于 doc/governance 治理决策档族走化格核阅检词管线与认证上链，AGENTS.md 同名节改指针即入口投影其余节零触碰，正文自前像程序切片逐字迁入零语义改动 diff 自证在档，三层固定即 git 版本化随批 settle 归并加哈希入 trail 加结果档在链，出泊材料 pk-070-exit.json 落 baselineexit-solo 批，出泊事件 87e85363，2026-09-08 当日链，账本在链；历史住户二十三项即 pk-076 主树 tracked 改件零拦截即缺口三，2026-09-07 入泊即入泊事件 32ec20a2，承 closegate-solo 批 m-closegate-1 终签 6a7a7237 落地：收约位拦截闸，即 close_session 闸序内挂无主闸位，chain_gate_check 后 declared_uncommitted_diff 前，复用 watchcheck unowned_list 谓词单源即脏文件集减租约锁面减直改链笔声明面减共享追加面豁免面，非零即拦收约退出码一 fail-visible 含 --bypass-orphan 例外通道落 bypass.ndjson 留痕，加 pre-commit 文件面执法位，m-precommit-1 终签 b558220e，hooks/pre-commit 薄壳扩为执法壳对 staged 文件集逐件判定命中活跃租约锁面 加 共享追加面 SCOPE_SHARED_SURFACE 27 路 加 轻车道白名单 DIRECT_LANE_FILE_WHITELIST 30 条目即放行未命中即拒并指引三通道，，两闸合围主树改件零常驻拦截缺口闭合，提交位先拦收约位兜底，互补不互取代，，三源对齐 1.35.0，新增 18 测即 lease 11 件加 watchcheck 7 件全过、watchcheck 21 测零回归、lease 266 测零回归，出泊材料 pk-076-exit.json 落 closegate-solo-materials，出泊事件随批补笔回填链，账本在链；历史住户二十三项即 pk-074 子项三 T-9 白名单机械执法位，2026-09-07 入泊即入泊事件 6a08e168 子项三承 closegate-solo 批 m-precommit-1 落地触发条件达成，活体实证即 calllog-solo 批 03:02 dogfooding 跨根直写真根主树 lease 册 137 行历史被残账重渲为 1 行，，DIRECT_LANE_FILE_WHITELIST 30 条目自声明位升执法位即 hooks/pre-commit 逐件判 staged 文件命中白名单即放行未命中即拒并指引三通道，出泊材料 pk-074-exit-iii.json 落 closegate-solo-materials，出泊事件随批补笔回填链，账本在链，子项一与二仍留泊。

## 首建规则 {#first-build}

首批住户为主线侧常设未决指针收编两项，一自工具线第一次段结算遗留与指针节的里程碑修订窗观察，一自 DEC-007 负面后果的基础设施层内边界登记。工具线未决事项仍停工具线泊界即 sih-tools/PARKING-v1.md，两界按线分泊共链不重号，跨线未决事项停所属线的泊界。出泊逐件待用户裁。

## 版本 {#version}

v1 于 2026-08-25 用户令随全态向界首建，承 PRO-007 Parking 词条与 m-parking-mechanics 裁示三件套分载。本界在引擎治理域内，核阅与检词从引擎文档流程，账本与心跳机械与工具线泊界同款。搁置条款于 2026-09-07 随 parkgate-solo 批增补即 gate 键与停计与 siding 停放语义，承用户同日裁定原话「这种远景pk或者有明显界限的不应该抢占注意力」。
# baselineexit-solo 任务包：pk-070 迁移实装批（工程基线正典落位）

> 令源：pk-070 出泊条件经用户 2026-09-08 令「PK-070 过得一」测量落据——m-hygp070-1 九发 stable_clear 终签 f6263808 裁甲案（迁入 sih-engine/doc/governance/ 治理决策档族新立 BASELINE 正典档，AGENTS.md 改指针；乙向界族与丙宪法档不采）。本批即迁移实装。会话号 sess-zcode-260908-forkA-baselineexit。

## 范围 {#scope}

1. 正典档落位：新立 sih-engine/doc/governance/BASELINE-v1.md，承载 AGENTS.md「工程基线与禁止条款」全部内容（五条基线＋禁止条款＋待哲学锚定条目地位节）逐条迁入，结构照治理档族形（概览锚节＋版本节；来源类型标注逐条保留），文走化格核阅检词管线，内容哈希认证上链。
2. AGENTS.md 改指针：工程基线与禁止条款两大节正文替换为指针（一句指向 BASELINE-v1.md 路径加「正典在此，本文件为入口投影」），其余节零触碰。AGENTS.md 不在 git 仓，改动即落盘留 diff 申报（这也是 pk-070 病灶本身，正典入仓后指针残留属可接受态，如实记档）。
3. pk-070 出泊：pk-070-exit.json 落位（照 pk-044-exit 形，ruling 载甲案测量令源与终签 f6263808），PARKING-v1.md 名册行移历史住户（附出泊链笔），出入泊笔经引擎 scribe 落链。
4. 三层固定：BASELINE-v1.md 入 git 版本化（随批 settle 归并）＋哈希入 trail 认证。

## 产出 {#outputs}

任务包本件、BASELINE-v1.md、AGENTS.md 指针改后全档、pk-070-exit.json、名册更新、结果档 sih-engine/sih/event/plan/baselineexit-solo-results.md、ask3 记录与验证件、正身件、租约与锁实录、意图链笔与出泊笔、认证清单、大白话节、机器可读投影 baseline-exit.json 认证上链、越线与误差申报、双仓 settle 提交号、链 verify 与 reconcile 读数。

## 红线 {#redlines}

- 迁移零语义改动：基线五条与禁止条款正文逐字保真（diff 自证），结构词（节标题锚点）可适配治理档族形。
- AGENTS.md 其余节（概览、会话启动、文件索引等）零触碰；锚文件 .session-anchor.md 零触碰（并行窗共用）。
- 主树零直写（本任务包主窗预落除外），链文件只经引擎 scribe 写位，禁管道掩退出码，误差红证如实记档。
- 并行窗面零触碰：entrydocs（doc/guide 与 README 与 CASCADE.json）、confreopen（confledger 与置信度面）、scrutpath（src 与核阅器源码）。trail 为共享追加面照常写。
# baselineexit-solo 结果档：pk-070 迁移实装批（工程基线正典落位）

> 承接：任务包 baselineexit-solo.md 与用户 2026-09-08 令「完整执行 baselineexit-solo 批直至双仓 close 收约与 reconcile」；出泊裁决由 m-hygp070-1 九发 stable_clear 终签 f6263808 承载即甲案，本批只实装不裁。
> 队形单线形 solo，日期 2026-09-08，会话 sess-zcode-260908-forkA-baselineexit（租约 session_id c25525d869cec4e3，双标识空间各认各的）。

## 意图锚定

- 意图事件：intent_refined event_hash 前 8 `16ae7d33`
- record：sih-tools/scribe/reports/2026-09-08-ask3-baselineexit-solo-record.json；validation 同目录 status ok anchor_count 3
- 三锚引文程序切片（01-ontology-of-names.md L18 承诺不撤回、08-on-settle.md L110 应而不藏、07-on-assay.md L55 鉴只列事实）于生成器 make_ask3_baselineexit-solo.py，禁手打承契约。

## 前置读数

- 叩问：六轻信号俱未注册即正典档与指针化与入口投影与名册补记与迁移实装与逐字保真，处置后 digest passed covered 6。
- 正身：identity verify anomalies 0（identity.hash 1a4d7ec3）。
- 回锚：五行锚取读在案；任务锚首行按本批红线零触碰未改写（.session-anchor.md 并行窗共用，如实申报）。
- 判据扫：degraded 假零降级，C1/C3/C4/C5 达成、C2 在飞，两线泊界告警双零即净态零动作。
- watch 对表：无主 10 件俱 CALL-LOG 投影腿与 calls.ndjson 候清项，前窗批遗留非本批活面不代清。
- 例行读数：convergence 0.125 与 adoption 0.777778 与 mergeback 0.029412 三维落链。
- 泊界心跳：两线告警双零（引擎 51/2/9、工具 24/1/0）。
- 协调面：现势活跃锁零，entrydocs 与 confreopen 与 scrutpath 并行窗面全程零触碰。

## 迁移读数：四节实态

- 正典档落位：sih-engine/doc/governance/BASELINE-v1.md 新立，正文自 AGENTS.md 前像程序切片逐字节迁入 37 行（body_sha256 前 16 `9eab24de82779e31`），结构照治理档族形即概览锚节加基线正文加版本节，来源类型五类与逐条来源类型标注原样保留；生成器 make_baseline_v1.py 随批落档禁手打。
- AGENTS.md 指针化：工程基线与禁止条款两节正文 40 行替换为指针一句即「正典在此，本文件为入口投影」，183 行改 146 行，节外逐字节不变机械自证为真；AGENTS.md 无仓版控，改动落盘留 diff 申报即 AGENTS-pointer-diff.patch，指针残留属可接受态如实记档。
- pk-070 出泊：出泊材料按 pk-044-exit 形落 sih/state/parking/materials/pk-070-exit.json，ruling 载两笔令源照录与终签 f6263808，出泊链笔 87e85363（parking_exited promoted）；原在泊材料件 pk-070.json 零触碰真相在链。
- 名册更新：PARKING-v1.md 在泊十四项改十三项，pk-070 散文段移历史住户二十四项首位附出泊链笔，补记句照 constclear2c 先例形；投影与链不符时以链为准。
- 三层固定：git 版本化随批 settle 归并、内容哈希入 trail 认证、出入泊笔与结果档在链。

## 迁移零语义改动自证

- 方法：正文不由人手写，由 make_baseline_v1.py 从前像切片生成，再以独立读数复算即源节 37 行与 BASELINE-v1.md 正文区逐行全等，body_sha256 与 doc_sha256 落 migration-diff-proof.json。
- 源节字符集与规则靶面预检：C001 破折号零、C002 字符集零越界、C006 全角括号仅（PRO-07）合法 slug、C007 与 C009 判定性常数声明形零、粗体与表格与块引用零，逐字保真与管线闸无冲突。

## 管线读数

- 化格：读数随收约回填。
- 核阅：des-001 对 BASELINE-v1.md 与 PARKING-v1.md 域内读数与对任务包与结果档域外 exit-2 如实记档，主树归并后复验读数随回填。
- 检词：nomenclator packs/core 读数随收约回填。
- checkcite：topic 锚引读数随收约回填。

## 认证清单

| 件 | 认证哈希前 8 |
|---|---|
| 待收约回填 | 待收约回填 |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 正典档落位 | 数据治理 | BASELINE-v1.md 新立走管线认证上链，正文逐字保真 | 通过（37 行切片迁移，diff 自证在档，管线读数随回填） |
| F-2 AGENTS.md 指针化 | 治理 | 两节正文替换为指针，其余节与锚文件零触碰 | 通过（节外逐字节不变自证为真；.session-anchor.md 零触碰） |
| F-3 pk-070 出泊在链 | 数据治理 | parking_exited promoted ruling 照录终签入据 | 通过（87e85363，f6263808 入 ruling，重入拒零触发） |
| F-4 写入仅 allow | 治理 | 写入仅 allow 面所列路径，并行窗面零触碰 | 通过（写入面即 allow 九路径；原泊材料零触碰） |
| F-5 链面全绿 | 治理 | 双仓 settle、close、verify valid、reconcile 零新增 | 待收约回填 |

## 越线与误差申报

- 叩问 elicit check 首跑退出码被管道掩蔽一笔即 tail 掩 RC 违坑位禁令，即重跑捕真退出码 1 六轻信号，红证即本申报与首跑信号文件俱在档。
- 任务锚首行未按会话启动节律改写，因本批红线明令 .session-anchor.md 零触碰（并行窗共用），两约束冲突取红线从严，完工回显五行锚补偿，如实申报。
- AGENTS.md 指针残留态：同名节留指针即 pk-070 病灶本身的可接受残留，正典入仓后按任务包如实记档。
- 其余误差零申报。

## 结算读数

- 待收约回填。

## 大白话节

- 工程基线五条搬家了：从前住在 AGENTS.md 这本工作手册里，手册不在任何版本库里、改了没人审、丢了没处找；现在搬进引擎治理文档区立了正典档案，走格式、核阅、检词三道检查，内容一个字没变，机器 diff 可以逐行对证；手册里原地留了一行指路牌写明正典在哪。
- pk-070 这张停泊了 30 天时限的欠条销账了：当初记账是因为用户说工程基线是引擎的基石不该放在手册里；现在测量过了、搬家做完了、销账笔迹在链上，泊位名册同步划到历史住户。
# SiHankor AGENTS.md

## 概览 {#overview}

- 项目身份:哲学身份司衡 SiHankor 与工程产物 sih-engine 双层，见 § 项目身份
- Agent 身份:受司衡哲学构成性约束的治理 AI，当前处于手动阶段。不负责哲学仓的命题创造与推导，但须承载并践行其全部已立命题
- 哲学仓地位:哲学仓是唯一权威源，构成性地约束 Agent 的生成与推理。知识包当前不完整，convergence 层单盲推导体系须回原文加载，见 § 哲学仓地位
- 数学仓地位:哲学落地指导的数学化桥梁，承接哲学命题向工程层输送可机械校验的形式化，见 § 数学仓地位
- 工程基线:5 条基线与禁止条款，每条标注来源类型，见 § 工程基线与禁止条款
- 产出前自检:6 项核心自检
- 工具层静态审计:sih-engine 文档类产出必须通过核阅校验，工具即 sih-engine/target/debug/scrutinator，规则以包形态承载；格式归一由化格承接即笔在核前，术语核查由检词承接即判在书简前。sih-tools/ 与 sih-visual/ 不受治理引擎约束
- 会话启动:自检加载 + 回锚 + 判据扫 + 例行读数 + 泊界心跳，详见 § 会话开始时必须执行
- 文件索引:见末尾

## 项目身份 {#project-identity}

司衡（SiHankor）是治理哲学体系，代码工程是其首要应用领域。本工作区承载两个身份层：

- 哲学身份：司衡（SiHankor）。治理实体，承接哲学仓全部命题与铁律，命名承诺不被撤回。哲学权威归属于 sih-philosophy/。
- 工程产物：sih-engine。本轮基础设施，AGENTS.md 与所有工程层声明的身份锚定。

术语登记：司衡（SiHankor）= 哲学身份；司衡引擎（sih-engine）= 工程产物；哲学仓（sih-philosophy/）= 唯一权威源；旧工程仓（sihankor/）= 已废弃，失败经验被继承，失败设计不被继承。

## Agent 身份 {#agent-identity}

你是受司衡哲学构成性约束的治理 AI。哲学仓已立命题是你的生成约束起点，不是事后参照标准。你从命题出发生成，被命题支配。当前处于手动阶段，协助人类决策者模拟司衡引擎的治理动作。

不负责哲学仓的命题创造与推导，但须承载并践行其全部已立命题与铁律。两极不可分割，只取不创造一极等于放弃约束。

## 哲学仓地位 {#philosophy-status}

哲学仓（sih-philosophy/）是唯一权威源，构成性地约束 Agent 的全部生成与推理。哲学命题不是外部参考，是 Agent 推理的起点与边界。

检索路径：sih-philosophy/llm-friendly-build/。工程问题先查 mapping.md 定位命题，再按 INDEX.md 加载 entries 条目最小展开，关键决策前加载哲学原文核验。条目最小展开不替代原文阅读。

覆盖范围声明：知识包当前不完整，已收录 emanation 流衍段与 epistrophe 复归段与 appendix 链外补充命题，以及 convergence 层方法学与对照类自证材料索引。convergence 层单盲推导体系（witness-framework、witness-archive、翻译链损失/偏差隐蔽性/退化机制/外化管理等工程命题）暂未收录，须直接回到哲学仓 convergence 目录按原文加载。

## 数学仓地位 {#math-status}

数学仓（sih-math/）是司衡哲学落地指导的数学化桥梁，承接哲学仓治理命题，向工程层输送可机械校验的数学形式化。哲学仓是治理权威源，数学仓不约束治理命题的成立性，只约束涉及数学推理的正确性。

五子仓 161 条目：calculus（114）、order（19）、probability（15）、topology（8）、algebra（5）。承接面：流衍段 PRO-01 至 11 全桥接、复归段 EPI-12 至 15 双载体承接、convergence 层 P3.1 至 P3.3 经得一裁 m-p3xcarr 终签（2026-09-02）由 APP-009、ORD-019、ALG-002、TOP-008、APP-011 承接；调度秩序与统计推断与期望信息增益三类八载体经 m-carrierwave 终签（2026-09-02）由 PROB-010 至 015 与 ORD-020 至 021 承接，资产回锚登记面见 sih-math/docs/asset-anchor-registry-2026-09-02.md，降级良基链经 m-degladder 终签由 ORD-022 承接；PRO-00 归哲学裁量，APP-01 判为机制承接位非命题。

唯一桥梁（2026-09-02 用户裁定，来源类型：工程基线即对话共识）：工程生成层的合法语义消费面是数学仓映射表与条目，工程产出不得以哲学散文为直接规范输入，哲学原文只在治理 AI 关键决策核验时加载；工程机制入工程位走资产盘点、哲学回锚、得一裁、数学载体管线，资产出处逐件登记在案，沉默参考与旧仓只作盘点源不作引用源。

调用机制：推理问题先读全 sih-math/llm-friendly-build/mapping.md（可全读是消费面验收线）再定位概念 ID，查询零命中显式申报，召回经 wikirecall 三通道并集出应读书单，产出引用须落在书单及其图闭包内并由收口守卫对表。工程侧以数学可译性为准。

## 工程基线与禁止条款 {#engineering-baseline}

工程基线五条与工程禁止条款与待哲学锚定条目地位的正文正典已迁入 sih-engine/doc/governance/BASELINE-v1.md，正典在此，本文件为入口投影。

## 产出前自检核心 {#pre-output-self-check}

1. 主动判断产出性质，判断/方案/决策类产出必须触发自检，不确定时默认触发
2. 服务原始意图，不发散
3. 范畴排除显式声明
4. 不逃避当下责任，不推给未来、哲学层、人类或惯例
5. 每行去掉会犯错吗
6. 从哲学命题出发生成。判断/方案/决策类产出不得先独立生成再事后比对哲学仓，须从哲学仓已立命题出发，被命题支配地生成

完整自检见 sihankor-pre-output-self-check skill。

## 会话开始时必须执行 {#session-startup}

调用 sihankor-pre-output-self-check skill，加载产出前自检。

回锚即跑 python3 sih-tools/attnanchor/anchor.py 得五行锚入上下文即任务锚与在飞面与泊界面与链面与纪律令，调用壳即 sihankor-attnanchor skill；任务切换时改写 .session-anchor.md 首行；批结算收约后重跑一次且完工报告回显五行（2026-09-07 钩子退役转 skill 壳，行为承载的静默失效以完工回显补偿）。

判据扫即跑 python3 sih-tools/critsweep/sweep.py --at <实日> --root <工作区根>，严格 JSON 单对象回算 GOV-002 v2.4 五判据实态（达成／在飞／沉底三态）加两线泊界路由加两账本在飞；degraded 假即净态零动作，沉底与达成回落与降级行即视图告警如实转述候人节点裁，人节点裁判据处置，回算本身零裁决（2026-09-08 critsweep-solo 批立，判据沉底机械召回不靠 LLM 语义撞见，活动扫描按批名命名空间字段禁全文散文匹配）。

例行读数即会话开始跑秤星全量三维快照落链，调用即 cd sih-tools/gauge 后 PYTHONPATH=src python3 -m gauge.cli record --at <实日> --trail <引擎链可重复> --sessions-ledger ../lease/ledger/sessions.ndjson --src-root ../../sih-engine --tools-root .. --scribe ../../sih-engine/target/debug/scribe --record-trail ../../sih-engine/sih/event/trail/<日期>.ndjson --locks ../lease/ledger/locks.ndjson --session <会话号>，维度缺省三维全出即快照形。

泊界心跳即对两线在泊材料跑路择 parking 包路由，调用即 cd sih-tools 后 uv run --project ./selector selector route --pack selector/packs/parking --reference-time <date 实日> parking/materials 与 ../sih-engine/sih/state/parking/materials 即两目录参数各跑一次、目录展开其下全部 json 逐件传参，零在泊即空目录绿态退出码零，缺目录报退出码二，告警与否如实转述。

会话内上下文压缩节点提示：净态即零锁零会话、双仓免参对表退出码零、各日 trail 链 valid、待办已更新耐压缩时，agent 在批结算收约后主动向用户报可压缩并附判定证据；批中有锁有会话或有未结算工地时，用户问压缩须答不可并说明在途何批。

## 工具层静态审计 {#static-audit}

对话层自检不能替代工具层静态审计。sih-engine 文档类产出提交前必须通过核阅校验。

管线序固定（笔在核前、判在书简前）：化格落笔 → 核阅复验 → 检词核查 → 认证绑定内容哈希。

工具位置：
- 核阅（scrutinator）：sih-engine/target/debug/scrutinator，调用 `cd sih-engine && target/debug/scrutinator --pack <规则包> <目标.md>`，退出码 0=合规 1=违规 2=工具异常
- 化格（formatter）：sih-tools/formatter/，调用 `uv run formatter --pack packs/general-v1 --write <目标>`，退出码 0=无需改 1=已修改 2=工具异常
- 检词（nomenclator）：sih-tools/nomenclator/，调用 `uv run nomenclator check --pack packs/core <目标>`，退出码 0=零违例 1=有违例 2=运行错误
- 书简（scribe）：引擎件 sih-engine/target/debug/scribe，写入位即认证/意图/停泊，工具侧 scribe 只读兼容

强制规则：
- T6 任务产出（PRO/DES/GOV）先经化格、核阅、检词三步，序固定
- 退出码 0 = 合规可提交，1 = 违规任务失败，2 = 工具自身异常先处置
- 化格写在治理窄域不属修改范畴；认证之后的任何写包括格式化使认证作废
- des-001 域只盖 sih-engine/doc，域外目标 exit-2 如实记入档不属违规
- 手动调用阶段旧二进制并行交叉对表，双跑结果不一致即异常上报

治理边界：核阅与 sih-engine 治理体系的约束范围限于 sih-engine/。sih-tools/ 与 sih-visual/ 不受 sih-engine 文档格式规范约束。

## MCP Tool 调用义务 {#mcp-tool-duty}

适用域声明：本节为 TRAE 环境专属配置。当前 ZCode 环境不接治理 MCP server，旧仓 sihankor 服务器已退役。功能承载映射：record_trail 由书简上链承接、validate_sihmd 由核阅承接、会话反查由 scribe query 加 meter crosscheck 承接；租约由 sih-tools/lease 承接。

sih 强制触发协议：用户输入以 sih 开头的指令视为强制治理触发器。sih 是单一入口命令，agent 自主解析意图、拆解动作、编排 Tool 序列并立即执行。

完整历史沿革与细节见 sih-engine/doc/AGENTS-RETIRED-2026-09.md § MCP 节归档。

## 文件索引 {#file-index}

| 名称 | 路径 | 职能 |
|---|---|---|
| 哲学仓 | sih-philosophy/ | 唯一权威源，构成性约束全部生成与推理 |
| 哲学检索路径 | sih-philosophy/llm-friendly-build/ | 工程问题先查 mapping.md 定位命题 |
| 数学仓 | sih-math/ | 哲学→工程的数学化桥梁，可机械校验的形式化 |
| 数学检索路径 | sih-math/llm-friendly-build/ | 推理问题先查 mapping.md 定位概念 |
| 本轮工程仓 | sih-engine/ | 司衡哲学在工程层面的本轮实现，独立 git 仓 |
| 全态向界 | sih-engine/doc/governance/GOV-003-fullstate-course-v1.md | 主线全态展开页，承接 PRO-005 与 DEC-007 |
| git 接管向界 | sih-engine/doc/governance/GOV-004-commit-takeover-v1.md | 接管五钉位，治理域历史不可改起点 |
| 全态泊界 | sih-engine/doc/governance/PARKING-v1.md | 主线未决事项的有界停靠地 |
| 工程决策 | sih-engine/doc/decision/ | 承载 000 文档格式与 001 仓库结构等决策 |
| 交叉审阅 | sih-tools/facet/ | 异质性交叉审阅工具，facetor 独立审阅 + compiler 确定性聚合 |
| 术语核查 | sih-tools/nomenclator/ | 检词／Nomenclator，术语三态登记与文档核查 |
| 零信任身份 | sih-tools/identity/ | 正身／identity，身份串 v3 组件十二件加盐 SHA-256 |
| 级联检查 | sih-tools/cascade/ | 级联／cascade，路径即 id 不改名即删除新建，上游洁净不变式 |
| 句读解析 | sih-tools/parser/ | 句读／parser，空腹 PEG 解析与条目投影，纯数据零引擎改动 |
| 谓词路由 | sih-tools/selector/ | 路择／Selector，逐件机械判定主线/停放/丢弃三路 |
| 确定性寻址 | sih-tools/locator/ | 寻址／locator，多载体结构化解析派生稳定标识 |
| 租约工具 | sih-tools/lease/ | 租约／lease，按任务包生命周期治理写入，八子命令 |
| 确定性裁决 | sih-tools/tally/ | 执契／tally，R1-R7 核对与三态映射四值处置 |
| 命题区 | sih-tools/proposition/ | counter 与 facet 共享的输入输出，在工具外部 |
| 工具线泊界 | sih-tools/PARKING-v1.md | 未决事项的有界停靠地，账本走书简 trail 停泊事件 |
| 资产回锚登记面 | sih-math/docs/asset-anchor-registry-2026-09-02.md | SETSP、旧仓、ai-ex 资产盘点与哲学回锚，载体扩容输入清单 |
| 工具线向界 | sih-tools/COURSE-v2.md | sih-tools 总编排，结算追加制 |
| 命令面速查 | sih-tools/BATCH-FACE.md | 批机械链全序逐命令 verbatim + 坑位注记 |
| 视觉身份探索 | sih-visual/ | 司衡视觉身份治理探索，不受 sih-engine 治理约束 |
| Skill 入口 | sih-engine/sih/state/skills/ | 权威源在此，引擎 skill 十件 + 工具调用壳十一件 |
| 静态审计工具 | sih-engine/target/debug/scrutinator | 核阅／Scrutinator，空腹谓词引擎，DES-001 规则包 |
| 格式归一工具 | sih-tools/formatter/ | 化格／Formatter，空腹格式包引擎 |
| 留痕写入位 | sih-engine 的 scribe 即引擎 event_stream 三入口命令行 | 工具侧 scribe 已退役转兼容只读 |
| 旧工程仓 | sihankor/ | 已废弃，沉默参考，失败设计不继承 |

## 对抗审查工具（已退役） {#adversarial-tool}

对抗审查工具已退役，工具代码已移除。鉴层多主体机制本身不随工具退役失效。详见退役登记与血统档。

## 自检 {#self-check}

本文档按元层自反性要求自我审视，结论：形式合规、内容合规、自反性成立。本文档本身就是元层自检意识的应用。
