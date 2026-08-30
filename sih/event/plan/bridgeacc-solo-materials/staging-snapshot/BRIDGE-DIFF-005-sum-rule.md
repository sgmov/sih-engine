---
entry: DIFF-005-sum-rule.md
agent: MiniServer-12581-210806
anchors:
  - pro: PRO-05 法一 顺因
    source: sih-philosophy/emanation/proodos/06-on-canon.md:49
    quote: "法一：顺因。尊重因果方向，介入点在前端。"
  - pro: PRO-07 鉴·客观
    source: sih-philosophy/emanation/proodos/07-on-assay.md:78
    quote: "客观：不预设立场，不急于下判断"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---
## 哲学桥接 {#philosophy-bridge}

- 借鉴源：和法则 $(g_1 + g_2 + \cdots + g_n)' = g_1' + g_2' + \cdots + g_n'$ 与导数概念同期诞生，Newton 流数术、Leibniz 记号，其证明依赖极限的线性性 $\lim_{h\to 0}[\Delta g + \Delta h]/h = \lim \Delta g/h + \lim \Delta h/h$，是 DIFF-007 线性性定理的有限和特例。
- 哲学命题：PRO-05 法一 顺因，锚句「法一：顺因。尊重因果方向，介入点在前端。」；PRO-07 鉴·客观，锚句「客观：不预设立场，不急于下判断。」
- 形式化：和法则是数学层面「顺因」的最纯净实例：总系统 $f = g_1 + g_2 + \cdots + g_n$ 的变化率不由总系统的宏观行为「投射」决定，而由各 $g_i$ 各自的局部因果链，各自的流数 / 偏导，独立决定，再以加法合并为总导数。介入点在各 $g_i$ 的内部因果结构，前端，不是 $f$ 整体，后端，：$f'(x) = \sum g_i'(x)$ 不是「对 $f$ 求导得到 $\sum g_i'(x)$」，而是「对各 $g_i$ 求导再求和」因果方向从 $g_i$ 流至 $f$，逆方向，先得 $f'$ 再分解，即失稳。条目工程映射「多轮对抗审阅中，总体攻击得分的变化率可以分解为各攻击维度得分变化率之和」即此结构：总体退化速率 $\dot{Q} = \sum_i \dot{Q}_i$ 不要求先测整体 $Q$ 再分解，后端、不可分，而是各维度 $Q_i$ 独立度量后相加，前端、可分，损失率为零：这是「客观」作为机械事实的体现：求和操作对 $g_i$ 的具体形态无预设，$g_i$ 连续、$g_i$ 不连续、$g_i$ 不可导均不影响规则形式，只影响规则是否适用。和法则失效，$g_i$ 不可导或无穷级数缺一致收敛，即「不客观」非「应然不合」，是「实然不达」，对治路径是回到 LIM-007 epsilon-delta 检验各 $g_i$ 的可导性，不通过整体 $f$ 的行为反推。司衡治理类比：审阅多维度的独立打分校准，前端，比依赖总分反推，后端，更稳；多 facetor 各自独立度量后聚合，顺因，vs 先看共识再回溯，逆因，的元层差别，决定了审查结构是否会在 $n$ 增大时退化。
