---
entry: ENTRY-TOP-007.md
agent: entry-topo-agent
anchors:
  - pro: PRO-08 应而不藏
    source: sih-philosophy/emanation/proodos/08-on-settle.md:228
    quote: "应辨：齐物论「应而不藏」+ 应帝王「用心若镜」"
  - pro: PRO-02 道一
    source: sih-philosophy/emanation/proodos/02-on-first-tao.md:80
    quote: "收敛不是自发的，它需要外部的、系统的、持续的治理力量"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---

# TOP-007 连续映射

状态：草稿，哲学到工程桥梁条目。

## 定义 {#definition}

连续映射（Continuous Map）是保持开集结构的拓扑空间间映射，是把分析学连续性语言一般化的拓扑基本概念。

设 $(X, \tau_X)$ 与 $(Y, \tau_Y)$ 是拓扑空间，$f: X \to Y$ 是映射。若对 $Y$ 中任意开集 $V$，原像 $f^{-1}(V)$ 是 $X$ 中的开集，即任意开集的原像为开集，则称 $f$ 是连续映射。

等价刻画。下列条件等价。

- 开集原像刻画：任意开集的原像为开集
- 闭集原像刻画：任意闭集的原像为闭集
- 邻域刻画：对任意 $x \in X$ 与 $f(x)$ 的任意邻域 $V$，存在 $x$ 的邻域 $U$ 使 $f(U) \subseteq V$

度量空间刻画。设 $(X, d_X)$ 与 $(Y, d_Y)$ 是度量空间，则上述条件等价于 epsilon-delta 刻画：对任意 $x \in X$ 与任意 $\varepsilon > 0$，存在 $\delta > 0$ 使对所有满足 $d_X(x', x) < \delta$ 的 $x' \in X$ 都有 $d_Y(f(x'), f(x)) < \varepsilon$。在度量空间中进一步等价于序列刻画：$x_n \to x$ 蕴含 $f(x_n) \to f(x)$，即保持收敛序列。

等价性的适用范围如实标注：在度量空间中，开集、闭集、epsilon-delta 与序列刻画互相等价；在一般拓扑空间中，开集原像与闭集原像刻画互推，序列刻画弱于连续，仅在度量空间等第一可数空间中与连续等价。

连续映射的基本性质。

- 复合封闭：若 $f: X \to Y$ 与 $g: Y \to Z$ 均连续，则 $g \circ f: X \to Z$ 连续
- 坐标刻画：对积拓扑下的积空间 $\prod_i X_i$，映射 $f: X \to \prod_i X_i$ 连续当且仅当每个坐标函数 $\pi_i \circ f$ 连续
- 图像闭：若 $Y$ 是 Hausdorff 空间，连续映射 $f: X \to Y$ 的图像 $\{(x, f(x)) \mid x \in X\}$ 在 $X \times Y$ 中闭

连续与一致连续。一致连续是强于逐点连续的条件：设 $f: X \to Y$ 是度量空间间映射，若对任意 $\varepsilon > 0$ 存在 $\delta > 0$ 使对所有满足 $d_X(x, x') < \delta$ 的 $x, x' \in X$ 都有 $d_Y(f(x), f(x')) < \varepsilon$，则称 $f$ 一致连续。逐点连续不蕴含一致连续：$f(x) = x^2$ 在 $\mathbb{R}$ 上连续但不一致连续。紧性提供桥梁：从紧度量空间到度量空间的连续函数必一致连续，此为 TOP-006 紧性的推论。

## 公理条件 {#axioms}

连续映射概念的成立依赖以下公理条件。

- 拓扑空间：定义域 $(X, \tau_X)$ 与值域 $(Y, \tau_Y)$ 均为拓扑空间
- 映射：$f: X \to Y$ 是函数
- 度量设定：使用 epsilon-delta 刻画或序列刻画时，两侧须配备诱导相应拓扑的度量，即 TOP-004
- 一致连续设定：两侧为度量空间，紧上必一致的推论进一步要求定义域紧，即 TOP-006

开集原像定义是最一般的，只要求两侧拓扑结构。各等价刻画依赖额外结构：epsilon-delta 刻画要求度量，序列刻画在第一可数空间才与连续完全等价。刻画的适用范围以额外结构为界。

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：连续性由分析学 epsilon-delta 语言推广为拓扑空间原像定义，Fréchet 1906 年给出度量空间版本，Hausdorff 1914 年给出一般拓扑空间版本
- 哲学命题：PRO-08 应而不藏，应辨之道家根为用心若镜，PRO-02 道一即发散自然、收敛必为
- 形式化：PRO-08 应辨的道家根是用心若镜，镜映物不失真。不失真的数学刻画即连续性：状态到应对的映射连续，则状态的小扰动只引起应对的小变化，应对无突变失真，连续性是对应对状态忠实的前提条件。PRO-02 道一确立收敛不是自发的，需要外部的、系统的、持续的治理力量，连续映射保持收敛序列：$x_n \to x$ 蕴含 $f(x_n) \to f(x)$，治理迭代的收敛经连续响应传递，为应层动作的收敛随状态的收敛提供结构条件

## 在 facet 的应用 {#facet-application}

- 应用场景：facet verdict 与闸阈值的稳定性
- 形式化：判据读数到 verdict 的响应建模为映射，verdict 映射连续时，读数的小扰动只引起 verdict 的有界波动，闸阈值判定无突变；判据空间经 TOP-006 验证为紧时，连续在紧域上保证一致连续，阈值敏感度可一致有界
- 借鉴方向：verdict 映射的连续性验证是阶段 5 公理验证的稳定性条件，判据空间的紧性是阈值一致有界的结构前置
- 边界：verdict 空间的拓扑或度量的具体构造、连续性的验证由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- TOP-004 度量空间：epsilon-delta 刻画与序列刻画的载体，度量结构是各刻画互推的公共地基
- TOP-005 完备度量空间：TOP-001 压缩映射的定义域是完备度量空间，压缩映射必连续
- TOP-006 紧集：连续映射把紧集映到紧集，极值定理与紧上必一致都以紧性为前提
- TOP-001 Banach 不动点：压缩映射满足 $d(T(x), T(y)) \leq q \cdot d(x, y)$ 且 $q < 1$，是强于连续的条件，不动点迭代同时依赖连续与压缩
- LIM-008 连续函数：分析侧对应概念，函数在函数域上的 epsilon-delta 连续定义，calculus 子仓，跨子仓引用

## 历史脉络 {#history}

- 1821 年 Cauchy 在 Cours d'analyse 中给出连续性的 epsilon-delta 语言
- 1872 年 Weierstrass 在柏林大学讲学中细化连续性的 epsilon-delta 定义，分析语言严密化
- 1906 年 Fréchet 在 Sur quelques points du calcul fonctionnel 中将连续性推广到度量空间
- 1914 年 Hausdorff 在 Grundzüge der Mengenlehre 中给出一一般拓扑空间的开集原像定义
- 1920s 后一般拓扑公理化，连续映射理论成为代数拓扑与函数空间论的基本工具

## 工程注意事项 {#engineering-notes}

应用连续性时需验证三件事。

1. 刻画适用范围是否满足：序列刻画只在度量空间等第一可数空间与连续等价，一般拓扑空间保持收敛序列只是序列连续，弱于连续
2. 逐点与一致是否混淆：逐点连续不蕴含一致连续，一致连续结论须验证定义域紧或给直接的 delta 论证
3. 拓扑是否配套：原像定义依赖两侧拓扑，度量诱导的拓扑下度量刻画与拓扑刻画等价

连续性的工程局限。连续性只保证开结构的局部保持，若需定量变化率如 Lipschitz 常数或连续模，须用一致连续或 Lipschitz 连续补足，TOP-001 压缩条件是更强的条件。

## 参考文献 {#references}

- Cauchy, A.L. (1821). Cours d'analyse de l'École Polytechnique
- Rudin, W. (1976). Principles of Mathematical Analysis, 3rd ed. McGraw-Hill
- Munkres, J.R. (2000). Topology, 2nd ed. Prentice Hall
- Willard, S.E. (2002). General Topology. Addison-Wesley
- Wikipedia "Continuous function" 条目
