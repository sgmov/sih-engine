---
entry: ENTRY-ORD-005.md
agent: entry-ord-005
anchors:
  - pro: PRO-02 道一
    source: sih-philosophy/emanation/proodos/02-on-first-tao.md:115
    quote: "道一：发散是默认方向，治理是收敛的构成性条件。"
  - pro: PRO-02 道一·发散形态
    source: sih-philosophy/emanation/proodos/02-on-first-tao.md:67
    quote: "每个 AI 模型的输出天然地按训练数据的模式，生成是发散的"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---

# ORD-005 链与反链

状态：已建，哲学到工程桥梁条目。

## 定义 {#definition}

链与反链(Chain and Antichain)是偏序集的两种极端子集结构：链是全序子集，反链是两两不可比子集。它们是度量偏序「有序度」的基本对象，承载链的极限分析与反链判定。

设 $(P, \leq)$ 是偏序集(ORD-001)。子集 $C \subseteq P$ 称为链，若其中任意两元素可比：

$$\forall a, b \in C,\ a \leq b \ \vee \ b \leq a \label{eq:ord005-chain}$$

即 $C$ 连同继承的 $\leq$ 构成全序集。$P$ 整体为链当且仅当 $P$ 是全序集。

子集 $A \subseteq P$ 称为反链，若其中任意两个不同元素不可比：

$$\forall a, b \in A,\ a \neq b \implies a \not\leq b \wedge b \not\leq a \label{eq:ord005-antichain}$$

边界约定。单点集既是链也是反链；空集空真地两者皆是。

有限偏序集的两个不变量。

- 宽度(width)：最大反链的基数，记 $\mathrm{width}(P)$
- 链长(height)：最长链的基数，记 $\mathrm{height}(P)$

两个极小极大定理。

Dilworth 定理。设 $P$ 是有限偏序集，宽度为 $w$。则 $P$ 可划分为 $w$ 条链的并，且不能用少于 $w$ 条链划分；等价地，$P$ 的链划分所需最少链数等于 $\mathrm{width}(P)$。

Mirsky 定理(对偶表述)。设 $P$ 是有限偏序集，最长链长为 $h$。则 $P$ 可划分为 $h$ 个反链的并，且不能用少于 $h$ 个反链划分；等价地，$P$ 的反链划分所需最少反链数等于 $\mathrm{height}(P)$。

Dilworth 与 Mirsky 构成对偶的极小极大对：前者用最大反链给出最小链划分数，后者用最长链给出最小反链划分数。

应用方向。偏序维数与线性扩张：$P$ 的偏序维数是扩张 $\leq$ 的线性序族(线性扩张)中使交恰为 $\leq$ 的最少个数；链与反链分析是估计维数与构造扩张的基本工具。

## 公理条件 {#axioms}

链反链结构与两个定理依赖以下公理条件。

- 偏序：$(P, \leq)$ 满足自反、反对称、传递，即 ORD-001 全部公理
- 有限性：Dilworth 定理与 Mirsky 定理的前提均为有限偏序集
- 划分语义：链划分是两两不交且并为 $P$ 的链族；反链划分对偶
- 严格序：证明中使用的严格序 $a < b$ 定义为 $a \leq b$ 且 $a \neq b$，由偏序派生且无循环

有限性条件不可随意放宽到任意无限偏序集：此时宽度定义为反链基数的上确界，最大反链可以不存在，划分的链数下界可以严格大于宽度。但不等式 $\mathrm{width}(P) \leq$ 链划分最少链数不依赖有限性：每个反链与每条链至多相交于一个元素。

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Dilworth 定理由 Dilworth 1950 年确立，与二分图 König-Egervary 定理等价；Mirsky 定理 1971 年给出对偶形式
- 哲学命题：PRO-02 道一，发散自然、收敛必为。原文：「道一：发散是默认方向，治理是收敛的构成性条件。」
- 形式化：反链是发散的序论形态。原文：「每个 AI 模型的输出天然地按训练数据的模式，生成是发散的」，多认知源各自沿发散方向独立运作，在无比较序时不同源的状态两两不可比，正是反链的定义。链是治理收敛的序论形态：施加比较序后状态构成任意两元可比的单序轨迹。反链宽度度量「不可并序方向」的最大独立族规模，Dilworth 定理给出有限比较结构下收敛动作至多需宽度条链的极小极大边界；治理收敛即把反链并序化。两个定理本身不立治理结论，只给序结构以可计算的边界

## 证明思路 {#proof-sketch}

Dilworth 定理的证明思路是归约到 König-Egervary 定理(二分图中最大匹配等于最小点覆盖)。设 $P$ 是有限偏序集。

- 构造。取 $P$ 的两份拷贝 $P_L$、$P_R$，以 $P_L \cup P_R$ 为顶点集构成二分图 $G$，$a_L$ 与 $b_R$ 之间有边当且仅当 $a < b$
- 匹配到链划分。$G$ 中规模为 $\nu$ 的匹配 $M$，其边构成顶点不相交的有向路(严格偏序无循环)，每条路是一条链；路与未匹配的单点共同构成 $P$ 的划分，恰含 $|P| - \nu$ 条链。最大匹配给出 $|P| - \nu$ 条链的链划分，故链划分最少链数不超过 $|P| - \nu$
- 点覆盖到反链。设 $S_L \cup S_R$ 是 $G$ 的最小点覆盖，令 $T = \{a \in P \mid a_L \in S_L\} \cup \{b \in P \mid b_R \in S_R\}$。若 $x < y$ 且 $x, y \notin T$，则边 $x_L y_R$ 未被点覆盖，矛盾；故 $P \setminus T$ 是反链。由 König-Egervary 定理 $|T| \leq |S_L| + |S_R| = \nu$，故 $\mathrm{width}(P) \geq |P| - \nu$
- 合并上两条：链划分最少链数等于 $\mathrm{width}(P)$

Mirsky 定理走对偶论证：令 $L_i = \{x \in P \mid$ 以 $x$ 为顶的链最长为 $i\}$，$i = 1, \ldots, h$，则 $L_1, \ldots, L_h$ 是 $P$ 的反链划分；又任何反链与最长链至多相交于一元，故少于 $h$ 个反链不能覆盖 $P$。

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 多源判据读数与 verdict 结构
- 形式化：facet 的多个认知源(不同模型、不同 shot)各自独立产生判据读数，在无比较序时不同源的读数两两不可比，构成反链成员；反链宽度度量同一批材料中独立不可并序方向的最大规模。治理收敛动作(投票、聚合)即把读数集划分为链族的过程：Dilworth 定理给出有限比较结构下链划分最少链数等于宽度，为收敛动作必须开设的并序通道数提供下界
- 借鉴方向：判据读数集配偏序后，单调迭代链的构造依赖 ORD-004，链的上确界存在性依赖 ORD-002
- 边界：facet 判据读数偏序的具体定义由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- ORD-001 偏序集：链与反链是偏序集的派生子集结构，本条目是 ORD-001 派生概念中链反链两项的展开
- ORD-002 完全格：完全格中任意子集(含任意链，可数或不可数)都有上确界与下确界，链的极限在格内保证存在，见公式 $\ref{eq:ord002-complete-lattice}$
- ORD-003 Knaster-Tarski 不动点：单调算子的迭代序列 $x_0 \leq f(x_0) \leq f^2(x_0) \leq \cdots$ 沿链上升，Kleene 构造 $\bigvee \{f^n(\bot)\}$ 要求 $f$ 保链上确界，不动点是该链的极限
- ORD-004 monotone operator：保序迭代产生单调链，是本条目在迭代分析中的主要来源结构

## 历史脉络 {#history}

- 1914 年 Hausdorff 在 Grundzüge der Mengenlehre 中引入链与反链的现代术语
- 1931 年 König 与 Egervary 定理确立二分图最大匹配与最小点覆盖的等价，与 Dilworth 定理等价
- 1940 年 Dushnik 与 Miller 证明任意基数为 $\kappa$ 的偏序集可划分为 $\kappa$ 条链，给出无限推广形式
- 1950 年 Dilworth 在 A decomposition theorem for partially ordered sets 中证明链划分定理
- 1971 年 Mirsky 给出对偶形式的反链划分定理
- 1980s 后链反链结构在程序语义、抽象解释与组合优化中成为基本工具

## 工程注意事项 {#engineering-notes}

应用链与反链时需验证三件事。

1. 偏序是否良定义：链反链判定相对给定的偏序进行，ORD-001 公理须先成立；用临时比较关系做的判定不是链反链判定
2. 有限性条件是否满足：Dilworth 与 Mirsky 以有限偏序集为前提；无限偏序集的朴素表述一般不成立，需改用推广形式或附加条件
3. 划分是否真划分：两两不交且并为 $P$；用覆盖(允许重叠)代替划分是常见错误，Dilworth 结论对覆盖不直接成立

链反链的工程局限。宽度给出链划分链数的下界，不直接给出划分本身，划分需经匹配计算恢复。偏序维数判定比宽度计算更难：判定偏序维数是否不超过给定数属于 NP 完全问题，链反链分析只给估计工具。

## 参考文献 {#references}

- Hausdorff, F. (1914). Grundzüge der Mengenlehre. Veit
- Dilworth, R.P. (1950). A decomposition theorem for partially ordered sets. Ann. of Math., 51(1), 161-166
- Mirsky, A. (1971). A generalization of a theorem of Dilworth. Advances in Mathematics, 6(3), 268-270
- Dushnik, B. & Miller, E.W. (1940). Partially ordered sets. American Journal of Mathematics, 63(1), 439-449
- Davey, B.A. & Priestley, H.A. (2002). Introduction to Lattices and Order, 2nd ed. Cambridge University Press
- Wikipedia "Dilworth's theorem" 条目
