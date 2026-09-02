# pendsweep-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 pendsweep-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/pendsweep-solo.md，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md（坑位注记逐条照办）。背景凭据：用户 2026-09-03 令「先把挂账全清了，得一裁一过一」；本批四件经前置分道均落确定性通道（依据 facet-measure 修订四与 m-sett001-crosscheck 先例），零采样零泊界事件。

## 件要（承任务包 §二，逐件）

1. **件一 tally 透传**：sih-tools/tally/src/tally/cli.py 的 assemble 步骤增读 identity 报告的 identity.core_hash 并载入材料 JSON（旧报告缺席空缺不判败）；R5 判定面 idcore-solo 已就绪勿动其判定语义；tally/tests 增透传用例与 R5 回放同判不回归用例；tally pytest 全绿计数入结果档。
2. **件二 金向量随冻**：sih-engine/src/scrutinator/fixtures/golden/ 全部 12 件逐件对表——以现行引擎核阅二进制对各件 targets 实跑，与冻结 findings/content_hashes 比对；漂移件逐件查漂移出处（目标文件 git log 定位改动批，已结算批合法改动即重录：只更新期望 findings 与 content_hashes，文件结构与断言消费逻辑零改，承 pk-036 deyimerge-switch 先例），无合法出处即停批报告不硬重录；重录后 cargo test 全绿；对表读数与重录清单 cmp 证据入结果档。
3. **件三 校准窗钉死**：sih-tools/gauge/CONTRACT.md 追加修订一条（编号递增）：ga-1 校准窗满条件即结算日 2026-09-02 后累计七个含例行读数的席位日或用户令提前；触发链四步即窗满、解析校准尝试、解析不可行、pk-042 出泊复检走得一裁过即实验批。
4. **件四 投影同步**：sih-engine/doc/governance/PARKING-v1.md 与 sih-tools/PARKING-v1.md 两名册 as-found 修正——pk-043 已出泊（promoted，intanchor-solo，2026-09-03）与撞号披露照录；pk-041 行补记出泊裁定已在案（mathpipe-full-program-v1.md）与记账位主会裁定（跨天 park 走合并视图通道，承 pk031gap 与 pk013exit 先例，mathpipe-a1 批执行时记账）；pk-042 行补记触发链；pk-036 行按件二实态如实更新；名册修正后与链上停泊事件逐条对表留证。**零泊界事件写入**——本批只修投影文档，进出泊动作一件不做。

## 机械链（照 BATCH-FACE 全序，本批参数如下）

ask3 记录（session sess-zcode-260903-pendsweep；三锚引文程序切片禁手打：06-on-canon.md 第 185 行、07-on-assay.md 第 61 行、08-on-settle.md 应而不藏所在行）→ 双门（scrutinator --pack packs/ask3 退出码 0 + ask3repeater --root 绝对路径退出码 0）→ elicit check --words 校准窗 --words 触发链 --words 投影同步 各词重复传入加 digest passed → 正身 verify（报告 sih-tools/identity/reports/2026-09-03-pendsweep-identity.json，不入册；报告自带 core_hash 即件一价值的活体旁证）→ lease open --package pendsweep-solo --repo 两仓绝对路径 --root /Users/moc/workspaces/SiHankor（一切锁操作显式 --session；撞锁即停批报告）→ 取锁按任务包请求写入节逐路径（显式枚举禁花括号展开）→ meter 包裹引擎 scribe intent 上链（--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-03.ndjson 绝对路径；链现 12 事件 valid 尾哈希 0c43f8225219406c 开头，settle 前后 wc -l 与末哈希对表留证；并行在途会话尾随承先例如遇即申报）→ inputlog 补录一笔（sih-engine/sih/event/inputlog/2026-09-03.ndjson，seq 递增，sess-zcode-260903-acceptor，逐字）：`先把挂账全清了，得一裁一过一`（note 即挂账清理令与分道申报）→ 工地施工：全部改动在 worktrees/sih-tools/pendsweep-solo 与 worktrees/sih-engine/pendsweep-solo，主树零直写；测试从工地树跑 → 管线（笔在核前：化格逐 md --write；核阅引擎二进制 --pack des-001 裸名——两名册在 doc/governance 属域内必须零违规，结果档与任务包域外 exit-2 如实记；检词 --pack packs/core 逐 md 零违例；findings 亲读禁管道掩退出码）→ 认证逐件 meter 包裹 scribe append → 双仓 settle --seq 1 --cert certification_completed 哈希前八位 → 放锁 → close（备份让位归并对表法 diff identical 留证）→ reconcile 双仓四类双零 → 当日链 verify → 调用册留痕（触及工具各一行）→ 结果档 sih-engine/sih/event/plan/pendsweep-solo-materials/pendsweep-solo-results.md（F 表、分道申报节、越线与误差申报、队形声明）→ 全部产物含链尾随批入版控。

## 红线

泊界事件零写入（进出泊唯人节点，本批零触碰停泊写入位）；pk-041/043/044 泊件本体与 mathpipe-a1 在途批零碰；AGENTS.md 与 identity 源码零碰；金向量只刷期望输出断言逻辑零改；v3 金标 82f460c2ac2741fc 不回归（identity verify 实跑值断言在案）；R5 判定语义与回退同判零改动；退出码三值零动；直提守卫在位严禁 plain git commit；scope 显式枚举禁花括号展开；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即停批；meter --quiet 位照 BATCH-FACE 坑位注记。

## 完工报告（最终回复直接输出）

意图哈希、F-1 至 F-5 逐项结论（tally 测试计数、金向量 12 件对表读数与重录清单、CONTRACT 修订号、名册对表读数）、词债登记清单、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、越线与误差申报、正身报告 core_hash 在场确认。
