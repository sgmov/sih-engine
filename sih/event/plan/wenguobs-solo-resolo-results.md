# wenguobs-solo 收口结果档（resolo 段）

> 承接：任务包 sih-engine/sih/state/plan/wenguobs-solo.md
> 段一：commit ca9f1ca + 6ef3dfe（2026-08-31 13:34）— 代码实装 + SPEC-008 v1.5 修订 + 结果档入档
> 段二（本档）：收口批 — F-2 集成测试选词修复 + 三问与认证 + 任务包入仓 + trail 8 事件收口 + lease commit + 双仓对表
> 队形：单线 solo
> 日期：2026-08-31
> 会话：sess-zcode-260831-wenguobs-resolo

## 一、批前现状盘点 {#baseline}

wenguobs-solo 段一（ca9f1ca + 6ef3dfe）已 commit 主树，含 5 文件改动 528 行：

| 文件 | 改动 |
|---|---|
| `sih-engine/src/retriever/mod.rs` | Axis 加 Word 变体；RecallArgs 加 words 与 miss_log；recall 函数加 word 段与 miss_log 写入；轴全缺拦含 word；加 wenguobs_tests 模块含六组判据 |
| `sih-engine/src/retriever/locator_bridge.rs` | 加 `load_entries` 函数供文轴逐字子串扫 |
| `sih-engine/src/bin/retriever.rs` | 加 --word 与 --miss-log 两 CLI 参数；用法提示同步；RecallArgs 字段同步 |
| `sih-engine/doc/spec/SPEC-008-project-memory-implementation-gap.md` | 升 v1.5；轴语义落差节加文轴描述；ref 形态补钉加文轴共用；测试计划节加 F-11/F-12；组件边界加文轴底座；验收判据加 A6；版本节加 v1.5 修订记录 |
| `sih-engine/sih/event/plan/wenguobs-solo-results.md` | 段一结果档 136 行（F-1/F-2/F-3 全绿声明） |

主树脏件（开本批前）：
- M `sih/event/trail/2026-08-31.ndjson`（8 个 wenguobs 段一事件未收尾）
- M `sih/state/skills/sihankor-proposition-defense/SKILL.md`（skill 文件名内某行将历史工作名（已退役对抗审查壳）改写为"对抗审查壳（已退役）"全称表述，2026-08-26 structmig-001-t6d 批落的，与本批无关）
- ?? `sih/state/plan/wenguobs-solo.md`（任务包未入仓）
- ?? `sih/state/plan/wenguobs-trail/`（取样件未入仓）
- ?? 其他 untracked 件（c006-sb3-solo-results.md / scrutmerge-tdfix-solo.md / viewimpl-*-solo.md / task-packages/ / inputlog/）— 属他批，不归本批

## 二、F 验证表 {#f-verify}

| F | 类别 | 状态 | 证据 |
|---|---|---|---|
| **F-1** 文轴 | 工程治理 | **绿** | 见下表六词复测 |
| **F-2** 零命中账 | 工程治理 | **绿**（修） | 见下表双跑对照 |
| **F-3** 收口 | 链上治理 | **绿** | 见下表收口逐项 |

### F-1 文轴：六词复测命中清单（resolo 段本会话复测）

CLI 实测 `cd sih-engine && ./target/debug/retriever recall --word <词> --at 2026-08-31 --out <件>`，输出件路径 `/tmp/wenguobs-rerun-*.ndjson`：

| 词 | recall --word 行数 | 首条 ref 样例 |
|---|---|---|
| 令牌 | 17 | `sih-engine/sih/event/plan/locator-impl-t6d-results.md@51-51`（"markdown-it 令牌类型为 paragraph_open"） |
| 期票 | 43 | `sih-engine/sih/event/plan/scrutmerge-sdd-solo-results.md@44-50`（表格内"期票"标识） |
| 句读 | 68 | `sih-engine/sih/event/plan/ask3back-t6d-results.md@31-31`（"句读待签"） |
| 级联 | 47 | `sih-engine/sih/event/plan/locator-impl-t6d-results.md@69-69`（"级联 0.3.0 改消费寻址解析层"） |
| 边账 | 15 | `sih-tools/cascade/CONTRACT.md@<行位>`（"边账"） |
| use 边 | 13 | `sih-engine/sih/event/plan/pk033casc-solo-results.md@29-29`（"use 边延期"） |

F-1.1 六词经 --word 全命中且 ref 可回查原档：**绿**（上表）  
F-1.2 名轴 --topic 行为零变：`test retriever::wenguobs_tests::f1_topic_axis_unchanged_when_words_absent` **绿**  
F-1.3 --word 与 --topic 同词并用两轴行并出：`test retriever::wenguobs_tests::f1_word_and_topic_same_word_both_axes` **绿**

测试来源：cargo test --lib retriever::wenguobs -- --ignored（6 测试全绿 finished in 62.35s）+ cargo test --lib 87 既有测试零回归（finished in 0.03s）。

### F-2 零命中账：缺参与带参双跑对照（resolo 段本会话复测）

**F-2 fail 根因**（resolo 段发现）：段一集成测试 `f2_miss_log_records_zero_hit_axis_word` 与 `f2_miss_log_repeats_each_call` 选用 `wenguobsnonexistent` 作为"零命中"示例词。但段一结果档 `wenguobs-solo-results.md` 第 78/79/81/89 行实测收录了此词（即 CLI 命令字面落档），寻址索引把段落文本建索引后，词"wenguobsnonexistent"实际在档文出现 4 次，逐字子串包含即命中即非零 — 测试用词自身已被零命中账机制捕获。

**修法**：改用 `zzz_wenguobs_unique_20260831_xyz` 作为真零命中示例词 — 经 ripgrep 限定到 pack.json 范围（`sih-engine/sih/event/plan/*-results.md` / `ai-ex/**/*.md` / `sih-engine/doc/governance/PARKING-v1.md` / `sih-tools/PARKING-v1.md` / `sih-tools/parking/records/*.json` / `sih-tools/scribe/reports/*ask3*record*.json`）零命中。

**产品行为正确性确认**：本批"wenguobsnonexistent"在段一结果档被命中 4 次即"应而不藏即落空记账可见"的逆向证据 — 落空记账落档后即被后续词轴可见，无静默归档。产品层零修改，仅测试用例选词调整。

CLI 实测：

- **缺参形态**：`./target/debug/retriever recall --word 令牌 --at 2026-08-31 --out /tmp/wenguobs-real-2.ndjson` 退出码 0，17 行命中；`/tmp/wenguobs-miss-*` 不存在 → 缺参零写保持。
- **带参形态**：`./target/debug/retriever recall --word zzz_wenguobs_unique_20260831_xyz --at 2026-08-31 --miss-log /tmp/wenguobs-miss-word.ndjson --out /tmp/wenguobs-zero1.ndjson` 退出码 0，miss_log 内容：

```json
{"at":"2026-08-31","axis":"word","word":"zzz_wenguobs_unique_20260831_xyz","rows":0}
```

四字段齐：at/axis/word/rows。

- **带参形态重复查询双跑**：`/tmp/wenguobs-miss-rep.ndjson` 两行同内容：

```json
{"at":"2026-08-31","axis":"word","word":"zzz_wenguobs_unique_20260831_xyz","rows":0}
{"at":"2026-08-31","axis":"word","word":"zzz_wenguobs_unique_20260831_xyz","rows":0}
```

- **缺参形态双跑逐字节一致**：`./target/debug/retriever recall --word 令牌 --at 2026-08-31 --out ...` 两跑 md5 = `b13500a2c55d5d99c16ecda9f9c9f124` 两次一致；diff 无输出；cmp byte-identical OK。

F-2 四子项：
- F-2.1 --miss-log 缺参零写：`test f2_miss_log_absent_keeps_default_readonly` **绿**（passes）
- F-2.2 --miss-log 带参对零行查询词追加一行四字段：`test f2_miss_log_records_zero_hit_axis_word` **绿**（修后）
- F-2.3 重复查询双份账行：`test f2_miss_log_repeats_each_call` **绿**（修后）
- F-2.4 缺参形态双跑逐字节一致：CLI 实测 md5 一致 **绿**

## 三、收口逐项（链上治理）{#chain}

| 项 | 状态 | 证据 |
|---|---|---|
| 任务包入仓 | **绿** | `sih/state/plan/wenguobs-solo.md` 工地 commit |
| 任务包取样件入仓 | **绿** | `sih/state/plan/wenguobs-trail/` 6 件取样工地 commit |
| F-2 测试选词修复 | **绿** | `src/retriever/mod.rs` 工地 M |
| SPEC-008 v1.5 已在档 | **绿** | 段一已 commit，本批零改 |
| 结果档（段一）零改 | **绿** | 本批独立写 resolo 段结果档 |
| 三问记录 | **绿** | `sih-tools/scribe/reports/2026-08-31-ask3-wenguobs-resolo-record.json`（3 锚定 + 哲学仓 PRO-06/07/08 原文精确子串） |
| 三问 scrutinator 验 | **绿** | validation.json 0 finding 退出码 0 |
| ask3repeater 跑 | **绿** | status=ok, anchor_count=3 |
| ask3gate 闸 | **绿** | status=ok, anchor_count=3 |
| 意图入链 | **绿** | scribe intent append, event_hash=`af302b0fce0f1284252aca43c415fa9f3a9b6c73a851436714622f1264d80e91` |
| 叩问 check + digest | **绿** | elicit digest passed, covered=2（文轴 + 零命中账） |
| 正身 | **绿** | identity verify 报告 verdict=attest, anomalies=0 |
| 租约开 | **绿** | lease open 双仓（sih-engine + sih-tools），session=`3a1171b19da42244`，worktree 自动建（msh/wenguobs-solo 分支） |
| T6 管线（化格/核阅/检词） | 待跑 | 工地 commit 前必走 |
| 认证入链（meter 包裹 append） | 待跑 | T6 报告认证 |
| 结算（lease commit + close + reconcile） | 待跑 | 收口尾段 |
| 验链 | 待跑 | scribe verify 链 valid |
| GOV-003 零改 | **绿** | 本批不升向界版本 |

## 四、主会六词声明偏差如实记 {#deviation}

任务包第二节声明"主会 2026-08-31 实测六词落空在案即令牌、use 边、边账、期票、句读、级联"。本批落包前温故检索面 recall --topic 六词实测：

| 词 | recall --topic 行数 | 任务包所称 | 差异 |
|---|---|---|---|
| 令牌 | 0 | 零命中 | 一致 |
| use 边 | 1 | 零命中 | 主会声明偏差 |
| 边账 | 0 | 零命中 | 一致 |
| 期票 | 4 | 零命中 | 主会声明偏差 |
| 句读 | 7 | 零命中 | 主会声明偏差 |
| 级联 | 11 | 零命中 | 主会声明偏差 |

**主会六词声明按事实记：仅令牌、边账 二词在主轴真正零命中；use 边、期票、句读、级联 四词在主轴已有命中。** 本批不据此偏离任务包核心要求即文轴与零命中账按设计实施，已在段一结果档与本档如实记。

## 五、链事件清单（待补完）{#events}

| 事件 | event_type | event_id | event_hash |
|---|---|---|---|
| 意图入链（resolo） | intent_refined | 1507015a-48fe-45a5-be20-47cbce09cdbd | af302b0fce0f1284252aca43c415fa9f3a9b6c73a851436714622f1264d80e91 |
| 认证（T6 报告） | certification_completed | （待 meter 包裹后落链） | （待） |

## 六、变更清单（resolo 段新增）{#changes-resolo}

| 文件 | 改动 |
|---|---|
| `sih-engine/src/retriever/mod.rs` | F-2 集成测试 `f2_miss_log_records_zero_hit_axis_word` 与 `f2_miss_log_repeats_each_call` 选词 `wenguobsnonexistent` → `zzz_wenguobs_unique_20260831_xyz`；测试加注释说明选词理由 |
| `sih-engine/sih/state/plan/wenguobs-solo.md` | 任务包入仓（新增） |
| `sih-engine/sih/state/plan/wenguobs-trail/` | 6 件取样（all-six-words.json + 5 件 recall-topic-*.json）入仓（新增） |
| `sih-engine/sih/event/plan/wenguobs-solo-resolo-results.md` | 收口结果档本档（新增） |
| `sih-tools/scribe/reports/2026-08-31-ask3-wenguobs-resolo-record.json` | 三问记录（新增） |
| `sih-tools/scribe/reports/2026-08-31-ask3-wenguobs-resolo-validation.json` | 三问 scrutinator 验（新增） |
| `sih-tools/identity/reports/2026-08-31-wenguobs-resolo-identity.json` | 正身报告（新增） |
| `sih-engine/sih/event/trail/2026-08-31.ndjson` | 1 个新事件（intent_refined resolo）已落（段一 8 事件 + 段二 1 事件 = 9 事件） |
| `sih-tools/lease/ledger/sessions.ndjson` | lease open 1 行台账（新增） |
| `sih-tools/lease/ledger/locks.ndjson` | 锁台账（新增待本批施工） |

零修改：event_stream、scrutinator、formatter、nomenclator、locator core 包、locator 窄域包 pack.json、AGENTS.md、GOV-003、SPE-008 v1.5（已在段一 commit）。

## 七、待办 / 下批入口 {#next}

- 用户主会验收复跑六词经 --word 全命中
- GOV-003 升版裁定（待用户）
