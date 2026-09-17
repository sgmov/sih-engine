# recallfix-solo 派单提示词

你是司衡治理体系的批执行代理，批名 recallfix-solo。工作区根 /Users/moc/workspaces/SiHankor。

先读两件再动手：sih-engine/sih/state/plan/recallfix-solo.md（本批任务包：使命与考古判则与红线与 allow 清单与完工回报形）、sih-tools/BATCH-FACE.md（批机械链全序命令正典，逐命令 verbatim 照跑）。

使命：清偿 mem_recall_f_suite f2_refs_machine_verifiable 存量红——检索器引用抽取把 basefix-solo-results.md 第 63 行红申报文本内的引用形字串录为活引用。考古先行定位抽取位，按机械抽取规则修复（红申报行内引用不录活引用或等价规则），终态 cargo test 全目标零败。

铁律提醒：
- 禁改 basefix-solo-results.md 与 pk050sw-solo-results.md 两结果档；禁弱化任何测试断言
- 引擎码批走完整管线与全测试族（cargo test 全目标含慢套件 99 秒）；金向量漂移只许哈希位与抽取输出位 diff，超界停批候裁
- 与主窗 0.9.0 发布执行并行：锁面含 sih-engine/sih/event/trail/2026-09-09.ndjson，冲突走 lease wait-turn，禁绕行禁 preempt
- 每条命令立即取退出码，失败即停整链，禁管道掩码
- scribe 二进制一律主树 sih-engine/target/debug/scribe；commit 须指向登记 worktree
- 完工回报按任务包第五节形给出，主窗要独立复算

完工回报以「✅ recallfix-solo 批机械链全序执行完毕」或「❌ recallfix-solo 批失败于第 N 步」开头。
