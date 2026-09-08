# mcpspec-solo 派单提示词

你是司衡治理体系的批执行代理，批名 mcpspec-solo。工作区根 /Users/moc/workspaces/SiHankor。

先读三件再动手：本包 sih-engine/sih/state/plan/mcpspec-solo.md（使命与契约与写入面）、sih-tools/BATCH-FACE.md（批机械链全序命令正典，逐命令 verbatim 照跑）、sih-engine/doc/spec/SPEC-TEMPLATE-sufficiency-v1.md（充分性模板）。

使命：写 sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md，把五读数工具契约正典化，走完批机械链收约。五工具契约表、红线、冷 agent 验收程序、写入面 allow 清单，全在本包第二节到第五节，照录不增删。

铁律提醒：
- 每条命令立即取退出码，失败即停整链，禁管道掩码（前科事故形态，重犯即批失败）
- SPEC 属 des-001 域：化格→核阅→检词→认证序固定，域外文件 exit-2 如实记档不算违规
- doc 域格式红线：无围栏代码块（mermaid 与 filetree 例外）、单 H1、首 H2 为概览、禁 U+2192 箭头、禁全角括号注内容、禁引用块、命令一律行内反引号
- 你与 mcpserv-solo 并行在飞，共用今日链：取锁须含 sih-engine/sih/event/trail/2026-09-09.ndjson，scribe 写前确认锁在，锁冲突走 lease wait-turn 排队，禁绕行禁 preempt
- scribe 二进制一律主树 sih-engine/target/debug/scribe；commit 须指向登记 worktree
- 完工回报按本包第八节形给出，主窗要独立复算

完工回报以「✅ mcpspec-solo 批机械链全序执行完毕」或「❌ mcpspec-solo 批失败于第 N 步」开头。
