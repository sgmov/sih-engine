# acceptclose-solo 结果档：主窗验收收口批

> 承接：任务包 acceptclose-solo.md 与用户 2026-09-08 令三腿。单件：CALL-LOG 族七件收编＋泊界名册对链复检修正＋closeguard 六笔计账归置。
> 队形单线形 solo，日期 2026-09-08，会话 sess-zcode-260908-fork3-acceptclose（session_id 38a10c13ab15318b）。

## 意图锚定

- 意图事件：intent_refined（event_hash 前 8 `67cd06ea`）
- record：sih-tools/scribe/reports/2026-09-08-ask3-acceptclose-solo-record.json
- validation：sih-tools/scribe/reports/2026-09-08-ask3-acceptclose-solo-validation.json（status ok anchor_count 3）
- 三锚引文程序切片（08-on-settle.md L110、07-on-assay.md L55、01-ontology-of-names.md L18）于生成器 make_ask3_acceptclose-solo.py，禁手打承契约。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0；ask3repeater exit 0 status ok anchor_count 3。
- 叩问：轻信号四件即收编、计账归置、名册投影、归属窗，digest passed covered 4。
- 正身：identity verify anomalies 0（identity.hash 6734a4a9）。
- 启动节律：判据扫（C4 attribution 已转 in_flight 即 constclear2 线开工，机械召回生效）＋例行读数三维落链＋watch exit 1 无主七件（即本批归属对象）＋两线心跳零告警。
- 协调面：活跃会话仅 confpreempt-solo 无锁；BATCH-FACE 与 AGENTS.md 漂移检定零漂（上批全读后零提交）。

## 腿一读数：CALL-LOG 族七件收编

- 收编前哈希快照在批材料（acc-before），七件主树态复制入工具仓工地，逐件 cmp 七件全 IDENTICAL 即零改写；calls.ndjson 权威腿 753 行现状字节照收（收编前 SHA256 前缀 507cda8ad22ba616），禁整文件重写红线达成。
- 收编通道即租约 allow 面十五路径全锁（九 CALL-LOG 与账本与 PARKING 面与批件），随批 settle 提交归属版控。

## 腿二读数：泊界名册对链复检

- 链面全清单：09-05 至 09-08 泊事件五十六笔（entered 二十七笔、exited 二十九笔）逐笔提取，对表材料落批 materials（parking-events-0905-0908 与事件哈希清单）。
- 漂移修正两形：改判五件转历史住户即 pk-072（6f200dc2）与 pk-046（4672e40e）与 pk-044（e5327aa7）与 pk-073（c8a09b5e）与 pk-053（41fb339c）各自出泊 promoted 承接批照录；补记八件漏记即 pk-062/063/064/065/066/069（archpark-genpark 系）与 pk-077 与 pk-078（带 gate 远景件）各携入泊事件哈希。
- 修正后在泊十五项名逐件在册（doc/governance/PARKING-v1.md 在泊名录节首段改写，原快照散文保留作历史沿革，链为准条款照录）。
- 主线计数语义：出泊件材料经 P101 计 mainline 属包面既有行为，语义注已落心跳与结算节，三选项呈报候人裁即甲维持现状／乙 state 改 exited 加谓词分轨／丙出泊材料移档，缺省落注不改谓词，谓词包零改动。

## 腿三读数：closeguard 计账归置

- 六笔 pre-close 提交逐笔 bypass 登记 exit 0 全过：c5c0852（critsweep-solo）与 f5073ab（reroute-solo）与 1d65ad3（sitruling-solo）与 3efff50（pkexits2-solo）与 228254c（pk063split-solo）与 0593a6d（constclear2-solo），全部引擎仓归属，理由栏载批名与事述。
- unbypassed 计数注：批前基线 68 系本日早间读数，其间并行批（pkexits2 与 constclear2 与 pk063split）归并使对账区间扩大，前后差数不可单归因；F-3 主证即六笔登记回执与 bypass 台账行直查（engine bypass 计数 141），计数漂移如实申报不虚构降数。

## 越线与误差申报

- 15 锁首轮一笔 exit 1（并发竞态），复扫零失败全在位，红证在档。
- 核阅 des-001 对工地路径形 PARKING-v1.md 报域外 exit 2：根因即 des-001 域 include 按根相对 glob（sih-engine/doc/**）判定，工地路径形（worktrees/...）不命中——管线调用形新坑，正形为归并后主树路径复验（本批收约补笔含主树核阅复验读数），红证与勘误候选如实记档。
- 名册原「在泊十一项」计数与实态十五项差四，系多批漏记累积（六件 09-06 入泊未投影与两件改判对冲），逐笔修正非整表重写。
- 其余误差零申报。

## 管线读数

- 化格：任务包与结果档与 PARKING-v1 过 packs/general-v1（工程地形式全 exit 0）。
- 核阅：des-001 工地路径形 exit 2 即域判定 glob 不命中（见越线申报），主树归并后复验读数随收约补笔。
- 检词：nomenclator packs/core 三件 exit 0 零违例。
- checkcite：任务包与结果档合并单件扫描，读数随回填。

## 认证清单

ask3 记录、验证件、正身件（JSON 三件 scribe append）＋内容哈希清单件（PARKING-v1 修正稿与结果档与任务包与名册对表材料，md 与工件 ReportNotJson 先例承 anchorskill）。读数随回填。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 七件收编 | 数据治理 | 收编后七件 git 净，calls.ndjson 零重写哈希一致 | 待收约回填（收编副本七件 cmp 全 IDENTICAL 在档） |
| F-2 名册对链 | 数据治理 | 泊事件全清单与修正后名册一致，语义三选项呈报谓词零改动 | 通过（五改判八补记十五项在册，注落心跳节） |
| F-3 计账归置 | 治理 | 六笔 sha bypass 在册 | 通过（六回执 exit 0，台账直查为主证） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（写入面即七件 CALL-LOG 族与 bypass 台账经命令与 PARKING 与批件与链与报告与工地） |
| F-5 链面全绿 | 治理 | 双仓 settle 在档，close 零失败，verify valid，reconcile unrouted 零新增 | 待收约回填 |

## 结算读数

- 双仓 settle：engine 工地归并 7767b4c 与 tools 工地归并 48bdcec4（三查过）；cert 取 b550e4af 即 ask3 记录认证前八位。
- 认证实录：ask3 记录 b550e4af、验证件 8ec8a266、正身件 39337ce4、checkcite 件 e1b68cad、内容哈希清单件 98711391（5 路径 sha256）五笔在链。
- 放锁收约：15 锁 unlock 一次全过（正身件 unlock 毕后让位删除，零重蹈）；close 一次过零 bypass 旗标即七件 CALL-LOG 面已入本批锁面豁免——归属窗语义达成，会话 38a10c13ab15318b revoked、双仓归并、工地清除。
- F-1 终读：收编后七件 git status 净、calls.ndjson SHA256 收编前后一致即 507cda8ad22ba616、753 行零重写。
- F-2 终读：主树核阅复验首跑 exit 1 出违例二十笔即本批插入段括号十一笔加 closegate 历史散文遗留九笔（含 ∪ 两笔与 C009 日期邻接误报一笔即「文规窗口 2026-09-07」正则误伤），逐笔归置后复跑核阅 exit 0 全绿；修正笔经 --no-verify 加 bypass 登记入版控，红证即两轮 findings 全文在批材料。
- 链 verify：valid，events 102。
- reconcile：双仓 unrouted 零；unbypassed 69 系对账区间扩大所致与六笔登记不可单归因的注记在 F-3 主证外如实保留；cert_missing 存量与批前同数。
- 收约补笔：F 表终态与结算读数即本笔，经 --no-verify 加 lease bypass 登记通道入版控（anchorskill 与 critsweep 与 sitruling 与 pk063split 先例同形）。
