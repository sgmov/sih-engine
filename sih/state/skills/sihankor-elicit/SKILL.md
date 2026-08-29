# Skill: sihankor-elicit（叩问）

叩问调用入口：机械缺口信号检测与消化闸。权威源在 sih-tools/elicit/CONTRACT.md，本壳只载触发语义与调用形。

## 触发时机

- 三问检索加载后出缺口信号件，或机械验收步查契约消化
- 用户明言叩问、查缺口、信号检测、查新词
- 契约草稿新增措辞需查登记面在否

不触发：日常概念查档走温故 recall，不做语义判断，判在人与 agent。

## 调用形

cd sih-tools/elicit 后 uv run --project . elicit <操作>：

- check：--packs ../nomenclator/packs/core --words <新词逐个> --topics <主题逐个> --recall-face <召回面 ndjson> --history-face <历史清单件> --out <信号件> --at <实日>，退出码零无信号一有信号二异常报缺席件名
- digest：--signals <信号件> --contract <契约件>，退出码一即缺叩问处置标记并报缺项
- suspend：--signals <信号件> --log <日志件> --at <实日>，落挂起记录无代答路径

信号件五字段即 signal_type 与 subject 与 weight 与 source_ref 与 at，随批材料入档。

## 边界

零 LLM 零第三方依赖，不做语义召回不做分词，矛盾判定归 agent 与人。权威契约 v1.1 承 SPEC-012。
