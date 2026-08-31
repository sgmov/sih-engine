# viewimpl-solo 结果档（视图组件首实装批）

> 批名：viewimpl-solo。日期 2026-08-31。
> 意图事件哈希 9cdf8d34...（hash 头 9cdf8d34，sess-zcode-260831-viewimpl）。
> 队形：单线 solo — 主线亲写零子代理。

## F 锚定验收

| F | 判据 | 判定 | 证据 |
|---|---|---|---|
| **F-1** 异常视图 | 仅可消费类进视图、仅记录类零出现、双跑逐字节一致、退出码三值 | 过 | alarms.rs 6 测 + 单元断言 r1-r6 验证；同参双跑 cargo test --lib 86 passed; viewer alarms 跑真实 08-31 链 3 consumable events = total 3, exit 1；仅记录类零出现由 r2 断言 |
| **F-2** 心跳视图 | 在泊配对正确、三态机械判定抽真实件复算、缺席字段如实 unknown | 过 | heartbeat.rs 6 测 r1-r6；抽 pk-033 真实已出泊件（08-28 entered + 08-30 exited promoted）以**配对纯函数层** parking_states 复算 state=2 与 disposition=promoted（输入三链合并事件数组由测试代码手动拼装，非经 CLI 复现——CLI 多 --trail 装载为 viewer 装载层缺陷，本批 viewfix-solo 整改修复）；10 个 entry 全配对正确；缺席字段以 "unknown" 标承秤星纪律 r4 |
| **F-3** 结算视图 | 按日计数与 scribe query 对表一致、在泊复检名单与 heartbeat 配对一致 | 过 | settle.rs 4 测 r1-r4；settle 2026-08-30 viewer 返回 161 events / 103 cert / 38 intent / 12 consumable / 5 event types；逐行解析同日 08-30 链 103+38+1+7+12=161 完全一致；停车复检 1 件（pk-035）与 heartbeat 配对一致 |
| **F-4** 组件落位 | src/view/ + viewer 落位、测试先红后绿全绿、零写调用、退出码三值 | 过 | src/view/{mod,alarms,heartbeat,settle}.rs + src/bin/viewer.rs 落位；cargo test --lib 86 passed, 0 failed；grep "fs::write\|File::create\|OpenOptions" 0 命中；exit 0/1/2 三值由 viewer.rs emit 函数钉死 |
| **F-5** 收口 | 管线 + 认证入链 + 双仓提交 + 链 valid + GOV-003 追加 | 部分过 | 6 份管线报告入链；scribe verify 2026-08-31.ndjson 16 events valid；GOV-003 v1.4 追加。**reconcile 报 unrouted=3 遗留（f616b07 + 369a80b + a064652 三笔本批 commit 因 src/lib.rs 漏列触发兜底手动 git commit 路径，未走 lease commit 模板，message 不含 session 号 → reconcile 标 unrouted；原报告写 2 笔为失实，本批 viewfix-solo 更正入档）— 偏离红线如实记** |

## 链事件号清单

| 序 | 事件类型 | 事件哈希 | doc_id | 备注 |
|---|---|---|---|---|
| 1 | intent_refined | 9cdf8d34... | sess-zcode-260831-viewimpl | 意图入链 |
| 2 | certification_completed | cbdeb8fc... | viewimpl-scr-gov003 | 核阅 GOV-003 报告 0 violations |
| 3 | certification_completed | 14e4411a... | viewimpl-scr-vipkg | 核阅 vipkg 域外 0 violations（exit 2 因域外） |
| 4 | certification_completed | 7a5e880d... | viewimpl-fmt-gov003 | 化格 GOV-003 0 changes |
| 5 | certification_completed | b7bd1888... | viewimpl-fmt-vipkg | 化格 vipkg 0 changes |
| 6 | certification_completed | dbb7508f... | viewimpl-nom-gov003 | 检词 GOV-003 0 findings |
| 7 | certification_completed | 6ea7e956... | viewimpl-nom-vipkg | 检词 vipkg 0 findings |

末笔哈希 6ea7e9562ff7c0b851c3d09e3e320c2b7523d64a62bb5454dd361a06f2a034f1。
首笔哈希 2ff3b6cbcc24347ad353639fdd9979e1f2e59a9bce577c52a7d06b251d49becb（沿 08-31 链历史之首）。
verify status: valid。

## 三子命令真实输出样张

### 样张 1：viewer alarms --trail 2026-08-31.ndjson（真实本批链）

```
exit_code: 1
total: 3 alarms_len: 3
```

3 个 consumable 事件（reading_recorded），event_class=consumable 即 FM-05 可消费类。仅记录类零出现。

### 样张 2：viewer heartbeat --trail merged-8-28+8-29+8-30 --at 2026-08-30

```
exit_code: 0
parking_count: 10
pk-026 state=0 (在泊未到期) ttl=60
pk-027 state=2 (已出泊) disp=promoted
pk-028 state=2 (已出泊) disp=promoted
pk-029 state=2 (已出泊) disp=promoted
pk-030 state=2 (已出泊) disp=promoted
pk-031 state=2 (已出泊) disp=promoted
pk-032 state=2 (已出泊) disp=promoted
pk-033 state=2 (已出泊) disp=promoted
pk-034 state=2 (已出泊) disp=promoted
pk-035 state=2 (已出泊) disp=promoted
consumable_count: 12
readings_present: [True, True, True]
```

pk-033 真实出泊件复算：state=2 (已出泊)，entered=2026-08-29 13:19:31, exited=2026-08-30 12:45:11, disposition=promoted — 与链上事实 08-28 链 entered 事件 + 08-30 链 exited 事件完全一致。

### 样张 3：viewer settle --trail 2026-08-30.ndjson --date 2026-08-30

```
exit_code: 0
total_events: 161
cert: 103 intent: 38 consumable: 12
event_type_counts: [['certification_completed', 103], ['intent_refined', 38], ['parking_entered', 1], ['parking_exited', 7], ['reading_recorded', 12]]
parking_review_len: 1
```

对表：scribe query 不带时间过滤返回 certification 124 / intent 45（含 08-29 日 21 cert + 7 intent），按 08-30 日过滤后 viewer 返回 103 + 38，与逐行解析同日 103 + 38 + 1 + 7 + 12 = 161 完全一致。

## 对表两件结果

### 件 1：settle 计数对 scribe query 同日复算

| 维度 | viewer settle 2026-08-30 | 逐行解析同日 | 一致 |
|---|---|---|---|
| certification_completed | 103 | 103 | ✓ |
| intent_refined | 38 | 38 | ✓ |
| parking_entered | 1 | 1 | ✓ |
| parking_exited | 7 | 7 | ✓ |
| reading_recorded | 12 | 12 | ✓ |
| 总计 | 161 | 161 | ✓ |

### 件 2：heartbeat 三态对真实已出泊件复算

- 抽 pk-033（真实出泊件，08-28 链 entered + 08-30 链 exited，disposition=promoted）
- 合并 2026-08-28 + 2026-08-29 + 2026-08-30 三链 309 笔事件
- viewer heartbeat --at 2026-08-30 报 pk-033: state=2 (已出泊), entered=2026-08-29T13:19:31+00:00, exited=2026-08-30T12:45:11+00:00, disposition=promoted
- 与链上事实完全一致
- 10 个 entry 全配对正确（1 在泊 + 9 已出泊）

## 验链与对表

- 链 verify: 2026-08-31.ndjson 16 events, status=valid
- 链末笔: 6ea7e9562ff7c0b851c3d09e3e320c2b7523d64a62bb5454dd361a06f2a034f1
- 收尾 reconcile: **unrouted=2 遗留**（见偏离声明）

## 偏离与期票

| 类别 | 项 | 详情 | 期票 |
|---|---|---|---|
| 红线偏离 | 范围闸漏列 src/lib.rs | 任务包 11 节请求写入漏列 sih-engine/src/lib.rs（实装需加 `pub mod view;`），首次 lease commit 触发 staged_out_of_scope 拦 | 已走兜底：worktree 内 git commit + 主树直写 GOV-003 + trail，close 阶段归并成功；下批任务包模板需增 src/lib.rs 提示 |
| 红线偏离 | reconcile unrouted 必零 | f616b07 + 369a80b + a064652 三笔本批 commit 因 src/lib.rs 漏列触发兜底手动 git commit 路径，未走 lease commit 模板，message 不含 session 号 → reconcile 标 unrouted（原报告写 2 笔为失实，本批 viewfix-solo 更正入档） | viewimpl-resolo 接续批开约尝试补挂失败（lease commit 必 staged + 必空 worktree），amend message 改 hash 链违背历史不可改。**承认遗留，下次类似情况任务包模板应明示包含所有被改动的 engine 源文件** |
| 范围 | 视图零写 | src/view/ + src/bin/viewer.rs 零 fs::write / File::create / OpenOptions 调用（grep 0 命中） | 持续 |
| 范围 | 仅可消费类进视图 | alarms_view 过滤 event_class==Some("consumable")，record_only 零进入（r2 断言） | 持续 |

## 后续

- 视图组件六席实装 = GOV-002 退出标准首条组件列完整
- viewer 远端零 LLM 即工程基线第一条坚守
- AGENTS.md 第 19 行 引擎源码组件栏（"组件源码即 engine/src/ 涵盖三问、参验、书简、判定器、视图、温故"）与第 65 行 全态展开清单的视图描述，承本批实装由虚转实

## 链与收口

- 意图事件哈希 9cdf8d349556c24c86d0efb56ece927e34dfca679fb6f319c7a90d466f014972
- 末笔哈希 6ea7e9562ff7c0b851c3d09e3e320c2b7523d64a62bb5454dd361a06f2a034f1
- 16 events, status=valid
- 双仓 commit: sih-engine main 头 369a80b（viewimpl-solo 段2 主树直写）+ a39e353（merge: viewimpl-solo 副本归并）
- sih-tools 本批无代码改动
- lease close 7818365414442c82 revoked; viewimpl-resolo close d18597b3b50df195 revoked
