---
name: "sihankor-slash-trigger"
description: "SiHankor governance forced trigger protocol. Invoke when user input starts with `sih ` or `sih,` or `sih：` prefix. Treats `sih` as a single-entry governance trigger that the agent must decompose into Tool calls. Trigger reads natural language description after `sih`, decomposes intent, orchestrates Tool sequence (parallel for independent, sequential for dependent), and executes immediately via MCP tools."
---

# SiHankor Slash Trigger Protocol

用户输入以 sih 加空格、逗号或中文冒号为指令起点的视为强制治理动作触发器。这是司衡工程的入口收敛协议，不是 TRAE 内置命令。

## 边界规则 {#boundary}

触发形式遵循严格边界规则。输入必须以 sih 后接空格、逗号、中文冒号或 ASCII 冒号之一，跟着自然语言描述，才算触发。其他位置出现 sih 不触发，例如：

- input: sih，登记会话开始 → 触发（sih 加中文逗号加空格）
- input: sih: 收工了 → 触发（sih 加冒号加空格）
- input: sih 审稿 → 触发（sih 加空格）
- input: 我 sih 了一下午 → 不触发（sih 不在起点）
- input: supervisor → 不触发（sih 是子串不是起点）
- input: SiH → 不触发（大小写敏感）

这是单字符前缀的弱信号风险的工程化缓解。

## 触发语义 {#trigger-semantics}

sih 是单一入口命令，不存在子命令或 verb 集合。用户输入 sih 后跟一个自然语言描述，agent 自主解析意图、拆解动作、编排 Tool 序列并立即执行。

三问协同：sih 触发到达时，治理动作类意图先过三问载体即 sihankor-intent-refine skill 的全流程四步，再入实质生成，承 DEC-006 说话口接线裁定。

斜杠前缀与自然语言意图相比具有更高优先级。Agent 在接收到 sih 时不应要求用户确认，也不应被上下文否决。

## 行为约束 {#behavioral}

agent 收到 sih 后必须立即解析，不得反问用户「你想调哪个 Tool」。agent 解析失败时应回退到反向校验本会话并询问用户澄清。agent 不应将 sih 视为空跑命令，每次接收都必须产生 trail 行。

## 与隐式映射的关系 {#vs-implicit}

sih 是 Layer 1 显式触发层，intent 隐式映射是 Layer 3 隐式映射层。两层并行存在，互不取代。当用户输入 sih 时，agent 不再走隐式映射推理，直接进入 sih 处理路径。

## 自然语言意图示例 {#examples}

- sih 我要登记会话开始，agent 解析为 record_trail（event_type=session_open）并附加 query_trust_score，可并行发起。
- sih 我要收工，agent 解析为 record_trail（event_type=session_close）并执行反向校验，可并行发起。
- sih 我要拿 foo 文件的编辑权，agent 解析为 acquire_lease。
- sih 我改完了，agent 解析为 release_lease，必须串行等 acquire 返回 lease_id。
- sih 帮我审一下 draft/my-doc.md，agent 解析为 validate_sihmd 与 evaluate_rules，可并行发起。
- sih 我现在的信任分是多少，agent 解析为 query_trust_score。
- sih 请审计一下我的当前会话，agent 扫描所述治理动作与 trail 不一致处追加 inconsistency 行。

这些示例仅展示意图多样性，不构成 verb 集合。

## 并行编排规则 {#parallel}

独立 Tool 指输入不依赖其他 Tool 输出，可并行发起，依赖 Tool 指输出被后续 Tool 用作输入，必须串行执行。

fanout 策略：agent 收到 sih 解析出 N 个独立 Tool 时，一次性发出 N 个 MCP call_tool 请求，然后等所有响应。fanin 策略：所有 Tool 响应到达后聚合结果，按依赖顺序串联，存在依赖关系时。

## 不应披露给用户的细节 {#no-detail}

agent 不应向用户披露：具体的 Tool 名称与参数 schema、是否调用了哪个 Tool 的细节、trail 行格式与字段、Tool 调用编排拓扑。

理由是用户表达治理意图，agent 翻译为治理动作，工程调用为隐式契约。这是司衡哲学中 actor 与 tool 之间的关注点分离。

## 与隐式映射的非对抗性 {#non-replacement}

sih 不取代隐式映射。隐式映射覆盖自然语言意图场景，由 agent 自主推导。sih 覆盖用户显式声明场景，由用户主动标记。这是司衡交互契约的两种正交形式，不冲突。

## 与 AGENTS.md 协议的关系 {#vs-agents}

本文承接 AGENTS.md 中 MCP Tool 调用义务章节下的 sih 协议段。当本 skill 被加载到 system prompt 时，与 AGENTS.md 协议规则一致，agent 应同时遵守两处语义约束。

## 跨工具层定位 {#cross-tool-layer}

权威源在 sih-engine/skills/sihankor-slash-trigger/，通过 symlink 投影到 .agents/skills/ 跨工具发现层。任何默认扫描 .agents/skills/ 的 agent 框架都加载本 skill，不限于单一框架。scripts/ 目录含 gather_identity.sh，用于采集 client 端 identity 提交 MCP server。

## 关键元层命题 {#meta}

sih 单一入口的本质是把 actor 与 tool 之间的关注点分离进一步细化为 actor 与 system prompt 之间的接口契约。用户通过 sih 表达的不是命令而是意图。这是司衡哲学中 actor 与 system 边界自治命题的具体实现。
