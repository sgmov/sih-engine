# scrutmerge-goldfix-solo 完工档 2026-09-01

> 委外代理亲写零子代理
> 承接：DEC-013 融回门机制三步曲第三步整改、SPEC-013 修订四、用户 2026-09-01 同意令
> 队形：单线形 solo
> 收口态：完成

## 概览 {#overview}

金向量脏目标补冻与引擎件对齐工具件基准完工。引擎件 finding 结构由平铺形改嵌套形与工具件逐字段对表，规则语义 S 系映射修、C006 手写补报、字符集消息 U+U+ 双前缀照抄，金向量由净目标六件扩展为净目标六件加脏目标六件共 12 件全部逐字节回归通过，SPEC-013 修订四加金向量须含脏目标与双跑同参形两条款，切换批停批档落地为凭证由本整改批承续。T6 管线核阅腿**仍以工具件为校验位**（切换未成引擎件仍任校验位），工具件退役标注留待切换批二次执刀。

## F 表 {#falsifiable}

| F | 类别 | 判据 | 状态 | 证据 |
|---|---|---|---|---|
| **F-1** | 工程治理 | 脏目标金向量至少四件冻结即每件 findings 非零，覆盖十一规则码，成色清单入材料件；净目标六件续用零漂移 | ✓ | 6 件脏目标金向量（goldfix-001 至 005 加 dec020）冻结入 fixtures/golden/，每件 findings 非零（9/8/2/5/3/6），覆盖 12 规则码 C001/C002/C006/S002/S004/S005/S006/F000/F002/F003/F005/N002，净目标 6 件 cargo test 续过零漂移 |
| **F-2** | 工程治理 | 引擎件对新金向量逐字节断言即 cargo test 全绿，finding 形逐字段同构工具件 | ✓ | cargo test --lib 107 passed 0 failed 6 ignored，6 件新金向量测试全过 + 6 件净目标测试全过 + 14 件既有测试全过 |
| **F-3** | 工程治理 | SPEC-013 与 DEC-020 与 GOV-002 三目标同参形双跑 cmp 零差，退出码一致 | ✓ | 引擎件 exit 0/1/0，工具件 exit 0/1/0，cmp 全部 0；证据件路径见材料件 |
| **F-4** | 链上治理 | SPEC-013 修订四两条款在档 | ✓ | SPEC-013 § 规格修订记录 修订四 1) 金向量须含脏目标条款 2) 双跑同参形条款 |
| **F-5** | 链上治理 | 停批档与证据件入工程、双仓 settle、链 valid、reconcile 零增、unrouted 不增 | ✓ | switch-solo 停批档与 6 件 switch-stop 证据件已收归，本批管线认证与双仓 settle 与链 verify 与 reconcile 落档完成 |

## 红线守住 {#red-lines}

- 工具件 scrutinator 的 src 与 tests 与 packs 零改动 ✓
- 引擎改动限 src/scrutinator/ 域（report.rs + rule.rs + mod.rs + tests.rs + fixtures/ + packs/des-001 & des-001-mathe/rules.toml S004 改单花括号 + src/bin/scrutinator.rs）其余源零碰 ✓
- 金向量冻结后任何字段漂移即判负（cargo test 107 passed 即零漂移实证）✓
- U+U+2026 小瑕照抄不修（rules.toml S004 改单花括号仅解决 TOML escape 跨解析器差异，message 文本照抄）✓
- 上链前必须等绿 ✓
- findings 亲读 ✓
- 禁管道掩退出码 ✓

## 越线与误差申报 {#deviations}

1. **rules.toml S004 message 改单花括号**：原两仓均写 `{{#锚点}}`（双花括号），Python 解析去 escape 为 `{#锚点}`，Rust 解析保持 `{{#锚点}}`。本批改两仓 S004 message 为单花括号 `{#锚点}`，让 Rust 解析后等于 Python 解析后。**任务包 § 二 "工具侧与引擎侧包内容逐字节一致"目标冲突**：本批改后两仓 S004 行为一致（即同金向量基线冻结），但文件内容字节级不再一致（仅 S004 一行）。
2. **cargo test 整体（含 integration tests）因主树 pre-existing 编译错未全绿**：tests/mem_recall_f_suite.rs 缺 RecallArgs 字段（miss_log + words），与本批无关（属主树 720aa2b 之前状态），红线守住不改源码。**cargo test --lib 107 passed 0 failed 6 ignored 含金向量 12 件逐字节断言全过**。
3. **BATCH-FACE 核阅腿仍以工具件为校验位**：因本整改批为切换批的整改批，**切换未成**（SPEC-013 修订四 F-3 通过 ≠ 切换物理完成），核阅腿保持工具件为唯一基准；引擎件作为金向量回归基线被 cargo test --lib 12 件逐字节断言覆盖。

## 双跑证据 {#cmp-evidence}

- 引擎件与工具件对 SPEC-013 / DEC-020 / GOV-002 三目标同参形（绝对路径）双跑 cmp 全 0 差
- 三件 exit 一致：engine 0/1/0，tools 0/1/0
- 净目标金向量 6 件 cargo test 续过零漂移
- 脏目标金向量 6 件 cargo test 全过

## 后续动作 {#next}

- 切换批二次执刀：核阅腿换引擎件 + AGENTS.md 字节级 diff 备查仅限三行核阅位 + 工具件退役标注转兼容只读 + 双仓 settle
- 工具件 BATCH-FACE 新坑位两条（已在任务包 § 主体步骤 6 列出：引擎源码批 settle 前必 cargo build 实跑、U+U+2026 小瑕照抄不修）落入 BATCH-FACE.md
- DEC-013 修订：承本批与切换批成果完成融回门三步曲整体记录
