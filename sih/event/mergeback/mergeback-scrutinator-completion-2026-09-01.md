# 核阅融回完成档 2026-09-01

本档是 DEC-013 融回门机制核阅组件的关闭凭证，三查对表逐项证据，融回门关闭。承 scrutmerge-switch-solo 批重开执行即用户 2026-09-01 切换批准重开与封窗令、goldfix 整改完成基础、SPEC-013 修订四同参形条款既立。

## 概览 {#overview}

- 三查全过即验收判据五条逐条证据、接口契约对表、AGENTS 改写连带五项结算::[三查对表](#checks)
- 全流程六批即 SDD 立制、TDD 先红后绿、tdfix 整改、切换挂起、goldfix 双轮整改、切换执行::[时间线](#timeline)
- 服役与交割概览即工具件七日服役、写入位与 trail 家位双交割、工具件转兼容只读::[交割](#handover)

## 三查对表 {#checks}

第一查验收判据全过，逐条对 SPEC-013 § 验收判据 A1-A6。

A1 三入口与生产面对表无漏项
: cargo test --lib 107 passed 0 failed 6 ignored，引擎件与工具件同包同目标同参形双跑 cmp 0 即零差，证据入材料件 `sih-engine/sih/event/plan/scrutmerge-goldfix-solo-materials/dualrun-cmp.json` 三目标 SPEC-013 DEC-020 GOV-002 与本批收编 `sih-tools/scribe/reports/2026-09-01-switch-stop-engine-*.json` 与 `2026-09-01-switch-stop-tools-*.json` 六件。

A2 配对不变量可判定
: cargo test 14 用例含 cli_positional_and_flag_forms_byte_identical 断言位参数形与旗标形同包同目标输出逐字节一致；multipack_attribution_pack_names 断言多包同载发现逐条携包名归因。

A3 向量集五类边界齐备复算一致
: 12 件金向量（6 净 + 6 脏）逐字节断言全过：净目标六件承 TDD 批冻结零漂移，脏目标六件承 SPEC-013 修订四条款在档。规则包平价回正由 S004 消息恢复 `{{#锚点}}` 双花括号原样与工具侧逐字节一致达成，引擎 `rule.rs:22-44` `render_message` helper 处理 `{{` → `{`、`}}` → `}` 转义对齐 Python str.format 语义。

A4 生产四文件复验全 valid
: 本批 T6 文档核阅以引擎件实跑即 `target/debug/scrutinator --pack $ROOT/sih-tools/scrutinator/packs/des-001 <目标.md>`，证据入 `sih-tools/scribe/reports/2026-09-01-switch-stop-engine-SPEC-013.json` 等六件；TDD 批结果档更正版与 SPEC-013 笔误修正版与 GOV-003 v1.6 修订件与 DEC-013 v1.1 修订件同入管线，化格与核阅与检词三件跑出 0 违规。

A5 退出码三值对齐
: 双跑 cmp 退出码三目标全一致即 SPEC-013 引擎 0 / 工具 0 / cmp 0、DEC-020 引擎 1 / 工具 1 / cmp 0、GOV-002 引擎 0 / 工具 0 / cmp 0；引擎件核阅工程仓结果档目标域外 exit-2 如实记承 pkgclose 与 viewreg-solo 先例。

A6 库面无 CLI 耦合
: 引擎件 `src/scrutinator/mod.rs` 库层 public API 四件即 `run` 与 `load_packs` 与 `render_report` 与 `build_manifest` 与三类型即 `EngineReport` 加 `PackManifest` 加 `RuleEntry`；bin 侧 `src/bin/scrutinator.rs` CLI 与 lib 解耦，CLI 变更不影响库面调用。

第二查接口契约未变，工具件 `sih-tools/scrutinator/CONTRACT.md` 退役登记加注即转兼容只读：六件机器形态全部保留即空腹、规则即配置、多规则包、json 报告、退出码三值、双版本戳加治理域显式声明；验收判据五条不变；融回门三查对表以本节第一查为底。引擎件 `sih-engine/src/scrutinator/mod.rs` 公共 API 与 CLI 二元皆按 SPEC-013 § 家位与模块形 § 库模块与命令行两节落位，CLI 二元 `scrutinator --pack <包> <目标>` 位置参数形与 `--target` 旗标形并存按出现顺序取并集，承 SPEC-013 修订二第 1 条。

第三查回迁债已评估，AGENTS.md 指针改写随连带改写五项一并结算归位承 PRO-005 全态节：根 AGENTS.md 三处核阅位即概览行 + 核阅调用行 + 文件索引行 改指引擎件 `sih-engine/target/debug/scrutinator`、BATCH-FACE.md 核阅腿命令同步改引擎件路径、新坑位两条入 BATCH-FACE 即引擎源码批 settle 前必 cargo build 加实跑行为探针（scribe 陈旧二进制课）加核阅腿改引擎件后工具件仅复验位、工具件 CONTRACT 退役标注 + CALL-LOG 尾行、SPEC-013 修订五 + DEC-013 v1.1 修订一 + GOV-003 v1.6 参验席实例归位三件。GOV-002 主线锁的退出标准未改：参验席由虚转实、视图位由虚转实两件已就位，闭合判据首条"任务包 007 至 013 执行完毕并关闭"已就位的子项由六组件扩为六组件加参验席实装，源即 SPEC-013 修订五切换执行记录。

## 时间线 {#timeline}

六批全流程：

1. **SDD 立制** scrutmerge-sdd-solo 批：核阅融回规格 SPEC-013 立，定义边界节与验收判据六条。
2. **TDD 先红后绿** scrutmerge-tdd-solo 批：引擎件 src/scrutinator/ 落位加金向量六件加 14 测试用例先红后绿，模块名实归位。
3. **tdfix 整改** scrutmerge-tdfix-solo 批：CLI 双形与空载形声明，三硬伤整改，撞锁跳链后由主会代偿补链实录见路标段正式收口。
4. **切换挂起** scrutmerge-switch-solo 批首开：主会裁定切换挂起转 goldfix 整改批，揭出三层差异（GOV-002 路径回显调用形伪差异、finding 结构平铺对嵌套、规则语义差即 SPEC-013 误报五处与 DEC-020 漏报六处与字符集消息 U+U+ 双前缀差），根因即金向量六件全零发现净目标对结构与语义天然免疫等价性从未被字节级钉住。
5. **goldfix 双轮整改** scrutmerge-goldfix-solo + round-2 批：金向量脏目标补冻 6 件 + 引擎件 finding 嵌套化 + S 系映射 + C006 补报 + 字符集 U+U+ 双前缀对齐；round-2 整改包平价回正 S004 消息恢复 `{{#锚点}}` 双花括号原样与工具侧逐字节一致 + 引擎 `rule.rs:22-44` `render_message` helper 处理转义 + 4 处替换位改 helper。cargo test --lib 107 passed 含金向量 12 件逐字节断言全过、3 目标同参形 cmp 零差、rules.toml 双仓零差。
6. **切换执行** scrutmerge-switch-solo 批重开（本批）：双跑判据 cargo build + test 全绿 + 三目标 cmp 零差证据入材料件 + 核阅位换旗 + 工具件退役 + 完成档三查 + SPEC-013 修订五 + DEC-013 v1.1 + GOV-003 v1.6 + 路标段正式收口 + inputlog 三笔 + 双仓 settle + 链 valid + 完工报告。

## 交割 {#handover}

核阅组件服役与交割：工具件 scrutiny 自 2026-08-27 孵化登记起七日服役，承书简融回先例的服役模式与写入位双交割，写入位即 `sih-engine/sih/event/trail/<日期>.ndjson` 由引擎件 scribe intent 与 append 写入。工具件转兼容只读，src 与 tests 与 packs 零改动，des-001 与 ask3 与 des-001-mathe 等规则包继续可由引擎件与本工具件双方加载，调用者经 `cd sih-engine && target/debug/scrutinator --pack $ROOT/sih-tools/scrutinator/packs/<包> <目标>` 为正典，旧形 `uv run scrutinator` 仅留兼容只读验证位。

trail 家位即 `sih-engine/sih/event/trail/2026-09-01.ndjson` 由引擎件 scribe append 写入，59 事件前（待本批认证与意图事件补入后即 +N），本批意图 1 件与认证若干 N 件经 meter 包裹补入，链 valid。

封装件即 `mergeback-scrutinator-completion-2026-09-01.md` 即本档归档入 `sih-engine/sih/event/mergeback/` 名随源不改。

## 边界 {#boundary}

- 本档不含规则内容的裁决，规则增删改归规则包版本管理
- 工具件 Python 实现不删不改语义，工具侧退役只标注与转兼容只读
- 切换批执行前 TDD 批必须全绿
- 切换批执行前金向量必须冻结
- 上链前必须等绿；读 findings 不只看退出码
- 锁被他在持即报不绕行

## 关联文件 {#related}

- 任务包：`sih-engine/sih/state/plan/scrutmerge-switch-solo.md`
- 规约：`sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md`
- 决策：`sih-engine/doc/decision/013-mergeback-gate.md`
- 全态：`sih-engine/doc/governance/GOV-003-fullstate-course-v1.md`
- 双跑证据：`sih-engine/sih/event/plan/scrutmerge-goldfix-solo-materials/dualrun-cmp.json`
- 切换自件六件：`sih-tools/scribe/reports/2026-09-01-switch-stop-*.json`
- 工具件退役标注：`sih-tools/scrutinator/CONTRACT.md` § 退役登记
- 路标段正式收口：`sih-engine/sih/event/plan/scrutmerge-tdfix-solo-results.md` 第 65 行后
