# deyimerge-sdd-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 deyimerge-sdd-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/deyimerge-sdd-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。用户已开得一融回门，本批是三步曲第一步：只落 SPEC-014 规格，零实现。

## 实装要点

1. **先全读三件**：sih-tools/facet/docs/CONTRACT-MODE-SPEC.md（采样腿 emit-contract/answer/score 契约）、sih-tools/tally/CONTRACT.md（机械腿 assemble/check/verify/sign/watch/crosscheck 契约）、sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md（规格体例先例，逐节仿其结构）。再盘点两工具 src 的对外函数面（engine.py、compiler.py、facet_stats*.py、tally/cli.py 已粗盘，须逐一细读签名与 IO 形）。
2. **SPEC-014 落 sih-engine/doc/spec/SPEC-014-attractor-mergeback-gap.md**，七节按任务包关键设计：家位（src/attractor/ 与二进制同名，代码标识符 attractor 承 DEC-020，治理名得一不变）、接口契约对表（两工具契约逐条零语义漂移，crosscheck 事件十五字段负载逐字段列）、双模并存条款（facet CLI 围堰原位保留即判据四；采样腿 llm_client 与采样 runner 不融回即组件层零 LLM 纪律，模块边界显式即引擎机械腿经进程边界调围堰采样腿）、验收判据（金向量逐字节含合同哈希与响应哈希与终签哈希、退出码对齐、des-011 判定规约包随迁、机械腿零网络零 LLM 不变量、跨腿契约 score 报告与 tally check 输入逐字段兼容）、金向量脏目标条款与同参形条款显式在场（承 SPEC-013 修订四教训原文引用）、回迁债（req/llm_client 依赖切分清单、跨仓子进程形态、DEC-001 归位映射行）、测试计划 T1 至 T6 先红后绿。
3. **腿切分清单是本规格的心脏**：逐文件列「融回 / 留堰」判定即 facet 的 contract_mode 与 facet_stats 与 validators 等纯机械件融回，llm_client 与 runner 与 env_loader 留堰，tally 全件机械融回；每行给一句理由，TDD 批按此逐条验。
4. 立名段零动作即 attractor 与得一 08-30 已登记在册，规格引用登记条目即可。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片：建议 06-on-canon 损补与 08-on-settle 留痕，或 01-ontology-of-names 双名理据）→ 双门（scrutinator packs/ask3 + ask3repeater --root 绝对，必须 0）→ elicit（--words 逐词重复：建议查 采样腿/机械腿/腿切分/归位映射/双模并存）加 digest passed → 正身（reports/2026-09-02-deyimerge-identity.json，不入册）→ lease open --package deyimerge-sdd-solo 双仓绝对路径（一切锁操作显式 --session <本批会话号>，撞 p3xbridge 或他会话锁即停批报告）→ 取锁（任务包请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-02.ndjson 绝对，链现 61 事件 valid 勿动前文；主树链尾有 3 事件未提交尾随本批一并入册——settle 前后 wc -l 与末哈希对表留证）→ inputlog 补录 seq 递增两笔（会话号 sess-zcode-260902-acceptor，逐字）：`数学已经做了，主线还有什么？`、`得一裁融回` → 工地施工（SPEC-014 与包档与材料在 worktrees/sih-engine/deyimerge-sdd-solo）→ 管线（化格工具件、核阅引擎件 --pack des-001 裸名即 SPEC-014 域内必须零违规、检词 core 包零违例，findings 亲读）→ 认证逐件 meter 包裹 append → 双仓 settle --seq 1 --cert 链上哈希前八位 → 放锁 → close（备份让位归并对表法，diff identical）→ reconcile 双仓保持 unrouted 0 cert_missing 0（bypass/unbypassed 双零）→ 当日链 verify → 调用册留痕（scribe、formatter、scrutinator、nomenclator 四件）→ 结果档 F 表写全 → 全部产物含链尾随批入版控。

## 红线

零实现即本批只落规格；采样腿留围堰即组件层零 LLM 纪律不许破；两工具 facet 与 tally 源码零改动；引擎源码零改动；守卫在位即严禁任何 plain git commit 直提；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁显式 --session；identity/reports 与既有存量 untracked 零收编（batch-materials、c006-sb3、viewimpl 系、08-30 inputlog、task-packages、sealwin3-solo-materials、leasepatch/proposition-defense 两 M、adjudisp 与 guardhook 既有件、p3xbridge 在途件零触碰）。

## 完工报告（最终回复直接输出）

意图哈希、SPEC-014 七节构一句、腿切分清单行数与留堰件数、教训条款落点、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、越线与误差申报。
