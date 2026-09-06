# regulamath-solo：文规登记面数学三件建条入仓

> 治理任务包（立文类，单线形 solo，DEC-018；**委外执行**：单代理持本任务包与提示词连续执行全链）
> 承接：用户 2026-09-06 令「数学三件批，委外，出提示词和任务包」；上游输入件即工作区根域外草稿 regula-design-draft-2026-09-06.md §五§六（登记面关系模型三件候选，零命中申报在案）
> 日期：2026-09-06

## 一、问题陈述 {#problem}

- 文规设计稿第五节登记面（户口簿，工作名）的关系模型数学支撑经映射表与五子仓查证零命中（2026-09-06），三件候选无家：关系模型本体、代入、证明义务
- 三件是登记面设计的直接数学需求：五字段（归因路径、失败定位、废弃死因、验证时间戳组、验证用包类型）、两针修正（动态位事件化投影、进度终态正交拆分）、三面（规格面、证据面、判据面）须从设计散文升为可机械校验的载体条目
- 类代表充分性已由 gvecmath-solo 批入 ORD-007 扩条目（2026-09-06 提交在档），**本批不含**
- docmath 批二已落推导档两载体（规格充分性、证伪覆盖度，sih-math/docs/docmath-carriers-derivation-2026-09-04.md），本批三件与之**不重复立载**：批二承内容充分性面，本批承登记结构面

## 二、关键设计 {#design}

### 2.1 三件内容边界（与既有载体逐件对表，重叠即停批上报）

**REL 关系模型本体**（候选工作名）：类型化关系模型——表、键、引用约束、差分（ADDED／MODIFIED／REMOVED 作为关系运算的机械形）、投影。登记面三面即三关系模式，五字段即必带属性。两针修正落形为两条定理：针一，事件化投影定理——表态是事件流的确定性折叠，同流必同态（投影唯一）；针二，正交拆分定理——进度与终态是积结构，「进行中但已通过」类混合矛盾态不可表示。邻件分工：ORD-019 承版本偏序与外化三性质，REL 承结构约束与投影唯一，各管各的不合载。

**SUB 代入**（候选工作名）：场景对规格条款的机械实例化算子——代入把条款中的变量位绑定到场景的具值，输出判据位（检验程序、期望退出码、期望产物）。特化保真定理：一般条款的逻辑蕴涵在代入下保持（实例被一般条款蕴涵，代入不得凭空补义）。邻件分工：PROB-018 双重有损链已承意图→规约→实现信息不增的边界，SUB 承代入算子自身的保真形，SUB 是 PROB-018 链上一环的机械刻画不是其重述。

**PROOF 证明义务**（候选工作名）：义务集、检查集、清偿关系三件套的账本结构。门正确性定理：门通过当且仅当义务全清偿（完备性）；检查只清偿其被指派的义务（健全性）。先红后绿中先红是义务的见证位。邻件分工：docmath 批二证伪覆盖度承覆盖的度量，PROOF 承义务账本的结构，度量挂在结构上不重立。

### 2.2 家位与号实

三件全落 algebra 子仓，号实开约时按 algebra/INDEX.md 实读取下一空位（预期 ALG-013／014／015）。候选名 REL／SUB／PROOF 是工作名不入词条。不开新子仓族（新族属用户 2026-08-25 先例权，不在委外权内）；若成文中发现 ALG 家位不称职即停批上报。中文名用标准术语（关系模型、代入、证明义务），**零新造词**，需新词即停批走立名流程。

### 2.3 命题与机器门

三命题 gid：m-regulamath-rel-1、m-regulamath-sub-1、m-regulamath-proof-1。单锚即可验证性——每条条款只问能不能机械验，不问哲学该不该。facet 三步加得一三步（check、verify 重放 identical、sign），stable_clear 即落据；boundary 或 violate 即停批如实呈报，不硬闯不改判据不重采样凑收敛。

### 2.4 机械校验场景

每条目携场景件（fixture 加复算脚本，Python 标准库零依赖）落 sih-math/docs/regulamath-scenarios-2026-09-06/，跑绿读数入批材料，双跑逐字节一致（承 docmath 批二三场景先例）。REL 场景至少覆盖：键唯一、投影唯一（同事件流两次折叠同态）、混合矛盾态不可构造；SUB 场景至少覆盖：代入保真一正例一反例（凭空补义被拒）；PROOF 场景至少覆盖：义务全清即门过、义务缺清即门拒、越指派清偿被拒。

### 2.5 哲学回锚

REL 锚 PRO-08（应而不藏）加 convergence 层 P3.2 外化管理（原文加载核验，llm-friendly-build 不覆盖 convergence 须回原文）；SUB 锚道四（规约与实现必有间隙）加 PRO-05（经 PROB-018 对照）；PROOF 锚 PRO-07（鉴只列事实，检验由可重复程序承载）。引文一律原文程序切片，禁手打。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门（核阅 ask3 包＋引擎 ask3repeater，三锚引文程序切片）
- [ ] 叩问 elicit check，信号如实处置后 digest passed
- [ ] 正身 identity attest
- [ ] watch 对表：在盘遗留如实转述，含 sih-math 未跟踪件（probability/entries/PROB-018 与 PROB-019 条目文件、sih/event/plan/gvecmath-solo 残件、__pycache__），**不豁免不代清不触碰**
- [ ] 泊界心跳两线（路择 parking 包）

### Cluster 2：落位与底稿

- [ ] 租约开工：--package regulamath-solo，三仓工地（sih-engine base main、sih-tools base integral-stage-build、sih-math base 开约时实读），--allow 按请求写入节
- [ ] 任务包与提示词两件从主树位拷入工地 state/plan（批输入件先例，后续编辑零主树写）
- [ ] 三条目成文，条目形照 ALG-012 全节形（定义与定理与证明、公理条件、可证伪、哲学桥接、应用、与其他概念的关系、历史脉络、工程注意事项）
- [ ] algebra/INDEX.md 三笔（已建行加历史行，只增不改旧数）；顶层 mapping.md 三行（登记面的表键与投影唯一、场景对条款的代入保真、检查对义务的清账完备）
- [ ] 场景件三组落 docs/regulamath-scenarios-2026-09-06/

### Cluster 3：facet 与得一

- [ ] 三命题各 emit-contract（零 LLM 零网络）→ 回填即席作答（谱系披露双声明即同席采样与成本加重，承 gvec 先例）→ score 携正身件
- [ ] attractor check → verify（重放 identical）→ sign，stable_clear 终签三笔入当日链

### Cluster 4：管线与结算

- [ ] 化格：条目与 INDEX 与 mapping 与任务包与结果档过 general-v1；场景件 JSON 过 json-canonical-v1
- [ ] 核阅 des-001：sih-math 与 state/plan 与 event/plan 均域外，exit-2 如实记档不属违规
- [ ] 检词 core：全部成文件零违例（历史死档词一律禁用，建条、入仓、收录为合规形）
- [ ] 温故 recall 底稿（--event --since 2026-09-06 --until 2026-09-06）
- [ ] 结果档 regulamath-solo-results.md 落 event/plan
- [ ] 三仓 settle（cert 取 ask3 记录认证哈希前八位）→ 放锁收约（close 撞主树同名未跟踪件按备份让位归并对表法，真分叉停批）→ 链 verify → reconcile 三仓 → 心跳复算 → CALL-LOG 落笔

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 三条目建条入仓 | 工程 | algebra/entries/ 三新件落位，INDEX 已建行三笔，mapping 三行在册 |
| **F-2** 终签在链 | 治理 | 三命题 attractor sign stable_clear 终签三笔入当日链，verify valid |
| **F-3** 场景全绿 | 工程 | 三组场景件复算全绿且双跑逐字节一致，读数入批材料 |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-5** 委外不越权 | 编组治理 | 零人节点代行、零绕行、零 plain commit、零预设重裁结论；在泊件零触碰；PROB-018 与 PROB-019 未跟踪件与 gvecmath 残件零触碰 |
| **F-6** 零代码改动 | 工程 | 引擎与工具源码零触碰 |

## 五、必读文件 {#read}

- 上游设计稿：工作区根 `regula-design-draft-2026-09-06.md` §五§六（三件内容边界、五字段、两针修正、三面）
- 条目形先例：`sih-math/algebra/entries/ALG-012-formal-grammar-and-parse-determinism.md`
- 数学批结算先例：`sih-engine/sih/event/plan/newcarr-solo-results.md`（三仓工地、三仓 settle、让位归并对表法）
- 批二推导档（不重复立载对表）：`sih-math/docs/docmath-carriers-derivation-2026-09-04.md`
- 已建条扩条目（本批不含）：`sih-math/order/entries/` ORD-007 扩条目节
- 邻件分工对表：ORD-019、PROB-018、ORD-010 三条目原文
- 命令面：`sih-tools/BATCH-FACE.md`（facet 三步、attractor、lease、坑位勘误）
- 金向量判定面背景：`sih-engine/doc/spec/SPEC-021-golden-vector-dual-method.md`

## 六、约束 {#constraints}

1. 闸不过即停批：boundary 或 violate 即如实呈报，不硬闯不改判据不重采样凑收敛
2. 中文零新造词，需新词停批走立名流程
3. 数学不装饰：定义定理证明自含可核，不引未证外部定理贴牌（数学装饰风险，docmath 程序 v1 §九）
4. 哲学引文原文程序切片禁手打；convergence 层命题回原文核验
5. 主树零直写（任务包与提示词源件即批输入件先例，落位后批内零主树写）；守卫在位禁 plain git commit
6. 退出码直读，禁管道掩码
7. 在泊件与在盘遗留零触碰不并批
8. 不动 AGENTS.md、不动引擎与工具源码、不开新子仓族、不动三工具命令面与退出码语义
9. 与 docmath 批二、measure-poly、mathpipe 三程序不重复立载，重叠无法以边界声明化解即停批上报

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 三仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 三仓相比批前零新增
- [ ] 结果档 regulamath-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- facet 回填由委外代理即席作答即同席位采样，谱系披露必须在 topic 文内双声明
- attractor 相对路径按调用 cwd 解析，--material 传相对路径且 cwd 置工地根（openhyg 勘误）
- 仓内条目计数历史存在口径漂移（INDEX 历史行与顶层概览不一致），本批只增不改旧数，历史行记实测值
- close 归并撞主树未跟踪件按备份让位归并对表法四步处置，真分叉停批上报
- sih-math 分支基线以开约时实读为准

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（intent 与 append 写位）、`sih-engine/target/debug/attractor`（check、verify、sign）、`sih-tools/facet/measure.py`（三步）、`sih-tools/lease`
- 跨仓引用：`sih-tools/proposition/DES/`、`sih-tools/facet/`、`sih-math/algebra/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/regulamath-solo.md` 与 `regulamath-solo-prompt.md`（批输入件经工地落位）
- `sih-math/algebra/entries/` 三新条目
- `sih-math/algebra/INDEX.md`
- `sih-math/llm-friendly-build/mapping.md`
- `sih-math/docs/regulamath-scenarios-2026-09-06/`
- `sih-engine/sih/event/trail/2026-09-06.ndjson`
- `sih-engine/sih/event/plan/regulamath-solo-results.md` 与 `regulamath-solo-materials/`
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`、`sih-tools/scribe/CALL-LOG.md`、`sih-tools/facet/`、`sih-tools/proposition/DES/`
- worktrees 三仓 regulamath-solo 工地
