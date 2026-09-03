# gatecap-solo：数学管线封顶批（判定性常数裸奔违规类 + 推导档机械复核）

> task-packages 治理任务
> 承接：mathpipe-full-program-v1.md 收尾批节即闸门自维持——不变量从此由闸门维持不靠惯例，这是「串联」的持久化；批四起六件已收官即本批为程序封顶件
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械即机械链全绿即放行
> 队形：单线形 solo，**单飞**（改闸门的批不与他批并发，开工前 lease status 验零活跃）；锁纪律机器位；scribe gated 写入带 --sessions
> 日期：2026-09-04（包立 2026-09-03，实日以本行为准：trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-*）｜ 温故检索：materials/recall-gatecap.json 双档零命中如实记 ｜ pk-045 参与者

## 一、问题陈述 {#problem}

六件接线毕后，判定性常数的载体引用与推导档仍靠批自觉，过闸无机械强制。程序档明言收尾批即核阅规则包新增「判定性常数裸奔」违规类（无载体引用加推导即违规）与 tally 新增推导档机械复核规则。

## 二、关键设计 {#design}

1. **核阅新违规类**：des-001 规则包增规则即域内文档声明判定性常数（阈值、判据、裁决规则形）而无同文档或邻近登记面的载体引用与推导档指针即违规 C 系（码位按包既有序列续），净目标零发现、脏目标即故意裸奔样文命中。规则为纯数据入包，fixtures 双目标（净脏）入 corpus。
2. **tally 复核规则**：tally 核对族增一条即裁决材料的推导档指针缺失或所指文件不存在即挂起四值处置，先红后绿。
3. 存量对表：des-001 域内既有文档全量过新规读数，存量命中即如实申报逐件清单（属申报非违规，处置归人节点泊界或勘误）。
4. CONTRACT 修订与 USAGE 同步；金向量即新规净脏双场景双跑。

## 三、F 表 {#falsifiable}

| F | 判据 |
|---|---|
| F-1 新规拦裸奔 | 脏目标命中码位且净目标零发现，双跑一致 |
| F-2 tally 复核 | 推导档缺指或所指不存在即挂起，先红后绿 |
| F-3 存量如实 | 域内全量读数与命中清单在档 |
| F-4 兼容 | 核阅与 tally 既有测试全绿 |

## 四、必读 {#read}

des-001 规则包与 corpus fixtures；sih-tools/tally/CONTRACT.md；mathpipe-full-program-v1.md 收尾批节；BATCH-FACE.md

## 五、请求写入 {#requested-writes}

sih-tools/scrutinator/packs/ ｜ sih-tools/scrutinator/fixtures/ ｜ sih-tools/tally/ ｜ sih-math/docs/ ｜ sih-engine/sih/state/plan/gatecap-solo.md ｜ sih-engine/sih/event/plan/gatecap-solo-results.md ｜ sih-engine/sih/event/plan/gatecap-solo-materials/ ｜ sih-engine/sih/event/trail/<实日>.ndjson ｜ sih-tools/scribe/reports/ ｜ sih-tools/scribe/CALL-LOG.md ｜ sih-tools/lease/CALL-LOG.md ｜ sih-tools/meter/counts/ ｜ sih-tools/nomenclator/packs/core/

## 六、验收 {#acceptance}

F-1 至 F-4 全过；存量命中清单在档；单飞零并发如实记；认证入链双仓结算收约对表在档。
