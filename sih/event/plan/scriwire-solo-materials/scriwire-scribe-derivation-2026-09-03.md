# scriwire-solo 载体推导档：scribe 事件链校验语义形式化

> 批：scriwire-solo（scribe 事件链载体接线批，mathpipe-full 程序批四起）
> 日期：2026-09-03
> 载体：ORD-019 版本偏序与外化状态存储（sih-math/order/entries/ORD-019-version-order-and-externalized-state-store.md）
> 消费侧：sih-engine/src/event_stream/（append、verify、park）
> 契约：sih-engine/doc/spec/SPEC-004-event-stream.md（载体引用节接线）
> 队形：单线形 solo，零子代理；冲突模式成员（pk-045 样本库）

## 1. 接线概览

scribe 事件链判定面即 prev_hash 链接校验与 append-only 外化存储与 verify
全链复算，原为无载体引用的裸奔态。本批将链校验语义接入 ORD-019 载体，
推导档承载三类链语义的形式化：**版本偏序**、**只增不改写**、**自最小元
传递复算**。本批只做接线与推导，不改链写入与校验行为（行为零变更，F-2）。

## 2. 载体映射

mapping.md「上下文外化三性质 → ORD-019」命中（mapping.md:196）。ORD-019
定义：状态外化落在版本序 $V$（$\preceq$ 偏序）上，外化状态存储满足三性质
——**持久性**（状态在序上单调外化不回退）、**版本化**（每次写入产生新
版本，旧版本保留不下消失）、**可审计**（任意版本可被定位与复算）。

scribe 事件链的实际语义即 ORD-019 三性质的工程实例：**版本偏序**即事件
流构成一条链、prev_hash 链接即覆盖关系；**只增不改写**即 append-only
外化存储保证偏序只增不删不改；**可审计**即 verify 全链自最小元起的传递
性复算保证任意版本可复核。三类的载体锚定见第 4 节。

## 3. 判定语义形式化

### 3.1 版本偏序（版本序由 prev_hash 覆盖关系构成）

设事件流为序列 $E = (e_0, e_1, \dots, e_{n-1})$，每事件带事件哈希
$h_i = \phi(e_i)$ 与前事件哈希 $\pi_i$。链上覆盖关系定义：事件 $e_i$
覆盖 $e_j$（记 $e_j \preceq e_i$）当且仅当自 $e_j$ 经 $h$ 链接可达 $e_i$。

**覆盖不变式**：相邻事件 prev_hash 严格衔接。

$$
\forall i \in [1, n-1],\quad \pi_i = h_{i-1}
$$

**版本偏序性质**：链上任意两事件可由覆盖关系比较，构成全链偏序
$\preceq$。首事件为最小元，其 prev_hash 取全零（genesis）。

承载位：event_stream/append.rs 第三项校验即 prev_hash 匹配前事件哈希，
`PrevHashMismatch` 即覆盖关系断链拒绝。这是 ORD-019「版本化」的
机械化，工程基线第四条（可验证性约束）的落地。

### 3.2 只增不改写（append-only 外化存储）

外化存储的偏序只增不改写：已写入事件不可改写或删除，只允许在尾部追加
新事件。

$$
E \subseteq E' \;\wedge\; \text{修改}(E) = \varnothing
\;\wedge\; \text{删除}(E) = \varnothing
$$

即任一后续状态 $E'$ 含 $E$ 的全部事件且无历史行被改写。这是 ORD-019
「持久性」的机械实现——状态在序上单调外化不回退。hash 链的不可篡改
保证即此性质的复核机制：任一历史字段变动使该事件哈希变，导致后一事件
prev_hash 失配，verify 暴露。

承载位：append.rs append 入口以追加模式写 NDJSON（`OpenOptions::append`），
trail 物理载体承接 DEC-001 只追加写约束。SPEC-004 验收依据第一条不可篡
改进而第五条只追加写。

### 3.3 自最小元传递复算（verify 全链复核）

verify 即版本偏序的可审计性复核：从最小元起沿链表逐事件核对 prev_hash
等于前事件哈希，全链通过则校验成立。

$$
\mathrm{verify}(E) = \bigwedge_{i=1}^{n-1} \left( \pi_i = h_{i-1} \right)
\;\wedge\; \forall e \in E:\; \text{hash}(e) \text{ 非空且为} 64 \text{位 SHA-256}
$$

承载位：verify.rs verify 入口全量/区间校验，复算位即
hash.rs::verify_chain——自最小元传递复算，任一断链返回断裂事件 ID。
这是 ORD-019「可审计」的机械化，保证任意版本可复核。

## 4. 载体锚点与源码接线位

| 语义 | 载体条目锚点 | 源码接线位 |
|---|---|---|
| 版本偏序 | ORD-019 版本化（每次写入新版本，覆盖关系） | append.rs 第三项校验（PrevHashMismatch） |
| 只增不改写 | ORD-019 持久性（状态单调外化不回退） | append.rs 追加写 + hash 链不可篡改 |
| 传递复算 | ORD-019 可审计（任意版本可复核） | verify.rs verify + hash.rs verify_chain |
| 重放面 | ORD-019 持久性（跨链目录全量重放） | park.rs load_parking_scope |

方法：源码判定位注释锚点在三个文件函数签名处标注载体引用与推导档路径，
属接线不属行为变更。

## 5. 金向量（合法追加与断裂链两场景）

金向量即链校验语义两场景的机械重放，落
materials/scriwire-solo-golden-vector.json：

- **合法追加链场景**：构造段合法链（各事件 prev_hash 衔接），走 verify
  全量 → 应过，返回首末哈希。
- **断裂链场景**：构造 prev_hash 不衔接的链，走 verify 全量 → 应拒
  `HashChainBroken`，返回断裂事件 ID。

重放判据：同参形双跑逐字节一致，金向量字节级冻结零漂移（F-3）。

## 6. 载体锚点

- 载体条目：sih-math/order/entries/ORD-019-version-order-and-externalized-state-store.md
- 映射表：sih-math/llm-friendly-build/mapping.md「上下文外化三性质 → ORD-019」（mapping.md:196）
- 哲学桥接：应而不藏 PRO-08，留痕是应鉴循环的构成性条件；P4.2 责任归属的四个可即