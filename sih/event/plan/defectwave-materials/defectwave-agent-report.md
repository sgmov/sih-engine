# defectwave 批子代理施工归报（全文照录）

工地：w/defectwave（分支 w/defectwave，基点 db82bd6）。全部先红后绿，零 git 写、零治理命令、零既有文件删移。

## 逐件：根因 / 改动 / 红转绿证据

### 缺陷一：lease open 正身/意图件缺席零校验
- 根因：cmd_open 对 identity/intent 路径 fs::read().unwrap_or_default() 直读，件缺席即空字节签发，record_sha256 落空串哈希。
- 改动：src/bin/lease.rs 新增 validate_open_inputs（:276）与调用位（:347，置于三台账参数检之后、目录骨架等一切副作用之前，零残留）；校验存在、非空、JSON 可解析、identity_hash 非空（同义键 /identity/identity_hash 与 /identity/hash 对齐既有取哈希面）；任一不满足 die(2)。
- 红证：tests/defectwave_open_input_gate.rs 四件修复前 exit 0 照常签发；绿证 5 passed 0 failed（含对照组 open_valid_inputs_still_issued_control，同义键 hash 形放行、台账恰一行）。

### 缺陷二：收约罚金取样窗口假罚（merge-then-close 形）
- 根因：collect_used_paths 在 close 时取样，手工归并后 base..branch 差分为空，used_source=worktree_diff 而集合空，实写锁全数误罚。
- 改动：src/bin/lease/commitlaw.rs :578-592（settle commit 成功后取样写缓存）；src/bin/lease/closegate.rs collect_used_paths 转 pub(crate)（:213）两边复用零复制，新增缓存三助手（:288/:295/:321，缓存件 <locks 台账同目录>/usedpaths/<session_id>.json），罚金块缓存优先（:1288）、缓存缺席回落现行 close 时取样形，sampled_at 只入罚单 detail 且仅罚单存在时（:1328），used_source 语义零变，回执零新增顶层键。
- 红证：tests/lease_usedpaths_penalty.rs 新增 settle_cache_survives_manual_merge_before_close——修复前罚单 unused_paths=[src/a.rs,src/b.rs] bill_points=2 used_source=worktree_diff（病灶签名与实批 9 点假罚同形）；绿证 7 passed 0 failed（既有六件零扰——其 harness 不走 commit settle，缓存缺席自然回落，行为字节不变）。

### 缺陷三：scrutinator 工地路径域匹配缺口
- 根因：normalize_worktree_rel 只认 worktrees/<仓>/<批>/ 布局，w/<名>/（今引擎工地惯例）不归一即域外 exit-2。
- 改动：src/scrutinator/rule.rs :244-260（补 w/<名>/rest 至 sih-engine/rest，段数不足返 None，域清单零扩面）。
- 红证：src/scrutinator/tests.rs 新增三件两件修复前红（w/ 形判域外、归一返 None），worktrees 形回归钉本即绿；绿证 lib 纯函数 3 passed 加 CLI 级集成 tests/defectwave_scrutpath_wform.rs 3 passed（w/ 形 exit 0、worktrees 形 exit 0 回归钉、域外仍 exit 2）。21 件 scrutinator lib CLI 级用例在共享 CARGO_TARGET_DIR 工地形因 CARGO_MANIFEST_DIR/target 回退位 NotFound 假红（基点同红；临时符号链接使 binary 可达后 21 件全绿，确证环境假红非本批引入）。

### 缺陷四：cascadeclose no_worktree_carrier 存量红
- 改动：纯测试改——tests/lease_cascadeclose_projection.rs 该件 open 后 close 前以 git worktree remove 正形拆除引擎工地（裸 rm 会留 worktree 元数据致 branch -d 拒删，已实测并修正），模拟载体灭失。
- 红证：修复前该件红，回执 cascade_gate engaged（result=updated）——--repo sih-tools 经 pk-104 推导并集实得双仓；绿证 6 passed 0 failed，生产码零改动。
- 实读结论：载体判定（closegate.rs cascade_projection_rebuild）实义为会话 repos 中 sih-engine 仓工地在席且其 doc/CASCADE.json 在案即载体；skip 形不是死形——pk-104 推导形后 sih-tools 真批仍可零引擎仓入场，载体中途灭失亦真实可达；红的真因是测试假设 --repo sih-tools 得单仓会话而推导并集实得双仓。处置即候选法改测试，断言面原样验证力不降。

### 缺陷五（密五件）：attnanchor 域覆盖不做 last-row-wins
- 根因：registry_domains 逐行扫凡 status==active 即收域根，撤牌笔不灭覆盖，撤牌域每次提交日误报。
- 改动：src/bin/attnanchor.rs :325-350（registry_domains 改 last-row-wins：按 domain_root 即覆盖单元取文件序末行 status，仅最终态 active 入覆盖面；单行域与 /private/tmp 探针行行为不变；输出改域根排序确定性面）。
- 红证：tests/defectwave_attnanchor_coverage.rs 两件修复前红（撤牌域 alarm_count=1 即病灶本体；混合域同红）；绿证 3 passed 0 failed（含 stopped→active 反转域仍覆盖的双向钉防过杀）。真台账验证：修复后 binary 对真工作区跑读数越限告警零条（sim-aesthetic-workbench 撤牌域告警灭）。

## 全量回归读数（cargo test --no-fail-fast）

- lease 系：t2_golden 4 / t3 2 / t4 1 / t5 5 / t6 5 / t7 7 / open_allow_parse 2 / openface 4 / gap_locksview 2，全绿。
- usedpaths 系 7 passed；cascadeclose 系 6 passed。
- scrutinator 系：lib 纯函数（含新三件）18 加 CLI 集成 3 全绿；21 件 lib CLI 级无链接时环境假红（基点同红，见上）。
- defectwave 五测试文件：5+7+6+3+3+3 全绿。
- lib 全量：257 passed 1 failed 8 ignored——唯一红 event_stream::tdd_tests::t2_production_trails_verify 系任务书预告的本工地环境假红族，未触碰。
- 集成全量（mergeall 全系、attractor 系、gap 系等约 50 套）全绿。

## 偏差申报

1. 行为变更连带 harness 适配（缺陷一闸序）：tests/lease_mergeback_t5_commit.rs 与 tests/lease_mergeback_t7_sweep.rs 原 harness 以不传或传 /nonexistent 身份件的旧松形过 open，闸前置后红属修复有意后果；适配形为夹具补合规 identity/intent 两件并显式传参，t5 与 t7 原验证力保住，t7 本有注记预告此演进属预期。闸序取输入件校验先于 stem 闸：正身件缺席与未立名两教学位并存时先报输入位，两俱 exit 2，候主会裁认可。
2. 缺陷二残余边界：末次 settle 之后 close 之前若经 closeguard 预收提交扫入新脏面，缓存取样不含该批文件，该窄缝可能残余罚点；settle-then-close 纪律内不可达，如实申报候裁。
3. 环境处置留痕：临时符号链接 w/defectwave/target 验毕即撤零残留；scrutinator 测试残留件已清；21 件环境假红与 event_stream 一件未修承任务书不修令。
4. 缺陷五键位裁量：last-row-wins 键取 domain_root（覆盖单元、函数输出位）而非 token_id——真台账两键判全同（SiInfer/InferServer 活、其余撤牌域灭），同根多令牌以文件序末行为准，码注与测试双向钉申明，候裁认可。
