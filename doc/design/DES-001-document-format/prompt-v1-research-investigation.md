# 第二段 Prompt 草稿: 调研(6 方向主题 KNOW 加 1 份综合 KNOW-008)

本规范承载第二段调研 prompt 的元规则。本 prompt 受 DES-001 knowledge-spec 加第一段 prompt 双重约束。本规范按司衡结构收敛对照的双视角论证,结构性元规则归决策文档,类型特异规范归本规范。

## 概览 {#overview}

- 任务定位:按 6 个调研方向组织知识包加综合报告,验证司衡 LLM wiki 能否跑通,治理严谨度是否优于 OKF。
- 范式:DES-001 knowledge-spec 加哲学仓三段式,加第一段 prompt 输出的 KB-NNN 知识包。
- 输入:第一段 prompt 产物 `sihankor/doc/knowledge/llm-friendly-build/`,KB-001 至 021 加 verified.md 19 条 A 类。
- 输出:`sihankor/doc/knowledge/KNOW-002` 至 `KNOW-007` 主题方向加 `KNOW-008` 综合,加 `INDEX.md` 加 `log.md`。
- 双轴时间范围:基础理论 1990-2020 哲学仓承接,现代工业实践 2020-2026 工程仓承接。
- 严档规则:强制 DES-001 元数据预填,引用底座白名单,反凑字黑名单,报告与知识包互索引,moc 拍板卡点。
- 元规则:本 prompt 输出文档必须通过 sih-doclint 校验,退出码 0 合规可提交。

## 输入 {#input}

### 第一段 prompt 产物知识包 {#stage1-output}

第一段 prompt 产物在 `sihankor/doc/knowledge/llm-friendly-build/`。INDEX.md 是内容目录。mapping.md 是引用底座到哲学命题 ID 映射。entries 目录 KB-001 至 021 是 19 条 A 类加 4 条 NSA 等价替代的最小展开。log.md 是时序日志。

### 引用底座白名单 19 条 A 类 {#white-list}

第一批开源项目三条: DACP, elliot35 开源。IncodeTechnologies/incode-idv-mcp,身份验证。Acacian/aegis,MCP 治理代理。第二批框架论文规范概念六条: Eclipse LMOS ARC,Agent Runtime Contract。arXiv 2510.24383,Policy Cards 论文。microsoft/agent-governance-toolkit v3.1.0,微软治理工具包。MCP 2026-07-28 规范,Anthropic 第 5 版。Agent Harness 概念,Salesforce 与 Google 与 OpenAI 阵营。指挥官调度官架构,Autogen 与 LangGraph 与 Agentforce。

NSA 等价替代四条加一篇综述: NIST AI 600-1,GenAI 风险管理画像。Anthropic Building effective agents,2024-12-19。Anthropic Effective context engineering,2025-09-29。Anthropic Effective harnesses,2025-11-26。arXiv 2404.13501,LLM Agent 记忆机制综述。

第四批长尾书七本: 博弈论经典,纳什沙普利等。隐秩序,Holland。工程师思维的跨界创意法,韦青推荐序。体系工程理论与方法,张维明等。智能优化算法及其应用,王凌或张生财版。Agentic Design Patterns,Antonio Gullí 2025-10。多智能体自主协同技术,吴傑宏 2024。

### 降级底座 B 类五条 {#downgrade-list}

TRiSM 框架,引用 Gartner 2024 原报告。OWASP MCP Top 10,引用 OWASP 2025 通用 Top 10 加 MCP Security Best Practices。LLM-Agent-Controller,作为领域概念。yinxulai/projitive,不引用。NSA MCP Guidance,改 NIST AI 600-1 加 Anthropic 系列。

### 黑名单 C 类七条 {#blacklist}

T/CAMETA 001123-2026 MCP 安全。bigmoon-dev/Aegis。RB-LLM Control。Hybrid Agency Framework。Bayesian Social Learning LLM Agents。ArGen 框架。arXiv 2604.27820 ObjectGraph。

## 6 方向(已锁定,2026-07-30 moc 拍板) {#six-directions}

按理论体系路线,6 方向锁定为 B 加 C 加 D 加 F 加 G 加 H 组合(从 A-H 候选中剔除候选 A 治理架构与候选 E 留痕,本批次不覆盖)。

### 方向 B 命名标识与 Identity {#direction-b}

承接哲学仓 PRO 01 立名本体论加命名失败加血统判定。时间轴 1990-2020 涵盖 Incode IDV 加公钥基础设施加 X.500,2020-2026 涵盖 LLM Agentic Identity 加 IncodeTechnologies/incode-idv-mcp。引用底座为 Incode IDV MCP 加 arXiv 2510.24383 中 identity 部分。

### 方向 C 跨域知识吸收与 Knowledge Absorption {#direction-c}

承接哲学仓知识吸收过程加 OKF wiki 范式。时间轴 1990-2020 涵盖 OWL 加本体工程加 Knowledge Graph,2020-2026 涵盖 Karpathy LLM Wiki 加 Anthropic context engineering。引用底座为 NIST AI 600-1 加 Anthropic Building effective agents 加 Anthropic Effective context engineering。

### 方向 D 鉴验证与 Verification and Validation {#direction-d}

承接哲学仓 PRO 06-07 鉴加三要素加司衡 PRO-07 鉴。时间轴 1990-2020 涵盖形式化方法加 Isabelle 加 Coq 加测试理论,2020-2026 涵盖 LLM eval 加 AgentBench 加 SWE-bench 加 Harness。引用底座为 Agent Harness 加 arXiv 2510.24383 中 Policy Cards 验证机制加多智能体自主协同技术。

### 方向 F 多智能体协同与 Multi-Agent Coordination {#direction-f}

承接哲学仓 PRO 04-05 加司衡多治。时间轴 1990-2020 涵盖 MAS 经典加博弈论加拍卖理论,2020-2026 涵盖 AutoGen 加 LangGraph 加 Agentforce 加指挥官调度官。引用底座为 Eclipse LMOS ARC 加指挥官调度官架构加 Agentic Design Patterns 加多智能体自主协同技术。

### 方向 G 形式化与体系工程 {#direction-g}

承接哲学仓 PRO 02-03 加工程仓启动方法论 TOGAF 加 CMMI。时间轴 1990-2020 涵盖 TOGAF 加 CMMI 加体系工程理论,2020-2026 涵盖 MCP Apps 加 MCP Tasks 加现代 MAS 体系。引用底座为 MCP 2026-07-28 规范加体系工程理论与方法加工程师思维的跨界创意法。

### 方向 H 优化算法与复杂适应 {#direction-h}

承接哲学仓 CAS 七要素加 Holland 隐秩序。时间轴 1990-2020 涵盖遗传算法加蚁群加粒子群加隐秩序,2020-2026 涵盖 LLM 与优化算法融合。引用底座为智能优化算法及其应用加隐秩序加博弈论经典。

## 输出结构 {#output}

### KNOW 文档目录 {#know-tree}

```filetree
sihankor/doc/knowledge/
├── KNOW-001-projects-references.md
├── KNOW-002-<方向1>-<slug>.md
├── KNOW-003-<方向2>-<slug>.md
├── KNOW-004-<方向3>-<slug>.md
├── KNOW-005-<方向4>-<slug>.md
├── KNOW-006-<方向5>-<slug>.md
├── KNOW-007-<方向6>-<slug>.md
├── KNOW-008-cross-cutting.md
├── index.md
└── log.md
```

每份 KNOW-NNN 必须填的 frontmatter 强制预填,字段清单如下。id 字段值为 KNOW-NNN 字符串。type 字段值为 know-research 字符串。direction 字段值为方向 ID 字符串。axis 字段值含 foundation 1990-2020 与 practice 2020-2026 两个子字段。status 字段值为 draft 或 moc-reviewed 或 archived 三选一。sources 字段值是 verified.md 引用 ID 数组,可附哲学仓 PRO-NNN ID 或第一段 prompt KB-NNN ID。tags 字段值是具体模块函数决策 ID 数组。related 字段值是其他 KNOW-NNN ID 数组。created 字段值为 YYYY-MM-DD 日期字符串。moc-reviewed 字段值为 pending 字符串。

每份 KNOW-NNN 正文必选章节为概览,外部引用按知识 spec 引用样式链接汇总至文末,经验沉淀为本次调研沉淀,术语映射按定义列表格式含术语加定义加来源三要素,双轴时间范围对比含 1990-2020 加 2020-2026,引用底座按 KB-NNN 索引,跨仓互链含 KB-NNN 加哲学仓 PRO-NNN 加跨 KNOW-NNN,引用约束。

### KNOW-008 综合报告特别要求 {#cross-cutting}

KNOW-008 跨方向对比矩阵采用 TOGAF 4 维度加 CMMI 5 级,具体定义如下。

TOGAF 4 维度是指 The Open Group Architecture Framework 的 4 大架构域。

- Business Architecture 业务架构层:对应方向 C 跨域知识吸收与 H 优化算法与 CAS,评估业务需求与组织能力匹配度。
- Data Architecture 数据架构层:对应方向 B 命名标识,评估元数据与命名本体论的覆盖度。
- Application Architecture 应用架构层:对应方向 D 鉴验证与 F 多智能体协同,评估应用层抽象与协同机制的成熟度。
- Technology Architecture 技术架构层:对应方向 G 形式化与体系工程,评估底层形式化方法与体系工程工具的支撑能力。

CMMI 5 级是指 Capability Maturity Model Integration 的 5 级成熟度模型。

- Level 1 Initial 初始级:过程不可预测,反应式。
- Level 2 Managed 已管理级:过程按项目级管理。
- Level 3 Defined 已定义级:过程按组织级标准化。
- Level 4 Quantitatively Managed 定量管理级:过程被度量与控制。
- Level 5 Optimizing 优化级:过程持续改进与创新。

每份主题 KNOW-002 至 007 必须在结尾给出 TOGAF 4 维度对位表(哪个方向对应哪几个维度,匹配度评估)与 CMMI 5 级自评(本方向当前处于哪一级,差几级到下一级,关键瓶颈是什么)。

KNOW-008 综合报告必须包含 6 方向 TOGAF 4 维度横向对比矩阵加 6 方向 CMMI 5 级纵向成熟度评估加司衡 LLM wiki 跑通验证对照 Karpathy 三层架构加三操作加两特殊文件加司衡优于 OKF 论证加关键发现加风险加 moc 拍板窗口。frontmatter 必须显式声明 direction cross-cutting。

### 报告与知识包互索引 {#cross-ref}

每份 KNOW-NNN 必须含 related 字段,显式指向其他 KNOW-NNN。KNOW-008 必须含 6 份 KNOW-002 至 007 的全索引。INDEX.md 必须含 8 份 KNOW 的总目加主题交叉引用表。

## 严档规则 {#strict-rules}

### 强制 DES-001 元数据预填 {#frontmatter-rule}

每份 KNOW-NNN 必须填 frontmatter 全部字段,任何字段留空等于 moc 拍板阶段直接拒收。status moc-reviewed 必须由 moc 手动签字。sources 字段只能填 verified.md 19 条 A 类加哲学仓 PRO-NNN 加 KB-NNN。axis 字段必须显式声明双轴 1990-2020 加 2020-2026。

### 引用底座白名单执行规则 {#white-list-exec}

任何引用必须能在 verified.md 19 条 A 类中定位。引用 verified.md 未列引用等于直接拒收。引用降级条目即 B 类 5 条必须显式标注 status downgraded 并显式声明降级原因。

### 反凑字黑名单 {#anti-fluff}

禁止抽象概括范式,例如「某 reference 对应某司衡概念」或「某 reference 与某命题相关」或「某 reference 可用于某场景」。必用具体模块函数决策 ID 范式,例如「某 reference 章节 X 第 Y 段具体定义了某具体函数 Z,该函数 Z 对应司衡某具体治理动作 ID」。任何「对应」「相关」「可用于」等抽象概括必须改写为「具体模块加函数加决策 ID」加「物理动作」加「反推可证伪」。

### KNOW 必选章节 {#required-sections}

每份 KNOW-NNN 必须满足 WRITING-KNOW-001 至 004 全部四条 CI 校验。WRITING-KNOW-001 为必选章节三类之一。WRITING-KNOW-002 为外部引用必须用引用样式链接汇总至文末。WRITING-KNOW-003 为术语映射必须用定义列表格式。WRITING-KNOW-004 为术语表必须含术语加定义加来源三要素。

### 工具层静态审计 {#tool-audit}

每份 KNOW-NNN 必须通过 si-doclint 校验,退出码 0。校验失败等于任务失败,不允许跳过。

### moc 拍板卡点 {#moc-checkpoint}

6 方向已锁定为 B 加 C 加 D 加 F 加 G 加 H 组合(2026-07-30 拍板)。KNOW-008 跨方向对比维度已锁定为 TOGAF 4 维度加 CMMI 5 级(2026-07-30 拍板)。关键决策点包括引用底座增删与 frontmatter 字段变更与每份 KNOW 的 TOGAF 维度对位与 CMMI 等级自评,必须用 ask_user 工具结构化提问。

## 操作流程 {#workflow}

### Step 1 ingest 读知识包起 KNOW 骨架 {#step-ingest}

读第一段 prompt 产物的 INDEX.md 加 mapping.md 加 entries 目录。选定 6 方向,从 A 至 H 候选清单。为每方向起草 KNOW-NNN skeleton,即必选章节加 frontmatter 占位。强制填 frontmatter 全部字段。log.md 追加「## [YYYY-MM-DD] ingest | KNOW-NNN | skeleton」。

### Step 2 query 检索综合 {#step-query}

按 6 方向逐份填充 KNOW-NNN。双轴时间范围对比。跨方向对比矩阵写入 KNOW-008。log.md 追加「## [YYYY-MM-DD] query | 方向 ID | 字数」。

### Step 3 lint 健康检查加 moc 拍板窗口 {#step-lint}

跑 si-doclint 工具层审计,退出码 0 等于合规。grep 全文检查 fabricated.md 7 条黑名单零命中。grep 全文检查每份 KNOW frontmatter 字段完整性。grep 全文检查每份 KNOW 必选章节存在性。moc 拍板用 ask_user 工具结构化校验每个引用是否在白名单。log.md 追加「## [YYYY-MM-DD] lint | pass/fail | 违规数」。

## 元规则与边界 {#meta-rules}

本 prompt 受 DES-001 knowledge-spec 加第一段 prompt 双重约束。本 prompt 输出文档必须通过 sih-doclint 校验退出码 0。任何 C 类 fabricated.md 引用严禁出现。任何 B 类 suspicious.md 引用必须显式声明降级。6 方向加跨方向对比维度加 frontmatter 字段变更等于 moc 拍板卡点。6 方向加 KNOW-008 等于 7 份 KNOW,覆盖 moc 拍板的 1 份综合报告加 6 份主题 KNOW。综合报告 KNOW-008 等于司衡 LLM wiki 跑通验证加治理严谨度对比矩阵加关键发现。

## 关联 {#relation}

元规则为 DES-001 knowledge-spec。范式参考为哲学仓 `sih-philosophy/llm-friendly-build/` 三段式。上游输入为本仓 `prompt-v1-knowledge-pkg.md` 产物。引用底座为 `sihankor/doc/research/2026-07-30-reference-factcheck/verified.md`。黑名单为 `sihankor/doc/research/2026-07-30-reference-factcheck/fabricated.md`。降级为 `sihankor/doc/research/2026-07-30-reference-factcheck/suspicious.md`。工具层为 `sihankor/tools/doclint/target/release/sih-doclint`。

## 认识论立场 {#epistemic-stance}

本 prompt 为 external-anchor(引用 verified.md 19 条 A 类加哲学仓 PRO-NNN)加 design-corollary(6 方向划分加 frontmatter 字段是工程设计选择)混合。

可证伪条件:若 KNOW-008 综合报告失去跨方向对比价值,或 6 方向失去对司衡工程仓核心治理层的覆盖,则本 prompt 作为调研工具失效,需重新评估方向划分。
