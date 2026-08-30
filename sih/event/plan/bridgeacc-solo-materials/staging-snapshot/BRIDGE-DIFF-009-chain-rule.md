---
entry: DIFF-009-chain-rule.md
agent: MiniServer-12668-210822
anchors:
  - pro: PRO-09 元
    source: sih-philosophy/emanation/proodos/09-on-arche.md:58
    quote: "元层承认这一不完整性：元层不是\"完美的元约束\"，元层是\"承认自身不完美的约束\""
  - pro: PRO-08 应几
    source: sih-philosophy/emanation/proodos/08-on-settle.md:95
    quote: "应几：感知正在形成的微兆"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---
## 哲学桥接 {#philosophy-bridge}

- 借鉴源：链式法则自 Leibniz 17 世纪直觉表述 $dy/dx = (dy/du)(du/dx)$，Cauchy 19 世纪严格证明；多变量形式 $dz/dt = (\partial f/\partial x)(dx/dt) + (\partial f/\partial y)(dy/dt)$ 是神经网络反向传播算法的数学根基。
- 哲学命题：PRO-09 元，锚句「元层承认这一不完整性：元层不是"完美的元约束"，元层是"承认自身不完美的约束"」；PRO-08 应几，锚句「应几：感知正在形成的微兆，几 = 几微、征兆。未来形态：趋势已显现、风险在酝酿、机会在浮现。」
- 形式化：链式法则是「承认自身不完美的元约束」的数学标本：其形式 $dy/dx = f'(g(x))\cdot g'(x)$ 仅在每层函数在对应点可导时成立，规则自身不带「保证成立」承诺，而是把前置条件，每层可导，显式声明在应用语境中：任意一层不可导，规则即失效，无「大约成立」或「近似成立」的中间态。条目原文已显式给出这一限制：「若中间层函数在某点不可导，即使外层和内层都可导，复合函数在该点也可能不可导」。这是元层最纯粹的形态：不是绕过不完美，是把不完美写进规则签名，让验证者一眼看出规则何时停止生效，与 PRO-05 法「知止」的「每条规则都应能回答"这条规则何时停止生效？"」是同源要求。多变量形式 $\partial z/\partial t = \sum_i (\partial f/\partial x_i)(dx_i/dt)$ 把链式法则从单链推广为偏导和，每条 $x_i(t)$ 通道独立贡献 $dx_i/dt$ 至总变化率：这恰是「应几」的工程实现：上游微变量 $\{dx_i\}$ 是「正在形成的微兆」，PRO-08 锚句，$\partial f/\partial x_i$ 是该微兆被下游 $z$ 感知的灵敏度权重，总 $dz/dt$ 是多微兆的加权求和：比单链更能匹配工程现实的多变量耦合。条目工程映射「最终产出质量对初始意图的灵敏度是各层变换导数的乘积」即链式法则的元层结构：$\partial Q/\partial I = \prod_k (\partial L_k/\partial L_{k-1})$：若任何一层导数 $|L_k'|\ll 1$，如 $da/dI<0$ 的稀释率或某层语义坍缩，乘积的绝对值向零衰减，整体意图信号沿链消失，此即信息洪流在微分几何中的精确描述。司衡治理类比：治理层数 $n$ 增大时意图保真度 $\prod_k |L_k'|$ 单调下降，除非每层 $|L_k'|\ge 1$，治理深度并非越多越好：这是「知止」的数学证明：超过链长阈值后治理收益转负。
