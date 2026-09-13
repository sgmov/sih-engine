# 腿四 sihmcp 分派面 registry 接线与 mcpline 收编方案（候批设计稿）

正典：SPEC-025-toolful-mergeback-v1.md 腿四与插件槽位协议节。本稿为 lease-mergeleg23-parallel 批施工期主线预备件，腿四批开工时升格为任务包正典。

## 现状（2026-09-14 实测）

- sihmcp 单进程在役（SIH_TRANSPORT=http，127.0.0.1:8765/mcp），tools/list 实测 19 具（alpha 九读数加 beta 十写面）。
- 分派面：src/mcpserver/server.rs 两段 match（写面 380 至 476 加读面 477 至 503），defs 数组静态手写；工具实现散布 mcpserver/alpha 与 beta 与 tools。
- registry 骨架在役：sih_engine::tools_registry（ToolProvider 四面加 ToolRegistry 加 PluginManifest），HeartbeatProvider 演示迁移已验（描述与 schema 逐字节对表在役 defs）。

## 接线方案（三步）

1. Provider 化逐具包装：19 具各包一 struct 实现 ToolProvider，call 委托现有函数；写面具需要连接态（ConnectionSession），registry call 签名扩 ctx 参数或 provider 捕获 Arc<SessionState>——取闭包捕获形（零 trait 变更）。
2. server.rs 分派改造：tools/list 改 registry.list() 派生（defs 数组退役）；call 分派改 registry.call 兜底，特殊分支（lease_open 立会话与 lease_close 拆会话的连接生命周期动作）保留显式 match 前置。
3. 回归锁：接线前后 MCP 面双跑对表（tools/list 逐具 name 加 description 加 schema 逐字节；三只读工具调用同参同出参），mcpdual-parallel 先例同形。

## mcpline 收编余量清单

- HTTP 8765 面与 MCP 协议：引擎已承载（首战批实测），收编完成度最高。
- web 管理台（/ 面板与 /tokens 台面）：引擎 viewer 或 httpface 对表迁移；面板静态资产缺席既有病随批治。
- bootstrap 加 init CLI（开域全链）：域自述卡与镜像核对与 --complete 补全形迁移为引擎 bin 或 sihmcp 子命令。
- writeface 域根语义与 write_gates：对表迁移（mcpline writeface 与引擎 mcpserver/tools 重叠面逐函数对表，单源化取一）。
- tokens 管理与登记册镜像：对表迁移。

## 判据

- R1 tools/list 与三只读调用接线前后逐字节一致。
- R2 外部插件 manifest 注册的工具可在 tools/list 显形并可调用（桥实装后；本批 manifest 解析在役即可）。
- R3 mcpline 对表清单逐项核销或申报，生产 spawn 面（.zcode config 的 sih-tools/mcpline 入口）切换为引擎单面。
