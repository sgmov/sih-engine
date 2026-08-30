# gaugewiring-solo：秤星补全批即落链腿与消费侧接线

> task-packages 治理任务
> 承接：sess-zcode-260830-gaugewiring 三问意图即 2026-08-30 链事件 68e52562、用户同日开工令即秤星补全
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

gaugeimpl 批留期票两张：一引擎侧 record 落链腿即 SPEC-011 修订三钉明的读数事件类型与落链守卫，秤星至今只能算不能落；二例行读数接线即 SPEC-011 消费侧三处声明面零落地。本批补全两腿，秤星从只算半件成为算落全件。

## 二、关键设计 {#design}

三件。一引擎侧即 reading_recorded 事件类型入写前必载 details 名单、新 reading 守卫模块机械校验七字段形态即维度三枚举与主体非空与值零到一或 insufficient 标记与窗口与公式版本 ga 数字与参照时间与摘要六十四位十六进制、多字段少字段拒、建议排序自动处置三字段显式拒、scribe 增 record 入口走锁位前查回显事件哈希。二秤星侧即 record 动作复用 read 算半逐字节同值、落链经引擎 record 入口、read 输出增 history 序列回看即链上同维同主体 reading_recorded 按链序回列补 F-3。三消费侧三处即租约 open 会话档附三维读数摘要与段结算附本批主体读数、读数缺席只报不拦即零拦截承 F-6、例行读数交付全量快照触发形即 gauge record 三维全出，周期化调度归章程批另开。

## 三、工作清单 {#work}

- [x] 引擎 reading 守卫与 record 入口先红后绿
- [x] 秤星 record 动作与 history 回看测试绿
- [x] 租约两处接线与测试绿
- [x] SPEC-011 修订四与两 CONTRACT 更新
- [ ] 结果档与管线与认证入链
- [x] 双仓段结算收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 落链腿 | 工程治理 | reading 守卫红转绿、record 落链后 scribe verify 链 valid、七字段形违例全拒 |
| **F-2** 算落一致 | 工程治理 | record 与 read 同参读数值逐字节一致、read history 回看取到自身历史 |
| **F-3** 接线在场 | 工程治理 | 租约 open 档与段结算输出附读数摘要、读数件缺席时只报不拦即主功能零阻断 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/spec/SPEC-011-governance-state-reading.md 即冻结契约
- 必读 2：sih-tools/gauge/src/gauge/cli.py 即算半现状
- 必读 3：sih-engine/src/bin/scribe.rs 即入口现状

## 六、约束 {#constraints}

1. 接口冻结面零触碰
2. 零第三方依赖两侧同守
3. 词债不过夜
4. 上链前必须等绿
5. 出参全显与追加前查链

## 七、验收标准 {#acceptance}

- [x] F-1 至 F-3 全过
- [ ] 三维快照首发落链即例行读数首跑在链
- [ ] 认证入链、双仓结算收约、reconcile unrouted 零

## 八、风险点 {#risks}

租约接线使 open 输出含读数值即时间弱相依、承 issued_at 已在档先例不增不确定性、读数缺席走显式缺席注记不静默。例行读数周期化越权即本批只交付触发形、调度入章程待用户另令。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 秤星补全开工令
- 链件：sih/event/trail/2026-08-30.ndjson 即意图 68e52562
- 关联：SPEC-011 修订三期票、消费侧三处、数学三柱 LIM-007 PROB-001 ORD-002

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[秤星]: 消解 即召回面开工前采集、秤星经英文对 gauge 行间接在场不构成新缺口
叩问处置[读数]: 消解 即同上经 SPEC-011 行间接在场、本批结果档入档后召回面自补强
叩问处置[守卫]: 消解 即机械形校直述工作名承 SPEC-011 原文用词不作登记
叩问处置[序列回看]: 消解 即链上历史按链序回列的直述、F-3 验收语不作登记
叩问处置[快照]: 消解 即全量三维一次读的直述惯例语不作登记
叩问处置[算半]: 消解 即承 gaugeimpl 批算而不落半件口径的直述
叩问处置[落链腿]: 消解 即承 gaugeimpl 期票原词直述不作登记
叩问处置[读数摘要]: 消解 即三维读数概要行直述不作登记

## 十一、请求写入 {#requested-writes}

- sih-engine/src/event_stream/
- sih-engine/src/bin/scribe.rs
- sih-engine/doc/spec/SPEC-011-governance-state-reading.md
- sih-engine/sih/state/plan/gaugewiring-solo.md
- sih-engine/sih/event/plan/gaugewiring-solo-results.md
- sih-tools/gauge/
- sih-tools/lease/src/lease/
- sih-tools/lease/tests/
- sih-tools/lease/CONTRACT.md
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/
