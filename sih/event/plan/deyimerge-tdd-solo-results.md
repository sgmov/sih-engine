# deyimerge-tdd-solo 结果档

> 得一融回三步曲第二步 TDD 实装批收口档
> 日期：2026-09-02。会话号：5e7a19c8bcbf8fd3。队形：单线形 solo。
> 承接：SPEC-014、deyimerge-sdd-solo、核阅 TDD 先例、adisp-guard-1 与 m-p3xcarr 终签材料

## 一、意图哈希与链位

- ask3 记录 sha256：`a838a2f0055f568fa43a993a9fba88f419c885beb0f82b0a7fc9da8d9b6c8a7c`
- 意图事件：`intent_refined`，event_hash 前八位 `ec0dd32e`（事件号 2ae47c49-de7b-4379-813f-fcdfe27b876e）
- 意图前链对表：85 事件（末哈希 `468d2b36`，valid）；意图后 86 事件（末哈希 `ec0dd32e`，valid）
- 双门：scrutinator ask3 包零违规 exit 0；ask3repeater status ok（三锚）
- 叩问：五词（随迁、重写、金向量、腿切分、归因）全轻级 unregistered，digest passed 5/5，处置行落包档
- 正身：identity verify anomalies 空（reports/2026-09-02-deyitdd-identity.json，不入版控）
- inputlog：2026-09-02.ndjson seq 9 补录放行令逐字笔（sess-zcode-260902-acceptor）

## 二、实装统计（F-1）

src/attractor/ 总行数 4794（.rs 实装 4794 含模块档注释），src/bin/attractor.rs 291，tests/ 三件 903。

| 模块（融回行对应） | 行数 | 职能 |
|---|---|---|
| mod.rs | 34 | 模块档与腿分层声明 |
| jsonc.rs | 196 | 围堰序列化形逐字节兼容层（canonical/紧凑形/PyError 信封/fsum/py_round） |
| contract_mode.rs | 485 | 合同 emit/load、响应严格对表、原文解析、dc 装配、飞轮幂等追加、计分材料、规范席位串、score_pipeline |
| stats/mod.rs | 16 | 门面 re-export 十一函数 |
| stats/metrics.rs | 114 | 距离度量族四函数 |
| stats/inf.rs | 175 | 推断校正族五函数（lgamma Lanczos） |
| stats/conv.rs | 324 | 收敛推断族二函数（置换检验 + ANOVA 含 betacf/reg_inc_beta） |
| compiler.rs | 1020 | 确定性聚合器全量（指标、发散发谱、决策收敛、aggregate） |
| validators.rs | 368 | 检验器注册表四件 |
| anchors.rs | 261 | 锚定三步验证 |
| model_utils.rs | 80 | 家族与框架提取 |
| paradigm_loader.rs | 119 | atom/chain yaml 装载（serde_yaml，Cargo.toml 随批声明） |
| tally.rs | 734 | 执契机械核对：check/verify/assemble/watch/sign 五子命令逻辑 + is_rules_version |
| src/bin/attractor.rs | 291 | 六子命令面 emit-contract/score/check/verify/sign/watch |
| tests/attractor_golden.rs | 353 | T1+T2 |
| tests/attractor_cli.rs | 252 | T3+T5 |
| tests/attractor_contract.rs | 286 | T4+T6 |
| lib.rs 导出行 | +2 | `pub mod attractor;` 与公共 API re-export 行 |
| Cargo.toml | +1 | serde_yaml = "0.9"（回迁债预期内，无网络无 LLM 依赖） |

## 三、腿切分清单 23 行逐条验（F-1）

融回十一行（每行对应物在位、零依赖留堰件，由 T4 t4_leg_split_manifest 测试机械钉死）：

1. contract_mode.py → src/attractor/contract_mode.rs（485 行）：融回确认。
2. facet_stats.py → src/attractor/stats/mod.rs（16 行门面）：融回确认，re-export 十一函数唯一公共入口。
3. facet_stats_metrics.py → src/attractor/stats/metrics.rs（114 行）：融回确认。
4. facet_stats_inf.py → src/attractor/stats/inf.rs（175 行）：融回确认。
5. facet_stats_conv.py → src/attractor/stats/conv.rs（324 行）：融回确认。
6. compiler.py → src/attractor/compiler.rs（1020 行）：融回确认，import 面仅 anchors 与 model_utils。
7. validators.py → src/attractor/validators.rs（368 行）：融回确认，无采样依赖。
8. anchors.py → src/attractor/anchors.rs（261 行）：融回确认。
9. model_utils.py → src/attractor/model_utils.rs（80 行）：融回确认。
10. tally/src/tally/cli.py 全件 → src/attractor/tally.rs（734 行）+ src/bin/attractor.rs（291 行）：融回确认，sign 链上写经子进程交书简。
11. tests/test_tally.py 测试基线 → tests/attractor_contract.rs T4+T6（原判据 tri_state/r5/verify/sign/assemble/watch 逐条对位随迁）：融回确认。
（附：paradigm_loader.py 为 SPEC-014 腿切分融回件依赖位，行十一内承载 → paradigm_loader.rs。）

留堰十二行：llm_client、engine.py、runner.py、env_loader、req、config、thinking_resolver、concurrency、persistence、audit、三腿 CLI 与 probes、timestamp——零迁移，围堰源码 git 对表零改动（本批双仓 settle 只动工地与锁册与链）。

零依赖确认：T5 源码扫描禁词表（llm_client/AsyncOpenAI/openai/reqwest/hyper/ureq/curl/TcpStream/api_key/env_loader/http 等）零命中；Cargo 依赖扫描零网络零 LLM 面；融回件依赖闭包全落融回件互依。时间戳 JSON 字面是围堰 trail 行 schema 契约字段（append_contract_runs 用标准时钟），非留堰件引用。

## 四、金向量清单（F-2）

冻结件 64 文件落 src/attractor/fixtures/golden/（冻结说明见其 README.md）：

- 净二：adisp-net（合同 39597 字节 + 核对报告 + signcheck + 退出码）、p3xcarr-net（核对报告 + signcheck + 退出码）
- 脏三：dirty-suspend-near（挂起）、dirty-return-r2（材料退回，R2 失败在案）、dirty-alarm-r7（裁决通过 + R7 超三次改写告警一条）
- 合同类拒收基线四形：缺发（`缺 1 发响应：['adisp-guard-1#r9']`）、键不符、shot 错位、raw 空——围堰 ValueError 消息逐字节冻结
- 计分材料二：adisp-guard-1-net（stable_clear，runs_written 9）、m-p3xcarr-net（stable_clear，runs_written 5），含合同哈希与响应哈希逐字节
- 合同 emit 金向量：围堰原件重 emit 与真实存量合同逐字节一致（39597 字节）后冻结真实字节
- 统计参照值：stats-values.json 固定输入电池（浮点文本形豁免判据）

围堰输出是唯一基准：全部期望由 freeze_golden.py 驱动围堰原件跑出，零自造。T2 哈希对表钉死冻结后零漂移。

## 五、红转绿迹与测试计数（F-3）

- 红态：tdd-red-run1.log——三测试件先行落仓，`cargo test` 编译失败 12 errors（sih_engine::attractor 缺位，SPEC-014 红判据原样）
- 绿态：tdd-green-run.log——attractor 测试全绿 28 件（golden 9 + cli 7 + contract 7 + 单测 5），全仓 155 passed / 1 failed / 6 ignored
- 唯一失败：`scrutinator::tests::golden_des001_gov003`——存量（主树同状态同样失败），GOV-003 文档在金向量冻结后更新致 content_hash 陈旧，双侧 findings 均零，修法属 scrutinator 金向量刷新批，不在本批域（红线：既有五组件源码零碰）

## 六、构建探针与活体双跑（F-4 证据）

- 构建探针：build-probe.log——cargo build 后实跑 `attractor check`，裁决报告三行在案，exit 0
- 活体双跑 cmp（同参形条款主证，零归一零 token）：五场景 check 全部 CMP-IDENTICAL 且退出码全对齐（0/0/0/1/0）；score 活体双跑 CMP-IDENTICAL（同工作区同路径串，围堰闸 v3 判 stable_clear 显式传引擎侧）

## 七、收口对表（F-4）

- 双仓 settle：tools `msh/deyimerge-tdd-solo` 段1 settle 提交与 engine `msh/deyimerge-tdd-solo` 段1 settle 提交（提交号见 lease commit 输出与链上事件）
- close：归并对表法 diff identical
- reconcile：双仓 unrouted/cert_missing/bypass/unbypassed 全零（读数见认证节后对表）
- 链 verify：valid（settle 后对表见认证节）

## 八、F 表

| F | 类别 | 判据 | 结论 |
|---|---|---|---|
| F-1 | 工程治理 | 融回十一行全落 src/attractor/，腿切分清单逐条验即融回件零依赖留堰件，留堰十二行与两工具源码零改动 | 过（第三节；围堰零改动由本批锁册与双仓 git 对表承载） |
| F-2 | 工程治理 | 金向量含净一加脏三加合同拒收基线，围堰件对同输入输出与引擎件逐字节（浮点文本形豁免外），证据入材料件 | 过（第四节 + T2 九件全绿 + 活体双跑 cmp 零差；净目标实际两件超配） |
| F-3 | 工程治理 | cargo test 全绿 T1-T6，红转绿迹入结果档 | 过（第五节；唯一失败为存量 scrutinator 金向量陈旧，与本批无关且主树同状态） |
| F-4 | 链上治理 | 双仓 settle 归并、reconcile 四类双零、链 valid、包档归档、构建探针 | 过（第七节 + 认证节读数） |

## 九、越线与误差申报

1. 链尾尾随：dispatch 基线 79 事件；意图前实为 85 事件——parkingpk-solo 批在本批开链前归并上链六事件（parking_entered pk-038、认证等）。承 guardhook 先例尾随归档，本批 intent 自 86 号事件起，验链 valid 全程。
2. 引擎主树并发归并：本批 lease open 时引擎 main 已含 parkingpk-solo 归并提交（299261e），本批工地基于该 HEAD 建立，无路径交集。
3. 判据 v3 闸未融回：SPEC-014 腿切分清单外文件不迁（maturation_gate 属 Path C 留堰），引擎 score 子命令以显式 `--gate-verdict` 参数承接闸上游产出，机械腿只装配不判闸；活体双跑中引擎侧 gate_verdict 取围堰 v3 原件判定值，逐字节 cmp 不受影响。
4. assemble 不在二进制子命令面：dispatch 钉子命令面六件（emit-contract/score/check/verify/sign/watch），assemble 逻辑落 lib（tally::assemble_material）由 T4 跨腿契约测试承载，与 SPEC-014 接口对表的机械腿六子命令在 lib 面齐备。
5. 金向量归一形：期望件路径位以 @ROOT@/@MATERIAL@/@TOPIC_ENTRY@ 三 token 冻结，测试反填后逐字节 cmp；同参形主证由活体双跑 cmp 零归一零差承载。
6. 置换检验 RNG：引擎侧 splitmix64 确定性实现，同 seed 双跑逐字节一致与统计结论等值承载判据；围堰 MT19937 序列不复刻（边界声明显式范畴排除内）。
7. 错误报文文本域：文件缺席类系统错误文本（如 io 错误平台串）与 Python 异常文本非逐字节，异常类型名对齐（ValueError/OSError/KeyError/JSONDecodeError）；该域不在四类契约工件逐字节判据内。
8. 存量失败一笔：scrutinator::tests::golden_des001_gov003 于主树与本工地同状态失败（GOV-003 content_hash 陈旧，findings 双零），非本批引入，留 scrutinator 金向量刷新批处置。
9. 包档勾选：任务包工作清单五项随本结果档全勾，验收标准 F-1 至 F-4 全过勾选随批归档。
