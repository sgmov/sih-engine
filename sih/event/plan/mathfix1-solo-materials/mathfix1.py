#!/usr/bin/env python3
"""mathfix1 批一修订：9 条目逐处手术。每处 (file, old, new) 精确替换，失配即报错。"""
import pathlib, sys

R = pathlib.Path('/Users/moc/workspaces/SiHankor/worktrees/sih-math/mathfix1-solo')
E = {
 'TOP-001': R/'topology/entries/TOP-001-banach-fixed-point.md',
 'TOP-002': R/'topology/entries/TOP-002-brouwer-fixed-point.md',
 'TOP-003': R/'topology/entries/TOP-003-knaster-tarski-mirror.md',
 'TOP-004': R/'topology/entries/TOP-004-metric-space.md',
 'TOP-005': R/'topology/entries/TOP-005-complete-metric-space.md',
 'ORD-001': R/'order/entries/ORD-001-partially-ordered-set.md',
 'ORD-002': R/'order/entries/ORD-002-complete-lattice.md',
 'ORD-003': R/'order/entries/ORD-003-knaster-tarski-fixed-point.md',
 'ORD-004': R/'order/entries/ORD-004-monotone-operator.md',
}
STATUS_OLD = "> 状态：已建 / 哲学 → 工程桥梁条目"
STATUS_NEW = "状态：已建，哲学到工程桥梁条目。"

EDITS = {k: [] for k in E}

for k in E: EDITS[k].append((STATUS_OLD, STATUS_NEW))

# TOP-001
EDITS['TOP-001'] += [
 ("- TOP-002 Brouwer 不动点定理：Brouwer 在有限维空间给出不动点存在性（不要求压缩性），Banach 在完备度量空间给出构造性不动点（要求压缩性）。Brouwer 是 Banach 的一般化但 Banach 更具工程可操作性",
  "- TOP-002 Brouwer 不动点定理：Brouwer 在紧凸集上给存在性不动点（要求连续），Banach 在完备度量空间给构造性不动点（要求压缩），两定理条件互不蕴含；把 Brouwer 推广到无穷维紧凸集的是 Schauder 与 Tychonoff，不是反向"),
 ("- Crandall, M.G. & Rabinowitz, P.H. (1970). Bifurcation from simple eigenvalues. J. Funct. Anal., 8(2), 321-340\n", ""),
]

# TOP-002
EDITS['TOP-002'] += [
 ("- 哲学命题：PRO-07 鉴（多主体协作打破自证循环），PRO-12 范式（偏序链的极限）",
  "- 哲学命题：PRO-07 鉴（多主体协作打破自证循环），PRO-02 道一（发散自然，收敛必为）"),
 ("- 形式化：PRO-07 鉴层要求多主体协作收敛到共识。Brouwer 定理形式化鉴层收敛的存在性：若判据空间是紧凸集，判据更新是连续映射，则必存在稳定判据（不动点）使多主体协作必然收敛。PRO-12 范式要求治理动作收敛到稳定状态，Brouwer 给出范式收敛的存在性证明，区别于 Banach 的压缩条件与 Knaster-Tarski 的偏序条件",
  "- 形式化：PRO-07 鉴层要求多主体协作收敛到共识。Brouwer 定理给出该收敛的存在性条件：判据空间为紧凸集且判据更新连续时稳定判据存在，但只保证存在不保证迭代到达。PRO-02 道一确立收敛是治理的构成性要求且收敛需外部治理力量，本定理给出此类外部迭代下稳定态存在的一组数学条件，区别于 Banach 的压缩条件与 Knaster-Tarski 的偏序条件"),
 ("- PROB-002 SLLN：强大数定律的证明中常用 Schauder-Tychonoff 不动点（Brouwer 的无穷维推广）",
  "- PROB-002 SLLN：强大数定律的证明走截断加 Borel-Cantelli 路线，不经由不动点定理；Schauder-Tychonoff 是 Brouwer 到无穷维紧凸集的推广，属本条外延"),
 ("- 1886 年 Henri Poincaré 在 Poincaré-Birkhoff 定理中给出平面圆环情形",
  "- 1912 年 Henri Poincaré 提出平面圆环情形的最后一几何定理，1913 年由 Birkhoff 证明完成"),
]

# TOP-003
EDITS['TOP-003'] += [
 ("Knaster-Tarski 不动点定理在拓扑侧有独立表述。拓扑侧不要求格结构，只要求偏序集配合 Scott 拓扑或 Lawvere-Tierney 拓扑，使偏序的极限可由拓扑刻画。\n\n拓扑侧 Knaster-Tarski 定理（Tarski 1955 在同一文章中给出两个版本）。设 $(L, \\leq)$ 是完备偏序集（dcpo, directed-complete partial order），其上配以 Scott 拓扑。设 $f: L \\to L$ 是 Scott 连续映射（Scott continuous map），即对任意有向子集 $D \\subseteq L$，$f(\\vee D) = \\vee f(D)$。$f$ 有最大不动点与最小不动点。",
  "序结构与连续性结合的拓扑侧不动点定理，正确形态分三层。\n\n其一，Tarski 1955，完全格加单调：$(L, \\leq)$ 是完全格，$f: L \\to L$ 单调，则不动点集非空且构成完全格，最大与最小不动点都存在，此即 ORD-003 的内容。\n\n其二，Kleene 构造，含底元 dcpo 加 Scott 连续：设 $(L, \\leq)$ 是含底元 $\\bot$ 的完备偏序集（dcpo），$f: L \\to L$ 是 Scott 连续映射，即对任意有向子集 $D \\subseteq L$ 有 $f(\\vee D) = \\vee f(D)$，则最小不动点存在且 $\\mathrm{lfp}(f) = \\vee\\{f^n(\\bot) \\mid n \\geq 0\\}$。Scott 连续在 dcpo 上一般不保证最大不动点存在：反例取自然数集配离散序，任何有向子集皆单元集故为 dcpo，平移映射 $f(n) = n + 1$ 是 Scott 连续自映射且没有任何不动点。最大不动点的存在需额外结构。"),
 ("$$\\mathrm{gfp}(f) = \\vee\\{x \\in L \\mid f(x) \\geq x\\} = \\vee\\{f^n(\\top) \\mid n \\geq 0\\} \\label{eq:top003-greatest-fixed-point}$$",
  "$$\\mathrm{lfp}(f) = \\vee\\{f^n(\\bot) \\mid n \\geq 0\\} \\label{eq:top003-greatest-fixed-point}$$"),
 ("$$\\mathrm{lfp}(f) = \\wedge\\{x \\in L \\mid f(x) \\leq x\\} = \\vee\\{f^n(\\bot) \\mid n \\geq 0\\} \\label{eq:top003-least-fixed-point}$$\n\n拓扑侧与序理论侧的关系。dcpo + Scott 拓扑 + Scott 连续 = 完全格（见公式 $\\ref{eq:ord002-complete-lattice}$）+ 偏序（见公式 $\\ref{eq:ord001-antisymmetry}$）+ 单调（ORD-003）的拓扑强化。Scott 连续蕴含单调（连续映射保持极限，连续映射保持偏序），单调不蕴含 Scott 连续。拓扑侧是序侧的强化。\n\n完备偏序集（dcpo）配以 Scott 拓扑后是 sober 空间（sober space）的特例。完全格（ORD-002）配以 Alexandrov 拓扑后是更具体的拓扑空间。两种拓扑选择给出 Knaster-Tarski 定理的不同入口。",
  "其三，超限迭代，完全格加一般单调：$\\omega$ 次迭代未必到达不动点。反例取 $L$ 为第一不可数序数加其顶元，$f(\\alpha) = \\alpha + 1$，唯一不动点为顶元，而可数次迭代序列的上确界停在第一无限序数；须沿序数超限迭代方至稳定（Cousot 与 Cousot 1979）。\n\n拓扑侧与序理论侧的关系。完全格是 dcpo 的特例，Scott 连续强于单调但两假设互不蕴含：完全格上的单调算子不必保任何无穷有向并，dcpo 上的 Scott 连续映射只义务保有向并。Kleene 层以连续性换可构造性，超限迭代层是一般单调的构造化。连续 dcpo 配以 Scott 拓扑是 sober 空间，一般 dcpo 的 Scott 空间不必 sober。"),
 ("- 完备偏序集：$(L, \\leq)$ 是 dcpo，即每个有向子集 $D \\subseteq L$ 都有上确界 $\\vee D$\n- Scott 拓扑：开集是上集且对有向并封闭的子集\n- Scott 连续：$f: L \\to L$ 在 Scott 拓扑下连续，即对有向 $D$，$f(\\vee D) = \\vee f(D)$\n- 自映射：$f$ 的像必须在 $L$ 内（即 $f: L \\to L$）\n\n定理证明的核心是 Scott 连续保证单调与有向并交换，与完全格版本（ORD-003）相同构造可适用。完全格版本是 dcpo 版本的特例（完全格 $\\subset$ dcpo）。\n\nLawvere-Tierney 拓扑（1969）给出更一般的拓扑框架。完全 Heyting 代数配以 j-算子是更广义的拓扑侧 Knaster-Tarski 的承载结构。",
  "Kleene 层的公理条件四件。\n\n- 含底元 dcpo：$(L, \\leq)$ 是 dcpo 且有最小元 $\\bot$\n- Scott 连续：$f$ 保所有有向上确界\n- 自映射：$f: L \\to L$\n- 结论范围：只有最小不动点被保证，最大不动点不保证\n\nTarski 层的公理条件两件：完全格加单调。超限迭代层的公理条件与 Tarski 层相同，只是构造从 $\\omega$ 步放宽到任意序数步。\n\nLawvere-Tierney 拓扑（1970 年代拓扑斯理论）给出更一般的拓扑框架，完全 Heyting 代数配 j-算子是其格论形态。"),
 ("设 $L$ 是 dcpo，$f: L \\to L$ 是 Scott 连续。Scott 连续蕴含单调（因有向集 $\\{x, y\\}$ 的上确界是 $x \\vee y$，$f(x \\vee y) = f(x) \\vee f(y)$ 蕴含 $f(x) \\leq f(x \\vee y)$ 与 $f(y) \\leq f(x \\vee y)$）。",
  "设 $L$ 是含底元 dcpo，$f: L \\to L$ 是 Scott 连续。Scott 连续蕴含单调：对可比的 $x \\leq y$，$\\{x, y\\}$ 有向且上确界为 $y$，$f(y) = f(x \\vee y) = f(x) \\vee f(y)$ 蕴含 $f(x) \\leq f(y)$。"),
 ("- 由 $f$ Scott 连续，$f(p) = f(\\vee\\{f^n(\\bot)\\}) = \\vee\\{f^{n+1}(\\bot)\\} = \\vee\\{f^n(\\bot) \\mid n \\geq 1\\}$\n- 因 $\\bot = f^0(\\bot) \\in \\{f^n(\\bot) \\mid n \\geq 0\\}$，有向并满足 $\\bot \\leq f^n(\\bot)$ 蕴含 $\\{f^n(\\bot) \\mid n \\geq 1\\} \\subseteq \\{f^n(\\bot) \\mid n \\geq 0\\}$\n- 故 $f(p) = \\vee\\{f^n(\\bot) \\mid n \\geq 1\\} \\leq \\vee\\{f^n(\\bot) \\mid n \\geq 0\\} = p$\n- 又 $p \\in \\{f^n(\\bot) \\mid n \\geq 0\\}$ 蕴含 $f(p) \\geq p$（因 $f$ 单调且 $\\bot \\leq p$）\n- 合并 $f(p) = p$\n\n最大不动点从 $\\top$ 反向迭代类似证得。",
  "- 由 $f$ Scott 连续，$f(p) = f(\\vee\\{f^n(\\bot)\\}) = \\vee\\{f^{n+1}(\\bot)\\} = \\vee\\{f^n(\\bot) \\mid n \\geq 1\\}$\n- 显然 $\\vee\\{f^n(\\bot) \\mid n \\geq 1\\} \\leq \\vee\\{f^n(\\bot) \\mid n \\geq 0\\} = p$\n- 又 $p \\geq f^n(\\bot)$ 对一切 $n$ 成立，单调给出 $f(p) \\geq f^{n+1}(\\bot)$，即 $f(p) \\geq p$\n- 合并 $f(p) = p$。最大不动点无对偶结论，反例见定义节"),
 ("- 哲学命题：PRO-12 范式（偏序链的极限），PRO-13 链外补充（拓扑侧扩展）",
  "- 哲学命题：PRO-02 道一（发散自然，收敛必为）"),
 ("- 形式化：PRO-12 范式要求治理动作收敛到稳定状态。拓扑侧 Knaster-Tarski 给出范式收敛的拓扑刻画：若范式状态空间是 dcpo，范式更新是 Scott 连续映射，则必存在稳定范式（最大最小不动点，见公式 $\\ref{eq:top003-greatest-fixed-point}$ 与 $\\ref{eq:top003-least-fixed-point}$）使范式迭代到该点。拓扑强化保证迭代可由极限交换刻画。PRO-13 链外补充要求在范式链外补充新元素，拓扑侧的 Scott 拓扑允许链外元素以极限形式存在，为链外补充提供拓扑语义",
  "- 形式化：PRO-02 道一确立收敛是治理的构成性要求且收敛需外部治理力量。本条给出该要求在序结构上的实现条件分层：外部迭代若保有向并则最小稳定态可构造到达，见公式 $\\ref{eq:top003-greatest-fixed-point}$；若仅单调则须超限迭代；条件不满足时收敛无保证，治理须回查迭代设计而非空谈稳定"),
 ("- ORD-003 Knaster-Tarski 不动点（序论侧）：同一对象的两种视角，ORD-003 在完全格上只需单调，TOP-003 在 dcpo 上需 Scott 连续。序论侧是拓扑侧的特例\n- ORD-002 完全格：完全格是 dcpo 的特例（见公式 $\\ref{eq:ord002-complete-lattice}$），完全格 + 单调 $\\subset$ dcpo + Scott 连续",
  "- ORD-003 Knaster-Tarski 不动点（序论侧）：同一对象族的两个定理，ORD-003 在完全格上只需单调得最大最小两点，本条 Kleene 层在含底元 dcpo 上需 Scott 连续只得最小点，两假设互不蕴含\n- ORD-002 完全格：完全格是 dcpo 的特例（见公式 $\\ref{eq:ord002-complete-lattice}$），但完全格上的单调算子不必 Scott 连续，结构包含不推出性质包含"),
 ("- TOP-004 度量空间：度量空间配以序拓扑（order topology）可视为 dcpo 配以 Alexandroff 拓扑的特例\n", ""),
 ("- 1969 年 Lawvere 与 Tierney 给出 j-算子框架",
  "- 1970 年代 Lawvere 与 Tierney 在拓扑斯理论中给出 j-算子框架"),
]

# TOP-004
EDITS['TOP-004'] += [
 ("度量空间是拓扑空间（TOP-002）配以距离结构的特例。由度量 $d$ 诱导的拓扑以开球 $B(x, r) = \\{y \\in X \\mid d(x, y) < r\\}$ 为基生成。",
  "度量空间是拓扑空间配以相容距离结构的特例：由度量 $d$ 诱导的拓扑以开球 $B(x, r) = \\{y \\in X \\mid d(x, y) < r\\}$ 为基生成，仓内尚无独立拓扑空间条目，见 TOP-006 待建。"),
 ("是度量空间最核心的条件，保证距离具有线性可加性。",
  "是度量空间最核心的条件，保证距离具有次可加性。"),
 ("- 应用场景：assayer 多厂判据的差异量化",
  "- 应用场景：facet 多厂判据的差异量化"),
 ("范式（PRO-12）的迭代收敛需度量结构。\n\n- 应用场景：sih-engine 范式状态机的迭代",
  "治理状态的迭代收敛需度量结构。\n\n- 应用场景：治理状态机（候选设计，尚无组件实装）"),
 ("- 哲学命题：PRO-12 范式（度量 = 范式的距离化），PRO-07 鉴（多主体距离的可量化）",
  "- 哲学命题：PRO-02 道一（发散自然，收敛必为），PRO-07 鉴（多主体距离的可量化）"),
 ("- 形式化：PRO-12 范式要求治理动作收敛到稳定状态。度量空间为范式提供距离结构：范式状态配以度量 $d$，范式状态间的接近程度由 $d$ 量化，范式迭代的\"收敛\"在度量意义下有明确刻画。PRO-07 鉴层要求多主体协作打破自证循环，度量空间为多主体判据的距离提供量化工具，多主体判据的距离可计算、可比较、可设阈值",
  "- 形式化：PRO-02 道一确立收敛是治理的构成性要求，度量结构使收敛可判定：状态配以度量 $d$ 后，接近程度可量化，迭代是否趋于稳定有明确的数学刻画。PRO-07 鉴层要求多主体协作打破自证循环，度量空间为多主体判据的距离提供量化工具，多主体判据的距离可计算、可比较、可设阈值"),
 ("- ORD-001 偏序集：度量空间按距离 $d(x, y) \\leq 1$ 配以离散序后是偏序集（ORD-001，见公式 $\\ref{eq:ord001-antisymmetry}$）的特殊情况",
  "- ORD-001 偏序集：任何集合配离散序即偏序集（ORD-001，见公式 $\\ref{eq:ord001-antisymmetry}$），度量结构与序结构相互独立，两概念正交"),
]

# TOP-005
EDITS['TOP-005'] += [
 ("- Cauchy 列收敛：每个 Cauchy 列收敛到 $X$ 中点\n- 闭球嵌套引理（Cantor 1895）：一族非空闭球嵌套且半径趋于 0 的交非空\n- Bolzano-Weierstrass 性质：每个有界无穷子集有聚点（在某些空间等价于完备性）",
  "- Cauchy 列收敛：每个 Cauchy 列收敛到 $X$ 中点\n- 闭球嵌套引理（Cantor 1883）：一族非空闭球嵌套且半径趋于 0 的交非空\n- Bolzano-Weierstrass 性质：每个有界无穷子集有聚点（在某些空间等价于完备性）\n\n注意 Baire 性质是完备性的推论而非判据：完备度量空间必为 Baire 空间，但 Baire 空间不必完备，如局部紧豪斯多夫空间。",
  ),
 ("- 直接验证：每个 Cauchy 列显式给出极限\n- 闭球嵌套引理：每个闭球嵌套族有非空交\n- Baire 性质：可数稠密开集的交稠密（Baire 纲定理）\n- Banach 定理前提：若存在完备度量空间 $(X, d)$ 与压缩映射 $T: X \\to X$，则 $T$ 有不动点（TOP-001，见公式 $\\ref{eq:top001-contractive}$）",
  "- 直接验证：每个 Cauchy 列显式给出极限\n- 闭球嵌套引理：每个闭球嵌套族有非空交\n- Banach 定理前提：若存在完备度量空间 $(X, d)$ 与压缩映射 $T: X \\to X$，则 $T$ 有不动点（TOP-001，见公式 $\\ref{eq:top001-contractive}$）"),
 ("- 借鉴源：Georg Cantor 1895 年在 Grundlagen einer allgemeinen Mannigfaltigkeitslehre 中给出闭球嵌套引理，Maurice Fréchet 1906 年在 Sur quelques points du calcul fonctionnel 中给出度量空间完备性的现代定义",
  "- 借鉴源：Georg Cantor 1883 年在 Grundlagen einer allgemeinen Mannigfaltigkeitslehre 中给出闭球嵌套原理的实数版本，Maurice Fréchet 1906 年在 Sur quelques points du calcul fonctionnel 中给出度量空间完备性的现代定义"),
 ("- 哲学命题：PRO-12 范式（完备 = 范式收敛可达），PRO-08 应而不藏（应层留痕序列的极限可达）",
  "- 哲学命题：PRO-02 道一（发散自然，收敛必为），PRO-08 应而不藏（应层留痕序列的极限可达）"),
 ("- 形式化：PRO-12 范式要求治理动作收敛到稳定状态。完备度量空间为范式提供收敛可达的数学结构：范式状态配以完备度量，范式迭代序列若 Cauchy 收敛则极限在状态空间内（见公式 $\\ref{eq:top005-limit-convergence}$），范式收敛可达。PRO-08 应层留痕形成序列，留痕序列的极限（若有）由完备性保证在空间中可达，使应层留痕闭合",
  "- 形式化：PRO-02 道一确立收敛是治理的构成性要求，完备度量空间给出收敛可达的一组充分条件：状态空间完备时 Cauchy 迭代序列的极限在空间内取得（见公式 $\\ref{eq:top005-limit-convergence}$），实现条件不满足时收敛无保证。PRO-08 应层留痕形成序列，留痕序列的极限由完备性保证在空间中可达，使应层留痕闭合"),
 ("- 应用场景：assayer 迭代判据",
  "- 应用场景：facet 迭代判据"),
 ("范式（PRO-12）的迭代收敛需完备度量空间。\n\n- 应用场景：sih-engine 范式状态机的迭代",
  "治理状态的迭代收敛需完备度量空间。\n\n- 应用场景：治理状态机（候选设计，尚无组件实装）"),
 ("- TOP-002 Brouwer 不动点：Brouwer 在紧凸集上给不动点（见公式 $\\ref{eq:top002-brouwer-fixed-point}$），紧性蕴含完备性（在某些条件下），Brouwer 定理可视为完备性 + 紧性的特例",
  "- TOP-002 Brouwer 不动点：紧度量空间必完备，故 Brouwer 的紧凸集上 Banach 的完备性前提自动满足，但 Brouwer 只要求连续不要求压缩，两定理条件互不蕴含"),
 ("3. 极限是否在 $X$ 内：极限点必须在 $X$ 中，不在 $X$ 中的极限不构成完备性",
  "3. 极限是否在 $X$ 内：完备性要求 Cauchy 列的极限可在 $X$ 本身中取得，完备化构造前的空间不满足此条"),
 ("- 1895 年 Georg Cantor 在 Grundlagen einer allgemeinen Mannigfaltigkeitslehre 中给出闭球嵌套引理",
  "- 1883 年 Georg Cantor 在 Grundlagen einer allgemeinen Mannigfaltigkeitslehre 中给出闭球嵌套原理的实数版本"),
 ("- Cantor, G. (1895). Grundlagen einer allgemeinen Mannigfaltigkeitslehre. Teubner",
  "- Cantor, G. (1883). Grundlagen einer allgemeinen Mannigfaltigkeitslehre. Teubner"),
]

# ORD-001
EDITS['ORD-001'] += [
 ("- 用严格偏序 <：x < y 当 x ≤ y 且 x ≠ y，公理为非自反 + 传递 + 三歧（x < y 或 y < x 或 x = y）",
  "- 用严格偏序 <：x < y 当 x ≤ y 且 x ≠ y，公理为非自反加传递，反对称性可由两者推出；三歧性是全序的性质不属严格偏序"),
 ("偏序集的对偶性。偏序 (P, ≤) 的对偶是 (P, ≥)，其中 x ≥ y 当 y ≤ x。偏序集与其对偶在定理上对偶（任意上确界 ↔ 下确界，链 ↔ 反链）。",
  "偏序集的对偶性。偏序 (P, ≤) 的对偶是 (P, ≥)，其中 x ≥ y 当 y ≤ x。偏序集与其对偶在定理上对偶（任意上确界 ↔ 下确界，极大元 ↔ 极小元，最大元 ↔ 最小元）；链在对偶下仍是链，反链仍是反链。"),
 ("- 哲学命题：PRO-12 范式（偏序 = 范式本体），PRO-13 链外补充（链外元素的偏序扩展）",
  "- 哲学命题：PRO-02 道一（发散自然，收敛必为）"),
 ("- 形式化：PRO-12 范式要求治理动作收敛到稳定状态。偏序集为范式提供本体结构：范式状态构成偏序集 P，范式迭代在 P 上按偏序收敛，范式的最大（小）元是稳定范式。偏序容许范式状态不可比，反映范式分支的多样性。PRO-13 链外补充要求在范式链外补充新元素，偏序集的链外扩展是链外补充的数学刻画",
  "- 形式化：PRO-02 道一确立收敛是治理的构成性要求。偏序集为收敛刻画提供本体结构：治理状态构成偏序集 P，迭代是否趋于稳定可在序意义下陈述，极大元与最大元是稳定态的序论形态。偏序容许状态不可比，反映治理分支的多样性；收敛的保证还需完全格或度量完备等强化条件，见 ORD-002 与 TOP-001"),
 ("范式（PRO-12）的状态空间配以偏序。\n\n- 应用场景：sih-engine 范式状态机",
  "治理状态的空间配以偏序。\n\n- 应用场景：治理状态机（候选设计，尚无组件实装）"),
 ("- 应用场景：assayer verdict 空间",
  "- 应用场景：facet verdict 空间"),
 ("- 应用场景：sih-engine 决策的信念更新",
  "- 应用场景：决策信念更新（候选设计，尚无组件实装）"),
 ("- TOP-004 度量空间：度量空间配以离散序 d(x, y) ≤ 1 是偏序集的特殊情况",
  "- TOP-004 度量空间：任何集合配离散序即偏序集，度量结构与序结构相互独立"),
]

# ORD-002
EDITS['ORD-002'] += [
 ("- 哲学命题：PRO-12 范式（范式作为偏序链的极限），PRO-13 链外补充（链外补充是偏序的扩展操作）",
  "- 哲学命题：PRO-02 道一（发散自然，收敛必为）"),
 ("- 形式化：PRO-12 范式要求治理动作收敛到稳定状态。完全格为范式提供严格的偏序框架：范式状态构成完全格 L，范式迭代在 L 上单调收敛。最小元是初始范式，最大元是终极范式，任意子集的上下确界是范式组合的极限定理。PRO-13 链外补充操作在完全格上有严格定义：链外元素是偏序中不在 L 内的点，其扩展保持完全格结构。",
  "- 形式化：PRO-02 道一确立收敛是治理的构成性要求。完全格为收敛刻画提供严格的序框架：治理状态构成完全格 L 时任意状态组合的上下确界在格内，最小元与最大元对应初始与终极稳定态，单调迭代的收敛存在性由 ORD-003 承载。"),
]

# ORD-003
EDITS['ORD-003'] += [
 ("- 最大不动点 $\\mathrm{gfp}(f) = \\vee\\{x \\in L \\mid f(x) \\leq x\\} = \\vee\\{f^n(\\top) \\mid n \\geq 0\\}$\n- 最小不动点 $\\mathrm{lfp}(f) = \\wedge\\{x \\in L \\mid f(x) \\geq x\\} = \\vee\\{f^n(\\bot) \\mid n \\geq 0\\}$",
  "- 最大不动点 $\\mathrm{gfp}(f) = \\vee\\{x \\in L \\mid f(x) \\geq x\\}$，即后不动点之并\n- 最小不动点 $\\mathrm{lfp}(f) = \\wedge\\{x \\in L \\mid f(x) \\leq x\\}$，即前不动点之交"),
 ("其中 $\\bot = \\wedge L$ 是完全格的最小元，$\\top = \\vee L$ 是最大元。$f^n(\\bot)$ 是 $f$ 从 $\\bot$ 开始的 $n$ 次迭代：$f^0(\\bot) = \\bot, f^1(\\bot) = f(\\bot), f^2(\\bot) = f(f(\\bot)), \\ldots$。\n\n最小不动点的等价刻画。$f$ 的最小不动点是单调迭代序列 $\\{f^n(\\bot)\\}_{n \\geq 0}$ 的极限（按 $L$ 的偏序取上确界）。最大不动点类似地从 $\\top$ 反向迭代得到。",
  "其中 $\\bot = \\wedge L$ 是完全格的最小元，$\\top = \\vee L$ 是最大元。前不动点即 $f(x) \\leq x$ 的点，后不动点即 $f(x) \\geq x$ 的点。\n\n迭代刻画须加连续性前提。一般单调算子的 $\\omega$ 次迭代未必到达不动点：取 $L$ 为第一不可数序数加其顶元，$f(\\alpha) = \\alpha + 1$，唯一不动点是顶元，而 $\\vee\\{f^n(\\bot)\\}$ 停在第一无限序数。若再加 $f$ 保链上确界，则 $\\mathrm{lfp}(f) = \\vee\\{f^n(\\bot)\\}$（Kleene 构造）；最大不动点的对偶迭代需 $f$ 保链下确界。一般单调情形须沿序数超限迭代（Cousot 与 Cousot 1979）。"),
 ("定理证明的核心是构造单调序列 $x_0 = \\bot, x_{n+1} = f(x_n)$，证 $\\{x_n\\}$ 单调递增，其上确界是不动点。完全格的任意子集有上确界保证序列极限存在。\n\nCousot-Cousot 1979 的构造性证明。Cousot 与 Cousot 给出 Tarski 定理的构造性版本，证 $f^n(\\bot)$ 的上确界可通过有限次迭代逼近，给出不动点的算法构造。",
  "定理证明的核心是对偶 closures：设 $A = \\{x \\in L \\mid f(x) \\leq x\\}$ 为前不动点集，$a = \\wedge A$。对 $x \\in A$ 有 $a \\leq x$，单调给出 $f(a) \\leq f(x) \\leq x$，故 $f(a)$ 是 $A$ 的下界，于是 $f(a) \\leq a$；再由 $f(a) \\leq a$ 与单调性得 $f(f(a)) \\leq f(a)$，即 $f(a) \\in A$，而 $a$ 是 $A$ 的下确界故 $a \\leq f(a)$。合并得 $f(a) = a$，且 $a$ 是最小不动点。对偶地 $\\vee\\{x \\mid f(x) \\geq x\\}$ 是最大不动点。\n\nCousot 与 Cousot 1979 的构造性版本把迭代放宽到超限：一般单调算子沿序数迭代至稳定，$\\omega$ 步未必足够，见定义节反例。"),
 ("以最小不动点为例。设 $p = \\wedge B$。证 $f(p) \\in B$（即 $f(f(p)) \\geq f(p)$）：\n\n- 由 $p \\in B$，$f(p) \\geq p$\n- 由 $f$ 单调，$f(f(p)) \\geq f(p)$\n- 故 $f(p) \\in B$\n- 由 $p$ 是 $B$ 的下确界，$p \\leq f(p)$\n- 即 $f(p) \\geq p$\n\n再证 $p \\leq f(p)$：\n\n- 由 $p \\in B$，$f(p) \\geq p$\n- 故 $p \\leq f(p)$\n\n合并得 $f(p) = p$。类似地证最大不动点 $= \\vee A$。\n\n迭代构造。最小不动点 $= \\vee\\{f^n(\\bot) \\mid n \\geq 0\\}$。由 $f$ 单调与 $f(\\bot) \\geq \\bot$，序列 $f^n(\\bot)$ 单调递增，由完全性上确界存在。极限是不动点。",
  "以最小不动点为例，$p = \\wedge A$，$A = \\{x \\mid f(x) \\leq x\\}$。\n\n- 对 $x \\in A$：$p \\leq x$ 与单调给出 $f(p) \\leq f(x) \\leq x$，故 $f(p)$ 是 $A$ 的下界，于是 $f(p) \\leq \\wedge A = p$\n- 由 $f(p) \\leq p$ 与单调性：$f(f(p)) \\leq f(p)$，故 $f(p) \\in A$\n- $p$ 是 $A$ 的下确界而 $f(p) \\in A$，故 $p \\leq f(p)$\n- 合并 $f(p) = p$，且 $p$ 是最小不动点\n\n对偶地 $\\vee B$（$B$ 为后不动点集）是最大不动点。迭代构造 $\\vee\\{f^n(\\bot)\\}$ 只在 $f$ 额外保链上确界时成立（Kleene），一般单调须超限迭代，见定义节反例。"),
 ("- 哲学命题：PRO-12 范式（偏序链的极限），PRO-13 链外补充（链外扩展）",
  "- 哲学命题：PRO-02 道一（发散自然，收敛必为）"),
 ("- 形式化：PRO-12 范式要求治理动作收敛到稳定状态。完全格上的单调算子迭代收敛到不动点，给范式收敛提供严格存在性证明：若范式状态空间可配为完全格，范式更新是单调算子（见公式 $\\ref{eq:ord003-monotone}$），则必存在稳定范式（不动点）使范式迭代到该点。PRO-13 链外补充要求在范式链外补充新元素，Knaster-Tarski 的对偶性保证链外元素的不动点也可在完全格中计算",
  "- 形式化：PRO-02 道一确立收敛是治理的构成性要求。本条给出该要求的序论实现条件：治理状态空间可配为完全格且更新为单调算子（见公式 $\\ref{eq:ord003-monotone}$）时稳定态存在，但存在不等于可达，可达需连续性或超限迭代；条件不满足时收敛无保证，治理须回查迭代设计"),
 ("- 形式化：verdict 空间可配为完全格（见公式 $\\ref{eq:ord002-complete-lattice}$），gate 算子是单调算子（见公式 $\\ref{eq:ord003-monotone}$），$f^n(\\bot)$ 迭代给出 verdict 的下界，$\\vee f^n(\\bot)$ 是 gate 的最小不动点",
  "- 形式化：verdict 空间可配为完全格（见公式 $\\ref{eq:ord002-complete-lattice}$），gate 算子是单调算子（见公式 $\\ref{eq:ord003-monotone}$）时最小与最大不动点存在；若 gate 另保链上确界则 $\\vee f^n(\\bot)$ 可构造到达最小不动点，仅单调则须超限迭代"),
 ("- PROB-001 WLLN（见公式 $\\ref{eq:prob001-wlln-limit}$） / PROB-002 SLLN（见公式 $\\ref{eq:prob002-slln}$）：Schauder 1927 用 Tarski 风格的方法证明 Banach 不动点，类似思路可用于证明概率收敛定理\n", ""),
 ("- 1927 年 Knaster 与 Tarski 在同篇文章中给出集合论与格论两个版本",
  "- 1928 年 Knaster 给出幂集格版本，1955 年 Tarski 推广到一般完全格"),
]

# ORD-004
EDITS['ORD-004'] += [
 ("- 严格单调：$x < y$ 蕴含 $f(x) < f(y)$\n- 强单调：$x < y$ 蕴含 $f(x) \\leq y$（弱版本）\n- 反单调：$x \\leq y$ 蕴含 $f(x) \\geq f(y)$",
  "- 严格单调：$x < y$ 蕴含 $f(x) < f(y)$\n- 反单调：$x \\leq y$ 蕴含 $f(x) \\geq f(y)$"),
 ("- 哲学命题：PRO-12 范式（单调 = 范式保持），PRO-13 链外补充（链外元素的单调扩展）",
  "- 哲学命题：PRO-02 道一（发散自然，收敛必为）"),
 ("- 形式化：PRO-12 范式要求治理动作收敛到稳定状态。单调算子（见公式 $\\ref{eq:ord004-monotone}$）为范式提供保持性结构：范式更新函数 $f$ 是单调算子（输入范式 $\\leq$ 输出范式蕴含更新后范式 $\\leq$ 更新后范式），范式迭代在单调性下保证收敛。单调性是范式自洽的数学基础。PRO-13 链外补充要求在范式链外补充新元素，单调算子可扩展到链外：链外元素保持单调性，链外补充在单调框架内有良定义",
  "- 形式化：PRO-02 道一确立收敛是治理的构成性要求。单调算子（见公式 $\\ref{eq:ord004-monotone}$）为治理更新提供保持性结构：更新函数保序时迭代行为的序分析可用，但单调性单独只保证不动点存在（完全格上）不保证可达，可达需连续性或超限迭代，见 ORD-003"),
 ("单调性蕴含不动点的不动点集是子格。设 $f: P \\to P$ 单调，若 $x, y$ 都是不动点，则 $x \\vee y$ 与 $x \\wedge y$ 也是不动点（因 $f(x \\vee y) = f(x) \\vee f(y) = x \\vee y$）。",
  "不动点集的结构由 Tarski 定理承载：完全格上的单调算子，其不动点集构成完全格。注意固定点格的并与交一般不同于环境格的并与交：单调不保证保并，两个不动点的并 $x \\vee y$ 满足 $x \\vee y \\leq f(x \\vee y)$ 只是后不动点，未必是不动点。"),
 ("Banach 压缩映射与单调性的关系。压缩映射蕴含严格单调：$x < y$ 蕴含 $d(x, y) > 0$ 蕴含 $d(f(x), f(y)) \\leq q \\cdot d(x, y) < d(x, y)$ 蕴含 $f(x) \\neq f(y)$ 蕴含 $f(x) < f(y)$。但单调不蕴含压缩，单调性是更弱的条件。",
  "Banach 压缩映射与单调性的关系。两条件互不蕴含：压缩性是度量性质，单调性是序性质，同一载体上的度量与序若无相容性约束则互不干预，压缩映射对某序可不单调，单调映射亦可不压缩。"),
 ("- 借鉴源：Knaster 1927 年在集合论版本中首次使用单调算子，Birkhoff-Tarski 1948 年在 Lattice Theory 中系统化，Tarski 1955 年在 Pacific J. Math. 中给出完全格上单调算子不动点的完整刻画",
  "- 借鉴源：Knaster 1928 年在幂集格版本中首次给出不动点结果，Birkhoff 1940 年在 Lattice Theory 中系统化序论工具，Tarski 1955 年在 Pacific J. Math. 中给出完全格上单调算子不动点的完整刻画"),
 ("范式（PRO-12）状态机的转移函数常配为单调算子。\n\n- 应用场景：sih-engine 范式状态机",
  "治理状态机的转移函数可配为单调算子。\n\n- 应用场景：治理状态机（候选设计，尚无组件实装）"),
]

def main():
    fail = 0
    for key, path in E.items():
        t = path.read_text(encoding='utf-8')
        for i, (old, new) in enumerate(EDITS[key]):
            if old not in t:
                print(f"失配: {key} #{i}: {old[:60]!r}")
                fail += 1
            else:
                t = t.replace(old, new, 1)
        path.write_text(t, encoding='utf-8')
    print("全部替换完成" if not fail else f"{fail} 处失配")

main()
