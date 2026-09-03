# facepark-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 facepark-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/facepark-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。用户实验令：跟数学仓零关系的小活得一裁，过了就收口——三件全走确定性通道（修订四），facet 零采样，机械门即实跑核验加双跑留证，过了即按全序收口。

## 件一：BATCH-FACE 两处漂移修（F-1、F-2）

- 落笔前先实跑定格事实：`cd /Users/moc/workspaces/SiHankor/sih-tools && uv run --project . elicit digest --help` 与一次最小真实调用（按 --help 实形拼参）确认 digest 实形；`uv run --project . meter --help` 与 `uv run --project . meter --quiet run -- true` 探针确认 meter 0.2.0 有无 --quiet。pendsweep-solo 申报的"digest 实形 --signals + --contract、meter 无 --quiet"是线索不是结论。
- 按实跑结果修 BATCH-FACE.md 四处：digest 段命令块（L63 起）、L81 坑位行、L288 meter 速查行、L294 --quiet 清单行。修前修后各留一次实跑输出证（路径记结果档）。语义只许向事实对齐，不许顺手改写无关行。

## 件二：泊材料对表（F-3、F-4、F-5、F-7）

- 三件 enter json 逐字段抄链：先 grep 链上停泊事件（sih-engine/sih/event/trail/2026-09-01.ndjson 与 2026-09-02.ndjson 与 2026-09-03.ndjson 中 event_type parking_entered 且 entry_id 为 pk-037、pk-039、pk-045，事件号 35ecddff、684cf410、c75f7401），details 的 title 与 exit_condition 逐字入档，entered_at 按东八区日界（pk-037 与 pk-039 记 2026-09-02，pk-045 记 2026-09-03），ttl 对名册行（pk-037/pk-039 无 ttl 限即不设或随 pk-017 体例，以名册行为准），path 填 doc/governance/PARKING-v1.md，state parked。形态逐字段镜像 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/parking/materials/pk-017.json。
- pk-042.json：先 cat 原件留底，对照 P101（id、path、state 三字段必载）与 pk-017.json 良形补齐，语义零改。
- pk-013-exit.json：cat 原件，grep 链上 pk-013 出泊事件核内容；归位判读按证据（同族 exit json 体例、行属），证据足则移位并申报，不足则原位不动如实记录判读。
- 收尾双线心跳复跑（照 AGENTS 泊界心跳公式，两目录各一次）留存证，对表 F-7 三点；告警如实转述。

## 件三：pk-046 OTel 入泊（F-6）

- 引擎 scribe park 一笔（sih-engine/target/debug/scribe，调用形照 BATCH-FACE scribe 段，经 meter 包裹，--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-03.ndjson 绝对路径），条目内容照任务包 F-6 原文，ttl 30 天。
- 引擎名册 doc/governance/PARKING-v1.md 当前在泊行五改六：pk-046 一句界定加出泊条件加 ttl 加所入泊事件哈希，措辞体例对齐行内既有条目；此件走 worktree 构造。
- materials json pk-046.json 同 F-3 形态。行文零外链，OpenTelemetry 全称。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片：06-on-canon 损补、07-on-assay 映照、08-on-settle 应而不藏；禁手打）→ 双门（scrutinator --pack packs/ask3 必须 0 加 ask3repeater --root 绝对必须 0）→ elicit check --packs sih-tools/nomenclator/packs/core --words OpenTelemetry --words GenAI --words gen_ai --words 泊材料 加 digest passed（新词过不了按检词流程登记或改写，词债不过夜；信号落任务包叩问处置节）→ 正身（reports/2026-09-03-facepark-identity.json 不入册）→ lease open --package facepark-solo --repo 两仓绝对路径 --root /Users/moc/workspaces/SiHankor（一切锁操作显式 --session sess-zcode-260903-facepark；撞锁即停批报告）→ 按任务包请求写入节取锁 → meter 包裹引擎 scribe intent 上链（--trail 2026-09-03 绝对；开工时 wc -l 与末哈希对表留证，在途尾随承先例并申报）→ inputlog 补录一笔（seq 递增，sess-zcode-260903-acceptor，逐字）：`上轮报告的三个去处：OTel 记泊界候选。pk-037/pk-039 出泊裁（等 mathpipe 或现在裁意向，你定）我也忘记是什么了？现在就能跑、跟数学仓零关系的小活得一裁，过了就收口。`（note 即台面小清与 OTel 入泊令）→ 工地施工：文档构造（名册、BATCH-FACE、任务包、dispatch、完成档、结果档）在 worktrees/sih-engine/facepark-solo 与 worktrees/sih-tools/facepark-solo；泊材料 json 属状态面在主树落盘但逐字段链上有据并在结果档逐件申报；主树其余零直写 → 管线（化格先行、核阅 des-001 于引擎名册必须 0、域外 exit-2 如实记、检词 core 零违例、findings 亲读禁管道掩退出码）→ 认证逐件 meter 包裹 append → 双仓 settle --seq 1 --cert 链上哈希前八位 → 放锁 → close（主树碰撞备份让位归并对表法 diff identical）→ reconcile 双仓四类双零 → 当日链 verify → 调用册留痕（scribe、formatter、scrutinator、nomenclator、selector、lease、identity、elicit、meter 触及者各一行）→ 结果档 F 表写全 → 全部产物含链尾随批入版控。

## 红线

出泊裁零执行（pk-037/039/042 去向归人节点）；泊界链事件只增 pk-046 一笔；pk-041/043 出泊账已在链零重放；历史 trail 零改写只追加；selector 源码与 parking 包零改动；gauge CONTRACT 零碰；引擎组件源码零碰；金向量零碰；AGENTS.md 零碰；mathpipe 在飞件零碰；泊材料 json 禁删禁自造字段禁外链；identity/reports 与既有存量 untracked 零收编；守卫在位严禁 plain git commit 直提；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即停批。

## 完工报告（最终回复直接输出）

意图哈希；件一实跑定格结论（digest 实形与 meter 有无 --quiet，各一处探针输出摘要）与四处修后表述；件二三件 enter json 落盘读数与 pk-042 整形前后 diff 摘要与 pk-013 归位判读结论；件三入泊事件哈希与名册行摘要；心跳双线前后读数对照（F-7 三点逐条）；认证清单；双仓 commit 号；链 verify 前后 wc -l 与末哈希对表；reconcile 读数；F 表逐条；越线与误差申报（含在途尾随件数）。
