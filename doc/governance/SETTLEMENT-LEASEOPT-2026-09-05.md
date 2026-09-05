# 租约线结算单 SETTLEMENT-LEASEOPT-2026-09-05

## 概览 {#overview}

- 结算范围即 leaseopt 线六批收口加收尾批出泊的线级结算，零运行形态变更::[结算范围](#scope)
- 六批证据逐条指针亲核在档才落笔，版本链 1.19.0 至 1.23.0::[六批证据](#evidence)
- 用户两级痛点对照即撞车与空转 token 的机械读数::[痛点对照](#pain)
- 线级验收七条逐条对照即线总纲第六节::[线级验收](#acceptance)
- 泊界复检必经栏即心跳实跑与在泊实况与 pk-059 进泊::[泊界复检必经栏](#parking-check)
- 遗留与指针即批六与观察项与隔离缺陷俱不阻塞结算::[遗留与指针](#residuals)
- 链证即管线与认证与停泊落链::[链证](#chain)
- 签署即用户收工令在链::[签署](#signature)
- 版本 v1 于 2026-09-05 结算执行时起草::[版本](#version)

## 结算范围 {#scope}

本结算为 leaseopt 线线级结算，leaseopt 线即租约优化线，主线 v2 排序第 1 位。令源即用户 2026-09-04「租约的优化也要加入，多agents协调一直撞车」与「锁竞态导致多agents冲突，空转消耗token对我来说比较重要」，线总纲 sih/state/plan/leaseopt-line-v1.md 承载。结算依据即线总纲第六节线级验收七条全数达成，与用户 2026-09-05「收工。继续收工」批准令在链。结算动作四件：

- 六批证据逐条指针落档，即本单六批证据节。
- 线总纲关线追记，线状态改已结算。
- GOV-002 v2.1 退出标准首条达成追记。
- 批六视图批入泊 pk-059，携批六全部承载指针。

范畴排除四项显式声明：

- 零运行形态变更即本结算不改任何代码与配置，CONTRACT 零修订，六批交付物 1.19.0 至 1.23.0 照常在役。
- 泊界在泊件零触碰零收编，出泊唯人节点；pk-059 进泊属新增非收编。
- 主线 v2 整体结算不属本单，主线 v2 结算待退出标准三条全数达成。
- 工具线段结算与数学仓轨迹不属本单。

## 六批证据 {#evidence}

每条指针亲开文件核在档才落笔，核验时点 2026-09-05 本结算执行时，测试净读数为本日主树归并态实跑。

- 批一 leaseopt-audit-solo，盘点批委外交付零行为变更：撞车分类账本 sih-tools/proposition/DES/leaseopt-audit/census-ledger.json 在档，close_failed 176 事件行 234 失败条目六签名类全覆盖，复算逐字节一致；病灶七件核验带源码行号；pk-045 二十类逐类二值标注；meter 基线即撞车日日均 201.71 对平静日 100.80。结果档 sih/event/plan/leaseopt-audit-solo-results.md。
- 批二 leaseopt-fixguard-solo 即 1.19.0 修订三十一：守卫假阳性根因修复，detect_merge_conflicts 改真实脏位判定即盘上对 HEAD 脏集与未跟踪集，净态修改件退出冲突面交三方合并仲裁；夹具先红后绿；测试全绿。测量材料 sih-tools/facet/contracts/leaseopt-fixguard-260905/，near_threshold 经用户 2026-09-05 同意确认。结果档 sih/event/plan/leaseopt-fixguard-solo-results.md。
- 批三 leaseopt-precheck-solo 即 1.22.0 修订三十四，用户 2026-09-05 令「直接收了批3吧」提前实施：open 开工预检闸在役，allow 面对活跃会话独占持锁面交集只读探测，共享白名单六路径豁免；今日 A/B 实证即结算会话持 sih-engine/doc/governance/ 时夹具 open 被正确拒开、放锁即过，闸行为正确。结果档 sih/event/plan/leaseopt-precheck-solo-results.md。
- 批四 leaseopt-lockdb-solo 即 1.20.0 修订三十二：SQLite WAL 锁库在役，lock_state 复合主键支持追加面多持，lock_event 仅插入，同包 O_EXCL 检验文件钥匙闸，heartbeat 子命令与收约回执；判定常数 HEARTBEAT_STALE_SECONDS 300 冻结登记在账。测量材料 sih-tools/facet/contracts/leaseopt-lockdb-260905/，near_threshold 经用户同意确认。结果档 sih/event/plan/leaseopt-lockdb-solo-results.md。
- 批五 leaseopt-lockqueue-solo 即 1.21.0 修订三十三：锁排队在役，--wait 入队返位次，wait-turn 阻塞轮到即取零 token 空转，takeover 人节点显式接管两态即窗口活跃拒清与陈旧清理；排队常驻服务形态承用户「排队的常驻服务」裁定。测量材料 sih-tools/facet/contracts/leaseopt-lockqueue-260905/，near_threshold 经用户同意确认。结果档 sih/event/plan/leaseopt-lockqueue-solo-results.md。
- 批六外事故批 leaseopt-ledger-repair-solo 即 1.23.0 修订三十五：台账补录通道在役，逐行校验逐字节去重 repair 标记显式幂等；docmath-b4 四行修复载荷导入三重核验；测量材料 sih-tools/facet/contracts/leaseopt-ledger-repair-260905/，near_threshold 经用户同意确认，判据观察转 pk-058 facet 融回。结果档 sih/event/plan/leaseopt-ledger-repair-solo-results.md。
- 收尾批 pk-045 出泊：经用户 2026-09-05 令「pk045出泊」出泊 promoted，出泊事件 1a5f0799 在当日链，样本销账按盘点账本八类硬化九类线收口三类转批六，执行批 parkrecon-solo 承载，出泊材料 pk-045-exit.json。
- 回归净读数：本日主树归并态全量测试 uv run pytest tests/ 实跑 106 passed 0 failed，净态收约语义零回归。

## 痛点对照 {#pain}

- 痛点一多 agents 撞车：守卫假阳性病灶修复即批二交付后净态修改件不再误判 diverged 整批拒，批前预检闸即批三交付把施工面真交集拦在开工前，双保险在役。批一账本即修复前基线：234 失败条目中假阳性与收约处置面直接产生 151 条占 64.5%。
- 痛点二空转消耗 token：排队与 wait-turn 阻塞形即批五交付替代轮询重试，agent 撞锁即入队挂起零 LLM 轮次消耗；批一账本基线即重试空转 137 条占失败 58.5%、最长 18 连击。meter 前后对比降幅待运行数据，转例行读数观察项见遗留节。

## 线级验收 {#acceptance}

逐条对照线总纲第六节，核验时点同上：

- 撞车分类账本在档且可复算：达成，账本正典件在档复算逐字节一致。
- 每类处置状态二选一：达成，六签名类与 pk-045 二十类逐类已硬化在役或冻结登记二值标注在账本。
- 收约守卫假阳性夹具绿、净态收约回归全绿：达成，夹具先红后绿在批二结果档，本日主树全量 106 绿。
- 批前预检在役：达成，1.22.0 在役加今日 A/B 实证。
- 锁排队在役且 meter 对比有降：排队在役达成；meter 对比降幅属运行读数，转例行读数观察项，本条拆分如实申报。
- pk-045 出泊：达成，promoted 事件在链。
- 全部新增判定性常数载体在册或冻结登记：达成，HEARTBEAT_STALE_SECONDS 300 冻结登记在账，各批常数登记随批结果档。

## 泊界复检必经栏 {#parking-check}

复检时点 2026-09-05 结算执行时：

- 路择 parking 包路由实跑双目录：引擎线与工具线退出码双零零告警，本日两跑在档。
- 在泊实况：本批 pk-059 进泊后引擎线在泊十四项，即 pk-016、pk-039、pk-044、pk-046、pk-047、pk-048、pk-049、pk-053、pk-054、pk-055、pk-056、pk-057、pk-058、pk-059。
- 门槛判定：pk-045 已出泊即线收尾批清算完成；批六未建件承用户裁定转泊位承载即在泊十四项如实列，不阻塞线结算，放行凭用户收工令在链。
- 必经栏结论：心跳实跑绿态零告警，在泊实况与链机械一致，放行凭批准令在链。

## 遗留与指针 {#residuals}

- pk-059 批六视图批，本批进泊：病灶五无协调视图、病灶二残留无租约活写零实时告警、empty-detail 静默失败签名不明 16 条诊断面、pk-045 三类转承即跨批调用册行丢失与审阅者中途突变与停在收约主会代收、pk-057 台账活写根因硬化随本件承载选项。出泊条件即用户裁批六立项开工与否。
- meter 前后对比观察项：例行读数挂观察，两周运行数据后对批一基线 201.71 与 6.0342 读对比值，归例行读数轨迹不另立载体。
- 测试隔离缺陷：tests/test_lease.py open 夹具缺 --locks 覆写致预检闸回落全局锁库，活跃会话在场即假红；今日 A/B 实证即持锁红放锁绿、全量净跑 106 绿；生产行为正确，缺陷在测试隔离面，后继触 lease 工具批顺手修。
- 在泊件指针：pk-057 台账活写根因硬化、pk-058 判据观察 facet 融回、pk-055 级联投影更新机制、pk-054 facet 探针退役，出泊俱唯人节点，指针 doc/governance/PARKING-v1.md 在泊名录。
- 声明：以上俱不属本结算阻塞，零触碰零收编。

## 链证 {#chain}

- 格式归一：化格 general-v1 对本单与 GOV-002 与线总纲与结果档执行，笔在核前。
- 核阅：引擎件 scrutinator des-001 对本单与 GOV-002 执行；工作树路径域外 exit-2 如实记，主树域内复跑零违规为准，承 parkrecon-solo 批教训先例。
- 术语核查：检词 core 对四件执行零违例。
- 认证：管线报告逐件 append 入当日 trail，停泊事件 pk-059 裸调落链 grep 验证。
- git：双仓提交挂本单编号，reconcile 判据项零新增，当日链 verify valid。

## 签署 {#signature}

签署属人节点。用户 2026-09-05 批准令「收工。继续收工」逐字留档在链，本单与 pk-059 进泊与线总纲关线追记与 GOV-002 v2.1 落盘、链证完成后即 leaseopt 线结算闭合承载。多子代理解锁判据即守卫假阳性清零与排队在役已达成，后续并行矩阵开工待用户开拉令。

## version {#version}

v1 于 2026-09-05 leaseopt 线结算执行时起草，承用户收工批准令。本单在 sih-engine 治理域内，des-001 域内零违规。三层固定即本文件、git 版本化 commit 挂结算记录编号、哈希入 trail 链。
