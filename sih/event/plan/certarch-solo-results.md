# certarch-solo 结果档：cert_missing 家族账面考古批（补笔五件销账两件，余件机械不可清呈人裁）

> 承接：任务包 certarch-solo.md 与用户 2026-09-08 令即「出一个账面考古批把全家族一次清掉——每件先机械试补（盘上若还有载原始内容哈希的报告件即补笔，标注补录语境）；不可重构的即销账登记（bypass 台账形，事由如实）。要么数学要么登记，账面不留悬空引用。你后台拉子代理做」。
> 队形单线 solo，日期 2026-09-08，会话 sess-zcode-260908-main-certarch（租约会话 3326c23ab0cb450f，双仓）。

## 意图锚定

- 意图事件：intent_refined event_hash 前 8 `e009d789`（event_id 5b7af3dc-7552-4060-80ec-a986ef119763）
- 双门：核阅 ask3 包 exit 0 零违规；ask3repeater exit 0 status ok anchor_count 3；三锚引文程序切片自 sih-philosophy 原文（01-ontology-of-names.md:18 与 08-on-settle.md:110 与 07-on-assay.md:55），生成器 make_ask3_certarch-solo.py 在 scribe/reports 随批
- 叩问：五轻信号即考古与补笔与销账与销账登记与悬空，digest passed covered 5（描述性用词不立名不登记）
- 正身：identity verify anomalies 0（attest，identity.hash 前八 73ba054c）

## 前置读数

- 回锚：五行锚在档，任务锚首行已切本批
- 判据扫（启动节律）：C1 与 C3 与 C4 与 C5 达成、C2 viewline 在飞 gap 3 日、零沉底，degraded 假
- watch 对表：exit 1 无主 16 件即 CALL-LOG 族与 calls.ndjson 候清项（callloghyg 线，与前批同形）呈报不代清
- 泊界心跳：引擎线 mainline 52 侧 2 废 9 告警零；工具线 mainline 24 侧 1 告警零
- 例行读数：三维落链 convergence 9ea2662c（0.5）与 adoption 8c11bb37（0.826087）与 mergeback 5c39ab62（lease 会话在册后同窗落链）
- 在飞面开工读数：活跃会话一即 confpreempt-solo（09326a760119e4ed，2026-09-07 签发）持锁零、双工地在盘；本批 open 异包不撞，取锁七路全过（materials 独占、trail 与双 reports 追加面），confpreempt 面零触碰零代收呈报在案

## 第一节 家族盘点（三仓 reconcile 全量）

| 仓 | cert_missing | session_orphan | 备注 |
|---|---|---|---|
| sih-tools | 5 | 19 | 家族五提交 |
| sih-engine | 3 | 24 | 家族三提交 |
| sih-math | 2 | 10 | 家族两提交 |

家族即 7 个 cert 前缀共 10 个 settle 提交：b15cddcf（chaingreen 双仓）、4ff60c8f（confpreempt tools）、1da6f777（projfix 双仓）、970c8299（hookfix 双仓）、e6880a63（entryunique tools 段2）、81ef1c7a（mathfix2 math 段2）、06c30867（fmtfix math 段2）。任务包已知成员四即 chaingreen、projfix、hookfix、confpreempt 全数命中，另考古出 entryunique 与 mathfix2 与 fmtfix 三员即以当跑实态为准不预设名单的实证。closeguard（764f1eaa）与 mainqm-adjud（35fa009c）两提交载 cert 缺席但会话不在册，归 session_orphan 类即跨批存量非本家族。

## 第二节 逐件考古与补笔（五件）

考古面六路：engine 与 tools 双仓 git 全史 pickaxe（含 trail 文件专扫）、engine 414 件与 tools 393 件悬空提交逐扫、worktrees/.close-backups/ 备份件、老家 sih-tools/scribe/trail 目录、盘上 CALL-LOG 与批材料与 settle 回执与 results 档、三仓台账。恢复出的五条原始事件线全部过 event_hash 重算一致性复核（外层字段字典序、内层插入序、时戳 +00:00 形、session_id 与 identity_hash 在场即入哈希，与今日链真事件校准后重算全对），且事件载 session_id 与对应 settle 提交体 session 行互证。五件原始报告体全部现盘在场，sha256 与原始认证笔所载 report_hash 逐字节一致。

| cert | 批 | 原始事件线恢复来源 | 报告现盘复核 | 补录笔（新哈希前 8） |
|---|---|---|---|---|
| b15cddcf | chaingreen-solo 段1 | close-backups 备份件（让位归并卷走 42 行的备份） | m-chaingreen-1-signcheck.json 一致 | 58ecf653 |
| 4ff60c8f | confpreempt-solo 段1 | engine git 史 be496cc 链文件第 142 行 | confpreempt pipeline-checkpoint.json 一致 | 601a340e |
| 1da6f777 | projfix-solo 段1 | engine 悬空对象 157f20c7 链文件 | projfix cert-report.json 一致 | 91399093 |
| 970c8299 | hookfix-solo 段1 | engine git 史 be496cc 链文件第 153 行 | hookfix verify 报告一致 | d2aa0d15 |
| e6880a63 | entryunique-solo 段2 | engine 悬空对象 cd2f28f7 链文件 | entryunique results-tests.json 一致 | 41547345 |

补录笔形态：补录报告 JSON 载 content_hashes 即 backfill 真值、原始 event_hash 全值、原始 event_id、原始时戳、原始报告路径与哈希、恢复来源指针、引用 settle 提交清单、现盘复核语句、分类不除声明；经引擎 scribe append 落 2026-09-08 trail，事件属显式标注补录非当日原笔，零回写旧日文件。

## 第三节 销账登记（两件）

| cert | 批 | 提交 | 销账通道 | 事由要点 |
|---|---|---|---|---|
| 81ef1c7a | mathfix2-solo 段2 | sih-math 93c4f0b7 | lease bypass（2026-09-08T13:07:53Z） | 六路考古零命中，老家链文件未版控已失，仅存 8 位前缀，认证笔全形不可重构不虚构补笔 |
| 06c30867 | fmtfix-solo 段2 | sih-math d561f17a | lease bypass（2026-09-08T13:08:00Z） | 同上，materials 报告体在盘但认证笔全形不可重构 |

两笔俱在 sih-tools/lease/ledger/bypass.ndjson 在册，事由照录考古结论。pkgclose 批 2026-08-31 reconcile 读数已见两旗、legacytwo 批 errata record-only 在案即历史已呈报存量。

## 第四节 收口读数（余件不可清呈人裁）

三仓 reconcile 复跑：cert_missing 即 tools 5 与 engine 3 与 math 2，与批前逐仓同数零新增零清除。补笔与销账两通道均不改该分类，机械不可清性三证：

1. 分类先序证：lease/src/lease/commitcore.py 即 session 行分支（cert_missing）先于 bypass 分支，bypass 登记不改分类，临时台账实验实证（单件登记后 bypass 类零变化 cert_missing 持平）。
2. 哈希新生成证：scribe append 恒新生成 event_hash（UUID v4 加链铸时戳 max(提示,链尾+1ms) 加重算哈希），永不等于原前缀；_cert_on_chain 只认 certification_completed 的 event_hash 前缀，补录笔载原始哈希于 content_hashes 即账面可追索但不冒充原笔。
3. 写位缺失证：ledger-repair 只盖 sessions 台账 issued/revoked 形无 trail 恢复写位；手写链文件属禁手写台账红线且断 prev_hash 链不可行。

处置：七 cert 全数两态落定即补笔五销账二零悬空零例外（F-2），余件 cert_missing 分类不可机械清除随批呈人裁，候选裁面即维持账面显形（补录笔已可追索）或修订分类器认账 bypass 形，工程层不越俎代庖。

## 第五节 全家族对账表

| 件名 | 提交号 | 处置态 | 证据指针 |
|---|---|---|---|
| chaingreen-solo 段1 tools | c4b56ece10 | 补笔（b15cddcf） | close-backups 备份件＋补录笔 58ecf653＋报告 sha256 一致 |
| chaingreen-solo 段1 engine | ead18e524f | 补笔（b15cddcf） | 同上 |
| confpreempt-solo 段1 tools | 71c34f3e61 | 补笔（4ff60c8f） | be496cc:142＋补录笔 601a340e＋报告 sha256 一致 |
| projfix-solo 段1 tools | cb609c9979 | 补笔（1da6f777） | 悬空 157f20c7＋补录笔 91399093＋报告 sha256 一致 |
| projfix-solo 段1 engine | 3bc29ae6c9 | 补笔（1da6f777） | 同上 |
| hookfix-solo 段1 tools | 576828c1ed | 补笔（970c8299） | be496cc:153＋补录笔 d2aa0d15＋报告 sha256 一致 |
| hookfix-solo 段1 engine | 0621bd3698 | 补笔（970c8299） | 同上 |
| entryunique-solo 段2 tools | 526e2be170 | 补笔（e6880a63） | 悬空 cd2f28f7＋补录笔 41547345＋报告 sha256 一致 |
| mathfix2-solo 段2 math | 93c4f0b7f9 | 销账（81ef1c7a） | bypass 台账 2026-09-08T13:07:53Z 事由照录 |
| fmtfix-solo 段2 math | d561f17a86 | 销账（06c30867） | bypass 台账 2026-09-08T13:08:00Z 事由照录 |

机器可读投影：certarch-solo-materials/certarch-ledger.json（认证上链见认证清单）。

## 管线读数

- 化格 general-v1：三 md 件（任务包与提示词件与结果档）逐件过管线，读数见批材料 pipeline-report
- 核阅 des-001：上列 md 件俱在 state/plan 与 event/plan 即 des-001 域外（des-001 域只盖 sih-engine/doc），exit 2 域外如实记档不属违规（BATCH-FACE 坑位正形）；本批零 sih-engine/doc 域内目标即零正形核阅面
- 检词 core：上列 md 件逐件 exit 0 零违例
- checkcite：recall 后合并单件扫描（任务包与提示词件与投影件与结果档），verdict pass，cited 零 missing 零（本批零 sih-math 推导档引用 ID）

## 认证清单

认证实录（settle cert 取 ask3 记录认证，见结算读数）：

| 件 | 认证哈希前 8 |
|---|---|
| ask3 记录 | c4ac69b7（settle cert 同取此值） |
| 验证件 | bb3d8993 |
| 正身件 | c8060fbb |
| checkcite 件 | f56e10e5 |
| 补录笔 b15cddcf | 58ecf653 |
| 补录笔 4ff60c8f | 601a340e |
| 补录笔 1da6f777 | 91399093 |
| 补录笔 970c8299 | d2aa0d15 |
| 补录笔 e6880a63 | 41547345 |
| 投影件 certarch-ledger.json | c14f4dbb |
| 内容清单件 v2（两预落 md sha256） | 6130d1ca（v1 即 0d11db21 亦在链） |
| 管线报告 v2 | ce0d3471（v1 即 5046b109 亦在链） |
| 红证归档件（lockpath 首跑） | ee7326e3 |

md 件直证申报：scribe append 只收 json 报告件，md 面经内容清单件与管线报告双 json 携 sha256 认证承先例。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 盘点全 | 治理 | 三仓全量枚举、以当跑实态为准不预设名单 | 通过（tools 5＋engine 3＋math 2 共 10 提交 7 cert，已知四员全命中另考古出三员） |
| F-2 两态零悬空 | 治理 | 每件处置必居其一即补笔或销账 | 通过（补笔 5 cert＋销账 2 cert＝7 cert 全覆盖零例外） |
| F-3 补笔补录语境显式 | 治理 | 事件属显式标注补录、载原始哈希与现盘复核语句 | 通过（五笔 content_hashes 载 backfill 真值＋原始全值哈希＋现盘 sha256 对表语句，落当日 trail 零回写） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过面申报（七路 allow 面内全合规；越线一笔即 lock 路径参误写错误目录，红证归档后即时机械清除并正确路径重取，详见误差申报） |
| F-5 链面全绿 | 治理 | 双仓 settle、close 零失败、verify valid、reconcile 零新增 | 通过（tools 0e886f42 与 engine 73c58cf 双 settle、归并 a6f59aac 与 49affba、close 双旗标 bypass 一跑成、verify valid 249 笔；cert_missing 零新增即余件不可清呈人裁非缺件，unbypassed 增量经 bypass 登记归零） |

## 越线与误差申报

- **lock 路径参误写（越线一笔）**：批量取锁首跑 --locks 误传 /sih-tools/ledger/locks.ndjson（缺 lease 段），七笔 lock 全落错误位新建目录（locks.ndjson 七行＋locks.db＋lockface-bills 两件）；正典台账零污染（尾行仍 c2close released 在案）；红证归档 red-evidence-lockpath-first-run.json（四件 sha256 俱录）后 rm -rf 错误目录，改正确路径重取七锁全过；写入位越 allow 面一笔如实申报不掩饰。
- **cert_missing 不可清除如实呈报**：批初即机械实证补笔与销账两通道均不改该分类（分类先序＋哈希新生成＋写位缺失三证），本批按任务包第四节「余件各附不可清事由呈人裁」正形收口，不硬凑归零。
- **identity --quiet 形避坑**：正身件直接全形态出件（anomalies 0，hash 73ba054c），未踩 constclear2c 瘦形态坑。
- **本批零工具代码改动**：CALL-LOG 无新笔义务，watch 无主 CALL-LOG 族候清项不代清（红线）。
- 其余误差零申报。

## 结算读数

- 双仓 settle：tools 段一 0e886f42（base integral-stage-build@e56e316b）、engine 段一 73c58cf（base main@2cc428e），cert 统一取本批 ask3 记录认证 c4ac69b7
- 放锁收约：七路径 unlock 全过零失败（放后他会话 sweepjson-solo b0487ab 共享追加面九锁共存实录，主会协调纪律在册）；close 首跑无主闸拦（红证 16 件俱 CALL-LOG 族与 calls.ndjson 候清项，归档 red-evidence-close-first.json 两件），双旗标 bypass 显式留痕即 bypassed_orphan 与 bypassed_calllog 两笔，close 复跑一跑成即会话 3326c23ab0cb450f revoked、双工地拆、双分支删
- 归并提交号：tools 归并 a6f59aac（closeguard 预收 76928493）、engine 归并 49affba（closeguard 预收 5982427）；预收提交与收约补笔提交俱经 bypass 通道登记即本批对 unbypassed 零新增
- 链 verify：valid，events 249，末哈希 52c2dbb6；本批链面即意图 1 笔与例行读数 3 笔与补录认证 5 笔与批件认证 10 笔（ask3 与验证与正身与 checkcite 与投影与红证与清单 v1v2 与管线 v1v2），同窗他会话笔共存如实记
- 三仓 reconcile 终读数：cert_missing 即 tools 5 与 engine 3 与 math 2 与批前逐仓同数零新增零清除（本批两 settle 提交 cert 在链归 routed 类）；unrouted 俱零；unbypassed 增量即预收两笔经 bypass 登记后归零；session_orphan 与 unbypassed 存量系跨批不代清
- 收约补笔：本节与红证归档两件即本笔，经直改链笔申报与 bypass 通道提交（anchorskill 与 c2close 先例同形）

## 大白话节

- **这批干了什么（说人话）**：账本上有十个「提交单据」引用了七张「验收凭证」，但凭证原件从账本（事件链）上丢了——有的是收约合并时被误卷走，有的是批没跑完就断了。本批把这七张凭证全部清点处置：五张找到了原件（有的在收约时的备份箱里，有的在 git 历史的旧版本里，有的在被删分支的悬空存档里），每张都验了「原件的指纹」（重算哈希一致）和「凭证所指的验收报告还在盘上且一字未变」（sha256 全对），然后在今天的账本上补写了五笔「补录说明」，写明这是补的不是当天的原笔；另外两张（mathfix2 和 fmtfix，八月底的）原件彻底找不到了，就在销账登记簿上如实记了一笔「找不到、不伪造」。
- **为什么账面数字没变（说人话）**：对账工具（reconcile）认死理：只有账本上存在「原始编号开头」的凭证行才算数，而任何新写的补录笔都会拿到新的编号（系统设计如此，编号由内容加时间自动生成，谁也造不出指定编号）。所以补录和销账都不改那几个「cert_missing」计数——这不是本批没干活，是机械上就清不掉。三条机械证据都写进档了，剩下怎么裁（比如改对账规则让它认补录笔）归人拍板，工程层不越权。
- **一句话（说人话）**：七张丢的凭证，五张补了说明、两张登记了找不到，账面不留悬空引用；计数表上的红旗需要人裁才能降，红证据全在档。

## 投影件路径

- 机器可读投影：sih-engine/sih/event/plan/certarch-solo-materials/certarch-ledger.json
- 恢复原始事件线五件：sih-engine/sih/event/plan/certarch-solo-materials/recovered-original-events/
- 红证归档：sih-engine/sih/event/plan/certarch-solo-materials/red-evidence-lockpath-first-run.json
- 补录报告五件：sih-tools/scribe/reports/2026-09-08-certarch-solo-recert-*.json
