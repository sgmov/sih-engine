# sweepimpl-solo 管线与链读数（化格→核阅→检词序固定，笔在核前）

> 批：sweepimpl-solo（会话 9920edf3eb4d4124）；日期 2026-09-07；单线 solo 零子代理
> 工具版本：lease 1.32.0（本批升）/ scrutinator 0.1.0 / formatter 0.2.0 / nomenclator 0.2.0 / identity 0.4.0 / meter 0.2.0

## 一、化格（formatter general-v1 --write）

| 目标 | 退出码 |
|---|---|
| sih-tools 工地 lease/CONTRACT.md | 0（无需改） |
| sih-tools 工地 lease/CALL-LOG.md | 0 |
| sih-tools 工地 nomenclator/CALL-LOG.md | 0 |
| sih-tools 工地 nomenclator/packs/core/terms.json（sweep 词条收编后归一） | 0 |
| sih-engine 工地 sih/state/plan/sweepimpl-solo.md | 0 |
| sih-engine 工地 sih/state/plan/sweepimpl-solo-prompt.md | 0 |

## 二、核阅（scrutinator --pack des-001 裸名）

| 目标 | 退出码 | 处置 |
|---|---|---|
| 工地 lease/CONTRACT.md | 2 | sih-tools 域外 exit-2 如实记，不属违规 |
| 工地 lease/CALL-LOG.md | 2 | 同上 |
| 工地 nomenclator/CALL-LOG.md | 2 | 同上 |
| 工地 nomenclator/packs/core/terms.json | 2 | 同上 |
| 工地 sih/state/plan/sweepimpl-solo.md | 2 | state/plan 域外 exit-2 如实记（BATCH-FACE 域外注记实测相符） |
| 工地 sih/state/plan/sweepimpl-solo-prompt.md | 2 | 同上 |
| 结果档 sweepimpl-solo-results.md（event/plan） | 2 | 域外 exit-2 如实记 |

## 三、检词（nomenclator check --pack core）

| 目标 | 退出码 | 读数 |
|---|---|---|
| 工地 lease/CONTRACT.md | 1→0 | 首跑红证：修订四十六自述句引用死词面被 dead_ban 拦（1 笔）＋存量两笔死词条目（billwire/rootanchor 修订带入，主树基线即红）；两笔存量改入账、自述句改写避引死后复检 0 违例 |
| 工地 lease/CALL-LOG.md | 0 | 零违例 |
| 工地 nomenclator/CALL-LOG.md | 0 | 零违例 |
| 工地 sih/state/plan/sweepimpl-solo.md | 0 | 零违例 |
| 工地 sih/state/plan/sweepimpl-solo-prompt.md | 0 | 零违例 |

新词三态处置：扫残留（en Sweep code sweep）批内登记 nomenclator core 包 terms.json（sweepimpl-solo 批已签项），收编后跑化格归一规范形（test_core_pack_shipped_canonical 同形 serialize sort_keys+ensure_ascii=False+indent=2）；幻影会话、僵尸锁、散位收据、无主工地四词随 sweep 词条定义承载不单独立名，elicit 五信号经叩问处置 digest passed covered 5。

## 四、测试（lease 全族，工地内跑）

- 基线：主树 204 passed（2026-09-07 批前实测）。
- 本批后：209 passed（204 基线零回归 + test_sweep.py 5 件），1 warning 与基线同。
- 首跑红证：test_sweep.py 首两跑红（夹具构造缺陷：选择性 git add 缺失致未跟踪收据被误提交、checkout 清空目录、evidence 断言错位、append_text API 不存在），修复后全绿；红证输出随批材料归档不清洗（f1-f2 首跑红见会话记录，末跑红证 sweep-scan-selfboot-rejected-red.txt 即工地自举形缺 --bills 被 self_boot_check 拒 exit 2 首跑如实归档）。

## 五、链与租约实录

- 意图笔：event_id 71bd80fe-7d67-41ee-b35d-b85ca2908135，event_hash 03ce2f5e15de681c…（scribe intent，gate 三带 --session 与 --sessions 过）。
- 链 verify（settle 前）：status valid，events 90。
- 锁面：十六笔一次取齐全 rc=0（append 共享面 trail 与 scribe/reports 与 identity/reports 与 materials；exclusive 十二笔）；活批 basemgrimpl-solo 的 scribe/CALL-LOG.md 独占锁未入 allow 面（declguard viewline 先例形，收约走 --ack-uncommitted 认领通道候而不扰）。
