# 任务包 C：序簇（ORD-005 链与反链）生而绿条目草稿

你是司衡工作区数学仓 sih-math 序理论子仓的条目起草子代理。主线编排者已拆簇派单，你领本簇一件，交稿后主线跑机械验收。只凭本包内容跑，不自创范围。

## 边界铁律（违反即废）

- 只允许写 `/Users/moc/workspaces/SiHankor/agent-drafts/entry/` 下的草稿与认领行，其余一切只读。
- 禁止入仓、跑 lease / scribe / git、改 AGENTS.md、上链、改 sih-math 或 sih-philosophy 任何文件。
- 你的产出是符号材料，属待校验草稿，不是入库条目。

## 工作对象（一件一稿）

- **ORD-005 链与反链**。登记语义：链的极限与反链判定。概念族：偏序迭代。

## 必读文件（先读后写，顺序固定）

1. 结构范本两件（照其节序、语气、密度写）：
   - `/Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-007-expectation.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-006-probability-measure.md`
2. 本仓既有序论条目（照其锚点用法、公式标注、关系节写法；它们是「已建」态）：
   - `/Users/moc/workspaces/SiHankor/sih-math/order/entries/ORD-001-partially-ordered-set.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/order/entries/ORD-002-complete-lattice.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/order/entries/ORD-003-knaster-tarski-fixed-point.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/order/entries/ORD-004-monotone-operator.md`
3. 哲学原文（找锚点引文，逐字节复制）：
   - `/Users/moc/workspaces/SiHankor/sih-philosophy/emanation/proodos/02-on-first-tao.md`

## 硬性格式（照核阅 0.3.0 规则，违一件即废）

1. 正文首行（front matter 之后第一个内容行）为 `# ORD-005 链与反链`，ID 前缀必须 ORD-。
2. 第一个二级标题必须是 `## 定义 {#definition}`。
3. 全文零全角括号中文内容（纯英文数字内容的全角括号可留）、零破折号 `——`、零围栏代码块 ``` 。
4. 引哲学命题锚 1 至 2 条。引文纪律五条：从原文复制粘贴逐字节一致；引号按原文件字符（原文用直引号 `"` 或「」就照抄）；连续片段禁止删节；引文不得含全角括号 `（` 与破折号 `——`；行号用 grep -n 实测后填。
5. 结构照范本八节，节名与锚点逐字：
   `## 定义 {#definition}` / `## 公理条件 {#axioms}` / `## 哲学桥接 {#philosophy-bridge}` / `## 在 facet 的应用 {#facet-application}` / `## 与其他概念的关系 {#relations}` / `## 历史脉络 {#history}` / `## 工程注意事项 {#engineering-notes}` / `## 参考文献 {#references}`。证明思路节可选（Dilworth 定理证明思路可载）。
6. 数学内容必须正确：定理陈述给条件，反例与边界如实，不确定的内容不写。

## 数学内容要点（正确性锚，非限定，可增补但要准）

- 链（chain）：偏序集 (P, ≤) 的子集 C，任意两元素可比较（对一切 a,b ∈ C，a ≤ b 或 b ≤ a），即全序子集。
- 反链（antichain）：偏序集 (P, ≤) 的子集 A，任意两个不同元素不可比较（对一切 a ≠ b ∈ A，a ≤ b 与 b ≤ a 均不成立）。
- 宽度（width）：偏序集中最大反链的基数。
- Dilworth 定理：有限偏序集，把 P 划分为链族所需的最少链数等于最大反链的基数（宽度）。对偶表述即最大反链 = 最小链划分。
- Mirsky 定理（对偶）：有限偏序集，把 P 划分为反链族所需的最少反链数等于最长链的基数（链长）。
- 链的极限/上确界：完全格中任意链（有向集）的上确界存在，由 ORD-002 承载；Knaster-Tarski 迭代沿链上升到达不动点，见 ORD-003 / ORD-004。
- 反例/边界：反链大小上界受结构约束（如实）；无限偏序集 Dilworth 需选择公理/推广形式，标注适用范围；偏序维数（dimension）与最小全序扩张数是链反链分析的应用方向。
- 关系：ORD-001 偏序集（载体）、ORD-002 完全格（链上确界存在）、ORD-003 Knaster-Tarski 不动点（沿链迭代）、ORD-004 monotone operator（保序迭代产生单调链）。

## 候选哲学锚点（已实测干净可命中；请 grep -n 复核行号并逐字节复制，可另选同文件更贴切行）

### ORD-005 链与反链（建议 2 锚）
- 锚一 PRO-02 道一「发散自然、收敛必为」，源 `sih-philosophy/emanation/proodos/02-on-first-tao.md`：
  - 候选 L115：`道一：发散是默认方向，治理是收敛的构成性条件。`（该行有 `> ` 前缀，引正文部分）
  - 或候选 L81：`没有治理，就没有收敛，这是构成性条件，不是偏好`
  - 桥接立意：链即可全序比较的状态序列，是治理施加收敛方向后的单序轨迹；反链即两两不可比较的多向状态，是发散默认方向（无治理时多认知源不可并序）。道一「发散是默认方向、治理是收敛的构成性条件」对应反链为发散默认、链为治理收敛产物。
- 锚二 PRO-02 道一·发散形态，源同上：
  - 候选 L67：`每个 AI 模型的输出天然地按训练数据的模式，生成是发散的`
  - 或候选 L68：`每次需求变更天然地引入新的实现选择，演化是发散的`
  - 桥接立意：多认知源独立运作、各自发散方向，构成反链成员两两不可比较；Dilworth 宽度即这种「不可并序方向」的最大独立族规模，治理收敛即把反链并序化。
- 两锚同出 02 文件合规（引文纪律不要求跨文件）；若主线后续要求命题多样可换，本波以可逐字节命中为准。

## 草稿格式

- 文件路径：`/Users/moc/workspaces/SiHankor/agent-drafts/entry/ENTRY-ORD-005.md`
- front matter 同桥接波：

```
---
entry: ENTRY-ORD-005.md
agent: <你的代理标识，须含 entry-ord 前缀>
anchors:
  - pro: <PRO-xx 命题名>
    source: sih-philosophy/emanation/proodos/02-on-first-tao.md:<行号>
    quote: "<逐字节引文，YAML 双引号串，内嵌双引号用 \" 转义>"
  - pro: <第二条，可省>
    source: <...>
    quote: "<...>"
selfcheck:
  pro_ids_verified: <锚数>
  quotes_verified: <锚数>
---
```

- 正文即完整条目（八节），front matter 之后第一行是 `# ORD-005 链与反链`。
- 注意：候选行 L115 有 `> ` 前缀、L67/L68/L81 有 `- ` 前缀，引正文部分即可（引文须为该行逐字节子串）。

## 交稿

1. 写一件草稿文件。
2. 在 `/Users/moc/workspaces/SiHankor/agent-drafts/entry/CLAIMS-C-order.ndjson` 追加一行认领：`{"entry":"ENTRY-ORD-005.md","agent":"<你的标识>","ts":"<ISO8601 UTC>"}`。
3. 最终回信给主线：报锚点源行号、自查五项（标题前缀/首节名/括号零/破折号零/引文逐字节）自核结果、任何不确定处如实标。
