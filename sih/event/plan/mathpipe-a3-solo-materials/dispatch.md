# mathpipe-a3-solo 执行指令（冲突模式）

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 mathpipe-a3-solo。先读 /Users/moc/workspaces/SiHankor/AGENTS.md 与任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/mathpipe-a3-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

## 冲突模式声明（用户令，承 pk-045）

本批与他批**故意并发启动**，冲突是测试目的不是事故。规则：撞他会话锁即等待有限重试（上限十次，逐次计数），不绕行不抢先；每次冲突即记一条入结果档冲突样本节（冲突点、时点、对方批、机械响应、解决路径、重试计数）；认证一律先落主树活链即 meter 包裹 scribe append 主链绝对路径，链文件只在 settle 前一次性拷入工地，严禁在工地链副本追加事件；收约撞主树未跟踪件或活写态走备份让位归并对表法，diff 非 identical 即停批上报不硬来。

## 会话启动（首动作）

例行读数 gauge record 全参形即 AGENTS 会话启动节，--session 即 ask3 记录 session_id；泊界心跳 attractor route --pack src/attractor/packs/parking --reference-time <实日> 两线材料目录，告警如实记。

## 批内容

gauge 两张期票清偿即 PROB-003 置信带与 PROB-005 贝叶斯语义：mapping.md 全读定条目、推导档落 sih-math/docs/、gauge 计算核接线、公式版本升 ga-2 且 ga-1 旧读数回放兼容、金向量冻结复算、判变逐件申报。

## 机械链

照 BATCH-FACE 全序：ask3（三锚引文程序切片即 07-on-assay.md:61 与 06-on-canon.md:185 与 08-on-settle.md:108，session_id 形如 sess-zcode-<实日无横线>-a3）→ 双门 → elicit 逐词 --words 加 digest（契约逐词叩问处置标记）→ 正身 → lease open --package mathpipe-a3-solo --repo sih-engine --repo sih-tools --repo sih-math --root 绝对（包名与文件名严格一致，allow 按请求写入节一次列全即冻结）→ 取锁（显式 --session，撞锁按冲突模式规则重试计数）→ meter 包裹 scribe intent → 工地三仓 → 管线三步 → 认证逐件 append 主链 → 三仓 settle --cert 链上哈希前八位 → 放锁 → close（备份让位对表法）→ reconcile 三仓 → 链 verify → 调用册留痕（追加行永不覆写他会话行）→ 结果档（含冲突样本节，起草前 retriever recall --event 切面为底稿）。

## 红线

ga-1 旧读数回放必须同判；判变零静默；源码零越 allow；上链前等绿 findings 亲读；zsh 引号与 CJK 词边界坑。

## 完工报告（最终回复直接输出）

意图哈希、两期票清偿结论（载体锚点、推导档路径、ga-2 读数、金向量、判变清单）、冲突样本节逐条（冲突点、对方批、重试计数、解决路径）、认证清单、三仓 commit 号、链 verify 对表、reconcile 读数、F 表、越线与误差申报。
