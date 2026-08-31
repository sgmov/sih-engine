---
entry: ENTRY-ORD-010.md
agent: 复归段补强波簇B
proposed_id: ORD-010
subrepo: order
id_reason: 复归段补强波按簇预留号段，簇B 第二件取 010，与簇内 ORD-006 分列，避并行簇号段冲突
anchors:
  - pro: EPI-15
    source: sih-philosophy/emanation/epistrophe/15-on-falsifiability.md:55
    quote: 每步命题的证伪条件须满足三要素：
  - pro: EPI-15
    source: sih-philosophy/emanation/epistrophe/15-on-falsifiability.md:204
    quote: 哲学链存在 3 步不可证伪命题
selfcheck:
  - 锚 1 至 2 逐字节命中：grep -nF 复核 15:55 命中；15:204 取该行不含破折号前段 哲学链存在 3 步不可证伪命题 命中，前段零破折号零全角括号
  - 三查：全文 grep 全角括号中文内容零处、破折号字符零处、围栏代码块零处
  - 八节在场且顺序固定，首行 H1 为 # ORD-010 证伪条件与反例搜索，首个二级标题为 定义
  - 数学主张：三要素形式化、反例搜索半可判定性定理含 Omega 可数且枚举给定前提，四态判据机械处给机械判据、哲学判据处如实标注；第二锚改用 204 行不含破折号前段，两引文均不含全角括号与破折号
---

# ORD-010 证伪条件与反例搜索

状态：草稿，复归段补强波簇B 产出，哲学到工程桥梁条目。

## 定义 {#definition}

证伪条件是把命题绑定到可观察测试的三元结构，即观测空间、观测映射、测试谓词。本条给 EPI-15 的证伪条件三要素与四态判定以可判定的数学形态。

设 $\Omega_{\mathrm{phen}}$ 是现象空间，$o: \Omega_{\mathrm{phen}} \to \Omega$ 是观测映射，$\Omega$ 是观测空间。观测对应物条件，三要素之一，即 $\Omega$ 等于 $o$ 的值域，每个观测点皆有可观察现象对应，值域外的观测点无观察对应物，称空点：

$$\Omega = o(\Omega_{\mathrm{phen}}) \label{eq:ord010-observation-range}$$

测试谓词 (test predicate) 是 $T: \Omega \to \{0, 1\}$。命题 $P$ 具有证伪条件 $T$，当且仅当 $P$ 逻辑等价于 $\Omega$ 上全体 $x$ 满足 $T(x) = 0$：

$$P \iff \forall x \in \Omega,\ T(x) = 0 \label{eq:ord010-falsification-condition}$$

由此 $P$ 被证伪当且仅当存在 $x$ 使 $T(x) = 1$，该 $x$ 称为反例 (counterexample)。

三要素的形式化。

- 要素一 观测对应物：值域条件 $\ref{eq:ord010-observation-range}$，$\Omega$ 无空点
- 要素二 独立性：$T$ 的定义不依赖 $P$ 的支持者；可判定表述为 $T$ 在基础观测语言中定义，其定义闭包不含 $P$ 的专名谓词，即语法非循环条件，可按谓词集机械检查
- 要素三 可重复性：$T$ 可判定，存在机械过程对任意 $x \in \Omega$ 有限步输出 $T(x)$

## 公理条件 {#axioms}

证伪条件与反例搜索的成立依赖以下公理条件。

- 可数观测空间：$\Omega$ 可数，存在枚举 $\Omega = \{x_0, x_1, x_2, \dots\}$，反例搜索遍历的前提
- 给定枚举：枚举序 $\{x_i\}$ 预先固定，搜索按此序遍历，序变更改变搜索行为
- 谓词可判：$T$ 可判定，要素三，对任意 $x_i$ 有限步得 $T(x_i)$
- 等价良定义：$P \iff \forall x \in \Omega, T(x) = 0$ 在给定语言中是良定义的逻辑等价

定理，反例搜索半可判定性。设 $\Omega$ 可数且枚举 $\{x_i\}$ 给定，$T$ 可判定，$P$ 等价于 $\forall x \in \Omega,\ T(x) = 0$，见公式 $\ref{eq:ord010-falsification-condition}$。则：

- 若 $P$ 假，即存在 $x^*$ 使 $T(x^*) = 1$，按枚举序逐项计算 $T(x_0), T(x_1), \dots$ 的反例搜索，到达 $x^*$ 的有限索引时有限步停机并输出反例 $x^*$
- 若 $P$ 真，搜索逐项得 $T(x_i) = 0$，不保证停机，不停机不是 $P$ 真的证据

即 $\neg P$ 是半可判定的 (semi-decidable)，递归可枚举；$P$ 是共半可判定的 (co-semi-decidable)。证明：$P$ 假时 $x^*$ 占枚举中有限索引 $k$，前 $k$ 项计算有限步完成，第 $k$ 项输出反例，搜索停机；$P$ 真时无反例，搜索不触发停机条件，可能无限继续。$\square$

四态判定，即 EPI-15 判定标记的数学形态。

- 可证伪：存在满足三要素的 $T$ 使 $P$ 等价于 $\forall x \in \Omega, T(x) = 0$。判据：给出该 $T$ 并核验值域条件、语法非循环、谓词可判，三项皆可机械检查
- 部分可证伪：$P$ 指定的证伪条件不完整；标准可判定形态为合取形态，$P$ 可分解为合取 $Q \land R$，$Q$ 可证伪而 $R$ 否。判据：给出分解，对 $Q$ 与 $R$ 分别套用可证伪与不可证伪判据。边界：原文「观察依赖诠释」类情形 (如 01- 步) 属要素三可重复性受限的形态，不属合取形态，本条只给合取形态的可判定判据，边界如实标注
- 不可证伪：$P$ 属于定义集 DEF 的后果闭包 $\mathrm{Cn}(\mathrm{DEF})$，承 ORD-006，无需观测即判。判据：核验 $P \in \mathrm{Cn}(\mathrm{DEF})$；$\mathrm{Cn}$ 不可判时退化为哲学判据，如实标注
- 自明：$P$ 为定义重言，属语言定义闭包。判据：核验 $P$ 是定义语言中的重言式，命题逻辑层面机械可判。本态为不可证伪态的子情形 (重言属 $\mathrm{Cn}(\mathrm{DEF})$)，原文四态分列两者

主张，不可证伪不等于伪命题。

- 不可证伪的 $P$ 可为真：重言式 (tautology) 不可证伪且为真；四态按证立方式分类，不按真假分类，可证伪态的真假由观测决定，不可证伪态的真假由定义与逻辑形式决定
- 不可证伪命题属定义、逻辑、定理层，其证立由定义与逻辑形式担保，不依赖经验观测；与无观察对应物的空话有本质区别，后者不可证伪且无定义或逻辑担保，是双缺

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Popper 可证伪性划界判据，递归论与算术层级，Pi-1 句子与半可判定性
- 哲学命题：EPI-15 可证伪性检测，证伪条件三要素即观察对应物、独立性、可重复性，与四态判定，不可证伪不等于伪命题
- 形式化：EPI-15 的三要素对应值域条件、语法非循环、谓词可判定，见定义节；四态按证立方式分类，可证伪与部分可证伪锚经验观测，不可证伪与自明锚定义与逻辑。反例搜索半可判定性给敢于自我证伪以机械形态：证伪条件可判时反例存在则有限步暴露，反例不存在则搜索不保证停机，命题为真不靠搜索停机担保，而靠定义与逻辑形式担保。

## 在 facet 的应用 {#facet-application}

- 应用场景：交叉审阅中命题的证伪条件核验与反例登记
- 形式化：facet 审阅对象命题配 $P$，证伪条件配 $T$ 与观测空间 $\Omega$；核验三要素即值域条件、语法非循环、谓词可判；登记反例即记录使 $T(x) = 1$ 的 $x$ 与检验过程
- 借鉴方向：半可判定性保证反例出现时有限步暴露并停机，facet 的反例搜索可设有限步预算，超预算未停机不判命题真，如实记录未决
- 边界：$\Omega$ 的构造、$T$ 的具体实现、枚举序的选定由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- ORD-006 闭包算子与后果算子：不可证伪态判据 $P \in \mathrm{Cn}(\mathrm{DEF})$ 承后果算子 $\mathrm{Cn}$ 与闭包，无需观测即判
- ORD-002 完全格：定义闭包与后果闭包构成格结构，重言式与定义闭包在其最小元侧
- 递归论：$\neg P$ 半可判定、$P$ 共半可判定，对应算术层级中 Pi-1 句子地位，$\forall x, T(x) = 0$ 是 Pi-1 形式
- 模型论：观测映射与值域条件给出命题的经验指称，$\mathrm{Cn}$ 的模型论基础见 ORD-006；部分可证伪与可证伪的区分不引入概率测度，与 PROB 子仓正交，证伪是存在性判定非测度判定

## 历史脉络 {#history}

- 1934 年 Popper 在 Logik der Forschung 中以可证伪性提出科学与非科学划界
- 1936 年 Turing 与 Church 独立确立可判定性与递归可枚举性，半可判定性概念成形
- 1940s Mostowski 与 Kleene 系统发展算术层级，Pi-1 句子与半可判定性对应
- 1960s 后可证伪性在科学哲学中发展为可检验性、可反驳性，与 Popper 划界并置

## 工程注意事项 {#engineering-notes}

应用证伪条件与反例搜索时需核验四件事。

1. 值域条件是否成立：$\Omega = o(\Omega_{\mathrm{phen}})$ 须核验，存在无观察对应物的空点时要素一破坏，证伪条件不成立
2. 语法非循环是否成立：$T$ 的定义闭包是否含 $P$ 的专名谓词须机械检查，含则要素二破坏，独立性失效
3. 谓词可判是否成立：$T$ 是否可判定须核验，不可判时反例搜索半可判定性前提失效，搜索可能永不输出
4. 枚举是否给定：$\Omega$ 可数且枚举序 $\{x_i\}$ 须预先固定，枚举缺失或序变更时搜索行为不可比

工程局限。$\Omega$ 不可数或 $T$ 不可判时，反例搜索半可判定性不成立，四态中可证伪态判据退化为存在性声明而非机械验证，须如实标注为哲学判据。

## 参考文献 {#references}

- Popper, K. (1934). Logik der Forschung. Johann Ambrosius Barth, Leipzig
- Rogers, H. (1987). Theory of Recursive Functions and Effective Computability. MIT Press
- Soare, R.I. (1987). Recursively Enumerable Sets and Degrees. Springer
- Tarski, A. (1936). Sur les systèmes de formules logiques et certains aspects de la définition et de la théorie des ensembles. Colloquium Mathematicum, 1, 7-13
- Mendelson, E. (1997). Introduction to Mathematical Logic, 4th ed. CRC Press
- Wikipedia "Falsifiability" 条目
