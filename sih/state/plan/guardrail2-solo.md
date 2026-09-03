# guardrail2-solo：写入门四闸批（同包唯一 + 重意图拒 + 会话在册验 + 收约预检）

> task-packages 治理任务
> 承接：用户 2026-09-03 故意同包双跑 locksplit 实测即两会话到收约均未发现双跑，机械指纹即链 167 与 168 双笔同记录意图与台账仅一会话与孪生批无租约裸跑上链；guardrail-solo 误差申报遗留即 close 半程不可达（收约两段式已三例即 mathrefmt2 与 facepark 与 idwire）
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械机械链全绿即放行
> 队形：单线形 solo，**本批单飞**（改写入门的批不与他批并发，开工前 lease status 验零活跃）
> 日期：2026-09-03
> 温故检索：materials/recall-guardrail2.json 双档零命中如实记
> 冲突测试：pk-045 参与者，单飞如实记零并发

## 一、问题陈述 {#problem}

四道该拦未拦：其一 lease open 不查同包活跃会话即同包双跑畅行；其二 scribe intent 不查同记录已在链即双笔意图畅行；其三意图与认证事件不验会话在册即无租约裸跑畅行；其四 close 前置态静态判定无法前瞻 git 合并内容冲突即半程不可达三例。

## 二、关键设计 {#design}

1. **闸一同包唯一**：lease open 增查 sessions 台账即同包已有 issued 未 revoked 会话即拒 PackageSessionActive 载会话号与包名，详情禁空串。
2. **闸二重意图拒**：scribe intent 增门即经 load_parking_scope 同款链目录全量重放查同 record 路径已有 intent_refined 即拒 IntentRecordUsedRejected；显式 --allow-reintent 默认关留补救位（facepark 丢事件重追加先例即合法重追加通道）。
3. **闸三会话在册验**：scribe intent 与 append 认证事件增验 --session 在 sessions 台账为活跃会话，无会话或已吊销即拒 SessionNotActive；显式 --no-session-reason <事由> 默认关留主会处置位（停泊重入与丢事件补救即此类）。两旗标均写 USAGE。
4. **闸四收约预检**：lease close 增 git merge-tree --write-tree 逐仓预检（在现有前置态探针后、真归并前），内容冲突即整批拒零动作载冲突文件清单详情，堵半程不可达（预检形承 guardrail-solo 误差申报候选案一）。
5. 单测各闸先红后绿含反例（双开拒、重意图拒、裸跑拒、预检冲突拒）加合法通道回归（先开后收、--allow-reintent、--no-session-reason、预检净过放行）；既有测试全绿即向后兼容。CONTRACT 修订与 USAGE 同步。

## 三、F 表 {#work}

| F | 判据 |
|---|---|
| F-1 四闸各拦 | 四反例实测被拒且详情非空 |
| F-2 合法通道不伤 | 先开后收与两旗标与净过回归全绿 |
| F-3 行为兼容 | lease 与 scribe 既有测试全绿 |
| F-4 写入仅 allow | 请求写入节所列 |

## 四、必读文件 {#read}

- 必读 1：sih-tools/lease/src/lease/（open 与 close 位）与 sih-engine/src/bin/scribe.rs（intent 与 append 位）与 src/event_stream/park.rs（scope 重放先例）
- 必读 2：sih-engine/sih/event/plan/locksplit-solo-results.md 双跑事故与 guardrail-solo-results.md 误差申报
- 必读 3：sih-tools/BATCH-FACE.md 调用面与坑位

## 五、约束 {#constraints}

1. 本批单飞，开工前验零活跃会话
2. 两旗标默认关即缺省收紧，向后兼容零破坏
3. 认证先落主树活链，账本 append-only 零改写
4. 词债不过夜 findings 亲读

## 六、请求写入 {#requested-writes}

- sih-tools/lease/
- sih-engine/src/bin/scribe.rs
- sih-engine/src/event_stream/
- sih-engine/sih/state/plan/guardrail2-solo.md
- sih-engine/sih/event/plan/guardrail2-solo-results.md
- sih-engine/sih/event/plan/guardrail2-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 四闸红绿与合法通道回归读数在档
- [ ] 认证入链双仓结算收约对表读数在档
