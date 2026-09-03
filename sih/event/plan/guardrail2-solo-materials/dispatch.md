# guardrail2-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 guardrail2-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/guardrail2-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

政策行（用户裁定 2026-09-03 原话照录）：「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械机械链全绿即闭环，得一异常才如实上报。

**本批单飞铁律**：改写入门的批不与他批并发。开工前查 `lease status` 确认零活跃会话，有在途即停手上报。

## 施工（四闸）

1. **闸一同包唯一**：lease open 查 sessions 台账，同包已有 issued 未 revoked 会话即拒 PackageSessionActive 载会话号包名，详情非空。红证即临时双开同包实测被拒。
2. **闸二重意图拒**：scribe intent 增门（重放面承 park 的链目录全量 load 形），同 record 路径已有 intent_refined 在链即拒 IntentRecordUsedRejected；`--allow-reintent` 默认关留补救位（facepark 丢事件重追加即合法通道）。红证即双投同记录实测被拒。
3. **闸三会话在册验**：scribe intent 与 append 认证事件验 --session 在台账为活跃，无或已吊销即拒 SessionNotActive；`--no-session-reason <事由>` 默认关留主会处置位。红证即无会话裸跑实测被拒。
4. **闸四收约预检**：lease close 在现有前置态探针后增 `git merge-tree --write-tree` 逐仓预检，内容冲突即整批拒零动作载冲突文件清单。红证即构造共享追加冲突面实测预检拒且零半程。
5. 单测各闸先红后绿含反例四族加合法通道回归四族（先开后收、--allow-reintent 重追加、--no-session-reason、预检净过放行）；lease 与 scribe 既有测试全绿即向后兼容；CONTRACT 修订与 USAGE 同步两旗标。

## 机械链（照 BATCH-FACE 全序，共享追加面 --mode append 短持）

ask3 记录（三锚引文程序切片禁手打：07-on-assay 纯粹映照、06-on-canon 由松到紧、08-on-settle 应而不藏）→ 双门 → 叩问 elicit check --packs sih-tools/nomenclator/packs/core --words 同包唯一 --words 重意图拒 --words 会话在册验 --words 收约预检 digest passed → 正身 → lease open --package guardrail2-solo --repo 两仓绝对路径（显式 --session）→ 取锁（施工面 exclusive；共享追加面 --mode append 短持）→ meter 包裹引擎 scribe intent 上链 → 工地施工 → 管线（py 与 rs 域外如实记；CONTRACT md 件化格核阅检词）→ 认证逐件 append 主树活链 → 链 settle 前一次性拷工地 → 双仓 settle --seq N --cert 前八位 → 放锁 → close（备份让位归并对表法；本批 close 即闸四首跑实测，读数如实记）→ reconcile → 当日链 verify。

## 冲突模式

本批单飞，预期零撞锁如实记零；开工即见他批在途即停手上报不硬闯。

## 红线

两旗标默认关即缺省收紧；账本 append-only 零改写；既有测试零破坏；主树零直写；守卫在位严禁 plain git commit 直提；findings 亲读；禁管道掩退出码；identity/reports 与既有存量 untracked 零收编。

## 完工报告（最终回复直接输出）

意图哈希、四闸各自红绿证据与反例读数、合法通道回归四族读数、既有与新测试计数、close 首跑预检实测读数、CONTRACT 修订读数、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、越线与误差申报。
