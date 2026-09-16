# adoptface 结算结果档（results）

> 批：adoptface（session 5509dccdc3df17b7，2026-09-16）
> 性质：T6 单线 solo 施工批结算——采纳面四件入库与治理收口
> 任务包：sih/state/plan/adoptface.md（主树未跟踪位惯例）
> 工地：worktrees/sih-engine/adoptface（分支 msh/adoptface，基点 494a351 与 main 同点）

## 一、批成果与验收判词（acceptance）

- 四件入库：其一 README 重写（README.md，+74/−12 统计口径 `git diff --stat`，五分钟跑起来段以现役动线为唯一源：`sih init` 落树、sihmcp stdio 缺省形配置块、会话启动五件）；其二接入指南与 AGENTS 模版（doc/guide/adoption-guide-v1.md 与 doc/guide/agents-template-v1.md，域自举到客户端配置到会话五件到租约首件单页动线，附问答九问）；其三 sih init 薄壳（src/bin/sih.rs 直调 src/mcpserver/bootstrap.rs 的 init_precheck 与 open_domain，零签发零镜像零客户端注册；tests/sih_init.rs 黑箱五用例；src/mcpserver/bootstrap.rs 仅两处 fn 放宽 pub）；其四 DEC-026 缺省 stdio 决策成文（doc/decision/026-adoption-default-stdio-v1.md，HTTP 面转显式选用位，只裁缺省面不裁通道存废）。
- 机械读数（本结算批复跑，施工态即 settle 219e2ce 原样）：`cargo test --workspace` 退出码 0，478 过 0 败 6 忽略 89 段（上批 mergeall-closeout 基线 473/0/6/87，增量即 sih_init 五用例新段）；其中 tests/sih_init.rs 五用例全过（正树落位、重跑显式拒幂等守卫、缺根拒、无 git 根拒逐项判词零写入、usage 退出码二）。让位后分支 tip 实态见偏差节申报（gap 段 t1/t2 为 main 基点固有红）。
- 硬纪律对表：Cargo.toml `git diff` 零行（版本 0.9.0 钉死、依赖清单零增删实读）；新增 .rs 两件头注前二十行正典指针核读在位（src/bin/sih.rs 含 SPEC-025 与 DEC-023 与 SPEC-026 与 DES-015 与 DES-014 与 SPEC-023；tests/sih_init.rs 含 SPEC-026 与 SPEC-025 与 DEC-023 与 DES-015），SDDG-2 机械门料过；主树他批在途件（src/bin 六 bin M 态与 sih/event、sih/state 旧档）零触碰实核。
- 链证：认证笔 d878dfe9（settle seq=7，cert_on_chain 过，commit 219e2ce 三检查全过）；意图笔 3ccf9a75（plain 形，DES-016）；gauge 三维快照落当日链（convergence 0.272727 与 adoption 1.0 与 mergeback 0.025641，ga-2）。

## 二、偏差申报（deviation，逐行承载词形）

- ask3 双门以 plain 意图替位：本批生成批但意图取 plain 形零哲学引文（DES-016 plain 形 validation 豁免条款），候人节点裁补审。
- src/mcpserver/bootstrap.rs 最小可见性放宽两处：init_precheck 与 open_domain 二 fn 由私有放宽为 pub（DEC-026 承载 sih init 薄壳跨 bin 调用位），签名全 pub 类型零私有泄漏，diff 注释逐处申报零行为变更，cargo test 全绿证。
- 任务包与批材料落主树未跟踪位：sih/state/plan/adoptface.md 与 sih/event/plan/adoptface-materials/（intent 与 identity 与 settlement）按惯例位落主树不入 commit，候批归并统一登记在档。
- 核阅域判实读：doc 三件（026 决策档与 guide 两件）核阅 exit-0 过；README.md 与 results.md 域外 exit-2（判词「目标不在任何已加载规则包声明的治理域内」，DES-001 域即 sih-engine/doc/**/*.md，两件本不在 doc 域）如实记档不属违规，规则域对 worktree 路径的显式扩面候批另裁。
- 版本钉死：Cargo.toml version = "0.9.0" 零动（SPEC-026 测试基线读数即按 0.9.0 取得，候 1.0.0 晋升批另裁）。
- 工地夹带件其一保留：tests/mergeall_t6_askroute.rs 增围堰不在位跳过守卫（SPEC-025 引擎位融回线配套测试的裸环境适配，本批施工时段 02:43 写入，服务 af-05 裸环境验收，主树同文件干净无分叉）。
- 工地夹带件其二让位退出：src/bin/parser.rs 的 resolve_pack_arg（SPEC-025 融回缺口 gap-packs-assets 承载，DES-019 引擎位包根候选序之工地保守序变体，02:48 写入）曾随 settle 219e2ce 入分支并使施工态全绿 478/0/6；收约预检撞真分叉闸（主树同文件为并发批 lease-cutover-parallel 在途 M 态且内容不同源，closegate.rs 分叉闸无绕行旗标，BATCH-FACE §10 真分叉形整批拒），按让位归并程序以 wip 笔退归并面，实现件在 219e2ce 考古位可达，候 lease-cutover-parallel 批归并统一裁决。
- 让位后分支 tip 测试实态如实申报：gap_parser_pack_assets 段 t1 与 t2 红（SPEC-025 gap-packs-assets 配套测试在 main 基点 494a351 在库而其实现为并发批在途 M 态，故基点固有红——本批 restore 后复跑实测 318 过 2 败即此二件，非本批引入退化）；其余段全绿。t1/t2 回绿路径即并发批实现归并。
- 收约绕行事由：无主闸按逐件归属申报显式绕行（主树六 bin M 态属并发批 lease-cutover-parallel、sih/event 与 sih/state 旧档 M 态属各原批，本批零触碰），落 bypass 台账留痕（DEC-024 显式绕行通道）。
- 任务包请求写入节与交付实名差集四件认领：doc/guide/adopt-v1.md 交付实名 adoption-guide-v1.md、doc/guide/AGENTS-template.md 交付实名 agents-template-v1.md（均 v1 后缀形对齐仓内正典命名惯例），doc/decision/026-mcp-default-stdio-v1.md 交付实名 026-adoption-default-stdio-v1.md（DEC-026 成文实名，采纳面缺省 stdio 语义不变），sih/event/plan/adoptface-materials/ 按惯例落主树未跟踪位不入 commit；四件以差集闸认领形逐路径带事由放行，候批修订任务包 §九对表。

## 三、可证伪条件核对（falsifiable）

| F 锚定 | 判据 | 结果 |
|---|---|---|
| F-1 | README 命令活体可跑、围堰旧调用形零残留 | 过（围堰调用形 grep 计数 0；sih init 活体由 tests/sih_init.rs 5/5 证；sihmcp 配置块对 DEC-023 载体形核读） |
| F-2 | 指南落位与 bootstrap 常量面一一对应 | 过（sih/event/trail 与 sih/ledger 与 sih/state/plan 与 sih/state/parking/materials 四目录及模板三件 grep 对表圆合） |
| F-3 | sih init 活体与 mcpserver 零行为变更 | 过（cargo test 全绿 478/0/6 含 sih_init 5 用例；bootstrap diff 纯 pub 放宽两处实读） |
| F-4 | DEC-026 成文过 000 格式与 T6 三步审计 | 过（结算批复跑：化格五件全 0 即 packs/formatter/general-v1；核阅 doc 三件 0、域外二件 exit-2 记档；检词五件 0 含 results 档一处 PRO-007 死档禁词改词复检 0，即 packs/nomenclator/core） |
| F-5 | 硬纪律五条 | 过（Cargo.toml diff 零行；头注指针核读在位；他批在途件零触碰实核；围堰零写入零调用；parser.rs 让位退出后交付面与任务包 §九请求写入节全对齐） |
