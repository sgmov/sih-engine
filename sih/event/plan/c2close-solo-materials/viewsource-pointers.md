# c2close-solo 腿二读数：人类视图与数据源指针核验

> 域外证据只读引用零治理主张。sih-visual 全域零触碰零写入，本件仅载指针与源码级读数。
> 视图落位指针即用户 2026-09-08 窗口交付件，交付语照录自任务包令源节即「视图测试版 dashboard-v3-flow-console（异常台默认页、红灯 12、焦点面、防爆炸带）与投影器 projector.py」。

## 一、视图落位指针与指纹

| 件 | 路径（只读） | sha256 | 规模 |
|---|---|---|---|
| 人类视图 | sih-visual/assets/viewer-dashboard-2026-09-06/dashboard-v3-flow-console.html | 3364845f0477f3852b92520b3f56e7fef05a054bd29f770befb06b5e093cd8dd | 2021 行 |
| 投影器 | sih-visual/assets/viewer-dashboard-2026-09-06/projector.py | 23eb36b1e45a92195b1924e7d177ae6e6f21293131c50936c622b9c5c2db2422 | 570 行 |
| 投影产物（交付态） | sih-visual/assets/viewer-dashboard-2026-09-06/data.js | f24e9748759ac0bc8bef0e179b9dd85657c6e38c48460977d1b876aa9da2af7c | 交付态 refTime 2026-09-08T10:21:08Z |

## 二、projector 数据源行级指针（源码级确认直读三源）

| 数据源 | projector.py 行级指针 | 确认事实 |
|---|---|---|
| 引擎链文件 | L31 TRAIL_DIR 即 sih-engine/sih/event/trail；L161-164 trail_files 取 *.ndjson 且排除 *splinter* 旁链；L174-270 逐件逐行读事件 | 直读引擎链文件全量（16 件，2026-08-22 至 2026-09-08，data.js meta.trail 在档） |
| 租约会话账本 | L32 SESSIONS 即 sih-tools/lease/ledger/sessions.ndjson；L166-172 sid2pkg 取 issued 行；L272-285 open_sessions 按 issued/revoked 事件序判在飞 | 直读租约会话账本，在飞判定与 lease core.active_sessions 同一事件序视图 |
| 双线泊材料 | L33 PARK_ENG 即 sih-engine/sih/state/parking/materials；L34 PARK_TOOLS 即 sih-tools/parking/materials；L289-319 scan_park 判在泊与出泊与超期（ttl_days 对 entered_at） | 直读双线泊材料目录，超期判定与泊界心跳同源 |
| 写出面 | L35 OUT 即本目录 data.js | 唯一写位在 sih-visual 域内自身目录，不写治理面 |

补充指针：L231-262 certification_completed 按 exit_code 分类即 1 为违例、2 为域外核；L477-489 SPC 违例按日聚合；L528-544 meta.rules 载判定规则文本。

## 三、视图件异常台行级指针（四告警语义落位）

| 语义 | dashboard-v3-flow-console.html 行级指针 | 数据流 |
|---|---|---|
| 违例批（红灯区） | L480 区标；L1968 reds 过滤 viol 大于零；L1988-1990 alRed 渲染 | projector L231-241 认证违例计数 → 批档 viol → 红灯行 |
| 在飞未收（黄灯区） | L489 区标；L1969 flys 过滤 st 等于 flying；L1992 黄灯行「在飞 · 会话开而未收 · 工地在途」 | projector L272-285 会话事件序 → 批档 st → 黄灯行 |
| 超期泊件（黄灯区） | L489 区标；L1970 ovd 取 aggr.overdue；L1993 黄灯行「泊件超期未裁」 | projector L289-319 泊料 ttl 判定 → 黄灯行 |
| 域外壳（黄灯区汇总） | L489 区标；L1971 shellBatches 过滤 tool 大于零；L1994 汇总行；L503 注文即「域外壳＝des-001 域外核阅 exit-2，非违规，只汇总计数不入行」 | projector L238-239 与 L243-244 exit_code 等于 2 计数 → 汇总行 |
| 灯号总判 | L1974-1976 红灯优先、黄灯次之、零异常绿灯 | 四语义汇入单灯号 |

数据装载链：L858 var VD 即 window.VIEWDATA（data.js 注入），视图只渲染投影件认证事实（L1964 注文「实机投影 · data.js 由 projector.py 确定性生成」）。

## 四、交付态读数对表（data.js 交付态）

| 读数 | 值 | 对表 |
|---|---|---|
| 意图批总数 | 309 | trail 全期投影 |
| 违例批（红灯） | 12 | 与交付语「红灯 12」一致 |
| 在飞批（黄灯） | 2 即 confpreempt-solo 与 packenv-solo | 交付时点会话事件序如实 |
| 在泊 | 54 · 超期 0 | 双线泊材料如实 |
| 链断 | 0 | 逐件 prev_hash 连续校验 |

## 五、结论

腿二绿：projector 源码级确认直读引擎链文件与租约会话账本与双线泊材料三源，行级指针在档；视图异常台四语义落位指针在档；写位仅在 sih-visual 域内自身 data.js，治理面零写。本批对 sih-visual 全域零触碰零写入。
