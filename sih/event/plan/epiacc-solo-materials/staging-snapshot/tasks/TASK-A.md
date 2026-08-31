# 任务包 A：复归段补强波 簇 A（EPI-12 与 EPI-13）

你是司衡工作区复归段补强波的子代理，负责簇 A 两件数学承载条目草稿。

## 边界铁律

- 只允许写 /Users/moc/workspaces/SiHankor/agent-drafts/epi/ 下文件，其余一切只读
- 禁止入仓、跑 lease/scribe/git、改 AGENTS.md 与任何 INDEX/mapping、上链
- ID 与表登记由主线入闸批承载，你只在草稿 front matter 记拟派号

## 交付物（两件草稿，文件名即定名）

1. /Users/moc/workspaces/SiHankor/agent-drafts/epi/ENTRY-ORD-007.md（EPI-12 载体）
2. /Users/moc/workspaces/SiHankor/agent-drafts/epi/ENTRY-ORD-008.md（EPI-13 载体）

某件确无诚实数学载体时，改写 /Users/moc/workspaces/SiHankor/agent-drafts/epi/analysis/REFUSE-<命题ID>.md 说明理由，该件草稿不写。两件的载体均已预判存在（见下数学内核），预期不触发 REFUSE。

## 共读材料（先读后写）

- 原文：sih-philosophy/emanation/epistrophe/12-on-epistrophe.md 与 13-on-grounding.md，全文读
- 知识包条目：sih-philosophy/llm-friendly-build/entries/EPI-12.md 与 EPI-13.md
- 风格范本：sih-math/probability/entries/PROB-007-expectation.md（八节结构与行文密度参照它）
- 序理论子仓已建条目（供 关系 节引用）：sih-math/order/entries/ 下 ORD-001 至 ORD-005

## 硬性格式（违一件即废）

1. 正文首行 `# ORD-007 推衍链的健全性与镜像检验` 或 `# ORD-008 引用图可达与孤悬判定`，ID 与中文名不可改
2. 第一个二级标题必须是 `## 定义 {#definition}`
3. 全文零全角括号中文内容（纯英文数字内容的括号可留，如 (Expectation) 式）、零破折号 ——、零围栏代码块；行内数学 $...$ 与展示数学 $$...$$ 允许
4. 结构八节顺序固定：`## 定义 {#definition}` / `## 公理条件 {#axioms}` / `## 哲学桥接 {#philosophy-bridge}` / `## 在 facet 的应用 {#facet-application}` / `## 与其他概念的关系 {#relations}` / `## 历史脉络 {#history}` / `## 工程注意事项 {#engineering-notes}` / `## 参考文献 {#references}`
5. front matter 字段齐全：entry / agent（填 复归段补强波簇A）/ proposed_id / subrepo（填 order）/ id_reason（一行选号理由）/ anchors / selfcheck
6. 锚 1 至 2 条，引文从原文复制粘贴逐字节一致，行号用 grep -n 实测后填入，引文不得含全角括号与破折号，锚 pro 必须是本战线命题（EPI-12 或 EPI-13），source 必须是本战线原文路径

## ENTRY-ORD-007（EPI-12）

front matter 锚点已预验，可直接使用（写前仍须自行 grep -nF 复核一遍）：

```yaml
anchors:
  - pro: EPI-12
    source: sih-philosophy/emanation/epistrophe/12-on-epistrophe.md:9
    quote: 但单向流衍有结构弱点：它只能证明「每步从前步来」，不能证明「每步真的」。
  - pro: EPI-12
    source: sih-philosophy/emanation/epistrophe/12-on-epistrophe.md:37
    quote: Epistrophe 是反推证立链。它从命题出发，向下追索其有效性与独立性，回答「这个命题是否真」。
```

数学内核（形式化节必须给出，禁止空话式承接）：

- 定义：带锚推衍链。语言 L，锚集 A 是 L 的子集，命题序列 p1 至 pn 满足对每个 i，pi 属于 Cn(前序步骤集并 A)，其中 Cn 是语义后果算子（Cn(Gamma) = 被 Gamma 语义蕴含的全体句子）。该条件形式化「每步从前步来」。
- 定义：健全性。TRUTH 为 L 中全体真句子集合，链健全当且仅当对每个 i，pi 属于 TRUTH。
- 定理（健全性定理，带条件）：若 (一) 锚集与前提皆真即 A 及链首前提属于 TRUTH，(二) Cn 保真即 Gamma 含于 TRUTH 蕴含 Cn(Gamma) 含于 TRUTH，则链全体步骤皆真。证明对 i 归纳，须写出归纳步骤。
- 核心主张（EPI-12 结构弱点的数学形态）：「每步从前步来」只保证每步属于前序的 Cn，是局部性质，逐边可检；「每步真的」是全局性质，依赖链首与锚集的真值，正向推衍不检查它。两者不等价：存在每步皆从前步来而某锚假的链，其终点为假。给出一个最小反例（两三步的链，锚假，各步均在 Cn 内，终点假）。
- 镜像方向（复归检验的两项）：(一) 有效性即从末端命题逐步向下追溯每步的 Cn 归属，是推衍嵌入的缩回；(二) 独立性即 pi 不属于 Cn(其余全部步骤并 A)，承独立公理标准判据（句子对理论独立当且仅当该理论不语义蕴含该句子）。
- 适用边界如实标注：Cn 可判定性依赖语言与理论；对不可判定理论，Cn 归属检验是半可判定的（可列出推衍，不可列全非推衍），如实标注不掩饰。

哲学桥接节要点：借鉴源填 Tarski 语义后承与健全性定理传统、Kuratowski 闭包算子；哲学命题 EPI-12 单向流衍链结构弱点与复归镜像检验；形式化说明「每步从前步来不等于每步真的」如何被局部性质与全局性质之分形式化，镜像即缩回方向。

## ENTRY-ORD-008（EPI-13）

front matter 锚点已预验：

```yaml
anchors:
  - pro: EPI-13
    source: sih-philosophy/emanation/epistrophe/13-on-grounding.md:7
    quote: 从 11- 倒推至 00-，检查每步核心命题是否被后续步骤引用或修正。
  - pro: EPI-13
    source: sih-philosophy/emanation/epistrophe/13-on-grounding.md:89
    quote: 逐步检查每个链步的核心命题，去掉「代码工程」「AI」「代码」等限定词后是否仍成立。
```

数学内核：

- 定义：链引用图。V 为链步集合（按链序拓扑排序），边分两类：引用边与修正边；p 到 q 的边表示 q 被后续步骤引用或修正。
- 定义：可达性。R 为边集的传递闭包；R 在无反环区域构成偏序（有向无环图可达性是偏序，与 ORD-001 偏序集、ORD-005 链与反链衔接）。
- 定义：孤悬节点。p 为孤悬当且仅当 p 非末端步骤且其出度为 0（无任何后续步骤引用或修正它）。
- 判定（落地检测一）：链通过落地检测当且仅当无孤悬节点。可判定性：对每个非末端步骤扫描后续步骤的引用与修正，有限机械检查。
- 定义（落地检测二，去语境化）：带语境限定的命题 P_C 是限定在语境域 D_C 上的谓词（D_C 如代码工程对象类）；去语境化命题 P_D 是同一谓词在无限制域 D 上的表述，D_C 是 D 的子集。检测即 P 在 D 上是否仍成立：通过当且仅当 D 减 D_C 中不存在使 P 不成立的元素。给出判定形式（补域反例搜索），并如实标注：补域无限时该检验是可判定性不保证的，须以有界反例搜索近似。
- 总判定：落地检测二维，引用孤悬检测与去语境化检测，两维皆过方为通过；只过一维的链须标注受限语境。

哲学桥接节要点：借鉴源填图论可达性与传递闭包、谓词论域限制；哲学命题 EPI-13 落地检测（倒推检查引用与修正、识别孤悬命题）与去语境化检验；形式化说明倒推检查即出度为零检测、去语境化即论域从 D_C 扩至 D 后谓词仍成立的检验。

## 公共质量要求（宁缺毋滥）

- 定义与公理条件节必须给出可判定的数学主张，不确定的陈述不写
- 反例与适用边界如实标注，强行编造形式化即失败
- 工程注意事项节写三至五条具体核验动作（编号列表）
- 参考文献节含经典文献（Tarski / Kuratowski / 图论教材 / 逻辑教材等真实条目）
- 全文 60 至 120 行

## 自检（写完后跑，结果记入 selfcheck 字段）

1. grep -nF 逐条复核两个锚引文在指定行逐字节命中
2. grep 全文：全角括号中文内容（（ 后接中文）、破折号 ——、围栏代码块 ``` 三查皆零
3. 首行 H1、首个二级标题、八节在场
4. 数学主张逐条自问：条件写全了吗，证明步骤对吗，反例最小吗
5. 引文是否含全角括号或破折号（有则换行，不可改引文）
