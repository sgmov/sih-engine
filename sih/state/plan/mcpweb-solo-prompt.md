# mcpweb-solo 派单提示词

你是司衡治理体系的批执行代理，批名 mcpweb-solo。工作区根 /Users/moc/workspaces/SiHankor。

先读两件再动手：sih-engine/sih/state/plan/mcpweb-solo.md（本批任务包：使命与设计钉与验收与红线与 allow 清单与完工回报形）、sih-tools/BATCH-FACE.md（批机械链全序命令正典，逐命令 verbatim 照跑）。

使命：mcpline 增统一 web 服务形态——单端口（缺省 8765）根路径承载视图面板静态页、/mcp 路径承载 MCP streamable HTTP 面；HTTP 面只暴露 α 五只读工具（安全红线：HTTP 无进程身份，未认证写面不得上网络口）；stdio 十五具面零回归；mcpline 版本 0.2.0 改 0.3.0 测试族全跑。

铁律提醒：
- 禁触 sih-visual/**（静态目录只读引用）与既有工具代码与 SPEC-023 与 .zcode 配置
- HTTP 冒烟五断言俱验：GET / 得面板、tools/list 恰五、chain_verify valid、β 工具缺席、stdio 回归加零写证明
- 每条命令立即取退出码，失败即停整链，禁管道掩码
- 与主窗发布收尾并行：锁面含 sih-engine/sih/event/trail/2026-09-09.ndjson，冲突走 lease wait-turn，禁绕行禁 preempt
- scribe 二进制一律主树 sih-engine/target/debug/scribe；commit 须指向登记 worktree
- 完工回报按任务包第六节形给出，主窗要独立复算

完工回报以「✅ mcpweb-solo 批机械链全序执行完毕」或「❌ mcpweb-solo 批失败于第 N 步」开头。
