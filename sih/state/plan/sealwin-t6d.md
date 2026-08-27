# sealwin-t6d：对表封窗批

> T6D task-packages 治理任务
> 承接：2026-08-27 用户批准 facet 补正方案捎主线一句、追认令与修订十、意图记录 2026-08-27-ask3-sealwin 即 hash 24c8a73c
> 范式：T6-D 范式——本任务包偏离：封窗小批，主线串行，无子代理
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- facet 两笔裸写 3451c6f 与 1592b4a 落追认令后永不入表，日常对表 tools 侧退出码常一即常亮红灯，真新增量会被淹。
- 用户已批准补正方案即 facet 侧补正会话加主线侧基线前移，本批承主线半。

## 二、关键设计 {#design}

### 2.1 界线前移 {#forward}

SEAL_BASES 即 sih-tools 界线自 6b5855c 前移至 d7f9338 即 retire 批归并点，窗内含两笔与十八笔追认存量即全部落为窗前事实，engine 界线不动。改线走版本管理升 1.6.0。

### 2.2 两笔点名留档 {#named}

契约修订十二点名 3451c6f 与 1592b4a 即已处置窗前事实，追认令永不入表条款不动即不进 SEAL_EXEMPTS，历史不改写名字留契约。

### 2.3 信号复位 {#signal}

终态判据即双仓免参对表退出码零，facet 补正会话与本批提交皆在窗后须路由可见。

## 三、工作清单 {#work}

### Cluster 1：三问与开工链（主线）

意图经引擎件入链、包档 wip、正身、open、锁。

### Cluster 2：封窗（副本流）

SEAL_BASES 前移、契约修订十二、升 1.6.0、测试全绿。

### Cluster 3：管线与段结算与收约

结果档、化格核阅检词、认证经引擎件、双仓段结算、收约、终态双仓对表零。

## 四、判据 {#criteria}

- L1 界线前移即 SEAL_BASES 即 sih-tools 为 d7f9338 且升 1.6.0
- L2 修订十二点名两笔与追认令不动声明齐备
- L3 测试全绿
- L4 管线绿认证上链链 valid
- L5 双仓段结算经正身路径、收约归并成、零在役锁
- L6 终态双仓免参对表退出码零

## 五、结果留档 {#results}

结果见 sealwin-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/sealwin-t6d.md
- sih-engine/sih/state/plan/sealwin-t6d-results.md
- sih-tools/lease/
- sih-tools/scribe/
- sih-tools/nomenclator/
