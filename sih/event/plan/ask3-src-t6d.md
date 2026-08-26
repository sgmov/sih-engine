# ask3-src-t6d：三问组件 src 实装批

> T6D task-packages 治理任务
> 承接：GOV-003 第 2 段、DEC-006 三问正式化、DES-013 组件设计、SPEC-005 接口规格、SETTLEMENT-001 遗留与指针节
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— **本任务包偏离：实施类，主线串行为主**
> 日期：2026-08-25

## 一、问题陈述 {#problem}

- 守位断口即三问触发纪律活在 skill 文本依赖代理自觉，2026-08-25 三治理批跳过即第二次违约，机械触发位缺位。
- 组件未落地即五组件名册零 src 实装，手动载体与引擎组件的双形态期无引擎侧本体。
- 意图到会话到锁到写的机械链缺首环即会话签发不验意图，无意图亦可开会即链头开放。

## 二、关键设计 {#design}

### 2.1 组件形态与落位 {#component-form}

三问承载引擎核心治理职能，按孵化纪律留仓实现即引擎 Rust src/ask3repeater，命名承 DEC-006 五件套即代码标识符 ask3repeater。诘察认知本体不实装即三问本质是 LLM 使用框架承 DEC-006 LLM 边界，组件提供记录类型与验收与事件构建的确定性面。写入下游即 event_stream append，intent_refined 事件类型已登记 SPEC-004 事件类型层级即 2026-08-22 随书简修订三。

### 2.2 错误四类验收 {#validation}

形态即必填字段与类型，契约即锚点非空与认知域深度双检与轮次，血统即哲学引文逐字比对原文与出处存在与证据行号落界，计量即 calls 单调。Rust 类型为格式权威源，验收函数零 LLM。

### 2.3 会话挂意图 {#intent-chaining}

工地外壳升 0.3.0 即 open 增 --intent 意图记录参数，经核阅 ask3 包子进程验收退出码零方签发，会话档增 intent 节即记录哈希与验收者与锚点数。机械链即无意图无会话无锁无写。批间小写的强制门留引擎运行时全量闭合位，本批登记不实装。

### 2.4 双形态期 {#dual-form}

skill 手动载体与引擎模块并存，降回时点须再裁承 DEC-006 负面后果，结果档明示并存态。

## 三、工作清单 {#work}

### Cluster 1：引擎 Rust 模块（主线写）

src/ask3repeater 四件即 record 记录类型、validate 错误四类验收、gate 意图事件构建、mod 导出，event_stream 的 EventInput 增可选 event_class 字段，cargo test 全绿。

### Cluster 2：工地外壳 0.3.0（主线写）

open 挂 --intent 即 ask3 子进程验收，无意图或验收非零拒签发，会话档载 intent 节，测试全绿。

### Cluster 3：治理收尾（主线串行验证）

机械链真机演示即意图到会话到锁到写，结果档、AGENTS 索引、三件管线、认证上链、双仓提交。

## 四、判据 {#criteria}

- A1 cargo test 全绿即四类错误各一触发与合格记录通过
- A2 血统核验机械可证即引文逐字比对原文与证据行号落界
- A3 意图事件构建即 event_type intent_refined 加 event_class record_only 加 doc_id 即会话标识
- A4 会话挂意图即无意图件或验收非零 open 拒，会话档载记录哈希
- A5 机械链闭合演示即意图到会话到锁到写真机跑通
- A6 写面封闭即 Rust 模块零 IO 除读原文与证据文件，Python 零写除台账
- A7 复演确定性即同参双跑一致
- A8 双形态登记即结果档明示 skill 并存与降回另裁

## 五、结果留档 {#results}

结果见 ask3-src-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/task-packages/ask3-src-t6d.md
- sih-engine/task-packages/ask3-src-t6d-results.md
- sih-engine/src/
- sih-engine/Cargo.toml
- AGENTS.md
- sih-tools/worktree/
- sih-tools/scribe/
