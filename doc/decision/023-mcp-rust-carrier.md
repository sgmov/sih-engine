# DEC-023 MCP 线载体 Rust 化直接替换决策

本决策成立司衡 MCP 线载体由 Python FastMCP 实装直接替换为 Rust rmcp 原生承载的定约。令源即用户 2026-09-11 令「rmcp在sihankor旧仓开发的时候就是定下技术栈选型，直接替换。」承前令「我们不要套一层壳啊，rustmcp也是官网支持的」。技术栈选型本身不是本决策新立：旧仓 sihankor 的 audit-040 已于 2026-07-20 裁定从零加 Rust 加 rmcp（Official Rust SDK），本决策承其继承回正典，立的是载体替换的位序与实施序与切换闸。相名转写声明：本令承域字符集闸一律拉丁转写相名，alpha 相与 beta 相指称希腊字母原名的同一相，承 DEC-022 转写先例。

## 概览 {#overview}

- 技术栈选型承旧仓 audit-040 继承，rmcp 0.5 承旧仓实测装配形，失败设计不继承::[选型继承](#inheritance)
- 载体落 DEC-001 源码位，引擎 bin sihmcp 加 lib 模块 src/mcpserver，依赖入引擎根 Cargo::[载体与结构位](#carrier)
- 载体名 sihmcp 走甲表新词认领在案，语义忠实终裁候立名程序人节点，serverInfo 名称变更是有意变更::[甲表认领](#stem)
- 实施四段递进，段1 已落笔在案，行为对等判据全绿前零切换::[四段实施序](#phases)
- 行为对等六判据即 P-1 到 P-6，parity 电池逐工具 JSON 等值判词承 mcpdual 先例形::[行为对等判据](#parity)
- 切换闸：注册切换与端口换防只在 P-4 全绿后执行，旧载体转冻结兼容只读形::[切换闸](#cutover)
- 工具本体边界：lease 与 critsweep 与 gauge 与 nomenclator 四族工具的 Rust 化不在本批::[工具本体边界](#boundary)
- 备选三案即独立 crate 成员与长期双载体并存与 mcpboot Python 形先行俱拒绝::[备选方案](#alternative)

## 选型继承 {#inheritance}

旧仓 sihankor 是沉默参考与盘点源，失败设计不继承，决策痕迹可挖。旧仓档 audit-040 落 `sihankor/sih/static/audit/audit-040-assay-tech-stack-decision-rust-rmcp.md`，2026-07-20 裁定：实现语言 Rust，MCP 封装 rmcp（Official Rust SDK），理据为协议层用成熟 SDK 不从零实现 JSON-RPC 握手、官方 SDK 保证协议版本跟进、单二进制分发；其时 audit-039 已实测 rmcp 握手 initialize 加 tools.list 加 tools.call 全通。旧仓 sih-server crate 承 rmcp 0.5 加 axum 0.8 双传输装配形，只作装配样板参照，代码不搬运。本批 rmcp 取 0.5 版线与旧仓实测同源，依赖本地缓存已验。

## 载体与结构位 {#carrier}

载体落 DEC-001 既定源码位：源码存引擎核心源代码，是 MCP server 与内部模块的承载。物理形三件：引擎根 Cargo 增依赖 rmcp 加 axum 加 tokio 加 async-trait 加 anyhow；lib 模块 `src/mcpserver/` 含 runtime 加 alpha 加 server 加 tests 四件；bin `src/bin/sihmcp.rs`。构建产物 `target/debug/sihmcp` 与引擎各 bin 同 target 位，单二进制分发承 audit-040 理据。引擎进程内库调消子进程缺陷族：chain_query 与 chain_verify 走 event_stream 库调，retriever 检索面与 beta 相 record 三具随段2 同形承接；Python 工具族子进程包裹与现行 argv 逐一对应。

## 甲表认领 {#stem}

载体名 sihmcp 经 lease open stem 查册闸甲表新词认领在案：概念锚 zh 司衡MCP承载，code 形无承，mcpline 查册 unknown 非既裁形，语素派生 sihmcp:new。语义忠实终裁候立名程序人节点，收敛后 nomenclator register 在册，承 DEC-017 修订五修订六程序。serverInfo 名称由 mcpline 改 sihmcp 是有意变更：载体身份承新名，线名与契约正典不变，SPEC-023 与线程序包 `sih-engine/sih/state/plan/mcpline-line-v1.md` 持续有效，载体形变更随批修订承载。

## 四段实施序 {#phases}

段1
: 骨架加 alpha 九具 stdio 面。已落笔 b38399c：九具注册与投影全通，chain_query 与 chain_verify 进程内库调，双载体对表八用例判词一致，测试一百九十六绿。

段2
: beta 相十写具移植。会话闸加五验加 CALL-LOG 加正身绑定加透传面照 DES-014 行为对等；lease 七具子进程 lease CLI，record 三具进程内 event_stream 承接。

段3
: HTTP 面加台面。streamable HTTP 承 rmcp StreamableHttpService，绑 127.0.0.1:8765 端点 /mcp；axum 台面承载视图与管理台与签发加撤销加域接入三动作；tokens 账本加域接入链固化五段。

段4
: parity 加切换加冻结。parity 电池新旧双跑逐工具判词；注册切换 sihmcp 二进制；8765 换防；mcpline Python 转冻结兼容只读形；SPEC-023 修订随批。

## 行为对等判据 {#parity}

P-1 alpha 九具 stdio 冒烟逐具出参与现行 mcpline 同形同日对表，错误载荷四字段同构。P-2 beta 十具租约全生命周期场景新面跑通，CALL-LOG 与链笔与回执与现行形同构。P-3 HTTP 面握手加列工具加读调用通，台面三动作与既有 tokens 账本行为对表。P-4 parity 电池脚本化场景集新旧双跑逐工具 JSON 判词一致。P-5 零写入守卫 alpha 面源码 grep 写动词零命中沿用。P-6 切换后链 verify valid 加 critsweep degraded 假加锚面五行回显。判词形承 mcpdual-parallel 批三路对表先例，材料落 `sih-engine/sih/event/plan/sihmcp-solo-materials/`。

## 切换闸 {#cutover}

零静默替换：8765 端口换防与 `.zcode/config.json` 注册切换只在 P-4 电池全绿后执行，验用临时端口不侵现役防位。切换序：注册切换，旧 python 防位停防，新面真防位接管，P-6 复验。旧载体 mcpline Python 转冻结兼容只读形：代码留档零删除，标注冻结位与后继指针，退役批次随段4 后另令。

## 工具本体边界 {#boundary}

lease 加 critsweep 加 gauge 加 nomenclator 四族工具本体的 Rust 化不在本批辖域，子进程承接维持，候选另批。本批替换的是 MCP 线的协议承载壳，不是工具本体；壳薄化与工具本体 Rust 化是两条线，混线即范围失控。

## 备选方案 {#alternative}

workspace 成员独立 crate
: 已拒绝
: 引擎仓顶层新增 mcp 成员节点违 DEC-001 五顶层节点分界，提交闸 staged_out_of_scope 实勘纠偏在案。

长期双载体并存
: 已拒绝
: 直接替换令已裁，长期并存即双执行者分叉与维护双份，违单线单源。

mcpboot Python 形先行
: 已拒绝
: 去壳令后 Python 面增量投资与令相悖，域接入链固化五段范围并入段3 由 Rust 载体承载，原 Python 形计划撤销归档。

## 关联 {#relation}

上游旧仓 audit-040 承选型正典与 audit-039 承 rmcp 实测；契约面 SPEC-023 与 DES-014 承 beta 相安全模型与线程序包 mcpline-line-v1；结构位 DEC-001 承五顶层节点与源码位与 DEC-022 承相名转写先例；立名程序 DEC-017 承甲表认领；对表先例 mcpdual-parallel 批；任务包 `sih-engine/sih/state/plan/sihmcp-solo.md`。
