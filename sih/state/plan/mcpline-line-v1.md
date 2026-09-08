# mcpline 线任务包 v1：MCP 实装线（α 相只读面先行）

> 线：mcpline（MCP 实装线，pk-077 出泊 promoted 即立项，用户 2026-09-09 令「得一裁」承得一测量 m-mcpopen-1 九发 stable_clear 执契终签 05353825）
> 令源：「得一裁」即三相形态与冷 agent 验收判据照录入线；自有运行时归 pk-079 分相承载不并入本线
> 形：单线 solo 逐批，每批独立立约独立收约

## 一、问题陈述 {#problem}

- 治理交互面现由 ZCode 会话内 agent 承载，零上下文外部 agent 无标准接入面；MCP 系跨工具标准但拉式即 agent 调用才有数据（pk-077 进泊语境在档）。
- AGENTS.md MCP 节归档载功能承载映射即 record_trail 由书简上链承接、validate_sihmd 由核阅承接、会话反查由 scribe query 加 meter crosscheck 承接、租约由 sih-tools/lease 承接；工作区 .zcode/config.json 存退役 sihankor server 即 enabled false。
- α 相只读面可先行即零写入零治理语义；写面须安全模型设计先行；注入面通用化终局即自有运行时归 pk-079 分相承载。

## 二、三相形态 {#phases}

**α 相只读面先行**

- 工具面：链查询、验链、判据扫、心跳、锁面读数包为 MCP 工具。
- 语义：零写入零治理语义；工具描述自足与报错即教学。
- 边界：MCP 面不新增第二个执行者，判定语义正典留确定性程序；读数只读投影面。

**β 相写面候前置**

- 工具面：record 与 lease 写操作经 MCP。
- 前置条件：先过会话与租约映射安全模型设计批，未过不开工。

**自有运行时分记**

- 注入面通用化终局即自有对话循环归 pk-079 分相承载（其分相四段之首即 α MCP 面），不并入本线。

## 三、批次拆解 {#batches}

**批一 α 相实装批（候开）**

- α 相五读数工具 MCP 化与冷 agent 验收，契约与实装细节在该批任务包定稿，本件只记形态不涉具体实装设计。

**批二 β 相安全模型设计批（α 相毕后）**

- 会话与租约映射安全模型设计，过得一裁落档；β 相开工的硬前置。

**批三 β 相实装批（候批二毕）**

- record 与 lease 写操作经 MCP，按批二设计落档实装；各批独立立约独立收约走完整机械链。

## 四、跨批红线 {#redlines}

- α 相零写入零治理语义：MCP 工具面不新增第二个执行者，判定语义正典留确定性程序
- β 相写面先过会话与租约映射安全模型设计批，未过不开工
- 自有运行时归 pk-079 分相承载，本线不并入
- GOV-002 与 v3 换版候人裁，本线不触碰

## 五、验收（线级） {#acceptance}

- 冷 agent 形：零上下文外部 agent 经 MCP 面零辅助完成一次被治理交互且链 verify valid

## 六、必读文件 {#read}

- sih-engine/sih/state/parking/materials/pk-077.json 与 pk-077-exit.json（立项令源与出泊裁决）
- sih-engine/sih/event/plan/settlement-v2-solo-results.md（pk-077 gate 触发达成读数节）
- sih-engine/sih/event/plan/mcpopen-solo-results.md（立项登记批结果档）
- sih-tools/BATCH-FACE.md（批机械链命令面正典）

## 七、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/mcpline-line-v1.md（本件线序登记，随 mcpopen-solo 批提交）
- 后继批逐批另立任务包申报写入面，本件不预占实装位
