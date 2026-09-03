# locatorwire-solo：寻址接 ORD-019 载体（m3clear 实例化候选第三件）

- 承接：m3clear-solo 处置清单实例化候选行「locator：ORD-019 承寻址与陈旧检测」；四件套形制承 ordwire 与 elicitwire 先例。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-locatorwire）｜ 温故检索：materials/recall-locatorwire.json 零命中如实记 ｜ pk-045 参与者
- 队形：单线形 solo，零子代理。接线与推导非行为变更。

## 背景（线索，实跑为准）

- locator 实体位 sih-tools/locator/（确定性寻址：多载体结构化解析派生稳定标识）；ORD-019（版本偏序与外化状态存储，order 子仓，mapping 行实取）现消费位 scribe 引擎侧（ordwire 批）——本批为第二消费位，多消费位承 ALG-002 先例零冲突。
- 语义对位（推导档承载）：稳定标识派生即外化状态的一致命名，陈旧检测即版本偏序上的可比判定；以工具实查判定位为准。

## F 清单

- F-1 四件套：载体引用（CONTRACT 增载体引用节或源内注记锚点位，实查后择一申报）、推导档 sih-math/docs/locatorwire-locator-derivation-2026-09-04.md、代码接线（判定位注记锚点三处内）、金向量（寻址派生与陈旧判定两场景双跑逐字节一致）。
- F-2 零行为变更：既有行为零改动，既有测试零回归。
- F-3 写入仅 allow。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package locatorwire-solo → 取锁（施工面 sih-tools/locator/ exclusive 长持；共享追加面 append 短持）→ meter 包裹引擎 scribe intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 append → settle 前一次性拷工地 → 双仓 settle --cert → 放锁 → close → reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；meter 包裹 2>/dev/null 对治；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-tools/locator/（源码、测试、fixtures）
- sih-math/docs/locatorwire-locator-derivation-2026-09-04.md
- sih-engine/sih/event/plan/locatorwire-solo-materials/ 与 locatorwire-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

工具行为零改动；不碰 sih-tools/gauge 与 sih-tools/latex-helper（并行批面）；主树零直写。
