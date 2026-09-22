# DEC-027 exitenvelope 归并状态裁定

本裁定处置 src/exitenvelope.rs 的双轨悬置：模块头注声称归并位与参数化承载位已在职，而全仓调用点为零，gauge、lease、critsweep、attnanchor 四 bin 仍各保留本地副本。令源即用户 2026-09-22 得一裁继续推进；建议源即迭代建议报告 REC-002 的 A 轨建议与 auditfix 序列分析，报告在档 sih/event/plan/auditfix-materials/report-original-backup.md。本裁定承 REC-002 的双轨二选一框架落单一轨位。

## 概览 {#overview}

- 问题即共享退出封套头注与代码事实漂移，归并承诺悬置无主::[问题](#problem)
- 裁定择 A 轨完成归并，B 轨撤回不取::[裁定](#verdict)
- 理由三条即意图层与执行层对位、头注可信度可机械复验、audit 链 traceback 可解释::[理由](#reasons)
- 执行约束即执行批次排融回线收口之后，对表基线不破两次::[执行约束](#execution)
- 事实注记即 attnanchor 副本非逐字节一致，施工前先重铸字节基线::[事实注记](#facts)
- 状态即已裁候执行，候融回线收口::[状态](#status)

## 问题 {#problem}

头注声称
: src/exitenvelope.rs 头注声称 critsweep、gauge、lease 三 bin 的 sort_json、emit、die 副本归并于此，meter 的 die_uncaught 与 calllogtool 的 die_value_error 以 die_plain 参数化承载，attnanchor 同承此位。

调用点事实
: 全仓调用点为零，grep 仅命中 src/lib.rs 的 re-export 自身；gauge、lease、critsweep、attnanchor 四 bin 仍各保留字节级本地副本。归并承诺与代码事实两轨悬置，认定与双轨提案载于迭代建议报告 REC-002。

## 裁定 {#verdict}

A 轨
: 择定。四 bin 改为引用引擎共享模块，meter 与 calllogtool 的变签 die 族以 die_plain 签名适配，exitenvelope 头注与代码事实对齐，报文格式逐字节不变。

B 轨
: 不取。撤回归并承诺、模块下沉仅测试可见、删除库面 re-export 的退让轨不取；归并意图已立，不以撤回代执行。

## 理由 {#reasons}

意图层与执行层对位
: exitenvelope 的家位与头注是已立的执行层意图，A 轨使代码事实追上意图层，B 轨使意图迁就残留副本，两轨唯 A 轨恢复对位。

头注可信度可机械复验
: 归并完成后，头注声称与调用点事实可由 grep 一类机械手段复验，文档与代码漂移类弱项归零，后续维护者按头注信任不付考古成本。

audit 链 traceback 可解释
: 报文发射单源后，bin 侧退出报文的 traceback 链条可解释可追踪，审计从报文到实现位一站可达。

## 执行约束 {#execution}

执行批次排融回线收口之后
: A 轨执行批次排在 DEC-013 融回门的租约融回加快照切换批收口之后。理由：gauge、critsweep、attnanchor 三件正处引擎位同参双跑对表在役，A 轨先动会使对表基线破两次。

## 事实注记 {#facts}

attnanchor 副本非逐字节一致
: gauge、lease、critsweep 三件副本与共享实现逐字节一致；attnanchor 副本无 die 函数，sort_json 写法差一处全限定形式。A 轨施工前须先重铸字节基线，对齐后归并，报文行为零漂移为准。

## 状态 {#status}

状态
: 已裁候执行，候融回线收口。裁定已立，执行批次候融回线收口后排定。

## 关联 {#relation}

建议源即迭代建议报告 REC-002 与 auditfix 序列分析，报告在档 sih/event/plan/auditfix-materials/report-original-backup.md；执行约束上游即 DEC-013 融回门；共享模块家位即 src/exitenvelope.rs；令源即用户 2026-09-22 得一裁继续推进。
