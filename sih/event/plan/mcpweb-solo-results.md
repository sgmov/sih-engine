# mcpweb-solo 结果档：mcpline 统一 web 服务形态——单端口双面与 HTTP 只读红线与 stdio 零回归

> 承接：任务包 sih-engine/sih/state/plan/mcpweb-solo.md 与用户 2026-09-09 令「我们视图部分是网页的一个服务，我们应该把MCP也做成一个HTTP的方式，使用的同一端口，但是使用的是不同的API」。产出即 mcpline web 入口件与 HTTP 只读面与版本 0.3.0 additive bump 与测试族扩展。
> 队形单线形 solo，日期 2026-09-09，会话 sess-zcode-260909-mcpweb（session_id 0da504d78c9672e5，lease 1.39.0）。并行在飞：主窗发布收尾（sess-zcode-260909-main-mcpdev）共用当日链；本批取锁时段主窗零持锁，九锁一次全取零 wait-turn 实发。

## 意图锚定

- 意图事件：intent_refined `679e26f2-b52c-4eae-9745-748c08311536`（event_hash `7acefeae9b0b773ebac2cf69b89d50295e29f7f2bc4a68ef729229098b569ac9`）
- record：sih-tools/scribe/reports/2026-09-09-ask3-mcpweb-solo-record.json
- validation：sih-tools/scribe/reports/2026-09-09-ask3-mcpweb-solo-validation.json
- 三锚引文程序切片自 proodos 06-on-canon（:149 知止划界）与 07-on-assay（:55 鉴只列事实）与 08-on-settle（:110 应而不藏）原文，双门（scrutinator ask3 包 exit 0 findings 空；ask3repeater exit 0 status ok anchor_count 3）原位复验。

## 前置读数

- 叩问：六词查检六信号（统一入口、单端口、异路径、只读红线、结构性缺席、静态页，俱轻级未注册），契约内六条处置后 digest passed covered 6。
- 正身：identity verify 零异常（anomalies 空），identity_hash `cc635b6ec50720ab87a9a3e3db51f8817f231bffc757c4437505d8b22523783a`。
- 租约：lease open 双仓登记（tools 工地 worktrees/sih-tools/mcpweb-solo 分支 msh/mcpweb-solo 基 integral-stage-build；engine 工地 worktrees/sih-engine/mcpweb-solo 分支 msh/mcpweb-solo 基 main），allow 九路径逐路径 lock 九锁全 rc=0（含当日链 trail 锁）。

## 主件读数（统一 web 服务形态）

- 新件 src/mcpline/web.py：统一入口 `python -m mcpline.web`，ASGI 组合——mcp SDK `streamable_http_app()` 路由（`/mcp`）+ 根路径 Route 入口页 + StaticFiles 静态兜底；lifespan 委托 mcp SDK app（streamable 会话管理器经 lifespan run）。旗标：`--host`（缺省 127.0.0.1）、`--port`（缺省 8765，避 SiInfer 在役服务口 8100 与 8200 与 8190 与 8191 与 3080）、`--static-dir`（缺省 `<SIH_ROOT>/sih-visual/assets/viewer-dashboard-2026-09-06`）、`--entry`（缺省 dashboard.html）。
- HTTP 只读红线（安全边界非可选优化）：HTTP 面独立 FastMCP 实例（`mcpline-http`，stateless_http + json_response）恰注册 α 五只读工具（chain_query / chain_verify / critsweep / heartbeat / locks_read，实现直用 stdio 面同名函数零新增语义）；β 十写工具（MATRIX_ROWS 全集）对 HTTP 实例零注册即结构性缺席（DES-014：身份与会话模型绑进程，HTTP 端口无进程身份，未认证写面不得上网络口）。`assert_for_readonly_face` 启动复核（注册位恰五且 β 交集空，违即 RuntimeError 拒启 fail-closed）；HTTP 面零 lease 连接会话（连接单例不在位，零会话零写）。
- 面板静态目录只读引用：零改 sih-visual 任何文件（红线对表：本批 sih-visual/** 零触碰）；静态目录缺席即拒启（零创建）。
- 版本 additive bump：pyproject 0.2.0 改 0.3.0；`src/mcpline/__init__.py` 的 `__version__` 0.1.0 改 0.3.0（存量漂移对表收正，版本嵌入处全数复核即此两位）；pyproject description 增统一 web 服务词面。
- README：形态节增统一 web 一行、新增「统一 web 服务形态（mcpweb-solo 批）」节（只读红线与启动命令形）、运行节双形态双命令、测试族五件改六件。
- CALL-LOG 册位如实转述：mcpline 不在 calllog 系统十九册（mcpline/CALL-LOG.md 缺席且权威腿 calls.ndjson 零 mcpline 行），本批不强制立册不裸增册，候 calllog 扩面通道裁。

## 测试族读数（六件 59 绿）

- 全测试族：工地内 `pytest tests/` exit 0，59 passed 零 fail（新件 test_web_smoke.py 四测入族；报告 mcpweb-solo-materials/full-tests-59-green.log）。
- HTTP 活服冒烟（8765 口实跑，读数 mcpweb-solo-materials/http-smoke-reading.json）五断言：
  - 断言一 GET /：200，content-type text/html，面板标识串「司衡视图」在场（title「司衡视图 · Dashboard 样张（v3 风格 × 可读字层）」）；
  - 断言二 tools/list：恰五（chain_query、chain_verify、critsweep、heartbeat、locks_read），β 交集空；
  - 断言三 chain_verify 经 HTTP：status valid、valid true、events 144（2026-09-09 当日链）；
  - 断言四 β 结构性缺席三重：注册位集合断言（恰五且 MATRIX_ROWS 交集空）+ 线上 tools/list 交集空 + tools/call lease_open 经 HTTP 拒（isError true，Unknown tool）；
  - 断言五 stdio 回归加零写证明：全测试族含 test_stdio_smoke（alpha 五工具真根回归 + beta fixture 写径闭环）与 test_zero_write（套件前后双仓 git status 全等）俱绿。
- MCP 客户端形：官方 mcp SDK `streamable_http_client`（mcp 1.30.0 新名，旧名 streamablehttp_client 已弃用警告故用新名），initialize → tools/list → call_tool 全序走通。

## 管线读数

- 化格：formatter packs/general-v1 对本档（sih-tools 域代码与 README 走 mcpline 域声明豁免即 mcpserv-solo 红线四先例，不入管线）。
- 核阅：des-001 对本档（sih/event/plan 域外）退出码二域外如实记档。
- 检词：nomenclator packs/core 对本档。

## 红证留痕

- test_web_smoke 首跑红：import 漏 `assert_for_readonly_face`（NameError，1 failed 3 passed），红证归档 mcpweb-solo-materials/tdd-web-first-red.log，修 import 复跑 4 绿，零删除零清洗。
- 管道掩码坑自接：先前批两笔 tail/tee 掩码教训在案，本批全链 RC 即取（zsh 无 PIPESTATUS，统一改 `> log 2>&1; echo EXIT=$?` 形），零假码推进。

## 越线与误差申报

- 零越线：写入面俱在任务包 allow 清单内；sih-visual/**、scribe、lease、gauge、critsweep、nomenclator、basemgr、SPEC-023、.zcode 配置零触碰；stdio 形态与 SPEC-023 契约零改（α 五工具出入参形零动，β 面矩阵零动）。
- 零新增判定语义零 LLM：web 面是既有 alpha 只读工具的传输投影，端口与静态目录旗标解析为 argparse 机械形。
- 勘误承接：书简意图按 2026-09-04 闸三勘误带 `--sessions` 参一次过；lease open 裸 stem 包名按 openhyg 勘误 TASK_PACKAGE_DIRS 解析命中 state/plan 位。

## 收约读数

- 收约与对账读数（close 链闸与 reconcile 双零与链 verify）随收约链笔留档，完工回报向主窗转述。
