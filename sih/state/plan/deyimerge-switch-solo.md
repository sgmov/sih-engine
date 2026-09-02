# deyimerge-switch-solo：得一融回切换批

> task-packages 治理任务
> 承接：用户 2026-09-02 切换放行令、SPEC-014 与 deyimerge-tdd 实装即验收过、DEC-013 三步曲第三步、核阅切换先例即 SPEC-013 修订五与 GOV-003 v1.6
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-02

## 一、问题陈述 {#problem}

得一引擎侧实装与金向量已验收即双跑 IDENTICAL 与禁词零命中与 156 测试绿，但判定职能的运行位仍在围堰 Python。切换批执刀即认证位换旗引擎 attractor、围堰 facet 与 tally 转兼容只读、完成档三查收口、GOV-002 判据一闭项。

## 二、关键设计 {#design}

五件。一即换旗：治理文档中判定器执行位改指引擎件 target/debug/attractor（DEC-020 关联表述、BATCH-FACE 执契与 facet 测量命令段），宪法节与启动命令零触碰，字节级 diff 自证。二即退役标注：facet 与 tally 两 CONTRACT 加退役转兼容只读标注承书简与核阅先例（采样腿 facet CLI 双模并存条款即继续可用，标注退役的是治理判定职能的强制位而非工具生命），调用册尾行。三即完成档：mergeback-attractor-completion 落 sih/event/mergeback/ 按书简与核阅先例三查（验收判据 A1-A5 引 SPEC-014、接口契约未变、回迁债清账），GOV-003 v1.7 即判定器席实例归位与判据一六席闭项，DEC-013 修订记录追加，SPEC-014 修订一即切换执行记录。四即 pk-036 出泊顺带：parkingpk 批泊档的 golden_des001_gov003 金向量重录按其出泊条件执行即重录加双跑 cmp IDENTICAL 后出泊，批次归属本批 rider。五即收口全链与 inputlog 补录。

## 三、工作清单 {#work}

- [x] 换旗与 diff 自证
- [x] 两 CONTRACT 退役标注与尾行
- [x] 完成档三查与 GOV-003 v1.7 与 DEC-013 与 SPEC-014 修订
- [x] pk-036 金向量重录出泊
- [x] 双仓收口对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 换旗 | 工程治理 | BATCH-FACE 执契段与 DEC-020 关联表述改引擎件，宪法节字节 diff 自证零碰 |
| **F-2** 退役 | 工程治理 | 两 CONTRACT 退役标注与调用册尾行在，src 零改动 |
| **F-3** 完成档 | 链上治理 | 三查完成档落 mergeback/、GOV-003 v1.7 判据一闭项表述、DEC-013 与 SPEC-014 修订在场 |
| **F-4** pk-036 | 工程治理 | 金向量重录与双跑 cmp IDENTICAL 证据在、出泊事件入链、PARKING 名册更新 |
| **F-5** 收口 | 链上治理 | 双仓 settle 归并、reconcile 四类双零、链 valid、全量入版控 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/sih/event/mergeback/mergeback-scrutinator-completion-2026-09-01.md 即完成档体例
- 必读 2：sih-engine/sih/state/plan/parkingpk-solo-results.md 即 pk-036 出泊条件
- 必读 3：sih-engine/doc/governance/PARKING-v1.md 即名册形态

## 六、约束 {#constraints}

1. 引擎 attractor 与 scrutinator 源码零改动即金向量重录只刷 fixtures 数据与断言目标（红线）
2. facet 与 tally 源码零改动（红线）
3. 宪法节与启动命令零触碰（红线）
4. 守卫在位严禁直提、链尾对表、撞锁显式 --session 即撞即停批

## 七、验收标准 {#acceptance}

- [x] F-1 至 F-5 全过

## 八、风险点 {#risks}

退役标注误伤采样腿双模即违判据四，防御即标注文显式区分判定职能强制位退役与 facet CLI 采样能力保留。金向量重录改断言目标越线即改语义，防御即只重录 content_hash 不动断言逻辑。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 切换放行令
- 链件：随批意图入当日链
- 关联：SPEC-014、deyimerge-tdd、核阅切换先例、pk-036、DEC-013

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[换旗]: 消解 即前批已消解
叩问处置[强制位]: 消解 即大白话直述即必须实走的职能位，非登记术语
叩问处置[退役标注]: 消解 即大白话直述即工具契约上登记判定职能强制位撤下转兼容只读的标注动作，措辞承核阅 CONTRACT 退役登记先例，非新造术语
叩问处置[出泊]: 消解 即承泊界既立用语即泊件满足出泊条件经人节点裁决离泊，PARKING-v1 四字组出有点承载，非新造术语

## 十一、请求写入 {#requested-writes}

- sih-tools/BATCH-FACE.md
- sih-tools/facet/CONTRACT.md
- sih-tools/facet/CALL-LOG.md
- sih-tools/tally/CONTRACT.md
- sih-tools/tally/CALL-LOG.md
- sih-engine/doc/governance/GOV-003-fullstate-course-v1.md
- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/doc/decision/013-mergeback-gate.md
- sih-engine/doc/spec/SPEC-014-attractor-mergeback-gap.md
- sih-engine/doc/decision/020-deyi-component-naming.md
- sih-engine/src/scrutinator/fixtures/golden/des-001-gov003.json
- sih-engine/sih/event/mergeback/mergeback-attractor-completion-2026-09-02.md
- sih-engine/sih/state/plan/deyimerge-switch-solo.md
- sih-engine/sih/event/plan/deyimerge-switch-solo-results.md
- sih-engine/sih/event/plan/deyimerge-switch-solo-materials/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/parking/materials/pk-036-exit.json
