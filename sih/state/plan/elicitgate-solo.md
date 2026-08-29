# elicitgate-solo：叩问闸位接线批

> task-packages 治理任务
> 承接：sess-zcode-260830-elicitgate 三问意图即 2026-08-30 链事件 dd146068、用户同日令即同意闸位接线批
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

叩问工具已实装而在闸位缺席：三问验收步只有核阅 ask3 包无消化闸，信号无从产出亦无从查收，skill 壳未注册即工具无入口语义。SPEC-012 实装期参数破冻未留痕即 --words 与 suspend 与候选参数三件超冻结面。

## 二、关键设计 {#design}

三件。一 intent-refine 修订九：检索加载步增叩问 check 出信号件随批材料、机械验收步增 digest 过闸即信号未消化不得上链、零信号即零处置直过。二 SPEC-012 升 v1.1：suspend 操作与 --words 与 --conflict-words 与 --history-face 参数破冻入修订记录、--input 收紧为显式词表承分词不做。三新壳 sihankor-elicit 注册：权威源 sih-engine/sih/state/skills 与 .agents 投影双实体文件、壳只载触发语义与调用形、权威在 sih-tools/elicit/CONTRACT.md。本批任务包即首件狗粮：check 出信号、包内逐信号叩问处置、digest 过闸后认证。

## 三、工作清单 {#work}

- [ ] skill 修订九落笔
- [ ] SPEC-012 v1.1 修订留痕
- [ ] 新壳两件落位
- [ ] 狗粮环即 check 出信号加包内处置加 digest 过闸
- [ ] 认证与双仓段结算收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 闸位成文 | 工程治理 | 修订九含 check 与 digest 两步接线与零信号直过条款、SPEC-012 v1.1 破冻留痕 |
| **F-2** 壳注册 | 工程治理 | sihankor-elicit 壳权威源与投影双件在位、触发语义与调用形齐、权威指 CONTRACT |
| **F-3** 狗粮闭环 | 链上治理 | 本批包经 check 出信号与包内叩问处置逐信号在案与 digest 退出码零、认证随绿入链 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/elicit/CONTRACT.md 即工具权威契约
- 必读 2：sih-engine/sih/state/skills/sihankor-intent-refine/SKILL.md 即五步制现形

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链前必须等绿
4. 叩问本体零改动
5. 出参全显与追加前查链

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 结果档落 sih/event/plan/elicitgate-solo-results.md，认证入链，双仓段结算收约

## 八、风险点 {#risks}

闸位误伤即泛词假信号致批卡死，收窄即 conflict-words 显式与零信号直过条款。双实体投影漂移即权威源为正如 .agents 投影过期以权威源为准随批同步。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 同意令
- 链件：sih/event/trail/2026-08-30.ndjson 即意图 dd146068
- 关联：SPEC-012 冻结接口、askloop 修订七、elicitimpl 实装批

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[接线]: 消解 即描述语指消费侧连接动作不立组件名走懒波
叩问处置[狗粮]: 消解 即描述语指自用测试惯例属行业泛称不立名
叩问处置[闸位]: 消解 即描述语指验收步闸位置不立名

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/skills/sihankor-intent-refine/SKILL.md
- sih-engine/sih/state/skills/sihankor-elicit/SKILL.md
- .agents/skills/sihankor-elicit/SKILL.md
- sih-engine/doc/spec/SPEC-012-elicit-contract.md
- sih-engine/sih/state/plan/elicitgate-solo.md
- sih-engine/sih/event/plan/elicitgate-solo-results.md
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
