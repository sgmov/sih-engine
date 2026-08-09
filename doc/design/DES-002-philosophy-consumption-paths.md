# 哲学仓消费路径设计

sih-engine 工程仓消费哲学仓 sih-philosophy/ 的内容。本设计定义消费路径的三个层次与消费边界声明。

## 概览 {#overview}

- 哲学仓是唯一权威指导源，工程层所有操作受其约束::[消费总则](#consumption-general)
- 流衍命题消费路径覆盖道法鉴应元五柱石与复归检验，查 llm-friendly-build::[流衍命题消费路径](#emanation-path)
- convergence 层消费路径覆盖单盲推导体系工程命题，查 KNOW-001::[convergence 层消费路径](#convergence-path)
- 消费边界声明哲学仓已覆盖什么与工程层自行降格映射什么::[消费边界声明](#consumption-boundary)

## 消费总则 {#consumption-general}

哲学仓 sih-philosophy/ 是唯一权威指导源。工程层不修改哲学仓，不伪托工程经验为哲学命题，不把 convergence 层对照命题标注为流衍命题。

哲学仓承载两个层次的内容。第一层是流衍链，含 emanation 流衍段即道法鉴应元五柱石、epistrophe 复归段即落地检测与可归约性检测与可证伪性检测、appendix 链外补充。第二层是 convergence 收敛验证产物，含跨仓对照方法学、单盲推导体系、术语映射、差异分析。

工程层消费哲学仓的方式是只读引用。工程层遇到工程问题时，先检索哲学仓定位对应命题，再按命题的最小展开判断是否需要深入，关键决策前加载哲学原文核验。条目最小展开不替代原文阅读。

## 流衍命题消费路径 {#emanation-path}

流衍命题消费路径覆盖哲学仓的 emanation 流衍段与 epistrophe 复归段与 appendix 链外补充。这是哲学仓的核心命题层，承载道法鉴应元五柱石。

检索入口是 sih-philosophy/llm-friendly-build/。代理遇到判据承载层、命名本体论、治理必要性、规约对齐、验证检验、应对留痕、元层治理、注意力加载、跨域检索类工程问题时，第一动作是查知识包的 mapping.md 定位对应哲学命题 ID，再按 INDEX.md 加载 entries 目录下的条目最小展开判断是否需要深入。

检索步骤四步。第一步识别工程问题类型，把当前遇到的工程问题抽象为一个名词短语。第二步查 mapping.md 左列，按名词短语检索找到对应行，找不到精确匹配时按近义词或上位词扩展检索。第三步加载命题条目，按命题 ID 加载 entries 目录下的对应条目文件，先读话题与最小展开判断是否需要深入。第四步核验，条目最小展开不替代原文阅读，关键决策前必须按原文定位加载哲学原文核验。

知识包覆盖范围。知识包当前收录 emanation 流衍段 22 项条目、epistrophe 复归段 4 项条目、appendix 链外补充 1 项条目，以及 convergence 层的方法学与对照类自证材料索引。知识包不完整，未收录 convergence 层单盲推导体系的工程命题。

## convergence 层消费路径 {#convergence-path}

convergence 层消费路径覆盖哲学仓 convergence 层单盲推导体系的工程命题。单盲推导体系是与流衍链对照验证的自底向上骨架，从工程痛点归纳而上。

检索入口是 sih-engine/doc/knowledge/KNOW-001-convergence-retrieval-map.md。代理遇到翻译链损失、上下文退化、外化管理、偏差检测、信息洪流、可验证性、注意力不对称类工程问题时，llm-friendly-build 检索路径不覆盖，须通过 KNOW-001 回到 convergence 原文定位加载。

检索步骤六步。第一步查 R 原则定义，定位 witness-archive/00-ai-coding-governance-principles.md 第 59 行至第 68 行的十条原则总览表。第二步查命题深度论证，按 R 原则对应的 P 篇章号定位同文件的对应篇章。第三步查哲学骨架对应，定位 witness-archive/03-philosophy-skeleton.md 的对应一级概念章节。第四步查术语映射，定位 convergence/00-terminology-mapping.md 的映射表。第五步查差异与盲区，定位 convergence/03-divergence-analysis.md。第六步查体系综述，定位 convergence/witness-framework.md。

编号体系区分。witness-framework.md 用 L1 标注一级概念、L2 标注二级概念、M 标注元原则。治理原则装订书用 C 标注部、P 标注篇、R 标注原则。两套坐标不可混用，引用规范详见 KNOW-001 编号体系说明节。

## 消费边界声明 {#consumption-boundary}

本节明确哲学仓已覆盖什么与工程层自行降格映射什么，避免代理误以为哲学仓提供工程实现细则。

哲学仓已覆盖的范围。流衍链覆盖治理的为什么与怎么治理，即道法鉴应元的哲学原理与方法论。convergence 层覆盖单盲推导体系的工程痛点诊断与治理原则，即治理什么。两者的收敛验证已完成，核心洞察层收敛率 87.5%。

工程层自行降格映射的范围。convergence 层差异分析第 5 区域明确：emanation 链不做工程展开，每个治理原则需配套可操作实践与可观测信号与可识别反模式，这是工程仓 sih-engine 的降格映射工作。具体降格映射包括：将 P2.1 速度不对称的哲学命题映射为 sih-engine 的产出速率限制机制，将 P3.2 外化管理的三性质映射为事件流的存储格式设计，将 R8 意图锚定原则映射为意图锚定的工程组件协议。

工程层不做的降格映射。工程层不修改哲学命题，不新增哲学命题，不把工程实证经验升格为哲学命题。工程实证经验若反复验证有效，是哲学仓后续演化的候选输入，由哲学仓按自身演化程序决定是否新增独立命题。

验证与鉴的层次区分。差异分析冲突 1 记录：工程层的验证是二元判断，哲学层的鉴含一次穿透即验证与度量同基准。sih-engine 的验证设计若只做二元判断会落入盲区，参验组件须补一次穿透。

## 关联 {#relation}

- 元规则：DEC-001 仓库结构决策
- 通用规范：DES-001 文档格式设计
- 检索路径：KNOW-001 convergence 层检索路径
- 失败复盘：sih-engine/tmp/old-repo-failure-analysis.md
- 启动任务包：sih-engine/tmp/T-001-sih-engine-bootstrap.md
- 哲学仓：sih-philosophy/

## 认识论立场 {#epistemic-stance}

本设计为 design-corollary，是工程设计选择。消费路径的层次划分与检索步骤是工程约定，非哲学必然。哲学仓的内容层次是客观存在的，消费路径是工程层对它的接口设计。

可证伪条件：若哲学仓 llm-friendly-build 后续完整收录 convergence 层，convergence 层消费路径作为独立入口的角色失效，需重新评估是否合并入流衍命题消费路径。

## 自检 {#self-check}

### 形式合规自检 {#formal-self-check}

- 一级标题无锚点，仅一个
- 二级及以上标题均带锚点
- 首个二级标题命名为概览
- 无破折号、无装饰符号、无 Unicode Emoji
- 全角括号仅用于单一治理编号

### 内容自检 {#content-self-check}

- 消费总则明确只读引用，不修改不伪托不升格
- 流衍命题消费路径四步检索承接 llm-friendly-build 现有设计
- convergence 层消费路径六步检索承接 KNOW-001 设计
- 消费边界声明区分哲学仓已覆盖与工程层降格映射，含验证鉴层次区分盲区

### 自反性结论 {#reflexive-conclusion}

本设计定义 sih-engine 消费哲学仓的工程接口。本设计本身经过自我审视，未发现违反 DES-001 格式规范的形态。
