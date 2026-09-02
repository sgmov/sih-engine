# listzero-solo：待令清单清零五修批

> task-packages 治理任务
> 承接：用户 2026-09-02 令「得一裁待零清单，一裁一过一开」；九项裁毕七实锤
> 队形：单线形 solo（主会话手跑）
> 日期：2026-09-02

## 一、问题陈述 {#problem}

待令清单五缺陷一批修净：一 tally sign_material 在 scribe 非零退出时仍打印 signed 误导读数者（cli.py L336 无条件打印、退出码唯真）；二 nomenclator dead_ban 无语境豁免致 Fréchet 法文题名 Points 撞死档七处存量（c006fin 六加 TOP-007 一）；三 retriever recall 零命中写零字节文件与无输出不可辨；四 scribe intent_event 只查验证件 findings 不查 status 致被拒验证件可入流；五编组认领协议无后写复核致先认领被覆盖。

## 二、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 tally 失败态 | 工程 | 构造 scribe 非零退出场景：stdout 含 sign failed 且不含 signed 字样，退出码非零 |
| F-2 Points 清零 | 治理 | TOP-007 与 calculus 七处存量复跑 findings=0；dead 词表 Points 仍在册 |
| F-3 信封非零 | 工程 | retriever recall 多词 topic --out 件非零字节且 JSON 可解析 count=0；单词命中路径行为不回归 |
| F-4 血统拒入 | 工程 | 构造 status 非 ok 验证件调 intent_event：拒入且错误指明 status；status ok 路径不回归 |
| F-5 全测绿 | 工程 | cargo build 零错加 cargo test 全绿；nomenclator 既有测试全绿 |
| F-6 检词化格 | 治理 | 改动 md 件即 skill 与 CONTRACT 检词零违例、化格如实 |
| F-7 lease 链 | 治理 | 双仓全程，verify 零，快照末笔后 |

## 三、请求写入 {#requested-writes}

- sih-tools/tally/src/tally/cli.py
- sih-tools/tally/CALL-LOG.md
- sih-tools/nomenclator/packs/core/
- sih-tools/nomenclator/src/nomenclator/check.py
- sih-tools/nomenclator/CONTRACT.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-engine/src/event_stream/intent.rs
- sih-engine/src/retriever/mod.rs
- sih-engine/src/bin/retriever.rs
- sih-engine/sih/state/skills/sihankor-marshalling/SKILL.md
- sih-engine/sih/state/plan/listzero-solo.md
- sih-engine/sih/event/plan/listzero-solo-results.md
- sih-engine/sih/event/plan/listzero-solo-materials/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- sih-tools/meter/counts/
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md

## 四、约束 {#constraints}

retriever 词面整串匹配语义不动；死词 Points 本体不摘册只加条目级 exempt；豁免只挂语境不挂全局；改动最小面不动既有 API 形；上链遇锁即等待；链快照在末笔后。

## 五、风险 {#risks}

引擎 worktree 全量重建耗时；intent_event 新校验若误伤现行 ok 形态即以既有 validation 实件回归测试兜底；豁免子串若过宽即以只匹配完整题名原文收窄。
