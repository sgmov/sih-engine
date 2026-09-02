# PROB-011 估计与置信区间

状态：草稿，carrierwave 波产出，资产回锚载体条目。

## 定义 {#definition}

参数未知时，估计量 $\hat\theta = \theta(X_1, \ldots, X_n)$ 给出点近似，置信区间 $C_\alpha$ 给出带保证的近似宽度，见公式 \ref{eq:prob011-coverage}。

<a id="eq:prob011-coverage"></a>
$$\Pr{}_{\theta}\big(\theta \in C_\alpha(X)\big) \ge 1 - \alpha \quad \text{for all } \theta$$

定理一，渐近区间构造。$\hat\theta$ 渐近正态且标准误 $\mathrm{se}$ 相合时，$\hat\theta \pm z_{1-\alpha/2}\,\mathrm{se}$ 的覆盖概率趋于 $1-\alpha$。

证明。由中心极限定理 $\frac{\hat\theta - \theta}{\mathrm{se}} \Rightarrow N(0,1)$，区间事件等价于标准正态落在分位之间，Slutsky 引理把相合标准误代入即得覆盖收敛。

定理二，覆盖是频率陈述。覆盖概率对重复抽样收敛到置信水平，对单次区间不断言 $\theta$ 落入的概率。

证明。示性函数 $1\{\theta \in C_\alpha\}$ 是独立同分布试验上的伯努利变量，强大数定律使其样本频率几乎必然收敛于覆盖概率，承 PROB-002。

## 公理条件 {#axioms}

- 公理一，覆盖先于数据：置信水平是构造性质，不是数据实现后的性质
- 公理二，区间宽度即间隙量化：宽度是近似误差的显式代价，不因区间存在而宣称点精确
- 公理三，变换相容：参数的一一变换下区间随变换走，保持覆盖

## 哲学桥接 {#philosophy-bridge}

- 承接命题：道四规约与实现必有间隙，准入依据为 assetwave-a 登记面锚 05-on-fourth-tao.md:15 与得一裁 m-carrierwave 终签
- 哲学命题：原文「> 道四：规约与实现必有间隙。」，见 05-on-fourth-tao 正本 L15，估计永远近似；置信区间把度量间隙显式化为可计算宽度，间隙不被消除而被报告
- 形式化：间隙的数学形态即覆盖概率与区间宽度的对价关系，精度声明须以区间形态出具
- 本条只给估计的区间形态，不立治理结论

## 应用 {#application}

- 信任评分权重校准：0.30 与 0.25 等经验权重的校准须以区间报告，升级路径的统计显著性门槛承 PROB-010
- 度量管道精度声明：采集误差以区间形态出具，符合度量产出是趋势信号不声称因果的边界
- 边界：贝叶斯可信区间与本条的频率区间语义不同，混用须显式声明

## 与其他概念的关系 {#relations}

- PROB-003 中心极限定理：区间构造的正态来源
- PROB-002 强大数定律：覆盖频率收敛的依据
- PROB-010 假设检验：区间与检验的对偶，水平 $\alpha$ 的区间与同水平检验等价

## 历史脉络 {#history}

- 1930 年代 Neyman 确立置信区间的覆盖语义
- 二十世纪末自助法区间把构造推广到难解分布

## 工程注意事项 {#engineering-notes}

1. 区间宽度随样本量以 $n^{-1/2}$ 收缩，样本不足时区间宽是诚实不是缺陷
2. 相合性破坏则渐近覆盖失效，标准误的估计须随条目复核
3. 单次区间的概率化误读是常见病，汇报语言须用覆盖频率表述

## 参考文献 {#references}

- Neyman, J. (1937). Outline of a Theory of Statistical Estimation Based on the Classical Theory of Probability. Philosophical Transactions of the Royal Society A, 236, 333-380
- Wasserman, L. (2004). All of Statistics. Springer, ch. 8
