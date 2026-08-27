# parseinit-t6d：自研检索解析工具立项批

> T6D-XX task-packages 治理任务
> 承接：sess-zcode-260827-parseinit 三问意图即 2026-08-27 链意图事件 7c3fa569、用户三裁定即开工令与不动 Python 版本与崩溃语料新工具成后重接测试与不再依赖 tree-sitter
> 范式：T6-D 范式—— **偏离：立项契约类单线，主线串行不派双子代理**
> 日期：2026-08-27

## 一、问题陈述 {#problem}

locator 的 code 载体解析依赖 tree-sitter 原生绑定，在 cpython 3.14.6 下确定性段错误即四文件实锤，整文件组合效应不可绕。用户裁定不动 Python 版本即止血撤销，以自研解析工具替代即零第三方解析依赖，崩溃语料转验收材料待新工具成后重接测试。本批立项即契约先行，不写引擎代码。

## 二、关键设计 {#design}

### 2.1 算法钉死

PEG 族序选择即有序选择从构造上消除歧义，同树恒定双跑逐字节一致，配 2025 自动容错恢复路线即 arXiv 2507.03629 与 ANTLR 错误节点包裹即错误处包节点永远产树。Lark 教训登记即歧义接受算法双跑多树，不走。

### 2.2 空腹形态

引擎零语言知识，语言包三件即词法表与产生式与条目映射纯数据无代码，新语言零引擎改动。ctags optlib 空腹先例。砍编辑器增量，保留索引级容错。文法包按深度分级即 v1 声明层全保真、表达式内部配平节点。产出对齐寻址条目十字段。

### 2.3 路线与命名

外切自研于 sih-tools 围堰，远期融回按 DEC-013 开发非移植即契约过去实现 Rust 重落金向量对拍。命名走工作名先行即物理名 parser、治理名不签，候选句读随汇报呈人节点，pk-025 入泊承载。

## 三、工作清单 {#work}

- [ ] DEC-016 落 doc/decision
- [ ] sih-tools/parser/CONTRACT.md 契约先行即操作面与语言包格式与容错定级与金向量纪律与验收判据与融回门
- [ ] pk-025 命名入泊与泊界投影
- [ ] GOV-002 概览行版本残留修正即 v1.3 改 v1.4 对齐版本节
- [ ] 任务包与结果档落位
- [ ] 管线三件零违例即核阅镜像真过规
- [ ] 双仓段结算收约、免参对表退出码零

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 立项成立 | 链上治理 | DEC-016 与 parser/CONTRACT.md 落档，算法选型与空腹形态与零依赖目标与路线四节齐 |
| **F-2** 裁定承接 | 链上治理 | 不动 Python 版本、崩溃语料转验收材料、不再依赖 tree-sitter 三条用户裁定显式落文 |
| **F-3** 命名不冒签 | 跨族治理 | 治理名零落档，句读仅以候选身份随汇报呈，pk-025 入泊在链 |
| **F-4** 不越界 | 跨族治理 | 零引擎代码、零 locator 运行变更、不入信息洪流工具群冻结清单 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/locator/CONTRACT.md 即被替代方操作面与条目十字段
- 必读 2：DEC-013 融回开发非移植与金向量纪律、DEC-015 外切贡献度即融回门取数
- 必读 3：GOV-002 冻结清单第 1 条即信息洪流工具群数量为二的范畴排除依据

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜即新词过检词
3. 人裁令源在版本行与决策正文显式引用

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 结果档落 sih/event/plan/parseinit-t6d-results.md，管线三件认证入链，双仓段结算收约

## 八、风险点 {#risks}

立项契约易犯期票膨胀即把实现细节写成承诺，以验收判据可证伪形态收紧；算法选型易犯意识形态化，以确定性证据链承载。

## 九、范式偏离声明 {#deviation}

立项契约类单线，保留 T6-D 命名与 F 锚定与管线认证。

## 十、关联文件 {#related}

- 任务包源：用户开工令与会话对谈
- 链件：sih/event/trail/2026-08-27.ndjson 即意图 7c3fa569
- 后续：SDD 批即落差规格与红绿实现均待令

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/decision/016-parser-initiation.md
- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md
- sih-engine/sih/state/plan/parseinit-t6d.md
- sih-engine/sih/event/plan/parseinit-t6d-results.md
- sih-engine/sih/event/trail/2026-08-27.ndjson
- sih-tools/parser/CONTRACT.md
- sih-tools/PARKING-v1.md
- sih-tools/parking/records/pk-025-enter.json
- sih-tools/scribe/reports/
- sih-tools/lease/
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
