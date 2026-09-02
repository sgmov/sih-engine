# predmerge-guard-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 predmerge-guard-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/predmerge-guard-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。用户令：谓词融回先过得一裁，过了就融。本批只做裁决，通过与否都如实，不融不强推。

## 裁决流程（全引擎件，红线）

1. **命题起草**：落 sih-tools/facet/facet_task_packages/predmerge-guard-1/topic.md，形制承 adisp-guard-1 先例（frontmatter 即 gid: predmerge-guard-1、ng: medium、n: 9、authored 标明 AI 起草席位与日期）。命题正文：「路择谓词件与按轮截流谓词族应经可插拔机制融回引擎侧——路择 core 与 parking 两谓词包（routes.toml 七谓词纯数据）融回判定器 attractor 模块加载，按轮截流谓词族融回三问模块，引擎 task-packages 内 facet 过程件归零」。必含**谱系披露与对己不利声明**：起草与采样与裁定执行位同一席位（ZCode:GLM-5.3-Flash:self-reported）；判命题成立即判本席位此前赖以工作的围堰 selector 谓词被引擎化接替、本席后续同类裁决必须走引擎件；利益指向 violate 方向即维持现状对本席短便，如实披露。基线材料：GOV-002 判据二三五原文、selector 两包谓词纯数据形态、SPEC-014 双模先例。anchors 引 GOV-002 与 SPEC-014 具体行。
2. **引擎件出合同**：`cd sih-engine && target/debug/attractor emit-contract --topic <topic.md> --ng-file <ng文本> --seat "ZCode:GLM-5.3-Flash:self-reported" --gid predmerge-guard-1 --title <题名> --shots 9 --atoms src/attractor/fixtures/golden/contract-emit/atom.yaml --ng-label medium --out <材料目录>/contract.json`。若旗标形与现款有出入以 `attractor emit-contract` 无参用法提示为准对齐。
3. **seat 基线先行**：当日 seat 标定若已有（查 meter counts 与 facet_task_packages 当日基线件）即对表复用；无即按 adisp 先例 temp_probe 形补一次基线件落材料目录（基线 json 本体放 facet_task_packages/predmerge-guard-1/ 域内，禁写域外）。
4. **九发作答**：以席位模型逐发作答写 responses.jsonl（每行 key/shot/raw，raw 内 JSON 即 decision/basis_regulation/reason/boundary_flag），禁钓样本禁改答，逐发独立作答。
5. **引擎件计分**：`target/debug/attractor score --contract <contract.json> --responses <responses.jsonl> ...`（旗标以无参用法为准），得分材料落材料目录。
6. **引擎件核对与终签**：`target/debug/attractor check --material <score 材料>`；gate stable_clear 即 `target/debug/attractor sign --material <件> --trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-02.ndjson ...` 出 crosscheck_completed 事件（sign 若经子进程书简即照 SPEC-014 契约）；挂起即不签、如实落「挂起归人」入结果档。meter 包裹所有 attractor 调用（meter run -- 计数）。

## 机械链

ask3 记录（三锚引文程序切片：建议 07-on-assay 自证循环节、06-on-canon 损补、08-on-settle 应而不藏；禁手打）→ 双门（scrutinator packs/ask3 必须 0 + ask3repeater --root /Users/moc/workspaces/SiHankor 必须 0）→ elicit check --packs sih-tools/nomenclator/packs/core --words 谓词融回 --words 可插拔 --words 对己不利声明 --words 首战自证 加 digest passed（信号在任务包叩问处置节落处置行）→ 正身（reports/2026-09-02-predmg-identity.json 不入册）→ lease open --package predmerge-guard-solo --repo /Users/moc/workspaces/SiHankor/sih-tools --repo /Users/moc/workspaces/SiHankor/sih-engine --root /Users/moc/workspaces/SiHankor（**一切锁操作显式 --session <本批会话号>；viewrider 系两陈旧会话与 assetwave-a 在飞会话均与本批无关，撞锁即停批报告**）→ 取锁（任务包请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-02.ndjson 绝对；链现 120 事件 valid，settle 前后 wc -l 与末哈希对表留证；主树链尾若有在途批新事件尾随入册承先例并申报）→ inputlog 补录一笔（seq 递增，sess-zcode-260902-acceptor，逐字）：`得意裁，过了就融`（note 即谓词融回先裁后融令）→ 工地施工（结果档与材料在 worktrees/sih-engine/predmerge-guard-solo；facet_task_packages/predmerge-guard-1/ 与 CALL-LOG 与 counts 在 worktrees/sih-tools/predmerge-guard-solo；**assettwave-a 在飞批工件零触碰**）→ 管线（化格工具件；核阅引擎件 --pack des-001 裸名——结果档域外 exit-2 如实记；检词 core 包零违例；findings 亲读禁管道掩退出码）→ 认证逐件 meter 包裹 append → 双仓 settle --seq 1 --cert 链上哈希前八位 → 放锁 → close（主树碰撞备份让位归并对表法 diff identical）→ reconcile 双仓四类双零 → 当日链 verify → 调用册留痕（scribe、facet、tally 三件加 formatter/scrutinator/nomenclator 如有触及）→ 结果档 F 表写全 → 全部产物含链尾随批入版控。

## 红线

裁决全流程引擎件（emit-contract/score/check/sign 皆 target/debug/attractor，禁用围堰 facet/tally 出裁）；不过即停即不立项 SDD 不写融回规格；selector 与 facet 与 tally 源码零改动；引擎源码零改动；守卫在位严禁 plain git commit 直提；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即停批；identity/reports 与既有存量 untracked 零收编（batch-materials、c006-sb3、viewimpl 系、08-30 inputlog、task-packages、sealwin3-solo-materials、leasepatch/proposition-defense 两 M、各已收批既有件、pk-037/038、assetwave-a 在途件全零触碰）。

## 完工报告（最终回复直接输出）

意图哈希、裁决结论一句（gate_verdict 加 disposition 加逐发 decision 分布）、引擎件全流程证据（合同与计分与核对与终签各产物路径）、基线对照声明、终签事件哈希、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、越线与误差申报、分流声明（过即 SDD 候令，不过即停）。
