# mcpsec-solo 批任务包：mcpline β 相写面会话与租约映射安全模型设计

> 线：mcpline（线程序包 sih-engine/sih/state/plan/mcpline-line-v1.md 批二）
> 令源：用户 2026-09-09 令「继续。多子代理协作」；α 相三批俱收约且线级验收 pass 即批二开工条件达成
> 形：solo 批独立立约独立收约，设计批零实装代码

## 一、使命

设计 MCP β 相写面（record 与 lease 写操作经 MCP）的会话与租约映射安全模型，落 sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md（编号按地面实况顺延核对），过得一裁落档，成为 β 相实装批的硬前置正典。

## 二、设计文档必答八问

1. 连接与会话映射：MCP stdio 连接与 lease 会话的生命周期绑定形（连接建立即 lease open 立会话？断开时收约形？孤儿连接与僵尸会话处置？）一对一、多对一、连接池各形取舍须给判词
2. 身份：连接握手时验什么、正身 identity 串 v3 组件十二件怎么绑、谁签发谁复核
3. 授权：工具级授权矩阵，record 与 lease 各写操作分别谁可调；本地可信 agent 与外部冷 agent 是否分级
4. 写路径复用：β 写面必须复用既有执法（scribe 闸二闸三、lease 五验、closeguard、git hooks），禁止另立第二套判定
5. 审计与重放：每笔写已上链的既有事实之上，重放与滥调防护（幂等键、锁冲突语义、wait-turn 排队透出形）
6. 失败语义：报错即教学在写面的边界——教学什么、不泄露什么（路径面、锁面细节、其他会话存在性）
7. 威胁模型：服务器进程被劫持、经工具描述的 prompt injection、恶意 agent 耗尽锁面与台账膨胀，各给缓解或明示不防
8. 边界申明：注入面与自有运行时归 pk-079 不在本设计；GOV-002 与 v3 换版不触碰

## 三、验收

- DES 档八问逐问有判词与依据； facet 得一测量 m-mcpsec-1 九发 stable_clear 加 tally 执契终签上链
- 化格核阅检词认证全管线绿；双仓 settle；reconcile 双零
- 得一不过即如实回报 fail 候裁，禁降格落档

## 四、红线

1. 本批零实装代码：不动 sih-tools/mcpline 与任何既有工具代码
2. β 实装批未过本设计不一裁不开工，本批也不预写实装
3. 判定语义正典留确定性程序：设计不得引入 LLM 裁决位
4. pk-079 与 GOV-002 v3 不触碰

## 五、写入面（allow 清单）

- sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md（新，编号按地面核对）
- sih-engine/sih/state/plan/mcpsec-solo.md 与 mcpsec-solo-prompt.md
- sih-engine/sih/event/plan/mcpsec-solo-results.md 与 mcpsec-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-mcpsec-1/

## 六、机械链序与铁律

- 全序照 sih-tools/BATCH-FACE.md verbatim；每条命令立即取退出码，失败即停，禁管道掩码
- 与 specfix-solo、toolhyg-solo 并行共用今日链：锁面含 trail 路径，冲突走 lease wait-turn，禁绕行禁 preempt
- scribe 二进制一律主树 target/debug；commit 须指向登记 worktree
- facet 测量按 sihankor-facet-measure 与 sihankor-tally skill 的既立流程，emit-contract 零 LLM

## 七、完工回报形

批名、座位号、链笔哈希（intent 与 seat 与 cert 与终签与 exit）、双仓 commit 哈希、scribe verify 全文、reconcile 增量、m-mcpsec-1 判词与终签哈希、DES-014 八问判词摘要。
