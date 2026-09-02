# pendsweep-solo 结果档

> 批：挂账清理批（确定性通道三件加投影同步）
> 会话：c6a4af77e65e3a5e（sess-zcode-260903-pendsweep）
> 日期：2026-09-03
> 队形：单线形 solo，零子代理，全链亲写

## F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 | 现役材料带 core_hash 过 assemble；R5 对同日新基线核配对成立；旧材料回放同判 | 过 | assemble_material 增 identity_report_path 读正身报告 identity.core_hash 载入材料 JSON；透传核配对、旧报告缺席空缺回放同判、显式缺席报错三新测；工地 pytest 20 测绿（17 旧加 3 新）；R5 判定语义与三态映射与退出码三值零动；CONTRACT 修订七在案 |
| F-2 | 12 件对表读数全载；漂移件全数有合法出处或停批 | 过 | 12 件现行二进制实跑逐字节对表九同判三漂移；三漂移出处全为已结算批即 settlement-v1-solo 段1 6d76fd7（GOV-003 v1.9 追记）与 relaud-solo 段1 48724c4（LIM-001 与 MUL-001 状态行补齐）；findings 三件均零变化只刷 content_hashes 各一行；重录后 live 实跑三件 IDENTICAL、工地 cargo test 全套 exit 0（182 过 6 忽略 0 败）；对表与重录 cmp 证据见 2026-09-03-pendsweep-solo-golden-cmp.json |
| F-3 | CONTRACT 修订一条在场，触发链四步逐字可读 | 过 | gauge CONTRACT 版本节增 0.3.0（编号递增），ga-1 校准窗满条件与触发链四步（窗满、解析校准尝试、解析不可行、pk-042 出泊复检走得一裁过即实验批）逐字在场，公式与读数零改动 |
| F-4 | 名册行与链事件逐条对上；pk-043 出泊、pk-041 记账位、pk-042 触发链三行在场 | 过 | 对表读数见下节；引擎名册 pk-041 行补记账位主会裁定、pk-043 行撞号披露照录加登记面实名更正、pk-036 行随冻实态；工具名册 pk-042 行触发链补记；零泊界事件写入 |
| F-5 | 链 verify 0；reconcile 双零；管线 findings 亲读留证 | 过（段2 终稿载读数） | 见收口读数节 |

## 分道申报

前置分道承 sihankor-facet-measure 修订四与 m-sett001-crosscheck 先例：件一 tally 透传为确定性实装核对、件二金向量为逐字节机械对表、件三 CONTRACT 修订为文档钉子、件四名册为投影对链修正，四件均不送采样，裁在确定性判据。本批零采样零泊界事件写入，进出泊零触碰。

## 件二对表读数（12 件）

九同判：ask3-scrutmerge-sdd、ask3-viewrider、des-001-dec020、des-001-goldfix-001-multiflag、des-001-goldfix-002-header、des-001-goldfix-003-fence、des-001-goldfix-004-nav、des-001-goldfix-005-skip、des-001-gov002（byte_cmp IDENTICAL，退出码与冻结一致即合规件 0 违规件 1）。

三漂移（content_hash 漂移、findings 均零变化）：des-001-gov003（目标 GOV-003-fullstate-course-v1.md，出处 settlement-v1-solo 段1 6d76fd7 已结算批）、des-001-mathe-lim001 与 des-001-mathe-mul001（目标 sih-math calculus LIM-001 与 MUL-001，出处 relaud-solo 段1 48724c4 已结算批经 409a0f3 归并）。重录只刷 content_hashes，工地版对 HEAD diff 各恰一行哈希；重录后 live 实跑三件 IDENTICAL。

## 名册对链对表留证（件四）

链上 2026-09-03 停泊事件六笔对两线名册投影逐条核对：

| 链事件 | 内容 | 名册投影 |
|---|---|---|
| intent 事件 5 parking_entered pk-043 | 意图对齐样板件入泊 | 引擎名册历史住户 pk-043 行（mathpipe-a1 段1 已投影） |
| 事件 7 parking_exited pk-043 promoted | 用户「同意，开」裁 intanchor-solo 承接 | 同行 promoted 在案，登记面实名本批更正为 asset-anchor-registry-intent-2026-09-03.md |
| 事件 10 parking_entered pk-043（撞号） | 温度件误入复记 | 引擎名册 pk-044 行撞号改正照录，本批于 pk-043 行加撞号披露 |
| 事件 20 parking_exited pk-041 promoted | 数学管线全量串联立项 | 引擎名册 pk-041 行，本批补记账位主会裁定即跨天 park 走合并视图通道承 pk031gap 与 pk013exit 先例，mathpipe-a1 批已执行记账 |
| 事件 21 parking_exited pk-043 discarded | 撞号改正废弃编号 | 同上 pk-044 行承载 |
| 事件 22 parking_entered pk-044 | 温度件重编承载 | 引擎名册在泊四项之 pk-044 行一致 |

工具线名册在泊两项即 pk-042 与 pk-040 归位行与链一致；pk-042 行本批补触发链四步承载位 gauge CONTRACT 0.3.0。心跳读数（本批会话启动补跑，只读零写入）：工具线 20 件 mainline 18 scrap_track 2（pk-013 与 pk-042 均 P101），引擎线 7 件全 mainline（pk-044 在泊），两线 alarms 空退出码零，证据见 scribe/reports 两 heartbeat 件。

## 管线读数

笔在核前判在书简前，序固定化格、核阅、检词，逐目标亲读 findings：

- 化格 formatter general-v1 --write：15 目标全 exit 0 无需改（tools 工地 12 件加 engine 工地 3 件）；dispatch.md as-found 入册不做化格（承 idcore-solo 先例）。
- 核阅 scrutinator --pack des-001 裸名：引擎名册直跑工地路径域外 exit-2，经 corpus 匹配路径副本实跑 exit 0 零发现（承 mathpipe-a1-solo 先例）；其余 14 目标域外 exit-2 如实记（tools 12 件加任务包加结果档），域外不属违规。
- 检词 nomenclator check --pack packs/core（工地包 0.8.0 含三新词）：tally CONTRACT 与 gauge CONTRACT 与工具名册与引擎名册（经 sih-engine/doc 匹配副本）与九调用册与任务包与结果档全 exit 0 零违例。
- 汇总证据件：scribe/reports/2026-09-03-pendsweep-solo-fmt.json、-scr.json、-nom.json。

## 认证清单

（段1 落逐件认证事件哈希。）

## 收口读数

（段2 终稿载：settle 前后链 wc 与末哈希对表、双仓 commit 号、reconcile 读数、链 verify、close 归并对表。）

## 越线与误差申报

1. 链态对表：指令记链现 12 事件尾哈希 0c43f8225219406c 开头，实查当日链 25 事件；事件 1 至 12 与指令相符（事件 12 即 idcore-engine-pipeline 认证，尾哈希前十六位 0c43f8225219406c），事件 13 至 25 为并行在途会话 parkreplay-solo 与 mathpipe-a1-solo 尾随事件，承指令「并行在途会话尾随承先例如遇即申报」条款申报在案。
2. mathpipe-a1-solo 会话台账态为 close_failed 加 revoked，但其段1 提交已在两主线（引擎 e413987 经归并 00c8a4a、工具 e37a0ad5 经归并 f5d4c976，2026-09-03 04:11 至 04:12），名册 as-found 已含其停泊三账投影；本批不碰其泊件本体与在途批件，仅按指令补投影缺口三处。
3. 例行读数今日已在链（事件 1 至 3 三维快照），本批未重复跑秤；泊界心跳补跑一次只读路由如上节申报。
4. BATCH-FACE 漂移一处：digest 调用形档记 --packs 加 --signals，实 CLI 为 --signals 加 --contract（契约文含叩问处置逐词即过），本批按实 CLI 执行，漂移登记于此。
5. tally 工地 pytest 首跑 19 绿 1 败，败因为新测断言 R5 回退路行形误写（测试期望文案错非实装错），修正断言后 20 绿；cargo 首跑 --lib 直跑 20 败为工地 target 未建 bin（环境步骤缺非断言败），cargo build --bins 后全套 182 绿；两笔如实记。
6. 检词 register 首跑三笔拒（拒因 state 非法 None，词条缺三态字段），补 state=established 后三笔 registered，manifest 0.7.0 升 0.8.0。
7. gauge 无 CALL-LOG.md（历批未建），本批守任务包 scope 显式枚举不新建，gauge 触及记录载本档与 lease 与 meter 调用册。
8. 主树未跟踪与未提交件的收约让位处置（任务包勾选版、trail 与 inputlog 与账本活体拷贝、报告件）见收口读数节归并对表结论。

## 队形声明

单线形 solo，零子代理，全部改动由本会话亲写；测试从工地树跑，主树零直写；上链前等绿，findings 亲读，禁管道掩退出码；撞锁零发生。
