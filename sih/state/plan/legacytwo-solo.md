# legacytwo-solo：存量封账批

> task-packages 治理任务
> 承接：用户 2026-09-02 令「得一裁待零清单，一裁一过一开」listzero 后余量
> 队形：单线形 solo（主会话手跑）
> 日期：2026-09-02

## 一、问题陈述 {#problem}

三件存量封账：一 TOP-004 L17 注记悬空（仍写见 TOP-006 待建，实 TOP-006 已建紧致集）改如实注记；二 reconcile 老旗三笔与 golden_des001_gov003 同败与 LIM-008 查无差异落 errata 档 record-only（mathfix2 段2 cert 81ef1c7a 与 fmtfix 段2 cert 06c30867 经全链清单核证不在任何现存日链，git init 基线 c556abb 先于会话制结构性不可路由）；三 mem_recall_f_suite 套件 RecallArgs 缺 words 与 miss_log 补齐三处初始化使 cargo test 全绿。

## 二、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 注记如实 | 治理 | TOP-004 L17 不再含见 TOP-006 待建字样，新注记与在册实况一致，三门过 |
| F-2 errata 落档 | 治理 | sih-math/docs/ 新档四笔俱在且每笔证断可复算 |
| F-3 套件修复 | 工程 | cargo test --test mem_recall_f_suite 全绿 |
| F-4 无越界改动 | 治理 | git 仅三处改动即一注记行一新档一测试三初始化 |
| F-5 lease 链 | 治理 | 双仓全程，verify 零，reconcile 较批前零新增，快照末笔后 |

## 三、请求写入 {#requested-writes}

- sih-math/topology/entries/TOP-004-metric-space.md
- sih-math/docs/
- sih-engine/tests/mem_recall_f_suite.rs
- sih-engine/sih/state/plan/legacytwo-solo.md
- sih-engine/sih/event/plan/legacytwo-solo-results.md
- sih-engine/sih/event/plan/legacytwo-solo-materials/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- sih-tools/meter/counts/
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md

## 四、约束 {#constraints}

不改既有链事件；不动已建条目数学内容；不重录金向量；不摘 reconcile 旗；上链遇锁即等待；链快照在末笔后。

## 五、风险 {#risks}

套件测试依赖姊妹仓实数据即工地环境败如实记；TOP-004 改行过化格若被改写即逐字 diff 复核锚段无损。
