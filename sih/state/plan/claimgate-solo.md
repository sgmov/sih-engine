# claimgate-solo：任务包领取登记批（领取账本 + lease claim/unclaim + ttl 过期）

> task-packages 治理任务
> 承接：用户 2026-09-03 提出原话照录「我在想是否任务包被领取也加个锁？」；主会判词即闸一同包唯一拦二次立约、闸三拦裸跑上链，而领取到立约之间（ask3 叩问正身段）有碰撞真空，领取登记补可见性与提前避撞；闸一到三仍为硬执法，领取登记为声明位非执法位
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械即机械链全绿即放行
> 队形：单线形 solo，**单飞**（改 lease 的批不与他批并发，开工前验零活跃）；scribe gated 写入带 --sessions
> 日期：2026-09-04（包立 2026-09-03，实日以本行为准：trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-*）｜ 温故检索：materials/recall-claimgate.json 双档零命中如实记 ｜ pk-045 参与者

## 一、问题陈述 {#problem}

委外领包到 lease open 之间无任何登记，双领同一包要到立约才被闸一拦，前置准备白干一遍；主会与用户也无「哪些包被谁在领」的即视图。

## 二、关键设计 {#design}

1. **领取账本**：sih-tools/lease/ledger/claims.ndjson append-only，字段即 package、claimant（会话预备号或委外自报标识）、claimed_at、ttl_minutes、status（claimed/released/expired）。账本属追加面即锁面用 append 模式，零互斥成本。
2. **两子命令**：`lease claim --package X --ttl 120 [--claimant 标识]` 即登记，同包已有未过期领取即拒 PackageAlreadyClaimed 载领取人与到期时刻（声明位拒绝非执法拒绝，exit 1 详情非空）；`lease unclaim --package X` 即释放。`lease status` 增 claims 视图（在领、过期即标注）。
3. **ttl 过期**：status 与 claim 判定时过期即自动标 expired，僵尸领取不靠人清。
4. **闸一联动**：lease open 成功时若同包有他人未过期领取即如实载入 open 输出警示行（不拒，硬执法仍归闸一）。
5. 单测先红后绿：双领拒、过期放行、释放后再领、status 视图、open 联动警示；既有测试全绿向后兼容；CONTRACT 修订与 USAGE 同步；金向量即双领拒与过期放行两场景双跑。

## 三、F 表 {#falsifiable}

| F | 判据 |
|---|---|
| F-1 双领拒 | 同包未过期再领即拒载详情 |
| F-2 ttl 过期 | 过期后可再领且旧账目标 expired |
| F-3 status 视图 | 在领与过期标注可见 |
| F-4 兼容 | lease 既有测试全绿 |

## 四、必读 {#read}

sih-tools/lease/src/lease/（open 与 status 位）；guardrail2-solo-results.md 闸一先例；BATCH-FACE.md

## 五、请求写入 {#requested-writes}

sih-tools/lease/ ｜ sih-engine/sih/state/plan/claimgate-solo.md ｜ sih-engine/sih/event/plan/claimgate-solo-results.md ｜ sih-engine/sih/event/plan/claimgate-solo-materials/ ｜ sih-engine/sih/event/trail/<实日>.ndjson ｜ sih-tools/scribe/reports/ ｜ sih-tools/scribe/CALL-LOG.md ｜ sih-tools/lease/CALL-LOG.md ｜ sih-tools/meter/counts/ ｜ sih-tools/nomenclator/packs/core/

## 六、验收 {#acceptance}

F-1 至 F-4 全过；金向量两场景双跑；单飞零并发如实记；认证入链双仓结算收约对表在档。
