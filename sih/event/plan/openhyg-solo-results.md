# openhyg-solo 结果档：开约面卫生批

> 会话：4e5f2ad1dee20ffa（双仓租约 tools@integral-stage-build + engine@main）
> 任务包：sih-engine/sih/state/plan/openhyg-solo.md（唯一规格源）
> 令源：用户 2026-09-05「先修租约」令 + 四条修复计划 + 设计对话三点裁定
> 工具版本：lease 1.23.0 → 1.25.0（本批升版）

## 一、事故与修复对位 {#accident}

| 事故 | 本批修复 | 落点位 |
|---|---|---|
| 缺陷一：裸 stem 只搜引擎计划面，包在跨仓合法位解析失败；相对路径按 cwd 解析 | TASK_PACKAGE_DIRS 冻结常量三目录 + 按表解析 + 相对路径按 root 拼接 | core.py resolve_package |
| 缺陷二：检验钥匙闸（副作用）先于包校验 | open 闸序重排：前五位只读化（preflight），钥匙闸降第六位首个副作用位 | core.py open_preflight / cli.py _cmd_open |
| 缺陷三：失败留残件堵重试 | 失败自收桌：所有权核验（opened_at+pid）删自家文件，两错并报 | cli.py _self_clean_check_file |
| 缺陷四：死文件时间戳当活进程信号，烧人类注意力 | PID 探针三态判 + 死 PID 机械自清留 stale_cleared + takeover 同判据 | cli.py _classify_window / lockdb.py record_stale_cleared |
| 设计裁定补洞：时间戳单判会误删活窗 | 探针不可知保守判活 + 活窗慢签发夹具证不误删（用户点名洞） | test_openhyg.py 活窗夹具 |

## 二、F-1 至 F-5 逐条判定 {#factors}

- **F-1 登记表解析：过。** math 侧 stem 命中即开（test_resolve_math_side_stem_hit + test_open_math_side_package_full_chain）；歧义列双径拒（test_resolve_ambiguity_lists_hits_and_refuses，报文含两条命中径与 refuse to guess）；零命中列全搜索面（test_resolve_zero_hit_lists_all_search_faces，三目录全列）；相对路径按 root 拼接（test_resolve_relative_path_joins_root_not_cwd，constmodel 缺陷红证收编）。
- **F-2 检验文件生命周期：过。** 出生全形（test_open_born_complete_check_file：identity_core=c×64、opened_at、pid=os.getpid()、pid_started 落盘即有）；前五位任一失败零残留（包不存在、歧义、意图血统败、预检拦截四红证夹具全过）；签发前失败自收桌（test_self_clean_on_execute_failure：worktree add 注入失败后检验文件已删）；所有权核验夹具在档（他窗覆盖不误删 + 两错并报两件）。
- **F-3 PID 判生死：过。** 误跑红证：活 PID 二开拒（test_live_pid_misrun_double_open_rejected，reason window_active basis pid-probe）；死 PID 自清留 stale_cleared 事件并放行（test_dead_pid_residue_self_clears_and_proceeds，事件表一笔 detail 载原文件快照，复核 SELECT 在档）；活窗慢签发夹具（PID 活、心跳停滞超阈）证不误删（test_live_window_slow_issuance_stale_heartbeat_not_deleted，reject window_active 且文件字节未动——用户点名洞）；无 pid 旧形回落心跳（test_legacy_form_fallback_heartbeat_criteria：新鲜拒同跑、停滞报僵尸，detail basis=heartbeat-fallback 如实申报）；跨启动复用对冲（test_pid_reuse_start_time_mismatch_treated_dead）；takeover 升级（test_takeover_dead_pid_clears_even_fresh_heartbeat：死窗心跳新鲜亦清并放锁）。
- **F-4 判定语义一裁：过（stable_clear，非 near_threshold，无需呈主会）。** facet 合同模式九发（gid m-openhyg-selfclear-1），9/9 comply 变卦 0%，谨慎信号 0/9，席位当日基线可用（体温 0.0，四命题对向全对）；attractor check 十二项全过 verdict pass；verify identical；机器终签链笔 7018cade（event cf7356c1，doc_id crosscheck-m-openhyg-selfclear-1）；对己不利声明与谱系披露在 topic.md。
- **F-5 零回归：过。** 基线实查 111 件（开工时收集确认）+ 本批新增 28 件 = 全族 139 件全绿；sessions 台账 issued 行字段集合零变更（test_sessions_issued_row_format_unchanged 十二键断言 + repos 子形态四键断言 + identity 三哈希断言）与确定性双跑逐字节一致（test_deterministic_double_run_unchanged）；ledgerwrite 写点唯一化机制零动；v3 正身体系与 identity_hash 与 core_hash 零动。

## 三、TDD 先红后绿实录 {#tdd}

- 红相：test_openhyg.py 28 件对旧代码跑，20 败 8 过（8 过者为新旧行为重合的回归守卫：引擎位命中、绝对路径直传、must_exist=False 回落、已让位文件不误删、行格式、双跑确定性等）。红相逐条清单落 materials/red-phase.txt。
- 绿相：实装 core.py（TASK_PACKAGE_DIRS、resolve_package 重写、open_session 拆 open_preflight+open_execute）与 cli.py（_pid_alive/_pid_started/_classify_window/_self_clean_check_file/_record_stale_cleared、_check_key_gate 重写、_cmd_takeover 升级、_cmd_open 闸序）与 lockdb.py（record_stale_cleared）后，全族 139 件全绿。
- 既有测试适配四处（如实申报）：①test_lockdb_key_gate_same_run_and_stale 二开拒因随闸序改 PackageSessionActive、出生即全断言增补、旧形停滞段先收约清台账再剥 pid 构造；②test_lockdb_takeover_stale_only 活窗拒改走探针基准、停滞段剥 pid 构造旧形；③test_lease test_zero_write_surface_closed unlink 写面预算 3→5（新增自清与自收桌两豁免位，沿用 leaseopt 线预算适配先例）；④test_lockdb_key_gate_imports 不变（_check_key_gate 仍在 cli 面）。

## 四、编排定位执行 {#orchestration}

- 本批先行令执行：watchcheck-solo 与 idenlane-guard-solo（批 C）候本批收约后开工，未抢未绕。
- 开工预检实录：open 时锁台账零在途独占持锁（批 B idenlane-human-solo 已于 2026-09-05T16:06Z 收约），零排队候位发生。
- 在途判定依 packhyg 条款：活跃会话 leaseopt-fixguard-solo（eaf80aa8）与 idenlane-solo（b8a6f90c）为旧账在册，持锁面零，判定非在途不代清。

## 五、活体验收 {#live}

- docmath-namefit-solo（本批事故正主）以裸 stem 按新语义重开：exit 0，会话 1c924498519b0dc8，包解析落 sih-engine/sih/state/plan/docmath-namefit-solo.md，检验文件出生即全（pid 48449 出生即在，session_id 签发后回填）；验收即收：close exit 0 revoked，凭据转写 plan 面活件清除。读数落 materials/live-acceptance.txt。

## 六、申报与偏差 {#declarations}

- 例行读数：三维落链 ga-2（convergence 0.0 / adoption 0.0 / mergeback 0.041667），零红如实转述。
- 泊界心跳：双线 alarms 空（tools 线 mainline 18 siding 1 即 pk-042 既有态；engine 线 mainline 29 scrap_track 6 既有态），零告警。
- watch 对表：检出无主 19 件，全部为 2026-09-05T16:06Z（idenlane-human-solo 收约窗）在盘遗留件，与 BATCH-FACE watchcheck-solo 批呈报在案清单同款，本批不豁免不代清，候人节点裁决如实转述。
- 术语八条随批入 core pack（自清、残件、探针、活窗、任务包目录登记表、出生即全、自收桌、接管），化格归一零改动，nomenclator 全族 30 件绿（装船正典在档）。
- 测试期锁库写面申报：测试缺省 tool_dir 锁库为工地副本 locks.db（不入版控），主树锁库零测试污染（亲查 lock_event 无测试笔）。
- 主树活写申报：检验文件 sih-tools/lease/ledger/checks/openhyg-solo.json 与链文件与台账为治理运行面，经引擎 scribe 与 lease 工具写位承载；其余待提交件全部经双仓工地 settle 通道。
- 活体验收会话 1c924498519b0dc8 的 issued/revoked 两行入真实台账为验收证据链，行格式与既有零异。
- settle 范围闸申报一：首次活体验收自工地缺省台账位入行（tool_dir 随工地源码定位），settle 范围验正确拦下工地位 sessions.ndjson（allow 面未含 lease/ledger），处置即工地位重置归零、验收对真实台账显式 --ledger/--locks 重做（新会话 1c924498519b0dc8），首跑行 90e0a99cd287edb0 随工地位重置消散不入任何台账如实申报。
- settle 范围闸申报二：标定账本 facet/probes/calibration/ledger.jsonl 一笔席位基线行落工地位，同因 allow 面未含不入批产，工地位重置归零；基线证据本体不受影响即 seat-baseline.json（已入合同目录随批入版控、哈希绑定入执契材料与链笔 7018cade），账本行缺席以此申报兜底。

## 七、收约读数 {#close-reading}

收约链（settle + close + reconcile + verify）读数由完工报告呈报并经收约补笔入本节（先例同形）。
