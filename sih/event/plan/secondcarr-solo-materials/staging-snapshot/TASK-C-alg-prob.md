# 复归段补强波 簇 C 任务书（PRO-03 代数件 + PRO-04 概率件）

编排主代理落盘。你是复归段补强波簇 C 子代理，零判定权：按本任务书起草两件数学条目草稿，机械格式与数学正确性由主线验收，你不做内容裁量，拿不准的数学陈述不写。

## 边界铁律

- 只允许写 /Users/moc/workspaces/SiHankor/agent-drafts/epi/ 下你的两件产出：草稿 ENTRY-ALG-010.md、ENTRY-PROB-009.md 与认领行 CLAIMS-C.ndjson
- 其余一切只读。禁止入仓、跑 lease/scribe/git、改 AGENTS.md 与任何 INDEX/mapping、上链
- 拟派号只记在草稿 front matter 的 proposed_id，主线入闸批才做 ID 与表登记

## 硬性格式法（违一件即废，先跑后写）

1. 首行 front matter 后正文第一非空行即 H1，形如 `# ALG-010 因式分解与不变性`，前缀限 ALG 或 PROB
2. 第一个二级标题必须是 `## 定义 {#definition}`
3. 全文零全角括号中文内容：全角括号内只许纯 ASCII（英文名、ID 编号），中文补充一律改逗号句或「即」字句；零破折号——；零围栏代码块（LaTeX 用 $ 与 $$）
4. 结构照范本 /Users/moc/workspaces/SiHankor/sih-math/probability/entries/PROB-007-expectation.md：H1、状态行、八节 定义/公理条件/哲学桥接/在 facet 的应用/与其他概念的关系/历史脉络/工程注意事项/参考文献，锚点照抄。状态行写 `状态：草稿，复归段补强波簇C 产出，哲学到工程桥梁条目。`。哲学桥接节用三条子项：借鉴源、哲学命题、形式化。正文不必逐字引原文，转述即可，引文只落 front matter
5. 哲学锚每件恰二条，纪律五条：引文与源行逐字节一致（连续片段禁删节）；引文不含全角括号与破折号；行号已预验（见下）；锚命题必须是本战线命题本身即 PRO-03 或 PRO-04，不得换成别的；front matter 内引文用双引号包裹

## 草稿 front matter 形态

```
---
entry: ENTRY-ALG-010.md
agent: 复归段补强波簇C
proposed_id: ALG-010
subrepo: algebra
id_reason: algebra 子仓 INDEX 已建 ALG-001 与 ALG-008 与 ALG-009，ALG-002 至 007 为预约位勿占，下一空号 ALG-010
anchors:
  - pro: PRO-03
    source: sih-philosophy/emanation/proodos/03-on-second-tao.md:89
    quote: "代码是意图的有形化，意图是代码的未形化；有形不能完全包含无形"
selfcheck:
  - 锚二条 grep -nF 复核命中
  - 三查：全文全角括号中文内容零处、破折号字符零处、围栏代码块零处
  - 八节在场且顺序固定，首行 H1 形对
  - 数学主张：（每件如实写本件的可判定主张与边界）
---
```

## 件一 ENTRY-ALG-010.md：ALG-010 因式分解与不变性（承载 PRO-03）

id_reason：algebra 子仓 INDEX 已建 ALG-001 与 ALG-008 与 ALG-009，ALG-002 至 007 为预约位勿占，下一空号 ALG-010。

锚二条（已预验命中，行号勿动）：
- pro PRO-03，source sih-philosophy/emanation/proodos/03-on-second-tao.md:89，quote "代码是意图的有形化，意图是代码的未形化；有形不能完全包含无形"
- pro PRO-03，source sih-philosophy/emanation/proodos/03-on-second-tao.md:95，quote "道二只说「意图先于代码」"

数学核心：
- 设定：集 I 为意图集，集 C 为代码集，f: I → C 为实现映射，f(i) 为实现意图 i 的代码。纤维 f 逆像 c 定义为意图 i 满足 f(i) = c 的 i 之集，一般集合版，ALG-008 给线性映射版
- 因式分解判据定理（定理一，本件核心，初等）：设 g: C → S 为任意映射。下列等价：一，存在映射 h: I → S 使 g 等于 h 复合 f，即 g 经 f 因式分解；二，g 在 f 的每一纤维上取常值，即 f(i) = f(j) 蕴含 g(f(i)) = g(f(j))。证明方向二推一：纤维上取常值则可定义 h(f(i)) = g(f(i))，良定义性由常值性担保。方向一推二：直接代入。唯一性命题：若 f 满射，则满足条件的 h 唯一；f 不满射时 h 在像外的取值任意，唯一性在像集上恢复
- 不变性定义（定义段）：g 经 f 因式分解时称 g 为 f-不变量，即 g 是意图的性质而非代码的纤维内差异。代码局域性质即不经 f 因式分解的 g，纤维内可变的读数
- 不变量-像对应命题（命题二）：f-不变量 g: C → S 与像集 f(I) 上的映射一一对应，对应为 g 映射到像集上限制 \bar{g}，反为 \bar{g} 映射到 \bar{g} 复合 f。像集即 I 按核等价类 i 与 j 等价当且仅当 f(i) = f(j) 的商。f-不变量即商上的函数
- 退化段（如实标注）：f 双射时纤维单点，一切 g 皆为不变量，因式分解退化，无区分力。f 非满射时像外代码无意图对应，治理域限像集内，像外不属治理对象
- 治理对象命题（一段，对应原文「治理的对象是意图」）：治理读数取值为 S 的函数 g 是治理对象当且仅当 g 是 f-不变量。原文「道二只说意图先于代码，不说意图必须文档化」的数学形态：文档化形式是纤维内选择，纤维内选择非不变量，不属道二层治理对象

哲学桥接节要点：
- 借鉴源：商构造与 universal property 初等形态，不变量概念传统，像-纤维-因式分解三元组
- 哲学命题：PRO-03 道二，意图先于代码，因果方向不可逆
- 形式化：原文「代码是意图的有形化，意图是代码的未形化」即 f 的像-纤维结构，代码为像，意图为原像集；不可逆由 ALG-008 承，纤维多重即左逆不存在；本条承治理对象侧，治理对象即 f-不变量即商上的函数；「有形不能完全包含无形」即 f 一般非满单，信息在纤维内折叠；「道二只说因果方向」即道二层只约束不变量侧，纤维内形式归法层。与 ALG-008 的分界：ALG-008 承不可逆性，本条承治理对象结构，两者共用纤维基础

facet 应用节：facet 判据读数不变性检查，同一意图的两个实现，两 shot 代码，在治理对象维度应给同一读数；风格与命名差异应不改读数，工程检查为取同一需求标识的两实现比对判据读数，差异即读数含非不变量分量，须定位

历史脉络节：商与 universal property 为 Bourbaki 代数教科书标准内容；不变量理论传统；像-纤维-因式分解为映射论标准三元组。年份按标准史实

工程注意事项节：不变性测试需同意图代码对，意图标注来源为需求文档标识，无标识时不变性不可测，如实标注；纤维采样工程为取同一需求标识的两实现比对；意图域 I 不可枚举时不变性为理想化条件，工程侧降级为抽样检验；f 非满射场景即无意图随机代码，治理域外，检测前须先判意图归属

参考文献节：Bourbaki 代数卷商构造章；标准范畴论入门 universal property 节；Wikipedia 商结构与 universal property 条目

## 件二 ENTRY-PROB-009.md：PROB-009 Fano 不等式与恢复误差下界（承载 PRO-04）

id_reason：probability 子仓 INDEX 现行已建至 PROB-007，PROB-008 由本波簇 B 预留，本件取下一空号 PROB-009。

锚二条（已预验命中，行号勿动）：
- pro PRO-04，source sih-philosophy/emanation/proodos/04-on-third-tao.md:17，quote "维护之前必须先恢复意图，恢复是维护的因果前提"
- pro PRO-04，source sih-philosophy/emanation/proodos/04-on-third-tao.md:111，quote "此锚定为启发性类比而非严格数学推导"

数学核心：
- 设定：有限意图集 I 基数 m 大于等于 2，码字集 C，实现映射 f: I → C 确定性给定。先验 P_I 为 I 上的正概率分布，P_I(i) 全部大于零。恢复器即解码器 s: C → I。误差事件为 s(f(i)) 不等于 i，误差概率 P_e 为 P_I 下该事件概率，显式写成对 i 的求和
- 后验与条件熵（定义段）：后验 P(i|c) 由 Bayes 公式，似然为 0-1 值因 f 确定性，显式写出。熵 H(X) 内联定义，负和 p log p，底取 e 并标注可换底。条件熵 H(I|C) 内联定义
- 自晦定理（定理一）：f 非单射，即存在两意图映同码字，蕴含 H(I|C) 大于 0。证明：非单射纤维含两意图 i 与 j，该码字下后验两值皆正，条件熵含该码字贡献为正。反向：f 单射蕴含 H(I|C) = 0，恢复可无错，s 在像上取 f 的单射逆
- Fano 不等式（定理二，本件核心）：对任意恢复器 s，P_e 大于等于 H(I|C) 减 1 整除 log m。证明走标准链：记 \hat{I} = s(C)，误差指示 E 为 1 当 \hat{I} 不等于 I 否则 0。则 H(I|C) = H(I|C,E) + I(I;E|C)，其中 I(I;E|C) = H(E|C) 因 E 为 I 与 C 的函数。H(I|C,E) = P_e 乘 H(I|C,E=1)，给定 C 与 E=1 时 I 至多 m 减 1 种取值，故 H(I|C,E=1) ≤ log(m−1)。H(E|C) ≤ H(E) 为二元熵 h(P_e)，且 h(P_e) ≤ 1，底取 e 时二元熵最大值 log 2 小于 1。综合 H(I|C) ≤ P_e log(m−1) + h(P_e) ≤ P_e log m + P_e − 加一减一整理得 P_e ≥ (H(I|C) − 1)/log m，整理步骤逐项写明。标准形式 P_e log(m−1) + h(P_e) ≤ H(I|C) 同节登记，注明推论形式由 h(P_e) ≤ 1 与 log(m−1) ≤ log m 两弱化推出
- 命名精度命题（命题三，对应原文「名字模糊，复的成本极高；名字精确，复的成本低」）：先验集中即命名精确，先验质量集中到 f 限制单意图集上，该集合上 f 单射时 H(I|C) = 0，Fano 下界归零。一般地先验越集中条件熵越小，下界越低。命名精度即先验选择，非工程可调参数，是登记层决定量
- 恢复必行性命题（一段，对应原文「意图必复」）：Fano 给的是精度下界非不可能性断言。f 非单射时任何恢复器 P_e 大于 0，恢复必行但精度有下界；下界由 H(I|C) 与 m 定，不由编码者技艺定，技艺改进可降先验不确定但非单射保证下界严格正
- Shannon 诚实边界（一段，对应锚二）：原文声明 Shannon 锚定为启发性类比非严格数学推导。本件 Fano 下界为有限离散情形的严格定理，不依赖 Shannon 信道模型，是「必复」精度下界的严格化。原文引用 Shannon 信源编码定理，无损编码需无限长度，与 Fano 下界的关系为：无限长度对应码字集扩张使 f 趋向单射，条件熵趋向零，下界趋向零，本条只登记此对应不展开

哲学桥接节要点：
- 借鉴源：Fano 1961 不等式，贝叶斯决策论最小错误估计，条件熵
- 哲学命题：PRO-04 道三，代码自晦，意图必复
- 形式化：原文「维护之前必须先恢复意图，恢复是维护的因果前提」即恢复器 s 先于维护动作，数学形态即估计问题先行；自晦即 f 非单射，定理一给 H(I|C) 大于零；必复即 Fano 下界，恢复存在且精度有下界；原文诚实声明 Shannon 锚定为启发性类比，本件给有限离散严格版；「技艺高者损失少，但仍有损失」的数学形态即先验可降下界但非单射保下界严格正。与 ALG-009 的分界：ALG-009 承恢复的代数结构，截面存在性与非唯一性，本条承恢复的精度下界，概率侧，两者互补

facet 应用节：facet 多 shot 判据，同一意图 m 个实现候选，k 次观察读数，判据恢复即估计问题，Fano 下界给判据恢复的不可约误差；术语登记精度即先验集中度，术语模糊则下界高，判据恢复不可靠，检词位精度有数学意义

历史脉络节：Shannon 1948 信息论，原文声明此处为启发性锚定；Fano 1961 误差概率计算；Cover 与 Thomas 教材标准形式。年份按标准史实，Fano 年份若不确定则写 Fano 不等式以 Cover 与 Thomas 教材形态引用不署原始年份

工程注意事项节：Fano 下界计算需先验 P_I，先验来自术语登记与需求文档，即名字即先验；m 可枚举因意图入名册登记；随机实现情形，同一需求生成不同实现，是二阶近似，如实标注为扩展不写入主定理；下界是下界非可达值，工程侧用于判据排序与验收门槛，不用于误差预测；底数约定写清，换底时常数项变

参考文献节：Fano 1961 或 Cover 与 Thomas Elements of Information Theory 的 Fano 不等式节；Shannon 1948 A Mathematical Theory of Communication；PROB-005 与 PROB-006 仓内条目；Wikipedia Fano 不等式条目

## 交稿要求

- 交稿前自检：对每件草稿跑全文检查，全角括号内若含中文即改；破折号零；围栏代码块零；八节在场；front matter YAML 可解析。自检结论写入 front matter selfcheck 字段，如实写
- 两件完成后逐件追加一行 CLAIMS-C.ndjson，形如 {"entry":"ENTRY-ALG-010.md","agent":"复归段补强波簇C","pro":"PRO-03","anchors":2,"selfcheck":"passed"}
- 数学拿不准的陈述不写，宁少勿错。反例与适用边界如实标注
- 只读参考材料：范本 sih-math/probability/entries/PROB-007-expectation.md；现有承载件 sih-math/algebra/entries/ALG-008 与 ALG-009 与 sih-math/probability/entries/PROB-005 与 PROB-006（对照防重复，引用其公式号须核对实体存在）；原文 sih-philosophy/emanation/proodos/03-on-second-tao.md 与 04-on-third-tao.md；llm 条目 sih-philosophy/llm-friendly-build/entries/PRO-03.md 与 PRO-04.md
