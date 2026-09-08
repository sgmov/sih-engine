# reroute-solo 结果档：C3 measure-poly 改道批——GOV-002 v2.6 换版与双出泊与判据扫登记

> 承接：任务包 reroute-solo.md 与用户 2026-09-08 三笔裁定照录（同意改道；pk-053 账全部清掉；口径令两态制零拍脑袋）。FORK-1 窗口执行。
> 队形单线形 solo，日期 2026-09-08，会话 sess-zcode-260908-reroute（承裁会话 3de400e8320dee5f；首会话 dbd8b8bce64ac52d 因 scope 冻结拆重开在案）。

## 意图锚定

- 首意图事件：intent_refined event_hash 前 8 `73fa0f84`（首记录 2026-09-08-ask3-reroute-solo-record.json，v2.5 目标号时点真实保留）＋补认证 `54397a99`（链证守门补笔）
- 承载意图事件：intent_refined event_hash 前 8 `0e3295a2`（记录 v2 件 2026-09-08-ask3-reroute-solo-record-v2.json）
- 三锚引文程序切片（01-ontology-of-names.md L18、08-on-settle.md L110、07-on-assay.md L55）于 ask3 记录，生成器 make_ask3_reroute-solo.py 随批落档，禁手打承契约。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0（首记录与 v2 记录各一跑）；ask3repeater exit 0 status ok anchor_count 3。
- 叩问：五轻信号即改道、确定性化、吸收闭项、回归数学模型、残余载体，全未登记；digest passed covered 5。
- 正身：anomalies 0，identity_hash 01cd226c，core_hash e1c9a7fa。
- 判据扫（本批启动节律）：C1/C5 达成、C2 在飞、C3 沉底 gap 4、C4 沉底 gap 5——本批即解除 C3。
- 锁面：首会话 14 锁全取后因 scope 冻结全放拆会话；承载会话 15 锁全取。

## 施工读数

- GOV-002 v2.6 三处：主线判定面句改写（测量聚合重设计→测量面确定性化）、第三条退出标准改写（四指针＋达成追记）、历史档 v2.6 条目入首位。
- progdoc 改道注记：measure-poly-rev1-progdoc.md 程序状态行就地注记即批一至批四不再开工。
- 泊笔三笔在链：pk-053 出泊 promoted `41fb339c`（ruling 含口径令原文照录）、pk-078 进泊 `1093400d`（带 gate 即置信度线阶段立项或结算件落 event/plan，正形携路由三字段）、pk-054 二次出泊被闸正拒即 OrphanExitRejected（核链发现其已于 2026-09-05 出泊在案，拒得正确，红证归档 pk054-exit-refused-红证.json）。
- critsweep registry：C3 条目更替即 GOV2-C3-measure-doly 退役、GOV2-C3-measure-determinism 立（evidence 态，四指针两形即五文件在档与两链事件在链）；测试 18 绿含改写三件与新增 evidence 达成件。
- 得一签署链 m-gov002v26-sign-1 全通：topic（anchors 指工地 v2.6 文件）→ 合同九发（seat ZCode:GLM-5.3-Flash）→ 回填九发 comply 依据单类 baseline_4 谨慎零 → 计分 stable_clear → assemble stable_clear → check pass 十二项 disposition 裁决通过 → verify identical → sign signed 即 crosscheck-m-gov002v26-sign-1 落链 event_hash 前 8 `33c0e600`，direction comply，重放锚在 DES 单元格。

## 管线读数

- 化格：引擎域四件过 packs/general-v1 全 exit 0 零改。
- 核阅：des-001 引擎域五件全 exit 0 零违规（GOV-002 两件、progdoc、结果档、任务包）。
- 检词：nomenclator check packs/core 三件全 exit 0 零违例。
- checkcite：recall 判据改道测量确定性主题后 checkcite verdict pass missing 零（报告件随批落档）。

## 认证清单

| 件 | 类型 | 状态 |
|---|---|---|
| ask3 记录 v2 | 意图记录 | 书简认证随批落 |
| 验证件 v2 | 双门读数 | 书简认证随批落 |
| 正身件 | 身份报告 | 书简认证随批落 |
| pk-053 出泊记录 | 泊界记录 | 链笔 41fb339c 在档，另走认证 |
| pk-078 进泊记录 | 泊界记录 | 链笔 1093400d 在档，另走认证 |
| tally 材料 | 执契材料 | 书简认证随批落 |
| 计分材料 | 测量材料 | 书简认证随批落 |
| 签署重放锚 | 终签件 | 书简认证随批落 |
| md 件（GOV-002 两件、progdoc、任务包、结果档） | 换版件 | 内容哈希清单件统一绑定（anchorskill 先例） |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 v2.6 与签署链 | 治理 | GOV-002 v2.6 在档且得一签署链笔在链 | 通过（crosscheck 33c0e600，裁决通过，verify identical） |
| F-2 两泊笔在链 | 数据治理 | pk-053 出泊 promoted 与 pk-078 进泊两笔在当日链，ruling 含口径原文照录 | 通过（41fb339c 与 1093400d；pk-054 已于 09-05 出泊在案零触碰如实记档） |
| F-3 判据扫复算 | 跨族治理 | sweep 复算 C3 报 achieved 新证据指针，双跑 IDENTICAL 退出码零，其余判据不漂 | 通过（收约后主树复算即 GOV2-C3-measure-determinism achieved gap 0 hits 19，双跑 IDENTICAL 退出码零，C1/C2/C4/C5 零漂移，degraded 假） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（写入面即 GOV-002 两件、progdoc、pk-078 材料、任务包与提示词、结果档与批材料、registry 与 tests、facet 合同目录、DES 单元格、链文件、报告目录、工地） |
| F-5 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 零新增 | 通过（engine 462dcd9 与归并 5c2ab09，tools 归并 4c974af5；close 携 bypass-orphan 与 bypass-calllog 即无主五件均非本批活面如实留痕；verify valid events 41；reconcile 双仓 unrouted 零即零新增，cert_missing 与 session_orphan 与 unbypassed 为在盘历史账面项归 callloghyg 与后继清账批） |

## 越线与误差申报

- **v2.5 号位撞车**：意图记录与任务包原稿以 v2.5 为目标号，施工中发现 v2.5 已被 govslim-solo（2026-09-07 向界瘦身）占用，本批顺延 v2.6；首意图链笔 73fa0f84 与补认证 54397a99 保留如实记档，历史档 v2.6 条目内申报此号位变更。
- **scope 冻结拆重开**：首会话 dbd8b8bce64ac52d 的 allow 面未含历史档（govslim 拆分后新增位）与 v26 DES 单元格，lock 补锁 scope_violation 拒即 lease open 时 scope 冻结不随后续包改写更新；处置即保存工地改件、清工地、补链证笔后拆会话重开（意图 v2、15 锁）；两次 close 被拒（工地卫生、链证守门）自纠，第三次 close 携 --ack-uncommitted 三路径放行（拆重开让位申报）与 --bypass-orphan/--bypass-calllog（callloghyg 候清项非本批活面）。
- **pk-054 二次出泊红证**：出泊前未核其链上进出泊实态，材料件 state null 陈旧投影误导；scribe park 闸正拒（OrphanExitRejected），核链后确认 09-05 已出泊，撤回二次出泊并修正三处文书；pk-078 进泊链笔 context 措辞含「pk-054 闭项后」小误，链笔不可改如实申报，材料件已修正。
- **执契链四处材料形修复**（判定内容零改动，红证全留档）：DES 单元格计分材料路径补 facet/ 前缀（assemble 按仓库根解析而计分录 facet 相对形）；tally 材料四路径归一为工地根相对形（check 按裸 cwd 解析）；seat-baseline 按 orphanrule 先例正形重制即 props 沿账本尾行（标定账本 pk-044 后只读、尾行 09-07 核 c0110e40 与当日正身核 e1c9a7fa 不配）、身份对随当日正身、date 标实跑日；首跑 R5 身份哈希不一致挂起红证在档（/tmp/check-full.json 归档件）。
- 其余误差零申报。

## 结算读数

- 双仓 settle：engine 462dcd9（归并 5c2ab09，closeguard 预收提交 f5073ab）；tools 归并 4c974af5（预收 7903f01a）；cert 取 72850bf2 即 ask3 v2 记录认证前八位。
- 认证实录：ask3v2 72850bf2、验证件 349e52ac、正身件 766baeb0、pk53 出泊记录 6fea544d、pk078 进泊记录 f2cdb682、执契材料 c870ed21、计分材料 46319f33、终签重放锚 01e45726、内容清单 6450be58 九笔在链，另 crosscheck 33c0e600 与泊笔 41fb339c、1093400d。
- 放锁收约：三阶段放锁即首会话 14 锁（拆重开前）、承载会话 13 锁、补锁 7 锁与两把 facet 自动派生窄锁（reroute-260908 与 reroute-solo 无尾斜杠形）；close 三跑即首跑无主闸五件（attractor 与 scribe 的 CALL-LOG 投影腿与 calllog 台账与 anchor.py 权限位，均 callloghyg 候清项非本批活面）携 bypass-orphan 与 bypass-calllog 留痕后成，revoked 真值，工地与分支清除。
- 链 verify：valid，events 41。
- reconcile：双仓 unrouted 零即本批零新增路由缺口。
- F-3 复算：见 F 表行。
- 施工期发现缺陷两笔申报：其一 lease commit pre-commit 守卫对非 ASCII 文件名（红证.json 八进制转义形）锁面对表失败致引擎 settle 首拒，改名 ASCII 形后过，缺陷候 guardcore 面；其二 facet 合同模式跑批自动派生无尾斜杠窄锁两把，unlock 需按派生形精确放行，先例申报。
- 收约补笔：F-3 与 F-5 与管线实录与结算读数即本笔，经 --no-verify 加 lease bypass 登记通道入版控（anchorskill 与 critsweep 先例同形）。
