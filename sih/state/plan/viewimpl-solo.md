# viewimpl-solo：视图组件首实装批

> task-packages 治理任务
> 承接：用户 2026-08-31 令即视图组件开工、按代理编组范式由执行代理完成、主会话验收
> 队形：单线形 solo——执行代理亲写零子代理、主会话守验收位
> 日期：2026-08-31

## 一、问题陈述 {#problem}

组件六席中三问、书简、温故已落引擎 src，视图为唯一零实装席位即引擎侧远端组件，心跳报数与结算单必经栏现由 agent 人话汇报承担。SPEC-004 第 40 行预埋视图组件经事件检索入口聚合异常信号即第二阶段形态，第 80 至 84 行钉死事件分类即可消费进人类视图与仅记录不进，DEC-007 第 48 与 81 行定视图为聚合输出组件即联动书简与参验与判定器的产出聚合出减少人类介入的效果。退出标准首条的组件列缺此一席即不可机械判定。

## 二、关键设计 {#design}

四件。一纯读聚合即视图零写零 LLM，数据源即书简 trail 事件流（参验与判定器产出已经链化），同参双跑逐字节一致。二三子命令即 viewer alarms 加 heartbeat 加 settle：alarms 即异常视图过滤 event_class 为可消费的事件分组呈报含事件哈希与 doc_id 指针，空即退出码零非空即一，仅记录类不进视图承 FM-05；heartbeat 即心跳视图在泊配对 entered 与 exited、每项按链上 entered_at 加 ttl_days 对参照日出在泊到期超期三态、最近三维读数 insufficient 如实标、可消费计数；settle 即结算单必经栏按日出当日事件类型计数、认证与意图计数、当前在泊复检名单承泊界清算门槛。三代码标识符 viewer 承 DEC-006 ask3repeater 先例即代码标识符非治理名，治理名视图已立 DEC-007 不重立。四退出码三值即零常态、一即 alarms 非空、二工具异常含链不可读。

## 三、工作清单 {#work}

- [ ] src/view/ 聚合纯函数三簇，先红后绿
- [ ] src/bin/viewer.rs 三子命令与用法串
- [ ] 对表两件即 settle 计数对 scribe query、heartbeat 三态对既有出泊件复算
- [ ] GOV-003 结算追加与 AGENTS 引擎源码行、管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 异常视图 | 工程治理 | 仅可消费类进视图即仅记录类零出现、指针逐条可回查链上、空零非空一、双跑逐字节一致 |
| **F-2** 心跳视图 | 工程治理 | 在泊配对正确含 --trail 可重复接续、三态机械判定抽一件已裁出泊件复算一致、缺席字段如实 unknown 不虚构 |
| **F-3** 结算视图 | 工程治理 | 按日计数与 scribe query 对表一致、在泊复检名单与 heartbeat 配对一致 |
| **F-4** 组件落位 | 工程治理 | src/view/ 与 viewer 落位、测试先红后绿全绿、零写调用纪律、退出码三值 |
| **F-5** 收口 | 链上治理 | 管线与认证入链、双仓提交、链 verify valid、reconcile unrouted 零、GOV-003 结算追加、AGENTS 行或延期如实记 |

## 五、必读文件 {#read}

- 必读 1：doc/spec/SPEC-004-event-stream.md 第 40 与 80 至 84 行即视图契约原文
- 必读 2：doc/decision/DEC-007-engineering-layer-ontology.md 第 48 与 81 行即聚合输出组件边界
- 必读 3：doc/governance/PARKING-v1.md 心跳节即心跳报数与结算单必经栏归视图族
- 必读 4：src/event_stream/park.rs 与 reading.rs 与 query.rs 与 event.rs 即事件载荷与分类与检索入口
- 必读 5：doc/governance/GOV-003-fullstate-course-v1.md 即结算追加纪律与版本现值

## 六、约束 {#constraints}

1. 视图零写即只读 trail 与零 LLM，不写任何文件不含临时件
2. 阈值维度归视图即 v1 只落三态机械判定不发明新阈值，新阈值须另批
3. GOV-002 判据文本零改、GOV-003 版本按现值递增即若 pkgclose 先落 v1.4 则本批 v1.5
4. AGENTS.md 撞锁即延期该行如 crossgraph 先例、不等待不绕行
5. 与 pkgclose-solo 共享当日链即若彼批在途先等其收约放链锁
6. 上链前必须等绿

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过即主会话验收复跑

## 八、风险点 {#risks}

在泊事件的 entered_at 取事件时间戳即链上单源，跨日链接续承书简先例后链覆盖前链；读数历史按维度取最近即不虚构趋势承秤星纪律；视图单独存在无意义即本批只交付聚合面，联动效果由后续真实使用浮出。

## 九、队形声明 {#formation}

单线形即执行代理亲写零子代理，主会话守验收位不代写。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 令
- 链件：sih/event/trail/<当日>.ndjson 即本批意图与认证
- 关联：SPEC-004、DEC-007、PARKING 心跳节、GOV-002 退出标准、读数与泊与跨方核毕三事件族

## 十一、请求写入 {#requested-writes}

- sih-engine/src/view/
- sih-engine/src/bin/viewer.rs
- sih-engine/Cargo.toml
- sih-engine/sih/state/plan/viewimpl-solo.md
- sih-engine/sih/event/plan/viewimpl-solo-results.md
- sih-engine/doc/governance/GOV-003-fullstate-course-v1.md
- sih-engine/sih/event/trail/2026-08-31.ndjson
- AGENTS.md
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[心跳视图]: 消解 即工作名直述即泊界心跳报数的机械聚合面、不做登记
叩问处置[结算单必经栏]: 消解 即工作名直述即结算视图的强制栏位、不做登记
叩问处置[视图]: 消解 即 DEC-007 子决策三已立聚合输出组件名视图、做承不做立
叩问处置[视图组件]: 消解 即任务包 #problem 节使用视图组件、DEC-007 已立名
叩问处置[聚合输出组件]: 消解 即 DEC-007 子决策三组件两类形态中的聚合类
叩问处置[alarms]: 消解 即 viewer 二进制三子命令之一 alarms、为代码标识符非治理名
叩问处置[heartbeat]: 消解 即 viewer 二进制三子命令之一 heartbeat、为代码标识符非治理名
叩问处置[settle]: 消解 即 viewer 二进制三子命令之一 settle、为代码标识符非治理名
叩问处置[viewer]: 消解 即代码标识符承 DEC-006 本名回滚即非治理名
叩问处置[viewimpl-solo]: 消解 即本批名 viewimpl-solo 视图组件首实装
叩问处置[视图组件首实装]: 消解 即本批主题视图组件首实装、与批名 viewimpl-solo 同指
