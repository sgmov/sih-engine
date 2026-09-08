# mcpserv-solo 批任务包：mcpline α 相只读面 MCP 服务器实装

> 线：mcpline（MCP 实装线，线程序包 sih-engine/sih/state/plan/mcpline-line-v1.md）
> 令源：用户 2026-09-09 令「你拉起多子代理，进行司衡引擎的mcp开发任务」；线内批一 α 相实装批拆三批承载，本批即其二
> 形：solo 批独立立约独立收约，与 mcpspec-solo 并行（写入面不相交）；五工具契约以本包第二节为实施正典，mcpspec-solo 落的 SPEC-023 为对表正典，两批同源同表

## 一、使命

实装 MCP 服务器 sih：位于 sih-tools/mcpline/，stdio 传输，暴露且只暴露五个只读工具，工具描述自足、报错即教学，附单元测试与零写入证明与 stdio 客户端冒烟测试。

## 二、五工具实施契约（与 mcpspec-solo 包同表）

| 工具名 | 入参 | 出参 | 承接 CLI（只读） |
|---|---|---|---|
| chain_query | date（YYYY-MM-DD，缺省实日）、event_type（可选过滤） | 当日链事件清单（哈希、事件型、主体字段） | sih-engine/target/debug/scribe query --trail sih-engine/sih/event/trail/<date>.ndjson |
| chain_verify | date | 逐笔校验与整链 valid 判词 | scribe verify --trail <同上> |
| critsweep | date | 五判据三态与泊界路由与两账本在飞，严格 JSON 单对象 | python3 sih-tools/critsweep/sweep.py --at <date> --root <工作区根>（stdout 捕获） |
| heartbeat | 无 | 秤星三维最新读数与距上快照间隔日 | cd sih-tools/gauge 且 PYTHONPATH=src python3 -m gauge.cli read（只读不落链） |
| locks_read | 无 | 未释放锁成对核算与活跃会话数 | uv run --project sih-tools/lease lease status（查册） |

- 工作区根解析：环境变量 SIH_ROOT，缺省取 sih-tools/mcpline 上两级
- 错误载荷四字段：error、what_this_tool_does、valid_params、canonical_pointers
- 工具描述自足：双语一句话加正典指针（指 SPEC-023 与线程序包路径）

## 三、红线（实现与测试须逐条断言）

1. 零写入：服务器代码不出现任何写模式子命令（scribe append 与 intent 与 park 与 record、gauge record、lease 写动词 lock 与 unlock 与 close 与 commit 等）；测试含一条守卫断言：对工具源码文本 grep 写动词零命中
2. 零写入证明：测试跑完五工具前后 git status 对表，sih-engine 与 sih-tools 两仓工作树洁净（ledger 尾笔类既有脏件可白名单豁免，豁免清单入测试注释）
3. 不新增第二个执行者：无状态，无重试改写，无判定语义，读数即投影
4. sih-tools 域外于引擎文档规范，mcpline/ 代码与其 README 不走化格核阅检词，但本批机械链序照走

## 四、技术形

- uv 项目：sih-tools/mcpline/pyproject.toml，依赖官方 mcp SDK（FastMCP），入口 python -m mcpline，stdio
- 测试：tests/ 下单元测试（每工具一函数级用例加一错误路径用例）加零写入守卫测试加 stdio 客户端冒烟测试（起服务器进程、列工具、逐个调用、断言出参形）
- 冒烟测试基准日取当日，critsweep 断言返回严格 JSON 单对象且含五判据键

## 五、写入面（allow 清单）

- sih-tools/mcpline/**（全新区）
- sih-engine/sih/state/plan/mcpserv-solo.md 与 mcpserv-solo-prompt.md（本包与派单）
- sih-engine/sih/event/plan/mcpserv-solo-results.md 与 mcpserv-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson（链笔）
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-mcpserv-1/（批机械链侧写）

禁触：sih-tools 既有工具目录（critsweep、gauge、lease、scribe 代码零改动——五承接路径全用现成只读形，不需要任何旗标增补）、sih-engine/doc/**（SPEC 归 mcpspec-solo）、工作区 .zcode 配置（注册归冷验批）

## 六、机械链序

- 全序照 sih-tools/BATCH-FACE.md verbatim 执行；每条命令立即取退出码，失败即停，禁管道掩码
- commit 须指向登记 worktree；closeguard pre-close commit 形照旧
- trail 锁冲突即 lease wait-turn 排队，禁绕行；与 mcpspec-solo 并行共用 trail，写前确认锁在
- scribe 二进制一律主树 sih-engine/target/debug/scribe

## 七、验收

- 五工具冒烟全绿；零写入守卫与零写入证明双测试过
- 认证上链、双仓 settle、reconcile 增量合规、worktree 拆净

## 八、完工回报形（主窗独立复算用）

批名、座位号、intent 与 seat 与 cert 与 exit 链笔哈希、双仓 commit 哈希、scribe verify 全文输出、reconcile 增量、五工具冒烟输出摘录、零写入守卫测试输出。
