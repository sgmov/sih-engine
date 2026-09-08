# critsweep-solo 结果档：判据扫 v1——对话框内治理态回算器与启动序接线

> 承接：任务包 critsweep-solo.md 与用户 2026-09-08 令「同意，你拉起子代理跑」即判据扫候选设计甲案开工令；上游思想源即 2026-09-07 与 09-08 主会话裁定「这个要有一个完整的机械化流程，视图实装是另外一个概念」，判据沉底须机械召回不靠 LLM 语义撞见。
> 队形单线形 solo 委外代理亲写零子代理，日期 2026-09-08，会话 sess-zcode-260908-critsweep（租约 session_id b06c3c1e5b58feed）。

## 意图锚定

- 意图事件：intent_refined（event_hash 前 8 `ef2f0b32`，当日链首笔）
- record：sih-tools/scribe/reports/2026-09-08-ask3-critsweep-solo-record.json
- validation：sih-tools/scribe/reports/2026-09-08-ask3-critsweep-solo-validation.json（双门第二门 ask3repeater 输出 status ok anchor_count 3）
- 三锚引文程序切片（07-on-assay.md L44 去蔽设计减偏方法、08-on-settle.md L110 应而不藏、07-on-assay.md L55 鉴只列事实）于 ask3 记录，生成器 make_ask3_critsweep-solo.py 随批落档，禁手打承契约。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0 零违规；引擎 ask3repeater exit 0 status ok anchor_count 3。
- 叩问：elicit check 出轻信号五件即 critsweep、判据扫、命名空间、回算器、沉底全未登记；ask3 契约内五条叩问处置（critsweep 与判据扫随工具本体批内登记检词收编，余三词本批不立名不登记）后 digest passed covered 5。
- 正身：identity verify anomalies 0 verdict attest（书简认证随批落）。
- 例行读数：gauge record 三维落链即 convergence 0.0（event_hash fc1e73ec）与 adoption 1.0（af2fbcb8）与 mergeback 0.03125（7699a880）。
- watch 对表：exit 0 净态无主修改零处（现势锁面 27 面；2026-09-07 在盘已知无主二件 anchor.py 权限位与 calls.ndjson 于本日对表读数已不在无主清单，如实转述不代清不触碰，候在途 orphanexec 线自处）。
- 泊界心跳：工具线 mainline 23 侧 1 废 0 告警零；引擎线 mainline 47 侧 2 废 9 告警一即 siding_surplus count 2 已知配置性误报（pk-070 与 pk-077 带 gate 远景件，系 parkgate 升级后旧阈未调，候收尾调优批，anchorskill 前置读数同款）。
- reconcile 批前基线：tools unrouted 0、engine unrouted 0（cert_missing 4/3 与 session_orphan 19/24 与 unbypassed 98/68 为在盘历史账面项，如实记档不代清）。

## 件读数：critsweep 五件与接线两件

| 件 | 落位 | 内容 |
|---|---|---|
| sweep.py | sih-tools/critsweep/sweep.py | 回算器本体：零 LLM 零网络纯标准库，退出码恒零，严格 JSON 单对象 stdout，根判据承 anchor.py 上溯形（--root 显式参优先），--at/--threshold/--window/--criterion 四参 |
| registry.json | sih-tools/critsweep/registry.json | 空腹五判据登记，判据文本自 GOV-002 §退出标准五行程序切片逐字节内嵌（构建器 make_registry.py 可重放），id 与 title 与 status_kind 与 evidence 与 token_scope 与 registered_at 与 derivation 全填，blocked_by 空缺省 |
| CONTRACT.md | sih-tools/critsweep/CONTRACT.md | 契约：机器形态、三面回算、散文对照面、登记、挂点、验证、边界七节 |
| tests/ | sih-tools/critsweep/tests/test_sweep.py | pytest 17 件：缺 trail 降级、缺账本降级、双跑一致、pk-044 对照、零批沉底 registered_at 计 gap、阈值边界（gap 3 在飞 gap 4 沉底 --threshold 覆调）、C4 令牌实报、证据达成与回落、C5 文本指纹、--criterion 重放、registry 形、命名空间单元扫描、零 LLM 静态扫描、泊界退出码二降级与 summary 缺席降级（r2 回归两件） |
| pyproject.toml | sih-tools/critsweep/pyproject.toml | workspace member 壳（uv run --project critsweep 解析成功 15 测绿） |
| 接线一 | AGENTS.md 会话启动节 | 概览行与启动节三件套改五件套即自检、回锚、判据扫、例行读数、泊界心跳，判据扫段载调用形与读数规则；归档件 AGENTS-archive-2026-09-08.md 落引擎工地材料（与主树改后件 cmp 逐字节一致） |
| 接线二 | sih-tools/BATCH-FACE.md | 增「判据扫挂点（critsweep-solo 批增）」节：挂点位置回锚之后例行读数之前（watch 对表挂点节先例同形）、verbatim 命令形双支、语义钉死三行 |
| 批材料 | sih-engine/sih/event/plan/critsweep-solo-materials/ | registry 派生文、first-run 双跑读数、pytest 绿证、AGENTS 归档件、make_registry 重放件 |

## 机器判据复算（工地条件读数，主树裸调见完工回显节）

- 双跑：worktrees 条件下同参同日 sweep.py 双跑 exit 0/0，输出 cmp 逐字节 IDENTICAL（materials/first-run-2026-09-08/run1.json 与 run2.json）。
- 五判据实态（--at 2026-09-08）：C1 leaseopt achieved（证据面文件在场加 pk-045 promoted 链事件在）；C2 viewline in_flight（last_hit 2026-09-05，gap 3 不大于阈值三）；C3 measure-poly sunk（批名命名空间零批记录，gap 4 自 registered_at 2026-09-04 起算）；C4 math-attribution 按令牌实报 sunk（last_hit 2026-09-03 mathclose-solo，gap 5）；C5 lease-line achieved（结算单在场加 CONTRACT 修订四十一至四十三指纹全中加 pk-072 promoted 链事件在）。
- pk-044 对照读数：散文对照面捕获 measure-poly 散文命中 2 件即 2026-09-05 pk-054 进泊与 2026-09-07 pk-044 出泊裁定，两件命名空间字段零命中不计活动——全文匹配会把裁定误计为程序活动的语义陷阱被机械排除，对照读数在档可复算。
- 测试：15 件全绿（首跑 2 红留痕后绿，红证 materials/pytest-red 面申报见越线节）；uv workspace member 形 `uv run --project critsweep python -m pytest` 15 件绿。

## 管线读数

- 引擎域四件（任务包镜像、结果档、registry 派生文、AGENTS 归档件）管线三步逐件 exit 0：化格 general-v1 四件零改（changes 空），核阅 des-001 四件零违规（state/plan 与 event/plan 目标现版域判读全放行，无域外二态触发，如实记档），检词 core 四件零违例。
- 工具域件（CONTRACT.md、BATCH-FACE.md、sweep.py、registry.json）核阅检词二步逐件 exit 0 零违规零违例；化格不越域如实申报（sih-tools 域件不受引擎格式规范约束，域别纪律）。
- checkcite（认证前必跑）：recall.py 书单出 18 条目，checkcite 对合并引用单件（CONTRACT+结果档+派生文）cited 空 missing 空 verdict pass exit 0，读数件 sih-tools/scribe/reports/2026-09-08-critsweep-solo-checkcite.json。
- 补笔复验：结算读数回填后结果档与 CONTRACT 修订二三步复跑，读数见收约补笔节。

## 认证清单

| 件 | 类型 | 链上哈希前八 |
|---|---|---|
| ask3 记录 r1 | 意图记录（intent 笔 ef2f0b32 后补认证） | 4a638fe9 |
| 管线报告 r1 | 三步读数 | 1aeb4952 |
| checkcite 读数件 | 书单对表 | d3d95f65 |
| 正身件 | 身份报告 | ed2843b1 |
| 内容哈希清单件 | md 与无仓控件 14 路径 sha256 统一绑定 | 780f7c46 |
| ask3 记录 r2 | 修复相意图记录（intent 笔 b345b896 后补认证） | 8affef44 |
| 管线报告 r2 | 修复相读数 | 8734d1dc |

- gauge 三维读数经 scribe record 直落链（fc1e73ec / af2fbcb8 / 7699a880），不另走认证 append。
- md 件与无仓控件走内容哈希清单件一件绑定认证形（anchorskill 先例；任务包镜像、结果档、派生文、AGENTS 归档件、CONTRACT、BATCH-FACE、主树 AGENTS.md、critsweep 源件与 pyproject 与 make_registry 重放件全在清单）。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 件落位与接线 | 实装 | critsweep 四件在位零 LLM 零网络静态扫描过，pyproject members 收编，主树裸调可跑；AGENTS.md 五件套与 BATCH-FACE 挂点节在档 | 通过（五件在位、零 LLM 静态扫描 pytest 绿、members 收编主树 `uv run --project critsweep` 解析绿、AGENTS.md 五件套与 BATCH-FACE 挂点节在档且随批入版控；主树裸调双跑 0/0 逐字节 IDENTICAL 见完工回显节） |
| F-2 判据实态复算对链 | 数据治理 | --at 2026-09-08 读数五判据三态对链可重放，pk-044 对照在档 | 通过（C1 achieved 与 C5 achieved 证据指针在档，C2 in_flight gap 3，C3 sunk 零批 gap 4 自 registered_at，C4 按令牌实报 sunk，逐判据 --criterion 重放形在 CONTRACT，pk-044 对照 2 件在档；主树与工地读数全同） |
| F-3 确定性纪律 | 跨族治理 | 同参同日双跑 cmp 逐字节 IDENTICAL、退出码全零；缺 trail 或账本对应面降级行可见不静默 | 通过（工地双跑与主树裸调双跑皆 0/0 IDENTICAL；缺 trail 与缺账本降级用例 pytest 绿；uv 嵌套退出码二降级回归用例绿） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（r1 写入面 12 路径与 r2 写入面 7 路径俱在请求写入节与 r2 意图域内；批后补笔经 --no-verify 加 lease bypass 显式登记通道不入本判据违约面） |
| F-5 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 较批前零新增 | 通过（读数见结算读数节；close 累计被拒三次皆自纠后成，如实申报） |

## 越线与误差申报

- 测试首跑 2 红（test_missing_trail_degrades_visibly 断言误写 criteria==[]、test_namespace_scan_unit 夹具 trail 目录位错置），红证在案即本节申报，修正后 15 绿；红非被测件缺陷而是测试件自身断言与夹具错，被测 sweep.py 零改动即红绿两态间零清洗。
- settle 首跑红（r1）：cert 传意图笔 ef2f0b32 被拒 cert_not_on_chain——cert 语义是 certification_completed 事件哈希前八位（commitcore._load_trail_hashes 只收认证笔），补 ask3 记录认证笔 4a638fe9 后成；红证 settle-first-red-2026-09-08.json 随 r1 引擎仓 seq 2 settle 入档。同跑暴露管道掩退出码违一支（uv|python3 读出假 EXIT=0），改即时捕获形，后段全程即时捕获。
- close r1 首跑被拒：红证文件系 settle 后补入未提交（工地卫生检查 untracked 一件），补锁重取后 settle seq 2（d1a82e1）再放锁；close r1 二跑被拒真分叉七件即主树批前温故检索件与裸调读数占位碰撞，走备份让位归并对表法四步（备份→让位→close→diff 全 IDENTICAL）后三跑成。
- 主树裸调红（r1 收约后）：uv workspace 形 pytest 1 红——parking_face 在 uv 嵌套环境把内层 selector 退出码二加 JSON 错误报文误计 routed（工地 plain python 形 15 绿未现，主树接线形现形，工地条件验证不算接线第四案）；红证 bare-run-2026-09-08/main-tree-uv-pytest-red.log 在档。源代码件不在直改车道白名单，同包重开完整租约仪式（r2）修 parking_face 退出码门（0 与 1 外一律降级加 summary 键在位断言）加回归用例两件，双形 17 绿，主树裸调重取双跑 0/0 IDENTICAL。r2 close 亦撞裸调读数未跟踪占位一处，备份让位归并对表 IDENTICAL 后成。
- 术语登记降格申报：r1 ask3 叩问处置承诺 critsweep 与判据扫随批登记 nomenclator core 包——收编面 terms.json 不在请求写入节与锁面内，批内零碰未越线；检词对全批件 exit 0 零违例即登记非绿性硬依赖，登记归后继检词收编批，承诺降格如实申报不代凑。
- close 闸 bypass-orphan 未动用：本日 watch 对表净态（r1 前置读数），两轮 close 皆未触发无主闸，先例通道备而未用。
- 工具本体试跑首日 in-flight 读数随批内时点浮动（active_sessions 与 held_locks 随在途会话变化），故金向量不含在飞计数，只含五判据三态与泊界两线（CONTRACT 验证节声明）。
- C4 判据四实态为沉底：令牌面 last_hit 2026-09-03 gap 5 超阈，此为按令牌诚实实报非判据达成判定；pk-053 冻结件清账进度裁决归人节点，工具只回算（任务包范畴排除条款）。
- watch 无主二件（anchor.py 权限位与 calls.ndjson）本日对表读数已不在无主清单，未代清未触碰，如实转述；anchor.py 本批零触碰（权限位无主变更候人节点裁，registry 派生不涉锚接）。
- 其余误差零申报。

## 结算读数

- 双仓 settle（r1）：tools 工地提交 67504d59（base integral-stage-build@187a6ad1，三查过）与归并 1f4e4466；engine 工地提交 43ccfcd（base main@68c66ca，三查过）与 seq 2 补笔 d1a82e1、closeguard pre-close 提交 c5c0852 与归并 2644696。
- 双仓 settle（r2）：tools 工地提交 27241fed、engine 工地提交 94ab23c（cert 8affef44，三查过）。
- 放锁收约：r1 12 锁、r2 7 锁 unlock 全过；close 两轮三跑成（r1 第三跑、r2 第二跑）即双仓归并、工地与分支清除、会话 b06c3c1e5b58feed 与 e4d9f6f05b092d28 俱 revoked、零失败；归并对表四处全 IDENTICAL（任务包主树备份 vs 归并、温故检索件、裸调 run1/run2、r2 红证）。
- 链 verify：valid，events 12，first ef2f0b32（r1 意图）last 8734d1dc（r2 管线认证）。
- reconcile 对基线：双仓 unrouted 0/0 较批前零新增；cert_missing（engine 3 与 tools 4）与 session_orphan（engine 24 与 tools 19）历史账面零新增，exit 1/1 与批前同源；unbypassed tools 98 零新增，engine 类计数 68→69（closeguard 预收提交一类计数 +1，非未决议题）而未决 unbypassed 议题清单空 0 笔，本批收约补笔两笔（tools 65494510 与 engine babbc8a）经 lease bypass 登记后议题清单保持空，如实记档。
- 收约补笔更正一笔：本节首写 engine 计数误作「68→69→70」，实数 68→69，更正件经同款 bypass 通道入版控，错数留痕于更正说明（应而不藏）。
- 温故检索：retriever recall --event critsweep-solo --since/until 2026-09-08 命中本批当日链笔（certification d3d95f65 等），落批材料 retriever-recall-2026-09-08.json；落包前主会零命中读数 critsweep-recall-pre-2026-09-08.json 随补笔入档。
- 收约补笔：结算读数与管线实录与认证实录与 F 表终态与完工回显即本笔，经 --no-verify 加 lease bypass 登记通道入版控（anchorskill 与 archpark 与 genpark 先例同形）。

## 完工回显

主树裸调首跑读数（修复后重取，工具自证）：`python3 sih-tools/critsweep/sweep.py --at 2026-09-08 --root /Users/moc/workspaces/SiHankor`

- 退出码 0，双跑 cmp 逐字节 IDENTICAL，degraded 假，降级零行。
- 五判据实态：GOV2-C1-leaseopt achieved（last_hit 2026-09-05 命中 39）；GOV2-C2-viewline in_flight（last_hit 2026-09-05，gap 3，命中 14）；GOV2-C3-measure-poly sunk（批名命名空间零批记录，gap 4 自 registered_at 2026-09-04，命中 0）；GOV2-C4-math-attribution sunk（last_hit 2026-09-03，gap 5，命中 23）；GOV2-C5-lease-line achieved（last_hit 2026-09-06，命中 7）。
- 泊界面：引擎线 mainline 47 侧 2 废 9 告警一（siding_surplus 已知配置性误报）；工具线 mainline 23 侧 1 废 0 告警零。
- 散文对照：measure-poly 散文命中 2 件（2026-09-05 pk-054 进泊与 2026-09-07 pk-044 出泊裁定）命名空间零命中不计活动——pk-044 对照判据机械成立。
- 在飞面随批内时点浮动不作金向量项（本笔读数 active_sessions 1、held_locks 0）。
- 完工回显即行为承载锚的静默失效补偿位执行：本批任务锚 sess-zcode-260908-critsweep（租约 b06c3c1e5b58feed 与 e4d9f6f05b092d28 两会话），判据扫已入启动节律五件套。
