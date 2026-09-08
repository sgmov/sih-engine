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

- 化格：引擎域两件过 packs/general-v1 全 exit 0 零改。
- 核阅：des-001 两件全 exit 0 零违规（任务包、结果档）。
- 检词：nomenclator check packs/core 两件全 exit 0 零违例。
- checkcite：recall 出泊泊界裁决主题后 checkcite verdict pass missing 零（报告件随批落档）。

## 认证清单

| 件 | 认证哈希前 8 |
|---|---|
| ask3 记录 | 61c53679 |
| 验证件 | d8127e26 |
| 正身件 | 9ef4629f |
| pk-042 出泊材料 | 6274b71a |
| pk-061 出泊材料 | 226a7a64 |
| pk-067 出泊材料 | 2ea7d33c |
| checkcite 件 | 359df88f |
| 内容清单件（md 两件 sha256） | aa60b855 |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 三笔出泊在链 | 数据治理 | 三笔 parking_exited promoted ruling 照录，预核双锚在链 | 通过（6bbaea21 与 e4898b46 与 d8f8887f，重入拒零触发） |
| F-2 出泊材料三件 | 数据治理 | 三件 *-exit.json 按 pk-044-exit 形落正确线别，终签哈希入 context，state exited | 通过（两线落位核对在档） |
| F-3 写入仅 allow | 治理 | 写入仅请求写入节所列路径；名册投影零触碰 | 通过（写入面即任务包、三件出泊材料、结果档与批材料、链文件、报告目录；施工滑步即删主树份在越线申报） |
| F-4 链面全绿 | 治理 | 双仓 settle、close 零失败、verify valid、reconcile 零新增 | 通过（tools 归并 7e00e48c 与 engine 归并 0c68e9e 各携 closeguard 预收提交；close 一跑成携 bypass-orphan 与 bypass-calllog 留痕即无主活面均 callloghyg 候清项非本批所写；verify valid events 76；reconcile 双仓 unrouted 零即零新增，cert_missing 与 unbypassed 为在盘历史账面项） |

## 越线与误差申报

- 出泊材料主树滑步一笔（即删自纠，红证即本申报）。
- 其余误差零申报。

## 结算读数

- 双仓 settle：tools 段一提交（归并 7e00e48c）与 engine 段一提交（归并 0c68e9e），cert 取 61c53679 即 ask3 记录认证前八位；closeguard 预收提交双仓在档。
- 放锁收约：9 锁 unlock 全零，残余锁零，close 一跑成 revoked 真值，工地与分支清除。
- 链 verify：valid，events 76。
- reconcile：双仓 unrouted 零即本批零新增路由缺口。
- 收约补笔：管线实录与认证清单与 F-4 与结算读数即本笔，经 --no-verify 加 lease bypass 登记通道入版控（anchorskill 与 critsweep 与 reroute 先例同形）。
