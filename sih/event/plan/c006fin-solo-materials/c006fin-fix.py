#!/usr/bin/env python3
"""c006fin-solo 126 行 146 对改写映射。用法:
  python3 c006fin-fix.py <math仓根> check   # 全对表，零落笔
  python3 c006fin-fix.py <math仓根> apply   # 逐行替换，任一失配即中止
"""
import sys, pathlib

R = [
# ALG-001
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 23, [("恰有 $n$ 个根（按重数计），这是","恰有 $n$ 个根，按重数计，这是")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 25, [("特征值的全体（按重数）称为","特征值的全体，按重数，称为")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 38, [("Hermite 矩阵（实对称 / 复共轭对称）必有","Hermite 矩阵，实对称或复共轭对称，必有")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 40, [("需进一步分析（非负矩阵的稳态即此情形，对应 Perron-Frobenius 定理 ALG-007）。","需进一步分析，非负矩阵的稳态即此情形，对应 Perron-Frobenius 定理 ALG-007。")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 48, [("包含 $A$ 的全部特征值（复数域 $\\mathbb{C}$ 是代数闭域，实数域 $\\mathbb{R}$ 不闭，实矩阵的特征值可能非实）","包含 $A$ 的全部特征值，复数域 $\\mathbb{C}$ 是代数闭域，实数域 $\\mathbb{R}$ 不闭，实矩阵的特征值可能非实")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 49, [("在 $F$ 上有解（复数域上自动满足）","在 $F$ 上有解，复数域上自动满足")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 51, [("线性无关特征向量（即 A 可对角化）","线性无关特征向量，即 A 可对角化"),("Hermite 矩阵（实对称）正交对角化是更精细结论","实对称 Hermite 矩阵的正交对角化是更精细结论")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 53, [("要求 A = A*（共轭转置等于自身），实对称矩阵是特例","要求 A = A* 即共轭转置等于自身，实对称矩阵是特例")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 55, [("要求 A ≥ 0（逐元非负）且 A 不可约","要求 A ≥ 0，逐元非负，且 A 不可约")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 60, [("PRO-07 鉴（多 voter 投票矩阵），PRO-02 道一（发散自然，收敛必为）","PRO-07 鉴即多 voter 投票矩阵，PRO-02 道一即发散自然、收敛必为")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 63, [("是行随机矩阵（每行和 $= 1$，元素非负）","是行随机矩阵，每行和 $= 1$ 且元素非负"),("恰为 $1$（Perron-Frobenius 定理 ALG-007）","恰为 $1$，由 Perron-Frobenius 定理 ALG-007")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 74, [("行随机矩阵 $P$（$P \\geq 0,\\ P \\mathbf{1} = \\mathbf{1}$）","行随机矩阵 $P$ 即 $P \\geq 0,\\ P \\mathbf{1} = \\mathbf{1}$"),("Perron 向量（对应特征值 $1$）。","Perron 向量，对应特征值 $1$。")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 79, [("转移矩阵 P（行随机、元素非负）。","转移矩阵 P，行随机且元素非负。")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 82, [("左特征向量（PROB-001 大数定律给出频域收敛保证）","左特征向量，PROB-001 大数定律给出频域收敛保证")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 83, [("矩阵（Perron-Frobenius + mixing time 估计）","矩阵，Perron-Frobenius + mixing time 估计")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 89, [("（候选设计，尚无组件实装）","，候选设计，尚无组件实装")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 90, [("$T v^* = v^*$（即特征值 $1$ 对应的特征向量）。","$T v^* = v^*$，即特征值 $1$ 对应的特征向量。")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 98, [("收敛到唯一不动点（Banach 不动点定理），收敛率","收敛到唯一不动点，由 Banach 不动点定理，收敛率")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 118, [("矩阵诱导的压缩算子（谱半径 $< 1$）的不动点","矩阵诱导的谱半径 $< 1$ 的压缩算子的不动点")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 121, [("上的单调映射（若 $A \\geq 0$），转移矩阵","上的单调映射，若 $A \\geq 0$，转移矩阵")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 138, [("限定为对称矩阵（特征值全实）","限定为特征值全实的对称矩阵")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 140, [("需额外分析（稳态分布 vs 周期振荡）","需额外分析，稳态分布 vs 周期振荡")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 144, [("高维矩阵（n > 10⁴）的特征值","高维矩阵 n > 10⁴ 的特征值")]),
("algebra/entries/ALG-001-matrix-and-eigenvalue.md", 145, [("病态矩阵（条件数大）的特征值","条件数大的病态矩阵的特征值")]),
# ORD-001
("order/entries/ORD-001-partially-ordered-set.md", 17, [("偏序允许不可比元素（x 与 y 既不 x ≤ y 也不 y ≤ x）。全序（链）是偏序的特殊情况","偏序允许不可比元素，即 x 与 y 既不 x ≤ y 也不 y ≤ x。全序即链，是偏序的特殊情况")]),
("order/entries/ORD-001-partially-ordered-set.md", 24, [("不存在严格更大（小）元的元素","前者不存在严格更大元，后者不存在严格更小元")]),
("order/entries/ORD-001-partially-ordered-set.md", 25, [("大于（小于）所有其他元素的元素","前者大于所有其他元素，后者小于所有其他元素")]),
("order/entries/ORD-001-partially-ordered-set.md", 27, [("边表示覆盖关系（x < y 且无 z 使 x < z < y）。","边表示覆盖关系，即 x < y 且无 z 使 x < z < y。")]),
("order/entries/ORD-001-partially-ordered-set.md", 57, [("PRO-02 道一（发散自然，收敛必为）","PRO-02 道一，发散自然、收敛必为")]),
("order/entries/ORD-001-partially-ordered-set.md", 64, [("每个链（可数或不可数）有上界","每个链，可数或不可数，有上界")]),
("order/entries/ORD-001-partially-ordered-set.md", 68, [("在定理上对偶（任意上确界 ↔ 下确界，极大元 ↔ 极小元，最大元 ↔ 最小元）；","在定理上对偶，即任意上确界 ↔ 下确界、极大元 ↔ 极小元、最大元 ↔ 最小元；")]),
("order/entries/ORD-001-partially-ordered-set.md", 72, [("Mirsky 定理（1971，对偶）。","Mirsky 定理，1971 年，对偶结论。")]),
("order/entries/ORD-001-partially-ordered-set.md", 80, [("（候选设计，尚无组件实装）","，候选设计，尚无组件实装")]),
("order/entries/ORD-001-partially-ordered-set.md", 89, [("v₁ 比 v₂ 更弱（更保守）","v₁ 比 v₂ 更弱也更保守")]),
("order/entries/ORD-001-partially-ordered-set.md", 96, [("（候选设计，尚无组件实装）","，候选设计，尚无组件实装")]),
("order/entries/ORD-001-partially-ordered-set.md", 130, [("偏序是否真偏序（非全序）：","偏序是否真偏序而非全序：")]),
("order/entries/ORD-001-partially-ordered-set.md", 133, [("不保证子集有上下确界（完全格是强化）。","不保证子集有上下确界，完全格才是强化。")]),
# ORD-002
("order/entries/ORD-002-complete-lattice.md", 17, [("- 任意子集有上确界（由任意子集有上确界可推出任意子集有下确界，对偶性）","- 任意子集有上确界，由任意子集有上确界可推出任意子集有下确界，对偶性")]),
("order/entries/ORD-002-complete-lattice.md", 18, [("- 任意子集有下确界（对偶）","- 任意子集有下确界，对偶成立")]),
("order/entries/ORD-002-complete-lattice.md", 20, [("完全格要求任意子集（可能无穷）有上确界与下确界","完全格要求任意子集，可能无穷，有上确界与下确界")]),
("order/entries/ORD-002-complete-lattice.md", 26, [("偏序关系（自反 / 反对称 / 传递）","偏序关系，自反 / 反对称 / 传递")]),
("order/entries/ORD-002-complete-lattice.md", 40, [("PRO-02 道一（发散自然，收敛必为）","PRO-02 道一，发散自然、收敛必为")]),
("order/entries/ORD-002-complete-lattice.md", 55, [("- TOP-003 Knaster-Tarski 不动点（拓扑侧）：同一对象的拓扑视角","- TOP-003 Knaster-Tarski 不动点的拓扑侧：同一对象的拓扑视角")]),
("order/entries/ORD-002-complete-lattice.md", 73, [("常以有限近似（如有限偏序）实现","常以有限近似实现，如有限偏序")]),
# ORD-003
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 21, [("（Kleene 构造）；","，即 Kleene 构造；"),("须沿序数超限迭代（Cousot 与 Cousot 1979）。","须沿序数超限迭代，见 Cousot 与 Cousot 1979。")]),
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 29, [("像必须在 $L$ 内（即 $f: L \\to L$）","像必须在 $L$ 内，即 $f: L \\to L$")]),
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 38, [("PRO-02 道一（发散自然，收敛必为）","PRO-02 道一，发散自然、收敛必为")]),
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 47, [("$A = \\{x \\in L \\mid f(x) \\leq x\\}$（$f$ 的前不动点）","$A = \\{x \\in L \\mid f(x) \\leq x\\}$，即 $f$ 的前不动点集")]),
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 48, [("$B = \\{x \\in L \\mid f(x) \\geq x\\}$（$f$ 的后不动点）","$B = \\{x \\in L \\mid f(x) \\geq x\\}$，即 $f$ 的后不动点集")]),
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 59, [("对偶地 $\\vee B$（$B$ 为后不动点集）是最大不动点","对偶地 $\\vee B$，$B$ 为后不动点集，是最大不动点")]),
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 71, [("Knaster-Tarski 在偏序集上不成立（任意子集未必有上下确界）","Knaster-Tarski 在偏序集上不成立，任意子集未必有上下确界")]),
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 73, [("给出构造性不动点（要求压缩），Knaster-Tarski 在完全格上给出存在性不动点（要求单调）。","给出构造性不动点，要求压缩，Knaster-Tarski 在完全格上给出存在性不动点，要求单调。")]),
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 74, [("域上给存在性不动点（要求连续），Knaster-Tarski 在完全格上给存在性不动点（要求单调）。","域上给存在性不动点，要求连续，Knaster-Tarski 在完全格上给存在性不动点，要求单调。")]),
("order/entries/ORD-003-knaster-tarski-fixed-point.md", 75, [("- TOP-003 Knaster-Tarski 不动点（拓扑侧）：同一对象的两种视角","- TOP-003 Knaster-Tarski 不动点的拓扑侧：同一对象的两种视角")]),
# ORD-004
("order/entries/ORD-004-monotone-operator.md", 1, [("# ORD-004 Monotone Operator（单调算子）","# ORD-004 Monotone Operator 单调算子")]),
("order/entries/ORD-004-monotone-operator.md", 13, [("称为单调算子（monotone operator，又称 isotone 或 order-preserving map）。","称为单调算子，英文 monotone operator，又称 isotone 或 order-preserving map。")]),
("order/entries/ORD-004-monotone-operator.md", 40, [("单调算子作为压缩映射（Banach 不动点）输入的额外条件","单调算子作为 Banach 不动点的压缩映射输入的额外条件")]),
("order/entries/ORD-004-monotone-operator.md", 48, [("PRO-02 道一（发散自然，收敛必为）","PRO-02 道一，发散自然、收敛必为")]),
("order/entries/ORD-004-monotone-operator.md", 49, [("只保证不动点存在（完全格上）不保证可达","只在完全格上保证不动点存在而不保证可达")]),
("order/entries/ORD-004-monotone-operator.md", 57, [("$f(x_0) \\leq f(x_1) = x_2$（由单调）蕴含","$f(x_0) \\leq f(x_1) = x_2$，由单调，蕴含")]),
("order/entries/ORD-004-monotone-operator.md", 69, [("（候选设计，尚无组件实装）","，候选设计，尚无组件实装")]),
("order/entries/ORD-004-monotone-operator.md", 70, [("迭代收敛到最大（小）不动点","迭代收敛到最大或最小不动点")]),
("order/entries/ORD-004-monotone-operator.md", 102, [("完备度量空间 + 压缩（蕴含严格单调）上给不动点","完备度量空间 + 压缩上给不动点，压缩蕴含严格单调")]),
("order/entries/ORD-004-monotone-operator.md", 112, [("扩展（变分不等式）","扩展到变分不等式")]),
# PROB-001
("probability/entries/PROB-001-law-of-large-numbers.md", 7, [("大数定律（Law of Large Numbers，弱形式 WLLN）描述","大数定律即 Law of Large Numbers 弱形式 WLLN，描述")]),
("probability/entries/PROB-001-law-of-large-numbers.md", 24, [("有限（即 $\\mathbb{E}[|X_i|] < \\infty$）","有限，即 $\\mathbb{E}[|X_i|] < \\infty$")]),
("probability/entries/PROB-001-law-of-large-numbers.md", 32, [("PRO-07 鉴（多主体协作打破自证循环）","PRO-07 鉴即多主体协作打破自证循环")]),
("probability/entries/PROB-001-law-of-large-numbers.md", 37, [("Bernoulli 大数定律的证明思路（Chebyshev 不等式方法）。","Bernoulli 大数定律的证明思路是 Chebyshev 不等式方法。")]),
("probability/entries/PROB-001-law-of-large-numbers.md", 49, [("具体实验配置（厂数 / shot 数 / 命题数）由","具体实验配置，厂数 / shot 数 / 命题数，由")]),
("probability/entries/PROB-001-law-of-large-numbers.md", 61, [("在 Ars Conjectandi（猜测术）中证明","在 Ars Conjectandi 即猜测术中证明")]),
# PROB-002
("probability/entries/PROB-002-strong-law-of-large-numbers.md", 7, [("强大数定律（Strong Law of Large Numbers，SLLN）描述","强大数定律即 Strong Law of Large Numbers，缩写 SLLN，描述")]),
("probability/entries/PROB-002-strong-law-of-large-numbers.md", 17, [("几乎必然（almost surely，又称 almost everywhere）收敛到期望","几乎必然，almost surely 又称 almost everywhere，收敛到期望")]),
("probability/entries/PROB-002-strong-law-of-large-numbers.md", 39, [("有限（即 $\\mathbb{E}[|X_i|] < \\infty$）","有限，即 $\\mathbb{E}[|X_i|] < \\infty$")]),
("probability/entries/PROB-002-strong-law-of-large-numbers.md", 48, [("PRO-07 鉴（多主体协作打破自证循环）","PRO-07 鉴即多主体协作打破自证循环")]),
("probability/entries/PROB-002-strong-law-of-large-numbers.md", 53, [("核心思路（Etemadi 1981 给出的简化版本）。","核心思路取 Etemadi 1981 给出的简化版本。")]),
("probability/entries/PROB-002-strong-law-of-large-numbers.md", 67, [("具体实验配置（厂数 / shot 数 / 命题数）由","具体实验配置，厂数 / shot 数 / 命题数，由")]),
("probability/entries/PROB-002-strong-law-of-large-numbers.md", 72, [("分布形态（正态近似）","分布形态即正态近似")]),
("probability/entries/PROB-002-strong-law-of-large-numbers.md", 75, [("monotone convergence theorem（极限与期望交换）","monotone convergence theorem，即极限与期望交换")]),
("probability/entries/PROB-002-strong-law-of-large-numbers.md", 79, [("Bernoulli 序列的 SLLN（Borel 强大数定律）","Bernoulli 序列的 SLLN 即 Borel 强大数定律")]),
# PROB-003
("probability/entries/PROB-003-central-limit-theorem.md", 7, [("中心极限定理（Central Limit Theorem，CLT）描述","中心极限定理即 Central Limit Theorem，缩写 CLT，描述")]),
("probability/entries/PROB-003-central-limit-theorem.md", 19, [("收敛到具体分布（正态）。","收敛到具体分布即正态。")]),
("probability/entries/PROB-003-central-limit-theorem.md", 44, [("PRO-07 鉴（多主体协作的统计稳定性）","PRO-07 鉴即多主体协作的统计稳定性")]),
("probability/entries/PROB-003-central-limit-theorem.md", 49, [("CLT 的证明思路（特征函数方法）。","CLT 的证明思路是特征函数方法。")]),
("probability/entries/PROB-003-central-limit-theorem.md", 79, [("（候选设计，尚无组件实装）","，候选设计，尚无组件实装")]),
("probability/entries/PROB-003-central-limit-theorem.md", 93, [("指数衰减速率（Chernoff 界）","指数衰减速率，如 Chernoff 界")]),
("probability/entries/PROB-003-central-limit-theorem.md", 112, [("方差无穷（重尾分布），CLT 不成立","方差无穷即重尾分布，CLT 不成立")]),
# TOP-001
("topology/entries/TOP-001-banach-fixed-point.md", 26, [("依赖 Banach 空间的完备性（保证 Cauchy 列收敛）与压缩性（保证迭代序列是 Cauchy 列）","依赖 Banach 空间的完备性即保证 Cauchy 列收敛，与压缩性即保证迭代序列是 Cauchy 列")]),
("topology/entries/TOP-001-banach-fixed-point.md", 31, [("PRO-08 应而不藏（应层留痕的收敛性），PRO-07 鉴（检验的可重复性）","PRO-08 应而不藏即应层留痕的收敛性，PRO-07 鉴即检验的可重复性")]),
("topology/entries/TOP-001-banach-fixed-point.md", 43, [("给存在性不动点（要求连续），Banach 在完备度量空间给构造性不动点（要求压缩），两定理","给存在性不动点，要求连续，Banach 在完备度量空间给构造性不动点，要求压缩，两定理")]),
# TOP-002
("topology/entries/TOP-002-brouwer-fixed-point.md", 19, [("保证像集与原集的拓扑关系（保持近邻）","保证像集与原集的拓扑关系即保持近邻")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 25, [("是有界闭集（Heine-Borel 定理在 $\\mathbb{R}^n$ 中的形式）","是有界闭集，此即 Heine-Borel 定理在 $\\mathbb{R}^n$ 中的形式")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 29, [("像必须在 $D$ 内（即 $f: D \\to D$）","像必须在 $D$ 内，即 $f: D \\to D$")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 31, [("不要求 f 是压缩映射（与 Banach 不同），不要求单调性（与 Knaster-Tarski 不同）","不要求 f 是压缩映射，与 Banach 不同，不要求单调性，与 Knaster-Tarski 不同")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 36, [("PRO-07 鉴（多主体协作打破自证循环），PRO-02 道一（发散自然，收敛必为）","PRO-07 鉴即多主体协作打破自证循环，PRO-02 道一即发散自然、收敛必为")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 43, [("反证法（Brouwer 1910 原始证明）。","反证法是 Brouwer 1910 的原始证明。"),("不存在连续对径点映射（Stereographic projection 反证），推出","不存在连续对径点映射，由 Stereographic projection 反证，推出")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 47, [("组合方法（Sperner 引理 1928）。","组合方法是 Sperner 引理 1928。"),("利用 Sperner 引理（每个有 Sperner 标记的三角剖分有全标记单纯形）证明","利用 Sperner 引理，即每个有 Sperner 标记的三角剖分有全标记单纯形，证明")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 60, [("给存在性不动点（要求连续）。Banach 给出收敛速度","给存在性不动点，要求连续。Banach 给出收敛速度")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 61, [("- TOP-003 Knaster-Tarski 不动点（拓扑侧）：","- TOP-003 Knaster-Tarski 不动点的拓扑侧："),("存在性不动点（要求单调），Brouwer","存在性不动点，要求单调，Brouwer"),("给存在性不动点（要求连续）。两者","给存在性不动点，要求连续。两者")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 74, [("均衡理论（纳什均衡存在性证明）的核心工具","均衡理论即纳什均衡存在性证明的核心工具")]),
("topology/entries/TOP-002-brouwer-fixed-point.md", 85, [("需结合迭代算法（Picard 迭代 / Newton 法等）。","需结合迭代算法，如 Picard 迭代 / Newton 法等。")]),
# TOP-003
("topology/entries/TOP-003-knaster-tarski-mirror.md", 1, [("# TOP-003 Knaster-Tarski 不动点（拓扑侧镜像）","# TOP-003 Knaster-Tarski 不动点拓扑侧镜像")]),
("topology/entries/TOP-003-knaster-tarski-mirror.md", 19, [("须沿序数超限迭代方至稳定（Cousot 与 Cousot 1979）。","须沿序数超限迭代方至稳定，见 Cousot 与 Cousot 1979。")]),
("topology/entries/TOP-003-knaster-tarski-mirror.md", 36, [("Lawvere-Tierney 拓扑（1970 年代拓扑斯理论）给出","Lawvere-Tierney 拓扑，1970 年代拓扑斯理论，给出")]),
("topology/entries/TOP-003-knaster-tarski-mirror.md", 41, [("PRO-02 道一（发散自然，收敛必为）","PRO-02 道一，发散自然、收敛必为")]),
("topology/entries/TOP-003-knaster-tarski-mirror.md", 50, [("（dcpo 的有向并存在）。证","，dcpo 的有向并存在。证")]),
("topology/entries/TOP-003-knaster-tarski-mirror.md", 57, [("只需有向子集有上确界（不必任意子集）","只需有向子集有上确界，不必任意子集")]),
("topology/entries/TOP-003-knaster-tarski-mirror.md", 91, [("- ORD-003 Knaster-Tarski 不动点（序论侧）：","- ORD-003 Knaster-Tarski 不动点的序论侧：")]),
("topology/entries/TOP-003-knaster-tarski-mirror.md", 94, [("存在性不动点（要求 Scott 连续）。两者不同空间不同条件","存在性不动点，要求 Scott 连续。两者不同空间不同条件")]),
("topology/entries/TOP-003-knaster-tarski-mirror.md", 95, [("存在性不动点（要求 Scott 连续）。两者都是存在性版本","存在性不动点，要求 Scott 连续。两者都是存在性版本")]),
# TOP-004
("topology/entries/TOP-004-metric-space.md", 23, [("$d(x, y) = |x - y|$（标准度量）","$d(x, y) = |x - y|$ 即标准度量")]),
("topology/entries/TOP-004-metric-space.md", 52, [("PRO-02 道一（发散自然，收敛必为），PRO-07 鉴（多主体距离的可量化）","PRO-02 道一即发散自然、收敛必为，PRO-07 鉴即多主体距离的可量化")]),
("topology/entries/TOP-004-metric-space.md", 69, [("（候选设计，尚无组件实装）","，候选设计，尚无组件实装")]),
("topology/entries/TOP-004-metric-space.md", 91, [("不要求度量结构（只用拓扑）","不要求度量结构，只用拓扑")]),
("topology/entries/TOP-004-metric-space.md", 98, [("在 Sur quelques points du calcul fonctionnel（博士论文）中引入","在博士论文 Sur quelques points du calcul fonctionnel 中引入")]),
("topology/entries/TOP-004-metric-space.md", 113, [("不刻画多点的拓扑关系（如连通性、紧性）。","不刻画多点的拓扑关系，如连通性、紧性。")]),
# TOP-005
("topology/entries/TOP-005-complete-metric-space.md", 19, [("每个有界无穷子集有聚点（在某些空间等价于完备性）","每个有界无穷子集有聚点，在某些空间等价于完备性")]),
("topology/entries/TOP-005-complete-metric-space.md", 25, [("完备 + 范数 = Banach 空间（赋范空间 + 完备性）","完备 + 范数 = Banach 空间，即赋范空间 + 完备性")]),
("topology/entries/TOP-005-complete-metric-space.md", 32, [("是度量空间（TOP-004 的全部公理，见三角不等式 $\\ref{eq:top004-triangle-ineq}$）","是度量空间，含 TOP-004 的全部公理，见三角不等式 $\\ref{eq:top004-triangle-ineq}$")]),
("topology/entries/TOP-005-complete-metric-space.md", 46, [("PRO-02 道一（发散自然，收敛必为），PRO-08 应而不藏（应层留痕序列的极限可达）","PRO-02 道一即发散自然、收敛必为，PRO-08 应而不藏即应层留痕序列的极限可达")]),
("topology/entries/TOP-005-complete-metric-space.md", 55, [("$x^* \\in \\bar{B}_m$（$\\bar{B}_m$ 闭）","$x^* \\in \\bar{B}_m$，由 $\\bar{B}_m$ 闭")]),
("topology/entries/TOP-005-complete-metric-space.md", 57, [("是压缩映射（$q < 1$）","是压缩映射，$q < 1$，")]),
("topology/entries/TOP-005-complete-metric-space.md", 77, [("（候选设计，尚无组件实装）","，候选设计，尚无组件实装")]),
("topology/entries/TOP-005-complete-metric-space.md", 99, [("度量空间是本概念的弱化（不完备允许）","度量空间是本概念的弱化，允许不完备")]),
("topology/entries/TOP-005-complete-metric-space.md", 107, [("在 Sur quelques points du calcul fonctionnel（博士论文）中给出","在博士论文 Sur quelques points du calcul fonctionnel 中给出")]),
("topology/entries/TOP-005-complete-metric-space.md", 116, [("$d$ 必须满足度量公理（见 TOP-004 三角不等式 $\\ref{eq:top004-triangle-ineq}$）","$d$ 必须满足度量公理，见 TOP-004 三角不等式 $\\ref{eq:top004-triangle-ineq}$")]),
("topology/entries/TOP-005-complete-metric-space.md", 120, [("不保证其他收敛模式（如弱收敛、依概率收敛、几乎处处收敛）。","不保证其他收敛模式，如弱收敛、依概率收敛、几乎处处收敛。"),("需结合具体数学工具（PROB-* 范畴）。","需结合具体数学工具，即 PROB-* 范畴。")]),
]

def main():
    root = pathlib.Path(sys.argv[1]); mode = sys.argv[2]
    assert mode in ("check","apply")
    files = {}
    for rel, ln, pairs in R:
        p = root/rel
        if p not in files: files[p] = p.read_text(encoding="utf-8").splitlines()
        line = files[p][ln-1]
        for old,new in pairs:
            if line.count(old) != 1:
                print(f"失配 {rel}:{ln} [{old[:30]}...] count={line.count(old)}"); sys.exit(1)
        for old,new in pairs:
            files[p][ln-1] = files[p][ln-1].replace(old,new)
    if mode=="apply":
        for p, lines in files.items():
            p.write_text("\n".join(lines)+"\n", encoding="utf-8")
        print(f"落笔 {len(files)} 文件 {sum(len(x[2]) for x in R)} 对")
    else:
        print(f"对表通过 {len(files)} 文件 {sum(len(x[2]) for x in R)} 对，零落笔")

if __name__=="__main__": main()
