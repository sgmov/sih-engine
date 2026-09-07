# 技术方案：两道门备料批 doorprep-solo

技术选择逐条回链（层间纪律：规格层 WHAT/WHY 在 spec-delta 与 scenarios，本件只承载 HOW）。

- 流程包落位 sih-tools/incubation/packs/flow-v1/，单数据件 flow-v1.json，纯 JSON 数据形与 sdd-v1 包族同构（识别面、出处面、映射条目三面），零外部依赖。回链 R-001, R-002
- 门规映射逐条挂包名加版本（sdd-v1 五包 @0.3.0、tdd-v0 判定包 @0.2.0），门实装批按出处面版本钉住接线。回链 R-002
- SDD 形任务包模板落 sih-tools/incubation/TASK-PACKAGE-SDD-TEMPLATE.md（目录级新文件出生地，流程包数据件与模板同家位 incubation），硬规则以显式引用承载不改 TASK-PACKAGE-TEMPLATE 原件（批前 sha256 对表在批材料）。回链 R-003, R-004
- 升形映射表落模板件尾节，既有任务包节逐节对表，缝以映射表承载不留暗缝。回链 R-003
- 样例五件落批材料 sdd-samples/（引擎仓 event/plan 材料位），以本批自身变更为对象；坏样例 bad-scenarios.md 落同目录隔离位不污染 sdd-v1 正式包目录。回链 R-005
- 自证程序用 sih-tools/checker 检查器实跑（使用不是修改），五包判定退出码直读，坏样例红证归档批材料。回链 R-005
