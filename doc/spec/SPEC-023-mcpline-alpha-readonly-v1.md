# SPEC-023 mcpline alpha 相只读面契约：读数工具 MCP 面契约正典 v1 五具修订一起七具修订二起八具修订三起九具

本规范承 mcpline 线程序包 v1 与 mcpspec-solo 批任务包成文，令源即用户 2026-09-09 令「你拉起多子代理，进行司衡引擎的mcp开发任务」，线立项承 pk-077 出泊 promoted 与得一终签 05353825 在链。本规范把 MCP alpha 相五读数工具的名称、入参、出参、错误语义、零写入红线、冷 agent 验收程序一次定稿，作为 MCP 实装批与冷 agent 验收批的对表正典。任务包第二节至第五节照录不增删。相名转写声明：线程序包原文以希腊字母书写相名，本文承域字符集闸一律拉丁转写为 alpha 相与 beta 相，指称同一。

## 概览 {#overview}

- 读数工具自修订三起九具即 chain_query 与 chain_verify 与 critsweep 与 heartbeat 与 locks_read 加 naming_guide 与 nomenclator_query 与 nomenclator_check 加 retriever_recall；两检词具随 mcpnomgate-solo 批入面，温故检索具随 wengumcp-parallel 批入面，立名指引具随 nomsupply 批入面；承接路径全是只读子命令或静态教学件，MCP 面零写入零治理语义::[工具契约](#tools)
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
- 出参：当日链事件清单即哈希、事件型、主体字段；matches=全日笔数、returned=过滤命中数即 MCP 面，承接 CLI 面 scribe query 同名字段 matches=过滤命中数，同名字段两面语义显式申报见修订五
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

### naming_guide {#naming-guide}

- 入参：无
- 出参：立名指引静态教学五段即立名五步形与 DEC-017 修订四修订五修订六指针与检词六态语义与 stem 闸拒教认领三语义与死档禁条，附正典指针与零裁决声明
- 承接：静态教学件零 CLI 承接；修订三增随 nomsupply 批，只读零裁决零 LLM 零写入，语义忠实终裁归立名程序人节点

### nomenclator_query {#nomenclator-query}

- 入参：word 即待查词
- 出参：术语六态即已立与懒波与死档与候补与未知附出处指针与词条摘要
- 承接 CLI 只读：`uv run --project sih-tools/nomenclator nomenclator query --pack sih-tools/nomenclator/packs/core --word <word>`；修订一随 mcpnomgate-solo 批增，命名动作前查册义务的机械承载位

### nomenclator_check {#nomenclator-check}

- 入参：target 即目标文档路径，工作区根相对或绝对形
- 出参：死档禁字级与懒波词两规则检出的行号与摘录入报告
- 承接 CLI 只读：`uv run --project sih-tools/nomenclator nomenclator check --pack sih-tools/nomenclator/packs/core <target>`；修订一增

### retriever_recall {#retriever-recall}

- 入参：topic 数组可选、word 数组可选、event 数组可选、since 与 until 与 archive 与 at 可选；四轴即 topic 与 word 与 event 与 time 轴，俱缺即拒教学不 spawn
- 出参：stdout NDJSON 原文透传、退出码三值透传即零成功一拦二异常、stderr 透传；out 与 miss-log 不投影即工具面零写盘
- 承接 CLI 只读：`sih-engine/target/debug/retriever recall ... --root <域根>`，cwd 不承根以 --root 显式传；修订二增，只报不判承 DEC-017 既判，零判定语义新增

### 错误语义与描述自足 {#errors} {#errors}

- 报错即教学：每错误载荷含 error 与 what_this_tool_does 与 valid_params 与 canonical_pointers 四字段；canonical_pointers 即承接 CLI 与 SPEC 位
- 工具描述自足：描述文本双语一句话加正典指针

## 红线 {#redlines}

1. alpha 相零写入零治理语义：MCP 面不新增第二个执行者，判定语义正典留确定性程序；各项承接路径全是只读子命令或静态教学件，v1 五项修订一起七项修订二起八项修订三起九项
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

## 修订记录 {#revisions}

2026-09-10 修订一：随 mcpnomgate-solo 批增两检词具即 nomenclator_query 与 nomenclator_check，令源用户 2026-09-10 令「同意，这个是既有功能没上mcp」承 DEC-017 修订四常设纪律即工程命名动作前查命名集与检词，执行位落 MCP 面；alpha 相读数面五具扩七具，词典中央位域无涉，register 不投影即登记立名入口红线；stdio 面 17 具升 19 具、HTTP 面 15 具升 17 具同步在 mcpline 0.8.0 实装。

2026-09-11 修订二：随 wengumcp-parallel 批增温故检索具即 retriever_recall，令源用户 2026-09-11 令「同意。开始M5的修复，你拉子代理」承司梦侧 M5 发现即温故未上 MCP 面与索引不覆盖新城域；alpha 相读数面七具扩八具，只报不判承 DEC-017 既判，零判定语义新增零新执行者，out 与 miss-log 不投影即工具面零写盘；承接 CLI 形 `sih-engine/target/debug/retriever recall ... --root <域根>`，cwd 不承根以 --root 显式传即 stdio 面中央根与 HTTP 面所绑域根；stdio 面 19 具升 20 具、HTTP 面 17 具升 18 具同步在 mcpline 0.9.0 实装。

2026-09-11 修订三：随 nomsupply 批增立名指引具即 naming_guide，令源 pk-090 出泊即用户 2026-09-11 令「拉起子代理逐项清账，可以并行」承清账并联批 debtclear-parallel 簇A；alpha 相读数面八具扩九具，内容五段静态教学即立名五步形与 DEC-017 修订指针与检词六态语义与 stem 闸拒教认领三语义与死档禁条，只读零裁决零 LLM 零写入，语义忠实终裁归立名程序人节点；stdio 外部面 18 具升 19 具、HTTP 面 18 具升 19 具同步在 mcpline 0.10.0 实装；同批件三检词包按域解析修订 nomenclator_query 与 nomenclator_check 两具即 canonical 域优先域内词典包回退中央包，件二 unknown 与错误载荷附立名程序摘要，DEC-017 修订六同批在册。

2026-09-11 修订四：随 sihmcp-solo 批载体形变更，令源用户 2026-09-11 令「rmcp在sihankor旧仓开发的时候就是定下技术栈选型，直接替换。」承 DEC-023 载体决策；MCP 线载体由 Python FastMCP 实装 sih-tools/mcpline 换 Rust rmcp 原生承载，引擎 bin sihmcp 加 lib 模块 src/mcpserver 承 DEC-001 源码位；本契九具工具契约与红线与验收程序零变化，行为对等由双载体对表电池判词承 mcpdual-parallel 先例形，材料 sih-engine/sih/event/plan/sihmcp-solo/materials/ 在档，alpha 九用例与 beta 生命周期八步与错误路径五步俱判词一致；chain_query 与 chain_verify 两具承接形由 scribe 子进程改引擎进程内库调 event_stream 即消子进程缺陷族，承接 CLI 指针保留作正典读形；载体名 serverInfo sihmcp 承 DEC-023 甲表认领候立名程序人节点终裁。
2026-09-17 修订五：随 spec023-matches-solo 批注明 matches 字段两面语义，令源 pk-083 出泊裁定承用户 2026-09-17 令「裁，多子代理并行」，双面注明形零破面，统一字段形不采：MCP 面 chain_query 出参 matches=全日笔数加 returned=过滤命中数，CLI 面 scribe query --trail 出参 matches=过滤命中数即零命中形 matches=0 退出码一。同名字段两面语义自此显式申报，两面事件集三路一致零语义损，mcpdual-parallel 批对表在档即发现一，字段语义差异为两面接口事实申报非缺陷；工具契约节 chain_query 条目出参行同批补注，两面代码零改动。
