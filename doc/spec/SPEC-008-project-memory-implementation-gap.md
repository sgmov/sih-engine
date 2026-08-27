# SPEC-008 温故实落差规格

本规格承接 SPEC-007 冻结契约与 DEC-014 立项决策与 DEC-013 第一步纪律，钉死温故实装批的引擎侧待建面。本规格是 SDD 产物即先于实现，实现按 TDD 执行即逐判据 F-1 至 F-8 先红后绿，红转绿留痕于本批结果档。工具侧现状权威即 sih-tools/locator/CONTRACT.md 与 sih-tools/locator/SPEC.md。

## 概览 {#overview}

- 现面即引擎 query 位可复用与寻址四子命令在役，落差为五档映射加三轴编排加切面产出加宿主命令面四件::[现面盘点](#baseline)
- 五档路径集即结论与经验与悬置与意图四档文件载体加事实档事件载体，分类规则单源在组件::[五档映射落差](#archives)
- 三轴语义即主题轴走寻址词查询条目级、事件轴走库内 query 位、时间轴按东八区日界含即含::[轴语义落差](#axes)
- ref 补钉即无行位 json 载体记路径加条目稳定标识，机械回验走寻址按标识直取::[ref 形态补钉](#ref-pin)
- 排序机械即档序固定档内文件载体先事件载体后，输出无评分无建议::[确定性与排序](#determinism)
- 组件边界即库面落 src/retriever 与命令面落 src/bin/retriever.rs，事件轴底座库内直调不走子进程::[组件边界](#boundary)
- 测试计划十组即 F-1 至 F-8 加边界与拦面单测，红态即入口建桩未实现::[测试计划](#tdd)
- 已知缺陷登记即寻址 core 包全仓构建段错误，本批窄域包绕开修复归另批::[缺陷登记](#defects)

## 现面盘点 {#baseline}

引擎已有即 src/event_stream 的 query 位，过滤五维即事件类型与操作者与时间起止与文档标识，本组件事件轴底座即库内直调此位，不另建查询逻辑。scribegate 命令行面的 query 子命令为单文件查，跨日链加载是落差由本组件补即按文件名字典序逐文件加载拼接。

寻址已有即四子命令 build 与 query 与 stale 与 vectors，条目十字段即 id 与路径与载体与类别与名与序与行起止与内容哈希与文本，词查询即按词边界匹配名与文本返回条目级命中。本组件主题轴底座即只读子进程调用 build 与 query 两步。

落差四件即五档路径集分类、三轴语义编排、切面记录七字段产出、宿主命令行面。工具侧新增仅窄域包数据件即 sih-tools/locator/packs/memory/pack.json，五档路径集预筛，零代码改动。

## 五档映射落差 {#archives}

分类规则单源在组件即按路径机械判定，窄域包仅预筛。路径集五件：

- 事实档即链上事件全体，载体记号 event，非文件。事件档属按事件类型机械判定即 parking_entered 与 parking_exited 归悬置档、intent_refined 归意图档、其余归事实档。
- 结论档即 sih-engine/sih/event/plan/ 下 *-results.md。
- 经验档即 ai-ex/ 下全部 md。
- 悬置档即 sih-engine/doc/governance/PARKING-v1.md 与 sih-tools/PARKING-v1.md 与 sih-tools/parking/records/ 下 json，加停泊事件。
- 意图档即 sih-tools/scribe/reports/ 下文件名含 ask3 与 record 的 json，加 intent_refined 事件。

一致性对表即测试断言窄域包内每个文件路径可归入恰一档且非事实档，防分类规则与包漂移。

## 轴语义落差 {#axes}

主题轴即每个 --topic 词一次寻址词查询，取条目级命中即 entries 不取 occurrences，多词命中各出一行 matched 记各词原样。中文词受词边界正则约束承寻址确定性细则不另造分词，登记为承自底座的已知边界。

事件轴即 --event 值对链上事件三路精确匹配即事件类型或 details.entry_id 或 details.session_id，命中按事件档属归档。时间轴即 since 与 until 对事件时间戳过滤，日界按东八区即时间戳折算正八区后取日期部分，边界日含即含，文件载体不参与时间轴。多轴命中各出一行即 axis 字段区分。

轴全缺拦即 topic 与 event 与 since 和 until 全缺退出码一。参数互斥违例 v1 零例即当前七参数两两可并，此拦面为占位判据落零例，日后参数增互斥即生效。

## ref 形态补钉 {#ref-pin}

SPEC-007 钉文件载体 ref 为路径加行区间。md 载体条目有行位即记路径:起行-止行。json 载体承寻址 SPEC 即无行位记 null，行区间条款在此现实下不可满足，本规格补钉即 json 载体 ref 记路径@条目稳定标识，机械回验即重建索引后寻址按标识直取复原该条目。此补钉不改七字段名与必填性即仍为路径加可机械回验定位件，是否构成破冻待人节点复核，判破冻即走 SPEC-007 修订记录不静默。

excerpt 即条目 text 逐字，事件载体即事件紧凑序列化逐字，上限二百五十六字符超长截断附三句点。matched 即主题轴记主题词原样、事件轴记事件记号原样、时间轴记事件时间戳原文。

## 确定性与排序 {#determinism}

排序档序固定即 fact 加 conclusion 加 experience 加 parked 加 intent。档内文件载体先行即路径字典序加行序加条目标识末位决胜，事件载体随后即链序，链序即 trail 目录文件名字典序加文件内行序。输出 ndjson 逐行七字段键序固定，不含随机量不读系统钟，同参同语料双跑逐字节一致。

索引策略即每次调用即时建索引于系统临时目录不落工作区，确定性优先于性能，代价即每次调用一次建索引开销，窄域包实测百九十四文件万二千条目秒级。缓存复用属后续优化不在本批。

## 组件边界 {#boundary}

库面即 src/retriever 模块，入口函数接收根路径与参数组返回切面行序列，根定位由命令面承担即从当前目录上溯找 sih-engine 与 sih-tools 并存的目录层，找不到退出码二。事件轴底座即库内直调 event_stream 的 query 位。主题轴底座即子进程只读调用寻址即 uv run --project 寻址目录，缺席或失败映射退出码二报缺席件名 locator。

命令面即 src/bin/retriever.rs 新二元，宿主名经 DEC-017 立名与修订一两跳由 memgate 经 retrievergate 改裸名 retriever 承撤gate令，本句旧名记述保留为修订史。子命令 recall 参数七件随 SPEC-007 冻结，输出走标准输出或 --out 文件。库面无 CLI 耦合即记忆模块不依赖 std::env。

## 测试计划 {#tdd}

红态即桩模块入口 todo 宏承空即测试编译过而断言败，绿态即实装完成全绿。十组如下。

F-1 五档覆盖
: 真实工作区组合三轴调用，断言五档各至少一行且 ref 可回验。

F-2 出处机械回验
: 每行 ref 按载体解析回源即 md 行区间重开文件比对、json 标识重建索引直取、事件哈希在链。

F-3 主题轴跨档
: 主题词 pk-024 断言至少三档即结论与悬置与意图。

F-4 事件轴一致
: 按事件类型过滤断言与 query 位直查结果集一致即事实档与意图档两路。

F-5 时间轴边界
: 真实链单日过滤全落当日，合成事件证日界含即含即首末瞬皆收。

F-6 只报不判
: 全部输出行键集恰七字段无 score 无 suggestion 无 ranking。

F-7 双跑一致
: 同参两跑输出逐字节相等。

F-8 退化不崩
: 假根缺寻址与缺 trail 各报缺席件名退出码二，五档载体原样。

F-9 边界单测
: 档属归位、排序序、截断、at 回显、多轴多行、拦面即轴全缺与档名非枚举。

F-10 包档一致
: 窄域包内文件路径全可归档恰一档。

## 缺陷登记 {#defects}

寻址 core 包全仓构建段错误即退出码 139，复现即 cd sih-tools/locator 后 uv run locator build --pack packs/core/pack.json --root ../.. --out 任意路径，root 收窄至 sih-engine 仍复现，root 收窄至 doc 烟测不复发，疑代码载体或大体量文件触发，定位与修复归另批不扩本批。本批窄域包不含代码载体即绕开。

## 验收判据 {#acceptance-criteria}

- A1 SPEC-008 在测试落盘前在档即落差先钉后实现
- A2 F-1 至 F-8 全绿且红态输出在结果档留痕
- A3 接口不破冻即七参数与七字段与退出码语义与 SPEC-007 逐条对表无违
- A4 底座只读即五档载体与 locator 与 scribegate 零修改，工具侧新增限窄域包
- A5 库面无 CLI 耦合即记忆模块可被 MCP 层直接调用

## 版本与固定 {#version}

v1.3 于 2026-08-28 即 DEC-017 修订一撤gate连带即宿主名改裸英文对。v1.2 于 2026-08-28 即 DEC-017 温故立名连带即组件名与模块路径与宿主名改写，窄域包路径 sih-tools/locator/packs/memory/pack.json 属寻址侧命名不动，语义零改。v1 于 2026-08-27 随实装批 mem-impl-t6d 落盘。2026-08-27 修订一即出处分隔符统一为 at 记号，md 载体 ref 由路径:行区间改路径@行区间，承用户直觉令与 facet 双向测量即 m-refsep 刀锋与 m-refsep-inv 接近临界两形皆无规约违反、歧义归人偏好由用户定 @，事件载体 ref 形态不动，随 pk024exit 批落码。落差补钉修订须走修订记录留痕，破冻显式可见。
