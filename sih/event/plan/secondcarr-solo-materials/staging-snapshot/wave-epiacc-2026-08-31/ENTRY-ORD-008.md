---
entry: 引用图可达与孤悬判定
agent: 复归段补强波簇A
proposed_id: ORD-008
subrepo: order
id_reason: order 子仓已占 ORD-001 至 ORD-005，ORD-006 与 ORD-007 已预派同波他件，本件承引用图偏序与闭包语义顺次取 ORD-008
anchors:
  - pro: EPI-13
    source: sih-philosophy/emanation/epistrophe/13-on-grounding.md:7
    quote: "从 11- 倒推至 00-，检查每步核心命题是否被后续步骤引用或修正。"
  - pro: EPI-13
    source: sih-philosophy/emanation/epistrophe/13-on-grounding.md:89
    quote: "逐步检查每个链步的核心命题，去掉「代码工程」「AI」「代码」等限定词后是否仍成立。"
selfcheck:
  锚引文逐字节复核: 两条锚经 grep -nF 复核，13-on-grounding.md 第 7 行与第 89 行逐字节命中，引文即该行全文
  三查: 全角括号中文零处、破折号零处、围栏代码块零处
  结构: 首行 H1 为 ORD-008 引用图可达与孤悬判定，首个二级标题为定义，八节顺序在场
  数学主张自查: 孤悬判定为有限出度统计可判定，去语境化判定为补域反例搜索且补域无限时不保证可判定并如实降级为有界搜索，总判定二维写全
  引文洁净度: 两条引文均不含全角括号与破折号
---

# ORD-008 引用图可达与孤悬判定

状态：草稿，拟派号 ORD-008，哲学到工程桥梁条目，未入索引。

## 定义 {#definition}

链引用图 (Chain Reference Graph) 是以链步为顶点、以引用与修正为边的有向图，是落地检测的对象。

设 $V = \{v_1, \ldots, v_n\}$ 是链步集合，配有链序 $v_1 < v_2 < \cdots < v_n$，即拓扑排序意义下早步在先、晚步在后。边集 $E = E_{\mathrm{ref}} \cup E_{\mathrm{corr}}$ 分两类：

$$E_{\mathrm{ref}} = \{(p, q) \mid q \text{ 显式引用 } p \text{ 的核心命题}\}, \quad E_{\mathrm{corr}} = \{(p, q) \mid q \text{ 修正 } p \text{ 的核心命题}\} \label{eq:ord008-edges}$$

边只由早步指向晚步：$(p, q) \in E$ 蕴含 $p < q$。称 $G = (V, E)$ 是链引用图，$p$ 到 $q$ 的边表示 $p$ 被晚步 $q$ 引用或修正。

可达性。设 $R$ 是边集 $E$ 的传递闭包：$R(p, q)$ 当且仅当存在长度至少为一的有向路从 $p$ 至 $q$。由边只沿链序方向，$G$ 无向环，$R$ 无自反且传递，是严格偏序；$R$ 与恒等关系的并在 $V$ 上构成偏序。

孤悬节点 (Orphan Node)。$p \in V$ 是孤悬的，当且仅当 $p$ 非末端步骤且其出度为零：

$$\mathrm{orphan}(p) \iff p \neq v_n \ \wedge \ |\{q \in V \mid (p, q) \in E\}| = 0 \label{eq:ord008-orphan}$$

判定 (落地检测一)。链 $G$ 通过落地检测，当且仅当 $G$ 无孤悬节点。可判定性：对每个非末端步骤 $p$，扫描全部晚步的显式引用与修正文本，统计出度；至多 $n - 1$ 个步骤、每步至多 $n$ 次文本检查，是有限机械检查，终止且给出正确结果。

去语境化命题对 (Decontextualized Pair)。设 $D$ 是无限制域，$D_C \subsetneq D$ 是语境域，$P$ 是谓词。带语境限定的命题 $P_C$ 是谓词 $P$ 限定在 $D_C$ 上的表述，即 $\forall x \in D_C,\ P(x)$；去语境化命题 $P_D$ 是同一谓词在 $D$ 上的表述，即 $\forall x \in D,\ P(x)$。

判定 (落地检测二)。前提：$P_C$ 作为链步核心命题成立，即 $\forall x \in D_C,\ P(x)$ 已确立。在此前提下 $P_D$ 通过去语境化检测，当且仅当补域中无反例，因 $D$ 是 $D_C$ 与 $D \setminus D_C$ 的并：

$$\forall x \in D \setminus D_C,\ P(x) \label{eq:ord008-decontextualized}$$

判定形式是补域反例搜索：在 $D \setminus D_C$ 中搜索使 $P$ 不成立的元素，找到即不通过，搜索完毕未找到即通过。适用边界：当 $D \setminus D_C$ 无限或不可有效枚举时，该搜索不保证终止，可判定性不保证；实践须以有界反例搜索近似，声明界限 $B$ 后在界内搜索，结论表述为「界限 B 内无反例」，不表述为「普遍成立」。

总判定。落地检测是二维的：引用孤悬检测，见公式 (eq:ord008-orphan)，与去语境化检测，见公式 (eq:ord008-decontextualized)，两维皆过方为通过。只过一维的链须标注受限语境。判定标记两列对应：孤悬检测给出通过、待审视、自明，去语境化检测给出通过、含语境依赖。

## 公理条件 {#axioms}

链引用图与两维判定依赖以下公理条件。

- 链序：$V$ 配有严格全序，边只由早步指向晚步，$G$ 无向环，传递闭包 $R$ 加恒等构成偏序
- 边可机械抽取：引用边与修正边的判定基于步间显式文本，以登记名或精确串搜索抽取，不用语义相似度
- 出度统计：$\mathrm{outdeg}(p) = |\{q \in V \mid (p, q) \in E\}|$，孤悬判定只依赖出度与是否末端
- 末端豁免：$v_n$ 是链末端步骤，豁免孤悬判定并标记自明；豁免集须显式声明
- 谓词同一性：去语境化只去除语境限定词，谓词 $P$ 的真值规则不变；若去限定词同时改变谓词内涵，本条目判定不适用，须另行标注
- 有界搜索：补域反例搜索须声明界限 $B$，超限结论降级为「界限 B 内无反例」

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：图论可达性与传递闭包 (Warshall 1962, West 图论教材)，谓词论域限制 (Tarski 1933)
- 哲学命题：EPI-13 落地检测与去语境化检验
- 形式化：EPI-13 的倒推检查是自末端至首端逐步检查核心命题是否被后续步骤引用或修正，其数学形态是链引用图的出度为零检测：倒推是遍历方向，判定准则本身是向前的出度统计，末端步以自明豁免。孤悬命题即出度为零的非末端节点，是原文「孤悬命题」的图论显形。去语境化检验是谓词论域从 $D_C$ 扩至 $D$ 后仍成立的检验，判定为补域 $D \setminus D_C$ 的反例搜索；补域无限时该搜索不保证可判定，以有界搜索近似并如实降级结论。两维总判定即落地检测的通过条件，一维通过而另一维不通过时标注受限语境

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 审阅发现的引用结构与聚合 verdict
- 形式化：facet 的逐条发现按生成序为链步，后续审阅轮次与 compiler 聚合 verdict 为晚步；发现被 verdict 或修正引用即有出边，零出边的非末端发现标记待审视，不进入聚合结论。含「本仓」「此模型」等语境限定的发现走去语境化：语境域为当前材料类，无限制域为全部材料类，补域反例搜索以有界材料样本近似
- 借鉴方向：两维总判定用于聚合门：两维皆过的发现进入 verdict，一维过的发现以受限语境标注
- 边界：facet 发现 schema 与 verdict 结构的具体定义由 facet 工程层决定，不属本条目范围

## 与其他概念的关系 {#relations}

- ORD-001 偏序集：$R$ 与恒等关系的并在 $V$ 上构成偏序，边集 $E$ 是链序严格偏序的子关系，见公式 (eq:ord008-edges)
- ORD-005 链与反链：$V$ 在链序下自身即链，边集只取该链的部分比较对；孤悬节点是偏序 $R$ 中无后继节点，即不被任何晚步可达的节点
- ORD-002 完全格：$\mathcal{P}(V \times V)$ 在 $\subseteq$ 下是完全格，传递闭包算子 $E \mapsto E^+$ 是其上的闭包算子，单调、扩张、幂等
- ORD-007 带锚推衍链 (同波草稿)：落地检测检链步间引用结构，带锚推衍链检步对锚集与前序的后承结构，两者同为复归段镜像检验的序论承载，一个看步间边，一个看步对锚的边

## 历史脉络 {#history}

- 1962 年 Warshall 在 A theorem on Boolean matrices 中给出布尔矩阵传递闭包的算法刻画
- 1962 年 Floyd 在 Algorithm 97 中给出路径闭包算法，传递闭包成为图算法标准问题
- 1972 年 Tarjan 在 Depth-first search and linear graph algorithms 中给出线性时间的图遍历与强连通判定，无向环判定即其推论
- 1933 年 Tarski 的语义框架中谓词真值定义在论域上，论域限制与扩展是谓词语义的基本操作
- 图论教材如 West 的 Introduction to Graph Theory 将可达性、传递闭包、拓扑排序列为有向图基础内容

## 工程注意事项 {#engineering-notes}

执行落地检测须完成以下核验动作。

1. 先抽边后判孤悬：对每步先机械抽取晚步对其核心命题的显式引用与修正，登记边清单，再计算出度；不出边清单直接报孤悬判定无效
2. 末端豁免显式声明：豁免集只含末端步骤，报告须列出豁免步与自明标记；非末端步骤进入豁免集即违判定定义
3. 边类型分记：引用边与修正边分别计数；修正边同时是前判失误信号，报告须单独标注修正边来源
4. 有界搜索声明界限：补域不可有效枚举时，报告须写明界限 B 的具体取值，结论表述为界限 B 内无反例，不表述为普遍成立
5. 两维判定分列：引用孤悬检测与去语境化检测输出为两个独立判定；只过一维的链标注受限语境，不得合并写作通过

## 参考文献 {#references}

- Warshall, S. (1962). A theorem on Boolean matrices. Journal of the ACM, 9(1), 11-12
- Floyd, R.W. (1962). Algorithm 97: Shortest path. Communications of the ACM, 5(6), 345
- Tarjan, R.E. (1972). Depth-first search and linear graph algorithms. SIAM Journal on Computing, 1(2), 146-160
- Tarski, A. (1933). Pojęcie prawdy w językach nauk dedukcyjnych. Prace Polskiego Towarzystwa Matematycznego
- West, D.B. (2001). Introduction to Graph Theory, 2nd ed. Prentice Hall
- Wikipedia "Transitive closure" 条目
