# retire-t6d：书简融回退役批

> T6D task-packages 治理任务
> 承接：2026-08-27 用户批准令、DEC-013 第三步、意图记录 2026-08-27-ask3-retire
> 范式：T6-D 范式——本任务包偏离：退役收口类，主线串行，无子代理
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- 切换批已毕即引擎件为生产写入位，trail 四件仍在工具侧旧居，家位迁移与日常对表缺省联动未做，退役标注与三查完成档缺位即融回门未闭。

## 二、关键设计 {#design}

### 2.1 家位迁移 {#migrate}

trail 四件跨仓迁移即工具仓删引擎仓增，逐字节不动，迁移后 scribe verify 与 scribegate verify 对新家全量验链四件全 valid 为判据。本批起一切链写入走新家。

### 2.2 lease 随迁 {#leasebump}

lease 缺省 trail 路径改指新家升 1.5.0，测试随行，契约修订十一，老路径不再缺省。

### 2.3 退役标注与完成档 {#retire}

scribe 契约修订六即退役声明与切换日期与承继者指称，调用册尾行收官。完成档落 sih/event/mergeback/ 即三查对表逐项证据。AGENTS 三处对齐。

## 三、工作清单 {#work}

### Cluster 1：三问与开工链（主线）

意图双腿验、包档登记 wip、正身、open、锁七件。

### Cluster 2：家位迁移（主检出）

四件跨仓移动加双验。

### Cluster 3：lease 随迁（副本流）

1.5.0 缺省路径加测试加契约修订十一。

### Cluster 4：退役标注与完成档与实态（副本流与根域）

修订六、尾行、完成档、AGENTS 三处。

### Cluster 5：管线与段结算与收约

意图经引擎件入新家、化格核阅检词、认证经引擎件、双仓段结算、收约归并。

## 四、判据 {#criteria}

- L1 迁移后双验即两侧验证器对新家四件全量 valid
- L2 lease 1.5.0 缺省 trail 即新家且对表免参可跑
- L3 修订六与尾行与完成档三查对表齐备
- L4 本批意图与认证经引擎件落新家即新家首写
- L5 管线绿认证上链链 valid
- L6 双仓段结算经正身路径、收约归并成、零在役锁

## 五、结果留档 {#results}

结果见 retire-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/sih/event/trail/
- sih-engine/sih/event/mergeback/
- sih-engine/sih/state/plan/retire-t6d.md
- sih-engine/sih/state/plan/retire-t6d-results.md
- sih-tools/scribe/
- sih-tools/lease/
- sih-tools/nomenclator/
- AGENTS.md
