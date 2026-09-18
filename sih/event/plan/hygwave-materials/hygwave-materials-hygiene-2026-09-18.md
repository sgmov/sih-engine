# hygwave 批材料卫生报告（件一：泊界材料面）

- 批名：hygwave（卫生批，两件之一）
- 日期：2026-09-18
- 工作工地：/Users/moc/workspaces/SiHankor/w/hygwave（分支 w/hygwave，基点 522321a）
- 处置对象：引擎线 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/parking/materials 与工具线 /Users/moc/workspaces/SiHankor/sih-tools/parking/materials
- 工具位：sih-engine/target/debug/selector route --pack src/attractor/packs/parking --reference-time 2026-09-18

## 病灶

判据扫泊界心跳每会话必报告警：引擎线侧线 16 加废件道 23 告警 siding_surplus 计 12 阈 2；工具线侧线 4 加废件道 1 告警计 4。两读数本批首跑复现如实（见下）。

## 处置前读数（2026-09-18 实跑）

- 引擎线：total 99，mainline 60，siding 16，scrap_track 23，alarms [siding_surplus count 12 threshold 2]，退出码 1
- 工具线：total 30，mainline 25，siding 4，scrap_track 1，alarms [siding_surplus count 4 threshold 2]，退出码 1

## 诊断方法与判词

三路谓词（pack v0.4.0 为正典）：P101 schema_required（id 与 path 与 state 三字段须字符串，缺即废件道）；P104 gate_hold（gate 在场须带 fired_at，缺即侧线，豁免告警计数）；P102 time_deadline（parking 对象缺 entered_at 或 ttl_days 或已过期即侧线，计入告警）；P103 parking_aging 只产老化告警不路由（本批两线均无老化件，最老入泊 2026-08-25 距参照日 24 天，阈 45 天）。

逐件对照链账：sih-engine/sih/event/trail 全量 ndjson 泊界事件按 entry_id 汇总末态，只读。判词如下。

1. 真在泊仅两件：pk-026（工具线，2026-08-28 入泊，链上无出泊事件，名册在泊名录同判）与 pk-078（引擎线，2026-09-08 入泊，链上无出泊事件，名册在泊名录同判）。
2. 其余 127 件全部属于已出泊事项：链上均有 parking_exited 末态（多数 2026-09-17 名册同步日集中出泊，其余散布 08-24 至 09-13），其中 62 件有 exit 伴件在目录。侧线 20 件全为 P102 失败：出泊后 TTL 到期跌落（如 pk-046 对 09-17 过期、pk-040 对 09-16 过期）或基件缺 parking 字段而伴件证已出泊（pk-091 至 pk-100 系列十件）。废件道 24 件全为 P101 失败：旧记录形（entry_id 加 action 形，无 id 与 path 与 state）或 exit 伴件缺 schema 字段。
3. 名册与材料差异：名册散文节个别句（如 pk-074 子项一与二仍留泊为 09-07 时点快照）与链账末态不符，名册自称与链不符以链为准，本批照链处置。

## 处置（保守序，零删除）

- 引擎线 98 件挪位至 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/parking/historic/，留 pk-078.json 原位
- 工具线 29 件挪位至 /Users/moc/workspaces/SiHankor/sih-tools/parking/historic/，留 pk-026.json 原位
- 零删除，零改名，零元数据改动；PARKING-v1.md 名册未动；trail 与租约台账未动；挪位目录即 historic，取名承名册 2026-09-08 语义注丙案原词「移档归 historic 目录出心跳面」
- 逐件清单（文件、id、处置前路由、失败谓词、链账末态、disposition、exit 伴件）见同目录 disposition-table.md，共 129 行（127 挪位加 2 留架）

## 处置后读数（2026-09-18 实跑）

- 引擎线：total 1，mainline 1（pk-078），siding 0，scrap_track 0，alarms []，退出码 0
- 工具线：total 1，mainline 1（pk-026），siding 0，scrap_track 0，alarms []，退出码 0

告警清零达成，验收达标。

## 偏差申报

1. 与 2026-09-08 甲案裁定的张力：名册心跳节载 acceptclose-solo 批三选项呈报（甲留册计 mainline，乙谓词分轨，丙移档 historic 出心跳面），裁定甲案承 m-adjudicate2-a1 终签，乙丙不采，谓词零改目录零迁移。当时出泊件 TTL 多未到期计入 mainline 无告警；其后 09-17 集中出泊加 TTL 陆续到期，出泊件成批跌侧线方成今日必报警警病灶。本批按 2026-09-18 用户令执行挪位归档（丙形），与甲案裁定构成事实修订，候人节点追认或另裁；若追认，名册语义注宜补记本批为新裁。
2. 材料元数据陈旧未改：挪位件内部 state 仍多标 parked（实际已出泊），本批零元数据改动，以挪位出心跳面承载卫生；件内补齐（如旧记录形补 id 与 path 与 state）留候后续有令再议。
3. 到期复检提醒照常预期：pk-026 TTL 至 2026-10-27，pk-078 TTL 至 2026-10-08，到期若仍未出泊将按包语义复报侧线提醒，属既定行为非残留缺陷。
4. 挪位目录 historic 为新建面，两线各一，selector 心跳调用面（materials 目录传参）零改，判据扫与心跳调用形零影响。
