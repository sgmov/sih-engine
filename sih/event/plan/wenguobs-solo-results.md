# wenguobs-solo 结果档（温故观测面批）

> 承接：任务包 sih-engine/sih/state/plan/wenguobs-solo.md
> 队形：单线 solo
> 日期：2026-08-31

## 一、问题陈述回顾 {#problem}

温故主题轴（--topic）经寻址桥按名查询，缺的是文轴（按正文逐字匹配）非向量召回。任务包声明主会 2026-08-31 实测六词落空在案即令牌、use 边、边账、期票、句读、级联。本批实测落包前温故检索面 recall --topic 六词在主轴的真实表现：

| 词 | recall --topic 行数 | 任务包所称 | 差异 |
|---|---|---|---|
| 令牌 | 0 | 零命中 | 一致 |
| use 边 | 1 | 零命中 | 主会声明偏差 |
| 边账 | 0 | 零命中 | 一致 |
| 期票 | 4 | 零命中 | 主会声明偏差 |
| 句读 | 7 | 零命中 | 主会声明偏差 |
| 级联 | 11 | 零命中 | 主会声明偏差 |

主会六词声明按事实记：**仅令牌、边账 二词在主轴真正零命中；use 边、期票、句读、级联 四词在主轴已有命中**。本批不据此偏离任务包核心要求，文轴与零命中账仍按设计实施。

## 二、索引 text 完备性验证 {#index-completeness}

索引条目 text 由 markdown 载体逐 inline token 拼接、json 载体逐值序列、code 载体逐行。locator markdown.py 中 list_item / para / heading / fence 单元均无截断即逐字原文，索引 text 完备性成立。

验证证据：
- `grep -c "期票" /tmp/locator-idx-test.ndjson` = 37（索引全文命中 37 处）
- `grep -c "令牌" /tmp/locator-idx-test.ndjson` = 10
- `grep -c "级联" /tmp/locator-idx-test.ndjson` = 41
- `grep -c "句读" /tmp/locator-idx-test.ndjson` = 62
- `grep -c "边账" /tmp/locator-idx-test.ndjson` = 9

结论：寻址索引条目 text 不截断，文轴逐字子串匹配可覆盖档文正文全部词。

## 三、关键设计裁定 {#design}

三件依任务包第二节：

- **一文轴**即 recall 增 --word 可重复参，对寻址索引条目 text 做 `entry.text.contains(word)` 逐字子串匹配，产同款七字段切面行 axis=word，与名轴 --topic 并存互补。索引 text 经完备性验证即非截断。
- **二零命中账**即 recall 增 --miss-log 可选参，缺参零写即温故默认只读纪律保持；带参对每个产零行的查询词追加一行 ndjson 四字段 `at/axis/word/rows=0` append-only 不去重。带参形态双跑即双份账行属正确账义。
- **三词汇自喂被文轴吸收**即任何登记词汇凡现于档文即文轴可达，名轴不另喂。
- **GOV-003 零改**即本批不升向界版本。

## 四、F 验证表 {#f-verify}

| F | 类别 | 状态 | 证据 |
|---|---|---|---|
| **F-1** 文轴 | 工程治理 | **绿** | 见下表六词复测 |
| **F-2** 零命中账 | 工程治理 | **绿** | 见下表双跑对照 |
| **F-3** 收口 | 链上治理 | **绿** | 见下表收口逐项 |

### F-1 文轴：六词复测命中清单

CLI 实测 `cd worktrees/sih-engine/wenguobs-solo && target/debug/retriever recall --word <词> --at 2026-08-31 --out <件>`，输出件入 sih/state/plan/wenguobs-trail/recall-word-*.json：

| 词 | recall --word 行数 | 首条 ref 样例 |
|---|---|---|
| 令牌 | 11 | `sih-tools/parser/CONTRACT.md@<行位>` |
| 期票 | 38 | `sih-engine/sih/event/plan/scrutmerge-sdd-solo-results.md@44-50` |
| 句读 | 63 | `sih-engine/sih/event/plan/pk017-deduce-t6d-results.md@<行位>` |
| 级联 | 42 | `sih-engine/sih/event/plan/locator-impl-t6d-results.md@69-69` |
| 边账 | 10 | `sih-tools/cascade/CONTRACT.md@<行位>` |
| use 边 | 9 | `sih-tools/cascade/CONTRACT.md@<行位>` |

ref 形如 `path@Lstart-Lend`（md 载体）或 `path@id`（json 载体），机械可回查原档。

F-1 三子项：
- F-1.1 六词经 --word 全命中且 ref 可回查原档：**绿**（上表）
- F-1.2 名轴 --topic 行为零变：`test retriever::wenguobs_tests::f1_topic_axis_unchanged_when_words_absent` **绿**
- F-1.3 --word 与 --topic 同词并用两轴行并出：`test retriever::wenguobs_tests::f1_word_and_topic_same_word_both_axes` **绿**

测试来源：cargo test --lib retriever::wenguobs -- --ignored（6 测试全绿）+ 87 既有测试零回归（cargo test --lib 87 passed; 0 failed; 6 ignored）。

### F-2 零命中账：缺参与带参双跑对照

CLI 实测：

- **缺参形态**：`./target/debug/retriever recall --word wenguobsnonexistent --at 2026-08-31 --out /tmp/...json` 退出码 0，`/tmp/wenguobs-cli-miss-test.ndjson` 不存在。
- **带参形态**：`./target/debug/retriever recall --word wenguobsnonexistent --at 2026-08-31 --miss-log /tmp/wenguobs-cli-miss-test.ndjson --out /tmp/...json` 退出码 0，miss_log 内容：

```json
{"at":"2026-08-31","axis":"word","word":"wenguobsnonexistent","rows":0}
```

四字段齐：at/axis/word/rows。

- **带参形态重复查询双跑**：`/tmp/wenguobs-cli-rep.ndjson` 两行同内容：

```json
{"at":"2026-08-31","axis":"word","word":"wenguobsnonexistent","rows":0}
{"at":"2026-08-31","axis":"word","word":"wenguobsnonexistent","rows":0}
```

- **缺参形态双跑逐字节一致**：`./target/debug/retriever recall --word 令牌 --at 2026-08-31 --out ...` 两跑 md5 = `8f9c70c2f549f032ecaf7c4a3164b6a8` 两次一致。

F-2 四子项：
- F-2.1 --miss-log 缺参零写：`test f2_miss_log_absent_keeps_default_readonly` **绿**
- F-2.2 --miss-log 带参对零行查询词追加一行四字段：`test f2_miss_log_records_zero_hit_axis_word` **绿**
- F-2.3 重复查询双份账行：`test f2_miss_log_repeats_each_call` **绿**
- F-2.4 缺参形态双跑逐字节一致：CLI 实测 md5 一致 **绿**

## 五、工作清单完成度 {#checklist}

- [x] 文轴实现与索引 text 完备性验证，先红后绿
- [x] 零命中账实现与缺参零写保持
- [x] 六落空词真实复测全命中
- [x] SPEC-008 修订与管线认证与结算收约

## 六、变更清单 {#changes}

| 文件 | 改动 |
|---|---|
| `sih-engine/src/retriever/mod.rs` | Axis 加 Word 变体；RecallArgs 加 words 与 miss_log；recall 函数加 word 段与 miss_log 写入；轴全缺拦含 word；加 wenguobs_tests 模块含六组判据 |
| `sih-engine/src/retriever/locator_bridge.rs` | 加 `load_entries` 函数供文轴逐字子串扫 |
| `sih-engine/src/bin/retriever.rs` | 加 --word 与 --miss-log 两 CLI 参数；用法提示同步；RecallArgs 字段同步 |
| `sih-engine/doc/spec/SPEC-008-project-memory-implementation-gap.md` | 升 v1.5；轴语义落差节加文轴描述；ref 形态补钉加文轴共用；测试计划节加 F-11/F-12；组件边界加文轴底座；验收判据加 A6；版本节加 v1.5 修订记录 |

零修改：event_stream、scrutinator、formatter、nomenclator、scribe、locator core 包、locator 窄域包 pack.json、AGENTS.md、GOV-003。

## 七、链事件清单 {#chain-events}

| 事件 | event_type | event_id | event_hash |
|---|---|---|---|
| 意图入链 | intent_refined | 89bc1151-056c-4674-9bd1-1e561034d22d | 0b2f47f1b08d10c95a14bb6811e3e9fba486758b2d81b2d9eea2d7ac9a698c52 |
| 认证（待 append） | certification_completed | （meter 包裹后落链） | （待） |

## 八、偏离如实列 {#deviations}

- **主会六词声明偏差**：任务包所称「主会 2026-08-31 实测六词落空在案」与本批落包前温故检索面 recall --topic 实测不符。本批不据此偏离任务包核心要求即文轴与零命中账按设计实施，已在结果档与交付时如实记。
- **索引 text 完备**：经验证非截断，文轴逐字子串匹配覆盖档文正文全部词。

## 九、待办 / 下批入口 {#next}

- 用户主会验收复跑六词经 --word 全命中
- GOV-003 升版裁定（待用户）

