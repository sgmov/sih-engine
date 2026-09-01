---
entry: ENTRY-ORD-014.md
agent: 复归段补强波簇B
proposed_id: ORD-014
subrepo: order
id_reason: order 子仓 INDEX 现行已建至 ORD-010，ORD-011 至 013 由本波簇 A 预留，本件取下一空号 ORD-014
anchors:
  - pro: PRO-01
    source: sih-philosophy/emanation/proodos/01-ontology-of-names.md:14
    quote: "司衡认为名字是本体，一个名字一旦确立，它参与构成被命名者"
  - pro: PRO-01
    source: sih-philosophy/emanation/proodos/01-ontology-of-names.md:18
    quote: "承诺不能事后撤回"
selfcheck:
  - 锚二条 grep -nF 复核命中，行 14 与行 18 各命中一次
  - 锚纪律：两引文均不含全角括号与破折号，行 14 引文为整行前段，整行含全角括号实体二字，改取前段事实入本记录
  - 三查：全文全角括号中文内容零处、破折号字符零处、围栏代码块零处
  - 八节在场且顺序固定，首行 H1 形对
  - 数学主张：乘积格在逐点序下为完全格，交与并逐分量计算；耦合算子单调，其不动点集为完全格，最小耦合解为 lfp(H)；F 与 G 保链上确界时 Kleene 迭代达最小解且序列逐点不减，迭代单调性证明含基础步与归纳步；常映射特例退化为 ORD-009 单向模型且唯一耦合解为 (n0, F(n0))；血统三档边界为单调性只在命名传统内成立
---

# ORD-014 耦合不动点与相互构成

状态：草稿，复归段补强波簇B 产出，哲学到工程桥梁条目。

## 定义 {#definition}

耦合不动点是两个完全格之积上耦合算子的不动点，是名字承诺与实体状态双向构成的数学形态；相互构成即名字决定实体且实体反定名字，非单向闭包。

设 $(N, \leq_N)$ 是完全格，名字承诺状态空间，设 $(E, \leq_E)$ 是完全格，实体状态空间。$F: N \to E$ 是单调算子，名字构成实体，名字状态决定实体状态；$G: E \to N$ 是单调算子，实体反定名字，实体运作反过来固定名字承诺。原文「一个名字一旦确立，它参与构成被命名者」是两者构成的双向方程组。

定理一，乘积格。$N \times E$ 在逐点序下为完全格，

$$(n_1, e_1) \leq (n_2, e_2) \iff n_1 \leq_N n_2 \text{ 且 } e_1 \leq_E e_2 \label{eq:ord014-pointwise-order}$$

交与并逐分量计算，

$$(n_1, e_1) \vee (n_2, e_2) = (n_1 \vee_N n_2,\ e_1 \vee_E e_2), \quad (n_1, e_1) \wedge (n_2, e_2) = (n_1 \wedge_N n_2,\ e_1 \wedge_E e_2) \label{eq:ord014-product-lattice}$$

证明：设 $S \subseteq N \times E$，$\pi_N$ 与 $\pi_E$ 是两个投影，上确界逐分量计算为 $\vee S = (\vee_N \pi_N(S),\ \vee_E \pi_E(S))$，两个分量确界由完全格条件存在；对一切 $(s_1, s_2) \in S$ 有 $(s_1, s_2) \leq \vee S$，任一上界 $v$ 满足 $s_1 \leq v_1$ 与 $s_2 \leq v_2$ 逐点，故 $\vee S \leq v$，上确界成立；下确界对偶。$\square$

耦合算子定义为

$$H: N \times E \to N \times E, \quad H(n, e) = (G(e), F(n)) \label{eq:ord014-coupled-operator}$$

$H$ 是单调算子。证明：若 $(n_1, e_1) \leq (n_2, e_2)$，则 $e_1 \leq_E e_2$ 且 $n_1 \leq_N n_2$，由分量单调性 $G(e_1) \leq_N G(e_2)$ 且 $F(n_1) \leq_E F(n_2)$，即 $H(n_1, e_1) \leq H(n_2, e_2)$。$\square$

定理二，耦合不动点存在。不动点集 $\{(n, e) \in N \times E \mid H(n, e) = (n, e)\}$ 构成完全格。证明：对定理一的乘积格与单调算子 $H$ 应用 Knaster-Tarski 定理 (ORD-003)，见公式 $\ref{eq:ord003-monotone}$。最小耦合解 $(n^*, e^*) = \mathrm{lfp}(H)$。当 $F$ 与 $G$ 均保链上确界时，$(n^*, e^*) = \bigvee_{k \geq 0} (n_k, e_k)$，Kleene 迭代

$$(n_{k+1}, e_{k+1}) = (G(e_k), F(n_k)) \label{eq:ord014-kleene}$$

自 $(n_0, e_0) = (\bot_N, \bot_E)$ 出发。迭代刻画须加连续性前提，ORD-003 已给不连续反例，引用不重复。

命题三，迭代单调性，即不可撤回的数学形态。Kleene 迭代序列逐点不减，即 $n_k \leq_N n_{k+1}$ 且 $e_k \leq_E e_{k+1}$ 对一切 $k$。证明：基础步，$(n_0, e_0) = (\bot_N, \bot_E)$ 是乘积格最小元，故 $(n_0, e_0) \leq (n_1, e_1)$；归纳步，设 $(n_k, e_k) \leq (n_{k+1}, e_{k+1})$，即 $n_k \leq_N n_{k+1}$ 与 $e_k \leq_E e_{k+1}$，由 $G$ 的单调性 $G(e_k) \leq_N G(e_{k+1})$，即 $n_{k+1} \leq_N n_{k+2}$，由 $F$ 的单调性 $F(n_k) \leq_E F(n_{k+1})$，即 $e_{k+1} \leq_E e_{k+2}$；由归纳，序列逐点不减。$\square$

含义：承诺一经迭代获得即在后续一切状态与极限中保留，撤回即非单调，被模型排除。此即原文「承诺不能事后撤回」的数学形态。结合定理二，逐点不减序列的上确界即最小耦合解，迭代获得的一切承诺均被其包含。

命题四，单向特例，与 ORD-009 的分界。$G$ 为常映射，即 $G(e) = n_0$ 对一切 $e \in E$ 时，耦合系统退化为单方向：名字分量固定于 $n_0$，实体分量由名字决定，唯一耦合解为 $(n_0, F(n_0))$。证明：不动点方程要求 $n = G(e) = n_0$ 且 $e = F(n)$，代入得 $e = F(n_0)$，唯一性由两分量分别确定。$\square$ 该退化形态与 ORD-009 的承诺闭包模型 $e = cl(\Sigma(n))$，见公式 $\ref{eq:ord009-entity}$，同形：名字外生固定，实体是名字的确定函数。ORD-009 是本条在 $G$ 恒定下的特例，本条把名字本身纳入演化。

血统三档与算子变更。命名传统内 $F$ 与 $G$ 固定，迭代序列按命题三单调；跨传统改名即算子变更，序列可非单调，ORD-009 的保留、延伸、限定三档分类相邻版本名字承诺状态之差。单调性只在传统内成立，边界如实标注。

## 公理条件 {#axioms}

耦合不动点与相互构成的成立依赖以下公理条件。

- 完全格：$(N, \leq_N)$ 与 $(E, \leq_E)$ 是完全格，见公式 $\ref{eq:ord002-complete-lattice}$
- 单调算子：$F: N \to E$ 与 $G: E \to N$ 满足单调性条件，见公式 $\ref{eq:ord003-monotone}$
- 连续性前提：定理二的 Kleene 迭代刻画要求 $F$ 与 $G$ 均保链上确界，链见公式 $\ref{eq:ord005-chain}$；无此前提时不动点集仍为完全格，但 $\omega$ 步迭代未必到达
- 常映射：命题四要求 $G$ 为常映射，退化形态非任意耦合系统自动满足

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Knaster-Tarski 定理在乘积格的应用，耦合不动点与博弈论均衡存在，不动点迭代
- 哲学命题：PRO-01 名字是本体与承诺不能事后撤回
- 形式化：原文「名字一旦确立，它参与构成被命名者」是双向构成，数学形态即耦合方程组 $e = F(n)$ 且 $n = G(e)$，非单向闭包；「承诺不能事后撤回」即 Kleene 迭代单调性，承诺状态逐点不减；后续推导受名字约束即实体分量由 $F$ 从名字分量决定。与 ORD-009 的分界：ORD-009 承单向闭包模型，名字外生，本条承耦合模型，名字内生演化，ORD-009 是特例

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 报告版本演化
- 形式化：facet 报告承诺状态，引用术语集与判据集，与被治理实体行为状态相互构成，落入耦合方程组；批内版本迭代为 Kleene 型迭代，逐点单调，承诺只累积；跨批治理规则变更即算子变更，是血统事件，三档分类适用
- 借鉴方向：耦合系统的不动点结构给报告承诺与实体行为的共演化提供模型，命题三支持批内承诺只累积的判定
- 边界：$F$ 与 $G$ 的具体定义与状态空间偏序构造由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- ORD-002 完全格：$N$ 与 $E$ 为完全格，见公式 $\ref{eq:ord002-complete-lattice}$，乘积格定理一是其直接应用
- ORD-003 Knaster-Tarski 不动点：定理二对乘积格应用 ORD-003，见公式 $\ref{eq:ord003-monotone}$，不连续反例引自该条
- ORD-004 monotone operator：$F$ 与 $G$ 是单调算子，见公式 $\ref{eq:ord004-monotone}$，$H$ 是两者的耦合形式
- ORD-005 链与反链：连续性前提为 $F$ 与 $G$ 保链上确界，链见公式 $\ref{eq:ord005-chain}$
- ORD-009 承诺闭包与立名判定：命题四示 ORD-009 的单向闭包模型是本条在常 $G$ 下的特例，见公式 $\ref{eq:ord009-entity}$，血统三档分类承自该条
- PRO-01 立名本体论：本条的源命题，双向构成与不可撤回是其数学形态

## 历史脉络 {#history}

- 1955 年 Tarski 证明完全格上单调算子的不动点定理
- 乘积格在逐点序下为完全格、交并逐分量计算，是序论教科书标准内容
- 1973 年 Scarf 的博弈均衡存在定理用策略格乘积，耦合不动点视角进入博弈论
- Picard 迭代传统研究不动点迭代序列的收敛，本条迭代在格序上进行，与度量空间上的 Picard 迭代不同空间

## 工程注意事项 {#engineering-notes}

应用耦合不动点迭代时需验证四件事。

1. 收敛判据：耦合迭代工程近似为 $k$ 轮往复，收敛判据为相邻两轮状态相等，即 $(n_{k+1}, e_{k+1}) = (n_k, e_k)$
2. 不收敛标注：有限步不收敛说明算子不保链上确界，须如实标注，并升级超限迭代或判定标记，不静默丢弃
3. 算子变更记录：版本控制须记录 $F$ 与 $G$ 算子与迭代轨迹，算子变更须走血统三档分类，否则序列单调性跨版本不可比
4. 机械可检：乘积格逐点序在状态空间有界时机械可检，单调性与迭代单调性可逐件核验

## 参考文献 {#references}

- Tarski, A. (1955). A lattice-theoretical fixpoint theorem and its applications. Pacific J. Math., 5(2), 285-309
- Scarf, H.E. (1973). Theorems of games with quasi-concave payoff functions
- Wikipedia "Knaster-Tarski theorem" 条目
- Wikipedia "Fixed-point theorem" 条目
