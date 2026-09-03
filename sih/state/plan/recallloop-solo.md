# recallloop-solo：召回质量度量闭环（命中率与信息增益，路线档第一档件三）

- 承接：pk-041 路线档第一档「召回质量度量闭环」（对 recall 记命中率与信息增益，检验三通道是否退化；载体 PROB-014、PROB-011；同时是 pk-037 语义升级评估的度量基座）；mathpipe-full 程序档 M-1。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-recallloop）｜ 温故检索：materials/recall-recallloop.json 零命中如实记 ｜ pk-045 参与者
- 队形：单线形 solo，零子代理。度量面新增，recall 判定行为零改动。

## 背景（线索，实跑为准）

- recall 实体位：sih-tools/wikirecall/recall.py（工具侧）；引擎侧 sih-engine/src/retriever 与 src/bin/retriever.rs（recall 子命令，双档 conclusion/experience 检索）。「三通道」语义以实跑与源码为准。
- pk-037（retriever 词面匹配语义升级评估）在泊：本批度量基座是其出泊评估的输入面，本批不裁 pk-037 出泊与否。
- PROB-014 与 PROB-011 语义以 sih-math/llm-friendly-build/mapping.md 实查为准，禁硬挂。

## F 清单

- F-1 四件套：载体引用（PROB-014/011 mapping 行实取）、推导档 sih-math/docs/recallloop-derivation-2026-09-04.md（命中率与信息增益度量语义、通道退化判据）、代码接线（度量件：recall 结果记录命中率与信息增益读数，通道级分列）、金向量（fixture 语料双跑逐字节一致加退化/未退化两态用例）。
- F-2 零行为变更：recall 检索与排序行为零改动，既有测试零回归；新度量测试先红后绿。
- F-3 度量可解释：输出载通道、命中数、总量、命中率、信息增益读数与判据依据，不裸报布尔。
- F-4 写入仅 allow。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package recallloop-solo → 取锁（施工面 sih-tools/wikirecall/ exclusive 长持；共享追加面 append 短持）→ meter 包裹引擎 scribe intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 append → settle 前一次性拷工地 → 双仓 settle --cert → 放锁 → close → reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；meter 包裹 2>/dev/null 对治；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-tools/wikirecall/（源码、测试、fixtures）
- sih-math/docs/recallloop-derivation-2026-09-04.md
- sih-engine/sih/event/plan/recallloop-solo-materials/ 与 recallloop-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

recall 检索排序行为零改动；不碰 sih-tools/gauge 与 sih-tools/elicit（并行批面）；pk-037 零处置；主树零直写。

## 裁决点预案

本批预期零新裁决点。若 PROB-014/011 语义不覆盖度量场景，属 M-3 模型域缺口：停批走清账路径申报（泊界登记，不扰人），不得硬挂或自造载体。
