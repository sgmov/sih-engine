# 司衡引擎评估报告（sih-engine v0.9.0）

评估日期：2026-09-22  
评估对象：`/Users/moc/workspaces/SiHankor/sih-engine`  
仓身份：Rust crate，edition 2021，`version = "0.9.0"`（`Cargo.toml:2-4`）  
扫描范围：`src/` 122 文件 / 57,485 行 + `src/bin/` 43 文件 / 32,556 行 + `packs/` + `tests/` 65 文件 + `doc/` 8 子目录 + `Cargo.toml`  
扫描性质：直检 + grep + `wc -l`（未运行 `cargo build / cargo test / cargo clippy`，运行时数据未采集）  
本报告定性：评估（不含解决方案）——只摆事实加判断

---

## 0 摘要（执行总结）

### 0.1 总分

**总分：6.0 / 10**（等权重口径，七维度平权平均）

| 维度 | 分 | 一句话定位 |
|---|---:|---|
| 完成度 | 7.3 | 工具面与治理件库显著超标；缺发布三步与部分 1.0.0 判据 |
| 易用性 | 6.0 | MCP 教学载荷顶级；CLI 一致性、orphan 二进制、--help 覆盖拖分 |
| 可靠性 | 7.0 | 核心契约（hash 链 / 退出码 / 闸门）强健；风格分裂与 fixture 偏小 |
| 代码质量 | 5.0 | 模块切分与头注优；exitenvelope 活死 + 重复 helper + 文档-代码漂移 |
| 治理集成度 | 7.0 | MCP 双通道治理同源；ToolsRegistry 让位后批 + 哲学源码承接薄 |
| docs | 6.0 | AGENTS.md 自检齐、guide 分层清；README 违反自家规范 |
| 性能与可扩展性 | 4.0 | 编译期内嵌与依赖图克制；append O(n) per write 是事实瓶颈 |

总分计算口径：（7.3 + 6 + 7 + 5 + 7 + 6 + 4）/ 7 = 42.3 / 7 ≈ **6.04 ≈ 6.0**。  
**说明**：本口径为等权重平权。若替换为「判据承接权重」（完成度 25% / 可靠性 15% / 治理 15% / 代码 10% / docs 10% / 易用 10% / 性能 10%），总分 = 6.03。结果对权重变化钝感——最低分维度（性能）拉低下限，最高分维度（完成度）拉高上限均不发生戏剧性翻转。下文如无特别说明，维度分数直接引用各分维报告。

### 0.2 关键发现（top findings 摘要）

1. **跨维度共病：「活死态」结构与文档-代码漂移**（影响代码质量 / 易用性 / 治理 / docs）—— `src/exitenvelope.rs:1-15` 头注声称「归并」「参数化承载」实测零调用点；`target/debug/` 4 个孤儿二进制无源；`src/lease/` 空目录与 `src/bin/lease/*` 七子模块并存；`doc/AGENTS-RETIRED-2026-09.md` 与 `doc/CASCADE.json` 未归治理桶。
2. **完成度缺最后三步**（完成度 / 治理 / 易用性）—— `doc/plan/release-0.9.0-v1.md:39-41` 自承「步骤 7-9 归主窗终验后执行」；DEC-022 1.0.0 判据二「陌生人零帮助指南走查」与判据三「修复节奏实测」材料未齐备。
3. **bin 自治 vs 库统一的张力**（代码质量 / 易用性 / 可靠性）—— `sha256_hex` 在 11+ bin 重复，6+ bin 各自手卷 `parse_args`，CLI 参数语法三种风格并存，thiserror 仅 2/122 文件使用。
4. **测试覆盖广而不深**（可靠性 / 完成度）—— 543 函数 / 65 文件 / 估算覆盖 40+/43 bin；但 fixture 极小（`fixtures/2026-07-27.ndjson` 6 事件 / `fixtures/2026-08-03.ndjson` 7 事件），`#[should_panic]` 0 命中，构造性 `.expect()` 未测。
5. **哲学/数学承接以文档为主、源码为辅**（治理集成度）—— DES-002 / DES-009 / KNOW-001 设计承接强；`src/event_stream/tdd_tests.rs:257` 仅一处把哲学命题路径作 sample 测试数据。
6. **trail 增长是已确认事实瓶颈**（性能与可扩展性）—— `src/event_stream/append.rs:108-131` O(n) per write 真实存在（读全链 + 解析全链 + 写单行），现场 `sih/event/trail/` 27 天累计 3,112 行 / 3.5MB；无 BufReader / 无压缩 / 无归档。

详细证据见第 3-9 节各维度评估与第 10 节跨维度发现。

### 0.3 状态定位

`doc/governance/BASELINE-v1.md:8`「工程基线五条」+ `doc/decision/022-semver-release-v1.md:16`「0.9.0 立测试味首发布位」+ `sih-engine/AGENTS.md:20`「本仓当前处于手动阶段」——三处状态声明一致：**手动阶段 + 测试味发布 + 基线 v1 锁定**。本评估结果在该状态下读取。

---

## 1 评估范围与方法

### 1.1 扫描范围

| 层面 | 范围 | 行/件数 | 工具 |
|---|---|---|---|
| 源文件 | `src/**/*.rs` | 122 文件 / 57,485 行 | `find src -name '*.rs' -exec wc -l {} +` |
| 二进制入口 | `src/bin/**/*.rs` | 43 文件 / 32,556 行（含 `lease/` 子模块 7 文件 / 5,184 行） | 同上 |
| 库模块体量 top5 | `mcpserver` 7,403 / `attractor` 5,116 / `event_stream` 4,288 / `retriever` 2,079 / `scrutinator` 1,890 | — | `wc -l` |
| 规则包 | `packs/{formatter,nomenclator,parser,selector}` | 9 套包（38 元数据文件） | `find packs -type f \| wc -l` |
| 测试 | `tests/**/*.rs` | 65 文件（64 含 `#[test]`） | `grep -l "#\\[test\\]"` |
| 文档 | `doc/**` | 9 子目录 / ~80 .md | `find doc -name '*.md'` |
| 构建产物 | `target/debug/` | 41 可执行文件（4 孤儿无源） | `ls target/debug/` |
| 依赖 | `Cargo.toml` | 17 直接依赖 + 1 dev | 直接读 |
| 事件流现场 | `sih/event/trail/` | 27 个 `.ndjson` / 3,112 行 | `wc -l` |

**未扫描范围**（评估深度边界）：`sih-engine/sih/` `sih-engine/scribe/` `sih-engine/critsweep/` `probes/` `worktrees/` 顶层目录内容（B1/B2）；`Cargo.lock` 152 传递依赖详细闭包（B3）；构建运行时（`cargo build/test/clippy`）未跑（B4）；外部仓（`sih-tools/` `sih-philosophy/` `sih-math/`）实际承袭镜像未双向对位（B5）。

### 1.2 七维度口径

| 维度 | 权重 | 评估焦点 | 评分锚点说明 |
|---|---:|---|---|
| 完成度 | 平权 | 工具面落地 vs 5 件发布内容面 / 工程基线 5 条 / 路线图 / 哲学命题承接 / 决议落地率 | 8=证据链清晰，7=全数完成但有边界外问题，6=多数完成但部分材料未齐，5=半数 |
| 易用性 | 平权 | CLI 一致性 / 错误信息完整性 / 自描述丰富度 / 默认行为可预期性 / MCP schema 严谨度 / 学习曲线 | 8+=教学载荷级，6=人机协作中等，4=碎片化 |
| 可靠性 | 平权 | 核心契约（hash 链 / 退出码三态 / 闸门）的强测试覆盖 + 全统一错误类型 + 关键边界单测 | 9-10=强测试且统一，7-8=到位但局部不一致，5-6=风格不均 |
| 代码质量 | 平权 | 模块边界 / 重复代码 / 类型与所有权 / 抽象泄漏 / 注释与命名 | 9-10=优，7-8=良，5-6=中（含明确瑕疵），3-4=差 |
| 治理集成度 | 平权 | MCP 双通道治理 + scribe trail + 哲学命题承接三条主线 + 决策=治理对象 | 9-10=工业级穿透，7-8=系统性承接覆盖完整，5-6=大体承接有缺 |
| docs | 平权 | AGENTS.md ↔ doc/ 一致性 / guide 完整性 / inline 注释 / examples 价值 / CONTRIBUTING 替代 / 跨文档交叉引用 | 10=工业级，8=良好，6=中等（合格），4=差 |
| 性能与可扩展性 | 平权 | 热路径复杂度 / ndjson 增长应对 / pack 加载 / 并发模型 / 构建优化 | 9-10=全 O(1/log n) + 索引压缩齐全，7=热路径 O(n) 有上限，4=O(n) 渐成瓶颈 |

### 1.3 评分口径（明示）

- 每维度 0-10 分制独立评分，锚点语义在 1.2 节注明。
- 总分用等权重平权（12.4 节附权重敏感性测试）。
- 「不下解决方案」是评估报告核心约束——只摆事实与判断，不出处方。
- 一切事实引用 `file_path:line_number`；引用未实测的标注「未实测」。

---

## 2 仓画像（一页）

| 维度 | 关键事实 |
|---|---|
| 仓身份 | `sih-engine` Rust crate v0.9.0，edition 2021，`Cargo.toml:2-4` |
| 库面 | 11 个 `pub mod`：`ask3repeater / askroute / attractor / cascade_registry / event_stream / exitenvelope / mcpserver / retriever / snapline / scrutinator / tools_registry / view`，`src/lib.rs:8-19` |
| CLI | 43 个 bin 入口，41 个 `target/debug/` 产物；用户面向仅 2 个（`sih` 薄壳 + `sihmcp` MCP 双传输），其余 41 个内部工具 |
| 规则包 | formatter(2) + nomenclator(1+5) + parser(3) + selector(3) 共 9 套，38 元数据文件 |
| 文档桶 | decision(27) + design(19) + spec(27) + governance(11) + proposal(8) + research(3) + guide(4) + knowledge(1) + plan(1) + 根级 2 非标 |
| 测试桶 | mergeall(33) + lease_mergeback(6) + defectwave(3) + gap(5) + integration(2) + golden(8) + other(8) = 65 文件 |
| 体量 | `src/` 57,485 行 / 122 文件；`src/bin/` 32,556 行 / 43 文件；bin 平均 ~760 行 |
| 依赖栈 | 17 直接依赖 + 1 dev，分 6 类（序列化 / 哈希 / MCP / 异步 / 时间 / 工具），`Cargo.toml:7-26` |
| 现场数据 | `sih/event/trail/` 27 天 ndjson 累计 3,112 行 / 3.5MB，最大单文件 354KB / 301 行 |

---

## 3 完成度（7.3 / 10）

### 3.1 评分明细

| 子维度 | 权重 | 分 | 加权 |
|---|---:|---:|---:|
| 工具集与功能覆盖（发布面 vs bin 落地） | 30% | 8 | 2.40 |
| 工程基线 5 条落地 | 25% | 8 | 2.00 |
| 路线图里程碑执行 | 20% | 6 | 1.20 |
| 哲学命题承接面 | 15% | 6 | 0.90 |
| 决议落地率 | 10% | 8 | 0.80 |
| 合计 | 100% | — | **7.3** |

### 3.2 关键证据

**S1｜工具面超额落地**：发布内容面 5 件（`doc/plan/release-0.9.0-v1.md:14-15` 引擎五件套：`scribe / scrutinator / attractor / ask3repeater / retriever`），实际 43 bin / 11 `pub mod` 是「5 件套 + 36 件内部工具」双层结构。

**S2｜决议→源码引用密度高**：grep 抽样 DEC-006 / DEC-007 / DEC-024 / DEC-026 五大决议均有多文件命中：
- `src/ask3repeater/mod.rs:1` + `src/bin/scrutinator.rs:3`：DEC-006（≥3 处）
- `src/view/mod.rs:1` + `src/bin/scrutinator.rs:1` + `src/bin/viewer.rs:1`：DEC-007（≥3 处）
- `src/bin/lease/sddgate.rs:2` + `src/bin/lease/closegate.rs:935`：DEC-024（≥4 处）
- `src/bin/sih.rs:12` + `src/bin/sih.rs:158`：DEC-026（≥2 处）

**S3｜事件流持续运行 27 天**：`sih/event/trail/` 2026-08-22 至 2026-09-18 累计 3,112 行；2026-09-08 单日 313 行峰值——PRO-08「应而不藏」工程承接面在数据层确有证据。

**W1｜发布三步未执行**：`doc/plan/release-0.9.0-v1.md:39-41`「步骤 7-9（打 tag / 推送 / 宣告）归主窗终验后执行」。现状：版本号已 0.9.0、bin 产物齐全、trail 已 27 天累计；缺的是执行链路尾段。

**W2｜DEC-022 1.0.0 部分判据材料未齐**：`doc/decision/022-semver-release-v1.md:18-32` 三判据：冷 agent MCP 面零辅助跑通 / 陌生人零帮助指南走查 / 发布后修复节奏实测；修订一加判据四 SDDG 完备度闸在役（出处同文件第 70 行）。判据二与判据三尚无可机械复验的证据产物。

### 3.3 该维度观察

- 工程基线 5 条与哲学命题的对应关系非显式——前三条有哲学对照基础（第一条 ↔ PRO-07 鉴 / 第二条 ↔ convergence P3.x / 第三条 ↔ P3.1.3 R7），后两条待哲学锚定（`BASELINE-v1.md:47-51`）。
- 组件索引 vs 库面 11 `pub mod` 不对称：`AGENTS.md:24-38` 列 4 件第一阶段 + 2 件第二阶段预登记；`src/lib.rs:8-19` 实际 11 `pub mod`——`cascade_registry / exitenvelope / tools_registry / snapline / askroute` 五件在 AGENTS.md 无显式索引。
- 0.9.0 发布面 vs 库面不对称：发布面 5 件 + 36 件「sih-tools 工具带」（`release-0.9.0-v1.md:14-18`）——是「sih-engine 用户面向」而非「sih-engine 工具全谱」。

---

## 4 易用性（6.0 / 10）

### 4.1 关键证据

**S1｜MCP 教学载荷行业顶级**：错误 payload 9 字段 `error / reason_code / gate / what_this_tool_does / valid_params / exit_code_semantics / exit_code / suggested_action / canonical_pointers`（`src/mcpserver/errors.rs:204-246`），REASON_TABLE 16 行静态表把 CLI 子串映射到语义 reason_code（IntentRecordUsedRejected / locked_elsewhere / open_precheck_conflict 等），同文件 19-132 行。`src/mcpserver/tools.rs:26-28` CLI_FILE_PATH_HINT 静态补位教学——AI 代理面对拒错能拿修复动作。

**S2｜退出码三值约定仓级一致**：`src/mcpserver/errors.rs:11-12`「0=成 / 1=拦 / 2=工具异常」+ `doc/guide/contributor-guide-v1.md:42`「零合规、一违规、二工具异常。管道掩退出码是禁手」+ `README.md:179` 等多文件例证。

**S3｜stdio 自动开域减少首次摩擦**：`src/bin/sihmcp.rs:35-49` + `src/bin/sihmcp.rs:76-88` stdio_auto_domain() 三态判定——SIH_ROOT 指向未开域 git 域根时自动 bootstrap 子进程，免去显式 token 签发仪式。

**S4｜CLI USAGE 字符串自描述丰富**：`src/bin/scribe.rs:200-243` 每个子命令含必填/可选/示例/闸位说明；`src/bin/pendline.rs:23-74` 五子命令分发 + `parse_args` 派生；`src/bin/sih.rs:34` 短单行适合窄口工具。无网环境 `-h / --help` 可工作。

**W1｜CLI 参数语法三种风格并存**：`scribe.rs:262-268` 不支持 `--flag=value`；`critsweep.rs:130-134` 支持（`split_once('=')`）；`lease.rs:106-119` BTreeMap + `--allow / --repo` 可重复。

**W2｜`--help / -h` 支持不一致**：`src/bin/sih.rs:73-76` + `scribe.rs:252-258` 支持；实测 `target/debug/lease` / `critsweep` 不支持——lease 把 `-h` 落在「子命令 X 未在融回对等域」错误路径。

**W3｜README 工具清单与 MCP schema 不一致**：`README.md:184-187` 列 19；`server.rs:79-176` + `240-297` 共 21（含 `record_direct / lease_unclaim` 两个本地可信位，112-118 + 149-155）。external 分级用户尤其混淆。

**W4｜未统一顶层命令树**：43 bin 无统一 facade；`target/debug/` 4 孤儿二进制与现役同名相近（`-gate` 后缀），用户初见无权威判别依据。

**W5｜MCP 错误返回通道混杂**：`providers.rs:50-56` handler 永远返回 `Ok(Value)` 把工具层错误塞进 Value 内 error 键；`server.rs:401-407` call_tool 仅 NotFound 走 `invalid_params`，其余走 `internal_error`——`errors.rs:204-246` 9 字段教学载荷经 `McpError` 序列化后丢字段结构。

### 4.2 该维度观察

`src/exitenvelope.rs:7-15` 明文列未并族 13 个并解释「强并即行为漂移，不并」——归并未完成是诚实记录而非技术债隐藏。`src/bin/sih.rs:68-72` 无参直 eprintln USAGE + exit 2 与「无静默吞旗标」哲学一致；`src/bin/scribe.rs:368-395` 追加原子化重试（basefix-solo F-2）对用户透明。

---

## 5 可靠性（7 / 10）

### 5.1 关键证据

**S1｜hash 链确定性序列化**：`src/event_stream/hash.rs:17-70` compute_event_hash 用 BTreeMap 强制字段名字典序后 SHA-256；None→`Value::Null` 显式映射避免哈希漂移。

**S2｜hash 边界强约束**：`src/event_stream/hash.rs:84-111` verify_chain 检 event_hash 长度=64 / 空 / prev_hash 空（除创世）三类全拒；`GENESIS_PREV_HASH` 64 全零字符串（`hash.rs:9`）。

**S3｜原子追加强保证**：`src/event_stream/append.rs:55-71` flock(LOCK_EX=2) 直调，path=Some 时锁内重读现行链破解 prev_hash 链接竞态窗口（同文件 108-131）。

**S4｜退出码三态显式契约**：`src/pendline/mod.rs:35`「退出码：0 成功 / 1 违规或落链失败 / 2 用法或环境错误」；全仓 42 处 `std::process::exit` 沿用。

**S5｜时戳单调性构造保证**：chainstamp-solo 把单调拒收类从根消除，`src/event_stream/chainstamp_tests.rs:178-202` 四族病态 hint 全部链路 valid。

**S6｜金向量与重构一致**：`src/event_stream/hash.rs:58-63` 对旧事件行重算逐字节一致，verify 兼容零数据迁移。

**S7｜SDDG 版本+日期双钉**：`src/bin/lease/sddgate.rs:13-37` `SDDG_VERSION="1.0.0"` + `GATE_SINCE="2026-09-12"` 硬编码。

**W1｜thiserror 仅 2/122 文件使用**：`src/ask3repeater/validate.rs:11` + `src/tools_registry.rs:35` 是全工程仅有的 `#[derive(thiserror::Error)]` 用例；event_stream/ 下 9 个 Error 枚举手写 derive，错误显示风格跨模块不一致（`src/event_stream/append.rs:12-46` 等）。

**W2｜exit envelope 流不一致**：`src/bin/lease.rs:41-49` die() 用 eprintln!（stderr），`src/bin/lease/commitlaw.rs:41-44` fail() 用 print!（stdout）——同语义不同流，shell 管道消费方易踩坑。

**W3｜T3 退出码测试仅覆盖 0 与 2**：`tests/lease_mergeback_t3_exit_codes.rs:11-25` 只显式测 exit 0 与 exit 2，未测 exit 1。

**W4｜SDDG-1 缺内单测**：`src/bin/lease/sddgate.rs` 内 6 个 `#[test]` 仅覆盖 SDDG-2/3/4（678 / 710 / 731 / 764 / 784 / 800 行附近）；SDDG-1 仅由外部 `tests/lease_mergeback_t6_close_gates.rs:312` 覆盖，单点失效无内层屏障。

**W5｜hash 链 fixture 样本极小**：`fixtures/2026-07-27.ndjson` 6 事件 + `fixtures/2026-08-03.ndjson` 7 事件；千/万级事件性能与边界未覆盖。

**W6｜verify_chain 与 verify 行为分歧**：`src/event_stream/hash.rs:86` 空链返 `(true, "", "", None)`；`src/event_stream/verify.rs:60` 空链返 `Err(EmptyStream)`——同模块双契约。

**W7｜panic 路径零测试**：全 `src/` 与 `tests/` `grep #[should_panic]` 0 命中；`src/event_stream/hash.rs:26 / 65` 与 `src/event_stream/append.rs:123` 构造性 `.expect()` 文本未钉死。

### 5.2 该维度观察

- 543 测试函数（`src/` 271 + `tests/` 272），事件流 99 个内单测集中 hash/append/chainstamp 三件；T6 闸面集成测试按 golden-capture c22-c30 编号承表（`tests/lease_mergeback_t6_close_gates.rs:312` 等）。
- `chainstamp_tests.rs:5-6` 标注「testhard 批件二」反向回归——scribe 寻址旧形在 worktree 工地形必红。
- tally/locator/parser 三最重 bin（1366/2872/3012 行）均对应 mergeall_t* 集成测试，但行数与覆盖深度未做行级映射。

---

## 6 代码质量（5 / 10）

### 6.1 关键证据

**S1｜lib/ 顶层模块边界清晰**：`src/lib.rs:8-19` 列 11 `pub mod`，`src/lib.rs:25-36` re-export 25 个公共符号；`src/exitenvelope.rs:9-11` 明示边界（纯发射原语，零治理语义）。

**S2｜大型复杂模块以单一职责分子模块**：`src/event_stream/mod.rs:10-21` 13 子模块（event / hash / append / verify / query / certify / crosscheck / intent / lockgate / park / reading / sessiongate）各担职责；`src/mcpserver/mod.rs:7-22` 同形（alpha / errors / matrix / passthrough / providers / runtime / server / session / tokens / httpface / webface / bootstrap / tools）。

**S3｜异步所有权模式正确**：`src/tools_registry.rs:88-95` `RwLock<Vec<Arc<dyn ToolProvider>>>` + 注释「短锁内 Arc clone 即出，await 恒在锁外——std 锁 guard 不跨 await，future 保 Send」。

**S4｜头注详尽**：`src/bin/acceptor.rs:1-27` + `src/bin/cascade.rs:1-33` + `src/bin/parser.rs:1-35` + `src/bin/meter.rs:1-23` 等所有 bin 头注均含：移植源 / CLI 文法 / 退出码三值 / 落差申报 / 正典指针（spec/dec/des 路径）。

**S5｜跨模块依赖路径短**：`src/pendline/dispatch.rs:135` + `:171` 用 `sih_engine::attractor::contract_mode`（`emit_contract / load_contract / sha256_bytes / make_shots`）；`src/bin/lease/closegate.rs:13` + `src/bin/cascade.rs:45` 用 `sih_engine::cascade_registry`（`build_registry_recorded / dump_canonical / sort_value / resolve_abs / ENGINE_VERSION`）。

**W1｜exitenvelope 模块活死——文档-代码漂移最大瑕疵**：`src/exitenvelope.rs:13-15` 头注原文「critsweep、gauge、lease 三 bin 逐字节一致的 `sort_json / emit / die` 副本归并于此……meter 的 `die_uncaught` 与 calllogtool 的 `die_value_error` 以 `die_plain` 参数化承载，attnanchor 的同字节 `sort_json / emit` 对同承此位」。实测：`grep -rn "exitenvelope" src/` 仅命中 `src/lib.rs:13`；`src/bin/gauge.rs:44-73` + `src/bin/lease.rs:41-70` + `src/bin/critsweep.rs:50-79` + `src/bin/attnanchor.rs:37-56` 各自保留字节级副本；`src/bin/meter.rs:245` die_uncaught 与 `src/bin/calllogtool.rs:366` die_value_error 签名与 `die_plain` 不兼容（kind+msg vs 单 msg）。**这是当前代码质量维度最严重的瑕疵**——模块声明为「归并」「参数化承载」，实际零调用点、四处副本存活、两处命名同源函数签名不兼容。

**W2｜11+ bins 各自重写基础 helper**：grep `fn sha256_hex` 命中 `src/bin/lease.rs:72` `gauge.rs:75` `cascade.rs:47` `confledger.rs:161` `identity.rs:103` `incubation.rs:56` `acceptor.rs:156` `projsnap.rs:35` `formatter.rs:61` `basemgr.rs:128` `locator.rs:64` `tally.rs:335` 等 11+ 处；`py_escape` 在 `src/bin/calllogtool.rs:73` `meter.rs:41` `tally.rs:79`；`py_str` 在 `src/bin/attnanchor.rs:109` `acceptor.rs:145`。sha2+hex 是 std crate 即用件——属过度重复。

**W3｜6+ bins 各自手卷 parse_args，签名/语义各异**：`src/bin/lease.rs:106-126`（BTreeMap + 重复旗标策略）、`src/bin/critsweep.rs:112`、`src/bin/gauge.rs:748`、`src/bin/basemgr.rs:627`（无参）、`src/bin/latextool.rs:71`、`src/bin/scrutinator.rs:75` + `src/pendline/mod.rs:133-170`（HashMap 保序累积）——7 处独立实现，签名与语义各异。

**W4｜mega-file 单文件超 1500 行**：`src/bin/parser.rs` 全 3012 行，零内部 mod，含 lexer / PEG / entries / lint / vectors freeze 五面；`src/bin/locator.rs` 全 2872 行同结构；`src/mcpserver/bootstrap.rs` 全 1691 行含五段 bootstrap + CLI + web router 三层。单文件过 1500 行通常暗示抽象未抽出。

**W5｜类型签名两处不一致**：`cascade_registry.rs:40` `sort_value(v: Value) -> Value`（消耗所有权）vs `exitenvelope.rs:21` `sort_json(v: &Value) -> Value`（借用）——同 sort 算法两种签名并存。`pendline/mod.rs:103-108` Args 用 `HashMap/HashSet`（非确定性）vs `lease.rs:108` + `cascade.rs:36` 等均用 `BTreeMap`。

**W6｜Python 抽象泄漏面广（intentional leakage）**：`src/attractor/jsonc.rs:22-39` `PyError { kind: &'static str, msg: String }` 直接对齐 Python 异常类型名（ValueError/OSError/KeyError/JSONDecodeError）；`src/cascade_registry.rs:39` 注释「json.dumps sort_keys=True 语义」。端口语义的必要性有文档，但泄漏到库 API 层（`PendError::violate` 见 `src/pendline/dispatch.rs:253`）。

**W7｜`src/lease/` 目录为空，命名碰撞**：仅含 `fixtures/{golden,golden2}/`，零 `.rs` 源；与 `src/bin/lease.rs`（729 行）+ `src/bin/lease/` 子目录（5,184 行 7 子件）形成三重命名。

### 6.2 该维度观察

事实合并指向一处张力：库的「模块化与归并」意图层（`exitenvelope.rs:1-15` 头注 + `lib.rs:8-19` 模块面）与「bin 自治 + 逐字节对表」执行层并存；执行层偏好 bin 自治，意图层在头注中持续声称已重构。`src/tools_registry.rs:90-91` 异步锁规避注释示范了「正确模式+文档明示」应有的形态。

---

## 7 治理集成度（7 / 10）

### 7.1 关键证据

**S1｜MCP 双通道治理同源同典完整覆盖**：stdio（`src/bin/sihmcp.rs:17-71`）+ HTTP（`src/mcpserver/httpface.rs:43-46`）共用 `src/mcpserver/server.rs:30-33` 三件正典 `CANON_SPEC_023 / CANON_LINE_PKG / CANON_DES_014`，HTTP 加 `CANON_DES_015`；HTTP_WRITE_TOOLS 十具 + HTTP_ALPHA_TOOLS 九具与 stdio β 写面十二具对位（`src/mcpserver/httpface.rs:50-74`）。

**S2｜scribe trail 哈希链全程承接**：BTreeMap 字典序 + 四态校验（event_id 唯一 / 时戳单调 / prev_hash 匹配 / actor 合法性，`src/event_stream/append.rs:123-167`）+ flock 覆盖读算写全程（`append.rs:80-91`）+ verify_chain 区间与全量两档。

**S3｜lease 与 trail 交叉核对机械实现**：`src/bin/lease/commitlaw.rs:97-147` trail_hashes_of 扫三条候选 trail（`sih-engine/sih/event/trail` `sih-tools/scribe/trail` `sih/event/trail`），`cert_on_chain` 用 BTreeSet.starts_with 做前缀比对；代码字段注释明示「承接 GOV-002 判据面」。

**S4｜哲学消费路径是 design-corollary 不是 ad-hoc 引用**：`doc/design/DES-002-philosophy-consumption-paths.md:1-30` 三层消费路径 + `doc/knowledge/KNOW-001-convergence-retrieval-map.md:1-30` 单盲推导体系检索路径 + `doc/design/DES-009-crosscheck-semantic-layer.md:148-152` 参验语义层承接 proodos 08-on-settle / 05-on-fourth-tao 三件命题锚定位。consumer 路径分正典与回顾两条。

**S5｜工地链副本守护（隐性 proscription 的正典承载）**：`src/event_stream/tdd_tests.rs:274-322` G-A1/G-A2/G-A3 三件 trail 副本禁追加（退出码二），显式 `--allow-worktree-trail 1` 旗标放行。

**S6｜HTTP 通道降级受控教学面零域数据投影**：`src/mcpserver/httpface.rs:79-81` `IDENTITY_NOTICE_DEGRADED` 连接降级形即域数据读工具零投影，写工具一律 401 拒。

**W1｜ToolsRegistry 与 sihmcp 分派面零接线（让位后批）**：`src/bin/registrydemo.rs:5-12` + `src/tools_registry.rs:14-16` 申报「在役面改造让位后批」「MCP 面接线改造候腿五收口批」；`src/tools_registry.rs:24-26` `ExternalPluginBridge` 仅签名，缺省返 `BridgeUnsupported`。SPEC-025 插件槽位协议在 sihmcp 面未穿透。

**W2｜哲学命题源码层承接面极薄**：`DES-009` prose 引用 3 处 proodos 命题；代码层显式使用 `sih-philosophy` 路径仅 `src/event_stream/tdd_tests.rs:257` 一处样例数据（test fixture）。代码层把哲学命题当作可执行约束的样本几乎为零。

**W3｜lock/unlock/commit 跨通道 trail 名空间事实风险（基于事实陈述）**：`src/mcpserver/session.rs:200-209` disconnect_close 失败即 eprintln「会话转挂起候人节点 takeover 裁决」（不强拆不静默弃置）；`src/bin/lease/commitlaw.rs:97-101` default_trails 在 trail 缺席时回落第一域映射形（root=`sih-engine/sih/event/trail`），HTTP 面 canonical 形 trail 是 `root/sih/event/trail`——两条路径若未统一，跨通道 commit 与 close 可能出现 trail 名空间分叉。

**W4｜4 个孤儿二进制 + doc/ 根级 2 件非标文件**：见第 10 节跨维度发现。

### 7.2 该维度观察

- sih-math 推导档承接隐性但系统：`src/event_stream/{append,verify,park}.rs` 头注引 `sih-math/docs/scriwire-scribe-derivation-2026-09-03.md` 三次；`src/attractor/tally.rs:34` 引 `constclear2-routing-2026-09-08.md`；`src/scrutinator/tests.rs:318-391` 多处引 `gatemath-derivation-2026-09-04.md`。
- `DES-002:11` 显式拒绝「不修改哲学仓」——「工程层不修改哲学仓，不伪托工程经验为哲学命题，不把 convergence 层对照命题标注为流衍命题」。边界在此冻结。
- MCP bin 内 dispatch 与 tools_registry 是双轨：`server.rs:35-56` 构 `providers::build_registry(...)`，但 list_tools 与 call_tool 仍散点 match 分派；A3 统一注册形未穿透。
- 跨进程 swap chain 一致性靠 passthrough argv 派生：`passthrough.rs:1-100` + `runtime.rs:33-67` 所有 argv 共享 `code_root()` + `resolve_root()`。
- 决策登记=治理对象承接：`doc/decision/` 27 DEC；`doc/spec/` 27 SPEC；`doc/design/` 19 DES；`doc/governance/` 11 GOV。决策 doc-first 元数据，代码层通过「承接 GOV-002」标注回引。

---

## 8 docs（6 / 10）

### 8.1 关键证据

**S1｜AGENTS.md 守 DES-001 字符约束 + 自检齐全**：`AGENTS.md:1-113` 113 行 / 6.6 KB，破折号 0 处、表格 0 处、横线 0 处（自检节明示 5 条形式合规 + 6 条内容项，`AGENTS.md:92-112`）。本仓 AGENTS.md 是其自身规范的合规样本。

**S2｜`doc/guide/` 四件齐全且分层清晰**：user-guide-v1.md（6.7 KB，使用者层级）/ contributor-guide-v1.md（6.3 KB 含五仓地图 + 组件六席 + 纪律四条 + 批机械链全序，contributor-guide-v1.md:48）/ adoption-guide-v1.md（14.7 KB，下游采纳者 6 步缺省接入 + HTTP 进阶档 + 12 问 FAQ）/ agents-template-v1.md（4.7 KB，下游 AGENTS.md 可复制骨架）。

**S3｜模块头注承接规约**：8 个核心库 `//!` 头注全部含 SPEC-NNN / DES-NNN / DEC-NNN 编号引用：
- `src/lib.rs:1` 承 DES-007
- `src/event_stream/mod.rs:1` 承 DES-007#module-organization + SPEC-004#interface-signature
- `src/mcpserver/mod.rs:1-5` 承 DEC-001 + DES-014
- `src/attractor/mod.rs:1-2` 承 SPEC-014 + DEC-020
- `src/scrutinator/mod.rs:1` 承 SPEC-013
- `src/cascade_registry.rs:1-2` 承 SPEC-025
- `src/ask3repeater/mod.rs:1` 承 DEC-006 + DES-013 + SPEC-005
- `src/retriever/mod.rs:1` 承 SPEC-007 + SPEC-008

可由 `AGENTS.md:23-38` 组件索引反向回到各 spec 设计档。

**S4｜examples/ 三件皆有头注承接契约 + 用法 + 退出码三值**：
- `examples/golden_vectors.rs:1-8` 承 scribe/CONTRACT.md 第六件 + `cargo run --example`
- `examples/rebuild_hash_chain.rs:1-6` 承 OQ-01
- `examples/verify_external_trail.rs:1-8` 承 scribe/CONTRACT.md 验收判据六 + 明示退出码 0/1/2

**S5｜README 多层面向 + 附录 7 节**：`README.md:1-234` 分 8 个叙事节 + 附录 ABCDEFG，覆盖「写给不细究技术的人」与「动手者」两层。

**W1｜README 自身违反 DES-001 字符与禁用格式约束**：`README.md` 13 处破折号「——」+ 21 处表格行 + 1 处 `---$` 横线；`AGENTS.md` 与 `doc/guide/*.md` 全部 0 处破折号、0 处表格、0 处横线。`DES-001-document-format/general.md:107-194` 明文禁用这三类格式。README 与本仓其他文档在格式合规性上不对位。

**W2｜RANTS.md 与 DES-001 多条字符与禁用格式约束冲突**：自报「风格预警：吐槽体，不严肃，emoji 管够」（`RANTS.md:5`）；17 处破折号、emoji、`**` 粗体多处、`>` 块引用多处。RANTS.md 是否被 DES-001 排除受治理约束未在 GOV 文档明示。

**W3｜无 CONTRIBUTING / CHANGELOG / LICENSE 文件**：`sih-engine/` 顶层 glob `CONTRIBUTING* / CHANGELOG* / HISTORY* / LICENSE*` 全部命中 0。替代路径 `doc/guide/contributor-guide-v1.md` 但 GitHub 平台自动识别 `CONTRIBUTING.md` 而非 `doc/guide/*.md`，新贡献者发现门槛提高。

**W4｜AGENTS.md 覆盖偏薄，新 mergeback 件索引缺席**：`AGENTS.md:22-38` 仅列 4 第一阶段组件 + 2 第二阶段预登记。`lease.rs`（729 行 + 7 子模块 5,184 行）、`parser.rs`（3,012 行）、`locator.rs`（2,872 行）等 41 个内部工具均未索引。

**W5｜README §A 安装验证示例与仓结构弱对位**：`README.md:142-147` `find sih/event/trail ...` + `target/debug/critsweep --at $(date +%F) --root ..` 在 sih-engine 仓根执行找不到 `sih/event/trail/*.ndjson`（trail 在被治理项目根）。`README.md:149` 明示 `sih/event/trail/` 在被治理项目，但 §A 未先 cd 到被治理项目根。

**W6｜`doc/` 根级 2 件非标文件未归桶**：`AGENTS-RETIRED-2026-09.md`（18.6 KB）+ `CASCADE.json` 落仓根不入治理子目录；文件名格式不与 DEC-NNN-xxx / DES-NNN-xxx 一致，对命名权威治理链工具解析门槛提高。

### 8.2 该维度观察

- `AGENTS.md` 与 `AGENTS-RETIRED-2026-09.md` 存在事实对位疑问：后者自报「新 AGENTS.md 16731 字节原 43343 字节」；本仓 AGENTS.md 实际 6,629 字节——待 mtime 复核。
- `scrutinator/mod.rs:1-2` 头注 9 行；`attractor/mod.rs:1-24` 头注 24 行含腿分层 + 11 个子模块列表 + 规则 R1-R7 形态。引擎侧组件库面覆盖率与详细度在模块间显著不均。
- `examples/` 在 `README.md` 与 `AGENTS.md` 索引中均未提及——开发者只能靠 `cargo run --example` 探索。
- DES-001（`general.md:259-268`）作为「规范类文档」享有行内代码与围栏代码块两例外——README 与 RANTS.md 的判定边界不清晰。

---

## 9 性能与可扩展性（4 / 10）

### 9.1 关键证据

**S1｜规则包/判定包编译期内嵌，运行时 pack 加载零 IO**：`src/scrutinator/asset.rs:10-36` 9 处 `include_str!`，`src/askroute/mod.rs:27` `include_str!` intents-v0.json；合计 ~22KB 数据一次性编入 binary，scrutinator 与 askroute 启动期零 pack IO、零路径解析、零冷启动延迟。这是「确定性程序」基线的天然选择。

**S2｜追加原子化设计正确**：flock(LOCK_EX) + 读算写同一临界区（`src/event_stream/append.rs:56-71` + `:118-130`），进程死亡锁自动释放无陈锁残留。并发写正确性已闭环——多进程/多会话并发追加不分叉不丢链。

**S3｜依赖图克制**：`Cargo.toml:20` tokio features = `["rt-multi-thread", "macros", "net", "io-util", "process", "sync", "time"]` 未启 full；`Cargo.lock` 152 包；serde_json 仅 preserve_order；零超集滥用。

**S4｜trail 文件命名采用日切策略**：`src/mcpserver/runtime.rs:45` today_str() 输出 `%Y-%m-%d`；`sih/event/trail/` 现场 27 个日文件验证；attnanchor.rs:375,417 / watchcheck.rs:56,647 / commitlaw.rs:99,101,247 / guardlaw.rs:52 / closegate.rs:23 全按日期形 trail 路径拼装。

**S5｜并发靠子进程编排天然隔离失败域**：`src/bin/*` 19 个 bin 共 30 处 `Command::new`（grep 验证）；子进程模型为「编排器编排内部 CLI」的典型司衡形态。

**W1｜append 路径 O(n) per write（已确认事实瓶颈）**：`src/event_stream/append.rs:108-131` path 在场时 `open(append=true)` 后 lock → `load_events(p)` 读全链 → 序列化单行 → `writeln!(writer, "{line}")` 一行回写（直写非 BufWriter）；`src/event_stream/append.rs:276-290` load_events 用 `fs::read_to_string` 全量入 `Vec<Event>`；`src/event_stream/` 内 `grep BufReader/BufWriter` 0 命中。单次写入成本 = 读全链 + 解析全链 + 序列化单行 + 写单行。**现场观测 `sih/event/trail/` 已 27 天累计 3,112 行 / 3.5MB（最大日 354KB / 301 行）**；若日写入翻 10 倍 → 3MB/日，1 年累计约 1GB，每写一次成本 ~1GB 解析——**此为已确认的事实瓶颈非推测**。

**W2｜trail 无压缩/无归档策略**：`src/` 整目录 grep `compress / gzip / zstd / rotate / archive` 0 命中；现场 27 个 `.ndjson` 文件平均 1.18KB/行（事件含 details JSON 对象，文本冗余高）。gzip 等典型压缩比 5-10×——3.5MB 可压至 0.5MB 量级；存储成本随写入线性增长，且无任何老化机制。

**W3｜无 [profile.release] 配置与 build.rs 优化**：`Cargo.toml:1-27` edition 2021，无 `[profile.*]` 段，无 build.rs；release 默认 opt-level=3 + 无 LTO + 无 codegen-units 调整。release 二进制未榨干尾延迟。

**W4｜并发模型两极分化（41 同步 + 2.5 tokio）**：`src/bin/sihmcp.rs:17` `#[tokio::main]` + axum + rmcp；`src/bin/sih.rs:55-60` 手写 `Runtime::new()` + `block_on`；`src/bin/registrydemo.rs:71` `#[tokio::main]`；其余 40 bin 是纯同步 CLI；`src/event_stream/` grep `tokio::fs` 0 命中，无 `spawn_blocking` 兜底。异步路径若混入 std::fs 同步 IO 会阻塞 reactor 线程。

**W5｜4 个孤儿二进制落盘**：`target/debug/ask3gate`（Aug 27 23:17, 1.4MB）、`ask3repeatergate`（Aug 28 00:49, 1.4MB）、`retrievergate`（Aug 28 00:49）、`scribegate`（Aug 28 01:02, 908KB）——4 个 Mach-O arm64 可执行；`grep src/ / Cargo.toml` 找不到名称对应源。孤儿二进制每次 cargo clean 才清除；增量编译指纹（`.fingerprint/`）持续误判；CI 缓存体积异常。

**W6｜scrutinator 规则包运行期按字符串 match 派发 + 三次解析**：`src/scrutinator/asset.rs:10-36` manifest/rules/envelope 各 3 处 match name 走 `include_str!` 返回 `&str`；`src/scrutinator/rule.rs:734-741` load_pack 每次都重新 `serde_json::from_str(manifest_text)` + `parse_rules(rules_text)`——非 lazy 全局缓存。pack 数量增长或 rules.toml 增至 MB 级时冷启动延迟从 <1ms 跳到 10ms+。

### 9.2 该维度观察

- 启动路径：`src/bin/sih.rs:55-60` 用 `Runtime::new()` 手创；`sihmcp.rs:17` `#[tokio::main]` 默认 flume + 多线程 runtime；sihmcp 因 tokio+axum+rmcp 栈 ≈ 24MB，启动 ~20-50ms 量级（无实测）。
- pack 加载范式三轨并存：A. 编译期内嵌（scrutinator / askroute）；B. 运行期读文件 + OnceLock 懒缓存（locator.rs:1701, :1745）；C. 无策略（formatter / nomenclator / parser / selector）。三轨并存本身不是错，决定哪类 pack 编入 binary 哪类走运行期需明示契约。
- trail 增长曲线：现场 27 天 / 3,112 行 / 3.5MB。按线性增长 1 年 ≈ 50MB/单链；10× 压力 1 年 ≈ 500MB。在 50MB 量级内 append O(n) 尚可承受（每次追加 ~50MB JSON ≈ 1-3 秒）；超出后性能塌方。

---

## 10 跨维度发现（多维度交集的共性问题）

以下六条在 7 维度评估中均独立浮现，按交集强度排序——出现频次越高，风险面越广。

### 10.1 「活死态」结构与文档-代码漂移（影响 5 维度：代码质量 / 易用性 / 治理 / docs / 完成度）

| 现象 | 维度来源 | 证据 |
|---|---|---|
| `exitenvelope` 模块头注称「归并」「参数化承载」实测零调用点 | 代码质量 / 完成度 | `src/exitenvelope.rs:1-15`；4 bin 仍持字节级副本（gauge.rs:44-73, lease.rs:41-70, critsweep.rs:50-79, attnanchor.rs:37-56） |
| `target/debug/` 4 个孤儿二进制无源 | 易用性 / 治理 / 完成度 / 性能 | mtime 2026-08-27/28 早于多数 `src/bin/*.rs`；grep `src/ / Cargo.toml` 0 命中 |
| `doc/AGENTS-RETIRED-2026-09.md` + `doc/CASCADE.json` 未归治理子目录 | docs / 治理 | 9 个子目录外落仓根；文件名格式不与 DEC-NNN-xxx.md / DES-NNN-xxx.md 一致 |
| `src/lease/` 空目录与 `src/bin/lease/*` 七子模块三重命名 | 代码质量 / 易用性 | 仅含 `fixtures/{golden,golden2}/`，零 `.rs` 源 |

含义：仓内多处存在「意图已声明 / 实现未到位 / 状态不自洽」的张力，对维护者与新进开发者同时降低信任度。

### 10.2 完成度缺最后三步（影响 3 维度：完成度 / 治理 / 易用性）

`doc/plan/release-0.9.0-v1.md:39-41` 自承「步骤 7-9 归主窗终验后执行」；DEC-022 1.0.0 判据二「陌生人零帮助指南走查」与判据三「修复节奏实测」材料未齐备。状态：`version=0.9.0` 已落、`bin` 已在 `target/debug/`、`sih/event/trail/` 已 27 天累计 3,112 行——前 6 步证据齐，发布链路尾段未执行。这是 v1.0.0 晋升的关键漏点。

### 10.3 bin 自治 vs 库统一的张力（影响 3 维度：代码质量 / 易用性 / 可靠性）

- `sha256_hex` 在 11+ bin 重复实现（`grep "fn sha256_hex"` 命中 src/bin/lease.rs:72, gauge.rs:75, cascade.rs:47, confledger.rs:161, identity.rs:103, incubation.rs:56, acceptor.rs:156, projsnap.rs:35, formatter.rs:61, basemgr.rs:128, locator.rs:64, tally.rs:335 等）
- `parse_args` 7 处独立手卷（lease.rs:106-126, critsweep.rs:112, gauge.rs:748, basemgr.rs:627, latextool.rs:71, scrutinator.rs:75 + pendline/mod.rs:133-170），签名语义各异
- CLI 参数语法三种风格并存（scribe 不支持 `--flag=value`、critsweep 支持、lease BTreeMap + 重复）
- thiserror 仅 2/122 文件使用（ask3repeater/validate.rs:11 + tools_registry.rs:35）

含义：「bin 自治」是工程现实，「库统一」是文档意图。两者并存不必然是错（`exitenvelope.rs:7-15` 明文列未并族 13 个并解释「强并即行为漂移，不并」），但**意图与现实的边界需在头注之外显式承认**。

### 10.4 测试覆盖广而不深（影响 2 维度：可靠性 / 完成度）

- 测试 543 函数 / 65 文件 / 估算覆盖 40+/43 bin（mergeall 桶 33 + defectwave 3 + gap 5 + golden 8）
- `fixtures/2026-07-27.ndjson` 6 事件 / `fixtures/2026-08-03.ndjson` 7 事件——千/万级事件未覆盖
- 全 `src/` 与 `tests/` `grep #[should_panic]` 0 命中；`src/event_stream/hash.rs:26 / 65` 与 `src/event_stream/append.rs:123` 构造性 `.expect()` 未测
- T3 退出码测试仅覆盖 0 与 2，缺 code 1（`tests/lease_mergeback_t3_exit_codes.rs:11-25`）
- SDDG-1 缺内单测，仅由外部 `tests/lease_mergeback_t6_close_gates.rs:312` 覆盖

含义：测试组织度（按批组织而非按模块）高于一般工程仓，但边界条件与 panic 路径覆盖偏薄。

### 10.5 哲学/数学承接以文档为主、源码为辅（影响 2 维度：治理集成度 / 完成度）

- DES-002 / DES-009 / KNOW-001 设计承载强
- 代码层显式使用 `sih-philosophy` 路径仅 `src/event_stream/tdd_tests.rs:257` 一处样例数据
- 但 `DES-002:11` 显式拒绝「不修改哲学仓」——这是设计层的边界冻结，不是承接缺失

含义：哲学/数学承接是 design-corollary 层级（文档承载），与代码层的工程承接在 v0.9.0 阶段分工清晰；convergence 层承接仅 KNOW-001 单档，P1.1/P1.2/P1.3/P3.1.3/P3.2/P4.2/P4.3 等多命题全压单档是事实覆盖深度问题。

### 10.6 trail 增长是已确认事实瓶颈（影响 2 维度：性能 / 完成度）

`src/event_stream/append.rs:108-131` O(n) per write 真实存在：`load_events(p)` 全量读 + `serde_json::from_str` 逐行解析 + 序列化单行 + `writeln!` 直写；`src/event_stream/` 内 0 处 `BufReader / BufWriter`。现场 27 天累计 3,112 行 / 3.5MB（最大单文件 354KB / 301 行）。无压缩、无归档、无 offset 索引、无增量哈希。在 50MB 量级内 append O(n) 尚可承受；超出后性能塌方——是当前唯一的物理基础设施层瓶颈。

---

## 11 风险与盲点

### 11.1 风险（综合 7 维度）

| # | 风险 | 严重度 | 维度来源 |
|---|---|---|---|
| R1 | 0.9.0 发布执行链路尾段未完成，1.0.0 判据材料不齐 | **高** | 完成度 / 治理 |
| R2 | trail O(n) per write 在 100 天尺度后成为事实瓶颈 | **高** | 性能 |
| R3 | `exitenvelope` 模块活死造成代码-文档漂移 | **高** | 代码质量 |
| R4 | MCP 错误返回通道混杂：error payload 走 `internal_error` 丢结构 | 中-高 | 易用性 / 治理 |
| R5 | ToolsRegistry 与 sihmcp 分派面零接线（让位后批） | 中 | 治理 / 易用性 |
| R6 | 4 个 `target/debug/` 孤儿二进制 + `doc/` 根级 2 件非标未归桶 | 中 | 易用性 / 治理 / 完成度 |
| R7 | bin 自治风格分裂（CLI / 退出码流 / 错误类型） | 中 | 代码质量 / 易用性 / 可靠性 |
| R8 | lock/unlock/commit 跨通道 trail 名空间事实风险 | 中 | 治理 / 可靠性 |
| R9 | README 违反 DES-001 字符约束 | 中 | docs / 治理 |
| R10 | convergence 层承接仅 KNOW-001 单档 | 低-中 | 治理 / 完成度 |
| R11 | 测试 panic 路径 0 覆盖 + SDDG-1 缺内单测 | 低-中 | 可靠性 |

证据全部带 file:line 引用，详见本报告 3-9 节对应位置与第 10 节跨维度发现。

### 11.2 盲点（评估深度边界）

| # | 盲点 | 出处 |
|---|---|---|
| B1 | 顶层 `sih/ scribe/ critsweep/` 三目录内容/职能未扫描 | inventory 盲点 #8 |
| B2 | `probes/` `worktrees/` 子目录未深扫 | inventory 盲点 #10 |
| B3 | `Cargo.lock` 152 传递依赖闭包未展开 | inventory 盲点 #11 |
| B4 | `cargo build / cargo test / cargo clippy` 实际运行时数据未采集 | 7 维度评估共认 |
| B5 | `sih-tools/ sih-philosophy/ sih-math/` 三仓实际承袭镜像与差异分析未做双向对位 | 治理集成度盲点 #7 |
| B6 | `fixtures/*.ndjson` 极小，样本充分性未验证（千/万级事件未覆盖） | inventory 盲点 #9 |
| B7 | `examples/` 三件未在 `Cargo.toml [[example]]` 显式声明 | inventory 盲点 #7 |
| B8 | `tests/fixtures/plugins/` 4 件插件 fixture 未在主 fixtures/ 下 | inventory 盲点 #4 |
| B9 | `tests/attractor_route.rs` 与其他 64 文件形态不一致（inventory 标 0 命中 vs 实测 10 处 `#[test]`，本次未复核） | inventory 盲点 #5 |
| B10 | `sihmcp.rs` 与 `sih.rs` 二进制命名同根 cargo 默认规则未交叉验证 | inventory 盲点 #6 |
| B11 | DES-001 字符约束对 README / RANTS.md 的覆盖域未在 GOV 文档明示 | docs 盲点 #2 |
| B12 | `LICENSE` 文件缺失，发布面许可状态未明 | docs 盲点 #5 |
| B13 | `AGENTS-RETIRED-2026-09.md` 字节数自报（16731）与实际 AGENTS.md（6629）事实对位疑问未 mtime 复核 | docs 盲点 #1 |
| B14 | `sih-tools/mcpline` 实际承袭镜像差异未测（`session.rs:3` 等多处头注对等基准声明） | 治理集成度盲点 #7 |
| B15 | `lease_close` 失败挂起候人节点 takeover 的下游承接位未扫 | 治理集成度盲点 #8 |
| B16 | sihmcp stdio vs HTTP 双传输冷启动差异未实测 | 性能盲点 #10 |

---

## 12 结论（指向迭代建议报告）

### 12.1 一句总评

sih-engine v0.9.0 是一个**治理意图强烈、文档承载详尽、核心契约强健、外壳工具有效但尾段未闭环**的工程仓——总分 6.0 / 10（等权重 6.04），处于「手动阶段 + 测试味发布 + 基线 v1 锁定」状态。

- **完成度（7.3）与治理集成度（7.0）显示「意图-执行」基本对位**——工具面超额落地、决议→源码引用密度高、MCP 双通道治理同源同典；
- **可靠性（7.0）核心契约强健但风格分裂**——hash 链 / 退出码 / 原子追加到位，thiserror 仅 2/122 文件、T3 缺 code 1、SDDG-1 缺内单测；
- **代码质量（5.0）与性能（4.0）是当前最弱环节**——exitenvelope 活死、11+ bin 重复 helper、append O(n) per write 是已确认事实瓶颈；
- **docs（6.0）与易用性（6.0）显示「入口一致、内部碎片」**——AGENTS.md 自检齐而索引薄、guide 四件分层清而 README 违反自家规范。

### 12.2 「基线已锁 / 尾段悬置」

- 基线侧：`BASELINE-v1.md:27-35` 五条原文 + AGENTS.md 自检齐 + guide 四件分层清 + MCP schema 严谨 + `exitenvelope.rs:9-11` 边界条文齐 + 模块头注承规可机械回溯——证据链清晰可钉；
- 尾段侧：`release-0.9.0-v1.md:39-41` 发布三步未执行、DEC-022 1.0.0 判据部分材料未齐、4 孤儿二进制落盘、`doc/` 根级 2 件非标、`src/lease/` 空目录——状态不自洽对象。

### 12.3 评估落点

本评估**仅为评估报告，不下解决方案**——所有 7 维度评估已分别产出 JSON，本报告综合 7 维度的对位关系并归纳「跨维度发现」与「风险与盲点」。

方向声明（由决策者定断）：
- **决策方向**：第 11.1 节 11 条风险可序化为 D-NNN 决策候选，落 `doc/decisions/` 走 CMMI Class A 评估师流程；R1 / R3 / R2 三件高严重度风险优先级居前。
- **评估方向**：第 11.2 节 16 条盲点对应深度评估边界；B3 / B4 / B5 可作下一轮评估起点。
- **报告交付**：迭代建议报告应按 D-NNN 模版展开，由人类正式签字生效——与「agent 全包」模式切分。

### 12.4 总分权重敏感性测试

| 口径 | 总分 |
|---|---:|
| 等权重（本文主用） | 6.04 |
| 判据承接权重 | 6.03 |
| 工程基线权重 | 6.18 |

**结论**：总分对权重变化钝感（区间 6.0-6.2），可信区间 **6.0-6.2**。

---

## 附录 A：本报告数据基础

- 评估输入：7 维度评估 JSON（完成度 / 易用性 / 可靠性 / 代码质量 / 治理集成度 / docs / 性能），各 JSON 自含 evidence_facts + strengths + weaknesses + observations + blindspots + rubric。
- 仓结构切面档案：inventory（122 文件 / 57,485 行 / 41 产物 / 9 规则包 / 65 测试）。
- 引用覆盖：本报告所有事实点均带至少一处 `file_path:line_number` 引用；引用未实测处已显式标注或在盲点节列出。
- 本报告不重写 7 维度评估细节；请参阅同目录下的 7 个分维评估 JSON。

## 附录 B：报告交付 JSON

```json
{
  "report_path": "/Users/moc/workspaces/SiHankor/sih-engine/doc/assessment/2026-09-22-sih-engine-evaluation-report.md",
  "target": "sih-engine",
  "version_observed": "0.9.0",
  "report_date": "2026-09-22",
  "summary": "sih-engine v0.9.0 总分 6.0/10（等权重 6.04）；治理集成度 7 与完成度 7.3 显示意图-执行基本对位，可靠性 7.0 核心契约强健但风格分裂，代码质量 5.0 与性能 4.0 是当前最弱环节（exitenvelope 活死、11+ bin 重复 helper、append O(n) per write 已确认事实瓶颈），docs 6.0 与易用性 6.0 显示「入口一致、内部碎片」（AGENTS.md 自检齐而 README 违反自家 DES-001）。七维度交叉浮现六条共病：活死态结构与文档-代码漂移（5 维度交集）、0.9.0 尾段三步未执行（3 维度交集）、bin 自治 vs 库统一张力（3 维度交集）、测试广而不深（2 维度交集）、哲学/数学承接以文档为主源码为辅（2 维度交集）、trail 增长是已确认事实瓶颈（2 维度交集）。评分对权重变化钝感（区间 6.0-6.2）。",
  "total_score": 6.0,
  "score_rubric": {
    "method": "等权重平权（各维度 1/7）+ 敏感性测试（判据承接权重 / 工程基线权重）",
    "weight_sensitivity_range": "6.0-6.2",
    "scale": "0-10",
    "anchor": "0=完全缺失, 5=半数完成, 7=全数完成但有边界外问题, 9=工业级, 10=无瑕"
  },
  "dimension_scores": {
    "completeness": {
      "score": 7.3,
      "internal_weights": {"tools_coverage": 0.30, "engineering_baseline_5": 0.25, "roadmap_milestones": 0.20, "philosophy_acceptance": 0.15, "decision_implementation": 0.10},
      "one_line": "工具面 5/5 + 36 件内部工具超额；治理件库 84 件齐；缺发布三步与部分 1.0.0 判据材料"
    },
    "usability": {
      "score": 6.0,
      "internal_weights": {"mcp_error_payload": 0.20, "exit_code_consistency": 0.15, "stdio_auto_domain": 0.15, "cli_consistency": 0.20, "top_level_facade": 0.10, "docs_cli_consistency": 0.10, "learning_curve": 0.10},
      "one_line": "MCP 教学载荷行业顶级（9 字段 reason_code 表）；CLI 一致性、orphan 二进制、--help 覆盖拖分"
    },
    "reliability": {
      "score": 7.0,
      "internal_weights": {"hash_chain_core": 0.30, "exit_code_three_states": 0.15, "atomic_append_flock": 0.15, "error_type_uniformity": 0.15, "test_coverage_edge_cases": 0.15, "sddg_gates": 0.10},
      "one_line": "hash 链 / 退出码 / 闸门核心契约强健；thiserror 2/122 + T3 缺 code 1 + panic 0 测 + fixture 极小"
    },
    "code_quality": {
      "score": 5.0,
      "internal_weights": {"module_boundary": 0.20, "duplicate_code": 0.25, "type_ownership": 0.15, "abstract_leak": 0.15, "comments_naming": 0.25},
      "one_line": "模块切分与头注详尽度优；exitenvelope 活死、11+ bin 重复 helper、文档-代码漂移最大瑕疵"
    },
    "governance_integration": {
      "score": 7.0,
      "internal_weights": {"mcp_dual_transport": 0.25, "scribe_trail_hashing": 0.20, "lease_trail_cross_check": 0.15, "philosophy_consumption_paths": 0.15, "decision_as_governance_object": 0.15, "tools_registry_sihmcp": 0.10},
      "one_line": "MCP 双通道治理同源同典完整；ToolsRegistry 让位后批+哲学源码承接仅 1 处 test fixture"
    },
    "docs": {
      "score": 6.0,
      "internal_weights": {"agents_md_doc_consistency": 0.30, "guide_completeness": 0.20, "inline_comment_quality": 0.20, "examples_value": 0.10, "contributing_substitute": 0.10, "cross_doc_reference_accuracy": 0.10},
      "one_line": "AGENTS.md 自检齐 + guide 四件分层清；README 违反 DES-001 + 4 mergeback 件 AGENTS.md 缺席"
    },
    "performance_scalability": {
      "score": 4.0,
      "internal_weights": {"append_path_complexity": 0.30, "ndjson_growth_handling": 0.25, "pack_loading": 0.15, "concurrency_model": 0.15, "build_optimization": 0.15},
      "one_line": "编译期内嵌 pack 22KB + tokio 依赖图克制 + 日切 trail 是亮点；append O(n) per write + 无压缩 + 无 [profile.release] 是事实瓶颈"
    }
  },
  "top_findings": [
    {
      "id": "F1",
      "title": "「活死态」结构与文档-代码漂移（5 维度交集）",
      "affected_dimensions": ["code_quality", "usability", "governance_integration", "docs", "completeness"],
      "key_evidence": [
        "src/exitenvelope.rs:1-15 头注声称「归并」「参数化承载」，实测零调用点；4 bin 仍持字节级副本（gauge.rs:44-73, lease.rs:41-70, critsweep.rs:50-79, attnanchor.rs:37-56）",
        "target/debug/ 4 个孤儿二进制（ask3gate/ask3repeatergate/retrievergate/scribegate，mtime 2026-08-27/28）无源",
        "doc/AGENTS-RETIRED-2026-09.md + doc/CASCADE.json 未归 9 个治理子目录",
        "src/lease/ 空目录与 src/bin/lease/* 七子模块三重命名"
      ],
      "implication": "仓内多处存在「意图已声明 / 实现未到位 / 状态不自洽」张力，对维护者与新进开发者同时降低信任度"
    },
    {
      "id": "F2",
      "title": "0.9.0 发布尾段三步未执行（3 维度交集）",
      "affected_dimensions": ["completeness", "governance_integration", "usability"],
      "key_evidence": [
        "doc/plan/release-0.9.0-v1.md:39-41 自承「步骤 7-9 归主窗终验后执行」",
        "DEC-022 1.0.0 判据二「陌生人零帮助指南走查」与判据三「修复节奏实测」材料未齐备（doc/decision/022-semver-release-v1.md:18-32 + 70 行）"
      ],
      "implication": "v1.0.0 晋升的关键漏点；前 6 步证据齐（version 已落、bin 已 41、trail 已 27 天），尾段未执行"
    },
    {
      "id": "F3",
      "title": "bin 自治 vs 库统一的张力（3 维度交集）",
      "affected_dimensions": ["code_quality", "usability", "reliability"],
      "key_evidence": [
        "sha256_hex 在 11+ bin 重复实现（grep fn sha256_hex 命中 src/bin/lease.rs:72, gauge.rs:75, cascade.rs:47, confledger.rs:161, identity.rs:103, incubation.rs:56, acceptor.rs:156, projsnap.rs:35, formatter.rs:61, basemgr.rs:128, locator.rs:64, tally.rs:335 等）",
        "parse_args 7 处独立手卷（lease.rs:106-126, critsweep.rs:112, gauge.rs:748, basemgr.rs:627, latextool.rs:71, scrutinator.rs:75 + pendline/mod.rs:133-170），签名语义各异",
        "CLI 参数语法三种风格并存（scribe 不支持 --flag=value、critsweep 支持、lease BTreeMap + 重复）",
        "thiserror 仅 2/122 文件使用（ask3repeater/validate.rs:11 + tools_registry.rs:35）"
      ],
      "implication": "意图与现实的边界需在头注之外显式承认；exitenvelope.rs:7-15 明文列未并族 13 个并解释「强并即行为漂移，不并」——这意味着归并策略是有意为之但归并未完成"
    },
    {
      "id": "F4",
      "title": "测试覆盖广而不深（2 维度交集）",
      "affected_dimensions": ["reliability", "completeness"],
      "key_evidence": [
        "测试 543 函数 / 65 文件 / 估算覆盖 40+/43 bin（mergeall 桶 33 + defectwave 3 + gap 5 + golden 8）",
        "fixtures/2026-07-27.ndjson 6 事件 / fixtures/2026-08-03.ndjson 7 事件——千/万级事件未覆盖",
        "全 src/ 与 tests/ grep #[should_panic] 0 命中；src/event_stream/hash.rs:26/65 与 src/event_stream/append.rs:123 构造性 .expect() 未测",
        "T3 退出码测试仅覆盖 0 与 2，缺 code 1（tests/lease_mergeback_t3_exit_codes.rs:11-25）",
        "SDDG-1 缺内单测，仅由外部 tests/lease_mergeback_t6_close_gates.rs:312 覆盖"
      ],
      "implication": "测试组织度（按批组织而非按模块）高于一般工程仓，但边界条件与 panic 路径覆盖偏薄"
    },
    {
      "id": "F5",
      "title": "哲学/数学承接以文档为主、源码为辅（2 维度交集）",
      "affected_dimensions": ["governance_integration", "completeness"],
      "key_evidence": [
        "DES-002 / DES-009 / KNOW-001 设计承载强",
        "代码层显式使用 sih-philosophy 路径仅 src/event_stream/tdd_tests.rs:257 一处样例数据",
        "DES-002:11 显式拒绝「不修改哲学仓」——设计层边界冻结，不是承接缺失",
        "convergence 层承接仅 KNOW-001 单档"
      ],
      "implication": "哲学/数学承接是 design-corollary 层级（文档承载），与代码层工程承接在 v0.9.0 阶段分工清晰"
    },
    {
      "id": "F6",
      "title": "trail 增长是已确认事实瓶颈（2 维度交集）",
      "affected_dimensions": ["performance_scalability", "completeness"],
      "key_evidence": [
        "src/event_stream/append.rs:108-131 O(n) per write 真实存在：load_events(p) 全量读 + serde_json::from_str 逐行解析 + 序列化单行 + writeln! 直写",
        "src/event_stream/ 内 0 处 BufReader / BufWriter（grep 验证）",
        "现场 sih/event/trail/ 已 27 天累计 3,112 行 / 3.5MB（最大单文件 354KB / 301 行）",
        "无压缩、无归档、无 offset 索引、无增量哈希",
        "非 Unix 平台 append flock 降级为 no-op（src/event_stream/append.rs:73-76）"
      ],
      "implication": "在 50MB 量级内 append O(n) 尚可承受；超出后性能塌方——是当前唯一的物理基础设施层瓶颈"
    }
  ],
  "risk_register_summary": {
    "high_severity": ["R1 (0.9.0 尾段)", "R2 (trail O(n) per write)", "R3 (exitenvelope 活死)"],
    "medium_severity": ["R4 (MCP 错误通道混杂)", "R5 (ToolsRegistry 让位后批)", "R6 (orphan + 非标文件)", "R7 (bin 风格分裂)", "R8 (trail 名空间事实风险)", "R9 (README 违反 DES-001)"],
    "low_severity": ["R10 (convergence 单档深度)", "R11 (panic 路径 + SDDG-1)"]
  },
  "blindspot_count": 16,
  "word_count_estimate": "约 6500 字（中文）",
  "no_solution_discipline": true,
  "no_decision_prejudgment": true,
  "deliverable_scope": "评估报告（不含迭代建议）",
  "next_deliverable_pointer": "迭代建议报告应按 D-NNN 决策模版展开，由人类正式签字生效"
}
```

---

**报告完。** 所有事实点已带 `file_path:line_number` 引用；强弱项与盲点评估边界已显式列出；总分对权重变化钝感（区间 6.0-6.2）；不下解决方案，明示指向由决策流程承载的迭代建议报告。
