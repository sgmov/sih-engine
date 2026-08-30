# 任务包 B：概率簇（PROB-004 大偏差原理 / PROB-005 Bayesian 更新）生而绿条目草稿

你是司衡工作区数学仓 sih-math 概率子仓的条目起草子代理。主线编排者已拆簇派单，你领本簇两件，交稿后主线跑机械验收。只凭本包内容跑，不自创范围。

## 边界铁律（违反即废）

- 只允许写 `/Users/moc/workspaces/SiHankor/agent-drafts/entry/` 下的草稿与认领行，其余一切只读。
- 禁止入仓、跑 lease / scribe / git、改 AGENTS.md、上链、改 sih-math 或 sih-philosophy 任何文件。
- 你的产出是符号材料，属待校验草稿，不是入库条目。

## 工作对象（两件各一稿）

1. **PROB-004 大偏差原理**。登记语义：异常事件的尾部概率。概念族：概率近似。
2. **PROB-005 Bayesian 更新**。登记语义：先验与证据的合成。概念族：信念更新。

## 必读文件（先读后写，顺序固定）

1. 结构范本两件（照其节序、语气、密度写）：
   - `/Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-007-expectation.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-006-probability-measure.md`
2. 本仓既有概率条目（照其锚点用法、公式标注、关系节写法；它们是「已建」态）：
   - `/Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-001-law-of-large-numbers.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-002-strong-law-of-large-numbers.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-003-central-limit-theorem.md`
3. 哲学原文（找锚点引文，逐字节复制）：
   - `/Users/moc/workspaces/SiHankor/sih-philosophy/emanation/proodos/08-on-settle.md`
   - `/Users/moc/workspaces/SiHankor/sih-philosophy/emanation/proodos/07-on-assay.md`

## 每件硬性格式（照核阅 0.3.0 规则，违一件即废）

1. 正文首行（front matter 之后第一个内容行）为 `# <ID> <中文名>`，ID 前缀必须 PROB-。本簇即 `# PROB-004 大偏差原理`、`# PROB-005 Bayesian 更新`。
2. 第一个二级标题必须是 `## 定义 {#definition}`。
3. 全文零全角括号中文内容（纯英文数字内容的全角括号可留）、零破折号 `——`、零围栏代码块 ``` 。
4. 每件引哲学命题锚 1 至 2 条。引文纪律五条：从原文复制粘贴逐字节一致；引号按原文件字符（原文用直引号 `"` 或「」就照抄，勿转成别种）；连续片段禁止删节；引文不得含全角括号 `（` 与破折号 `——`；行号用 grep -n 实测后填。
5. 结构照范本八节，节名与锚点逐字：
   `## 定义 {#definition}` / `## 公理条件 {#axioms}` / `## 哲学桥接 {#philosophy-bridge}` / `## 在 facet 的应用 {#facet-application}` / `## 与其他概念的关系 {#relations}` / `## 历史脉络 {#history}` / `## 工程注意事项 {#engineering-notes}` / `## 参考文献 {#references}`。证明思路节可选。
6. 数学内容必须正确：定理陈述给条件，反例与边界如实，不确定的内容不写。

## 数学内容要点（正确性锚，非限定，可增补但要准）

### PROB-004 大偏差原理
- 定义：设独立同分布随机变量序列，部分和均值 S_n/n 满足大偏差原理（LDP），速率函数 I，则对闭集 F 有 limsup (1/n) log P(S_n/n ∈ F) ≤ -inf_{x∈F} I(x)，对开集 G 有 liminf (1/n) log P(S_n/n ∈ G) ≥ -inf_{x∈G} I(x)。I 良值下凸，I(x)=0 恰在均值处。
- Cramér 定理：独立同分布、矩生成函数在均值邻域有限时，速率函数 I(x) = sup_t (t x - log M(t))，即对偶（Legendre-Fenchel 共轭）。
- 良态性：速率函数在紧集上连续（良速率函数）时上下界收紧为等式。
- 边界/反例：无矩生成函数（重尾）时 LDP 可能不成立或速率函数退化；Sanov 定理（经验测度）是 LDP 在符号空间上的形式，速率函数为相对熵。
- 与中心极限的对照：CLT 给小偏差（O(1/sqrt n) 尺度）的高斯刻画，LDP 给大偏差（指数尺度）刻画，两者互补不互推。
- 关系：PROB-001/PROB-002（均值收敛，LDP 刻画偏离的指数速率）、PROB-003 CLT（小偏差互补）、PROB-006 概率测度（载体）、PROB-007 期望（I 零位即均值/期望）。

### PROB-005 Bayesian 更新
- 定义：设假设 H，观测证据 E。贝叶斯定理 P(H|E) = P(E|H) P(H) / P(E)。P(H) 先验，P(E|H) 似然，P(H|E) 后验，P(E) 边际似然（证据因子）。
- 多假设：后验归一化 P(H_i|E) ∝ P(E|H_i) P(H_i)，对 i 求和归一。
- 共轭先验：Beta-Bernoulli、Normal-Normal 等族，更新封闭。
- 边界：先验非正则（improper prior）时后验仍可为正则，但须验证；证据与假设条件独立假设不成立时朴素贝叶斯失效；后验敏感于先验（弱证据下）。
- 关系：PROB-006 概率测度（条件概率与全概率的载体）、PROB-007 期望（后验期望）、PROB-001/002（似然序列的频率解释）、ORD-002 完全格（信念空间序结构，标注跨子仓）。

## 候选哲学锚点（已实测干净可命中；请 grep -n 复核行号并逐字节复制，可另选同文件更贴切行）

### PROB-004 大偏差原理（建议 1 至 2 锚）
- 锚一 PRO-08 应几，源 `sih-philosophy/emanation/proodos/08-on-settle.md`：
  - 候选 L100：`应几：当前尚不可观测的、需要预判的`
  - 桥接立意：大偏差给当前尚不可观测的异常偏离一个指数级小概率的精确速率，应几的预判有了可计算的量级：偏离越远速率函数 I 越大、概率指数衰减越快，预判窗口可量化。
- 锚二（可选）PRO-08 应对，源同上：
  - 候选 L72：`面对未预见的 bug：怎么应对`
  - 或候选 L102：`应辨与应几共同构成司衡的"应对"维度：既处理已发生的事，也预防未发生的事。`（该行含直引号 "应对"，YAML 双引号串内须转义为 \"）
  - 桥接立意：LDP 把「未发生的异常」的概率速率显式化，应对前置检测有尾部口径可依。
- 若只用一锚即取锚一，须保证至少 1 锚。

### PROB-005 Bayesian 更新（建议 2 锚）
- 锚一 PRO-07 鉴·映照，源 `sih-philosophy/emanation/proodos/07-on-assay.md`：
  - 候选 L79：`映照：只反映事实，不投射判断`
  - 桥接立意：后验即判断随证据更新而只反映事实（似然/观测），不投射先前的判断（先验投射）；Bayesian 更新是「映照不投射」的贝叶斯形式化，后验权重由证据主导而非先验预设。
- 锚二 PRO-07 鉴·客观，源同上：
  - 候选 L78：`客观：不预设立场，不急于下判断`（干净，无内嵌引号）
  - 或候选 L53：`代码工程的"客观"是：在检验时不预设"应该是什么"，只看"实际是什么"。`（含三处直引号，YAML 双引号串内须逐一转义为 \"）
  - 桥接立意：客观即不预设立场，对应更新以似然为证据驱动、不急于以先验定论；弱证据下后验仍受先验牵引，是客观性的边界如实标注。

## 草稿格式

- 文件路径：`/Users/moc/workspaces/SiHankor/agent-drafts/entry/ENTRY-PROB-004.md` 与 `/Users/moc/workspaces/SiHankor/agent-drafts/entry/ENTRY-PROB-005.md`
- front matter 同桥接波：

```
---
entry: ENTRY-PROB-004.md
agent: <你的代理标识，须含 entry-prob 前缀>
anchors:
  - pro: <PRO-xx 命题名>
    source: <sih-philosophy/...md:行号>
    quote: "<逐字节引文，YAML 双引号串，内嵌双引号用 \" 转义>"
  - pro: <第二条，可省>
    source: <...>
    quote: "<...>"
selfcheck:
  pro_ids_verified: <锚数>
  quotes_verified: <锚数>
---
```

- 正文即完整条目（八节），front matter 之后第一行是 `# PROB-004 大偏差原理`。
- 注意：引文含直引号 `"` 时（07 文件 L53、08 文件 L102），YAML 双引号串内须转义为 `\"`，否则 front matter 解析失败。

## 交稿

1. 写两件草稿文件。
2. 在 `/Users/moc/workspaces/SiHankor/agent-drafts/entry/CLAIMS-B-probability.ndjson` 追加两行认领（一件一行）：`{"entry":"ENTRY-PROB-004.md","agent":"<你的标识>","ts":"<ISO8601 UTC>"}`。
3. 最终回信给主线：两件各报锚点源行号、自查五项（标题前缀/首节名/括号零/破折号零/引文逐字节）自核结果、任何不确定处如实标。
