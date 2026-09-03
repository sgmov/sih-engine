# caswire-solo 载体推导档：cascade 级联判定语义形式化

> 批：caswire-solo（cascade 级联检查载体接线批，mathpipe-full 程序批四起）
> 日期：2026-09-03
> 载体：ORD-016 良基关系与倒推终止（sih-math/order/entries/ORD-016-well-founded-relation-and-backward-termination.md）
> 消费侧：sih-tools/cascade/src/cascade/core.py（级联判定核心：check_targets / latest_cert_hashes / build_edges）
> 契约：sih-tools/cascade/CONTRACT.md（载体引用节接线）
> 队形：单线形 solo，零子代理；冲突模式成员（pk-045 样本库）

## 1. 接线概览

cascade 级联判定语义既是治理判定常数（改变它会改变上游洁净的放行/拦截判定，
属 M-1 判定性判据），原为无载体引用无推导档的裸奔态。rev1 账本坐实 cascade 为
可指认未实例化件。本批将其接入 ORD-016 载体，推导档承载级联判定语义的形式化：
**上游依赖倒推遍历的良基性与终止性**与**洁净判定逐层传播**。本批只做接线与推导，
不改 cascade 判定行为（行为零变更，F-2）。

## 2. 载体映射

mapping.md「倒推终止与良基 → ORD-016」命中（mapping.md:190）。ORD-016 定义：
设步集 $Steps$ 有限，引用关系 $c \succ p$ 表示步 $c$ 引用或修正步 $p$。关系 $\succ$
良基，当且仅当不存在无限严格降链 $x_0 \succ x_1 \succ \cdots$。定理一：自任一步出发、
每步沿 $\succ$ 移动到被引用任一步的遍历程序必终止，当且仅当 $\succ$ 良基。定理二：
有限集上 $\succ$ 良基当且仅当传递闭包 $\succ^+$ 无自环（无引用环）。死区定理：
落地检测通过条件即死区为空。

cascade 的实际判定行为即 ORD-016 语义的工程实例：**倒推遍历**即按引用即边的依赖集
逆引用方向检上游、其终止性由引用关系良基担保；**基线比较**即当前内容哈希与链上
最近认证哈希对表（ordwire 先例沿用 M-1 判定性判据）；**洁净判定**即对表出三态
clean/dirty/unverified 并由 dirty 上游拒下游写入。三类的载体锚定见第 4 节。

## 3. 判定语义形式化

### 3.1 引用关系与上游依赖集结

设文档集 $Steps = \{d_1, \dots, d_n\}$ 有限，引用关系 $d \succ u$ 当且仅当文档 $d$
以引用词指向文档 $u$（引用即边，cascade build 建边）。对 $d \in Steps$，上游依赖集
$\mathrm{Up}(d) = \{u \in Steps \mid d \succ u\}$ 即 $d$ 引用的文档集，为 $\succ$-后继
（ORD-016 的出邻居集 $Out(d)$）。边表即 `registry["edges"]`，磁盘实存性由 build 扫描
语料根建立，$Steps$ 有限，边可机械抽取（显式引用词 + 五族正则）。

### 3.2 基线比较（Baseline Comparison）

设链上最近认证哈希映射 $C: \mathrm{AbsPath} \to \mathrm{digest}$，由
`latest_cert_hashes` 从链上 certication_completed 事件按 doc_id 取最近报告的内容哈希
构建。对上游 $u$，当前磁盘哈希 $h = \mathrm{hash}(u)$：

$$
\mathrm{state}(u) = \begin{cases}
  \mathrm{unverified} & C(u) = \bot \quad (\text{无认证史}) \\
  \mathrm{clean}      & C(u) = h \quad (\text{当前等于链上基线}) \\
  \mathrm{dirty}      & C(u) \ne h \quad (\text{当前偏离链上基线})
\end{cases}
$$

基线即链上最近认证哈希，`C(u) == h` 为洁净判据，属 ORD-016 落地判定（到用终点的
路径存在 / 无悬空）在哈希对表面上的对应：认证即节点落地，未认证即 unverified 不拦
承有度、无锚即不加阻断。

### 3.3 洁净判定（Cleanliness Determination）

对目标 $d$ 的整组上游做 3.2 的逐上游判定，聚合出：

$$
\mathrm{dirty}(d) = \{ u \in \mathrm{Up}(d) \mid \mathrm{state}(u) = \mathrm{dirty} \}
$$

$$
\mathrm{verdict}(d) = \begin{cases}
  \mathrm{blocked} & \mathrm{dirty}(d) \ne \emptyset \\
  \mathrm{writable} & \mathrm{otherwise}
\end{cases}
$$

即**任一脏上游拒下游写入**，洁净判定沿依赖边自上游单向传向下游（拒绝方向与引用
方向相反，逆引用流传播）。这是 ORD-016「局部单轮检查」的工程形态：对每步查其
出邻居（上游集）非脏即放行，脏即拦。语义方向即死区定理的传播直觉——死的判断经
边传播为下游被拦，属治理收紧方向，不发明代码没有的谓词。

**声明（行为零变更）**：cascade 现行 check 只对每目标做**单层直接上游**对表，不做
传递闭包递归深倒推，也不实现 gfp 全局死区迭代。推导如实记：若未来扩展到递归深倒推
或全局死区迭代，须按 ORD-016 死区定理对 $F$ 的最大不动点做全迭代，且须先查无环
（定理二有限集无环《=》良基），本批范围不含该扩展。单层直查的终止性由 $Steps$
有限直接担保（每目标上游集有界），深倒推终止性则由引用关系良基担保。

### 3.4 倒推遍历终止性（Backward Traversal Termination）

倒推遍历即自下游出发逐层逆引用方向移动到被引用上游的遍历。ORD-016 定理二：有限集
上 $\succ$ 良基当且仅当无引用环。cascade 的引用图 $Steps$ 有限，故构造环即构造
无限 $\succ$-降链；若无环则引用图良基，自任一步出发沿 $\succ$ 逆移的任一执行路径
有限（定理一），即倒推遍历必终止。cascade 对环的兜底即语义边界节「上游被改未过管线
认证时拦」的乐观锁闸：引用图要构成环必须有自引用链，自引排除与多义无主入注记
保证边表无自环错误，同文档不可自引即不可自身成环，引用环须跨件，跨件环由 DISK 上
各件经认证哈希对表在 check 时不加伪造深度即不伪装终止。单层直查下遍历耦合步数
有界（直查一步），无环 + 有限给整体有界。

工程侧复杂度：单层直查对每目标扫描其上游集，$|Up(d)|$ 有界于边表总度数，
整体为空 $O(\sum_d |Up(d)|) = O(|E|)$（ORD-016 有限集无环检查复杂度同阶）。

## 4. 载体锚点与源码接线位

| 语义 | 载体条目锚点 | 源码接线位 |
|---|---|---|
| 倒推遍历与终止性 | ORD-016 定理一良基《=》终止 + 定理二有限无环 | core.py `build_edges`（建依赖边）/ `check_targets`（逐上游遍历） |
| 基线比较 | ORD-016 落地判定与用终点路径存在 | core.py `latest_cert_hashes`（链上最近认证哈希） |
| 洁净判定 | ORD-016 局部单轮检查与死区传播 | core.py `check_targets`（三态 [clean/dirty/unverified] 与 verdict [blocked/writable]） |

方法：源码判定位注释锚点在 core.py 各判定函数签名处标注载体引用与推导档路径，
属接线不属行为变更。

## 5. 金向量（洁净链全过与污染链上游拒两场景）

金向量即级联判定语义两场景的机械重放，落
materials/caswire-cascade-golden-vector.json：

- **洁净链全过**：链 A→B（A 引用 B），B 当前哈希等于链上认证 → B clean → A
  verdict writable；另隔离开文档 C 无上游 → writable——洁净链全体放行。
- **污染链上游拒**：链 A→B，B 当前哈希偏离链上认证 → B dirty → A verdict blocked
  ——污染上游处拒下游，退出码一。

重放判据：replay_golden.py 双跑逐字节一致，与冻结金向量 IDENTICAL，零漂移（F-3）。

## 6. 载体锚点

- 载体条目：sih-math/order/entries/ORD-016-well-founded-relation-and-backward-termination.md
- 映射表：sih-math/llm-friendly-build/mapping.md「倒推终止与良基 → ORD-016」（mapping.md:190）
- 哲学桥接：良基归纳与结构递归的治理收紧（06-on-canon 治理力度由松到紧），
  落地检测与去语境化（EPI-13），出邻居直查是局部检查、全局终态交死区不动点
