# scrutmerge-switch-solo：核阅融回切换批

> task-packages 治理任务
> 承接：用户 2026-09-01 批准通行令、DEC-013 融回门第三步、SPEC-013 边界节即切换批执行项、tdfix2 后切换闸条件全齐即主会 2026-08-31 核验
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-01

## 一、问题陈述 {#problem}

核阅融回三步曲前两步已毕即 SPEC-013 规格与引擎侧实装加测试守卫，但 T6 管线核阅腿的强制校验位仍指工具件 sih-tools/scrutinator。切换批执刀即认证位换旗引擎件、工具件退役转兼容只读、双跑判据机械证等价、融回完成档三查收口。

## 二、关键设计 {#design}

主体四件。一即换旗：AGENTS.md 工具层静态审计节三处核阅位即概览行与核阅调用行与文件索引行改指引擎件 target/debug/scrutinator，BATCH-FACE.md 核阅腿命令同步改引擎路径；本批自身 T6 文档核阅即以引擎件跑即切换狗粮位。二即双跑判据：cargo build 加 cargo test 全绿含金向量六件逐字节断言与 101 测试；引擎件与工具件对三真实目标即 SPEC-013 与 DEC-020 与 GOV-002 各双跑，cmp 零差为切换成立判据，版本戳例外若触发即如实记并停批报主会。三即退役：工具件 CONTRACT 加退役标注承书简先例即转兼容只读、调用册落尾行、三包留档不删、src 与 tests 与 packs 零改动。四即完成档：mergeback-scrutinator-completion-2026-09-01.md 落三查对表，GOV-003 v1.6 追记即参验席实例归位，SPEC-013 修订三即切换执行记录，DEC-013 修订随体例。

顺带 rider 四件。一即 tdfix2 材料入库即 scribe/reports 下 2026-08-31-tdfix2 前缀与 ask3-tdfix2 前缀全部十七件 untracked 入册，engine 侧 scrutmerge-tdfix-solo.md 包档一并入册。二即版控尾巴收编即 engine 链尾 2026-09-01.ndjson 与 cmdface-solo.md 叩问补笔、tools 三件 r2 报告与计数件。三即 tdfix 结果档路标段更正为正式收口记录即 2026-08-31 主会代偿补链实录：前任接手代理撞锁跳链、主会重跑双门绿后补意图 993abd97 与十九认证入 31 链即当时 118 事件 valid、recall-face 件如实拒认非报告形。四即 scribe 调用册收尾笔即核验 viewreg 行已付与否，未付即补。

另立新坑位两条入 BATCH-FACE 即引擎源码批 settle 前必 cargo build 加实跑行为探针（scribe 陈旧二进制课）、核阅腿改引擎件后工具件仅复验位。

## 三、工作清单 {#work}

- [ ] cargo build 与 cargo test 全绿与三目标双跑 cmp 零差
- [ ] AGENTS 三处核阅位换旗与 BATCH-FACE 核阅腿与新坑位
- [ ] 工具件退役标注与调用册尾行
- [ ] 完成档三查与 GOV-003 v1.6 与 SPEC-013 修订三
- [ ] rider 四件与 inputlog 两笔补录
- [ ] 本批管线以引擎件核阅、认证入链、双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 换旗 | 工程治理 | AGENTS 三处与 BATCH-FACE 核阅腿皆指引擎件，本批 T6 文档核阅实跑引擎件零违规即 des-001 域内 |
| **F-2** 双跑 | 工程治理 | cargo test 全绿含金向量、三目标引擎件对工具件 cmp 零差、报告入材料件 |
| **F-3** 退役与完成档 | 链上治理 | CONTRACT 退役标注与尾行在、三查完成档落 mergeback/、GOV-003 v1.6、SPEC-013 修订三 |
| **F-4** rider | 链上治理 | tdfix2 十七件与包档入册、版控尾巴五件收编、路标段正式收口、scribe 收尾笔核验或补笔、BATCH-FACE 两坑位 |
| **F-5** 收口 | 链上治理 | 双仓 settle、链 valid、reconcile 零增、unrouted 不增 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md 即边界节与验收判据
- 必读 2：sih-engine/sih/event/mergeback/mergeback-scribe-completion-2026-08-27.md 即完成档体例先例
- 必读 3：sih-engine/doc/decision/013-mergeback-gate.md 即三步曲与修订体例

## 六、约束 {#constraints}

1. 引擎源码零改动即只 build 与跑测试（红线）
2. 工具件 scrutinator 只改 CONTRACT 与 CALL-LOG，src 与 tests 与 packs 零改动（红线）
3. 金向量冻结即任何字段漂移判负（红线）
4. AGENTS.md 写经 lease --allow AGENTS.md，宪法节与启动命令零触碰（红线）
5. 上链前必须等绿、findings 亲读、禁管道掩退出码
6. 撞锁即报不绕行即显式带 --session

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过

## 八、风险点 {#risks}

双跑版本戳例外触发即引擎件与工具件版本号不同步，防御即先对表两侧版本再跑 cmp，不同步即停批报主会不硬切。AGENTS 换旗误伤宪法节，防御即改动前后字节级 diff 备查仅限三行核阅位。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-01 批准通行令
- 链件：随批意图入当日链
- 关联：scrutmerge-sdd 与 tdd 与 tfix 系前批、DEC-013、SPEC-013、书简融回先例

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[换旗]: 消解 即大白话直述即认证位切换，非登记术语
叩问处置[狗粮位]: 消解 即大白话直述即自产自用位，非登记术语
叩问处置[收编]: 消解 即大白话直述即入册，非登记术语
叩问处置[代偿]: 消解 即大白话直述即主会代为补链，非登记术语
叩问处置[核阅融回]: 消解 即大白话直述即核阅工具融回引擎的过程描述，非登记术语
叩问处置[切换执行]: 消解 即大白话直述即认证位切换的执行动作，非登记术语
叩问处置[工具件退役]: 消解 即大白话直述即工具外切流程的兼容只读态，非登记术语
叩问处置[路标收口]: 消解 即大白话直述即迁链路标由占位转正式收口记录，非登记术语
叩问处置[封窗]: 消解 即 lease 既立术语承 sealwin2 先例
叩问处置[追认档]: 消解 即大白话直述即封窗明细档，非登记术语

## 十一、请求写入 {#requested-writes}

- AGENTS.md
- sih-engine/doc/decision/013-mergeback-gate.md
- sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md
- sih-engine/doc/governance/GOV-003-fullstate-course-v1.md
- sih-engine/sih/event/mergeback/mergeback-scrutinator-completion-2026-09-01.md
- sih-engine/sih/event/plan/scrutmerge-tdfix-solo-results.md
- sih-engine/sih/state/plan/scrutmerge-tdfix-solo.md
- sih-engine/sih/state/plan/cmdface-solo.md
- sih-engine/sih/state/plan/scrutmerge-switch-solo.md
- sih-engine/sih/event/plan/scrutmerge-switch-solo-results.md
- sih-engine/sih/event/plan/scrutmerge-switch-solo-materials/
- sih-engine/sih/event/trail/2026-09-01.ndjson
- sih-engine/sih/event/inputlog/2026-09-01.ndjson
- sih-tools/scrutinator/CONTRACT.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/BATCH-FACE.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scribe/reports/2026-08-31-tdfix2-*
- sih-tools/scribe/reports/2026-08-31-ask3-tdfix2-*
- sih-tools/scribe/reports/2026-09-01-ask3-cmdface-r2-record.json
- sih-tools/scribe/reports/2026-09-01-ask3-cmdface-r2-validation.json
- sih-tools/scribe/reports/2026-09-01-cmdface-r2-elicit-signals.ndjson
- sih-tools/scribe/reports/
- sih-tools/meter/counts/2026-09-01.ndjson
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
