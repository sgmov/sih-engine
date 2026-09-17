# mcpauth-solo 批任务包：HTTP 面多项目识别与分项目写授权安全模型设计

> 令源：用户 2026-09-09 令「http要分项目的如何做，也要做写入权限」；承接 mcpweb-solo HTTP 只读面在役与 DES-014 stdio 安全模型在档
> 形：solo 设计批独立立约独立收约，零实装代码；本设计落档过得一即 mcpweb HTTP 写面实装批的硬前置

## 一、使命

设计 HTTP 面多项目识别与分项目写授权安全模型，落 sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md（DES 编号对地面核对顺延），facet 得一测量落据，成为 HTTP 写面实装批的硬前置正典。

## 二、设计必答八问

1. 项目识别形：token 签发通道（人节点或主窗签发位）、台账形（tokens.ndjson 哈希落册、明文一次性）、传输形（Authorization Bearer 头对路径段对每项目端口，三形取舍判词）
2. 识别与会话映射：token 与 lease 会话一对一形；身份锚从进程正身到令牌正身的 DES-014 变体形，五验在 HTTP 下各验什么
3. 授权矩阵：scope 三档 readonly 与 write:ns=<项目命名空间> 与 custom 的语义；写操作包命名空间裁剪形；与既有 scope_violation 与 locked_elsewhere 与 wait_turn 理由码的原位衔接
4. 写面复用：零新增执法——令牌只裁剪透传，scribe 三闸与租约五验与 closeguard 与 git hooks 原位生效；record_append 无幂等闸的 HTTP 下风险与缓解（锁纪律强制形或明示不防）
5. 审计与撤销：信封主体加令牌身份的形；撤销即台账行状态变与在飞会话拒绝形；令牌轮换形
6. 威胁模型：令牌泄露、跨项目越权、重放、枚举、台账膨胀，各给缓解或明示不防；缺省绑定 127.0.0.1，远程与 HTTPS 暴露明示不防入边界
7. 失败语义：未认证与越权与限流在 HTTP 状态码与 MCP 错误载荷的映射；报错即教学边界（不泄露他项目存在性）
8. 边界申明：本设计不涉自有运行时（pk-079）与 GOV-002 v3；SiInfer 彩排写授权即本设计第一实例但不预占

## 三、验收

- DES 档八问逐问判词与依据；facet 得一测量 m-mcpauth-1 九发 stable_clear 加 tally 执契终签上链
- 化格核阅检词认证全管线绿；双仓 settle；reconcile 双零
- 得一不过即如实回报 fail 候裁，禁降格落档

## 四、红线

1. 零实装代码：不动 sih-tools/mcpline 与任何既有工具代码
2. 判定语义正典留确定性程序：设计不得引入 LLM 裁决位
3. 实装批未过本设计不一裁不开工，本批不预写实装
4. 与在飞批共用今日链：锁面含 trail 路径，冲突走 lease wait-turn 禁绕行禁 preempt

## 五、写入面（allow 清单）

- sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md（新，编号对地面核对）
- sih-engine/sih/state/plan/mcpauth-solo.md 与 mcpauth-solo-prompt.md
- sih-engine/sih/event/plan/mcpauth-solo-results.md 与 mcpauth-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-mcpauth-1/

## 六、完工回报形

批名、座位号、链笔哈希（intent 与 cert 与终签与 exit）、双仓 commit 哈希、scribe verify 全文、reconcile 增量、m-mcpauth-1 判词与终签哈希、八问判词摘要、scope 与 token 台账的稿形要点。
