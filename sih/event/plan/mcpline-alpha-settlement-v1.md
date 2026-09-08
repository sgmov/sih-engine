# mcpline α 相结算件：冷 agent 线级验收成立

> 线：mcpline（MCP 实装线）；批：mcpcold-solo（线内批三，前批 mcpspec-solo 与 mcpserv-solo 俱收约）
> 令源：用户 2026-09-09 令「你拉起多子代理，进行司衡引擎的mcp开发任务」；对表正典 SPEC-023
> 结算判定：α 相线级验收成立（cold agent verdict pass）。本件落 sih-engine/sih/event/plan/ 即 pk-070 gate 触发条件达成，候裁归人节点

## 一、验收判据与判定

- 判据（线程序包第五节照录）：冷 agent 零上下文外部 agent 经 MCP 面零辅助完成一次被治理交互且链 verify valid
- 判定：成立。冷 agent（会话 sess_b8dccf1f-1130-46db-bac4-b1f8288cc5e0，prompt 只含任务句与可用事实）自主调 chain_query 与 chain_verify 各一笔，判词 valid，零求助零越界写；主窗侧独立 scribe verify 复证 valid 同读数（74 笔，首 25989caf 末 8d1e2340）

## 二、证据指针

- 冷 agent transcript：mcpcold-solo-materials/cold-transcript-extract.json（工具名与入参与出参与终答全录）与 cold-rollout-model-io.jsonl（两笔模型请求 verbatim）与 cold-run-cli-response.json（CLI 结构化回执）
- 起跑形与零写授权机制：mcpcold-solo-materials/cold-launch-record.json；prompt 正典：sih/state/plan/mcpcold-solo-prompt.md
- 注册只增不改证据：mcpcold-solo-materials/registration-and-rollback.md 加 zcode-config-before/after-image.json 双影像
- 零写入证明：mcpcold-solo-materials/gitstatus-engine-before/after.txt 与 gitstatus-tools-before/after.txt（双仓 status 逐字节全等）加 verify-prebatch.json 与 verify-postcold.json
- 契约对表：mcpcold-solo-materials/spec-023-contract-compare.md（五工具与错误四字段与红线逐项，零实质差异）

## 三、五工具读数实录（stdio 面，2026-09-09）

读数全档：mcpcold-solo-materials/stdio-five-tool-readings.json（tools/list 加逐工具调用加错误探针，生成器 make_stdio_readings.py 随档）。

- chain_query：date 2026-09-09，matches 74，事件清单含哈希与事件型与主体字段
- chain_verify：date 2026-09-09，status valid，valid true，首 25989caf 末 8d1e2340
- critsweep：date 2026-09-09，严格 JSON 单对象，五判据俱 achieved、parking 双线零告警、inflight 一会话十锁（恰为本批在飞实态）
- heartbeat：三维 convergence 0.125 与 adoption 0.0 与 mergeback 0.028571（computed_at 2026-09-09，sequence ok），days_since_last_snapshot 0
- locks_read：held_count 10（本批十锁逐一对成对），active_sessions 1
- 错误探针两笔：chain_query 与 chain_verify 各喂非法 date，载荷恰四字段（error 与 what_this_tool_does 与 valid_params 与 canonical_pointers）

## 四、α 相遗留

- 出参超集申明：chain_query 出参信封字段（date/trail/matches/returned）系契约核心三字段外补充，对表单判超集不判差异
- 遗留跟踪件：sih-tools/lease/reports/sweep-latest.json 系 2026-09-08 critsweep 开发期遗留跟踪件，与本线无关，本批前后零变化，如实申明候人节点处置
- 工作区 AGENTS.md 在场事实：冷 agent 环境自动投影 AGENTS.md（含治理语境与路径），系 ZCode 装载器行为非本批 prompt 输入；冷 agent 实际仅经 MCP 面两工具完成，未触 AGENTS.md 内任何路径或 CLI 形。外部 agent 完全零语境形态归 pk-079 自有运行时承载
- CLI 帮助面漂移：zcode 0.16.5 帮助面宣示的 --allowed-tools 与 --max-turns 与 --settings 三旗标解析器拒识（bundle 零命中），零写授权由 --disallowed-tools 单旗标承载；漂移事实记入 cold-launch-record.json 候工具线处置

## 五、β 相前置指针

- β 相写面开工硬前置：会话与租约映射安全模型设计批（线程序包批二），未过不开工（SPEC-023 红线三）
- 写面范围预告：record 与 lease 写操作经 MCP；写授权与冷 agent 会话的租约映射、MCP 面身份与锁归属是设计批核心问题
- 自有运行时（注入面通用化与完全零语境外部 agent 形态）归 pk-079 分相承载，不并入本线
- pk-070（工程基线五条正典居所迁离 AGENTS.md）：本结算件落 event/plan 即其 gate 触发条件达成，正典落位裁决候人节点
