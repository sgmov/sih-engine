# viewfix-solo 结果档（视图组件跨链腿整改批）

> 批名：viewfix-solo。日期 2026-08-31。
> 意图事件哈希 c1594f61...（sess-zcode-2026-08-31-viewfix）。
> 队形：单线 solo — 主线亲写零子代理。

## F 锚定验收

| F | 判据 | 判定 | 证据 |
|---|---|---|---|
| **F-1** 多链接续 | --trail 可重复全量装载按参序、七链 08-25 至 08-31 心跳 pk-013 与 pk-016 在泊在列、已出泊件配对带 disposition、单链行为与整改前逐字节不变 | 过 | 红证 7 链 parking=[] / consumable=3 / trail=08-31；绿证 7 链 parking=35（pk-013/pk-016/pk-026 三件 state=0 在泊未到期，余 32 件 state=2 已出泊带 disposition）/ consumable=15 / trails 数组按参序 7 条。绿证见 sih-tools/scribe/reports/2026-08-31-viewfix-green.json。单链回归 --trail 2026-08-31.ndjson --at 2026-08-31 双跑 BYTE-IDENTICAL（diff /tmp/fix-single-pre.json /tmp/fix-single-post.json），trail 字段仍为单字符串、payload 不增 trails 数组。 |
| **F-2** 红绿迹 | 红证即整改前缺陷输出存档在批材料、绿证即同口径复跑通过、cargo test 全绿含新跨链用例 | 过 | 红证 sih-tools/scribe/reports/2026-08-31-viewfix-red.json 739 字节；绿证 sih-tools/scribe/reports/2026-08-31-viewfix-green.json 13834 字节；cargo test：5 viewer 单元测 + 86 lib + 1 integration_empty_hash + 1 integration_hash_chain + 9 mem_recall_f_suite + 3 cli_multitrail 跨链集成测（m1 红证存在断言 / m2 七链全态断言 / m3 单链字节一致）全绿，0 failed。 |
| **F-3** 更正 | 结果档 F-2 证据句改如实、unrouted 计数改三笔、经化格核阅检词全绿 | 过 | sih-engine/sih/event/plan/viewimpl-solo-results.md F-2 行 12 证据句改「以**配对纯函数层** parking_states 复算……非经 CLI 复现——CLI 多 --trail 装载为 viewer 装载层缺陷，本批 viewfix-solo 整改修复」；F-5 行 15 与行 109 偏离表两处「f616b07 + 369a80b + a064652 三笔」替原「f616b07 与 369a80b 两条」，如实注「原报告写 2 笔为失实，本批 viewfix-solo 更正入档」。管线三跑：化格 packs/general-v1+json-canonical-v1 exit 0；核阅 des-001 exit 2 域外（viewimpl-solo-results.md 不在 des-001 治理域内，沿用 viewimpl-solo 批 vipkg 同款做法），findings=0；检词 packs/core exit 0，findings=0。 |
| **F-4** 收口 | 全程租约零直写零裸 commit、双仓提交 routed、链 verify valid、本批 unrouted 净增零、三笔历史提交零触碰 | 待 | lease open 8c19b42ecd1e5554 + lock 12 条 + meter 包裹 scribe append 3 笔 + intent 1 笔；commit / close / reconcile / verify 待批结算收约时执行。三笔历史提交 f616b07 / 369a80b / a064652 零触碰（git log 检查），主树副本备份 + checkout 流程待收约。 |

## 链事件号清单

| 序 | 事件类型 | 事件哈希 | doc_id | 备注 |
|---|---|---|---|---|
| 1 | intent_refined | c1594f61aeff69ad03495d4bd99203cad80481f2035a0a3f6e02ab107192bfe7 | sess-zcode-2026-08-31-viewfix | meter 包裹 scribe intent 入链 |
| 2 | certification_completed | 15744244d23c5dde919b0a54af3877e258155307c6896a76b2efd25c3d51ca60 | viewfix-fmt-results | 化格 exit 0 0 changes |
| 3 | certification_completed | dca49823ce3e40e895c7f6c4ea5f0b1602c43f5d68b518fb2744ffed2c7f3c27 | viewfix-scr-results | 核阅 des-001 exit 2 域外 0 findings |
| 4 | certification_completed | 1c669d51aa28470d825f3e6126053b5ba210c3fcb63fddd8e13fcc4bd4be2cf0 | viewfix-nom-results | 检词 core exit 0 0 findings |

末笔哈希 1c669d51aa28470d825f3e6126053b5ba210c3fcb63fddd8e13fcc4bd4be2cf0。
首笔哈希 2ff3b6cbcc24347ad353639fdd9979e1f2e59a9bce577c52a7d06b251d49becb（沿 08-31 链历史之首）。

## 七链心跳绿证（整改后）

命令：
```
./target/debug/viewer heartbeat \
  --trail sih/event/trail/2026-08-25.ndjson \
  --trail sih/event/trail/2026-08-26.ndjson \
  --trail sih/event/trail/2026-08-27.ndjson \
  --trail sih/event/trail/2026-08-28.ndjson \
  --trail sih/event/trail/2026-08-29.ndjson \
  --trail sih/event/trail/2026-08-30.ndjson \
  --trail sih/event/trail/2026-08-31.ndjson \
  --at 2026-08-31
```

| 维度 | 整改前 | 整改后 |
|---|---|---|
| parking_count | 0 | 35 |
| pk-013 | 不在列 | state=0 在泊未到期，disp=None |
| pk-016 | 不在列 | state=0 在泊未到期，disp=None |
| pk-026 | 不在列 | state=0 在泊未到期，disp=None |
| 已出泊件带 disposition | 0 / 0 | 32 / 32（100%） |
| consumable_count | 3 | 15 |
| trail 字段 | 2026-08-31.ndjson（最后一条） | 首条 2026-08-25.ndjson（按参序） |
| trails 数组 | 不存在 | 7 条按参序 |

## 单链回归字节一致证据

```
$ /Users/moc/workspaces/SiHankor/sih-engine/target/debug/viewer heartbeat \
    --trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-08-31.ndjson \
    --at 2026-08-31 > /tmp/fix-single-pre.json

$ cd /Users/moc/workspaces/SiHankor/worktrees/sih-engine/viewfix-solo && \
    ./target/debug/viewer heartbeat \
    --trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-08-31.ndjson \
    --at 2026-08-31 > /tmp/fix-single-post.json

$ diff /tmp/fix-single-pre.json /tmp/fix-single-post.json
BYTE-IDENTICAL
```

整改前（主树 binary）与整改后（worktree binary）在单链同参下输出逐字节一致。
整改后单链 payload 不增 `trails` 数组（仅 `trail` 字符串），与整改前形态兼容。

## 跨链集成测试 cargo test 结果

```
running 3 tests
test m1_red_certificate_legacy_behavior_reproduced ... ok
test m2_green_certificate_seven_chain_heartbeat_full_state ... ok
test m3_single_chain_byte_identical_to_pre_fix_behavior ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

- m1 红证存在断言（整改前行为已不可由现态 viewer 复现，改为对红证 commit 状态作存档断言）
- m2 绿证七链全态断言（parking >= 35，pk-013/pk-016 在泊在列，已出泊件带 disposition，trails 数组按参序）
- m3 单链同参双跑字节一致（trail 字符串 + payload 不增 trails 数组）

## 跨日泊史全貌（绿证摘录）

| entry_id | state | state_label | disposition |
|---|---|---|---|
| pk-001 ~ pk-012 | 2 | 已出泊 | promoted（pk-003 / pk-015 为 discarded） |
| **pk-013** | **0** | **在泊未到期** | None（08-25 入泊未出） |
| pk-014 | 2 | 已出泊 | promoted |
| **pk-016** | **0** | **在泊未到期** | None（08-25 入泊未出） |
| pk-015 | 2 | 已出泊 | discarded |
| pk-017 ~ pk-025 | 2 | 已出泊 | promoted |
| **pk-026** | **0** | **在泊未到期** | None（08-30 入泊未出） |
| pk-027 ~ pk-035 | 2 | 已出泊 | promoted |

## 任务包十一节请求写入与实际产物对表

| 任务包条目 | 实际产物 | 状态 |
|---|---|---|
| sih-engine/src/bin/viewer.rs | worktree 副本修复 read_args 与 load_trails_or_die + 5 单元测 | 已施工 |
| sih-engine/src/view/ | 防御性列入（src/view/heartbeat.rs 配对纯函数零改动） | 已锁位、零触碰 |
| sih-engine/src/lib.rs | 防御性列入（pub mod view 已存在，零改动） | 已锁位、未 stage |
| sih-engine/sih/event/plan/viewimpl-solo-results.md | worktree 副本更正 F-2 句 + unrouted 二改三 | 已施工 |
| sih-engine/sih/state/plan/viewfix-solo.md | 任务包（含 12 节叩问处置 key 修订） | 已落档 |
| sih-engine/sih/event/plan/viewfix-solo-results.md | 本档 | 已落档 |
| sih-engine/sih/event/trail/2026-08-31.ndjson | scribe intent + 3 append 链事件入档 | 链上累加 |
| sih-tools/scribe/reports/ | 9 件报告（红证 / 绿证 / recall / ask3-record / ask3-validation / signals / identity / 3 管线） | 已落盘 |
| sih-tools/formatter/CALL-LOG.md | 化格调用留痕 | 工具侧 |
| sih-tools/scrutinator/CALL-LOG.md | 核阅调用留痕 | 工具侧 |
| sih-tools/nomenclator/CALL-LOG.md | 检词调用留痕 | 工具侧 |
| sih-tools/scribe/CALL-LOG.md | 工具侧 scribe 已退役转沉默参考 | 沉默 |
| sih-tools/meter/counts/ | meter 包裹计数 | 自动落档 |

## 偏离与期票

| 类别 | 项 | 详情 | 期票 |
|---|---|---|---|
| 工具层 | 核阅 des-001 报 domain_mismatch | viewimpl-solo-results.md 不在 des-001 治理域内（sih-engine/doc/**），沿用 viewimpl-solo 批 vipkg 同款做法，findings=0 | 持续 |
| 工具层 | 化格 packs/general-v1 + json-canonical-v1 0 changes | viewimpl-solo-results.md 改后已合规，工具层无事项 | 持续 |
| 范围 | 配对纯函数 src/view/heartbeat.rs 零改动 | 病只在 viewer.rs 装载层，承接 06-on-canon 损补定向调节 | 持续 |
| 范围 | 三笔历史提交 f616b07 / 369a80b / a064652 零触碰 | git log 检查无 commit hash 改动；unrouted 残迹三笔处置归人节点 | 持续 |

## 后续

- 视图组件跨链腿修复落地，配对纯函数零改动红线守住
- viewimpl-solo 批结果档两处失实句入档更正（"CLI 复算" 改 "配对纯函数层复算"，"unrouted=2" 改 "unrouted=3"）
- viewer 跨链测试三件入舱：m1 红证存档断言 / m2 七链全态断言 / m3 单链回归字节一致
- 后续批次凡涉及 viewer 多链用例，可直接调用 viewfix-solo 同款 --trail 参序装载
