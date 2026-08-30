---
entry: ENTRY-PROB-005.md
agent: entry-prob-b
anchors:
  - pro: PRO-07 鉴·映照
    source: sih-philosophy/emanation/proodos/07-on-assay.md:79
    quote: "映照：只反映事实，不投射判断"
  - pro: PRO-07 鉴·客观
    source: sih-philosophy/emanation/proodos/07-on-assay.md:78
    quote: "客观：不预设立场，不急于下判断"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---
# PROB-005 Bayesian 更新

状态：已建，哲学到工程桥梁条目。

## 定义 {#definition}

贝叶斯更新即 Bayesian updating，是先验信念与观测证据合成后验信念的更新规则，核心是贝叶斯定理。

设 $H$ 是假设，$E$ 是已观测证据。若 $P(E) > 0$，贝叶斯定理给出

$$P(H \mid E) = \frac{P(E \mid H) P(H)}{P(E)} \label{eq:prob005-bayes}$$

其中 $P(H)$ 是先验分布，$P(E \mid H)$ 是似然，$P(H \mid E)$ 是后验分布，$P(E)$ 是边际似然即证据因子，只起归一化常数的作用。

多假设情形。设 $H_1, H_2, \ldots, H_K$ 构成假设空间的划分，由全概率公式，边际似然为

$$P(E) = \sum_{i=1}^{K} P(E \mid H_i) P(H_i) \label{eq:prob005-total-probability}$$

后验分布正比于似然与先验之积：

$$P(H_i \mid E) \propto P(E \mid H_i) P(H_i) \label{eq:prob005-posterior-proportional}$$

序贯更新。证据依次到达时，上一观测后的后验分布成为下一次更新时的先验：

$$P(H \mid E_1, E_2) = \frac{P(E_2 \mid H) P(H \mid E_1)}{P(E_2 \mid E_1)} \label{eq:prob005-sequential}$$

其中 $P(E_2 \mid E_1) = \sum_i P(E_2 \mid H_i) P(H_i \mid E_1)$。给定同一组证据的后验分布不依赖证据处理的先后次序，序贯更新可结合。

共轭先验。若某先验分布族在给定似然族更新后，后验仍属于同一族且更新为封闭形式参数变换，则称该先验族对该似然族是共轭先验。两个典型例子：

- Beta-Bernoulli 共轭：先验 $p \sim \mathrm{Beta}(\alpha, \beta)$，观测到 $k$ 次成功与 $n - k$ 次失败，后验 $p \mid E \sim \mathrm{Beta}(\alpha + k, \beta + n - k)$
- Normal-Normal 共轭：先验 $\theta \sim N(\mu_0, \tau_0^2)$，似然是已知方差 $\sigma^2$ 的正态分布，观测到 $n$ 个样本均值 $\bar{x}$，后验仍为正态，后验均值是先验均值与样本均值的精度加权平均

边界。若先验是不当先验 improper prior，即不是归一化的概率测度，后验仍可能可归一，但须验证 $\int P(E \mid H) \pi(dH) < \infty$，验证失败则后验无定义。贝叶斯更新依赖条件分布 $P(E \mid H)$ 的良定义；朴素贝叶斯所用的条件独立假设不成立时，似然乘积把相关证据当作重复的独立证据，高估其分辨力，后验过度尖锐。

## 公理条件 {#axioms}

贝叶斯更新的成立依赖以下公理条件。

- 概率空间：$(\Omega, \mathcal{F}, \mathbb{P})$ 已配置，条件概率与正则条件概率存在，见 PROB-006
- 证据可归一：$P(E) > 0$；$P(E) = 0$ 时后验无逐点定义，须以密度意义处理
- 先验：先验是假设空间上的概率测度；不当先验须通过后验可归一性检验
- 似然：条件分布 $P(E \mid H)$ 对每个 $H$ 存在且良定义，其形式由模型决定，不依赖先验

后验是条件概率，其代数形式由 Kolmogorov 公理完全决定，贝叶斯更新不引入新公理，引入的是先验与似然的选取负担，该负担属于建模侧。

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Bayes 1763 年首次系统给出逆概率推理，Laplace 1774 年独立发展，de Finetti 1937 年建立协调性基础
- 哲学命题：PRO-07 鉴，鉴论原文话题即治理如何检验自己
- 形式化：客观要求不预设立场，不急于下判断。贝叶斯更新把该姿态形式化：先验是初始立场，似然是证据给出的事实，后验由两者之积决定，更新由似然驱动，先验不独自定论。映照要求只反映事实，不投射判断：证据充分时后验由似然主导，先验的权重被证据因子稀释，后验反映的是观测事实而非先验投射的判断。边界如实标注：弱证据下后验仍受先验牵引，结论的客观性依赖先验声明的诚实性

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 多厂 voting 的候选设计信念更新
- 形式化：命题状态的信念作先验，各 shot 的判据读数作证据，后验按序贯更新规则迭代，见公式 $\ref{eq:prob005-sequential}$；共轭先验族内更新为封闭形式，后验均值可作信念的摘要统计量
- 借鉴方向：序贯信念更新是贝叶斯更新的工程对应物，但具体先验选择与似然参数化由 facet 工程层决定，不属本条目范围
- 边界：本条目不承诺 facet 现行 voting 已实现信念更新，只提供信念合成的严格化路径

## 与其他概念的关系 {#relations}

- PROB-006 概率测度：后验是概率空间上的条件概率，全概率公式是边际似然的载体，见公式 $\ref{eq:prob006-countable-additivity}$
- PROB-007 期望：后验期望是后验分布的一阶摘要统计量，后验均值是决策准则下的决策目标
- PROB-001 大数定律：固定参数下的观测序列是 Bernoulli 型序列，似然的频率解释由大数定律的收敛承载，见公式 $\ref{eq:prob001-wlln-limit}$
- PROB-003 中心极限定理：Bernstein-von Mises 定理给后验分布的渐近正态性，CLT 是其核心组件，见公式 $\ref{eq:prob003-clt-limit}$
- ORD-002 完全格：似然具有单调似然比性质时，后验随观测单调，后验间的序关系与假设空间的序结构相容，跨子仓参照

## 历史脉络 {#history}

- 1763 年 Bayes 完成 An Essay towards solving a Problem in the Doctrine of Chances，Price 于 1764 年整理遗作发表，逆概率推理首次系统提出
- 1774 年 Laplace 独立发展逆概率方法，并应用于天文参数估计
- 1812 年 Laplace 在 Théorie analytique des probabilités 中系统化贝叶斯方法
- 1920s Bernstein 与 von Mises 的工作给出后验分布的渐近正态性，即 Bernstein-von Mises 定理
- 1937 年 de Finetti 建立主观概率的协调性基础
- 1961 年 Raiffa 与 Schlaifer 系统化共轭先验与贝叶斯决策理论

## 工程注意事项 {#engineering-notes}

应用贝叶斯更新时需验证四件事。

1. 先验可归一性：不当先验的后验可能仍可归一，但须验证似然与先验的积分有限，验证结果须留痕
2. 弱证据敏感性：证据数量少或信息量低时，后验由先验主导，结论对先验选取敏感，先验选择须显式声明并评估敏感性
3. 证据相关性：条件独立假设不成立时，朴素似然乘积高估相关证据的信息量，导致过度自信，使用前须评估证据间相关性
4. 证据因子数值稳定性：连续参数情形边际似然是积分，数值计算须防下溢；证据因子同时是模型比较的归一化基础

## 参考文献 {#references}

- Bayes, T. (1763). An Essay towards solving a Problem in the Doctrine of Chances. Phil. Trans. R. Soc., 53, 370-418
- Laplace, P.S. (1812). Théorie analytique des probabilités. Courcier
- de Finetti, B. (1937). La prévision: ses lois logiques, ses sources mathématiques. Annales de l'Institut Henri Poincaré, 7, 1-68
- Raiffa, H. & Schlaifer, R. (1961). Applied Statistical Decision Theory. MIT Press
- Wikipedia "Bayesian inference" 条目
