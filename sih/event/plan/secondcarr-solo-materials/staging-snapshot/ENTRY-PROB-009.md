---
entry: ENTRY-PROB-009.md
agent: 复归段补强波簇C
proposed_id: PROB-009
subrepo: probability
id_reason: probability 子仓 INDEX 现行已建至 PROB-007，PROB-008 由本波簇 B 预留，本件取下一空号 PROB-009
anchors:
  - pro: PRO-04
    source: sih-philosophy/emanation/proodos/04-on-third-tao.md:17
    quote: "维护之前必须先恢复意图，恢复是维护的因果前提"
  - pro: PRO-04
    source: sih-philosophy/emanation/proodos/04-on-third-tao.md:111
    quote: "此锚定为启发性类比而非严格数学推导"
selfcheck:
  - 锚二条 grep -nF 复核命中 04-on-third-tao.md 第 17 行与第 111 行，引文零全角括号零破折号
  - 三查：全文全角括号中文内容零处、破折号字符零处、围栏代码块零处
  - 八节在场且顺序固定，首行 H1 形对
  - 数学主张：自晦定理两方向证明完整，Fano 定理二按标准链五步逐项写明，底数 e，h(P_e) 小于等于 1 的理由为底 e 时二元熵最大值 log 2 小于 1，命题三先验支持集形式弛缓正先验条件已标注，恢复必行性下界严格正，m 等于 2 与条件熵不大于 1 的边界已标注
  - 整理释明：任务书综合段写作 h(P_e) 弱化至 P_e 量级，但二元熵在小参数下大于参数本身，本件按数学一致形取 h(P_e) 小于等于 1 为常数项，整理为 H(I|C) 小于等于 P_e log m 加 1 后移项得下界
---

# PROB-009 Fano 不等式与恢复误差下界

状态：草稿，复归段补强波簇C 产出，哲学到工程桥梁条目。

## 定义 {#definition}

Fano 不等式给任意恢复映射的恢复误差以条件熵刻画的普适下界，与自晦定理共同确定必复的精度边界。

设定。$I$ 为有限意图集，$|I| = m \geq 2$，$C$ 为码字集，$f: I \to C$ 为确定性实现映射。$P_I$ 为 $I$ 上的正概率分布，$P_I(i) > 0$ 对一切 $i$。恢复器即解码器为映射 $s: C \to I$。记恢复值 $\hat{I} = s(f(I))$，误差事件为 $\hat{I} \neq I$，误差概率为

$$P_e = \sum_{i \in I: s(f(i)) \neq i} P_I(i) \label{eq:prob009-error-prob}$$

代码随机变量 $C$ 取值 $c \in f(I)$ 的概率为 $P(C = c) = \sum_{i: f(i) = c} P_I(i)$，像外概率为 0。

后验与条件熵。$f$ 确定性使似然 $P(c \mid i)$ 取 0-1 值：$c = f(i)$ 时 $P(c \mid i) = 1$，否则为 0。由 Bayes 公式，后验为

$$P(i \mid c) = \frac{P_I(i)}{\sum_{i': f(i') = c} P_I(i')} \ (f(i) = c), \qquad P(i \mid c) = 0 \ (f(i) \neq c) \label{eq:prob009-posterior}$$

离散分布的熵 $H(X) = -\sum_x P_X(x) \log P_X(x)$，$\log$ 底取 e 为默认约定，可换底，换底时熵值乘常数因子，不等式相对形式不变。条件熵为

$$H(I \mid C) = -\sum_{c} P(C = c) \sum_{i} P(i \mid c) \log P(i \mid c) \label{eq:prob009-conditional-entropy}$$

定理一 (自晦定理)。$f$ 非单射蕴含 $H(I \mid C) > 0$；$f$ 单射蕴含 $H(I \mid C) = 0$，且恢复可无错。

证明。正向：设 $f$ 非单射，存在 $i \neq j$ 使 $f(i) = f(j) = c$。先验正，故 $P(C = c) \geq P_I(i) > 0$，该码字下后验至少两值皆正：$P(i \mid c) = P_I(i) / \sum_{i': f(i') = c} P_I(i') \in (0, 1)$，同法 $P(j \mid c) \in (0, 1)$。至少含两个正值的分布熵严格为正，该码字对条件熵的贡献 $P(C = c) H(I \mid C = c)$ 为正，故 $H(I \mid C) > 0$。反向：设 $f$ 单射，每个非空纤维单元素，对每个 $c \in f(I)$，条件分布 $P(I \mid C = c)$ 为单位分布，$H(I \mid C = c) = 0$；像外 $c$ 有 $P(C = c) = 0$。故 $H(I \mid C) = 0$。取 $s$ 在 $f(I)$ 上为 $f$ 的单射逆，像外任意，则对一切 $i$，$s(f(i)) = i$，恢复无错。证毕。

定理二 (Fano 不等式)。对任意恢复器 $s$，

$$P_e \geq \frac{H(I \mid C) - 1}{\log m} \label{eq:prob009-fano}$$

证明，标准链五步。记 $\hat{I} = s(C)$，误差指示 $E = \mathbf{1}_{\{\hat{I} \neq I\}}$，即 $\hat{I} \neq I$ 时 $E = 1$，否则 $E = 0$。

第一步，链分解。由条件熵链式法则，

$$H(I \mid C) = H(I \mid C, E) + I(I; E \mid C)$$

其中 $I(I; E \mid C) = H(E \mid C) - H(E \mid I, C)$ 为条件互信息。$E$ 是 $(I, C)$ 的确定性函数，$s$ 固定，$\hat{I} = s(C)$ 由 $C$ 决定，$E$ 由 $I$ 与 $\hat{I}$ 决定，故 $H(E \mid I, C) = 0$，得 $I(I; E \mid C) = H(E \mid C)$。

第二步，误差支界定。$H(I \mid C, E) = P_e H(I \mid C, E = 1) + (1 - P_e) H(I \mid C, E = 0)$。给定 $C$ 与 $E = 0$，$\hat{I} = I$，$I$ 至多一种取值，$H(I \mid C, E = 0) = 0$。给定 $C$ 与 $E = 1$，$\hat{I} = s(C)$ 为定值且 $I \neq \hat{I}$，$I$ 至多取 $m$ 个值中的 $m - 1$ 个，$H(I \mid C, E = 1) \leq \log(m - 1)$。故 $H(I \mid C, E) \leq P_e \log(m - 1)$。

第三步，二元熵。$E$ 为参数 $P_e$ 的 Bernoulli 变量，条件化降熵给 $H(E \mid C) \leq H(E) = h(P_e)$，其中二元熵函数

$$h(p) = -p \log p - (1 - p) \log(1 - p), \qquad h(0) = h(1) = 0 \label{eq:prob009-binary-entropy}$$

第四步，二元熵上界。底取 e 时二元熵最大值在 $p = 1/2$ 取到，$h(1/2) = \log 2 < 1$，log 2 约 0.6931。故对一切 $p \in [0, 1]$，$h(p) \leq 1$。底数约定理由在此写明：底取 e 时下界中的常数项恰为 1；若底取 2，则 $h(p) \leq 1$ bit，公式常数项为 1 bit，两形皆成立，本件固定底 e。

第五步，综合。第一至四步给

$$H(I \mid C) \leq P_e \log(m - 1) + h(P_e) \label{eq:prob009-fano-standard}$$

此为 Fano 不等式标准形式，同节登记。逐项弱化：$\log(m - 1) \leq \log m$，且 $h(P_e) \leq 1$，得 $H(I \mid C) \leq P_e \log m + 1$。加一减一整理，1 移项在左，两边除以 $\log m$，得 $P_e \geq (H(I \mid C) - 1) / \log m$，即公式 $\ref{eq:prob009-fano}$。推论形式由标准形式经 $\log(m - 1) \leq \log m$ 与 $h(P_e) \leq 1$ 两弱化推出。证毕。

适用边界。$m = 2$ 时 $\log(m - 1) = 0$，标准形式退化为 $H(I \mid C) \leq h(P_e)$，推论形式 $P_e \geq (H(I \mid C) - 1) / \log 2$ 仍成立；$H(I \mid C) \leq 1$ 时推论下界不大于 0，此时 $P_e$ 非负本身是唯一有效信息。

命题三 (命名精度)。设 $A \subseteq I$ 为单意图集，即先验 $P_I$ 支撑在 $A$ 上：$i \notin A$ 时 $P_I(i) = 0$，此弛缓主设定的正先验条件，弛缓已标注，且限制 $f|_A$ 单射，则 $H(I \mid C) = 0$，Fano 下界归零。

证明。对 $c \in f(A)$，$f^{-1}(c) \cap A$ 单元素，限制单射保证，故 $P(I \mid C = c)$ 为单位分布，$H(I \mid C = c) = 0$；$f(A)$ 外 $P(C = c) = 0$。故 $H(I \mid C) = 0$。证毕。

一般定量形式。设 $p_A = P(I \in A)$，$B = \mathbf{1}_{\{I \in A\}}$。$B$ 是 $I$ 的函数，条件化增熵给 $H(I \mid C) \leq H(I, B \mid C)$；链式法则 $H(I, B \mid C) = H(I \mid C, B) + H(B \mid C)$；给定 $C$ 与 $B = 1$ 时 $I$ 属于 $A \cap f^{-1}(C)$，为单元素，$H(I \mid C, B = 1) = 0$，$B = 0$ 支 $H(I \mid C, B = 0) \leq \log m$，故 $H(I \mid C, B) \leq P(I \notin A) \log m$；条件化降熵给 $H(B \mid C) \leq H(B) = h(p_A)$。综合

$$H(I \mid C) \leq h(p_A) + P(I \notin A) \log m \label{eq:prob009-naming-precision}$$

先验支撑越集中于 $f$ 单注入子集，右端越低，极端 $p_A = 1$ 即归零形式。命名精度即先验选择，非工程可调参数，是登记层决定量：名字精确则先验集中，下界低，复的成本低；名字模糊则先验分散，下界高。

恢复必行性命题。Fano 下界给的是精度下界非不可能性断言：恢复恒存在，任意映射 $s: C \to I$ 皆为恢复器，被下界约束的是其精度。$f$ 非单射时，取 $c$ 与 $i \neq j$ 使 $f(i) = f(j) = c$，则对任意恢复器 $s$，$s(c)$ 至多等于 $i$ 与 $j$ 之一，故 $P_e \geq \min(P_I(i), P_I(j)) > 0$，恢复必行但精度下界严格正；下界由先验与纤维结构定，不由编码者技艺定。先验改进即命名精度可降 $H(I \mid C)$ 与下界，但非单射纤维保证任何恢复器 $P_e > 0$，下界不由技艺消除。

Shannon 诚实边界。原文声明 Shannon 锚定为启发性类比而非严格数学推导，锚定提供独立参照，不声称道三由 Shannon 定理反证。本件 Fano 下界为有限离散情形的严格定理，不依赖 Shannon 信道模型与信源编码定理，是必复精度下界的严格化。原文引用 Shannon 信源编码定理，无损编码需编码长度无限制；其与 Fano 下界的关系为：无限长度对应码字集 $C$ 扩张使实现映射 $f$ 趋向单射，条件熵趋向零，Fano 下界趋向零。本条只登记此对应，不展开。

## 公理条件 {#axioms}

自晦定理与 Fano 不等式依赖以下公理条件。

- 有限意图集：$m = |I| \geq 2$ 有限，Fano 下界要求 $m$ 有限，$\log m$ 分母有意义
- 正先验：$P_I(i) > 0$ 对一切 $i$，后验与条件熵无 0 log 0 边界；命名精度命题弛缓为支撑集形式，弛缓已在该命题标注
- 确定性实现：$f: I \to C$ 确定性，似然 $P(c \mid i)$ 取 0-1 值；随机实现，即同一需求生成不同实现，是二阶近似，属扩展，不写入主定理
- 确定性解码器：$s: C \to I$ 确定性；随机解码器不进入主定理
- 底数约定：$\log$ 底取 e，换底时下界常数项改变，底 2 给 1 bit，比较须同底
- 有限概率空间：有限集上计数分布满足 PROB-006 公理，条件概率良定义

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Fano 1961 误差概率计算，贝叶斯决策论最小错误估计，条件熵
- 哲学命题：PRO-04 道三，代码自晦意图必复 (04- § 道三命题)
- 形式化：原文「维护之前必须先恢复意图，恢复是维护的因果前提」即恢复器 $s$ 先于维护动作，数学形态即估计问题先行；自晦即 $f$ 非单射，定理一给 $H(I \mid C) > 0$；必复即 Fano 下界，恢复存在且精度有下界；原文诚实声明 Shannon 锚定为启发性类比，本件给有限离散严格版；「技艺高者损失少，但仍有损失」的数学形态即先验可降下界，但非单射纤维保任何恢复器 $P_e$ 严格正。与 ALG-009 的分界：ALG-009 承恢复的代数结构，截面存在性与非唯一性，本条承恢复的精度下界，概率侧，两者互补

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 多 shot 判据，判据恢复
- 形式化：同一意图 $m$ 个实现候选，$k$ 次观察读数，判据恢复即估计问题，Fano 下界给判据恢复的不可约误差；术语登记精度即先验集中度，术语模糊则先验分散，条件熵高，下界高，判据恢复不可靠，检词位精度有数学意义
- 借鉴方向：下界计算需先验，先验取自术语登记与需求文档，即名字即先验；登记层精度经先验进入下界，工程层不重估
- 边界：$k$ 次观察与读数模型由 facet 工程层决定；随机实现是二阶近似，不写入主定理

## 与其他概念的关系 {#relations}

- PROB-005 Bayesian 更新：定义节后验由 Bayes 公式 $\ref{eq:prob005-bayes}$ 给出，恢复问题是贝叶斯估计问题，最小错误解码器取后验众数
- PROB-006 概率测度：有限集分布是 PROB-006 的特例，见 $\ref{eq:prob006-countable-additivity}$
- ALG-008 线性映射的核与纤维：非单射纤维是歧义载体，公式 $\ref{eq:alg008-coset}$ 为线性版，定理一的两意图纤维是其一般集合形态
- ALG-009 秩零化度与截面：ALG-009 承恢复的代数结构，截面存在性与非唯一性，公式 $\ref{eq:alg009-section}$；任何截面残差非零，公式 $\ref{eq:alg009-residual}$；本条承恢复的精度下界，概率侧，两者互补，任何恢复器 $P_e > 0$ 的代数形态即残差命题

## 历史脉络 {#history}

- 1948 年 Shannon A Mathematical Theory of Communication 确立信息论，原文声明道三在此的 Shannon 锚定为启发性锚定，非严格推导
- 1961 年 Fano 误差概率计算给误差概率与条件熵的不等式形态，即 Fano 不等式
- Cover 与 Thomas 教材固定 Fano 不等式标准形式，为估计与信道编码章标准工具

## 工程注意事项 {#engineering-notes}

应用 Fano 下界时需验证五件事。

1. 先验可用性：Fano 下界计算需先验 $P_I$，先验来自术语登记与需求文档，即名字即先验；先验非工程可调参数，是登记层决定量，先验登记须留痕
2. $m$ 可枚举性：$m$ 可枚举，因意图入名册登记，$m$ 为名册计数；意图未入册则 $m$ 低估，下界随之低估，须如实标注
3. 随机实现：同一需求生成不同实现是二阶近似，如实标注为扩展，不写入主定理
4. 下界非可达值：Fano 下界是下界非可达或可预测值，工程侧用于判据排序与验收门槛，不用于误差预测
5. 底数约定：$\log$ 默认底取 e，换底时下界常数项改变，底 2 给 1 bit，公式形态与比较须同底

## 参考文献 {#references}

- Fano, R.M. (1961). On the computations needed to translate a language. IRE Trans. Information Theory
- Shannon, C.E. (1948). A Mathematical Theory of Communication. Bell System Technical Journal, 27, 379-423
- Cover, T.M. & Thomas, J.A. (2006). Elements of Information Theory, 2nd ed. Wiley
- PROB-005 Bayesian 更新 与 PROB-006 概率测度 仓内条目
- Wikipedia "Fano's inequality" 条目
