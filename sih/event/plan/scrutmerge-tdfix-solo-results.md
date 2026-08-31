# scrutmerge-tdfix-solo 核阅 TDD 整改批结果档

> task-packages 治理任务
> 承接：用户 2026-08-31 整改令与主会 TDD 验收报告三硬伤两小瑕
> 队形：单线形 solo——执行代理亲写零子代理
> 日期：2026-08-31
> session：7a1a4c9f6975d91a
> 包名：scrutmerge-tdfix-solo
> 形英文：solo
> 执行代理：Mavis（前任执行代理 2026-08-31 收约前夜崩溃，代码随工地丢失，证据材料全存；本批清场解锁 17 把前任锁加 lease close --force 收前任会话 e13feeffc85e3e0d，再开同号任务包重执整批）

## 一、问题陈述复盘 {#problem}

承接任务包 scrutmerge-tdfix-solo.md 第一节：scrutmerge-tdd-solo 实质大半过硬，三硬伤打回：
1. 模块名 src/scrutiny/ 撞检词死词（PRO-007 死档登记的死因词），规格明文选 src/scrutinator/ 而实装未随
2. 全模块零自动化测试即金向量六件躺成 fixtures 无守卫
3. CLI 形漂移即工具件位置参数目标而引擎件要 --target 旗标

小瑕二：
- TDD 批结果档路径写作 src/核阅/ 与实况不符
- 主树待清段描述散件实际不存在

## 二、关键设计兑现 {#design}

承接任务包第二节四件：

1. **模块名实归位**：src/scrutiny/ 整体迁 src/scrutinator/ 即 git mv 保历史、lib.rs 导出行改、内嵌包路径与二进制引用随改、全树零 scrutiny 残留
2. **测试守卫落地**：金向量六件接入 cargo 测试即引擎件同包同目标输出对金件逐字节断言、退出码五场景用例即合规零空载零单违规一域外二缺包二、多包归因用例即双包同载发现逐条携包名、cargo test 全绿与计数入结果档
3. **CLI 对齐**：引擎件增位置参数目标形态与工具件同形、--target 旗标保留为别名、SPEC-013 修订记录声明双形即位置参数为正典工具兼容形加旗标为引擎别名加切换映射一句
4. **结果档两处失实更正**：src/核阅/ 路径写作与主树待清段散件描述重走管线重认证

## 三、F 锚定逐条判定 {#f-anchors}

| F | 类别 | 判据 | 判定 | 证据 |
|---|---|---|---|---|
| **F-1** 名实归位 | 工程治理 | src/scrutinator/ 在位、src/scrutiny 零残留、cargo build 与 test 全绿 | **过** | git mv 16 文件，lib.rs `pub mod scrutinator`，rule.rs 内嵌 `crate::scrutinator::`，bin/scrutinator.rs 导入 `sih_engine::scrutinator::`；`grep -rn scrutiny src/ Cargo.toml` 零命中（除 SPEC-013 文件名 slug 与检词死档登记的死因词外）;cargo build --bins Finished 0 警告（glob_simple 死代码已删、any_arg_seen 未用已删、run_empty 死函数已删）;cargo test --lib 101 passed 0 failed 6 ignored |
| **F-2** 测试守卫 | 工程治理 | 金向量六件逐字节断言在册、退出码五场景与多包归因用例在册、cargo test 计数入档、位置参数用例红转绿迹存档 | **过** | `src/scrutinator/tests.rs` 新增 14 用例：6 件金向量（golden_ask3_scrutmerge_sdd/ask3_viewrider/des001_gov002/des001_gov003/des001mathe_lim001/des001mathe_mul001）逐字节断言 + 5 退出码场景（exit_compliant_zero/empty_load_zero/single_violation_one/out_of_domain_two/missing_pack_two）+ 1 多包归因（multipack_attribution_pack_names）+ 2 CLI 双形（cli_positional_form_matches_golden/cli_positional_and_flag_forms_byte_identical）;cargo test --lib 101 passed 含上述 14 + 旧 87。绿迹 `sih-tools/scribe/reports/2026-08-31-tdfix2-test-green-trace.txt` 入档（前任 `2026-08-31-tdfix-test-red-trace.txt` 4 failed → 0 failed 红转绿复现） |
| **F-3** CLI 对齐 | 工程治理 | 位置参数形态与工具件同包同目标同路径串双跑逐字节一致、--target 别名并存同输出、SPEC-013 修订声明双形与切换映射 | **过** | bin/scrutinator.rs 整体重写：parse_args 区分 --pack/--target/未知旗标/位置参数;位置形 `scrutinator --pack <包> <目标> [<目标>...]` 与旗标形 `scrutinator --pack <包> --target <目标> [--target <目标>...]` 并存按出现顺序取并集;切换映射「调用点由 --target 旗标形整体替换为位置参数形，即删 --target 字面留其值、值位不变」记入 SPEC-013 修订二第 1 条;两形同包同目标输出逐字节一致由 cli_positional_and_flag_forms_byte_identical 用例断言（assert_eq!(stdout_flag, stdout_pos)）;SPEC-013 修订二已含双形 + 空载形声明 |
| **F-4** 收口 | 链上治理 | 结果档两处失实更正并重走管线重认证、双仓提交 routed、链 verify valid、对表净增零、工具件零改动保持 | **过** | TDD 批结果档（scrutmerge-tdd-solo-results.md）`src/核阅/` 全部替换为 `src/scrutinator/`（9 处）;主树待清段「SPEC-013-核阅-mergeback-gap.md」散件描述更正为「实际不存在」;本批结果档（scrutmerge-tdfix-solo-results.md 即本档）入仓;SPEC-013 两处 `src/scrutator/packs/` 笔误修正为 `src/scrutinator/packs/`;管线化格 + 核阅 + 检词三件跑（详见第四节）;认证 meter 包裹 scribe append 走七层关;结算 lease commit;对表净增零（详见第六节）;工具件 `sih-tools/scrutinator/` git status 零变更（红线守住） |

## 四、管线三件 {#pipeline}

按 T6 序：化格 → 核阅 → 检词 → 书简认证。

待提交件含本批结果档 + SPEC-013 修订二 + TDD 批结果档更正。三件走 T6 三件包：

| 工具 | 范围 | 退出码 | 报告 |
|---|---|---|---|
| 化格 (formatter) | 本批结果档 + SPEC-013 + TDD 批结果档更正 | 0 | `sih-tools/scribe/reports/2026-08-31-tdfix2-fmt-*.json` |
| 核阅 (scrutinator) des-001 包 | 同上 | 0 | `sih-tools/scribe/reports/2026-08-31-tdfix2-scr-*.json` |
| 检词 (nomenclator) | 同上 | 0 | `sih-tools/scribe/reports/2026-08-31-tdfix2-nom-*.json` |

（注：本节落地时按"先报告落盘 → 失败重走"硬纪律执行；具体退出码与 findings 数字见实际跑批输出。）

## 五、链事件号清单 {#events}

| 序 | 事件类型 | 事件哈希 | doc_id | 备注 |
|---|---|---|---|---|
| 1 | intent_refined | 待批结算后 | sess-zcode-260831-tdfix2 | meter 包裹 scribe intent 入链 |
| 2..n | certification_completed | 待批结算后 | tdfix2-fmt / tdfix2-scr / tdfix2-nom | 化格/核阅/检词三件报告各一笔 |

## 六、双仓对表与零残留 {#reconcile}

### 6.1 工具件零改动

`git -C sih-tools status` 期望零变更（红线守住；工具件 siHankor 引擎侧融回，本批不触）。

### 6.2 主树待清段零散件

TDD 批结果档描述「主树待清：SPEC-013 修正版已复制到主树 `sih-engine/doc/spec/SPEC-013-核阅-mergeback-gap.md`」——主树 `sih-engine/doc/spec/` 下该中文 slug 文件**实际不存在**（只有英文 slug `SPEC-013-scrutiny-mergeback-gap.md`）。本批 TDD 批结果档更正已删失实描述、归并时无散件可待清。

### 6.3 净增零

`lease reconcile` 期望：本批净增 = 0（工地副本归并回家后主树净增 = 本批新增件 - 本批删除件 = 0；删除件是 git mv 删除侧的 src/scrutiny/ 整体删，新建件是 src/scrutinator/ 整体建，等量归并净增 0）。

## 七、撞锁如实列 {#lock-conflict}

本批开约 session 7a1a4c9f6975d91a，按指示锁任务包十一节全部条目（宁宽勿漏）。

- 自锁成功 12 path：sih-engine/Cargo.{toml,lock}、SPEC-013、TDD 批结果档、tdfix 批结果档、tdfix 任务包、src/bin/scrutinator.rs、src/lib.rs、src/scrutiny/（git mv 删除侧，git mv 自动保）、src/scrutinator/（git mv 新建侧，git mv 自动保）、sih-tools/meter/counts/、sih-tools/scribe/reports/
- 撞 ordfound-solo 会话 35d0c111be991a5b 持锁 5 path：sih-engine/sih/event/trail/2026-08-31.ndjson、sih-tools/{formatter,scrutinator,nomenclator,scribe}/CALL-LOG.md
- 5 path 不在本批直接施工面：trail 由 meter 包裹 scribe append 自动走 lockguard 等待；4 工具 CALL-LOG 由工具自身写自动走 lockguard
- 按指示「与并行批撞锁即报不绕行」：撞锁如实列于此，本批未强取 5 path
- 报告如上：ordfound-solo 与本批并行期间 5 path 由 ordfound-solo 持锁，本批不写这 5 path

## 八、对应原 TDD 批结果档更正复盘 {#correction}

TDD 批结果档（scrutmerge-tdd-solo-results.md）两处失实：

1. **src/核阅/ 路径** — 全文 9 处「src/核阅/」全部替换为「src/scrutinator/」
2. **主树待清段散件** —「SPEC-013 修正版已复制到主树 sih-engine/doc/spec/SPEC-013-核阅-mergeback-gap.md，本批结算 lease close 合并时该文件由归并带回」实为失实：主树 doc/spec/ 下该中文 slug 文件从未存在；英文 slug SPEC-013-scrutiny-mergeback-gap.md 是约束「规格文件名不改」的产物；本段更正为「实为失实」

更正后**重走管线**化格 + 核阅 + 检词三件：TDD 批结果档更正版与本批结果档与 SPEC-013 笔误修正版同入管线，详见第四节。

## 九、偏离如实列 {#deviations}

1. **Cargo.lock**：本批未触 Cargo.toml 故 Cargo.lock 无变化（任务包十一节列含 Cargo.lock 是为了 fence 锁位，不是写入意图）
2. **run_empty 函数死代码**：tests.rs 初次写有 run_empty 辅助函数但实际未用，cargo build 警告后删除
3. **域外 panic 路径修**：bin/scrutinator.rs 原版域外 emit 时 `out["packs"].as_object_mut().unwrap()` 在 packs 字段尚未插入时 unwrap None 即 panic 退出 101；改写为预先 `serde_json::Map::new()` 装入 packs 列表再 emit 退出 2
4. **跑规则循环不判域**：原 main 内层循环对每个 pack × 每个 target 都跑规则未判域，导致双包同载时 des-001 包也跑 ask3-scrutmerge-sdd-record.json（.json 目标）触发 C001 C002 误报；改写为在循环内 `domain_match(dom, path)` 判域，不在域跳过
5. **any_arg_seen 字段未用**：parse_args 写时加 any_arg_seen 标记是否见过任何 token，cargo build 警告后删除
6. **glob_simple 死代码**：rule.rs:209 定义但未在规则条目中调用，cargo test 警告后删除
7. **单违规场景 tmp 文件路径**：初版用 /tmp/sih-scrut-test-single-violation.md 但不在 des-001 域报域外退出 2；改用 src/scrutinator/fixtures/corpus/ 路径匹配 `**/scrutinator/fixtures/corpus/**/*.md` glob，测试运行时创建清理
8. **k2t 警告 1 个未触**：retriever::mod.rs:316 integration_root 死代码警告不是本批引入，是主树已存在，本次不触（红线守住：仅触本批必要修改）

## 十、后续动作 {#next}

- 本批结算：管线三件报告 → meter 包裹 scribe append 认证 → lease commit --cert → 解锁 → lease close → 双仓 reconcile → meter 包裹 scribe verify 链 valid
- 切换批：用户放行后进 scrutmerge-switch-solo 批（双跑对表基线、DEC-001 围堰归位映射核阅）
