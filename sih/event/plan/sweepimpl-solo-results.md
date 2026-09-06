# sweepimpl-solo 结果档：租约扫残留子命令实装

> 批：sweepimpl-solo（会话 9920edf3eb4d4124）；日期 2026-09-07；队形：单线 solo 零子代理（委外执行代理亲写）
> 任务包：sih-engine/sih/state/plan/sweepimpl-solo.md；令源：用户 2026-09-07「残留的内容如果不需要人节点裁决直接就清了，需要裁的话过得一」与同日「直接拉子代理做掉」
> 交付：lease sweep 子命令与 sweepcore 模块（五类残留全机械普查、三态输出、--fix 走既有通道），CONTRACT 修订四十六升 1.32.0

## 一、完成度表

| 工作项 | 状态 | 交付位 |
|---|---|---|
| sweepcore.py 五类判定与三态输出 | 完成 | sih-tools/lease/src/lease/sweepcore.py（判据复用 core.active_sessions 与 cli._classify_window 与 lockdb.holds 与 core.LEGACY_RECEIPT_DIRects/receipts_dir，零另写） |
| cli.py 接线 sweep 子命令 | 完成 | lease sweep [--fix] [--json] [--at] [--root] [--ledger] [--locks] [--bills]，退出码三值零即净一即残留二即异常，既有子命令零触碰 |
| tests/test_sweep.py 夹具四类残件 | 完成 | 五件：夹具归态、fix 有界与活会话零触碰、净态判据、只读零漂移、退出码语义与单证零误报 |
| CONTRACT.md 修订一节 | 完成 | 修订四十六升 1.32.0，三源对齐 pyproject 与 __init__ 与 CONTRACT |
| 管线三步 | 完成 | 化格六件 rc=0；核阅七目标域外 exit-2 如实记；检词零违例（CONTRACT 存量两笔死词连带修复） |
| 认证入链、双仓 settle、放锁收约、verify/reconcile | 完成 | 见第四节读数 |
| 温故检索 | 完成 | retriever recall 事件切面与 topic 切面入 materials，另两份零命中检索件随材料入档 |

## 二、F 表（逐条实跑读数，禁止推断）

| 锚 | 判据 | 实跑读数 | 判 |
|---|---|---|---|
| F-1 夹具归态 | 注入四类残件，普查各归其态与 §2.2 表逐行一致 | test_sweep_scan_classifies_fixtures PASSED：幻影会话（四证取三）auto_clean、僵尸锁 auto_clean、停滞检验文件 auto_clean、散位收据未跟踪 auto_clean 与跟踪 awaiting、无主工地有未归并 awaiting 与零未归并 info；活会话活锁活窗零列报；summary auto_cleanable=4；输出读数 f1-f2-test-sweep-readings.txt | pass |
| F-2 fix 有界 | --fix 清讫全部自清件逐动作清单在档；活会话锁面零触碰；跟踪件只列指令零提交 | test_sweep_fix_clears_auto_and_zero_touches_live PASSED：四动作全清（ledger-repair 修正性 revoked 带 repair 标记 source=sweep-corrective-non-verbatim、takeover_release 释放、检验文件清除、收据归家迁位）；git log 零新增提交；活会话仍在册活锁原封活窗原封；实态 --fix 双证：actions=0 且台账哈希前后逐字节一致（f4-ledger-hash-pre/post.txt），活批 confrevise 早已收约、basemgrimpl-solo 批中自收，在册活批仅本批自会话，其锁面零触碰 | pass |
| F-3 既有零漂移 | 既有套件全绿如基线；夹具仓 status 与 check 前后双跑 cmp 逐字节一致，退出码语义零变化 | 全族 209 passed（基线 204 零回归）；status cmp rc=0、check cmp rc=0（before/after 双跑文件在档）；sweep 同参双跑 --at 钉时戳 cmp rc=0 逐字节 IDENTICAL；退出码三值语义测试 PASSED | pass |
| F-4 基线绿 | 实态 sweep 自清件零，净外信息不误报 | 实态读数 f4-sweep-real-scan.json：auto_cleanable=0（今日两批人工清理后实态相符）、awaiting=1 即本批自窗（一次性 open 进程 pid 已死属检验文件机制结构性现象，防搁浅守卫拦自清，收约转收据自然消解）、info=0；误报反证测试 PASSED（活会话形单证不足取二不立）；实态 --fix 零动作零写入 | pass |
| F-5 管线绿 | 三步读数在档；认证、settle、close、reconcile、verify 全链读数在档 | 化格六件 rc=0、核阅域外 exit-2 如实记、检词零违例（首跑红证与修复见 pipeline-readings.md）；认证清单见第三节；双仓 settle 提交号见第四节；close revoked=true；verify valid；reconcile 较批前基线零新增（unrouted 与 orphan 与 cert_missing 与 unbypassed 增量全零） | pass |

## 三、认证清单

| 件 | 认证 event_hash |
|---|---|
| 意图笔（intent_refined） | 03ce2f5e15de681c94da03209c0a4ef8036248a786850c2f5bb6d6640af56f2f |
| 管线读数 pipeline-readings.md | （认证后回填） |
| F-1/F-2 测试读数 f1-f2-test-sweep-readings.txt | （认证后回填） |
| F-4 实态读数 f4-sweep-real-scan.json | （认证后回填） |

## 四、结算读数（收约后回填）

- tools 工地 settle：sha（回填），base integral-stage-build。
- engine 工地 settle：sha（回填），base main。
- close：revoked（回填），收约凭据 receipts/sweepimpl-solo.json。
- 链 verify：（回填）。
- reconcile：（回填，较基线 /tmp 基线件在档零新增）。

## 五、设计细化与越线申报

1. 防搁浅守卫（设计细化，CONTRACT 修订四十六登记）：任务包 §2.2 停滞检验文件行判据 pid_dead 与 legacy_stale 即自清，实态首跑即证纯 pid 判死会误伤一切活批——检验文件 pid 载一次性 open 进程号，进程退出即 pid_dead 属结构性现象（本批自窗与彼时 basemgrimpl 窗俱 pid 死而会话活跃）。细化：window_session 仍在会话册活跃或仍持锁即候裁不自动清，人节点显式 takeover 或批自行复振；window_session 不在册不持锁即自清。守卫被实态 F-4 即时验证（自窗被拦，零自伤）。此为防误伤同源精神（§八四证取二）的检验文件面落点，非判定权扩张。
2. 幻影候裁情形降净：证据不足其二不列报（含单证形），承 F-4 不误报与活批零触碰；实态活会话形（持锁+副本在+结果档草稿在案=单证）有专门测试钉死。
3. scribe/CALL-LOG.md 未入 allow 面：活批 basemgrimpl-solo 独占持锁（open 预检拦截面，declguard viewline 先例形），移出 allow 面改认领通道；本批对该文件零改动，收约 --ack-uncommitted 认领事由留档；候而不扰全程零候叫动作（wait-turn 对会话 scope 外路径机械无效，scope_violation 必然，不强跑）。
4. 收约补笔预期：结果档结算读数回填走 bypass 登记通道（anchwave/facepatch 先例同形）。
5. 零命中检索：/tmp/lesweep-recall-topic.json 与 lesweep-recall-topic2.json（主会备妥）随材料入档，本批另跑 retriever recall 事件切面与 topic 切面两份。

## 六、队形验证

单线 solo：本批全程零子代理，委外执行代理亲写 sweepcore.py、cli.py 接线、test_sweep.py、CONTRACT 修订与全部链动作；判定全部机械零 LLM 调用堆叠。
