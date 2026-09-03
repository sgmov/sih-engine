# ordwire-lease-solo 执行指令（冲突模式）

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 ordwire-lease-solo。先读 /Users/moc/workspaces/SiHankor/AGENTS.md 与任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/ordwire-lease-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

## 冲突模式声明（用户令，承 pk-045）

本批与他批**故意并发启动**，冲突是测试目的不是事故。规则：撞他会话锁即等待有限重试（上限十次逐次计数），不绕行；每次冲突记一条入结果档冲突样本节（冲突点、时点、对方批、机械响应、解决路径、重试计数）；本批主战场即 lease 工具本体，锁竞争必与他批高频相遇，冲突样本节预期最厚即如实记；认证一律先落主树活链，链文件 settle 前一次性拷工地，严禁工地链副本追加；收约让位走备份对表法，非 identical 即停批上报。

## 会话启动（首动作）

例行读数 gauge record 全参形；泊界心跳两线材料目录。

## 批内容

lease 锁机制载体接线四件套：ORD-020 条目定位（mapping.md 全读），推导档承载锁判定语义形式化即互斥与死锁不自由与等待终止性，lease CONTRACT 引用节与源码判定位锚点注释，金向量即撞锁与让路两场景机械重放。行为零变更即若推导发现实现与载体不符停批申报不静默改。

## 机械链

照 BATCH-FACE 全序：ask3（三锚引文程序切片即 07-on-assay.md:61 与 06-on-canon.md:185 与 08-on-settle.md:108，session_id 形如 sess-zcode-<实日无横线>-ordwire）→ 双门 → elicit 加 digest → 正身 → lease open --package ordwire-lease-solo --repo sih-engine --repo sih-tools --repo sih-math --root 绝对（allow 一次列全即冻结）→ 取锁（显式 --session，撞锁重试计数）→ meter 包裹 scribe intent → 工地三仓 → 管线三步 → 认证逐件 append 主链 → 三仓 settle → 放锁 → close（备份让位对表法）→ reconcile 三仓 → 链 verify → 调用册追加留痕 → 结果档（含冲突样本节，retriever recall --event 切面为底稿）。

## 红线

lease 判定行为零变更即既有测试全绿是底线；推导与实现不符即停批申报；源码零越 allow；上链前等绿 findings 亲读。

## 完工报告（最终回复直接输出）

意图哈希、四件套结论（ORD-020 锚点、推导档路径、接线位、金向量读数）、冲突样本节逐条（预期锁竞争最厚）、认证清单、三仓 commit 号、链 verify 对表、reconcile 读数、F 表、越线与误差申报。
