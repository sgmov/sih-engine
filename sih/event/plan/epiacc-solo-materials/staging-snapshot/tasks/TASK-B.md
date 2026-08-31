# 任务包 B：复归段补强波 簇 B（EPI-14 与 EPI-15）

你是司衡工作区复归段补强波的子代理，负责簇 B 两件数学承载条目草稿。

## 边界铁律

- 只允许写 /Users/moc/workspaces/SiHankor/agent-drafts/epi/ 下文件，其余一切只读
- 禁止入仓、跑 lease/scribe/git、改 AGENTS.md 与任何 INDEX/mapping、上链
- ID 与表登记由主线入闸批承载，你只在草稿 front matter 记拟派号

## 交付物（两件草稿，文件名即定名）

1. /Users/moc/workspaces/SiHankor/agent-drafts/epi/ENTRY-ORD-006.md（EPI-14 载体）
2. /Users/moc/workspaces/SiHankor/agent-drafts/epi/ENTRY-ORD-010.md（EPI-15 载体）

某件确无诚实数学载体时，改写 /Users/moc/workspaces/SiHankor/agent-drafts/epi/analysis/REFUSE-<命题ID>.md 说明理由，该件草稿不写。两件的载体均已预判存在（见下数学内核），预期不触发 REFUSE。

## 共读材料（先读后写）

- 原文：sih-philosophy/emanation/epistrophe/14-on-reducibility.md 与 15-on-falsifiability.md，全文读
- 知识包条目：sih-philosophy/llm-friendly-build/entries/EPI-14.md 与 EPI-15.md
- 风格范本：sih-math/probability/entries/PROB-007-expectation.md（八节结构与行文密度参照它）
- 序理论子仓已建条目（供 关系 节引用）：sih-math/order/entries/ 下 ORD-001 至 ORD-005

## 硬性格式（违一件即废）

1. 正文首行 `# ORD-006 闭包算子与后果算子` 或 `# ORD-010 证伪条件与反例搜索`，ID 与中文名不可改
2. 第一个二级标题必须是 `## 定义 {#definition}`
3. 全文零全角括号中文内容（纯英文数字内容的括号可留）、零破折号 ——、零围栏代码块；行内数学 $...$ 与展示数学 $$...$$ 允许
4. 结构八节顺序固定：`## 定义 {#definition}` / `## 公理条件 {#axioms}` / `## 哲学桥接 {#philosophy-bridge}` / `## 在 facet 的应用 {#facet-application}` / `## 与其他概念的关系 {#relations}` / `## 历史脉络 {#history}` / `## 工程注意事项 {#engineering-notes}` / `## 参考文献 {#references}`
5. front matter 字段齐全：entry / agent（填 复归段补强波簇B）/ proposed_id / subrepo（填 order）/ id_reason（一行选号理由）/ anchors / selfcheck
6. 锚 1 至 2 条，引文从原文复制粘贴逐字节一致，行号用 grep -n 实测后填入，引文不得含全角括号与破折号，锚 pro 必须是本战线命题（EPI-14 或 EPI-15），source 必须是本战线原文路径

## ENTRY-ORD-006（EPI-14）

front matter 锚点已预验，可直接使用（写前仍须自行 grep -nF 复核一遍）：

```yaml
anchors:
  - pro: EPI-14
    source: sih-philosophy/emanation/epistrophe/14-on-reducibility.md:29
    quote: 判定标记：可归约 / 跳跃引入。
  - pro: EPI-14
    source: sih-philosophy/emanation/epistrophe/14-on-reducibility.md:117
    quote: 无跳跃引入。Daoist 范畴作为外部锚定统一在 00- + 01- 的命名传统选择中，不构成外部预设的跳跃。
```

数学内核（形式化节必须给出，禁止空话式承接）：

- 定义：闭包算子。设 S 是集合，cl 是 P(S) 到 P(S) 的映射，cl 为闭包算子当且仅当满足三公理：扩张性（X 含于 cl(X)）、单调性（X 含于 Y 蕴含 cl(X) 含于 cl(Y)）、幂等性（cl(cl(X)) = cl(X))。给出经典实例（拓扑闭包、子空间生成、凸包、逻辑后果）。
- 定义：后果算子。语言 L 上 Cn(Gamma) = 被 Gamma 语义蕴含的全体句子；证明 Cn 满足闭包三公理（逐条给论证：扩张性由 Gamma 语义蕴含 Gamma 自身，单调性由语义蕴含的单调性，幂等性由语义蕴含的传递性）。
- 判定（EPI-14 检测的数学形态）：链 p1 至 pn，锚集 A（预锚定，如道家范畴或外部锚定，是唯一允许的外部输入）。第 i 步可归约当且仅当 pi 属于 Cn(前序步骤集并 A)；链通过 Reducibility 检测当且仅当每步可归约；跳跃引入即存在 i 使 pi 不属于 Cn(前序步骤集并 A)，即 pi 依赖 A 与前序之外的句子。
- 主张（Reducibility 与 Derivability 之分）：可归约锚定语义后承 Cn，可推性锚定特定语法体系的语法推衍；在健全且完备的演算中两者重合，但判定标记选语义侧使判定不依赖公理化的选择。如实标注：重合依赖完备性，不完备演算中语法推衍严格小于语义后承，此时两标记可分。
- 锚集规则：加入 A 不构成跳跃（EPI-14 外部锚定统一于命名传统选择）；依赖 A 之外者构成跳跃。可判定性：A 有限且 Cn 可判定时，检测为有限次成员判定，机械可执行。

哲学桥接节要点：借鉴源填 Tarski 后果算子与闭包算子公理传统、Knaster-Tarski 不动点（闭包算子不动点是完全格，衔接 ORD-002 与 ORD-003）；哲学命题 EPI-14 向上收拢归一与无外部跳跃检验；形式化说明每步可归约到前步即每步属于前序与锚集的后果闭包，跳跃即闭包外元素。

## ENTRY-ORD-010（EPI-15）

front matter 锚点已预验：

```yaml
anchors:
  - pro: EPI-15
    source: sih-philosophy/emanation/epistrophe/15-on-falsifiability.md:55
    quote: 每步命题的证伪条件须满足三要素：
  - pro: EPI-15
    source: sih-philosophy/emanation/epistrophe/15-on-falsifiability.md:204
    quote: 哲学链存在 3 步不可证伪命题——它们不是「immune to falsification」的伪命题，而是属于不同认识层的命题：
```

注意：第二条引文含破折号，但验收规则要求引文不含 ——，主线已改用该行不含破折号的前段。写前用 grep -nF 复核；若该前段仍含 —— 或全角括号，改锚 15-on-falsifiability.md 第 61 行「缺失任一要素，该命题的证伪条件不成立。」（已预验干净），并在 selfcheck 记录改锚。

数学内核：

- 定义：观测空间与观测映射。现象空间 Omega_phen，观测映射 o 从 Omega_phen 到观测空间 Omega；观测对应物条件（要素一）即 Omega 等于 o 的值域，观测点皆有可观察现象对应。
- 定义：测试谓词。T 是 Omega 到 0 与 1 的谓词。命题 P 具有证伪条件 T 当且仅当 P 逻辑等价于「Omega 上全体 x 满足非 T(x)」，即 P 被证伪当且仅当存在 x 使 T(x) 为 1。
- 三要素形式化：(一) 观测对应物即上述值域条件；(二) 独立性即 T 的定义不依赖 P 的支持者，可判定的表述是 T 在基础观测语言中定义、其定义闭包不含 P 的专名谓词（语法非循环条件，可按谓词集检查）；(三) 可重复性即 T 可判定，存在机械过程对任意 x 有限步输出 T(x)。
- 定理（反例搜索半可判定性，带条件）：T 可判定且 P 等价于 Omega 上全称非 T 时：(一) 若 P 假，则按固定枚举序遍历 Omega 并计算 T 的反例搜索有限步停机并输出反例；(二) 若 P 真，搜索可不保证停机，不停机不是 P 真的证据。即非 P 是半可判定的（递归可枚举），P 是共半可判定的。写出 Omega 可数且枚举给定的前提。
- 四态判定（每态给判据，判据不机械处如实标注为哲学判据）：可证伪即存在满足三要素的 T 使 P 等价于 Omega 上全称非 T；部分可证伪即 P 可分解为 Q 与 R 的合取，Q 可证伪而 R 否；不可证伪即 P 属于定义集 DEF 的后果闭包 Cn(DEF)（承 ORD-006，无需观测即判）；自明即 P 为定义重言，属语言定义闭包。
- 主张（不可证伪不等于伪命题）：不可证伪的 P 可为真（重言真 Tautology 不可证伪且真）；四态按证立方式分类，不按真假分类；不可证伪命题属定义、逻辑、定理层，其证立由定义与逻辑形式担保，不依赖经验观测，这与无观察对应物的空话有本质区别。

哲学桥接节要点：借鉴源填 Popper 可证伪性判据、递归论与算术层级（Pi-1 句子与半可判定性）；哲学命题 EPI-15 证伪条件三要素与四态判定与不可证伪不等于伪命题；形式化说明三要素即值域条件、语法非循环、可判定性，四态即按证立方式的分类。

## 公共质量要求（宁缺毋滥）

- 定义与公理条件节必须给出可判定的数学主张，不确定的陈述不写
- 反例与适用边界如实标注，强行编造形式化即失败
- 工程注意事项节写三至五条具体核验动作（编号列表）
- 参考文献节含经典文献（Tarski / Kuratowski / Popper / 递归论教材如 Rogers 或 Soare / 模型论教材等真实条目）
- 全文 60 至 120 行

## 自检（写完后跑，结果记入 selfcheck 字段）

1. grep -nF 逐条复核锚引文在指定行逐字节命中
2. grep 全文：全角括号中文内容、破折号 ——、围栏代码块 ``` 三查皆零
3. 首行 H1、首个二级标题、八节在场
4. 数学主张逐条自问：条件写全了吗，证明步骤对吗，判据是否机械，哲学判据处是否如实标注
5. 引文是否含全角括号或破折号（有则换行，不可改引文）
