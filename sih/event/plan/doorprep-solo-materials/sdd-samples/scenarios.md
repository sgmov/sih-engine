# 场景清单：两道门备料批 doorprep-solo

### Requirement: R-001 流程包数据形
流程包 flow-v1 SHALL 以纯数据形声明任务类门规映射且门实装批零硬编码直接接线。

#### Scenario: S-001 流程包数据形落位读数
- **WHEN** 读 sih-tools/incubation/packs/flow-v1/flow-v1.json 并按 JSON 解析
- **THEN** 包名 flow-v1、版本 0.1.0、任务类治理批类条目在位，SDD 门件族五条加 TDD 门查族四条映射齐
- 判据：json 解析加键面点查 期望退出码 0 期望产物 flow-v1.json

### Requirement: R-002 首例治理批类映射
flow-v1 首例 SHALL 声明治理批类 SDD 门吃 sdd-v1 五包与 TDD 门吃 tdd-v0 判定包形。

#### Scenario: S-002 首例映射条目读数
- **WHEN** 读 flow-v1.json 治理批类条目
- **THEN** SDD 门 documents 五条各挂 sdd-v1 包名加版本，TDD 门 checks 四条各挂 tdd-v0 判定包加 TC 编号
- 判据：json 解析加条目点查 期望退出码 0 期望产物 flow-v1.json 映射条目清单

### Requirement: R-003 SDD 形任务包模板
SDD 形任务包模板 SHALL 承载升形映射表与租约结构节保留。

#### Scenario: S-003 模板升形映射表在位
- **WHEN** 读 sih-tools/incubation/TASK-PACKAGE-SDD-TEMPLATE.md
- **THEN** 升形映射表七行在位（问题陈述升变更提案、规格差分新增、F 锚升场景清单、关键设计升技术方案、工作清单升任务清单、结构节保留、请求写入引用）
- 判据：模板件节面点查 期望退出码 0 期望产物 TASK-PACKAGE-SDD-TEMPLATE.md

### Requirement: R-004 硬规则吸收显式引用
SDD 形任务包模板 SHALL 显式引用 TASK-PACKAGE-TEMPLATE 承载硬规则且不改彼件。

#### Scenario: S-004 模板引用与原件零改读数
- **WHEN** 读 TASK-PACKAGE-SDD-TEMPLATE.md 的请求写入节与元信息节
- **THEN** 逐路径分行与版本位必填条款示式引用 TASK-PACKAGE-TEMPLATE，且该原件 sha 对表批前零变
- 判据：sha256sum TASK-PACKAGE-TEMPLATE.md 对表 期望退出码 0 期望产物 批前哈希对表记录

### Requirement: R-005 模板门形机器自证
SDD 形任务包模板 SHALL 以门形机器自证：好样例五件过检查器全绿，坏样例红证在档。

#### Scenario: S-005 好样例五件检查器全绿
- **WHEN** 以 sdd-v1 五包逐件跑本批样例五件（proposal、spec-delta、scenarios、tech-design 加 reference scenarios、tasks 加 reference scenarios）
- **THEN** 检查器五跑退出码全零 verdict 全 pass 零 findings
- 判据：检查器五跑 期望退出码 0 期望产物 五跑读数（退出码）

#### Scenario: S-006 坏样例检查器红证
- **WHEN** 以 scenario-list 包跑坏样例 bad-scenarios.md（S- 编号重号加缺判据行）
- **THEN** 检查器退出码 1 verdict fail，重号与缺判据行三态失败定位在报
- 判据：检查器跑 bad-scenarios.md 期望退出码 1 期望产物 坏样例红证（first-red 留档）
