# gitexclude：飞轮隔离——sih init 落域自动把 sih/ 写入项目 .git/info/exclude

> 令源：用户 2026-09-17 开批裁定（飞轮隔离批，动态工作流结算前置链立约开工）
> 范式：T6 单线 solo，主代理亲写零子代理

## 一、问题陈述 {#problem}

- 病灶：sih init 落域后治理数据飞轮（sih 树：event/trail 链、state/plan 任务包、domain.json 等运行时治理数据）全量落入用户项目根。用户项目自带 git 时，sih 树整树成未跟踪文件——污染用户 git status，飞轮数据面临误提交风险，司衡治理数据与用户项目版本历史纠缠。
- 暴露面是普遍形非边缘形：init_precheck 第二项强制「域根下 .git 在位」（src/mcpserver/bootstrap.rs:281），即凡开域必是 git 仓，凡开域即遇此暴露面。
- 原则（用户原话意）：司衡引擎结构不动，sih init 落下的治理数据飞轮（sih 树）不得进入用户项目的 git。

## 二、关键设计 {#design}

- 2.1 排除位选型：写 `.git/info/exclude` 而非 `.gitignore`——.gitignore 是进版本库的用户可见共享文件，写它即改用户仓内容；info/exclude 是 git 本地私有排除位（`git ls-files --others --exclude-from=.git/info/exclude`），不进版本库、零污染用户仓，飞轮与用户项目 VCS 解耦且用户仓内容零改动。
- 2.2 落点位：sih init 薄壳（src/bin/sih.rs）open_domain 成功后追加排除步骤——域根 `.git/info/exclude` 末尾追加司衡标记段（幂等去重：已有标记段不重复写），随即以 `git check-ignore` 机械验证 sih/ 被忽略，写入与验证结果入结构化出参（exclude 段带 verdict 与路径）；check-ignore 判负即验红照引擎惯例出教学 JSON 退出码一。
- 2.3 引擎结构零改动：src/mcpserver/ 零改动，bootstrap 内核与 MCP bootstrap 工具位零触碰；本批施工面限 src/bin/sih.rs 薄壳与 tests/ 与文档。薄壳位先行，MCP 开域位是否跟进同一行为候人节点另裁，不在本批。
- 2.4 零回溯：引擎仓自身 sih 树为跟踪态历史既成（GOV-004 只追加），本批不改写引擎仓自身排除态，只改未来采纳者新开域的行为。
- 2.5 文档对表：README 三步接入段与 doc/guide/adopt-v1.md 涉 init 动线处补飞轮隔离行为一句（排除位写哪、为何不写 .gitignore）。

## 三、工作清单 {#work}

- [ ] ge-01：src/bin/sih.rs 薄壳扩展——open_domain 成功后幂等写 .git/info/exclude 司衡标记段 + git check-ignore 机械验证 + 出参带 exclude 段
- [ ] ge-02：tests/sih_init.rs 配套黑箱扩展——临时 git 仓活体 init 后 exclude 段在位且 check-ignore 判 sih/ 被忽略、重跑幂等零重复段
- [ ] ge-03：文档对表——README.md 与 doc/guide/adopt-v1.md 涉 init 段补飞轮隔离行为
- [ ] ge-04：验收——cargo test 全绿、Cargo.toml 0.9.0 与依赖清单零变动对表、临时仓活体验证 git check-ignore 判 sih/ 被忽略、T6 三步审计（化格、核阅、检词）过

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 薄壳行为 | 临时 git 仓活体跑 sih init 后域根 .git/info/exclude 含司衡标记段且 `git check-ignore sih/` 退出码 0；同一域根重跑 init 标记段仅一份（幂等去重） |
| **F-2** | 零结构改动 | src/mcpserver/ diff 为零、Cargo.toml 0.9.0 钉死与依赖清单零增删、cargo test 全绿 |
| **F-3** | 非污染 | 活体 init 后用户面 `git status --porcelain` 零 sih/ 未跟踪项 |
| **F-4** | 文档一致 | README 与 adopt-v1 涉 init 段落与薄壳实际行为一致（写 info/exclude 不写 .gitignore） |
| **F-5** | 硬纪律 | 他批在途件零触碰（主树 src/bin/{formatter,locator,nomenclator,parser,selector,tally}.rs 的 M 态与 sih/event 与 sih/state 旧档）、围堰只读、T6 管线序固定 |

## 五、必读文件 {#read}

- sih-engine/src/bin/sih.rs（薄壳现状，出参形与退出码惯例）
- sih-engine/src/mcpserver/bootstrap.rs（init_precheck 六项与 open_domain 落地五步，只读基准零改动）
- sih-engine/tests/sih_init.rs（黑箱测试现形与临时仓夹具惯例）
- sih-engine/README.md（三步接入段）与 sih-engine/doc/guide/adopt-v1.md（接入动线）
- sih-engine/sih/state/plan/adoptface.md（薄壳批前位与薄壳设计源）

## 六、约束 {#constraints}

1. src/mcpserver/ 零改动——引擎结构不动，MCP bootstrap 位零触碰，施工面限 src/bin/sih.rs 与 tests/ 与文档
2. Cargo.toml 版本 0.9.0 钉死，依赖清单零增删
3. 不碰他批在途件：主树 src/bin/{formatter,locator,nomenclator,parser,selector,tally}.rs 的 M 态与 sih/event 与 sih/state 旧档零触碰
4. 围堰只读：sih-tools/ 零写入（identity verify 只读运行唯一例外，PYTHONDONTWRITEBYTECODE=1）
5. 全部施工写入工地副本路径，主树零直写（任务包与批材料按惯例落主树 sih/{state,event}/plan/ 未跟踪位）
6. 禁 git commit/push 直呼，commit 由 lease commit 机械落笔
7. T6 管线序固定：化格、核阅、检词、认证
8. .git/info/exclude 写入以幂等去重为铁律——重复 init 零重复段，排除段带司衡标记注释行以便机械识别
9. 非域根 .git 缺席形（预检二已拒）不到 exclude 步——exclude 步仅在 open_domain 成功后执行，零新错误路径分支

## 七、范式偏离声明 {#deviation}

- 本批无流程偏离。边界如实申报：MCP 面开域（sihmcp bootstrap_domain）不随本批获得 exclude 行为，薄壳位先行，MCP 位跟进与否候人节点另裁（见 2.3）。

## 八、关联文件 {#related}

- sih-engine/sih/event/plan/gitexclude-results.md（随批产出）
- sih-engine/sih/event/plan/gitexclude-materials/（意图件与正身件）

## 九、请求写入 {#requested-writes}

- sih-engine/src/bin/sih.rs
- sih-engine/tests/sih_init.rs（黑箱扩展）
- sih-engine/README.md
- sih-engine/doc/guide/adopt-v1.md
- sih-engine/sih/event/plan/gitexclude-results.md
- sih-engine/sih/event/plan/gitexclude-materials/
