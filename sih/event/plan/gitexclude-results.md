# gitexclude 结算结果档（results）

> 批：gitexclude（session 129b20a249e74549，2026-09-17）
> 性质：T6 单线 solo 施工批结算——飞轮隔离一位入库与治理收口
> 任务包：sih/state/plan/gitexclude.md（主树未跟踪位惯例）
> 工地：worktrees/sih-engine/gitexclude（分支 msh/gitexclude，基点 67131b1 即 adoptface 副本归并点）

## 一、批成果与验收判词（acceptance）

- 一件主交付入库：sih init 飞轮隔离步（src/bin/sih.rs 薄壳扩展）——open_domain 成功路径尾随 `flywheel_git_exclude`（src/bin/sih.rs:112-133 与 158-213）：域根 `.git/info/exclude` 幂等追加一行 `sih/`（info/ 目录缺席则创建；已有该行幂等跳过），随后 `git -C <域根> check-ignore -q sih/` 机械验证，出参带 `flywheel_git_exclude` 段（method 与 verified）；追加或验证失败不回滚 init（数据已落盘），失败详情经 warning 字段如实显形，零静默零假判。
- 配套黑箱扩展（tests/sih_init.rs）：沙箱域根由空 `.git` 目录改真 git 仓（`git init -q`，tests/sih_init.rs:61-68，check-ignore 要求合法仓形）；成功形用例追加五断言（出参 method 形、verified 真值、零 warning、exclude 含 `sih/` 行、`git check-ignore` 退出码零，tests/sih_init.rs:198-226）。
- 文档对表三处：README.md:44 隔离保证一句、doc/guide/adoption-guide-v1.md:64 隔离一句与 :140 问答新增一问、doc/decision/026-adoption-default-stdio-v1.md:82 DEC-026 v2 修订记录（飞轮与用户项目 VCS 解耦保证，隔离失败不回滚 warning 显形）。
- 机械读数（本结算批复跑，施工态原样）：`cargo test --workspace` 退出码 0，478 过 0 败 6 忽略 89 段（与 adoptface 基线同数，本批零新段，扩展断言并入既有 sih_init 段五用例全过）；活体补验两形——活体 A 临时真 git 仓跑 sih init 退出码 0、出参 verified 真值零 warning、exclude 恰一行 `sih/`、check-ignore 退出码 0、`git status --porcelain` 零行（sih/ 整树对用户 git 隐形）；活体 B 预置 `sih/` 行形（幂等去重分支）init 后该行仍恰一份且用户既有排除行零扰动、check-ignore 退出码 0。
- T6 三步审计（本结算批复跑，序固定化格、核阅、检词先于认证）：化格三件交付（README 与 026 决策档与接入指南）全 0 无需改（packs/formatter/general-v1）；核阅 doc 两件 exit-0、README 与本结果档域外 exit-2 记档（判词「目标不在任何已加载规则包声明的治理域内」，DES-001 域即 sih-engine/doc/**/*.md，两件本不在 doc 域，不属违规）；检词四件 0 findings 0（packs/nomenclator/core）。
- 硬纪律对表：src/mcpserver/ `git diff` 零行、Cargo.toml 与 Cargo.lock `git diff` 零行（版本 0.9.0 钉死与依赖清单零增删实读，Cargo.toml:3）；主树他批在途件（src/bin 六 bin M 态、sih/event 三个旧 results 档 M 态、sih/event/trail/2026-09-11.ndjson M 态）零触碰实核（主树 `git status --porcelain` 实读与本批写入面零交集）；围堰 sih-tools/ 零源码写入（identity 复用批材料只读件）。
- 链证：settle 认证笔与意图笔经 scribe append 落 2026-09-17 当日链（seq 与 event_hash 见 lease commit 台账与结算回报），gauge 三维快照随后落链（ga-2）。

## 二、偏差申报（deviation，逐行承载词形）

- ask3 双门以 plain 意图替位：本批生成批但意图取 plain 形零哲学引文（DES-016 plain 形 validation 豁免条款，intent.json exclusions 自报），候人节点裁补审。
- 排除段无司衡标记注释行：任务包 §六约束八言「排除段带司衡标记注释行以便机械识别」，实装为整行 `sih/` 精确匹配幂等去重（src/bin/sih.rs:164-185），幂等语义等价达成（活体 B 证零重复）；标记注释行未落，SPEC-025 薄壳位承载形与任务包字面偏离如实申报，候批修订任务包对表。
- 判负行为改写：任务包 §2.2 言「check-ignore 判负即验红照引擎惯例出教学 JSON 退出码一」，实装为隔离失败不回滚 init、warning 字段如实显形、退出码仍零（src/bin/sih.rs:112-133 与 187-213）；DEC-026 v2 修订承载此形（doc/decision/026-adoption-default-stdio-v1.md:82），任务包未回写，候人节点裁。
- 出参段名与字段形：任务包 §2.2 言「exclude 段带 verdict 与路径」，实装键名 `flywheel_git_exclude` 带 method 与 verified、无路径字段（src/bin/sih.rs:127-128；tests/sih_init.rs:200-209 断言同形钉死），SPEC-026 测试承载形与任务包字面偏离如实申报，候批对表。
- 文档件名简写认领：任务包与租约 allow 清单写 doc/guide/adopt-v1.md，交付实名 doc/guide/adoption-guide-v1.md（v1 后缀形对齐仓内正典命名惯例，adoptface 批同形差集认领先例），候批修订任务包 §九对表。
- 工地夹带件申报：src/bin/parser.rs 的 resolve_pack_arg 保守序变体（SPEC-025 融回缺口 gap-packs-assets 承载，DES-019「exe 派生首位」之保守序差异在码注申报，src/bin/parser.rs:2305-2330）非任务包 §九清单件，随本工地在途一并结算；主树同名为并发批 lease-cutover-parallel 在途 M 态且实现不同源（engine_pack_roots exe 派生首位形），让位归并处置与收约判词实录见偏差节末行与当日链台账，统一裁决候 lease-cutover-parallel 批。
- 决策修订随批落笔认领：DEC-026 v2 修订段落（doc/decision/026-adoption-default-stdio-v1.md:82）非任务包 §九清单件，承载词即 DEC-026 自身且为判负行为改写项的正典位，候批修订任务包对表。
- 核阅域判如实记档：README.md 与本结果档在 des-001 规则域（sih-engine/doc/**/*.md）域外，核阅判词以实跑退出码为准如实记档，不属违规，规则域对 worktree 路径的显式扩面候批另裁。

## 三、可证伪条件核对（falsifiable）

| F 锚定 | 类别 | 判据 | 结果 |
|---|---|---|---|
| F-1 | 薄壳行为 | 临时 git 仓活体跑 sih init 后域根 .git/info/exclude 含 `sih/` 行且 `git check-ignore sih/` 退出码 0；重跑幂等零重复 | 过（活体 A：退出码 0、verified 真值、exclude 恰一行、check-ignore 0；幂等两形证——同域重跑 init 被幂等守卫前置拒零副作用（tests/sih_init.rs init_rerun 用例）、exclude 步去重分支活体 B 预置形证零重复） |
| F-2 | 零结构改动 | src/mcpserver/ diff 零、Cargo.toml 0.9.0 钉死与依赖零增删、cargo test 全绿 | 过（mcpserver diff 0 行；Cargo.toml/Cargo.lock diff 0 行、version = "0.9.0"（Cargo.toml:3）；cargo test --workspace 退出码 0，478 过 0 败 6 忽略 89 段） |
| F-3 | 非污染 | 活体 init 后用户面 git status --porcelain 零 sih/ 未跟踪项 | 过（活体 A 实测 porcelain 零行，sih/ 整树经 info/exclude 对用户 git 隐形） |
| F-4 | 文档一致 | README 与指南涉 init 段落与薄壳实际行为一致（写 info/exclude 不写 .gitignore） | 过（README.md:44 与 doc/guide/adoption-guide-v1.md:64、:140 与 doc/decision/026-adoption-default-stdio-v1.md:82 逐处核读，与 src/bin/sih.rs:158-213 实装同形；.gitignore 零触碰） |
| F-5 | 硬纪律 | 他批在途件零触碰、围堰只读、T6 管线序固定 | 过（主树六 bin 与三个旧 results 档与 09-11 trail 的 M 态实核零交集；sih-tools/ 零源码写入；化格、核阅、检词序固定先于认证，读数见机械读数节） |
