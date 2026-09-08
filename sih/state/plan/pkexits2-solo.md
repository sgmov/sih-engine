# pkexits2-solo：三卡出泊执行批（pk-042 与 pk-061 与 pk-067）

> 治理任务包（出泊执行类，单线形 solo，DEC-018）
> 承接：用户 2026-09-08 FORK-1 令「执行 pkexits2-solo 批（三卡出泊执行批）」，出泊裁决已由人节点经 sitruling-solo 三张终签承载，本批只执行不裁；先例形 pkexits-solo 与 pk013exit-solo 与 pk-044-exit.json
> 会话：sess-zcode-260908-fork1-pkexits2；日期：2026-09-08

## 一、问题陈述 {#problem}

- 三卡出泊裁决已在链（m-sitruling-1 终签 8d489e00、m-sitruling-3 终签 fc17e1ce、m-sitruling-4 终签 4f94b63f，三笔 crosscheck_completed 于 2026-09-08 当日链核实在档），出泊链笔与出泊材料未落，泊界名册仍挂三卡
- 出泊唯人节点已履行，执行面即机械落笔：三笔 parking_exited 上链＋三件出泊材料按 pk-044-exit.json 形落位

## 二、令源照录 {#rulings}

- pk-042 出泊：数值校准定性为零信任补位非过度设计（m-sitruling-1 stable_clear 终签 8d489e00 承载，09-04「过度设计」判词作废）；DE 不立项、触发器保留即 ga-1 校准进入解析不可行期时复检；系数清账归 pk-053 两态制即 constclear2 承载。注意此件在工具线泊界 sih-tools/parking/materials/pk-042.json。
- pk-061 出泊：裁统一即立项，形态为信封统一、正文两型保留（m-sitruling-3 终签 fc17e1ce）；候实装批另立项，本批只出泊登记后继位。
- pk-067 出泊：借鉴范围裁差分语法采纳，文档规格与配置规格两型分立，统一解释器不立项挂 spec-delta 触发器（m-sitruling-4 终签 4f94b63f）；归档合并纪律与向界结算追加制同构，落注不另立项；候文规后继批。

## 三、关键设计 {#design}

- 三件出泊材料按 pk-044-exit.json 形（action exit、disposition、entry_id、id、parking 含 entered_at 与 exited_at 与 ttl_days、path、ruling 令源照录、state exited、ttl_days）落两线：pk-042-exit.json 落工具线 sih-tools/parking/materials/，pk-061-exit.json 与 pk-067-exit.json 落引擎线 sih-engine/sih/state/parking/materials/；终签哈希入 context 即链上实证 crosscheck_completed 哈希与进泊笔哈希
- 三笔 parking_exited 经引擎 scribe park 上链，disposition promoted，ruling 照录令源
- 原在泊材料件（pk-042.json 与 pk-061.json 与 pk-067.json）零触碰，真相在链；泊界名册投影更新留泊界复检批不并批

## 四、工作清单 {#work}

- [ ] 三件 *-exit.json 材料落两线工地
- [ ] 三笔 parking_exited 上链（scribe park，逐笔 capture event_hash）
- [ ] 任务包与结果档与批材料管线三步与 checkcite 与认证
- [ ] 双仓 settle 与放锁 close 与 reconcile 与 verify 与回填补笔

## 五、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 三笔出泊在链 | 数据治理 | 当日链 parking_exited 三笔 entry_id 即 pk-042 与 pk-061 与 pk-067，disposition promoted，ruling 含令源照录，重入拒零触发；三卡进泊笔（fd5de7f7 与 f1ab980f 与 ebec521e）与三张终签（8d489e00 与 fc17e1ce 与 4f94b63f）在链预核读数落批材料 |
| **F-2** 出泊材料三件 | 数据治理 | 三件 *-exit.json 按 pk-044-exit.json 形落正确线别，终签哈希入 context，state exited |
| **F-3** 写入仅 allow | 治理 | 写入仅请求写入节所列路径；名册投影 PARKING-v1.md 零触碰 |
| **F-4** 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 较批前零新增 |

## 六、必读文件 {#read}

- sih-tools/BATCH-FACE.md（全文，本批开工前已读）
- sih-engine/sih/state/plan/pkexits-solo.md 与 pkexits-solo-results.md（先例形）
- sih-engine/sih/state/parking/materials/pk-044-exit.json（出泊材料形先例）

## 七、约束 {#constraints}

1. 出泊裁决已由令源承载本批只执行不裁，ruling 照录不添裁
2. 主树零直写；链文件只经引擎 scribe 写位；禁管道掩退出码；误差红证如实记档
3. 名册投影与原在泊材料与 watch 呈报件零触碰
4. 守卫在位禁 plain commit；补笔走 bypass 登记；CALL-LOG 走 lease call-log append
5. scribe append 只认 JSON：md 件走内容哈希清单件认证形（anchorskill 先例）

## 八、验收标准 {#acceptance}

F-1 至 F-4 全过；收约后零本批活跃锁零活跃会话；结果档 pkexits2-solo-results.md 落 event/plan。

## 九、风险点 {#risks}

- pk-042 进泊笔形旧（2026-09-02 无 parking 字段）——出泊闸若核字段形即以链笔为准如实申报
- close 撞 callloghyg 候清项活面即 bypass-orphan 与 bypass-calllog 先例通道

## 十、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/pkexits2-solo.md
- sih-engine/sih/state/parking/materials/pk-061-exit.json
- sih-engine/sih/state/parking/materials/pk-067-exit.json
- sih-tools/parking/materials/pk-042-exit.json
- sih-engine/sih/event/plan/pkexits2-solo-results.md
- sih-engine/sih/event/plan/pkexits2-solo-materials/
- sih-engine/sih/event/trail/2026-09-08.ndjson
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
