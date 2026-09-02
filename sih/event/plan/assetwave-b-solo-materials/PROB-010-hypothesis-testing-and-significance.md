# PROB-010 假设检验与显著性

状态：草稿，carrierwave 波产出，资产回锚载体条目。

## 定义 {#definition}

假设检验给「机械判定与偶然波动的界线」一个概率承载：在原假设 $H_0$ 与备择假设之间，用检验统计量 $T$ 的观测值 $t_{obs}$ 对照其在 $H_0$ 下的分布作判定。p 值定义为 $H_0$ 成立时出现不弱于观测的稀有度，见公式 \ref{eq:prob010-pvalue}。

<a id="eq:prob010-pvalue"></a>
$$p = \Pr{}_{H_0}(T \ge t_{obs})$$

显著性水平 $\alpha$ 在察看数据之前固定，$p \le \alpha$ 则拒绝 $H_0$。

定理一，p 值标准均匀。$T$ 在 $H_0$ 下分布连续时，$p \sim U(0,1)$。

证明。对 $u \in (0,1)$，$\Pr(p \le u) = \Pr(T \ge F^{-1}(1-u)) = 1 - F(F^{-1}(1-u)) = u$，故 p 值在 $H_0$ 下无偏向任何区间的趋势，阈值 $\alpha$ 的含义即误报率上界。

定理二，似然比最优。Neyman-Pearson 引理：在给定水平 $\alpha$ 的全部检验中，似然比检验的功效最大。

证明。设优势区域 $S = \{L_1/L_0 \ge k\}$，对任意同水平检验的示性函数 $\rho$，在 $S$ 上 $L_1 \ge k L_0$ 使 $E_1\[\rho\] \le E_1\[1_S\] + c(E_0\[1_S\] - E_0\[\rho\])$ 的逐点不等式成立，积分即得功效不等式。

## 公理条件 {#axioms}

- 公理一，水平先验：$\alpha$ 固定于察看数据之前，事后择界使误报率失去控制
- 公理二，拒绝语义：拒绝只断言观测在 $H_0$ 下稀有，不断言因果，不断言 $H_0$ 为真时的概率
- 公理三，可重复：同一数据同一水平给出同一判定，判定是 $p$ 与 $\alpha$ 的确定性比较

## 哲学桥接 {#philosophy-bridge}

- 承接命题：鉴层检验的机械可校验性，准入依据为 assetwave-a 登记面锚 07-on-assay.md:61 与得一裁 m-carrierwave 终签
- 哲学命题：原文「镜的特点是纯粹映照」，见 07-on-assay 正本 L61，镜映照而不投射；检验统计量映照数据与规约的偏离，判定交给 p 与水平的确定性比较，不投射结论
- 形式化：误报率上界即水平 $\alpha$ 的频率语义，p 值的均匀性定理保证阈值含义可复算
- 本条只给检验的概率形态，不立治理结论

## 应用 {#application}

- meter 与 facet 的席位基线漂移告警：以漂移指标为统计量、以既往账本分位为水平，告警判定机械可复算
- 信任评分升级路径的显著性门槛：L2 至 L1 升级要求效果达显著性，本条给出该要求的精确含义
- 边界：多重比较场景须水平分配校正，样本量与功效的权衡属实验设计，不属本条目

## 与其他概念的关系 {#relations}

- PROB-003 中心极限定理：检验统计量渐近正态的来源
- PROB-004 大偏差原理：稀有事件尾部概率的精细刻画
- PROB-002 强大数定律：覆盖与误报率的频率语义基础

## 历史脉络 {#history}

- 1933 年 Neyman 与 Pearson 奠定最优检验理论，Fisher 的 p 值传统与其在 20 世纪后半叶合流
- 1970 年代以来多重比较与效应量报告成为实务标准

## 工程注意事项 {#engineering-notes}

1. 多重检验不校正则整体误报率失控，族错误率须显式分配
2. p 值不是效应大小，显著不等于重要，报告须附效应量与区间
3. 水平事后调整属择界，判定结果不可复算

## 参考文献 {#references}

- Neyman, J. & Pearson, E. S. (1933). On the Problem of the Most Efficient Tests of Statistical Hypotheses. Philosophical Transactions of the Royal Society A, 231, 289-337
- Lehmann, E. L. & Romano, J. P. (2005). Testing Statistical Hypotheses. Springer, 3rd ed.
