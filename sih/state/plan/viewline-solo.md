# viewline-solo：视图线入向界——GOV-002 v2.2 换版与双泊件出泊

> 令源：用户 2026-09-05「视图要进入向界」并点名 pk-047 + pk-059；承同日裁定「人类视图统一由视图组件承载」「jsonl 和数据库双写」
> 范式：T6 单线 solo——换版与出泊执行批，零子代理

## 一、问题陈述 {#problem}

- **问题 1**：视图组件（viewer）承载人类视图已是在裁事实（双写裁定、pk-059 承载面），但主线向界无此线——视图工作在 GOV-002 主线一句话里无名分。
- **问题 2**：pk-059（批六 watch 视图批）出泊条件「用户裁批六立项开工与否」被本令触发：裁即立项。pk-047（OpenTelemetry GenAI 契约参考）出泊条件「视图组件契约起草批开工时用户裁参考与否」同被触发，用户点名 pk-047 即裁参考入材料面。
- **问题 3**：视图线无向界载体：线程序包、边界、与 watchcheck 及置信度组件的接线缺正式立文。

## 二、关键设计 {#design}

### 2.1 GOV-002 v2.2 换版

- 主线一句话改三线形：协调面（租约线，已结算）+ 判定面（measure-poly，在飞）+ 视图面（新增：协调视图与实时告警经视图组件实装，人类视图统一由组件承载，数据源为链投影库）。
- 退出标准加第四条：视图组件在役即 viewer 实装承载人类视图与告警、数据源接投影库、告警语义接 watchcheck 对表。主线 v2 结算待四条全绿。
- 版本节 v2.2 追记：换版令源即用户 2026-09-05「视图要进入向界」；旧版经 git 历史可查。
- 命题分道申报：本换版为用户令直入加状态完成类，按修订三改道确定性管线（管线三步加链证），不送采样。

### 2.2 双泊件出泊

- pk-059 出泊 promoted：出泊条件达成（用户裁批六立项），承接形即视图线立项，watch 对表段由 watchcheck-solo 批在飞承接。
- pk-047 出泊 promoted：出泊条件达成（契约起草批开工 + 用户点名即裁参考），OpenTelemetry GenAI 语义约定入视图契约参考材料面。

### 2.3 视图线程序包

- sih-engine/sih/state/plan/viewline-line-v1.md：批一 契约起草批（viewer 数据源=投影库、告警语义=watchcheck 对表、参考面=pk-047 OTel GenAI 材料、置信度组件聚合视图挂依赖）；批二 viewer 实装批；验收=F- 锚定随批。

## 三、工作清单 {#work}

- [ ] view-01：GOV-002 v2.2 换版三处编辑
- [ ] view-02：viewline-line-v1.md 线程序包立文
- [ ] view-03：pk-059-exit.json 与 pk-047-exit.json 落工地，scribe park 裸调落链 grep 验证
- [ ] view-04：PARKING-v1.md 名录更新（十四项减二为十二项，历史住户增二）
- [ ] view-05：管线三步与认证与双仓 settle 与 close 与 reconcile 与 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 换版 | GOV-002 v2.2 主线句三线形与退出标准第四条与版本节在档，其余节零字节改动 |
| **F-2** | 线程序包 | viewline-line-v1.md 在档，两批拆解与依赖在文 |
| **F-3** | 双出泊 | 链上 parking_exited pk-059 与 pk-047 带 promoted 与 ruling，grep 验证在档 |
| **F-4** | 投影 | 名录十二项与链机械一致 |
| **F-5** | 管线 | GOV-002 主树域内核阅零违规（工作树域外如实记），化格检词零违例 |
| **F-6** | 收口 | 双仓 settle、close、reconcile 判据项零新增、verify valid |

## 五、必读文件 {#read}

- 主线向界：sih-engine/doc/governance/GOV-002-mainline-lock-v1.md（v2.1 现文）
- 双泊件：sih-engine/sih/state/parking/materials/pk-059.json、pk-047.json
- 双写裁定：sih-engine/sih/event/plan/leaseoptsettle-solo-results.md 第六节
- 换版先例：GOV-002 版本节 v2 换版条目与 SETTLEMENT-V1

## 六、约束 {#constraints}

1. 出泊唯人节点：本批承载的即用户 2026-09-05 令裁，裁定文逐字入链
2. GOV-002 其余节（冻结清单、范畴排除）零字节改动
3. 零代码变更零 CONTRACT 修订；线批一（契约起草）不属本批，只立程序包
4. 停泊写入裸调逐笔 grep 验证（pk-057 附记纪律）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 主树 des-001 复跑零违规（域外掩判读教训承 parkrecon 先例）
- [ ] CALL-LOG 双笔随批

## 八、风险点 {#risks}

- 退出标准加条即主线结算时点后移：令源明确（视图进入向界），结算顺延如实呈报不遮
- 名录计数与出泊事件次序：先落链后改投影，防投影先行账实分叉

## 九、范式偏离声明 {#deviation}

换版执行批零子代理单线 solo；保留 T6-D 命名约定、F 锚定、双仓同步。

## 十、关联文件 {#related}

- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md
- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/sih/state/plan/viewline-line-v1.md（随批产出）
- sih-engine/sih/event/plan/viewline-solo-results.md（随批产出）

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md、PARKING-v1.md
- sih-engine/sih/state/parking/materials/pk-059-exit.json、pk-047-exit.json
- sih-engine/sih/state/plan/viewline-line-v1.md
- sih-engine/sih/event/plan/viewline-solo-results.md 与 materials/
- sih-engine/sih/event/trail/2026-09-05.ndjson、CALL-LOG 双笔
