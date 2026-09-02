# sweepclea6-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 sweepclea6-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/sweepclea6-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。用户令：六小件全部批准一次修完，按去人类节点实验流「得一裁一过一执行一，未过进泊界」流水执行。**命题写法遵 SPEC-016（刚生效，本批即规范首用）。**

## 件一：assettwave-a 残局收拾（欠账执行，不出裁）

1. 会话 ede05ac82f712700 现态 close_failed：读 its allow 面，若工地净（worktrees/sih-engine/assetwave-a-solo 已提交 c4bc43c 即净）即 `lease close --package assettwave-a-solo --force --root /Users/moc/workspaces/SiHankor` 收出 revoked 行；拆不动即如实记录留待，不硬闯。
2. **两笔回执补链**：sih-tools/scribe/reports/2026-09-02-assetwave-a-fmt-registry.json 与 -nom-registry.json 两件载体在档，经 meter 包裹引擎 scribe append --exit-code 按文件实码（fmt 0、nom 0，亲开核后填）补链。这就是此前被覆盖丢失的两笔认证的正式补录。
3. **主树 43 件盘点**：git status 双仓 assetwave 系 untracked 逐件列清单——ask3 记录/验证件/裁决工件入 scribe/reports 随批入册；identity/reports 一律除外；task-packages 内 f-anchors 已归档不再动；逐件处置入结果档。
4. 工地拆净：close 成功即 worktree 自删；失败即手工 `git worktree remove` 加删支并如实申报。

## 件二至件六：五命题流水（每件全流程引擎件出裁，过即执行）

五命题都单锚 baseline_4，gid sweep-2 至 sweep-6，各九发独立重采，seat 基线对表当日最新标定件（9634b1de 即 predspec2 批标定；漂移即重标定）。**对己不利声明前置**（尤其件三：双前缀瑕疵是本席位照抄基准所致，修它即自纠）。命题写法遵 SPEC-016 六项 frontmatter 必载。过即签即当场执行；boundary 或不过即不签，入泊 pk-042 续号（pk-042.json 加停泊事件加名册更新），不硬来。

- **件二（sweep-2）BATCH-FACE 两坑**：命题「操作手册 BATCH-FACE 应补两坑位行——meter --quiet 旗标须置子命令前（实测 unrecognized arguments 实录）与 lease open 撞同包陈旧 issued 会话须先 close（predsplitAB 与 deyimerge-sdd 两实录）」。执行：坑位速查表加两行，verbatim 含实测报错关键词。
- **件三（sweep-3）U+U+2026 双前缀双侧同步修**：命题「字符集消息 U+U+ 双前缀笔误应双侧同步修正（工具件源头与引擎件照抄处与受影响金向量）」。执行顺序：①先扫受影响面——`grep -rn "U+U+" sih-tools/scrutinator/src/ sih-engine/src/scrutinator/` 定位工具件源头生成位；②工具件改单前缀 `U+{cp:04X}`；③引擎件 rule.rs 对应位同步改（含 290 行注释）；④受影响金向量清单（grep golden 含 U+U+ 的件）逐件以围堰修后输出重录；⑤cargo test 全绿；⑥三目标同参形双跑 cmp IDENTICAL 复验；⑦SPEC-013/016 若有「照抄不修」条款引用即加修订注「双前缀已双侧同步修正于本日」。红线：规则语义零变即只改消息文本格式。
- **件四（sweep-4）viewrider 两陈旧会话清理**：命题「租约台账两陈旧活跃会话应清理补 revoked 行删残支」。执行：72e2e40cd689 与 4b5ff07f7db4 零锁在持，`lease close --package viewrider-solo --force` 与 viewrider-resolo 同款（若 --package 语义不中即按台账 session 定位），msh/viewrider-solo 残支删，台账 revoked 行落。
- **件五（sweep-5）散件回灌**：命题「三处在途散件应入册防丢失」。执行：sealwin3-solo-materials/ 三 dispatch、deyimerge-switch-solo-materials/postmerge-verification.{json,log}、predmerge-guard-solo-materials/ 与 scrutmerge-switch-solo-materials/ 在途件逐件 git add 入册（identity 除外）。
- **件六（sweep-6）listzero CONTRACT 补笔**：命题「tally CONTRACT 修订记录应补五修一笔补正漏记」。执行：CONTRACT.md 修订记录节按体例补「2026-09-02 五修：签署印守卫（sign 三态 refused/sign failed/signed）源即 listzero-solo 批 commit 5bfb95b6」，写前先 `git show 5bfb95b6 -- tally/` 亲核对表实际改动面。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片：08-on-settle 应而不藏、06-on-canon 损补、07-on-assay 映照；禁手打）→ 双门（scrutinator packs/ask3 必须 0 + ask3repeater --root 绝对必须 0）→ elicit check --packs sih-tools/nomenclator/packs/core --words 流水 --words 残局 --words 双前缀 --words 回执 --words 坑位（逐词重复）加 digest passed（信号落任务包叩问处置节）→ 正身（reports/2026-09-02-sweep6-identity.json 不入册）→ lease open --package sweepclea6-solo --repo 两仓绝对路径 --root /Users/moc/workspaces/SiHankor（一切锁操作显式 --session <本批会话号>；撞锁即停批）→ 取锁（请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-02.ndjson 绝对；链现 254 事件 valid，settle 前后 wc -l 与末哈希对表留证；在途批尾随承先例并申报）→ inputlog 补录一笔（seq 递增，sess-zcode-260902-acceptor，逐字）：`全部批准，一次修完。`（note 即六小件流水令与 pk-013 候裁声明在案）→ 工地施工（全部在 worktrees/sih-engine/sweepclea6-solo 与 worktrees/sih-tools/sweepclea6-solo；主树零直写；锁内账本与 scribe/reports 例外承先例）→ 管线（笔在核前：化格工具件；核阅引擎件 --pack des-001 裸名——BATCH-FACE 域外如实记；检词 core 零违例；findings 亲读禁管道掩退出码）→ 认证逐件 meter 包裹 append → 双仓 settle --seq 1 --cert 链上哈希前八位 → 放锁 → close（主树碰撞备份让位归并对表法 diff identical）→ reconcile 双仓四类双零（engine bypass 台账存量 1 正常态）→ 当日链 verify → 调用册留痕（scribe、facet、tally、formatter、scrutinator、nomenclator、lease 触及者各一行）→ 结果档 F 表写全 → 全部产物含链尾随批入版控。

## 红线

全流程引擎件出裁九发独立重采；不过不硬签入泊 pk-042；引擎 scrutinator 改动限消息文本与注释与测试与金向量数据、规则判定语义零变；工具件 scrutinator 仅件三授权改动（消息格式源头一处）加 CONTRACT 类零碰；守卫在位严禁 plain git commit 直提；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即停批；identity/reports 与存量 untracked（batch-materials、c006-sb3、viewimpl 系、08-30 inputlog、task-packages、leasepatch/proposition-defense 两 M、pk-037/038/039、assettwave 与 extinv 在飞件）按件一盘点清单处置、清单外零收编。

## 完工报告（最终回复直接输出）

意图哈希、件一残局三尾巴处置实录（revoked 行、两笔回执补链哈希、43 件盘点处置表）、五场裁决逐场结论（gid、gate、disposition、逐发 basis 分布）、过件执行证据逐件（两坑行、双前缀三处与重录清单与双跑 cmp、两会话 revoked、散件入册清单、CONTRACT 补笔）、入泊件（若有）pk 号与事件哈希、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、越线与误差申报。