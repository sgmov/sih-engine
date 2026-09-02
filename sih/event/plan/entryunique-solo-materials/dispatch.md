# entryunique-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 entryunique-solo。先读 /Users/moc/workspaces/SiHankor/AGENTS.md 与任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/entryunique-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。派工序：本批为三批之二（ledgrev-solo 之后、mathpipe-a2-solo 之前），同一时点仅一批在途，撞当日链锁即等待不绕行。

## 会话启动（首动作，缺一不可）

1. 例行读数：同 BATCH-FACE 前置节 gauge record 全参形，--session 即 ask3 记录 session_id。
2. 泊界心跳：attractor route --pack src/attractor/packs/parking --reference-time <实日> 两线材料目录，告警与否如实记。

## 批内容要点

park enter 增第三道门号源唯一拒即重放面内 entry_id 出现过任何 parking_entered 即拒（承 parkreplay-solo 修订一重放面）。红绿在测试链目录 /tmp 跑，生产链只作追加面。实装位 sih-engine/src/event_stream/park.rs 与 src/bin/scribe.rs，SPEC-006 修订二。红态抓法：修前用已出泊号复用 enter 得 appended，绿态即修后同参形号源唯一拒。收约后重编主树 target/debug/scribe 并烟测。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（session_id 形如 sess-zcode-<实日无横线>-entryunique；三锚引文程序切片禁手打，锚位即 07-on-assay.md:61、06-on-canon.md:185 即新门是单向收严、08-on-settle.md:108）→ 双门（scrutinator packs/ask3 exit 0 加 ask3repeater status ok）→ elicit check 逐词 --words（词单如实跑，号源唯一已登记应零信号）→ digest（契约逐词 叩问处置[词] 标记）→ 正身 → lease open --package entryunique-solo --repo sih-engine --repo sih-tools --root 绝对（allow 一次列全即冻结）→ 取锁 → meter 包裹 scribe intent → 工地（worktrees 双仓，主树零直写）→ cargo test 加单测四件（新号过、在泊重入拒回归、已出泊号复用拒、跨天已用号拒；先在三金向量漂移败如实披露非本批引入）→ 红绿两态读数落 materials → SPEC-006 修订二走三步（化格加核阅经工地 fixtures/corpus 副本加检词）→ 认证逐件 append → 双仓 settle → 放锁 → close（备份让位归并对表法）→ reconcile 双仓零新增 → 链 verify → 主树 cargo build --bin scribe 重编加烟测一行 → 调用册留痕 → 结果档。

## 结果档（起草前必跑切面检索）

retriever recall --event certification_completed --since <开工日> --until <止日> --at <实日> --out materials/recall-results.json，链上事实以切面为底稿。F 表写全，红绿读数在档。

## 红线

不改 exit 两门、不新增事件类型、历史链零回溯；五子命令语义与退出码零变化（旧新二进制 verify 与 query 输出 cmp IDENTICAL）；守卫在位严禁 plain git commit 直提；上链前等绿；findings 亲读；撞锁即停批；mod.rs 类再输出若需改而未列 allow，改调用侧全径导入不扩 scope（parkreplay 先例）。

## 完工报告（最终回复直接输出）

意图哈希、门实装结论（单测四件读数、红绿两态、cargo test 计数与先在三败披露）、SPEC-006 修订二一句、认证清单、双仓 commit 号、链 verify 对表、reconcile 读数、F 表、越线与误差申报。
