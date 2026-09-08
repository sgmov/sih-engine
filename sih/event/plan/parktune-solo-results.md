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
