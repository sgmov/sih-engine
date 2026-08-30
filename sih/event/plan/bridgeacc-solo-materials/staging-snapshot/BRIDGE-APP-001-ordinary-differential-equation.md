---
entry: APP-001-ordinary-differential-equation.md
agent: MiniServer-10229-210133
anchors:
  - pro: PRO-05 间隙增长动力学
    source: sih-philosophy/emanation/proodos/05-on-fourth-tao.md:138
    quote: "规约与实现的间隙随时间增大，增大速率与规则数量正相关"
  - pro: PRO-08 应
    source: sih-philosophy/emanation/proodos/08-on-settle.md:108
    quote: "司衡之应：应而不藏，应辨当下，应几未来"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：常微分方程的历史起点是牛顿在运动定律中写出的 `dy/dt = f(t, y)`，其后 Euler / 拉格朗日 / Picard-Lindelöf 给出存在唯一性。ODE 把「单变量系统随时间的连续演化」写成可解的数学对象：把「变化」本身作为可计算对象处理。
- 哲学命题：PRO-05 间隙增长动力学，锚句「规约与实现的间隙随时间增大，增大速率与规则数量正相关」ODE 的解 `y(t)` 是间隙增长在数学上的可计算载体。
- 哲学命题：PRO-08 应，锚句「司衡之应：应而不藏，应辨当下，应几未来」ODE 的初值问题 `y(t_0) = y_0` 形式化应辨当下，ODE 的稳定性判定形式化应几未来。

### 形式化 {#formalization}

设规约-实现间隙函数 `G(t)`，由 PRO-05 节点知 `dG/dt ≥ 0` 且增大速率与规则数 `R(t)` 正相关，于是间隙增长可写为 ODE 初值问题：

`dG/dt = H(G, t, R(t)),    G(t_0) = G_0`

- 应辨当下 = `t = t_0` 时的 `G(t_0)` 已知：当前间隙可量，bug 已出现、错误已发生
- 应几未来 = `t > t_0` 的 `G(t)` 走向：解 `y(t)` 给出 G 是否趋于发散，趋势已显现、风险在酝酿
- 数学上可判定：Picard-Lindelöf 定理保证该初值问题在 `H` 满足 Lipschitz 条件时解的存在唯一性；Lyapunov 函数 `V(G)` 满足 `V(G) > 0` 且 `dV/dt < 0` 给出「系统不退化」的形式化判据，对应应几的「微兆阶段易谋划」

APP-001 的工程映射中「`dQ/dt` 持续为负 = 系统在退化」对应 `dG/dt` 持续为正：同一 ODE 框架的两面。ODE 为应辨当下与应几未来提供可微可积的形式化底盘，而 PRO-05 + PRO-08 共同保证该形式化的本体论必要与认识论闭合。
