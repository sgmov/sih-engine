# mathrefmt2-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 mathrefmt2-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/mathrefmt2-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

政策行（用户裁定 2026-09-03 原话照录）：「等我裁的东西首先过得一，得一有问题的异常才让我看」——本批纯机械，机械链全绿即闭环，不扰人节点；得一异常才如实上报。

## 施工

1. 枚举对表：calculus 概览首节条目盘面应为 51（APP10/HIS16/MUL10/NS3/SER5/SPEC7），与 INDEX 与前缀计数双向对表；与并发批 mathquote-calc-solo 撞锁让位后以当刻盘面重跑枚举再施工。
2. 变换承 mathrefmt-solo 波一逐字同款：概览导航块移除加话题改定义节，正文零改动；每条 diff 删行必须全为锚点导航行（正则全匹配），否则该条停批申报。改后 calculus 114 条全定义首节。
3. 金向量联动（波一教训入判据）：全量 cargo test --lib，本波含 MUL 即 golden_des001mathe_mul001 大概率漂移，凡本波引发的期望漂移随批重冻（只刷期望零改消费逻辑）加 live 全绿双跑，禁止漏报。若 goldlim-refreeze-solo 已先行落链，lim001 应已绿，对表读数。
4. 管线三步逐条（化格→核阅→检词），S005 域内读数，findings 亲读。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片禁手打）→ 双门 → 叩问 elicit check --packs sih-tools/nomenclator/packs/core --words 概览归一 --words 定义首节 digest passed → 正身 → lease open --package mathrefmt2-solo --repo 三仓绝对路径（一切锁操作显式 --session）→ 取锁（请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail 主树绝对路径，护栏一在役严禁工地链副本）→ 工地施工 → 管线 → 认证逐件 meter 包裹 append 主树活链 → 链 settle 前一次性拷工地 → 双仓 settle --seq N --cert 前八位 → 放锁 → close（备份让位归并对表法，diff 非 identical 即停批上报）→ reconcile → 当日链 verify。

## 冲突模式（承 pk-045）

与 mathquote-calc-solo 故意写面重叠即同 51 条目文件并发；撞锁有限重试上限十次逐次计数；冲突样本节必载（冲突点/时点/对方批/机械响应/解决路径/重试计数）；让位后重跑枚举以当刻盘面施工；收约前先放锁后拆工地。

## 红线

只做波一同款变换禁正文语义改写；INDEX 与 VERSION 零触碰；主树零直写；守卫在位严禁 plain git commit 直提；findings 亲读；禁管道掩退出码；identity/reports 与既有存量 untracked 零收编。

## 完工报告（最终回复直接输出）

意图哈希、枚举对表读数、逐条变换与零信息丢失验证、cargo test 全量读数与漂移随冻记录、管线三步读数、认证清单、三仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、冲突样本节（含写面重叠实测）、越线与误差申报。
