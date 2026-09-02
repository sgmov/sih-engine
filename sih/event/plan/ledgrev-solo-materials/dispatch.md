# ledgrev-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 ledgrev-solo。先读 /Users/moc/workspaces/SiHankor/AGENTS.md 与任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/ledgrev-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。派工序：ledgrev-solo 为三批之首（后续 entryunique-solo 与 mathpipe-a2-solo），同一时点仅一批在途，撞当日链锁即等待不绕行。

## 会话启动（首动作，缺一不可）

1. 例行读数：cd /Users/moc/workspaces/SiHankor/sih-tools/gauge 后 PYTHONPATH=src python3 -m gauge.cli record --at <实日> --trail <引擎链可重复> --sessions-ledger ../lease/ledger/sessions.ndjson --src-root ../../sih-engine --tools-root .. --scribe ../../sih-engine/target/debug/scribe --record-trail ../../sih-engine/sih/event/trail/<日期>.ndjson --locks ../lease/ledger/locks.ndjson --session <会话号即 ask3 记录 session_id>
2. 泊界心跳：/Users/moc/workspaces/SiHankor/sih-engine/target/debug/attractor route --pack /Users/moc/workspaces/SiHankor/sih-engine/src/attractor/packs/parking --reference-time <实日> /Users/moc/workspaces/SiHankor/sih-engine/sih/state/parking/materials /Users/moc/workspaces/SiHankor/sih-tools/parking/materials，告警与否如实记入结果档。

## 批内容要点

任务包四件即三态语义修订（白名单加引用面分级加 SPEC 消歧）、判定性复核（资源性规则精化加逐件申报）、对挂表核验（六对逐对核锚点）、四件命题层补段（过机械三层检查）。原账本三件只读不动，rev1 三件同目录另立。机械三层检查命令即 python3 sih-engine/sih/state/skills/sihankor-proposition-defense/probes/check_three_proposition_audit.py <目标.md> --json。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（session_id 形如 sess-zcode-<实日无横线>-ledgrev；三锚引文程序切片逐字节子串禁手打，锚位即 sih-philosophy/emanation/proodos/07-on-assay.md:61 镜纯粹映照、06-on-canon.md:185 力度由松到紧、08-on-settle.md:108 应而不藏）→ 双门（cd sih-tools/scrutinator 后 uv run scrutinator --pack packs/ask3 必须 exit 0；引擎 ask3repeater --root 绝对路径必须 status ok）→ elicit check --packs sih-tools/nomenclator/packs/core 逐词 --words（词单自批文实跑为准，如 引用面 与 白名单 与 对挂核验）→ 消解契约文档逐词必含 叩问处置[词] 标记行再 digest（缺标记即 digest blocked）→ 正身（identity verify，reports 件不入册）→ lease open --package ledgrev-solo --repo sih-engine --repo sih-tools --repo sih-math --root 绝对（包名与任务包文件名严格一致；allow 按请求写入节一次列全，open 即冻结，租内补锁不扩 scope）→ 逐路径取锁（显式 --session）→ meter 包裹引擎 scribe intent 上链（--trail 绝对 --locks --session）→ 工地施工（worktrees 三仓，主树零直写）→ 管线三步（笔在核前：化格 formatter --pack packs/general-v1 --write；核阅引擎件 --pack des-001 裸名，math docs 与 event/plan 域外 exit-2 如实记不属违规，引擎侧 doc 域内件可拷入工地 src/scrutinator/fixtures/corpus/ 副本跑后即删；检词 core，新词先 register 且登记件必含 state: established 落工地包随批入版控）→ 认证逐件 meter 包裹 append（--locks --session）→ 三仓 settle --cert 链上哈希前八位 → 放锁 → close（主树碰撞走备份让位归并对表法，diff 非 identical 即停批上报）→ reconcile 三仓零新增 → 当日链 verify → 调用册留痕 → 结果档。

## 结果档（起草前必跑切面检索）

cd sih-engine 后 target/debug/retriever recall --event certification_completed --since <开工日> --until <止日> --at <实日> --out materials/recall-results.json，本段链上事实以切面为机械底稿不以会话记忆复述。F 表写全，归类变动申报表与对挂核验表逐件在档，机械三层检查四件 all_covered 读数在档。

## 红线

原账本三件字节不动；待确认清单只呈报不代裁；三仓源码零改动；守卫在位严禁 plain git commit 直提；上链前必须等绿；findings 亲读禁管道掩退出码；撞锁即停批报告；identity/reports 与存量 untracked 零收编；zsh 变量加引号且 CJK 邻接处 \b 不成立。

## 完工报告（最终回复直接输出）

意图哈希、四件各自结论（rev1 三态读数前后对表、归类变动件数、对挂六对核验结果、四件补段 all_covered 读数）、认证清单、三仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、越线与误差申报。
