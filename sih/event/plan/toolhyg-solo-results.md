# toolhyg-solo 结果档：工具卫生三件——callloghyg 扩面与 confpreempt 残行处置与 zcode 死旗标漂移记泊

> 承接：任务包 sih-engine/sih/state/plan/toolhyg-solo.md 与用户 2026-09-09 令「继续。多子代理协作」即 mcpline 波三批并行派单。产出即 guardcore.py 白名单 additive 增两卷与 confpreempt 停滞会话正道处置落链与 pk-080 泊位三件。
> 队形单线形 solo，日期 2026-09-09，会话 sess-zcode-260909-toolhyg（session_id a8d94ceaff746071，lease 1.39.0）。并行在飞：mcpsec-solo 与 specfix-solo 共用当日链（trail 属 SCOPE_SHARED_SURFACE 共享追加面，零冲突零 wait-turn 实发）。

## 意图锚定

- 意图事件：intent_refined `143d071b-bf7a-41c6-952b-30f2c7804fed`（event_hash `f3db4aec07d7b926...`）
- record：sih-tools/scribe/reports/2026-09-09-ask3-toolhyg-solo-record.json
- validation：sih-tools/scribe/reports/2026-09-09-ask3-toolhyg-solo-validation.json
- 三锚引文程序切片自 proodos 06-on-canon 与 07-on-assay 与 08-on-settle 原文，逐字节断言在生成过程内。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0 findings 空；ask3repeater exit 0 status ok anchor_count 3。
- 叩问：十词查检九信号（停泊笔、双旗、扩面、收编面、检验文件、死旗标、残行、泊位、直改车道；白名单已注册零信号），契约内九条处置后 digest passed covered 9。
- 正身：identity verify 0.5.0 attest 零异常，identity_hash 6d18fb6b26de6180...。
- 租约：lease open 十四路径 allow 双仓登记，逐路径 lock 十四锁全 rc=0。

## 件一读数（callloghyg 扩面）

- 白名单扩面：sih-tools/lease/src/lease/guardcore.py 的 DIRECT_LANE_FILE_WHITELIST 增 sih-tools/attnanchor/CALL-LOG.md 与 sih-tools/attractor/CALL-LOG.md 两卷（acceptor 与 cascade 之间 sorted 位），冻结面由廿一册扩至廿三册，纯追加零删改零移动。
- 测试契约同步：tests/test_guard.py 的 test_direct_lane_file_whitelist_frozen 集合断言 additive 增两 expected 条目与 docstring 扩面注记。
- 版本零 bump：GUARD_VERSION 1.16.0 与 lease 包版本俱不动（行为面 additive 白名单，全测试族无版本断言即测试契约不强制 bump，recclsf 先例的重跑条款未触发）。
- 收编面单源核：lease/core.py calllog_treadmill_faces 单源自 DIRECT_LANE_FILE_WHITELIST 派生（禁第三份拷贝条款），两新册随清单机械生效，core.py 零改动。
- 全测试族：工地内 pytest tests/ 两跑俱 exit 0，309 passed 零 fail（报告 toolhyg-solo-materials/lease-test-family.log）。
- 双旗病根收编：主树 attnanchor/CALL-LOG.md 与 attractor/CALL-LOG.md 的待收投影行（settlement-v2 与 fusadopt 与 mcpopen 三会话纯追加，权威腿 calls.ndjson 已在版控含同数行）随本批 settle 入分支，收约时盘面内容与分支 tip 逐字节一致即 released_by=settle，双旗 bypass-orphan 与 bypass-calllog 根源消除（收约实测见本档收约读数节）。

## 件二读数（confpreempt 残行处置）

- 会话：09326a760119e4ed（2026-09-07 confpreempt-solo，lease 1.33.0 签发，零现势锁，链上仅 intent_refined 05762749 与 certification_completed 5979b9f2 两笔，检验文件 confpreempt-solo.json 缺位）。
- 第一腿 takeover 实跑取拒证：`lease takeover --package confpreempt-solo` 退出码 1，报文「检验文件缺位（无在营窗口，可直接 open）」，拒证全文在批材料 confpreempt-takeover-refusal.json。
- 第二腿 ledger-repair 补录：择合规者为 ledger-repair（takeover 无处置对象即检验件缺位，残行本体是台账缺 revoked 终笔正落补录域），载荷 rebuilt-revocation-from-ledger-evidence 形照 idenlane-solo 会话卫生核查先例，source 显式声明 verbatim 记否，`lease ledger-repair` 退出码 0 repaired 1 rejected 0 skipped 0，载荷与事由全文在批材料 confpreempt-revoked-payload.ndjson。
- 复核：处置后 active_sessions 4 即 specfix-solo 与 mcpsec-solo 与 release09-solo 与本批，confpreempt 09326a760119e4ed 已出在册视图。
- 铁律对表：全程零手改台账 ndjson，追加经 lease ledger-repair 子命令 flock 临界区。

## 件三读数（zcode 死旗标漂移记泊）

- 泊位：pk-080（序号顺延核零占用，全库最在泊 pk-079），工具线泊界。
- 三件：其一材料件 sih-tools/parking/materials/pk-080.json 照 pk-075 JSON 形（action enter、ttl 30 天、state parked）；其二 sih-tools/PARKING-v1.md 在泊名录投影行插于收束句前（additive 一处）；其三停泊笔 scribe park 落当日链 event_id `1beedd2e-60ec-4abe-ab5a-ac14e168b0f1`（event_hash `9bd4b2e0c55759a3...`），scribe confirm found true index 91。
- context 双录：mcpcold-solo 批申报原文逐字照录（sih-engine/sih/event/plan/mcpcold-solo-results.md 第七十四行）加本批复现实测（批 shell 内 zcode CLI 不在 PATH 即 which 退出码 127，四处候选位俱无可执行件，三旗标拒识无可本地复现，两录俱载零代裁候人节点复核）。

## 管线读数

- 化格：formatter packs/general-v1 对本档与 PARKING-v1.md。
- 核阅：des-001 对本档（sih/event/plan 域外）与 PARKING-v1.md（sih-tools 域外）俱退出码二域外如实记档。
- 检词：nomenclator packs/core 对本档与 PARKING-v1.md。

## 红证留痕

- 叩问 check 首跑经管道 tail 掩码取到 tail 退出码 0，即改裸跑取真码 1（有信号），未据假码推进。
- takeover 首跑经 tee 管道且 zsh 无 PIPESTATUS 掩取 tee 退出码 0，即改重定向裸跑取真码 1（拒证），未据假码推进。
- 两笔红证即本节如实留档，零删除零清洗。

## 越线与误差申报

- 零越线：写入面俱在任务包 allow 清单内；lease 其余模块、critsweep、gauge、scribe、mcpline、sih-engine/doc/** 零触碰。
- 勘误注记：BATCH-FACE 书简意图 verbatim 形缺 --sessions 参数，本批按 2026-09-04 闸三勘误带参一次通过，属勘误承接非漂移。

## 收约读数

- 收约机械读数（calllog_gate 与 treadmill 与无主闸与差集闸与 bypass 双旗消除实证）随收约链笔 revoked 行 detail 留档，完工回报向主窗转述。
