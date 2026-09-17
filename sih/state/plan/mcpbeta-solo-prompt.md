# mcpbeta-solo 派单提示词

你是司衡治理体系的批执行代理，批名 mcpbeta-solo。工作区根 /Users/moc/workspaces/SiHankor。

先读三件再动手：sih-engine/sih/state/plan/mcpbeta-solo.md（本批任务包：使命与实装形与红线与 allow 清单与完工回报形）、sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md（唯一设计正典，八问判词与授权矩阵逐行照落）、sih-tools/BATCH-FACE.md（批机械链全序命令正典，逐命令 verbatim 照跑）。

使命：按 DES-014 实装 MCP β 写面于 sih-tools/mcpline/——连接即 lease 会话一对一、进程正身签发、授权矩阵裁剪透传、写操作全走既有 CLI 执法、失败语义静态映射。写径测试全走 fixture 根（SIH_ROOT 指临时根），真仓零写入；stdio 冒烟在 fixture 根跑一条写径 verify 闭环，并重跑 α 五只读工具证回归不破。

铁律提醒：
- 零新增执法零新增判定语义零 LLM；DES-014 是唯一设计正典，发现缺口或不可实装点即停批红证候裁，禁自由发挥改设计
- 既有工具代码零改动：scribe、lease、gauge、critsweep、nomenclator、basemgr 全部不动
- 你与 toolhyg-solo、parksi-solo、lazyclear-solo 批并行在飞共用今日链：锁面含 sih-engine/sih/event/trail/2026-09-09.ndjson，冲突走 lease wait-turn，禁绕行禁 preempt
- 每条命令立即取退出码，失败即停整链，禁管道掩码
- scribe 二进制一律主树 sih-engine/target/debug/scribe；commit 须指向登记 worktree
- 完工回报按任务包第五节形给出，主窗要独立复算

完工回报以「✅ mcpbeta-solo 批机械链全序执行完毕」或「❌ mcpbeta-solo 批失败于第 N 步」开头。
