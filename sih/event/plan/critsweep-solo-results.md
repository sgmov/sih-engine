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
| tests/ | sih-tools/critsweep/tests/test_sweep.py | pytest 15 件：缺 trail 降级、缺账本降级、双跑一致、pk-044 对照、零批沉底 registered_at 计 gap、阈值边界（gap 3 在飞 gap 4 沉底 --threshold 覆调）、C4 令牌实报、证据达成与回落、C5 文本指纹、--criterion 重放、registry 形、命名空间单元扫描、零 LLM 静态扫描 |
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

见管线读数节（认证前填写）。

## 认证清单

见认证实录节（认证后填写）。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 件落位与接线 | 实装 | critsweep 四件在位零 LLM 零网络静态扫描过，pyproject members 收编，主树裸调可跑；AGENTS.md 五件套与 BATCH-FACE 挂点节在档 | 工地面全过（五件在位、静态扫描绿、members 收编 uv 解析绿、接线两件在档）；主树裸调读数见完工回显节（回填） |
| F-2 判据实态复算对链 | 数据治理 | --at 2026-09-08 读数五判据三态对链可重放，pk-044 对照在档 | 通过（C1 achieved 与 C5 achieved 证据指针在档，C2 in_flight gap 3，C3 sunk 零批 gap 4 自 registered_at，C4 按令牌实报 sunk，逐判据 --criterion 重放形在 CONTRACT，pk-044 对照 2 件在档） |
| F-3 确定性纪律 | 跨族治理 | 同参同日双跑 cmp 逐字节 IDENTICAL、退出码全零；缺 trail 或账本对应面降级行可见不静默 | 通过（工地双跑 0/0 IDENTICAL，缺 trail 与缺账本降级用例 pytest 绿，主树裸调双跑见完工回显节（回填）） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（写入面即 critsweep 五件与 pyproject members 与 BATCH-FACE 挂点节与 AGENTS.md 原地改与任务包镜像与结果档与批材料与当日链与两报告目录与双工地，全部在请求写入节 12 路径内） |
| F-5 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 较批前零新增 | 见结算读数节（回填） |

## 越线与误差申报

- 测试首跑 2 红（test_missing_trail_degrades_visibly 断言误写 criteria==[]、test_namespace_scan_unit 夹具 trail 目录位错置），红证在案即本节申报，修正后 15 绿；红非被测件缺陷而是测试件自身断言与夹具错，被测 sweep.py 零改动即红绿两态间零清洗。
- 工具本体试跑首日 in-flight 读数随批内时点浮动（active_sessions 与 held_locks 随在途会话变化），故金向量不含在飞计数，只含五判据三态与泊界两线（CONTRACT 验证节声明）。
- C4 判据四实态为沉底：令牌面 last_hit 2026-09-03 gap 5 超阈，此为按令牌诚实实报非判据达成判定；pk-053 冻结件清账进度裁决归人节点，工具只回算（任务包范畴排除条款）。
- watch 无主二件（anchor.py 权限位与 calls.ndjson）本日对表读数已不在无主清单，未代清未触碰，如实转述；anchor.py 本批零触碰（权限位无主变更候人节点裁，registry 派生不涉锚接）。
- 其余误差零申报。

## 结算读数

（收约后回填：双仓 settle 提交号、放锁 close 读数、reconcile、verify、收约补笔）

## 完工回显

（主树裸调首跑读数回填位：python3 sih-tools/critsweep/sweep.py --at 2026-09-08 --root /Users/moc/workspaces/SiHankor）
