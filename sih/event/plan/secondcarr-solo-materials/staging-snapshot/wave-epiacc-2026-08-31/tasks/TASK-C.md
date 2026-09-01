# 任务包 C：复归段补强波 簇 C（PRO-01 与 PRO-03 与 PRO-04）

你是司衡工作区复归段补强波的子代理，负责簇 C 三件数学承载条目草稿。

## 边界铁律

- 只允许写 /Users/moc/workspaces/SiHankor/agent-drafts/epi/ 下文件，其余一切只读
- 禁止入仓、跑 lease/scribe/git、改 AGENTS.md 与任何 INDEX/mapping、上链
- ID 与表登记由主线入闸批承载，你只在草稿 front matter 记拟派号

## 交付物（三件草稿，文件名即定名）

1. /Users/moc/workspaces/SiHankor/agent-drafts/epi/ENTRY-ORD-009.md（PRO-01 载体）
2. /Users/moc/workspaces/SiHankor/agent-drafts/epi/ENTRY-ALG-008.md（PRO-03 载体）
3. /Users/moc/workspaces/SiHankor/agent-drafts/epi/ENTRY-ALG-009.md（PRO-04 载体）

某件确无诚实数学载体时，改写 /Users/moc/workspaces/SiHankor/agent-drafts/epi/analysis/REFUSE-<命题ID>.md 说明理由，该件草稿不写。三件的载体均已预判存在（见下数学内核），预期不触发 REFUSE。

## 共读材料（先读后写）

- 原文：sih-philosophy/emanation/proodos/01-ontology-of-names.md（728 行，重点读 § 名字的本体论地位、§ 立名失败的形态、§ 追加节 2 血统判定三原则、§ 追加节 5 命名机制中间态）、03-on-second-tao.md、04-on-third-tao.md，后两件全文读
- 知识包条目：sih-philosophy/llm-friendly-build/entries/ 下 PRO-01-name-ontology.md、PRO-01-naming-failure.md、PRO-01-lineage-principles.md、PRO-03.md、PRO-04.md
- 风格范本：sih-math/probability/entries/PROB-007-expectation.md（八节结构与行文密度参照它）
- 序理论与代数子仓已建条目（供 关系 节引用）：sih-math/order/entries/ 下 ORD-001 至 ORD-005、sih-math/algebra/entries/ 下 ALG-001

## 硬性格式（违一件即废）

1. 正文首行分别为 `# ORD-009 承诺闭包与立名判定` / `# ALG-008 线性映射的核与纤维` / `# ALG-009 秩零化度与截面`，ID 与中文名不可改
2. 第一个二级标题必须是 `## 定义 {#definition}`
3. 全文零全角括号中文内容（纯英文数字内容的括号可留）、零破折号 ——、零围栏代码块；行内数学 $...$ 与展示数学 $$...$$ 允许
4. 结构八节顺序固定：`## 定义 {#definition}` / `## 公理条件 {#axioms}` / `## 哲学桥接 {#philosophy-bridge}` / `## 在 facet 的应用 {#facet-application}` / `## 与其他概念的关系 {#relations}` / `## 历史脉络 {#history}` / `## 工程注意事项 {#engineering-notes}` / `## 参考文献 {#references}`
5. front matter 字段齐全：entry / agent（填 复归段补强波簇C）/ proposed_id / subrepo / id_reason（一行选号理由）/ anchors / selfcheck
6. 锚 1 至 2 条，引文从原文复制粘贴逐字节一致，行号用 grep -n 实测后填入，引文不得含全角括号与破折号，锚 pro 必须是本战线命题（PRO-01 或 PRO-03 或 PRO-04），source 必须是本战线原文路径

## ENTRY-ORD-009（PRO-01，subrepo 填 order）

id_reason 参考：序理论子仓 INDEX 已建至 ORD-005，下一空号 ORD-009；承诺闭包是集合上的闭包算子，不动点结构承 ORD-002 完全格与 ORD-003 Knaster-Tarski。

front matter 锚点已预验（写前仍须自行 grep -nF 复核）：

```yaml
anchors:
  - pro: PRO-01
    source: sih-philosophy/emanation/proodos/01-ontology-of-names.md:6
    quote: 名字不是标签，名字是本体性的，是关于"被命名的是什么"的最早承诺。一旦名字确立，后续的推导、定义、工程实现都受这个名字约束。
  - pro: PRO-01
    source: sih-philosophy/emanation/proodos/01-ontology-of-names.md:14
    quote: 司衡认为名字是本体，一个名字一旦确立，它参与构成被命名者（实体）：
```

注意：第二条引文含全角括号（实体），验收规则要求引文不含全角括号。写前 grep -nF 复核该行；若确含（），改锚为第 6 行单锚或另找该行不含全角括号的前段「司衡认为名字是本体，一个名字一旦确立，它参与构成被命名者」（已预验为逐字节子串），并在 selfcheck 记录改锚。

数学内核：

- 定义：命名系统。N 为名字集，S 为语句与承诺集，Sigma 是 N 到 P(S) 的承诺种子映射（名字承诺的承诺集），cl 是 P(S) 上的闭包算子（承 ORD-006）。名字 n 构成的实体 e(n) = cl(Sigma(n))。
- 主张一（名字是本体）：「名字参与构成被命名者」形式化为 e(n) = cl(Sigma(n))：实体不是独立于名字的外生给定，而是名字承诺种子的闭包；名字是生成元，实体是其闭包。
- 主张二（承诺不可事后撤回）：闭包算子幂等且扩张，e(n) = cl(cl(Sigma(n))) 且 Sigma(n) 含于 e(n)；Sigma(n) 一经固定，e(n) 唯一确定。不可撤回的数学形态：e(n) 是单调算子 F(X) = X 并 Conseq(Sigma(n)) 的最小不动点（Conseq 为后果规则族），Knaster-Tarski 保证最小不动点存在且稳定，即 F(lfp) = lfp；移除闭包内承诺须重定义闭包算子本身，在给定算子下无操作可实现。
- 主张三（立名失败判定）：被命名者实际运作 b 是行为与性质的集合；立名失败当且仅当 b 与 e(n) 错位，分两向：b 不含量于 e(n) 即运作逸出承诺闭包，e(n) 不含量于 b 即承诺无对应运作（过承诺）。判据：b 与 e(n) 可判定时计算两向差集，任一向非空即立名失败。
- 主张四（血统三档判定）：术语新旧义集合 M_old 与 M_new 是全集 U 的子集：保留当且仅当 M_new = M_old，延伸当且仅当 M_old 真含于 M_new，限定当且仅当 M_new 真含于 M_old。三档两两互斥；不可比情形 M_old 与 M_new 互不含量原文未覆盖，如实标注为边界。
- 主张五（文本根与义理根与中间态）：承诺种子的来源标签三分：文本根（直接引经典原文）、义理根（依义理延伸）、中间态（词组不见于原典而属义理延伸的过渡态，登记时须附延伸方向）。

哲学桥接节要点：借鉴源填 Knaster-Tarski 不动点定理与闭包算子传统、集合论子集关系；哲学命题 PRO-01 名字是本体与承诺不可撤回与立名失败形态与血统三原则；形式化说明参与构成即闭包生成、不可撤回即不动点稳定、立名失败即行为集与承诺闭包错位、三档即子集关系三分。

## ENTRY-ALG-008（PRO-03，subrepo 填 algebra）

id_reason 参考：代数子仓 INDEX 已建 ALG-001，ALG-002 至 ALG-007 为预约待建概念（向量空间、线性映射等），下一空号 ALG-008；核与纤维是线性映射论核心概念，线性模型为一般映射论提供可计算工具。

front matter 锚点已预验：

```yaml
anchors:
  - pro: PRO-03
    source: sih-philosophy/emanation/proodos/03-on-second-tao.md:15
    quote: 道二：意图先于代码。
  - pro: PRO-03
    source: sih-philosophy/emanation/proodos/03-on-second-tao.md:19
    quote: 这是代码工程的因果方向：意图 -> 代码。不可逆。
```

数学内核：

- 定义：设 V 与 W 是域 F 上的向量空间，f 是 V 到 W 的线性映射。核 ker f = V 中映到 0 的元素集；像 im f = f(V)。
- 定义：纤维。对 w 属于 W，f 在 w 的纤维 f^-1(w) = V 中映到 w 的元素集；w 不在 im f 时纤维为空，否则取 v0 使 f(v0) = w，则 f^-1(w) = v0 加 ker f，是 ker f 的仿射陪集。
- 定理一：f 单射当且仅当 ker f = 0 向量空间。写出双向证明。
- 定理二（纤维结构）：f 的所有非空纤维彼此双射；有限维时各非空纤维元素数等于 ker f 的维数对应的基数，一般情形为等基数。
- 定理三（左逆存在性）：f 在像上存在左逆 g（g 从 im f 到 V 且 g 复合 f 等于恒等）当且仅当 f 单射。并给出一般映射版本：任意映射 f 从 I 到 C 在像上存在左逆当且仅当 f 单射（该版本不依赖线性结构）。
- 主张（PRO-03 载体）：实现关系「意图到代码」建模为映射 f 从 I（意图集）到 C（代码集）。「意图先于代码」即代码在像中，每个代码是某个意图的像；「因果方向不可逆」即 f 一般非单射，纤维含多个意图，像上无左逆即从代码恢复意图的映射不存在。线性模型中：不可逆等价于 ker f 非零，歧义集为陪集 v0 加 ker f。
- 诚实标注：一般映射版本（单射等价于可恢复）对任意映射成立；向量空间模型提供核的可计算工具（核是子空间、维数可算）；线性假设是一阶模型，不是对意图结构的断言。

哲学桥接节要点：借鉴源填线性代数核像理论、映射论纤维与左逆经典结果；哲学命题 PRO-03 道二意图先于代码与因果方向不可逆；形式化说明意图先于代码即代码属实现像、不可逆即非单射与无左逆、歧义即纤维多重性。

## ENTRY-ALG-009（PRO-04，subrepo 填 algebra）

id_reason 参考：紧承 ALG-008 的下一空号 ALG-009；秩零化度与截面是有限维线性映射论核心定理，承接核像工具量化信息损失并给出恢复的数学形态。

front matter 锚点已预验：

```yaml
anchors:
  - pro: PRO-04
    source: sih-philosophy/emanation/proodos/04-on-third-tao.md:15
    quote: 道三：代码自晦，意图必复。
  - pro: PRO-04
    source: sih-philosophy/emanation/proodos/04-on-third-tao.md:72
    quote: 把道二"意图 -> 代码"的因果链具体化：因果链中有信息损失，损失是"自晦"的客观属性
```

注意：第二条引文以行内前缀「- 」开头，引文取该前缀之后的连续片段，已预验为逐字节子串；写前仍须自行 grep -nF 复核。

数学内核：

- 定义与定理（秩零化度）：设 V 是有限维向量空间，f 是 V 到 W 的线性映射，则 dim V = rank f + dim ker f，其中 rank f = dim im f。写出标准证明（先证 ker f 与某补空间直和分解 V，再比维数）。
- 主张一（信息损失量化）：损失维数 = dim ker f；f 无损（单射）当且仅当 dim ker f = 0 当且仅当 rank f = dim V，必要条件 dim W 不小于 dim V。
- 主张二（鸽笼原理与基数版本，非线性的诚实形式）：编码空间 C_n 为长度 n 的有限码字集，I 为意图集；若 |I| 大于 |C_n|，则任何编码映射从 I 到 C_n 非单射，至少两个意图共享同一码字。「符号系统必有信息损失」的可判定形式即该基数不等式；线性模型中即 dim W 小于 dim V 时任何线性编码有损。
- 定义与定理（截面即恢复映射）：s 从 im f 到 V 是 f 的截面当且仅当 f 复合 s 等于 im f 上恒等。定理：截面存在当且仅当 V 可分解为 ker f 与某子空间 U 的直和，此时 s 把 w 映到其唯一 U 分量；U 固定则 s 唯一确定，U 不固定则 s 不唯一。「意图必复」的数学形态：恢复映射 s 须由外部提供分裂子空间 U（先验、语境、边信息），f 自身不能确定唯一的 s。
- 主张三（恢复残差）：设真意图 i 属于 V，代码 c = f(i)，恢复输出 s(c)；则 i 与 s(c) 之差属于 ker f，残差恰为核元素。技艺改进只改变残差幅度不改变其存在性，当且仅当 dim ker f 大于 0 时。
- 诚实标注：Shannon 信息论锚定在原文中自我声明为启发性类比而非严格数学推导，本条目的损失量化在有限维线性模型内成立，对一般符号系统只保留基数版本（鸽笼原理）的严格性，不越出模型声称。

哲学桥接节要点：借鉴源填有限维线性代数秩零化度定理、直和分解与截面、鸽笼原理；哲学命题 PRO-04 道三代码自晦意图必复与符号系统必有信息损失；形式化说明自晦即编码有损（核非零或基数不等式）、必复即恢复是截面且须外部边信息、残差即核元素。

## 公共质量要求（宁缺毋滥）

- 定义与公理条件节必须给出可判定的数学主张，不确定的陈述不写
- 反例与适用边界如实标注，强行编造形式化即失败
- 工程注意事项节写三至五条具体核验动作（编号列表）
- 参考文献节含经典文献（线性代数教材如 Strang / Hoffman-Kunze / Ax、序理论与不动点如 Davey-Priestley / Grätzer 等真实条目）
- 每件全文 60 至 120 行

## 自检（写完后跑，结果记入 selfcheck 字段）

1. grep -nF 逐条复核锚引文在指定行逐字节命中
2. grep 全文：全角括号中文内容、破折号 ——、围栏代码块 ``` 三查皆零
3. 首行 H1、首个二级标题、八节在场
4. 数学主张逐条自问：定理条件写全了吗，证明步骤对吗，模型边界是否诚实标注
5. 引文是否含全角括号或破折号（有则换行或取干净前段，不可改引文）
