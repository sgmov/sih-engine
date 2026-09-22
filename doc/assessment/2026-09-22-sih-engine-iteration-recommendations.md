# 司衡引擎迭代建议（sih-engine v0.9.0-v1.0.0）

建议日期：2026-09-22
建议对象：`/Users/moc/workspaces/SiHankor/sih-engine`
输入：7 维度评估结果，评估报告见 `/Users/moc/workspaces/SiHankor/sih-engine/doc/assessment/2026-09-22-sih-engine-evaluation-report.md`。
仓状态：手动阶段 + 测试味发布 + 基线 v1 锁定；出处：`sih-engine/AGENTS.md:20` + `doc/decision/022-semver-release-v1.md:16` + `doc/governance/BASELINE-v1.md:8`。

## 概览 {#sec0-overview}

### 0.1 评估基线 {#sec0-1-baseline}

7 维度评分详见评估报告第 1 节与第 3-9 节，按维度、分数、定位列示：

- 完成度 7.3：工具面超标；缺发布三步与部分 1.0.0 判据
- 易用性 6.0：MCP 教学载荷顶级；CLI 一致性、--help 覆盖拖分
- 可靠性 7.0：核心契约强健；风格分裂与 fixture 偏小
- 代码质量 5.0：模块切分与头注优；exitenvelope 活死 + 重复 helper
- 治理集成度 7.0：MCP 双通道治理同源；分派面 registry 化已落地，2026-09-14 腿四，残余收尾见 REC-015
- docs 6.0：AGENTS.md 自检齐、guide 分层清；README 违反自家规范
- 性能与可扩展性 4.0：编译期内嵌与依赖图克制；append O(n) per write 是事实瓶颈

总分 6.0 / 10，等权重平权。最低分维度即性能 4.0，是事实瓶颈而非推测：trail 27 天累计 3,112 行 / 3.5MB 后 O(n) per write 已可观测。

### 0.2 建议清单概览 {#sec0-2-rec-list}

本报告按 P0 / P1 / P2 三级给出 23 条建议，按优先级、计数、主轴列示：

- P0：7 条；主轴：0.9.0 发布闭环 / 治理面核心异常 / 关键契约覆盖
- P1：9 条；主轴：重复代码归并 / 风格统一 / 可扩展性补强
- P2：7 条；主轴：标准件补齐 / mega-file 拆分 / 性能优化配置

### 0.3 三条主轴 {#sec0-3-axes}

1. 轴一 0.9.0 收口与 1.0.0 准备：完成发布三步，即 tag/push/宣告，补齐 DEC-022 1.0.0 三判据与判据四材料。
2. 轴二「活死态」治理：清理 exitenvelope 漂移、4 个孤儿二进制、src/lease/ 空目录、doc/ 根级非标文件，让意图层与执行层对位。
3. 轴三「bin 自治 vs 库统一」张力收敛：sha256_hex / parse_args / CLI 参数风格 / thiserror / append IO / 哲学命题源码承接，把分散实现逐步沉淀到 lib/ 库层。

## 1 优先级框架 {#priority-framework}

### 1.1 三维评分口径 {#sec1-1-scoring}

每条建议按以下三维评估，单维度评「高 / 中 / 低」：

- 影响（Impact）：对完成度、用户入口契约、治理面穿透、关键契约正确性、扩展性的影响
  - 高：影响评估总分 ±0.3 分以上，或破坏核心契约
  - 中：影响评估总分 ±0.1-0.3 分，或影响局部体验
  - 低：影响评估总分 ±0.1 分以下，或纯风格优化
- 紧急（Urgency）：时间敏感性
  - 高：阻塞 0.9.0 / 1.0.0 关键路径，或错误累积已发生
  - 中：下个版本窗口内须有动作，否则技术债沉淀
  - 低：可延后 6 个月以上
- 依赖（Dependency）：本建议对其他建议的依赖
  - 轻：不依赖其他建议或仅依赖已确定件
  - 中：依赖 1-2 项可同时启动的建议
  - 重：依赖架构级决策或大动作前置

### 1.2 P0 / P1 / P2 划分标准 {#sec1-2-priority-tiers}

```
P0 必须做：影响高 × 紧急高 × 依赖轻，或同等紧度的轴线一/二主轴
P1 应该做：影响高 × 紧急中 × 依赖中，或影响中 × 紧急高 × 依赖轻
P2 可做：影响中或低 × 紧急低，或依赖重待架构决策
```

边界判定：当一条建议同时属两条主轴，如 REC-002 既是发布治理面又是代码质量，优先 P0；当验收判据需新决策登记（DEC-NNN）或新设计稿（DES-NNN），提升一档紧急度；当依赖尚未决策的架构级选型，归 P1 或 P2。

### 1.3 不在本报告覆盖范围 {#sec1-3-out-of-scope}

- 哲学仓内容更新：哲学仓大版本固定，工程仓不动哲学仓内容。
- 跨仓（sih-tools / sih-math / sih-philosophy）一致性双向对位，仅仓内可机械复验。
- `cargo build / test / clippy` 实测运行时数据：评估报告盲点明确未跑。
- 顶层 `sih/ scribe/ critsweep/` 三个目录内容：任务范围聚焦 `src/`，此三目录内容/职能未扫。

## 2 建议清单：按优先级 {#recommendations}

每条建议按 `REC-NNN` 编号，字段依序为：问题、建议、影响、验收判据、依赖、推荐行动窗口。

### REC-001 [P0] 完成 0.9.0 发布三步：tag/push/宣告 {#rec-001-release-three-steps}

问题：发布计划步骤 7-9 未执行。`doc/plan/release-0.9.0-v1.md:39-41` 明示「归主窗终验后执行」，git 状态未取证；发布计划 `doc/plan/release-0.9.0-v1.md` 步骤 5，即链验证，可复算但未落地；DEC-022 全文无步骤条款，步骤序列在发布计划。完整度评估 W4 直接扣分项。

建议：主窗终验成立后执行三步：
1. `git tag v0.9.0` 落 tag，与 `Cargo.toml:3` version 位双载体对表
2. `git push origin v0.9.0` 推送 tag
3. 写 `sih/event/plan/0.9.0-release-announcement-v1.md` 宣告件，载发布内容面、DEC-022 1.0.0 判据材料指针

影响：完整度评估的发布悬置项归零；发布计划步骤 5，即链验证，在档；为 1.0.0 晋升批提供首批基线。

依赖：主窗终验成立，即 DEC-022 1.0.0 判据一冷 agent MCP 面零辅助跑通的 transcript 材料归档后。

推荐行动窗口：短期 0-4 周。

### REC-002 [P0] 修复 exitenvelope 文档与代码漂移：活死模块清理 {#rec-002-exitenvelope-drift}

问题：`src/exitenvelope.rs:4-5` 头注声称「归并」「参数化承载」，实测零调用点，grep 全仓仅命中 `src/lib.rs:13` 自身。`src/bin/{gauge,lease,critsweep}.rs` 保留逐字节一致 `sort_json/emit/die` 副本；`attnanchor.rs` 仅 emit 一致、无 `die`、`sort_json` 差一处全限定写法；`meter.rs:245 die_uncaught` 与 `calllogtool.rs:366 die_value_error` 签名与 die_plain 不兼容。代码质量评估最严重弱项，severity 高。

建议：决策落地双轨二选一，登记为 DEC-027：
- A 轨：完成归并。`gauge/lease/critsweep/attnanchor` 四 bin 改为 `use sih_engine::exitenvelope::{sort_json, emit, die}`；`meter/calllogtool` 的 `die_uncaught/die_value_error` 改为 `die_plain(rc, msg)` 签名适配；`exitenvelope` 头注与代码事实对齐
- B 轨：撤回归并承诺。`exitenvelope` 模块下沉为 `#[cfg(test)] pub mod`，仅测试二进制用；头注改「已撤回归并承诺，待架构层重审」；删除库面 re-export

A 轨工作量较大，需逐 bin 验证；B 轨工作量小但承认工程债务。建议 A 轨 + 任务包拆为「exitenvelope-merge-finalize-solo」分批落地。

影响：代码质量评估文档-代码漂移弱项归零；后续维护者按头注信任可机械复验；audit 链 traceback 可解释。

验收判据：`grep -rn 'use sih_engine::exitenvelope\|exitenvelope::' src/` 命中 >= 4，按归并轨 A 应至少 4 bin 引用；或 `exitenventory` 模块头注明文承认撤回且 `src/lib.rs:13` re-export 删除。

依赖：登记为新决策 `DEC-027-exitenvelope-merging-status.md`，待签；落地为任务包 `exitenvelope-merge-finalize-solo` 或 `exitenvelope-merge-withdraw-solo`。

推荐行动窗口：短期 0-4 周。

### REC-003 [P0] 清理 target/debug/ 4 个孤儿二进制 + 构建缓存策略 {#rec-003-orphan-binaries}

问题：`target/debug/{ask3gate,ask3repeatergate,retrievergate,scribegate}` mtime 2026-08-27/28 早于当前 `src/` 结构，arm64 Mach-O 可执行落盘无对应源。CI 缓存体积异常；增量编译指纹持续误判；潜在误导新开发者认为存在该入口。

建议：登记 `DEC-028-target-cleanup-policy.md` 决策：
1. 当下清理：手动 `rm target/debug/{ask3gate,ask3repeatergate,retrievergate,scribegate}` 并跑 `cargo clean` 验证
2. 长效机制：加 `tools/clean_orphans.sh`，在 git 工作树扫除 Mach-O 无源二进制，纳入 CI
3. 文档层：`doc/guide/contributor-guide-v1.md` 加「目标目录卫生」节明示 orphan 二进制不应存在

验收判据：`target/debug/` 无 mtime 早于对应 `src/bin/` 文件的 Mach-O 二进制；`tools/clean_orphans.sh` 在 CI 流水线执行并报告零孤儿。

依赖：决策登记 `DEC-028`。

推荐行动窗口：短期 0-4 周。

### REC-004 [P0] 同步 README 工具清单与 MCP schema：19 至 21 {#rec-004-readme-mcp-sync}

问题：`README.md:184-187` 列 9 只读 + 10 写 = 19 工具；`src/mcpserver/server.rs:79-176 + 240-297` 实际 21 工具，多 record_direct + lease_unclaim 两个本地可信位。易用性弱项 #3，本地可信位仅在行内散文提名、未计入 9+10 清单计数口径，对 external 分级用户尤其混淆。

建议：修 `README.md:184-187`，补 `record_direct` + `lease_unclaim` 两行；`src/mcpserver/server.rs:419-449` 为供测试对表的 `tool_names()`/`beta_tool_names()` 函数，现值 9+12=21 已正确，无须改。

影响：易用性弱项 #3 归零；用户按 README 期望工具集与 list_tools 返回对位；本地可信位 governance 边界在文档层显式。

验收判据：`README.md` 工具清单与 `sihmcp list_tools` 21 具逐一对表，外部分级 19 + 本地可信位 2 入列标注。

依赖：无前置；同步走 PR 即可。

推荐行动窗口：短期 0-4 周。

### REC-005 [P0] T3 退出码 code=1 测试补齐 + commitlaw/lease 流一致性 {#rec-005-t3-exit-codes}

问题：可靠性评估弱项两件合一：
- `tests/lease_mergeback_t3_exit_codes.rs`，实跨 10-26 行，仅显式测 exit 0 与 exit 2，缺 exit 1；且该件头注自称「T3 退出码三值全表」，载 SPEC-024 验收判据 A2，而实装仅两值，漂移较此更重，核心契约覆盖不全
- `src/bin/lease.rs:41-49 die()` 用 `eprintln!`（stderr），`src/bin/lease/commitlaw.rs:41-44 fail()` 用 `print!`（stdout），同语义不同流

建议：双轨：
1. 补 T3 测试：在 `tests/lease_mergeback_t3_exit_codes.rs` 加 `#[test] fn t3_code1_violation_or_chain_failure()`，使用 `lease` 子命令触发 `code 1` 退出路径
2. 流一致性登记为 `DEC-029-stdout-stderr-discipline.md`：明确「错误载荷统一 stderr，成功载荷 stdout」，`commitlaw.rs` 的 `fail()` 改为 `eprintln!`；其他 `die()` 一并核查对齐

验收判据：`tests/lease_mergeback_t3_exit_codes.rs` 含 `code 1` 测试且通过；`grep -rn 'print!.*"error"\|eprintln!.*"error"' src/bin/lease/` 命中流统一为 stderr。

依赖：决策登记 `DEC-029`。

推荐行动窗口：短期 0-4 周。

### REC-006 [P0] SDDG-1 内单测补强：fail-closed 测试覆盖 {#rec-006-sddg1-tests}

问题：可靠性弱项。`src/bin/lease/sddgate.rs` 内单测仅覆盖 SDDG-2/3/4 计 6 个，SDDG-1 仅由外部 `tests/lease_mergeback_t6_close_gates.rs:312` 覆盖，单点失效无内层屏障。DEC-022 修订一已把 SDD/TDD 完备度闸列为晋升判据四，闸面不全测则判据四难以闭环。

建议：在 `sddgate.rs` 内部测试模块加 `#[cfg(test)] mod sddg1_tests`，至少 3 个测试：
1. `sddg1_happy_path_teaching_embed_present`：正典含 teaching embed 时闸面拒
2. `sddg1_missing_teaching_embed_fail_closed`：teaching embed 缺席时按 fail-closed 返回 reject
3. `sddg1_malformed_teaching_embed_fail_closed`：embed 形可解析但语义不合时按 fail-closed 返回 reject

影响：可靠性弱项归零；DEC-022 判据四材料在档；SDD 完备度闸单点失效有内层屏障。

验收判据：`grep -c '#\[test\]' src/bin/lease/sddgate.rs` 命中 >= 9；`cargo test --bin lease` 全通过，sddgate 为 lease bin 内模块非独立 bin。

依赖：无前置。

推荐行动窗口：短期 0-4 周。

### REC-007 [P0] convergence 层承接深度补强：KNOW-001 拆分为多档 {#rec-007-convergence-split}

问题：完成度弱项 W5。convergence 层承接仅 `doc/knowledge/KNOW-001-convergence-retrieval-map.md` 单档，承载 P1.1/P1.2/P1.3/P3.1/P3.2/P4.2/P4.3 等多命题，覆盖深度未实测。AGENTS.md:42-63 七处均指向单档，无第二档证据。

建议：登记 `DEC-030-knowledge-multifile-policy.md`，明确「知识档按命题族分档」的归桶规则。然后将 `KNOW-001` 拆分：
- `KNOW-001-convergence-retrieval-map-v1.md` 留为索引档，指对下列子档
- `KNOW-002-witness-framework-derivation-v1.md` 承载 L1/L2/M 三阶证明体系，即 L1 形式逻辑 / L2 范畴论 / M 模型
- `KNOW-003-convergence-retrieval-steps-v1.md` 承载六步检索步骤的工程化映射
- `KNOW-004-retrieval-binding-canon-v1.md` 承载 C/P/R 两套编号体系的区分

影响：完成度弱项 W5 归零；convergence 层承接深度可实测；为 convergence 层命题的代码层承接提供清晰路径，与 §5.1 哲学源码承接度量协同。

验收判据：`doc/knowledge/` 目录下 KNOW-002/003/004 三档存在；`KNOW-001` 索引档指对子档；每子档至少含一节「工程承接面」明示代码层锚定位。

依赖：决策登记 `DEC-030`；与 §5.1 哲学源码承接度量协同推进。

推荐行动窗口：中期 1-3 月。

### REC-008 [P1] sha256_hex / parse_args 重复代码归并到库层 {#rec-008-hashutil-cliargs}

问题：代码质量弱项。`sha256_hex` 在 `src/bin/{lease,gauge,cascade,confledger,identity,incubation,acceptor,projsnap,formatter,basemgr,locator,tally,parser}.rs` 共 13 bin 重复；`parse_args` 在 `src/bin/{lease,critsweep,gauge,basemgr,latextool,scrutinator}.rs` + `src/pendline/mod.rs` 共 7 处独立手卷，签名/语义各异。

建议：双轨归并：
1. 在 `src/lib.rs` 新增 `pub mod hashutil`，公开 `sha256_hex(bytes: &[u8]) -> String` 一函数，封装 sha2 + hex；13 bin 改为 `use sih_engine::hashutil::sha256_hex`
2. 在 `src/lib.rs` 新增 `pub mod cliargs`，公开 `parse_kv_flags(argv: &[String]) -> BTreeMap<String, Vec<String>>` 一函数，含 `--flag=value` 与 `--flag value` 双形支持；7 处 `parse_args` 改为调用 `cliargs::parse_kv_flags`，各 bin 在其上层做参数语义特化

注：归并不必一步到位，建议登记 `DEC-031-hashutil-cliargs-merge.md` 决策，分批归并：先 hashutil 易行；cliargs 涉及迁移行为，分 2-3 批。

影响：代码质量弱项 #2 与 #3 归零；13 与 7 处重复消除；维护者改哈希算法或 CLI 行为只需改一处。

验收判据：`grep -rn 'fn sha256_hex' src/bin/` 命中 <= 1，仅库层；`grep -rn 'fn parse_args' src/` 命中 <= 1；所有 bin `cargo build` 通过。

依赖：决策登记 `DEC-031`；分批归并任务包 `hashutil-merge-solo` + `cliargs-merge-solo`。

推荐行动窗口：中期 1-3 月，cliargs 涉及迁移，分批。

### REC-009 [P1] CLI 参数风格统一：3 种并存统一为 1 种 {#rec-009-cli-style}

问题：易用性弱项 #1。`src/bin/scribe.rs:262-268` 不支持 `--flag=value`；`src/bin/critsweep.rs:130-134` 支持 `--flag=value`，即 `split_once('=')`；`src/bin/lease.rs:106-119` BTreeMap 收集 + allow/repo 重复键支持。三种风格并存。

建议：以 `critsweep` 形为标准，即支持 `--flag=value` + `--flag value` 双形，登记 `DEC-032-cli-flags-convention.md` 决策。然后：
1. `scribe.rs` 改为支持 `--flag=value`，增加 split_once 路径
2. `lease.rs` 重复键支持保留，不与 `critsweep` 形冲突，可统一在 `cliargs::parse_kv_flags` 处理
3. 在 `doc/guide/contributor-guide-v1.md` 加一节「CLI 参数风格约定」明示 `=` 可选

影响：易用性弱项 #1 归零；用户跨工具切换无需记每种的子集；复制粘贴脚本在工具之间通用。

验收判据：所有 bin `cargo run -- --help` 输出一致，均含 `KEY=VALUE` 或 `KEY VALUE` 示例；`tests/cli_multitrail.rs` 加一组跨 bin 参数一致性测试。

依赖：REC-008 的 `cliargs-merge` 任务包；`DEC-032`。

推荐行动窗口：中期 1-3 月。

### REC-010 [P1] AGENTS.md 内部工具 bin 索引补全 {#rec-010-agents-index}

问题：docs 弱项 W4。`AGENTS.md:22-38` 仅列 4 个第一阶段组件：意图锚定 / 符号材料生成 / 参验 / 事件流，加第二阶段预登记 2 件：视图 / 微积分。`src/bin/` 实际 36 个内部工具 bin，`src/bin/` 实测 36 个 .rs，Cargo.toml 零 `[[bin]]` 段自动发现，`lease/` 目录 7 子模块不计独立 bin；`lease.rs` 729 行 + 7 子模块 5184 行、`parser.rs` 3012 行、`locator.rs` 2872 行均未在 AGENTS.md 索引。新贡献者只读 AGENTS.md 会以为仓只有 4 个组件。

建议：登记 `DES-020-agents-md-internal-bins-index.md` 设计稿。DES 实数 19、编号至 019，顺延即 020；原拟 DES-024 系跳号勘误。明确 AGENTS.md 索引分两段：
- 第一段「阶段组件」保留现有 4+2：意图锚定 / 符号材料生成 / 参验 / 事件流 / 视图 / 微积分
- 第二段「内部工具 bin」加 36 bin 总表：路径 + 行数 + 移植源 + 入口点 + 与阶段组件关系

注：与评估观察 O2 一致，组件索引 vs 库面 12 mod 的不对称应同时承认：src/lib.rs 实测 12 个 pub mod，12 mod 中 cascade_registry / exitenvelope / tools_registry / snapline / askroute 五件未在 AGENTS.md 出现。

影响：docs 弱项 W4 归零；评估观察 O2 同时归零；新贡献者入门曲线降陡。

验收判据：`AGENTS.md` 含两段索引；36 bin 全列；12 mod 全列；总行数 <= 200，守 DES-001 字符约束前提下。

依赖：`DES-020` 登记。

推荐行动窗口：短期 0-4 周，设计稿写作 + AGENTS.md 重写工作量小。

### REC-011 [P1] thiserror 错误处理跨模块统一 {#rec-011-thiserror}

问题：可靠性弱项。`thiserror` 仅 2/122 文件使用，即 `src/ask3repeater/validate.rs:11` + `src/tools_registry.rs:35`；`src/event_stream/` 9 个领域 Error 枚举均非 thiserror；其中 4/9 即 Append、Verify、Reading、Crosscheck 手写完整 `derive(Debug, Clone, PartialEq, Eq)`，余 5/9 即 Intent、LockGate、Park、Certify、SessionGate 仅 `derive(Debug)`，错误显示风格跨模块不一致。

建议：登记 `DEC-039-error-derive-policy.md`，决策补强，原 OQ-09；原拟号 DEC-009 与既有 009-git-governance-boundary.md 冲突，勘误改号。明确「event_stream 库件须用 thiserror」。然后：
1. `src/event_stream/{append,verify,intent,...}.rs` 9 个 Error 枚举改为 `#[derive(thiserror::Error)]`，按 `#[error("...")]` 标注
2. 既有 derive 保留：4/9 全四元、5/9 仅 Debug；PartialEq 用于 match 解构
3. `tests/` 内 match 错误变体的测试无须改写，语义不变

影响：可靠性弱项归零；错误显示风格跨模块一致；外部用户面对错误时拿到一致诊断。

验收判据：`grep -rn 'enum.*Error' src/event_stream/` 全部 `#[derive(thiserror::Error)]`；9 个 Error 枚举验证完毕；`cargo test --lib event_stream` 全通过。

依赖：`DEC-039` 登记；`Cargo.toml` 已含 `thiserror = "1"`，零依赖增。

推荐行动窗口：中期 1-3 月。

### REC-012 [P1] README / RANTS 字符约束遵循：DES-001 general.md:107-194 {#rec-012-des001-chars}

问题：docs 弱项 W1 与 W2。`README.md` 含 13 处全角破折号 + 20 处表格行 + 1 处水平分割线；`RANTS.md` 含 17 处破折号 + 大量 emoji + 粗体 + 块引用。DES-001 明文禁用前三者，但未在 GOV 文档明示 README / RANTS 是否受覆盖。

建议：登记 `DEC-033-des-001-scope-clarification.md`，明示：
- AGENTS.md / doc/guide/ / DEC/DES/SPEC/GOV/KNOW/PRO 文档族受 DES-001 全约束
- README.md 作为入口宣传件享「行内代码 + 围栏代码块 + 必要表格」三种例外
- RANTS.md 作为吐槽体风格件不归入 DES-001 受辖文档，但根级落仓须在 AGENTS.md 自检节明示「RANTS.md 风格豁免」

然后处理：
1. README.md 破折号改为「,」「;」或句号；保留表格，README 入口级允许表格作概览
2. RANTS.md 根级位置不动，加 frontmatter `style: exempt-from-DES-001` 标注

影响：docs 弱项 W1 与 W2 归零；DES-001 适用范围清晰化；入口级文档与规范类文档边界分明。

验收判据：`README.md` 破折号 0 处；`RANTS.md` frontmatter 含 `style: exempt`；`DEC-033` 在档。

依赖：`DEC-033` 登记。

推荐行动窗口：短期 0-4 周，README 改写 + DEC-033 登记。

### REC-013 [P1] append 路径 O(n) per write 优化 {#rec-013-append-opt}

问题：性能评估弱项 W1，事实瓶颈。`src/event_stream/append.rs:108-131` 路径在场时 open + flock + `load_events(p)` 全量读 + writeln! 一行回写。`src/event_stream/` 内 `BufReader/BufWriter` 零命中。现场 `sih/event/trail/` 27 天累计 3,112 行 / 3.5MB，最大单文件 354KB / 301 行；按线性增长 1 年约 50MB/单链，每次 append 全量解析成本随链长线性增长。

建议：登记 `DES-021-trail-append-optimization.md` 设计稿明确优化路径，不一次性大动，分两阶段：
- 阶段一，小步，BufWriter + 增量哈希缓存：`append.rs:128` 改为 `BufWriter::new(file)`；当日文件维持日切不变；`hash.rs` 改为维护「链尾事件到 tail_hash」轻量索引，可走 mmap 或辅助文件 `sih/event/trail/<date>.idx`
- 阶段二，中等，按 offset 索引的「尾段读 + append」：trail 文件末尾 64KB 缓冲即可，每次 append 仅读尾段；`hash` 计算时通过索引链上溯
- 阶段三，待评估，切日文件 + 跨日链接：日切已存在，跨日链接可在 `verify_chain` 加 `cross_day_link` 表

阶段一已足够应对 1 年增长曲线：50MB 链 + BufWriter 单写成本 < 100ms。

影响：性能评估弱项 W1 缓解；trail 增长曲线下 append 成本从 O(n) 降为 O(1) 单日 + O(1) 跨日链查询。

验收判据：`tests/integration_hash_chain.rs` 扩展为 1MB / 10MB / 50MB fixture；hyperfine 实测 append 在 50MB trail 上 < 100ms。

依赖：`DES-021` 登记 + `decision/trail-append-optimization-solo` 任务包。

推荐行动窗口：中期 1-3 月即阶段一；长期 3-6 月即阶段二/三。

### REC-014 [P1] trail 压缩 / 归档策略引入 {#rec-014-trail-compression}

问题：性能评估弱项 W2。`src/` 内 `compress|gzip|zstd` 零命中，`archive` 于 retriever 记忆档案域命中百余处，与 trail 压缩无关；现场 27 个 `.ndjson` 文件平均 1.18KB/行，事件 JSON 含 details，文本冗余高。典型压缩比 5-10×，3.5MB 可压至 0.5MB 量级。存储成本随写入线性增长，且无任何老化机制。

建议：登记 `DES-022-trail-compression-policy.md` 明确压缩策略：
1. 格式选型：选 zstd，即 `zstd = "0.13"` crate 待评估，而非 gzip，压缩比相当、解压更快、原生日志场景更友好；不选嵌套 YAML / 多层 JSON，按 AI 生成友好度评估，仅简单文件后缀 `.ndjson.zst`
2. 触发时机：日切时刻触发压缩，`src/mcpserver/runtime.rs:45` 的 `today_str` 路径延后 24h 即压缩昨日文件，该文件全文 176 行、196 行不存在、原为行号笔误；不在线追加压缩，避免与 flock 路径冲突
3. 压缩与 verify 兼容：`verify_chain` 须检测后缀 `.zst` 自动走 zstd 解压路径；保留原 `.ndjson` 解压路径，向后兼容
4. 归档策略：超 90 天的压缩文件可移至归档子目录 `sih/event/trail/archive/`，不删仅移

影响：性能评估弱项 W2 归零；trail 存储成本降 5-10×；verify 链路兼容。

验收判据：`src/event_stream/verify.rs` 支持 `.zst` 后缀自动解压；`tools/compress_yesterday_trail.sh` 在 CI 流水线每日执行；`sih/event/trail/*.ndjson.zst` 存在。

依赖：`DES-022` 登记 + `trail-compression-solo` 任务包。

推荐行动窗口：中期 1-3 月。

### REC-015 [P2] registry 接线收尾：申报注了结 + SPEC-025 修订 + fixture 验证 {#rec-015-registry-wiring}

问题：治理集成度弱项 #1 的接线主体已落地：2026-09-14 腿四接线批完成分派面 registry 化，`src/mcpserver/server.rs` 的 `list_tools` 走 `self.registry.list()`，366-369 行；`call_tool` 走 `self.registry.call()`，401 行，注释明言 A3 接线；`src/mcpserver/providers.rs` 统一注册内置 19 具 ToolProvider，散点 match 分派已废弃。残余三件收尾：其一 `src/bin/registrydemo.rs:4-8` 与 `src/tools_registry.rs:16-20` 旧申报注，即「在役面改造让位后批」与「MCP 面接线改造候腿五收口批」，未对了结；其二 SPEC-025 插件槽位协议修订未登记；其三插件 fixture `tests/fixtures/plugins/` 入 registry 加载未验证。

建议：三收尾子项一次收口，登记 `DEC-034-mcp-registry-wiring.md`：
1. `src/bin/registrydemo.rs:4-8` 与 `src/tools_registry.rs:16-20` 旧申报注加了结注记，非删除，明示接线已由腿四接线批落地
2. 登记 `SPEC-025-plugin-slot-protocol.md` 修订，与 `sih-tools/mcpline/` 行为对等基准对齐
3. 插件 fixture `tests/fixtures/plugins/` 加载入 registry 跑通验证

影响：治理集成度弱项 #1 收尾归零；SPEC-025 修订在档；registry 分派面防回退。

验收判据：`call_tool` 走 registry 回归判据在档，防回退；插件 fixture 加载入 registry 判词在档；SPEC-025 修订在档。

依赖：`DEC-034` 登记，收尾体量小。

推荐行动窗口：短期 0-4 周。

### REC-016 [P1] 不变式正测钉住：REC-016 修订，should_panic 不可达 {#rec-016-invariant-tests}

问题：可靠性弱项。全 `src/` 与 `tests/` `grep '#[should_panic]'` 0 命中；`hash.rs:26` / `hash.rs:65` / `append.rs:123` 三处构造性 `.expect()` 文本未通过测试钉死。若 `.expect` 触发条件实际发生，如哈希公式版本漂移，无测试捕捉。关键事实：三处 expect 属理论上不可达的构造性不变式，`String` 与 `BTreeMap<String, Value>` 的 serde 序列化实际不可失败，`#[should_panic]` 无法直接触发；强测不可达路径须注入化重构，动哈希核心契约，不取。

建议：在 `src/event_stream/` 内单测改钉三条不变式正测：
1. hash.rs 全字段事件可哈希，全字段 serde 走通即构造性不变式 1
2. hash 载荷序列化确定性，同输入两跑逐字节一致且非空，即不变式 2
3. append 后链尾必在，`append_in_memory` 于空链追加后 `last()` 为 Some，即不变式 3

未来引入可失败序列化源时再补 should_panic。

影响：可靠性弱项归零；构造性不变式以正测钉死；防哈希公式版本漂移未同步。

验收判据：`src/event_stream/` 内 `#[test]` 总数自 99 增至 102；`cargo test --lib event_stream` 全通过。

依赖：无前置。

推荐行动窗口：短期 0-4 周。

### REC-017 [P1] doc/ 根级非标文件归桶：AGENTS-RETIRED、CASCADE.json {#rec-017-doc-root-files}

问题：docs 弱项 W6 + 治理集成度弱项 #5。`doc/AGENTS-RETIRED-2026-09.md` 实测 42.7 KB 即 43,725 bytes，与 `doc/CASCADE.json`（100,825 bytes）直接落仓根，未归入 `governance/decision/design/spec` 任一子目录。命名格式不与 `DEC-NNN-xxx.md` / `DES-NNN-xxx.md` 一致，对遵循命名权威的治理链工具，即 `nomenclator` / `scrutinator`，解析门槛提高。

建议：
1. `doc/AGENTS-RETIRED-2026-09.md` 移至 `doc/retired/AGENTS-RETIRED-2026-09.md`，新建 `doc/retired/` 子目录承载退役 AGENTS；登记 `DEC-035-doc-retired-directory-policy.md` 明示「retired 子目录承载过期治理件」
2. `doc/CASCADE.json` 移至 `doc/knowledge/CASCADE-decidable-cascade-v1.json` 或在子目录定位后再议，如属级联决策索引则归 `doc/decision/` 子目录新建 `cascade-index/`

影响：docs 弱项 W6 + 治理集成度弱项 #5 归零；doc/ 根级不再承担「废弃件归档」职能；命名权威解析一致性。

验收判据：`doc/AGENTS-RETIRED-2026-09.md` 与 `doc/CASCADE.json` 在根级 0 件；`doc/retired/` 与目标子目录存在；`DEC-035` 在档。

依赖：`DEC-035` 登记。

推荐行动窗口：短期 0-4 周。

### REC-018 [P2] CONTRIBUTING / CHANGELOG / LICENSE 标准文件引入 {#rec-018-standard-files}

问题：docs 弱项 W3。`sih-engine/` 顶层 `glob CONTRIBUTING* CHANGELOG* HISTORY* LICENSE*` 命中 0 件。GitHub / crates.io 等平台自动识别 `CONTRIBUTING.md` 而非 `doc/guide/contributor-guide-v1.md`，新贡献者发现门槛提高。LICENSE 缺失使 crates.io 发布受阻，待确认。

建议：
1. `CONTRIBUTING.md` 顶层：`ln -s doc/guide/contributor-guide-v1.md CONTRIBUTING.md` 软链接或写 50 行摘要 + 详细链接
2. `CHANGELOG.md` 顶层：写 0.9.0 段，载发布内容面 + DEC-022 1.0.0 判据链接；未来 PATCH/MINOR 段累加
3. `LICENSE` 顶层：评估仓库许可证意向，建议 Apache-2.0 或 MIT，待 m 决策；登记 `DEC-036-license-selection.md`

影响：docs 弱项 W3 归零；GitHub / crates.io 自动识别生效；新贡献者发现门槛降低。

验收判据：所列 3 文件存在；`cargo publish --dry-run` 不报 LICENSE 缺失。

依赖：`DEC-036` 决策。

推荐行动窗口：短期 0-4 周，轻量工作。

### REC-019 [P2] examples/ 在 Cargo.toml `[[example]]` 显式声明 {#rec-019-example-decls}

问题：inventory 盲点 #7。`examples/{golden_vectors,rebuild_hash_chain,verify_external_trail}.rs` 3 件靠 cargo 自动发现，`Cargo.toml:1-26` 无 `[[example]]` 段。版本锚定与依赖关系不显式。

建议：在 `Cargo.toml` 加 3 个 `[[example]]` 段，每段含 `name` + `path` + 必要 `required-features`。例如：
```
[[example]]
name = "golden_vectors"
path = "examples/golden_vectors.rs"
```

影响：inventory 盲点 #7 归零；examples 依赖关系清晰；与 `dev-dependencies` 版本对齐可机械复验。

验收判据：`grep -c '\[\[example\]\]' Cargo.toml` 命中 >= 3；`cargo build --examples` 通过。

依赖：无前置。

推荐行动窗口：短期 0-4 周。

### REC-020 [P2] release profile 优化：[profile.release] {#rec-020-release-profile}

问题：性能评估弱项 W3。`Cargo.toml:1-27` 无 `[profile.*]` 段，无 `build.rs`，grep 验证。release 默认 `opt-level=3` + 无 LTO + 无 `codegen-units` 调整。36 个 bin 编译时间未短化。

建议：加 `[profile.release]` 段：
```
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = "symbols"
```
thin LTO + codegen-units = 1 在 36 bin 规模上可接受编译时长；`strip = "symbols"` 进一步缩二进制体积。

影响：性能评估弱项 W3 归零；release 二进制体积降 30-50%；运行延迟榨干尾延迟。

验收判据：`time cargo build --release` 实测编译时长记录；`ls -la target/release/sihmcp` 体积记录；与 debug 对比降幅。

依赖：无前置；改 Cargo.toml 即可。

推荐行动窗口：中期 1-3 月。

### REC-021 [P2] parser / locator / bootstrap mega-file 拆分 {#rec-021-megafile-split}

问题：代码质量弱项 #4。`src/bin/parser.rs` 3012 行 / `src/bin/locator.rs` 2872 行 / `src/mcpserver/bootstrap.rs` 1691 行，parser.rs 与 locator.rs 零内部 mod；bootstrap.rs 于 1469 行有一处 `#[cfg(test)] mod tests`，测试模块非功能子模块。reader 须全文件滚动；改一处风险高。

建议：登记 `DEC-037-mega-file-split-policy.md`，按功能分子模块：
1. `parser.rs` 拆为 `parser/{lexer,peg,entries,lint,vectors,main}.rs` 五子件
2. `locator.rs` 拆为 `locator/{paths,resolver,indexer,query,main}.rs` 五子件
3. `bootstrap.rs` 拆为 `bootstrap/{domain_setup,cli,web_router,main}.rs` 四子件

注：parser.rs:3003-3011 启动 512MB 栈线程容纳深递归；若 mod 化，至少栈配置与公开面可单元测。

影响：代码质量弱项 #4 归零；单文件 < 800 行；可单测子模块。

验收判据：所列 3 文件改写为子目录模块；`cargo build` 通过；`cargo test` 全通过；行数校验 <= 800/子模块。

依赖：`DEC-037` 登记 + 三个任务包 `parser-split-solo` / `locator-split-solo` / `bootstrap-split-solo`。

推荐行动窗口：长期 3-6 月，重构工作量大。

### REC-022 [P2] fixtures 大规模样本扩充：trail 增长压测数据 {#rec-022-fixture-expansion}

问题：评估盲点。`fixtures/2026-07-27.ndjson` 仅 6 事件、`fixtures/2026-08-03.ndjson` 仅 7 事件、`fixtures/golden/event-stream-vectors.json` 仅 5 个金向量。样本充分性未验证；trail 增长压测无基准。

建议：扩充 fixtures：
1. `fixtures/golden/event-stream-vectors-1k.json`：1,000 事件金向量，自动生成
2. `fixtures/golden/event-stream-vectors-10k.json`：10,000 事件金向量
3. `fixtures/stress/trail-1mb.ndjson`：1MB 真实链路，可从 `sih/event/trail/2026-09-02.ndjson` 354 KB 扩到 1MB
4. `fixtures/stress/trail-50mb.ndjson`：50MB 链路，用于 REC-013 性能基准

影响：评估盲点 B10 / 可靠性弱项「fixture 偏小」归零；trail 增长压测有基准；金向量覆盖范围更全。

验收判据：`fixtures/golden/` 与 `fixtures/stress/` 目录扩充完毕；`cargo test` 全通过；`hyperfine` 实测 50MB trail append 耗时记录。

依赖：REC-013 append 优化，即基于此基准评估。

推荐行动窗口：中期 1-3 月。

### REC-023 [P2] concurrency 模型抽象统一：33 同步 + 3 tokio {#rec-023-concurrency}

问题：性能评估弱项 W4。36 个 bin 中 3 个用 tokio，即 sihmcp、sih、registrydemo；其余 33 bin 是纯同步 CLI。`src/event_stream/` 内 `tokio::fs` 零命中，无 `spawn_blocking` 兜底。异步路径若混入 std::fs 同步 IO 会阻塞 reactor 线程。

建议：登记 `DEC-038-concurrency-abstraction-policy.md` 明确两类边界：
1. CLI bin，33 个：维持同步 IO，不引入 tokio；与 stdio/HTTP 入口（sihmcp）通过子进程编排隔离
2. server bin，即 sihmcp + 后续 service bin：统一走 tokio + axum 异步栈；同步 IO 走 `spawn_blocking` 兜底
3. `event_stream` 库件：维持同步 IO，即 std::fs + flock，仅在 sihmcp 服务侧调 `runtime::run_in_blocking_pool` 包装

影响：性能评估弱项 W4 归零；并发模型清晰；IO 模型抽象统一，避免异步上下文踩坑。

验收判据：`DEC-038` 在档；`src/mcpserver/runtime.rs` 加 `run_in_blocking_pool` 函数；`grep -rn 'tokio::fs' src/event_stream/` 命中 0。

依赖：`DEC-038` 登记。

推荐行动窗口：长期 3-6 月，架构级抽象落地。

## 3 实施路线图 {#roadmap}

### 3.1 短期 0-4 周：落地 P0 + 轻量 P1 {#roadmap-short}

P0 七条须在 0.9.0 发布前落地，即发布计划步骤 7-9 执行前：

- W1：REC-001 发布三步 + REC-002 exitenvelope 治理 + REC-003 孤儿二进制 + REC-004 README 同步 + REC-006 SDDG-1 内单测
- W2-W3：REC-005 T3 code 1 + 流一致性 + REC-010 AGENTS.md 索引补全 + REC-012 字符约束 + REC-016 不变式正测 + REC-017 doc/ 根级归桶 + REC-018 标准文件 + REC-019 examples 显式
- W4：REC-007 启动即 KNOW-001 拆分 + REC-011 启动即 thiserror 统一

### 3.2 中期 1-3 月：落地 P1 重头戏 {#roadmap-mid}

- M1：REC-022 fixtures 压测基准先行，先有尺子再优化 + REC-008 hashutil 归并先行 + REC-013 阶段一，BufWriter + tail hash 索引 + REC-014 zstd 压缩
- M2：REC-008 cliargs 归并分批 + REC-009 CLI 风格统一 + REC-011 thiserror 统一
- M3：REC-007 KNOW-001 拆分收口 + REC-015 registry 接线收尾 + REC-020 release profile

### 3.3 长期 3-6 月：架构级动作 {#roadmap-long}

- Q1-Q2：REC-013 阶段二/三，offset 索引 + 跨日链接 + REC-021 mega-file 拆分三件
- Q3：REC-023 concurrency 模型统一 + 1.0.0 晋升批准备，DEC-022 判据一/二/三/四 材料归档

### 3.4 路线图依赖图：简化 {#roadmap-deps}

```
REC-001 发布三步：依赖 REC-002/003/004/005/006 全部就位，主窗终验成立后执行
REC-008 hashutil/cliargs 归并：与 REC-009 CLI 风格统一共享 cliargs，DEC-031 登记
REC-022 fixtures 压测基准：先于 REC-013 append 优化，先有尺子，DES-021 登记
REC-014 trail 压缩：与 REC-013 共用 BufWriter / 索引抽象
REC-015 registry 接线收尾：SPEC-025 插件槽位协议修订登记
REC-007 KNOW-001 拆分：与 §5.1 哲学源码承接度量协同
```

## 4 风险与权衡 {#risks}

### 4.1 关键风险点 {#risks-key}

按风险、影响面、缓解三字段列示：

- 风险：exitenvelope 归并破坏字节级副本稳定性；影响面：gauge/lease/critsweep 逐字节对表测试可能红；缓解：走 A 轨同步更新金向量，走 B 轨头注明示撤回
- 风险：append 优化引入新并发 bug；影响面：hash 链核心契约破坏则不可逆；缓解：分阶段方案，阶段一最小动，保留 `--legacy-append` 旗标
- 风险：thiserror 改造破坏 match 解构；影响面：9 个 Error 枚举变体匹配；缓解：保留 `derive(PartialEq, Eq)`，逐 Error 迁移并跑全测试
- 风险：registry 接线破坏 MCP 双通道兼容；影响面：HTTP / stdio 同源正典破坏；缓解：sihmcp + integration_http + stdio_auto_bootstrap 三轮回归
- 风险：KNOW-001 拆分破坏索引档引用；影响面：AGENTS.md:42-63 七处引用须同步；缓解：索引档迁移过渡期双指对，同时指向旧路径与新路径
- 风险：trail 压缩触发 zstd 解压失败；影响面：`verify_chain` 路径破坏；缓解：保留原 `.ndjson` 路径，向后兼容，verify.rs 后缀检测二走

### 4.2 关键权衡 {#tradeoffs-key}

权衡一 A 轨 vs B 轨（exitenvelope）：A 轨完成意图层与执行层对位但工作量大；B 轨工作量小但承认工程债务。建议 A 轨。

权衡二 REC-013 阶段一 vs 阶段二/三：阶段一 BufWriter + tail hash 索引工作量 1-2 周缓解 1 年增长曲线；阶段二/三 offset 索引工作量 1-2 月应对 5 年增长。建议阶段一立刻 + 阶段二/三 候后批。

权衡三 CONTRIBUTING.md 软链接 vs 摘要：建议摘要 + 软链接双管齐下。

权衡四 thiserror + anyhow 边界：service 侧 anyhow + 库件侧全面 thiserror 化，不引入混用治理复杂度。

权衡五 P0 同步 vs 分批执行：主窗终验成立时间 T 距离当下 0-4 周时，建议 4 条同步并行，W1 主 + W2-3 副；W4 单 REC-007 相对独立单列。

## 5 度量与回验 {#metrics}

### 5.1 度量指标 {#metrics-list}

每条 REC 落地后须有可观测度量，按维度分：

代码质量：重复函数计数收敛，即 `fn sha256_hex` / `fn parse_args`；单文件 <= 1500 行，REC-021 目标 <= 800；bin 调用共享库件比例 >= 80%；thiserror 覆盖 = 9/9。

可靠性：event_stream 测试函数 >= 102，基线 99 + REC-016 修订后不变式正测 3，`#[should_panic]` 因不可达路径撤除；T3 测试 >= 3；SDDG 测试 >= 9；fixture 总行数 >= 50,000。

性能：50MB trail append 耗时 < 100ms，REC-013 阶段一后；trail 存储 < 1MB，REC-014 后；release sihmcp 体积 <= 12MB。

易用性：README 工具清单 = 21 与 MCP defs 对位；`split_once('=')` 覆盖 >= 30 bin；`--help` 退出 0 的 bin 占比 >= 80%。

治理：DEC-NNN 落地率持续上升；`sih-philosophy/` 源码承接 >= 5，当前窄口径 1：字面路径引用实测 src/event_stream/tdd_tests.rs:257 一处，宽口径命题锚承接 >= 8 处含 attractor 模块，计数口径候 DEC 裁定，两读数并列在案；ToolsRegistry 接线 >= 2 处派发。

docs：DES-001 合规，README 例外；AGENTS.md 含 4+2 + 36 bin 表；CONTRIBUTING / CHANGELOG / LICENSE 齐全。

### 5.2 回验机制 {#metrics-reverify}

每条 REC 落地后须三轮回验：

1. 静态回验，PR 阶段：CI 跑 cargo build / test / clippy / fmt 全通过
2. 集成回验，merge 前：`tests/integration_hash_chain.rs` + `tests/integration_empty_hash.rs` + `tests/mergeall_final_bridge.rs` 三件集成测试
3. 治理回验，DEC 登记时：签字 + 评估依据 + 风险记录，CMMI Class A 评估师；`audit/000N-decision-D-NNN.md` 双写到审计链

P0 须三轮全过；P1 须前两轮；P2 须第一轮。

### 5.3 与既有评价的关系 {#metrics-relation}

本报告作为评估报告的姊妹篇存在：评估报告回答「是什么」，即 what；本报告回答「怎么办」，即 how。引用约定：

- 评估报告引用为 `eval-{维度}-{编号}`，如 `eval-代码质量-W1`
- 本报告引用为 `REC-NNN`
- 交叉引用格式：`REC-NNN` 详见 `eval-XXX-YYY`

回验时建议两份报告对照阅读。

## 6 修订记录 {#revisions}

2026-09-22 初版：基于 7 维度评估结果，总分 6.0 / 10，出具 23 条建议：P0:7 / P1:10 / P2:6。评估报告：`sih-engine/doc/assessment/2026-09-22-sih-engine-evaluation-report.md`。后续修订追踪：每条 REC 落地时在本文末加 `REC-NNN 落地于 YYYY-MM-DD，载 PR #NNNN + DEC-NNN`；季度评审时回看 REC 完成率与评估总分变化。

2026-09-22 修订一：勘误。四子代理逐条核查十七组出入就地修正：REC-015 重写为收尾批，腿四接线已落地；REC-016 重写为不变式正测形，should_panic 不可达；DEC-009 改号 DEC-039；DES-024/025/026 改号 020/021/022；bin 计数 41/43 改 36；sha256_hex 实数 13；其余行号与体积与口径偏差详见正文与批材料 auditfix-results.md。
