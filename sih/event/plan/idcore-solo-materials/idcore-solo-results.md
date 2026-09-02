# idcore-solo 结果档

> 会话 b63050ab887b20ee（sess-zcode-260903-idcore 治理会话），单线形 solo 委外代理亲写零子代理
> 日期 2026-09-03，任务包 sih-engine/sih/state/plan/idcore-solo.md，决策材料 pk041-solo-materials/identity-drift-proposal.md §4/§7/§8 为准绳
> 凭据：用户 2026-09-03 采纳裁「正身修复同意，ai-ex沉淀同意」（inputlog 2026-09-03 seq 4 逐字在案）

## 一、F 表（逐项结论）

| F | 类别 | 判据 | 结论 |
|---|---|---|---|
| F-1 | 实装 | core_hash 金标逐字节；五易变件与盐全改核哈希不变；五向篡改各变；v3 三件 git diff 为空 | 过。金标 core-v1 串哈希 82f460c2ac2741fc4ee8d104d7ef14803f744da14a24b1d7356bd3bd3531d1dc 测试逐字节断言过（test_core_hash_golden_byte_exact）；五易变件（pid、ppid、parent_start、net_time、timestamp）全改加异盐核哈希不变（test_core_hash_ignores_five_volatile_and_salt）；mac、hostname、user、sandbox_id、血统 token 五向单点篡改各变（test_core_hash_five_way_tamper_each_changes）；identity_string、identity_hash、new_salt、COMPONENT_ORDER 四件经 git show HEAD 逐字节比对 identical，红线零触 |
| F-2 | 实装 | 报告新字段在场；退出码 0/1/2 三场景实测不变 | 过。报告 identity.core_hash 与 identity.core_components 在场（test_cli_report_shape_exact 更新断言含两新字段），header 增 net_probe 三态标记；活体实测 attest=0、mismatch=1、声明键域外=2，三值与既有语义不动 |
| F-3 | 实装 | 台账新会话 identity 字段含 core_hash；旧键全保留 | 过。load_identity 增读 core_hash 入台账 identity 字段（test_open_ledger_row_carries_core_hash）；旧报告缺席即空缺不判败、identity_hash 与 file_sha256 全保留（test_load_identity_legacy_report_without_core_hash）；make_session_id 派生与 lockcore 三键硬闸零改动经 git 比对证明 |
| F-4 | 实装 | 四漂移报告材料对旧基线回放落回退路径同判挂起；同席新基线含 core_hash 即配对成立 | 过。R5 核哈希优先配对实装，双带 core_hash 即核配对（test_r5_core_hash_prio_pairs_even_when_full_drifts：full 漂移不挂起）；任一缺席回退现行为，旧材料重放逐字节同判（test_r5_fallback_replay_byte_identical_to_pre_idcore、test_r5_fallback_when_core_hash_absent_replays_old_verdict）；R5_VALUES 三态映射经 git 比对 identical。2026-09-02 四漂移件（58e22070、14cfa9c2、9634b1de、d9fd7386）按提案 §5.2 T7b 读数同席 core 全同 82f460c2，新方案下机械认定为同一席位 |
| F-5 | 文档 | 三 CONTRACT 修订各一条，修订号递增 | 过。identity 修订八、lease 修订二十三、tally 修订六，各承既有修订节体例，版本位零动（纯增量落位承任务包请求写入节范围，版本文件不在写入节） |
| F-6 | 测试 | 三工具 pytest 全绿计数入档；新增用例先红后绿或随实装绿 | 过。identity 49 绿（新增核哈希组 15 测）、lease 62 绿（新增 3 测）、tally 17 绿（新增 4 测），合计 128 绿零跳零红；存量报告 full 复算逐字节回归实跑核了 40 份带全哈希件逐字节一致（判据下限 39） |
| F-7 | 收口 | 链 verify 0；reconcile 双零；管线 findings 亲读留证 | 过（见 §七 链对表与 §五 申报）。ask3 双门两 0、叩问 digest passed covered 3、认证逐件上链、双仓 settle、reconcile 双仓四类双零、当日链 verify valid |

## 二、通道错配改道记录节

命题 m-idcore 与 m-idcore2 各经一轮五发重采后 gate 落 boundary，两轮读数如下（flywheel-trail 只读摘录，原文在 sih-tools/proposition/DES/m-idcore{,2}/flywheel-trail.jsonl）：

- m-idcore gate_assessment（2026-09-02T18:38:41Z，layer 1）：verdict_v3=boundary，v3 子判据挂 ['basis_consensus', 'decision_stable']，n_runs=5，boundary_rate=0.2，distinct_basis=[baseline_1, baseline_4]，noise_models=[MiniMax-M2.7]，dc_fingerprint=40bb04a9da0f3965。
- m-idcore2 gate_assessment（2026-09-02T18:40:59Z，layer 1）：verdict_v3=boundary，v3 子判据挂 ['decision_stable', 'basis_consensus']，n_runs=5，boundary_rate=0.2，distinct_basis=[baseline_1, baseline_5]，noise_models=[MiniMax-M2.7]。

改道依据：sihankor-facet-measure 修订一——两轮反对集中于起草权元问题非机制内容，落确定性核对通道不三跑。即不再发起第三轮同通道重采，改以本批确定性通道承接：金标常量逐字节断言、存量报告 full 复算回归、R5 旧材料重放同判、退出码三场景实测，四类机械核对全部在案（§一 F 表）。通道错配如实载入不藏，boundary 未过即改道非硬闯，承 06-on-canon 法四顺势与 07-on-assay 外部独立验证对冲。

## 三、金标与回归读数

- 金标串：core-v1|mac=1cf64c65312e|hostname=MiniServer|user=moc|boottime=1787917459|sandbox_id=|lineage=zcode-cli>zcode-host-local>ZCode
- 金标哈希：82f460c2ac2741fc4ee8d104d7ef14803f744da14a24b1d7356bd3bd3531d1dc（测试逐字节断言与本批批前独立复算双证一致）
- 存量回归：identity/reports 42 件中 40 份带全哈希件按 盐|v3 身份串 用工具自带函数复算逐字节一致，零不符（2 份无哈希空件除外），判据下限 39 成立
- 归一化四例：host-local-1 与 host-local-2 同 token 同核；zcode、MiniMax、TRAE 三 harness 两两异；空 ancestry 出空 token 不判败；通用包装段集剔除（python3>uv>zsh>x>launchd → x）
- 复演双跑：--salt 加 --inject 加 --at 同参双跑 stdout cmp IDENTICAL
- 离线：--no-net 活体 net_probe=disabled、退出码 0；net_time 置空核哈希不变；boottime 加一秒核哈希变（换纪元语义与租约 boottime_drift 拦截一致）

## 四、词债登记清单

检词 core 包 terms.json 三条 established 登记（manifest 0.6.0 升 0.7.0）：

| 词 | en | 说明 |
|---|---|---|
| 核哈希 | core_hash | 身份核六件无盐确定性 SHA-256，core-v1 键序固定，同席同日连认键 |
| 连认 | cross-report recognition | 同一逻辑身份跨报告机械等值认定，配对键只认核哈希 |
| 身份核 | identity core | 十二组件中稳定子集六件，进核哈希；易变件与新鲜度件不进核 |

叩问信号三笔（核哈希、连认、身份核，2026-09-03-idcore-solo-elicit-signals.ndjson）经登记消解，elicit check 复验零信号，digest passed covered 3。

## 五、越线与误差申报

1. 调用册留痕落位：任务包请求写入节未列各工具 CALL-LOG.md 路径，段1 settle 范围面（allow 十四路径）不含之；按指令链序「reconcile → 链 verify → 调用册留痕 → 结果档 → 全部产物含链尾随批入版控」，调用册行与链尾、账本、meter 计数同落尾随入版控段（承 sweepclea6-solo 段2 bffbee02 先例），不在段1 范围面。触改工具四册（identity、lease、tally、nomenclator）各一行随尾随段入版控，scribe、formatter、scrutinator 三者仅调用未改码，其一行留本档 §六。
2. 认证报告件落位：管线认证报告 JSON 落 sih-tools/scribe/reports/（锁内账本与 scribe/reports 例外承先例），哈希入链不入版控。
3. close 归并对表：主树同名未跟踪件仅任务包一件，备份让位归并后 diff 备份对归并件的差异应恰为勾选七行即本批意图内变更，逐字对表留证于完工链路；dispatch.md 主树未跟踪件不进工地零改动，尾随段原样入版控。
4. tally assemble 计分材料透传 core_hash 不在本批（实装要点限 R5 段约 134 至 169 行），R5 判定面已就绪、材料侧透传属后续批，并存期旧材料自动回退现行为无碍。
5. 本批 identity/lease/tally 版本位零动（pyproject 与 __init__ 不在任务包写入节），三 CONTRACT 修订条目内如实注记；检词 core 包 manifest 在写入节内照升 0.7.0。
6. 正身件 2026-09-03-idcore-identity.json 承零写路径只 stdout 由批流程落盘，承任务包禁区 identity/reports 新报告不入版控。
7. 任务包禁区行死词一处（粤语义死档词，检词 dead_ban 拦）随批修正为入版控，语义不变，检词复验零违例；结果档同词三处同改。

## 六、调用册

七册各一行已落各 CALL-LOG.md 尾（identity、lease、tally、nomenclator 触改四册与 scribe、formatter、scrutinator 调用三册），随尾随段入版控。

## 七、链对表（终稿）

| 节点 | 事件数 | 尾哈希 | 事件 |
|---|---|---|---|
| 批前 | 8 | 14e4d03ff028cf7a | certification_completed（intanchor 段尾） |
| intent 上链后 | 9 | 529473138450db92 | intent_refined（本批意图，record_hash b7ae1a74f750605e） |
| 并行件 | 10 | 40b9b6669373cd8e | parking_entered pk-043（并行走会话所写非本批，如实申报） |
| tools 认证后 | 11 | 8552b6ae8f99da53 | certification_completed（tools 管线报告 cb5eb724cabf3cb1） |
| engine 认证后 | 12 | 0c43f8225219406c | certification_completed（engine 管线报告） |

- scribe verify：status valid，12 events，first 94f1dd00d94ad3b7，last 0c43f8225219406c
- 双仓段1 settle：tools 6d0c83c6（cert 8552b6ae，base integral-stage-build@db4e7a18）、engine dac3393（cert 0c43f822，base main@ca983b6），四验 checks 全过
- close 归并：tools f45a52ac、engine c0df454，双仓 branch msh/idcore-solo 删支、worktree 自删、拆本吊销一次成
- 归并对表：任务包备份让位 diff 恰为勾选七行加死词修正一行即意图内变更，其余零差异
- reconcile：tools unrouted 0、cert_missing 0、unbypassed 0、bypass 0；engine unrouted 0、cert_missing 0、unbypassed 0、bypass 1（存量常态在案）——四类双零
- 链 verify 终态：valid（上表）

## 八、队形声明

单线形 solo——委外代理亲写零子代理。测量采样属命题面两轮在案旧账（§二），本批未发起任何新的模型采样或委外代理调用；全部实装、测试、管线、认证、结算由本会话单线执行。
