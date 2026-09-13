# lease-leg4wiring-solo 任务包（单线形）

## 形声明与令源

- 队形：单线 solo，主线亲写（在役 MCP 分派面改造高敏感，上下文连续性优先）。
- 令源：用户 2026-09-14 goal 续推令腿四；正典 SPEC-025 加方案稿 leg4-registry-wiring-design.md（本批升格随批归档）。

## 目标（接线三步加收编核销）

1. 19 具 Provider 化：alpha 九读数加 beta 写面逐具包 ToolProvider，写面具闭包捕获连接态（零 trait 变更）。
2. tools/list 改 registry.list() 派生（defs 数组退役）；call 分派 registry.call 兜底，lease_open 与 lease_close 连接生命周期动作保留显式前置。
3. MCP 双跑对表锁：接线前后 tools/list 逐具 name 加 description 加 schema 逐字节一致，三只读工具调用同参同出参（mcpdual 先例同形）。
4. mcpline 收编核销：余量五项逐项判词（web 台面、bootstrap/init、writeface 域根语义、tokens 管理、生产 spawn 面），可迁者迁、未迁者申报让位。
5. 生产 spawn 面：工作区 AGENTS.md 与 .zcode config 对表更新（.zcode 改动须客户端重连生效，如实申报）。

## 验收判据

- F1 接线后 cargo test 全量绿（含在役 mcpserver 测试面）。
- F2 双跑对表：MCP 出参逐字节锁（接线前基线先捕获）。
- F3 收约归并重建后 sihmcp 重启实测三连（initialize 加 tools/list 19 具加只读调用）。
- F4 收编核销清单逐项判词在档。
- F5 全族回归不破。

## 明确不做

- 进程外插件桥实装（A4 维持）；腿六长尾候下批。
- 在役 8765 进程施工期不重启（收约归并后方重启切换）。

## 改动文件清单

sih-engine/src/mcpserver/server.rs（分派面）、src/mcpserver/providers.rs（新，19 具包装）、src/tools_registry.rs（如须扩 ctx 形）、src/bin/sihmcp.rs（如须）、tests/leg4_wiring.rs（新）、本包、方案稿随批、结果档与 materials、当日 trail、工作区 AGENTS.md 与 .zcode/config.json（级联）。
