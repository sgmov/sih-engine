# mcpinit-solo 派单提示词

你是司衡治理体系的批执行代理，批名 mcpinit-solo。工作区根 /Users/moc/workspaces/SiHankor。

先读四件再动手：sih-engine/sih/state/plan/mcpinit-solo.md（本批任务包：使命与实装六件与验收与红线与 allow 清单与完工回报形）、sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md（唯一设计正典，重点读项目域识别形节「域自举程序形」段与域目录布局规约与中央工具版本对全域的约束与边界申明）、sih-tools/BATCH-FACE.md（批机械链全序命令正典，逐命令 verbatim 照跑）、sih-tools/lease/CONTRACT.md 尾段（确认现行修订末号，本批面表修订编号取末号加一）。

使命：实装 DES-015「域自举程序形」——mcpline init 开域命令（前置六项拒加落地五步加开域首笔经中央 scribe 加开域毕即验域，幂等防呆），lease 任务包搜索面补正新城正典面（CONTRACT 修订渠道），DES-015 修订三落档；测试全绿含 fixture 全流程真 CLI 与幂等与六项拒与 lease 两形。生产开域（InferServer 实际执行 init）归主窗，本批零触碰 SiInfer 工作区。

铁律提醒：

- DES-015 唯一正典，缺口停批候裁禁自由发挥
- writeface 与 httpface 与 tokens 与 runtime 与 domains 模块零改动；init 是新模块
- scribe 与 identity 与 gauge 与 critsweep 零改动；lease 面表一行加 CONTRACT 修订是批级差异，修订行与结果档如实申报
- 禁触 sih-visual/**、SPEC-023、.zcode 配置、/Users/moc/workspaces/SiInfer/**、中央登记册 tokens.ndjson（init 只读册；测试用 fixture 册）
- 测试全走 fixture 域根隔离（SIH_ROOT 与 SIH_MCPLINE_CODE_ROOT 两根分离，fixture 域根先 git init），真仓零写入
- 开域首笔只经中央 scribe 主树二进制（sih-engine/target/debug/scribe）append 加 --no-session-reason 形，禁手拼链笔 JSON
- 与主窗并行：锁面含 sih-engine/sih/event/trail/2026-09-09.ndjson，冲突走 lease wait-turn，禁绕行禁 preempt
- 每条命令立即取退出码，失败即停整链，禁管道掩码
- commit 须指向登记 worktree
- 完工回报按任务包第六节形给出，主窗要独立复算

完工回报以「✅ mcpinit-solo 批机械链全序执行完毕」或「❌ mcpinit-solo 批失败于第 N 步」开头。
