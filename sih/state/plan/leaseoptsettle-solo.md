# leaseoptsettle-solo：租约线结算批

> leaseoptsettle-solo 租约优化线结算执行批
> 令源：用户 2026-09-05「收工。继续收工」
> 范式：T6-D 偏离单线 solo——零子代理确定性结算批

## 一、问题陈述 {#problem}

- **问题 1**：leaseopt 线六批全收口（audit、fixguard、precheck、lockdb、lockqueue、ledger-repair）加收尾批 pk-045 出泊落链，线级验收逐条达成，缺正式结算单落档。
- **问题 2**：批六 watch 视图批未开工，为线唯一未建件，承用户裁定归主线泊位承载，须入泊（pk-059）携 pk-045 三类销账转承与 pk-057 随批六承载指针。
- **问题 3**：GOV-002 v2 退出标准首条即 leaseopt 线级验收，三项判据本日全部机械达成，按 v1.6 先例追记换版 v2.1。

## 二、结算动作 {#design}

- 结算单 SETTLEMENT-LEASEOPT-2026-09-05.md 落 doc/governance/，承 SETTLEMENT-V1-2026-09-02 先例形：逐批证据表、用户痛点对照、线级验收逐条、泊界复检必经栏、遗留与指针、链证、签署。
- pk-059 进泊：批六视图批泊位，context 载病灶五与病灶二残留与 empty-detail 诊断面与 pk-045 三类转承与 pk-057 随批六指针。
- leaseopt-line-v1.md 关线追记：线状态改已结算，指针结算单。
- GOV-002 v2.1：退出标准首条达成追记与证据指针，其余零字节改动，版本节追记 v2.1。

## 三、工作清单 {#work}

- [ ] settle-01：结算单起草与逐批证据指针亲核
- [ ] settle-02：pk-059 泊件落工地并 scribe park 裸调落链 grep 验证
- [ ] settle-03：线总纲关线追记与 GOV-002 v2.1 追记
- [ ] settle-04：管线三步双文档与认证上链
- [ ] settle-05：双仓 settle 与 close 与 reconcile 与 verify 与主树 des-001 复跑

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 结算单在档 | doc/governance/SETTLEMENT-LEASEOPT-2026-09-05.md 落盘，六批证据指针逐条亲核在档 |
| **F-2** | 批六入泊 | 链上 parking_entered pk-059 在档，context 载三类转承与 pk-057 指针 |
| **F-3** | 换版追记 | GOV-002 退出标准首条追记与 v2.1 版本节在档，其余节零字节改动 |
| **F-4** | 关线追记 | leaseopt-line-v1.md 线状态已结算追记在档 |
| **F-5** | 管线 | 结算单与 GOV-002 域内核阅零违规（主树复跑），化格检词双文档零违例 |
| **F-6** | 收口 | 双仓 settle、close、reconcile 判据项零新增、verify valid |

## 五、必读文件 {#read}

- 结算先例：sih-engine/doc/governance/SETTLEMENT-V1-2026-09-02.md
- 线总纲：sih-engine/sih/state/plan/leaseopt-line-v1.md
- 主线向界：sih-engine/doc/governance/GOV-002-mainline-lock-v1.md
- 六批结果档：sih-engine/sih/event/plan/leaseopt-{audit,fixguard,lockdb,lockqueue,ledger-repair,precheck}-solo-results.md
- 盘点账本：sih-tools/proposition/DES/leaseopt-audit/census-ledger.json
- 泊件：sih-engine/sih/state/parking/materials/pk-045-exit.json 与 pk-057.json

## 六、约束 {#constraints}

1. 结算零代码变更零行为变更，CONTRACT 零修订
2. 泊界在泊件零触碰零收编，出泊唯人节点；pk-059 进泊属新增非收编
3. 结算单在 des-001 域内须零违规，工作树路径域外 exit-2 教训承 parkrecon-solo 先例收口主树复跑

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] pk-059 停泊事件裸调落链逐笔 grep 验证
- [ ] 名录投影 pk-059 行与链一致（parkrecon-solo 复位后首笔新增行同规）

## 八、风险点 {#risks}

- tools 侧 CALL-LOG 两笔 allow 面本批显式纳入，防 staged_out_of_scope 复发
- GOV-002 换版属域内文档批，核阅必须主树复跑清零才收口

## 九、范式偏离声明 {#deviation}

确定性结算批，零子代理单线 solo；保留 T6-D 命名约定与 F 锚定与双仓同步。

## 十、关联文件 {#related}

- sih-engine/doc/governance/SETTLEMENT-LEASEOPT-2026-09-05.md（随批产出）
- sih-engine/sih/event/plan/leaseoptsettle-solo-results.md（随批产出）
- sih-engine/doc/governance/PARKING-v1.md（pk-059 名录行随批）

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/governance/（SETTLEMENT-LEASEOPT-2026-09-05.md 新增、GOV-002-mainline-lock-v1.md 追记、PARKING-v1.md 名录 pk-059 行）
- sih-engine/sih/state/plan/leaseopt-line-v1.md（关线追记）
- sih-engine/sih/state/parking/materials/pk-059.json（新增）
- sih-engine/sih/event/plan/leaseoptsettle-solo-results.md 与同目录 materials/
- sih-engine/sih/event/trail/2026-09-05.ndjson（停泊与认证追加）
- sih-tools/lease/CALL-LOG.md 与 sih-tools/scribe/CALL-LOG.md（随批留痕）
