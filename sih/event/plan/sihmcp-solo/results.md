# sihmcp-solo 批结果档：MCP 线载体 Rust 化直接替换

> 会话 f85c53836d0eb3a5（sess-zcode-260911-sihmcp），2026-09-11，任务包 sih-engine/sih/state/plan/sihmcp-solo.md
> 令源：用户 2026-09-11 令「rmcp在sihankor旧仓开发的时候就是定下技术栈选型，直接替换。」承前令「我们不要套一层壳啊，rustmcp也是官网支持的」；用户 2026-09-11 令「继续完成，多子代理并行加速，直到rmcp完全替换完成」

## 一、四段实施实录

段1（b38399c）
: 引擎位载体实装：lib 模块 src/mcpserver（runtime 加 alpha 加 server）加 src/bin/sihmcp.rs，rmcp 0.5 stdio 传输，alpha 九具投影；chain_query 与 chain_verify 进程内库调 event_stream。双载体对表八用例判词一致，测试 196 绿。

段1 补（6fd2f81）
: DEC-023 落笔 doc/decision/023-mcp-rust-carrier.md，T6 三闸绿（化格零改、核阅零 findings、检词零 findings）。

段2（5f5e23f）
: writeface 五件移植（matrix 加 errors 加 passthrough 加 session 加 tools）：分级矩阵与 structural 裁剪注册面、一对一绑定卫、九字段教学载荷与他会话号裁剪、正身签发与断开收约。tools/list 十九具两载体集合差空，beta 未绑路径五用例判词一致，mcpserver 测试十三绿。

段3 基座（42b48d8）加编组位（ea2cc21）加本体（c1b3d9a）
: tokens 登记册移植先行；三子代理并联：甲 httpface（streamable 加 Bearer 闸逐调用新读登记册加域布局两形加会话表加 fail-closed 启动复核）、乙 webface（台面七路由：视图加令牌台加签发加撤销两步加说明书）、丙 bootstrap（域接入链五段加 CLI 形加台面三路由两步确认）；组装层合并三路由面，开域两路由归丙簇真形消解冲突，main http 臂接线（SIH_TRANSPORT=http，SIH_HTTP_BIND 覆写）。HTTP 面冒烟：握手 serverInfo sihmcp 0.9.0、tools/list 十九具、locks_read 见真锁、chain_verify valid、台面三页 200。mcpserver 测试四十二绿。

## 二、判据判定（P-1 到 P-6）

P-1 alpha 九具 stdio 对表
: 成立。材料 p1-alpha-dualrun.txt 八用例 ALL-IDENTICAL（locks_read 加 chain_verify 加 chain_query 两形加 naming_guide 加 nomenclator_query 加 heartbeat 加 retriever_recall）。

P-2 beta 十具生命周期
: 成立。材料 p2-lifecycle-battery.txt：fixture 根隔离形（夹具工场承 mcpline tests/fixture_root.py，真仓零写入）open 加 lock 加 intent 加 commit wip 加 append 加 commit settle 加 unlock 加 close 八步 ALL-IDENTICAL；错误路径五步 p2-beta-errorpaths.txt ALL-IDENTICAL。

P-3 HTTP 面加台面
: 成立。材料 p3-http-smoke.md：握手加列具加读调用通，台面三页 200，Bearer 闸九测（缺头加不在册加停行 401，readonly 加 custom 403，domain_write 放行）。

P-4 parity 电池
: 成立。上三件材料即电池集，逐工具 JSON 判词一致（值域容差：会话号与哈希与时戳）。

P-5 零写入守卫
: 成立。守卫分层三档在 src/mcpserver/tests.rs：alpha 全禁词面（runtime 加 alpha 加 bin）、注册面只扫直写盘原语（server.rs）、writeface 直写盘禁入（session.rs 正身暂存与 tokens.rs 登记册两白名单单点）。

P-6 切换后复验
: 批后执行位：主树构建加 .zcode/config.json 注册切换加 8765 换防后链 verify valid 加 critsweep 判读，结果随声明笔落链。

## 三、对等偏差申报（承 DEC-023 行为对等红线，逐条）

- HTTP 会话制对 Python 无会话制：rmcp StreamableHttpService 每 MCP 会话一连接会话，标识根于 initialize 锚定；逐调用识别与 scope 裁决仍每笔新读登记册零缓存。
- 读具域形式未承载：alpha 九读具内部 resolve_root，HTTP 所绑域根对读具不生效（候补位：读具收 root 覆写参，另批）；写面台账 argv 两形锚定候 canonical 域实态另批。
- 响应编码：rmcp 固定单事件 SSE 流，Python 纯 JSON，两形皆 MCP streamable 规约合法形。
- naming_guide 与错误教学文案字节级对表（含 Python 原文「闸加nomenclator」无空格串接保真）。

## 四、仪式实录

三轮 scope 实勘重开在链（allow 即全量写入面无缺省面加甲表派生表不含批形段加 lib.rs 与 tests 面缺口），协调常识三条落链；tests 内联化循 event_stream/tdd_tests.rs 先例；守卫面分层三档；另窗并行批 gateshape 锁面零交集，提交以 --session 消解。

## 五、候令移交

- sihmcp 载体名语义忠实终裁候立名程序人节点（甲表认领在案）。
- 读具域形式加 canonical 域写锚定：候另批（甲簇申报位）。
- mcpline Python 冻结形：随本批注记，退役批次另令。
