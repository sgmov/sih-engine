# pkexits2-solo 结果档：三卡出泊执行批（pk-042 与 pk-061 与 pk-067）

> 承接：任务包 pkexits2-solo.md 与用户 2026-09-08 FORK-1 令；出泊裁决由 sitruling-solo 三张终签承载，本批只执行不裁。
> 队形单线形 solo，日期 2026-09-08，会话 sess-zcode-260908-fork1-pkexits2（session_id c3f48933b3433025）。

## 意图锚定

- 意图事件：intent_refined event_hash 前 8 `1a5cf5e6`
- record：sih-tools/scribe/reports/2026-09-08-ask3-pkexits2-solo-record.json；validation 同目录
- 三锚引文程序切片（01-ontology-of-names.md L18、08-on-settle.md L110、07-on-assay.md L55）于 ask3 记录，生成器 make_ask3_pkexits2-solo.py 随批落档，禁手打承契约。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0；ask3repeater exit 0 status ok anchor_count 3。
- 叩问：三轻信号即出泊执行批、两线、后继位；digest passed covered 3。
- 正身：anomalies 0。
- 判据扫（启动节律）：C1/C3/C5 达成、C2 在飞、C4 沉底 gap 5，两线泊界告警零，degraded 假。
- watch 对表：exit 1 呈报 CALL-LOG 投影腿与 calls.ndjson 等 callloghyg 候清项，非本批活面不代清。
- 例行读数：convergence 0.125 与 adoption 0.857143 与 mergeback 0.029412 三维落链。
- 预核（F-1 前置）：三卡进泊笔 pk-042 fd5de7f7（2026-09-02）与 pk-061 f1ab980f、pk-067 ebec521e（2026-09-06）在链；三张终签 m-sitruling-1 8d489e00 与 m-sitruling-3 fc17e1ce 与 m-sitruling-4 4f94b63f 俱 crosscheck_completed 在 2026-09-08 当日链。

## 件读数：三笔出泊与三件材料

| 卡 | 出泊链笔 | 材料 | 线别 | disposition | 终签入 context |
|---|---|---|---|---|---|
| pk-042 | 6bbaea21 | pk-042-exit.json | 工具线 | promoted | 8d489e00（m-sitruling-1） |
| pk-061 | e4898b46 | pk-061-exit.json | 引擎线 | promoted | fc17e1ce（m-sitruling-3） |
| pk-067 | d8f8887f | pk-067-exit.json | 引擎线 | promoted | 4f94b63f（m-sitruling-4） |

- 三件材料按 pk-044-exit.json 形（action exit、state exited、parking 三字段、ruling 令源照录、context 链上实证），原在泊材料件零触碰，名册投影 PARKING-v1.md 零触碰留泊界复检批。
- 施工伤自纠一笔：出泊材料首笔误写主树一份（工地铁律滑步），即删主树份仅工地在档，park 闸改读工地路径，滑步如实申报。

## 管线读数

随收约回填。

## 认证清单

随收约回填。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 三笔出泊在链 | 数据治理 | 三笔 parking_exited promoted ruling 照录，预核双锚在链 | 通过（6bbaea21 与 e4898b46 与 d8f8887f，重入拒零触发） |
| F-2 出泊材料三件 | 数据治理 | 三件 *-exit.json 按 pk-044-exit 形落正确线别，终签哈希入 context，state exited | 通过（两线落位核对在档） |
| F-3 写入仅 allow | 治理 | 写入仅请求写入节所列路径；名册投影零触碰 | 通过（写入面即任务包、三件出泊材料、结果档与批材料、链文件、报告目录；施工滑步即删主树份在越线申报） |
| F-4 链面全绿 | 治理 | 双仓 settle、close 零失败、verify valid、reconcile 零新增 | 待收约回填 |

## 越线与误差申报

- 出泊材料主树滑步一笔（即删自纠，红证即本申报）。
- 其余误差零申报。

## 结算读数

待收约回填。
