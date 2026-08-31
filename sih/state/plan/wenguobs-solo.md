# wenguobs-solo：温故观测面批

> task-packages 治理任务
> 承接：用户 2026-08-31 令即温故观测面立项（关键词面加零命中账）、主会评估在案即缺的是文轴非向量召回
> 队形：单线形 solo——执行代理亲写零子代理、主会验收位
> 日期：2026-08-31

## 一、问题陈述 {#problem}

温故主题轴经寻址桥按名查询即匹配条目名，字面词出现在档文正文而未作条目名时即零命中。主会 2026-08-31 实测六词落空在案即令牌、use 边、边账、期票、句读、级联，六词皆真实存在于档案文档正文。缺的是文轴非语义检索，且落空无账即词汇覆盖缺口不可见。

## 二、关键设计 {#design}

三件。一文轴即 recall 增 --word 可重复参，对寻址索引条目之 text 做逐字子串匹配，产同款七字段切面行即 axis 取 word，与名轴 --topic 并存互补即名轴查标识文轴查正文；索引 text 若为截断摘录须先验证完备性即取一件已知含词档复测命中，截断即如实声明边界不虚构完备。二零命中账即 recall 增 --miss-log 可选参，带参时对每个产零行的查询词追加一行 ndjson 即字段 at 加 axis 加 word 加 rows 零值四件，append-only 不去重即重复查询重复记行如实；缺参零写即温故默认只读纪律保持，双跑逐字节一致在缺参形态下不变，带参形态双跑即双份账行属正确账义非违例。三词汇自喂被文轴吸收即任何登记词汇凡现于档文即文轴可达，名轴不另喂，此设计裁定入档。GOV-003 零改即本批不升向界版本。

## 三、工作清单 {#work}

- [ ] 文轴实现与索引 text 完备性验证，先红后绿
- [ ] 零命中账实现与缺参零写保持
- [ ] 六落空词真实复测全命中
- [ ] SPEC-008 修订与管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 文轴 | 工程治理 | --word 逐字子串扫档文、主会六落空词全命中且 ref 可回查原档、名轴 --topic 行为零变、索引截断若有即声明 |
| **F-2** 零命中账 | 工程治理 | --miss-log 带参才写、逐零行查询词一行四字段、缺参形态双跑逐字节一致、带参形态重复查询重复记行 |
| **F-3** 收口 | 链上治理 | SPEC-008 修订在档、测试先红后绿全绿、管线认证入链、双仓提交、对表净增零、GOV-003 零改 |

## 五、必读文件 {#read}

- 必读 1：src/retriever/mod.rs 即 recall 核与主题轴经寻址桥实形
- 必读 2：src/retriever/locator_bridge.rs 即 query_word_entries 按名查询与 LocatorEntry 之 text 字段
- 必读 3：doc/spec/SPEC-008-project-memory-implementation-gap.md 即温故接口规格与修订位

## 六、约束 {#constraints}

1. 温故默认只读纪律保持即零写面仅 --miss-log 带参一径
2. 名轴语义零改即 --topic 仍按名查询
3. 文轴匹配为逐字子串即不做分词不做模糊不做语义
4. 上链前必须等绿、findings 亲读、禁管道掩退出码
5. 范围闸若拦即零提交收约改包重开；一切待提交件先进工地从工地提交，禁主树直写与收约后手工归并
6. 与并行批撞锁即报不绕行

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过即主会复验

## 八、风险点 {#risks}

寻址索引条目 text 若系截断摘录即长档尾部词漏配，先验证后实现，截断即如实声明承拦多不拦漏边界；--word 与 --topic 同词并用时两轴行并出即 axis 字段区分不混。

## 九、队形声明 {#formation}

单线形即执行代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 令与主会评估
- 链件：sih/event/trail/<当日>.ndjson
- 关联：SPEC-007 与 SPEC-008、寻址桥、主会零命中实录六词

## 十一、请求写入 {#requested-writes}

- sih-engine/src/retriever/
- sih-engine/src/bin/retriever.rs
- sih-engine/src/lib.rs
- sih-engine/Cargo.toml
- sih-engine/doc/spec/SPEC-008-project-memory-implementation-gap.md
- sih-engine/sih/state/plan/wenguobs-solo.md
- sih-engine/sih/event/plan/wenguobs-solo-results.md
- sih-engine/sih/event/plan/wenguobs-solo-resolo-results.md
- sih-engine/sih/state/plan/wenguobs-trail/
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[文轴]: 消解 即工作名直述即按正文逐字检索之轴、不做登记
叩问处置[零命中账]: 消解 即工作名直述即落空词记账面、不做登记
