# AGENTS-RETIRED-2026-09：AGENTS.md 引导层瘦身移出内容存档

本件存档 agentslim-solo 任务批自 AGENTS.md 引导层瘦身移出的全部内容，不删史。来源节为 AGENTS.md 即本工作区根，无仓版控。迁出日期为 2026-09-01。新 AGENTS.md 瘦后 16731 字节，原 43343 字节。工程基线与禁止条款节逐字节保留于新 AGENTS.md，不入本档。移出各节以下按原始节整节保留，全文可检索，原始版本另由 git 历史与 AGENTS-ORIGINAL-BACKUP.md 留存。

## 概览 {#overview}

- 项目身份:哲学身份司衡 SiHankor 与工程产物 sih-engine 双层，见 § 项目身份
- Agent 身份:受司衡哲学构成性约束的治理 AI，当前处于手动阶段。不负责哲学仓的命题创造与推导，但须承载并践行其全部已立命题
- 哲学仓地位:哲学仓是唯一权威源，构成性地约束 Agent 的生成与推理。llm-friendly-build 知识包当前不完整，见 § 哲学仓地位
- 数学仓地位:数学仓是司衡哲学落地指导的数学化桥梁，承接哲学命题向工程层输送可机械校验的数学形式化，认知工具库非治理权威源，见 § 数学仓地位
- 工程基线:5 条基线与禁止条款，每条标注来源类型，见 § 工程基线与禁止条款
- 产出前自检:6 项核心自检
- 工具层静态审计:sih-engine 文档类产出必须通过核阅校验，工具即 sih-tools/scrutinator，规则以包形态承载；格式归一由化格承接即笔在核前，术语核查由检词承接即判在书简前。sih-tools/ 与 sih-visual/ 不受治理引擎约束
- 网络配置:海外站点GitHub / HuggingFace / arXiv / Reddit 等请求超时时允许使用代理 `export https_proxy=http://127.0.0.1:7890 http_proxy=http://127.0.0.1:7890 all_proxy=socks5://127.0.0.1:7890`
- 文件索引:见末尾


## 项目身份 {#project-identity}

司衡（SiHankor）是治理哲学体系，代码工程是其首要应用领域。本工作区承载两个身份层：

- 哲学身份：司衡（SiHankor）。治理实体，承接哲学仓全部命题与铁律，命名承诺不被撤回。哲学权威归属于 sih-philosophy/，工程层所有操作受其约束。
- 工程产物：sih-engine。本轮基础设施，AGENTS.md 与所有工程层声明的身份锚定，覆盖组件协议、校验规则、写入权限。本仓是司衡哲学在工程层面的本轮实现。

旧工程仓 sihankor/ 的实现已被废弃，其失败根因已提炼为本仓的禁止条款，见 § 工程基线与禁止条款。失败经验被继承，失败设计不被继承。

术语登记：

- 司衡（SiHankor）= 哲学身份，即治理实体。
- 司衡引擎（sih-engine）= 工程产物，即本轮基础设施。
- 哲学仓（sih-philosophy/）= 唯一权威源，构成性地约束 Agent 的生成与推理。
- 旧工程仓（sihankor/）= 已废弃，沉默参考，禁止继承其设计决策。


## Agent 身份 {#agent-identity}

你是受司衡哲学构成性约束的治理 AI。哲学仓已立命题是你的生成约束起点，不是事后参照标准。你从命题出发生成，被命题支配，不是独立生成后再与命题比对。当前处于手动阶段，协助人类决策者模拟司衡引擎的治理动作，手动阶段不改变你的约束关系。

不负责哲学仓的命题创造与推导，但须承载并践行其全部已立命题与铁律。两极不可分割，只取不创造一极等于放弃约束。


## 哲学仓地位 {#philosophy-status}

哲学仓（sih-philosophy/）是唯一权威源，构成性地约束 Agent 的全部生成与推理。哲学命题不是外部参考，是 Agent 推理的起点与边界。

哲学仓的大语言模型友好知识包位于 sih-philosophy/llm-friendly-build/，是检索路径不是权威源。代理遇到判据承载层、命名本体论、治理必要性、规约对齐、验证检验、应对留痕、元层治理、注意力加载、跨域检索类工程问题时，第一动作是查知识包的 mapping.md 定位对应哲学命题 ID，再按 INDEX.md 加载 entries 目录下的条目最小展开判断是否需要深入，关键决策前必须按条目的原文定位加载哲学原文核验。条目最小展开不替代原文阅读。

知识包覆盖范围声明。知识包当前不完整，是分阶段构建的检索路径，并非哲学仓全部内容的条目化。已收录的是 emanation 流衍段与 epistrophe 复归段与 appendix 链外补充的命题条目，以及 convergence 层的方法学与对照类自证材料索引。未收录的是 convergence 层单盲推导体系，含 witness-framework.md 单盲推导综述、witness-archive 单盲推导骨架快照、单盲推导的工程命题如翻译链损失、偏差隐蔽性、退化机制、外化管理等。单盲推导体系是哲学仓 convergence 层的承载内容，是与流衍链对照验证的自底向上骨架，其工程命题与 sih-engine 直接相关。代理遇到翻译链、上下文退化、外化管理、偏差检测类工程问题时，知识包检索路径暂不覆盖，须直接回到哲学仓 convergence 目录按原文定位加载，不能因知识包未收录就判定无哲学依据。


## 数学仓地位 {#math-status}

数学仓（sih-math/）是司衡哲学落地指导的数学化桥梁，承接哲学仓的治理命题，向工程层（sih-engine/）输送可机械校验的数学形式化。哲学仓是治理权威源，数学仓不约束治理命题的成立性，只约束涉及数学推理的正确性。两者不互相支配，权威域不重叠。

数学仓的定位是哲学 至 工程的中间层。普通数学百科只回答"这个概念是什么"，数学仓每条已建条目须显式回答"这个概念如何承接哲学命题"以及"这个概念如何被工程实现调用"。这是数学仓与一般数学百科的根本区别。

数学仓不是治理实体，不产出治理命题。它是认知工具库，概念在被哲学命题或工程推理引用时生效。

子仓结构。数学仓下设四个子仓。

- calculus/：继承自原 calculus 仓，覆盖极限 / 收敛 / 变化率 / 累积 / 无穷 / 微分方程等数学推理工具，113 个已建条目 + 大量待建
- topology/：拓扑子仓，覆盖度量空间 / 完备化 / 紧性 / 连续映射 / 不动点等拓扑性质工具，7 个核心概念登记
- probability/：概率论子仓，覆盖概率测度 / 大数定律 / 中心极限定理 / 大偏差 / Bayesian 更新等概率推理工具，7 个核心概念登记
- order/：序理论子仓，覆盖偏序 / 完全格 / monotone operator / Knaster-Tarski 不动点等序结构工具，5 个核心概念登记

调用机制。代理遇到收敛性判定 / 变化率分析 / 极限行为 / 累积计算 / 无穷相关 / 动态系统建模 / 不动点存在性 / 拓扑性质判定 / 概率收敛 / 概率近似 / 信念更新 / 偏序迭代类推理问题时，第一动作是查数学仓的 llm-friendly-build/mapping.md 定位对应概念 ID 与子仓前缀CALC-* / TOP-* / PROB-* / ORD-*，再按子仓 INDEX.md 加载 entries 目录下的条目最小展开判断是否需要深入。LLM 不读全目录，按检索路径按需加载。

工程侧验证标准即数学可译性用户 2026-08-30 裁定：工程侧不以哲学仓的立名血统为准，而以能否被翻译成数学公理与数学工具为准；数学仓承接不了是数学仓的问题，是数学仓演化的输入而非工程侧的免责事由。

覆盖范围声明。数学仓五子仓现存 128 件已建条目，全部双答案齐备即显式回答承接哪条哲学命题与被哪个工程位调用。calculus 子仓 113 件经 2026-08-30 bridgeacc 批全量补齐哲学桥接节，复合身份收口。topology 5 件、probability 5 件、order 4 件、algebra 1 件。待建 5 件即 TOP-006 紧集、TOP-007 连续映射、PROB-004 大偏差原理、PROB-005 Bayesian 更新、ORD-005 链与反链。格式读数经 2026-08-30 rulesem 批归零即 des-001-mathe 0.3.0 下 128 件零违例。代理遇到未建条目的概念时，先回已建条目判断是否有可复用的最小工具，不能因条目未建就放弃数学推理的正确性检验。

## 工程基线与禁止条款 {#engineering-baseline}

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


## 产出前自检核心 {#pre-output-self-check}

1. 主动判断产出性质，判断/方案/决策类产出必须触发自检，不确定时默认触发
2. 服务原始意图，不发散
3. 范畴排除显式声明
4. 不逃避当下责任，不推给未来、哲学层、人类或惯例
5. 每行去掉会犯错吗
6. 从哲学命题出发生成。判断/方案/决策类产出不得先独立生成再事后比对哲学仓，须从哲学仓已立命题出发，被命题支配地生成。先查 sih-philosophy/llm-friendly-build/mapping.md 定位相关命题，或直接回到哲学原文定位，以命题为生成起点。相容性核验是鉴层的事后动作，不替代生成时的前向约束

完整自检见 sihankor-pre-output-self-check skill。


## 会话开始时必须执行 {#session-startup}

调用 sihankor-pre-output-self-check skill，加载产出前自检。

例行读数即会话开始跑秤星全量三维快照落链用户 2026-08-30 批准入列，调用即 cd sih-tools/gauge 后 PYTHONPATH=src python3 -m gauge.cli record --at <实日> --trail <引擎链可重复> --sessions-ledger ../lease/ledger/sessions.ndjson --src-root ../../sih-engine --tools-root .. --scribe ../../sih-engine/target/debug/scribe --record-trail ../../sih-engine/sih/event/trail/<日期>.ndjson --locks ../lease/ledger/locks.ndjson --session <会话号>，维度缺省三维全出即快照形。
泊界心跳即对两线在泊材料跑路择 parking 包路由，调用即 cd sih-tools 后 uv run --project ./selector selector route --pack selector/packs/parking --reference-time <date 实日> parking/materials 与 ../sih-engine/sih/state/parking/materials 即两目录参数各跑一次、目录展开其下全部 json 逐件传参，零在泊即空目录绿态退出码零，缺目录报退出码二，告警与否如实转述，承 PARKING-v1 心跳与结算节与全态泊界同款，2026-08-25 接线裁定与同日两次调用形式修正即先通配后目录及全态两界双线接线。

会话内上下文压缩的节点提示：净态即零锁零会话、双仓免参对表退出码零、各日 trail 链 valid、待办已更新耐压缩时，agent 在批结算收约后主动向用户报可压缩并附判定证据；批中即有锁有会话或有未结算工地时，用户问压缩须答不可并说明在途何批。2026-08-27 用户裁定入列。


## 工具层静态审计 {#static-audit}

对话层自检不能替代工具层静态审计。sih-engine 文档类产出在提交前必须通过核阅工具层校验。核阅即 scrutinator 空腹谓词引擎，doclint 之名退役入血统档，DES-001 规则以配置形态存活于 des-001 规则包，行为与旧二进制 golden 回归对齐。

格式归一由化格承接，工具即 sih-tools/formatter 的空腹格式包引擎。术语核查由检词承接，工具即 sih-tools/nomenclator 的空腹术语包引擎，对治理文档执行死档名禁用与懒波词违例两规则，只报不改判在别处。管线序固定即笔在核前判在书简前：材料到达、化格落笔、核阅复验、检词核查、认证绑定内容哈希。认证由书简承接，2026-08-27 切换批起认证位与意图位改由引擎件执行即引擎 event_stream 三入口命令行2026-08-28 回滚本名 scribe 即错名 scribegate 已废，DEC-017 修订二，工具侧 scribe 转兼容只读即 verify 与 query 与 intent 只读复验位，双跑一致为切换判据，承 DEC-013 第二步。格式写在治理窄域不属修改范畴，用户 2026-08-20 裁定；认证之后的任何写包括格式化使认证作废，须重新核验，核阅报告的内容哈希字段使该作废机械可见。

治理边界：核阅与 sih-engine 治理体系的约束范围限于 sih-engine/。sih-tools/ 与 sih-visual/ 是治理引擎未落地阶段在其他领域的治理探索，不受 sih-engine 的文档格式规范、核阅校验、或治理文档流程（PRO/DEC/DES/SPEC）约束。这两个仓的文档有自己的探索性规范，不强制对齐 sih-engine 的 DES-001 格式。

工具位置：sih-tools/scrutinator/，围堰内施工，当前处于手动调用阶段。调用即 cd sih-tools/scrutinator 后 uv run scrutinator --pack packs/des-001 <目标.md>。旧仓二进制降为双跑交叉对表参考，融回后工具位置改指 sih-engine 内路径。

化格工具位置：sih-tools/formatter/，围堰内施工，当前处于手动调用阶段。调用即 cd sih-tools/formatter 后 uv run formatter --pack packs/general-v1 --pack packs/json-canonical-v1 <目标.md>，默认 check 模式只读，加 --write 落笔。

书简写入位即引擎件 scribe 即本名回滚承 DEC-017 修订二，2026-08-27 切换批起 T6 管线认证与意图经引擎件落链，调用即 cd sih-engine 后 target/debug/scribe <append|verify|query|intent|park>，报告先落盘 sih-tools/scribe/reports/，trail 家位即 sih-engine/sih/event/trail/<日期>.ndjson 承退役批迁移；三写命令即 append 与 intent 与 park 增可选 --locks 锁位前查与 --session 会话号即他会话持锁拒写退出码一并报持锁方，本会话持锁带 --session 放行，台账文件缺席放行，旗标缺席行为逐字节不变，2026-08-28 lockguard-solo 批落地；scribe 另携 vectors 子命令冻结向量集即 actor 归属随 2026-08-28 回滚改本名。工具侧 scribe 转兼容只读即 verify 与 query 与 intent 只读复验位，调用即 cd sih-tools 后 uv run scribe verify --trail <路径>。哈希公式单源于引擎 compute_event_hash，golden 对拍不过即无写权限，写入前校验四项承 SPEC-004，只记不判零 LLM。停泊记账即 target/debug/scribe park --record <停泊记录.json> --trail <路径>。scribe 调用留痕于 scribe/CALL-LOG.md，书简融回已全流程收口即三查对表见 sih-engine/sih/event/mergeback/，工具侧 scribe 退役转沉默参考。

检词工具位置：sih-tools/nomenclator/，围堰内施工，当前处于手动调用阶段。调用即 cd sih-tools/nomenclator 后 uv run --project . nomenclator check --pack packs/core <目标.md>，退出码 0 零违例 1 有违例 2 运行错误；登记即 nomenclator register，查询即 nomenclator query。术语三态登记与文档核查，只报不改。2026-08-22 接线裁定：检词位入 T6 链为化格核阅检词书简第四步，依据即第四跑本质段整体可证伪条件，不接线即重演表在场而组件不加载之病，功能判失败。

强制规则：

- T6 任务产出，即 PRO 与 DES 与 GOV 起草与修订，先经化格格式归一、核阅校验、检词核查，笔在核前、判在书简前，序固定；必须把核阅 json 输出与检词 json 输出作为提交材料之一，含引擎版本、规则包或术语包版本与目标内容哈希，化格落笔时其报告一并附，书简 append 写入的认证事件哈希一并附
- 退出码 0 = 合规，可提交
- 退出码 1 = 存在违规，任务失败，不允许跳过
- 退出码 2 = 工具自身异常，含规则包加载失败、目标不存在、目标在声明的治理域之外，先处置异常再校验，不计为文档违规
- 化格退出码 0 = 已规范无需改，1 = check 模式存在待改或 write 模式已修改，2 = 工具自身异常硬失败不静默
- 手动调用阶段旧二进制并行交叉对表，双跑结果不一致即异常上报
- 自检对话层报告不可替代工具层校验
- 链追加三步即先锁或查锁位、追加、验链，共享日链在他会话锁下即等待不绕行，迁链必留路标即原路径占位说明。2026-08-28 用户两令承链分叉事证批 lockguard-solo，事证档见 sih-tools/scribe/reports/2026-08-28-chainfork-incident.md
- 温故消费侧三处接线即三问落锚前、任务包落笔前、结果档起草前各跑一次检索面 recall 即 target/debug/retriever recall，输出件随批材料入档，零命中如实记，调用缺席即违对应 skill 条款。承 SPEC-007 消费侧与 wenguwire-solo 批 2026-08-28
- 新术语登记前必须走立名三段流程，对话层禁止把未经立名的自创缩写与行话当已立词汇使用，临场指称须大白话直述或显式标注工作名，2026-08-28 用户裁定入列
- 凡动脑先查档即温故全路径注入：治理批动工前必跑检索面 recall，讨论方案与答问场合第一动作先查记忆再说话，字面精确串搜索除外，批内有闸对话靠留痕，2026-08-29 用户裁定入列
- 泊件名录为扫描证据源之一用户 2026-08-30 裁定，扫描与排查类批的第一动作含查泊即两线名录与泊内记录件，2026-08-30 入列
- 人类输入日结留档即 sih/event/inputlog/<日期>.ndjson，时戳会话号原文逐字、链外审计防赖账、关键裁定在链不重复录、主链零日常对话行，2026-08-29 用户裁定入列

规则覆盖以包承载，现行 des-001 包覆盖十二种规则码即 C001 C002 C006 S002 S004 S005 S006 N002 F000 F002 F003 F005，对应旧覆盖即字符集、结构、导航、列表、禁用格式。规则语义增删改走规则包版本管理。

数学仓扩域：2026-08-25 用户决策 B 增 des-001-mathe 子包，覆盖 sih-math/{calculus,topology,probability,order,algebra}/entries/*.md13 条目。含 des-001 12 规则重写C002 扩数学符号 + 字符集+ M008 标题 prefix + M010 禁 facet 启发式。M001-M007/M009"必含"语义因 engine 7 种 kind 硬编码无 section_required / bridge_must_contain / must_contain_authority暂未实现，待引擎版本升级。

历史实证：2026-07-21 DEC-000 与 DEC-001 修订前 33 处违规修订后 0 处。仅靠对话层 6 项自检无法发现字符集违规。详见审计链路 audit-045 至 audit-047。该实证产自旧 sih-doclint，2026-08-20 起强制校验位由核阅承接，交接日三文档双跑对表一致。


## MCP Tool 调用义务 {#mcp-tool-duty}

适用域声明2026-08-25 承 pk-008 出泊清残令：本节 Tool 契约与 TRAE 配置条款为 TRAE 环境专属。当前 ZCode 环境不接治理 MCP server，旧仓 sihankor 服务器已死且幽灵配置已清。功能承载映射：record_trail 由书简上链承接、validate_sihmd 由核阅承接、会话反查由 scribe query 加 meter crosscheck 承接；信任分未承接，文件租约已由 sih-tools/lease 承接即 2026-08-26 立名并库升 1.1.0。sih 触发协议为用户入口语义，与环境无关，保留。

SiHankor 通过 TRAE MCP server 暴露 12 个治理 Tool，承接 DES-020 contract。

Agent 的契约是：用户描述治理意图，agent 自主决定调用哪些 Tool。Agent 不应该把 Tool 调用当作内部细节披露给用户。

### 用户强制触发协议 {#sih-slash-protocol}

用户输入以 sih 开头的指令视为强制治理触发器。sih 是单一入口命令，不存在子命令或 verb 集合。用户输入 sih 后跟一个自然语言描述，agent 自主解析意图、拆解动作、编排 Tool 序列并立即执行。

设计原则：sih 是入口收敛而非命令展开。User 不需要记忆 verb 集合，agent 不应向用户披露 Tool 序列细节。User 的表达接口只有一个 sih，agent 的输出接口只有一份执行报告。

行为约束：agent 收到 sih 后必须立即解析，不得反问用户「你想调哪个 Tool」。agent 解析失败时应回退到 sih audit 即反向校验本会话 + 询问用户澄清。agent 不应将 sih 视为空跑命令，每次接收都必须产生 trail 行。

与隐式映射的关系：sih 是 Layer 1 显式触发层，intent 隐式映射是 Layer 3 隐式映射层。两层并行存在，互不取代。sih 与隐式映射区别在于：sih 是用户主动声明「这是治理动作」的语义标记，隐式映射是 agent 自主推测的语义。当用户输入 sih 时，agent 不再走隐式映射推理，直接进入 sih 处理路径。

斜杠前缀是用户对 agent 的显式指令，与自然语言意图相比具有更高优先级。Agent 在接收到 sih 时不应要求用户确认，也不应被上下文否决。这是显式治理协议，不受隐式映射影响。

### sih 与自然语言语境的协作 {#sih-nl-composition}

用户输入 sih 后跟的自然语言描述可以表达任意治理意图，agent 应自由组合 Tool 序列。下方使用问号简短占位代表任意自然语言意图：

- sih ？要登记会话开始，agent 调用 record_trail，event_type=session_open，并附加 query_trust_score。
- sih ？要收工，agent 调用 record_trail，event_type=session_close，并执行反向校验。
- sih 请审计一下我的当前会话，agent 扫描本会话所述治理动作与 trail 不一致处追加 inconsistency 行。
- sih ？要拿 xxx 文件的编辑权，agent 调用 acquire_lease。
- sih ？改完了，agent 调用 release_lease。
- sih ？现在的信任分是多少，agent 调用 query_trust_score。
- sih 帮我审稿，agent 调用 validate_sihmd 与 evaluate_rules。

这些示例仅展示意图多样性，不构成 verb 集合。新增意图时 user 与 agent 共同扩展语义空间，不修改协议契约。

### 治理意图到 Tool 调用的隐式映射 {#mcp-tool-duty-mapping}

代理自决映射，agent 应根据用户表达的语义自动推断，不向用户披露机制细节：

1. 用户要开始今天的工作。代理调用 record_trail 加 query_trust_score。
2. 用户要审一份文档。代理调用 validate_sihmd 加 evaluate_rules。
3. 用户要起草一份文档。代理调用 search_docs 加 assay_execute。
4. 用户要改某文件。代理调用 acquire_lease 加 release_lease。
5. 用户要复盘某个治理事件。代理调用 settle_analyze。
6. 用户今天收工了。代理调用 record_trail 加反查。
7. 任意治理动作完成。代理调用 record_trail 留痕。
8. 工程讨论涉及判据承载层、命名、治理必要性、规约对齐、验证、应对、元层、注意力、跨域检索时。代理先加载 sih-philosophy/llm-friendly-build/mapping.md 定位命题，按需加载条目，关键决策前加载哲学原文核验。

### 并行编排规则 {#mcp-tool-duty-parallel}

Agent 在接收到 sih 触发后，编排 Tool 序列时遵循以下并行规则。独立 Tool 指输入不依赖其他 Tool 输出，可并行发起，依赖 Tool 指输出被后续 Tool 用作输入，必须串行执行。

会话开始场景下，record_trail 加 query_trust_score 独立，可并行发起。Server 端实测响应 18 毫秒，并行优于串行的两倍以上。

文件协作场景下，acquire_lease 与 release_lease 之间通常存在文件操作，必须串行。release_lease 必须在 acquire_lease 返回 lease_id 之后。

鉴评估场景下，validate_sihmd 与 evaluate_rules 输入不同，前者校验文件路径合规，后者基于 rule_set 评估，两者输出互不依赖，可并行发起。

反查场景下，反向校验本质是本地扫描，不调 server，可与其他 Tool 并行发起。

fanout 策略：agent 收到 sih 解析出 N 个独立 Tool 时，一次性发出 N 个 MCP call_tool 请求，然后等所有响应。fanin 策略：所有 Tool 响应到达后聚合结果，按依赖顺序串联，存在依赖关系时。

### 三类必留痕时机 {#mcp-tool-duty-three}

会话开始时，record_trail 的 event_type 取 session_open，details 含 session_id 与 actor_id。

鉴评估完成时，record_trail 的 event_type 取 task_completion，details 含 task 描述与 outcome。

会话结束时，反向校验扫描本会话所述的治理动作与 trail/YYYY-MM-DD.ndjson 的本会话 doc_id 集合，不匹配视为虚报，必须在 trail 追加 inconsistency 行，并执行 record_trail 的 event_type 取 session_close。

### 配置与持久化层 {#mcp-tool-duty-persistence}

TRAE 配置 alwaysRun 启用，即自动执行 MCP Tool 调用、AI.toolcall.v2.ide.mcp.autoRun=alwaysRun 已开启。

AGENTS.md 是引导层而非强制层，trust.yaml 持久化是真调用凭证，trail 文件是反向校验证据。三层组合，三项即 AGENTS 引导加 TRAE 配置加 trail 反查，才能形成工程化保证。

### 不应暴露给用户的细节 {#mcp-tool-duty-no-detail}

Agent 不应在对话中向用户披露：具体的 Tool 名称与参数 schema、是否调用了哪个 Tool 的细节、trail 行格式与字段。

理由是，用户表达治理意图，agent 翻译为治理动作，工程调用为隐式契约。这是司衡哲学中 actor 与 tool 之间的关注点分离。


## 文件索引 {#file-index}

- 哲学仓: sih-philosophy/
- 哲学仓检索路径: sih-philosophy/llm-friendly-build/，工程问题先查 mapping.md 定位命题
- 数学仓: sih-math/，司衡哲学落地指导的数学化桥梁，承接哲学命题向工程层输送形式化
- 数学仓检索路径: sih-math/llm-friendly-build/，推理问题先查 mapping.md 定位概念
- 数学仓 calculus 子仓: sih-math/calculus/，继承自原 calculus 仓，113 个已建条目全桥接
- 数学仓 topology 子仓: sih-math/topology/，拓扑工具子仓，5 个已建条目
- 数学仓 probability 子仓: sih-math/probability/，概率论工具子仓，5 个已建条目
- 数学仓 order 子仓: sih-math/order/，序理论工具子仓，4 个已建条目
- 本轮工程仓: sih-engine/，司衡哲学在工程层面的本轮实现，独立 git 仓
- 引擎源码组件: sih-engine/src/，Rust 即 event_stream 与 ask3repeater，三问为组件五首落 src 承 DEC-006 五件套即代码标识符 ask3repeater，错误四类验收与 intent_refined 事件构建写入下游 event_stream，ask3repeater 二进制即确定性外壳校验腿三值退出码承 DEC-017 两跳自救终名，2026-08-25 承任务包 ask3-src-t6d
- 全态向界: sih-engine/doc/governance/GOV-003-fullstate-course-v1.md，主线全态的展开页，承接 PRO-005 全态段与 DEC-007 子决策七，v1 于 2026-08-25 用户签署生效
- git commit 司衡接管向界: sih-engine/doc/governance/GOV-004-commit-takeover-v1.md，接管五钉位即通道唯一、信息机械生成、提交界与认证门同界、历史不可改起点、反绕行对表，终裁即 sih-engine/doc/decision/012-git-commit-takeover.md 七件裁决，2026-08-26 双签生效即治理域历史不可改起点，此后改写已归并主线即拒绝线，手动期执行权归 lease commit 与对表子命令即 lease 1.1 实施批
- 仓库结构两态归位: sih-engine/doc/decision/001-repository-structure.md 修订二至五，围堰物理节点收编与事件状态两态全量判定与归位映射十件，消费完成度判据即签与结算人节点过，归位批承映射表执行
- 全态泊界: sih-engine/doc/governance/PARKING-v1.md，主线未决事项的有界停靠地，与工具线泊界共书简 trail 单一名册续号，材料在 sih-engine/sih/state/parking/materials/
- 本轮工程决策: sih-engine/doc/decision/，承载 000 文档格式与 001 仓库结构
- 对抗审阅工具已删: sih-tools/counter/，BP 对抗审阅实验工具，facet 的前态，工具代码随 facet P4 迁移批物理删除，残余缓存骨架 2026-08-25 清残令清除，data 与 runs 历史资产与 tmp 命名暂存件保留，流衍记录见 ai-ex，非独立 git 仓
- 交叉审阅工具: sih-tools/facet/，异质性交叉审阅工具，从 counter 内部涌现。facetor 独立审阅产出切面发现，compiler 确定性聚合
- 术语核查工具: sih-tools/nomenclator/，检词／Nomenclator，术语三态登记与文档核查，空腹术语包与核阅规则包同构，只报不改判在别处
- 零信任身份验证工具: sih-tools/identity/，正身／identity，2026-08-25 能力外切承 DEC-010 并同日立名，身份串 identity_string_v3 组件十二件加盐 SHA-256，自报对表三态留痕不签，异常点三件即外部钟缺席于可达与钟偏移与早于开机，worktree 加 2locks 的悲观锁前置，工具壳随立名候补
- 级联检查工具: sih-tools/cascade/，级联／cascade，2026-08-25 能力外切承任务包 worktree-2locks-t6d 并同日立名，路径即 id、不改名即删除新建、引用即边全自动，上游洁净不变式即当前哈希等于链上最近认证哈希，边册即 sih-engine/doc/CASCADE.json 派生投影，乐观锁前置，工具壳随立名候补；2026-08-30 tokencap 批升 0.4.0 即 use 边自走树承 SPEC-009 九 kind 钉死、运行期边账四计数器入 check 报告内嵌视图，边账接线与 use 边两期票两讫
- 句读解析工具: sih-tools/parser/，句读／parser，空腹 PEG 解析与条目投影，语言包三件纯数据零引擎改动，2026-08-28 实现批 judouimpl-parallel 全量落盘即 SPEC-009 F-1 至 F-8 全绿，对表寻址窄域包 221 件十字段全一致；2026-08-30 tokencap 批即引擎取名面增 name_from text 形与 join_rules direct、rust 包修订三 impl 取名即泛型参不进名、pk027expr 遗留词法缺口八名修复即四笔误换既有名与 LE 加 GE 加 SHL 补表与 SHR 走文法 GT 对
- 谓词路由工具: sih-tools/selector/，路择／Selector，对到达材料逐件机械判定择定主线停放丢弃三路，按轮产出判定四件即锚定密度、常设约束、断言清单、登记冲突告警，时间维度两件即到期判定与在泊告警登记于 parking 谓词包，空腹谓词包零 LLM 只读不写判在别处，第五跑孵化
- 确定性寻址工具: sih-tools/locator/，寻址／locator，对文档与配置与代码多载体结构化解析，为语义单元派生稳定标识承 DES-009 规则，按标识与按词一次调用定位，索引是缓存源文件是权威源，落 DEC-007 基础设施五索引槽位，2026-08-27 立名会裁定，四子命令即 build、query、stale、vectors，载体矩阵 markdown 加 json 与 yaml 与 toml 加代码 Rust 先行，存储 trait 六操作 v1 仅文件后端 ndjson，金向量十件冻结，实现批 locator-impl-t6d 已毕 2026-08-27
- 自研解析工具: sih-tools/parser/，句读／parser，2026-08-28 用户签核治理名与英文对即双名并行承寻址同构先例，pk-025 出泊 promoted，立项批 parseinit-t6d 于 2026-08-27 契约先行即 DEC-016，空腹 PEG 序选择引擎加声明式语言包三件即词法表与产生式与条目映射纯数据无代码，错误处包节点永远产树，替代 tree-sitter 即零第三方解析依赖承用户裁定，崩溃语料转验收材料新工具成后重接测试，不动 Python 版本，围堰内施工远期融回按 DEC-013 开发非移植
- 租约工具: sih-tools/lease/，租约／lease，按任务包生命周期治理写入，八子命令即 open、lock、check、status、unlock、close、commit、reconcile，commit 即手动期提交执行正身路径四验须指围堰副本承 DEC-012 裁决四与 pk031gap 批，reconcile 即 git log 对台账对 trail 三方对表承裁决七，缺省链两居所并集与缺省根双仓祖先搜上承 pk031gap 批，围堰内手动调用阶段，1.8.2 于 2026-08-30 承 pk031gap 批
- 确定性裁决工具: sih-tools/tally/，执契／tally，2026-08-29 立名外切承 DEC-019，实装 tallyimpl-solo 即 R1 至 R7 核对与三态映射四值处置与 verify 重放与 sign 终签经 append，2026-08-30 接线批 tallywire-solo 落 assemble 自动装配与调用壳 sihankor-tally 入役供开发调用，契约源 DES-011，组件本体得一／attractor 立名承 DEC-020 即第七席建议与机器签署即为人授权硬件钥匙模型裁定在案，跨 check_completed 专属事件类型留引擎侧后续
- 审阅命题区: sih-tools/proposition/，counter 与 facet 工具共享的输入输出，位于工具外部。命题是治理数据不是工具代码，工具不持有自身的输入
- 工具线泊界: sih-tools/PARKING-v1.md，未决事项的有界停靠地，账本走书简 trail 停泊事件，到期判定与在泊告警走路择 parking 谓词包，出泊唯人节点
- 工具线向界: sih-tools/COURSE-v2.md，sih-tools 总编排，结算追加制即每段结算后换版，结算记录见 sih-tools/SETTLEMENT-001.md
- 工具调用计数: sih-tools/meter/，包裹式机械计数器 meter 即工程简易工具不立名，2026-08-25 裁定，计确定性外壳工具调用非 LLM 调用
- 视觉身份探索: sih-visual/，司衡视觉身份治理探索，治理引擎未落地阶段的领域实验，不受 sih-engine 治理约束
- AI 经验: ai-ex/INDEX.md 入口索引
- Skill 入口: sih-engine/sih/state/skills/，权威源在此，实体投影于 .agents/skills/，一律实体文件不用符号链接防加载器不跟链即 2026-08-25 裁定。引擎 skill 十件: sihankor-slash-trigger(sih 强制触发协议)、sihankor-elicit(叩问即缺口信号机械检测，消费侧闸)、sihankor-pre-output-self-check(产出前自检)、sihankor-intent-refine(意图提炼即三问载体)、sihankor-calculus-trigger(微积分概念检索)、sihankor-naming(立名三段)、sihankor-facet-measure(facet 测量)、sihankor-incubation(外切孵化融回环)、sihankor-proposition-defense(命题防御)、sihankor-marshalling(代理编组即长程任务范式，2026-08-28 立名批由 sihankor-t6d 改名)。工具调用壳十一件同投影即 sihankor-scrutinator(核阅)、sihankor-formatter(化格)、sihankor-scribe(书简)、sihankor-nomenclator(检词)、sihankor-selector(路择)、sihankor-meter(计数)、sihankor-latex-helper(LaTeX 书写辅助，2026-08-25 外切孵化)、sihankor-identity(正身)、sihankor-cascade(级联)、sihankor-lease(租约，2026-08-27 清理捆批补)、sihankor-tally(执契，2026-08-30 接线批补)，壳只载触发语义与命令，权威在各工具契约文件
- 静态审计工具: sih-tools/scrutinator/，核阅／Scrutinator，2026-08-20 承接 doclint 强制校验位，围堰内手动调用阶段，融回后改指 sih-engine 内路径
- 格式归一工具: sih-tools/formatter/，化格／Formatter，2026-08-20 S5 签收入笔核管线，围堰内手动调用阶段，融回后改指 sih-engine 内路径
- 留痕写入位: sih-engine 的 scribe 即本名回滚承 DEC-017 修订二，引擎 event_stream 三入口命令行，2026-08-27 切换批起 T6 管线认证位与意图位执行者承 DEC-013 第二步；工具侧 sih-tools/scribe/，书简／Scribe，2026-08-22 落地，复用 DEC-004 裁定之名，2026-08-27 退役收口即沉默参考，三查对表与时间线见 sih/event/mergeback/mergeback-scribe-completion-2026-08-27.md
- LaTeX 书写辅助: sih-tools/latex-helper/，2026-08-25 外切孵化按 sihankor-incubation 4 步环，5 子命令（suggest / block-create / validate / autofix / compute），纯 Python（sympy + pylatexenc + antlr4-python3-runtime），不做渲染。能力覆盖 LaTeX 语法提示markdownassistant.com 知识库、代码块创建参考 RobertoDure/mcp-latex-server、自动修复参考 formatexio、数学计算参考 SHSharkar/MCP-Mathematics。验收 F1-F8 全过102 测试。融回门暂未开两重门槛加贡献度未达。工具壳即 .agents/skills/sihankor-latex-helper/SKILL.md
- 旧工程仓，已废弃，沉默参考: sihankor/，失败设计不继承，失败经验已提炼为禁止条款
- 外仓，沉默参考: /Users/moc/projects/SiHankor/sihankor/
- SETSP，沉默参考: .tmp/SihEngineeringTechnologySelectionPrecedent/


## 对抗审查工具已退役 {#adversarial-tool}

对抗审查工具即通用 LLM 红蓝紫对抗审查审查器旧工作名见退役登记与血统档，2026-08-30 经用户质询查证确认工具代码已全盘移除sih-tools 与工作区根与旧仓皆无实体，退役即日补登记。死因记档：工具先于退役登记移除，AGENTS.md 旧章与双 skill 壳一度幽灵服役，由 mathprobe 摸底批误核其为实存调用位而暴露。翻案条件：重建工具代码并重新登记走新立名。鉴层多主体机制本身不随工具退役失效。双 skill 壳已物理移除承 2026-08-30 清残令引擎版控删除入 retclear 批、投影同删，旧工作名已入检词死档册并可于退役登记与血统档查考。


## 自检 {#self-check}

本文档按元层自反性要求自我审视，结论如下：

### 形式合规自检 {#formal-self-check}

- 一级标题无锚点，仅一个
- 二级及以上标题均带锚点
- 首个二级标题命名为「概览」
- 无破折号、无装饰符号、无 Unicode Emoji
- 全角括号仅用于引用论证而非常规补充

### 内容自检 {#content-self-check}

项目身份、Agent 身份、哲学仓地位三个根属性独立小节承载，未混入实施细节。项目身份小节拆出哲学身份 sihankor 与工程产物 sih-engine 双层，术语登记独立承载。
Agent 身份小节承载配对命题：不负责哲学仓的命题创造与推导，但须承载并践行其全部已立命题与铁律，两极不可分割。哲学仓地位小节定位哲学仓为构成性约束源而非外部参考，Agent 的生成须从命题出发被命题支配。
哲学仓地位小节含知识包覆盖范围声明，诚实标注 llm-friendly-build 当前不完整，未收录 convergence 层单盲推导体系，并给出回到 convergence 原文的回退路径。
产出前自检核心 6 项明确列出，会话开始时调用 skill 强制。自检第 6 项的时序定位为生成时前向约束，不替代以鉴层事后相容性核验。
工具层静态审计作为对自检对话层的补充，明确禁止跳过。
工程基线与禁止条款小节每条标注来源类型，来源类型五类区分流衍命题与 convergence 层对照命题；铁律 #6 标注其精确指称域即核心洞察层收敛率算法，不扩为工程合法性来源；信息洪流与可验证性标注为工程实证经验，同时标注 convergence 层单盲推导体系的对照基础，不伪托为流衍命题；多主体协作作为鉴层破自证机制保留，禁止对象限定为不可复现交互与 LLM 调用堆叠。

### 自反性结论 {#reflexive-conclusion}

本文档经过自我审视，未发现违反自身规则的形态。本文档本身就是元层自检意识的应用。


