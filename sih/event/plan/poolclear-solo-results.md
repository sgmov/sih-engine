# poolclear-solo 结果档：泊界投影小清批——名册对链、出场件配对、pk-013-exit 处置、席位基线 core_hash 收尾

- 批次：poolclear-solo 单线 solo（重派批，前一执行死于模型请求故障死在开工前零残留）；会话 sess-zcode-260904-poolclear（lease 会话号 4c807de2ead52808，锁操作全显式）
- 队形：单线形亲写零子代理；全 F 项走确定性通道，facet 零采样
- 泊界链事件：intent 一笔加认证十二笔（含结果档检词一笔），历史 trail 零改写只追加，停泊事件零新增（本批零入泊零出泊，只做投影对齐）

## 意图哈希

ask3 记录 sha256 c44412d3f728734dabc8a02cb4c7a5e09371fbbb92997f7583e827e35a3df409（intent 事件 51cab13d55c8d4f301a09f9d749a48a19d71676413e9ff7ec94da62765ec81bb，三锚程序切片 07-on-assay:61 映照、06-on-canon:185 力度、08-on-settle:108 应而不藏，双门 exit 0/0 status ok）。

## F 表（完成度）

- F-1 名册在泊段与全链 enter/exit 配对直数对齐：过（实跑复核为准，读数差额与线属实查照录如下）。全链直数脚本复算（全十三日 trail 逐事件 enter/exit 按 entry_id 序配对，剔 pk023spec splinter 重放件）：在泊十一项即 pk-016/026/039/042/044/045/046/047/048/049/052，非任务包读数十项——差额 pk-052 系 mathscan-solo 批今日入泊（事件 42d232ce，链 169 行，主会验收时点 167 笔之后上链），其事件自注「名册投影行与 state/parking/materials 副本交后批按链补齐」即本批为指定承接批。实查三笔：其一 pk-026 与 pk-042 已在工具线名册在泊投影（tools PARKING-v1.md「当前在泊两项即 pk-042……pk-026……」，材料 sih-tools/parking/materials/ 各有 enter 件且 path 字段锚 PARKING-v1.md 即工具线，承 mathpipe-a1-solo 跨线归位与 pendsweep 补记在案），补行位已在所属线名册成立，引擎册按两界按线分泊铁律不重投影（若引擎册亦补两行即跨册双投影与 path 锚行属不合，facepark F-5 pk-013-exit 归位判读先例同族）；其二 pk-037 行已系出泊照录形（pk037impl-solo 批 a78f08d 落位），实取出泊事件 b68b178a 与行内哈希逐字对上、promoted 与裁定文一致，本批零改；其三 pk-052 行照链补入引擎册（逐字段抄停泊事件即 title 与两案内容与出泊条件与入泊事件 42d232ce 与 2026-09-04 入泊与 ttl 30 天）并在泊计数八改九、行序随档即升序插于 pk-049 后，历史段相应补记实查为零（pk-037/050/051 出泊行均已在位，引擎线无未投影出泊件）。名册过化格零改、核阅 exit 0、检词零违例三门。
- F-2 pk-051-exit.json 出场件补位：过。逐字段抄链上出泊事件 f6b3293e（ruling 与 disposition 逐字节程序断言一致），形态承 pk-050-exit.json 同日出场件先例（action/disposition/entry_id/id/parking/path/ruling/state/ttl_days），ttl 30 与 entered_at 2026-09-04 实取入泊事件 97ee4c20，path 指引擎册承 pk-051.json 既有锚；enter/exit 配对归真即 pk-051.json（state exited，basefix 落位）加本件成对，gqueue 计数面随 enter/exit 链数自洽。
- F-2b pk-052.json 副本落 engine materials（实跑复核新增面，mathscan 交接指定）：过。自 mathscan-solo-materials 逐字节拷入（cmp identical），与链事件 42d232ce 五公共字段（entry_id/title/context/exit_condition/ttl_days）程序断言一致。
- F-3 pk-013-exit.json P101 scrap 缺口处置：过，择二照链补字段。实查：其一 selector 谓词种册固定（EVALUATORS 五 kind 注册表，pack.py 装载时 kind 白名单校验），schema_required 无条件豁免能力，routes.toml 数据面无法表达「出场件形态豁 P101」即包豁免路线须改 selector 源码撞禁区，如实申报择据；其二照链补字段链上有据——path 指引擎册（facepark F-5 判归引擎侧在案）、state exited（pk-050-exit/pk-037-exit 出场件形态族同形）、ttl_days 90 与 entered_at 2026-08-25 实取入泊事件 1696662e、exited_at 2026-09-02 实取出泊事件 c05087fa；既有五字段（action/entry_id/disposition/ruling/id）逐字节保留程序断言过。路由实查：批前 scrap_track（P101 fail）转批后 mainline（P101/P102/P103 全过）。selector 源码零触碰、routes.toml 零写入，F-5 路由判定面变化只及本出场件自身材料，在泊路由零变。
- F-4 席位基线 core_hash 字段收尾：过，TDD 先红后绿。实装即 temp_probe score 增 core_hash 可选参（正身报告 identity.core_hash 透传，非六十四位十六进制硬闸承 pk-035 同款，缺席即字段不入账承 idcore-solo R5「任一缺席回退现行为」消费语义）与 CLI --identity-report 分支读 identity.core_hash 透传（--identity-hash 直给形态无核哈希源即字段缺席如实回退）；测试即 test_temp_probe_identity.py 新增三测（携带入基线、缺席回放安全、畸哈希拒）先红 2 failed 后绿 6/6；全测试族批前（stash 基线）479 绿 6 红、批后 482 绿 6 红，失败集逐条 diff identical 即六红均 ng_assembler 存量缺夹具与本批零关，零回归；存量标定基线与真账本零触碰（测试全走 tmp LEDGER，真账本零写），下回合标定自然携带新字段；CLI 接线以本批正身件进程内验证（core_hash 82f460c2 透传入基线为真）。
- F-5 写入仅 allow 与判变申报：过。写入面全在 allow（十一 exclusive 锁面加 trail 与 meter 两 append 短持面）；F-3 未走包豁免即 routes.toml 零写入、selector 谓词数据与源码零触碰，路由判定面变化实测仅 pk-013-exit 自身 scrap 转 mainline 与两新件入 mainline，工具线 pk-042 siding 读数批前批后同态，在泊路由判定零变达成。

## 认证清单（链上 append 逐笔，全 meter 包裹，2026-09-04 链）

| # | 报告件 | 实跑退出码 | 链上事件哈希 |
|---|---|---|---|
| 1 | ask3-gate1-scrut.json（scrutinator ask3 包） | 0 | 4bd059e414b41b9c |
| 2 | ask3-poolclear-solo-validation.json（ask3repeater） | 0 | 0a6325dd2451206f |
| 3 | poolclear-elicit-digest.json | 0 | 3b7be0b1023d9903 |
| 4 | poolclear-fmt-roster.json | 0 | 187286d07d16e21d |
| 5 | poolclear-scr-des001-roster-worktree.json（工地路径域外 exit-2 记录） | 2 | 1a8f205f8100cb9b |
| 6 | poolclear-scr-des001-roster-corpuscopy.json（域匹配副本终跑） | 0 | 868bbd8c8b05420d |
| 7 | poolclear-nom-roster.json | 0 | 4a54e2f4411bfc56 |
| 8 | poolclear-nom-calllog.json | 0 | f1edb9c447f417e7 |
| 9 | poolclear-heartbeat-tools.json（批前批后同态 19=18+0+1） | 0 | 4b342672a375d3da |
| 10 | poolclear-heartbeat-engine-before.json（批前 19=18+1+0） | 0 | 274b1f9d7b5d5cae |
| 11 | poolclear-heartbeat-engine-after.json（工地批后 21=21+0+0） | 0 | 53ae14ce745d34c8 |

结果档自身检词一件（poolclear-nom-results.json）认证随后上链不在本表（自指限制，越线节申报）；settle cert 取末表笔前八位 53ae14ce。

## 心跳读数对表（工具线 selector，参照时间 2026-09-04）

- 工具线（主树）：total 19（mainline 18、scrap_track 0、siding 1 即 pk-042 P102 无 parking 块 fail-closed 承 facepark F-7 已知形态），批前批后同态零变化，告警空。
- 引擎线主树批前：total 19（mainline 18、scrap_track 1 即 pk-013-exit P101、siding 0），告警空。
- 引擎线工地批后：total 21（mainline 21、scrap_track 0、siding 0），告警空——pk-013-exit 转 mainline 加 pk-051-exit 与 pk-052 两新件 mainline；主树归并后读数待 close 归并对表节回填。

## 越线与误差申报

1. 直数差额：任务包与 dispatch 载在泊十项（无 pk-052），实跑复核十一项——pk-052 系主会验收时点（链 167 笔）之后由 mathscan-solo 收口批入泊（链 169 行），任务包 F-1 自带「实跑复核为准」即按十一项执行，名册计数相应记九项（引擎线）加工具线两项合计对上链面十一项。
2. F-1 补行位判变：pk-026 与 pk-042 未补入引擎册，实查两行已在工具线名册在泊投影成立（跨线归位先例与材料 path 字段锚属工具线），引擎册补行即构成跨册双投影违两界按线分泊铁律，故 F-1 的补行动作落位于 pk-052（真缺口）；pk-037 行改出泊照录经实查已由 pk037impl-solo 落位且哈希逐字对上，本批零改。两处均系任务包作成时点早于并行批落位所生陈旧，非本批规避。
3. 锁面扩展申报：任务包锁路径全集外并入 allow——sih-tools/facet/tests/test_temp_probe_identity.py（F-4 测试正典居 facet/tests/ 与任务包「facet/probes/（temp_probe 与测试）」括注同义，据实查即既有 temp_probe 测试同居 tests/）、sih-tools/scribe/reports/ 与 sih-tools/meter/counts/（BATCH-FACE 链步骤既定写入位，facepark/goldlim 同款）、identity 报告一件、双工地目录、facet/CALL-LOG.md（CALL-LOG 留痕强制位）。lease open 并入任务包请求写入节原文七条。
4. 核阅首跑红转绿照录：域匹配副本首跑 exit 1 两笔 C006 全角括号违例即本批新 pk-052 行照抄链上出泊条件的「（gauge 日扫旁）」与「（批尾 close 前扫）」括号叙事，按 basefix 收口回填「名册全角括号逐处重写」先例改写为顿号形后复跑 exit 0 零 findings，链事件语义零损。
5. des-001 于名册的域匹配路径副本法（承 mathpipe-a1-solo 与 pendsweep-solo 与 facepark 先例）：工地直跑路径域外 exit-2 如实记并认证（表笔 5），副本实跑终态 exit 0（表笔 6），副本用后即删不入版控。
6. 叩问五轻信号（投影/出场件/泊位/直数/补行）以判读处置全数在档不登记——nomenclator 属本批禁区零碰，登记留待后批；digest passed 全信号 covered。
7. identity/reports 不入版控照禁区；主树既有存量 untracked（mathscan-solo-materials/dispatch.md 等他批件）零收编，本批随批入版控只取任务包与 dispatch 两件自批。
8. meter run 包裹命令输出经 2>/dev/null 对治；全程禁管道掩退出码，唯一一次 tee 管道（ask3repeater 首跑）因 zsh PIPESTATUS 未计退出码即原样重跑直取退出码 0 为准，首跑输出与重跑逐字同。
9. 数学仓零碰；retriever 与 identity 与 tally 与 gauge 与 formatter 与 meter 与 nomenclator 与 parser 与 cascade 工具本体零碰（调用面除外）；历史 trail 零改写；停泊事件零新增。

## 冲突样本节

零。mathscan-solo（dc38bb4c）已收口（537da93）后开工，全程无撞锁无让位无 trail 竞态；十一 exclusive 锁一次取清，append 短持面十三轮即取即放零争用。

## 收口读数（收约后回填）

待收约后回填。
