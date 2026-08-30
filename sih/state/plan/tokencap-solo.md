# tokencap-solo：句读令牌捕获清三笔期票批

> task-packages 治理任务
> 承接：sess-zcode-260830-tokencap 三问意图即 2026-08-30 链事件 0c5792f6、修正意图即同日链事件 4c8847fa 与 f5346728、用户同日同意令
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30，修订一即开工首跑发现前批缺陷并入修复、修订二即机械对账改四换名四补表

## 一、问题陈述 {#problem}

三笔期票在案。一即句读 impl 取名：mapping 声明 name_from IDENT 而树实况 impl 头全是规则节点，impl 条目名 None 实锤，泛型参数在头部件先于类型名即配平取中风险。二即级联 use 边：条目映射无 use 族即 rs 间 import 关系不在边域。三即级联运行期边账：四计数器随册零值在位、运行期计数未接线。三笔同源即都需解析树的细粒度取用。修订一增第四件：开工首跑发现 pk027expr 上线的 rust 包 lint 真退出码一，文法引用八词法名即 BANGEQ、DAMP、DBANG、DBAR、GE、LE、SHL、SHR 未入词法表，前批验收被管道掩码，v2 表达式层对比较与逻辑运算符由换名缺件而不可达，缺陷在本批并入修复。

## 二、关键设计 {#design}

五件。一引擎取名面最小扩展：name_from 增 text 形即条目取名复用既有文本派生机（raw、scalar_text、raw_lines、join_tokens、join_rules 全系可用），join_rules 增 direct 即限直接子规则不降深，langpack 校验同步收形，空腹纪律保持零语言知识。二 rust 包 mapping 修订三：impl_item 取名改 ty_bounds 加 ty 直接子区间以 for 连接合成，泛型参规则不进名即修 name None。三 use 边走级联自走树：承 SPEC-009 九 kind 钉死即 use 不入条目映射，级联在既有解析程内自走 use_item 节点取 use_tree 区间，符号末段唯一解析建代码到代码边，多义无主与通配入注记拦多不拦漏，自引排除。四运行期边账入 check 报告内嵌：四计数器即 consulted、upstream_changes、blocked、hits 为本次运行事实视图，零写保持即册与链零动。五词法缺口修复：机械对账即文法引用一百名缺八，四为既有记号的文法笔误即 e_cmp 的 BANGEQ 换 NE 与 e_and 的 DAMP 换 AND2 与 e_or 的 DBAR 换 OR2 与 e_unary 的 DBANG 换 AND2，四为真缺即 tokens.json 补 LE 与 GE 与 SHL 与 SHR 且序皆先于 LT 与 GT，lint 收口真退出码零，三件修订号随批升三。

## 三、工作清单 {#work}

- [ ] 引擎 name_from text 形与 join_rules direct 与校验面，先红后绿
- [ ] rust 包 mapping 修订三即 impl 取名、过滤对表、向量重冻
- [ ] 词法缺口四换名四补表、lint 真退出码零
- [ ] 级联 use 走树与符号解析建边与注记，先红后绿
- [ ] check 运行期边账内嵌与零写保持
- [ ] 契约修订二件与升版、AGENTS.md 工具行两处
- [ ] 管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 引擎取名面 | 工程治理 | name_from text 形与 join_rules direct 落 entries 与 langpack、先红后绿、markdown 与 json 包零动 |
| **F-2** impl 取名 | 工程治理 | impl 条目名非 None 即直接子规则区间合成、泛型参不进名、过滤对表即旧条目流除 impl 名外逐字节不变、向量重冻全过 |
| **F-3** use 边 | 工程治理 | use_item 自走树取区间、末段唯一解析建边、多义无主与通配入注记、自引排除、check 与 orphans 同表、双跑逐字节一致 |
| **F-4** 运行期边账 | 工程治理 | check 报告内嵌四计数器、复演确定、零写保持即册与链双跑零动 |
| **F-5** 词法缺口 | 工程治理 | lint 真退出码零、四换名四补表后八引用名全落既有或新增、v2 表达式层对比较与逻辑与移位运算符可达即树成节点、v1 过滤对表仍逐字节不变 |
| **F-6** 收口 | 链上治理 | 两契约修订与升版、认证入链、双仓结算收约、unrouted 零 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/parser/src/parser/entries.py 即投影机与文本派生
- 必读 2：sih-tools/parser/src/parser/langpack.py 即 schema 校验面
- 必读 3：sih-tools/cascade/src/cascade/core.py 即现行抽取与检查
- 必读 4：sih-engine/doc/spec/SPEC-009-parser-implementation-gap.md 即九 kind 钉死与深阶出版纪律

## 六、约束 {#constraints}

1. tokens 与 grammar 改动限词法缺口修复四补表与四换名即修订一二放开、原约束据链事件 4c8847fa 与 f5346728 撤销
2. 不破 SPEC-009 九 kind 即 use 不入条目映射
3. use 解析拦多不拦漏即多义无主与通配不建边入注记
4. check 与 orphans 零写路径保持
5. 双跑逐字节一致
6. 上链前必须等绿即 lint 真退出码零为硬条件

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过

## 八、风险点 {#risks}

符号末段同名多件常见即 use 边稀疏，如实入注记不硬配承语义边界节。impl 名变化使向量与对表基线重置即重冻与过滤对表如实记。别名取原名与通配跳过皆机械规则，语义损失拦多不拦漏。词法补件激活新记号使配平汤菜单或有不收即由 v2 表达式层接住、以 v1 过滤对表守不变。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 同意令即令牌捕获批
- 链件：sih/event/trail/2026-08-30.ndjson 即意图 0c5792f6 加修正意图 4c8847fa 与 f5346728
- 关联：pk027expr 与 pk033casc 两前批的期票段、两 CONTRACT、SPEC-009

## 十一、请求写入 {#requested-writes}

- sih-tools/parser/src/parser/
- sih-tools/parser/packs/rust/tokens.json
- sih-tools/parser/packs/rust/grammar.json
- sih-tools/parser/packs/rust/mapping.json
- sih-tools/parser/packs/rust/vectors/
- sih-tools/parser/tests/
- sih-tools/parser/CONTRACT.md
- sih-tools/cascade/src/cascade/
- sih-tools/cascade/tests/
- sih-tools/cascade/CONTRACT.md
- sih-tools/cascade/pyproject.toml
- sih-engine/sih/state/plan/tokencap-solo.md
- sih-engine/sih/event/plan/tokencap-solo-results.md
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-engine/doc/CASCADE.json
- AGENTS.md
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[impl 取名]: 消解 即召回面按登记主题轴零命中如实记、原文已直读核验在查档段
叩问处置[use]: 消解 即同上召回面机械零命中、use 边语义已从 CONTRACT 修订四与 SPEC-009 原文核验
叩问处置[令牌]: 消解 即工作名即解析树细粒度取用、召回面零命中如实记
叩问处置[句读]: 消解 即主题词未入召回登记轴、契约与引擎源已直读
叩问处置[级联]: 消解 即同上主题词未入登记轴、core.py 已直读
叩问处置[边账]: 消解 即召回面零命中如实记、边账语义从 CONTRACT 机器形态节原文核验
叩问处置[期票]: 消解 即召回面零命中如实记、三笔期票从两前批结果档原文定位
叩问处置[令牌捕获]: 消解 即工作名直述大白话不做登记、批内一致使用
叩问处置[use 边]: 消解 即工作名直述即 rs import 关系边、不做登记
叩问处置[运行期边账]: 消解 即工作名直述即 check 报告内嵌四计数器视图、不做登记
叩问处置[直接子]: 消解 即引擎参数直述即 join_rules 限直接子规则、不做登记
