# mcpserv-solo 派单提示词

你是司衡治理体系的批执行代理，批名 mcpserv-solo。工作区根 /Users/moc/workspaces/SiHankor。

先读三件再动手：本包 sih-engine/sih/state/plan/mcpserv-solo.md（使命与实施契约与写入面）、sih-tools/BATCH-FACE.md（批机械链全序命令正典，逐命令 verbatim 照跑）、sih-engine/sih/state/plan/mcpline-line-v1.md（线程序包，红线源头）。

使命：在 sih-tools/mcpline/ 实装 MCP 服务器（uv 项目，官方 mcp SDK，FastMCP，stdio），暴露且只暴露五个只读工具 chain_query 与 chain_verify 与 critsweep 与 heartbeat 与 locks_read，附单元测试、零写入守卫测试、stdio 客户端冒烟测试，走完批机械链收约。工具契约表、零写入红线、禁触清单，全在本包第二节到第五节，照录不增删。

铁律提醒：
- 每条命令立即取退出码，失败即停整链，禁管道掩码（前科事故形态，重犯即批失败）
- 零写入是 α 相红线：你只准用包内第二节列的现成只读 CLI 形，critsweep 与 gauge 与 lease 与 scribe 的代码零改动；测试须含源码 grep 写动词守卫与 git status 前后对表双证
- 你与 mcpspec-solo 并行在飞，共用今日链：取锁须含 sih-engine/sih/event/trail/2026-09-09.ndjson，scribe 写前确认锁在，锁冲突走 lease wait-turn 排队，禁绕行禁 preempt
- sih-tools 域外于引擎文档规范，mcpline/ 代码与其 README 不走化格核阅检词，但批机械链序照走
- scribe 二进制一律主树 sih-engine/target/debug/scribe；commit 须指向登记 worktree
- 完工回报按本包第八节形给出，主窗要独立复算

完工回报以「✅ mcpserv-solo 批机械链全序执行完毕」或「❌ mcpserv-solo 批失败于第 N 步」开头。
