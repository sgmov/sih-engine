# mathpipe-a3 推导档：gauge ga-2 期票清偿（PROB-003 置信带与 PROB-005 贝叶斯语义）

> 承接：mathpipe-a3-solo 批件一载体定位与推导档落 sih-math/docs/，本档为推导内容在批内材料的暂存副本，sih-math/docs/ 正位落盘待 sih-math/docs/ 锁释放后拷入。
> 载体：sih-math/probability/entries/PROB-003-central-limit-theorem.md 与 PROB-005-bayesian-updating.md，均已在册（mapping.md 概率近似与信念更新节）。

## 一、adoption 维占比的统计语义

adoption 维读数为租约会话正身双腿占比：窗口内 identity_hash 非空会话数 k ÷ issued 会话数 n。每笔 issued 会话是否带正身双腿视为独立伯努利试验，占比即样本比例 p̂ = k/n。ga-1 只报点估计 p̂，无不确定度刻画；ga-2 补两件统计摘要即置信带与后验均值，承 gspec-solo 期票登记。

## 二、PROB-003 置信带（CLT）

中心极限定理（PROB-003）断言独立同分布随机变量之和的标准化分布收敛到标准正态，见 PROB-003 条目公式 (eq:prob003-clt-limit)。样本比例 p̂ 的标准化形式依 CLT 渐近正态，95% 双侧置信带为

$$p̂ \pm z_{0.975} \sqrt{\frac{p̂(1-p̂)}{n}}$$

其中 z_{0.975} = 1.96 为标准正态 97.5% 分位。工程接线：

- 半宽 half = 1.96 × √(p̂(1-p̂)/n)
- lower = max(0, p̂ − half)，upper = min(1, p̂ + half)，收口到 [0,1] 界
- n 零时样本比例无定义，不出置信带（ask3 注入约束：n 零时不出置信带）
- 边界如实标注：Wald 区间在 p̂ 近 0 或 1 时半宽趋零，区间退化为点，属 CLT 正态近似的已知边界行为，本批如实接线不修饰

## 三、PROB-005 贝叶斯语义（后验均值）

贝叶斯更新（PROB-005）把先验信念与观测证据合成后验，见 PROB-005 条目公式 (eq:prob005-bayes)。adoption 占比 p 取 Beta-Bernoulli 共轭先验：先验 p ~ Beta(α, β)，观测 k 次成功（带正身）与 n−k 次失败后，后验 p | E ~ Beta(α+k, β+n−k)，见 PROB-005 条目共轭先验节。

先验参数显式声明（ask3 注入约束）：α=1, β=1 即 Beta(1,1) 均匀先验，后验均值为

$$\mathbb{E}[p \mid E] = \frac{\alpha + k}{\alpha + \beta + n} = \frac{k+1}{n+2}$$

即拉普拉斯续则。选择依据：均匀先验为最弱信息先验，弱证据下后验受先验牵引但不过度，n 增大时后验均值收敛到样本比例 p̂，承 PROB-005 边界节"证据充分时后验由似然主导"。n 零时无观测，后验均值 = α/(α+β) = 0.5 即纯先验，本批 n 零时不出后验均值字段与置信带一致，避免虚构统计摘要。

## 四、schema 增量扩展

ga-2 在 ga-1 七必填字段上只增不删，adoption 维可带两扩展字段：

- confidence_band：对象 {lower, upper}，零到一数值且 lower ≤ upper，承 PROB-003
- posterior_mean：零到一数值，承 PROB-005

convergence 与 mergeback 维不加扩展字段。ga-1 七字段读数回放兼容即守卫仍收。模型参数（α=1, β=1, z=1.96）入 adoption 维 inputs_digest 即证据标识集含会话账本哈希与先验参数与 z 值，可机械复算。

## 五、判变申报

ga-2 与 ga-1 的 adoption 维基础值 value = k/n 逐字节同值即基础值不回改，判变面为零；新增字段只加统计摘要不改基础值。判变清单见批结果档逐件申报或如实记零。
