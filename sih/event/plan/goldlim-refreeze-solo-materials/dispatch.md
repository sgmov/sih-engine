# goldlim-refreeze-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 goldlim-refreeze-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/goldlim-refreeze-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

政策行（用户裁定 2026-09-03 原话照录）：「等我裁的东西首先过得一，得一有问题的异常才让我看」——本批纯机械，机械链全绿即闭环，不扰人节点；得一异常（boundary/不过/工具 exit 2/链守卫事故）才如实上报。

## 施工

1. 现况复算：cargo test --lib 应现 139 过 2 败即 golden_des001mathe_lim001 期望漂移（冻结 f609116bdefd8de346f0fc6114f80557c061ac8cf62f3549f61efc5297b7df71 对盘上 a83b5ba8629611d6f5beb950ba9efb3b04ed69dab425d5d02167e043c5005e83）与 test_evidence_file_path_form 顺序抖动，读数留档。
2. 单点改：src/scrutinator/tests.rs 中该金向量期望 content_hash 刷为盘上实哈希，diff 仅此一处，消费逻辑零改（pendsweep 随冻先例）。
3. live 双跑：cargo test --lib 全绿两跑一致；evidence 测试全量跑复现与否如实记（不改代码，处置另批）。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片禁手打）→ 双门（scrutinator packs/ask3 exit 0 + ask3repeater --root 绝对 exit 0）→ 叩问 elicit check --packs sih-tools/nomenclator/packs/core --words 随冻 --words 期望漂移 digest passed → 正身 identity verify → lease open --package goldlim-refreeze-solo --repo 引擎与工具两仓绝对路径（一切锁操作显式 --session）→ 取锁（请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail 主树绝对路径）→ 工地施工 → 管线（化格→核阅→检词，md 件笔在核前；rs 件域外如实记；findings 亲读）→ 认证逐件 meter 包裹 append 主树活链 → 链 settle 前一次性拷工地 → settle --seq N --cert 链上哈希前八位 → 放锁 → close（备份让位归并对表法）→ reconcile → 当日链 verify。

## 冲突模式（承 pk-045）

撞锁有限重试上限十次逐次计数；冲突样本节必载，零撞锁如实记零；收约前先放锁后拆工地。

## 红线

只刷期望值禁改判定逻辑；主树零直写；守卫在位严禁 plain git commit 直提；findings 亲读；禁管道掩退出码；identity/reports 与既有存量 untracked 零收编。

## 完工报告（最终回复直接输出）

意图哈希、改前改后测试读数对表、tests.rs 单点 diff 证据、live 双跑读数、evidence 抖动观察读数、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、冲突样本节、越线与误差申报。
