# 任务包 A：拓扑簇（TOP-006 紧集 / TOP-007 连续映射）生而绿条目草稿

你是司衡工作区数学仓 sih-math 拓扑子仓的条目起草子代理。主线编排者已拆簇派单，你领本簇两件，交稿后主线跑机械验收。只凭本包内容跑，不自创范围。

## 边界铁律（违反即废）

- 只允许写 `/Users/moc/workspaces/SiHankor/agent-drafts/entry/` 下的草稿与认领行，其余一切只读。
- 禁止入仓、跑 lease / scribe / git、改 AGENTS.md、上链、改 sih-math 或 sih-philosophy 任何文件。
- 你的产出是符号材料，属待校验草稿，不是入库条目。

## 工作对象（两件各一稿）

1. **TOP-006 紧集**。登记语义：决策空间是否紧。概念族：拓扑性质。
2. **TOP-007 连续映射**。登记语义：映射是否连续。概念族：拓扑性质。

## 必读文件（先读后写，顺序固定）

1. 结构范本两件（照其节序、语气、密度写）：
   - `/Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-007-expectation.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-006-probability-measure.md`
2. 本仓既有拓扑条目（照其锚点用法、公式标注、关系节写法；它们是「已建」态）：
   - `/Users/moc/workspaces/SiHankor/sih-math/topology/entries/TOP-001-banach-fixed-point.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/topology/entries/TOP-004-metric-space.md`
   - `/Users/moc/workspaces/SiHankor/sih-math/topology/entries/TOP-005-complete-metric-space.md`
3. 哲学原文（找锚点引文，逐字节复制）：
   - `/Users/moc/workspaces/SiHankor/sih-philosophy/emanation/proodos/02-on-first-tao.md`
   - `/Users/moc/workspaces/SiHankor/sih-philosophy/emanation/proodos/08-on-settle.md`

## 每件硬性格式（照核阅 0.3.0 规则，违一件即废）

1. 正文首行（front matter 之后第一个内容行）为 `# <ID> <中文名>`，ID 前缀必须 TOP-。本簇即 `# TOP-006 紧集`、`# TOP-007 连续映射`。
2. 第一个二级标题必须是 `## 定义 {#definition}`。
3. 全文零全角括号中文内容（纯英文数字内容的全角括号可留）、零破折号 `——`、零围栏代码块 ``` 。
4. 每件引哲学命题锚 1 至 2 条。引文纪律五条：从原文复制粘贴逐字节一致；引号按原文件字符（原文用「」或直引号就照抄，勿转义成别种）；连续片段禁止删节（引的必须是原文连续一段）；引文不得含全角括号 `（` 与破折号 `——`；行号用 grep -n 实测后填。
5. 结构照范本八节，节名与锚点逐字：
   `## 定义 {#definition}` / `## 公理条件 {#axioms}` / `## 哲学桥接 {#philosophy-bridge}` / `## 在 facet 的应用 {#facet-application}` / `## 与其他概念的关系 {#relations}` / `## 历史脉络 {#history}` / `## 工程注意事项 {#engineering-notes}` / `## 参考文献 {#references}`。若有值得载明的证明可加 `## 证明思路 {#proof-sketch}`（可选，范本 PROB-007/006 无此节，TOP-005 有）。
6. 数学内容必须正确：定理陈述给条件，反例与边界如实，不确定的内容不写。

## 数学内容要点（正确性锚，非限定，可增补但要准）

### TOP-006 紧集
- 定义：拓扑空间子集 K，每个开覆盖有有限子覆盖（覆盖性定义）；度量空间中等价于序列紧性（每个序列有收敛子列）与完全有界加完备。
- Heine-Borel：R^n 中子集紧当且仅当闭且有界。
- 性质：紧集在 Hausdorff 空间中闭；连续映射把紧集映到紧集（紧性的拓扑不变性）；极值定理：紧空间上实值连续函数必取到最大最小值；有限交性质（一族闭集若任意有限子族有交则全族有交）。
- 反例/边界：有界不蕴含紧（如 (0,1) 在 R 中有界不闭不紧；无穷维单位球不完备意义下有界不紧）；紧性不传递到子集的非闭子集；离散紧当且仅当有限。
- 关系：TOP-004 度量空间（载体）、TOP-005 完备度量空间（紧性需完备+完全有界）、TOP-007 连续映射（紧性经连续映射保持、极值定理）、TOP-001 Banach 不动点（压缩映射在完备度量空间，紧性是其变体语境）。

### TOP-007 连续映射
- 定义：f: X → Y，拓扑定义即开集原像为开集（等价于闭集原像为闭集、邻域原像含邻域）；度量定义即 ε-δ；序列定义即保持收敛序列。三者在一般拓扑与度量空间下的等价性如实标注适用范围。
- 性质：连续映射的复合连续；有限坐标函数连续当且仅当全函数连续（积拓扑）；连续映到 Hausdorff 空间的图像闭。
- 一致连续：单点连续不蕴含一致连续（如实给反例，如 f(x)=x^2 在 R 上连续不一致连续；在紧集上连续函数必一致连续，此为紧性推论）。
- 关系：TOP-004 度量空间、TOP-005 完备度量空间、TOP-006 紧集（紧上连续一致、极值）、TOP-001（压缩映射必连续）、LIM-008 连续函数（分析侧对应概念，标注跨子仓）。

## 候选哲学锚点（已实测干净可命中；请 grep -n 复核行号并逐字节复制，可另选同文件更贴切行）

### TOP-006 紧集（建议 2 锚）
- 锚一 PRO-02 道一「收敛必为」，源 `sih-philosophy/emanation/proodos/02-on-first-tao.md`：
  - 候选 L81：`没有治理，就没有收敛，这是构成性条件，不是偏好`
  - 候选 L115：`道一：发散是默认方向，治理是收敛的构成性条件。`（该行有 `> ` 前缀，引正文部分即可）
  - 桥接立意：紧性（序列紧）保证留痕序列必有收敛子列，是「收敛必为」在拓扑结构上的一组充分条件；决策空间紧时极值存在，决策有确定端点。
- 锚二 PRO-08 应而不藏，源 `sih-philosophy/emanation/proodos/08-on-settle.md`：
  - 候选 L108：`司衡之应：应而不藏，应辨当下，应几未来。`（该行有 `> ` 前缀）
  - 桥接立意：应层留痕形成序列，紧性保证该序列存在收敛子列，使留痕的收敛子结构有存在性保证。

### TOP-007 连续映射（建议 2 锚）
- 锚一 PRO-08 用心若镜（镜映忠实=连续），源 `sih-philosophy/emanation/proodos/08-on-settle.md`：
  - 候选 L228：`应辨：齐物论「应而不藏」+ 应帝王「用心若镜」`（含「」角括号，非全角圆括号，合规）
  - 桥接立意：用心若镜即映物不失真，状态到决策的映射连续即小扰动引起小决策变化、响应无突变失真；连续性是「映」不失真的数学刻画。
- 锚二 PRO-02 道一，源 `sih-philosophy/emanation/proodos/02-on-first-tao.md`：
  - 候选 L80：`收敛不是自发的，它需要外部的、系统的、持续的治理力量`
  - 桥接立意：连续映射保持收敛序列，为治理迭代的收敛传递提供结构条件。
- 注：若嫌两锚都偏 PRO-08/PRO-02，可把 TOP-007 锚二换为 02 文件其它收敛行，但须保持 1 至 2 锚且逐字节可命中。

## 草稿格式

- 文件路径：`/Users/moc/workspaces/SiHankor/agent-drafts/entry/ENTRY-TOP-006.md` 与 `/Users/moc/workspaces/SiHankor/agent-drafts/entry/ENTRY-TOP-007.md`
- front matter 同桥接波：

```
---
entry: ENTRY-TOP-006.md
agent: <你的代理标识，须含 entry-topo 前缀>
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

- 正文即完整条目（八节），front matter 之后第一行是 `# TOP-006 紧集`。
- 注意：引文若含直引号 `"`（如 07 文件那种），YAML 双引号串内须转义；本簇候选行不含直引号，可直接双引号包裹。

## 交稿

1. 写两件草稿文件。
2. 在 `/Users/moc/workspaces/SiHankor/agent-drafts/entry/CLAIMS-A-topology.ndjson` 追加两行认领（一件一行）：`{"entry":"ENTRY-TOP-006.md","agent":"<你的标识>","ts":"<ISO8601 UTC>"}`。
3. 最终回信给主线：两件各报锚点源行号、自查五项（标题前缀/首节名/括号零/破折号零/引文逐字节）自核结果、任何不确定处如实标。
