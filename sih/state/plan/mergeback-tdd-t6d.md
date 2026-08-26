# mergeback-tdd-t6d：书简融回TDD批

> T6D task-packages 治理任务
> 承接：2026-08-27 用户开工令、SPEC-006 落差规格与 T1 至 T6 测试计划、DEC-013 第一步、意图记录 2026-08-27-ask3-mbtdd 即 hash 4862b32d
> 范式：T6-D 范式——本任务包偏离：TDD 实现类，主线串行，无子代理
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- SPEC-006 已钉落差即三入口与对拍壳与命令行面，实现缺位，T1 至 T6 全红待建。

## 二、关键设计 {#design}

### 2.1 先红 {#red}

按 T1 至 T6 先写测试与桩模块，测试先行入口未建即红态，红态计数留痕。

### 2.2 后绿 {#green}

实现三入口即 certify 报告消费八项负载、intent 双件消费零发现放行、park 配对不变量链序重放；向量集五类边界冻结入 fixtures/golden；scribegate 五子命令退出码三值；生产 trail 四文件全量复验。

### 2.3 不重构 {#norefactor}

既有 append 四验与哈希公式不动即只扩展新模块，库面无 CLI 耦合。

## 三、工作清单 {#work}

### Cluster 1：三问与开工链（主线）

意图双腿验与上链、包档登记 wip、正身、open、锁、双仓副本。

### Cluster 2：先红（副本流）

测试六组与桩模块，cargo test 红态留痕。

### Cluster 3：后绿（副本流）

三入口实现与向量集冻结与 scribegate，cargo test 全绿。

### Cluster 4：管线与段结算与收约

结果档红绿计数、化格核阅检词、认证上链、双仓段结算、收约归并。

## 四、判据 {#criteria}

- L1 红态先行即测试先于实现且红态计数在档
- L2 T1 向量集五类边界冻结且复算一致
- L3 T2 生产四文件全量复验 valid
- L4 T3 与 T4 与 T5 三入口行为合 SPEC-006
- L5 T6 五子命令退出码三值
- L6 管线绿认证上链链 valid
- L7 双仓段结算经正身路径、收约归并成、零在役锁

## 五、结果留档 {#results}

结果见 mergeback-tdd-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/src/
- sih-engine/fixtures/
- sih-engine/Cargo.toml
- sih-engine/sih/state/plan/mergeback-tdd-t6d.md
- sih-engine/sih/state/plan/mergeback-tdd-t6d-results.md
- sih-tools/lease/
- sih-tools/nomenclator/
- sih-tools/scribe/
