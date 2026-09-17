# sihmcp-solo 批任务包：MCP 线载体 Rust 化直接替换（rmcp 正典继承）

> 线：mcpline（MCP 实装线，线程序包 sih-engine/sih/state/plan/mcpline-line-v1.md）
> 令源：用户 2026-09-11 令「rmcp在sihankor旧仓开发的时候就是定下技术栈选型，直接替换。」承前令「我们不要套一层壳啊，rustmcp也是官网支持的」
> 盘点源：旧仓 sihankor audit-040（2026-07-20 技术栈决策正典：Rust 加 rmcp Official Rust SDK，audit-039 实测握手全通）加旧仓 sih-server crate（rmcp 0.5 加 axum 0.8 装配样板，只作盘点源不作引用源，失败设计不继承）
> 形：solo 批独立立约独立收约，四段递进，parity 电池全绿前零切换

## 一、使命

MCP 线载体由 Python FastMCP（sih-tools/mcpline）直接替换为 Rust rmcp 原生承载：引擎新 bin sihmcp 加 lib 模块 mcpserver，stdio 与 streamable HTTP 双传输，十九工具（alpha 相九读数加 beta 相十写面）行为对等投影。引擎工具进程内库调消子进程缺陷族，Python 工具族子进程包裹维持，parity 电池判词一致后注册切换与端口换防。线名与契约正典不变（SPEC-023 与线程序包持续有效，修订承载载体形变更）。

## 二、四段实施序

- 段1 骨架加 alpha 九具（stdio）：引擎 Cargo 增 rmcp 0.5 加 axum 0.8 加 tokio 加 async-trait；src/mcpserver/ 模块骨架；src/bin/sihmcp.rs stdio 形。九具承接形：chain_query 与 chain_verify 进程内库调 event_stream；retriever_recall 进程内库调 retriever；critsweep 与 heartbeat 与 locks_read 与 nomenclator_query 与 nomenclator_check 子进程包裹（现行 CLI 形逐一对应）；naming_guide 静态教学件读取。DEC-023 决策文档同段落笔。
- 段2 beta 十具写面：writeface 移植（会话闸加五验加 CALL-LOG 加正身绑定加 passthrough）；lease 七具子进程 lease CLI（绝对路径纪律在调用位）；record 三具进程内 event_stream::append 承接，闸逻辑承接位实勘 src/bin/scribe.rs 后定形。
- 段3 HTTP 面加台面：streamable HTTP（127.0.0.1:8765 /mcp，rmcp StreamableHttpService）；axum 台面（/ 视图加 /tokens 管理台加签发加撤销加开域三动作）；tokens 账本；开域链固化五段（签发加开域加镜像加客户端注册旗标，mcpboot 范围并入本载体，Python mcpboot 计划撤销归档）。
- 段4 parity 加切换加冻结：parity 电池新旧双跑逐工具判词；.zcode/config.json 注册切换 sihmcp 二进制；8765 换防（停 python 防位）；mcpline Python 转兼容只读冻结形；AI-MANUAL 与 README 载体注记；SPEC-023 修订四；结算收约。

## 三、行为对等判据（可证伪）

- P-1 alpha 九具 stdio 冒烟：逐具调用出参与现行 mcpline 同形同日对表，错误载荷四字段（error、what_this_tool_does、valid_params、canonical_pointers）同构
- P-2 beta 十具：租约全生命周期场景（open 加 lock 加 intent 加 commit 加 unlock 加 close）新面跑通，CALL-LOG 与链笔与回执与现行形同构
- P-3 HTTP 面：/mcp 握手加列工具加读调用通；台面三动作与既有 tokens 账本行为对表
- P-4 parity 电池：脚本化场景集新旧双跑，逐工具 JSON 判词一致（mcpdual-parallel 先例形）
- P-5 零写入守卫：alpha 面源码 grep 写动词零命中断言沿用
- P-6 切换后链 valid 加 critsweep degraded 假加锚面五行回显

## 四、红线

1. 行为对等：十九工具出入参与错误语义与现行 mcpline 逐字段对表，不新增第二个语义执行者
2. 域纪律：sih-engine 域内文档走化格加核阅加检词加认证 T6 序；mcpserver 源码按 DEC-001 归 src 位（源码即 MCP server 与内部模块承载）
3. 零静默替换：8765 换防与注册切换只在 P-4 全绿后执行，验用临时端口不侵现役防位
4. 语汇：标识牌非凭据（DEC-010 红线）；载体名 sihmcp 走甲表新词认领，语义忠实终裁候立名程序人节点
5. 旧仓承径：audit-040 与 sih-server 只作盘点源与装配样板参照，代码不搬运，frontmatter 假设族失败设计不继承
6. 工具本体不动：lease 加 critsweep 加 gauge 加 nomenclator 工具族 Rust 化不在本批（子进程承接维持，另批候裁）

## 五、写入面（allow 清单）

- sih-engine/Cargo.toml 与 Cargo.lock
- sih-engine/src/mcpserver/** 与 sih-engine/src/bin/sihmcp.rs
- sih-engine/doc/decision/023-mcp-rust-carrier.md（DEC-023）
- sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md（修订四）
- sih-engine/sih/state/plan/sihmcp-solo.md 与 sihmcp-solo-prompt.md
- sih-engine/sih/event/plan/sihmcp-solo-results.md 与 sihmcp-solo-materials/
- sih-engine/sih/event/trail/2026-09-11.ndjson（链笔）
- sih-tools/mcpline/**（冻结形加 AI-MANUAL 加 README）
- .zcode/config.json（注册切换位）
- sih-tools/identity/reports/ 与 sih-tools/scribe/reports/（批机械链侧写）

## 六、明确不做

- lease 与 critsweep 与 gauge 与 nomenclator 工具本体的 Rust 化（候裁另批）
- stdio 与 HTTP 两面工具集分叉（两面同十九具）
- 旧仓代码搬运
- mcpboot Python 形实施（范围并入段3，原计划文档撤销归档）
- pk-098 其一锁生命周期设计变更（候裁维持）
