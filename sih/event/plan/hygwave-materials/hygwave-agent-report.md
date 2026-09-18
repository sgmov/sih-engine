# hygwave 批子代理施工归报

## 件一：泊界材料面卫生

处置前后读数对照（selector route --pack src/attractor/packs/parking --reference-time 2026-09-18）：

| 线 | 处置前 | 处置后 |
|---|---|---|
| 引擎线 sih-engine/sih/state/parking/materials | mainline 60 / siding 16 / scrap 23，告警 siding_surplus 12 阈 2，exit 1 | mainline 1（pk-078）/ 0 / 0，零告警，exit 0 |
| 工具线 sih-tools/parking/materials | mainline 25 / siding 4 / scrap 1，告警 4 阈 2，exit 1 | mainline 1（pk-026）/ 0 / 0，零告警，exit 0 |

诊断：逐件对照 trail 泊界事件（只读），真在泊仅 pk-026（08-28 入泊）与 pk-078（09-08 入泊），与名册在泊名录同判；其余 127 件全部链上已出泊（多数 09-17 名册同步日集中出泊）。侧线全是 P102（出泊后 TTL 到期跌落，或基件缺 parking 字段而伴件证已出泊，如 pk-091~100 系）；废件道全是 P101（旧记录形 entry_id/action 缺 id/path/state）。

处置（保守序）：引擎线 98 件、工具线 29 件挪位至两线新建 parking/historic（取名承名册 09-08 语义注丙案原词），两件真在泊留原位。零删除、零改名、零元数据改动；PARKING-v1.md、trail、租约台账全未动。逐件清单（文件、id、处置前路由、失败谓词、链账末态、disposition、exit 伴件）129 行落 disposition-table.md。

报告落位：hygwave-materials/hygwave-materials-hygiene-2026-09-18.md，化格 0 检词 0（清单表同过）。

## 件二：watchcheck 冗余 acquire 台账卫生读数

改动点（仅 src/bin/watchcheck.rs，围堰零触碰）：
- 顶置 REDUNDANT_ACQUIRE_THRESHOLD: usize = 3（冗余 >3 才呈报，形承 LOCKFACE_WIDE_THRESHOLD 先例，canon 头注指 hygwave 批加 faceprecise-analysis.md §1.4）
- redundant_acquire_alarms：按 (path,session) 回放原始流 acquired；max_repeat_acquires 即单路径最多笔数（对齐 §1.4 重复至 17 次），redundant_total 即减一求和（对齐 §1.4 朴素回放 112）；批名自同目录 sessions.ndjson issued 事件反查，缺席记未知；只报不裁
- judge 重构为 compute_reading 加 render_text（文本面逐字保形）加 render_json；CLI 增 --json 机读面，卫生清单挂可选键 redundant_acquires（零冗余不出键，向后兼容）；文本面净态与无主两分支各增呈报节

红转绿：tests/watchcheck_redundant_acquire.rs（canon 头注在位）改造前红 4/4（--json 未知旗标退 2、文本面无卫生节）改后绿 4/4（清单正确、零冗余键不出、阈值边界 3 不报 4 报、package 反查与未知兜底）；金向量 mergeall_t4_watchcheck 3/3 绿（文本面零回归）。

真台账读数（sih-tools/lease/ledger/locks.ndjson，exit 0）：23 会话超阈呈报，预期件 idwire-solo 28d326f924916cd0 在列（package=idwire-solo，max_repeat_acquires=17，redundant_total=112），与 faceprecise §1.4 实证逐字吻合；榜首其后为 closegate-solo（20）、leaseoptsettle-solo（16）、scriwire-solo（12）等。

## 全量回归读数

lib 段：228 passed 30 failed 8 ignored——30 败全在 testhard 批域（本批零触碰，工地基点在 testhard 归并前，主树同基点 39/39 全绿）；event_stream 9 件 scribe 未构建同因且主仓同基点同 9 件同败即基点既有与批无关。集成段（66 目标）：264 至 268 过；watchcheck 两目标终态 4/4 加 3/3 绿。lease 4 目标（8 件）与 gap_parser_pack_assets（2 件）在共享 target 纪律下败——主仓同基点与缺省 target 布局复验全绿，坐实为既有环境耦合（manifest 寻址与批纪律冲突），非本批引入。工作树 diff 终态：M src/bin/watchcheck.rs 加新增 tests 加 materials，无未意改动。

## 偏差申报

1. 与 09-08 甲案裁定张力（最重要）：名册心跳节载 acceptclose-solo 批三选项呈报（甲留册计 mainline、乙谓词分轨、丙移档 historic），甲案经 m-adjudicate2-a1 终签、乙丙不采。当时出泊件 TTL 多未到期；其后集中出泊加 TTL 陆续到期方成今日病灶。本批按 2026-09-18 批令执行丙形挪位，与甲案构成事实修订，候人节点追认；若追认宜补记名册语义注。
2. 材料元数据陈旧未改（挪位件内 state 多仍标 parked）：以挪位出心跳面承载卫生，件内补齐留候有令再议。
3. pk-026 TTL 至 10-27、pk-078 至 10-08，到期未出泊将按包语义复报提醒，属既定行为。
4. 件二挂现有 check 读数 JSON 可选键：现存 CLI 只有文本面并无既有 JSON 读数面，按可选键加向后兼容语义新增 --json 面（缺省文本面不动，金向量逐字对表为证）——对任务假设的如实偏差。
5. 共享 target 跨源树干扰：批纪律共享 target 与主仓构建交错时同 metadata 哈希实例间存在陈旧工件复链（曾令新测假红）；已以强制重建加缺省 target 布局复验收口，建议后批留意的坑位注记。

## 主会移植注记

子代理将 127 件挪位做在主树（工地纪律违，零内容损害），主会移植：挪位面已原样移植入两租约工地（engine 99 行加 tools 30 行含 historic 目录行），两主树已还原净位（checkout 加删 historic），挪位随本批入典。
