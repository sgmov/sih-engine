# mathpipe-a2-solo 结果档

> 批：facet 判定数学化批（mathpipe-full 程序批二）
> 会话：b42021a7f4400f23（已吊销，异常申报见披露节）
> 日期：2026-09-03
> 队形：单线形 solo，零子代理
> 意图哈希：351eabd5cb47f5d5a3812f8eba1df235521a04f3f81bdc4a489f71653bc6fda3（会话台账 intent.record_sha256 对表一致，ask3 验证件 status ok 三锚）

## F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 载体推导在案 | 载体锚点磁盘实存且推导档在册 | 过 | PROB-010 假设检验与显著性实存 sih-math/probability/entries/PROB-010-hypothesis-testing-and-significance.md，mapping.md:200 命中"检验判定的偶然界线"；推导档 sih-math/docs/mathpipe-a2-derivation-2026-09-03.md 在册随批入版控（996f7f0） |
| F-2 金向量一致 | 切换后 gate 输出复算逐字节一致 | 过 | 金向量 mathpipe-a2-golden-vector.json（25218 字节）38 trail 全量复算逐字节一致，认证 33fd3872 承 |
| F-3 判变零静默 | 判变清单逐件申报人确认，无未申报判变 | 过 | 判变七件逐件列档人节点确认，missed_knife_edge 0 且 knife_leak_ids 空 |
| F-4 行为可复算 | 判定路径全量重跑可复算，α 显式给参不读钟 | 过 | α 由代码常数 ALPHA=0.05 显式给参，boundary_low_by_test 与 adjudicate_v2_test 纯函数式无钟依赖；probe 三件同参同结果 |
| F-5 写入仅 allow | 写入仅请求写入节所列 | 过（附披露） | 写入全落 allow 十七路；settle 曾误 stage 校准账本与 probe 结果已撤回按 allow 重提（披露 2） |

## 载体定位结论

- 载体 ID：**PROB-010 假设检验与显著性**（sih-math/llm-friendly-build/mapping.md:200，检验判定的偶然界线）。
- 推导档：sih-math/docs/mathpipe-a2-derivation-2026-09-03.md，承载检验统计量 **T=k**、零假设 **H0: boundary_rate ≤ 0.004**、上尾 p 值、**α=0.05** 语义与二项检验判据。
- 消费面验收线：mapping.md 全读通过，零命中不成立未触发 M-3。

## α 裁定留档

- 取值：**α = 0.05**（承源码 ALPHA 与通行水平，取值由人裁正确性由数学检验）。
- 用户原话："0.05（推荐）"。
- 留档位：materials/alpha-ruling.md 与 sih-engine/sih/event/inputlog/2026-09-03.ndjson seq 6。

## 判据切换与金向量读数

- 切换面：r3a_gate_v2.py 判定路径从 BOUNDARY_RATE_THRESHOLD 占比阈值 0.34 切换为二项检验判据（新增 adjudicate_v2_test），接线 facet_stats_inf.py 函数族（boundary_test_pvalue / binomial_test_pvalue / boundary_low_by_test）。
- 模型参数：ALPHA=0.05，DEFAULT_BOUNDARY_BASELINE=0.004，均显式给参不读钟。
- 金向量读数（38 trail）：切换前 v1 计数 boundary 28 / near_threshold 1 / stable_clear 9；v2 计数 boundary 17 / near_threshold 12 / stable_clear 9；**v2_test 计数 boundary 24 / near_threshold 7 / stable_clear 7**。
- 派生量：missed_knife_edge 0，knife_leak_ids 空，false_injury_reduced true，monotone_ok true，knife_demoted_to_near [lw2-p_monitor__L1]。
- 复算：金向量字节级冻结，双跑逐字节一致零漂移。

## 判变清单（7 件，人节点已确认）

| # | guidance_id | from | to |
|---|---|---|---|
| 1 | lw-c2vote_vote | near_threshold | boundary |
| 2 | lw-c3_program | near_threshold | boundary |
| 3 | lw-r3b_v2_selfsign | stable_clear | boundary |
| 4 | lw3-c2vote_vote__fb | stable_clear | boundary |
| 5 | lw3-dualviol__fb | near_threshold | boundary |
| 6 | lw4-d0 | near_threshold | boundary |
| 7 | lw5-g1-fmtnly | near_threshold | boundary |

漏放核查：missed_knife_edge 0 即无漏放；零未申报判变，判变方向全部为由宽到紧（near_threshold/stable_clear → boundary），符合由松到紧治理方向。

## 管线读数

- 化格：推导档 md 目标 exit 0 无需改；两 py 变更件 general-v1 域外（只盖 md/json/yaml/toml）exit 2 如实记不属违规。
- 核阅：des-001 三目标均域外（math docs 与 py 非 sih-engine/doc 域）exit 2 如实记不属违规；ask3 双门第一门 exit 0 零违规。
- 检词：三目标 exit 0 零违例（工地 core 包含三新词）。
- 词债：检验判据、推导档、判变清单三词 established 登记入工地 core 包随批入版控（manifest 0.9.0）。

## 认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| ed76aba2 | 意图笔（scribe intent） | ask3 记录 351eabd5 |
| 77ab6625 | 管线报告（2026-09-03-mathpipe-a2-pipeline.json） | — |
| 33fd3872 | 金向量（materials/mathpipe-a2-golden-vector.json） | — |
| 0c3fbde1 | 推导档报告（2026-09-03-mathpipe-a2-derivation.json） | 3a660d40… |
| 8bf29b8a | 变更件报告（2026-09-03-mathpipe-a2-changed-files.json） | f59c7054… / 235f07c9… |

## 三仓 commit 号

- sih-engine：段1 dce3fec → 归并 d427c34（main HEAD）
- sih-tools：段1 341911af + 段2 1a41cf82 → 归并 d2fc3673（integral-stage-build HEAD）
- sih-math：段1 996f7f0 → 归并 9751353（main HEAD）

## 链 verify 对表

trail 2026-09-03.ndjson 51 事件，status valid，first 94f1dd00，last 8bf29b8a，链尾即变更件认证笔。

## reconcile 读数

| 仓 | unrouted | cert_missing | unbypassed | routed/total |
|---|---|---|---|---|
| sih-engine | 0 | 0 | 0 | 84/85（bypass 1 为界前存量） |
| sih-tools | 0 | 0 | 0 | 55/55 |
| sih-math | 0 | 2（2026-08-30 存量） | 1（baseline init 存量） | 60/63 |

三仓 unrouted 均零，cert_missing 与 unbypassed 全为批前存量（mathfix2-solo、fmtfix-solo、数学仓基线 init），本批零新增。

## 披露与越线申报

1. **会话吊销异常**：收约 close 时引擎工地残留未提交 inputlog（共享同日态主树已存），close_failed 后强拆让位即本会话 b42021a7f4400f23 于 2026-09-02T22:34:10Z 吊销。工具与数学两仓收约已归并，引擎归并 d427c34 已落主树；被让位者仅引擎工地 inputlog 残件且主树同日态已存，零数据损失。本结果档与 materials/recall-results.json 因会话吊销无法走 lease commit（commit 四验须指围堰副本），作工作树未提交件留档，待主会话决定是否另立会话补提交。
2. **settle 范围误 stage**：工具仓 settle 曾误 stage 校准账本与 probe 结果等 out-of-scope 件被 lease commit 拒（staged_out_of_scope），已撤回按 allow 重提，CALL-LOG 在册。
3. **判变阈值语义**：判变七件全部由宽判据升为检验判据后更严判定（near_threshold/stable_clear → boundary），为治理收紧方向符合由松到紧，非误伤。

## 承接

批三即 ledgrev-solo 已开（会话 c8744a784725b2b1），probes 分析件阈值不属本批，判变方向与金向量读数可作为批三判据归档参照。
