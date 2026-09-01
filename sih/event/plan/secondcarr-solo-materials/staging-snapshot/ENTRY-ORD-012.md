---
entry: ENTRY-ORD-012.md
agent: 复归段补强波簇A
proposed_id: ORD-012
subrepo: order
id_reason: order 子仓 INDEX 现行已建至 ORD-010，ORD-011 由本波簇 A 同簇预留，本件取下一空号 ORD-012
anchors:
  - pro: EPI-13
    source: sih-philosophy/emanation/epistrophe/13-on-grounding.md:7
    quote: "从 11- 倒推至 00-，检查每步核心命题是否被后续步骤引用或修正。"
  - pro: EPI-13
    source: sih-philosophy/emanation/epistrophe/13-on-grounding.md:85
    quote: "12 步全部通过落地检测。无悬空命题。"
selfcheck:
  - "锚二条 grep -nF 复核命中，行号 7 与 85"
  - "三查：全文全角括号中文内容零处、破折号字符零处、围栏代码块零处"
  - "八节在场且顺序固定，首行 H1 形对，首节为定义"
  - "数学主张：终止定理给沿引用关系下降遍历的终止性等价于引用关系良基；有限判定定理给有限集上良基等价于传递闭包无自环即无引用环；死区定理给死区为 F 的最大不动点，F 为不含 T 且出邻居全含于 X 的节点集，经补集共轭与落地集最小不动点相对偶证得；任务书原 F 不含 x 不属于 T 条件，本件补上并在 T 与死区不相交时退化回任务书形态，差异已记未决"
---

# ORD-012 良基关系与倒推终止

状态：草稿，复归段补强波簇A 产出，哲学到工程桥梁条目。

## 定义 {#definition}

良基关系 (Well-founded Relation) 与倒推终止 (Backward Termination) 给 EPI-13 倒推检查程序的序论终止担保，以及全局死区的不动点刻画。

设定：步集 $Steps$ 有限，引用关系为 $c \succ p$，表示步 $c$ 引用或修正步 $p$。有向图 $G$ 以 $Steps$ 为节点集，边由引用者指向被引用者，即边 $c \to p$ 当且仅当 $c \succ p$。对 $s \in Steps$，出邻居集 $Out(s) = \{p \in Steps \mid s \succ p\}$ 是 $s$ 引用或修正的步骤集，即 $\succ$-后继。

良基定义。关系 $\succ$ 良基，当且仅当不存在无限严格降链

$$x_0 \succ x_1 \succ x_2 \succ \cdots \label{eq:ord012-well-founded}$$

定理一，终止定理。设遍历程序自任一步出发，每步从当前节点移到其引用的任意一步，即由 $c$ 移到 $p$ 当 $c \succ p$。则该遍历必终止，即全体执行路径有限，当且仅当 $\succ$ 良基。

证明。

- 方向一，非良基给不终止：若 $\succ$ 非良基，存在无限链 $x_0 \succ x_1 \succ x_2 \succ \cdots$，遍历自 $x_0$ 出发沿之逐移，$x_0 \to x_1 \to x_2 \to \cdots$，为无限执行路径，遍历不终止
- 方向二，良基给终止：若 $\succ$ 良基，遍历的任一执行是逐步沿 $\succ$ 移动的序列，执行本身即 $\succ$-降链；若执行无限，则为无限严格降链，与良基矛盾。故全体执行有限，遍历终止

定理二，有限判定定理。有限集上，$\succ$ 良基当且仅当 $\succ$-传递闭包 $\succ^+$ 无自环，即无引用环。

证明。

- 无自环给良基：设传递闭包有自环，即某 $x$ 使 $x \succ^+ x$，则存在引用环 $x = y_0 \succ y_1 \succ \cdots \succ y_k = x$，$k \geq 1$，重复该环给无限链 $y_0 \succ y_1 \succ \cdots \succ y_k \succ y_0 \succ y_1 \succ \cdots$，$\succ$ 非良基；逆否，$\succ$ 良基则传递闭包无自环
- 良基给无自环：设 $\succ$ 非良基，存在无限链 $x_0 \succ x_1 \succ x_2 \succ \cdots$，在有限集 $Steps$ 上由鸽笼原理某步重复，$x_i = x_j$ 且 $i < j$，则 $x_i \succ x_{i+1} \succ \cdots \succ x_j = x_i$ 为引用环，传递闭包有自环；逆否，传递闭包无自环则 $\succ$ 良基

König 引理，登记不证。每个无限有限分叉树有无限路径。用途：无限步集情形下，将每步展开为节点、其引用步骤展开为子，成引用展开树，树有限分叉当且仅当每步引用有限多步；非良基等价于存在无限 $\succ$-降链，即引用展开树有无限路径，有限分叉假设下由 König 引理，有无限路径等价于展开树无限。本条只登记引理与用途。

倒推程序建模。EPI-13 倒推检查程序对每步 $s$ 检查其出邻居，即 $s$ 引用或修正的步骤，$\succ$-后继，非空，即 $Out(s) \neq \emptyset$，为局部单轮检查。全局落地性质：$s$ 落地当且仅当存在从 $s$ 出发到用终点集 $T$ 的有向路，$T$ 为预设，含自明豁免步与外部引用步。死区 $D$ 定义为无有向路到 $T$ 的节点集。

定理三，死区定理。设 $F: \mathcal{P}(Steps) \to \mathcal{P}(Steps)$ 为

$$F(X) = \{x \in Steps \mid x \notin T \wedge Out(x) \subseteq X\} \label{eq:ord012-deadzone-operator}$$

即出邻居全含于 $X$ 且不在 $T$ 中的节点集。则 $D$ 是 $F$ 的最大不动点。

证明。令 $R = Steps \setminus D = \{s \mid \text{存在从 } s \text{ 到 } T \text{ 的有向路}\}$ 为落地集。定义 $G: \mathcal{P}(Steps) \to \mathcal{P}(Steps)$ 为 $G(X) = T \cup \{x \mid Out(x) \cap X \neq \emptyset\}$。则 $R$ 是 $G$ 的最小不动点。

- $R$ 是 $G$ 的不动点：取 $x \in R$，即 $x$ 可达 $T$。若 $x \in T$ 则 $x \in G(R)$；否则 $x$ 有长度至少为一的路 $x \to v_1 \to \cdots \to v_k \in T$，$v_1 \in Out(x)$ 且 $v_1$ 可达 $T$，$v_1 \in R$，$Out(x) \cap R \neq \emptyset$，$x \in G(R)$，故 $R \subseteq G(R)$。反向，取 $x \in G(R)$，则 $x \in T \subseteq R$，或 $Out(x) \cap R \neq \emptyset$ 使某出邻居可达 $T$ 从而 $x$ 可达 $T$，$x \in R$，故 $G(R) \subseteq R$。合并 $R = G(R)$
- $R$ 是最小不动点：设 $X$ 是 $G$ 的任一不动点，$X = G(X)$。由路长归纳证 $R \subseteq X$。基步，$T \subseteq G(X) = X$。归纳步，设所有至多 $k$ 步可达 $T$ 的节点在 $X$ 中，取恰 $k+1$ 步可达 $T$ 的 $x$，$x \to v_1 \to \cdots \to T$，$v_1$ 至多 $k$ 步可达 $T$ 故 $v_1 \in X$，$Out(x) \cap X \neq \emptyset$，$x \in G(X) = X$。故 $R \subseteq X$，$R = \mathrm{lfp}(G)$

$F$ 与 $G$ 为补集共轭。对任意 $X$，$\overline{G(\overline{X})} = \{y \mid y \notin T \wedge Out(y) \cap \overline{X} = \emptyset\} = \{y \mid y \notin T \wedge Out(y) \subseteq X\} = F(X)$，即 $F(X) = \overline{G(\overline{X})}$。由 Knaster-Tarski 的最小不动点与最大不动点对偶，$\mathrm{gfp}(F) = \overline{\mathrm{lfp}(G)} = \overline{R} = D$。故 $D$ 是 $F$ 的最大不动点。

有限迭代。$F$ 自全集 $Steps$ 起逐轮收缩，令 $X_0 = Steps$，$X_{n+1} = F(X_n)$，则 $X_1 = F(Steps) = Steps \setminus T$，且 $X_{n+1} \subseteq X_n$，$F$ 单调且 $X_{n+1} \subseteq X_n$ 给 $F(X_{n+1}) \subseteq F(X_n)$。降链在有限集上有限轮内稳定，轮数上限为 $Steps$ 的基数，稳定值即最大不动点 $D$。出度零检测即局部单轮近似，检出无出邻居的节点，是死区的子集；全局死区为全迭代不动点 $D$，补上出邻居非空但全部路径通向死区的节点。

反例。设 $Steps = \{A, B\}$，$A$ 引用 $B$，即 $A \succ B$，边 $A \to B$，$B$ 无出边，$Out(B) = \emptyset$，$Out(A) = \{B\}$。$T$ 不含 $B$，亦不含 $A$。局部单轮检查：$B$ 出度零，被检出；$A$ 出度非零，漏检。全局死区：$B$ 无出边且 $B \notin T$，无有向路到 $T$，$B \in D$；$A$ 唯一有向路为 $A \to B$，止于 $B \notin T$，无有向路到 $T$，$A \in D$。死区为 $A$ 与 $B$ 两元集，被全迭代检出。

两维判定。落地检测通过条件即死区为空。引用环存在时倒推程序不终止，须先查无环；工程侧有限集无环检查复杂度为边数乘点数阶，即 $O(|E| \cdot |V|)$。

## 公理条件 {#axioms}

本条定理的成立依赖以下公理条件。

- 有限步集：$Steps$ 有限，基数 $|Steps|$；无限情形见 König 引理登记的边界
- 引用关系良定义：$c \succ p$ 是良定义二元关系，步 $c$ 引用或修正步 $p$，边可机械抽取，显式文本或登记名搜索
- 出邻居集：$Out(s) = \{p \in Steps \mid s \succ p\}$ 是 $s$ 引用或修正的步骤集，$\succ$-后继
- 用终点集预设：$T \subseteq Steps$ 在检测前预给定且检测中不变，含自明豁免步与外部引用步；$T$ 的选定是检测输入，不受落地检测约束
- 良基或无环：$\succ$ 良基，有限集上等价于传递闭包无自环，即无引用环

适用边界。本条全部判定以 $Steps$ 有限、边可机械抽取为前提。无限步集情形下良基不再等价于无环，须依 König 引理登记与有限分叉假设，判定随之下调。

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：良基归纳与结构递归，König 1927，Dershowitz 与 Jouannaud 1990 改写终止，有向图可达性
- 哲学命题：EPI-13 落地检测与去语境化检验
- 形式化：原文「从 11- 倒推至 00-」是良基序上的逆向遍历，程序终止性由引用关系良基性担保，有限链情形即无环检查；「真命题必有用」即节点到用终点的路径存在性，是全局可达性质；「无悬空命题」即死区为空；去语境化维度由 ORD-008 承载，本条只承倒推程序侧。与 ORD-008 的分界，ORD-008 给可达性图视角与出度零判定，本条给程序良基性视角与全局死区不动点，性质对程序

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 报告倒推，切面发现的落地检查
- 形式化：每条切面发现须被后续裁定步引用即落地；发现引用图有限故先查无环，无环则倒推遍历终止可机械执行；悬空发现即死区成员，局部出度检查会漏检只引用其他悬空发现的发现，出度非零但全部路径通向死区，全局死区迭代补上
- 借鉴方向：死区迭代轮数有限可机械执行，输出死区集合作为告警材料
- 边界：发现 schema、引用边的具体抽取、用终点集 $T$ 的具体内容均由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- ORD-008 引用图可达与孤悬判定：ORD-008 给可达性图视角与出度零孤悬判定，见公式 $\ref{eq:ord008-orphan}$，本条给程序良基性视角与全局死区不动点，性质对程序，两者互补
- ORD-011 共归纳与镜像对偶，同波簇件：倒推终止的良基性条件与 ORD-011 可及元段同源，两者共享「不存在无限严格降链」的良基结构
- ORD-005 链与反链：无限引用链是引用序中的链，良基即不存在无限降链，见公式 $\ref{eq:ord005-chain}$
- ORD-002 完全格：死区定理的 $F$ 是幂集格上的单调算子，见公式 $\ref{eq:ord002-complete-lattice}$，其最大不动点存在由 Knaster-Tarski 定理（ORD-003）担保

## 历史脉络 {#history}

- 结构归纳与良基关系传统，良基归纳作为终止性证明的基础工具
- 1927 年 König 引理，无限有限分叉树有无限路径
- 1990 年 Dershowitz 与 Jouannaud 在改写系统终止性中系统化良基序作为终止性证明核心
- 终止性检查的工程传统，无环与良基作为终止担保的两种机械形态

## 工程注意事项 {#engineering-notes}

执行倒推检查须完成以下核验动作。

1. 先查无环再倒推：引用环使递归倒推不终止，工程侧先跑无环检查再倒推，或限深并如实标注截断；有限集无环检查复杂度为边数乘点数阶
2. 死区迭代可机械执行：死区迭代轮数有限，轮数上限为 $Steps$ 基数，输出死区集合作为告警材料
3. $T$ 版本化：$T$ 的设定，即自明豁免与外部引用清单，须版本化，$T$ 变更改变死区判定，报告须记录 $T$ 版本
4. 局部对全局：局部出度检查会漏检只引用其他悬空发现的发现，须全局死区迭代补上，不得只出局部检查结果而不标注

## 参考文献 {#references}

- Dershowitz, N. & Jouannaud, J.-P. (1990). Rewrite systems. In: Handbook of Theoretical Computer Science, Vol. B, Elsevier
- West, D.B. (2001). Introduction to Graph Theory, 2nd ed. Prentice Hall
- König, D. (1927). Über eine endliche Menge von unendlichen Ketten
- Wikipedia "Well-founded relation" 条目
- Wikipedia "König's lemma" 条目
