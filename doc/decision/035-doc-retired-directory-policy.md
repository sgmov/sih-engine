# DEC-035 文档退役目录与数据件归桶政策

本政策处置 doc 根级两件的去向与 DES-001 的域界条款。doc 根级两件即过期治理件 doc/AGENTS-RETIRED-2026-09.md 与数据件 doc/CASCADE.json：前者归新设退役目录，后者归 knowledge 载体。目录归置只动地址不改内容，数据件与行文材料的域界随本政策显式声明。

## 概览 {#overview}

- 新设 doc/retired/ 承载过期治理件，AGENTS-RETIRED 迁入保留原名::[退役目录](#retired-dir)
- doc/CASCADE.json 迁 doc/knowledge/CASCADE-decidable-cascade-v1.json，decision/ 只承载编号裁定件::[数据件归桶](#data-bucket)
- DES-001 行文禁令只及 .md 行文材料，.json 与 .ndjson 数据件不入行文约束::[域界条款](#scope-clause)
- 工作区根 AGENTS.md 指针随直改链笔批后更新，不入本批 git 面::[指针更新](#pointer-update)
- 验收三条即根级两件清零、退役目录与归位件在档、引擎侧引用零指旧路径::[验收](#acceptance)

## 退役目录 {#retired-dir}

doc/retired/
: 新设目录，承载过期治理件。已退役而不销档的历史文档归此，全文保留可读与可引用；目录级归置不改写件内容，迁移即移动。

AGENTS-RETIRED-2026-09
: doc/AGENTS-RETIRED-2026-09.md 迁入 doc/retired/ 保留原名。该件是历史沿革的归档承载，退役不销档，迁址后旧引用以路径更新承接。

## 数据件归桶 {#data-bucket}

CASCADE 迁移
: doc/CASCADE.json 迁 doc/knowledge/CASCADE-decidable-cascade-v1.json。理由：该件是级联决策图数据件，归 knowledge 载体；decision/ 目录只承载 DEC-NNN 编号裁定件，数据件混居裁定目录使目录身份不纯。

## 域界条款 {#scope-clause}

行文域界
: DES-001 的字符与排版与结构禁令只及 .md 行文材料；.json 与 .ndjson 数据件不入字符集与表格等行文约束，数据件的格式归各数据规格承载。

域外实证
: 核阅工具 scrutinator 对 doc/CASCADE.json 判 exit 2 域外；des-001 包域声明的 include 族全为 .md 通配，即 sih-engine/doc/**/*.md 与两夹具通配，.json 与 .ndjson 不在域内。证据件在批材料 rulenorm-materials/json-scope-evidence.json。

## 指针更新 {#pointer-update}

工作区根 AGENTS.md
: 其文件索引指向两件旧路径的指针，随直改链笔批后更新，不入本批 git 面；本批 git 面只含 doc 内迁址与归位。

## 验收 {#acceptance}

根级清零
: doc 根级两件即 AGENTS-RETIRED-2026-09.md 与 CASCADE.json 清零，不再位于 doc 根与 decision/。

归位在档
: doc/retired/AGENTS-RETIRED-2026-09.md 与 doc/knowledge/CASCADE-decidable-cascade-v1.json 在档，内容迁移零改写。

引用零指旧
: 引擎侧引用零指旧路径，指向两件的引用全部对齐新路径。

## 关联 {#relation}

DES-001 规则包与核阅域声明；退役归档件 doc/retired/AGENTS-RETIRED-2026-09.md；级联决策图数据件 doc/knowledge/CASCADE-decidable-cascade-v1.json；证据件 sih/event/plan/rulenorm-materials/json-scope-evidence.json。
