# pk035impl-solo：席位身份入正身实现批

> task-packages 治理任务
> 承接：sess-zcode-260830-pk035impl 三问意图即 2026-08-30 链事件 01c1d9e4、用户同日开工令即 pk-035 出泊、得一四通道机器终签即设计输入
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

席位串是自由文本已出真漂移即标定账本同席位两日两形，材料无可回验锚。用户裁定席位立身份走正身、泊内草案成稿、得一四通道终签在案即 3706bac0 17250813 86abafbb 01bf2697。本批出泊实装四落点，正身本体零改。

## 二、关键设计 {#design}

四件。一 facet 计分腿即 measure score 增 --identity-report 必挂参、读正身报告身份哈希与规范席位串即三段式大小写敏感、计分材料 version 2 强制携带双字段、缺报即拒并注记。二探针腿即 temp_probe score 增同参、账本行增 identity_hash 列、漂移配对哈希优先即双方带哈希按哈希、任一缺席回退现行席位串配对。三执契腿即 assemble 透传哈希、R5 双带哈希即比对、哈希异按漂移挂起走优先级映射、任一缺席回退现行为即旧材料与旧基线重放逐字节同判。四契约腿即 tally CONTRACT 修订与 identity CONTRACT 修订留痕与 facet-measure SKILL 修订加投影同步、正身本体零改仅契约记外部携带界。

## 三、工作清单 {#work}

- [ ] facet 材料腿先红后绿
- [ ] 探针腿与执契腿测试绿
- [ ] 两 CONTRACT 与 SKILL 修订加投影
- [ ] pk-035 出泊 promoted 与名录更新
- [ ] 结果档与管线与认证入链
- [ ] 双仓段结算收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 强制携带 | 工程治理 | 计分材料与账本行带 identity_hash 与规范串、缺报拒、facet 测试绿 |
| **F-2** 哈希优先 | 工程治理 | 漂移与 R5 双带哈希按哈希判、哈希异挂起、回退路径旧件重放同判、tally 测试绿 |
| **F-3** 界与出泊 | 链上治理 | 正身组件代码零改即 git diff 空、pk-035 exit promoted 入链、名录五项、认证在链、reconcile unrouted 零 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/proposition/DES/pk035-ch1-identity 即得一裁决材料四件
- 必读 2：sih-tools/tally/src/tally/cli.py 即 R5 现行块
- 必读 3：sih-tools/facet/probes/temp_probe.py 即标定现行

## 六、约束 {#constraints}

1. 正身本体零改
2. 旧件零改写即回退兼容
3. 词债不过夜
4. 上链前必须等绿
5. 并行批 fmtc2b-solo 避让即共享件收约按备份还原模式

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链、双仓结算收约、reconcile unrouted 零

## 八、风险点 {#risks}

并行批在跑即共享日链与 PARKING 名录可能撞归并、按既定备份还原模式处置不绕行。回退路径若有误伤即旧件重放测试先行钉死再动 R5。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 开工令
- 链件：sih/event/trail/2026-08-30.ndjson 即意图 01c1d9e4
- 关联：得一终签四件、pk-035 泊件、identity CONTRACT、facet 单席位规格

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[席位]: 消解 即召回面开工前采集零命中如实记、本批结果档入档后自补强
叩问处置[正身]: 消解 即同上、正身为已立名零命中因检索面未及新档
叩问处置[出泊]: 消解 即泊界既用惯例语直述不作登记
叩问处置[投影]: 消解 即技能实体副本机制直述不作登记
叩问处置[挂起]: 消解 即执契三态既用值直述不作登记
叩问处置[规范席位串]: 消解 即框架:模型:版本三段式直述工作名不作登记
叩问处置[身份哈希]: 消解 即正身报告哈希直述不作登记

## 十一、请求写入 {#requested-writes}

- sih-tools/facet/contract_mode.py
- sih-tools/facet/measure.py
- sih-tools/facet/tests/
- sih-tools/facet/probes/temp_probe.py
- sih-tools/tally/src/tally/
- sih-tools/tally/tests/
- sih-tools/tally/CONTRACT.md
- sih-tools/tally/pyproject.toml
- sih-tools/identity/CALL-LOG.md
- sih-tools/identity/CONTRACT.md
- sih-engine/sih/state/skills/sihankor-facet-measure/SKILL.md
- sih-engine/sih/state/plan/pk035impl-solo.md
- sih-engine/sih/event/plan/pk035impl-solo-results.md
- sih-tools/parking/materials/pk-035-exit.json
- sih-tools/PARKING-v1.md
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/
