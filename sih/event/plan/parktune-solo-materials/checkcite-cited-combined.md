# parktune-solo 结果档：泊界侧轨积压告警调优——gate 远景件豁免计数

> 承接：任务包 parktune-solo.md 与主会 2026-09-08 编排令即 FORK-2 窗口段；上游即 parkgate-solo 批（2026-09-07 落 gate 远景廊）遗留配置性误报——带 gate 的远景件 pk-070 与 pk-077 落 siding 轨触发 siding_surplus 告警（阈值二现侧二即告警），gate 件入侧轨是设计行为非积压，每日心跳误报稀释告警可信度违基线三注意力只投异常。
> 队形单线形 solo 委外代理亲写零子代理，日期 2026-09-08，会话 sess-zcode-260908-parktune（租约 session_id 83ce1a7c9b5145d0）。

## 意图锚定

- 意图事件：intent_refined（event_hash 前 8 `8aaa4f9b`）
- record：sih-tools/scribe/reports/2026-09-08-ask3-parktune-solo-record.json
- validation：sih-tools/scribe/reports/2026-09-08-ask3-parktune-solo-validation.json（双门第二门 ask3repeater 输出 status ok anchor_count 3）
- 三锚引文程序切片（07-on-assay.md L41 工具假阳性句、08-on-settle.md L110 应而不藏、07-on-assay.md L55 鉴只列事实）于生成器 make_ask3_parktune-solo.py 随批落档，禁手打承契约。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0 零违规；引擎 ask3repeater exit 0 status ok anchor_count 3。
- 叩问：elicit check 五词出轻信号即豁免、告警语义、侧轨积压、数据形、代码形全未登记；ask3 契约内五条叩问处置（俱普通词面描述性使用，本批不立名不登记，语义钉死在两册 CONTRACT 修订记档）后 digest passed covered 5。
- 正身：identity verify anomalies 0 verdict attest。
- 例行读数：gauge record 三维落链即 convergence 1.0（4b24ce70）与 adoption 0.945402 会话窗与 mergeback 0.030303（cb6d2891）。
- 判据扫：degraded 假；C1 与 C5 achieved、C2 in_flight gap 3、C3 sunk gap 4、C4 sunk gap 5——沉底两判据为视图告警项如实转述候人节点裁，回算零裁决。
- watch 对表：无主修改 2 件即 sih-tools/calllog/calls.ndjson（已跟踪修改）与 sih-tools/critsweep/CALL-LOG.md（未跟踪新件），mtime 同刻 2026-09-08T00:45:23Z 疑似 critsweep 批后 CALL-LOG 补记，按零 token 二值协议呈人节点裁决，不代清不触碰。
- 泊界心跳（改前红证在案即 materials/acceptance/red-before-engine-line.json 与 before-tools-line.json）：工具线 exit 0 主线 23 侧 1 废 0 告警零（侧轨件 pk-042 非因 P102 到期落轨）；引擎线 exit 1 主线 47 侧 2 废 9 告警一即 siding_surplus count 2 threshold 2——侧轨两件正是 pk-070 与 pk-077 俱 failed P104 gate_hold，即本批目标误报的旧代码红证。
- 温故检索：scribe query --topic parktune-solo 出 16 笔即当日链全量；对照组 --topic zzz-qqq-xyzzy 同出 16 笔即 --topic 过滤语义宽松对照组亦全量返回，本批主题在链无专属笔即等价零命中读数，读数件随批材料 recall-pre-2026-09-08.json。
- reconcile 批前基线：双仓 unrouted 0、engine cert_missing 3 与 tools 4、session_orphan 24 与 19、unbypassed engine 69 与 tools 98 为在盘历史账面项，如实记档不代清。

## 勘察读数与形制订夺

- 包与代码实态：routes.toml [alarms] 节现两键即 siding_surplus_threshold=2 与 mainline_starvation_threshold=0；告警计数逻辑在围堰 route.py route_batch 即 counts["siding"] >= pack.siding_surplus_threshold 全量计数无豁免机制；pk-070 的 gate 键为对象无 fired_at 键、pk-077 的 gate.fired_at 为显式 null，两者俱因 P104 gate_hold 判败落 siding 即首败谓词即豁免判定的机械依据。
- 双载体 cmp 基线：改前 sih-tools/selector/packs/parking/ 与 sih-engine/src/attractor/packs/parking/ 逐字节一致（包 0.3.0）。
- 引擎侧落后债确认：sih-engine/src/attractor/route.rs 为十谓词版即 ALL_KINDS 不含 gate_hold、parking_aging 无 gate 停计——parkgate-solo 批缺陷披露在案「二进制重编与引擎侧 route 双跑归引擎侧下批或例行重编时承载」，即本批。
- 金向量现状：expected 冻结于 2026-09-02 即 parking 包 0.2.0 与十谓词时代；parkgate 批升包 0.3.0 未重冻，T2 哈希对表处于漂移态（引擎随迁包 0.3.0 对冻结 0.2.0），先例批如实申报候本批承载。
- **形制订夺：数据驱动的代码形**。纯数据形不可表达即三路皆死：其一 [alarms] 现有两键零豁免机制，改阈值即调大淹没真积压违修复保真；其二改 P104 route_on_fail 即 gate 件离侧轨违验收硬项 pk-070/pk-077 仍侧轨；其三谓词重排不解计数问题告警在批级不在谓词级。定夺形即 routes.toml [alarms] 新增可选键 siding_surplus_exempt_kinds（缺省空数组零豁免向后兼容、值域限于谓词 kinds 否则拒包 fail-closed），消费逻辑在围堰 route.py 与 pack.py 及引擎 route.rs 两侧同改；豁免判定按首败谓词 kind 机械归类即 failed_predicate 指向豁免 kind 谓词的侧轨件不计入积压计数，点火后 gate 因 time_deadline 首败落轨不豁免即真事件保真。取舍依据逐条落批材料不猜，承任务包第二节先勘后改条款。
- 请求写入节批内修订：原表漏列四路径即 sih-tools/selector/src/selector/ 与 sih-tools/selector/pyproject.toml 与 sih-engine/src/attractor/route.rs 与 sih-engine/tests/attractor_route.rs，代码形必写面即勘察定夺的正下游，任务包本体在写入面内批内修订补全，修订依据即本节，越线申报节同记。

## 施工读数

- 围堰 selector 源改三件：pack.py 加 Pack.siding_surplus_exempt_kinds 属性与 _parse_tail 四元组与 _parse_exempt_kinds（非法 kind 拒包）；route.py 的 route_batch 有余告警改积压计数即排除首败谓词 kind 属豁免清单的侧轨件，summary.siding 全量计数不隐藏；模块 docstring 补豁免语义与载体引用。
- parking 包升 0.4.0 即 manifest 版本进位与 [alarms] 声明 siding_surplus_exempt_kinds = ["gate_hold"]；工具 pyproject 0.5.0 进位 0.6.0（报告头 tool.version 0.4.0 不动承修订五先例）；CONTRACT 修订七登记即概览判定与告警节补豁免语义句。
- 引擎 attractor 同改与追平：route.rs 的 ALL_KINDS 十一件（PARKING_KINDS 三件含 gate_hold）与 check_gate_hold 结构判定与 evaluate 分发与 parse_params 无参白名单与 parking_aging 未点火 gate 停计（parkgate 修订六语义引擎侧补齐）与 parse_tail 四元组与 parse_exempt_kinds 与 route_batch 积压计数排除段；SELECTOR_VERSION 0.4.0 不动对表围堰 __version__。
- 引擎 attractor/CONTRACT.md 新建册：对表基准、有余告警豁免语义、修订记录一即本批三件同步（豁免对表、gate_hold 追平、金向量随冻重录）。
- 测试：围堰新增 tests/test_alarm_exempt.py 八测即误报灭、真报留、混合面计数、点火后到期计入、包装载豁免键、缺省零豁免零漂移、非法 kind 拒包、双跑一致；引擎 route.rs 模块单测新增三测即 t4_gate_hold_hold_and_release（fired_at 缺省与显式 null 两形判败、在场判过、缺省非对象判过）、t4_surplus_exempt_kinds_parktune（缺省零豁免旧行为、豁免后零告警全量不隐藏、真积压仍告警、点火后到期计入）、t4_aging_suspended_for_unfired_gate（停计与复计），t4_pack_validation_rejects 补非法豁免 kind 拒包断言；tests/attractor_route.rs 的 t6 装配位谓词数断言随包四谓词适配（3 至 4，声明变更非违规）。
- 双载体包 cmp：改后 sih-tools/selector/packs/parking/ 与 sih-engine/src/attractor/packs/parking/ 逐字节一致（工地 diff -r IDENTICAL）。

## 验证读数

| 验证 | 实态 |
|---|---|
| 改后两线心跳 F-1 前半 | 工具线 exit 0 主线 23 侧 1 废 0 告警零零漂移；引擎线 exit 0（改前 exit 1）主线 47 侧 2 废 9 告警零，pk-070/pk-077 仍 siding failed P104 即设计轨保持 |
| 真积压夹具 F-1 后半 | 三件非 gate 到期夹具（materials/acceptance/backlog-fixture-input/，临时目录不改真泊件）siding 3 告警 siding_surplus count 3 exit 1 即真报留（绿证 backlog-fixture-green.json） |
| 红证留档 | 改前引擎线误报读数 red-before-engine-line.json 即旧代码 count 2 告警在案，先红留痕未清洗 |
| 跨实现对表 | 引擎 attractor 二进制与围堰新代码同参两线输出 cmp 逐字节 IDENTICAL（tools-line 与 engine-line 双绿） |
| 同参双跑 F-3 | 引擎二进制与围堰 CLI 各自同参双跑 cmp 逐字节 IDENTICAL，退出码一致（0/0） |
| 测试套 | 围堰 uv 形 159 全绿（151 既有加 8 新增）；引擎 route 模块单测 14 绿；attractor_route 集成 10 测 9 绿 1 红即 t0 工地条件差异（对表主树围堰包仍 0.3.0，收约归并后主树双包 0.4.0 一致即绿，非缺陷如实记档） |
| 金向量重冻 F-2 | 按 SPEC-021 T9 基线种随冻重录：期望由围堰 Python 原件（新代码新包）重跑重冻、消费逻辑零改；漂移归因纯期望过期即 core 四场景逐字节零漂移、parking 三场景差异仅 pack.version 0.2.0 至 0.4.0 与 P104 谓词记录（含 parkgate 批 0.3.0 未随冻部分）、badpack-kind 报文 ALL_KINDS 扩 gate_hold，漂移集包含于申报影响集（drift-attribution.txt 在档）；T2 金向量三测（byte_identical 与 badpack_envelope 与 live_lib）全绿即引擎新代码对重冻期望逐字节一致 |
| 引擎重编 | 工地 cargo build --bin attractor 过（唯一 warning 为 src/retriever/mod.rs 既有死码 integration_root 与本批改笔无关）；主树重编随收约归并后承载见完工回显 |

## 管线读数

- 化格：引擎域 md 件（任务包镜像、结果档、attractor/CONTRACT.md、金向量 README.md）过 packs/general-v1；工具域件（selector/CONTRACT.md）化格不越域如实申报（sih-tools 域件不受引擎格式规范约束，域别纪律）。
- 核阅：des-001 对上述件逐件跑；src/attractor/CONTRACT.md 与 state/plan 与 event/plan 目标域外 exit-2 如实记入档不属违规（金向量 README.md 在 src/attractor/fixtures/ 下同域外）。
- 检词：nomenclator packs/core 逐件。
- checkcite（认证前必跑）：recall.py 书单对批引用件拼接扫描形单次 --cited，读数件 sih-tools/scribe/reports/2026-09-08-parktune-solo-checkcite.json。

## 认证清单

| 件 | 类型 | 链上哈希前八 |
|---|---|---|
| ask3 记录 | 意图记录（intent 笔 8aaa4f9b） | 待回填 |
| 正身件 | 身份报告 | 待回填 |
| 管线报告 | 三步读数 | 待回填 |
| checkcite 读数件 | 书单对表 | 待回填 |
| 验证件 | 红绿证与心跳与双跑读数 | 待回填 |
| 内容哈希清单件 | md 与无仓控件统一绑定 | 待回填 |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 误报灭真报留 | 实装 | 改后实跑两线路由：pk-070/pk-077 在侧轨零 siding_surplus 告警；真积压夹具≥三件仍触发告警，红绿证在档 | 待终态回填 |
| F-2 双载体一致 | 跨族治理 | 围堰与引擎包 cmp 逐字节一致（或两侧代码同改后同参双跑输出一致），引擎重编后验证，金向量按 GV3 处置在档 | 待终态回填 |
| F-3 双跑确定性 | 跨族治理 | 改后同参同日双跑 cmp 逐字节 IDENTICAL 退出码一致 | 待终态回填 |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径，reroute-solo 面（GOV-002、critsweep、facet、DES、引擎泊材料）零触碰 | 待终态回填 |
| F-5 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 较批前零新增 | 待终态回填 |

## 越线与误差申报

- 请求写入节批内修订（扩面申报）：原表漏列四源码路径，勘察定夺代码形后批内修订任务包请求写入节补全（任务包本体在写入面内），修订依据落勘察读数节，F-4 对表基准即修订后请求写入节十八路径。编制疏漏非越线，如实记档。
- 测试首跑红两件：test_fired_gate_expired_counts 断言误写（单件 count 1 低于阈值本不告警）与夹具 ttl 缺省 60 未到期，俱测试件自身错误，被测源码零改动修正后绿，红态在施工过程如实留痕。
- t0_migrated_packs_byte_identical 工地红：对表基准指主树围堰包（仍 0.3.0 未收约），工地引擎包已 0.4.0 即跨对表红是工地条件差异非缺陷；收约归并后主树双包俱 0.4.0 复验即绿，主树复验读数见完工回显。
- plain python 形 11 红：CLI 子进程形测试在无 selector 可执行命令的环境下红（主树同红），uv 形 159 全绿即正典读数，环境差异非回归。
- 金向量重冻归因：parking 三场景期望差异含 parkgate 批 0.3.0 时代未随冻的 P104 谓词记录（旧冻结 0.2.0 无 P104 行），本批重冻一并带上即两个批次的合法变更一次收口，归因明细在 drift-attribution.txt。
- watch 无主二件不代清不触碰，呈人节点二值裁决（前置读数节）。
- 其余误差零申报。

## 结算读数

（收约后回填）

## 完工回显

（收约后回填）
# 路择契约

## 概览 {#overview}

- 路择／Selector，2026-08-19 与 08-20 用户签署双名，登记于 sih-engine PRO-007 已签项，孵化环第五跑，判定为能力外切，即已签未建组件的外切建身::[孵化登记](#incubation)
- 词条：受托持人批的谓词包，对到达材料逐件机械判定，择定主线、停放、丢弃三路之一。空腹零 LLM，只读不写，不裁决，判在别处::[定义承接](#definitions)
- 三路英文路径词承词条即 mainline、siding、scrap_track::[判定与告警](#routing)
- 范围即谓词族十一件，单件材料判定四件即 schema、锚定白名单、写入边界、状态标注，按轮产出判定四件即锚定密度、常设约束核对、断言须附清单、登记冲突告警，时间维度三件即到期判定与在泊告警与搁置判定::[谓词包契约](#pack)
- 验收判据累计即首跑八条加二跑九条加泊界批三条，全部可证伪::[验收判据](#criteria)

## 孵化登记 {#incubation}

第五跑对象即路择，名已立而工程零落地为触发，与书简同款名实错位即已立之名配未建之身，处置不同即书简是未建组件在围堰内建身再回迁的先例，路择是能力外切。执行范式为 T6-D 即双子代理并行加主线串行验证，任务包 sih-engine/task-packages/selector-cutout-t6d.md，F 锚定八条跑前立文。回归标本即引擎 task-packages 过程件，2026-08-18 记十一件现存八件，漂移如实记录以现存为准。

## 定义承接 {#definitions}

三处承接。一承 PRO-007 路择词条全文即三路判定、空腹、只读、判在别处、有余告警与饥饿告警为词条职能非名背义。二承 COURSE-v1 第 2 段路线即单件材料判定的配置族先行含 schema、锚定白名单、写入边界、状态标注四件，按轮判定的族随后，两族今均在册。三承姊妹范式即核阅的空腹包形态、化格的退出码三值、检词的双版本戳加治理域，同一谓词引擎底座族。

边界五条：不裁决即路由与告警是机械事实，材料最终去向判在别处；不写即零写路径零状态残留；不判语义即谓词只作字段、结构、存在性与模式匹配，不解读 claim 文本与结论真值；不越谓词族即新谓词须经契约修订登记在案；不背名释义即有余与饥饿与登记冲突告警为职能在案，路径词 mainline、siding、scrap_track 为字段枚举值非翻译承诺。

## 谓词包契约 {#pack}

谓词包为目录，结构同核阅规则包即 manifest.toml 加 routes.toml。manifest 承 name、version、domain 含 include 与 exclude，domain 为治理域声明入报告头。routes 承 [[predicates]] 数组与 [defaults] 与 [alarms]。谓词四件加按轮判定四件即八件族。单件材料判定族：schema_required 查必填字段在且类型正，route_on_fail 定败向；state_annotation 查 state 在枚举内；anchor_whitelist 查 anchors 逐项匹配白名单 glob；write_boundary 查 requested_writes 逐项匹配允许根 glob。按轮产出判定族，轮记录即材料五字段上加 round 键含 assertions、conclusions、receipts 三数组，round 键缺省或非对象时按轮判定的谓词一律判败即 fail-closed：anchor_density 查断言证据路径在 resolve_root 下真实存在且有效锚占比达 density_threshold；standing_constraints 查回执 tool 集覆盖 required_receipts 全部；assertion_lists 查每断言携带非空 evidence 数组只查结构不解析真伪；registry_conflict 为告警位非路由位，见判定与告警节。route_on_fail 与 pass_route 取值限于三路枚举。空腹即工具本体零内置谓词零内置路由，全部判定知识来自包。回执自包含即 flagged_terms 写在轮记录内，本工具不解析外部报告不读他包，零跨包耦合。时间维度族三件：停泊材料即材料五字段外加 parking 键含 entered_at 与 ttl_days，键缺省或非对象或字段非法时时间谓词一律判败即 fail-closed；time_deadline 到期判定即参照时间越过 entered_at 加 ttl_days 判败，到期日当日即败，缺省 route_on_fail 取 siding；parking_aging 在泊告警即告警位非路由位禁配 route_on_fail，参数 aging_threshold_days 为正数；gate_hold 搁置判定即 gate 键为对象且 fired_at 缺省判败走 route_on_fail 即未点火搁置，fired_at 在场判过交后续谓词定路，gate 键缺省或非对象判过即非搁置件零影响，只查结构不解读触发器语义，点火判定在别处。参照时间由命令行显式给参不读钟，承只读确定条款。

## 判定与告警 {#routing}

判定算法顺序钉死：谓词按包内声明顺序逐条评估，首败定路即首个失败谓词的 route_on_fail 决定材料去向，全过走 defaults 即 pass_route。逐材料输出全部谓词过败记录，机械透明可复核。

告警四件批级：siding 件数大于等于 siding_surplus_threshold 即有余告警，积压计数排除首败谓词 kind 属 [alarms] 节 siding_surplus_exempt_kinds 豁免清单的侧轨件即豁免只及指定 kind 的设计行为，缺省零豁免即全量计数，summary 的 siding 仍为全量侧轨数即豁免不隐藏停计事实；mainline 件数小于等于 mainline_starvation_threshold 即饥饿告警；轮结论 kind 为 establishes_term 且其 term 撞任一回执 flagged_terms 即登记冲突告警，形态含 kind、term、conclusion；parking_aging 即包内声明该谓词时参照时间与 entered_at 差值达 aging_threshold_days 逐件产在泊超期告警，形态含 kind、id、age_days、threshold，未点火搁置件即 gate 键为对象且 fired_at 缺省零告警即停计，siding 计数仍可见即停计不隐藏。告警入报告并置退出码一，告警是事实不是动作，本工具不因告警改路由不因告警拦批。批级告警仅在非空批产出即空批零告警，防空批饥饿误报。

## 机器形态 {#machine}

五件承范式：空腹即包承载全部判定知识；单子命令即 selector route；json 报告加退出码三值即 0 路由毕无告警、1 路由毕有告警含登记冲突、2 包非法或材料缺失或解析失败；双版本戳加治理域入报告头；只读确定即零写路径、同输入双跑逐字节一致、除命令行参数不读环境。材料为 JSON 记录，字段五项即 id、path、anchors、requested_writes、state，轮记录加 round 键，停泊材料加 parking 键含 entered_at 与 ttl_days，字段与结构见任务包 selector-round-level-t6d.md 2.1 与 2.2 节，冲突时以本契约为准。route 收可选 --reference-time 参照时间参数即 ISO 日期，时间谓词专用，不读系统钟；给参时报告头增 reference_time 键，未给参不增即旧输出形态不变。停泊材料在 routed 输出携带 parking 真值标记，普通材料不携带。MATERIAL 位置参数为 JSON 文件或目录，目录即展开其下全部 json 子件，零材料即空批为合法绿态零路由零告警退出码零，路径缺席即退出码二。

## 验收判据 {#criteria}

累计十四条全可证伪。首跑八条即任务包 selector-cutout-t6d 第四节 F1 至 F8：F1 三路各有一件判定符合；F2 四谓词各正反例反例走各自败向；F3 双败取包序首败；F4 有余告警阈值触发退出码一；F5 饥饿告警同；F6 空包全走 pass_route 零告警；F7 双跑逐字节一致加源码零写调用；F8 八件过程件基线全路由入档。二跑九条即任务包 selector-round-level-t6d 第四节 F1 至 F9：F1 孤儿路径标本判败走 siding；F2 密度阈值达标线；F3 缺回执判败；F4 空清单判败；F5 冲突标本出 registry_conflict 告警退出码一；F6 旧件零伤即旧测零改全绿旧输出逐字节不变；F7 轮语料双跑一致源码零写；F8 空腹轮包全走 pass_route；F9 第五跑真实轮记录路由入档。三跑三条即任务包 parking-selector-time-t6d 第四节 F-2 与 F-3 与 F-5：到期判败未到期过缺键缺参判败、在泊超期逐件告警与同输入同参照时间双跑逐字节一致、五住户基线实跑路由入档。验证以集成测试与基线实跑承载，结果文件逐条列表。

## 融回门 {#merge}

三查对表：验收判据八条逐条过；接口契约未变即以本文件为比对基准；回迁债评估含引擎侧接入成本。融回评估即体量与时间两重门槛，主判据贡献度，空转调用量不计入，达标由用户裁量。逐轮判定族的后续契约另立，不挤占本契约版本。

## 载体引用 {#carrier}

谓词划分语义承载 ALG-002 等价关系与商集隔离（sih-math/algebra/entries/ALG-002-equivalence-relation-and-quotient-isolation.md，mapping.md:197）。三类语义形式化即谓词族诱导等价关系（同判定路即同类）、三路即商集三分块（完备且互斥，三路必居其一且仅一）、代表选取与规范形（routed 条目即类规范形，route 即商映射像，failed_predicate 即类边界谓词），推导档 sih-math/docs/selwire-selector-derivation-2026-09-04.md，源码判定位锚点在 selector/src/selector/pack.py（load_pack 谓词装载位）与 selector/src/selector/route.py（route_material 逐件判定位与三路归位位注释锚点）。词面谓词划分在数学仓 mapping 零命中已显式申报，判归商集第二消费位承 idwire 双载体先例（ALG-002 第二消费面）。本批为接线与推导非行为变更，判定行为零改动。

## 修订记录 {#revisions}

2026-09-08 修订七：工具升 0.6.0，有余告警豁免语义登记在案即 [alarms] 节新增可选键 siding_surplus_exempt_kinds（缺省空即零豁免向后兼容，值须为合法谓词 kind 字符串数组否则拒包 fail-closed），首败谓词 kind 属豁免清单的侧轨件不计入有余告警的积压计数即豁免只及指定 kind 的设计行为，summary 的 siding 仍为全量侧轨数即豁免不隐藏停计事实，parking 谓词包升 0.4.0 声明 gate_hold 豁免即 gate 远景件入侧轨是设计行为非积压，点火后 gate 因 time_deadline 首败落侧轨不豁免即真事件计入，既有谓词语义与三路枚举零改即旧包旧输出逐字节不变，新增八测合计一百五十九测全绿，承用户 2026-09-08 编排令即 parktune-solo 批泊界侧轨积压告警调优，误报灭真报留承基线三注意力只投异常。

2026-09-07 修订六：工具升 0.5.0，搁置谓词 gate_hold 登记在案即 gate 键为对象且 fired_at 缺省判败走 route_on_fail、点火在场判过、缺省或非对象判过零影响，parking_aging 对未点火搁置件停计即零老化告警而 siding 计数仍可见，parking 谓词包升 0.3.0 加 P104 前置于 P102 即首败定路使搁置件到期免判，三路枚举与既有谓词语义零改即 PRO-007 词条三路承接面零触碰，承用户 2026-09-07 令泊界升级带锚落地即 parkgate-solo 批，gate 语义锚挂 ORD-022 邻接锚即回升必经 ρ 门控回升，乙类零命中维持候选在 sih-math/docs/anchorwave-parking-course-2026-09-07.md。

2026-08-22 修订一：契约初版随第五跑 T6-D 批起草，同日实现与八判据验收毕，基线首跑入 CALL-LOG。

2026-08-22 修订二：升 0.2.0，按轮判定的谓词族四件登记在案即锚定密度、常设约束核对、断言须附清单、登记冲突告警，批级告警由两件扩为三件，轮记录 schema 与 fail-closed 语义随案，回执自包含零跨包耦合，登记冲突告警以包声明为门即空腹纪律。轮记录件在 routed 输出携带 round 真值标记，普通材料不携带以保旧输出逐字节不变。承任务包 selector-round-level-t6d.md，向后兼容为硬约束即旧测零改旧输出逐字节不变。

2026-08-25 修订三：升 0.3.0，时间维度谓词族两件登记在案即 time_deadline 到期判定与 parking_aging 在泊告警，批级告警由三件扩为四件，parking 谓词包即 packs/parking 首建，停泊材料 schema 与 fail-closed 语义随案，告警以包声明为门。参照时间由 route 的 --reference-time 显式给参不读系统钟，给参时报告头增 reference_time 键未给参不增，停泊材料在 routed 输出携带 parking 真值标记，旧包旧输出逐字节不变。承任务包 parking-selector-time-t6d.md，新增十三测合计一百三十七测全绿，向后兼容即旧测除只读白名单登记 datetime 标准库条目外零改。

2026-08-25 修订四：升 0.4.0，MATERIAL 位置参数收目录即展开其下全部 json 子件，心跳与常规调用同一形态免疫 shell 通配语义，零材料即空批为合法绿态零路由零告警退出码零，批级告警仅在非空批产出防空批饥饿误报，非空批行为逐字节不变。承泊界心跳零在泊态实跑暴露三断点即出泊清残删目录、空 glob 中止、零参数拒载，新增三测合计一百四十测全绿。

2026-08-25 修订五：parking 谓词包升 0.2.0，治理域声明扩域盖全态泊界即 include 增 ../sih-engine/parking/** 与 ../sih-engine/doc/governance/PARKING-v1.md，域为声明入报告头不拦材料路径即谓词语义零改，工具版本不动即 0.4.0。承 2026-08-25 用户令建全态的向与泊界，引擎侧全态泊界首建随全态向界 GOV-003 同批接线，心跳双线双跑同款即工具线 parking/materials 与全态线 ../sih-engine/parking/materials 各跑一次。停泊名册单一名册续号即自 pk-013 起与工具线共链不重号。
# 引擎路择契约（attractor route）

> 家位 sih-engine/src/attractor/route.rs，承 SPEC-015 谓词融回规格与 autoflow2-solo 批实装；本册随 parktune-solo 批（2026-09-08）新建，承载引擎侧路由谓词机对表基准与修订记档。围堰 sih-tools/selector/CONTRACT.md 是谓词语义的权威正典，本册只记引擎侧对表实态与两侧同步义务，语义冲突以围堰册为准。

## 对表基准 {#baseline}

- 围堰现行文即 sih-tools/selector/src/selector/{predicates,route,pack,cli}.py 四件源码与两包四件纯数据，十一 kinds 与 fail-closed 语义与 route_on_fail 缺省与批级告警逐条对表，零语义漂移。
- 报告头 tool.version 承围堰 __version__ 即 0.4.0，工件工具名字段是契约面标识非二进制身份。
- 错误信封承围堰 f-string 原文形；toml 与 json 解析失败的异常报文是运行时差异面，拒收退出码与信封形态对表，此为显式范畴排除（SPEC-015 原条款）。
- 包纯数据随迁 src/attractor/packs/{core,parking}/ 与围堰逐字节一致即同参形构成条件，T0 测试钉死。

## 有余告警豁免语义 {#surplus-exempt}

承围堰修订七（parktune-solo 批 2026-09-08）：[alarms] 节可选键 siding_surplus_exempt_kinds（缺省空即零豁免，值须为合法谓词 kind 字符串数组否则拒包 fail-closed），首败谓词 kind 属豁免清单的侧轨件不计入有余告警的积压计数，summary 的 siding 仍为全量侧轨数即豁免不隐藏停计事实。parking 谓词包 0.4.0 声明 gate_hold 豁免即 gate 远景件入侧轨是设计行为非积压，点火后 gate 因 time_deadline 首败落侧轨不豁免即真事件计入。误报灭真报留承工程基线三注意力只投异常。

## 修订记录 {#revisions}

2026-09-08 修订一：本册新建随 parktune-solo 批。三件同步落地：其一有余告警豁免语义与围堰修订七逐条对表（parse_exempt_kinds 与 route_batch 积压计数排除段）；其二追平 parkgate-solo 批遗留债即 gate_hold 谓词（ALL_KINDS 十一件、check_gate_hold 结构判定、evaluate 分发、无参白名单）与 parking_aging 对未点火 gate 停计（围堰修订六语义引擎侧补齐）；其三金向量按 SPEC-021 T9 基线种随冻重录即期望由围堰 Python 原件跑出重冻、消费逻辑零改，重冻依据即 parking 包 0.3.0 升 0.4.0 与 ALL_KINDS 报文扩 gate_hold 与告警 count 语义三面合法变更的纯期望过期。t6 装配位谓词数断言随包四谓词适配（3 至 4，声明变更非违规）。
# parktune-solo：泊界侧轨积压告警调优——gate 远景件豁免计数

> 治理任务包（实装类，单线形 solo，委外代理亲写零子代理，DEC-018）
> 承接：parkgate-solo 批（2026-09-07 落 gate 远景廊）后遗留配置性误报——带 gate 的远景件 pk-070 与 pk-077 落 siding 轨触发 siding_surplus 告警（阈值二，现侧二即告警），gate 件入侧轨是设计行为非积压，主会 2026-09-08 编排令即 FORK-2 窗口段
> 并行协调：reroute-solo（FORK-1）在飞或即将开租约，本批写入面与其零交集；废轨修复与 confpreempt 收口不属本批
> 日期：2026-09-08

## 一、问题陈述 {#problem}

- 路择 parking 包（双载体逐字节同源即 sih-tools/selector/packs/parking/ 与 sih-engine/src/attractor/packs/parking/，SPEC-015）告警语义：siding_surplus_threshold=2 把侧轨件数超阈当积压告警，但 gate 远景件（出泊条件挂未来事件者）入侧轨是 parkgate 设计行为，非积压——每日心跳误报一条，稀释告警可信度（狼来了效应，违基线三注意力只投异常）
- 修复语义须保真：真积压（非 gate 件侧轨超阈）仍须告警；豁免只及 gate 件

## 二、关键设计 {#design}

- **先勘后改**：实读 routes.toml 与 manifest 与 pk-070/pk-077 材料实态字段（gate 形态以盘面为准），确定 gate 件豁免可否由包数据表达（如谓词重排或 alarm 节扩展）；若包数据不可表达即改引擎告警逻辑（围堰 selector 与引擎 attractor/route 两侧同改），数据形与代码形选择及依据逐条落批材料，不猜
- **双载体纪律**：SPEC-015 双跑对表——围堰包与引擎包改动后须逐字节一致（cmp），引擎包经 include_str! 编译期内嵌须 cargo build 重编主树后再验；金向量八场景若因包数据合法变更而变，按 GV3 随冻重录（期望重冻、消费逻辑零改），重冻依据落材料
- **验收读数**：改后 --reference-time 2026-09-08 路由两线，pk-070/pk-077 仍在侧轨（或其设计轨）但 siding_surplus 告警零；构造真积压夹具（三件非 gate 侧轨件）证明告警仍触发——红绿证俱在档

## 三、工作清单 {#work}

- [x] 勘察：routes.toml 与 manifest 现文、pk-070/pk-077 材料字段、双载体 cmp 基线、金向量现状
- [x] 温故检索（落包前主会编排令在档；本批自跑 recall --topic 补档，零命中如实记）
- [x] 三问双门→叩问→正身→租约→意图→工地施工（数据形或代码形按勘察定）
- [ ] 管线三步→checkcite→认证→双仓 settle→放锁 close→对账对表→verify→回填 bypass→CALL-LOG
- [ ] 结果档 parktune-solo-results.md 落 event/plan

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 误报灭真报留 | 实装 | 改后实跑两线路由：pk-070/pk-077 在侧轨零 siding_surplus 告警；真积压夹具≥三件仍触发告警，红绿证在档 |
| **F-2** 双载体一致 | 跨族治理 | 围堰与引擎包 cmp 逐字节一致（或两侧代码同改后同参双跑输出一致），引擎重编后验证，金向量按 GV3 处置在档 |
| **F-3** 双跑确定性 | 跨族治理 | 改后同参同日双跑 cmp 逐字节 IDENTICAL 退出码一致 |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径，reroute-solo 面（GOV-002、critsweep、facet、DES、引擎泊材料）零触碰 |
| **F-5** 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 较批前零新增 |

## 五、必读文件 {#read}

- `sih-tools/selector/packs/parking/routes.toml` 与 `manifest.toml`；`sih-engine/src/attractor/packs/parking/`（双载体）
- `sih-engine/sih/state/parking/materials/pk-070.json` 与 `pk-077.json`（gate 字段实态）
- `sih-tools/BATCH-FACE.md`（路由调用面节、坑位勘误全节、金向量重放 cwd 约定）
- 先例：`sih-engine/sih/event/plan/parkgate-solo-results.md` 与 `critsweep-solo-results.md`

## 六、约束 {#constraints}

1. 零子代理；.session-anchor.md 与 .zcode/config.json 与 anchor.py 零触碰（跨窗口竞态在案）
2. reroute-solo 在飞面零触碰；撞锁不绕行走 wait-turn 或候批重试，禁 bypass 抢锁
3. 主树零直写；禁 plain commit；禁管道掩退出码；先红留痕；md 认证走内容哈希清单形
4. 泊材料本体（pk-070/pk-077 等任何在泊件 JSON）零改动——本批只调告警语义不改泊件
5. 真积压判定阈值与 gate 判定字段名以勘察为准，语义变更落两册 CONTRACT（selector 与 attractor）修订记档

## 七、验收标准 {#acceptance}

F-1 至 F-5 全过；收约后零本批活跃锁零活跃会话；结果档落 event/plan；改后心跳读数（引擎线告警零）入结果档。

## 八、风险点 {#risks}

- 引擎代码改动牵金向量重冻与 cargo 重编，批期变长——数据形可表达即优先数据形，取舍依据落材料
- reroute-solo 若与本批争报告目录锁即 append 形互不憋或排队候叫，如实记等待

## 九、范畴排除 {#exclusions}

- 废轨 9 件修复（撞 reroute 泊材料面）、confpreempt 收口（候 W1 后）、gate 谓词语义扩展新功能、critsweep 术语收编，皆不属本批

## 十、请求写入 {#requested-writes}

- sih-tools/selector/packs/parking/
- sih-tools/selector/src/selector/
- sih-tools/selector/pyproject.toml
- sih-tools/selector/CONTRACT.md
- sih-tools/selector/tests/
- sih-engine/src/attractor/packs/parking/
- sih-engine/src/attractor/route.rs
- sih-engine/src/attractor/CONTRACT.md
- sih-engine/src/attractor/fixtures/route/
- sih-engine/tests/attractor_route.rs
- sih-engine/sih/state/plan/parktune-solo.md
- sih-engine/sih/event/plan/parktune-solo-results.md
- sih-engine/sih/event/plan/parktune-solo-materials/
- sih-engine/sih/event/trail/2026-09-08.ndjson
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- worktrees/sih-tools/parktune-solo
- worktrees/sih-engine/parktune-solo

> 2026-09-08 勘察修订（批内修订，承第二节先勘后改条款）：原表漏列四路径即 sih-tools/selector/src/selector/（告警消费逻辑 route.py 与 pack.py 在此）、sih-tools/selector/pyproject.toml（版本进位惯例）、sih-engine/src/attractor/route.rs（引擎侧同改面）、sih-engine/tests/attractor_route.rs（t6 谓词数断言随包四谓词适配）。勘察定夺数据驱动的代码形即纯包数据不可表达豁免语义，代码路径为代码形必写面，修订依据落批材料与结果档越线申报节。
