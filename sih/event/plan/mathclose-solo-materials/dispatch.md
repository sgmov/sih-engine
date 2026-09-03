# mathclose-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 mathclose-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/mathclose-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

政策行（用户裁定 2026-09-03 原话照录）：「等我裁的东西首先过得一，得一有问题的异常才让我看」——本批纯机械，机械链全绿即闭环不扰人节点；得一异常才如实上报。

## 件一：括注扫尾（必须先做）

1. 枚举脚本：对 sih-math 自基线 0eab3ae（mathrepo-v1）以来的四波提交 a4372c1（mathrefmt）、e4d5724（mathquote2）、bd60afd（mathquote-calc）、56e90c8（mathrefmt2）逐文件归因，产出改写条目清单，预期 147 件即 calculus 113 加四子仓 34，APP-011 零触碰。清单入 materials 可 grep 复算。
2. 逐件状态行改「已建，哲学到工程桥梁条目（<波名> 增补）」，多波波名以「加」连接，波名取批名去 -solo。只动状态行一行，正文零改，逐件 diff 验证。
3. 三步管线逐件（化格→核阅→检词），findings 亲读。
4. **时序留证**：件一完成后记录 wc 与尾哈希与 LIM-001 加 MUL-001 盘上实哈希读数，此读数即件二重冻依据。倒序即停批。

## 件二：金向量重冻（件一完成后）

1. 取件一完成后的 LIM-001 与 MUL-001 盘上实哈希，刷 sih-engine/src/scrutinator/fixtures/golden/des-001-mathe-lim001.json 与 des-001-mathe-mul001.json 各恰一行 content_hash，消费逻辑零改（pk-036 与 pendsweep 随冻先例）。
2. cargo test --lib 全绿两跑一致（现况 139 过 2 败即此双件，重冻后应 141 过 0 败 6 忽略；evidence 测试顺序抖动单跑绿属已知另案如实记）。

## 件三：主会 riders 入库

主树现未提交件逐字节 identical 入库零改字：sih-engine/doc/governance/PARKING-v1.md（七项名册与 pk-046 金向量方法论行与 pk-047 OTel 行含撞号披露）、sih/state/parking/materials/pk-046.json 与 pk-047.json、sih-tools/lease/ledger/ 活写件。入库前逐件备份对表，非 identical 即停批上报（这些是主会处置件，一个字不许动）。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片禁手打：07-on-assay 镜即纯粹映照、06-on-canon 由松到紧、08-on-settle 应而不藏）→ 双门 → 叩问 elicit check --packs sih-tools/nomenclator/packs/core --words 括注扫尾 --words 随冻 digest passed → 正身 → lease open --package mathclose-solo --repo 三仓绝对路径（一切锁操作显式 --session）→ 取锁（请求写入节逐路径，一次列全）→ meter 包裹引擎 scribe intent 上链（--trail 主树绝对路径，护栏一在役严禁工地链副本）→ 工地施工（件一→件二→件三序不可倒）→ 管线 → 认证逐件 meter 包裹 append 主树活链 → 链 settle 前一次性拷工地 → 双仓 settle --seq N --cert 前八位 → 放锁 → close（备份让位归并对表法）→ reconcile → 当日链 verify。

## 冲突模式（承 pk-045，用户 2026-09-03 插队裁定在役）

用户裁定原话「链上冲突，可以插队，并不存在太大的冲突点」。执行口径：链追加面插队合法即 trail 锁到手即追加不必等整批收约，尾随读数如实记；共享台账与调用册撞锁有限重试上限十次逐次计数；收约遇 trail 冲突取事件数并集超集且 scribe verify 过为唯一放行形，禁盲取单侧；严禁非书简通道改写链文件（facepark 丢事件事故根因）。冲突样本节必载（含插队读数，零撞锁如实记零）；收约前先放锁后拆工地。

## 锁纪律（2026-09-03 批群死复盘修正，本节优先级高于上文任何长持表述）

上轮四批死于共享面长持锁即插队被憋死。本批起锁分两类：

- **共享追加面即取用即锁、写完即放**：trail 文件（锁 sih-engine/sih/event/trail/2026-09-03.ndjson 文件级，禁锁目录）、sih-tools/scribe/reports/、五本 CALL-LOG、sih-tools/meter/counts/、sih-tools/lease/ledger/、nomenclator core 包。意图上链与逐笔认证各自短持（锁→追加→即放），CALL-LOG 与 meter 逐笔同理。禁批生命周期长持任何共享追加面。
- **施工面锁批内长持照旧**：数学仓条目目录、fixtures/golden/ 两件、PARKING 名册、泊材料、批文件。这些需要真互斥。
- 取锁顺序：先施工面后共享面；共享面逐笔取放，撞锁有限重试十次后放掉全部共享面等待重试，施工面可保。

## 红线

件一件二时序倒置即停批；allow 面一次列全禁静默扩锁（goldlim 教训）；riders 件零改字；主树零直写；守卫在位严禁 plain git commit 直提；findings 亲读；禁管道掩退出码；identity/reports 与既有存量 untracked 零收编。

## 完工报告（最终回复直接输出）

意图哈希、件一枚举计数与括注读数与时序留证、件二重冻前后哈希对表与 cargo 双跑读数、件三 riders identical 对表、管线三步读数、认证清单、三仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、冲突样本节、越线与误差申报。
