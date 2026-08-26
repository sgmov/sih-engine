# switch-t6d：书简融回切换批

> T6D task-packages 治理任务
> 承接：2026-08-27 用户开工令与租约确认、DEC-013 第二步双模并存、意图记录 2026-08-27-ask3-switch 即引擎件首写 hash 956866c5
> 范式：T6-D 范式——本任务包偏离：切换类，主线串行，无子代理
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- TDD 批已证引擎件三入口与退出码与向量集，生产首写未行，认证位与意图位仍在工具侧 scribe。
- 双模并存需切换判据即双跑一致在案，与只读声明与索引实态对齐。

## 二、关键设计 {#design}

### 2.1 切换即本批自举 {#selfhost}

本批意图已由 scribegate 入链即 hash 956866c5 为引擎件首笔生产写入，本批认证亦经 scribegate append 落链，失败即回退 scribe 并如实登记。

### 2.2 双跑一致 {#dualrun}

每次引擎件写入后 scribe verify 与 scribegate verify 同链跑，判据即两侧皆 valid 且事件数与末哈希一致，证据入结果档。

### 2.3 只读声明与实态对齐 {#readonly}

scribe CONTRACT 修订即兼容只读自本批起，AGENTS 三处即认证位改指、scribegate 调用式、文件索引。trail 物理位置不动即家位迁移承退役批。

## 三、工作清单 {#work}

### Cluster 1：三问与开工链（主线）

意图经引擎件入链、包档登记 wip、正身、open、锁。

### Cluster 2：只读声明与实态（副本流）

scribe CONTRACT 修订于副本、AGENTS 三处于根域直改。

### Cluster 3：管线与认证即引擎件（副本流与主线）

化格核阅检词照旧，认证经 scribegate 加双跑一致证据。

### Cluster 4：段结算与收约

调用册两行、双仓段结算、收约归并。

## 四、判据 {#criteria}

- L1 引擎件首笔生产写入在链即意图事件 956866c5
- L2 双跑一致即本批每次写入后两侧 verify 同事件数同末哈希
- L3 scribe 只读声明与 AGENTS 三处与实态一致
- L4 管线绿认证上链链 valid 即认证事件出自引擎件
- L5 双仓段结算经正身路径、收约归并成、零在役锁

## 五、结果留档 {#results}

结果见 switch-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/switch-t6d.md
- sih-engine/sih/state/plan/switch-t6d-results.md
- sih-tools/scribe/
- sih-tools/lease/
- sih-tools/nomenclator/
- AGENTS.md
