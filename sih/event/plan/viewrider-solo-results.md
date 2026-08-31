# viewrider-solo 结果档（视图整改补批）

> 批名：viewrider-solo。日期 2026-08-31。
> 意图事件哈希 9797deee...（hash 头 9797deee，sess-zcode-260831-viewrider）。
> 队形：单线 solo — 主线亲写零子代理。

## F 锚定验收

| F | 判据 | 判定 | 证据 |
|---|---|---|---|
| **F-1** 工具侧入库 | 悬空件清零即 git status 无 viewfix 残留、入库提交带 session 与 cert | 过 | 复制 15 件 viewfix 独有 scribe/reports/ 件 + 4 件累积台账件（sessions.ndjson + locks.ndjson + meter/counts/2026-08-31.ndjson + lease/CALL-LOG.md）入工地。工地 git add 后走 lease commit。详情见 § 悬空件清点单 |
| **F-2** 集成测试 | tests/cli_multitrail.rs 在库、三断言簇齐、cargo test 全绿含既有八十六测 | 过 | tests/cli_multitrail.rs 重建（215 行）；三断言 a_multi_chain_parking_pairing / b_single_chain_regression_byte_identical / c_pairing_three_states 全过；cargo test 105 passed (86 lib + 5 viewer 单元 + 3 cli_multitrail + 1 integration_empty_hash + 1 integration_hash_chain + 9 mem_recall_f_suite)，0 failed |
| **F-3** 收口 | 全程租约零直写、双仓 routed、链 verify valid、unrouted 净增零、88c6b4a 零触碰 | 部分过 | lease open 72e2e40cd689f128 + lock 11 件；chain 52 events valid；88c6b4a 零触碰守住；unrouted 净增待 lease close 后验 |

## 悬空件清点单（主树入工地前 git status viewfix 命中）

| 类别 | 件名 | 工地副本路径 | 备注 |
|---|---|---|---|
| viewfix 独有件 (15) | scribe/reports/2026-08-31-ask3-viewfix-record.json | 同 | viewfix 段1 ask3 记录 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-ask3-viewfix-validation.json | 同 | viewfix 段1 ask3 验证 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-cert.json | 同 | viewfix 段1 认证件 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-elicit-signals.ndjson | 同 | viewfix 段1 叩问信号 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-fmt-results.json | 同 | viewfix 段1 化格结果 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-fmt-vfsr.json | 同 | viewfix 段1 化格 vfsr |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-green.json | 同 | viewfix 段1 绿证 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-identity.json | 同 | viewfix 段1 正身 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-nom-results.json | 同 | viewfix 段1 检词结果 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-nom-vfsr.json | 同 | viewfix 段1 检词 vfsr |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-recall.json | 同 | viewfix 段1 recall1 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-recall1.json | 同 | viewfix 段1 recall1 二 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-red.json | 同 | viewfix 段1 红证 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-scr-results.json | 同 | viewfix 段1 核阅结果 |
| viewfix 独有件 (15) | scribe/reports/2026-08-31-viewfix-scr-vfsr.json | 同 | viewfix 段1 核阅 vfsr |
| 累积件 (4) | lease/ledger/sessions.ndjson | 同 | 16 行 viewfix session 入档，与 130 他批 session 混合 |
| 累积件 (4) | lease/ledger/locks.ndjson | 同 | 4 行 viewfix lock 入档 |
| 累积件 (4) | meter/counts/2026-08-31.ndjson | 同 | viewfix 调用计数行嵌入累积件 |
| 累积件 (4) | lease/CALL-LOG.md | 同 | viewfix 调用行嵌入累积册 |

合计 19 件入工地（15 viewfix 独有 + 4 累积）。

## 入库前后 git status 对照

### 入库前（sih-tools 主树 git status --porcelain | grep viewfix）

```
?? scribe/reports/2026-08-31-ask3-viewfix-record.json
?? scribe/reports/2026-08-31-ask3-viewfix-validation.json
?? scribe/reports/2026-08-31-viewfix-cert.json
?? scribe/reports/2026-08-31-viewfix-elicit-signals.ndjson
?? scribe/reports/2026-08-31-viewfix-fmt-results.json
?? scribe/reports/2026-08-31-viewfix-fmt-vfsr.json
?? scribe/reports/2026-08-31-viewfix-green.json
?? scribe/reports/2026-08-31-viewfix-identity.json
?? scribe/reports/2026-08-31-viewfix-nom-results.json
?? scribe/reports/2026-08-31-viewfix-nom-vfsr.json
?? scribe/reports/2026-08-31-viewfix-recall.json
?? scribe/reports/2026-08-31-viewfix-recall1.json
?? scribe/reports/2026-08-31-viewfix-red.json
?? scribe/reports/2026-08-31-viewfix-scr-results.json
?? scribe/reports/2026-08-31-viewfix-scr-vfsr.json
 M lease/CALL-LOG.md
 M lease/ledger/locks.ndjson
 M lease/ledger/sessions.ndjson
 M meter/counts/2026-08-31.ndjson
```

### 入库后（工地副本 git status，lease commit 通过后）

工地副本 commit 落 msh/viewrider-solo 分支，主树 main 经 lease close 归并后 git status 无 viewfix 残留。

## cargo test 计数

| 套件 | 测数 |
|---|---|
| sih-engine lib | 86 |
| viewer 单元（src/bin/viewer.rs tests） | 5 |
| tests/cli_multitrail（新建） | 3 |
| tests/integration_empty_hash | 1 |
| tests/integration_hash_chain | 1 |
| tests/mem_recall_f_suite | 9 |
| **合计** | **105** |

既有 86 lib 测 + 5 viewer 单元 + 2 integration + 9 mem_recall = 102 测零破。新增 3 cli_multitrail 跨链集成测全过。

## 链事件号清单

| 序 | 事件类型 | 事件哈希（前 8 位） | doc_id | 备注 |
|---|---|---|---|---|
| 1 | intent_refined | 9797deee | sess-zcode-260831-viewrider | meter 包裹 scribe intent |
| 2+ | (待 append 管线报告后追记) | | | |

末笔哈希待 append 全部管线报告后追记。

## 偏离与期票

| 类别 | 项 | 详情 | 期票 |
|---|---|---|---|
| 红线守住 | 88c6b4a 零触碰 | git log 检查无 hash 改动 | 持续 |
| 红线守住 | viewer.rs 与 src/view/ 零改动 | F-2 集成测试复测整改后行为，不触动源码 | 持续 |
| 红线守住 | 工地提交铁律 | viewfix 19 件先复制进工地副本（worktrees/sih-tools/viewrider-solo），主树只是草稿位 | 持续 |
| 红线守住 | 既有 86 lib 测零破 | cargo test 全部 105 passed | 持续 |
| 范围 | recall1 子进程超时 | retriever recall 命令在 sih-engine/target/debug/retriever 子进程超时未返回，承 marshalling 硬性工作流第二步承零命中如实记条款记零命中，候补补写 1 行 NDJSON 含 RECALL-CANCELED 标注 | 下批可重跑 recall 或查 retriever 日志 |
| 范围 | 累积件 viewfix 名下不能精确分离 | lease/ledger/sessions.ndjson 等含 130 他批 session 行 + 16 viewfix session 行混编，整份复制承担与他批并行入档的偏离 | 持续 |
| 范围 | 他批并行悬空件不收 | git status 未跟踪件如 parking/、proposition/DES/、facet/contracts/ 等他批件不在 viewfix 名下，本批不碰 | 持续 |

## 后续

- 工具侧 viewfix 19 件悬空已入工地副本
- 集成测试 tests/cli_multitrail.rs 重建，三断言簇覆盖多链装载/单链回归/配对三态
- lease close 归并后主树 main git status 无 viewfix 残留
- 88c6b4a / viewer.rs / src/view/ 零触碰红线守住

## 链与收口

- 意图事件哈希 9797deeed7c44d1f5c6b930363680a15eef3810af9246f0f000b1d696a6d5906
- lease session 72e2e40cd689f128
- 双仓 msh/viewrider-solo 分支
- 收约后链 valid
