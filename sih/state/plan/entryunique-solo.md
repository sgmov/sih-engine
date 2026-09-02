# entryunique-solo：停泊门号源唯一小批

> task-packages 治理任务
> 承接：pk-043 撞号实证与主会话审阅推荐，用户令委外执行
> 队形：单线形 solo——确定性代码与主线亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-entryunique.json 即双档零命中如实记

## 一、问题陈述 {#problem}

停泊门 enter 不查历史已用号即已出泊号可复用。2026-09-03 pk-043 撞号实证即 intanchor-solo 同日进出件与温度直觉件重号，单一名册续号不重号规约无机械承重。编号分配读名册投影而投影滞后于链，撞号必然复发。词号源唯一已登记（parkreplay-solo 批），实装待人节点裁即本批承接。

## 二、关键设计 {#design}

park enter 增第三道门号源唯一拒：判据即重放面（链目录全量承 parkreplay-solo 修订一）内该 entry_id 出现过任何 parking_entered 即拒，无论当前在泊或已出泊。exit 两门语义不变，无新增事件类型。SPEC-006 park 入口节修订二。现存链历史双 enter（pk-043 撞号改正案）不受影响即门只拦新 enter 请求，verify 与 query 与历史链零回溯。

单测四件：新号过、在泊重入拒（既有回归）、已出泊号复用拒、跨天已用号拒。红绿：修复前复用已出泊号被接受即红态在档，修复后拒即绿态在档。

## 三、工作清单 {#work}

- [ ] 号源唯一拒实装于 event_stream park 模块加 scribe 接线
- [ ] 单测四件加既有套件回归全绿
- [ ] SPEC-006 修订二走化格核阅检词三步
- [ ] 红绿两态读数在档
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 红绿** | 工程 | 测试链目录已出泊号复用，修前 appended 退出码零，修后号源唯一拒退出码一，两态在档 |
| **F-2 既有门回归** | 工程 | 重入拒与无主出拒与跨天出泊既有单测全绿，cargo test 除先在三金向量漂移外全绿 |
| **F-3 语义窄改** | 工程 | 五子命令输出与退出码旧新对表零变化，verify 单链零回溯 |
| **F-4 写入仅 allow** | 治理 | 本批写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/src/event_stream/park.rs 即现行门与重放面
- 必读 2：sih-engine/doc/spec/SPEC-006-scribe-mergeback-gap.md 即修订一现行文
- 必读 3：sih-tools/BATCH-FACE.md 调用面与坑位
- 必读 4：sih-engine/sih/event/plan/parkreplay-solo-results.md 先例

## 六、约束 {#constraints}

1. 不新增事件类型不改 exit 门，上链遇锁即等待
2. 词债不过夜，findings 亲读
3. 生产链只作追加面，红绿用测试链目录
4. 历史链零回溯即不重写任何既有事件

## 七、请求写入 {#requested-writes}

- sih-engine/src/event_stream/park.rs
- sih-engine/src/bin/scribe.rs
- sih-engine/doc/spec/SPEC-006-scribe-mergeback-gap.md
- sih-engine/sih/state/plan/entryunique-solo.md
- sih-engine/sih/event/plan/entryunique-solo-results.md
- sih-engine/sih/event/plan/entryunique-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] cargo test 既有套件回归全绿（先在三金向量漂移披露在案）
- [ ] SPEC-006 修订二三步过
- [ ] 认证入链，双仓段结算收约，对表读数在档
