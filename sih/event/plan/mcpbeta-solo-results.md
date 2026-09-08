# mcpbeta-solo 批结果档：mcpline beta 相写面实装

> 线：mcpline（线程序包 sih-engine/sih/state/plan/mcpline-line-v1.md 批三）；形：solo 批独立立约独立收约
> 设计正典：sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md（sha256 fa7f2cc59603bd79，与任务包对表一致）
> 会话：a364e862ea348078；意图笔：intent_refined bdea246b20be8914（2026-09-09 链）

## 一、使命达成申报

按 DES-014 八问判词实装 MCP beta 写面于 sih-tools/mcpline/（版本 0.1.0 → 0.2.0）：连接即 lease 会话一对一、进程正身签发、授权矩阵路由裁剪透传、写操作全走既有 CLI 执法零新增判定、失败语义静态映射。alpha 五只读工具零改动共存（回归绿）；既有工具代码（scribe、lease、gauge、critsweep、nomenclator、basemgr）零改动；真仓零写入（套件级双仓 git status 全等证明绿）。

## 二、实装形逐节对表（DES-014 判词 → 实装位）

| DES-014 节 | 判词 | 实装位 |
|---|---|---|
| 一 连接与会话映射 | 一对一，连接建立即立会话，断开走收约，禁多对一禁池化 | writeface/session.py：ConnectionSession 绑定形 + auto_open_from_config（连接级自动开）+ disconnect_close（零写轻收约/有写全收约/未成显形候 takeover）；server.py：进程始即连接生（atexit 断开钩）；二次 open 结构性拒 |
| 二 身份 | 进程正身形，签发即采集，逐写复核 | session.ensure_identity：identity verify 生产路径采集（v3 十二件，盐随机），lease open 与 lock/unlock/wait-turn 逐写传正身报告复核；SIH_SESSION_ID 连接派生值注入 |
| 三 授权 | 路由层矩阵形，零 LLM 裁决位 | writeface/matrix.py：MATRIX_ROWS 十二行静态表 + EXTERNAL_ALLOWED 十行最小写集；分级经 SIH_MCPLINE_AGENT_CLASS（缺省 external 最小特权，非法值按 external）；无行即不注册即拒透传（takeover/bypass 结构性缺席） |
| 四 写路径复用 | 零新增判定，薄投影 | writeface/passthrough.py + tools.py：参数透传、结果透传、退出码透传；scribe 三护栏与 lease 六闸五验与 closeguard 与 git hooks 原位（fixture 实跑逐闸证）；禁令三条承载：无 allow/deny 替代执法、无执法结果缓存、零直写盘（grep 守卫，tempfile 正身暂存单点豁免） |
| 五 审计与重放 | 承既有链 | intent 以 record 路径为幂等键（闸二拒重放实跑证，IntentRecordUsedRejected 原样透出）；append 无幂等闸如实申报（README 与工具描述载明，重复笔在链可审计）；锁冲突理由码不吞不译不重试代劳；wait-turn 独立工具透出（timeout 到如实返退出码） |
| 六 失败语义 | 分级裁剪静态映射 | writeface/errors.py：REASON_TABLE 十七条静态表（理由码/闸位/释义/动作）；路径相对化两级通用；外部位他会话号裁剪与预检冲突只载计数；零动态拼接不可信数据 |
| 七 威胁模型 | 三向缓解或明示不防 | 工具描述冻结为契约文本（静态、无插值、只载语义）；明示不防位（进程劫持实时阻止、客户端 LLM 注入后主动调用、台账膨胀）在 README 与描述文本照录 |
| 八 边界申明 | 不触碰 GOV-002 与 v3 与 pk-079 | 实装以现行工具版本为界（lease 1.39.0、identity 0.4.0、引擎 scribe 现行）；零版本预设零换版触碰 |

## 三、矩阵行覆盖表（每行对应测试名）

| 矩阵行 | local | external | 测试名（tests/test_write_matrix.py 除注明外） |
|---|---|---|---|
| lease_open | 可调 | 可调 | test_matrix_row_lease_open_both_classes |
| record_intent | 可调 | 可调 | test_matrix_row_record_intent_both_classes |
| record_append | 可调 | 可调 | test_matrix_row_record_append_both_classes |
| record_park | 可调 | 可调 | test_matrix_row_record_park_both_classes |
| record_direct | 可调 | 拒透传 | test_matrix_row_record_direct_local_only；外部位调用拒透传：tests/test_write_gates.py::test_external_class_refuses_direct_and_unclaim；透传正形：tests/test_write_gates.py::test_direct_passthrough_local_only |
| lease_lock | 可调 | 可调 | test_matrix_row_lease_lock_both_classes；五验透传：tests/test_write_gates.py::test_lock_unlock_passthrough_and_scope_gate；锁冲突：tests/test_write_gates.py::test_locked_elsewhere_conflict_surfaces |
| lease_unlock | 可调 | 可调 | test_matrix_row_lease_unlock_both_classes；加持锁验透传：tests/test_write_gates.py::test_lock_unlock_passthrough_and_scope_gate |
| lease_wait_turn | 可调 | 可调 | test_matrix_row_lease_wait_turn_both_classes |
| lease_claim | 可调 | 可调 | test_matrix_row_lease_claim_both_classes；claims 拒：tests/test_write_gates.py::test_claims_and_unclaim_routing |
| lease_unclaim | 可调 | 拒透传 | test_matrix_row_lease_unclaim_local_only；tests/test_write_gates.py::test_claims_and_unclaim_routing |
| lease_commit | 可调 | 可调 | test_matrix_row_lease_commit_both_classes；四验透传：tests/test_write_gates.py::test_commit_wip_passthrough |
| lease_close | 可调 | 可调 | test_matrix_row_lease_close_both_classes；链闸透传：tests/test_write_gates.py::test_close_chain_gate_and_session_not_active |
| （takeover/bypass） | 拒透传 | 拒透传 | test_takeover_bypass_absent_from_matrix_and_registry |
| （分级缺省与非法值） | — | — | test_default_agent_class_external；test_invalid_agent_class_maps_external |
| （注册面裁剪） | 十七工具 | 十五工具 | test_tool_listing_local_class_full_face；test_tool_listing_external_class_minimal_face；stdio 面：tests/test_stdio_smoke.py::test_stdio_smoke_local_class_full_face |

一对一与身份与断开收约与失败语义测试：tests/test_write_gates.py::test_open_binds_single_session、test_open_twice_refused_one_to_one、test_write_before_open_refused、test_open_gate_intent_validation_in_place、test_disconnect_zero_write_light_close、test_disconnect_with_writes_full_close、test_error_payload_trims_paths_and_foreign_sessions。

## 四、测试读数

- 全量套件：**55 passed，0 failed**（终跑 43.61s，退出码零；全文 log 落本批 materials/full-suite-final.log）。
  - 矩阵行组 15 绿；零写守卫与证明组 5 绿；alpha 单测组 15 绿；闸位透传组 17 绿；stdio 冒烟 3 绿。
- fixture 写径 stdio 冒烟闭环（tests/test_stdio_smoke.py::test_stdio_smoke_beta_write_path_verify_loop，摘录 log 落 materials/beta-smoke-excerpt.log）：lease_open 立会话（会话号十六位透出）→ lease_lock → record_intent（会话号与 identity_hash 信封在链）→ record_append → chain_verify（alpha 读工具，status valid，events==2）→ record_park → lease_unlock → lease_close，八步退出码全零。
- alpha 五只读工具回归：test_stdio_smoke_alpha_five_regression 真根重跑绿（chain_verify valid、chain_query matches>=1、heartbeat 三维、locks_read 成对核算、critsweep 五判据键）。
- 真仓零写证明：test_zero_write_proof_whole_suite_window（conftest pytest_sessionstart 快照 vs 套件尾双仓 git status --porcelain 全等）与 test_zero_write_proof_alpha_five_calls_real_repos 双证绿。
- 首跑红证九发如实留痕：materials/red-evidence-first-runs.md；实装代码红证两发（direct 漏 --sessions、commit repo 根锚缺位），俱补参/归一修复，判定语义零改动，DES-014 零触碰。

## 五、实装形补注（候主窗复核）

1. 连接级 open 的参数承载：DES-014 第一节定「握手完成后 server 调 lease open」——立会话所需 package 与 intent 系客户端域参数，server 不可知，故实装两形：部署配置齐备（SIH_MCPLINE_PACKAGE/SIH_MCPLINE_INTENT 等 env）即连接建立时自动开；缺省形由客户端经 lease_open 工具显式开。两会话形俱守一对一（二次开结构性拒）。矩阵 open 行两分级俱可调照落。
2. 断开轻收约形：零写连接 close 传 --trail 指本会话专属链径（零写会话该径依事实不存在），链闸按既有语义空转并如实注记 workspace_chainless（lease 既有「零链面守门空转零静默」形）；有写连接走缺省全链形链闸严格执法。--force 与 bypass/ack 旗标永不透传（server 不强拆，人节点裁决位不代行）。
3. 路径参数根锚定归一：record/validation/report/intent/commit repo 相对形按数据根拼绝对（lease commit 的 Path.resolve() 无根锚位、子进程 cwd 无关），静态归一非判定。
4. 调用方分级承载：SIH_MCPLINE_AGENT_CLASS 静态部署配置（local/external），缺省 external 最小特权；分级是部署面声明非运行时判定，零 LLM。
5. proposition/DES/m-mcpbeta-1/ 未用：本批验收形无得一测量（任务包第五节验收不含 facet/tally），allow 面该径零写入，如实申报。

## 六、误差申报

- 首跑红证九发（见红证档），无清洗未删档；修复俱在测试工场与调用参形与补参，无一处改既有工具、无一处改判定语义。
- 共享面锁经 wait-turn 排队取得（specfix-solo 在飞持 trail 与 reports 与 identity 三径，wait-turn 三发 exit 0 取得，waited=true，零绕行零 preempt）。

## 七、收约读数（结算时回填）

- 链笔：intent bdea246b20be8914；认证笔与本批 exit 笔见当日链；链 verify 全文见完工回报。
- 双仓 settle：tools 提交与 engine 提交哈希见完工回报；reconcile 增量双零见完工回报。
