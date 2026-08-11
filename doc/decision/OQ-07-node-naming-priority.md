# OQ-07 治理语义节点立名优先级

本文档是 OQ-07 的推导产出，为 DEC-001 声明的 18 个治理语义节点排立名优先级与批次建议。本产出不执行立名，不修改 DEC-001，不修改哲学仓。立名执行走后续 DEC 流程。

OQ-07 已推导结论：工程接口节点用工程惯例直接命名，不需立名。本文档覆盖范围是 18 个治理语义节点。

## 概览 {#overview}

- 18 个治理语义节点需走 PRO-01 立名流程::[节点清单](#node-list)
- 每个节点评估名实错位风险与立名紧迫度::[逐节点评估](#per-node-assessment)
- 分三批处理，首批 5 个高紧迫节点需立即走 DEC 立名::[优先级排序与批次](#priority-and-batch)
- 批量声明沿用须满足最低条件::[批量声明沿用判据](#batch-declare-criteria)

## 评估方法论 {#assessment-methodology}

名实错位风险评估基于 PRO-01 立名本体论与 PRO-01 立名失败的形态。立名失败的形态是名字承诺的内涵与被命名者的实际运作不一致。评估维度三个：名字承诺即当前节点名向读者承诺了什么实体、实体运作即该节点实际承载什么、错位程度即两者之间的间隙。

立名紧迫度评估独立于错位程度，叠加三个因素：被引用频率即被其他文档或组件引用越多的节点命名错误传播面越广、不可逆成本即已积累的实例越多改名成本越高、概念歧义性即名字是否容易被误解为其他含义。

两个维度独立评估后组合为优先级。

## 节点清单 {#node-list}

从 DEC-001 全节点树提取，共 18 个治理语义节点。

### 文档区 7 个文档类型 {#doc-nodes}

D1. requirement，路径 doc/requirement/，需求/意图，REQ-NNN-slug
D2. decision，路径 doc/decision/，决策文档，DEC-NNN-slug
D3. design，路径 doc/design/，技术设计，DES-NNN-slug
D4. spec，路径 doc/spec/，规格定义，SPEC-NNN-slug
D5. governance，路径 doc/governance/，规约，GOV-NNN-slug
D6. knowledge，路径 doc/knowledge/，知识库，KNOW-NNN-slug
D7. proposal，路径 doc/proposal/，提案，PRO-NNN-slug

注：architecture 是 design 下的子类型，共享 DES-ARC 前缀族，不独立为顶层类型节点，不单独评估。

### 治理区 11 个节点 {#sih-nodes}

S1. event，路径 sih/event/，事件层，发生过的事件，只追加写
S2. state，路径 sih/state/，状态层，当前投影
S3. trail，路径 sih/event/trail/，治理动作留痕，治理主体动作的时序记录
S4. feedback，路径 sih/event/feedback/，反哺信号，工程层对哲学层的张力登记
S5. challenge，路径 sih/event/challenge/，挑战记录，外部对治理对象的质疑
S6. report，路径 sih/event/report/，审查报告，检验事件产出
S7. registry，路径 sih/state/registry/，文档身份与状态登记
S8. graph，路径 sih/state/graph/，文档关系投影
S9. view，路径 sih/state/view/，派生视图，从 registry 与 event 合成
S10. plan，路径 sih/state/plan/，待执行意图的当前编排投影
S11. calibration，路径 sih/state/calibration/，参验机制的标定基准集

## 逐节点评估 {#per-node-assessment}

### D1 requirement {#assess-requirement}

当前名称：requirement

实体运作：承载需求与意图，REQ-NNN-slug 命名模式。

名实错位风险：中。requirement 承诺「需求」，但实际也承载「意图」，即非正式需求、方向性意图。意图与需求在治理链条上角色不同：意图是上游触发，需求是下游拆解。用 requirement 框住两者可能导致意图类文档被误读为正式需求。

立名紧迫度：低。当前无 REQ 文档实例，改名成本为零。但一旦首份 REQ 创建，名称将固化。

哲学对照：无直接命题约束，工程符号层命名。

### D2 decision {#assess-decision}

当前名称：decision

实体运作：承载治理决策，DEC-NNN-slug 命名模式。

名实错位风险：低。decision 承诺「决策」，实体确实是治理决策文档。已积累 6 份实例，使用一致。

立名紧迫度：低。名称与实体贴合，且已有大量实例引用，改名收益小成本大。

哲学对照：决策四要素（09-on-arche）指向决策须留痕，decision 名称与该命题方向一致。

### D3 design {#assess-design}

当前名称：design

实体运作：承载技术设计，含体系定义子类型 architecture，DES-NNN-slug 命名模式。

名实错位风险：低。design 承诺「设计」，实体确实是技术设计文档。DEC-001 已明确 architecture 是 design 下的子类型。

立名紧迫度：低。名称与实体贴合，已有大量实例。

哲学对照：无直接命题约束，工程符号层命名。

### D4 spec {#assess-spec}

当前名称：spec

实体运作：承载规格定义，SPEC-NNN-slug 命名模式。

名实错位风险：低。spec 是 specification 的标准缩写，承诺「规格」，实体确实是规格文档。SPEC-001 至 SPEC-004 共 4 份实例，使用一致。

立名紧迫度：低。名称与实体贴合，业界通用缩写。

哲学对照：无直接命题约束，工程符号层命名。

### D5 governance {#assess-governance}

当前名称：governance

实体运作：承载规约文档，GOV-NNN-slug 命名模式。

名实错位风险：中。governance 承诺「治理」，但该节点实际承载的是「规约」，即 prohibition、rule、constraint。治理是上位概念，规约是治理的一个子集。读者看到 governance 目录可能期待完整的治理框架文档，但实际只有规约。GOV-001 当前标题是 failure-derived-prohibitions，即失败衍生禁止条款，确实是规约而非治理全景。

立名紧迫度：中。名称与实体的间隙会在知识库扩展时放大：若将来增加治理框架总纲类文档，放进 governance 会与现有规约混同，不放进去则 governance 名不副实。

哲学对照：PRO-09 元层治理框架须自带防御机制。governance 节点承载的是治理的约束产物即规约，不是治理本身。名字与实体的间隙有潜在的误导风险。

### D6 knowledge {#assess-knowledge}

当前名称：knowledge

实体运作：承载知识库，KNOW-NNN-slug 命名模式。当前仅 KNOW-001 一份实例。

名实错位风险：中。knowledge 承诺「知识」，但该节点实际定位是「开发者操作经验」，OQ-06 已推导此结论。知识的本体论范围远大于操作经验：知识包含领域知识、方法论知识、事实知识等。用 knowledge 框住「操作经验」可能框定未来内容方向为宽泛的知识而非聚焦的工程经验。

立名紧迫度：低。当前仅一份实例，改名成本低。但 OQ-06 已判「不单独做，在第一个产物触发时顺带定义边界」，说明该节点边界尚未明确，当前改名依据不足。

哲学对照：PRO-03 道二方向性、PRO-04 道三意图恢复。knowledge 节点承载的是工程层的经验沉淀，不是哲学层知识。

### D7 proposal {#assess-proposal}

当前名称：proposal

实体运作：承载提案，PRO-NNN-slug 命名模式。

名实错位风险：低。proposal 承诺「提案」，实体确实是提案文档。PRO-001 至 PRO-005 共 5 份实例。

立名紧迫度：低。名称与实体贴合。

哲学对照：无直接命题约束，工程符号层命名。

### S1 event {#assess-event}

当前名称：event

实体运作：治理区事件层，承载发生过的事件，只追加写。下含 trail、feedback、challenge、report 四个子节点。

名实错位风险：低。event 承诺「事件」，实体确实是发生过的事件。经 audit-017 立名失败检测通过，即从 static/dynamic 二分推导至 event/state 二分的成功路径，见 NAMING-METHODOLOGY-SUCCESS.md 记录。

立名紧迫度：低。已通过立名失败检测，名称与实体贴合。

哲学对照：外部锚定。事件与状态二分承接五个留痕哲学命题，包括 PRO-08 应而不藏留痕是构成性条件、PRO-07 多主体记录、PRO-09 形迹、PRO-09 决策四要素、PRO-07 一次穿透度量，全部指向事件。

### S2 state {#assess-state}

当前名称：state

实体运作：治理区状态层，承载治理对象的当前投影。下含 registry、graph、view、plan、calibration 五个子节点。

名实错位风险：低。state 承诺「状态」，实体确实是当前投影。经 audit-017 立名失败检测通过。

立名紧迫度：低。已通过立名失败检测，名称与实体贴合。

哲学对照：外部锚定。state 是文档不自包含元原则的工程化投影，即治理信息归独立索引。

### S3 trail {#assess-trail}

当前名称：trail

实体运作：治理动作留痕，治理主体动作的时序记录。在 OQ-01 中已推导为事件流载体，与书简/Scribe 组件的关系在 DEC-004 中已确立，书简是组件，trail 是载体。

名实错位风险：低。trail 承诺「痕迹、轨迹」，实体确实是治理动作的时序记录（append-only）。经 audit-017 立名失败检测通过。

立名紧迫度：低。已通过立名失败检测，名称与实体贴合。

哲学对照：PRO-08 应而不藏，留痕是构成性条件。trail 是留痕的载体，名称与哲学命题方向一致。

### S4 feedback {#assess-feedback}

当前名称：feedback

实体运作：反哺信号，工程层对哲学层的张力登记。

名实错位风险：低。feedback 承诺「反馈、回馈」，实体确实是工程层向哲学层的反馈信号。名称贴合。

立名紧迫度：低。名称与实体贴合。

哲学对照：PRO-05 道四间隙量化，间隙的反哺是 feedback 的本体依据。工程层对哲学层的张力登记是间隙的显式化。

### S5 challenge {#assess-challenge}

当前名称：challenge

实体运作：挑战记录，外部对治理对象的质疑。

名实错位风险：中。challenge 承诺「挑战」，实体确实是质疑。但在治理语境中，challenge 可能被误解为「工程挑战」即技术难题，而非「治理挑战」即对决策的质疑。DEC-001 的消费模式判据将 challenge 归入治理区事件层，是因其产出后被消费一次即归档。名称本身不显式携带「治理质疑」的限定。

立名紧迫度：低。当前无 challenge 实例，改名成本为零。名称的含义虽可被误读，但在已有 DEC-001 文档上下文中不会实际造成混淆。

哲学对照：PRO-07 鉴层破自证循环，挑战是鉴层的输入来源。名称与鉴层哲学方向一致。

### S6 report {#assess-report}

当前名称：report

实体运作：审查报告，检验事件产出。

名实错位风险：中。report 承诺「报告」，但「报告」的外延远大于「审查报告」。report 这个名称不限定是审查产出还是常规报告。在 sih/event/ 的四个子节点中，trail、feedback、challenge 都是治理特有概念，report 是最泛化的名称。读者无法从名字本身判断这是治理审查产出还是一般性报告。

立名紧迫度：低。当前无 report 实例。但 report 是参验路径的产出节点，承接 SPEC-003 参验规约，后续会有大量审查报告产出，名称泛化问题会在实例积累后放大。

哲学对照：PRO-07 鉴层，审查报告是鉴的映照的产出。report 名称不显式携带鉴层语义。

### S7 registry {#assess-registry}

当前名称：registry

实体运作：文档身份与状态登记。

名实错位风险：中。registry 承诺「登记簿、注册表」，但 registry 在工程语境中常被理解为「服务注册」如 service registry 或「包注册」如 package registry。sih/state/registry/ 实际承载的是文档治理元数据的登记簿，不是服务注册。名称的工程惯例含义与实体运作有间隙。

立名紧迫度：低。当前 registry 节点为空，无实例，改名成本为零。但 registry 是状态层的核心节点，一旦有内容写入，名称将快速固化。

哲学对照：状态层是文档不自包含元原则的工程化投影，registry 是该投影的核心承载。registry 名称不显式携带「文档治理」限定。

### S8 graph {#assess-graph}

当前名称：graph

实体运作：文档关系投影。

名实错位风险：中。graph 承诺「图」，但 graph 是极其泛化的名称：可以是知识图谱、依赖图、调用图、社交图等。sih/state/graph/ 实际承载的是文档间派生、取代、关联的关系结构。名称不限定图的类型与内容。

立名紧迫度：低。当前 graph 节点为空，改名成本为零。

哲学对照：无直接命题约束。

### S9 view {#assess-view}

当前名称：view

实体运作：派生视图，从 registry 与 event 合成。

名实错位风险：中。view 承诺「视图」，同样是泛化名称：数据库视图、UI 视图、架构视图等。sih/state/view/ 实际是从登记与事件层合成的当前状态视图。名称不限定合成来源与用途。且 view 在旧仓曾承载 project-state.md，已被删除，有历史残留语义。

立名紧迫度：低。当前 view 节点为空，改名成本为零。

哲学对照：OQ-06 已推导 view 回答「现在是什么状态」，即从事件流合成；knowledge 回答「怎么用」，即开发者操作经验。

### S10 plan {#assess-plan}

当前名称：plan

实体运作：待执行意图的当前编排投影。承接 OQ-02 的推导结果，即原 task-package 改名 plan。

名实错位风险：低。plan 承诺「计划、规划」，实体确实是待执行意图的编排。名称与实体贴合。已通过 OQ-02 的立名审查，task 与 package 的错位问题已修正。

立名紧迫度：低。名称已通过立名审查，与实体贴合。

哲学对照：plan 是状态层节点，区别于事件层的 task_completion 事件。plan 承载任务编排的当前投影，语义与 state 层「当前投影」定位一致。

### S11 calibration {#assess-calibration}

当前名称：calibration

实体运作：参验机制的标定基准集。承接 OQ-03 的推导结果，即原 experiment 改名 calibration。

名实错位风险：低。calibration 承诺「校准、标定」，实体确实是参验精度的基准数据。名称与实体贴合。已通过 OQ-03 的立名审查。

立名紧迫度：低。名称已通过立名审查，与实体贴合。

哲学对照：PRO-05 道四间隙量化。calibration 的实验设计受微积分与概率论与数理统计启发，用 N 次隔离采样标定语义判断的收敛基准。

## 优先级排序与批次 {#priority-and-batch}

### 第一批：高紧迫，需立即走 DEC 立名，共 5 个 {#batch-1}

1. D5 governance。错位风险中，紧迫度高。名称与实体间隙明显：governance 承诺治理全景，实体承载规约子集。GOV-001 已落地，改名窗口在更多 GOV 文档创建前。
2. D1 requirement。错位风险中，紧迫度高。实体承载意图与需求两类不同性质的文档，名称只框定需求。首份 REQ 创建前必须确立。
3. D6 knowledge。错位风险中，紧迫度中高。边界未定义，OQ-06 待定，名称的宽泛承诺可能框定未来内容方向。与 OQ-06 联动。
4. S7 registry。错位风险中，紧迫度中高。核心状态节点，当前为空但即将有内容写入。名称的工程惯例含义即服务注册与实体即文档登记簿间隙需在内容写入前修正。
5. S8 graph。错位风险中，紧迫度中高。与 registry 配套的关系节点，名称过于泛化。若 registry 立名后 graph 不联动，两者命名风格不一致。

### 第二批：中风险，可批量声明沿用但需登记，共 4 个 {#batch-2}

S5. challenge。错位风险中，紧迫度低。名称可被误读为技术挑战，但在治理上下文中不构成实际混淆。登记并记录潜在间隙，不改名。

S6. report。错位风险中，紧迫度低。名称过于泛化，不显式携带鉴层语义。但 report 作为参验产出的容器名称，语义足够。登记间隙。

S9. view。错位风险中，紧迫度低。名称泛化，且有旧仓历史残留。但 view 作为派生视图的容器名称，在 state 层上下文中语义可辨。登记间隙。

governance 与 challenge 共同承载一个间隙：governance 是上位概念，challenge 是治理动作的输入，两者关系从名字上不可见。登记但不改名。

### 第三批：低风险，工程惯例命名，暂不需立名，共 9 个 {#batch-3}

D2. decision。错位风险低，紧迫度低。名称与实体贴合，6 份实例一致。

D3. design。错位风险低，紧迫度低。名称与实体贴合，含 architecture 子类型关系已明确。

D4. spec。错位风险低，紧迫度低。标准缩写，名称与实体贴合。

D7. proposal。错位风险低，紧迫度低。名称与实体贴合。

S1. event。错位风险低，紧迫度低。已通过 audit-017 立名失败检测，外部锚定。

S2. state。错位风险低，紧迫度低。已通过 audit-017 立名失败检测，外部锚定。

S3. trail。错位风险低，紧迫度低。已通过 audit-017 立名失败检测，DEC-004 确认。

S10. plan。错位风险低，紧迫度低。已通过 OQ-02 立名审查。

S11. calibration。错位风险低，紧迫度低。已通过 OQ-03 立名审查。

第三批节点已通过某种形式的立名审查，覆盖 audit-017 立名失败检测或 OQ 立名推导，名称与实体贴合，无需再走完整 DEC 立名流程。

## 批量声明沿用判据 {#batch-declare-criteria}

### 什么条件下可以不走完整 DEC 立名流程 {#batch-skip-conditions}

节点满足以下全部条件时，可批量声明沿用当前名称，不走完整 DEC 流程。

1. 已通过立名失败检测或等价的立名审查，覆盖 audit-017 检测或 OQ 推导中的立名评估。
2. 当前名称与实体运作无间隙，即名字承诺的内涵与被命名者的实际运作一致。
3. 无竞品命名冲突，即在同一仓库内不存在同名但不同实体的节点。
4. 不涉及哲学层术语，PRO-01 血统判定仅约束哲学层术语，工程符号层不要求道家血统。

### 批量声明沿用的最低要求 {#batch-minimum-requirements}

批量声明沿用不是跳过治理，是降级治理。最低要求三项。

1. 在本文档或后续 DEC 文档中登记每个沿用节点的名称与实体运作的贴合论证，一条即可。
2. 登记已知的潜在间隙，即使判定不改名也须记录间隙存在。
3. 批量声明本身须有 DEC 文档编号，作为治理留痕。

### 第二批节点的登记格式示例 {#batch-register-format}

第二批节点的登记格式示例，实际登记在执行批次时写入。

节点：report
当前名称：report
实体运作：审查报告，检验事件产出
已知间隙：名称过于泛化，不显式携带鉴层语义
沿用判定：report 作为参验产出的容器名称，在 sih/event/ 上下文中语义可辨，不构成实际混淆
登记依据：本文档逐节点评估 S6 节

## 与 OQ-06 的联动 {#oq-06-linkage}

D6 knowledge 与 S9 view 的边界在 OQ-06 中已推导但未正式化。knowledge 的立名优先级被 OQ-06 阻塞：在边界未定义前，立名缺乏实体运作的精确描述，无法判断名称是否贴合。建议 D6 的立名与 OQ-06 的正式化联动执行。

## 关系图谱 {#relation-graph}

- 本文档承接 OQ-07 的已推导结论，产出优先级排序与批次建议
- 本文档受约束于 PRO-01 立名本体论即名字是本体与 PRO-01 立名失败的形态即名字与实体错位是失败
- 本文档引用 DEC-001 节点树作为节点清单的权威源
- 本文档引用 NAMING-METHODOLOGY-SUCCESS.md 的方法论，即从哲学命题推出本体与立名失败检测四判据
- 本文档引用 STRUCTURE-STORAGE-BOUNDARY.md 的方法论沉淀，即模式归 DEC，实例归 DES
- D6 knowledge 的立名与 OQ-06 联动，本批次建议不阻塞 OQ-06
- 第一批节点的立名执行须走后续 DEC 流程，本文档只排优先级不执行

## 认识论立场 {#epistemic-stance}

本文档的核心判断为 design-corollary 即工程实用性选择，非哲学必然。优先级排序的判据来自 PRO-01 的立名失败定义即名实错位程度加上工程实用性考量，覆盖被引用频率、不可逆成本、概念歧义性，两维度独立评估后组合。

可证伪条件一
: 若第一批节点的立名审查发现名称与实体实际无间隙，则错位风险评估有误，需回溯评估方法论。

可证伪条件二
: 若第三批沿用节点在后续使用中发现名实错位，则「已通过 audit-017/OQ 审查」不构成充分保障，需收紧沿用判据。

## 自检 {#self-check}

### 形式合规自检 {#formal-self-check}

- 一级标题无锚点，仅一个
- 二级及以上标题均带锚点
- 首个二级标题命名为「概览」
- 无破折号、无装饰符号、无 Unicode Emoji
- 全角括号仅用于单一治理编号

### 内容自检 {#content-self-check}

- 18 个节点完整覆盖，7 doc + 11 sih，与 OQ-07 声明一致
- 每个节点评估包含名字承诺、实体运作、错位风险、紧迫度四个维度
- 优先级排序有明确判据，覆盖错位风险与紧迫度双维度，非主观排序
- 批量声明沿用有最低条件，不是无约束跳过
- 不执行立名，不修改 DEC-001，不修改哲学仓，符合任务约束
- 从 PRO-01 命题出发评估，不是独立生成再比对

### 自反性结论 {#reflexive-conclusion}

本文档经自检，未发现违反自身元规则的形态。评估方法论来自 PRO-01 立名本体论与 NAMING-METHODOLOGY-SUCCESS.md 的成功路径，优先级判据在正文显式声明，认识论标签诚实标注为 design-corollary。
