# regconf-sdd-solo：路择登记冲突谓词扩域落差规格批

> task-packages 治理任务
> 承接：sess-zcode-260828-regconfsdd 三问意图即 2026-08-28 链事件 5d6b6f2d、用户同日令批即「路择登记冲突谓词扩域 SDD 批准」
> 队形：单线形 solo——主线亲写零子代理，规格批默认形
> 日期：2026-08-28

## 一、问题陈述 {#problem}

路择登记冲突告警现面是轮记录回执自包含式即结论 kind 为 establishes_term 且其 term 精确出现在任一回执 flagged_terms 才产告警。四处落差：告警形态无 id 与路径不可归件、结论种类硬编码单值、匹配唯全串相等即死档词嵌于新名一类包含形态漏检、回执参与面无界即任意工具回执的 flagged_terms 皆参查。扩域行为面未钉即实现无据，先落落差规格后实现。

## 二、关键设计 {#design}

SDD 承 SPEC-008 与 SPEC-009 先例即先钉落差后实现。规格钉扩域四件：告警归件即冲突告警补 id 与路径两字段；匹配面数据化即 match 参数 exact 与 contains 两枚举省 exact；结论面数据化即 establish_kinds 表省 establishes_term 单值；回执面数据化即 receipt_tools 表省全回执。三边界不破：空腹即新语义全在包参数零引擎语言知识、回执自包含即零跨包耦合检词登记册不进路择、告警位恒不参与定路即判在别处。缺省形输出逐字节零漂移即向后兼容为硬约束，告警形态变更以工具版本显式承载。

## 三、工作清单 {#work}

- [ ] SPEC-010 落盘即现面盘点与落差四件与边界排除与测试计划 F-1 至 F-6
- [ ] 管线三件零违例即化格核阅检词对 SPEC-010 与本包
- [ ] 认证三笔经引擎件入引擎链
- [ ] 结果档落位、双仓段结算收约、免参对表退出码零

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 规格在档实现零动 | 工程治理 | SPEC-010 在 sih-engine/doc/spec 在档，selector 源码与测试 git diff 空即实现归后续批 |
| **F-2** 管线三件零违例 | 链上治理 | 化格核阅检词三件对两文档退出码零，报告落 scribe reports 位 |
| **F-3** 认证入链对表 | 链上治理 | 认证三笔在引擎链当日 trail 验链 valid，双仓免参对表退出码零 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/selector/CONTRACT.md 即工具侧现状权威含判定与告警节
- 必读 2：sih-engine/doc/spec/SPEC-009-parser-implementation-gap.md 即 SDD 结构先例

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链前必须等绿
4. 缺省形逐字节零漂移即向后兼容为硬约束
5. 期票不冒充完备即规格钉落差非实现承诺，实现判据 F-1 至 F-6 面向后续批

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 结果档落 sih/event/plan/regconf-sdd-solo-results.md，认证三笔入链，双仓段结算收约

## 八、风险点 {#risks}

规格批实现零动即风险在规格自身完备度。落差四件若漏现存语义面即后续实现批返工，以现面盘点逐字段对表 predicates.py 收口即冲突检测全路径只此一函数。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。旧偏离条款不再适用即选形即声明。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-28 批准令
- 链件：sih/event/trail/2026-08-28.ndjson 即意图 5d6b6f2d
- 后续：实现批随 SPEC-010 测试计划 F-1 至 F-6 先红后绿，队形届时另选

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/spec/SPEC-010-registry-conflict-expansion-gap.md
- sih-engine/sih/state/plan/regconf-sdd-solo.md
- sih-engine/sih/event/plan/regconf-sdd-solo-results.md
- sih-engine/sih/event/trail/2026-08-28.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
