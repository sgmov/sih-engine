# lease-merge-t6d：租约立名登记与并库施工批

> T6D task-packages 治理任务
> 承接：2026-08-26 用户双签租约／lease、DEC-011 已签、worktree-2locks-t6d 全簇、ask3-src-t6d
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— **本任务包偏离：实施类，主线串行为主**
> 日期：2026-08-26

## 一、问题陈述 {#problem}

- 双名已签即租约与 lease，第三段登记未行即词条、血统档簇九、死档续档、连带改写未落。
- 并库裁定未施工即工地外壳与锁本体两工具两壳两台账，acquire 与 release 子命令未归 lock 与 unlock。
- close 只拆不检即锁未清零可收工、副本分支无归并无删除。

## 二、关键设计 {#design}

### 2.1 立名登记 {#naming-register}

词条租约／lease 入检词 established，血统档簇九落档承推导史，死档续档即 worktree 借机制名、工地机制位偏差、匝道不批、摆渡百度同音、inout 语言关键字、backor 后门近形、prune 同域子命令、双锁锁具锁钥分立项死、锁本体退役，候补档即 replica、escrow、tributary、outin、瓮城、渡口、营盘落选存候补。

### 2.2 并库 {#merge}

sih-tools/worktree 与 sih-tools/locks 合并为 sih-tools/lease，包名 lease，六子命令即 open、lock 即原 acquire、check、status、unlock 即原 release、close。双台账随迁即 sessions.ndjson 与 locks.ndjson 同居 lease/ledger，历史行原文保留。两调用册合并为一。

### 2.3 close 三检 {#close-gates}

close 加三道检查按序：租内锁清零即本会话在锁非零拒收工；副本分支有新提交即归并主线后删支、无提交即跳归并直接删支；工地移除。认证门与编辑位迁移留编辑位迁移批。

## 三、工作清单 {#work}

### Cluster 1：登记与连带改写（主线写）

词条登记、PRO-007 簇九与死档续档、AGENTS 两行并一行、DEC-011 工具位更新、任务包与结果档。

### Cluster 2：并库施工（主线写）

目录迁移与包重命名、子命令改名、close 三检实现、测试合并全绿、旧目录清理。

### Cluster 3：治理收尾（主线串行验证）

真会话开工全目标锁、三件管线绿认证上链、收工即新 close 三检实跑、双仓提交。

## 四、判据 {#criteria}

- L1 词条与血统档落位即簇九与死档续档入档核阅绿
- L2 并库后 lease 六子命令全可用测试全绿
- L3 lock 与 unlock 语义与原 acquire 与 release 逐字对齐
- L4 close 锁未清零拒收工退出码一
- L5 close 有提交分支归并后删支、无提交跳归并删支
- L6 双台账随迁历史原文保留
- L7 真会话全链即意图到会话到锁到写到收工
- L8 写面封闭即源码写调用仅台账与 git

## 五、结果留档 {#results}

结果见 lease-merge-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/task-packages/lease-merge-t6d.md
- sih-engine/task-packages/lease-merge-t6d-results.md
- sih-engine/doc/proposal/PRO-007-naming-clearance-draft.md
- sih-engine/doc/decision/011-locks-and-package-binding.md
- AGENTS.md
- sih-tools/worktree/
- sih-tools/locks/
- sih-tools/lease/
- sih-tools/scribe/
