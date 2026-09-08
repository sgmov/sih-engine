# billwire-solo:账单接线与工地路径基准统一批

> 令源:用户 2026-09-06 令「开，出任务包和提示词」,承主会 leaseup-solo 验收报告两缺口:账单零接线死代码与 T-12 工地路径绝对化未做
> 范式:T6 单线 solo,委外代理亲写零子代理(用户亲转发委外)
> 定位:leaseup 线补批,leaseup-solo 交付的接线落地与裂缝收口
> 前置:leaseup-solo 已收约(174 基线),closefix 新归并机械在役,锁面零在途

## 一、问题陈述 {#problem}

- **账单零接线**:bill_session_start 与 bill_lock 与 bill_close_unused 三函数在 core.py 在档有测试,但全代码零调用点——首笔真实账单永不发生,主树 lockface-bills.ndjson 缺席与 lockdb 无 lock_bill 表为证;leaseup-solo 结果档「账单台账由 open_execute 触发」表述与实态不符,属交付声明偏差在案
- **账单位置未定**:接线时 bills_ndjson 若走 tool_dir 缺省即重蹈工地定位陷阱(先例三连:openhyg 活体验收首跑、批 C 首跑、leaseup 批自举窗),位置必须与 --ledger/--locks 同源解析
- **T-12 未做**:core.py open_preflight 两处相对路径裂缝即相对 repo 的 cwd 兜底 resolve 分支与 worktrees_root 相对形态未绝对化,注册路径(工作区根基准)与 git worktree add 实际落位(仓内基准)分裂,leaseup 批亲历 prune 加重 add 手术(其越线申报二在案)而手术未变固定程序

## 二、关键设计 {#design}

### 2.1 接线三处

- open_execute 签发成功后调 bill_session_start(开工面计费)
- lock 命令成功取锁后调 bill_lock(首免判定承函数内 lock_bill 表计数语义)
- close 收约路径调 bill_close_unused(锁面减实际改动面逐件计费);与 closefix 并集归并新机械交互即账单台账属追加面,收约全路径零离盘实测
- 接线即活体:本批自身的开工与加锁与收约产生真实首笔账单落主树正典位,ndjson 与 lock_bill 双跑一致

### 2.2 位置单源

- bills_ndjson 解析与 sessions/--ledger 同源:显式参优先,缺省 tool_dir,禁独立缺省第三条路
- 防陷阱夹具:自工地 cwd 调用形下账单仍落主树正典位或显式位,不落工地位

### 2.3 T-12 绝对化

- open_preflight 输出的 repo 与 worktree 一律 resolve 绝对化后注册再调 git;相对 repo 的 cwd 兜底分支删除改根相对单源;close 消费台账历史相对串时同 resolve 兼容旧行
- 回归夹具:相对 root 与相对 repo 传参下注册路径与 git worktree add 实际落位一致,夹具真跑 git 非 mock

### 2.4 裁定承前

- 计费语义已过一裁(m-leaseup-bill-1 终签 1fda6a88),本批为该裁语义的接线落地;批内判定:若判纯实装零语义变化即声明承前裁并落依据,若判需新裁即跑 facet 合同模式 near_threshold 呈用户,不自行终签

## 三、工作清单 {#work}

- [ ] T-1 三调用点接线(open_execute 与 lock 与 close)
- [ ] T-2 bills_ndjson 位置单源解析与防工地陷阱夹具
- [ ] T-3 T-12 绝对化:open_preflight 双裂缝修复加 close 消费兼容
- [ ] T-4 TDD 先红后绿:接线红(调用位缺席)、位置陷阱红(工地 cwd 形)、路径基准红(相对传参分裂形),三族转绿
- [ ] T-5 活体验收:本批自身全流程真实账单三事件落主树正典位,双跑一致
- [ ] T-6 BATCH-FACE 坑位更新(lease repo 路径条目随 T-12 改写,worktree 相对路径坑注记)
- [ ] T-7 全测试族零回归与双仓 settle 加 close(新机械)加 reconcile 加 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 接线在位 | 三调用点在档,移除任一即测试红 |
| **F-2** | 真实首单 | 本批活体 bill 三事件落主树 lockface-bills.ndjson,lockdb lock_bill 表同笔在册,双跑一致 |
| **F-3** | 位置单源 | 工地 cwd 调用形夹具账单不落工地位;显式参传参落显式位 |
| **F-4** | 路径基准 | 相对 root 与相对 repo 夹具注册路径与 git 实际落位逐字节一致;cwd 兜底分支灭失 |
| **F-5** | 裁定 | 承前裁声明或新裁在档,依据落结果档 |
| **F-6** | 零回归 | 174 基线全绿加新增全绿;closefix 并集归并与 openhyg 与批 C 与 leaseup 交付零动;台账行格式零变更 |

## 五、必读文件 {#read}

- 死代码现场:sih-tools/lease/src/lease/core.py 三 bill 函数与 open_execute 与 open_preflight
- 先例:openhyg-solo-results.md 申报一(工地定位)与 leaseup-solo-results.md 越线申报二(路径错位手术)
- 前裁:m-leaseup-bill-1 材料(siha-tools proposition/DES/m-leaseup-bill-1/)与终签链笔 1fda6a88
- 机械链:sih-tools/BATCH-FACE.md(含全部勘误节)

## 六、约束 {#constraints}

1. 计费数值与语义零改动(前裁冻结),只接线不改判定;若判需新裁即跑不自行终签
2. 台账行格式零变更;sessions 与 locks 与 bypass 零改面;ledgerwrite 唯一写点零动;账单台账走 append_row 写点
3. 历史台账行相对串兼容:close 消费旧行 resolve 不改写原行(append-only 零改写)
4. scribe 裸调逐笔 grep 禁 meter 包裹掩败;退出码真值核禁管道掩
5. 范畴排除未裁四项不动:T-9 执法位、四陈旧会话销账、wenguobs 双写、locks 镜像
6. 收约走 closefix 新机械;任务包绝对路径;des-001 域外 exit-2 如实记
7. 开工前实查锁面,撞在途即 wait-turn 排队不抢不绕

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 双仓 settle + close(新机械) + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/billwire-solo-results.md 与 materials/
- [ ] lease CONTRACT 修订与 CALL-LOG 双笔与 BATCH-FACE 坑位随批

## 八、风险点 {#risks}

- 接线与 closefix 并集归并交互:close 路径新增账单写点须过纯追加语义(实测夹具)
- 位置单源改动波及 lock 命令签名:显式参透传保持向后兼容
- 活体账单依赖本批自身全流程:若本批 allow 面未用锁即 bill_close_unused 零事件,如实记不硬造

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- 上游:leaseup-solo(死代码与裂缝来源)与其验收报告(主会 2026-09-06)
- 前裁:m-leaseup-bill-1
- 下游:置信度台账消费(pending pk-073 建模批)

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/(core.py、cli.py、lockcore.py 若及)与 CONTRACT.md 与 CALL-LOG.md
- sih-tools/lease/tests/(三族夹具)
- sih-tools/BATCH-FACE.md(坑位更新)
- sih-engine/sih/event/plan/billwire-solo-results.md 与 materials/ 与 sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/ledger/lockface-bills.ndjson(首笔真实账单随批落)
- sih-engine/sih/event/trail/2026-09-06.ndjson 与后续日链
