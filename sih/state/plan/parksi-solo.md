# parksi-solo 批任务包：SiInfer 彩排泊置入泊（pk-080）

> 令源：用户 2026-09-09 原话「司衡引擎的mcp做好了再上，到时候是那边的agent接入，有问题我会回司衡窗口报，你现在不要管SiInfer」
> 形：solo 微批独立立约独立收约，与在飞四写批共用今日链走租约排队

## 一、使命

将 SiInfer 彩排按用户裁定泊置：新立 sih-engine/sih/state/parking/materials/pk-080.json（进泊件）与 doc/governance/PARKING-v1.md 册行（行形照地面现有行实况），scribe park 停泊笔落链，走完批机械链收约。

## 二、pk-080 内容定稿（照此入泊）

- entry_id：pk-080；title：SiInfer 彩排泊置——MCP 线毕候令，接入形为彼侧 agent
- context：令源照录用户原话；前情三行——只读踏勘已毕归档会话（要点：第一口候选 InferServer/hidream-ft-taste-model.md 已与司衡语义接壤、三源结构须锚 commit hash、SiInfer 侧 .vscode/mcp.json 接入未挂）；红线：彩排前司衡侧对 SiInfer 零动作，彼侧 agent 接入后故障由人节点回司衡窗口报
- gate：trigger 即 mcpline 线批三 β 实装收约结算件落 sih-engine/sih/event/plan/（用户「MCP 做好了」的机械判读）；declared_at 2026-09-09
- exit_condition：人节点裁彩排开工（立彩排批，SiInfer 全程只读加咬死 commit hash 加彼侧 agent 接入形）或裁搁置闭项出泊；gate 触发亦不出泊，唯人节点
- related：["pk-077"]；source：主会 2026-09-09 会话用户裁定；ttl_days：90；parking.entered_at：2026-09-09

## 三、红线

1. 本批对 SiInfer 工作区零访问零读取零写入：泊置是司衡侧裁定档，踏勘已止
2. PARKING-v1.md 属 des-001 域：只增册行不改他行，化格核阅检词序固定
3. 与在飞 mcpsec、specfix、toolhyg、release09 四批共用今日链：锁面含 trail，冲突走 lease wait-turn，禁绕行禁 preempt

## 四、写入面（allow 清单）

- sih-engine/sih/state/parking/materials/pk-080.json（新）
- sih-engine/doc/governance/PARKING-v1.md（只增册行）
- sih-engine/sih/state/plan/parksi-solo.md 与 parksi-solo-prompt.md
- sih-engine/sih/event/plan/parksi-solo-results.md 与 parksi-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-parksi-1/

## 五、机械链序与完工回报形

- 全序照 sih-tools/BATCH-FACE.md verbatim；每条命令立即取退出码，失败即停，禁管道掩码；scribe 二进制一律主树 target/debug；commit 须指向登记 worktree
- 完工回报：批名、座位号、链笔哈希（含 park 笔）、双仓 commit 哈希、scribe verify 全文、reconcile 增量、pk-080 册行与进泊件路径
