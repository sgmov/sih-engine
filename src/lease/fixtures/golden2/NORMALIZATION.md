# NORMALIZATION.md — leg2 domain2 金向量捕获非确定性字段与归一化建议

域：`work/leg2-fixtures/domain2`（fixture 域，双 git 仓 sih-engine + sih-tools，canonical 台账位 `sih/ledger/`，链位 `sih-engine/sih/event/trail/2026-09-13.ndjson`）。
运行形式：`cd sih-tools/lease && uv run --project . lease <子命令> … --root <domain2> --ledger/--locks/--bills/--claims 全显式`；链笔走 `$ROOT/sih-engine/target/debug/scribe intent/append`。
捕获时间：2026-09-13；lease 工具版本 1.46.0；identity/guard 版本见各回执。

## 会话号（可复现，非随机场）

- `SID=b516664e0251a887`（goldc）、`SID2=2af46b28cdc6c2f4`（goldd）。
- 派生式 `make_session_id = sha256(f"{package}|{issued_at}|{identity.file_sha256}|{'|'.join(repos)}")[:16]`，对 `--at` 与正身件字节哈希确定。Rust 移植测试若冻结 `--at` 与 identity 件字节，可复算同号；identity 件字节变则号变。

## 逐场景非确定性字段与占位符建议

通用（全部场景）：
- 绝对路径 `/Users/moc/workspaces/SiHankor/work/leg2-fixtures/domain2/…` → 占位 `<D2>`（域根）与 `<D2>/sih-engine`、`<D2>/worktrees/sih-engine/goldc` 等。回执中 `package_path`、`repos[].repo/worktree`、`commit --repo`、`gauge.missing[]`、`ledger/bypass.ndjson` 路径俱属此。
- identity 件 `file_sha256` / `identity_hash` / `core_hash`（c01 回执 identity 块、会话号派生输入）→ 占位 `<IDENT_SHA>`；fixture identity 件字节定则值定。

按场景：

| 场景 | 非确定性字段 | 归一化建议 |
|---|---|---|
| c01/c24 open | `issued_at`（本文取 `--at` 原值全 ISO 形）、`gauge.missing[]` 绝对路径、stem_check teaching 中词典包路径 | `issued_at` 冻结 `<AT>`；路径 `<D2>` 化 |
| c02/c16 lock/unlock | `acquired_at`/`released_at`（`--at` 冻结值） | 冻结 `<AT>` |
| c03 identity drift | `detail.observed`（hostname 等自报值，fixture 定值） | 定值可比对 |
| c04/c13/c25/c26 scribe | `timestamp`（Utc::now 实钟）、`event_id`（uuid4）、`event_hash`、`prev_hash`（链式依赖） | `<TS>`、`<UUID>`、`<HASH>`；断言面只取 event_type 与 session_id 与 details 关键词 |
| c05 wip 成功 | `commit` 短号（git 实钟提交）、message 内 session 号 | 短号 `<SHA>`；message 模板逐字节断言（`{stem} wip {subject}\n\nsession: {SID}\n\n{note}\n`） |
| c06 nothing_staged | 无时变字段（纯拒绝形） | 全形可逐字节冻结 |
| c07/c08 settle 用法拒（exit 2） | 无 | 全形冻结 |
| c09 cert_not_on_chain | `detail.trails[]` 绝对路径 | `<D2>` 化 |
| c10/c11/c12 四验拒绝 | `detail.repo/repos/worktrees` 绝对路径 | `<D2>` 化 |
| c14 settle 成功 | `commit` 短号、`base`（`master@<merge-base短号>`）、cmd 中 `--cert` 前八位（链哈希派生） | `<SHA>`、`<CERT8>`；`checks` 数组与 message 模板逐字节冻结 |
| c15 staged_out_of_scope | `detail.allow[]`（open 时已定的 allow 面，定值） | 定值可比对 |
| c17–c22 SDDG 拒绝 | `t0_intent`/`t1_first_impl`（scribe/git 实钟）、`zero_pointer_hits[].repo` 绝对路径、`uncarried_entries[].doc`（定值） | 时戳 `<TS>`/`<TS1>`；repo `<D2>/sih-engine`；gate 判词结构（verdict/reason/清单）逐字段冻结 |
| c23 SDDG-4 bypass 成功收约 | `revoked_at`（`--at` 冻结）、`removed[]` 路径、`bypass.ndjson` 行内 `at`（实钟） | 时戳 `<AT>`/`<TS>`；bypass 行 `event/reason/gates` 逐字节冻结 |
| c27 无主闸拒绝 | `unowned[].mtime`（实钟）、`unowned[].path`（定值 `sih-engine/README.md`） | mtime `<MTIME>`；清单结构与三通道全文逐字节冻结 |
| c28/c29 差集闸拒绝 | 无时变字段 | 全形冻结 |
| c30 认领放行成功收约 | `revoked_at`、`removed[]` 路径、`acks[].reason`（定值） | 时戳占位，结构冻结 |
| c31 bypass 登记行 | 行内 `at`（`--at` 冻结）、`repo` 路径、`sha`（提交派生） | `<AT>`、`<D2>`、`<SHA>` |
| c32 status 视图 | `header` 路径、历史事件内全部时戳与会话号 | 结构冻结，时戳占位 |

## 捕获期行为发现（如实记录，Rust 移植对表用）

1. **F1 · date-only `--at` 令 close 全路径崩**：`--at 2026-09-13`（leg-1 惯用短形）使 `issued_at` 原样入账为 `2026-09-13`；close 在 SDDG 之后 `calllog_treadmill_applicable`/`calllog_guard_applicable` 做 `fromisoformat(issued) > fromisoformat(冻结常量)` 比较时 naive 对 aware 抛 `TypeError`（未捕获，traceback 退出）。证据件：`findings/finding-dateonly-at-treadmill-crash/`。本次全序列改用全 ISO 形 `--at 2026-09-13T00:00:00+00:00`。Rust 移植建议：`--at` 归一化为全 ISO 或判定比较 naivize 归一。
2. **F2 · `--at` 原值入账**：open 对 `--at` 不归一，`issued_at` 即旗标原字符串（leg-1 金向量 `issued_at=2026-09-13T00:00:00+00:00` 与本捕一致，皆全 ISO 形所致）。移植时冻结同一形。
3. **F3 · close 期 bypass 台账硬锚 `tool_dir()`**：`--bypass-sddgate/--bypass-orphan/--bypass-calllog` 的留痕行写死 `tool_dir()/ledger/bypass.ndjson`（真工具台账位），不受 `--root` 锚定。为不碰真实台账，c23 经域内 shadow 副本（`<D2>/leasetool/src/lease`，`PYTHONPATH` 前置 + 真仓 `.venv` 依赖）运行，bypass 行落 `<D2>/leasetool/ledger/bypass.ndjson`。Rust 移植建议：bypass 台账改 root 锚定或显式 `--bypass-ledger` 透传。`lease bypass` 子命令本身支持 `--bypass-ledger`（c31 直跑真 CLI）。
4. **F4 · 未跟踪目录形态触发真分叉误判**：主树 `?? sih/event/`（目录折叠形，目录内仅链文件）会使分支独有新件 `sih/event/plan/x.md` 命中 `_hits_untracked` 目录前缀匹配，close 报「存在真分叉冲突（非纯追加形）」整批拒。首捕 c19 踩中后以「结果档迁根位 + 链文件快照入版控」绕开，未留独立捕获件（报文全文：`收约被阻：存在真分叉冲突（非纯追加形），整批拒零动作。{"repo": "<D2>/sih-engine", "diverged_files": ["sih/event/plan/goldc-results.md"]}`）。
5. **F5 · SDDG-3 零偏差词形位置敏感**：`零偏差` 在「## 偏差」节内仍按条目逐条查承载词形（条目无 pk/DEC 词形即拒）；散见形（无偏差节 + 文中含零偏差）才走 declared_zero 放行。c19 用散见形。
6. **F6 · 账单台账有效位**：`--bills` 旗标在 open/lock/close 内部经 `resolve_bills_ledger(ledger=…)` 解析为 `<ledger 目录>/lockface-bills.ndjson`，旗标值不直用；本捕有效账单位 `<D2>/sih/ledger/lockface-bills.ndjson`。
7. **F7 · stem 查册闸 skip 形**：domain2 预置 `sih-tools/` 目录后走 first_domain 分支，nomenclator 词典包缺席 → `checked:false, disposition:pack_absent_skip`（c01/c24 回执显形）。
8. **F8 · scribe 意图记录复用闸**：同 record 路径二次 `scribe intent` 拒（`IntentRecordUsedRejected`）；goldc 重跑换 `intent-goldc-p2.json` 路径绕开。意图件在 close 场景间按需增删「规格消费面申报」句以切换 SDDG-1 申报通道（c18/c23 加、c22 删），链上 `record_hash` 保持首笔值——回执如实呈现此差异。
9. **隔离手段**：watchcheck 的 `dirty_set` 只扫 `<root>/sih-tools` 与 `<root>/sih-engine` 两仓，故 domain2 的 `sih-tools` 亦 git init 并入初提交，保无主闸判定域洁净；`sih-tools/lease/ledger/`、`sih-engine/sih/event/trail/`、`sih-engine/sih/state/plan/` 均为 watchcheck 豁免面。

## 真实面零触碰申明

真台账 `sih-tools/lease/ledger/`、真链 `sih-engine/sih/event/trail/`、`sih-tools/lease` 与 `sih-engine` 源码零改动；已验证真台账零 `leg2-fixtures` 字样。c23 之外全部场景直跑真 CLI（显式四台账参）；c23 经 shadow 副本（见 F3）。工作区并行主会话对真台账的正常写入与本捕无关。
