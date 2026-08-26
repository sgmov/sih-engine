# structrev-001-t6d：DEC-001 结构修订批

> T6D task-packages 治理任务
> 承接：2026-08-26 用户结构债即过令即不能等融回否则路径与痕迹失效、DEC-001 节点树与物理仓漂移实测
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— **本任务包偏离：修订类，主线串行为主**
> 日期：2026-08-26

## 一、问题陈述 {#problem}

- 物理仓长出节点树未声明的节点即顶层 task-packages、skills、parking、probes、examples、fixtures、state、tmp、OPEN-QUESTIONS.md，doc 下 research 与 CASCADE.json，tests 与树的 test 不一致，requirement 未建。
- 治理区事件层物理围堰于 sih-tools 即 trail 与锁台账在外仓，树内 sih/event 空占位，无对应声明即融回迁移无基准。
- 顶层 state/tasks 与 sih/state/plan 职责重叠，物理双位无归位声明。

## 二、关键设计 {#design}

### 2.1 节点收编 {#incorporate}

按消费模式与工程职责两轴归类，不新造第三轴。task-packages、skills、parking、OPEN-QUESTIONS 收编为治理流程支撑域顶层节点。probes、examples、fixtures、tests 收编为工程域节点即 Cargo 与测试约定位。doc 下 research 与 knowledge 收编为文档区节点，requirement 登记为待建空节点。tmp 不入树即临时工作区标注清理纪律。

### 2.2 围堰对应与归位映射 {#mapping}

新增节声明逻辑节点与物理载体的对应即 trail 对应 sih-tools/scribe/trail、锁台账对应 sih-tools/lease/ledger、工具集对应外仓 sih-tools、级联投影对应 doc/CASCADE.json。融回按映射表迁移，trail 历史行不改写即事件只追加，旧路径引用经映射表解析不失效。

### 2.3 归位待迁移件 {#pending-move}

顶层 state/tasks 声明为 sih/state/plan 的物理早期位，归位动作归后续批。sih/static 声明为治理区静态档节点。

## 三、工作清单 {#work}

### Cluster 1：修订起草（主线写）

DEC-001 修订二即节点树更新、围堰对应与归位映射节、修订记录。

### Cluster 2：管线与认证（主线串行验证）

化格核阅检词三绿，报告落盘逐件上链。

### Cluster 3：段结算提交（主线写）

机械 message 提交，呈现待签。

## 四、判据 {#criteria}

- L1 修订后节点树覆盖全部实测物理节点即无未声明残件
- L2 收编归类只用既有两轴即消费模式与工程职责
- L3 围堰对应表含 trail、锁台账、工具集、级联投影四件
- L4 归位映射声明 trail 历史行不改写与旧路径经映射表解析
- L5 三件管线绿认证上链链 valid
- L6 段结算 message 用机械模板
- L7 全程租约在册锁覆盖写点
- L8 DEC-001 文件名不变即修订不改名

## 五、结果留档 {#results}

结果见 structrev-001-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/task-packages/structrev-001-t6d.md
- sih-engine/task-packages/structrev-001-t6d-results.md
- sih-engine/doc/decision/001-repository-structure.md
- sih-tools/scribe/
