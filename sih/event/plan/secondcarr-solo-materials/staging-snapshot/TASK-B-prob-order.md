# 复归段补强波 簇 B 任务书（EPI-15 概率件 + PRO-01 序论件）

编排主代理落盘。你是复归段补强波簇 B 子代理，零判定权：按本任务书起草两件数学条目草稿，机械格式与数学正确性由主线验收，你不做内容裁量，拿不准的数学陈述不写。

## 边界铁律

- 只允许写 /Users/moc/workspaces/SiHankor/agent-drafts/epi/ 下你的两件产出：草稿 ENTRY-PROB-008.md、ENTRY-ORD-014.md 与认领行 CLAIMS-B.ndjson
- 其余一切只读。禁止入仓、跑 lease/scribe/git、改 AGENTS.md 与任何 INDEX/mapping、上链
- 拟派号只记在草稿 front matter 的 proposed_id，主线入闸批才做 ID 与表登记

## 硬性格式法（违一件即废，先跑后写）

1. 首行 front matter 后正文第一非空行即 H1，形如 `# PROB-008 信息内容与可证伪性`，前缀限 PROB 或 ORD
2. 第一个二级标题必须是 `## 定义 {#definition}`
3. 全文零全角括号中文内容：全角括号内只许纯 ASCII（英文名、ID 编号），中文补充一律改逗号句或「即」字句；零破折号——；零围栏代码块（LaTeX 用 $ 与 $$）
4. 结构照范本 /Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-007-expectation.md：H1、状态行、八节 定义/公理条件/哲学桥接/在 facet 的应用/与其他概念的关系/历史脉络/工程注意事项/参考文献，锚点照抄。状态行写 `状态：草稿，复归段补强波簇B 产出，哲学到工程桥梁条目。`。哲学桥接节用三条子项：借鉴源、哲学命题、形式化。正文不必逐字引原文，转述即可，引文只落 front matter
5. 哲学锚每件恰二条，纪律五条：引文与源行逐字节一致（连续片段禁删节）；引文不含全角括号与破折号；行号已预验（见下）；锚命题必须是本战线命题本身即 EPI-15 或 PRO-01，不得换成别的；front matter 内引文用双引号包裹

## 草稿 front matter 形态

```
---
entry: ENTRY-PROB-008.md
agent: 复归段补强波簇B
proposed_id: PROB-008
subrepo: probability
id_reason: probability 子仓 INDEX 现行已建至 PROB-007，下一空号 PROB-008
anchors:
  - pro: EPI-15
    source: sih-philosophy/emanation/epistrophe/15-on-falsifiability.md:44
    quote: "真命题的标志：敢于自我证伪"
selfcheck:
  - 锚二条 grep -nF 复核命中
  - 三查：全文全角括号中文内容零处、破折号字符零处、围栏代码块零处
  - 八节在场且顺序固定，首行 H1 形对
  - 数学主张：（每件如实写本件的可判定主张与边界）
---
```

## 件一 ENTRY-PROB-008.md：PROB-008 信息内容与可证伪性（承载 EPI-15）

id_reason：probability 子仓 INDEX 现行已建至 PROB-007，下一空号 PROB-008。

锚二条（已预验命中，行号勿动）：
- pro EPI-15，source sih-philosophy/emanation/epistrophe/15-on-falsifiability.md:44，quote "真命题的标志：敢于自我证伪"
- pro EPI-15，source sih-philosophy/emanation/epistrophe/15-on-falsifiability.md:65，quote "tautology 在所有条件下成立，无适用边界"

数学核心：
- 设定：概率空间 (Omega, F, P)（承 PROB-006，引用其公理节）。命题 P 为可测事件，即 Omega 上的 F-可测子集，承载该命题在所有观察下的成立情形
- 信息内容定义（公式，带 label）：C(P) := P(P 的补) = 1 − P(P)。Popper 以命题为假的概率度量其信息含量，本条给测度论形态
- 零内容定理（定理一）：P 为重言，即 P 等于全集 Omega，当且仅当 C(P) = 0。离散观察空间，即 Omega 上每单点测度非零，此当且仅当仍成立。连续空间情形只给几乎必然方向：C(P) = 0 蕴含 P 几乎必然成立，反之不必然，边界如实标注
- 内容单调性定理（定理二）：P 逻辑蕴含 Q，即事件包含 P 含于 Q，蕴含 C(P) ≥ C(Q)。证明：补集反向包含，测度单调。逆否不成立段：取 Omega 为四元均匀空间，P 与 Q 为不相交两元子集，则 C(P) = C(Q) = 1/2 而 P 与 Q 互不蕴含，单调性非等价
- 合取内容命题（命题三）：C(P 交 Q) ≥ max(C(P), C(Q))。证明：P 交 Q 的补含 P 的补且含 Q 的补，测度次可加给出方向。用途：命题加强即合取，内容不减，命题越强越易被证伪
- 四态映射（定义段，对应原文判定标记）：在观测语言给定下。自明或重言态即 C(P) = 0。可证伪态即 C(P) > 0 且证伪条件满足三要素，三要素判定承 ORD-010。部分可证伪态即 C(P) > 0 但指定证伪条件只覆盖可证伪域的一部分，剩余内容未被条件覆盖。不可证伪态分两源：重言源即 C(P) = 0，定理层源即命题由公理层推论担保，观测不介入。原文明确不可证伪不等于伪命题，数学形态即 C(P) = 0 的命题仍可为真，重言式为典型
- 治理权重命题（一段，对应原文「硬约束不怕错，怕不可被证伪」）：治理风险权重落在 C(P) 上而非 P 的真假上：C(P) = 0 的硬约束不可被观察反证，C(P) > 0 的硬约束可被观察反证。本条只给权重定义，不立治理结论

哲学桥接节要点：
- 借鉴源：Popper 可证伪划界标准与信息含量讨论，测度论概率
- 哲学命题：EPI-15 可证伪性检测，四态判定与不可证伪不等于伪命题
- 形式化：原文「敢于自我证伪，明确指定证伪条件」即 C(P) > 0 加证伪条件三要素满足；「tautology 在所有条件下成立，无适用边界」的数学形态即 P 等于全集，C(P) = 0；四态按内容分界，重言态零内容，可证伪态正内容加三要素，部分可证伪态正内容而条件覆盖不全，定理层不可证伪态内容被公理层吸收；不怕错怕不可被证伪即治理权重在内容不在真假。与 ORD-010 的分界：ORD-010 给三要素可判定性与反例搜索半可判定性，定性侧，本条给内容测度 C 与单调性，定量侧，两者互补

facet 应用节：facet 判据读数，C 等于零的判据在所有观察下成立，是重言判据，读数为噪声；内容单调性给判据排序依据，判据越强内容越大，被证伪风险越高；多厂读数排序可按内容分级，高内容判据异常优先报

历史脉络节：Popper 1934 Logik der Forschung 划界标准与 1963 Conjectures and Refutations 信息含量讨论；Kolmogorov 1933 测度论框架；Popper 自身内容测度尝试被批非机械，本条 P 的补测度版本为测度论改写，如实标注。年份按标准史实

工程注意事项节：C 依赖观察空间上的先验 P，先验由谁给、如何版本化是工程层问题，本条不解决；离散治理日志场景下 C 等于零检查机械可执行，枚举观察集；连续观察场景零内容弱化为几乎必然，判定须降级标注；内容单调性用于判据排序时注意先验一致，跨先验比较无意义

参考文献节：Popper 1934 与 1963；Kolmogorov 1933；Billingsley 1995 Probability and Measure；Wikipedia 可证伪性与逻辑概率条目

## 件二 ENTRY-ORD-014.md：ORD-014 耦合不动点与相互构成（承载 PRO-01）

id_reason：order 子仓 INDEX 现行已建至 ORD-010，ORD-011 至 013 由本波簇 A 预留，本件取下一空号 ORD-014。

锚二条（已预验命中，行号勿动）：
- pro PRO-01，source sih-philosophy/emanation/proodos/01-ontology-of-names.md:14，quote "司衡认为名字是本体，一个名字一旦确立，它参与构成被命名者"
- pro PRO-01，source sih-philosophy/emanation/proodos/01-ontology-of-names.md:18，quote "承诺不能事后撤回"

数学核心：
- 设定：完全格 (N, ≤_N) 为名字承诺状态空间，完全格 (E, ≤_E) 为实体状态空间。F: N → E 单调，名字构成实体，名字状态决定实体状态。G: E → N 单调，实体反定名字，实体运作反过来固定名字承诺。原文「名字参与构成被命名者」即双向方程组
- 乘积格定理（定理一）：N 乘 E 以逐点序为完全格，交与并逐分量计算。证明直接
- 耦合算子定义：H: N 乘 E → N 乘 E，H(n, e) = (G(e), F(n))，单调，分量单调性直接得
- 耦合不动点存在定理（定理二）：H 的不动点集构成完全格，承 ORD-003 Knaster-Tarski 定理作用于乘积格。最小耦合解 (n*, e*) = lfp(H)。当 F 与 G 均保链上确界时，(n*, e*) = 所有 k 上 (n_k, e_k) 的并，Kleene 迭代 (n_{k+1}, e_{k+1}) = (G(e_k), F(n_k)) 自 (⊥_N, ⊥_E) 出发。迭代刻画须加连续性前提，ORD-003 已给不连续反例，引用不重复
- 迭代单调性命题（命题三，不可撤回的数学形态）：Kleene 迭代序列逐点不减，即 n_k ≤ n_{k+1} 且 e_k ≤ e_{k+1} 对一切 k。证明：Kleene 迭代自最小元出发，单调算子下序列单调，基础步由最小元性质得，归纳步由算子单调性得。含义：承诺一经迭代获得即在后续一切状态与极限中保留，撤回即非单调，被模型排除。此即原文「承诺不能事后撤回」的数学形态
- 单向特例（命题四，与 ORD-009 的分界）：G 为常映射时耦合系统退化为单方向，名字固定，实体由名字决定，即 ORD-009 的承诺闭包模型 e 等于 cl(Sigma(n))。ORD-009 是本条在 G 恒定下的特例，本条把名字本身纳入演化
- 血统三档与算子变更（一段）：命名传统内 F 与 G 固定，迭代单调；跨传统改名即算子变更，序列可非单调，三档保留延伸限定分类相邻版本名字承诺状态之差，单调性只在传统内成立，边界如实标注

哲学桥接节要点：
- 借鉴源：Knaster-Tarski 定理在乘积格的应用，耦合不动点与博弈论均衡存在，不动点迭代
- 哲学命题：PRO-01 名字是本体与承诺不能事后撤回
- 形式化：原文「名字一旦确立，它参与构成被命名者」是双向构成，数学形态即耦合方程组 e = F(n) 且 n = G(e)，非单向闭包；「承诺不能事后撤回」即 Kleene 迭代单调性，承诺状态逐点不减；后续推导受名字约束即实体分量由 F 从名字分量决定。与 ORD-009 的分界：ORD-009 承单向闭包模型，名字外生，本条承耦合模型，名字内生演化，ORD-009 是特例

facet 应用节：facet 报告版本演化，报告承诺状态，引用术语集与判据集，与被治理实体行为状态相互构成；批内版本迭代单调，承诺只累积；跨批治理规则变更即算子变更，是血统事件，三档分类适用

历史脉络节：Tarski 1955 定理，乘积格应用为教科书标准内容；Scarf 1973 博弈均衡存在定理用策略格乘积；Picard 迭代传统。年份按标准史实

工程注意事项节：耦合迭代工程近似为 k 轮往复，收敛判据为相邻两轮状态相等；有限步不收敛说明算子不保链上确界，须如实标注并升级超限迭代或判定标记；版本控制须记录 F 与 G 算子与迭代轨迹，算子变更须走血统三档分类；乘积格逐点序在状态空间有界时机械可检

参考文献节：Tarski 1955；Scarf 1973 Theorems of games with quasi-concave payoff functions；Wikipedia Knaster-Tarski 与不动点定理条目

## 交稿要求

- 交稿前自检：对每件草稿跑全文检查，全角括号内若含中文即改；破折号零；围栏代码块零；八节在场；front matter YAML 可解析。自检结论写入 front matter selfcheck 字段，如实写
- 两件完成后逐件追加一行 CLAIMS-B.ndjson，形如 {"entry":"ENTRY-PROB-008.md","agent":"复归段补强波簇B","pro":"EPI-15","anchors":2,"selfcheck":"passed"}
- 数学拿不准的陈述不写，宁少勿错。反例与适用边界如实标注
- 只读参考材料：范本 sih-math/probability/entries/PROB-007-expectation.md；现有承载件 sih-math/order/entries/ORD-003 与 ORD-009 与 ORD-010（对照防重复，引用其公式号须核对实体存在）；原文 sih-philosophy/emanation/epistrophe/15-on-falsifiability.md 与 sih-philosophy/emanation/proodos/01-ontology-of-names.md；llm 条目 sih-philosophy/llm-friendly-build/entries/EPI-15.md 与 PRO-01-name-ontology.md 与 PRO-01-lineage-principles.md 与 PRO-01-naming-failure.md
