# gchart-solo：秤星升控制图（SPC 控制限与越限告警，路线档第一档件一）

- 承接：pk-041 路线档第一档「秤星升控制图」（统计过程控制加变点检测，基线期遥测定控制限，越限即告警；载体 PROB-013、PROB-010；哲学锚 PRO-07）；mathpipe-full 程序档第一节 M-1。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-gchart）｜ 温故检索：materials/recall-gchart.json 零命中如实记 ｜ pk-045 参与者
- 队形：单线形 solo，零子代理。读路径新增，record 写路径零改动。

## 背景（线索，实跑为准）

- gauge 现势：formula ga-2（LIM-007、ORD-002、PROB-001、PROB-003、PROB-005 在位，期票已清偿）；载体位先例即 cli.py 源内注记锚点与 sih-math/docs/mathpipe-a3-derivation-2026-09-03.md 推导档。
- PROB-013（平稳性与变点检测）现消费位 identity（idwire）；PROB-010（假设检验与显著性）现消费位 facet（facetmath 推导档）——本批为两载体增消费位，多消费位承 ALG-002 先例零冲突。
- 告警语义对齐工程基线第三条：控制限越限即异常信号，人只看视图告警不看原始日志。

## F 清单

- F-1 四件套：载体引用（gauge 既有载体位形制：cli.py 注记锚点 + 数学仓 mapping 行引用 PROB-013/010，mapping 行号实取）、推导档 sih-math/docs/gchart-derivation-2026-09-04.md（控制限统计语义：基线期均值与控制限、变点检测判据、越限告警谓词）、代码接线（新只读子命令或读路径模块，record 写路径零改动）、金向量（fixture 读数序列双跑逐字节一致加越限/未越限两态用例）。
- F-2 零行为变更：既有 gauge record 与三维读数路径零改动，既有测试零回归；新读路径测试先红后绿。
- F-3 告警可解释：越限输出载超限侧与限值与统计依据行，不裸报布尔。
- F-4 写入仅 allow。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package gchart-solo → 取锁（施工面 sih-tools/gauge/ exclusive 长持；共享追加面 append 短持）→ meter 包裹引擎 scribe intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 append → settle 前一次性拷工地 → 双仓 settle --cert → 放锁 → close → reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；meter 包裹 2>/dev/null 对治；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-tools/gauge/（源码、测试、fixtures）
- sih-math/docs/gchart-derivation-2026-09-04.md
- sih-engine/sih/event/plan/gchart-solo-materials/ 与 gchart-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

record 写路径与读数事件 schema 零改动（ga-2 在役面）；不碰 sih-tools/wikirecall 与 sih-tools/elicit（并行批面）；主树零直写。

## 裁决点预案

本批预期零新裁决点（两载体已在仓已裁定）。若实跑发现 PROB-013/010 语义不覆盖控制限场景，属 M-3 模型域缺口：停批走清账路径申报（泊界登记，不扰人），不得硬挂或自造载体。
