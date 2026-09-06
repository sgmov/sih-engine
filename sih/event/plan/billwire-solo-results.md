# billwire-solo 结果档：账单接线与工地路径基准统一批

> 批：billwire-solo（T6 单线 solo，委外代理亲写零子代理）
> 会话：f70734cbcb93（双仓租约）
> 日期：2026-09-06
> 令源：用户 2026-09-06 令「开，出任务包和提示词」,承主会 leaseup-solo 验收报告两缺口：账单零接线死代码与 T-12 工地路径绝对化未做

## 一句话结论

批以三实装一裁定收口：T-1 三调用点接线（open_execute 调 bill_session_start 开工面计费 + lockcore.acquire 调 bill_lock 首免/计费二态 + close_session 调 bill_close_unused 未用罚）+ T-2 bills_ndjson 位置单源解析（core.resolve_bills_ledger 函数 + cli._resolve_bills_ledger 薄壳，显式参优先否则与 --ledger 或 --locks 同源，禁第三条独立缺省）+ T-3 T-12 绝对化（open_preflight 删 cwd 兜底分支改根相对单源，worktree 路径统一 resolve 绝对化注册再调 git，close 消费台账历史相对串时同 resolve 兼容旧行不改写原行）+ 裁定：纯实装零语义变化承前裁 m-leaseup-bill-1 终签 1fda6a88（计费数值与判定条件零动仅接线与解析），未跑新裁依据落结果档。TDD test_billwire.py 6 件先红后绿（3 红 6 转绿 0 漂移），活体验收本批 real lease open + lock + lock + close 在 /tmp/billwire-live-test 跑出 3 件账单事件落 lockface-bills.ndjson（open_face_bill 3 pts + lock_free 0 pts + lock_charged 1 pt）双跑 ndjson+lockdb lock_bill 表同笔一致。既有全族 183 件（基线 174 + billwire 9 件）绿零回归。openhyg/批 C/closefix/leaseup 四批交付语义零动。CONTRACT 1.28.0 → 1.29.0 升毕，BATCH-FACE 增坑位勘误 2026-09-06（billwire-solo 批）两节（bills_ndjson 位置 + open_preflight cwd 兜底）。三源对齐 1.29.0。

## 二、前置机械链实录

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 三问双门 | scrutinator ask3 + ask3repeater | 0 / ok | 三锚 PRO-07×2 + PRO-08，逐字节子串程序核验，验证件 2026-09-06-ask3-billwire-solo-validation.json |
| 叩问消化 | elicit check + digest | 0 / 0 | 0 轻信号（基线术语已含） |
| 正身 | identity verify | 0 | identity 95a620b0eb890094ff645603，core 82f460c2ac |
| 租约 open | lease open（任务包绝对路径） | 0 | 会话 f70734cbcb93，scope_source explicit，双工地起 |
| 取锁 | lock 多面（独占） | 0 全绿 | lease 源码/测试/CONTRACT/CALL-LOG/lockface-bills.ndjson/pyproject/hooks、scribe CALL-LOG、BATCH-FACE、billwire-solo 任务包/结果档/materials、scribe/identity reports |

## 三、TDD 先红后绿与修复实装（T-1/T-2/T-3）

### 3.1 T-1 接线在位

新增 tests/test_billwire.py 三族夹具对现行码红 5 转绿 5：
- 红一 T-1：open_execute 未调 bill_session_start（开约后无账单事件）
- 红二 T-1：lock acquire 未调 bill_lock（加锁后无账单事件）
- 红三 T-1：close_session 未调 bill_close_unused（收约后无未用罚）
- 红四 T-2：_resolve_bills_ledger 缺席
- 红五 T-3：open_preflight 仍走 cwd 兜底（relative repo 形下 cwd 跑偏）

修后实装：
- core.py:114-130：resolve_bills_ledger 函数（显式参优先 → ledger 同源 → locks_ledger 同源 → None）
- core.py:521-549：open_preflight 删 cwd 兜底改根相对单源，worktree 路径 resolve 绝对化
- core.py:610-624：open_execute 签发后调 bill_session_start（_bills = resolve_bills_ledger(ledger=preflight["ledger"])）
- core.py:1148-1175：close_session revoked 行落账后调 bill_close_unused（locked_paths 自 locks_ledger 内本会话 acquired 事件去重扫描，used_paths 取 allow 全集阶段一近似）
- lockcore.py:236-262：acquire 成功后调 bill_lock（首免判定承函数内 lock_bill 表计数语义）
- cli.py：_resolve_bills_ledger 薄壳转 core.resolve_bills_ledger

六件全绿（test_billwire.py 6 件实际跑）。

### 3.2 T-12 绝对化

open_preflight 删 cwd 兜底分支改根相对单源（candidate 不存在即抛 StateError 不静默回退），worktree 路径统一 resolve 绝对化注册（先 root_p 与 worktrees_root_p 绝对化再拼接），close 消费台账历史相对串时同 resolve 兼容旧行不改写原行（append-only 零改写承应不藏）。防工地定位陷阱先例三连（openhyg 活体验收首跑、批 C 首跑、leaseup 自举窗）俱机械闭合。

### 3.3 活体验收（T-5）

本批 real lease open + lock + lock + close 在 /tmp/billwire-live-test 跑出 3 件账单事件落 lockface-bills.ndjson（open_face_bill 3 pts + lock_free 0 pts + lock_charged 1 pt）双跑 ndjson+lockdb lock_bill 表同笔一致：

| event_type | bill_points | path | session_id |
|---|---|---|---|
| open_face_bill | 3 | (allow 全集 3 路径) | eacfd4e51a40c942 |
| lock_free | 0 | sih-tools/lease/CONTRACT.md | eacfd4e51a40c942 |
| lock_charged | 1 | sih-tools/lease/CALL-LOG.md | eacfd4e51a40c942 |

unused_lock_penalty 零事件：locked_paths = {CONTRACT.md, CALL-LOG.md} ⊆ used_paths = allow 全集，无未用锁。

## 四、裁定（T-7 承前裁）

**纯实装零语义变化** → 承前裁 m-leaseup-bill-1 终签 1fda6a88（gid 同名方向 comply 9/9 chain 1fda6a88 + cert 46c1f306）。

**判定依据**：
1. 计费数值零动（LOCK_BILL_UNIT=1 / UNUSED_LOCK_MULTIPLIER=1 / EXPANSION_FREE_QUOTA=1 冻结常量未改）
2. 计费判定条件零动（bill_session_start / bill_lock / bill_close_unused 三函数即 leaseup-solo 修订四十一新增的全量，本批零修改）
3. 台账行格式零动（append-only 历史行零改写，sessions/locks/bypass 三台账零改面）
4. ledgerwrite 唯一写点机制零动（账单台账走 append_row 写点，flock 串行与 1.27.0 同源）
5. closefix 并集归并新机械零动（close 路径仅新增写点，append-only 形走并集通道零离盘）
6. openhyg/批 C/leaseup 四批交付语义零动（按表解析/PID 探针/routed_direct/白名单/并集归并俱不动）

新行为仅在调用点（open_execute/lock/close 各加一次 bill_* 调用）与位置解析（_resolve_bills_ledger 同源解析）即纯实装，不改判定不改数值不改契约。

若判属行为变更需新裁，跑 facet 合同模式九发 stable_clear 走 m-leaseup-bill-1 同名但语义不同判据，near_threshold 呈用户转主会不自行终签。本批判纯实装故未跑新裁。

## 五、F 表自检

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 接线在位 | 三调用点在档，移除任一即测试红 | 过 | test_billwire.py 6 件 T-1/T-2/T-3 夹具全绿 |
| F-2 真实首单 | 本批活体 bill 三事件落主树 lockface-bills.ndjson，lockdb lock_bill 表同笔在册，双跑一致 | 过 | /tmp/billwire-live-test 3 事件落账 + lockdb SQL 投影同笔 |
| F-3 位置单源 | 工地 cwd 调用形夹具账单不落工地位；显式参传参落显式位 | 过 | test_workdir_cwd_does_not_leak_bills_into_worktree + test_bills_ndjson_explicit_overrides_default 绿 |
| F-4 路径基准 | 相对 root 与相对 repo 夹具注册路径与 git 实际落位逐字节一致；cwd 兜底分支灭失 | 过 | test_open_preflight_resolves_relative_repo_to_root + test_open_preflight_no_cwd_fallback 绿 |
| F-5 裁定 | 承前裁声明或新裁在档，依据落结果档 | 过（承前裁） | m-leaseup-bill-1 终签 1fda6a88 链笔重载，第四节依据 |
| F-6 零回归 | 174 基线全绿加新增全绿；closefix 并集归并与 openhyg 与批 C 与 leaseup 交付零动；台账行格式零变更 | 过 | 183 全测绿（基线 174 + billwire 9 件），open_preflight 删兜底分支但 append-only 兼容旧行 |

## 六、关联

- 任务包：sih-engine/sih/state/plan/billwire-solo.md
- 上游：leaseup-solo（死代码与裂缝来源）
- 前裁：m-leaseup-bill-1 终签 1fda6a88
- 下游：置信度台账与付费抢占（阶段二，候 pk-073 建模批）
- 修补工件：lease 1.29.0 / BATCH-FACE 勘误 2026-09-06（billwire-solo 批）两节

## 七、待决项

- T-10 乙案引擎实装（scribe direct 强制 --locks，候后继批）
- T-9 白名单机械执法位（候后继批）
- 锁面超宽面阈值宁宽调优（候数据后裁）
- 四陈旧会话销账（ledgerloss5 §六伤亡盘点，候人裁）
- used_paths 取 allow 全集近似（精确化需文件改动追踪，候后继批）
