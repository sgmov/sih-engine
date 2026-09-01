---
entry: ENTRY-ORD-006.md
agent: 复归段补强波簇B
proposed_id: ORD-006
subrepo: order
id_reason: 承 order 子仓现有序列 001 至 005，006 为下一空号，闭包算子与完全格及 Knaster-Tarski 不动点同域
anchors:
  - pro: EPI-14
    source: sih-philosophy/emanation/epistrophe/14-on-reducibility.md:29
    quote: 判定标记：可归约 / 跳跃引入。
  - pro: EPI-14
    source: sih-philosophy/emanation/epistrophe/14-on-reducibility.md:117
    quote: 无跳跃引入。Daoist 范畴作为外部锚定统一在 00- + 01- 的命名传统选择中，不构成外部预设的跳跃。
selfcheck:
  - 锚 1 至 2 逐字节命中：grep -nF 复核 14:29 与 14:117 均命中，两引文零全角括号零破折号
  - 三查：全文 grep 全角括号中文内容零处、破折号字符零处、围栏代码块零处
  - 八节在场且顺序固定，首行 H1 为 # ORD-006 闭包算子与后果算子，首个二级标题为 定义
  - 数学主张：闭包三公理与独立性反例、Cn 为闭包算子逐条证明、可归约判定为有限成员判定，条件与证明步骤完整；两引文均不含全角括号与破折号，未改引文
---

# ORD-006 闭包算子与后果算子

状态：草稿，复归段补强波簇B 产出，哲学到工程桥梁条目。

## 定义 {#definition}

闭包算子 (closure operator) 是幂集格上同时满足扩张性、单调性、幂等性的算子，刻画封闭集合的生成结构。

设 $S$ 是集合，$\mathcal{P}(S)$ 是其幂集，按包含序 $\subseteq$ 构成完全格 (ORD-002)。映射 $\mathrm{cl}: \mathcal{P}(S) \to \mathcal{P}(S)$ 称为闭包算子，当且仅当对任意 $X, Y \subseteq S$ 满足三公理：

$$\text{扩张性: } X \subseteq \mathrm{cl}(X) \qquad \text{单调性: } X \subseteq Y \implies \mathrm{cl}(X) \subseteq \mathrm{cl}(Y) \qquad \text{幂等性: } \mathrm{cl}(\mathrm{cl}(X)) = \mathrm{cl}(X) \label{eq:ord006-closure-axioms}$$

满足 $\mathrm{cl}(X) = X$ 的子集 $X$ 称为闭集，闭集全体构成闭包算子的不动点集。

经典实例。

- 拓扑闭包：$S$ 配拓扑时 $\mathrm{cl}$ 取拓扑闭包，闭集即闭集族；凸包：$S$ 是赋范空间，$\mathrm{cl}(C) = \mathrm{conv}(C)$，闭集即凸集
- 子空间生成：$V$ 是域上的向量空间，$W \subseteq V$，$\mathrm{cl}(W) = \mathrm{span}(W)$，闭集即子空间；逻辑后果：$\mathrm{cl}$ 取语言上的语义后果算子 $\mathrm{Cn}$，见下，闭集即闭包理论

三公理独立性。单调性不能由扩张性与幂等性推出。取 $S = \{a, b, c\}$，令 $\mathrm{cl}(\emptyset) = \{c\}$，其余非空 $X$ 令 $\mathrm{cl}(X) = X$。则扩张性逐点成立，幂等性逐点成立，而 $\emptyset \subseteq \{a\}$ 时 $\mathrm{cl}(\emptyset) = \{c\} \not\subseteq \{a\} = \mathrm{cl}(\{a\})$，单调性破坏。故三公理须并置为定义，不可删减。

后果算子 (consequence operator) 是闭包算子在语言论的实例。设 $\mathcal{L}$ 是形式语言，$\Gamma \subseteq \mathcal{L}$。语义后果算子定义为

$$\mathrm{Cn}(\Gamma) = \{\varphi \in \mathcal{L} \mid \Gamma \models \varphi\} \label{eq:ord006-cn}$$

即被 $\Gamma$ 语义蕴含的全体句子，$\Gamma \models \varphi$ 表 $\varphi$ 在 $\Gamma$ 的每个模型中为真。

定理。$\mathrm{Cn}$ 是 $\mathcal{P}(\mathcal{L})$ 上的闭包算子，即满足三公理 $\ref{eq:ord006-closure-axioms}$。

证明。

- 扩张性：对任意 $\gamma \in \Gamma$，$\Gamma$ 的任一模型满足 $\gamma$，故 $\Gamma \models \gamma$，$\gamma \in \mathrm{Cn}(\Gamma)$，得 $\Gamma \subseteq \mathrm{Cn}(\Gamma)$
- 单调性：设 $\Gamma \subseteq \Delta$ 且 $\varphi \in \mathrm{Cn}(\Gamma)$，即 $\Gamma \models \varphi$。$\Delta$ 的任一模型是 $\Gamma$ 的模型，故亦满足 $\varphi$，得 $\Delta \models \varphi$，$\varphi \in \mathrm{Cn}(\Delta)$
- 幂等性：$\mathrm{Cn}(\Gamma) \subseteq \mathrm{Cn}(\mathrm{Cn}(\Gamma))$ 由扩张性。反向，设 $\varphi \in \mathrm{Cn}(\mathrm{Cn}(\Gamma))$，即 $\mathrm{Cn}(\Gamma) \models \varphi$。取 $\Gamma$ 的任一模型 $M$，对任意 $\psi \in \mathrm{Cn}(\Gamma)$ 有 $\Gamma \models \psi$ 故 $M \models \psi$，即 $M \models \mathrm{Cn}(\Gamma)$。用引理，若 $\Sigma \models \varphi$ 且 $M \models \Sigma$ 则 $M \models \varphi$，由语义蕴含定义取逆否立得，故 $M \models \varphi$，得 $\Gamma \models \varphi$，$\varphi \in \mathrm{Cn}(\Gamma)$

可归约判定，即 EPI-14 的数学形态。设 $p_1, \dots, p_n$ 是命题链，$A \subseteq \mathcal{L}$ 是锚集，预锚定的唯一允许外部输入，如外部锚定传统。记前序步骤集 $P_i = \{p_1, \dots, p_{i-1}\}$。

- 第 $i$ 步可归约，当且仅当 $p_i \in \mathrm{Cn}(P_i \cup A)$
- 链通过 Reducibility 检测，当且仅当对每个 $i = 1, \dots, n$，$p_i \in \mathrm{Cn}(P_i \cup A)$
- 跳跃引入，即存在 $i$ 使 $p_i \notin \mathrm{Cn}(P_i \cup A)$，即 $p_i$ 依赖 $A$ 与前序步骤之外的句子

锚集规则。加入 $A$ 不构成跳跃，$A$ 是唯一允许的外部输入，承 EPI-14 外部锚定统一于命名传统选择；依赖 $A$ 之外者构成跳跃。可判定性：$A$ 有限且 $\mathrm{Cn}$ 成员判定可机械执行时，整链检测为至多 $n$ 次成员判定，有限步完成。

## 公理条件 {#axioms}

闭包算子与可归约判定的成立依赖以下公理条件。

- 幂集格：$\mathcal{P}(S)$ 按包含序为完全格，$\ref{eq:ord002-complete-lattice}$，三公理在该格上陈述
- 语义良定义：$\Gamma \models \varphi$ 是良定义二元关系，$\mathrm{Cn}$ 的定义 $\ref{eq:ord006-cn}$ 依赖模型论的模型与满足关系
- 锚集固定：$A$ 在检测前预给定，检测中不变；$A$ 的选定本身不受可归约检测约束，是检测的输入

可归约判定与可推性判定的分界。可归约锚定语义后承 $\mathrm{Cn}$，可推性锚定特定语法体系的语法推衍。在健全且完备的演算中两者重合：健全性给语法推衍蕴含语义蕴含，完备性给语义蕴含蕴含语法推衍，故语法推衍与语义后承互逆。如实标注：重合依赖完备性；不完备演算中语法推衍严格小于语义后承，此时两标记可分，可归约判定仍锚语义侧，不依赖公理化选择。

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Tarski 后果算子与闭包算子公理传统，Kuratowski 闭包公理，Knaster-Tarski 不动点 (ORD-003)
- 哲学命题：EPI-14 可归约性检测，向上收拢归一之潜能，验证每步核心命题可归约回前步而无外部跳跃
- 形式化：EPI-14 的每步可归约到前步，即每步属于前序步骤与锚集的后果闭包 $\mathrm{Cn}(P_i \cup A)$，见定义节判定；跳跃即闭包外元素，存在 $i$ 使 $p_i \notin \mathrm{Cn}(P_i \cup A)$。锚集 $A$ 承外部锚定统一于命名传统选择，是唯一允许的外部输入，加入 $A$ 不构成跳跃。本条只给检测的数学形态与可判定性，不立治理结论，判定标记可归约与跳跃引入由检测执行。

## 在 facet 的应用 {#facet-application}

- 应用场景：交叉审阅中确认发现集的封闭性与新增发现的归约判定
- 形式化：facet 聚合后的确认发现集配为 $\mathcal{L}$ 的子集，新增发现 $p$ 到达时检查 $p \in \mathrm{Cn}(\text{已确认发现} \cup A)$；在闭包内即既有发现的逻辑后果，归约通过，在闭包外即引入新预设，标记跳跃
- 借鉴方向：闭包算子幂等性保证重复检测同一发现集得同一闭包，检测可重复；单调性保证发现集扩大则闭包单调扩大
- 边界：$\mathcal{L}$ 的具体语义、$\mathrm{Cn}$ 的可判定实现、锚集 $A$ 的选定由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- ORD-002 完全格：$\mathcal{P}(S)$ 按包含序是完全格，闭包算子定义于其上，见公式 $\ref{eq:ord002-complete-lattice}$
- ORD-003 Knaster-Tarski 不动点：闭包算子是单调算子，其不动点集即闭集全体构成完全格，Knaster-Tarski 给出该不动点集的格结构
- ORD-004 monotone operator：闭包算子是单调算子的特例，额外要求扩张性与幂等性
- 拓扑侧：闭包算子不动点即闭集生成 $S$ 上的拓扑，闭包公理与 Kuratowski 闭包公理等价，拓扑视角见 topology 子仓
- 逻辑侧：$\mathrm{Cn}$ 的闭集即闭包理论，与逻辑理论 (theory) 一致，逻辑理论全体按包含序构成完全格

## 历史脉络 {#history}

- 1920s 至 1930s Kuratowski 系统化闭包公理，拓扑闭包算子公理化
- 1936 年 Tarski 在论形式体系的模型中引入语义后果算子与模型论框架
- 1955 年 Tarski 在 lattice-theoretical fixpoint theorem 中将闭包算子与不动点定理统一，闭包算子不动点是完全格
- 1970s 后闭包算子在抽象解释 (abstract interpretation) 中成为语义近似的核心工具，Kleene 构造沿序数迭代

## 工程注意事项 {#engineering-notes}

应用闭包算子与可归约判定时需核验四件事。

1. 语义后承是否机械可判：$\mathrm{Cn}$ 的成员判定 $p_i \in \mathrm{Cn}(P_i \cup A)$ 须可执行；一阶逻辑的 $\mathrm{Cn}$ 仅递归可枚举不可判定，检测须降为语法推衍或限制语言片段，如实标注可判定性边界
2. 锚集是否预给定且固定：$A$ 在检测中变更会使判定失去参照，须登记 $A$ 的内容与选定依据
3. 单调性是否成立：若所用算子不满足单调性，闭包扩大会非单调，重复检测结果不稳定
4. 幂等性是否成立：不幂等则检测依赖执行次数，须验证 $\mathrm{cl}(\mathrm{cl}(X)) = \mathrm{cl}(X)$

工程局限。一阶逻辑语义后承不可判定，Gödel 不完备性给出不完备演算中语法推衍严格小于语义后承的实例，可归约判定在一般语言上须退化为语法侧或有限近似。

## 参考文献 {#references}

- Tarski, A. (1936). Sur les systèmes de formules logiques et certains aspects de la définition et de la théorie des ensembles. Colloquium Mathematicum, 1, 7-13
- Tarski, A. (1955). A lattice-theoretical fixpoint theorem and its applications. Pacific J. Math., 5(2), 285-309
- Kuratowski, K. (1933). Topologie, vol. 1. Warsaw
- Mendelson, E. (1997). Introduction to Mathematical Logic, 4th ed. CRC Press
- Cousot, P. & Cousot, R. (1979). Abstract interpretation: a unified lattice model for static analysis of programs by construction or approximation of fixpoints. POPL
- Davey, B.A. & Priestley, H.A. (2002). Introduction to Lattices and Order, 2nd ed. Cambridge University Press
