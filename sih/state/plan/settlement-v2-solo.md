# settlement-v2-solo 任务包：主线 v2 结算落链批

> 令源：用户 2026-09-09 令「批准」即 SETTLEMENT-V2-2026-09-09.md 经批准生效；承同日令「出结算单」草案已过化格核阅检词管线三步全绿。
> 队形：单线 solo，主窗后台拉起子代理执行，主窗独立复算。批日 2026-09-09，会话号 sess-zcode-260909-main-settlev2。

## 范围四腿 {#scope}

1. 结算件认证：SETTLEMENT-V2-2026-09-09.md 经内容哈希清单件绑定认证上链即 manifest 载结算单与批件 sha256。
2. 落链全序：批机械链走完即意图笔、认证笔、双仓 settle、unlock、close、reconcile；结算单随批提交入版控。
3. 结果档落 event/plan：sih-engine/sih/event/plan/settlement-v2-solo-results.md 含大白话节——本件落位即 pk-077 的 gate 触发条件即「主线 v2 结算件落 sih-engine/sih/event/plan/」机械达成，批内读数呈报该达成；pk-077 材料零触碰零改 fired_at 即出泊唯人节点，点火事实以链与结果档承载候人裁。
4. 收口读数：链 verify 即 2026-09-09 当日链、双仓 reconcile、判据扫复算五判据俱 achieved 终读。

## 产出 {#outputs}

任务包本件与提示词件（主窗预落）、认证事件号、结算单入版控提交号、结果档、意图链笔、ask3 记录与验证件、正身件、租约与锁实录、大白话节、投影件 settlement-v2.json 认证上链、双仓 settle 提交号、链 verify 与 reconcile 读数、越线与误差申报。

## 边界与红线 {#redlines}

- GOV-002 节点换版 v3 不入本批即候人裁先例。
- 结算单文本零改动即已批准形逐字入版控，管线读数照录已绿三步。
- 在泊件材料含 pk-077 零触碰；pk-079 零触碰。
- 主树零直写（两件预落与结算单预置除外）；链文件只经引擎 scribe 写位；禁管道掩退出码；误差红证如实记档。
