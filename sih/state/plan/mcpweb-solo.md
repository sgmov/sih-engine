# mcpweb-solo 批任务包：mcpline HTTP 传输面——与视图面板同端口异路径统一服务

> 令源：用户 2026-09-09 令「我们视图部分是网页的一个服务，我们应该把MCP也做成一个HTTP的方式，使用的同一端口，但是使用的是不同的API」
> 形：solo 批独立立约独立收约，工具线代码批

## 一、使命

mcpline 增统一 web 服务形态：单端口同时承载视图面板静态页与 MCP streamable HTTP 面，异路径即视图在根路径、MCP 在 /mcp。零改 sih-visual 任何文件。

## 二、设计钉（照此实装）

1. 统一入口：python -m mcpline.web（或等价命名），ASGI 形——mcp SDK 的 streamable_http_app 挂 /mcp，视图静态目录挂 /；端口旗标缺省 8765 可覆写（避 SiInfer 在役服务口 8100 与 8200 与 8190 与 8191 与 3080）；静态目录旗标缺省指 sih-visual/assets/viewer-dashboard-2026-09-06/
2. HTTP 面只读红线（安全边界非可选优化）：HTTP 传输只暴露 α 五只读工具 chain_query 与 chain_verify 与 critsweep 与 heartbeat 与 locks_read；β 十写工具仅 stdio 面在列——理由即 DES-014 身份与会话模型绑进程，HTTP 端口无进程身份，未认证写面不得上网络口
3. stdio 形态零回归：原 python -m mcpline 十五具面照旧（本窗与冷验通道依赖它）
4. mcpline 版本 additive bump 0.2.0 改 0.3.0，测试族全跑（recclsf 条款：版本嵌入报告处全数复核）

## 三、验收

- HTTP 冒烟：起服后 GET / 得面板 HTML（200 且含面板标识串）；MCP streamable HTTP 客户端 initialize 加 tools/list 断言恰五只读；chain_verify 调用 valid；β 写工具 HTTP 面结构性缺席断言
- stdio 回归：十五具与五只读探针与 fixture 写径冒烟照旧全绿；零写证明照旧（测试前后双仓 git status 全等）
- 认证上链、双仓 settle、reconcile 双零

## 四、红线与禁触

1. 禁触 sih-visual/**（零接触，静态目录只读引用）
2. 禁触既有工具代码（scribe、lease、gauge、critsweep、nomenclator、basemgr）与 SPEC-023（传输无关契约档）与 .zcode 配置
3. 零新增判定语义零 LLM；DES-014 红线对本批继续有效
4. 与主窗发布收尾并行：锁面含 sih-engine/sih/event/trail/2026-09-09.ndjson，冲突走 lease wait-turn 禁绕行禁 preempt

## 五、写入面（allow 清单）

- sih-tools/mcpline/**（含 tests 扩展与 pyproject 版本位）
- sih-engine/sih/state/plan/mcpweb-solo.md 与 mcpweb-solo-prompt.md
- sih-engine/sih/event/plan/mcpweb-solo-results.md 与 mcpweb-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-mcpweb-1/

## 六、完工回报形

批名、座位号、链笔哈希、双仓 commit 哈希、scribe verify 全文、reconcile 增量、HTTP 冒烟读数（路径、端口、工具数、验链判词）、HTTP 只读红线断言读数、stdio 回归读数、启动命令行形（README 形一句）。
