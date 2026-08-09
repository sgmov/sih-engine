# 第一段 Prompt 草稿: 知识包构建(沿用 OKF 范式 + 严档元数据)

本规范承载第一段知识包构建 prompt 的元规则。本 prompt 受 DES-001 knowledge-spec 与哲学仓 `llm-friendly-build/` 三段式双重约束。本规范按司衡结构收敛对照的双视角论证,结构性元规则归决策文档,类型特异规范归本规范。

## 概览 {#overview}

- 任务定位:把已 fact-check 的 A 类引用底座构建成可被 KNOW-002 至 008 调研消费的知识包。
- 范式:DES-001 knowledge-spec 加哲学仓三段式,加 Karpathy LLM wiki 的 raw/wiki/schema 三层架构。
- 输入:`sihankor/doc/research/2026-07-30-reference-factcheck/verified.md`(19 条 A 类)。
- 输出:`sihankor/doc/knowledge/llm-friendly-build/` 目录三段式。
- 严档规则:强制 DES-001 元数据预填,引用底座白名单,反凑字黑名单,moc 拍板卡点。
- 元规则:本 prompt 输出文档必须通过 sih-doclint 校验,退出码 0 合规可提交。

## 输入 {#input}

### 引用底座白名单 {#white-list}

- 第一批开源项目三条:DACP(elliot35) / IncodeTechnologies/incode-idv-mcp / Acacian/aegis。
- 第二批框架论文规范概念六条:ArGen 不在内 / Eclipse LMOS ARC / arXiv 2510.24383 Policy Cards / microsoft/agent-governance-toolkit v3.1.0 / MCP 2026-07-28 规范 / Agent Harness 概念 / 指挥官调度官架构。
- NSA 等价替代四条:NIST AI 600-1 / Anthropic Building effective agents / Anthropic Effective context engineering / Anthropic Effective harnesses / arXiv 2404.13501 LLM Agent 记忆机制综述。
- 第四批长尾书七本:博弈论经典 / 隐秩序(原书单误标复杂适应系统) / 工程师思维的跨界创意法(原书单误标工程师的思考法则) / 体系工程理论与方法 / 智能优化算法及其应用 / Agentic Design Patterns(原书单误标 AGENTIC 多智能体架构模式) / 多智能体自主协同技术。

### 降级底座 {#downgrade-list}

- TRiSM 框架,引用 Gartner 2024 原报告。
- OWASP MCP Top 10,引用 OWASP 2025 通用 Top 10 加 MCP Security Best Practices。
- LLM-Agent-Controller,作为领域概念。
- yinxulai/projitive,不引用。
- NSA MCP Guidance,改 NIST AI 600-1 加 Anthropic 系列。

### 黑名单 {#blacklist}

- T/CAMETA 001123-2026 MCP 安全。
- bigmoon-dev/Aegis。
- RB-LLM Control。
- Hybrid Agency Framework。
- Bayesian Social Learning LLM Agents。
- ArGen 框架。
- arXiv 2604.27820 ObjectGraph。

### 范式参考 {#paradigm}

哲学仓三段式在 `sih-philosophy/llm-friendly-build/`,包含 INDEX.md 加 mapping.md 加 entries/。Karpathy LLM wiki 三层是 raw(不可变原始素材)加 wiki(LLM 维护的 markdown 目录)加 schema(约束 wiki 结构的契约)。哲学仓映射法在 `sih-philosophy/llm-friendly-build/mapping.md` 的双源交集法,工程问题映射到哲学命题 ID。

## 输出结构 {#output}

知识包三段式目录如下。

```filetree
sihankor/doc/knowledge/llm-friendly-build/
├── INDEX.md
├── mapping.md
├── log.md
├── entries/
│   ├── KB-001-dacp.md
│   ├── KB-002-incode-idv.md
│   ├── KB-003-aegis.md
│   ├── KB-004-arc-lmos.md
│   ├── KB-005-policy-cards.md
│   ├── KB-006-ms-agt.md
│   ├── KB-007-mcp-spec.md
│   ├── KB-008-harness.md
│   ├── KB-009-cs-model.md
│   ├── KB-010-nist-ai.md
│   ├── KB-011-anthropic-agents.md
│   ├── KB-012-anthropic-context.md
│   ├── KB-013-anthropic-harnesses.md
│   ├── KB-014-llm-mem-survey.md
│   ├── KB-015-game-theory.md
│   ├── KB-016-hidden-order.md
│   ├── KB-017-eng-thinking.md
│   ├── KB-018-sose.md
│   ├── KB-019-opt-algos.md
│   ├── KB-020-agentic-patterns.md
│   └── KB-021-mas-coord.md
└── CHANGELOG.md
```

每个 KB-NNN 必须填的 frontmatter 强制预填,字段清单如下。id 字段值为 KB-NNN-slug 格式字符串。type 字段值为 kb-entry 字符串。status 字段值为 verified 或 downgraded 或 archived 三选一。sources 字段值是 verified.md 引用 ID 数组,可附 arXiv 编号或 GitHub URL 或 ISBN。tags 字段值是具体模块函数决策 ID 数组,不允许抽象词。related 字段值是 DES-001 子文件 ID 或哲学仓 PRO-NNN ID 数组。created 字段值为 YYYY-MM-DD 日期字符串。moc-reviewed 字段值为 pending 或 approved 字符串。

KB-NNN 正文必选章节为概览,关键事实(只列 A 类已核实事实),引用底座(白名单内),跨仓互链,引用约束。

## 严档规则 {#strict-rules}

### 强制 DES-001 元数据预填 {#frontmatter-rule}

每条 KB-NNN 必须填 frontmatter 全部字段,任何字段留空等于 moc 拍板阶段直接拒收。status verified 必须基于 verified.md 白名单。sources 字段只能填 verified.md 已列引用 ID。tags 必须挂具体模块函数决策 ID,例如 MCP-authorization 或 Acacian-aegis-v0.5.0 或 NIST-AI-600-1。

### 引用底座白名单执行规则 {#white-list-exec}

任何引用必须能在 verified.md 19 条 A 类中定位。引用 verified.md 未列引用等于直接拒收。引用降级条目必须显式标注 status downgraded 并显式声明降级原因。

### 反凑字黑名单 {#anti-fluff}

禁止抽象概括范式,例如「某 reference 对应某司衡概念」或「某 reference 与某命题相关」或「某 reference 可用于某场景」。必用具体模块函数决策 ID 范式,例如「某 reference 章节 X 第 Y 段具体定义了某具体函数 Z,该函数 Z 对应司衡某具体治理动作 ID」。

### 哲学仓映射 {#philosophy-mapping}

mapping.md 必须用双源交集法,工程问题或 KB-NNN 条目映射到哲学命题 ID。关键决策前必须按哲学仓 `sih-philosophy/llm-friendly-build/entries/PRO-NNN.md` 加载原文核验。条目最小展开不替代原文阅读。

### moc 拍板卡点 {#moc-checkpoint}

关键决策点(6 方向、引用底座调整、frontmatter 字段增删)必须用 ask_user 工具结构化提问,不允许在对话中列选项让 moc 选。

## 操作流程 {#workflow}

### Step 1 ingest 读源提取更新 KB 条目 {#step-ingest}

读 verified.md 19 条 A 类。每条 reference 提取 1 到 2 句关键事实。写入对应 KB-NNN-slug.md。强制填 frontmatter 全部字段。log.md 追加「## [YYYY-MM-DD] ingest | KB-NNN | 状态」。

### Step 2 query 检索综合 {#step-query}

接受 6 方向调研问题。检索 KB-NNN 条目加 mapping.md。输出 6 份主题 KNOW-002 至 007 加 1 份综合 KNOW-008。log.md 追加「## [YYYY-MM-DD] query | 方向 ID | 结果」。

### Step 3 lint 健康检查 {#step-lint}

跑 si-doclint 工具层审计。grep 全文检查 fabricated.md 7 条黑名单零命中。grep 全文检查每个 KB-NNN frontmatter 字段完整性。log.md 追加「## [YYYY-MM-DD] lint | pass/fail | 违规数」。

## 元规则与边界 {#meta-rules}

本 prompt 受 DES-001 通用格式规范加 knowledge-spec 双重约束。本 prompt 输出文档必须通过 sih-doclint 校验退出码 0。任何 C 类 fabricated.md 引用严禁出现。任何 B 类 suspicious.md 引用必须显式声明降级。6 方向、引用底座增删、frontmatter 字段变更等于 moc 拍板卡点。

## 关联 {#relation}

元规则为 DES-001 knowledge-spec。范式参考为哲学仓 `sih-philosophy/llm-friendly-build/` 三段式。引用底座为 `sihankor/doc/research/2026-07-30-reference-factcheck/verified.md`。黑名单为 `sihankor/doc/research/2026-07-30-reference-factcheck/fabricated.md`。调研 prompt 为本仓 `prompt-v1-research-investigation.md`。

## 认识论立场 {#epistemic-stance}

本 prompt 为 external-anchor(引用 verified.md 19 条 A 类)加 design-corollary(知识包目录结构与 frontmatter 是工程设计选择)混合。

可证伪条件:若知识包失去引用来源即 verified.md 19 条 A 类失效,或 frontmatter 字段失去强制预填价值,则本 prompt 作为知识工程工具失效,需重新评估。
