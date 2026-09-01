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
  - pro: EPI-15
    source: sih-philosophy/emanation/epistrophe/15-on-falsifiability.md:65
    quote: "tautology 在所有条件下成立，无适用边界"
selfcheck:
  - 锚二条 grep -nF 复核命中，行 44 与行 65 各命中一次
  - 锚纪律：两引文均不含全角括号与破折号，行 44 引文为整行破折号前段，行 65 引文为整行破折号后段
  - 三查：全文全角括号中文内容零处、破折号字符零处、围栏代码块零处
  - 八节在场且顺序固定，首行 H1 形对
  - 数学主张：零内容定理在离散观察空间为当且仅当，一般空间只给几乎必然方向并以 Lebesgue 反例标注边界；内容单调性单向成立，四元均匀空间反例示等内容不蕴含逻辑蕴含；合取内容 C(P 交 Q) 不减于 max(C(P), C(Q))；四态映射与治理权重只给定义不立治理结论
---

# PROB-008 信息内容与可证伪性

状态：草稿，复归段补强波簇B 产出，哲学到工程桥梁条目。

## 定义 {#definition}

信息内容是命题为假的概率的测度，是 Popper 信息含量思想的测度论形态；可证伪性按内容大小与证伪条件覆盖判定。

设 $(\Omega, \mathcal{F}, P)$ 是概率空间 (PROB-006)，见公式 $\ref{eq:prob006-countable-additivity}$。命题给定为可测事件，即 $\Omega$ 上的 $\mathcal{F}$-可测子集，承载该命题在所有观察下的成立情形。本条中测度字母与命题事件字母同形，均为 $P$：在 $P(P^c)$ 形式的表达式中，外层 $P$ 是概率测度，内层 $P$ 是命题事件，$P^c = \Omega \setminus P$ 是补事件。

命题 $P$ 的信息内容定义为

$$C(P) := P(P^c) = 1 - P(P) \label{eq:prob008-info-content}$$

即 Popper 以命题为假的概率度量其信息含量，本条给测度论形态。

定理一，零内容定理。

- 第 I 段：$P$ 为重言，即 $P = \Omega$，蕴含 $C(P) = 0$。证明：$P^c = \emptyset$，$P(\emptyset) = 0$。此段不依赖空间附加条件。
- 第 II 段：离散观察空间，即 $\Omega$ 上每单点可测且测度严格为正，$P = \Omega$ 当且仅当 $C(P) = 0$。反向证明：若 $P \neq \Omega$，则 $P^c \neq \emptyset$，取 $x \in P^c$，由单点可测与测度单调性 $P(P^c) \geq P(\{x\}) > 0$，即 $C(P) > 0$。
- 第 III 段：一般空间，$C(P) = 0$ 蕴含 $P$ 几乎必然成立，即补事件 $P^c$ 测度为零；$P$ 几乎必然成立不蕴含 $P = \Omega$，即 $C(P) = 0$ 不蕴含 $P$ 为重言。反例：$\Omega = [0, 1]$，$\mathcal{F}$ 为 Borel sigma 代数，$P$ 为 Lebesgue 测度，取事件 $A = [0, 1)$，则 $C(A) = P(\{1\}) = 0$ 而 $A \neq \Omega$。边界标注：连续观察空间下，零内容检查只能降级为几乎必然检查。

定理二，内容单调性。$P$ 逻辑蕴含 $Q$，即事件包含 $P \subseteq Q$，蕴含 $C(P) \geq C(Q)$。证明：补集反向包含，$P \subseteq Q$ 给出 $Q^c \subseteq P^c$，由测度单调性 $P(Q^c) \leq P(P^c)$，即 $C(Q) \leq C(P)$。$\square$

逆否不成立。取 $\Omega$ 为四元均匀空间，即 $\Omega = \{a, b, c, d\}$ 且 $P(\{x\}) = 1/4$ 对每个 $x$，取事件 $A = \{a, b\}$ 与 $B = \{c, d\}$，则 $C(A) = C(B) = 1/2$ 而 $A$ 与 $B$ 不相交，互不蕴含。即等内容不蕴含逻辑蕴含，单调性单向成立，非等价。

命题三，合取内容。对任意两事件 $P$ 与 $Q$，

$$C(P \cap Q) \geq \max(C(P), C(Q)) \label{eq:prob008-conjunction}$$

证明：$(P \cap Q)^c = P^c \cup Q^c$ 含 $P^c$ 且含 $Q^c$，由测度单调性 $C(P \cap Q) = P((P \cap Q)^c) \geq P(P^c) = C(P)$，同理 $C(P \cap Q) \geq C(Q)$。$\square$ 用途：命题加强即合取，内容不减，命题越强越易被证伪。

四态映射，对应原文判定标记。在观测语言给定下，命题按内容与证伪条件分为四态。

- 自明或重言态：$C(P) = 0$
- 可证伪态：$C(P) > 0$ 且证伪条件满足三要素，三要素判定承 ORD-010
- 部分可证伪态：$C(P) > 0$ 但指定证伪条件只覆盖可证伪域的一部分，剩余内容未被条件覆盖
- 不可证伪态，分两源：重言源即 $C(P) = 0$；定理层源即命题由公理层推论担保，观测不介入

原文明确不可证伪不等于伪命题，数学形态即 $C(P) = 0$ 的命题仍可为真，重言式为典型。四态按内容与证立方式分类，不按真假分类。

治理权重，对应原文「硬约束不怕错，怕不可被证伪」。治理风险权重落在 $C(P)$ 上而非 $P$ 的真假上：$C(P) = 0$ 的硬约束不可被观察反证，$C(P) > 0$ 的硬约束可被观察反证。本条只给权重定义，不立治理结论。

## 公理条件 {#axioms}

信息内容与四态映射的成立依赖以下公理条件。

- 概率空间：$(\Omega, \mathcal{F}, P)$ 已配置，含 PROB-006 的全部公理条件，即非负性、规范性、可列可加性、定义域封闭
- 可测性：命题事件属于 $\mathcal{F}$，内容只定义在可测事件上
- 离散性：定理一第 II 段要求 $\Omega$ 上每单点可测且测度严格为正；无此条件时反向弱化为第 III 段的几乎必然方向
- 证伪条件：可证伪态与部分可证伪态要求证伪条件已被指定，三要素判定承 ORD-010
- 定理层源：不可证伪态的定理层源要求公理层已给定，命题由公理层推论担保
- 治理权重：不引入新公理，只是 $C(P)$ 上的权重定义

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Popper 可证伪划界标准与信息含量讨论，测度论概率
- 哲学命题：EPI-15 可证伪性检测，四态判定与不可证伪不等于伪命题
- 形式化：原文「真命题的标志：敢于自我证伪，明确指定证伪条件」即 $C(P) > 0$ 加证伪条件三要素满足；「tautology 在所有条件下成立，无适用边界」的数学形态即 $P = \Omega$，$C(P) = 0$；四态按内容分界，重言态零内容，可证伪态正内容加三要素，部分可证伪态正内容而条件覆盖不全，定理层不可证伪态内容被公理层吸收；不怕错怕不可被证伪即治理权重在内容不在真假。与 ORD-010 的分界：ORD-010 给三要素可判定性与反例搜索半可判定性，定性侧，本条给内容测度 $C$ 与单调性，定量侧，两者互补

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 判据读数与排序
- 形式化：$C$ 等于零的判据在所有观察下成立，是重言判据，读数为噪声，不进入判定；内容单调性给判据排序依据，判据越强内容越大，被证伪风险越高；多厂读数排序可按内容分级，高内容判据异常优先报
- 借鉴方向：$C$ 等于零检查是机械预过滤，离散观察集下可枚举执行；内容分级要求观测语言与先验固定，先验变更时分级不可比
- 边界：观察空间构造与先验选定由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- PROB-006 概率测度：$C(P)$ 定义完全依赖概率测度，见公式 $\ref{eq:prob006-countable-additivity}$，补集公式与测度单调性是其直接推论
- ORD-010 证伪条件与反例搜索：ORD-010 给三要素与四态判定的定性形态，见公式 $\ref{eq:ord010-observation-range}$ 与 $\ref{eq:ord010-falsification-condition}$；本条给内容测度 $C$ 的定量形态，定性侧与定量侧互补
- ORD-002 完全格：事件族 $\mathcal{F}$ 对补集与交运算封闭，内容单调性与合取内容在此封闭性上良定义；事件族在包含序下一般不构成完全格，本条不要求完全格结构
- EPI-15 可证伪性检测：本条的源命题，四态映射是其判定标记的数学形态

## 历史脉络 {#history}

- 1933 年 Kolmogorov 在 Grundbegriffe der Wahrscheinlichkeitsrechnung 中确立概率论的测度论框架
- 1934 年 Popper 在 Logik der Forschung 中提出划界标准，科学命题须有明确证伪条件
- 1963 年 Popper 在 Conjectures and Refutations 中讨论信息含量，理论禁止的可能事态越多，信息含量越大
- Popper 自身的内容测度尝试被批非机械，本条 $C(P) = P(P^c)$ 版本为测度论改写，如实标注；年份按标准史实

## 工程注意事项 {#engineering-notes}

使用信息内容时需验证四件事。

1. 先验来源：$C$ 依赖观察空间上的先验 $P$，先验由谁给、如何版本化是工程层问题，本条不解决，先验未登记时比较无意义
2. 离散场景：离散治理日志场景下 $C$ 等于零检查机械可执行，枚举观察集逐件查成员关系
3. 连续场景：连续观察场景下零内容弱化为几乎必然，判定须降级标注，不报为严格 $C$ 等于零
4. 先验一致：内容单调性用于判据排序时注意先验一致，跨先验比较无意义

## 参考文献 {#references}

- Popper, K. (1934). Logik der Forschung. Johann Ambrosius Barth, Leipzig
- Popper, K. (1963). Conjectures and Refutations. Routledge
- Kolmogorov, A.N. (1933). Grundbegriffe der Wahrscheinlichkeitsrechnung. Springer
- Billingsley, P. (1995). Probability and Measure, 3rd ed. Wiley
- Wikipedia "Falsifiability" 条目
- Wikipedia "Logical probability" 条目
