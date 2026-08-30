---
entry: ENTRY-PROB-004.md
agent: entry-prob-b
anchors:
  - pro: PRO-08 应几
    source: sih-philosophy/emanation/proodos/08-on-settle.md:100
    quote: "应几：当前尚不可观测的、需要预判的"
  - pro: PRO-08 应对
    source: sih-philosophy/emanation/proodos/08-on-settle.md:102
    quote: "应辨与应几共同构成司衡的\"应对\"维度：既处理已发生的事，也预防未发生的事。"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---
# PROB-004 大偏差原理

状态：已建，哲学到工程桥梁条目。

## 定义 {#definition}

大偏差原理即 Large Deviation Principle，缩写 LDP，是随机变量和的尾部概率渐近理论，刻画样本均值偏离期望的概率的指数衰减速率。

设 $X_1, X_2, \ldots$ 是独立同分布随机变量序列，公共期望 $\mathbb{E}[X_i] = \mu$ 有限。定义样本均值

$$S_n = \frac{1}{n} \sum_{i=1}^{n} X_i \label{eq:prob004-sample-mean}$$

称序列 $\{S_n\}$ 以速率函数 $I$ 满足大偏差原理，若下列两个不等式成立：

上界：对一切闭集 $F \subseteq \mathbb{R}$，

$$\limsup_{n \to \infty} \frac{1}{n} \log \mathbb{P}(S_n \in F) \leq -\inf_{x \in F} I(x) \label{eq:prob004-upper}$$

下界：对一切开集 $G \subseteq \mathbb{R}$，

$$\liminf_{n \to \infty} \frac{1}{n} \log \mathbb{P}(S_n \in G) \geq -\inf_{x \in G} I(x) \label{eq:prob004-lower}$$

速率函数 $I$ 是下半连续凸函数，取值于 $[0, \infty]$，在使 LDP 成立的条件之下，$I(x) = 0$ 恰在 $x = \mu$ 处成立，且 $x \neq \mu$ 时 $I(x) > 0$。LDP 断言 $\mathbb{P}(S_n \in A) \approx e^{-n \inf_{x \in A} I(x)}$ 在指数阶意义下成立，即偏离的概率随 $n$ 指数衰减，衰减速率由集合内最接近均值的 $I$ 值决定。

Cramér 定理。若矩生成函数 $M(t) = \mathbb{E}[e^{tX_1}]$ 在 $0$ 的某邻域内有限，则样本均值满足 LDP，速率函数是 $\log M$ 的 Legendre-Fenchel 共轭：

$$I(x) = \sup_{t \in \mathbb{R}} \left( tx - \log M(t) \right) \label{eq:prob004-cramer-rate}$$

良态性。若 $I$ 是良速率函数，即其一切水平集 $\{x : I(x) \geq \alpha\}$ 紧，则上界对一切闭集 $F$ 收紧为等式：

$$\lim_{n \to \infty} \frac{1}{n} \log \mathbb{P}(S_n \in F) = -\inf_{x \in F} I(x)$$

且对满足 $\inf_{G} I = \inf_{\overline{G}} I$ 的开集 $G$，即边界不改变速率函数下确界的开集，下界同样收紧为等式。

Sanov 定理是 LDP 在概率测度空间上的形式：经验测度序列满足 LDP，速率函数是经验测度与真实分布之间的相对熵即 Kullback-Leibler 散度。

## 公理条件 {#axioms}

Cramér 定理的成立依赖以下公理条件。

- 独立同分布：$X_1, X_2, \ldots$ 相互独立且同分布
- 期望有限：$\mathbb{E}[X_i] = \mu$ 有限，这决定速率函数的零点
- 矩生成函数有限：$M(t) = \mathbb{E}[e^{tX_1}] < \infty$ 在 $0$ 的某邻域内成立，这是保证 $\log M$ 的 Legendre 变换给出在 $\mu$ 处取唯一零的速率函数的充分条件
- 概率空间：$(\Omega, \mathcal{F}, \mathbb{P})$ 为一般概率空间即可，完备性非必需

矩生成函数有限条件的失真是本条目的主要边界。重尾情形下，和的尾部概率可能不再由全部 $n$ 项共同决定而由单一极端值主导，指数衰减形式 $e^{-n I(x)}$ 失真，LDP 在以上意义下可能不成立或速率函数退化。

两个不等式强度不对称：上界由指数型 Markov 不等式即 Chernoff 界直接给出，一般易于建立；下界需要构造实现偏离的路径；良态下的等式还要求水平集紧。

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Cramér 1938 年给出随机游走和的速率函数，Chernoff 1952 年给出指数矩方法，Dembo 与 Zeitouni 1998 年给出理论的系统处理
- 哲学命题：PRO-08 应，应论原文话题即治理如何应对未预见的挑战
- 形式化：应几针对当前尚不可观测的、需要预判的。大偏差原理为这类异常提供可精确计算的指数速率：偏离均值越远，速率函数 $I$ 取值越大，概率衰减越快，预判窗口与尾部风险量级均可量化。应辨与应几共同构成应对维度：LDP 把未发生的异常的概率速率显式化，应对的前置检测有尾部量化的口径可依

## 证明思路 {#proof-sketch}

上界的证明思路是指数型 Markov 不等式方法。

对任意 $t > 0$，由 Markov 不等式与 $X_i$ 的独立性，

$$\mathbb{P}(S_n \geq x) \leq e^{-nt} M(t)^n \label{eq:prob004-chernoff-bound}$$

取对数除以 $n$ 后令 $n \to \infty$，得 $\limsup_{n \to \infty} \frac{1}{n} \log \mathbb{P}(S_n \geq x) \leq tx - \log M(t)$，右端再对 $t$ 取下确界即得上界，见公式 $\ref{eq:prob004-upper}$。取到等价的 $t$ 由驻点条件 $x = (\log M)'(t)$ 刻画，即公式 $\ref{eq:prob004-cramer-rate}$ 的 Legendre 变换结构。下界需要构造实现偏离的路径：在 Cramér 条件下对分布作指数扭曲，扭曲后的测度下和的均值移至 $x$，Radon-Nikodym 导数给出变测度的代价，其渐近指数恰为 $I(x)$。

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 多厂 voting 的判据偏差尾部风险
- 形式化：多厂多 shot 投票的输出建模为 iid 采样，对固定阈值 $\varepsilon > 0$，均值偏离真值的尾部概率以速率 $I(\mu + \varepsilon)$ 指数衰减，即对充分大的 $n$ 与任意 $\delta > 0$，$\mathbb{P}(S_n \geq \mu + \varepsilon) \leq e^{-n(I(\mu + \varepsilon) - \delta)}$；给定虚警率 $\alpha$ 所需的 shot 数 $n$ 可由 $n \geq \log(1/\alpha) / (I(\mu + \varepsilon) - \delta)$ 反解，闸阈值设置因此有速率函数给出的量化口径
- 借鉴方向：尾部风险指数估计是 LDP 的工程对应物，但具体实验配置，厂数 / shot 数 / 命题数，由 facet 工程层决定，不属本条目范围
- 边界：iid 假设与矩生成函数有限性的成立性由 facet 工程层决定，本条目只提供尾部口径的严格化路径

## 与其他概念的关系 {#relations}

- PROB-001 大数定律：WLLN 断言样本均值的偏离概率消失，见公式 $\ref{eq:prob001-wlln-limit}$，LDP 进一步刻画其消失的速率：对任意固定 $\varepsilon > 0$，$\lim_{n \to \infty} \frac{1}{n} \log \mathbb{P}(S_n \geq \mu + \varepsilon) = -I(\mu + \varepsilon)$
- PROB-002 强大数定律：在 Cramér 条件下，LDP 给出指数估计，联合 Borel-Cantelli 引理得 $\sum_n \mathbb{P}(S_n \geq \mu + \varepsilon) < \infty$，偏离事件几乎必然只发生有限次，这是 SLLN 的一个标准证明路径
- PROB-003 中心极限定理：CLT 给样本均值小偏差在 $O(1/\sqrt{n})$ 尺度下的高斯刻画，LDP 给大偏差在指数尺度下的刻画，两者互补不互推
- PROB-006 概率测度：LDP 是概率空间上事件概率的渐近陈述，样本均值是其上的随机变量
- PROB-007 期望：速率函数的零点是 $X_i$ 的期望 $\mu$，$I$ 的零位即样本均值收敛的中心

## 历史脉络 {#history}

- 1938 年 Cramér 在随机游走和的尾部概率渐近估计中给出速率函数公式，是 Cramér 定理的原始形式
- 1952 年 Chernoff 给出指数矩方法与 Chernoff 界，是大偏差方法的起点
- 1957 年 Sanov 定理建立经验测度的 LDP，速率函数为相对熵
- 1960s 至 1970s Gärtner、Ellis、Varadhan 分别把 LDP 推广到随机过程并建立 Laplace 原理
- 1998 年 Dembo 与 Zeitouni 的 Large Deviations Techniques and Applications 给出理论的系统严格处理

## 工程注意事项 {#engineering-notes}

应用 LDP 时需验证四件事。

1. 矩生成函数是否有限：若判据值重尾，$M(t)$ 可能在 $0$ 的邻域外无穷，Cramér 定理不成立，尾部可能由单一极端值主导，LDP 的指数形式失真
2. 阈值是否固定：LDP 是对固定阈值 $\varepsilon$ 的陈述，阈值若随 $n$ 移动，速率随之改变，用虚警率反解样本数时须注意
3. 良态性：$I$ 的水平集紧性决定上下界是否收紧为等式，有限维均值偏离的应用中自动满足
4. 适用区域：正态近似适用于中心区域，指数近似适用于尾部区域；尾部区域真实衰减不比高斯尾部慢，正态近似会高估尾部概率，用 CLT 估计尾部风险是反保守的

## 参考文献 {#references}

- Cramér, H. (1938). On some combinatorial and asymptotic questions in the theory of probability. Skrifter utgivna af K. Svenska Vetenskaps-Societen i Uppsala, N.F. Mat.-Fys., 2
- Chernoff, H. (1952). A measure asymptotic expansion for sums of independent random variables. Annals of Mathematical Statistics, 23, 509-520
- Dembo, A. & Zeitouni, O. (1998). Large Deviations Techniques and Applications, 2nd ed. Springer
- Durrett, R. (2010). Probability: Theory and Examples, 4th ed. Cambridge University Press
- Wikipedia "Large deviations principle" 条目
