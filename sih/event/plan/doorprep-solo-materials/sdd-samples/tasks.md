# 任务清单：两道门备料批 doorprep-solo

1. 成文流程包数据形 flow-v1.json 落 sih-tools/incubation/packs/flow-v1/，首例治理批类映射（SDD 门五件加 TDD 门四查）条目齐。引 R-001, R-002 证据：flow-v1.json 在位清单（产物哈希）
2. 成文 SDD 形任务包模板 TASK-PACKAGE-SDD-TEMPLATE.md，升形映射表七行落位，硬规则显式引用 TASK-PACKAGE-TEMPLATE 原件批前哈希对表。引 R-003, R-004 证据：模板件在位加原件 sha 对表记录（产物哈希）
3. 样例五件按模板生成（proposal、spec-delta、scenarios、tech-design、tasks）落批材料 sdd-samples/，以本批自身变更为对象，引用闭包真闭。引 S-001, S-002, S-003, S-004, S-005 证据：样例五件在位清单（产物哈希）
4. 好样例五件过检查器五跑全绿（退出码直读留读数），坏样例 bad-scenarios.md 检查器红证归档批材料。引 S-005, S-006 证据：五跑读数加红证文件（退出码）
5. 管线三步（化格、核阅域外记档、检词）过 flow-v1、模板件、样例五件。引 S-001, S-003 证据：管线三步退出码读数（退出码）
6. facet 与得一：m-doorprep-1 出题回填计分，check、verify、sign stable_clear 终签落链。引 S-005 证据：终签哈希（链事件引用）
7. 双仓 settle、放锁收约、对账对表、结果档落 event/plan。引 S-005 证据：双仓提交号与链 verify 读数（退出码加链事件引用）
