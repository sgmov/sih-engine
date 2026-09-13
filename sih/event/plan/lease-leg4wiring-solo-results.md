# lease-leg4wiring-solo 批结果档

## 结算要点

- 批名 lease-leg4wiring-solo（单线形；stem 甲表认领 leg4wiring:new，概念锚 zh 腿四接线，solo 段闸弃，终裁登记候人节点）
- 会话 c48a310db19c09b2 双仓，九锁，意图笔在链；正典 SPEC-025 加方案稿 leg4-registry-wiring-design.md 随批升格归档
- 令源用户 2026-09-14 goal 续推令腿四；主线亲写（在役 MCP 分派面高敏感）

## 接线三步落地

1. 19 具 Provider 化：providers.rs（mcpserver 模块）ClosuredProvider 通用包装，alpha 九读数与 beta 矩阵暴露写面全数 ToolProvider 注册，写面具闭包捕获 Arc<Mutex<ConnectionSession>>（conn 本为 Arc，零结构改造）；描述与 schema 与 defs 数组同源零漂移，注册序与 tool_defs 拼接序一致。
2. tools/list 改 registry.list() 派生（defs 数组退役为数据源保留）；call_tool 分派全量走 registry.call，矩阵 precheck 原形保留（授权矩阵拒透传语义零变），未知工具 NotFound 还原 invalid_params 原文。
3. 双跑对表锁全达成：tools/list 基线指纹 7e1c9328608f7438 接线前后逐字节一致（19 具）；heartbeat 加 locks_read 加 chain_query 三只读新旧面同参同出参（8765 在役旧码对 8766 新码临时面对表）。httpface 每连接 factory 形按连接构建 registry（conn 会话身份语义保留）。

## mcpline 收编五项核销（判词在档）

- HTTP 8765 面与 MCP 协议：引擎 httpface 已承载（本批实测）——核销。
- web 管理台：webface 路由全量在役（/ 面板加 /tokens 台面加签发撤销两步确认）——核销。
- bootstrap/init CLI：bootstrap.rs bootstrap_domain 加 run_cli 在役——核销。
- writeface 域根语义：mcpserver tools 全函数在役——核销。
- tokens 管理：tokens.rs 五函数在役——核销。
- 生产 spawn 面：.zcode config sih server 入口已为引擎 sihmcp（前批切换，本批核验坐实）；AGENTS.md MCP 节行文级联更新——核销。

## 测试读数

- 接线回归锁 tests/leg4_wiring.rs 四测绿（registry list 与 defs 逐字一致加调用路由加 NotFound 形加 alpha 九加 beta 暴露全在册）。
- mcpserver 模块 42 测全绿（接线影响面零破）；lib 余败与主树同集（既有环境红非接线引入，主树对照坐实）。
- 落差申报：instructions 自报 mcpline 线名保留（线名非载体名，SPEC-023 词形）；ClosuredProvider 对 defs Tool 名经字面量表回射（19 具有限名单 drift 即 unreachable）。

## 明确不做与后件

- 进程外插件桥实装 A4 维持缺席申报；腿六长尾约 8000 行候下批。
- 在役 8765 进程施工期零触碰，收约归并后方重启切换（重启实录随完工报告）。
