# convergence 层检索路径

本知识库文档是 sih-engine 工程仓消费哲学仓 convergence 层的检索路径。哲学仓不修改。代理通过本文档间接消费 convergence 层内容。

哲学仓 sih-philosophy/ 的 llm-friendly-build 知识包当前不完整，只收录 emanation 流衍段与 epistrophe 复归段与 appendix 链外补充的命题条目，以及 convergence 层的方法学与对照类自证材料索引。convergence 层单盲推导体系的工程命题未收录。本文档补这个检索缺口。

本文档归位知识库类型，承载术语映射类内容，即工程问题到哲学命题的映射。承接 DES-001 知识库文档特异化规范 WRITING-KNOW-003 术语映射定义列表格式要求。

## 概览 {#overview}

- convergence 层承载单盲推导体系，是与流衍链对照验证的自底向上骨架::[单盲推导体系定位](#blind-derivation-position)
- 两套编号体系须先区分：witness-framework 用 L1/L2/M，治理原则装订书用 C/P/R::[编号体系说明](#numbering-system)
- 工程问题到 convergence 条目的术语映射是核心产出，覆盖九类工程问题::[术语映射](#term-mapping)
- 每条映射标注认识论地位，是对照证立非流衍命题::[认识论地位标注](#epistemic-status)
- 代理遇到工程问题按六步检索优先序定位::[检索优先序](#retrieval-priority)

## 单盲推导体系定位 {#blind-derivation-position}

单盲推导体系是哲学仓 convergence 层的承载内容。它由一个独立 AI 在完全隔离上下文的前提下，自底向上从工程痛点推导得出，使用其自主的术语体系，不沿用流衍链的道法鉴应元五柱石命名。它与流衍链在 convergence 层做对照验证，核心洞察层收敛率 87.5%。

单盲推导体系的认识论地位是对照证立。它从工程痛点归纳而上，流衍链从道家本体流衍而下，两者方向相反但归属相同，都属于司衡哲学。它不是流衍命题本身，不能以 PRO 命题 ID 引用。工程层引用时标注 convergence 层对照命题。

llm-friendly-build 检索路径暂未覆盖 convergence 层单盲推导体系。代理遇到翻译链、上下文退化、外化管理、偏差检测类工程问题时，知识包检索路径不适用，须通过本文档回到 convergence 原文定位加载。

## 编号体系说明 {#numbering-system}

convergence 层有两套编号体系，代理须先区分，不可混用。

第一套是 witness-framework.md 的骨架编号。它用 L1 标注一级概念共 10 条，用 L2 标注二级概念共 43 条，用 M 标注元原则共 5 条。这是骨架抽象层，定位在 convergence/witness-framework.md 第 28 行至第 136 行。L1 与 L2 是概念层级，M 是跨概念的元原则。

第二套是 witness-archive/00-ai-coding-governance-principles.md 的治理原则装订书编号。它用 C 标注部共 7 个，用 P 标注篇共 21 个如 P3.1，用 R 标注原则共 10 条。这是治理原则的展开层，定位在该文件第 59 行至第 68 行的 R 原则总览表，以及第 76 行起的 P 篇章。

两套体系的关系。L1 与 L2 是骨架抽象，C 与 P 与 R 是治理原则装订书的展开。同一个工程问题在两套坐标里都能定位。例如信息洪流在骨架层对应 L1-4 比例度量与 L2-4.4 治理疲劳，在装订书层对应 P2.1 速度不对称与 R1 验证比例原则。代理检索时两套坐标可交叉使用。

引用规范。引用骨架层用 L1-N 或 L2-N.M 格式。引用装订书层用 P 篇章号如 P3.1 或 R 原则号如 R4。两者都须附文件路径定位。不可把 P 篇章号与 PRO 流衍命题号混淆，前者是单盲推导的工程命题，后者是流衍链的哲学命题。

## 术语映射 {#term-mapping}

工程问题到 convergence 层条目的术语映射。每条按定义列表格式承载，含工程问题术语、定义即对应 convergence 条目、来源即原文定位三要素。承接 WRITING-KNOW-004 术语表三要素要求。

信息洪流与注意力不对称
: LLM 生成速度与人类审查注意力之间的结构性不对称，治理系统须假设注意力是稀缺资源
: 骨架层 L1-4 比例度量，定位 convergence/witness-framework.md 第 37 行。L2-4.1 风险分级与 L2-4.4 治理疲劳，定位该文件第 72 行起。装订书层 P2.1 速度不对称，定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 320 行，核心论点是生成边际成本趋零与验证成本恒定的不对称不可消除。R1 验证比例原则，定位该文件第 59 行。元原则 M-1 比例性原则，定位 witness-framework.md 第 130 行起。

可验证性与可审计性
: 所有写入操作须满足来源可追溯与结果可机械校验与历史不可篡改
: 骨架层 L1-6 留痕即治理，定位 convergence/witness-framework.md 第 39 行。L2-6.1 决策留痕与 L2-6.5 留痕的抗操纵性，定位该文件第 87 行起。装订书层 P4.2 责任归属界定，定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 472 行，核心论点是留痕从事后文档重定义为同时性治理基础设施，四个可即可追溯与可审计与可反驳与可继承。P4.3 决策留痕审计，定位该文件第 515 行，核心论点是动态轨迹是治理的一等对象。R4 来源归属原则，定位该文件第 62 行。

确定性程序执行
: 治理操作的执行归确定性程序，LLM 产出仅作为待校验的符号材料
: 骨架层 L1-7 反馈闭合，定位 convergence/witness-framework.md 第 40 行。L1-8 治理自限，定位该文件第 41 行。装订书层 P5.2 校准工程路径，定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 591 行。R6 反馈环中断原则，定位该文件第 64 行。元治理决策 D003 基础层仲裁者，定位 convergence/witness-archive/03-philosophy-skeleton.md 第 491 行，核心论点是提议权对主对话即 AI 关闭。D004 治理变更的治理，定位该文件第 537 行，核心论点是风险下限不可下调与源头优先自检与提议权对主对话关闭三重防御。

人类只看异常
: 确定性程序自动处理绝大多数操作，人类只在视图告警时介入不看原始日志
: 装订书层 P3.1.3 软退化先兆信号，定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 229 行，核心论点是软退化比硬截断更危险，三个先兆信号即引用精度下降与决策摇摆频率上升与约束遗漏率增加，需纵向审查而非横向审查。P5.2.3 校准反馈嵌入，定位该文件第 623 行起，核心论点是校准反馈嵌入是注意力管理系统，偏差信号必须在注意力经济中有竞争力。R7 交付度量原则，定位该文件第 65 行，核心论点是度量交付结果而非生成指标。

LLM 只生成符号材料
: LLM 不拥有写入知识包与事件流与意图锚定的权限，产出仅作为待校验材料
: 骨架层 L1-2 意图锚定，定位 convergence/witness-framework.md 第 35 行。L2-2.1 翻译链不可逆与 L2-2.5 意图可见性，定位该文件第 57 行起。装订书层 P1.3 意图保真路径，定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 162 行，核心论点是三维锚定即正向与负向与边界，双路径纠正即修代码与修意图。R8 意图锚定原则，定位该文件第 66 行。元治理决策 D003 提议权对主对话关闭，定位 convergence/witness-archive/03-philosophy-skeleton.md 第 491 行。

上下文退化与注意力中段稀释
: Transformer 注意力机制在长序列下中段信息提取精度显著低于首尾，退化在硬上限之前即开始
: 装订书层 P3.1 退化机制，定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 201 行。三节展开：P3.1.1 注意力稀释是 Transformer 架构的数学必然，定位第 205 行。P3.1.2 连续退化曲线三阶段即平台期与衰减期与加速衰减期，拐点位置需本地校准，定位第 217 行。P3.1.3 软退化比硬截断更危险，定位第 229 行。

外化管理与状态外化
: 持久状态须从模型内部迁移到模型外部，以显式可查询版本化存储承载
: 骨架层 L1-9 显式化运动，定位 convergence/witness-framework.md 第 42 行。L2-9.1 假设外化，定位该文件第 109 行起。装订书层 P3.2 外化管理，定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 242 行。三节展开：P3.2.1 LLM 隐式记忆三重结构性缺陷即容量有限与退化渐进与审计不可行，定位第 246 行。P3.2.2 外化三性质即持久性与版本化与可审计性，定位第 258 行。P3.2.3 管理成本与粒度悖论，外化不是最大化而是最优化，定位第 272 行。

翻译链损失
: 意图到代码的翻译链每一步都是有损编码，损失是语义结构变形
: 装订书层 P1.1 翻译链损失，定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 76 行。三节展开：P1.1.1 四步翻译链即意图到需求到 prompt 到 AI 内部建模到代码，定位第 80 行。P1.1.2 损失分布非对称即前端重后端隐，定位第 92 行。P1.1.3 不可逆性的数学论证，治理策略必须前置，定位第 104 行。

偏差检测
: AI 生成的形式正确性掩盖语义偏差，偏差检测须引入外在于代码的意图基准
: 装订书层 P1.2 偏差隐蔽性，定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 119 行。三节展开：P1.2.1 形式正确性陷阱即 AI 核心能力就是形式正确性，定位第 123 行。P1.2.2 三维偏差信号即不一致与过度实现与低度实现，定位第 135 行。P1.2.3 意图鲜活度半衰期，审查链必须包含原始意图持有者，定位第 149 行。

## 认识论地位标注 {#epistemic-status}

本节标注术语映射中所有条目的认识论地位，避免代理误用。

第一条，convergence 层对照基础。术语映射中所有 convergence 条目都是对照证立，不是流衍命题。它们的认识论地位是单盲推导体系从工程痛点归纳而上的证立，与流衍链对照验证收敛率 87.5%。代理引用时不可标注为 PRO 命题或 EPI 命题，须标注为 convergence 层对照命题。

第二条，与流衍链的收敛率。核心洞察层即 10 个一级概念层收敛率 87.5%，定位 convergence/witness-framework.md 第 184 行至第 196 行。五层独立收率：原则层 75%，方法论层 70%，元原则层 50%，术语体系层 30% 因单盲推导使用自主术语，工程落地层 40%。工程落地层收率最低，是 sih-engine 应重点关注的结构性盲区。

第三条，llm-friendly-build 检索路径暂未覆盖。术语映射中所有条目都不在 llm-friendly-build 的 mapping.md 与 INDEX.md 与 entries 目录中。代理遇到这些工程问题时，查 mapping.md 查不到，须通过本文档回到 convergence 原文。这是检索缺口，不是哲学依据缺口。

## 检索优先序 {#retrieval-priority}

代理遇到工程问题时的检索步骤，按优先序执行。

第一步查 R 原则定义。定位 convergence/witness-archive/00-ai-coding-governance-principles.md 第 59 行至第 68 行的十条原则总览表。R 原则是最浓缩的治理原则，先查这里判断工程问题对应哪条原则。

第二步查命题深度论证。按 R 原则对应的 P 篇章号，定位同文件的对应篇章。P 篇章是 R 原则的深度展开，含论证骨架与本篇贡献。

第三步查哲学骨架对应。定位 convergence/witness-archive/03-philosophy-skeleton.md 的对应一级概念章节。骨架快照提供痛点到根因到推导路径的三段式标注。

第四步查术语映射。定位 convergence/00-terminology-mapping.md 的映射表。映射表承载哲学层术语与工程层术语的对应关系，共 20 条。

第五步查差异与盲区。定位 convergence/03-divergence-analysis.md。差异分析记录单盲推导与流衍链的 6 区域差异与 10 条术语冲突，标注哪些已闭合哪些不适用。

第六步查体系综述。定位 convergence/witness-framework.md。体系综述提供 L1 与 L2 与 M 坐标下的整体视图，含收敛率统计。

## 已知盲区 {#known-gaps}

本节记录 convergence 层对 sih-engine 落地的已知盲区，代理须知晓。

第一个盲区是工程映射显式化。差异分析第 5 区域明确：emanation 链不做工程展开，每个治理原则需配套可操作实践与可观测信号与可识别反模式，这是工程仓 sih-engine 的降格映射工作。这意味着 convergence 层提供哲学对照基础，但不提供工程实现细则，sih-engine 须自行做降格映射。

第二个盲区是验证与鉴的层次区分。差异分析冲突 1 记录：工程层的验证是二元判断，哲学层的鉴含一次穿透即验证与度量同基准。sih-engine 的验证设计若只做二元判断会落入盲区，须补一次穿透。

第三个盲区是术语体系层收率最低。单盲推导使用自主术语体系，与流衍链的道法鉴应元术语不重合，术语体系层收率仅 30%。代理引用 convergence 条目时须注意术语差异，必要时查 convergence/00-terminology-mapping.md 做术语转换。

第四个盲区是 M 编号的两个索引空间。witness-framework 的 M-1 至 M-5 是五条元原则，red-team 攻击向量用 RT-M-1 至 RT-M-4。引用时须加 RT 前缀区分，不可混用。

## 关联 {#relation}

- 元规则：DEC-001 仓库结构决策
- 通用规范：DES-001 文档格式设计
- 类型规范：DES-001 知识库文档特异化规范
- 哲学仓 convergence 层：sih-philosophy/convergence/
- 哲学仓 llm-friendly-build：sih-philosophy/llm-friendly-build/
- 失败复盘：sih-engine/tmp/old-repo-failure-analysis.md
- 启动任务包：sih-engine/tmp/T-001-sih-engine-bootstrap.md

## 认识论立场 {#epistemic-stance}

本文档为 external-anchor 与 design-corollary 混合。术语映射部分为 external-anchor，来源是哲学仓 convergence 层原文。组织加工部分为 design-corollary，工程问题到 convergence 条目的检索路径设计是工程设计选择。

可证伪条件：若哲学仓 convergence 层后续被 llm-friendly-build 完整收录，本文档作为检索缺口补全的角色失效，需重新评估其定位。

## 自检 {#self-check}

### 形式合规自检 {#formal-self-check}

- 一级标题无锚点，仅一个
- 二级及以上标题均带锚点
- 首个二级标题命名为概览
- 无破折号、无装饰符号、无 Unicode Emoji
- 全角括号仅用于单一治理编号

### 内容自检 {#content-self-check}

- 编号体系说明独立承载，两套坐标区分清晰，引用规范明确
- 术语映射覆盖九类工程问题，每条用定义列表格式承载术语与定义与来源三要素，满足 WRITING-KNOW-003 与 WRITING-KNOW-004
- 认识论地位标注明确区分对照证立与流衍命题，不伪托
- 已知盲区诚实记录，含工程映射盲区与验证鉴层次区分与术语收率与 M 编号冲突
- 检索优先序六步可执行，每步附文件路径与行号定位

### 自反性结论 {#reflexive-conclusion}

本文档是工程仓消费哲学仓的检索路径，不修改哲学仓。本文档归位知识库类型术语映射类，承接 DES-001 知识库文档特异化规范。本文档本身经过自我审视，未发现违反 DES-001 格式规范的形态。本文档的存在是 llm-friendly-build 检索缺口的部分补全，不替代哲学仓原文阅读。
