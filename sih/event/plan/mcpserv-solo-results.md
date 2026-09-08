# mcpserv-solo 结果档：mcpline α 相只读面 MCP 服务器实装批

> 承接：任务包 mcpserv-solo.md（第二节五工具契约表为实施正典，mcpspec-solo 落的 SPEC-023 为对表正典，两批同源同表）与线程序包 mcpline-line-v1.md（红线源头）；派单令源用户 2026-09-09「你拉起多子代理，进行司衡引擎的mcp开发任务」。
> 队形单线 solo 委外执行，批日 2026-09-09。会话号 sess-zcode-260909-mcpserv（lease 会话 90ab7417cb4410dd，双仓 worktrees msh/mcpserv-solo）。与 mcpspec-solo（会话 bd45929ddeb6ff5e）并行在飞共用当日链。

## 意图锚定

- 意图事件：intent_refined，event_hash 前 8 `bba35d36`（event_id 24dbf2ba-0f34-44e2-8b2c-16f0e7378715，闸三带 --session 与 --sessions 首跑即过）
- 双门：核阅 ask3 包 exit 0 零违规 findings 0（content_hash 65fd1436）；ask3repeater exit 0 status ok anchor_count 3；三锚引文程序切片自 sih-philosophy 原文（07-on-assay.md:55 与 01-ontology-of-names.md:18 与 08-on-settle.md:110），逐字节子串内嵌，生成器 make_ask3_mcpserv-solo.py 随报告目录提交
- 叩问：八轻信号 8/8（FastMCP、MCP 服务器、stdio、五工具、冒烟测试、只读面、锁面读数、零写入）digest passed covered 8，处置八行俱「不立名不登记」描述性使用
- 正身：identity verify anomalies 0，identity.hash 前 8 `60d0a940`（core_hash e1c9a7fa）
- ask3 记录 sha256：`65fd1436c7b610e4d92e2e23eaad0fb14357ce92765a7ec3531086ff8c25c078`（核阅 content_hashes 与租约 issued 事件两处一致）

## 前置与开工读数

- 直提守卫：双仓 core.hooksPath 俱指 sih-tools/lease/hooks 在位
- 锁面开工：开工前现势零本批路径持锁；open 后九路径锁全取零失败零排队（sih-tools/mcpline、state/plan 两件、event/plan 结果档与材料目录、trail 2026-09-09、报告两目录、DES m-mcpserv-1）；共享面（trail 与报告两目录）与 mcpspec-solo 双会话并存俱共享锁形，scribe 写前确认锁在，零绕行零 preempt 零 wait-turn 排队实际发生
- 主窗预落两件（mcpserv-solo.md 与 mcpserv-solo-prompt.md）主树零改动随批提交
- 回锚与判据扫与例行读数等会话启动节律归主窗节律位，本委外批零代跑如实申报

## 实装读数（链步六）

- 工地：worktrees/sih-tools/mcpserv-solo/mcpline/ 全新区；形即 uv 项目 pyproject.toml（依赖官方 mcp SDK FastMCP，钉 mcp>=1.2.0,<2，FastMCP 为 v1 线 API）+ src/mcpline（runtime 与 server 与 __main__）+ tests 三族 + README + .gitignore，入口 python -m mcpline，stdio 传输
- 五工具契约逐格实施（照任务包第二节照录不增删）：

| 工具 | 承接 CLI（只读） | 出参投影 |
|---|---|---|
| chain_query | sih-engine/target/debug/scribe query --trail <date> | 当日链事件清单（event_hash、event_type、subject、doc_id、timestamp），date 缺省实日，event_type 可选过滤 |
| chain_verify | scribe verify --trail <date> | status/valid 判词与 events 数与首尾哈希 |
| critsweep | python3 sih-tools/critsweep/sweep.py --at <date> --root <root> | 严格 JSON 单对象原样投影（criteria 五判据与 parking 双线与 inflight 两账本） |
| heartbeat | cd sih-tools/gauge 且 PYTHONPATH=src python3 -m gauge.cli read（只读不落链） | 三维最新读数与 last_snapshot_at 与 days_since_last_snapshot |
| locks_read | uv run --project sih-tools/lease lease status（查册） | held_count 与 held_locks 成对核算与 active_sessions 数 |

- 工作区根解析 SIH_ROOT 缺省取 sih-tools/mcpline 上两级；错误载荷四字段 error 与 what_this_tool_does 与 valid_params 与 canonical_pointers（报错即教学）；工具描述双语一句话加正典指针（SPEC-023 与线程序包路径）
- 测试：14 绿全过（stdio 客户端冒烟 1 即起服务器进程列工具断言恰五工具逐个调用断言出参形 + 单元 11 即每工具函数级与错误路径用例 + 零写入双证 2）；冒烟基准日取当日，critsweep 断言严格 JSON 单对象含五判据键（GOV2-C1 至 C5 全在）
- 红绿线：五跑四红一绿全量归档 materials/tdd-red-green.md 与 pytest-final-green.log（先红留痕纪律）；四红依次为 mcp 2.x 移除 FastMCP（钉 <2 修复）、VIRTUAL_ENV 继承污染（clean_env 扩剔修复）、lease 工地自检卫（cwd 定工作区根修复）、bogus-root cwd 缺席 OSError（run_readonly 收编 rc=127 修复）
- critsweep 与 gauge 与 lease 与 scribe 四承接 CLI 代码零改动，五承接路径全用现成只读形零旗标增补

## 管线读数（链步七）

- 结果档（本件）：化格与核阅与检词三步照跑，读数见认证清单表下管线行
- mcpline 代码与其 README：sih-tools 域外于引擎文档规范，红线四明文免化格核阅检词，照录申报零三步
- 书单对表：本批产出引用零推导档引用 ID（纯工程实装批，零数学书单引用），10.5 节 checkcite 无对表对象，如实申报

## 认证实录（链步八，逐件经引擎 scribe append，meter 包裹，闸三带 --session 与 --sessions）

| 件 | 认证哈希前 8 |
|---|---|
| ask3 记录 | a80b1772 |
| 验证件 | d4c1caf4 |
| 正身件 | c873c680 |
| manifest 内容哈希清单件 | 见投影件 mcpserv-solo.json 内载与完工回报（settle cert 同取此值，补笔回填本表） |
| 投影件 mcpserv-solo.json | 见完工回报（补笔回填本表） |

md 件直证承先例走内容哈希清单件（manifest 覆盖任务包两件与结果档与 tdd-red-green.md 四件，定格 settle 前实值；收约补笔后哈希漂移如实申报）。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 五工具契约照录 | 实装 | 暴露且只暴露五只读工具，契约表逐格不增删 | 通过（冒烟断言工具名清单恰五；契约表逐格实施见实装读数节） |
| F-2 零写入红线 | 治理 | 源码 grep 写动词守卫与 git status 前后对表双证 | 通过（test_zero_write_guard_source_grep 与 test_zero_write_proof_git_status_unchanged 双绿；四承接 CLI 代码零改动） |
| F-3 不新增第二执行者 | 治理 | 无状态零重试改写零判定语义读数即投影 | 通过（五工具俱现成只读 CLI 出参投影，错误载荷四字段即教学转述） |
| F-4 写入仅 allow | 治理 | 写入仅九路径 allow 面内加双工地加链经 scribe | 通过（报告两目录承前裁惯例；主窗预落两件零改动随批提交；m-mcpserv-1 预占零写入如实申报） |
| F-5 链面全绿 | 治理 | 双仓 settle、close 零失败、verify valid、reconcile 零新增 | 通过（读数见结算读数节，收约补笔载终值） |

## 越线与误差申报

- 叩问 check 先于 ask3 记录定稿一跑：预登记实测信号数（8/8 失配规避，reroute record-v2 坑位规避）；链序闸门位序不变（双门与 digest 俱在记录落档后跑），如实申报
- m-mcpserv-1 DES 路径 allow 预占零写入：本批任务包无得一测量腿（线级得一裁 m-mcpopen-1 已在立项批终签），照录不增删，预占位如实申报零写入
- 首跑红证四跑归档（mcp 2.x 红与 VIRTUAL_ENV 红与 self_boot 红与 OSError 红），先红留痕纪律执行在档
- locks_read 子进程 cwd 定工作区根：lease 工地自检卫（rootanchor-solo 勘误位）对工位 cwd 要求三参全传，查册是只读形非工地自举写账，cwd 定根承该卫语义，承接参数形零改动，如实申报
- manifest 与投影件认证哈希未入本表定稿：两件认证序在本件 manifest 定格之后，哈希经投影件与完工回报承载并收约补笔回填，如实申报
- 其余误差零申报（闸三首跑即过；九锁零冲突零排队；管线三步读数见下；零写入证明测试零撞窗）

## 大白话节

- **本批做了什么（说人话）**：给司衡引擎做了一个「只读查询窗口」。外部 agent 以后不用懂司衡内部工具，拿标准 MCP 协议就能问五件事：某天的账本记了什么（chain_query）、账本对不对得上（chain_verify）、五条治理判据现在什么状态（critsweep）、引擎的心跳读数（heartbeat）、现在谁拿着锁（locks_read）。只会答，不会改，服务器里连一个写文件的动词都搜不到（有测试专门搜）。
- **零写入怎么证明（说人话）**：两道证明。一道搜源码：把服务器代码全文搜一遍写动作的词，一个都不许有；一道跑前后对比：五个工具全部调一遍，调之前和调之后分别给两个代码仓拍照，两张照片必须一模一样。
- **报错即教学（说人话）**：工具出错时不甩一句「失败了」，而是四句话一起说：错在哪、这工具是干嘛的、参数该怎么给、去哪查正典文档。
- **本批不做什么（说人话）**：不碰四个底层工具的代码、不碰 SPEC 文档（归并行批 mcpserv-solo 的姊妹批 mcpspec-solo 落）、不做写操作（β 相要等安全模型设计批）。

## 投影件路径

- 机器可读投影：sih-engine/sih/event/plan/mcpserv-solo-materials/mcpserv-solo.json
- manifest 内容哈希清单件：sih-engine/sih/event/plan/mcpserv-solo-materials/manifest-sha256.json
- 红绿线：sih-engine/sih/event/plan/mcpserv-solo-materials/tdd-red-green.md 与 pytest-final-green.log
- smoke 摘录：sih-engine/sih/event/plan/mcpserv-solo-materials/smoke-excerpts.json（生成器 make_smoke_excerpts.py 同目录）
- ask3 生成器：sih-tools/scribe/reports/make_ask3_mcpserv-solo.py（随批提交）
- 实装区：sih-tools/mcpline/（pyproject 与 src/mcpline 与 tests 三族与 README）
