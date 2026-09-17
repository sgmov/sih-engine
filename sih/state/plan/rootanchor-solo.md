# rootanchor-solo:位置锚定根与链证守门批

> 令源:用户 2026-09-06 令「修」,承主会 billwire-solo 验收定性:活体账单随工地蒸发(同坑第三批)、本批链证为零(链 85 事件与批前分毫不动)、交付声明与实态不符连续两批
> 范式:T6 单线 solo,委外代理亲写零子代理(用户亲转发委外)
> 定位:leaseup 线第三批,位置锚与链证两缺口一次钉死
> 前置:billwire-solo 已收约(183 基线),锁面零在途

## 一、问题陈述 {#problem}

- **位置锚错根**:sessions 与 locks 与 bypass 与 lockface-bills 台账位经 tool_dir 即代码位置解析,工地自举形(批在工地代码副本内跑 CLI)下全部解析到工地位,工地拆除即蒸发——先例三连:openhyg 活体验收首跑、leaseup 自举窗、billwire 本体三笔账单随工地蒸发(billwire 报告自述入账即工地台账位,主树 lockface-bills 缺席与 lock_bill 表未建亲核在案)
- **夹具测错形态**:billwire 防陷阱夹具测主树代码加工地 cwd,未测工地代码自举形,夹具全绿而生产照坑
- **无链证守门**:批可在零意图笔零认证笔下收约,billwire 即证(其会话 f70734cb 双行在账而链上零本批事件);其工地材料目录空仅结果档在档,ask3 记录在 scribe/reports 可回查
- **billwire 遗留补证**:意图笔与认证笔缺、账单三事件蒸发待重放

## 二、关键设计 {#design}

### 2.1 位置锚换根

- 新增工作区根发现函数:从目标仓 git toplevel 向上找工作区标记(多仓并立形,单实现),台账位解析(sessions 与 locks 与 bypass 与 lockface-bills 与 checks)全部改走根锚;tool_dir 形降为回退位且触发告警行
- 链位不属 lease 域(scribe --trail 显式传参),本批零引擎改动

### 2.2 自举自卫

- CLI 入口自检:cwd 或 __file__ 落 worktrees/ 之下即硬拒(exit 2,报文指引),除非 --ledger 与 --locks 与账单位三参显式全传(自测与复盘合法形)
- 硬拒先于任何读写零副作用

### 2.3 链证守门

- close 前置新增:核本会话在正典链(工作区根发现位)有 intent_refined 笔与 certification_completed 笔,缺即拒收约,报文指明缺笔类
- 直改车道与无租约形不属 close 域零影响;跨日链查即签发日至收约日逐日

### 2.4 billwire 补证

- 意图笔:ask3 记录在 scribe/reports 在档,经 scribe intent 补落正典链,补录事实如实入档
- 认证笔:管线报告在案者补,缺者如实申报缺口不伪造(billwire 工地材料目录空在案,能补尽补)
- 账单三事件:按 billwire 报告在案读数(open_face_bill 3pts 与 lock_free 0pts 与 lock_charged 1pts,报告载两会话号 f70734cb 与 eacfd4e5 如实照录)重放落主树 lockface-bills.ndjson,repair 形标记即重放非原笔,lock_bill 表同笔

### 2.5 判定语义

- 自举硬拒与链证守门属判定行为变更,共过一裁(facet 合同模式);补证重放非行为变更零裁

## 三、工作清单 {#work}

- [ ] T-1 根锚发现函数与台账位解析全量换锚,tool_dir 降回退位加告警
- [ ] T-2 自举自卫:工地位三参缺即硬拒零副作用,显式全传即正常
- [ ] T-3 链证守门:close 前核意图笔与认证笔,缺即拒;直改车道零影响
- [ ] T-4 TDD 先红后绿:真形态夹具即工地代码自举形跑 CLI(在工地副本内执行,非 cwd 模拟),账单与台账落根下正典位;零链证会话 close 拒;三参自测形放行
- [ ] T-5 billwire 补证:意图笔补链、认证笔能补尽补缺则申报、账单三事件重放落正典位
- [ ] T-6 判定语义一裁(facet 合同模式)
- [ ] T-7 全测试族零回归与双仓 settle 加 close(过守门)加 reconcile 加 verify 与 BATCH-FACE 坑位更新

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 根锚 | 工地自举真形态夹具:工地代码副本内跑 CLI,四类台账与账单俱落工作区根下正典位;tool_dir 回退告警在档 |
| **F-2** | 自举自卫 | 工地位无显式三参即 exit 2 零读写;显式全传即正常;硬拒先于副作用 |
| **F-3** | 链证守门 | 零链证会话 close 被拒(夹具红转绿);有笔会话放行;直改车道零影响 |
| **F-4** | 补证 | billwire 意图笔在链;认证笔补或缺口申报在档;账单三事件重放落主树带 repair 标记,lock_bill 同笔 |
| **F-5** | 一裁 | 自举硬拒与链证守门 stable_clear 过执契,near_threshold 呈用户 |
| **F-6** | 零回归 | 183 基线全绿加新增全绿;五前批交付零动;台账行格式零变更 |

## 五、必读文件 {#read}

- 验收定性:主会 billwire 验收(本包问题陈述即摘)与 billwire-solo-results.md 自述工地入账段
- 位置解析现状:sih-tools/lease/src/lease/cli.py 与 core.py 的 tool_dir 族与 resolve 族
- 先例三连:openhyg-solo-results.md 申报一与 leaseup-solo-results.md 越线申报二
- 补证材料:sih-tools/scribe/reports/2026-09-06-ask3-billwire-solo-record.json 与 billwire-solo-results.md
- 机械链:sih-tools/BATCH-FACE.md(含全部勘误节)

## 六、约束 {#constraints}

1. 一裁红线:near_threshold 呈用户转主会不自行终签
2. 台账行格式零变更;ledgerwrite 唯一写点零动;补证重放带 repair 标记不伪装原笔;补录时序如实不伪造
3. 引擎零改动(链位 scribe --trail 显式传参既定);五前批交付零回归(closefix 并集归并与 openhyg 与批 C 与 leaseup 与 billwire 接线)
4. 范畴排除未裁不动:T-9 执法位与四陈旧会话销账与 wenguobs 双写与 locks 镜像与 T-10 乙案
5. scribe 裸调逐笔 grep 禁 meter 包裹掩败;退出码真值核禁管道掩
6. 本批活体即考试:批自身账单必须落主树正典位、链上必须有本批意图与认证笔、close 必须过守门——缺一样即 F 不及格,如实申报不粉饰
7. 收约走 closefix 新机械加本批新守门;任务包绝对路径;des-001 域外 exit-2 如实记

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 本批活体三证在档:主树账单有本批笔、正典链有本批意图与认证、close 守门读数
- [ ] 双仓 settle + close(过守门) + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/rootanchor-solo-results.md 与 materials/(本批材料必须落档不重蹈空目录)
- [ ] lease CONTRACT 修订与 CALL-LOG 双笔与 BATCH-FACE 坑位随批

## 八、风险点 {#risks}

- 根锚发现的标记判定:工作区无显式标记文件时以多仓并立形推断,边界形(单仓复制出工作区外)即回退加告警,宁告警不误判
- 链证守门与历史会话兼容:已收约历史会话不重检,守门只对新 close 生效
- 补证意图笔的时序:补录事件时间戳即当下,事件内容载原始记录哈希,不伪造原时序
- 自举硬拒波及既有测试族:工地形测试夹具需显式三参适配,适配面如实记

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- 上游:billwire-solo 验收(主会 2026-09-06)与先例三连批
- 同线:leaseup-solo 与 billwire-solo
- 下游:置信度台账消费(pk-073 建模批后)

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/(cli.py、core.py、lockcore.py 若及)与 CONTRACT.md 与 CALL-LOG.md
- sih-tools/lease/tests/(真形态夹具与守门夹具)
- sih-tools/lease/ledger/lockface-bills.ndjson(billwire 三事件重放与本批活体账单随批落)
- sih-tools/BATCH-FACE.md(坑位更新)
- sih-engine/sih/event/plan/rootanchor-solo-results.md 与 materials/ 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/2026-09-06.ndjson 与后续日链
