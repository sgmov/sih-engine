# mcpcold-solo 批任务包：mcpline α 相冷 agent 验收与结算

> 线：mcpline（MCP 实装线，线程序包 sih-engine/sih/state/plan/mcpline-line-v1.md）
> 令源：用户 2026-09-09 令「你拉起多子代理，进行司衡引擎的mcp开发任务」；线内批一 α 相实装批拆三批承载，本批即其三，前批 mcpspec-solo 与 mcpserv-solo 毕并经主窗复算后开工
> 形：solo 批独立立约独立收约

## 一、使命

把 α 相从「代码在」推进到「线级验收成立」：注册 sih server 入工作区配置，起真冷 agent（零上下文零治理语境）只给任务句，断言其经 MCP 面零辅助完成一次被治理交互且链 verify valid，证据入档，出 α 相结算件落 event/plan。

## 二、验收判据（线级，照录不增删）

冷 agent 零上下文外部 agent 经 MCP 面零辅助完成一次被治理交互且链 verify valid。

## 三、程序形

1. 注册：工作区 ZCode 配置增补 sih server 条目（指向 sih-tools/mcpline 入口，stdio），只增不改，退役 sihankor 条目原样保留；注册形与回滚法记入材料档
2. 冷 agent：新起子代理，prompt 只含任务句与 MCP 工具面可用事实，不含任何治理语境、批名、文件路径教学；任务句形：「用 sih 工具查一下今天的治理链并验证它完整，告诉我结论」
3. 断言：冷 agent 自主调 chain_query 与 chain_verify 成功、未求助、未越界写；主窗侧独立跑 scribe verify 复证 valid
4. 对表：服务器工具面与 SPEC-023 契约表逐项对表（工具名、入参、出参、错误四字段），差异即记
5. 证据：全 transcript、工具调用日志、对表单入 sih-engine/sih/event/plan/mcpcold-solo-materials/
6. 结算件：sih-engine/sih/event/plan/mcpline-alpha-settlement-v1.md，载验收判据、证据指针、五工具读数实录、遗留与 β 前置指针；结算件落 event/plan 即 pk-070 gate 触发条件达成，候裁归人节点

## 四、红线

1. 零辅助：冷 agent prompt 禁出现工具用法示例、参数表、路径指引；工具描述自足即它唯一的教学面
2. 冷 agent 会话零写授权：其读写只经 MCP 面，主窗不复权
3. 失败即如实判 fail：冷 agent 卡死、误用、越界都照录，不修饰；fail 即 α 不结算，批回报 fail 结论候裁
4. 本批不实装任何代码修补：服务器缺陷如实记档回 mcpline 线，不在本批内改 sih-tools/mcpline

## 五、写入面（allow 清单）

- 工作区 ZCode 配置（仅增 sih server 条目一处）
- sih-engine/sih/state/plan/mcpcold-solo.md 与 mcpcold-solo-prompt.md（本包与派单）
- sih-engine/sih/event/plan/mcpcold-solo-results.md 与 mcpcold-solo-materials/ 与 mcpline-alpha-settlement-v1.md
- sih-engine/sih/event/trail/2026-09-09.ndjson（链笔）
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-mcpcold-1/（批机械链侧写）

## 六、机械链序

全序照 sih-tools/BATCH-FACE.md verbatim；每条命令立即取退出码，失败即停，禁管道掩码；scribe 二进制一律主树 target/debug。

## 七、验收

- 冷 agent 验收成立或如实 fail；对表单零差异或差异全记；结算件落档；认证上链、双仓 settle、reconcile 增量合规

## 八、完工回报形

批名、座位号、链笔哈希、双仓 commit 哈希、scribe verify 全文、reconcile 增量、冷 agent verdict（pass/fail）与 transcript 指针、对表单结论、结算件哈希、pk-070 点火申明。
