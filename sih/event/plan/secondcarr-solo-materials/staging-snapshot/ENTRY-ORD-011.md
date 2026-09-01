---
entry: ENTRY-ORD-011.md
agent: 复归段补强波簇A
proposed_id: ORD-011
subrepo: order
id_reason: order 子仓 INDEX 现行已建至 ORD-010，下一空号 ORD-011
anchors:
  - pro: EPI-12
    source: sih-philosophy/emanation/epistrophe/12-on-epistrophe.md:9
    quote: "但单向流衍有结构弱点：它只能证明「每步从前步来」，不能证明「每步真的」"
  - pro: EPI-12
    source: sih-philosophy/emanation/epistrophe/12-on-epistrophe.md:91
    quote: "Epistrophe 不是 Emanation 的替代或反驳。Epistrophe 是 Emanation 的镜像"
selfcheck:
  - "锚二条 grep -nF 复核命中，行号 9 与 91"
  - "三查：全文全角括号中文内容零处、破折号字符零处、围栏代码块零处"
  - "八节在场且顺序固定，首行 H1 形对，首节为定义"
  - "数学主张：镜像对偶定理在逆序双射 d 下给 lfp(f) 等于 d 逆像 gfp(f^d) 与 gfp(f) 等于 d 逆像 lfp(f^d)，f^d 单调性由三次逆序复合证得；两原理独立性由常顶与常底两映射例见证；可及元段仅在有限 S 假设下给可及元集与 lfp 重合，无限情形标注为子集"
  - "差异记录：任务书原例仅用常顶映射并称 1 不是后不动点，核验 1 是常顶映射后不动点，本件补常底映射例见证反向不蕴含，两原理互不蕴含成立，详见完成报告未决疑点"
---

# ORD-011 共归纳与镜像对偶

状态：草稿，复归段补强波簇A 产出，哲学到工程桥梁条目。

## 定义 {#definition}

共归纳 (Coinduction) 与镜像对偶 (Mirror Duality) 在完全格上单调算子的不动点结构之上，给出归纳原理与共归纳原理的两条推论性原理，以及逆序双射下不动点集的镜像对偶定理。

设 $(L, \leq)$ 是完全格，见公式 $\ref{eq:ord002-complete-lattice}$，$f: L \to L$ 是单调算子，见公式 $\ref{eq:ord004-monotone}$。$f$ 的前不动点即 $f(x) \leq x$ 的点，后不动点即 $f(x) \geq x$ 的点，最小不动点 $\mathrm{lfp}(f)$ 为全体前不动点之交，最大不动点 $\mathrm{gfp}(f)$ 为全体后不动点之并，皆已定于 ORD-003，见公式 $\ref{eq:ord003-monotone}$，本条不重复定义。

定理一，归纳原理。设 $R \in L$ 是 $f$ 的前不动点，即 $f(R) \leq R$，则 $\mathrm{lfp}(f) \leq R$。

证明。$\mathrm{lfp}(f)$ 是全体前不动点之交，而 $R$ 是其中一个前不动点，交不大于任一成员，故 $\mathrm{lfp}(f) \leq R$。

定理二，共归纳原理。设 $R \in L$ 是 $f$ 的后不动点，即 $f(R) \geq R$，则 $R \leq \mathrm{gfp}(f)$。

证明。$\mathrm{gfp}(f)$ 是全体后不动点之并，而 $R$ 是其中一个后不动点，并盖过任一成员，故 $R \leq \mathrm{gfp}(f)$。

命题，两原理独立性。前不动点类与后不动点类互不蕴含。取 $L$ 为二元素链 $0 < 1$，$0$ 为最小元，$1$ 为最大元。若 $f$ 为常顶映射 $f(x) = 1$ 对所有 $x$，则前不动点类为 $\{1\}$，后不动点类为 $\{0, 1\}$，即 $0$ 是后不动点而非前不动点，因 $f(0) = 1$ 不小于等于 $0$，示后不动点不蕴含前不动点。对偶地，若 $f$ 为常底映射 $f(x) = 0$ 对所有 $x$，则前不动点类为 $\{0, 1\}$，后不动点类为 $\{0\}$，即 $1$ 是前不动点而非后不动点，因 $f(1) = 0$ 不小于等于 $1$，示前不动点不蕴含后不动点。前者 $\mathrm{lfp}(f) = \mathrm{gfp}(f) = 1$，后者 $\mathrm{lfp}(f) = \mathrm{gfp}(f) = 0$，两原理在同一格上管住不同的候选集。

定理三，镜像对偶定理，本件核心。设 $(L, \leq)$ 与 $(M, \leq)$ 是完全格，$d: L \to M$ 是逆序双射，即对任意 $x, y \in L$，$x \geq y$ 当且仅当 $d(x) \leq d(y)$，$f: L \to L$ 是单调算子。定义 $f$ 的镜像算子 $f^d: M \to M$ 为

$$f^d = d \circ f \circ d^{-1} \quad \text{即} \quad f^d(m) = d(f(d^{-1}(m))) \label{eq:ord011-mirror-operator}$$

其中 $d^{-1}: M \to L$ 是 $d$ 的逆映射，亦为逆序双射。断言三件。

- $f^d$ 是单调算子
- 对任意 $x \in L$，$x$ 是 $f$ 的不动点当且仅当 $d(x)$ 是 $f^d$ 的不动点
- $\mathrm{lfp}(f) = d^{-1}(\mathrm{gfp}(f^d))$ 且 $\mathrm{gfp}(f) = d^{-1}(\mathrm{lfp}(f^d))$

证明。

- 单调性：设 $M$ 中 $m_1 \leq m_2$。由 $d$ 逆序双射，$d^{-1}$ 亦逆序，给 $d^{-1}(m_1) \geq d^{-1}(m_2)$。由 $f$ 单调给 $f(d^{-1}(m_1)) \geq f(d^{-1}(m_2))$。由 $d$ 逆序给 $d(f(d^{-1}(m_1))) \leq d(f(d^{-1}(m_2)))$，即 $f^d(m_1) \leq f^d(m_2)$，$f^d$ 单调
- 不动点传递：$f^d(d(x)) = d(f(d^{-1}(d(x)))) = d(f(x))$，故 $f^d(d(x)) = d(x)$ 当且仅当 $d(f(x)) = d(x)$，由 $d$ 单射消去给 $f(x) = x$
- $\mathrm{lfp}$ 侧：令 $g = \mathrm{gfp}(f^d)$。由 Knaster-Tarski，$g$ 是 $f^d$ 的不动点，$f^d(g) = g$，由不动点传递给 $d^{-1}(g)$ 是 $f$ 的不动点，故 $d^{-1}(g) \geq \mathrm{lfp}(f)$。又任取 $f$ 的前不动点 $p$，$f(p) \leq p$，则 $f^d(d(p)) = d(f(p))$，由 $d$ 逆序给 $d(f(p)) \geq d(p)$，即 $d(p)$ 是 $f^d$ 的后不动点。由 $g$ 为全体 $f^d$ 后不动点之并给 $d(p) \leq g$，施加逆序 $d^{-1}$ 给 $p \geq d^{-1}(g)$，故 $d^{-1}(g)$ 是全体 $f$ 前不动点的下界，$d^{-1}(g) \leq \mathrm{lfp}(f)$。合并两侧得 $\mathrm{lfp}(f) = d^{-1}(g) = d^{-1}(\mathrm{gfp}(f^d))$
- $\mathrm{gfp}$ 侧：对偶论证。令 $\ell = \mathrm{lfp}(f^d)$，由 Knaster-Tarski 给 $f^d(\ell) = \ell$，由不动点传递给 $d^{-1}(\ell)$ 是 $f$ 的不动点，故 $d^{-1}(\ell) \leq \mathrm{gfp}(f)$。任取 $f$ 的后不动点 $q$，$f(q) \geq q$，则 $f^d(d(q)) = d(f(q))$，由 $d$ 逆序给 $d(f(q)) \leq d(q)$，即 $d(q)$ 是 $f^d$ 的前不动点。由 $\ell$ 为全体 $f^d$ 前不动点之交给 $\ell \leq d(q)$，施加逆序 $d^{-1}$ 给 $d^{-1}(\ell) \geq q$，故 $d^{-1}(\ell)$ 是全体 $f$ 后不动点的上界，$\mathrm{gfp}(f) \leq d^{-1}(\ell)$。合并两侧得 $\mathrm{gfp}(f) = d^{-1}(\ell) = d^{-1}(\mathrm{lfp}(f^d))$

共归纳实例。设带标记转移图的状态集为 $S$，$F$ 是 $\mathcal{P}(S \times S)$ 上的关系集算子，对 $R \subseteq S \times S$，

$$F(R) = \{(s, t) \in S \times S \mid \forall a,\ s \xrightarrow{a} s' \implies \exists t',\ t \xrightarrow{a} t' \wedge (s', t') \in R\} \label{eq:ord011-bisim-operator}$$

即满足所有转移保持的转移对关系，双模拟即 $F$ 的最大不动点 $\mathrm{gfp}(F)$。共归纳原理给验证法则：要证 $R$ 是双模拟，证 $R$ 是 $F$ 的后不动点即可，即逐对核验转移保持条件。本条不展开过程代数细节，$F$ 仅作关系集算子示例。

知止衔接，可及元概念。登记本条与递归终止概念的衔接，不展开。设 $L = \mathcal{P}(S)$ 为幂集格，$f: L \to L$ 是单调算子，定义升迭代 $f^0(\emptyset) = \emptyset$，$f^{n+1}(\emptyset) = f(f^n(\emptyset))$，由 $f$ 单调与 $\emptyset$ 为最小元给 $f^n(\emptyset) \subseteq f^{n+1}(\emptyset)$。称 $s \in S$ 是 $f$-可及元，当且仅当存在自然数 $n$ 使 $s \in f^n(\emptyset)$，即 $s$ 在有限轮内进入自空集出发的升迭代链。可及元集是 $\mathrm{lfp}(f)$ 的子集，在 $S$ 有限假设下降迭代至多 $|S|$ 轮即达不动点，该不动点即 $\mathrm{lfp}(f)$，可及元集与 $\mathrm{lfp}(f)$ 重合。可及元集成员判定半可判定，向上枚举轮次即决归属，是 $\mathrm{lfp}(f)$ 的可判定侧。支撑依赖关系的良基性，即不存在自某元出发的无限严格降链，是知止的数学形态，工程侧见工程注意事项。本条只登记关系。

## 公理条件 {#axioms}

本条定理的成立依赖以下公理条件。

- 完全格：$(L, \leq)$ 与 $(M, \leq)$ 是完全格，见公式 $\ref{eq:ord002-complete-lattice}$，任意子集有上下确界
- 单调算子：$f: L \to L$ 满足 $x \leq y$ 蕴含 $f(x) \leq f(y)$，见公式 $\ref{eq:ord004-monotone}$；$f^d: M \to M$ 的单调性由镜像对偶定理第一条证得
- 逆序双射：$d: L \to M$ 满足 $x \geq y$ 当且仅当 $d(x) \leq d(y)$，是双射，$d^{-1}: M \to L$ 亦为逆序双射
- 不动点存在性：Knaster-Tarski 定理，见 ORD-003，保 $\mathrm{lfp}(f)$ 与 $\mathrm{gfp}(f)$ 存在，$\mathrm{lfp}$ 为全体前不动点之交，$\mathrm{gfp}$ 为全体后不动点之并

镜像对偶定理的适用边界。逆序双射 $d$ 是结构同构，须显式给出并核验双射性与逆序性两性质，$d$ 平凡即恒等映射时 $M = L$，两方向退化为同一检验，镜像失效。

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Tarski 1955 Knaster-Tarski 定理，共归纳与最大不动点，双模拟与进程代数，可及元与良基性
- 哲学命题：EPI-12 复归元概念，单向流衍结构弱点与复归镜像检验
- 形式化：单向推衍即归纳侧，每步从前步来是逐边可检的局部性质，生成集即 $\mathrm{lfp}$，即从锚出发能推出的全体；复归检验即共归纳侧，命题是否真对应是否在所有反向展开中存活，即后不动点包含关系，是全局性质。原文「不替代不反驳是镜像」的数学形态即镜像对偶定理，镜像 $d$ 给出两链不动点集的一一对应，两原理独立即两链检验维度不可相互替代；终止于递归终止即知止，对应可及元与良基性条件。本条只给检测的数学形态，不立治理结论

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 判据链的镜像检验，正向检验与镜像检验的同构审计
- 形式化：facet 的判定链配为完全格上的升迭代，判定集为 $\mathrm{lfp}$ 即规则包可推出的全体；镜像对偶定理给正向不动点与镜像不动点的一一对应，镜像 $d$ 为正向判定空间与反向验证空间间的显式逆序双射；$d$ 平凡即恒等时两方向退化同一检验，镜像失效
- 借鉴方向：共归纳验证的有界展开近似，$k$ 轮展开通过是后不动点检验的有限近似，通过不蕴含真共归纳，须如实标注近似性与轮数
- 边界：判定格的具体配法、gate 算子的具体形式、镜像 $d$ 的具体构造由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- ORD-003 Knaster-Tarski 不动点：本条承其 $\mathrm{gfp}$ 与 $\mathrm{lfp}$ 定义，见公式 $\ref{eq:ord003-monotone}$，深化为归纳与共归纳两原理及镜像对偶定理
- ORD-007 推衍链的健全性与镜像检验：ORD-007 给正向链局部对全局的健全性拆分与镜像缩回的操作判据，见公式 $\ref{eq:ord007-anchored-chain}$，本条给共归纳刻画与顺序对偶镜像定理，一为性质一为结构，两者互补不重叠
- ORD-004 monotone operator 与 ORD-002 完全格：本条全部结构建立在其上，见公式 $\ref{eq:ord004-monotone}$ 与 $\ref{eq:ord002-complete-lattice}$
- ORD-012 同波簇件：倒推终止的良基性条件与本条可及元段同源，两者共享「不存在无限严格降链」的良基结构

## 历史脉络 {#history}

- 1955 年 Tarski 在 Pacific J. Math. 证明 Knaster-Tarski 不动点定理，确立完全格上单调算子不动点的格结构
- 1970s 起 Scott 域论与共归纳语义，最大不动点作为最大解的承载，程序语义中的 coinductive 判定
- 1981 年 Park 在 On bisimulation and simulations 中给双模拟，关系集算子的最大不动点
- 1990s Barr 的终煤代数 (terminal coalgebra) 与类型论共归纳，共归纳的范畴论形态
- 可及元与良基性概念承 Kleene 递归论与集合论良基关系传统

## 工程注意事项 {#engineering-notes}

应用镜像对偶定理与共归纳验证时须核验以下事项。

1. 共归纳验证的有界近似：机器侧共归纳验证需有界展开近似，$k$ 轮展开通过是后不动点检验的有限近似，通过不蕴含真共归纳，报告须标注轮数与近似性，不得表述为共归纳成立
2. 镜像同构审计：镜像对偶定理可审计镜像检验与正向检验是否同构；$d$ 平凡即恒等映射时两方向退化同一检验，镜像失效；$d$ 非平凡须显式给出，并核验逆序双射三性质即双射性、逆序性、逆存在
3. 不动点结构适用性：$\mathrm{lfp}$ 与 $\mathrm{gfp}$ 存在性依赖完全格加单调算子条件，见 ORD-003；单调性破坏则不动点结构不保证
4. 可及元判定边界：可及元侧成员判定半可判定，向上枚举轮次即决归属；无限步集情形下该枚举不保证终止，须降级为有界近似并标注

## 参考文献 {#references}

- Tarski, A. (1955). A lattice-theoretical fixpoint theorem and its applications. Pacific J. Math., 5(2), 285-309
- Park, D.M.R. (1981). On bisimulation and simulations. In: Information Processing 81, North-Holland
- Cousot, P. & Cousot, R. (1979). Abstract interpretation: a unified lattice model for static analysis of programs by construction or approximation of fixpoints. POPL
- Barr, M. (1993). Terminal coalgebras in programming. Theoretical Computer Science, 111(1), 129-158
- Davey, B.A. & Priestley, H.A. (2002). Introduction to Lattices and Order, 2nd ed. Cambridge University Press
- Wikipedia "Knaster-Tarski theorem" 条目
- Wikipedia "Coinduction" 条目
