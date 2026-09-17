# mcpauth-rev1-solo 批任务包：DES-015 防呆形修订与 facet 轻量重测

> 令源：用户 2026-09-09 简化裁定原话「我觉得没有那么复杂，我们的机制是为了防止善意的误操作，锁工作区，而不是做安全防护」；承接 mcpauth-solo 批 DES-015 已收约在档与基底改令在档
> 形：轻量修订批独立立约独立收约，零实装代码；旧测 m-mcpauth-1 存对照，重测以新 gid m-mcpauth-2 承载（改写强制新 gid 加谱系披露纪律）

## 一、使命

DES-015 按防呆形修订六点全落，facet 命题按修订稿轻量重测九发 stable_clear 后 tally 执契终签上链，成为 HTTP 写面实装批的现行正典。

## 二、修订必答六点

1. 令牌降为项目标识牌：明文短标识即可，台账改明文登记册（token_id 与 domain_root 与 scope 与 status 与 issued_at 与 issued_by），砍哈希落册与明文一次性交付与轮换双活窗口三件
2. 签发台进视图：mcpline.web 加令牌管理台（签发列表撤销），无管理钥，签发带一步确认防手滑，签发即台账追加行
3. 鉴权语义改识别语义：连接头带项目标识即可（Bearer 头形留否由执行批判词），错误直给教学语，砍 401 五因不区分与反枚举形
4. 威胁模型章缩为防呆清单：防错域、防误写、防并发踩踏、防不可追溯四条，对抗面（泄露劫持枚举重放）整章移除或概览一句带过「本机制防善意误操作不做安全防护」
5. 留守不动：域绑定、租约五验与锁队列、链上归因（信封记项目标识非哈希）、撤销即停行、127.0.0.1 缺省绑定（缺省形态非安全边界表述）
6. 修订记录载用户裁定原文与日期
7. 增「多人多agent预留」一节只声明缝不预装机制（令源用户 2026-09-09 问「未来多项目多人多agent引入司衡引擎治理如何预留鉴权入口」），四缝：一凭证槽位不变律即身份解析单点（连接头带标识、服务端查册单点解析）立为设计律未来名字牌升级签名牌只换牌不拆墙；二归因槽位即链信封主体预留人级粒度扩展且台账 append-only 加行即扩不改历史；三传输缝即 127.0.0.1 缺省绑定与远程不防表述为部署缝而非永久约束，多人版扩展路径即绑址放开加 TLS 反代加通道认证；四签发台即人节点入口即视图签发台三动作即未来 RBAC 动作集，多人版先认操作者人级身份再签人绑令牌归因闭环到人；节末总纲「四缝留齐，加人即换牌加行」

## 三、验收

- 修订稿六点逐点有判词与依据；管线三步绿；facet m-mcpauth-2 九发 stable_clear 加 tally 执契终签上链
- 得一不过即如实回报 fail 候裁，禁降格落档

## 四、红线

1. 零实装代码：不动 sih-tools/mcpline 与任何既有工具代码，管理台只作设计定形
2. 留守五件不动：域绑定、租约五验与锁队列、链上归因信封记项目标识、撤销即停行、127.0.0.1 缺省绑定
3. 判定语义正典留确定性程序：修订不得引入 LLM 裁决位
4. 与在飞批共用今日链：锁冲突走 lease wait-turn 禁绕行禁 preempt

## 五、写入面（allow 清单）

- sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md（修订）
- sih-engine/sih/state/plan/mcpauth-rev1-solo.md
- sih-engine/sih/event/plan/mcpauth-rev1-solo-results.md 与 mcpauth-rev1-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-mcpauth-2/

## 六、完工回报形

批名、座位号、链笔哈希（intent 与 cert 与终签与 exit）、双仓 commit 哈希、scribe verify 全文、reconcile 增量、m-mcpauth-2 判词与终签哈希、修订六点判词摘要、与 m-mcpauth-1 旧测对照说明。
