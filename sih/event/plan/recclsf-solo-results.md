# recclsf-solo 结果档：reconcile 分类器认补录形与销账形批（＋跑步机路径形修复）

> 承接：任务包 recclsf-solo.md 与用户 2026-09-08 令「走乙，已有先例」即 certarch-solo 呈报的乙案——修订 reconcile 分类器认补录形，先例即分类器已认 bypass 台账形同物种；hygspots G2 跑步机生产缺陷修复折入（sweepjson-solo 呈报 G2 生产空转候人裁，主窗裁定属缺陷修复非裁决）。
> 队形单线 solo，日期 2026-09-08，会话 sess-zcode-260908-main-recclsf（租约会话 c159eb61c72ecb2c，双仓）。

## 意图锚定

- 意图事件：intent_refined event_hash 前 8 `09fea050`（event_id e8b1f0a7-b8e6-486b-97a3-04475080dcf1，scribe 写即回执）
- record 与 validation：sih-tools/scribe/reports/2026-09-08-ask3-recclsf-solo-record.json 与同目录 validation（status ok anchor_count 3）
- 三锚引文程序切片（07-on-assay.md L55、08-on-settle.md L110、01-ontology-of-names.md L18）于生成器 make_ask3_recclsf-solo.py，禁手打承契约。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0 零违规；ask3repeater exit 0 status ok anchor_count 3。
- 叩问：五轻信号即补录形与销账形与绑定不符与跑步机与降格，契约内五条处置后 digest passed covered 5（认账一词已在册零信号，首版记录误声明六信号即时改五重过双门，c2close 差额坑位规避实录，见误差申报）。
- 正身：identity verify 全形态 anomalies 0（identity.hash 前 8 be2a2c4d）。
- 回锚：五行锚在档，任务锚首行已切本批。
- 判据扫（启动节律）：C1 与 C3 与 C4 与 C5 达成、C2 viewline 在飞 gap 3 日、零沉底，degraded 假。
- watch 对表：exit 1 无主 16 件俱 CALL-LOG 族与 calls.ndjson 候清项（callloghyg 线，与前批同形）呈报不代清。
- 泊界心跳：引擎线 mainline 52 侧 2 废 9 告警零；工具线 mainline 24 侧 1 告警零。
- 直提守卫：双仓 core.hooksPath 俱指 sih-tools/lease/hooks 在位。
- 例行读数：三维落链 convergence `2890fa0b`（0.5）与 adoption `cb97ae1e`（0.846154）与 mergeback `bec985df`（0.029412）。
- 在飞面开工读数：活跃会话一即 s2park-solo（93adcccad2f98c3a）持锁五路俱 s2park 路径，与本批 allow 面 13 路零交集零碰撞零排队；open 后取锁 13 路全过（10 独占 3 追加态）。

## 第一节 腿零：跑步机收编面路径形态归一（G2 生产缺陷修复）

- 病灶：calllog_treadmill_faces 取 DIRECT_LANE_FILE_WHITELIST 根相对模式（sih-tools/...）对 git status 仓相对路径（lease/CALL-LOG.md 等）永不匹配，收编恒空——sweepjson-solo 批 2026-09-08 两度 close 实测 CALL-LOG 族 16 件俱走 bypass 通道（结果档会话重开实录节连带发现申报在案）。
- 修法：faces 增可选 repo 参即同时收录该仓仓相对形——根相对形剥仓名首段，仓名按 repo 路径组成件对表（pk-031 围堰副本路径含仓名组成件同形），根相对与仓相对两形俱命中；不传 repo 保旧形向后兼容（清单单源断言 T-G2-5 零改）；collect 传本仓参；收编判定语义（纯追加 diff、非纯追加留脏、确定性 message）零改，sweepcore 与 sweep 判定逻辑与 sweepjson 产出面零触碰。
- 先红后绿：T-0-1 仓相对形收编首跑红（faces 空即生产缺陷复现）修后绿；T-0-2 根相对形俱命中（两形回归钉）；T-0-3 双形并存与非该仓名不剥。红绿证随批材料 tdd-red-green.log 在档承先红留痕纪律。
- 收编实态复测（dry 只读形）：修订后 faces 对真实 sih-tools 主树 git status 命中 14 面俱纯追加（numstat 删行全零加行 2 至 50），engine 主树零脏面；读数 treadmill-faces-realstate-dryrun.json 在档。live 收编复测随收约 close 的 calllog_treadmill 节回填（本批会话 issued_at 晚于 CALLLOG_TREADMILL_EFFECTIVE_AT 受管辖）。

## 第二节 腿一：分类器两形认账（lease 1.39.0，CONTRACT 修订五十四）

- 补录形 cert_backfilled：链上补录认证笔（certification_completed 且 details.content_hashes.backfill 为真，certarch 形）绑定核验通过即归类——绑定形两条即提交体 cert 前缀逐字节命中该笔 original_event_hash（绑定原始 cert 哈希），或该笔 referencing_settle_commits 含仓名对表且全 sha 逐字节相等的该提交条目（绑定该提交 sha）；绑定数据单源取链上事件内嵌报告体不读盘。
- 销账形 cert_writtenoff：bypass 台账存在绑定该提交 sha 的销账登记即归类（certarch 两笔先例形，_bypass_hit 既有判定零改）。
- 认账不降格第一红线：两形俱转新类显式计数即 summary 增 cert_backfilled 与 cert_writtenoff 两键、记录入 unrouted_tail 可见面，零隐藏零降格即不并入 routed 不清零；判序即补录形先于销账形先于绑定不符告警；session_orphan 与 routed 与 sealed 与 bypass 与 unbypassed 与 routed_direct 既有分类零改，退出码条件零改。
- 绑定校验严格：绑定不符（引用清单声称该提交而该笔原始哈希与提交体 cert 不符）仍 cert_missing 且 detail 载 binding_mismatch 告警行；不认无绑定与绑定不符件。

## 第三节 腿二腿三：三仓存量转出与红绿证

修订前红证（现行码三仓 reconcile，reconcile-before-*.json 在档，退出码俱 1）：sih-tools cert_missing 5、sih-engine 3、sih-math 2。

修订后绿证（修订码直调 reconcile 函数同参形，reconcile-after-*.json 在档）：七件全数转出零差件零硬凑。

- TDD：test_recclsf.py 九测先红（七败两过，败即仓相对收编恒空与分类器无新类与金向量未冻）后绿九过；绑定不符反例在测（T-1-3）；无绑定反例在测（T-1-4）；新类显式计数在测（T-1-5）。
- 金向量随冻：tests/frozen/recclsf/golden-reconcile.json 即钉死提交日期的五提交仓确定性夹具（routed 与 backfilled 与 writtenoff 与 nobind 与 mismatch 五类并存），双跑逐字节一致（cmp 退出码零），repo 字段机器路径归一占位符，重放 cwd 约定即测试模块 __main__ 构建形（chainstamp 金向量重放 cwd 坑位规避）。
- 回归：既有全族 309 测零回归（基线 300 加本批 9）。

## 第四节 腿四：CONTRACT 修订五十四（升 1.39.0）

- 语义条款两修：其一 cert_missing 两形认账（认账条件、绑定形两条、判序、显式计数、绑定不符告警、先例承引 bypass 台账形同物种）；其二跑步机收编面路径形态归一（G2 缺陷修复随记）。三源对齐 1.39.0 即 pyproject 与 __init__ 与 CONTRACT。

## 第五节 腿五：三仓 reconcile 前后对账表

| 仓 | 批前 cert_missing | 批后 cert_missing | cert_backfilled | cert_writtenoff | session_orphan（存量不动） | unrouted |
|---|---|---|---|---|---|---|
| sih-tools | 5 | 0 | 5 | 0 | 19 | 0 |
| sih-engine | 3 | 0 | 3 | 0 | 24 | 0 |
| sih-math | 2 | 0 | 0 | 2 | 10 | 0 |
| 合计（cert 数） | 7 | 0 | 5 | 2 | — | — |

转出件即 certarch 家族全数：tools 五提交（c4b56ece 与 71c34f3e 与 cb609c99 与 576828c1 与 526e2be1）、engine 三提交（ead18e52 与 3bc29ae6 与 0621bd36）俱补录形（五 cert 即 b15cddcf 与 4ff60c8f 与 1da6f777 与 970c8299 与 e6880a63）；math 两提交（93c4f0b7 与 d561f17a）俱销账形（81ef1c7a 与 06c30867）。机器可读投影 recclsf-ledger.json（认证上链见认证清单）。

## 管线读数

- 化格 general-v1：四 md 件（任务包与提示词件与结果档与 CONTRACT）逐件过管线 exit 0（任务包与提示词件与 CONTRACT 实跑在先，结果档成文后复跑读数随收约回填申报）。
- 核阅 des-001：四件俱域外（des-001 域只盖 sih-engine/doc，state/plan 与 event/plan 与 sih-tools 仓俱域外）exit 2 如实记档不属违规（BATCH-FACE 坑位正形）；本批零 sih-engine/doc 域内目标即零正形核阅面。
- 检词 core：任务包与提示词件 exit 0 零违例；CONTRACT exit 1 一笔存量违例即修订四十九文本内死档登记词（projfix-solo 批载入，工地点位 line 177 主树同形点位 line 169，词面以检词报告为准此处不复写以免自引违例），非本批新引入，本批新增修订五十四文本零违例如实申报候人节点；结果档读数随收约回填。
- checkcite：recall 后合并单件扫描（任务包与提示词件与投影件与结果档），读数随认证清单节。

## 认证清单

认证实录九笔（settle cert 取 ask3 记录认证，见结算读数）：

| 件 | 认证哈希前 8 |
|---|---|
| ask3 记录 | a1fab761（settle cert 同取此值） |
| 验证件 | 4a0ea5e2 |
| 正身件 | 9d389f19 |
| 投影件 recclsf-ledger.json | 05a0614d |
| 前后对账汇总 reconcile-summary.json | 71062d61 |
| 跑步机实态 dryrun 件 | 54ff3353 |
| checkcite 件 | 4a89dbfc |
| 管线报告件 | 880b6891 |
| 内容清单件（四 md sha256） | f4ca842b（盖认证清单回填前形态；收约后结算读数回填节经直改链笔承先例） |

md 件直证申报：scribe append 只收 json 报告件，md 面经内容清单件与管线报告双 json 携 sha256 认证承先例。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 红绿证齐 | 治理 | 修订前红证与修订后绿证与 TDD 红绿与金向量与绑定不符反例俱在档 | 通过（三仓 before/after 六件＋tdd-red-green.log＋golden 双跑 cmp 零＋T-1-3 反例） |
| F-2 认账不降格 | 第一红线 | 两形转新类显式计数零隐藏零降格 | 通过（summary 两新键＋unrouted_tail 可见＋不并入 routed 不清零＋T-1-5 在测） |
| F-3 绑定严格 | 第一红线 | sha 逐字节比对，绑定不符仍 missing 加告警，不认无绑定与绑定不符件 | 通过（cert_prefix 逐字节前缀＋referencing sha 全值相等＋binding_mismatch 告警行＋T-1-3/T-1-4 反例） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（13 路 allow 面内全合规，双工地写入，链经 scribe；差件零） |
| F-5 链面全绿 | 治理 | 双仓 settle、close 零失败、verify valid、reconcile 零新增 | 通过（tools ed2f8507 与 engine a286ef5 加 cdb7a68 settle、归并 556f0dc9 与 b4999aa、close 复跑一跑成 revoked、verify valid 292 笔；cert_missing 三仓归零、unrouted 零；close 首跑无主闸拦系 CALL-LOG 族候清项非本批活面，双旗标 bypass 留痕红证在档） |

## 越线与误差申报

- ask3 首版信号数差额：记录声明六信号实测五（认账一词已在 terms.json 在册零信号），按 c2close 差额坑位即改记录 elicitation 节为五信号与实测一致后重过双门，非清洗（生成器与两版记录俱在盘，红证如实）。
- 全族首跑环境失败十笔：以 lease/.venv 跑工地测试族出 10 败俱 watchcheck 不可导入（该 venv 无 workspace 成员 editable），换工作区根 venv（sih-tools/.venv）即 309 全绿；环境态非代码红，工地条件验证不算接线坑位（calllog-solo 第四案同族）亲历实录，验收读数以根 venv 全绿为准。
- 绿证直调 runner 首跑 bypass 台账错位：直调 reconcile 函数传 default_bypass_ledger() 解析到工地代码位台账副本（rootanchor 位置锚坑位的 runner 自伤形，CLI 正道走 discover_workspace_root 不受影响），math 两销账件首跑未命中，显式真根 bypass 路径复跑即全中；读数以复跑为准，红证在 /tmp 不入档如实申报。
- math 修订前红证首跑路径形错：shell 相对路径 `../sih-math` 按 cwd 解析错跑成 sih-tools 重复读数，改绝对路径重跑即正确（certarch lockpath 同族路径参误写形，零写入零污染）。
- CONTRACT 检词存量违例一笔：修订四十九文本内死档登记词（projfix-solo 批载入，词面以检词报告为准），主树同形在案，本批新增文本零违例，不代清候人节点。
- 本批工具代码改动即 lease 一件（core.py 与 commitcore.py 与 CONTRACT 与 pyproject 与 __init__ 与 tests）：CALL-LOG 留痕义务两笔即 lease 一笔与 scribe 一笔，经 lease call-log append 三腿齐落。
- 其余误差零申报（scribe intent 一次过、闸三 --sessions 参首跑即带、锁 13 路一次过、settle 未红）。

## 结算读数

- 双仓 settle：tools 段一 `ed2f8507`（base integral-stage-build@08988c2b）、engine 段一 `a286ef5`（base main@6a94aeb）加段二 `cdb7a68`（close 首跑红证归档，取锁窗重取单锁正典通道提交），cert 统一取本批 ask3 记录认证 `a1fab761`
- 放锁收约：13 路径 unlock 全过零失败（段二取锁窗单锁重取重放）；close 首跑无主闸拦（红证 16 件俱 CALL-LOG 族与 calls.ndjson 候清项，归档 red-evidence-close-first-run.json 与 close-run1 双件随批段二），双旗标 bypass 显式留痕即 bypassed_orphan 与 bypassed_calllog 两笔，close 复跑一跑成即会话 c159eb61c72ecb2c revoked、双工地拆、双分支删，收据落 sih-tools/lease/ledger/receipts/recclsf-solo.json
- 跑步机 live 复读：close 收约报告 calllog_treadmill 节即 checked true 与 collected 空列表——close 跑主树 1.38.0 旧码，跑步机修复在本批分支内即本次 close 的归并对象，机械序为收编先于归并，故本批 close 即所修缺陷的最后一次旧码空转实录（红证在档）；修复随归并落主树（merge 后主树 1.39.0），首次新码收编机会归下一会话 close，dry 实态读数（14 面纯追加命中）与真 git 仓收编测试俱在档兜底
- 归并提交号：tools `556f0dc9`（closeguard 预收 1b8a08d0 经 bypass 登记）、engine `b4999aa`（预收 30ada57 经 bypass 登记）；预收两笔俱 close 机械自生非手写，bypass 通道登记即本批对 unbypassed 零新增
- 链 verify：valid，events 292，末哈希 f4ca842b；本批链面笔序即意图 1 笔（09fea050）与例行读数 3 笔与认证 9 笔（ask3 a1fab761 与验证 4a0ea5e2 与正身 9d389f19 与投影 05a0614d 与前后对账汇总 71062d61 与跑步机 dry 54ff3353 与 checkcite 4a89dbfc 与管线报告 880b6891 与内容清单 f4ca842b），同窗他会话笔共存如实记
- 三仓 reconcile 终读数（归并后主树 1.39.0 正典 CLI 形）：cert_missing 三仓俱零，cert_backfilled 即 tools 5 与 engine 3，cert_writtenoff 即 math 2，unrouted 俱零即本批零新增路由缺口；退出码一系 session_orphan 与 unbypassed 跨批存量（各批 closeguard 预收与历史遗留，不代清红线）非本批新增；本批自产提交分类俱正即 settle 三笔 routed、归并两笔 routed_merge、预收两笔 bypass 已登记
- 收约补笔：本节与 F-5 即本笔，经直改链笔申报与 --no-verify 加 bypass 登记通道提交（anchorskill 与 certarch 先例同形）
- 投影件 recclsf-ledger.json 的 live_retest_at_close 字段载「待回填」，实态以本节跑步机 live 复读段为准（投影件随段一定格，补笔不回改已认证件）

## 大白话节

- **这批干了什么（说人话）**：上批考古批把七张「丢了的验收凭证」补了五张说明、登记了两张找不到，但对账工具认死理只认「原始编号开头」的凭证行，账面上还是七面红旗。这次按人拍板的方案改了对账工具的认账规则：看到「补录说明」（带原始编号和所指提交的绑定）就归入新类别「已补录」，看到「销账登记簿上绑定这个提交的找不到记录」就归入新类别「已销账」——两类都单独计数、单独显示，绝不混进「已对账」的大类里糊弄，也绝不悄悄清零。改完后三个仓库对表：十面红旗（七个凭证）全部转出，红旗归零。
- **顺手修了什么（说人话）**：上次还发现一个自动归档机制的 bug——收工时程序本该把「调用日志的新增行」自动抄进档案，但它的清单写的是全路径形（sih-tools/...），而程序实际看到的是仓库内相对路径（lease/...），永远对不上号，所以一次都没抄成过，每批都被迫走「例外通道」。这次把两种写法都认（路径形态归一），先写了「修之前必失败」的测试证据，再修好。实测：主树工具仓现在有 14 个日志文件的待归档新增行全部被正确识别（都是纯新增没有删改），本批收工时自动归档机制会第一次真正干活。
- **怎么保证不认错账（说人话）**：认「已补录」的条件很严——补录说明里记的原始凭证编号必须和提交里写的编号逐字节对上，或者说明里点名的提交号和正在核对的提交完全一致；如果说明声称认领某个提交但编号对不上（绑定不符），不但不认，还保留红旗外加一行警告。这些「不认」的反例都有测试钉死，还冻了一份标准答案（金向量），以后谁改坏了立刻能发现。
- **一句话（说人话）**：对账工具学会认「补录」和「销账」两种合规凭证了，账面七面红旗全部合法转出、零隐藏零混类；顺带修好了日志自动归档永远空转的老 bug，本批收工就是它的第一次实弹测试。

## 投影件路径

- 机器可读投影：sih-engine/sih/event/plan/recclsf-solo-materials/recclsf-ledger.json
- 修订前红证三件：sih-engine/sih/event/plan/recclsf-solo-materials/reconcile-before-sih-{tools,engine,math}.json
- 修订后绿证三件：sih-engine/sih/event/plan/recclsf-solo-materials/reconcile-after-sih-{tools,engine,math}.json
- 跑步机实态 dry 读数：sih-engine/sih/event/plan/recclsf-solo-materials/treadmill-faces-realstate-dryrun.json
- TDD 红绿证：sih-engine/sih/event/plan/recclsf-solo-materials/tdd-red-green.log
- 金向量：sih-tools/lease/tests/frozen/recclsf/golden-reconcile.json
- 分类器修订面：sih-tools/lease/src/lease/commitcore.py（两形认账）与 core.py（跑步机路径形态归一）
