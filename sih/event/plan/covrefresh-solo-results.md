# covrefresh-solo 结果档

> 批：covrefresh-solo 数学管线覆盖账本刷新 rev2（纯脚本机械批，零行为变更零裁决点）
> 会话：d10ee82894c6544f（sess-zcode-260904-covrefresh）
> 日期：2026-09-04
> 队形：单线形 solo，零子代理（确定性脚本 + 主线亲写）
> 政策行：用户 2026-09-04 裁定「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——本批零裁决点，机械链全绿自行收口，不设等你令节

## 一、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| F-1 rev2_script.py 同参双跑逐字节一致 | 完成 | 双跑 exit 0×2，三件 cmp IDENTICAL×3，双跑证据件上链（acdee4ba） |
| F-2 三态刷新与接线逐件反映 | 完成 | 三态 10/5/8（合计 23），接线十件逐件入账，漏项核对 findings 空 |
| F-3 新判已实例化双实存核验 | 完成 | 17 行 16 实存 + 1 承继缺口如实申报（gauge LIM-007 盘缺），新判 14 行全实存 |
| F-4 分类两表按 rev1 口径重扫 | 完成 | 命名 114（31/16/67）改判承继 12/12、新增 5 件逐件申报；字面量 561 零归类变动、新增 9 消失 1 逐件清单入账 |
| F-5 写入仅 allow | 完成 | 产出仅 coverage rev2 四件与 materials 与结果档与 trail 与 reports 与 CALL-LOG 与词债三件，三仓源码 diff 见收约附表 |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 脚本可复算 | rev2_script.py 同参双跑逐字节一致，落 materials | 过 | run A 落 math 工地 coverage 目录、run B 落 /tmp/covrun-b，ledger-rev2.json 与 env-params-rev2.json 与 summary-rev2.md 逐字节 IDENTICAL，sha256 十六位 2a17370dd3a3c4dd / 580881cd914615cb / fca2d24beaf8632d |
| F-2 三态刷新 | 逐机制三态重判与源码扫描对表零漏项，六件接线批与 gatecap 与 facet A2 与 gauge ga-2 逐件反映 | 过 | 语料内非 SPEC 白名单 ID 命中全数被载体映射覆盖，scan_findings 空；接线反映表十件全过双门（源码推导 + 推导档在案） |
| F-3 新判已实例化核验 | 每一新判已实例化件 SIH 引用位与磁盘 entry 双实存 | 过 | 新判 14 行全实存；承继行 gauge LIM-007 INDEX 在盘缺如实申报（见越线申报第二），不属新判域 |
| F-4 判定性/资源性分类面刷新 | 两表按 rev1 口径重扫，归类变动逐件申报零静默 | 过 | 命名常数 rev1 生效面（109 件）对 rev2 重扫（114 件）：分类变动 0、消失 0、新增 5 件逐件申报；rev1 十二件资源性改判（相对路径+常数名）键承继 12/12 实命中；比较位字面量 553→561：分类变动 0、新增 9 消失 1 均为 f-string 格式宽度伪影逐件清单入账 |
| F-5 写入仅 allow | 产出仅落任务包请求写入节 | 过 | rev2 四件落 sih-math/docs/mathpipe-coverage-2026-09-03/（rev1 三件原账本与 rev1 三件时间戳未动只读）；材料与结果档落 engine；报告与词债落 tools；rev1_script.py 口径基只读 |

## 三、rev2 三态读数（rev1 → rev2 对表）

| 状态 | rev1 | rev2 | rev2 机制名单 |
|---|---|---|---|
| 已实例化 | 1 | 10 | gauge、cascade、facet、identity、lease、scrutinator(tools)、selector、tally、scribe(engine)、scrutinator(engine) |
| 可指认未实例化 | 12 | 5 | scribe(tools)、ask3repeater、attractor、retriever、viewer |
| 无可指认载体 | 10 | 8 | elicit、formatter、latex-helper、locator、locks、meter、nomenclator、parser |

新判已实例化 9 件（gauge 承继续列），全部承接线批落地：tally（ORD-006/008/011，tallywire2 批）、selector（ALG-002 第二消费位，selwire 批）、lease（ORD-020，ordwire 批）、scribe(engine)（ORD-019，scriwire 批）、identity（ALG-002/PROB-013，idwire 批）、cascade（ORD-016，caswire 批）、scrutinator(tools) 与 scrutinator(engine)（ORD-008，gatecap 批 C007/C008）、facet（PROB-010，mathpipe-a2 批 facet_stats_inf 检验函数族 A2）；gauge 增 PROB-003/PROB-005 源码推导面（mathpipe-a3 批 ga-2 期票清偿）。

已实例化双门收紧：源码引用载体且推导档在案，十件推导档磁盘核验逐件在案零缺（mathpipe-a2/a3、tallywire、selwire、ordwire、scriwire、idwire、caswire、gatecap 八件推导档）。

## 四、新判已实例化双实存核验表（17 行）

| 机制 | 概念 ID | 概念名 | 子仓 | SIH | 磁盘 | 结果 | 新判 |
|---|---|---|---|---|---|---|---|
| cascade | ORD-016 | 良基关系与倒推终止 | order | ✓ | ✓ | 实存 | 是 |
| facet | PROB-010 | 假设检验与显著性 | probability | ✓ | ✓ | 实存 | 是 |
| gauge | LIM-007 | epsilon-delta 定义 | calculus | ✓ | — | 缺失 | 否（承继，缺口申报） |
| gauge | ORD-002 | 完全格 | order | ✓ | ✓ | 实存 | 否（承继） |
| gauge | PROB-001 | 大数定律（WLLN） | probability | ✓ | ✓ | 实存 | 否（承继） |
| gauge | PROB-003 | 中心极限定理（CLT） | probability | ✓ | ✓ | 实存 | 否（承继，ga-2 新增面） |
| gauge | PROB-005 | Bayesian 更新 | probability | ✓ | ✓ | 实存 | 否（承继，ga-2 新增面） |
| identity | ALG-002 | 等价关系与商集隔离 | algebra | ✓ | ✓ | 实存 | 是 |
| identity | PROB-013 | 平稳性与变点检测 | probability | ✓ | ✓ | 实存 | 是 |
| lease | ORD-020 | 全序资源分配与死锁自由 | order | ✓ | ✓ | 实存 | 是 |
| scrutinator(tools) | ORD-008 | 引用图可达与孤悬判定 | order | ✓ | ✓ | 实存 | 是 |
| selector | ALG-002 | 等价关系与商集隔离 | algebra | ✓ | ✓ | 实存 | 是 |
| tally | ORD-006 | 闭包算子与后果算子 | order | ✓ | ✓ | 实存 | 是 |
| tally | ORD-008 | 引用图可达与孤悬判定 | order | ✓ | ✓ | 实存 | 是 |
| tally | ORD-011 | 良基归纳与递归终止 | order | ✓ | ✓ | 实存 | 是 |
| scribe(engine) | ORD-019 | 版本偏序与外化状态存储 | order | ✓ | ✓ | 实存 | 是 |
| scrutinator(engine) | ORD-008 | 引用图可达与孤悬判定 | order | ✓ | ✓ | 实存 | 是 |

rev1 对挂七行由本表承接；selector 零命中已由 selwire 批 ALG-002 接线消解（商集第二消费位）。

## 五、分类重扫读数（F-4）

| 面 | rev1（原账本+改判） | rev2 重扫 | 变动 |
|---|---|---|---|
| 命名常数 | 109（判定性 27 / 资源性 16 / 待确认 66） | 114（判定性 31 / 资源性 16 / 待确认 67） | 新增 5 件逐件申报：ALPHA 与 DEFAULT_BOUNDARY_BASELINE（facet_stats_inf，判定性）、PRIOR_ALPHA 与 PRIOR_BETA（gauge cli，判定性）、Z_95（gauge cli，待确认）；分类变动 0、消失 0 |
| 比较位字面量 | 553（判定性 76 / 待确认 477） | 561（判定性 76 / 待确认 485） | 分类变动 0；新增 9 消失 1，逐件清点均为 f-string 格式宽度数字伪影（mathquote enumerate.py 材料件与 r3a_gate_v2 打印形），清单落 ledger-rev2.json classification.diff_vs_rev1 节 |

rev1 十二件判定性改判资源性以（相对路径，常数名）键承继，本批实命中 12/12，逐件细目落 ledger-rev2.json reclassification_inherited 节，零静默。

## 六、管线读数

| 目标 | 化格 | 核阅 | 检词 |
|---|---|---|---|
| summary-rev2.md | exit 0 无需改 | exit 2 域外如实记（des-001 域仅 sih-engine/doc） | exit 0 零违例 |
| ledger-rev2.json | exit 0 无需改 | exit 2 域外如实记 | exit 0 零违例 |
| env-params-rev2.json | exit 0 无需改 | exit 2 域外如实记 | exit 0 零违例 |
| covrefresh-solo-results.md | exit 0 无需改 | exit 2 域外如实记（event/plan 不在 des-001 域） | exit 0 零违例 |

词债登记：载体三态、期票、账面漂移三件 established 入工地 core 包（terms 131→134）随批入版控；叩问三信号 digest passed covered 3。

## 七、冲突样本节（pk-045 样本库）

1. coverage 目录 exclusive 首取撞锁：facetmath-solo（会话 c44edb7ecb8d7210）append 持位（读共享），locked_elsewhere 撞锁计数 1/10；按 append+append 共存锁型语义改取 append 键即得，本批对 coverage 目录零直写（产出落 math 工地由 close 归并），串行化点移至 close 归并。
2. trail 追加态两次取放（intent 与认证五笔）即取即得零撞锁；与 facetmath-solo 与 m3clear-solo 同日共链，批期链 32 事件 valid（并行批共笔并入，非本批独笔）。
3. scribe append 认证五笔同锁短持内完成，零撞锁；meter 包裹 2>/dev/null 对治在役。

## 八、认证清单

| 件 | event_id 前八 | event_hash 前八 |
|---|---|---|
| intent（ask3 三锚 1a8d84d1，双门过，digest covered 3） | d5bd2331 | 7a746aab |
| 2026-09-04-covrefresh-solo-pipeline.json | 19f02452 | 87a059e9 |
| 2026-09-04-covrefresh-solo-double-run.json | aa21c877 | acdee4ba |
| ledger-rev2.json | ddc9af7f | 15ee1262 |
| env-params-rev2.json | ce194ca0 | f65f868b |

全经引擎 scribe append 主树活链（meter 包裹，闸三 --session 加 --sessions 在役）。summary-rev2.md 与结果档为 md 件，scribe append 只收 json（ReportNotJson 如实记），版本锚走 git settle，rev1 先例同形。

## 九、越线与误差申报

1. **coverage 目录锁型偏差**：施工面 exclusive 意图因并行批 facetmath-solo append 读共享让位为 append 共存（撞锁计数 1），本批对 coverage 目录零直写，归并串行化由 close 承载；两批对该目录写集零交叠（本批仅新增 rev2 四件）。
2. **gauge LIM-007 双实存承继缺口**：LIM-007 在 calculus INDEX 在册而 calculus/entries/ 磁盘零文件（calculus 子仓 114 行 INDEX 对 0 磁盘条目件，条目以 llm-friendly-build 与 task-package 形承载），属数学仓条目磁盘面数据缺口；本批只记不代修（禁区：命题区与条目面归 m3clear 批面），呈后续批输入。F-3 新判域 14 行不受影响全实存。
3. **facet 分类面快照位声明**：并行批 facetmath-solo 正在 facet/probes 施工（工地内），主树 facet 面本批扫描时点稳定，双跑逐字节一致为时点一致性证据；facetmath 归并后命名常数与字面量两表须随下次刷新复核，本读数不预设其归并后形。
4. **三仓 settle 对 dispatch 双仓字样的扩越**：tools 仓随批收编报告七件与 CALL-LOG 两笔与词债三件与 meter counts（tallywire2/selwire 先例），避 ledgrev 误差申报第一的同款收编缺口；math 与 engine 两仓为批件本体，如实申报不静默。
5. **md 件不入链认证**：summary-rev2.md 与本结果档未经 scribe append（工具只收 json），以 git settle 哈希锚定，rev1 先例同形。
6. **scrutinator 域外 exit 2 三笔**：des-001 治理域仅 sih-engine/doc，math coverage 三件域外如实记入管线报告，不属违规（AGENTS.md 工具层静态审计条款原文口径）。
7. **管线读数首跑取码伪影更正**：核阅与检词首跑 echo 内命令替换吞退出码（basename 的 0 冒充），即判无效并全量重跑取真码（scrutinator 2×3、nomenclator 0×3），重跑读数为本档第六节正形；此为执行层观测失误更正申报，零数据影响。

## 十、队形验证

单线形 solo 成立：本批全部写入由会话 d10ee82894c6544f 亲写，零 Agent/Task 子代理调用，零 LLM 直改链文件（链写入全经引擎 scribe），零主树直写（产出经 math/engine/tools 三工地 settle 归并）。

## 十一、收约附表（close 后回填）

### 三仓 commit 号（回填位）
### close 记录（回填位）
### reconcile 读数（回填位）
### 链 verify（回填位）

## 十二、命题层

立题立场：本结果档的立题是「账本与源码实况的重新对表」——账面漂移整体刷新触及载体归因的时效准确性，刷新以可复算脚本承载，不把刷新本身当立题。

应用命题映射：PRO-07 鉴要求检验由可重复程序承载，rev2_script.py 同参双跑逐字节一致即其应用；A-A3.1 鉴层破自证循环——漏项核对线以独立语料扫描对表载体映射，不以映射自证映射；A-A4.1 候选建议生成器——双实存核验只产实存/缺失结果，gauge LIM-007 盘缺只记不代修；A-A4.2 裁决权归确定性引擎——三态判与改判承继由脚本按规则机械执行；PRO-08 应而不藏——rev1 三件原账本只读零改写，刷新前后读数并列可回溯。

治理贡献：rev2 把账面拉回源码实况（10 已实例化 / 5 可指认未实例化 / 8 无可指认载体），人类注意力只投向归类变动申报、双实存缺口与漏项 findings 三类异常信号，其余读数跟可复算脚本走，治理贡献即信息洪流降维与权责归一。
