# facepark-solo 结果档：台面小清批——BATCH-FACE 漂移修、泊材料对表、OTel 入泊候选

- 批次：facepark-solo 单线 solo；会话 sess-zcode-260903-facepark（lease 会话号 0aa9b5cecdbcaa34，锁操作全显式）
- 队形：单线形亲写零子代理；三件全走确定性通道（修订四，承 pendsweep-solo 先例），facet 零采样
- 泊界链事件：只增 pk-046 入泊一笔（99b85acc），出泊裁零执行，历史 trail 零改写只追加

## 意图哈希

ask3 记录 sha256 999df6d529a38c01a7f084a90edfbc19aeeddf91daf0fb1234de4eb692206cc5（intent 事件 4e553df7557d228854a8bd14096daedab9a9fdaca35622d78c803104aeca58b4，三锚程序切片 06-on-canon:73 损补、07-on-assay:61 映照、08-on-settle:108 应而不藏，双门 exit 0/0）。

## F 表

- F-1 BATCH-FACE digest 段修：过。实跑定格 elicit digest --help 实形为 `--signals SIGNALS --contract CONTRACT [--quiet]`，--contract 必填，无 --packs；修前原形（含 --packs 缺 --contract）实跑 exit 2 报 `the following arguments are required: --contract`，修后新形实跑 exit 0 `{"digest": "passed", "covered": 4}`。证据 scribe/reports/2026-09-03-facepark-solo-digest-before.json 与 -after.json。修一处（L63 起命令块）。
- F-2 meter --quiet 三处修：过。meter 0.2.0 根位与子命令位探针均报 `meter: error: unrecognized arguments: --quiet` exit 2（pendsweep-solo 申报证实），meter 确无 --quiet。修两处：L288 坑位行改「meter 无 --quiet」与 2>/dev/null 对治形，L294 清单行摘除 meter 0.2.0；L81 坑位行判零改——该行 --quiet 指身份核验 identity verify 实有旗标（--help 在证），表述与实跑一致，pendsweep 申报判 L81 为 meter 漂移一处不成立，如实申报。证据 -meter-before.json 与 -meter-after.json。
- F-3 引擎泊材料补三件：过。pk-037.json、pk-039.json、pk-045.json 落 sih/state/parking/materials/，逐字段抄链上停泊事件（title 与 exit_condition 逐字），形态 pk-041/pk-044 事件镜像字段序加 pk-017 schema 骨架（id/path/state/parking/anchors/requested_writes）。链事件号：pk-037 即 35ecddff（哈希 2366a291）、pk-039 即 4071befb（哈希 e98cfdac）、pk-045 即 c75f7401（哈希 e46ea82a）。entered_at 东八区：pk-037/pk-039 记 2026-09-02，pk-045 记 2026-09-03。ttl 以链上 ttl_days=30 为准（名册行 037/039 未载 ttl，链为准），path 指名册，state parked，禁自造字段零外链。
- F-4 pk-042 整形：过。原件缺 path 与 state 两字段（P101 三字段缺二）走 scrap_track；整形只补 "path": "PARKING-v1.md" 与 "state": "parked"，既有字段语义零改逐字保留，落盘转齐行形。修后工具侧心跳 pk-042 告别 scrap_track。
- F-5 pk-013-exit 归位判读：过，判归位引擎侧，移位执行。判读过程：其一内容与链上 pk-013 出泊事件（63e40685，promoted）逐字对上；其二行属——pk-013 系引擎线住户（引擎册有出泊行、引擎材料有 enter 件 path 指引擎册），工具侧 exit 同族八件全带 path "PARKING-v1.md" 属工具线，pk-013-exit 与同族行属不合；其三引擎线 exit 体例先例 pk-043.json 居引擎材料。证据足即移位：sih-tools/parking/materials/pk-013-exit.json 原位移入 sih-engine/sih/state/parking/materials/，内容与 tools HEAD 版逐字节 cmp identical，零整形（F-5 只判归位不扩面）。
- F-6 pk-046 入泊：过。三件套齐——引擎 scribe park 一笔（事件 ba844e2a，哈希 99b85acc41cdad8e3191a55bd4491b754a0f09c07a73a6674da9d05928d60842，ttl 30 天）；引擎名册当前在泊行五改六（pk-046 一句界定加出泊条件加 ttl 加入泊事件 99b85acc，措辞对齐行内既有条目，OpenTelemetry 写全称零外链）；materials json pk-046.json 同 F-3 形态（逐字段对链上事件）。park 记录首投被拒 ActionMissing（缺 action 字段，拒绝零留痕），补 "action": "enter" 重投通过，照录。
- F-7 心跳双线复跑对表：过。前读数（批前定格留存证）：工具线 total 20（mainline 18、scrap_track 2 即 pk-013 与 pk-042、siding 0、alarms 空），引擎线 total 7（pk-013/015/016/017/041/043/044 全 mainline、scrap_track 0、siding 0、alarms 空）。后读数：工具线 total 19（mainline 18、scrap_track 0、siding 1 即 pk-042 P102、alarms 空），引擎线 total 12（mainline 11、scrap_track 1、siding 0、alarms 空）。三点逐条：引擎侧 pk-037/pk-039/pk-045/pk-046 四件可见全 mainline；工具侧 pk-042 告别 scrap_track 转 siding（P102 时限谓词无 parking 块不过，承 F-7 预期路由）；告警读数双线空（siding 1 未越 siding_surplus 阈 2，如实转述无告警）。已知局限照任务包登记为观察项不在本批修：已出泊件（pk-013、pk-015、pk-041、pk-043）材料仍在、路由面不区分已出泊；本批新增引擎侧 scrap_track 1 即 pk-013-exit.json（F-5 只归位不整形，缺 path/state 走 P101 scrap，与该观察项同源）。
- F-8 收口：过（见收口读数节，回填于收约后）。

## 件一实跑定格结论（重申）

digest 实形 `elicit digest --signals SIGNALS --contract CONTRACT [--quiet]`，--contract 必填指向载叩问处置的契约文档即 ask3 记录；meter 0.2.0 无 --quiet（根位子命令位双探针 exit 2）；identity verify / formatter / lease 根命令 / elicit check 与 digest / nomenclator register 的 --quiet 实有（L294 保留部分与实跑一致）。

## 件二读数（重申）

三件 enter json 落盘字段全链上有据；pk-042 整形前后 diff 摘要即新增 "path": "PARKING-v1.md" 与 "state": "parked" 两键加落盘齐行形，余逐字节同；pk-013-exit 归位判读结论引擎侧移位，内容零改。

## 认证清单（链上 append 逐笔，全 meter 包裹）

| # | 报告件 | 实跑退出码 | 链上事件哈希 |
|---|---|---|---|
| 1 | ask3-gate1-scrut.json | 0 | 05b40a09c0713acb |
| 2 | ask3-facepark-solo-validation.json | 0 | dda86d45c0996956 |
| 3 | elicit-digest.json | 0 | b535eff7ebd2bce5 |
| 4 | fmt-roster.json | 0 | e3c1b1f6f1ec9380 |
| 5 | fmt-batchface.json | 0 | 04b1f2d737f9889d |
| 6 | scr-des001-roster.json（工地直跑域外 exit-2 记录） | 2 | d131cc66be738d7f |
| 7 | scr-des001-roster-corpuscopy.json（域匹配副本 exit 0） | 0 | eea2e265374ac277 |
| 8 | scr-des001-batchface.json（域外 exit-2 记录） | 2 | 001532e2dc9bf4c7 |
| 9 | nom-roster.json | 0 | 701eb36d769fa3ce |
| 10 | nom-batchface.json | 0 | bf2a055591f25e79 |
| 11 | heartbeat-tools-before.json | 0 | 8953739364056164 |
| 12 | heartbeat-engine-before.json | 0 | 9234d46312b5d578 |
| 13 | heartbeat-tools-after.json | 0 | aae5ad6d2df046fb |
| 14 | heartbeat-engine-after.json | 0 | f8a08b7518d12c1e |
| 15 | digest-before.json（修后改装 JSON 信封，转录逐字） | 2 | 23e29e419ebc31fc |
| 16 | digest-after.json（同上信封） | 0 | ac473ec7c98e6479 |
| 17 | meter-before.json（同上信封） | 2 | 9dea716cdc624f74 |
| 18 | meter-after.json（同上信封） | 0 | eefabea2d693e719 |

signals.ndjson 一件非 json 报告形 append 拒一笔（工具限制零留痕），随批入版控承先例只认证 digest；证据原件转录改装备注在案。末笔 eefabea2 即 settle cert 取前八位。

## 越线与误差申报

1. 在途尾随：开工基线 123 行（末哈希 5920ff70），排队等待期间 goldlim-refreeze-solo（会话 1969c8e43057dd94）收口、mathrefmt2-solo（会话 12c35fa47a54b93f）取共享锁收口，两批向 09-03 链尾随 4 笔（123→127，末哈希 fc186170）；本批 15:19:10 释锁后开工，全程与其零争锁零互写。
2. 排队等待约 54 分钟（14:25 首查在途至 15:19 全清），承 goldlim 先例撞在途锁排队待释，未绕行未抢写。
3. 锁面扩展申报：任务包锁路径全集外、dispatch 机械链明令的必写面并入 allow——inputlog 2026-09-03.ndjson（补录令）、scribe/reports/ 与 meter/counts/ 与各工具 CALL-LOG 九本（BATCH-FACE 链步骤的既定写入位，goldlim/pendsweep 同款）、identity 报告一件、双工地目录、F-5 归位目的地 sih-engine/sih/state/parking/materials/pk-013-exit.json（F-5 判移位后需写，判读在案）。lease open 自动并入任务包请求写入节原文八条。
4. 存量未提交投影修正携带：主树引擎册在批前已有未提交修改（pk-045 行冲突样本账扩为四轮十二类二十件，mtime 14:43，非本批所出，该批 allow 无名册——冲突样本照录）；本批工地名册以其为基底叠五改六，未收编未回退，收约碰撞四步法后 diff 对表申报只含本批两处编辑。
5. 事件号线索勘误：dispatch 载 pk-039 停泊事件号 684cf410，链上实查 684cf410 系 intent_refined 事件（其 event_hash 即 pk-039 停泊事件 prev_hash），pk-039 parking_entered 实为 4071befb；任务包已声明线索以实跑为准，按实取 4071befb。
6. park 记录首投 ActionMissing 拒一笔（零留痕），补 action 字段重投过。
7. des-001 于名册的域匹配路径副本法（承 mathpipe-a1-solo 与 pendsweep-solo 先例）：工地直跑路径域外 exit-2 如实记，副本实跑 exit 0 findings 零，副本用后即删不入版控；BATCH-FACE 与结果档域外 exit-2 如实记不属违规。
8. identity/reports 不入版控照禁区；主树既有存量 untracked 零收编。
9. meter run 包裹的命令输出经 2>/dev/null 对治 uv 日志混入（本批修定的 BATCH-FACE 新形）。

## 收口读数（收约后回填）

- 双仓 settle：engine 56821d8（cert eefabea2，base main@1cf5650）、tools 508caf94（cert eefabea2，base integral-stage-build@15fa4202）。engine settle subject 出现「facepark-solo 段1」前缀重复（lease 机械前缀加本批措辞叠写），信息行 session/cert 完整，照录。
- 放锁：28 锁一次放清零持有。
- close 共五轮：轮一被阻（引擎 merge_diverge 即名册预期合并内容加 trail 活写态）；轮二工具仓收成（35dbe43f 归并删支拆工地）、引擎仓被两件未跟踪件阻（dispatch 与任务包，备份让位）；轮三被阻（trail 主线工作件与分支快照 diverge）；轮四失败（merge 撞 trail 脏面 aborting）；轮五成——主线 trail 以 wip 形预提交两笔（01d4ec7 145 行、6267e27 146 行含 gvec 意图笔，均自然过守卫零 --no-verify）、分支 trail 同文 wip 一笔（321e773）后归并 216f318、分支删支、工地拆除、会话 revoked。
- 四步法对表：名册 diff 恰为五改六加 pk-046 行两处编辑零多余；九本调用册备份与归并结果 identical；pk-042.json 主树未跟踪件移开由归并物化同文；scribe/reports 二十件同文。
- meter counts 抢救：工具仓先收时归并了工地旧计数快照，本批 21 行认证计数被让位覆盖，按 pk-045「跨批调用册行丢失经备份抢救」先例自分装备份逐行回 append（时序瑕疵照录即回 append 行落 gvec 行后，行内自带时间戳可排序）。
- reconcile：sih-tools unrouted 0 与 session_orphan 0 与 unbypassed 0 与 cert_missing 1（即 526e2be entryunique-solo 段2 旧账，批前已存在，零新增达标）；sih-engine 四类全零（bypass 7 系历史已登记账）。双仓 exit 0。
- 链 verify：valid，146 事件，first 94f1dd00，末 6615723f（gvec 意图笔）。批前 123 行（末 5920ff70）→ 完工 146 行：本批净增 20 笔即意图加停泊加认证十八笔中在链十八笔认证加 gvec 与 mathrefmt2 尾随，详见事故节。

## 事故申报：意图笔与 pk-046 首笔停泊事件被在途批 trail 活写覆盖丢失

- 事实：意图笔 4e553df7 与 pk-046 停泊笔 99b85acc 于 15:21-15:22 确证上链（当时 wc 128/129 行且尾事件读数在案），其后认证十八笔于 15:34-15:36 逐笔 append 成功；待 15:31 首笔主线 trail wip 提交（01d4ec7）时链面已无此两笔（该提交 18 insertions 即 127+18 认证），git 全版本（01d4ec7、321e773、6267e27、216f318）grep 皆零。当前链 124-127 为 mathrefmt2 四笔、128-145 为本批十八笔认证直续 fc186170、146 为 gvec 意图笔，verify valid 即现链自洽，但意图与停泊两笔不在其中。
- 判定：属并发在途批（mathrefmt2 收束段或其归并）对共享 trail 面的活写覆盖竞态，承 pk-045 冲突样本「链分叉」与「跨批调用册行丢失」同族；本批认证十八笔因直续 fc186170 而链面自洽未损，损失限于意图与停泊两笔。
- 影响：pk-046 停泊的链上账暂缺，名册行所引 99b85acc 成幽灵哈希，泊界铁律投影与链不符时以链为准即 pk-046 链上暂未在泊；pk-046.json 材料面与名册行文（界定、出泊条件、ttl）不受影响。
- 补救受阻：重追加两笔（append-only 合规）于收约后实跑，scribe 报「链路径在他会话锁下」即 trail 锁在 gvec-method-solo 会话 8e4e8065d863b287 手中；按撞锁即停批红线不绕行，重录挂起。
- 后续动作（归人节点裁时机）：trail 锁释放后以原 park 记录重追加停泊笔并以新哈希改名册行入泊事件 citations、以原 ask3 记录重追加意图笔；或由主会裁令插队协调。本结果档与本报告全程如实留痕。
- 名册行维持已归并文本不二次改写，避免在无链据状态下再动投影；本节即诚实视图。
