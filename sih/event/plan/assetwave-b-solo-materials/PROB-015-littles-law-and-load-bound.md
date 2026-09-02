# PROB-015 Little 定律与负载界

状态：草稿，carrierwave 波产出，资产回锚载体条目。

## 定义 {#definition}

稳态队列的三量恒等：在途数 $L$ 等于到达率 $\lambda$ 乘平均逗留时间 $W$，见公式 \ref{eq:prob015-little}。

<a id="eq:prob015-little"></a>
$$L = \lambda W$$

定理一，Little 定律。稳态且初始与终末在途有限的样本路径上，时平均 $\hat L$、到达率 $\hat\lambda$ 与平均逗留 $\hat W$ 满足 $\hat L = \hat\lambda \hat W$，极限下即 $L = \lambda W$。

证明。记 $[0,T]$ 内完成数 $N$，累积在途面积 $\int_0^T L_t\,dt = \sum_{i \le N} W_i$。两边除以 $T$：左边趋于 $L$，右边 $= (N/T)(\sum W_i / N)$，两个因子分别趋于 $\lambda$ 与 $W$，乘积即得。

定理二，稳定性必要条件。服务率 $\mu$ 单服务台时，利用率 $
ho = \lambda/\mu < 1$ 是稳态在途有限的必要条件；$
ho \ge 1$ 时 $L$ 无界增长。

证明。$
ho < 1$ 缺失则到达功量超过服务功量，单位时间净积压 $(\lambda - \mu) t$ 线性发散，$L$ 无稳态；必要性由 $L = \lambda W$ 与 $W \ge 1/\mu$ 联立即 $L \ge
ho / (1 - 0)$ 的下界发散。

## 公理条件 {#axioms}

- 公理一，稳态前提：恒等式在稳态下成立，瞬态数字不满足且不可外推
- 公理二，并发有度：同一持有者同时在途任务数有上限，封顶值即 $
ho$ 控制的工程化
- 公理三，可测性：三量皆可从链上事件计数得出，不需透视队列内部

## 哲学桥接 {#philosophy-bridge}

- 承接命题：法有度的并发封顶，准入依据为 assetwave-a 登记面锚 06-on-canon.md:185 与得一裁 m-carrierwave 终签
- 哲学命题：原文「治理力度可由松到紧，不可由紧到松」，见 06-on-canon 正本 L185，持而盈之不如其已；负载封顶是「不如其已」的量化形态，$
ho$ 逼近一即盈满
- 形式化：编组并联度上限与租约并发封顶的参数以 $\rho$ 余量表述，容量规划由恒等式机械换算
- 本条只给负载的恒等形态，不立治理结论

## 应用 {#application}

- 编组并联度上限：并联队形的在途子代理数与单件时长的乘积给出容量需求，反解可得安全并联度
- 租约并发封顶参数：每会话同时在途租约数的上限以余量表述
- 边界：等待时间分布与公平性不属恒等式范围，须引入排队模型

## 与其他概念的关系 {#relations}

- PROB-001 与 PROB-002 大数定律：时平均收敛的依据
- PROB-003 中心极限定理：在途数的涨落刻画
- ORD-020 全序资源分配与死锁自由：并发秩序的序侧约束，本条给量侧约束

## 历史脉络 {#history}

- 1961 年 Little 证明恒等式，1975 年 Stidham 给样本路径证明
- 二十世纪末成为容量规划的标准工具

## 工程注意事项 {#engineering-notes}

1. 稳态检验是前置义务，承 PROB-013 的平稳判定
2. 封顶参数只控 $
ho$，等待分布对到达模式敏感，峰谷比大时须留更大余量
3. 链上计数窗口的选择影响 $\hat\lambda$，窗口须远长于单件时长

## 参考文献 {#references}

- Little, J. D. C. (1961). A Proof for the Queuing Formula L = lambda W. Operations Research, 9, 383-387
- Stidham, S. (1974). A Last Word on L = lambda W. Operations Research, 22, 417-421
