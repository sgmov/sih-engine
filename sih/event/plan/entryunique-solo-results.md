# entryunique-solo 结果档

> 批：停泊门号源唯一小批
> 会话：段1 9cfb1107223bb5a7 + 段2 284219e7928accdf（sess-zcode-260903-entryunique）
> 日期：2026-09-03
> 队形：单线形 solo，零子代理，全链亲写

## F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 红绿 | 测试链目录已出泊号复用，修前 appended 退出码零，修后号源唯一拒退出码一，两态在档 | 过 | 红态 red-exited-reuse.json 即修前复用已出泊号被接受 appended 退出码零；绿态 green-exited-reuse.json 即修后号源唯一拒 EntryIdUsedRejected 退出码一；两态材料在档 |
| F-2 既有门回归 | 重入拒与无主出拒与跨天出泊既有单测全绿，cargo test 除先在三金向量漂移外全绿 | 过 | park scope_tests 13 过（新号过、在泊重入拒回归、已出泊号复用拒、跨天已用号拒加既有九件）；tdd 7 过（t4_park_invariants 断言同步为出泊后号源唯一拒）；lib 134 过 6 忽略；环境敏感两败为既有互斥依赖 PYTHONHOME 非本批引入，见环境申报节 |
| F-3 语义窄改 | 五子命令输出与退出码旧新对表零变化，verify 单链零回溯 | 过 | 旧新 verify 与 query 对表 IDENTICAL；diff 面即 SPEC-006 与 park.rs 与 tdd_tests.rs 三件；追加面仍单链即当日链自身链序，跨天出泊即泊入在先日链 |
| F-4 写入仅 allow | 本批写入仅请求写入节所列 | 过 | 段1 与段2 全部写入落请求写入节；收段重开补列两件即 tdd_tests 与 inputlog 入请求写入节后段2 承接 |

## 收段重开申报

任务包请求写入段漏列两件即 src/event_stream/tdd_tests.rs 与 sih/event/inputlog/2026-09-03.ndjson，段1 settle 范围验拒收即 staged_out_of_scope，处置按 contract01-solo 收段重开先例：两件抢救暂存（补丁 /tmp/entryunique-solo-seg2.patch）、任务包补列（c143934）、本会话收段（9cfb1107223bb5a7 归并 366e3ee）、重开会话携完整范围（284219e7928accdf）、段2 settle 承接，全程留痕。

## 环境敏感测试申报

mem_recall_f_suite 六败为 locator 经 uv run 调用受 PYTHONHOME 泄漏影响（主仓同败），ask3repeater 一败为 unset PYTHONHOME 后 Python 工具缺依赖，两败互斥依赖环境变量、均非本批引入；park 与 tdd 本批相关测试在两种环境下均全绿。

## 管线读数

笔在核前判在书简前，序固定化格、核阅、检词：

- SPEC-006 修订二：化格 exit 0 零改、核阅 des-001 exit 0 零违规（C006 全角括号一处改写后复检零发现）、检词 exit 0 零违例
- 结果档：化格 exit 0、核阅 des-001 域外 exit-2 如实记（sih/event/plan 不在 des-001 域）、检词 exit 0

## 并行链线性化申报

用户批准并行开工即 mathpipe-a2-solo 与 entryunique-solo 同日在途，当日链 trail/2026-09-03.ndjson 出现双叉：主树侧 mathpipe-a2 三件（sess-mathpipea2、mathpipe-a2-pipeline、mathpipe-a2-golden-vector）与工地侧段2四件认证同以 a5152850 为父。链必须线性，处置即段2四件认证重挂于 mathpipe-a2 三件之后（重放面即主树 49 事件链加 scribe append 四件），得 53 事件单链 verify valid。mathpipe-a2 三件哈希不动，段2四件认证哈希因 prev 变更而更新（见收口读数）。

## 收口读数

- 链 verify：valid 53 事件（段1 46 加 mathpipe-a2 三件加段2 四件认证）
- reconcile 双仓：零新增
- 认证：段1 六件（intent 加 fmt 加 scr 加 nom 加 redgreen 加 tests）入链，段2 四件（results-fmt 5e36283a 加 results-scr f0ef6297 加 results-nom d0295490 加 results-tests e6880a63）入链
- 主树重编 target/debug/scribe 并烟测

## 验收

- [x] F-1 至 F-4 全过
- [x] cargo test 既有套件回归全绿（环境敏感两败披露在案）
- [x] SPEC-006 修订二三步过
- [x] 认证入链，双仓段结算收约，对表读数在档
