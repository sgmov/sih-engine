# 规格差分：两道门备料批 doorprep-solo

## ADDED Requirements

### Requirement: R-001 流程包数据形
文规流程包 flow-v1 SHALL 以纯数据形声明任务类的门规映射：SDD 门件族（逐件挂格式包名加版本）与 TDD 门查族（逐项挂判定包名加版本），数据形与格式包及判定包同构（manifest 风即名、版本、映射条目）。

### Requirement: R-002 首例治理批类映射
flow-v1 首例 SHALL 声明治理批类的 SDD 门吃 sdd-v1 五包（change-proposal、spec-delta、scenario-list、tech-design、task-list）与 TDD 门吃 tdd-v0 判定包形（TC-001 至 TC-004 四查）。

### Requirement: R-003 SDD 形任务包模板
SDD 形任务包模板 SHALL 承载升形映射表（问题陈述升变更提案、关键设计升技术方案、工作清单升任务清单、F 锚升 R-／S- 场景形、补规格差分一件）并保留租约结构节（必读、约束、验收、风险、关联、请求写入）。

### Requirement: R-004 硬规则吸收显式引用
SDD 形任务包模板 SHALL 显式引用 sih-tools/lease/TASK-PACKAGE-TEMPLATE.md 承载逐路径分行与版本位必填等硬规则且不改彼件（租约线资产）。

### Requirement: R-005 模板门形机器自证
SDD 形任务包模板 SHALL 以门形机器自证：以本批自身为变更对象按模板生成样例五件，好样例过检查器（sdd-v1 五包判定）全绿，坏样例（缺判据行或重号）检查器红证在档。
