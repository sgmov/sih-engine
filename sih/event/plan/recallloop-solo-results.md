# recallloop-solo 结果档

- 日期：2026-09-04，队形：单线形 solo，会话：sess-zcode-260904-recallloop（租约 f16265b471cfca41）
- 载体：PROB-014 期望信息增益 + PROB-011 估计与置信区间（mapping.md:204 与 :201 实查，条目磁盘实存，m-carrierwave 终签波）

## 完成度表

| 件 | 状态 |
| --- | --- |
| 度量件 metrics.py | ✅ 三通道通道级分列，纯函数只读消费 recall 计划，同输入逐字节同输出 |
| 命中率读数 | ✅ 威尔逊覆盖区间出具，z 常数三件随档登记，零候选不出点估计 |
| 信息增益读数 | ✅ 候选收窄运算化 log2(N/k) 比特，先验来源随档登记 |
| 通道退化判据 | ✅ 词面与图零增益（哑火或全并）判退化，骨架结构缺件判退化，逐通道判据依据全出 |
| 推导档 | ✅ sih-math/docs/recallloop-derivation-2026-09-04.md，双载体分面七节 |
| 金向量 | ✅ 四场景冻结 fixtures/golden，双跑逐字节一致，冻结件零绝对路径携重放寻径约定 |
| 测试 | ✅ 新测十一用例先红后绿，recall 既有六用例零回归全绿 |
| 真仓冒烟 | ✅ sih-math 查「锁序」union 7，词面 1/7 IG 2.8074 比特，图 6/7 IG 0.2224 比特，骨架 6/6，degraded 假 |

## F 验证表

| F | 结果 |
| --- | --- |
| F-1 四件套 | ✅ 载体引用 mapping 实取 + 推导档 + 代码接线 + 金向量 |
| F-2 零行为变更 | ✅ recall.py 与 selftest.py 与 checkcite.py 与 verify_triggers.py 零修改，既有测试零回归，新测先红后绿 |
| F-3 度量可解释 | ✅ 输出载通道与命中数与总量与命中率含区间与信息增益比特数与退化态与判据依据，不裸报布尔 |
| F-4 写入仅 allow | ✅ 锁路径全集内施工，exclusive 五路 append 短持即取即放 |

## 认证清单

- 链 2026-09-04.ndjson intent 一笔：78e2cfad（ask3 三锚 6f1c0917，双门第一门 exit 0 零违规、repeater status ok，digest passed covered 4）
- 认证四笔（meter 包裹 append 主树活链）：管线 6e5a262e、推导档 ab95e0aa、金向量 909dd260、变更件 dfe872e6
- 闸三：--session 租约会话加 --sessions 台账参数双带，逐笔过

## 越线与误差申报

- 夹具初版漏建 probability 与 algebra 目录两处 FileNotFoundError 即修，度量断言期望字典缺退化键一处即修，均在绿相前。
- 核阅 des-001 域外 exit 2 两笔如实记（推导档 sih-math/docs 与结果档 sih/event/plan 均不在 des-001 域 sih-engine/doc 内），不属违规。
- recall.py 全批零修改，度量面零改动召回判定与排序。
- 越线自报：CALL-LOG 两笔（scribe 与 lease）写入时锁命令 --locks 与 --ledger 误传 lease/lease/ledger 双写路径致四锁四解锁全数未生效，写入在无锁态发生，事后对账确认撞锁窗口内无他会话持同面锁（gchart 仅持 trail 与 reports 与 meter/counts 的 append 共存位），未造成实害；教训与 meter 无 --quiet 同族即静默失败须亲读退出码，管道掩码禁令扩及锁命令。
- 本档认证清单节为认证后回填，管线三步读数取自回填前档身，回填后化格与检词复跑零改动在收口读数。

## 冲突样本节

- sih-math/docs/recallloop-derivation-2026-09-04.md exclusive 首取撞 elicitwire-solo（31bc9b8a1b0f9e25）目录级 sih-math/docs 独占锁，locked_elsewhere 有限重试十次逐次计数未得，按不绕行让位处理，施工面先行；推导档落笔前再试一次获锁成功（对方收约释放），零抢占零绕行。
- 与 gchart-solo（4db66354ad676374）并行：其 append 持 trail 与 scribe/reports 与 meter/counts，与本批 append 短持共存零撞锁；其 exclusive sih-tools/gauge 与本批 wikirecall 施工面互斥不相交。
- 领取登记闸：开工前 claim 一笔 ttl 240（claimant sess-zcode-260904-recallloop），lease open 闸一联动警示行载本批自领记录，如实记非异常。
- 收约阶段共享面冲突两起如实记：其一 close 首试被 tools 主树锁台账 merge_diverge 拦（共享运行面脏），按既定通道主树 bypass 快照两笔收编（36b6dedd 台账与 lease CALL-LOG、9c60d987 meter 计数四笔与 scribe CALL-LOG，两笔均 bypass 登记）后过预检；其二 merge 阶段主树未跟踪同名批件挡归并，按备份让位归并对表法四步（备份十七件、让位、close 归并、cmp 逐件 IDENTICAL）处置。批期 scribe/CALL-LOG.md 与 meter 计数曾被并发批回退抹行，分支副本携超集归并即自愈，批期共享面写入时序以链与分支为准。
- close 尾段 sih-math 首仓归并成、tools 与 engine 两仓 merge_failed 后让位重跑全过，failed 空数组 session revoked；gchart-solo 链尾随同步（475832c）已把本批四笔认证收进 engine 主树 trail，双批并集超集共笔。

## 收口读数

批期读数：intent 上链为链第 53 笔（批前 47 事件起链，并行批共笔在先），verify valid；金向量四场景 run1 = run2 = 冻结件 IDENTICAL；checkcite 对表通过（推导档与结果档引用全落书单图闭包）。

收约回填：三仓 settle 段一提交号 sih-tools d35d11ad、sih-math 1f9446e、sih-engine 5474e90（cert 均 dfe872e6）；归并号 sih-math 021887a、sih-tools 5e55381c、sih-engine e3ef748，三工地拆除，会话 f16265b471cfca41 revoked；reconcile 批内基线对表（tools@20c37119、engine@0017372、math@8d0bce2）unrouted 零 cert_missing 零 unbypassed 零全净；当日链 verify valid 66 事件（首哈希 05a8a75e 末哈希 83aa35d3）；回填后本档化格与检词复跑零改动。

## 队形验证

单线形 solo：主线亲写，零子代理派单，全程无 Agent 调用。
