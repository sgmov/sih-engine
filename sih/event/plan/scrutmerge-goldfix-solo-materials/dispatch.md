# scrutmerge-goldfix-solo 执行指令

你在 SiHankor 工作区 `/Users/moc/workspaces/SiHankor` 执行治理批 `scrutmerge-goldfix-solo`，任务包在 `sih-engine/sih/state/plan/scrutmerge-goldfix-solo.md`，先读全文。背景：切换批双跑揭出引擎件与工具件真差异，主会已裁定定性并实证根因即金向量六件全零发现。本批以**工具件为唯一基准**修引擎件，属 TDD 整改类，引擎源码解锁但限 scrutinator 模块。

## 开工前置核

`lease status` 确认 scrutmerge-switch-solo 会话已收约关闭；未关即停批报主会，不代收。核 `sih-tools/scribe/reports/` 有无 `2026-09-01-switch-stop-` 前缀证据件；若无，把 `/tmp/sw-engabs-*.json` 与 `/tmp/sw-tool-abs-*.json` 六件复制为该前缀入 reports（停批证据保全）。

## 主体步骤

1. **脏语料与基准金向量**：在 `sih-engine/src/scrutinator/fixtures/corpus/` 构造含违例目标至少四件，覆盖 C001、C006、S002、S004、S005、S006、F000、F002、F003、F005、N002 十一码（C002 允许表难自然触发可构造触发形）；外加真目标 `doc/decision/020-deyi-component-naming.md`（现成 C006 六处加 U+2026 字符集发现）。每件跑工具件 `cd sih-tools/scrutinator && uv run scrutinator --pack packs/des-001 <绝对路径>` 生成期望输出，**逐字节冻结**入 `src/scrutinator/fixtures/golden/`，含工具件一切格式特征——特别是消息里「字符 U+U+2026」的双前缀写法，那是基准原样，照抄。成色清单（每件 findings 计数与命中规则码）落材料件。
2. **引擎结构对齐**：`src/scrutinator/report.rs` 的 finding 序列化改工具件嵌套形即字段为 `pack`、`rule_id`、`path`、`location: {line}`、`message`，键序与空值形以基准金向量为准。
3. **引擎语义对齐**：修 S 系 rule_id 与消息映射（SPEC-013 上引擎当前误报五处「标题层级跳级」挂 S002/S004 名下，工具件零发现，引擎须归零）；补 C006 判定（DEC-020 六处须逐条出）；字符集消息格式对齐工具件含双前缀。**先跑新金向量测试见红，再修到全绿**，红转绿迹留档。
4. **双跑复验**：SPEC-013、DEC-020、GOV-002 三目标，两侧**同参形**（绝对路径、同包名）各跑，`cmp` 零差且退出码一致，证据入材料件。
5. **SPEC-013 修订四**：金向量须含脏目标条款加双跑同参形条款。
6. **停批档**：核 `sih/event/plan/scrutmerge-switch-solo-results.md` 是否已有切换代理的停报记录；若无，按以下事实代写：切换批双跑揭出三层差异即 GOV-002 路径回显调用形伪差异同参形即零差、finding 结构差即平铺对嵌套、规则语义差即 SPEC-013 误报五处与 DEC-020 漏报六处与消息格式差；根因即金向量六件全零发现；主会 2026-09-01 裁定切换挂起转本整改批。
7. **inputlog 补录四笔**：`sih/event/inputlog/<当日>.ndjson` 追加，会话号 sess-zcode-260901-acceptor，逐字转录：`切换批展开是什么`、`批准通行`、`cmp 部分零差，遇任务包未精确预见的"行为差异非版本戳差异"情况。按红线"上链前必须等绿"与风险点"不硬切"，我必须停下报主会裁定。`、`同意`。

## 机械链

照旧一步不缺：自写 ask3 记录（哲学引文从 sih-philosophy 原文程序切片逐字节子串）→ 双门（工具件 ask3 包加引擎 ask3repeater）→ 新词 elicit check（--words 逐词重复）加 digest → 正身 → `lease open --package scrutmerge-goldfix-solo --repo <两仓> --root /Users/moc/workspaces/SiHankor`（显式带 --session，撞锁即报）→ 逐路径取锁 → meter 包裹引擎 scribe intent 上链 → 一切待提交件先进工地从工地施工 → 管线笔在核前（化格工具件、核阅工具件即切换未成引擎件仍任校验位、检词）→ 认证逐件 meter 包裹 append → 双仓 settle（--cert 链上哈希前八位）→ 放锁 → close（主树碰撞按备份让位归并对表法）→ reconcile 双仓（基线 engine unrouted 4 / tools unrouted 2 加 cert_missing 8，零新增）→ 链 verify valid → 调用册留痕 → 结果档按 F 表写全。本批全部产物含链尾随批入版控。改引擎源码后 settle 前必 `cargo build` 并实跑一次行为探针（BATCH-FACE 坑位）。

## 红线

工具件 scrutinator 的 src、tests、packs 零改动；引擎改动限 src/scrutinator/ 域；金向量冻结后零漂移；U+U+2026 照抄不修；上链前必须等绿；findings 亲读；禁管道掩退出码。

## 完工报告

意图哈希、金向量成色清单、红转绿迹与 cargo test 计数、三目标双跑 cmp 证据、认证清单、双仓 commit 号、链 verify、F 表、越线与误差申报。
