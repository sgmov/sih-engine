# queueing-solo：注意力预算排队化（WIP 队列读数与洪流前兆，路线档第一档件二）

- 承接：pk-041 路线档第一档「注意力预算排队化」（泊界在泊数、批在途数、会话并发数按队列 WIP 读数，到达率超服务率即洪流前兆；载体 PROB-015；哲学对照 convergence P3.1）；工程基线第二三条即信息洪流根因的度量面。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-queueing）｜ 温故检索：materials/recall-queueing.json 零命中如实记 ｜ pk-045 参与者
- 队形：单线形 solo，零子代理。只读度量新增，治理写路径零改动。

## 背景（线索，实跑为准）

- gauge 只读伴件先例：gchart（gc-1，PROB-013/010，sih-tools/gauge，基线期定限越限告警）——本批同形制。
- 三读数源（实查后定性）：泊界在泊数（链 park 事件或泊材料目录）、批在途数（lease 会话台账活动会话）、会话并发数与锁持有（lease 锁台账）；到达率与服务率窗口估计取台账历史行，窗口参数显式声明为模型参数。
- PROB-015 语义以 sih-math/llm-friendly-build/mapping.md 实查为准，禁硬挂。

## F 清单

- F-1 四件套：载体引用（PROB-015 mapping 行实取）、推导档 sih-math/docs/queueing-derivation-2026-09-04.md（WIP 队列语义、到达/服务率估计、洪流前兆判据、窗口参数模型参数声明）、代码接线（gauge 只读子命令或模块，承 gchart 先例）、金向量（fixture 台账三 WIP 读数双跑逐字节一致加前兆/平稳两态用例）。
- F-2 零行为变更：gauge record 与既有读路径零改动，既有测试零回归；新测试先红后绿。
- F-3 前兆可解释：前兆输出载三 WIP 读数、到达/服务率、判据依据行，不裸报布尔。
- F-4 写入仅 allow。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package queueing-solo → 取锁（施工面 sih-tools/gauge/ exclusive 长持；共享追加面 append 短持）→ meter 包裹引擎 scribe intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 append → settle 前一次性拷工地 → 双仓 settle --cert → 放锁 → close → reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；meter 包裹 2>/dev/null 对治；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-tools/gauge/（源码、测试、fixtures）
- sih-math/docs/queueing-derivation-2026-09-04.md
- sih-engine/sih/event/plan/queueing-solo-materials/ 与 queueing-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

gauge record 与 gchart 零改动；台账文件零写（只读度量）；不碰 sih-tools/latex-helper 与 sih-tools/locator（并行批面）；主树零直写。

## 裁决点预案

本批预期零新裁决点。若 PROB-015 语义不覆盖判据场景属 M-3 模型域缺口：停批走清账路径泊界登记（不扰人不呈裁），禁硬挂禁自造载体。
