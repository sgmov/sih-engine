# judouwire-solo 批任务包：locator code 载体切句读与崩溃语料真重接

> 令源：用户 2026-09-11 令「修复」承 tree-sitter 重接债链上判因（本会话 2026-09-11 判因记录：DEC-016 第三占位批即崩溃语料重接测试批从未开立，实现批 F-1 借名未接实，融回义务散文无承载致意图静默降级；实测真语料 58 件 rust 旧载体崩 21 件即 36%，四件原始语料至今全崩）
> 形：solo 批单线，主线亲写零子代理
> 日期：2026-09-11 ｜ 查册：judouwire unknown 走 --new-stem 认领开约，packs/core 在飞他批登记候批面
> version: v1

## 一、使命

把 DEC-016 既裁「不再依赖 tree-sitter，code 载体由自研解析承接」从能力证明位落进消费执行位：locator 的 code 载体从 tree-sitter 原生绑定切到句读（parser）引擎与 rust 语言包，崩溃语料四件走 locator 端到端真重接（补第三占位之实），金向量按纪律再冻结，零依赖判据复用到消费仓，温故 recall 轴活体回归。DEC-016 病灶（义务无承载）随批两小件：DEC-016 销账修订与 DEC-013 融回门机械承载条款。

## 二、关键设计

**件一 载体切换（locator/src/locator/carriers/code.py）**

- parse_code(path, src, lang) 签名与出参形零改：units 列表 {kind, name, line_start, line_end, text}，行位序排序（line_start, line_end, kind, name）对齐旧 start_byte 序语义
- 内部换句读：langpack.load_pack(rust 包) 加 engine.parse_text 加 entries.project_entries 取条目五字段回投 units；path source 接线 parser 仓（pyproject 撤 tree-sitter 与 tree-sitter-rust 两依赖，加 [tool.uv.sources] parser path 依赖）
- carriers_meta 的 code 元数据换 judou 引擎名与 parser 版本与 rust 包 revision
- rust 包位解析承现形 queries 同族约束：仓 sibling 相对定位（parser/packs/rust），缺位报错显形不静默

**件二 端到端真重接（locator tests 新增）**

- 崩溃语料四件（validate.rs 与 append.rs 与 query.rs 与 mod.rs）按内容取副本入 locator tests fixtures（承实现批语料副本先例）
- 端到端测试：四件逐件 parse_code 零崩加 build 管线级零 parse_errors——若树旁路依赖回流此测试即进程级炸红
- 先红留痕：真语料 58 件旧引擎逐文件隔离探测实录（崩 21 件清单）与本会话 139 信号量复现实录入批材料

**件三 金向量再冻结（locator vectors）**

- 旧 expected 先入批材料留痕；新 expected 以句读输出冻结；冻结后双跑逐字节一致回归
- 新旧对表漂移报告随批入材料：真语料 58 件句读零崩 776 条；双活 37 件对表旧 311 对新 301，逐字节全等 294，行位漂 6（impl 语义差为主），text 漂 1，缺失 10（const/static 复杂 ty 与恢复吞并为主），新独有 7；漂移根因两句——impl 名语义差（实现批已登记待深阶）与 rust 包 v1 声明层 ty 形覆盖缺口（归 pk027expr 后继），本批零文法改动如实登记
- stable_id 判词：294 全等条目标识零漂；漂移条目标识随 text/name 漂移如实换新，现役生产索引（memory 包）走 markdown 与 json 载体零涉 code 载体，漂移面收敛于 locator 自身金向量

**件四 零依赖判据复用消费仓**

- locator 环境 uv tree 零 tree-sitter 分布机械输出入档；locator 源树 tree_sitter 字符串零命中测试断言

**件五 温故轴活体回归**

- retriever recall --topic 与 --word 全轴真跑主树现形二进制（retriever 经 locator_bridge 拉起 locator），命中与退出码实录入档；locator_bridge 写死路径属 M2/rootanchor 域感知范围零触碰（wengumcp-parallel 在飞其面，协调红线）

**件六 DEC-016 病灶两小件（sih-engine/doc/decision）**

- 016-parser-initiation.md 修订一：三占位逐个销账（落差规格批 judousdd-t6d 已销、实现批 judouimpl-parallel 已销、崩溃语料重接测试批由本批销）；病灶判词一句入档（占位批蒸发即散文义务无机械承载）
- 013-mergeback-gate.md 修订：融回门增机械承载条款——凡决策文declared占位批或切流或远期融回义务，其承载批收约前必须落 pk 泊位或后继任务包号，未承载即不得闭项

## 三、工作清单

- [ ] T-0 崩溃实录与 AB 漂移材料入批材料（已完成于判因阶段，转存正典位）
- [ ] T-1 code.py 切句读加 pyproject 接线（先红后绿）
- [ ] T-2 崩溃语料四件端到端测试
- [ ] T-3 金向量再冻结加双跑一致回归加漂移报告入档
- [ ] T-4 零依赖机械证明两件
- [ ] T-5 recall 轴活体回归读数
- [ ] T-6 DEC-016 销账修订与 DEC-013 融回门条款，管线三步
- [ ] T-7 主树复跑 locator pytest 全绿加 parser pytest 零改动回归
- [ ] T-8 认证上链双仓 settle reconcile 双零链 verify valid 收约

## 四、可证伪条件

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 真重接 | 工程治理 | 崩溃语料四件 locator 端到端零崩零 parse_errors，测试在档 |
| **F-2** 切流成立 | 工程治理 | locator 源 tree_sitter 零命中加 uv tree 零分布两机械输出在档 |
| **F-3** 确定性 | 确定性 | vectors 冻结后双跑逐字节一致 |
| **F-4** 金向量纪律 | 工程治理 | 旧 expected 入材料留痕加新 expected 冻结加漂移报告在档 |
| **F-5** 温故轴回归 | 工程治理 | recall --topic 与 --word 真跑退出码零命中在档 |
| **F-6** 病灶销账 | 治理 | DEC-016 三占位下落逐个在档加 DEC-013 机械承载条款在档，两件管线零违例 |
| **F-7** 只读面 | 工程治理 | parser 仓与句读语言包与 Rust 组件内容哈希零变 |

## 五、管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门（scrutinator packs/ask3 加引擎 ask3repeater）→ 叩问 elicit check 加 digest（切流与载体两信号教学形处置加登记候批面）→ 正身 identity verify → lease open --package judouwire-solo --new-stem（查册 unknown 认领）→ 取锁（施工面 exclusive；trail 与 reports append 短持）→ meter 包裹引擎 scribe intent 上链 → 工地施工（worktrees 双仓 judouwire-solo）→ 管线三步（化格→核阅→检词）→ 认证逐笔 append → 双仓 settle --cert → 放锁 → close → reconcile → 当日链 verify → 回锚重跑 → 完工报告回显五行。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；禁管道掩退出码。

## 六、请求写入（锁路径全集）

- sih-tools/locator/（源码、pyproject、uv.lock、vectors、tests、fixtures）
- sih-engine/doc/decision/（016 与 013 两修订）
- sih-engine/sih/state/plan/judouwire-solo.md（本包）
- sih-engine/sih/event/plan/judouwire-solo-materials/ 与 judouwire-solo-results.md
- sih-engine/sih/event/trail/2026-09-11.ndjson（经引擎 scribe）
- sih-tools/scribe/reports/ 与 sih-tools/identity/reports/（批件）

## 七、禁区

parser 仓源与语言包零改动；句读 rust 包文法零触碰；retriever 与 scrutinator 与 mcpline 零触碰（wengumcp-parallel 与 mcpmanual-solo 在飞其面）；nomenclator packs 零写（mcpmanual-solo 持锁）；工作台 P0/P0.5 划出；路径写死清单其余十六处划出；主树零直写。

## 八、协调位

wengumcp-parallel（M5 温故 MCP 投影与新城索引）在飞：其引擎面 retriever 三文件与本批零相交，其 mcpline 面与本批零相交；本批 recall 回归跑主树现形二进制，两批归并后主树复跑互验。mcpmanual-solo 在飞持 packs/core 与 doc/design 锁：本批新词登记与决策文档面分属 decision 目录零冲突。
