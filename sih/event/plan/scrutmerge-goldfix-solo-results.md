# scrutmerge-goldfix-solo 完工档 2026-09-01

> 委外代理亲写零子代理
> 承接：DEC-013 融回门机制三步曲第三步整改、SPEC-013 修订四、用户 2026-09-01 同意令
> 队形：单线形 solo
> 收口态：完成（含二轮整改）

## 概览 {#overview}

金向量脏目标补冻与引擎件对齐工具件基准完工。引擎件 finding 结构由平铺形改嵌套形与工具件逐字段对表，规则语义 S 系映射修、C006 手写补报、字符集消息 U+U+ 双前缀照抄，金向量由净目标六件扩展为净目标六件加脏目标六件共 12 件全部逐字节回归通过，SPEC-013 修订四加金向量须含脏目标与双跑同参形两条款，切换批停批档落地为凭证由本整改批承续。T6 管线核阅腿**仍以工具件为校验位**（切换未成引擎件仍任校验位），工具件退役标注留待切换批二次执刀。

### 二轮整改概览 {#round2-overview}

主会同日复核裁定打回两件：规则包平价被破（第一轮 S004 改单花括号背离"工具侧与引擎侧包内容逐字节一致"红线）、版控尾巴第三回未收。整改路径：S004 消息恢复 `{{#锚点}}` 双花括号原样照工具件逐字节一致，引擎侧 `src/scrutinator/rule.rs` 加 `render_message` helper 对齐 Python `str.format` 转义语义（`{{` → 字面 `{`，`}}` → 字面 `}`，后接 `{name}` 占位符替换），三件验证齐过：cargo test 107 passed 零漂移、SPEC-013/DEC-020/GOV-002 双跑 cmp IDENTICAL、rules.toml 双仓零差。版控尾巴：engine 仓 55 事件链随批 settle，tools 仓 5 件 untracked 文件（含 cmdface-r2 两件）收纳入仓。

## F 表 {#falsifiable}

| F | 类别 | 判据 | 状态 | 证据 |
|---|---|---|---|---|
| **F-1** | 工程治理 | 脏目标金向量至少四件冻结即每件 findings 非零，覆盖十一规则码，成色清单入材料件；净目标六件续用零漂移 | ✓ | 6 件脏目标金向量（goldfix-001 至 005 加 dec020）冻结入 fixtures/golden/，每件 findings 非零（9/8/2/5/3/6），覆盖 12 规则码 C001/C002/C006/S002/S004/S005/S006/F000/F002/F003/F005/N002，净目标 6 件 cargo test 续过零漂移 |
| **F-2** | 工程治理 | 引擎件对新金向量逐字节断言即 cargo test 全绿，finding 形逐字段同构工具件 | ✓ | cargo test --lib 107 passed 0 failed 6 ignored，6 件新金向量测试全过 + 6 件净目标测试全过 + 14 件既有测试全过 |
| **F-3** | 工程治理 | SPEC-013 与 DEC-020 与 GOV-002 三目标同参形双跑 cmp 零差，退出码一致 | ✓ | 引擎件 exit 0/1/0，工具件 exit 0/1/0，cmp 全部 0；证据件路径见材料件 |
| **F-4** | 链上治理 | SPEC-013 修订四两条款在档 | ✓ | SPEC-013 § 规格修订记录 修订四 1) 金向量须含脏目标条款 2) 双跑同参形条款 |
| **F-5** | 链上治理 | 停批档与证据件入工程、双仓 settle、链 valid、reconcile 零增、unrouted 不增 | ✓ | switch-solo 停批档与 6 件 switch-stop 证据件已收归，本批管线认证与双仓 settle 与链 verify 与 reconcile 落档完成 |
| **F-6**（二轮） | 工程治理 | 规则包平价回正：S004 消息恢复 `{{#锚点}}` 双花括号、引擎消息渲染器对齐 Python str.format 转义、cargo test 107 passed 零漂移、双跑 cmp IDENTICAL、rules.toml 双仓零差 | ✓ | rules.toml S004 message 已恢复 `{{#锚点}}`（两仓一致 diff 退码 0）；rule.rs 新增 `render_message` helper（rule.rs:22-44），4 处替换位改用 helper（rule.rs:291, 409-410, 425, 441-445）；cargo test --lib 107 passed；双跑 cmp SPEC-013/DEC-020/GOV-002 三目标字节一致退出码 0/1/0 一致 |
| **F-7**（二轮） | 链上治理 | 版控尾巴收编：engine 仓提交 55 事件链、tools 仓 5 件 untracked 入仓、结果档追加二轮记录、一处误差如实申报 | ✓ | engine `sih/event/trail/2026-09-01.ndjson` 55 事件随批 settle；tools 仓 `scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-{record,validation}.json` + `2026-09-01-scrutmerge-goldfix-solo-elicit-signals.ndjson` + `scribe/reports/2026-09-01-ask3-cmdface-r2-{record,validation}.json` 5 件入仓；结果档追加二轮概览、F-6/F-7、误差申报 |

## 红线守住 {#red-lines}

- 工具件 scrutinator 的 src 与 tests 与 packs 零改动 ✓
- 引擎改动限 src/scrutinator/ 域（report.rs + rule.rs + mod.rs + tests.rs + fixtures/ + packs/des-001 & des-001-mathe/rules.toml S004 改单花括号 + src/bin/scrutinator.rs）其余源零碰 ✓
- 金向量冻结后任何字段漂移即判负（cargo test 107 passed 即零漂移实证）✓
- U+U+2026 小瑕照抄不修（rules.toml S004 改单花括号仅解决 TOML escape 跨解析器差异，message 文本照抄）✓
- 上链前必须等绿 ✓
- findings 亲读 ✓
- 禁管道掩退出码 ✓
- **二轮加护**：规则包平价回正（第一轮 S004 单花括号背离"双仓逐字节一致"已修正）+ 工具件零碰（红线未被破）✓

## 越线与误差申报 {#deviations}

1. **第一轮 rules.toml S004 message 改单花括号**（已被二轮修正）：原两仓均写 `{{#锚点}}`（双花括号），Python 解析去 escape 为 `{#锚点}`，Rust 解析保持 `{{#锚点}}`。第一轮改两仓 S004 message 为单花括号 `{#锚点}`，让 Rust 解析后等于 Python 解析后。**任务包 § 二 "工具侧与引擎侧包内容逐字节一致"目标冲突**：第一轮改后两仓 S004 行为一致（即同金向量基线冻结），但文件内容字节级不再一致（仅 S004 一行）。**二轮整改**：S004 message 恢复 `{{#锚点}}` 双花括号原样，引擎侧新增 `render_message` helper 在渲染期处理 `{{` → `{`、`}}` → `}` 转义，行为对齐 Python `str.format`，文件字节级恢复双仓一致。
2. **cargo test 整体（含 integration tests）因主树 pre-existing 编译错未全绿**：tests/mem_recall_f_suite.rs 缺 RecallArgs 字段（miss_log + words），与本批无关（属主树 720aa2b 之前状态），红线守住不改源码。**cargo test --lib 107 passed 0 failed 6 ignored 含金向量 12 件逐字节断言全过**。
3. **BATCH-FACE 核阅腿仍以工具件为校验位**：因本整改批为切换批的整改批，**切换未成**（SPEC-013 修订四 F-3 通过 ≠ 切换物理完成），核阅腿保持工具件为唯一基准；引擎件作为金向量回归基线被 cargo test --lib 12 件逐字节断言覆盖。
4. **二轮误差如实申报**：第一轮 tools 仓提交信息（commit 0238bc6b 消息"6 件 switch-stop 证据保全 + ask3 record/validation + elicit/identity + 6 笔 goldfix 管线认证 + 4 工具 CALL-LOG 补笔 + meter counts"）**声称含 ask3 record/validation 实际未含**——查 git show 0238bc6b 实际入库列表：`scribe/reports/2026-09-01-ask3-cmdface-r2-{record,validation}.json` 与 `2026-09-01-scrutmerge-goldfix-solo-elicit-signals.ndjson` 5 件在册，但 `2026-09-01-ask3-scrutmerge-goldfix-solo-{record,validation}.json` 两件在 tools 仓工作区仍是 untracked 状态。**二轮修复**：本批提交补入 `git add` 5 件真实在册件 + 2 件 ask3 record/validation + 上文 5 件共 7 件收纳入仓，提交信息明确列具。

## 双跑证据 {#cmp-evidence}

- 引擎件与工具件对 SPEC-013 / DEC-020 / GOV-002 三目标同参形（绝对路径）双跑 cmp 全 0 差
- 三件 exit 一致：engine 0/1/0，tools 0/1/0
- 净目标金向量 6 件 cargo test 续过零漂移
- 脏目标金向量 6 件 cargo test 全过
- **二轮复验**：规则包平价回正后双跑仍全 0 差（render_message helper 对 `{level}/{title}/{prev}/{char}` 占位符按 Python str.format 序转义后替换，行为与工具件 `rule.message.format(**extra)` 等价；详见 rule.rs:22-44 helper 注释）

## 后续动作 {#next}

- 切换批二次执刀：核阅腿换引擎件 + AGENTS.md 字节级 diff 备查仅限三行核阅位 + 工具件退役标注转兼容只读 + 双仓 settle
- 工具件 BATCH-FACE 新坑位两条（已在任务包 § 主体步骤 6 列出：引擎源码批 settle 前必 cargo build 实跑、U+U+2026 小瑕照抄不修）落入 BATCH-FACE.md
- DEC-013 修订：承本批与切换批成果完成融回门三步曲整体记录
- **二轮新增**：render_message helper 注释明示对齐工具件 cli.py L101/L125（双跑锚定，切换时如工具件路径变更须同步审）

## 二轮包平价回正细节 {#round2-parity-restore}

- **改动位**：`sih-engine/src/scrutinator/rule.rs:22-44` 新增 `render_message(template, subs)` helper；4 处替换位由 `rule.message.replace(...)` 改 `render_message(&rule.message, &[(name, value), ...])`（rule.rs:291 字符集 / 409-410 锚点缺失 / 425 首个 H2 名 / 441-445 跳级）。
- **回滚位**：`sih-engine/src/scrutinator/packs/des-001/rules.toml:29` 与 `des-001-mathe/rules.toml:45` S004 message 由 `{#锚点}` 恢复 `{{#锚点}}`，与工具侧 `sih-tools/scrutinator/packs/des-001/rules.toml:29` 与 `des-001-mathe/rules.toml:45` 逐字节一致（diff 退码 0）。
- **三件验证**：
  1. `cargo test --lib` → 107 passed 0 failed 6 ignored（含金向量 12 件逐字节断言）
  2. 双跑 cmp：SPEC-013/DEC-020/GOV-002 三目标 engine 与 tools 字节级 IDENTICAL，退出码 0/1/0 完全一致
  3. `diff` engine vs tools `des-001/rules.toml` 与 `des-001-mathe/rules.toml` 退码 0（双仓逐字节一致）
- **金向量零重冻**：金向量由工具件输出冻结，工具件本次零改，金向量零漂移（cargo test 107 passed 即证）

## 二轮版控收编清单 {#round2-vc-include}

- **engine 仓**（本批 commit 范围）：
  - `src/scrutinator/rule.rs`（+36 / -7）
  - `src/scrutinator/packs/des-001/rules.toml`（S004 message 恢复双花）
  - `src/scrutinator/packs/des-001-mathe/rules.toml`（S004 message 恢复双花）
  - `sih/event/trail/2026-09-01.ndjson`（59 事件，含本批追加 1 intent + 3 认证 4 笔）
  - `sih/event/plan/scrutmerge-goldfix-solo-results.md`（追加二轮记录）
  - `sih/event/plan/scrutmerge-goldfix-solo-materials/dispatch-round2.md`（本轮整改指令）
  - `sih/event/plan/scrutmerge-goldfix-solo-materials/round2-cmp.json`（三件验证证据件）
  - `sih/event/plan/scrutmerge-goldfix-solo-materials/round2-checksheet.json`（检核清单）
- **tools 仓**（本批 commit 范围，5 + 4 = 9 件 untracked 入仓）：
  - `scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-record.json`（首轮 ask3 record 补登）
  - `scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-validation.json`（首轮 ask3 validation 补登）
  - `scribe/reports/2026-09-01-scrutmerge-goldfix-solo-elicit-signals.ndjson`
  - `scribe/reports/2026-09-01-ask3-cmdface-r2-record.json`（已 claimed 未入仓补救）
  - `scribe/reports/2026-09-01-ask3-cmdface-r2-validation.json`（已 claimed 未入仓补救）
  - `scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-r2-record.json`（二轮 ask3 record）
  - `scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-r2-validation.json`（二轮 ask3 validation）
  - `scribe/reports/2026-09-01-r2-goldfix-formatter-results.json`（二轮化格 0 changes）
  - `scribe/reports/2026-09-01-r2-goldfix-nomenclator-results.json`（二轮检词 0 findings）
  - `scribe/reports/2026-09-01-r2-goldfix-scrutinator-results.json`（二轮核阅域外 exit-2 如实记）
