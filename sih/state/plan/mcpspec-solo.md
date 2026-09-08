# mcpspec-solo 批任务包：mcpline α 相只读面契约档 SPEC-023

> 线：mcpline（MCP 实装线，线程序包 sih-engine/sih/state/plan/mcpline-line-v1.md）
> 令源：用户 2026-09-09 令「你拉起多子代理，进行司衡引擎的mcp开发任务」；线内批一 α 相实装批拆三批承载，本批即其一
> 形：solo 批独立立约独立收约，与 mcpserv-solo 并行（写入面不相交）

## 一、使命

立 α 相只读面契约正典：写 sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md，把五读数工具的名称、入参、出参、错误语义、零写入红线、冷 agent 验收程序一次定稿，作为 MCP 实装批与冷 agent 验收批的对表正典。

## 二、五工具契约（包内定稿，SPEC 照此正典化）

| 工具名 | 入参 | 出参 | 承接 CLI（只读） |
|---|---|---|---|
| chain_query | date（YYYY-MM-DD，缺省实日）、event_type（可选过滤） | 当日链事件清单（哈希、事件型、主体字段） | sih-engine/target/debug/scribe query --trail sih-engine/sih/event/trail/<date>.ndjson |
| chain_verify | date | 逐笔校验与整链 valid 判词 | scribe verify --trail <同上> |
| critsweep | date | 五判据三态与泊界路由与两账本在飞，严格 JSON 单对象 | python3 sih-tools/critsweep/sweep.py --at <date> --root <工作区根>（stdout 捕获） |
| heartbeat | 无 | 秤星三维最新读数与距上快照间隔日 | cd sih-tools/gauge 且 PYTHONPATH=src python3 -m gauge.cli read（只读不落链） |
| locks_read | 无 | 未释放锁成对核算与活跃会话数 | uv run --project sih-tools/lease lease status（查册） |

错误语义（报错即教学）：每错误载荷含 error、what_this_tool_does、valid_params、canonical_pointers（承接 CLI 与 SPEC 位）四字段。工具描述自足：描述文本双语一句话加正典指针。

## 三、红线（SPEC 须逐条载）

1. α 相零写入零治理语义：MCP 面不新增第二个执行者，判定语义正典留确定性程序；上表五承接路径全是只读子命令
2. critsweep 经 stdout 捕获，不写 sweep-latest.json 或任何仓内文件；实装批须以 git status 洁净证零写入
3. β 相写面不在本 SPEC 范围，只留一句指针到线程序包批二
4. 自有运行时归 pk-079，本 SPEC 不涉注入面

## 四、冷 agent 验收程序（SPEC 载正典形）

线级验收判据照录：冷 agent 零上下文经 MCP 面零辅助完成一次被治理交互且链 verify valid。SPEC 载程序形：注册 sih server 入工作区配置 → 起零上下文子代理只给任务句 → 断言完成链查询与验链且 verify valid → 全 transcript 入档。

## 五、写入面（allow 清单）

- sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md（新）
- sih-engine/sih/state/plan/mcpspec-solo.md 与 mcpspec-solo-prompt.md（本包与派单）
- sih-engine/sih/event/plan/mcpspec-solo-results.md 与 mcpspec-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson（链笔）
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-mcpspec-1/（批机械链侧写）

## 六、机械链序与格式义务

- 全序照 sih-tools/BATCH-FACE.md verbatim 执行；每条命令立即取退出码，失败即停，禁管道掩码
- SPEC-023 属 des-001 域（sih-engine/doc）：化格→核阅→检词→认证序固定；格式红线：无围栏代码块（mermaid 与 filetree 例外）、单 H1、首 H2 为概览、禁 U+2192、禁全角括号注内容、禁引用块、命令一律行内反引号
- SUFFICIENCY 模板照 doc/spec/SPEC-TEMPLATE-sufficiency-v1.md 充分性检查
- trail 锁冲突即 lease wait-turn 排队，禁绕行；与 mcpserv-solo 并行共用 trail，写前确认锁在

## 七、验收

- SPEC-023 五节齐：问题陈述、工具契约（第二节全表）、红线、验收程序、必读文件
- 化格 0、核阅 0、检词 0、认证上链、双仓 settle、reconcile 增量合规

## 八、完工回报形（主窗独立复算用）

批名、座位号、intent 与 seat 与 cert 与 exit 链笔哈希、双仓 commit 哈希、scribe verify 全文输出、reconcile 增量、SPEC-023 顶层小节清单。
