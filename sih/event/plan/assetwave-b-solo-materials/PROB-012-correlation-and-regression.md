# PROB-012 相关与回归

状态：草稿，carrierwave 波产出，资产回锚载体条目。

## 定义 {#definition}

两个度量联合波动时，Pearson 相关系数 $\rho$ 给线性共变的强度，线性回归给单向近似，见公式 \ref{eq:prob012-rho}。

<a id="eq:prob012-rho"></a>
$$\rho = \frac{\mathrm{Cov}(X,Y)}{\sigma_X \sigma_Y}, \qquad y = a + b x + \varepsilon$$

定理一，界。Cauchy-Schwarz 不等式给 $|\rho| \le 1$，等号当且仅当 $Y$ 是 $X$ 的严格线性函数。

证明。对任意实 $t$，$0 \le \mathrm{Var}(tX + Y) = t^2 \sigma_X^2 + 2t\,\mathrm{Cov}(X,Y) + \sigma_Y^2$，二次型非负的判别式条件即 $\mathrm{Cov}^2 \le \sigma_X^2 \sigma_Y^2$。

定理二，相关与回归的等价。简单回归斜率 $b = \rho\, s_y / s_x$，判定系数 $R^2 = \rho^2$，最小二乘解存在且唯一当 $s_x > 0$。

证明。残差平方和对 $(a,b)$ 是凸二次型，一阶条件给出正规方程，解出 $b = \mathrm{Cov}(x,y)/s_x^2$，代入得 $R^2 = \mathrm{SSR}/\mathrm{SST} = \rho^2$。

## 公理条件 {#axioms}

- 公理一，相关非因果：$\rho \ne 0$ 不断言作用方向，混淆与反向均可产生同值相关
- 公理二，线性限定：$\rho$ 只刻画线性共变，非线性关系可并存 $\rho = 0$
- 公理三，趋势信号语义：度量管道产出是相关关系，因果声明需受控实验

## 哲学桥接 {#philosophy-bridge}

- 承接命题：法顺势的度量语义，准入依据为 assetwave-a 登记面锚 06-on-canon.md:185 与得一裁 m-carrierwave 终签
- 哲学命题：原文「治理力度可由松到紧，不可由紧到松」，见 06-on-canon 正本 L185，顺势调节何时达到严格；度量趋势是顺势的观测面，相关系数是趋势信号的可计算形态
- 形式化：规则密度与治理开销这类双时序的联动以 $\rho$ 报告，正负号与幅度即趋势方向与强度
- 本条只给共变的代数形态，不立治理结论

## 应用 {#application}

- G1 度量的趋势信号：rule_density 与 governance_overhead_ratio 的 Pearson 相关即其 trend_signal 字段
- 温故检索与批结果的相关审计：检索命中与批产出的关联以相关刻画
- 边界：时间序列自相关使普通相关高估，须先白化，归 PROB-013

## 与其他概念的关系 {#relations}

- PROB-003 中心极限定理：样本相关的渐近推断
- PROB-013 平稳性与变点：时序场景的共变前置条件
- APP-009 信息洪流注意力预算：注意力稀释与产出退化的共变刻画

## 历史脉络 {#history}

- 1885 年 Galton 与 Pearson 确立相关与回归框架
- 最小二乘远溯 Gauss 与 Legendre 的轨道测量

## 工程注意事项 {#engineering-notes}

1. 混淆变量不作声明则相关被误读为因果，报告须附变量清单
2. 异常点可主导 $\rho$，稳健相关须并列报告
3. 复杂度 $O(n)$ 单遍可算，实时管道可用

## 参考文献 {#references}

- Pearson, K. (1900). On the Correlation of Characters not Quantitatively Measurable. Philosophical Transactions of the Royal Society A, 195, 1-47
- Wasserman, L. (2004). All of Statistics. Springer, ch. 14
