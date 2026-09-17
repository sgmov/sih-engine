# 任务包：doorsimpl-solo（两道门实装）

## 元信息（版本位必填）

- 批名：doorsimpl-solo
- version: v1
- 日期：2026-09-07
- 范式：单线形 solo，委外代理亲写零子代理
- 令源：用户 2026-09-07「同意」即两道门实装批出任务包与提示词；本任务包为 SDD 形首个强制食客（doorprep 模板条款）

## 一、变更提案（由问题陈述升形）

# 变更提案：doorsimpl-solo

## 问题

- 两道门未实装：租约开约不验任务包 SDD 形、收约不跑四查，「不过门不开工、不过四查不收约」仍是纪律不是机械事实
- 备料已全齐且全绿：四查链主树绿（chaingreen-solo 收口）、检查器与 TDD 验收工具与基线向量管理工具三机在役、sdd-v1 五包 0.3.0、tdd-v0 判定包、flow-v1 流程包 0.1.0、SDD 形模板在位
- 委外与直写走同一租约通道，门挂上即两类执行同一受门

## 意图

把两道门挂上租约闸：开约挂 SDD 门（检查器按流程包验任务包件族），收约挂 TDD 门（验收工具按流程包跑查族），门规全数据零硬编码。

## 范畴排除

- 不做门规数据重设计：flow-v1 即门规，本批只接线不改其设计
- 不动三机本体：检查器、TDD 验收工具、基线向量管理工具零改
- 不强制存量任务类：未声明流程包的类灰度跳过并记链，不拒
- 不做覆盖机深机制与批专属判定包深化：属后继批
- 不裁两模板合一归属：候两线协调裁

## 令源

用户 2026-09-07 会话令「同意」，承接文规向界设计稿 §一（两道门与流程包）与 §九分批序的末批。

## 二、规格差分（新增件）

# 规格差分：doorsimpl-solo

## ADDED Requirements

### Requirement: R-001 SDD 门
租约 open 对声明流程包的任务类 SHALL 逐件跑检查器（按流程包 sdd_gate.documents 的包名加版本钉住），任一件退出码 1 即门拒不开工，退出码 2 即工具异常先处置不硬闯。

### Requirement: R-002 TDD 门
租约 close 对声明流程包的任务类 SHALL 跑 TDD 验收工具（按流程包 tdd_gate.checks 的判定包），聚合退出码非零即门拒不收约。

### Requirement: R-003 门事件上链
两门的每次判定 SHALL 记链事件，三态即门过、门拒、门跳过，带判定读数。

### Requirement: R-004 门实现空腹
门实现 SHALL 从流程包读取件族与查族映射，租约源码零硬编码包路径零任务类字面。

### Requirement: R-005 既有命令面零破坏
租约既有子命令、旗标、退出码语义 SHALL 零变化，门是加法不是改法。

### Requirement: R-006 灰度条款
未声明流程包的任务类 SHALL 跳过门并记链门跳过事件，不拒开工不拒收约。

## 三、场景清单（由 F 锚升形，独立成件）

# 场景清单：doorsimpl-solo

### Requirement: R-001 SDD 门
租约 open 对声明流程包的任务类 SHALL 逐件跑检查器，退出码 1 即门拒不开工。

#### Scenario: S-001 好任务包过 SDD 门
- **WHEN** 以本任务包五节为件族（节位即件位，必要时节抽取为临时件验形且不改内容）对 sdd-v1 五包逐节跑检查器
- **THEN** 五节逐包退出码全零
- 判据：sih-tools/checker 逐包跑本任务包节件 期望退出码 0 期望产物 彩排逐包读数件

#### Scenario: S-002 坏任务包被门拒
- **WHEN** open 声明流程包且任务包场景节缺判据行（坏样例即缺判据形）
- **THEN** 检查器退出码 1，门拒，open 拒绝开工，门拒事件在链
- 判据：门测试夹具跑坏样例 期望退出码 1 期望产物 门拒读数与链事件笔

### Requirement: R-002 TDD 门
租约 close 对声明流程包的任务类 SHALL 跑 TDD 验收工具，聚合退出码非零即门拒不收约。

#### Scenario: S-003 绿判定包过 TDD 门
- **WHEN** close 时四查链为绿（主树判定包端到端退出码 0）
- **THEN** TDD 门过，close 放行，门过事件在链
- 判据：门测试跑当前主树判定包 期望退出码 0 期望产物 门过读数与链事件笔

#### Scenario: S-004 红判定包被门拒
- **WHEN** close 时判定包含漂移（构造一字节期望漂移）
- **THEN** 验收工具退出码 1，门拒，close 拒绝收约，门拒事件在链
- 判据：门测试夹具构造漂移 期望退出码 1 期望产物 门拒读数与链事件笔

### Requirement: R-003 门事件上链
两门的每次判定 SHALL 记链事件，三态即门过、门拒、门跳过。

#### Scenario: S-005 门事件三态在链
- **WHEN** 门测试覆盖门过、门拒、门跳过三路径各至少一笔
- **THEN** 三态事件各至少一笔在链且带判定读数
- 判据：链 grep 三态事件各至少一笔 期望退出码 0 期望产物 三态事件清单件

### Requirement: R-004 门实现空腹
门实现 SHALL 从流程包读取映射，租约源码零硬编码包路径零任务类字面。

#### Scenario: S-006 门模块零硬编码
- **WHEN** 对租约门模块源码 grep 流程包路径与任务类字面（sdd-v1、tdd-v0、治理批类等）
- **THEN** 零命中
- 判据：grep 测试 期望退出码 0 期望产物 grep 测试读数件

### Requirement: R-005 既有命令面零破坏
租约既有子命令、旗标、退出码语义 SHALL 零变化。

#### Scenario: S-007 既有回归全绿
- **WHEN** 跑租约既有测试全族
- **THEN** 零回归全绿
- 判据：lease 既有测试全族 期望退出码 0 期望产物 全族测试读数件

### Requirement: R-006 灰度条款
未声明流程包的任务类 SHALL 跳过门并记链门跳过事件，不拒。

#### Scenario: S-008 未声明类灰度跳过
- **WHEN** open 一个未声明流程包的任务类（测试夹具形）
- **THEN** open 成功，门跳过事件在链
- 判据：门测试夹具未声明类 open 期望退出码 0 期望产物 跳过事件笔与 open 读数件

## 四、技术方案（由关键设计升形）

# 技术方案：doorsimpl-solo

技术选择逐条回链：

- 挂点选 lease open 与 close 的既有校验链内：SDD 门在 allow 面解析后、工地建立前；TDD 门在差集闸后即收约末道，门拒不留半开状态。回链 R-001, R-002
- 门判定以 dry-run 夹具先行单测（不真开约的门判定函数形），坏包拒好包过先红后绿。回链 R-001, R-002
- 流程包解析器独立函数，包名加版本钉住按 packs_home 解析，寻径失败即门拒报缺件。回链 R-004
- 门事件三态复用书简 append 通道，读数载判定摘要与包版本。回链 R-003
- 灰度默认即未声明跳过，跳过也记链。回链 R-006

## 五、任务清单（由工作清单升形）

# 任务清单：doorsimpl-solo

1. 前置读数全套（三问双门、叩问、正身、watch、泊界心跳、例行读数）。引 R-005 证据：各步退出码
2. 开工前置核：租约线收官宣告在案（attnanchor 与 confpreempt 收约、链面回退缺陷处置在案），未收官即排队候位零写入。引 R-005 证据：宣告或排队读数
3. 本任务包 SDD 门彩排：五节按包逐节过检查器。引 R-001, S-001 证据：检查器逐包读数
4. 租约开工（--package doorsimpl-solo，双仓工地，--allow 按十二节逐路径分行）。引 R-005 证据：open 退出码
5. 门模块实装（open 挂 SDD、close 挂 TDD、事件三态、流程包解析、灰度）。引 R-001, R-002, R-003, R-004, R-006 证据：实现与测试读数
6. 门测试先行红证（坏包拒、漂移拒）后全绿（好包过、绿判定包过、灰度跳过、回归全族）。引 S-002, S-004, S-005, S-006, S-007, S-008 证据：红证与绿读数归档
7. 管线：化格、核阅（域外记档）、检词（懒波词面规避）。引 R-005 证据：退出码
8. facet 三步加得一三步，m-doorsimpl-1 stable_clear 终签入链。引 R-003 证据：终签在链
9. 本批自身收约走自己刚立的 TDD 门（首个过门收约，四查对象即 flow-v1 声明的判定包）。引 R-002, S-003 证据：门过事件在链
10. settle 收约、链 verify、reconcile、心跳复算、主树复跑判定包退绿、结果档与 CALL-LOG 与完工报告。引 R-003 证据：收口读数

## 六、必读文件

- 门规数据：`sih-tools/incubation/packs/flow-v1/flow-v1.json`
- 三机与包族：`sih-tools/checker/`、`sih-tools/acceptor/`、`sih-tools/incubation/packs/sdd-v1/`、`sih-tools/acceptor/packs/tdd-v0-sddchecker.json`
- 模板与本包同形源：`sih-tools/incubation/TASK-PACKAGE-SDD-TEMPLATE.md`
- 租约结构：`sih-tools/lease/CONTRACT.md`（修订序列至 1.33.0）、`sih-tools/lease/TASK-PACKAGE-TEMPLATE.md`
- 设计稿：工作区根 `regula-design-draft-2026-09-06.md` §一§八
- 命令面：`sih-tools/BATCH-FACE.md`

## 七、约束

1. 开工前置：租约线收官宣告（在途即排队候位零写入）
2. 租约源码改动面限门模块与 open 与 close 挂点，既有子命令、旗标、退出码语义零变化；三机零改
3. 门规只吃流程包数据零硬编码（grep 测试证明）；门拒不留半开状态
4. 先红后绿：门测试先行红证归档
5. 主树零直写（批输入件先例）；守卫禁 plain commit；退出码直读禁管道掩码
6. 在泊件与在盘遗留零触碰不并批；懒波词面全称规避；中文零新造正式词

## 八、验收标准

- [ ] S-001 至 S-008 全过，R-001 至 R-006 落实
- [ ] 本批收约经自己实装的 TDD 门放行，门过事件在链
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 doorsimpl-solo-results.md 落 event/plan 含 SDD 门彩排读数节

## 九、风险点

- 租约收官时点与挂点选择的缝：挂点实装前先对 1.33.0 的 open 与 close 校验链做挂点测绘，缝大即停批上报
- 首个过门收约的四查对象是 flow-v1 声明的判定包（检查器线健在性）非批专属判定包，批专属深化属后继（范式偏离声明节在档）
- 门测试夹具不得污染正式包目录与台账（隔离位）
- 与在途批共享面：撞锁排队如实呈报，链归并按纯追加并集超集通道，真分叉停批

## 十、范式偏离声明

本任务包为 SDD 形首个强制食客：单文件五节形，节位即件位（节抽取验形不改内容）。首个过门收约的四查对象为流程包声明的判定包而非批专属判定包，批专属深化后继批承办。

## 十一、关联文件

- 工具：`sih-engine/target/debug/scribe`、`sih-engine/target/debug/attractor`、`sih-tools/facet/measure.py`、`sih-tools/lease`、`sih-tools/checker`、`sih-tools/acceptor`

## 十二、请求写入（逐路径分行书写）

- `sih-engine/sih/state/plan/doorsimpl-solo.md`
- `sih-tools/lease/src/`（门模块与挂点，目录级声明：门实装所必需的租约源码最小面）
- `sih-tools/lease/tests/`（门测试，目录级声明：测试件隔离位）
- `sih-tools/lease/CONTRACT.md`（修订条目追加）
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/doorsimpl-solo-results.md`
- `sih-engine/sih/event/plan/doorsimpl-solo-materials/`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- `sih-tools/facet/`
- `sih-tools/proposition/DES/`
- worktrees 双仓 doorsimpl-solo 工地
