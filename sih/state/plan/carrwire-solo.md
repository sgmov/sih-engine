# carrwire-solo：四新载体接线批（ORD-023→formatter、PROB-016→meter、ORD-024→nomenclator、ALG-012→parser）

- 承接：newcarr-solo 终签四载体（ORD-023 重写系统确定性幂等、PROB-016 计数测度可加性、ORD-024 区间序跨度匹配、ALG-012 形式文法解析确定性），其 F-4 零接线遗留「接线归后续逐工具批」即本批；四件套形制承 latexwire（四载体一工具）与 locatorwire 先例。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-carrwire）｜ 温故检索：materials/recall-carrwire.json 结果如实记 ｜ pk-045 参与者
- 队形：单线形 solo，零子代理。接线与推导为非行为变更批。

## 背景（线索，实跑为准）

- 四载体 newcarr-solo 终签在位，条目路径：order/entries/ORD-023-abstract-rewrite-system-deterministic-normal-form.md、probability/entries/PROB-016-counting-measure-and-additivity.md、order/entries/ORD-024-interval-order-and-span-matching.md、algebra/entries/ALG-012-formal-grammar-and-parse-determinism.md；mapping.md 四行在 209 至 212 区段。
- 语义对位（推导档承载，判定位以工具实查为准）：formatter 归一化收敛即重写系统不动点与唯一范式（幂等场景金向量正对 ORD-023）；meter 计数对账即测度可加性恒等式；nomenclator 命中跨度即区间序与互斥链采纳唯一；parser 有序选择即候选式优先全序派生唯一。
- 实体位：sih-tools/formatter/（SPEC.md 与 INCUBATION.md 在位无 CONTRACT）、sih-tools/meter/、sih-tools/nomenclator/、sih-tools/parser/。

## F 清单

- F-1 四件套 ×4（逐工具）：载体引用（CONTRACT 增载体引用节，formatter 无 CONTRACT 则 SPEC 或源内注记锚点位，实查择一申报）、推导档（合一件 sih-math/docs/carrwire-carriers-derivation-2026-09-04.md 分四节，逐工具载体引用与判定位与金向量）、代码接线（判定位注记锚点，AST 同形零行为）、金向量（各工具典型场景双跑逐字节一致；formatter 加「同一输入二遍格式化输出恒同」幂等场景）。
- F-2 零行为变更：四工具既有测试批前批后同绿零回归。
- F-3 检词零违例；锚点新词按登记先例入词表，词债不过夜。
- F-4 写入仅 allow。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package carrwire-solo → 取锁（施工面四工具目录 exclusive 长持；共享追加面即 trail 与 scribe/reports 与 CALL-LOG 与 meter/counts 与 ledger 一律 --mode append 短持即取即放）→ meter 包裹引擎 scribe intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 append 主树活链 → settle 前一次性拷工地 → 三仓 settle --cert → 放锁 → close（备份让位归并对表法）→ reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；meter 包裹 2>/dev/null 对治；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-tools/formatter/ 与 sih-tools/meter/ 与 sih-tools/nomenclator/ 与 sih-tools/parser/（源码注记、CONTRACT/SPEC、测试、nomenclator 词表 registers、CALL-LOG 各自面）
- sih-math/docs/carrwire-carriers-derivation-2026-09-04.md
- sih-engine/sih/event/plan/carrwire-solo-materials/ 与 carrwire-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

四工具行为零改动（AST 同形）；数学仓 entries 与 INDEX 与 mapping.md 零触碰（载体已终签，禁改已签条目）；sih-tools/retriever 与 identity 与 tally 与 gauge 与 latex-helper 与 locator 零碰（pk037impl 筹备面与已收批面）；主树零直写；金向量冻结禁工地绝对路径。
