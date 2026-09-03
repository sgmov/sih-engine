# elicitwire-solo：叩问接 ORD-008 载体（m3clear 实例化候选第一件）

- 承接：m3clear-solo 处置清单实例化候选行「elicit：ORD-008 承消化闸（待接线批）」；载体接线四件套形制全承 ordwire 与 tallywire2 与 selwire 先例。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-elicitwire）｜ 温故检索：materials/recall-elicitwire.json 两命中如实记（elicitgate-solo-results 消化闸闸位、elicitimpl-solo-results F-3 消化闸）｜ pk-045 参与者
- 队形：单线形 solo，零子代理。接线与推导非行为变更。

## 背景（线索，实跑为准）

- elicit 现势：叩问站（词债登记与核查）已上闸位——check 信号面与 digest 消化闸（信号未消化不得上链，零信号直过），SPEC-012 v1.1 破冻留痕先例在案。
- ORD-008（引用图可达与孤悬判定，order 子仓，mapping 行实取）现消费位：scrutinator（gatecap C007/C008）、tally（tallywire2 对挂行）——本批为第三消费位，多消费位承 ALG-002 先例零冲突。
- 语义对位（推导档承载）：消化闸即引用可达性——登记信号逐项被处置（可达），未处置信号即孤悬，孤悬即不得上链；零信号直过即空图平凡可达。

## F 清单

- F-1 四件套：载体引用（elicit CONTRACT 增载体引用节，若无 CONTRACT 则源内注记锚点位形制照 gauge 先例，实查后择一并在结果档申报）、推导档 sih-math/docs/elicitwire-elicit-derivation-2026-09-04.md、代码接线（check/digest 判定位注记锚点三处内）、金向量（digest 过闸与未消化拦截两场景双跑逐字节一致）。
- F-2 零行为变更：elicit 既有 check/digest/词债登记行为零改动，既有测试零回归。
- F-3 写入仅 allow。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package elicitwire-solo → 取锁（施工面 sih-tools/elicit/ exclusive 长持；共享追加面 append 短持）→ meter 包裹引擎 scribe intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 append → settle 前一次性拷工地 → 双仓 settle --cert → 放锁 → close → reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；meter 包裹 2>/dev/null 对治；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-tools/elicit/（源码、测试、fixtures）
- sih-math/docs/elicitwire-elicit-derivation-2026-09-04.md
- sih-engine/sih/event/plan/elicitwire-solo-materials/ 与 elicitwire-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

elicit 判定行为零改动；不碰 sih-tools/gauge 与 sih-tools/wikirecall（并行批面）；主树零直写。
