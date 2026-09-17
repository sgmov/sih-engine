# 数学管线全量载体覆盖审计汇总（rev3）

> orig: mathscan-solo 全盘扫描件，原账本三件与 rev1 三件与 rev2 三件只读不动，本文件为 rev3 汇总

## 载体三态（rev3）{#carriers}

| 状态 | 数量 | 机制 |
|---|---|---|
| 已实例化 | 19 | cascade、elicit、facet、formatter、gauge、identity、latex-helper、lease、locator、locks、meter、nomenclator、parser、scrutinator(sih-tools)、selector、tally、scribe(sih-engine)、scrutinator(sih-engine)、wikirecall |
| 可指认未实例化 | 5 | scribe(sih-tools)、ask3repeater、attractor、retriever、viewer |
| 无可指认载体 | 0 |  |

rev2 读数 10 已实例化 / 5 可指认未实例化 / 8 无可指认载体（23 件）；rev3 机制名册 24 件（增补 wikirecall 第 24 件，pk037impl 语义层落位，任务包所称 retriever 语义层以磁盘实态勘误为 sih-tools/wikirecall，引擎 retriever 实扫零数学 ID 词面），同名跨域机制以括注域区分。

## 接线反映（逐件）{#wiring}

| 机制 | 域 | 源码推导载体 | 推导档 | 三态 |
|---|---|---|---|---|
| tally | sih-tools | ORD-006、ORD-008、ORD-011 | tallywire-tally-derivation-2026-09-04.md | 已实例化 |
| selector | sih-tools | ALG-002 | selwire-selector-derivation-2026-09-04.md | 已实例化 |
| lease | sih-tools | ORD-020 | ordwire-lease-derivation-2026-09-03.md、leasewire-derivation-2026-09-04.md | 已实例化 |
| scribe | sih-engine | ORD-019 | scriwire-scribe-derivation-2026-09-03.md | 已实例化 |
| identity | sih-tools | ALG-002、PROB-013 | idwire-identity-derivation-2026-09-03.md | 已实例化 |
| cascade | sih-tools | ORD-016 | caswire-cascade-derivation-2026-09-03.md | 已实例化 |
| scrutinator | sih-tools | ORD-008 | gatecap-derivation-2026-09-04.md | 已实例化 |
| scrutinator | sih-engine | ORD-008 | gatecap-derivation-2026-09-04.md | 已实例化 |
| facet | sih-tools | PROB-010 | mathpipe-a2-derivation-2026-09-03.md、facetmath-derivation-2026-09-04.md | 已实例化 |
| gauge | sih-tools | LIM-007、ORD-002、PROB-001、PROB-003、PROB-005、PROB-010、PROB-013、PROB-015、PROB-016 | mathpipe-a3-derivation-2026-09-03.md、queueing-derivation-2026-09-04.md、gchart-derivation-2026-09-04.md、contribmath-derivation-2026-09-04.md | 已实例化 |
| formatter | sih-tools | ORD-023 | carrwire-carriers-derivation-2026-09-04.md | 已实例化 |
| meter | sih-tools | PROB-016 | carrwire-carriers-derivation-2026-09-04.md | 已实例化 |
| nomenclator | sih-tools | ORD-024 | carrwire-carriers-derivation-2026-09-04.md | 已实例化 |
| parser | sih-tools | ALG-012 | carrwire-carriers-derivation-2026-09-04.md | 已实例化 |
| wikirecall | sih-tools | ALG-011、PROB-011、PROB-014、PROB-017 | pk037impl-derivation-2026-09-04.md、recallloop-derivation-2026-09-04.md | 已实例化 |
| locks | sih-tools | ORD-020 | leasewire-derivation-2026-09-04.md | 已实例化 |
| elicit | sih-tools | ORD-008 | elicitwire-elicit-derivation-2026-09-04.md | 已实例化 |
| locator | sih-tools | ORD-019 | locatorwire-locator-derivation-2026-09-04.md | 已实例化 |
| latex-helper | sih-tools | ALG-001、INT-007、LIM-007、SER-001 | latexwire-derivation-2026-09-04.md | 已实例化 |

已实例化双门：源码引用载体且推导档在案（rev3 推导档位为列表，逐份磁盘核验）；推导档磁盘核验逐件在案零缺。

facetmath 挂起申报：facetmath-derivation-2026-09-04.md 为门控挂起形：声称消费侧 probes/maturation_gate.py boundary_criterion 门控位实测零 PROB-010 词面命中，PROB-010 源码面仍由 facet_stats_inf.py 既有面承载；本批不扩 facet 载体面，挂起态如实申报不代判。

## rev2 后接线批反映（十批逐件）{#batches}

| 批 | 反映 |
|---|---|
| carrwire-solo | formatter ORD-023 与 meter PROB-016 与 nomenclator ORD-024 与 parser ALG-012 四件源码推导入面，四件无可指认载体转已实例化 |
| pk037impl-solo | wikirecall 新机制位（第 24 件）PROB-017 与 ALG-011 源码推导，判据 PROB-014 与 PROB-011 复用 recallloop 档 |
| leasewire-solo | locks 首锚 ORD-020 源码推导入面，无可指认载体转已实例化 |
| elicitwire-solo | elicit ORD-008 源码推导入面，转已实例化 |
| locatorwire-solo | locator ORD-019 源码推导入面，转已实例化 |
| latexwire-solo | latex-helper LIM-007 与 INT-007 与 SER-001 与 ALG-001 计算载体族源码推导入面，转已实例化 |
| queueing-solo 与 gchart-solo 与 contribmath-solo | gauge 载体面 PROB-015 与 PROB-010 与 PROB-013 与 PROB-016 扩充，推导档三份增补 |
| facetmath-solo | facet 判据档增补（门控挂起形，载体面零扩如实申报） |
| basefix-solo | locks 与链与收约修复，载体注记级（ORD-020 与 ORD-019 回归），leasewire 档与 basefix 档在案 |

## 分类分布（rev3）{#classification}

| 面 | 判定性 | 资源性 | 待确认 | 合计 |
|---|---|---|---|---|
| 命名常数 | 34 | 20 | 85 | 139 |
| 比较位字面量 | 83 | 0 | 555 | 638 |

归类对表（对 rev1 生效面）：命名常数分类变动 3 件、rev3 新增 33 件、rev1 有而 rev3 无 3 件；比较位字面量变动 0 件、新增 93 件、消失 8 件。逐件细目落 ledger-rev3.json classification.diff_vs_rev1 节，零静默。rev2 计数对表落 classification.rev2_counts_comparison 节。

资源性改判承继：rev1 十二件判定性改判资源性以（相对路径，常数名）键承继，本批实命中 15 件，逐件细目落 ledger-rev3.json reclassification_inherited 节。

## 双实存核验{#f3}

| 机制 | 概念 ID | 概念名 | 子仓 | SIH | 磁盘 | 结果 | rev3 新判 |
|---|---|---|---|---|---|---|---|
| cascade | ORD-016 | 良基关系与倒推终止 | order | ✓ | ✓ | 实存 | 否（承继） |
| elicit | ORD-008 | 引用图可达与孤悬判定 | order | ✓ | ✓ | 实存 | 是 |
| facet | PROB-010 | 假设检验与显著性 | probability | ✓ | ✓ | 实存 | 否（承继） |
| formatter | ORD-023 | 抽象重写系统与确定性范式 | order | ✓ | ✓ | 实存 | 是 |
| gauge | LIM-007 | epsilon-delta 定义 | calculus | ✓ | ✓ | 实存 | 否（承继） |
| gauge | ORD-002 | 完全格 | order | ✓ | ✓ | 实存 | 否（承继） |
| gauge | PROB-001 | 大数定律（WLLN） | probability | ✓ | ✓ | 实存 | 否（承继） |
| gauge | PROB-003 | 中心极限定理（CLT） | probability | ✓ | ✓ | 实存 | 否（承继） |
| gauge | PROB-005 | Bayesian 更新 | probability | ✓ | ✓ | 实存 | 否（承继） |
| gauge | PROB-010 | 假设检验与显著性 | probability | ✓ | ✓ | 实存 | 否（承继） |
| gauge | PROB-013 | 平稳性与变点检测 | probability | ✓ | ✓ | 实存 | 否（承继） |
| gauge | PROB-015 | Little 定律与负载界 | probability | ✓ | ✓ | 实存 | 否（承继） |
| gauge | PROB-016 | 计数测度与可加性 | probability | ✓ | ✓ | 实存 | 否（承继） |
| identity | ALG-002 | 等价关系与商集隔离 | algebra | ✓ | ✓ | 实存 | 否（承继） |
| identity | PROB-013 | 平稳性与变点检测 | probability | ✓ | ✓ | 实存 | 否（承继） |
| latex-helper | ALG-001 | 矩阵与特征值 | algebra | ✓ | ✓ | 实存 | 是 |
| latex-helper | INT-007 | 微积分基本定理 | calculus | ✓ | ✓ | 实存 | 是 |
| latex-helper | LIM-007 | epsilon-delta 定义 | calculus | ✓ | ✓ | 实存 | 是 |
| latex-helper | SER-001 | 无穷级数 | calculus | ✓ | ✓ | 实存 | 是 |
| lease | ORD-020 | 全序资源分配与死锁自由 | order | ✓ | ✓ | 实存 | 否（承继） |
| locator | ORD-019 | 版本偏序与外化状态存储 | order | ✓ | ✓ | 实存 | 是 |
| locks | ORD-020 | 全序资源分配与死锁自由 | order | ✓ | ✓ | 实存 | 是 |
| meter | PROB-016 | 计数测度与可加性 | probability | ✓ | ✓ | 实存 | 是 |
| nomenclator | ORD-024 | 区间序与串跨度匹配 | order | ✓ | ✓ | 实存 | 是 |
| parser | ALG-012 | 形式文法与解析确定性 | algebra | ✓ | ✓ | 实存 | 是 |
| scrutinator | ORD-008 | 引用图可达与孤悬判定 | order | ✓ | ✓ | 实存 | 否（承继） |
| selector | ALG-002 | 等价关系与商集隔离 | algebra | ✓ | ✓ | 实存 | 否（承继） |
| tally | ORD-006 | 闭包算子与后果算子 | order | ✓ | ✓ | 实存 | 否（承继） |
| tally | ORD-008 | 引用图可达与孤悬判定 | order | ✓ | ✓ | 实存 | 否（承继） |
| tally | ORD-011 | 良基归纳与递归终止 | order | ✓ | ✓ | 实存 | 否（承继） |
| scribe | ORD-019 | 版本偏序与外化状态存储 | order | ✓ | ✓ | 实存 | 否（承继） |
| scrutinator | ORD-008 | 引用图可达与孤悬判定 | order | ✓ | ✓ | 实存 | 否（承继） |
| wikirecall | ALG-011 | 向量空间与余弦相似度 | algebra | ✓ | ✓ | 实存 | 是 |
| wikirecall | PROB-011 | 估计与置信区间 | probability | ✓ | ✓ | 实存 | 是 |
| wikirecall | PROB-014 | 期望信息增益 | probability | ✓ | ✓ | 实存 | 是 |
| wikirecall | PROB-017 | 逆文档频率与自信息加权 | probability | ✓ | ✓ | 实存 | 是 |

承继位缺口零：全部源码推导载体 INDEX 与磁盘双实存（on_disk 判定按子仓条目目录实态枚举，calculus 条目实居 llm-friendly-build/entries/ 特例由实态枚举承载，rev2 伪缺口根除）。

## 扫描对表零漏项{#scan}

语料内非 SPEC 白名单 ID 命中全部被载体映射覆盖，漏项核对 findings 2 笔；SPEC 双义 ID 不在漏项核对域，消歧规则与规格引用面归属零变动承 rev1。非白名单占位号（wikirecall 自测件 APP-999 与 ORD-905 与 TOP-901 与 TOP-902 等）不在漏项核对域，如实记。

## 口径三分列（缺陷②防误报位）{#caliber}

- 白名单正身号全集 177（calculus 114 + order 21 + probability 19 + topology 8 + algebra 15）
- 磁盘条目文件 177 件（实态目录枚举：calculus 114 件居 calculus/llm-friendly-build/entries；order 21 件居 order/entries；probability 19 件居 probability/entries；topology 8 件居 topology/entries；algebra 15 件居 algebra/entries）
- 别名记账 4 笔（CALC 系迁移注记，单列不计数）：CALC-001→HIS-015；CALC-003→HIS-016；CALC-008→LIM-007；CALC-031→DIFF-032
- INDEX 在册无磁盘 0 笔（待建概念正常缺省态，已建缺盘为漂移红，逐笔见 ledger-rev3.json ledger_caliber 节）；磁盘孤儿 0 笔

口径声明：白名单数与条目文件数与别名数三分列，任两数不等不构成误报，差异由本节三类清单解释（rev2 时 165/171 口径分歧的根因是三口径混用，本节根除）。

## spec 消歧 {#spec}

- SPEC 双义交集（数学仓 ∩ 引擎规格档）：SPEC-001, SPEC-002, SPEC-003, SPEC-004, SPEC-005, SPEC-006, SPEC-007
- 消歧规则：引擎 src 引用位默认引擎侧，除非白名单命中且引用上下文明示数学条目

## 命题层

立题立场：本汇总的立题是「账本与源码实况的重新对表」——通过 rev2 后十批接线逐件反映与两缺陷修复，使账本读数重新映照源码实况，触及载体归因的时效准确性，不把刷新本身当立题。

应用命题映射：PRO-07 鉴要求检验由可重复程序承载，rev3_script.py 同参双跑逐字节一致即其应用；A-A3.1 鉴层破自证循环——漏项核对线以独立语料扫描对表载体映射，不以映射自证映射；A-A4.2 裁决权归确定性引擎——三态判与改判承继由脚本按规则机械执行；PRO-08 应而不藏——rev2 三件只读零改写，rev1 至 rev3 读数并列可回溯，facetmath 挂起态与非白名单占位号如实申报不藏。

治理贡献：账面漂移整体刷新与两缺陷根除后，人类注意力只需投向漏项核对 findings 与双实存核验缺失与归类变动申报三类异常信号，其余读数跟可复算脚本走，治理贡献即信息洪流降维与权责归一。

