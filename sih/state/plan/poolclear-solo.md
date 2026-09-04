# poolclear-solo：泊界投影小清批（名册对链、出场件配对、pk-013-exit 处置、席位基线 core_hash 收尾）

- 令源：主会例行验收累积小伤池，用户「继续」推进下择并行窗口清账（mathscan-solo 施工面不相交）。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-poolclear）｜ 队形：单线形 solo，零子代理。
- 性质：投影整形与包规则注记级小批，行为变更仅 F-3 一处测试面（见下）。

## F 清单

- F-1 名册在泊段与全链直数对齐：全链 enter/exit 配对直数在泊十项（pk-016/026/039/042/044/045/046/047/048/049，实跑复核为准）——pk-026 与 pk-042 照链补行（逐字段抄停泊事件）、pk-037 行改出泊照录（pk037impl 实装承接，出泊事件实取哈希）、在泊计数与行序随档更新、历史段相应补记；名册过化格→核阅→检词三门。
- F-2 pk-051-exit.json 出场件补位：逐字段抄链上 pk-051 出泊事件（leasewire 泊位、basefix 出泊 f6b3293e），enter/exit 配对归真（gqueue 计数面随之归真）。
- F-3 pk-013-exit.json 处置：实查 selector 路由包对出场件的 P101 判定缺口（出场件 schema 与在泊件同查致 scrap_track），择一处置——包规则补出场件形态豁免（routes.toml 数据面）或照链补字段（链上 exit 事件有据方可）；择一申报依据，selector 源码零触碰。
- F-4 席位基线 core_hash 字段收尾：facet/probes/temp_probe score 落 core_hash 入基线 json（idcore-solo 消费侧已在、生产侧此为最后一环），TDD 先红后绿；存量基线文件零改写（下回合标定自然携带新字段）。
- F-5 写入仅 allow；判变申报：F-3 若走包豁免，路由判定面变化只及出场件类，在泊路由零变，如实申报。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package poolclear-solo → 取锁（施工面 exclusive 长持；共享追加面一律 --mode append 短持）→ meter 包裹引擎 scribe intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 append 主树活链 → settle 前一次性拷工地 → settle --cert（tools 与 engine；math 若零写入则双仓）→ 放锁 → close（备份让位归并对表法，lease 1.17.0 并集复查闸在位）→ reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；meter 包裹 2>/dev/null 对治；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/sih/state/parking/materials/（pk-051-exit.json 与 pk-013-exit.json 处置）
- sih-tools/selector/packs/parking/routes.toml（F-3 择包豁免时）
- sih-tools/facet/probes/（F-3 temp_probe 与测试）
- sih-engine/sih/event/plan/poolclear-solo-materials/ 与 poolclear-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

在泊路由判定零变（F-3 只及出场件类）；存量基线文件零改写；数学仓零碰；retriever 与 identity 与 tally 与 gauge 与 formatter 与 meter 与 nomenclator 与 parser 与 cascade 零碰；主树零直写；链文件禁非书简通道改写。
