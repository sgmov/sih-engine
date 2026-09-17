# mcpbeta-solo 批任务包：mcpline β 相写面实装

> 线：mcpline 线程序包批三；令源用户「继续。多子代理协作」与线序推进
> 前置已达成：批二 mcpsec-solo 落 DES-014 且 m-mcpsec-1 九发 stable_clear 执契终签 1763a223 在链
> 形：solo 批独立立约独立收约，与在飞写批共用今日链走租约排队

## 一、使命

按 DES-014（sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md，sha256 fa7f2cc5）八问设计正典，实装 MCP β 写面：连接与会话一对一、进程正身签发、路由层授权矩阵、写操作透传既有执法零新增判定、失败语义静态映射。DES-014 是本批唯一设计正典，本包不另立契约；实装与设计冲突处停批红证候裁，禁自由发挥改设计。

## 二、实装形

- 服务器扩展仍在 sih-tools/mcpline/，stdio，与 α 五只读工具同进程共存；写工具集与授权矩阵逐行照 DES-014 第三节矩阵落
- 写操作一律透传既有 CLI 执法（scribe append 与 intent 与 park、lease 子命令），服务器零新增判定语义零新增执法；矩阵只做裁剪
- 连接级 lease open 的意图门衔接照 DES-014 第一问与第三问判词实装；冷 agent 最小写集按矩阵
- 测试隔离：fixture root 形——SIH_ROOT 指临时根，工场化 trail 与台账与链件，单测逐矩阵行加逐闸透传；stdio 冒烟在 fixture 根实跑一条写径（写后 verify 闭环）；真仓零写证明（测试套件前后双仓 git status 全等断言）

## 三、红线

1. 零新增执法零新增判定语义零 LLM：设计不得引入、实装更不得引入
2. 既有工具代码零改动：scribe、lease、gauge、critsweep、nomenclator、basemgr 全部不动
3. 对真实双仓零写入：所有写径测试走 fixture 根；真仓只读
4. 发现 DES-014 缺口或不可实装点：停批红证候裁，本批无权修设计
5. 与在飞 toolhyg、parksi、lazyclear 批共用今日链：锁面含 trail，冲突走 lease wait-turn，禁绕行禁 preempt

## 四、写入面（allow 清单）

- sih-tools/mcpline/**（含 tests 扩展）
- sih-engine/sih/state/plan/mcpbeta-solo.md 与 mcpbeta-solo-prompt.md
- sih-engine/sih/event/plan/mcpbeta-solo-results.md 与 mcpbeta-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-mcpbeta-1/

## 五、机械链序与验收

- 全序照 sih-tools/BATCH-FACE.md verbatim；每条命令立即取退出码，失败即停，禁管道掩码；scribe 二进制一律主树 target/debug；commit 须指向登记 worktree
- 验收：全测试绿含 fixture 写径冒烟闭环与零写证明；α 五只读工具回归不破（冒烟重跑 α 五工具）；认证上链、双仓 settle、reconcile 双零
- 完工回报形：批名、座位号、链笔哈希、双仓 commit 哈希、scribe verify 全文、reconcile 增量、矩阵行覆盖表（每行对应测试名）、fixture 写径冒烟输出摘录、零写证明读数
