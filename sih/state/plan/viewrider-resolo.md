# viewrider-resolo：viewrider-solo 范围闸漏列后接续

> task-packages 治理任务
> 承接：viewrider-solo 段1 提交后段2 触发范围闸拦（lease/CALL-LOG.md、lease/ledger/locks.ndjson、lease/ledger/sessions.ndjson 三件漏列）
> 队形：单线 solo
> 日期：2026-08-31

## 一、问题陈述

viewrider-solo 段1 已 commit 336851c（sih-engine 工地：tests/cli_multitrail.rs + 任务包 + 结果档）。段2 段3 因 lease/ledger/* 与 lease/CALL-LOG.md 漏列 allow 触发 staged_out_of_scope。本批接续：撤销段1 单独 commit、重开 lease 扩 allow、双仓重 commit、close 归并。

## 二、关键设计

一撤销 + 重开 + 重 commit：段1 已 commit 撤销，lease 重开 allow 补 lease/CALL-LOG.md + lease/ledger/sessions.ndjson + lease/ledger/locks.ndjson 三件，sih-engine 与 sih-tools 双仓工地重新 staged 走 lease commit 提交。

## 三、可证伪条件

| F | 判据 |
|---|---|
| F-1 | 范围闸不拦 |
| F-2 | 双仓 commit 落 msh/viewrider-resolo 分支 |
| F-3 | 链 valid |

## 四、约束

88c6b4a 零触碰、viewer.rs 零改、src/view/ 零改、全程租约零直写。

## 五、请求写入

- sih-engine/sih/state/plan/viewrider-resolo.md
- sih-engine/tests/cli_multitrail.rs
- sih-engine/sih/event/plan/viewrider-solo-results.md
- sih-engine/sih/state/plan/viewrider-solo.md
- sih-engine/sih/event/trail/2026-08-31.ndjson
- sih-tools/scribe/reports/（15 件 viewfix）
- sih-tools/lease/CALL-LOG.md
- sih-tools/lease/ledger/locks.ndjson
- sih-tools/lease/ledger/sessions.ndjson
- sih-tools/meter/counts/2026-08-31.ndjson
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md

## 六、叩问处置

- 叩问处置[viewrider-resolo]: 消解 即本批名 viewrider-resolo 范围闸漏列后接续
- 叩问处置[范围闸漏列]: 消解 即工作名直述即 lease allow 列表漏列承接主树台账件
- 叩问处置[撤销段1]: 消解 即工作名直述即 git reset --soft HEAD~1 撤销段1 commit 336851c
