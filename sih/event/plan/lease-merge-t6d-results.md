# lease-merge-t6d 结果档

> 承 lease-merge-t6d.md，2026-08-26 首跑结果。

## Cluster 1 登记与连带改写 {#cluster1}

毕。词条租约／lease 入检词 established，血统档簇九落档即六轮猎词推导史，死档续档三十余词死因入档，AGENTS 两行并一行，DEC-011 工具位表述随并库更新加修订三。

## Cluster 2 并库施工 {#cluster2}

毕。目录经 git mv 即 worktree 迁 lease 历史可溯，锁件吸收为 lockcore 即 locks 目录清理，包名 lease 升 1.0.0，六子命令即 open、lock、check、status、unlock、close，acquire 与 release 更名 lock 与 unlock 语义逐字对齐，close 三检实装即租内锁清零、分支归并删支两态、拆本吊销，open 记基线分支为归并目标位。双台账随迁即 ledger 下 sessions 与 locks 两册历史原文保留，调用册两册合一。二十测全绿含归并实测即副本提交经 close 归并基线分支产物落盘。

## Cluster 3 治理收尾 {#cluster3}

毕。真会话 96c7cf78 即意图 round2 挂链 2ba8b5dc 双腿验收开工，十锁全持，收工经新 close 三检实跑即锁清零、无提交分支跳归并删支、双仓拆本、吊销入档。披露两件即本会话以旧名开工新名收工属并库自举，意图初版引文被引擎 ask3gate 打回即逐字替换后按被拒重提纪律 round2 上链，初版事件 d81345f6 与修正版事件 2ba8b5dc 同链在案。

## F 锚定验证表 {#f-table}

| 锚定 | 判据 | 结果 |
|---|---|---|
| L1 词条与血统档落位 | 检词登记加 PRO-007 簇九 | 过 |
| L2 六子命令全可用 | 二十测全绿 | 过 |
| L3 lock 与 unlock 语义对齐 | 原五验与锁态规则原样 | 过 |
| L4 close 锁未清零拒收工 | held_locks 测试退出码一 | 过 |
| L5 close 归并删支两态 | 有提交归并产物落盘与无提交跳归并删支两测 | 过 |
| L6 双台账随迁原文保留 | sessions 与 locks 两册行未改 | 过 |
| L7 真会话全链 | 96c7cf78 意图到会话到锁到收工 | 过 |
| L8 写面封闭 | 源码扫描测试 | 过 |

## 范式验证 {#paradigm}

实施类单代理直跑，跑前立文含 F 锚定与请求写入段，偏离声明在任务包头部。
