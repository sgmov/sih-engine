# scriwire-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 scriwire-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/scriwire-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md，四件套形制与红绿流程照 /Users/moc/workspaces/SiHankor/sih-engine/sih/event/plan/ordwire-lease-solo-results.md 先例。

政策行（用户裁定 2026-09-03 原话照录）：「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械机械链全绿即闭环，得一异常才如实上报。

## 施工

1. 载体四件套照任务包二节：mapping.md:196 载体引用核验、推导档落 sih-math/docs/（版本偏序即 prev_hash 覆盖关系、append-only 只增不改写、verify 自最小元传递复算）、引擎 src/event_stream/ 判定位注释锚点（append 链接位、verify 复算位、park 重放面位）加 SPEC-004 增载体引用节、金向量两场景（合法追加链 verify 过与断裂链 verify 拒）机械重放。
2. 行为零变更：只加注释与文档节零改链写入与校验逻辑（护栏在役禁自拆），引擎 cargo test --lib 全绿（金向量双红 lim001 与 mul001 为 mathclose-solo 在途件如实记不属本批），changed-files 报告记 wiring_only_no_behavior_change。
3. 三步管线（推导档与 SPEC-004 域内化格核阅检词；rs 件域外如实记），findings 亲读。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片禁手打）→ 双门 → 叩问 elicit check --packs sih-tools/nomenclator/packs/core --words 载体接线 --words 版本偏序 digest passed → 正身 → lease open --package scriwire-solo --repo 三仓绝对路径（一切锁操作显式 --session，allow 一次列全禁静默扩锁）→ 取锁（请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail 主树绝对路径，护栏一在役严禁工地链副本）→ 工地施工 → 管线 → 认证逐件 meter 包裹 append 主树活链 → 链 settle 前一次性拷工地 → 双仓 settle --seq N --cert 前八位 → 放锁 → close（备份让位归并对表法）→ reconcile → 当日链 verify。

## 冲突模式（承 pk-045，用户 2026-09-03 插队裁定在役）

用户裁定原话「链上冲突，可以插队，并不存在太大的冲突点」。执行口径：链追加面插队合法即 trail 锁到手即追加不必等整批收约，尾随读数如实记；共享台账与调用册撞锁有限重试上限十次逐次计数；收约遇 trail 冲突取事件数并集超集且 scribe verify 过为唯一放行形，禁盲取单侧；严禁非书简通道改写链文件（丢事件事故根因）。冲突样本节必载（含插队读数），零撞锁如实记零。

## 锁纪律（2026-09-03 批群死复盘修正，本节优先级高于上文任何长持表述）

上轮四批死于共享面长持锁即插队被憋死。本批起锁分两类：

- **共享追加面即取用即锁、写完即放**：trail 文件（锁 sih-engine/sih/event/trail/2026-09-03.ndjson 文件级，禁锁目录）、sih-tools/scribe/reports/、五本 CALL-LOG、sih-tools/meter/counts/、sih-tools/lease/ledger/、nomenclator core 包。意图上链与逐笔认证各自短持（锁→追加→即放），CALL-LOG 与 meter 逐笔同理。禁批生命周期长持任何共享追加面。
- **施工面锁批内长持照旧**：引擎 src/event_stream/、SPEC-004、sih-math/docs/、批文件。这些需要真互斥。
- 取锁顺序：先施工面后共享面；共享面逐笔取放，撞锁有限重试十次后放掉全部共享面等待重试，施工面可保。

## 红线

只加注释与文档节禁改链逻辑；工具侧 scribe 零碰；主树零直写；守卫在位严禁 plain git commit 直提；findings 亲读；禁管道掩退出码；identity/reports 与既有存量 untracked 零收编。

## 完工报告（最终回复直接输出）

意图哈希、四件套读数、cargo 全量计数与零行为变更证据、金向量双跑、管线三步读数、认证清单、三仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、冲突样本节（含插队实测）、越线与误差申报。
