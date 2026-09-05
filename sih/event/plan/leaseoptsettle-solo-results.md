# leaseoptsettle-solo 结果档

> 批：leaseoptsettle-solo（租约优化线结算执行批）
> 会话：d647c8ff1cc1025c（ask3 侧 sess-zcode-260905-leaseoptsettle，双标识空间各认各）
> 日期：2026-09-05。队形：单线形 solo，零子代理。
> 令源：用户 2026-09-05「收工。继续收工」即线结算批准令；结算先例 SETTLEMENT-V1-2026-09-02.md。

## 一句话结论

leaseopt 线正式关闭：结算单 SETTLEMENT-LEASEOPT-2026-09-05.md 落 doc/governance/，六批证据指针逐条亲核在档，线级验收七条全数达成，pk-059 批六视图批入泊（c1b4fc2d）携批六全部承载指针，线总纲关线追记，GOV-002 v2.1 退出标准首条达成追记，主树归并态全量测试净跑 106 绿，链 verify valid。

## 一、F 表（完成度表）

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 结算单在档 | 六批证据指针亲核在档 | 过 | doc/governance/SETTLEMENT-LEASEOPT-2026-09-05.md，六结果档与账本与 CONTRACT 版本行与测量材料逐条亲开核在档 |
| F-2 批六入泊 | parking_entered pk-059 在档 | 过 | 链上 parking_entered#c1b4fc2d，context 载三类转承与 pk-057 指针 |
| F-3 换版追记 | GOV-002 首条追记与 v2.1 在档 | 过 | 退出标准首条追记与版本节 v2.1，其余节零字节改动 |
| F-4 关线追记 | 线总纲线状态已结算 | 过 | leaseopt-line-v1.md 线状态行在档 |
| F-5 管线 | 域内核阅零违规 | 过 | 主树域内复跑见收口读数；化格检词双文档零违例 |
| F-6 收口 | settle 与 close 与 reconcile 与 verify | 见收口读数 |

## 二、测试隔离缺陷发现与 A/B 实证（本批核查读数）

结算亲核跑主树归并态全量测试，首跑 1 failed 105 passed，失败件 test_path_inclusion_conflict 报 FileNotFoundError。根因定位：该测试 open 夹具只传 --ledger 不传 --locks，批三预检闸回落全局锁库，读得本批结算会话 d647c8ff1cc1025c 实持 sih-engine/doc/governance/，与夹具 allow 面 sih-engine/doc 相交即正确拒开，台账未产出致断言前 FileNotFoundError。A/B 实证即持锁跑红、放锁跑绿、复取锁复原；放锁窗口全量净跑 106 passed 0 failed。判定：生产行为正确即预检闸按设计对活跃会话交集拒开，缺陷在测试隔离面即夹具未与全局锁库隔离，后继触 lease 工具批顺手修，登记结算单遗留节。跨批合并组合即批三预检闸乘全局锁库在场形态无单批工作树测过，本批补测即实。

## 三、越线与误差申报

1. 例行读数与例扫：本日已由 parkrecon-solo 批落链（convergence 0.0、adoption 1.0、mergeback 0.043478）与例扫双跑四路 IDENTICAL checkmath zero_drift，本批不重复落每日快照，如实申报。
2. meter 对比降幅缺运行数据：线级验收第五条拆分申报，排队在役达成、对比降幅转例行读数观察项。
3. 认证与停泊裸调逐笔 grep 验证在档，meter 包裹掩败教训承 pk-057 附记。

## 四、管线与链

- 泊界心跳：引擎 attractor route 双目录退出码 0/0 零告警
- 三问双门：ask3 三锚（PRO-07 鉴、PRO-08 应、P3.1 注意力预算原文程序切片逐字节子串）双门零违规，digest passed covered 5
- 正身：identity verify verdict attest anomalies 0
- 租约：open d647c8ff1cc1025c 双仓，锁八路径含 tools 侧 CALL-LOG 两笔（parkrecon-solo 疏漏教训承接）
- 管线：化格、核阅、检词逐件执行，读数见认证节与收口读数
- 停泊：pk-059 裸调落链 c1b4fc2d grep 验证一笔在档

## 五、收口读数（close 后回填）

- 待回填：双仓 settle 号、主树 des-001 复跑读数、reconcile 读数、链 verify 终读
