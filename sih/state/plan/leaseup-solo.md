# leaseup-solo:租约修复升级批（收约尾部修复与锁面经济记账与追加形正典化）

> 令源:用户 2026-09-06 令「租约的修复升级先做，这是向界第一笔。」承同日系列裁定：锁面定价先记账后算分（阶段一零置信度依赖）、开工锁少与首次临时加锁免费、锁面形状文规治理不机器拦截、共享面追加形正典化与 T-10 甲案闭案、哨兵锁面呈报
> 范式:T6 单线 solo,委外代理亲写零子代理(用户亲转发委外)
> 定位:向界第一笔即本批承 GOV-002 追记入线（v2.3）
> 前置:closefix-solo 已收约(1.27.0 新归并机械在役,冻结避坑令已退役),基线 161 测

## 一、问题陈述 {#problem}

- **pk-072 收约尾部自毁**:close_session 的 append_event 函数体内延迟 import ledgerwrite,工地自举调用形下工地被本进程自拆即 import 抛 ModuleNotFoundError 未被捕获静默退出,revoked 行未落(复跑幂等收敛,零数据损失,closefix 活体验收钉死)
- **锁面零成本**:锁文件夹零代价、囤锁零代价、未用锁零代价——改啥锁啥无经济理性支撑;文规理由已裁不机器拦,但零计费零呈报即理由条款悬空
- **共享追加面锁形乱用**:gvec 批对当日 trail 取 exclusive 实证在案;追加面应一律 append 形(用户已裁),缺省形与正典文均未落地;T-10 甲案(追加面豁免合法化)未闭案
- **向界未记**:租约修复升级线为主线向界新段第一笔,GOV-002 无承载行

## 二、关键设计 {#design}

### 2.1 pk-072 修复

- ledgerwrite import 顶置 core 模块面(模块加载期解析,消除自拆后延迟 import)
- 回归夹具:自毁 cwd 形(收约代码自工地执行且工地被移除)下 revoked 行照落,先红后绿

### 2.2 锁面账单记账（阶段一纯记录,零结算零评分）

- 新台账 `lease/ledger/lockface-bills.ndjson`(追加式,flock 写点走 ledgerwrite.append_row):逐会话账单事件——开工面计费、首次临时加锁免费(每会话一次)、后续加锁逐路径计费、收工未用锁罚 × 件数
- 计费单位为记账点数非置信度:LOCK_BILL_UNIT=1 与 UNUSED_LOCK_MULTIPLIER=1 与 EXPANSION_FREE_QUOTA=1 冻结常量,治理约定走定义化通道(constmodel 先例,注释锚用途不锚值级推导)
- 零余额零扣款零结算:账单只记录,grandfather 条款即激活前行为不追溯(用户已裁),置信度台账后建时消费账单事件
- 双写形:ndjson 正典加 lockdb SQL 投影表 lock_bill(JSONL 正典与数据库双写既裁通用形)

### 2.3 文规锁面理由条款

- lease CONTRACT 增条款:锁文件夹或仓根的任务包须在请求写入节携带锁面理由行,缺理由即文规瑕疵记 CALL-LOG;零机器拦截(用户裁定不靠租约拦)
- 任务包模板注记随条款(范本面,非执法面)

### 2.4 哨兵锁面呈报

- watchcheck 增锁面形状巡逻:超宽面判定即仓根锁、整仓锁、单会话锁面超 LOCKFACE_WIDE_THRESHOLD(冻结常量,起步宁宽)——只报不拦,呈人节点(基线三:注意力只投异常信号)
- 与既有无主巡逻同跑同形,双跑 cmp 逐字节一致

### 2.5 追加形正典化与 T-10 甲案闭案

- 共享追加面清单(承 SCOPE_SHARED_SURFACE 六路加 CALL-LOG 两笔)批取锁一律 append 形:lock 命令对该清单路径缺省 append(可显式 exclusive 覆写,覆写即哨兵呈报项),正典文入 CONTRACT
- T-10 甲案闭案:追加面豁免合法化落档(直改笔对追加面免锁门即既成事实正典化),乙案作废,两案材料归档批 C 结果档不重出
- 引擎侧零实装(甲案零成本既定)

### 2.6 向界第一笔

- GOV-002 v2.3 追记:租约修复升级线入线承载行(线程序包 leaseup-line-v1.md 落 state/plan,线程序包形承 viewline-line 先例),版本节同步

## 三、工作清单 {#work}

- [ ] T-1 pk-072 修复:import 顶置加自毁 cwd 回归夹具先红后绿
- [ ] T-2 锁面账单台账与 lockdb 投影:四类事件逐笔记账(开工面/首免/后续加锁/未用罚),flock 写点,双写一致夹具
- [ ] T-3 文规锁面理由条款入 CONTRACT 与任务包模板注记
- [ ] T-4 哨兵锁面呈报:超宽面三态判定(仓根/整仓/超阈值)加双跑一致加零误报夹具
- [ ] T-5 共享面 append 缺省实装与 CONTRACT 正典化,T-10 甲案闭案落档
- [ ] T-6 GOV-002 v2.3 追记与线程序包落位
- [ ] T-7 判定语义一裁(锁面计费与首免与追加形缺省三变更,facet 合同模式)
- [ ] T-8 全测试族零回归与双仓 settle 加 close(走 closefix 新机械)加 reconcile 加 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | pk-072 | 自毁 cwd 夹具 revoked 行照落,红转绿;既有 161 基线零回归 |
| **F-2** | 账单记账 | 四类事件逐笔在账,首免恰一次,未用罚按件数,零余额零结算,ndjson 与 SQL 双跑一致,激活前零追溯 |
| **F-3** | 哨兵呈报 | 超宽面三态只报不拦,净态零误报,双跑 cmp 逐字节一致 |
| **F-4** | 追加形正典 | 共享面锁缺省 append 夹具,显式覆写出呈报项,T-10 甲案闭案档在案 |
| **F-5** | 一裁 | 三变更 facet 合同模式 stable_clear 过执契,near_threshold 呈用户转主会 |
| **F-6** | 零回归 | 全测试族绿,openhyg 与批 C 与 closefix 交付语义零动,台账行格式零变更 |
| **F-7** | 向界第一笔 | GOV-002 v2.3 与线程序包在档过管线 |

## 五、必读文件 {#read}

- pk-072 泊材料:sih-engine/sih/state/parking/materials/pk-072.json 与 closefix-solo-results.md 首跑静默退出根因节
- 哨兵现状:sih-tools/watchcheck/(CONTRACT.md 与 core.py 与 constants.py)
- 共享面清单:sih-tools/lease/src/lease/ 内 SCOPE_SHARED_SURFACE
- 批 C 两案材料:sih-engine/sih/event/plan/idenlane-guard-solo-results.md 第四节与 materials/t10/
- 向界:sih-engine/doc/governance/GOV-002-mainline-lock-v1.md(v2.2 现行)
- 机械链:sih-tools/BATCH-FACE.md(含全部勘误节与冻结令退役节)

## 六、约束 {#constraints}

1. 判定语义三变更(计费/首免/append 缺省)共过一裁,near_threshold 呈用户转主会不自行终签
2. 账单零结算零评分零置信度依赖(阶段一分阶段既裁,置信度后行);grandfather 零追溯
3. 台账行格式零变更;sessions 与 locks 与 bypass 台账零改面;ledgerwrite 唯一写点机制零动
4. 文规零机器拦截:理由条款为契约与范本面,哨兵只报不拦
5. 引擎侧零实装(T-10 甲案零成本既定);openhyg 与批 C 与 closefix 交付零回归
6. 判定常数与清单零裸奔,定义化通道入注释入 CONTRACT;scribe 裸调逐笔 grep 禁 meter 包裹掩败;退出码以真值核(掩败家族当日三连咬教训,禁管道掩退出码)
7. 范畴排除四项未裁不在本批:T-9 白名单执法位、四陈旧会话销账、wenguobs 双写处置、locks 镜像 131 行
8. 收约走 closefix 新机械;任务包绝对路径;des-001 域外 exit-2 如实记

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-7 全过
- [ ] 双仓 settle + close(新机械) + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/leaseup-solo-results.md 与 materials/
- [ ] lease CONTRACT 修订与 watchcheck CONTRACT 修订与 CALL-LOG 双笔与 BATCH-FACE 随批
- [ ] pk-072 出泊材料随批(修复承载出泊)

## 八、风险点 {#risks}

- append 缺省改变锁形缺省面广:须过全族金向量与既有 append 共存语义回归(locksplit 1.14 交付)
- 账单事件写点与 closefix 并集通道交互:账单台账即追加面,收约归并按新机械并集不盖版(实测夹具)
- 哨兵阈值宁宽勿漏:阈值过紧即每批自鸣(哨兵风险既档),起步宽呈报噪声如实估
- 一裁三变更共材:命题分三段各判,材料一段承载防依据族分散(basisunion 并集判据已在役,单锚归约 predspec2 先例)

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- 上游:closefix-solo(新归并机械与 pk-072 发现)与 idenlane-guard-solo(T-10 材料)与 pk-073(经济制度定型,账单为其阶段一落地)
- 下游:置信度台账与付费抢占(阶段二,候 pk-073 建模批)

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/(core.py、cli.py、lockdb.py、guardcore.py 若及)与 lease/CONTRACT.md 与 CALL-LOG.md
- sih-tools/watchcheck/(src 与 CONTRACT.md 与 CALL-LOG)
- sih-tools/lease/ledger/lockface-bills.ndjson(新台账,随批首建)
- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md(v2.3 追记)与 sih-engine/sih/state/plan/leaseup-line-v1.md(线程序包)
- sih-engine/sih/event/plan/leaseup-solo-results.md 与 materials/ 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/state/parking/materials/pk-072-exit.json(出泊材料)
- sih-engine/sih/event/trail/2026-09-06.ndjson 与后续日链
