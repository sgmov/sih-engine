# PROB-014 期望信息增益

状态：草稿，carrierwave 波产出，资产回锚载体条目。

## 定义 {#definition}

序贯提问的价值由期望信息增益量化。离散目标的熵 $H(Y) = -\sum_y p(y) \log p(y)$，问题 $Q$ 的增益为条件熵的下降，见公式 \ref{eq:prob014-ig}。

<a id="eq:prob014-ig"></a>
$$IG(Q) = H(Y) - \mathbb{E}\big[H(Y \mid Q)\big]$$

定理一，非负性。任意问题的 $IG(Q) \ge 0$，等号当且仅当 $Y$ 与 $Q$ 独立。

证明。$H(Y) - H(Y \mid Q) = I(Y;Q) \ge 0$ 是互信息的非负性，由 $\log$ 的凹性与 Jensen 不等式对 $\sum q \log (q/p)$ 逐项应用即得。

定理二，贪心序贯的近似界。候选问题的增益函数在子模性下，每步取最大增益的贪心选择经 $k$ 步达到最优 $k$ 步增益的 $1 - 1/e$ 倍。

证明。子模与单调使价值函数满足 Nemhauser 递归：最优 $k$ 步集与贪心集的差距每步至少衰减因子 $(1-1/k)$，$k$ 步后 $\mathrm{OPT} - f(G_k) \le (1-1/k)^k \mathrm{OPT} \le e^{-1} \mathrm{OPT}$。

## 公理条件 {#axioms}

- 公理一，增益以回答为条件：期望对回答分布取，先验偏则增益偏，先验来源须登记
- 公理二，观察者效应：提问本身改变被测意图的分布，增益是条件量不是固有量
- 公理三，序贯可见：已获回答并入后验，后续增益按更新后的分布计算

## 哲学桥接 {#philosophy-bridge}

- 承接命题：道二意图先于代码与应辨，准入依据为 assetwave-a 登记面锚 03-on-second-tao.md:15 与 08-on-settle.md:5 与得一裁 m-carrierwave 终签
- 哲学命题：原文「> 道二：意图先于代码。」，见 03-on-second-tao 正本 L15，意图先于代码；追问在意图未形时进行，按增益排序即以最少提问锁定最多意图
- 形式化：四道追问的固定顺序升级为增益排序，跳道记录即先验更新的登记
- 本条只给问题排序的形态，不立治理结论

## 应用 {#application}

- 追问引擎的问题排序：四道追问按期望增益排序，分支触发的追加道以剩余增益判定
- elicit 信号的优先级：多信号并发时按信息增益排告警次序
- 边界：问题文本的措辞与人类作答质量不属概率范围

## 与其他概念的关系 {#relations}

- PROB-008 信息内容与可证伪性：同族，彼条给信息内容与证伪的关系，本条给序贯获取的排序
- PROB-005 Bayesian 更新：回答并 入后验的机制
- ORD-021 依赖 DAG 与拓扑序：追问分支的依赖结构

## 历史脉络 {#history}

- 1948 年 Shannon 奠定熵与互信息
- 1978 年 Nemhauser 等给子模最大化的贪心界，序贯选择理论成型

## 工程注意事项 {#engineering-notes}

1. 子模性破坏时贪心界失效，选项间强互补的场景须改用穷举或声明近似失效
2. 高增益问题可能是高负担问题，代价加权是工程层扩展，须显式登记
3. 先验偏置直接扭曲排序，先验的来源与版本须随材料留痕

## 参考文献 {#references}

- Shannon, C. E. (1948). A Mathematical Theory of Communication. Bell System Technical Journal, 27, 379-423
- Nemhauser, G. L., Wolsey, L. A. & Fisher, M. L. (1978). An Analysis of Approximations for Maximizing Submodular Set Functions. Mathematical Programming, 14, 265-294
