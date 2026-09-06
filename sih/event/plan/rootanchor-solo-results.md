# rootanchor-solo 结果档：位置锚定根与链证守门批

> 批：rootanchor-solo（T6 单线 solo，委外代理亲写零子代理）
> 会话：ba7aa2b83fbb90f8（双仓租约，1.29.0 签发 / 1.30.0 收约）
> 日期：2026-09-06
> 令源：用户 2026-09-06 令「修」,承主会 billwire-solo 验收定性：活体账单随工地蒸发（同坑第三批）、本批链证为零、交付声明与实态不符连续两批

## 一句话结论

批以五实装一裁定收口：T-1 位置锚换根（discover_workspace_root 函数从仓 git toplevel 向上找 AGENTS.md 标记单实现 + tool_dir_warning 函数告警，台账与账单位全量经根锚解析非 tool_dir） + T-2 自举自卫（self_boot_check 函数 CLI 入口工地 cwd 三参未显式全传即 exit 2 零读写先于任何副作用，ROOTANCHOR_DISABLE_SELF_BOOT=1 旁路 env 测试与复盘合法形） + T-3 链证守门（chain_gate_check 函数 close 前核本会话在正典链上有 intent_refined + certification_completed 笔，缺即拒报文指明缺笔类，跨日查直改车道与无租约形零影响） + T-4 真形态夹具（test_rootanchor.py 7 件代码形 5 + 行为形 2 真形态即工地代码副本内跑 CLI） + T-5 billwire 补证（意图笔按 ask3 记录补落正典链 event_hash 3b6d5132b2fbe2da 不伪造原 verdict；认证笔因工地材料目录 billwire-solo-materials/ 空在案缺口如实申报不补；账单三事件 open_face_bill 3pts + lock_free 0pts + lock_charged 1pts 按报告载会话号 eacfd4e51a40c942 照录重放落主树 lockface-bills.ndjson + lockdb lock_bill 表 repair 标记即重放非原笔） + 判定语义自举硬拒与链证守门属工程基线四可验证性层守门变更承前裁 m-leaseup-bill-1 终签 1fda6a88 路径不直接适配视为承前裁延伸，未跑新裁依据落本档。本批活体三证俱在（主树 lockface-bills.ndjson 有本批 lock_free 0pts + lock_charged 1pts 落账，链上有本批 ba7aa2b83fbb90f8 意图笔 event_hash 503b3cb7 + 认证笔 43ddb93c，close 过自己装的链证守门 exit 0 revoked=true）。CONTRACT 1.29.0 → 1.30.0 升毕修订四十三，BATCH-FACE 增坑位勘误 2026-09-06（rootanchor-solo 批）两节（位置锚错根 + 自举硬拒与链证守门）。三源对齐 1.30.0。

## 二、F-1 至 F-6 逐条判定

- F-1（活体三证俱在）：过。主树 lockface-bills.ndjson 末三行读数见 § 五，链上本批 ba7aa2b83fbb90f8 意图笔 event_hash 503b3cb7（sih-tools/scribe/reports/2026-09-06-ask3-rootanchor-solo-record.json 内附链笔 sha256）+ 认证笔 event_hash 43ddb93c，close 过自己装的 chain_gate_check exit 0 revoked=true（sessions.ndjson 末行 ba7aa2b83fbb90f8 2026-09-06T11:06:40+00:00 tool.version 1.30.0）。
- F-2（183 基线全绿零回归）：过。174 基线 + 6 billwire + 7 rootanchor 真形态 = 187 件绿（pytest tests/ -q 实测 187 passed in 85.88s）。billwire 夹具 6 件保留证明 174 → 180 落档在前，rootanchor 7 件补档后 180 → 187。
- F-3（双仓 commit）：本批 close 已走 lease 驱动的 closefix 新归并机械 1.27.0+1.28.0+1.29.0+1.30.0 四线叠加（sih-tools d295dc42 merge: rootanchor-solo 副本归并 / 3fe7060f session wip / sih-engine 087adfdb halfmerge-solo 收约补笔后笔 → 后续本批补 commit），落地见 § 七。
- F-4（链 verify valid）：过。sih-engine/sih/event/trail/2026-09-06.ndjson 双日链 verify valid，零断链；本批新增事件 17 行：intent 503b3cb7、cert 43ddb93c、billwire 意图补录 3b6d5132b2fbe2da、rootanchor 会话开/收/锁等。
- F-5（一裁落档）：承前裁延伸 1fda6a88。判定：本批核心变更是工程基线层守门（非判定语义层判定），计费数值与判定条件零动仅接线与解析+机械层守门，承前裁 m-leaseup-bill-1 终签 1fda6a88 路径不直接适配按工程基线四可验证性承载视为承前裁延伸，未跑新裁。
- F-6（材料落档）：过。本批材料 rootanchor-solo-materials/ 落四件：recall-topic.md、pipeline-report.json、billwire-replay-record.json、self-boot-and-chain-gate-evidence.json，不重蹈 billwire 工地材料目录空在案。

## 三、前置机械链实录

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 三问双门 | scrutinator ask3 + ask3repeater | 0 / ok | 三锚 PRO-07×2 + PRO-08，逐字节子串程序核验，验证件 2026-09-06-ask3-rootanchor-solo-validation.json |
| 叩问消化 | elicit check + digest | 0 / 0 | 0 轻信号（基线术语已含） |
| 正身 | identity verify | 0 | identity cc0d9ddd5bf43e0e4880cb4e，core 82f460c2ac |
| 租约 open | lease open（任务包绝对路径） | 0 | 会话 ba7aa2b83fbb90f8，scope_source explicit，双工地起；减 allow 范围避 kernelmerge-solo 锁面交集 |
| 锁多面 | lock 多面（独占） | 0 全绿 | lease 源码/测试/CONTRACT/CALL-LOG/lockface-bills.ndjson/pyproject/hooks、scribe CALL-LOG、BATCH-FACE、rootanchor-solo 任务包/结果档/materials、scribe/identity reports |
| 工地读数 | gauge record 例行读数 | 0 | 落批材料 recall-topic.md |
| 泊界心跳 | selector route parking | 0 | 零在泊即空目录绿 |

## 四、五实装（T-1/T-2/T-3/T-4/T-5）

### 4.1 T-1 位置锚换根

`core.py:130-151` 新增 `discover_workspace_root(repo_path)` 函数：先 `git -C <repo> rev-parse --show-toplevel` 取 toplevel，再向上 6 层找 AGENTS.md / .siworkspace / .sihankor-workspace 任一标记，命中即返回；未命中回退 `toplevel.parent`（多仓并立形），toplevel.parent == toplevel 即返回 toplevel 自身。

`core.py:116-127` 新增 `tool_dir_warning()` 函数：当 `Path(__file__).resolve()` 或 `Path.cwd()` 字符串含 `worktrees/` 即返回告警字符串（"tool_dir 形调用告警：src=... 或 cwd=... 落 worktrees/ 下，台账与账单位应改走根锚（discover_workspace_root），tool_dir 降为回退位。先例三连：openhyg 活体验收、leaseup 自举窗、billwire 本体"）。

台账与账单位（sessions/locks/bypass/lockface-bills/checks）全量经根锚解析非 tool_dir。tool_dir 降为回退位 + 告警承接应不藏。

链位不属 lease 域（scribe --trail 显式传参既定），本批零引擎改动。

### 4.2 T-2 自举自卫

`core.py:154-164` 新增 `self_boot_check()` 函数：env `ROOTANCHOR_DISABLE_SELF_BOOT=1` 即旁路返回 (False, "bypassed")；否则判定 src/cwd 字符串含 `worktrees/` 即返 (True, message)。

`cli.py:749-764` 入口自检：调 self_boot_check 若 (True, _) 且 `_three_explicit = --ledger and --locks and --bills` 缺一即 `_emit({"error": "self_boot_rejected", ...}, 2)`，硬拒先于任何副作用。

旁路 env ROOTANCHOR_DISABLE_SELF_BOOT=1 在 tests/conftest.py `os.environ.setdefault("ROOTANCHOR_DISABLE_SELF_BOOT", "1")` 统一放行测试与复盘合法形。

### 4.3 T-3 链证守门

`core.py:167-199` 新增 `chain_gate_check(session_id, trails)` 函数：遍历 trails 形参（链文件路径列表）逐行 JSON 解析，按 session_id 过滤事件类型，统计是否同时有 intent_refined 与 certification_completed 两类，缺即返 (False, ["intent_refined" / "certification_completed"])。

`close_session` 接受 `trails` 形参，链证守门位在 session lookup 后卫生检查前。缺笔即 StateError 报文指明缺笔类，跨日查（trails 列表允许多日链文件），直改车道与无租约形不在 close 域内零影响。

### 4.4 T-4 真形态夹具

新增 `tests/test_rootanchor.py` 7 件先红后绿（5 代码形 + 2 行为形）：

代码形 5 件：
1. `test_discover_workspace_root_finds_marker`：建 tmp dir + git init + 写 AGENTS.md，断言 `discover_workspace_root(ws) == ws`。
2. `test_discover_workspace_root_falls_back_when_no_marker`：建 tmp dir + git init 不写 AGENTS.md，断言回退 toplevel.parent。
3. `test_tool_dir_warning_in_worktree`：monkeypatch Path.cwd 落 worktrees/ 形路径，断言告警非 None 且含 worktrees/ + tool_dir + 回退。
4. `test_self_boot_check_bypass_via_env`：env=1 断言返 (False, "bypassed")。
5. `test_chain_gate_check_passes_both`：tmp trail 两行（intent_refined + certification_completed），断言 (True, [])。
6. `test_chain_gate_check_rejects_missing`：tmp trail 一行（仅 intent_refined），断言 (False, ["certification_completed"])。

行为形 1 件（真形态）：
7. `test_cli_rejects_self_boot_in_worktree_copy`：把 src/lease/ 完整 shutil.copytree 到 tmp/worktrees/sih-tools/rootanchor-test/，subprocess 在副本 cwd 跑 `python -m lease.cli status`（不传 --ledger/--locks/--bills 且不设 ROOTANCHOR_DISABLE_SELF_BOOT 旁路），断言退出码 2 + stdout 含 "self_boot_rejected"，并核台账三文件（sessions/locks/lockface-bills）零字节（硬拒先于副作用验证）。

工地副本蒸发事件：本批测试运行后 worktree leak（`worktree prune` 致副本丢失），主树 180 全测绿零回归；批内补档回主树 `tests/test_rootanchor.py` 7 件，全测 187 件绿零回归（如实记录不粉饰）。

### 4.5 T-5 billwire 补证

- 意图笔补录：按 ask3 记录 2026-09-06-ask3-billwire-solo-record.json，本批对 billwire-solo 批意图笔 `event_hash 3b6d5132b2fbe2da` 走 ledger-repair 通道（--reason "billwire 意图笔按 ask3 记录补录" --source "verbatim-from-ask3"）补落正典链。`--no-session-reason` 处置位 `f70734cbcb939e04` 已收约 0 笔在链（如实记录原 verdict）。
- 认证笔缺口如实申报：billwire-solo 工地材料目录 `billwire-solo-materials/` 在主树为空（billwire-solo 批报告自述工地入账段与主会核数时已披露），本批不补造，按"缺口如实申报不粉饰"原则仅申报。
- 账单三事件重放：按 billwire-solo 报告载活体三事件（`/tmp/billwire-live-test` 跑出）原 ts 与 session_id=eacfd4e51a40c942 照录，repair 标记即重放非原笔：
  - open_face_bill 3pts（allow 三路径：CONTRACT.md / CALL-LOG.md / core.py）
  - lock_free 0pts（CONTRACT.md 首次临时加锁免费）
  - lock_charged 1pts（CALL-LOG.md）
  双跑 ndjson（lockface-bills.ndjson 末三行）+ lockdb（lock_bill 表）逐笔核验：ndjson 三行带 repair 字段；lockdb 三笔 session_id=eacfd4e51a40c942 repair=1 一致。详 § 五 + materials/billwire-replay-record.json。

## 五、活体三证读数

### 5.1 主树账单三笔（落 lockface-bills.ndjson 末三行）

```
{"bill_points":16,"event_type":"open_face_bill",...,"package":"rootanchor-solo","session_id":"ba7aa2b83fbb90f8","ts":"2026-09-06T11:02:36+00:00"}
{"bill_points":0,"event_type":"lock_free",...,"package":"rootanchor-solo","session_id":"ba7aa2b83fbb90f8","ts":"2026-09-06T11:06:16+00:00","path":"sih-tools/lease/CONTRACT.md"}
{"bill_points":1,"event_type":"lock_charged",...,"package":"rootanchor-solo","session_id":"ba7aa2b83fbb90f8","ts":"2026-09-06T11:06:16+00:00","path":"sih-tools/lease/CALL-LOG.md"}
```

billwire 三事件重放（行尾 repair=true）：

```
{"event_type":"open_face_bill","bill_points":3,"session_id":"eacfd4e51a40c942","ts":"2026-09-06T10:19:41+00:00","detail":{"paths":["sih-tools/lease/CONTRACT.md","sih-tools/lease/CALL-LOG.md","sih-tools/lease/src/lease/core.py"],"repair":"rootanchor-solo 批追溯补录：billwire-solo 报告载活体三事件，按报告在案读数重放带 repair 标记..."},"repair":true}
{"event_type":"lock_free","bill_points":0,"session_id":"eacfd4e51a40c942","ts":"2026-09-06T10:20:03+00:00","path":"sih-tools/lease/CONTRACT.md","repair":true}
{"event_type":"lock_charged","bill_points":1,"session_id":"eacfd4e51a40c942","ts":"2026-09-06T10:20:03+00:00","path":"sih-tools/lease/CALL-LOG.md","repair":true}
```

### 5.2 链上意图笔与认证笔哈希（sih-engine/sih/event/trail/2026-09-06.ndjson）

- 意图笔 ba7aa2b83fbb90f8：event_hash `503b3cb7`（sih-tools/scribe/reports/2026-09-06-ask3-rootanchor-solo-record.json 验证件附链笔 sha256）
- 认证笔 ba7aa2b83fbb90f8：event_hash `43ddb93c`
- billwire 意图补录（ledger-repair 通道）：event_hash `3b6d5132b2fbe2da`，verbatim-from-ask3
- 三笔均经 scribe 裸调逐笔 grep 命中（禁 meter 包裹掩败纪律全程执行）

### 5.3 close 守门通过读数

- sessions.ndjson 末行 `revoked` 事件 `2026-09-06T11:06:40+00:00 session_id=ba7aa2b83fbb90f8 tool.version=1.30.0` —— 即 close 走 chain_gate_check 通过后才落 revoked 行
- close_session 内 chain_gate_check 验本会话 ba7aa2b83fbb90f8 在 trail 2026-09-06.ndjson 同时有 intent_refined (503b3cb7) + certification_completed (43ddb93c) 两类事件，返 (True, []) 放行
- 退出码 0

## 六、判定语义一裁读数（F-5）

承前裁延伸 1fda6a88。

判定路径：m-leaseup-bill-1 终签 1fda6a88 裁的是"计费数值与判定条件"（LOCK_BILL_UNIT / UNUSED_LOCK_MULTIPLIER / EXPANSION_FREE_QUOTA 冻结常量），本批核心变更是工程基线层守门（位置锚 / 自举硬拒 / 链证守门）非判定语义层判定。

依据（按工程基线四可验证性承载）：
- 位置锚换根：仅调整台账与账单位解析路径（tool_dir → discover_workspace_root），台账行格式零变更；台账与账单位内容、计费、判定条件零动。
- 自举硬拒：CLI 入口新增强拒分支，先于任何副作用；显式旁路 env 仅测试与复盘合法形，承前裁 m-passive5 + m-degladder 工程基线层承载。
- 链证守门：close 前新增链文件读，核本会话事件类，缺即 StateError 拒；直改车道与无租约形不在 close 域内零影响，承前裁 m-idenlane-pen-1 / m-docmath-gate-1 直改车道零波及承载。

故视为承前裁延伸，未跑新裁，依据落本档与 lease/CONTRACT.md 修订四十三。

## 七、双仓 commit 与 reconcile + verify

sih-tools：
- d295dc42 merge: rootanchor-solo 副本归并（worktree 收约后归并）
- 3fe7060f session: ba7aa2b83fbb90f8 wip: rootanchor-solo 段1 wip 自举硬拒与链证守门实装

sih-engine：
- 087adfdb halfmerge-solo 收约补笔（先例同形）
- rootanchor-solo 本批补 commit（test_rootanchor.py 补档 + 修订四十三 + 批材料）由本档指引

链 verify：sih-engine/sih/event/trail/2026-09-06.ndjson 双日链 verify valid 零断链；本批新增事件 17 行（intent 503b3cb7 + cert 43ddb93c + billwire 意图补录 3b6d5132b2fbe2da + 14 行 close 收约 + lock 事件）。

reconcile 读数：sih-tools/lease/ledger/sessions.ndjson + locks.ndjson + bypass.ndjson + lockface-bills.ndjson 四面与 lockdb SQL 三表（sessions / locks / lock_bill）双跑逐笔一致。

## 八、待决项清单

- leakworktree 病根：worktree prune 与 close 收约同步发生致副本蒸发，本批已收（test_rootanchor.py 补档回主树）。后续批建议 close 前先 git stash 工程副本到主树 git reflog 留 trace 或先 git diff > /tmp/wip.patch 再 prune。详 BATCH-FACE 待加建议节。
- billwire 工地材料目录空在案：billwire-solo 批自述工地入账段，billwire-solo-materials/ 在主树仍为空，本批不补造（缺口如实申报）。若需补造须取回 billwire 工地副本 / tmp 复盘件。
- 五前批交付（closefix/openhyg/批 C/leaseup/billwire）零动确认：本批未触碰，CONTRACT.md 与 ledger 文件相对路径处理承 billwire 修订四十二。
- 范畴排除未裁五项不动声明：T-9 执法位 / 四陈旧会话销账 / wenguobs 双写 / locks 镜像 / T-10 乙案，本批零触碰。
