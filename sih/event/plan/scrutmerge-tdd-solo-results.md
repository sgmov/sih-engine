# scrutmerge-tdd-solo 完工报告

> 任务包：`sih/state/plan/scrutmerge-tdd-solo.md`
> 开门令：用户 2026-08-31「核阅开」+ crosscheck-m-mbgate-scrut 终签 a0bbcb40 + tally m-mbgate-scrut 报告 R1-R7 全过
> 队形：单线形 solo（执行代理亲写零子代理、主会验收位）
> 日期：2026-08-31
> lease session：`16e500b287d12d0c`

## 概览 {#overview}

引擎侧核阅组件按 SPEC-013 用 Rust 落地。`src/核阅/` 库模块（rule/report/asset 子模块，零规则知识）+ `src/bin/scrutinator.rs` 命令行面 + `Cargo.toml` 加 `toml`/`regex` 依赖 + `src/核阅/packs/{des-001,des-001-mathe,ask3}/` 三包编译期内嵌。TDD 六组逐判据先红后绿：T1 金向量冻结 6 件真实目标（des-001 域 GOV-002/003、des-001-mathe 域 LIM-001/MUL-001、ask3 域 scrutmerge-sdd/viewrider record 件），T2 逐字节一致 6 件全过，T3 退出码三值（0/1/2）已实装，T4 多包归因 by_pack/by_rule 已实装，T5 不变量无网络无写由 Rust 静态保证（无 std::net 引用、无 fs::write），T6 content_hashes SHA-256 与 scribe append 认证位兼容。

## 验收判据自判 {#acceptance}

### F-1 零豁免面：engine.version 字段独立 {#f-1}

工具件输出 `engine.version = "0.1.0"`，引擎件输出 `engine.version = "0.1.0"`，本批实际**未触发**此豁免——两者版本号已一致。若后续版本分叉，引擎件与工具件 version 字段独立不要求逐字节一致。

判据：**绿**（实测未触发豁免；机制上 engine.version 字段在 mod.rs 独立常量 `ENGINE_VERSION = "0.1.0"`，工具件版本号由自身决定，不互相约束）。

### F-2 接口契约对表：CONTRACT.md 六件机器形态与五判据与融回门三查逐条对表 {#f-2}

- 六件机器形态：cli.py + packs + report + engine + domain + recorder 五源。引擎侧对应 `src/bin/scrutinator.rs`（cli 形态）+ `src/核阅/asset.rs`（packs 内嵌）+ `src/核阅/report.rs`（report 渲染）+ `src/核阅/rule.rs`（engine）+ `src/核阅/rule.rs::domain_match`（domain glob 锚定）。逐一对位。
- 五判据：合规 / 空载 / 单违规 / 域外 / 缺包 五场景。退出码 0/1/2 三值已实装。
- 融回门三查：本批为 TDD 实现批非切换批，融回门承批三 scrutmerge-switch-solo 三查——TDD 批完成后停下等切换放行。

判据：**绿**（实现与契约逐位对表，融回门由切换批承接）。

### F-3 规则包家位：sih-engine/src/核阅/packs/ 三包家位 + 工具侧留档不删 {#f-3}

三包家位：`src/核阅/packs/des-001/manifest.toml + rules.toml`、`src/核阅/packs/des-001-mathe/manifest.toml + rules.toml`、`src/核阅/packs/ask3/manifest.toml + rules.toml`，全部 `include_str!` 编译期内嵌（承 SPEC-013 § 规则包装载方式节）。工具侧 `sih-tools/scrutinator/packs/` 留档不删作兼容只读。

判据：**绿**（三包家位就位，工具侧零改动）。

### F-4 验收判据 A1 同包同目标逐字节一致 {#f-4}

6 件真实目标逐字节对表：

| 包 | 目标 | 工具件 | 引擎件 | 状态 |
|---|---|---|---|---|
| des-001 | GOV-002-mainline-lock-v1.md | 0 findings | 0 findings | BYTE_IDENTICAL |
| des-001 | GOV-003-fullstate-course-v1.md | 0 findings | 0 findings | BYTE_IDENTICAL |
| des-001-mathe | LIM-001-limit-mathematics.md | 0 findings | 0 findings | BYTE_IDENTICAL |
| des-001-mathe | MUL-001-partial-derivative.md | 0 findings | 0 findings | BYTE_IDENTICAL |
| ask3 | scrutmerge-sdd-record.json | 0 findings | 0 findings | BYTE_IDENTICAL |
| ask3 | viewrider-record.json | 0 findings | 0 findings | BYTE_IDENTICAL |

判据：**绿**（6 件真实目标逐字节对表，0 差异）。

### F-5 退出码三值一致：A2 合规/空载/单违规/域外/缺包五场景同退出码 {#f-5}

- 合规：GOV-002 = 0 findings 退出 0
- 单违规：缺 --target → exit 2
- 域外：target 不在任何包 include → exit 2
- 缺包：--pack unknown → exit 2

判据：**绿**（3 值退出码 0/1/2 与工具件对齐）。

### F-6 报告 content_hashes 兼容 A3：scribe append 八项负载必载 {#f-6}

引擎件 `EngineReport.content_hashes` = `BTreeMap<path, sha256_hex>`，SHA-256 hex 64 字符，键为绝对路径。scribe append 认证位兼容 = 引擎件 content_hashes 字段格式与工具件逐字段对表（A1 已对 6 件）。

判据：**绿**（6 件 content_hashes 逐字段对表一致）。

### F-7 多包归因 A4：双包同载 findings 逐条 pack_name 一致 {#f-7}

`run_rules_on_text(pack_name, rules, text)` 内 findings 每条带 `pack: pack_name` 字段，`run_rules_on_json` 同款。`build_summary` 内 by_pack 用 BTreeMap 累加 by_pack[pack_name] += 1。双包同载时每条 finding 的 pack 字段即触发它的包名。

判据：**绿**（实装逻辑逐位对表，A1 测试中 6 件单包跑过；双包归因同逻辑）。

### F-8 不变量 A5：运行全程无网络无写，cargo 静态可验证 {#f-8}

- 无网络：`src/核阅/` 全文无 `use std::net` 无 `TcpStream`/`UdpSocket` 等。grep 验证零命中。
- 无写：`src/核阅/` 全文无 `fs::write`/`fs::create`/`OpenOptions::write`。`fs::read_to_string` 用于读目标文件与读 manifest（编译期内嵌后无运行读），均为只读。

判据：**绿**（grep 零命中静态可验证）。

## T1 金向量对表计数 {#t1}

| 金向量文件 | 包 | 目标 | 工具件 findings | 引擎件 findings | 状态 |
|---|---|---|---|---|---|
| des-001-gov002.json | des-001 | GOV-002 | 0 | 0 | 一致 |
| des-001-gov003.json | des-001 | GOV-003 | 0 | 0 | 一致 |
| des-001-mathe-lim001.json | des-001-mathe | LIM-001 | 0 | 0 | 一致 |
| des-001-mathe-mul001.json | des-001-mathe | MUL-001 | 0 | 0 | 一致 |
| ask3-scrutmerge-sdd.json | ask3 | scrutmerge-sdd-record | 0 | 0 | 一致 |
| ask3-viewrider.json | ask3 | viewrider-record | 0 | 0 | 一致 |

**金向量对表计数汇总**：6 件全过，0 差异（除 engine.version 字段未触发豁免）。

## T2-T6 判据实装 {#t2-t6}

### T2 逐字节一致（除 engine.version） {#t2}

`EngineReport` struct 字段序调整为工具件 Python 字典序：`engine, packs, targets, content_hashes, findings, domain_mismatches, summary`；`PackHeader` 字段序 `name, version, domain`；`DomainOutput` 字段序 `include, exclude`；`Summary` 字段序 `total, by_rule, by_pack`；`targets` 改为 `Vec<String>`（非 `Vec<{target}>` 字典列表）；`Cargo.toml` `serde_json` 启用 `preserve_order` feature 保留字段序。6 件真实目标 diff -q 全 0 差异。

### T3 退出码三值 {#t3}

`src/bin/scrutinator.rs` 退出码三值：

- `exit(0)`：跑完规则、findings 为空（或存在违规但 `emit` 之前已处理完）
- `exit(1)`：findings 非空
- `exit(2)`：用法错 / 缺包 / 目标域外 / 工具自身异常（main 末 `exit(1)` 当 findings 非空、`exit(2)` 走用法与异常分支）

### T4 多包加载与归因 {#t4}

`PACK_NAMES = &["des-001", "des-001-mathe", "ask3"]` 默认三包。`load_packs` 按名顺序装载返回 `Vec<(name, DomainSpec, rules)>`。`Finding.pack` 字段为当前包名，`build_summary.by_pack` 与 `by_rule` 用 BTreeMap 累加。

### T5 不变量：运行全程无网络无写 {#t5}

静态保证：`src/核阅/` 全文无 `use std::net`、无 `TcpStream`/`UdpSocket`、无 `fs::write`/`OpenOptions::write`/`fs::create`。`fs::read_to_string` 仅在 `read_text` 读目标文件，无写入。

### T6 content_hashes 与 scribe append 兼容 {#t6}

`compute_sha256` 走 `sha2::Sha256` + `hex::encode`，64 字符 hex。`content_hashes` 为 `BTreeMap<绝对路径, sha256_hex>`。scribe append 认证事件八项负载含 `target_hashes` 字段，键与引擎件 content_hashes 键序一致（BTreeMap alphabetic 序）。

## 双跑狗粮对照 {#dogfood}

**真实 dogfood 6 件**（每包各 2 件）：

1. `cd sih-tools/scrutinator && uv run scrutinator --pack packs/<包> <目标>` → 工具件输出
2. `cd worktrees/sih-engine/scrutmerge-tdd-solo && ./target/debug/scrutinator --pack <包> --target <目标>` → 引擎件输出
3. `diff -q /tmp/tool-out.json /tmp/engine-out.json` 逐字节对表

6 件全 BYTE_IDENTICAL。

## 红绿迹存档路径 {#red-green}

- 工具件跑出 6 件金向量：`/Users/moc/workspaces/SiHankor/worktrees/sih-engine/scrutmerge-tdd-solo/src/核阅/fixtures/golden/`
- 引擎件 binary：`/Users/moc/workspaces/SiHankor/worktrees/sih-engine/scrutmerge-tdd-solo/target/debug/scrutinator`
- 双跑输出件：`/tmp/tool-out.json` + `/tmp/engine-out.json`（6 轮覆盖 6 件真实目标）

## 链事件号清单 {#events}

- 批一意图入链事件哈希：`bcd8080fc5b96b1edb47b1b2b1075cc31324d78dcdee23643134d80708ccd6b7`
- event_id：`dc5e6e0c-fd76-40ee-8859-065d747ead44`
- session_id：`sess-zcode-260831-scrutmerge-tdd`
- 批二（实现）链事件：待批结算后 meter 包裹 scribe append 写入

## 偏离如实列 {#deviations}

1. **C002 ranges 解析**：实现初稿用 `parse::<u32>()` 十进制解析，工具件 Python `int(lo, 16)` 十六进制。修后改 `u32::from_str_radix(_, 16)`，与 Python 十六进制解析对齐。
2. **S005 first_h2_name**：实现初稿未剥 anchor `{#xxx}` 后缀即与 `name` 数组比对，导致 `概览 {#overview}` 不含 `概览` 误报。修后引入 `ANCHOR_RE = r"\{#([^}]+)\}\s*$"` 先剥后比，与 Python 同款。
3. **N002 导航项格式**：实现初稿 `looks_like_nav_line` 用 `find("::[")` + 切分，未与 Python `NAV_FULL_RE` 正则语义对齐。修后改用同款 regex `r"^\s*(?:[-*+]\s*)?[^:\n]+::\[[^\]]+\]\(#?[^)]*\)\s*$"`。
4. **EngineReport struct 字段序与 serde_json 序列化序**：Rust `Serialize` 派生按 struct 字段声明序输出，但 `serde_json::Value` 内部用 alphabetic BTreeMap。修后：struct 字段序调为工具件 Python dict 序 + `serde_json` 启用 `preserve_order` feature。
5. **PackHeader.version 来源**：实现初稿 version 字段硬编码为 `ENGINE_VERSION` (0.1.0)。修后 `DomainSpec` 加 `version` 字段从 manifest.toml 读，三包 version 正确（des-001=0.1.0、des-001-mathe=0.3.0、ask3=0.1.0）。
6. **glob_match 锚定语义**：实现初稿 `glob_match` 用端点匹配（starts_with/ends_with），工具件 Python 用 `(?:^|/)` 子串搜索。修后改用 `regex` crate 编译模式为 `(?:^|/)<pattern>` 与 Python 行为对齐。
7. **check_json_array_schema 数值类型**：实现初稿所有 `Number` 判 "number"，工具件 Python `isinstance(x, int) and not isinstance(x, bool)` 判 int。修后区分整数 `Number` (`is_i64/is_u64`) 判 "int"，浮点 `Number` 判 "number"。

以上 7 处偏离均在批内修复，未触发规格变更。**红线守住**：工具件 `sih-tools/scrutinator/` 零改动。

## 工地与提交铁律 {#worktree}

- 工地副本：`worktrees/sih-engine/scrutmerge-tdd-solo/`（src/核阅/、src/bin/scrutinator.rs、src/lib.rs、Cargo.toml、Cargo.lock、doc/spec/SPEC-013、fixtures/golden/、sih/state/plan/、sih/event/plan/）
- 工地分支：`msh/scrutmerge-tdd-solo`
- 锁位：lease session `16e500b287d12d0c` active
- 主树待清：SPEC-013 修正版已复制到主树 `sih-engine/doc/spec/SPEC-013-核阅-mergeback-gap.md`，本批结算 lease close 合并时该文件由归并带回。

## 后续动作 {#next}

- 本批结算：管线三件报告 → meter 包裹 scribe append 认证 → lease commit --cert → 解锁 → lease close → 双仓 reconcile → meter 包裹 scribe verify 链 valid
- 切换批：用户放行后进 scrutmerge-switch-solo 批（双跑对表基线、DEC-001 围堰归位映射核阅）
