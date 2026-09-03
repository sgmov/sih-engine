# facepark-solo 任务包：台面小清批——BATCH-FACE 漂移修、泊材料对表、OTel 入泊候选

- 性质：登记与对表类小批。三件全走确定性通道（修订四；承 pendsweep-solo 先例），facet 零采样；机械门即事实实跑核验加双跑留证加 findings 亲读。
- 意图来源：2026-09-03 用户令，inputlog 逐字在 dispatch。
- 会话：sess-zcode-260903-facepark。trail：sih-engine/sih/event/trail/2026-09-03.ndjson，起链长度以开工时 wc -l 为准，在途尾随承先例如实申报。
- 队形：单线 solo，构造全在 worktrees，主树零直写（状态面除外，见禁区说明）。

## 背景（三条线索，执行时以实跑实查为准）

- 线索一：pendsweep-solo 批申报 BATCH-FACE 两处漂移——digest 实形含 --signals 与 --contract；meter 0.2.0 无 --quiet（而 BATCH-FACE L81 坑位行、L288 速查行、L294 清单行仍按有 --quiet 表述）。申报是线索不是结论，落笔前必须实跑核验。
- 线索二：泊界心跳 2026-09-03 读数——引擎侧在泊材料缺 pk-037、pk-039、pk-045 三件 enter json（链上停泊事件在档：pk-037 即 35ecddff、pk-039 即 684cf410、pk-045 即 c75f7401）；工具侧 pk-042.json 谓词 P101 不过（schema_required：id、path、state 三字段）走 scrap_track；pk-013-exit.json 在工具侧材料无 enter 伴件走 scrap_track。
- 线索三：用户令 OTel 记泊界候选——视图层借鉴 OpenTelemetry GenAI 语义约定（gen_ai.*）作为视图组件契约参考面的候选，入泊登记。

## F 清单

- F-1 BATCH-FACE digest 段（L63 起至该节止）与实跑一致：先实跑 elicit digest --help 加一次真实小调用定格实形，再修文档；命令行双跑留证（修前修后各一），输出路径记结果档。
- F-2 BATCH-FACE meter --quiet 三处（L81 坑位、L288 速查、L294 清单）与实跑一致：实跑 meter --help 与 meter --quiet run -- true 探针定有无；无则三处改为如实表述（uv 日志对治写 2>/dev/null 形），有则保留并修正漂移描述。
- F-3 引擎泊材料补三件：pk-037.json、pk-039.json、pk-045.json 落 sih/state/parking/materials/，逐字段取自链上停泊事件加名册行（entry_id、title、exit_condition、entered_at 东八区、ttl_days、path 指名册、state parked），形态镜像同目录 pk-017.json；禁自造字段禁外链。
- F-4 pk-042.json 整形：对照 P101 三字段与同目录良件形态补齐或修正；修后工具侧心跳 pk-042 告别 scrap_track。内容语义零改，只修 schema 面。
- F-5 pk-013-exit.json 归位判读：核其内容对链上 pk-013 出泊事件；按证据判工具线或引擎线归位（同族 exit json 现居工具侧材料），证据不足则原位不动并如实申报判读过程。
- F-6 pk-046 入泊：条目即视图层 OpenTelemetry GenAI 语义约定（gen_ai.*，agent 与 MCP 遥测提案中）作为视图组件契约参考面候选；出泊条件即视图组件契约起草批开工时用户裁参考与否，裁参考即入契约参考材料面，裁不用即废弃；ttl 30 天。动作三件套：引擎 scribe park 一笔入 09-03 链、引擎名册当前在泊行五改六（一句界定加出泊条件加 ttl 加入泊事件哈希）、materials json 一件（F-3 同款形态）。行文零网络外链，OpenTelemetry 写全称。
- F-7 心跳复跑双线对表：引擎侧 pk-037、pk-039、pk-045、pk-046 四件可见；工具侧 pk-042 路由 siding；siding_surplus 告警读数如实转述。已知局限登记为观察项不在本批修：已出泊件（pk-013、pk-015、pk-041、pk-043）材料仍在、路由面不区分已出泊，心跳对在泊数只会偏高不会漏报。
- F-8 收口：双仓 settle --cert、reconcile 四类双零、当日链 verify、调用册逐工具一行、结果档 F 表写全、全部产物含链尾随批入版控。

## 管线

化格在核前：引擎 doc/governance/PARKING-v1.md 与工具侧 BATCH-FACE.md 与本批触及其他 md 走 uv run formatter --pack packs/general-v1（化格改动画 diff 亲读）。核阅：doc/governance/PARKING-v1.md 走 des-001 必须 0；BATCH-FACE.md 与结果档域外 exit-2 如实记不属违规。检词 core 零违例；新词（OpenTelemetry、GenAI、gen_ai 等）先 elicit check --words 定夺，过不了按检词流程登记或改写，词债不过夜。

## 请求写入（锁路径全集，全部显式 --session）

- /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md
- /Users/moc/workspaces/SiHankor/sih-tools/PARKING-v1.md（仅 F-5 判需归位时触及）
- /Users/moc/workspaces/SiHankor/sih-tools/parking/materials/pk-042.json
- /Users/moc/workspaces/SiHankor/sih-tools/parking/materials/pk-013-exit.json（仅 F-5 判归位时）
- /Users/moc/workspaces/SiHankor/sih-engine/doc/governance/PARKING-v1.md
- /Users/moc/workspaces/SiHankor/sih-engine/sih/state/parking/materials/ 下 pk-037.json、pk-039.json、pk-045.json、pk-046.json
- /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-03.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

- pk-037、pk-039、pk-042 出泊裁零执行——出泊唯人节点，本批只修材料与登记，不裁去向。
- 泊界链事件只增 pk-046 入泊一笔；pk-041、pk-043 出泊账已在链，零重放零改写；历史 trail 零改写只追加。
- selector 源码与 packs/parking 零改动；gauge CONTRACT 零碰；引擎组件源码零碰；金向量零碰；AGENTS.md 零碰；mathpipe 在飞件零碰。
- 泊材料 json 禁删除既有件；禁自造链上无据的字段；禁网络外链入档。
- identity/reports 不入版控；守卫在位严禁 plain git commit 直提；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即停批报告。

## 验收

F-1 至 F-8 全过；心跳双线读数对表（F-7 三点）；管线零违规；链 verify valid 且前后 wc -l 与末哈希对表留证；reconcile 四零。
