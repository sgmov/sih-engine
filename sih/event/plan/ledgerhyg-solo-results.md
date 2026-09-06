# ledgerhyg-solo 结果档：台账卫生三件处置批

> 批：ledgerhyg-solo（T6 单线 solo，委外代理亲写零子代理）
> 会话：60d7b42a3ff21341
> 日期：2026-09-06
> 令源：用户 2026-09-06 令「得一裁」（主会解读：三小件处置方案打包一命题过得一裁，裁过即执行）
> 任务包：sih-engine/sih/state/plan/ledgerhyg-solo.md

## 一句话结论

批以一裁收口三件处置：件一四陈旧会话销账补录 7 行 verbatim 已在主会前批 5cba19e5+5daac427 完成本批仅核实与文档化收口、件二 wenguobs 双写视图去重在 status 与 reconcile 视图层加 (event, session_id, issued_at) 三键 view_dedup 字段台账字节零动 8 件夹具全绿 195 件基线零回归、件三 locks 镜像缺 131 行不补加文档化即 data-hygiene-report.json 补 ledgerhyg_solo_disposition 三处置段；一裁 m-ledgerhyg-hyg-1 9 发 stable_clear 机器终签链笔 be7fc3c0b8413480dc91657d6f5dfcde52e0c34eac47914443202d242ed63928；双仓 settle + close（过 closefix 新归并机械 1.30.0 加链证守门） + reconcile + verify（主树二进制链 114 事件 valid）；本批实装形与规约意图一致即 append-only 历史行零改写零删除、台账行格式零变更、ledgerwrite 唯一写点零动、账单台账零触碰、判定语义零变化、ledger 改前改后 SHA-256 6ba3830d5491bf96e4935189ef18f4a983d2ffdefc030d56d676fe7920ea6124 逐字节一致。

## 一、T-1 三件证据核实

### 件一 四陈旧会话销账补录（在档现状）

| session_id | package | 行号 | 事件 | verbatim | source |
|---|---|---|---|---|---|
| be316fe3596bf16e | idenlane-human-solo | 735 | revoked | None（原始在册） | ledgerloss5 §六前已入 |
| be316fe3596bf16e | idenlane-human-solo | 757 | issued | True | verbatim-from-vcs 父提交 2b7b1ce2 |
| 1d463181f4bf7570 | watchcheck-solo | 755 | issued | True | verbatim-from-vcs 父提交 e8d632da |
| 1d463181f4bf7570 | watchcheck-solo | 756 | revoked | True | verbatim-from-vcs 父提交 e8d632da |
| ab0aa6b23047a03d | docmath-namefit-solo | 758 | issued | True | verbatim-from-vcs 父提交 e9dc3cfb |
| ab0aa6b23047a03d | docmath-namefit-solo | 759 | revoked | True | verbatim-from-vcs 父提交 e9dc3cfb |
| dc8a180223aa590f | regula-rename-solo | 760 | issued | True | verbatim-from-vcs 父提交 3a7ad35d |
| dc8a180223aa590f | regula-rename-solo | 761 | revoked | True | verbatim-from-vcs 父提交 3a7ad35d + idenlane-human 收约备份件 737 行版 |

合计 8 行在档（1 be316fe revoked 原始 + 7 行 verbatim 补录），俱经 ledger-repair 通道，repaired_at=2026-09-06T06:31:08+00:00 标记显式，源 VCS 父提交指针俱可逐笔 grep 命中。证据不足者：零——四会话名单逐个有 VCS 源或备份件，无候裁清单。

### 件二 wenguobs 双写行（字节对表）

同 session_id 3a1171b19da42244 同 2026-08-31 issued/revoked 同内容成对四组共 8 行（行 335/336、337/338、339/341、342/343），四笔 issued 行字段全同唯 allow 面递增（14→16 项），四笔 revoked 行 reason 各异如实载重开事由（范围闸拦改包重开、lease 1.9.0 重开同 session_id 漏 identity_hash、lockcore active_sessions 重复 pop bug、换新 identity 报告）。原始台账字节 SHA-256 改前改后逐字节一致 6ba3830d5491bf96e4935189ef18f4a983d2ffdefc030d56d676fe7920ea6124。view_dedup 实装后 raw=14 unique=8 collapsed_groups=2 collapsed_total_extra=6（即 4 issued 合并为 1 + 4 revoked 合并为 1，wenguobs-solo 包面外同会话不同包形不误伤）。

### 件三 locks 镜像缺 131 行

lockdb SQLite 正典 6032 笔 acquired/released 与 mirror 5901 笔差 131 笔 db 独有；acquired 69 + released 62；日期分布 2026-09-05 计 109 笔 + 2026-09-06 计 22 笔与 ledgerloss5 §一伤亡窗吻合；top7 会话 164006f3ac45fcaf 28 + ab0aa6b23047a03d 22 + 917bbc4e8c948f38 22 + 8ed84146513364f2 18 + be316fe3596bf16e 18 + 445c12fbb3e4283c 16 + 4bdcc62b5357c7a8 7。state_divergence mirror_derived 13 = db_lock_state 13 = only_in_mirror 0 = only_in_db 0 零判定漂移。判定正典在 lockdb 无损，镜像缺行属可见性损失不属判定漂移。修不修随用户裁，处置方向不补加文档化。

## 二、T-2 一裁命题材料与 facet 合同模式测量，执契终签

### 命题

gid: m-ledgerhyg-hyg-1
title: 台账卫生三件处置命题：四陈旧会话销账补行、wenguobs 双写视图去重零改写、locks 镜像缺行不补文档化俱符基线四
n: 9
ng: medium

### facet 合同模式九发

合同：sih-tools/facet/contracts/ledgerhyg-260906/m-ledgerhyg-hyg-1/contract.json（n_declared=9）
响应：sih-tools/facet/contracts/ledgerhyg-260906/m-ledgerhyg-hyg-1/responses.jsonl（9 发同判 comply）
基线：sih-tools/facet/contracts/ledgerhyg-260906/m-ledgerhyg-hyg-1/seat-baseline.json（identity_hash 1c408342e1995b1cad615d0cbad2f647faadd3cd932e0f4ae7f6f04448603970 可用）

### 计分

measure.py --score：9/9 comply 变卦 0% 旗 0；闸门裁决（判据 v3）= stable_clear

### tally assemble + check + verify + sign

- tally-material.json：sih-tools/proposition/DES/m-ledgerhyg-hyg-1/tally-material.json
  - kind: tally-check-input
  - dc_fingerprint: ed378b88a9a06b6c
  - identity_hash: 1c408342e1995b1cad615d0cbad2f647faadd3cd932e0f4ae7f6f04448603970
- attractor check：verdict=pass direction=comply disposition=裁决通过（12 件零失败 R1-R7 全过）
- attractor verify：identical
- attractor sign：signed=m-ledgerhyg-hyg-1
  - scribe crosscheck 链笔：be7fc3c0b8413480dc91657d6f5dfcde52e0c34eac47914443202d242ed63928（crosscheck_completed）
  - certification_completed 链笔：ab5577e92b23f109b68e065a50d01a151a32878e45312a220364a1da132f72f3
  - intent_refined 链笔：27b7f2b4bfc0274ae4bc82fecbd63dfafdeb55b3dd7491870a8ffd805d800489

非 near_threshold 免呈报转主会条款不触发。

## 三、T-3 件一销账补录

实装形：四陈旧会话销账补录 7 行 verbatim 已在主会前批 5cba19e5（主会补录三笔 watchcheck 1d463181 两行加 idenlane-envelope issued 逐字）+ 5daac427（伤亡 7 行补录）两批完成。

本批动作：仅做证据核实与文档化收口。8 行在档（1 原始 be316fe revoked + 7 verbatim）SHA-256 6ba3830d5491bf96e4935189ef18f4a983d2ffdefc030d56d676fe7920ea6124 改前改后逐字节一致在档。证据不足者：零——四会话名单逐个有 VCS 源或备份件，无候裁清单。

## 四、T-4 件二视图去重实装与夹具

### 实装

sih-tools/lease/src/lease/core.py 新增 `view_dedup_events(events, package_stem=None)` 函数：按 (event, session_id, issued_at) 三键合并同内容行；原始台账字节零动；判定语义零变化（active_sessions 配对判不活跃与原工具完全一致）。

status 输出新增 `view_dedup` 字段（位于 sessions 子段）：含 summary（raw_events/unique_events/dedup_keys/collapsed_groups/collapsed_total_extra）与 events（去重后的事件列表）。

reconcile 通道不动判定面（承 des-011）：git log 对会话台账对 trail 对 bypass 四对表语义与原工具完全一致。

### 夹具

sih-tools/lease/tests/test_ledgerhyg.py 8 件全绿：

1. test_view_dedup_wenguobs_doublewrite_collapses_to_pair：wenguobs 8 行去重即 2（1 issued + 1 revoked），collapsed_groups=2 collapsed_total_extra=6
2. test_view_dedup_different_sessions_kept：合法同包多会话形不同 session_id 不去重
3. test_view_dedup_different_times_kept：同 session_id 但不同时刻 issued 不去重
4. test_view_dedup_does_not_change_active_sessions_semantics：判定语义零变化
5. test_view_dedup_preserves_first_seen_order：同键后到者被吸收先到者保留
6. test_view_dedup_summary_keys：summary 字段契约
7. test_view_dedup_empty_events：空列表空 summary
8. test_view_dedup_original_bytes_zero_change：台账行字节零动

### 全测试族

lease 195 件全绿（187 基线 + 8 新增 ledgerhyg 夹具），零回归。台账行格式零变更，ledgerwrite 唯一写点零动，账单台账零触碰。

## 五、T-5 件三文档化

sih-engine/sih/event/plan/closefix-solo-materials/hygiene/data-hygiene-report.json 补三段：

- locks_mirror_vs_lockdb.ledgerhyg_solo_disposition：decision="不补加文档化"，rationale 载判定正典 lockdb 无损 6032 笔 acquired/released 零判定漂移，missing_rows_pointer 载缺行清单指针（by_event/by_date/by_session_top 三表 + anchors 锚位 + verdict_no_drift），zero_mirror_writes=true，append_only_preserved=true
- sessions_wenguobs_doublewrite.ledgerhyg_solo_disposition：decision="读数视图去重零改写"，rationale 载 (event, session_id, issued_at) 三键视图去重，view_dedup_jig 指针 core.py view_dedup_events + status.sessions.view_dedup，test_fixture 指针 test_ledgerhyg.py 8 件全绿，ledger_bytes_zero_change=true
- ledgerhyg_solo_session_closeout：四陈旧会话销账补录收口段，sessions 数组 4 项每项载 package/rows/issued_repaired/revoked_repaired/source 源 VCS 父提交指针；total_rows=7 all_evidence_complete=true evidence_audit_file 指针 first-run-2026-09-06/run1/four-sessions-audit.txt

## 六、T-6 全测试族零回归与双仓 settle + close + reconcile + verify

### 全测试族

lease 195 件全绿（187 基线 + 8 新增 ledgerhyg 夹具），零回归。

### 双仓 settle

sih-tools 仓 commit e3b4aad6：ledgerhyg-solo 视图去重 view_dedup 实装（修订四十四）
sih-engine 仓 commit b62ebd0：ledgerhyg-solo 件三文档化：data-hygiene-report.json 补三处置段

两仓各 cert=ab5577e92b23f109b68e065a50d01a151a32878e45312a220364a1da132f72f3 命中链上 certification_completed 事件。

### close

lease close --package ledgerhyg-solo（过 closefix 新归并机械 1.30.0 加链证守门）
- session_id: 60d7b42a3ff21341
- revoked: true
- 双仓分支 msh/ledgerhyg-solo 删除
- 双仓工地 worktrees/{sih-tools,sih-engine}/ledgerhyg-solo 拆除

真分叉冲突旁证一处：detect_merge_conflicts 目录形态误判（closefix §九旁证同款），处置按备份让位归并对表法（备份主树件 → checkout/unlink 让位 → close 归并 → diff 备份与归并结果 must identical 验证），备份件存 .close-backups/ledgerhyg-solo/60d7b42a/。

### reconcile

sih-tools 仓：summary {total:356, routed:125, routed_direct:1, bypass:54, cert_missing:82, session_orphan:19, unbypassed:76, unrouted:0, sealed:0, first_routed:125}
sih-engine 仓：summary {total:401, routed:148, routed_direct:0, bypass:82, cert_missing:106, session_orphan:24, unbypassed:41, unrouted:0, sealed:0, first_routed:148}

### verify

引擎 scribe verify --trail：status=valid，events=114，first_hash=7ac5aefcfa91a0f716600770835a5ebf1a8725362a1d604217b9d0199da55980，last_hash=ab5577e92b23f109b68e065a50d01a151a32878e45312a220364a1da132f72f3

## 七、F 表自检

| F 锚定 | 类别 | 判据 | 状态 |
|---|---|---|---|
| F-1 | 一裁 | 单命题九发 stable_clear 过执契终签在链，near_threshold 呈用户 | ✅ 9/9 comply stable_clear 链笔 be7fc3c0 |
| F-2 | 销账 | 四会话 revoked 行入账逐笔验证，repair 标记真假如实（逐字或重建） | ✅ 7 行 verbatim=true + 1 原始 be316fe revoked（ledgerloss5 §六前已入）；逐字与源 VCS 父提交指针俱在；证据不足者零 |
| F-3 | 双写视图 | 去重夹具绿即同键双行计数为一；原始台账改前改后哈希逐字节一致 | ✅ 8 件全绿；SHA-256 6ba3830d5491bf96e4935189ef18f4a983d2ffdefc030d56d676fe7920ea6124 改前改后逐字节一致；raw=14 unique=8 collapsed=2 |
| F-4 | 镜像文档 | 数据卫生报告定案段在档，零镜像写 | ✅ data-hygiene-report.json 三处置段在档 ledgerhyg_solo_disposition + ledgerhyg_solo_session_closeout；零镜像写 |
| F-5 | 零回归 | 187 基线全绿；台账行格式零变更；判定语义零变化 | ✅ 195 件全绿（187 基线 + 8 新增）；台账行格式零变更；ledgerwrite 唯一写点零动；账单台账零触碰；active_sessions 配对判不活跃与 reconcile 四对表语义与原工具完全一致 |

## 八、版本与工件

- 三源对齐 1.30.0（pyproject + __init__ + CONTRACT 修订四十四）
- lease/src/lease/core.py view_dedup_events 函数 + status.sessions.view_dedup 字段
- lease/tests/test_ledgerhyg.py 8 件夹具
- lease/CONTRACT.md 修订四十四（视图去重注记）
- lease/CALL-LOG.md 留痕一笔
- scribe/CALL-LOG.md 留痕一笔
- data-hygiene-report.json 三处置段补
- facet/contracts/ledgerhyg-260906/ 合同 + topic + responses + seat-baseline + score-material
- proposition/DES/m-ledgerhyg-hyg-1/ tally-material + tally-check-report + signcheck + flywheel-trail

## 九、关联

- 任务包：sih-engine/sih/state/plan/ledgerhyg-solo.md
- 盘点源头：sih-engine/sih/event/plan/ledgerloss5-solo-results.md §六与 closefix-solo-materials/hygiene/data-hygiene-report.json
- 上游：ledgerloss5-solo（件一件三盘点）+ closefix-solo（件三报告基线）
- 一裁命题：m-ledgerhyg-hyg-1（sih-tools/proposition/DES/m-ledgerhyg-hyg-1/）
- 同线：leaseup 线三批收口后卫生批
