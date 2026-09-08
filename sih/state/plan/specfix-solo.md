# specfix-solo 批任务包：SPEC-021 与 SPEC-022 内容充分性节补齐

> 令源：用户 2026-09-09 令「继续。多子代理协作」；队列项，红证源头 mcpspec-solo 批材料 first-run-2026-09-09/（双跑 cmp 逐字节一致存量红两笔）
> 形：solo 批独立立约独立收约，与 mcpsec-solo、toolhyg-solo 并行

## 一、使命

sih-engine/doc/spec/SPEC-021-golden-vector-dual-method.md 与 SPEC-022-envelope-v1.md 各补内容充分性节（照 doc/spec/SPEC-TEMPLATE-sufficiency-v1.md 形），消主树核阅存量红两笔，管线全绿收约。

## 二、做法约束

- 充分性节内容从两档自身正文程序切片取材，禁凭空新立判词；对表行逐行给锚（节锚与文内证据）
- SPEC-023 充分性节为同形先例（三对表行），可参照形不改其文
- 补节属 doc 域修改：化格、核阅、检词序固定；两档改后各自核阅终绿即存量红消解，双跑 cmp 复证
- 认证上链与内容哈希绑定照旧

## 三、写入面（allow 清单）

- sih-engine/doc/spec/SPEC-021-golden-vector-dual-method.md 与 SPEC-022-envelope-v1.md（仅增充分性节）
- sih-engine/sih/state/plan/specfix-solo.md 与 specfix-solo-prompt.md
- sih-engine/sih/event/plan/specfix-solo-results.md 与 specfix-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-specfix-1/

禁触：doc 其余一切文件；两档既有节零语义改动（格式化格归一除外且不得改义）。

## 四、机械链序与铁律

- 全序照 sih-tools/BATCH-FACE.md verbatim；每条命令立即取退出码，失败即停，禁管道掩码
- 并行三批共用今日链：锁面含 trail 路径，冲突走 lease wait-turn，禁绕行禁 preempt
- doc 域格式红线：无围栏代码块、单 H1、首 H2 为概览、禁 U+2192、禁全角括号注内容、禁引用块、命令行内反引号
- scribe 二进制一律主树 target/debug；commit 须指向登记 worktree

## 五、验收

- 两档核阅 rc=0；各自化格 0 检词 0；认证上链、双仓 settle、reconcile 双零
- 完工回报形：批名、座位号、链笔哈希、双仓 commit 哈希、scribe verify 全文、reconcile 增量、两档核阅终值
