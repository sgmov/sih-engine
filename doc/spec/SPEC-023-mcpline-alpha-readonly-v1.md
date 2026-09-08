# SPEC-023 mcpline alpha 相只读面契约：五读数工具 MCP 面契约正典

本规范承 mcpline 线程序包 v1 与 mcpspec-solo 批任务包成文，令源即用户 2026-09-09 令「你拉起多子代理，进行司衡引擎的mcp开发任务」，线立项承 pk-077 出泊 promoted 与得一终签 05353825 在链。本规范把 MCP alpha 相五读数工具的名称、入参、出参、错误语义、零写入红线、冷 agent 验收程序一次定稿，作为 MCP 实装批与冷 agent 验收批的对表正典。任务包第二节至第五节照录不增删。相名转写声明：线程序包原文以希腊字母书写相名，本文承域字符集闸一律拉丁转写为 alpha 相与 beta 相，指称同一。

## 概览 {#overview}

- 五读数工具即 chain_query 与 chain_verify 与 critsweep 与 heartbeat 与 locks_read，承接路径全是只读子命令，MCP 面零写入零治理语义::[工具契约](#tools)
- 报错即教学即每错误载荷四字段，工具描述自足双语一句话加正典指针::[错误语义与描述自足](#errors)
- 红线四条逐条载即零写入零治理语义与 critsweep stdout 捕获零落盘与 beta 相指针与自有运行时归 pk-079::[红线](#redlines)
- 冷 agent 验收判据与四步程序形与红证形逐条载::[验收程序](#acceptance)
- 范围收敛即本规范只承载契约档零实装设计，实装归 MCP 实装批::[问题陈述](#problem)

## 问题陈述 {#problem}

- 治理交互面现由会话内 agent 承载，零上下文外部 agent 无标准接入面；MCP 系跨工具标准，拉式即 agent 调用才有数据，承线程序包问题陈述节照录。
- alpha 相只读面先行即零写入零治理语义先行；写面候 beta 相须先过会话与租约映射安全模型设计批，未过不开工；注入面通用化终局即自有对话循环归 pk-079 分相承载，不并入本线。
- 范围收敛声明：本规范 v1 只承载 alpha 相五读数工具契约档，零 MCP 代码零实装设计零注入面；判定语义正典留确定性程序，MCP 面不新增第二个执行者。

## 工具契约 {#tools}

契约全表内容照任务包第二节逐项转写为逐工具条目形；域格式闸禁表格，条目形与表格形内容同一，四栏即工具名与入参与出参与承接 CLI 只读。

### chain_query {#chain-query}

- 入参：date 即 YYYY-MM-DD 缺省实日；event_type 可选过滤
- 出参：当日链事件清单即哈希、事件型、主体字段
- 承接 CLI 只读：`sih-engine/target/debug/scribe query --trail sih-engine/sih/event/trail/<date>.ndjson`

### chain_verify {#chain-verify}

- 入参：date
- 出参：逐笔校验与整链 valid 判词
- 承接 CLI 只读：`scribe verify --trail <同上>`；同上即 chain_query 项所载 trail 路径形

### critsweep {#critsweep}

- 入参：date
- 出参：五判据三态与泊界路由与两账本在飞，严格 JSON 单对象
- 承接 CLI 只读：`python3 sih-tools/critsweep/sweep.py --at <date> --root <工作区根>`，stdout 捕获

### heartbeat {#heartbeat}

- 入参：无
- 出参：秤星三维最新读数与距上快照间隔日
- 承接 CLI 只读：`cd sih-tools/gauge 且 PYTHONPATH=src python3 -m gauge.cli read`，只读不落链

### locks_read {#locks-read}

- 入参：无
- 出参：未释放锁成对核算与活跃会话数
- 承接 CLI 只读：`uv run --project sih-tools/lease lease status`，查册

### 错误语义与描述自足 {#errors}

- 报错即教学：每错误载荷含 error 与 what_this_tool_does 与 valid_params 与 canonical_pointers 四字段；canonical_pointers 即承接 CLI 与 SPEC 位
- 工具描述自足：描述文本双语一句话加正典指针

## 红线 {#redlines}

1. alpha 相零写入零治理语义：MCP 面不新增第二个执行者，判定语义正典留确定性程序；五项承接路径全是只读子命令
2. critsweep 经 stdout 捕获，不写 sweep-latest.json 或任何仓内文件；实装批须以 git status 洁净证零写入
3. beta 相写面不在本 SPEC 范围，只留本句指针到线程序包批二即会话与租约映射安全模型设计批未过不开工，见 sih-engine/sih/state/plan/mcpline-line-v1.md
4. 自有运行时归 pk-079，本 SPEC 不涉注入面

## 验收程序 {#acceptance}

线级验收判据照录：冷 agent 零上下文经 MCP 面零辅助完成一次被治理交互且链 verify valid。SPEC 载程序形四步：

1. 注册 sih server 入工作区配置
2. 起零上下文子代理只给任务句
3. 断言完成链查询与验链且 verify valid
4. 全 transcript 入档

验收判据红证形逐条：

- 断言腿红证：构造子代理需会话上下文或人工辅助方能完成链查询与验链的实录，即零辅助判据红；构造链 verify 非 valid 而断言通过的实录，即断言腿伪
- 零写入腿红证：构造 MCP 工具调用后 git status 不洁或 sweep-latest.json 在盘的实录，即零写入红线红

## 必读文件 {#read}

- sih-engine/sih/state/plan/mcpspec-solo.md 本批任务包
- sih-engine/sih/state/plan/mcpline-line-v1.md 线程序包
- sih-engine/doc/spec/SPEC-TEMPLATE-sufficiency-v1.md 充分性模板
- sih-tools/BATCH-FACE.md 批机械链命令面正典
- sih-engine/sih/state/parking/materials/pk-077.json 线立项令源
- sih-engine/sih/event/plan/mcpopen-solo-results.md 线立项登记批结果档
- 五工具承接命令面即工具契约节各条所载 CLI 位

## 内容充分性 {#sufficiency}

- 判据红证对表：验收程序节红证形逐条即断言腿与零写入腿两构造形，任一构造实录在档即令对应判据红，红证构造性在档如实申报
- 判定性常数挂锚对表：本文无声明的判定性常数；des-001 C007 与 C008 与 C009 行级与邻近级闸在役核验
- 约束算子对表：本文无约束算子面，如实申报
