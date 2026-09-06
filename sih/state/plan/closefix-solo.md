# closefix-solo:收约归并机械修复（台账盖版与 stash 吞件与回补网三病一役）

> 令源:用户 2026-09-06 令「修复」（承 ledgerloss5-solo 调查批根因呈报与 idenlane-guard-solo 批 stash 吞件发现，泊件 pk-071 出泊承载）
> 范式:T6 单线 solo,委外代理亲写零子代理(主会拉子代理执行)
> 域:sih-tools/lease 归并机械族(core.py 与 cli.py 与 lockdb.py)

## 一、问题陈述 {#problem}

三病同族俱在 close 归并机械,证据链俱在 ledgerloss5-solo 结果档（含沙盒四红证）与 idenlane-guard-solo 结果档:

- **病一 台账盖版**:core.py:1056 两端点差集把主干侧前进误入归并面;core.py:1336-1347 allow_and_merge 分支版整文件 checkout 盖掉主树活面(无锁无回补,checkout 0.008 秒绕开 flock 而 append_row 阻塞 2.01 秒);core.py:872-873 预收提交 git add -A 固化盖版;core.py:551 回补网前置已破结构性跳过。生产实证五笔丢会话行(前四笔归因经调查批重裁同签名,第五笔 watchcheck 窗口钉死)
- **病二 stash 吞件**:close 前置舞步对主树未跟踪件 git stash push -u 后无 pop 通道,engine 主树六笔 closeguard stash 累积在册,2026-09-05 整日链与四十五项未跟踪件一度全体离盘(idenlane-guard-solo 批 stash apply 保引用全量复位,六笔引用留作审计)
- **病三 回补网自死锁**:恢复非空时 restore_missing 到 append_row 同路径二次 flock 同进程自死锁(沙盒乙景 12 秒超时探针固定);即便次序修好可达,此锁缺陷仍致回补挂死
- **连带数据卫生**:wenguobs-solo issued 行 337/339 历史双写(2026-08-31 重开舞步遗留),sessions.ndjson 761 行在档;locks.ndjson 镜像历史窗伤亡行(SQLite 正典在册,镜像影响面评估未做)

## 二、关键设计 {#design}

### 2.1 病一修法:台账面冻结豁免（用户已裁方向）

- close 归并对台账追加面(LEASE_LEDGER_APPEND_FACES 冻结常量:sessions.ndjson 与 locks.ndjson 与 bypass.ndjson 与 checks 目录)永不整文件 checkout 盖版,只走并集追加通道——重读现行活面自算并集超集落盘,分支版仅作并集输入不作替换源
- 归并面差集改三点式(base 与 branch 与 live 三点),主干侧前进(live 有 base 无)不入归并面即不触发让位判定
- 预收提交最小化:git add -A 收窄为归并目标路径面,台账面活行不被固化进分支基线

### 2.2 病二修法:stash 通道

- 前置舞步 stash push -u 后收约全路径(成功与失败与拒)必有 apply/pop 复位通道,收约毕主树未跟踪件零离盘;六笔历史 stash 逐笔 apply 复位后 drop(保引用审计件先落档)
- 结构性替代候选一并评估:前置舞步改「未跟踪件照单登记不清场」形态,若可行即根除 stash 舞步

### 2.3 病三修法:回补网去锁重入

- restore_missing 复用已持锁上下文(ledger_lock 可重入形或内联写),消除同进程二次 flock;回补网位次前移至 checkout 之前,盖版不可达即不复存在

### 2.4 数据卫生

- wenguobs 双写行处置:append-only 台账不改写历史行,双写行留档披露加 count 视图去重(方案呈用户裁,本批不代裁)
- locks 镜像与 lockdb 对表评估:ndjson 镜像按 lockdb 正典全量重放校验,漂移行清单呈报(修不修随裁)

### 2.5 判定语义

- close 行为变更(台账面永不盖版 + stash 必复位)过得一裁(facet 合同模式),near_threshold 呈用户转主会

## 三、工作清单 {#work}

- [x] T-1 台账面冻结豁免实装(并集追加通道 + 三点式差集 + 预收提交最小化)
- [x] T-2 stash 复位通道实装(结构性替代:照单不清场),双仓历史 stash 二十三笔 closeguard 逐笔复位 drop 加 WIP 二笔核实超越随批 drop 带审计件(实数超出任务包立包时六笔,后继批 close 继续累积所致)
- [x] T-3 回补网去锁重入(ledger_lock 可重入)与位次前移(台账面并集通道即盖版不可达)
- [x] T-4 ledgerloss5 沙盒四红证全数转绿(甲乙丙 H-3 四景回归)
- [x] T-8 既有全族测试零回归(基线 149,新增 12 件合计 161 绿零适配)
- [x] T-9 数据卫生:wenguobs 双写呈报与 locks 镜像对表评估(materials/hygiene/data-hygiene-report.json)
- [x] T-10 判定语义一裁(facet 合同模式九发 stable_clear,链笔 39b33ff8)
- [x] T-11 BATCH-FACE 冻结避坑令退役登记(修复落地后收约前冻结不再必要,勘误节收编)

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 盖版根除 | ledgerloss5 沙盒甲景(生产函数直调 7 行丢失重放)转绿;并发活写窗 close 主树活行零丢失(压测形:close 期间他进程追加,收约毕行全在) |
| **F-2** | stash 根除 | 未跟踪件 close 全路径零离盘(成功/失败/拒三态夹具);六笔历史 stash 复位 drop 且审计件在档 |
| **F-3** | 回补网 | 乙景自死锁转绿;回补位次前移后丙景零丢失维持 |
| **F-4** | 零回归 | 全族 149+ 绿;台账行格式零变更;openhyg 与批 C 交付语义(按表解析/PID 探针/routed_direct/白名单)零动 |
| **F-5** | 一裁 | close 行为变更 facet 合同模式 stable_clear 过执契,near_threshold 呈用户 |
| **F-6** | 数据卫生 | wenguobs 双写与 locks 镜像漂移清单呈报在档(处置随用户裁) |

## 五、必读文件 {#read}

- 根因调查:sih-engine/sih/event/plan/ledgerloss5-solo-results.md(§一代码行级与沙盒红证)
- stash 发现:sih-engine/sih/event/plan/idenlane-guard-solo-results.md(重大新发现节)
- 病灶源码:sih-tools/lease/src/lease/core.py(1056 与 1336-1347 与 872-873 与 551 位族)与 cli.py 与 ledgerwrite.py 与 lockdb.py
- 契约前史:sih-tools/lease/CONTRACT.md 修订二十八至三十九
- 机械链:sih-tools/BATCH-FACE.md(含全部勘误节与冻结避坑令)

## 六、约束 {#constraints}

1. 台账行格式零变更零迁移;append-only 历史行零改写(wenguobs 双写不删不改,只呈报)
2. 判定语义一裁红线;near_threshold 呈用户转主会
3. TDD 先红后绿:ledgerloss5 沙盒四景为现成红证,转绿为验收
4. 在途批避让:开工预检撞锁即 wait-turn 排队;共享面按排队语义
5. scribe 写入裸调逐笔 grep 验证禁 meter 包裹掩败;des-001 域外 exit-2 如实记;任务包绝对路径
6. openhyg 与批 C 交付零回归:按表解析与 PID 探针与 routed_direct 与白名单语义俱不动
7. 收约自身即活体验收:本批 close 须走新机械完成,收约读数即 F-1/F-2 生产实证

## 七、验收标准 {#acceptance}

- [x] F-1 至 F-6 全过(逐条判定见结果档 F 表)
- [x] 双仓 settle + close(走新机械活体验收) + reconcile + verify
- [x] 结果档 sih-engine/sih/event/plan/closefix-solo-results.md 与 materials/
- [x] CONTRACT 修订四十(1.27.0)与 CALL-LOG 双笔与 BATCH-FACE 勘误随批
- [x] pk-071 出泊材料随批(修复承载出泊,pk-071-exit.json)

## 八、风险点 {#risks}

- 并集通道与 closeguard 让位判定交互:三点式差集改动须过 is_pure_append_conflict 全族金向量
- stash apply 复位与主树并发写竞态:复位窗锁面评估,必要时复用 ledger_lock
- 历史六笔 stash 内容重叠(同文件多版本):apply 序按 stash 栈序后进先出逐笔对表
- 行为变更面广:一裁材料须覆盖三病各自语义位

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- 上游调查:ledgerloss5-solo(根因)与 idenlane-guard-solo(stash 发现)
- 出泊承载:pk-071(本批交付即出泊)
- 前科:pk057fix-solo(1.24.0,只修了追加路未修归并路——调查批重裁在案)

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/(core.py、cli.py、lockdb.py、ledgerwrite.py)
- sih-tools/lease/tests/(沙盒四景回归与三态夹具)
- sih-tools/lease/CONTRACT.md 与 CALL-LOG.md、sih-tools/scribe/CALL-LOG.md
- sih-tools/BATCH-FACE.md(冻结避坑令退役勘误)
- sih-engine/sih/event/plan/closefix-solo-results.md 与 materials/
- sih-engine/sih/state/parking/materials/pk-071-exit.json(出泊材料)
- sih-engine/sih/event/trail/2026-09-06.ndjson 与后续日链
