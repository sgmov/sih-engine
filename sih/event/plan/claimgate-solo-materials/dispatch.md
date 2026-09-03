# claimgate-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 claimgate-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/claimgate-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

政策行（用户裁定 2026-09-03 原话照录）：「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械机械链全绿即闭环，得一异常才如实上报。

**本批单飞铁律**：改 lease 的批不与他批并发。开工前查 `lease status` 验零活跃会话，有在途即停手上报。

## 施工（领取登记）
1. 领取账本 sih-tools/lease/ledger/claims.ndjson append-only（package、claimant、claimed_at、ttl_minutes、status 五字段），账本即追加锁面。
2. 子命令：`lease claim --package X --ttl 120 [--claimant 标识]` 同包未过期再领即拒 PackageAlreadyClaimed 载领取人与到期时刻（声明位拒绝 exit 1 详情非空）；`lease unclaim --package X` 释放；`lease status` 增 claims 视图即在领与过期标注；过期在判定时自动标 expired。
3. 闸一联动：lease open 成功时同包有他人未过期领取即在输出载警示行（不拒，硬执法归闸一）。
4. 单测先红后绿：双领拒、过期放行、释放后再领、status 视图、open 联动警示五族；既有测试全绿向后兼容；CONTRACT 修订与 USAGE 同步；金向量即双领拒与过期放行两场景双跑。

## 机械链
ask3 记录（三锚引文程序切片禁手打）→ 双门 → 叩问 elicit check --packs sih-tools/nomenclator/packs/core --words 领取登记 --words 领取账本 digest passed → 正身 → lease open --package claimgate-solo --repo 两仓绝对路径（显式 --session）→ 取锁（施工面 exclusive；共享追加面 --mode append 短持）→ meter 包裹引擎 scribe intent 上链（--trail 主树绝对 --sessions 台账路径）→ 工地施工 → 管线（py 件域外如实记）→ 认证逐件 append 主树活链 → 链 settle 前一次性拷工地 → 双仓 settle --seq N --cert 前八位 → 放锁 → close（备份让位归并对表法）→ reconcile → 当日链 verify。

## 冲突模式
本批单飞，预期零撞锁如实记零；开工即见他批在途即停手上报不硬闯。

## 红线
领取是声明位非执法位禁抢闸一职责；账本 append-only 零改写；既有测试零破坏；主树零直写；守卫在位严禁 plain git commit 直提；findings 亲读；禁管道掩退出码；identity/reports 与既有存量 untracked 零收编。

## 完工报告
意图哈希、五族红绿证据、金向量两场景双跑、CONTRACT 修订读数、既有与新测计数、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、越线与误差申报。
