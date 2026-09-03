# settlement-v1-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 settlement-v1-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/settlement-v1-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。用户已批准主线 v1 结算，本批落结算单与双向界换版，零运行形态变更。

## 实装要点

1. **证据指针亲核后才落笔**：结算单每条指针必须亲开文件确认在档——判据一指针 sih/event/mergeback/ 四完成档（scribe/scrutinator/attractor/predicate-completion）加 doc/decision/ 007 至 013 关闭状态；判据二与判据五指针 SPEC-015 与 mergeback-predicate-completion-2026-09-02.md 与 task-packages 目录实况；判据三指针 src/ask3repeater/intercept.rs 与 SPEC-015；判据四指针 sih-tools/facet/CONTRACT.md 退役登记节与 SPEC-014 双模条款。
2. **结算单 SETTLEMENT-V1-2026-09-02.md** 落 doc/governance/，体例承 sih-tools/SETTLEMENT-001.md（概览/结算范围/闭段证据/结算批沉淀/泊界复检/版本固定诸节）。必备内容：五判据逐条达成证据；全态定义逐句对表（组件级联与工具调用链闭合、非封存态即损补节律运转中、知止锚点即什么叫做完了的回答）；结算批沉淀即本批三件（结算单、GOV-003 v1.9、GOV-002 v1.6）；遗留指针如实列（泊界在泊 pk-036 出泊后现泊件、pk-040、挂账清单要点）并声明不属结算阻塞；例行读数与心跳照常声明。
3. **GOV-003 v1.9**：全态定义节或版本节追记「2026-09-02 主线 v1 结算经用户批准承载，SETTLEMENT-V1 承载明细」；增长纪律与全态非封存态表述零动。
4. **GOV-002 v1.6**：退出标准节追记五条达成（各附一句达成形与日期），版本节追记 v1.6 即主线 v1 结算 2026-09-02 经用户批准；冻结清单与范畴排除零动。
5. **链上结算事件**：结算单走认证入链即构成结算留痕；例行读数 gauge record 与泊界心跳 selector route 两目录照常跑并留痕（会话启动义务）。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片：08-on-settle 应而不藏、06-on-canon 损补、07-on-assay 映照；禁手打）→ 双门（scrutinator packs/ask3 必须 0 + ask3repeater --root 绝对必须 0）→ elicit check --packs sih-tools/nomenclator/packs/core --words 结算单 --words 知止锚点 加 digest passed（信号落任务包叩问处置节）→ 正身（reports/2026-09-02-settle-identity.json 不入册）→ lease open --package settlement-v1-solo --repo 两仓绝对路径 --root /Users/moc/workspaces/SiHankor（一切锁操作显式 --session；撞锁即停批）→ 取锁（请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-02.ndjson 绝对；链现约 204+ 事件 valid，settle 前后 wc -l 与末哈希对表留证；在途批尾随承先例并申报）→ inputlog 补录一笔（seq 递增，sess-zcode-260902-acceptor，逐字）：`1、批准。2、3展开我没看懂`（note 即结算批准令；2 与 3 的展开由主会对话承载不入链）→ 工地施工（全部文档在 worktrees/sih-engine/settlement-v1-solo 与 worktrees/sih-tools/settlement-v1-solo；主树零直写；锁内运行时账本与 scribe/reports 例外承先例）→ 管线（笔在核前：化格工具件；核阅引擎件 --pack des-001 裸名——SETTLEMENT-V1 与 GOV-002 与 GOV-003 属 doc/governance 域内**必须零违规**；结果档域外 exit-2 如实记；检词 core 零违例；findings 亲读禁管道掩退出码）→ 认证逐件 meter 包裹 append → 双仓 settle --seq 1 --cert 链上哈希前八位 → 放锁 → close（主树碰撞备份让位归并对表法 diff identical）→ reconcile 双仓四类双零（engine bypass 存量 1 正常态）→ 当日链 verify → 调用册留痕（scribe、formatter、scrutinator、nomenclator 四件）→ 结果档 F 表写全 → 全部产物含链尾随批入版控。

## 红线

结算零运行形态变更即不改任何代码与配置；向界冻结条款零破即只追加不回写；泊界在泊件与挂账零碰零收编（pk-037/038/040、batch-materials、c006-sb3、viewimpl 系、08-30 inputlog、task-packages、sealwin3-solo-materials、leasepatch/proposition-defense 两 M、assettwave 与 extinv 在飞件全零触碰）；证据指针亲核在档才落笔；守卫在位严禁 plain git commit 直提；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即停批。

## 完工报告（最终回复直接输出）

意图哈希、五判据逐条证据指针清单（亲核确认一句）、结算单节构一句、双向界换版要点、例行读数与心跳读数、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、越线与误差申报。