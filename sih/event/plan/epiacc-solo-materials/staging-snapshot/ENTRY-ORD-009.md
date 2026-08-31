---
entry: "承诺闭包与立名判定"
agent: "复归段补强波簇C"
proposed_id: "ORD-009"
subrepo: "order"
id_reason: "序理论子仓 INDEX 已建至 ORD-005，ORD-006 至 ORD-008 为复归段其他战线分派，下一空号 ORD-009；承诺闭包是集合上的闭包算子，不动点结构承 ORD-002 完全格与 ORD-003 Knaster-Tarski"
anchors:
  - pro: "PRO-01"
    source: "sih-philosophy/emanation/proodos/01-ontology-of-names.md:6"
    quote: '名字不是标签，名字是本体性的，是关于"被命名的是什么"的最早承诺。一旦名字确立，后续的推导、定义、工程实现都受这个名字约束。'
  - pro: "PRO-01"
    source: "sih-philosophy/emanation/proodos/01-ontology-of-names.md:14"
    quote: "司衡认为名字是本体，一个名字一旦确立，它参与构成被命名者"
selfcheck:
  - "自检一 锚引文 grep -nF 复核：第 6 行引文逐字节命中；第 14 行整行含全角括号段 (实体)，按任务包预案改锚为该行不含全角括号的前段，前段逐字节命中，改锚成立"
  - "自检二 三查：全文零全角括号中文内容，零破折号，零围栏代码块，均以 grep 实测"
  - "自检三 结构：正文首行 H1 为 ORD-009 承诺闭包与立名判定，首个二级标题为 定义 {#definition}，八节按固定顺序在场"
  - "自检四 数学主张自问：闭包算子三性质写全且证明 sketch 给出，lfp 存在性与稳定性承 Knaster-Tarski 并写明 Conseq 为后果算子的前提，立名失败两向差集可判定，血统三档互斥且不可比情形如实标为边界，无空话式承接"
  - "自检五 引文检查：两引文均不含全角括号与破折号，第二引文为第 14 行前段 (整行含全角括号)，改锚已记录"
  - "终验 机械验收：verify-entry.py 全绿零失败，锚逐字节命中，H1 与 proposed_id 一致，八节在场，全文 120 行在 60 至 120 区间"
---
# ORD-009 承诺闭包与立名判定

状态：草稿，哲学到工程桥梁条目，复归段补强波簇 C，拟派号待主线入闸批登记。

## 定义 {#definition}

承诺闭包 (Commitment Closure) 是以名字为生成元、以其承诺种子集的闭包为被命名实体的构造，立名判定是实体与实际运作之间的错位判定与术语新旧义关系的可判定判定。

设 $N$ 为名字集，$S$ 为语句与承诺集。$\Sigma: N \to \mathcal{P}(S)$ 是承诺种子映射，$\Sigma(n)$ 是名字 $n$ 直接承诺的承诺集。$cl: \mathcal{P}(S) \to \mathcal{P}(S)$ 是 $\mathcal{P}(S)$ 上的闭包算子，满足三性质：

$$\text{扩张: } X \subseteq cl(X), \quad \text{单调: } X \subseteq Y \implies cl(X) \subseteq cl(Y), \quad \text{幂等: } cl(cl(X)) = cl(X) \label{eq:ord009-closure}$$

名字 $n$ 构成的实体定义为

$$e(n) = cl(\Sigma(n)) \label{eq:ord009-entity}$$

闭包算子的专门条目在序理论子仓待建，本条目以公式 $\ref{eq:ord009-closure}$ 内联固定其三性质，使本条目自足。

主张一 (名字是本体)。主流指称论把名字视为标签，实体独立于名字外生给定；名字本体论的数学形式化是公式 $\ref{eq:ord009-entity}$：实体不是外生给定，而是名字承诺种子的闭包。名字是生成元，实体是其闭包；后续推导、定义、工程实现受名字约束，数学上即一切展开须落在 $e(n)$ 内。

主张二 (承诺不可事后撤回)。

- 幂等与扩张给出 $e(n) = cl(cl(\Sigma(n)))$ 且 $\Sigma(n) \subseteq e(n)$；$\Sigma(n)$ 一经固定，$e(n)$ 由 $cl$ 唯一确定
- 设 $\mathrm{Conseq}: \mathcal{P}(S) \to \mathcal{P}(S)$ 是后果算子，即单调、扩张、幂等三性质同时成立。则 $cl = \mathrm{Conseq}$ 本身是闭包算子，且 $e(n)$ 是单调算子 $F(X) = X \cup \mathrm{Conseq}(\Sigma(n))$ 的最小不动点。证明：$F$ 的不动点即满足 $\mathrm{Conseq}(\Sigma(n)) \subseteq X$ 的 $X$，其最小者为 $\mathrm{Conseq}(\Sigma(n))$ 自身；Knaster-Tarski 定理 (ORD-003) 在完全格 $\mathcal{P}(S)$ 上保证最小不动点存在且稳定，即 $F(\mathrm{lfp}) = \mathrm{lfp}$
- 不可撤回的数学形态：闭包算子的闭集族构成完全格，其包含于 $e(n)$ 的成员恰为 $\{cl(X) \mid X \subseteq e(n)\}$。若承诺 $s \in e(n)$ 满足 $s \notin cl(e(n) \setminus \{s\})$，则 $e(n) \setminus \{s\}$ 不是闭集，移除 $s$ 必须重定义闭包算子本身 (改 Conseq 或改 $\Sigma$)，在给定算子下无操作可实现

主张三 (立名失败判定)。设 $b \subseteq S$ 是被命名者的实际运作，即行为与性质的集合。立名失败当且仅当 $b$ 与 $e(n)$ 错位，分两向：

- 运作逸出承诺闭包：$b \setminus e(n) \neq \emptyset$
- 承诺无对应运作，即过承诺：$e(n) \setminus b \neq \emptyset$

判据：当 $b$ 与 $e(n)$ 可判定 (成员关系可测) 时计算两向差集，任一向非空即立名失败。原文谓错位会自我放大：推导链沿名字的承诺展开，承诺侧随推导单调扩展，被命名者沿另一条路径独立演化，两向差集在演化中不自我消解，判定在每一时刻以该时刻的 $b$ 与 $e(n)$ 进行。

主张四 (血统三档判定)。设 $M_{\mathrm{old}}, M_{\mathrm{new}} \subseteq U$ 为术语旧义与新义的集合，$U$ 为公共全集。

- 保留：$M_{\mathrm{new}} = M_{\mathrm{old}}$
- 延伸：$M_{\mathrm{old}} \subsetneq M_{\mathrm{new}}$
- 限定：$M_{\mathrm{new}} \subsetneq M_{\mathrm{old}}$

三档两两互斥，因相等、真超集、真子集三种关系互斥。边界：$M_{\mathrm{old}}$ 与 $M_{\mathrm{new}}$ 互不含量 (部分重叠) 的情形原文未覆盖，如实标注为模型外情形，不强行归入三档。

主张五 (文本根、义理根与中间态)。设 $\mathrm{src}: N \to \{\text{文本根}, \text{义理根}, \text{中间态}\}$ 是承诺种子的来源标签三分：文本根为直接引经典原文，义理根为依义理延伸，中间态为词组不见于原典而属义理延伸的过渡态。中间态名字登记时须附延伸方向，即登记数据含方向字段，方向缺失则登记不完备。

## 公理条件 {#axioms}

- 集合配置：$N$、$S$ 为集合，$\Sigma: N \to \mathcal{P}(S)$ 是全映射
- 闭包算子：$cl$ 满足扩张、单调、幂等，即公式 $\ref{eq:ord009-closure}$
- 后果算子：主张二的最小不动点陈述要求 Conseq 同时满足单调、扩张、幂等；仅单调时 lfp 存在但 $cl = \mathrm{Conseq}$ 的识别不成立
- 运作的可判定性：主张三要求 $b \subseteq S$ 的成员关系可测；工程实现取 $b$ 为有限可枚举集
- 义集的可判定性：主张四与主张五要求 $M_{\mathrm{old}}$、$M_{\mathrm{new}}$ 是 $U$ 上可判定的子集；工程实现取有限集

主张一与主张二不依赖可判定性，对任意闭包算子成立；主张三至主张五是可判定判定，集合无穷时可判定性是附加条件而非自动性质。

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：Knaster-Tarski 不动点定理与序理论的闭包算子传统，集合论子集关系
- 哲学命题：PRO-01 名字是本体与承诺不能事后撤回 (01- § 名字的本体论地位)，PRO-01-naming-failure 立名失败即名字与实体错位 (01- § 立名失败的形态)，PRO-01-lineage-principles 血统判定三原则 (01- § 追加节 2)
- 形式化：参与构成即闭包生成，$e(n) = cl(\Sigma(n))$，名字是生成元实体是闭包；不可撤回即不动点稳定，$e(n)$ 是单调算子的最小不动点，给定算子下不能从内部移除；立名失败即行为集 $b$ 与承诺闭包 $e(n)$ 的两向差集错位；保留延伸限定三档即子集关系的三分，不可比是原文未覆盖的边界

## 在 facet 的应用 {#facet-application}

- 应用场景：facet 交叉审阅的审阅切面登记与切面漂移检测
- 形式化：每个审阅切面的登记是命名操作，其承诺种子集是切面直接承诺的承诺 (审阅对象、判定范围、输出形态)，切面的判定范围即 $e(n)$；切面对承诺闭包之外的材料给出判定时触发主张三的两向差集判定，非空方向即切面漂移信号
- 借鉴方向：血统三档判定给切面名称的版本演化提供可判定关系，切面改名时按新旧义集判定保留、延伸或限定；文本根、义理根、中间态的来源标签三分给切面名称的登记提供来源字段
- 边界：承诺种子集的具体定义与 Conseq 的具体规则族由 facet 工程层决定，本条目只给判定形态

## 与其他概念的关系 {#relations}

- ORD-001 偏序集：$\mathcal{P}(S)$ 配以包含是偏序集，是本条目的承载结构
- ORD-002 完全格：$\mathcal{P}(S)$ 是完全格，闭集族也构成完全格 (闭集族的并的闭包是上确界)，最小不动点的存在承此结构
- ORD-003 Knaster-Tarski 不动点：主张二的最小不动点存在性与稳定性 $F(\mathrm{lfp}) = \mathrm{lfp}$ 直接承 ORD-003
- ORD-004 monotone operator：$F(X) = X \cup \mathrm{Conseq}(\Sigma(n))$ 是 $\mathcal{P}(S)$ 上的单调算子
- ALG-008 线性映射的核与纤维：立名失败是两向差集错位，ALG-008 中意图到代码映射的错位是纤维多重性，两者同属承诺与运作不对齐的可判定形态

## 历史脉络 {#history}

- 1928 年 Knaster 给出幂集格上闭包类集合的不动点结果，闭集族构成完全格的结构进入序理论
- 1955 年 Tarski 证明完全格上单调算子的不动点定理 (ORD-003)
- 闭包算子进入程序语义与抽象解释，作为最小不动点即语义的承载 (Cousot-Cousot 1979)，与本文字本体论的最小不动点即实体结构同构
- 哲学仓 01- 追加节 2 与追加节 5 的血统三档与中间态机制，是子集关系三分在治理侧的对应物

## 工程注意事项 {#engineering-notes}

应用承诺闭包与立名判定时需验证五件事。

1. 闭包算子是否良定义：扩张、单调、幂等须逐条检验；幂等最易遗漏，须用非闭集 $X$ 测 $cl(cl(X)) = cl(X)$
2. 运作集是否可判定：立名失败判定要求 $b$ 的成员关系可测，$b$ 无穷时须有有限采样或枚举方案，方案随判定记录一并登记
3. 全集是否先固定：血统三档判定在公共全集 $U$ 上进行，$M_{\mathrm{old}}$ 与 $M_{\mathrm{new}}$ 的全集不对齐会把不可比误判为某一档
4. 不可比边界的处置：$M_{\mathrm{old}}$ 与 $M_{\mathrm{new}}$ 互不含量时报告为模型外情形，不强行归入三档，也不静默丢弃
5. 改锚留痕：本条目第二锚因第 14 行整行含全角括号改取前段，改锚事实记入 front matter 的 selfcheck，入闸批核验时以本记录为准

## 参考文献 {#references}

- Tarski, A. (1955). A lattice-theoretical fixpoint theorem and its applications. Pacific J. Math., 5(2), 285-309
- Knaster, B. (1928). Un théorème sur les fonctions d'ensembles. Ann. Soc. Polon. Math., 6, 133-134
- Cousot, P. & Cousot, R. (1979). Constructive versions of Tarski's fixpoint theorem in abstract interpretation. Springer
- Davey, B.A. & Priestley, H.A. (2002). Introduction to Lattices and Order, 2nd ed. Cambridge University Press
- Grätzer, G. (2011). Lattice Theory: Foundation. Springer
- Wikipedia "Closure operator" 条目
