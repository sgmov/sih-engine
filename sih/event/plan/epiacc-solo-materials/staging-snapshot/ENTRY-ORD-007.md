---
entry: 推衍链的健全性与镜像检验
agent: 复归段补强波簇A
proposed_id: ORD-007
subrepo: order
id_reason: order 子仓已占 ORD-001 至 ORD-005，ORD-006 已预派 EPI-14 归约性承载，本件承闭包算子与链推理语义顺次取 ORD-007
anchors:
  - pro: EPI-12
    source: sih-philosophy/emanation/epistrophe/12-on-epistrophe.md:9
    quote: "但单向流衍有结构弱点：它只能证明「每步从前步来」，不能证明「每步真的」。"
  - pro: EPI-12
    source: sih-philosophy/emanation/epistrophe/12-on-epistrophe.md:37
    quote: "Epistrophe 是反推证立链。它从命题出发，向下追索其有效性与独立性，回答「这个命题是否真」。"
selfcheck:
  锚引文逐字节复核: 两条锚经 grep -nF 复核，12-on-epistrophe.md 第 9 行与第 37 行逐字节命中，引文为该行子串且不含行内其他内容
  三查: 全角括号中文零处、破折号零处、围栏代码块零处
  结构: 首行 H1 为 ORD-007 推衍链的健全性与镜像检验，首个二级标题为定义，八节顺序在场
  数学主张自查: 健全性定理两条件写全，归纳基步与归纳步逐步写出，最小反例为两步链锚假各步均在 Cn 内终点假，Cn 可判定性边界按可判定与半可判定如实分层标注
  引文洁净度: 两条引文均不含全角括号与破折号
---

# ORD-007 推衍链的健全性与镜像检验

状态：草稿，拟派号 ORD-007，哲学到工程桥梁条目，未入索引。

## 定义 {#definition}

带锚推衍链 (Anchored Derivation Chain) 是语义后果算子意义下的命题序列，形式化单向流衍链的「每步从前步来」。

设 $L$ 是语言，$\mathrm{Cn}$ 是其语义后果算子：对 $L$ 的任一子集 $\Gamma$，

$$\mathrm{Cn}(\Gamma) = \{\varphi \in L \mid \Gamma \models \varphi\} \label{eq:ord007-con}$$

即被 $\Gamma$ 语义蕴含的全体句子。设 $A \subseteq L$ 是锚集，$p_1, \ldots, p_n$ 是 $L$ 中的命题序列。称 $(A, p_1, \ldots, p_n)$ 是带锚推衍链，若对每个 $i = 1, \ldots, n$，

$$p_i \in \mathrm{Cn}(\{p_1, \ldots, p_{i-1}\} \cup A) \label{eq:ord007-anchored-chain}$$

该条件即「每步从前步来」的严格形态：每步是前序步骤与锚集的语义后承，$i = 1$ 时前序步骤集为空，链首由锚集单独支撑。

健全性 (Soundness)。设 $\mathrm{TRUTH}$ 是 $L$ 中全体真句子的集合。带锚推衍链 $(A, p_1, \ldots, p_n)$ 健全，当且仅当对每个 $i = 1, \ldots, n$，$p_i \in \mathrm{TRUTH}$。

健全性定理 (带条件)。设 $(A, p_1, \ldots, p_n)$ 是带锚推衍链，且满足 (一) 锚集与链首前提皆真，即 $A \subseteq \mathrm{TRUTH}$；(二) $\mathrm{Cn}$ 保真，即对任一 $\Gamma \subseteq \mathrm{TRUTH}$，$\mathrm{Cn}(\Gamma) \subseteq \mathrm{TRUTH}$。则对每个 $i = 1, \ldots, n$，$p_i \in \mathrm{TRUTH}$。

证明对 $i$ 归纳。基步 $i = 1$：由链条件 (eq:ord007-anchored-chain) 有 $p_1 \in \mathrm{Cn}(A)$；由条件 (一) $A \subseteq \mathrm{TRUTH}$，由条件 (二) $\mathrm{Cn}(A) \subseteq \mathrm{TRUTH}$，故 $p_1 \in \mathrm{TRUTH}$。归纳步：设 $p_1, \ldots, p_i \in \mathrm{TRUTH}$ 为归纳假设。由链条件 $p_{i+1} \in \mathrm{Cn}(\{p_1, \ldots, p_i\} \cup A)$；由归纳假设与条件 (一) 有 $\{p_1, \ldots, p_i\} \cup A \subseteq \mathrm{TRUTH}$，由条件 (二) 有 $\mathrm{Cn}(\{p_1, \ldots, p_i\} \cup A) \subseteq \mathrm{TRUTH}$，故 $p_{i+1} \in \mathrm{TRUTH}$。归纳完成，链全体步骤皆真。

结构弱点的数学形态。「每步从前步来」，即条件 (eq:ord007-anchored-chain)，是局部性质：逐边独立可检，每条成员关系 $p_i \in \mathrm{Cn}(\{p_1, \ldots, p_{i-1}\} \cup A)$ 是一次独立判定。「每步真的」是全局性质：依赖锚集的真值，正向推衍不检查它。两者不等价。

最小反例 (两步链)。设 $L$ 含命题变元 $a, c$，锚集 $A = \{a\}$，链为 $p_1 = a$，$p_2 = a \vee c$。步骤一：$p_1 \in \mathrm{Cn}(A)$，由 $a \models a$ 与后果算子的包含性成立。步骤二：$p_2 \in \mathrm{Cn}(\{a\})$，由 $a \models a \vee c$ 与单调性成立。取解释 $I_0$ 使 $a$ 与 $c$ 皆假，则 $p_1$ 与 $p_2$ 在 $I_0$ 下皆假。该链每步皆从前步来而终点假，是「假融贯」的最小模型：内部融贯不保证真。

镜像方向 (复归检验的两项)。一、有效性。从末端命题 $p_n$ 逐步向下追溯每步的 $\mathrm{Cn}$ 归属：对 $i = n, n-1, \ldots, 1$ 依次判定 $p_i \in \mathrm{Cn}(\{p_1, \ldots, p_{i-1}\} \cup A)$。正向方向由 $A$ 生成至 $p_n$，镜像方向由 $p_n$ 回溯至 $A$，判定的是同一组成员关系条件，方向相反，即推衍嵌入的缩回。有效性回答「该步是否被合法生成」，仍不回答「该步是否真」：后者的剩余条件正是锚集真值，即健全性条件 (一)。二、独立性。设 $T_i = \{p_1, \ldots, p_n\} \setminus \{p_i\} \cup A$，则 $p_i$ 对链独立，当且仅当 $p_i \notin \mathrm{Cn}(T_i)$，承独立公理标准判据：句子对理论独立当且仅当该理论不语义蕴含该句子。独立性回答「该步是否为其余步骤的冗余后承」，不独立者可能是抽象推导的副产品，应标记待审视。

适用边界。$\mathrm{Cn}$ 归属检验的可判定性依赖语言与理论：对可判定理论，成员判定算法终止且给出正确结果；对不可判定理论，$\mathrm{Cn}(\Gamma)$ 的成员可枚举而不可判定，检验退化为半可判定：可列出推衍，不可列全非推衍，搜索无果不等于非成员。本条目全部判定陈述以链有限、成员判定给出为前提。

## 公理条件 {#axioms}

带锚推衍链与健全性定理依赖以下公理条件。

- 语言与语义：$L$ 配有语义满足关系 $\models$，$\mathrm{Cn}$ 由 $\models$ 定义，见公式 (eq:ord007-con)
- 闭包算子三性质：$\mathrm{Cn}$ 满足包含性 $\Gamma \subseteq \mathrm{Cn}(\Gamma)$、单调性 $\Gamma \subseteq \Delta \implies \mathrm{Cn}(\Gamma) \subseteq \mathrm{Cn}(\Delta)$、幂等性 $\mathrm{Cn}(\mathrm{Cn}(\Gamma)) = \mathrm{Cn}(\Gamma)$，是 $\mathcal{P}(L)$ 上的闭包算子
- 保真条件：$\mathrm{TRUTH}$ 对 $\mathrm{Cn}$ 封闭，即健全性定理条件 (二)；对塔斯基语义，真值在逻辑后果下保真，该条件成立
- 锚集固定：$A$ 在链开始前给定且链中不增锚，锚的变更须开新链
- 有限性：$n$ 为自然数，归纳与镜像回溯均为有限过程
- 可判定性分层：成员判定按语言与理论的可判定性分层，不可判定理论下检验为半可判定，结论表述随之降级

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Tarski 语义后承与健全性定理传统 (Tarski 1933, 1936)，Kuratowski 闭包算子 (Kuratowski 1948)
- 哲学命题：EPI-12 单向流衍链结构弱点与复归镜像检验
- 形式化：EPI-12 断单向流衍「只能证明每步从前步来，不能证明每步真的」。本条目把该弱点拆为局部性质与全局性质之分：条件 (eq:ord007-anchored-chain) 逐边可检，是局部性质；全体步骤属于 $\mathrm{TRUTH}$ 是全局性质。健全性定理表明全局性质成立须锚真且 $\mathrm{Cn}$ 保真，而正向推衍只检查局部性质，不检查锚的真值；最小反例即原文「假融贯」的模型，链内融贯而锚假，终点假。镜像方向即缩回方向：有效性自末端回溯逐边 $\mathrm{Cn}$ 归属，独立性判 $p_i \notin \mathrm{Cn}(T_i)$，两者构成复归检验对单向推衍的机械补全

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 判据链，规则包到聚合 verdict 的判定序列
- 形式化：锚集 $A$ 为规则包与冻结向量集，步骤 $p_1$ 至 $p_n$ 为确定性程序产出的逐项判定，如规则命中、哈希对表、金向量回归；条件 (eq:ord007-anchored-chain) 即每条判定可溯源至规则与材料的检查；健全性条件 (一) 即规则包与向量集自身正确，条件 (二) 即确定性程序只从给定材料与规则产出判定。镜像有效性即自最终 verdict 回溯每条判定的溯源；独立性用于识别某条判定是否为其余判定的冗余后承
- 借鉴方向：终点判定不得仅凭「每步从前步来」全体通过即报通过，须另查锚集真值，即规则包对金向量的回归
- 边界：facet 的 verdict 结构与判定序列的具体定义由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- ORD-001 偏序集：$\mathcal{P}(L)$ 配以 $\subseteq$ 是偏序集，$\mathrm{Cn}$ 成员判定是该偏序上二元关系 $\Gamma \subseteq \mathrm{Cn}(\Delta)$ 类判定的核心
- ORD-002 完全格：$\mathcal{P}(L)$ 在 $\subseteq$ 下是完全格，$\mathrm{Cn}(\Gamma)$ 是包含 $\Gamma$ 的最小闭集，闭包的极小性在该格中陈述
- ORD-003 Knaster-Tarski 不动点：闭包算子 $\mathrm{Cn}$ 的不动点即 $\mathrm{Cn}$ 闭集，亦即理论；链的上下文序列 $\{p_1\} \cup A \subseteq \{p_1, p_2\} \cup A \subseteq \cdots$ 单调上升至 $\mathrm{Cn}(A \cup \{p_1, \ldots, p_n\})$
- ORD-004 monotone operator：$\mathrm{Cn}$ 是单调算子，单调性为闭包算子三性质之一；保真条件 (二) 是单调性与 $\mathrm{TRUTH}$ 封闭的合成
- ORD-005 链与反链：上下文序列是 $\mathcal{P}(L)$ 中的升链，本条目只用链的一维增长，不用反链结构

## 历史脉络 {#history}

- 1933 年 Tarski 在 Pojęcie prawdy w językach nauk dedukcyjnych 中确立语义真值与后承关系
- 1936 年 Tarski 在 Kalkułem semantycznym 中给出语义后果算子的特征性质，本条目闭包三性质即其逻辑语境形态
- 1948 年 Kuratowski 在 Topologie I 中系统化闭包算子理论，$\mathrm{Cn}$ 三性质即其闭包公理的特例
- 1931 年 Gödel 不完备性定理划定可判定理论边界，本条目适用边界节的半可判定标注承其结构
- 1950s 起一阶逻辑健全性定理成为逻辑教材标准内容，本条目健全性定理是其沿链的逐步形态

## 工程注意事项 {#engineering-notes}

使用带锚推衍链判定须执行以下核验动作。

1. 锚集先行登记：开链前固定 $A$ 并登记其真值依据，如规则包版本与金向量哈希；锚未登记而开链的，健全性条件 (一) 不可检
2. 逐边记录成员依据：每条判定 $p_i$ 记录其使用的上下文 $\{p_1, \ldots, p_{i-1}\} \cup A$ 的具体条目与所引材料，使镜像回溯可机械执行
3. 镜像方向复验：终点判定产出后自末端回溯逐条复验成员关系，任一步不可回溯即标记待审视，不得记通过
4. 局部通过不推全局：所有边检通过不等于终点为真；终局判定前须独立核查锚集真值，如规则包对金向量的回归
5. 可判定性声明：判定所依赖的理论可判定时成员检验终止；依赖不可判定理论时声明半可判定，搜索无果不得表述为非成员

## 参考文献 {#references}

- Tarski, A. (1933). Pojęcie prawdy w językach nauk dedukcyjnych. Prace Polskiego Towarzystwa Matematycznego
- Tarski, A. (1936). Kalkułem semantycznym. Prace Polskiego Towarzystwa Matematycznego
- Kuratowski, K. (1948). Topologie I. PWN
- Gödel, K. (1931). Über formal unentscheidbare Sätze der Principia Mathematica und verwandter Probleme I. Monatshefte für Mathematik und Physik
- Enderton, H. (2001). A Mathematical Introduction to Logic, 2nd ed. Academic Press
- Wikipedia "Consequence operator" 条目
