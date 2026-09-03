# autoflow2-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 autoflow2-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/autoflow2-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。用户实验令：两件全过得一裁，裁一过一执行一，未过进泊界——去人类节点三态分流首次全流程实测。

## 件一：谓词融回实装连跑（凭据已立：predsplit-a1/a2/a3 三枚 stable_clear 终签在链）

### 第一步 SDD：SPEC-015-predicate-mergeback-gap.md

体例全承 SPEC-014。七节家位与接口对表与双模条款与验收判据与脏目标条款与回迁债与测试计划。要点：
- 家位：路由谓词落 src/attractor/packs/{core,parking}/（routes.toml 纯数据随迁加 manifest），attractor 增 route 子命令即谓词包装载加逐件判定加三路路由输出；可插拔机制即规则包形态承 scrutinator packs 先例。
- 截流谓词融三问：src/ask3repeater/ 增截流谓词装配位，形式承 selector predicates.py 的谓词形态（schema_required 等七谓词 kinds）与 GOV-002 判据三文本「按轮判定的截流谓词族融回三问模块」，接口自钉宁窄勿宽。
- 融回基准：sih-tools/selector/src/selector/{predicates,route,pack}.py 与 packs/{core,parking}/routes.toml 全读；selector 七谓词 kinds 与 route 三态（主线/停放/丢弃）与 scraap_track 语义逐条对表。
- 金向量：净目标用真实 task-packages 材料跑围堰 selector 实测输出冻结；脏目标至少三形（schema 缺字段、未知谓词 kind、域外路径）加多包归因基线。

### 第二步 TDD：实装

- src/attractor/ 增 route 模块（pack 装载、谓词判定、三路输出）与 packs 数据；src/ask3repeater/ 增截流谓词位；src/bin/attractor.rs 增 route 子命令；lib.rs 导出；Cargo 按需。
- T1-T6 先红后绿承 SPEC-015 测试计划；金向量逐字节（引擎 route 输出对围堰 selector 输出，同参形 cmp）；cargo test 全绿；cargo build 后实跑 attractor route 一行探针留证。
- 过程件归零：sih-engine/task-packages/f-anchors-x11-t6d.md 归档迁移至 sih/event/plan/ 过程件档（git mv 保历史或复制入档删原位，形态照 GOV-002 判据五「引擎 task-packages 内 facet 过程件归零」机械判定——归零后 task-packages 目录零 facet 过程件）。
- 围堰 selector 源码零改动。

### 第三步 切换

- 完成档 mergeback-predicate-completion-2026-09-02.md 三查（引 SPEC-015 验收判据与 TDD 双跑证据；接口契约未变；回迁债清账）。
- GOV-003 v1.8：判据二三五闭项表述（路择谓词融判定器、截流谓词族融三问、过程件归零），至此 GOV-002 五条全绿即主线 v1 结算条件达成——只表述状态不宣称结算，结算归人节点。
- BATCH-FACE 路由段改引擎件正典加围堰兼容只读注；DEC-013 修订追记。
- PARKING 名册若有谓词相关泊档如实更新（pk-037/038 不属本批零触碰）。

## 件二：命题写法规范先裁后行

1. **起草命题**：sih-tools/facet/facet_task_packages/predspec-guard-1/topic.md，gid predspec-guard-1、n: 9、单锚 baseline_4（可验证性即规范使命题质量机械可查）。命题：「得一命题选题写法规范应立为引擎向界层规范文件 SPEC-016——一命题一裁决依据族、禁跨主题捆绑、双述不罚、frontmatter 必载项、对己不利声明模板五条款成文」。**对己不利声明双倍显式**：规范将约束本席位后续每一次命题起草，且本命题自身是元层自指即规范若立即本命题应按规范被评——利益与元层双重披露。
2. **全流程引擎件出裁**：emit-contract（atoms 承 contract-emit 基线）→ 九发独立重采（禁复用任何旧响应）→ score → check；seat 基线对表当日最新标定件（predsplitAB 批标定 identity_hash 58e22070，同席当日可复用；漂移即重标定承先例）。
3. **三态分流（本实验流核心）**：
   - stable_clear 裁决通过 → sign 终签入链 → 执行：落 doc/spec/SPEC-016-attractor-proposition-drafting.md，内容骨架即四场实测结论（一命题一裁决依据族；禁跨主题捆绑即 predmerge-guard boundary 实证；双述不罚即 a1 双基线并述仍收敛实证；frontmatter 必载项即 gid/ng/n/seat/authored/anchors；对己不利声明模板即 adisp 与 predspec 两先例结构化；附录即身份漂移观察）→ SPEC-016 走管线域内零违规。
   - boundary 或不过 → 不签不硬来 → 进泊界：sih-tools/parking/materials/pk-039.json 落档（entry_id pk-039、title 命题写法规范另拟、exit_condition 用户裁载体或重拟、ttl_days 14）、停泊事件经书简 park 入链、PARKING-v1.md 名册更新。
   - 两态必居其一，结果档 F-5 按实态判。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片：06-on-canon 损补、07-on-assay 映照、08-on-settle 应而不藏；禁手打）→ 双门（scrutinator packs/ask3 必须 0 + ask3repeater --root 绝对必须 0）→ elicit check --packs sih-tools/nomenclator/packs/core --words 裁行泊 --words 谓词包 --words 截流谓词 --words 命题写法规范 加 digest passed（信号落任务包叩问处置节）→ 正身（reports/2026-09-02-autoflow2-identity.json 不入册）→ lease open --package autoflow2-solo --repo 两仓绝对路径 --root /Users/moc/workspaces/SiHankor（**一切锁操作显式 --session；撞锁即停批报告**）→ 取锁（任务包请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-02.ndjson 绝对；链现 180 事件 valid，settle 前后 wc -l 与末哈希对表留证；在途批尾随承先例并申报）→ inputlog 补录一笔（seq 递增，sess-zcode-260902-acceptor，逐字）：`2条全过得一，裁一过一执行一，未过进泊界。这是一个去人类节点的实验工作流程`（note 即去人类节点实验流令）→ **工地施工**：全部实装与文档在 worktrees/sih-engine/autoflow2-solo 与 worktrees/sih-tools/autoflow2-solo；主树零直写 → 管线（笔在核前：化格工具件；核阅引擎件 --pack des-001 裸名——SPEC-015 与 SPEC-016 与 GOV-003 域内必须零违规，完成档与结果档域外 exit-2 如实记；检词 core 零违例；findings 亲读禁管道掩退出码）→ 认证逐件 meter 包裹 append → 双仓 settle --seq 1 --cert 链上哈希前八位 → 放锁 → close（主树碰撞备份让位归并对表法 diff identical）→ reconcile 双仓四类双零 → 当日链 verify → 调用册留痕（scribe、formatter、scrutinator、nomenclator、selector、facet、tally 触及者各一行）→ 结果档 F 表写全 → 全部产物含链尾随批入版控。

## 红线

四枚 pred 终签是件一唯一开工凭据；件二无凭据必先裁且不过不硬签；围堰 selector 与 facet 与 tally 源码零改动；金向量冻结零漂移、围堰输出唯一基准；引擎既有组件源码只增不破（attractor 增 route、ask3repeater 增截流位、lib.rs 导出、Cargo 依赖）；守卫在位严禁 plain git commit 直提；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即停批；identity/reports 与既有存量 untracked 零收编（batch-materials、c006-sb3、viewimpl 系、08-30 inputlog、task-packages 除 f-anchors 归档外零碰、sealwin3-solo-materials、leasepatch/proposition-defense 两 M、各已收批既有件、pk-037/038、assettwave 与 extinv 在飞件全零触碰）。

## 完工报告（最终回复直接输出）

意图哈希、件一三步各自结论（SPEC-015 七节一句、TDD 行数与测试计数与双跑证据、切换完成档与 v1.8 表述与过程件归零实证）、件二裁决结论（gid、gate、disposition、逐发分布）与分流实态（SPEC-016 落档或 pk-039 进泊事件哈希）、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、越线与误差申报、实验流复盘一句（裁行泊三态哪态由机器全自主、哪态见人工痕迹）。