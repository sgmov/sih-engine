# resarch-solo 执行指令（冲突模式）

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 resarch-solo。先读 /Users/moc/workspaces/SiHankor/AGENTS.md 与任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/resarch-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

## 冲突模式声明（用户令，承 pk-045）

本批与他批**故意并发启动**，冲突是测试目的不是事故。规则：撞他会话锁即等待有限重试（上限十次逐次计数），不绕行；每次冲突记一条入结果档冲突样本节；本批目标件即 inputlog 与调用册与名册皆为高频共享面，冲突概率最高即如实逐条记；认证一律先落主树活链，链文件 settle 前一次性拷工地，严禁工地链副本追加；收约让位走备份对表法，非 identical 即停批上报；调用册只追加永不覆写他会话行，若他批同时追加同名册即共存不互删。

## 会话启动（首动作）

例行读数 gauge record 全参形；泊界心跳两线材料目录。

## 批内容

纯归档零改字三件：引擎 inputlog 2026-09-03（seq 1 至 7 逐笔对表连续无缺号）、工具仓五条 ledgrev 调用册行（逐条在档）、引擎 PARKING-v1 名册 pk-045 行（与链上入泊事件 e46ea82a 对表）。入库前后逐字节 identical，发现的任何内容问题申报不改。

## 机械链

照 BATCH-FACE 全序：ask3（三锚引文程序切片即 07-on-assay.md:61 与 06-on-canon.md:185 与 08-on-settle.md:108，session_id 形如 sess-zcode-<实日无横线>-resarch）→ 双门 → elicit 加 digest → 正身 → lease open --package resarch-solo --repo sih-engine --repo sih-tools --root 绝对（allow 一次列全即冻结）→ 取锁（显式 --session，撞锁重试计数）→ meter 包裹 scribe intent → 工地双仓 → 对表读数 → 认证 append 主链 → 双仓 settle → 放锁 → close（备份让位对表法）→ reconcile → 链 verify → 结果档（含冲突样本节）。

## 红线

零改字即逐字节 identical 是验收线；他批冲突不抢先；上链前等绿 findings 亲读。

## 完工报告（最终回复直接输出）

意图哈希、三件对表读数（逐字节 identical 证明）、冲突样本节逐条（共享面冲突如实）、认证清单、双仓 commit 号、链 verify 对表、reconcile 读数、F 表、越线与误差申报。
