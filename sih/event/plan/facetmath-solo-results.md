# facetmath-solo 结果档

> 批：facetmath-solo（mathpipe 程序批二落地件：facet boundary 判据切换收口）
> 日期：2026-09-04。租约会话：a34864c69935b9da（ask3 侧 sess-zcode-260904-facetmath，双标识空间各认各）。
> 队形：单线形 solo，零子代理。
> 承接：mathpipe-full-program-v1 批二、mathpipe-a2-solo（函数族注册与探针先例）、pk-045。

## 一句话结论

得一裁九发三态判 boundary，切换挂起进泊界（pk-049），生产路径保持旧判据；门控关闭形落地（boundary_criterion 缺省 ratio 逐字节不变、test 位待役全验证），判变审计 868 件零静默、114 件判变全部由宽到紧入泊界登记（pk-048），在档历史 verdict 一字未动。

## 一、F 表（完成度表）

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 生产切换 | 判据位接线，α 显式给参缺省 0.05，占比阈值位退役为基线参数，diff 亲读 | 改（门控关闭形） | F-3 得一裁判 boundary，机械分流切换挂起；落地形为 maturation_gate boundary_criterion 参数：缺省 ratio 生产行为逐字节不变，test 位已实现待役（boundary_low_by_test，p_h0=0.004，α=0.05 显式给参不读钟）；diff 57 行亲读在第三节 |
| F-2 判变审计零静默 | 在档可复算件双判据对跑，判变逐件泊界登记，不改写在档 verdict | 过 | 868 件对跑零错误，archived_mismatch=0，判变 114 件全部由宽到紧，逐件清单金向量在档，pk-048 泊界登记，在档 verdict 一字未动 |
| F-3 得一裁三态 | facet 测量九发三态机械分流 | 过（boundary） | gid facetmath-switch-1 九发：decision 全 comply、boundary_rate 0.0，依据族三值分散致 basis_consensus 挂，闸判 boundary；stable_clear 未出现故切换不在役 |
| F-4 金向量 | 切换后判据位 fixture 双跑逐字节一致 | 过 | 审计载荷零时间戳确定性装配，双跑 cmp IDENTICAL，sha256 ca9dda80cfe8bbe8…（全值见认证节）；判据位 fixture 七用例含 repr 双跑一致断言 |
| F-5 测试 | 全绿零回归 + 切换测试先红后绿 | 过 | 红：实现前 KeyError: boundary_criterion；绿：7 用例全过；工地全族 479 过 + 6 败（AGENTS.md 未入 git 工地缺件既有态，主树对照同族 18/18 全过）；主树全族 478 全过含 7 新用例，旧判据测试零改型 |
| F-6 推导档 | sih-math/docs 在档，概念以在仓命中为准 | 过 | docs/facetmath-derivation-2026-09-04.md，载体 PROB-010（mapping.md:200 命中），checkcite pass，翻桌率基线零专条如实申报以 p_h0 承载 |
| F-7 写入仅 allow | 写入仅请求写入节所列 | 过（附披露 3） | 写入全落 allow 十九路；测量 trail 重定向入本批命题目录（adjudisp 越线先例预防形）如实申报 |

## 二、得一三态节（F-3）

命题：「facet boundary 判据由占比阈值切换为二项检验判据（α=0.05）」。测量：gid facetmath-switch-1，合同模式九发（出题 emit-contract 零 LLM、席位 ZCode:GLM-5.3-Flash:self-reported 逐发实写作答、计分判据 v3 闸），飞轮 trail 落本批命题目录 facet/contracts/facetmath-260904/（trail_path 在 score 材料在册）。

- 闸读数：verdict **boundary**，v3_rule 无，九发 decision 全 comply、boundary_flag 全 false、boundary_rate 0.0、voids 0；子判据唯 basis_consensus 挂——依据族三值分散（baseline_4×5、baseline_1×3、baseline_5×1），distinct_basis=3 > 1 且 ≠2 故 B1 降级不适用。
- 先例同形：gvec-method-guard-1 九发全合而依据族分散判 boundary 不签（pk-046 承载）。加采不可救（distinct_basis 只增不减），不重跑同命题（挑选作答禁令）。
- 机械分流：boundary → 切换挂起进泊界（park 事件 752aa64a），生产路径保持旧判据，F-1 以门控关闭形落地；stable_clear 分支未触发，切换不在役收口不成立。
- 计分材料：contract sha256 6fd810f4a7d8a2bb…、responses sha256 58de0f6b46fe826e…、identity hash 665833701bcc6960…（正身件同源）。

## 三、F-1 diff 亲读（门控关闭形）

工地 diff 共 57 行（maturation_gate.py），逐段亲读：

1. import 扩展（+5/−1）：boundary_test_pvalue 单名扩为四名（ALPHA、DEFAULT_BOUNDARY_BASELINE、boundary_low_by_test、boundary_test_pvalue），全部 A2 已注册名。
2. v3_base 与 v3 签名各 +1：boundary_criterion: str = "ratio"（缺省即旧判据）。
3. docstring +10/+4：门控语义、F-3 裁决与 pk-049 指称、非法值 ValueError、检验载荷在册声明。
4. 判据位核心（+21/−1）：ratio 支路 v3_boundary_low = adj_max < thr 与原行逐字节等价；test 支路 boundary_low_by_test(k, n, p_h0=0.004, alpha=0.05) 加 boundary_test_pvalue 载荷；非法值抛 ValueError；boundary_k/n 双支路共算。
5. 返回载荷（早退 +4/−1、常态 +4/−1）：boundary_criterion 与 metrics.boundary_k/boundary_n/boundary_test_p 在册。
6. 快道与降级零改：V3-U 快道让位、B2 红线、near 带语义一字未动。

facet_stats_inf.py 零改动（参数注记以 maturation_gate docstring 承载：ALPHA=0.05 与基线 0.004 消费位在门控 docstring 指称，源内 A2 标记原样）。旧判据测试零改型：缺省形下行为零即旧行为，既有用例无一需动。

## 四、判变审计节（F-2）

- 对跑对象：在档可复算 facet 裁决材料 868 件（facet_task_packages 合同包 2 件 + proposition/DES 飞轮 trail 866 件），双判据（ratio/test）各复算一次，零错误零静默丢。
- 复算回归哨：archived_mismatch = 0——868 件在档 operative verdict 与在役判据复算全量一致，在档裁决零漂移，审计与在役闸互证。
- 判变（ratio≠test）：114 件，全部由宽到紧——near_threshold→boundary 74 件、stable_clear→boundary 40 件，零由紧到宽；刀锋集零漏放（lw2-e1_repair__L3 near→boundary 收紧仍不放行）。verdict 分布：ratio boundary 388/near 159/stable 321 → test boundary 502/near 85/stable 281。
- 合规形态：切换挂起期间判变件零实际改判，逐件清单金向量在档（materials/facetmath-golden-dual-audit.json，认证 0f1f34fb），泊界登记 pk-048（park 事件 1b563a66），在档历史 verdict 一字未动，切换只对新测量生效。
- 探针纪律：facetmath_dual_audit.py 零 trail 写入、零时间戳确定性输出（承 r3a 先例的 trail 写入策略）。

## 五、金向量节（F-4）

- 双跑：同参形两次实跑 cmp 逐字节 IDENTICAL；载荷零时间戳零随机（确定性装配承 SPEC-015 同参形条款精神）。
- 判据位 fixture：单席九发一旗形（k=1/n=9）ratio 判过（率 0.111<0.34）而 test 判挂（p≈0.0353≤0.05），verdict_v3 stable_clear→boundary 翻转断言在册；V3-U 快道让位语义零改断言在册；boundary_low_by_test 单元锚（k=0 过、k=1/n=5 挂、n=0 过、违例 ValueError）在册。
- 金向量 sha256：ca9dda80cfe8bbe8e125799bd68d738147a6ce527f77c6367f2b52008227fe03，认证事件 0f1f34fb。

## 六、测试节（F-5）

- 先红：test_toggle_keyword_exists 于实现前实跑 FAILED（KeyError: boundary_criterion），红态证据在录。
- 后绿：tests/test_boundary_switch.py 7 用例全过（门控参数在册、缺省 ratio 零回归、同数据双判据分叉、快道零改、金向量 fixture 双跑、boundary_low_by_test 已知值锚、违例输入）。
- 零回归：主树全族 478 全过（含 7 新用例，471 既有全绿）；工地全族 479 过 + 6 败，6 败全部为 test_ng_assembler 族 FileNotFoundError（AGENTS.md 不在 git 仓、工地无此件的既有环境态，与主树同族对照 18/18 全过，非本批回归）。

## 七、认证清单

| 事件哈希前八 | 对象 | 说明 |
|---|---|---|
| 0e712cd5 | 意图笔（intent_refined，event_id 4788e4d6） | ask3 记录 sha 4d5359e1…，双门过（scrutinator ask3 零违规 + ask3repeater ok 三锚） |
| 1b563a66 | parking_entered pk-048 | 判变清单 114 件泊界登记 |
| 752aa64a | parking_entered pk-049 | 切换挂起泊界登记 |
| 8035026d | 管线报告（2026-09-04-facetmath-solo-pipeline.json） | 管线读数与测量与审计与测试汇总 |
| 52c7d5ad | 推导档报告（…-derivation.json） | 推导档内容哈希与载体锚点与 checkcite |
| 120e73de | 变更件报告（…-changed-files.json） | 十件变更与哈希 |
| 0f1f34fb | 金向量（facetmath-golden-dual-audit.json） | 判变审计载荷冻结 |
| （链尾笔） | 本档三步管线报告（…-results-pipeline.json） | 本档化格与检词读数，哈希见当日链 |

叩问：五词三轻信号（收口批、翻桌率基线、门控关闭），digest passed 3/3，三词 established 登记入工地 core 包（0.11.0）随批入版控。泊界心跳：双线在泊材料路择零告警（引擎线 13 件 12 主线 1 scrap_track，工具线 19 件 18 主线 1 siding，alarms 空）。

## 八、越线与误差申报

1. 任务包判据位指称勘误：任务包与 dispatch 称生产判据位在「facet_stats.py 与 engine.py」——实跑核实 facet_stats.py 为纯 re-export 门面、src/engine.py 为原子调用引擎均无 boundary 判定位，生产判定路径实为 probes/maturation_gate.py 的 v3 闸判据位（measure.py 与 singleseat.py 与 layer2_signoff.py 的决策依据位），本批按实态施工，任务包背景节自注「线索，实跑为准」。
2. 简报基线值勘误：通报称 DEFAULT_BOUNDARY_BASELINE=0.34，源内实值 0.004（Kimi 标定翻桌率，注释明载 0.34=85 倍翻桌率太宽松），实跑为准。
3. 测量 trail 重定向：score 契约默认飞轮 trail 落 proposition/DES/<gid>/，该路径不在本批 allow 且命题区涉并行批 m3clear 面；按 adjudisp-solo 越线先例的预防形，出题计分复用同一 contract_mode 机械内核、仅把 trail 重定向入本批命题目录 facet/contracts/facetmath-260904/，闸与对表与计分材料逻辑零改动，trail_path 在 score 材料在册。
4. formatter 探针误传：化格阶段一次误传不存在目标路径（sih-math-x）报 domain_mismatch，工具异常零影响，同目标正式调用 exit 0 无需改，如实记。
5. 泊材料先于化格：pk-048/049 两材料经 scribe park 上链在先（停泊记录哈希绑链），未走化格（json 在 general-v1 域内但认证后任何写使认证作废），字节保持 park 时原样。
6. 并行批领地避让：facet/contracts/ 为 m3clear 同日写入面，本批仅锁与本批命题子目录 facetmath-260904/；PARKING-v1.md 名册行归 m3clear 写入面，本批泊界投影零改，名册投影缺口由主会验收按链补记（链为准）。

## 九、冲突样本节

本批与 covrefresh-solo、m3clear-solo 同日在跑。撞锁零次（共享追加面 trail 均短持即取即放，每次 append 前后锁取放配对）；链冲突零次（append 模式下并行活写零覆盖，本批七笔事件与并行批事件共存一链 verify valid）；收约碰撞待 close 后如实补记。停泊撞号零次：pk-048/049 号位经链查询与泊材料目录双重核实空闲后续号。

## 十、收口读数

- 链：开工前 26 事件 valid → 本批七笔（intent 一加 park 两加认证四）后 34 事件 valid（本档写时读数，收口后终读数见下）。
- 双门：scrutinator ask3 包零违规 exit 0；ask3repeater status ok（三锚，引文程序切片自 07-on-assay 69 行、06-on-canon 185 行、08-on-settle 114 行逐字节子串）。
- 管线：推导档化格 exit 0 无需改、核阅 des-001 域外 exit 2 如实记不属违规、检词 exit 0 零违例（工地 core 包 0.11.0）；本档三步读数见链尾认证报告。
- 词债：翻桌率基线、收口批、门控关闭三词 established 入工地 core 包，manifest 0.10.0→0.11.0。
- 书单守卫：wikirecall 词通道命中 PROB-010，checkcite pass（missing 空），翻桌率基线零专条如实申报。
- 收约读数（close 与 reconcile 与 settle 提交号）见完工报告与链事件。

## 十一、队形验证

队形验证：单线形 solo 零子代理，全部改动由本会话亲写，机械链按 BATCH-FACE（含 2026-09-04 勘误节）执行，闸三写入全带 --session 在册加 --sessions 台账路径，禁管道掩退出码全程遵守。
