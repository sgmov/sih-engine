# scrutmerge-goldfix-solo：金向量脏目标补冻与引擎对齐批

> task-packages 治理任务
> 承接：用户 2026-09-01 同意令、切换批停报即主会同日裁定三层差异定性、金向量六件全零发现根因实证
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-01

## 一、问题陈述 {#problem}

切换批双跑判据揭出引擎件与工具件真差异两类：一是 finding 对象结构差即引擎平铺形 rule_id 与 line 与 message 与 pack 对工具嵌套形 pack 与 rule_id 与 path 与 location.line 与 message；二是规则语义差即 SPEC-013 引擎误报五处 S 系挂跳级消息、DEC-020 引擎漏报六处 C006、字符集消息 U+U+2026 双前缀单前缀之差。根因实证即金向量六件 findings 全零，净目标对 finding 结构与语义天然免疫，等价性从未被字节级钉住。规约洞在 SPEC-013 判据未要求含发现目标。

## 二、关键设计 {#design}

五件。一即脏目标金向量补冻：以工具件为基准，构造含发现目标至少四件覆盖 C001 加 C006 加 S002 加 S004 加 S005 加 S006 加 F000 加 F002 加 F003 加 F005 加 N002 十一码即 C002 允许表难自然触发可构造，脏语料落 sih-engine/src/scrutinator/fixtures/corpus/ 即双件治理域内，真目标 DEC-020 直接入冻；净目标金向量六件原样续用。二即结构对齐：引擎 finding 序列化改嵌套形逐字段同构工具件。三即语义对齐：修 S 系 rule_id 与消息映射即 SPEC-013 上引擎须零发现、补 C006 判定即 DEC-020 六处须逐条出、字符集消息格式照抄工具件含 U+U+2026 小瑕即字节一致契约下瑕疵也抄。四即规约堵洞：SPEC-013 修订四即金向量须含脏目标与双跑同参形两条款。五即停批档收编：切换批停报记录与双跑证据件入册。

引擎版本维持 0.1.0 即本批是实现纠偏非新增语义，升版留切换后随包语义变更统一走。工具件 U+U+2026 小瑕不修即基准侧冻结，双侧同步修瑕挂账后续批。

## 三、工作清单 {#work}

- [ ] 脏语料构造与工具件基准金向量补冻
- [ ] 引擎 finding 结构对齐嵌套形
- [ ] S 系映射与 C006 补报与消息格式对齐
- [ ] SPEC-013 修订四
- [ ] 停批档与证据件收编、管线认证、双仓收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 金向量成色 | 工程治理 | 脏目标金向量至少四件冻结即每件 findings 非零，覆盖十一规则码，成色清单入材料件；净目标六件续用零漂移 |
| **F-2** 结构对齐 | 工程治理 | 引擎件对新金向量逐字节断言即 cargo test 全绿，finding 形逐字段同构工具件 |
| **F-3** 双跑复验 | 工程治理 | SPEC-013 与 DEC-020 与 GOV-002 三目标同参形双跑 cmp 零差，退出码一致 |
| **F-4** 规约堵洞 | 链上治理 | SPEC-013 修订四两条款在档 |
| **F-5** 收口 | 链上治理 | 停批档与证据件入册、双仓 settle、链 valid、reconcile 零增、unrouted 不增 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/scrutinator/src/ 即工具件基准实现
- 必读 2：sih-engine/src/scrutinator/rule.rs 与 report.rs 即待修位
- 必读 3：sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md 即判据与修订体例

## 六、约束 {#constraints}

1. 工具件 scrutinator 的 src 与 tests 与 packs 零改动（红线）即基准侧冻结
2. 引擎改动限于 src/scrutinator/ 模块与测试与语料与金向量，其余源零碰（红线）
3. 金向量冻结后任何字段漂移即判负（红线）
4. U+U+2026 小瑕照抄不修（红线）
5. 上链前必须等绿、findings 亲读、禁管道掩退出码
6. 开工前 lease status 核 scrutmerge-switch-solo 已收约，未关即报不代收

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过

## 八、风险点 {#risks}

脏语料构造过窄即仍有规则码未被钉住，防御即成色清单逐码勾验加主会验收亲跑。结构改动破坏净目标金向量，防御即净目标六件测试续跑不删。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-01 同意令
- 链件：随批意图入当日链
- 关联：scrutmerge-sdd 与 tdd 与 tfix 系前批、切换批停报与主会裁定、SPEC-013

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[脏目标]: 消解 即大白话直述即含违例发现的目标，非登记术语
叩问处置[成色]: 消解 即大白话直述即覆盖质量，非登记术语
叩问处置[纠偏]: 消解 即大白话直述即纠偏返正，非登记术语
叩问处置[同参形]: 消解 即大白话直述即两侧参数逐字符一致，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/src/scrutinator/report.rs
- sih-engine/src/scrutinator/rule.rs
- sih-engine/src/scrutinator/mod.rs
- sih-engine/src/scrutinator/tests.rs
- sih-engine/src/scrutinator/fixtures/corpus/
- sih-engine/src/scrutinator/fixtures/golden/
- sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md
- sih-engine/sih/state/plan/scrutmerge-goldfix-solo.md
- sih-engine/sih/event/plan/scrutmerge-goldfix-solo-results.md
- sih-engine/sih/event/plan/scrutmerge-goldfix-solo-materials/
- sih-engine/sih/event/plan/scrutmerge-switch-solo-results.md
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-engine/sih/event/inputlog/<当日>.ndjson
- sih-tools/scribe/reports/2026-09-01-switch-stop-*
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
