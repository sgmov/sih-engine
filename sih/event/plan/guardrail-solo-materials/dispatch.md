# guardrail-solo 执行指令（冲突模式）

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 guardrail-solo。先读 /Users/moc/workspaces/SiHankor/AGENTS.md 与任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/guardrail-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

## 冲突模式声明（用户令，承 pk-045）

本批与他批故意并发。撞他会话锁即等待有限重试（上限十次逐次计数），每次冲突记一条入结果档冲突样本节（冲突点、时点、对方批、机械响应、解决路径、重试计数）。认证一律先落主树活链即 meter 包裹 scribe append 主链绝对路径，链文件 settle 前一次性拷工地，严禁工地链副本追加。收约撞主树未跟踪件或活写态走备份让位归并对表法，diff 非 identical 即停批上报。

## 会话启动（首动作）

例行读数 gauge record 全参形即 AGENTS 会话启动节，--session 即 ask3 记录 session_id；泊界心跳 attractor route --pack src/attractor/packs/parking --reference-time <实日> 两线材料目录。

## 批内容（三护栏一余件）

1. 防分叉：scribe 四写入入口（append/intent/park/record）--trail 路径含 worktrees/ 即拒 exit 2，错文「工地链副本禁追加即认证先落主链」，显式覆写旗标默认关；单测四件；SPEC-006 修订三。
2. 收约原子性：lease close 前置态检查即目标仓合并态或共享面脏即整批拒零部分动作，错误详情透出禁空串（ordwire 事故样本即空详情致后续误操作）；lease CONTRACT 修订加单测。
3. 守卫补漏：commit-msg 增拒类即批名前缀形无 session 行者拒，回放 aea1768 同形消息必拦（构造等价消息实测），合法三形（settle/wip/merge）全放行回归。
4. 余件入库：sih-engine/sih/event/plan/ordwire-lease-solo-results.md 第 5 条行与 sih-tools/lease/CALL-LOG.md 更正行随批入版控（当前为主树活写态）。

## 机械链

照 BATCH-FACE 全序：ask3（三锚引文程序切片即 07-on-assay.md:61 与 06-on-canon.md:185 即护栏单向收紧与 08-on-settle.md:108，session_id 形如 sess-zcode-<实日无横线>-guardrail）→ 双门 → elicit 逐词 --words 加 digest（契约逐词叩问处置标记）→ 正身 → lease open --package guardrail-solo --repo sih-engine --repo sih-tools --root 绝对（allow 一次列全即冻结）→ 取锁（显式 --session，撞锁重试计数）→ meter 包裹 scribe intent → 工地双仓（含主树 cargo test 与 lease 测试全绿）→ 管线三步（SPEC 修订与 CONTRACT 域内件必过，py 域外如实记）→ 认证逐件 append 主链 → 双仓 settle --cert 链上哈希前八位 → 放锁 → close（备份让位对表法）→ reconcile → 链 verify → 调用册追加留痕 → 结果档（含冲突样本节，起草前 retriever recall --event 切面为底稿）。

## 红线

五子命令与 close 合法形行为零回退；aea1768 反例必拦是 F-3 硬判据；守卫在位严禁 plain git commit 直提；上链前等绿 findings 亲读；主树 cargo test 与 lease 测试全绿（先在三金向量漂移败披露除外）。

## 完工报告（最终回复直接输出）

意图哈希、三护栏各自红绿（工地路径拒证、close 反例拒证、守卫反例拦证）、余件入库读数、冲突样本节逐条、认证清单、双仓 commit 号、链 verify 对表、reconcile 读数、F 表、越线与误差申报。
