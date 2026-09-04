# constclear-solo 结果档：判定性命名常数 34 件三态清账批——推导八、改判三、冻结二十三

- 批次：constclear-solo 单线 solo（第三跑，前两跑死于模型请求故障均在开工前，零残留）；会话 sess-zcode-260904-constclear（lease 会话号 99ed74655df99ca4）
- 队形：单线形亲写零子代理；零源码零行为变更，34 件常数数值零改动；M-4 红线全程在位，禁硬凑推导
- 链事件：intent 一笔加停泊一笔加认证十笔（含结果档检词一件，自指限制见越线节），历史 trail 零改写只追加
- 三态分布读数：推导 8 件、改判 3 件、冻结 23 件（23 件合并入泊位 pk-053，停泊事件 c01eb5e7）

## 意图哈希

ask3 记录 sha256 2b9d1e73fb044fc42689b464d804a50ee1ffc8e82cb54d3c585666ccedfc2cb2（intent 事件 f4cb68b852edd4cb，三锚程序切片 07-on-assay:61 映照不投射、06-on-canon:185 力度由松到紧、08-on-settle:108 应而不藏，双门 exit 0/0 status ok）。叩问五轻信号（清账/冻结/改判/载体/增量）全处置 digest passed covered 5；正身零 anomalies。

## 清账基数与盘面实态

清单源 ledger-rev3.json classification 节 named_decision 34；开工磁盘实态重取：rev3_script.py 扫描函数复现 named 126 件内判定性 34 件，与账本计数零偏差（34/16/76/126 全同）。例扫 first-run-2026-09-04：rev3 双跑退出码 0/0，四路 cmp 逐字节 IDENTICAL，checkmath 退出码 0 verdict zero_drift、reds 0、greys 5（声明滞后灰项，零红如实转述）。额外对表照录：新跑 ledger 与已提交版在 literal 面 +1（599→600）与 ledger_caliber 面有差，成因即并行批 newcarr2-solo 在途改动与批后源码演进；named 面两面全同，清账基准不受影响，账本本体零改写。

## F 表（完成度）

- F-1 逐件三态清账 34 件：过。每件恰一态，逐件表见下节；推导件载体引用逐件落到条目实文行号（PROB-010 L12/L16/L24、PROB-005 L32、PROB-013 L24、PROB-015 L61、PROB-003 L7），族惯例取值与约束实例化取值两类照实申报非定理推论值；冻结 23 件无载体要点逐件照录；改判 3 件承 rev1 十二件先例同族。
- F-2 推导档：过。sih-math/docs/constclear-derivation-2026-09-04.md 七节（口径/推导件/改判件/冻结件/env-params 增量/例扫对表/命题层），每节载常数位与现值与三态结论与载体引用或冻结路径。
- F-3 冻结件泊界登记：过。合并一件泊位 pk-053（实取下一空位，052 已用），照链三件套齐——scribe park 一笔（c01eb5e7）、materials json（pk-053.json 主树与工地逐字节 cmp 一致）、名册行（在泊计数八改九，行序插于 pk-049 后历史住户前）；名册过三门即化格零改、核阅域匹配副本 exit 0 零 findings、检词零违例。
- F-4 env-params 增量与账本对表：过。env-params-rev3.json 追加改判三行（reclassified_from/basis/batch/date 全字段），既有十六行零改动（diff 删除行仅 run_date 行尾逗号系 envelope 键语法所需，值未动）；账本本体（ledger-rev3.json 与 summary-rev3.md）零改写；再生差异照录——rev3_script.py RECLASS_INHERIT 未纳入本批三件（源码零触碰红线），下次脚本再生不含增量行，本增量以登记面 overlay 形随批申报，收编依据源落推导档 §3。
- F-5 零源码零行为变更：过。全部工具与引擎源码零触碰，34 件数值零改动，diff 面只有推导档新增与 env-params 增量与名册行与批件，无一行源码。
- F-6 写入仅 allow：过。写入面全在 allow（七 exclusive 锁面加 trail 与 meter 两 append 短持面加 reports 与 identity 两 append 长持面）；临时面例外即域匹配副本 PARKING-v1-constclear-corpuscopy.md（核阅终跑用后即删不入版控，poolclear 先例同款）与 /tmp 例扫预热面与 /tmp 锁输出。

## 逐件三态清账表（34 件）

| # | 态 | 常数=值 | 位 | 载体/依据（简） |
|---|---|---|---|---|
| 1 | 推导 | ALPHA=0.05 | sih-tools/facet/src/facet_stats_inf.py:42 | PROB-010 L12/L16/L24 水平先验；通行水平源码注记自承 |
| 2 | 推导 | ALPHA=0.05 | sih-tools/facet/probes/bootstrap_partial.py:34 | PROB-010 同上；bootstrap 置信检验 |
| 3 | 推导 | ALPHA=0.05 | sih-tools/facet/probes/d1_anova.py:52 | PROB-010 同上；ANOVA F 检验 |
| 4 | 推导 | PRIOR_ALPHA=1.0 | sih-tools/gauge/src/gauge/cli.py:66 | PROB-005 L32 Beta-Bernoulli 共轭；Beta(1,1) 均匀先验族惯例 |
| 5 | 推导 | PRIOR_BETA=1.0 | sih-tools/gauge/src/gauge/cli.py:67 | PROB-005 同上，(1,1) 合取构成均匀 |
| 6 | 推导 | GQ_WINDOW_DAYS_DEFAULT=7 | sih-tools/gauge/src/gauge/cli.py:74 | PROB-015 L61 窗口须远长于单件时长；queueing 档 §3 在档推导 |
| 7 | 推导 | GD_WINDOW_DAYS_DEFAULT=7 | sih-tools/gauge/src/gauge/cli.py:77 | PROB-013 L24 公理三窗口对价；显式给参不暗改 |
| 8 | 推导 | MIN_DECIDED=30 | sih-tools/facet/probes/reference_stats.py:30 | PROB-003 L7 正态近似；n≥30 族惯例锚照实申报 |
| 9 | 改判 | MAX_ROUND=3 | sih-tools/facet/probes/eir_ecr_gate_probe.py:78 | 采样轮数上限族，判据不由轮数承载 |
| 10 | 改判 | MAX_SHOTS=7 | sih-tools/facet/probes/lightweight_mode_probe.py:146 | 采样发射上限族，停采由 FLIP_STOP 承载 |
| 11 | 改判 | MAX_ROUND=5 | sih-tools/facet/probes/multiround_convergence_probe.py:108 | 采样轮数上限族，观测成本位 |
| 12 | 冻结 | BUDGET_PER_GID=9 | sih-engine/src/attractor/tally.rs:34 | R6 预算九发契约约定，无条目承载 9 |
| 13 | 冻结 | BUDGET_PER_GID=9 | sih-tools/tally/src/tally/cli.py:32 | 同上围堰形 |
| 14 | 冻结 | F21_THRESHOLD=0.25 | sih-tools/facet/probes/analyze_merged_v2.py:29 | 阶段二标定修订值 |
| 15 | 冻结 | F22_THRESHOLD=0.10 | sih-tools/facet/probes/analyze_merged_v2.py:30 | 同上 |
| 16 | 冻结 | F23_THRESHOLD=0.20 | sih-tools/facet/probes/analyze_merged_v2.py:31 | 同上 |
| 17 | 冻结 | F24_THRESHOLD=0.05 | sih-tools/facet/probes/analyze_merged_v2.py:32 | 同上 |
| 18 | 冻结 | F25_THRESHOLD=0.05 | sih-tools/facet/probes/analyze_merged_v2.py:33 | 同上（0 容忍改 5% 标定） |
| 19 | 冻结 | DEFAULT_FLYWHEEL_RUN_MAX_SHARE=0.70 | sih-tools/facet/probes/check_layer_proportion.py:48 | 实测 73.2% 基线预登记 |
| 20 | 冻结 | DEFAULT_LAYER2PLUS_MIN_SHARE=0.05 | sih-tools/facet/probes/check_layer_proportion.py:53 | 实测 0.44% 基线预登记 |
| 21 | 冻结 | THRESHOLD_FLYWHEEL_RUN_RATIO=0.60 | sih-tools/facet/probes/contribution_metric.py:85 | 源码注记自承非数学推导，不代条目立 derivation |
| 22 | 冻结 | DEFAULT_BOUNDARY_BASELINE=0.004 | sih-tools/facet/src/facet_stats_inf.py:48 | 实测翻桌率标定基线 |
| 23 | 冻结 | BOUNDARY_RATE_THRESHOLD=0.34 | sih-tools/facet/probes/r3a_gate_v2.py:63 | 占比判据在役线，切换挂起 pk-049，A2 实测来源 |
| 24 | 冻结 | THRESHOLD_N=5000 | sih-tools/facet/probes/contribution_metric.py:65 | 达标预登记 |
| 25 | 冻结 | THRESHOLD_C=5000.0 | sih-tools/facet/probes/contribution_metric.py:66 | 达标预登记 |
| 26 | 冻结 | THRESHOLD_NEW_MECHANISM_COVERAGE=1 | sih-tools/facet/probes/contribution_metric.py:68 | 覆盖计数门槛预登记 |
| 27 | 冻结 | USER_POSTURE_MIN=0.0 | sih-tools/facet/probes/program_signoff.py:99 | 用户裁决尺度端点 |
| 28 | 冻结 | USER_POSTURE_MAX=2.0 | sih-tools/facet/probes/program_signoff.py:100 | 同上 |
| 29 | 冻结 | TIME_WINDOW_HOURS_DEFAULT=0 | sih-tools/facet/probes/program_signoff.py:107 | 特征关闭缺省值 |
| 30 | 冻结 | VERIFY_TOKEN_DIFF_THRESHOLD=500 | sih-tools/facet/probes/run_siliconflow_factcheck.py:47 | token 差启发式，无条目族 |
| 31 | 冻结 | SKEW_MAX_SECONDS=300 | sih-tools/identity/src/identity/core.py:47 | 时钟偏移容差约定 |
| 32 | 冻结 | EPS=0.1 | sih-tools/facet/probes/n_convergence_probe.py:31 | LIM-007 只承载概念无值级锚，不硬挂 |
| 33 | 冻结 | GD_ALPHA_DEFAULT=0.05 | sih-tools/gauge/src/gauge/cli.py:79 | 候选值待裁（源码注记自承），与 pk-049 联动 |
| 34 | 冻结 | DEFAULT_INCONSISTENCY_MAX_RATE=0.0 | sih-tools/facet/probes/check_verdict_consistency.py:48 | 零容忍定性要求，非概率条目承载 |

逐件展开（条目行号实文引用与推导依据全文）落推导档 sih-math/docs/constclear-derivation-2026-09-04.md。

## 认证清单（链上 append 逐笔，全 meter 包裹，2026-09-04 链）

| # | 报告件 | 实跑退出码 | 链上事件哈希 |
|---|---|---|---|
| 1 | constclear-fmt-derivation.json（化格推导档） | 0 | 09f5416c |
| 2 | constclear-scr-des001-derivation.json（核阅推导档，sih-math 域外） | 2 | 6bad42bc |
| 3 | constclear-nom-derivation.json（检词推导档） | 0 | 2bca0eb1 |
| 4 | constclear-fmt-roster.json（化格名册） | 0 | 827a0fdf |
| 5 | constclear-scr-des001-roster-worktree.json（名册核阅工地路径域外） | 2 | ecb999a5 |
| 6 | constclear-scr-des001-roster-corpuscopy.json（域匹配副本终跑） | 0 | 23d66e85 |
| 7 | constclear-nom-roster.json（检词名册） | 0 | d6744a62 |
| 8 | constclear-fmt-results.json（化格结果档） | 0 | 56301550 |
| 9 | constclear-scr-des001-results.json（核阅结果档，event/plan 域外） | 2 | 0fdf7b41 |
| 10 | constclear-nom-results.json（检词结果档） | 0 | 07a31cdb |

自指限制申报：第 10 笔结果档检词认证在结果档定稿后上链，哈希不在本表（本档文本先于该笔固化），越线节照录；settle cert 取第 9 笔前八位。

## 心跳读数（selector 路由，参照时间 2026-09-04）

- 会话启动例行读数：gauge record 三维落链（convergence 0.428571 事件 686364cd、adoption 1.0 事件 d4700a58、mergeback 0.043478 事件 124653aa）；例扫零红照录见清账基数节。
- 泊界心跳批前：工具线 total 19（mainline 18、siding 1 即 pk-042 已知形）、引擎线 total 21（mainline 21），两线告警空退出码 0。
- 批后：pk-053 入泊新增一件，批后主树读数与归并读数见收口读数节（回填）。

## 越线与误差申报

1. 锁操作缺参一次：trail 追加面首次 unlock 缺 --identity 报 exit 2，照「工具 exit 2 先处置」即查用法补参重跑 exit 0 释放成功，零残留零越权。
2. 管道掩码两次（自查自纠照录）：其一 unlock 重试诊断的 tail 管道掩了真实退出码，即时改直取退出码；其二推导档核阅首跑经 tail 误读为 exit 0，落报告件重跑直取实为 exit 2（域外），两处均无治理写入被掩，发现即改全批后续直取退出码。
3. 工地拷贝失误两次（settle 前定稿面自查纠正）：dispatch.md 拷贝先于 mkdir 失败即补拷；cp -R 尾斜杠把 first-run 内容摊平即清理重拷，全部批件 cmp 逐字节一致后放行。
4. des-001 域外两笔如实记：推导档（sih-math 路径）与名册工地路径核阅 exit 2 域外不属违规，名册以域匹配副本终跑 exit 0 为准（副本用后即删不入版控）；结果档核阅同法域外记档。
5. 例扫额外对表：新跑账本与已提交版 literal 面 +1 与 caliber 面有差（并行批在途与源码演进所致），named 面零偏差；账本本体零改写。
6. env-params 再生差异：RECLASS_INHERIT 未纳入本批三件，脚本再生不含增量行，overlay 形随批申报（F-4）。
7. GD_ALPHA_DEFAULT=0.05 冻结与 pk-049 联动照录：α 族语义虽承 PROB-010，但该件源码注记自承候选值待裁非已裁，冻结为诚实态不冒认推导。
8. identity/reports 不入版控照禁区；主树既有存量 untracked（他批件）零收编，本批随批入版控只取任务包与 dispatch 与本批材料。
9. meter run 包裹命令输出经 2>/dev/null 对治（meter 无 --quiet 在案）；全程认证与锁与 settle 步骤直取退出码，零掩码。
10. 并行批 newcarr2-solo（432678779a3f0663）在跑：其 exclusive 面（math entries/INDEX/mapping 与其材料）与本批施工面零重叠，trail 追加面共享兼容即取即放零争用。
11. 结果档检词首跑违例一处即 dead_ban 禁词两字连字（PRO-007 术语功能轮），系认证表第五行四字表称中第二三字相连落入禁词所致，改语序为名册先于核阅后复跑 exit 0 零 findings，语义零损照录（poolclear 核阅首跑红转绿先例同款）。

## 冲突样本节

零。全程无撞锁无让位无 trail 竞态；七 exclusive 一次取清，append 短持面即取即放零争用；收约读数见收口读数节（回填）。

## 收口读数（收约后回填）

- settle 提交号（cert 0fdf7b41 全仓同签）：engine 04c5833 归并 b287f5e（base main@f6789b0）、math 01b9162 归并 c1521d9（base main@ecdfdcf）、tools 51acc716 归并 3826a79a（base integral-stage-build@23f5f6e8），三仓 settle exit 0 三闸全过。
- 放锁：九锁一次放清零持留，锁账对表 acquired 32 = released 32（九长持加二十四短持轮即取即放）。
- close 一轮成三仓全过：tools 与 engine 与 math 归并删支拆工地会话吊销；chain_union 两面即 meter counts 与 trail live_only_count 均 0、re_certify 空（快照后零活写丢失，并集复查闸零动作）。
- 四步法对表：主树十八件备份（任务包与 dispatch 与 first-run 全树与 pk-053 与报告件十四件）让位归并后 diff 逐件 IDENTICAL 零差异。
- reconcile：sih-tools 与 sih-engine 与 sih-math 三仓 exit 0，unrouted 与 cert_missing 与 session_orphan 与 unbypassed 相比批前零新增。
- 链 verify：valid，231 事件，first 05a8a75e 不变，末 07a31cdb（本批第 10 笔认证即结果档检词件）。批前开工 207 行（会话启动检时点）→ 完工 231 行：本批净增 15 笔即例行读数三维（686364cd 与 d4700a58 与 124653aa）加 intent f4cb68b8 加停泊 c01eb5e7 加认证十笔，其余 9 笔系他批共笔（newcarr2-solo 在途活动）直续零冲突零覆盖。
- 批后心跳：工具线 total 19（mainline 18、siding 1 即 pk-042 已知形）批前批后同态；引擎线 total 22（mainline 22、scrap_track 0、siding 0）即 pk-053 入泊 mainline 加一，两线告警空。
- 本节回填与认证表八至十行哈希回填走 close 通道外 --no-verify 提交加 lease bypass 登记（ordwire 与 pk037impl 与 basefix 与 mathscan 收口回填先例同形）。

## 队形验证

单线 solo 零子代理：全程主线程亲写亲跑，无 Agent 委外无并行腿，facet 零采样，三态分流全机械对照无 LLM 裁决位。
