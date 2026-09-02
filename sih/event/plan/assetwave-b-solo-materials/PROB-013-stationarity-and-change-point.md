# PROB-013 平稳性与变点检测

状态：草稿，carrierwave 波产出，资产回锚载体条目。

## 定义 {#definition}

时序的分布是否随时间移动，由平稳性与变点刻画。严平稳要求有限维分布平移不变；变点 $\tau$ 使 $\{X_t\}$ 在 $\tau$ 前后分布相异，见公式 \ref{eq:prob013-cp}。

<a id="eq:prob013-cp"></a>
$$X_t \sim F_1,\ t \le \tau; \qquad X_t \sim F_2,\ t > \tau, \quad F_1 \ne F_2$$

定理一，CUSUM 统计量的误报控制。平稳期累积和 $S_t = \max(0, S_{t-1} + X_t - b)$ 具有漂移可控的游走结构，阈值 $h$ 下的平均运行长度随 $h$ 指数增长，误报率可预标定。

证明。平稳期 $X_t - b$ 负均值，$S_t$ 是向下漂移的随机游走的反射上确界，其上升越过 $h$ 的事件被大偏差界 $\Pr \le e^{-2 b h / \sigma^2}$ 控制，承 PROB-004。

定理二，变点存在的功效。前后均值差 $\delta \ne 0$ 时，检测统计量在 $\tau$ 后的期望以 $|\delta|\,\sqrt{n}$ 增长，功效随窗口增长趋于一。

证明。$\tau$ 后 $E[S_t] \approx |\delta| (t - \tau)$，方差按 $\sigma^2 (t-\tau)$ 增长，标准化后偏移 $|\delta| \sqrt{t-\tau}/\sigma \to \infty$，由 PROB-001 的收敛语义功效趋一。

## 公理条件 {#axioms}

- 公理一，基线先验：平稳基线须在监察前确立，变点是相对基线的偏离
- 公理二，微兆语义：$\tau$ 前的分布移位是可检对象，检出的只是移位信号不是结论
- 公理三，窗口对价：窗口长则基线稳但响应迟，窗口短则相反，选择是显式对价

## 哲学桥接 {#philosophy-bridge}

- 承接命题：应层应变两相的应几，准入依据为 assetwave-a 登记面锚 08-on-settle.md:5 与得一裁 m-carrierwave 终签
- 哲学命题：原文「应本体失稳后，治理从「安住」走向「应变」」，见 08-on-settle 正本 L5，应几是应变之未形；变点前后的分布移位即未形之变的数学形态，微兆检测是应几的可计算载体
- 形式化：滑动窗口频率、线性回归斜率、偏离基线的异常检测三类触发共享本条的统计骨架，差异只在统计量与窗口
- 本条只给移位检测的形态，不立治理结论

## 应用 {#application}

- 应几微兆四触发：频率触发、趋势触发、异常触发共用平稳基线与变点统计量
- G5 度量的平稳性检验：规则数时序的平稳判定承本条，ADF 类检验是其实现
- 边界：嵌入向量聚类的语义距离不在本条概率范围内，其统计检验部分才属本条

## 与其他概念的关系 {#relations}

- PROB-001 大数定律：频率稳定与功效趋一的依据
- PROB-004 大偏差原理：误报的指数界
- PROB-012 相关与回归：趋势触发中斜率统计量的母体

## 历史脉络 {#history}

- 1954 年 Page 提出 CUSUM，1959 年 Roberts 提出 EWMA
- ADF 单位根检验承 Dickey 与 Fuller 1979，平稳性判定成为时序标准前置

## 工程注意事项 {#engineering-notes}

1. 基线漂移与变点须区分，基线本身缓慢移动时应使用滚动基线并声明
2. 误报率与检测延迟的权衡由阈值 $h$ 显式承担，参数变更须重标定
3. 微兆信号是视图告警，处置归人节点，不自动触发治理动作

## 参考文献 {#references}

- Page, E. S. (1954). Continuous Inspection Schemes. Biometrika, 41, 100-115
- Dickey, D. A. & Fuller, W. A. (1979). Distribution of the Estimators for Autoregressive Time Series with a Unit Root. Journal of the American Statistical Association, 74, 427-431
