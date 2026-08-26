# mergeback-sdd-t6d：书简融回SDD批

> T6D task-packages 治理任务
> 承接：2026-08-27 用户第一批融回批令即书简首件、SDD 与 TDD 范式令、rustmcp 定调、意图记录 2026-08-27-ask3-mbsdd 即 hash 5fb7ecb4
> 范式：T6-D 范式——本任务包偏离：SDD 立制类，主线串行，无子代理
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- 融回门机制在孵化环第四步有雏形即三查，但作为引擎侧可复用机制未成立，首件融回缺执行规程。
- 引擎 event_stream 已有骨架即 append 与 hash 与 verify 与 query 承 SPEC-004，对 scribe 生产面落差未钉死即 intent 与 park 与报告消费三入口加 golden 对拍壳加退出码面。

## 二、关键设计 {#design}

### 2.1 DEC-013 融回机制 {#dec013}

借鉴孵化环三查升格为引擎 DEC。门开归人节点即用户批令留痕。执行三步：引擎开发即 SDD 规格先行加 TDD 失败测试先行、双模并存切换即 T6 管线改指引擎件而工具件转兼容只读、工具侧退役即契约标注加调用册尾行加融回完成档。此后每件融回复用。

### 2.2 SPEC-006 落差规格 {#spec006}

钉死引擎侧待建面：三入口签名即 intent 双件消费、park 双动作与配对不变量、报告消费 append 即 certification_completed 八项负载；golden 对拍判据即向量集含创世与空值与缺省与非 ASCII 与时间戳边界，生产 trail 四文件全量复验；退出码三值；组件边界即库加命令行二元先立、MCP 暴露位承引擎主线 DES-020；TDD 测试计划逐判据先红后绿。

### 2.3 范式约束 {#paradigm}

SDD 即本批只出 DEC 与 SPEC 不写实现。TDD 即实现批每判据先写失败测试后补实现，红转绿留痕。

## 三、工作清单 {#work}

### Cluster 1：三问与开工链（主线）

意图双腿验与上链、包档登记 wip、正身、open、锁、双仓副本。

### Cluster 2：DEC-013（副本流）

融回机制三步与三查升格，落 doc/decision/013-mergeback-gate.md。

### Cluster 3：SPEC-006（副本流）

落差规格与 TDD 测试计划，落 doc/spec/SPEC-006-scribe-mergeback-gap.md。

### Cluster 4：管线与段结算与收约

化格核阅检词、认证上链、双仓段结算、收约归并。

## 四、判据 {#criteria}

- L1 DEC-013 三步与三查与门开留痕齐备且与用户批令一致
- L2 SPEC-006 落差清单与 scribe 生产面对表无漏项
- L3 三入口签名与数据契约与配对不变量机械可验
- L4 golden 对拍判据含向量集边界与生产四文件复验
- L5 TDD 测试计划逐判据可先红后绿执行
- L6 管线绿认证上链链 valid
- L7 双仓段结算经正身路径、收约归并成、零在役锁

## 五、结果留档 {#results}

结果见 mergeback-sdd-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/doc/decision/
- sih-engine/doc/spec/
- sih-engine/sih/state/plan/mergeback-sdd-t6d.md
- sih-engine/sih/state/plan/mergeback-sdd-t6d-results.md
- sih-tools/scribe/
