# c2close2-solo 腿三读数：投影器源码级五源复核（含 sweep 出件消费通道与红灯语义现态）

> 域外证据只读引用零治理主张。sih-visual 全域零触碰零写入，本批零运行 projector（其唯一写位在 sih-visual 域内自身 data.js），本件仅载指针与源码级读数与交付态 data.js 只读对表。
> 复核基线即 c2close-solo 腿二指针件（三源直读）与 sweepjson-solo 腿四接线（第五源）与红灯语义修复（收约即消解、活红限在飞末笔）。

## 一、视图件指纹（复核时点）

| 件 | 路径（只读） | sha256 | 规模 |
|---|---|---|---|
| 投影器 | sih-visual/assets/viewer-dashboard-2026-09-06/projector.py | 8cef84834c2db7e169b0ffb209c7c01048da3e87ee59d6629d12be6ee12227dd | 634 行 |
| 人类视图 | sih-visual/assets/viewer-dashboard-2026-09-06/dashboard-v3-flow-console.html | a39502d0694d7b9697c5c48a457a81cf7021dbdd70f677e6f262dd957abd5558 | 异常台默认页 |
| 投影产物（交付态快照） | sih-visual/assets/viewer-dashboard-2026-09-06/data.js | 1f1492c615fbf100a53ac6f1373a5a325512737edddca265905acb8f00dd878b | refTime 2026-09-08T13:55:00Z |

## 二、projector 五源行级指针（源码级确认直读五源）

| 数据源 | projector.py 行级指针 | 确认事实 |
|---|---|---|
| 引擎链文件 | L36 TRAIL_DIR 即 sih-engine/sih/event/trail；L171-174 trail_files 取 *.ndjson 且排除 *splinter* 旁链；L184-307 逐件逐行读事件，L190-195 prev_hash 链连续校验 | 直读引擎链文件全量，链断读数入 aggr.chainBreaks |
| 租约会话账本 | L37 SESSIONS 即 sih-tools/lease/ledger/sessions.ndjson；L177-182 sid2pkg 取 issued 行；L294-307 open_sessions 按 issued/revoked 事件序判在飞 | 直读租约会话账本，在飞判定与 lease 会话台账同一事件序视图 |
| 引擎线泊材料 | L38 PARK_ENG 即 sih-engine/sih/state/parking/materials；L311-332 scan_park 判在泊与出泊与超期（ttl_days 对 entered_at） | 直读引擎线泊材料目录 |
| 工具线泊材料 | L39 PARK_TOOLS 即 sih-tools/parking/materials；L338-341 双线扫描与合计 | 直读工具线泊材料目录，超期判定与泊界心跳同源 |
| 哨检出件（第五源） | L42 SWEEP_OUT 即 sih-tools/lease/reports/sweep-latest.json；L343-367 sweep_view 装配，L349-353 缺档或不可解析即 face=absent 零显示，L362-367 逐字段照录判定原文即类与对象与态与判据与通道五字段 | 直读哨检出件位，视图零判定零改写零汇总解释，判定语义正典在 lease sweep（docstring L21-23 与注释 L344-345 同载） |
| 写出面 | L45 OUT 即本目录 data.js | 唯一写位在 sih-visual 域内自身目录，不写治理面 |

## 三、黄灯面消费通道复核（条件二构造性对齐）

- 消费形：L346-348 sweep_view 缺省 face=absent 带 src 与 note 字段；L354-359 在场时透传 tool 与 at 与 verdict 与 exit_code 与 summary；L362-367 items 按 SWEEP_CLASS_ORDER 五类序逐条照录哨检判定原文，视图零自判（docstring L21-23 判定规则同步登记）。
- stdout 报告行：L624-627 哨检出件行即在场相呈 verdict 与判定原文条数、缺档相呈「缺档零显示」。
- 交付态 data.js 只读对表：sweep face=present、verdict=residue、at=2026-09-08T13:30:00+00:00、items 一条逐字段照录即 stale_check_file 与对象 sweepjson-solo.json 检验文件与态 awaiting 与判据「pid 探针四态=pid_dead 而窗会话在册活跃或仍持锁（防搁浅守卫）」与通道 human-takeover，视图零判定。快照时点申报同 sweepjson 先例：正典消费位 sweep-latest.json 系 2026-09-08T13:30Z 快照，投影快照按需经 lease sweep --out 再生属设计语义非缺陷。

## 四、红灯语义现态读数（收约即消解与活红限在飞末笔）

- 源码位：L264-273 报告现态制即每份报告现态取其最后一次认证退出码（trail 时序 last-wins）；L413-414 注释载「收约即消解：已收约批的红证全部转历史注记（链的收约闸即消解判定）」；L415-420 即 flying 批 viol 取 last_cert_rc 等于 1 否则零、closed 批 viol 与 findings 俱归零；L426-431 note 呈「活违例 · 末笔认证红」与「历史红 N 随收约消解」双形。
- 交付态 data.js 只读对表：violBatches 0（零活红）、chainBreaks 空、overdue 空、aggr 批 319 与认证 1582；红灯历史语义即收约消解在役，历史红不点活灯属审计注记不入判据面，与命题第三分句同形。
- 视图异常台语义锚点复核（dashboard-v3-flow-console.html 只读）：L482 红灯区违例批、L491 黄灯区在飞未收与超期泊件与域外壳、L507 域外壳注文、L1932-1933 实机异常面、L1980 异常台页零注文，俱在位。

## 五、结论

条件二绿：projector 源码级确认直读引擎链与会话账本与双线泊材料与哨检出件五源，行级指针在档；黄灯面消费通道即 sweep 出件在场逐字段照录与缺档零显示双路在档；红灯语义现态即收约即消解与活红限在飞末笔在源码与交付态读数双确认。本批对 sih-visual 全域零触碰零写入零运行。
