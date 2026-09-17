# release09-solo 批任务包：0.9.0 首发布工程——SemVer 晋升判据 DEC 与发布清单与数学仓推送卫生

> 令源：用户前令接受 0.9.0 首发布（测试味定位）与 1.0.0 机械晋升判据方案；用户 2026-09-09 令「继续。多子代理协作」
> 形：solo 批独立立约独立收约，与 mcpsec-solo、specfix-solo、toolhyg-solo 并行

## 一、使命三件

**件一 SemVer 晋升判据 DEC**：sih-engine/doc/decision/015-semver-release-v1.md（编号对地面实况核对顺延），载三节：0.9.0 语义（首发布即测试味版本，α 相只读 MCP 面在列）；1.0.0 机械晋升判据（外部冷 agent 经 MCP 面零辅助跑通成立、陌生人零帮助按指南走查成立、发布后修复节奏建立）；治理语义 SemVer 定约（MAJOR 即治理语义破坏、MINOR 即判定语义增量、PATCH 即执法面修补）。

**件二 0.9.0 发布清单**：sih-engine/doc/plan/release-0.9.0-v1.md，逐项列发布内容面（引擎五件套与 sih-tools 工具带与 mcpline 服务器与 SPEC-023 与视图仓指针）、已知边界（β 写面未开、GOV-002 v3 候裁）、回滚法；清单内打 tag 步骤标注「归主窗终验后执行」，本批不打 tag 不发布。

**件三 数学仓推送卫生**：sih-math/.gitignore 只增不删（排除运行残留与缓存类），服务数学仓推送远程的前置状态；推送动作本身不在本批。

## 二、红线

1. 本批不打 tag 不推送不发布：三件都是文档与卫生件，执行位归主窗终验后
2. 不动 mcpline、lease、critsweep、gauge、scribe 代码；不动 doc/design、doc/spec（在飞批域）
3. DEC 与清单属 des-001 域：化格核阅检词认证序固定
4. 1.0 判据必须机械可验：每条判据附判定命令或材料指针，禁「感觉成熟」形

## 三、写入面（allow 清单）

- sih-engine/doc/decision/015-semver-release-v1.md（新）
- sih-engine/doc/plan/release-0.9.0-v1.md（新）
- sih-math/.gitignore（只增）
- sih-engine/sih/state/plan/release09-solo.md 与 release09-solo-prompt.md
- sih-engine/sih/event/plan/release09-solo-results.md 与 release09-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-release09-1/

## 四、机械链序与铁律

- 全序照 sih-tools/BATCH-FACE.md verbatim；每条命令立即取退出码，失败即停，禁管道掩码
- 并行四批共用今日链：锁面含 trail 路径，冲突走 lease wait-turn，禁绕行禁 preempt
- doc 域格式红线：无围栏代码块、单 H1、首 H2 为概览、禁 U+2192、禁全角括号注内容、禁引用块、命令行内反引号
- scribe 二进制一律主树 target/debug；commit 须指向登记 worktree

## 五、验收与完工回报形

- 三件管线全绿、认证上链、双仓 settle（math 仓随批或单独 commit 均可但须留痕）、reconcile 双零
- 完工回报：批名、座位号、链笔哈希、各仓 commit 哈希、scribe verify 全文、reconcile 增量、三件各自终值与路径
