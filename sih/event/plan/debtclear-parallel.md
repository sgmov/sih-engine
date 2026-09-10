# debtclear-parallel 清账并联批任务包

> 形：并联 parallel。主线簇零——用户 2026-09-11 令「拉起子代理逐项清账，可以并行。另外还需要拉一个子代理，对司衡引擎内的写死路径再排查一遍」，三写簇一读簇全子代理各领，主线保留验收与结算与 commit 位。本工作区租约下并联形首例如实申报。令源二：pk-090 出泊条件「用户令出泊立批走引擎域纪律」由本令兑现。批机械链全序正典：sih-tools/BATCH-FACE.md，逐命令 verbatim 承接不重述。

## 欠账到簇映射

- 簇A nomsupply：pk-090 立名供给三件全量加甲乙定案落形 + mcpline test_heartbeat_happy 跨批红随批根因修复
- 簇B anchorcover：回锚链面覆盖越限告警，用户 2026-09-11 裁形即会话视图层跳出异常、扫批作废
- 簇C pathaudit：司衡引擎与工具写死路径只读排查，零写无租约
- 簇D tybound：judouwire 留债之 ty_bounds 深阶件兑现
- 不在本批：judouwire 批名债（用户裁留债不追认记档，零动作）；reinit 候裁候用户令；工作台 P0/P0.5（司梦域不归本工作区）

## 簇A nomsupply 批

批名 nomsupply，查册 unknown，开约 --new-stem 认领，概念锚申报：zh 立名供给，派生 nom 即 mcpnomgate 既立 nom- 语素、supply 即供给；无既裁 code 形即显式申报无承。正典输入：sih-engine/sih/state/parking/materials/pk-090.json 全文、sihankor-naming SKILL.md、sih-engine/doc/decision/017-wengu-naming.md、sih-tools/nomenclator/CONTRACT.md、sih-tools/lease/CONTRACT.md。

- 件一 naming_guide MCP 只读具 alpha 相：内容五段即立名五步形、DEC-017 修订四与五指针、检词六态语义、stem 闸拒教认领三语义、死档禁条；stdio 外部面 18 升 19、HTTP 18 升 19，面计数同步位即 EXPECTED_TOOLS_EXTERNAL 与 HTTP_ALPHA_TOOLS 与 test_write_matrix 与 test_web_smoke 与 test_stdio_smoke 与 AGENTS.md MCP 节计数与 SPEC-023 修订三与 mcpline README 与 AI-MANUAL alpha 表；mcpline 版本位 0.9.0 升 0.10.0 双点位
- 件二 nomenclator_query unknown 教学扩载：错误与空态载荷附立名程序摘要，stdio 与 HTTP 两面 core 共用
- 件三 检词包按域解析：canonical 域根优先域内包、回退中央包；两根分离循 stem 闸先例即码根 sibling 解析加包自数据根；fixture canonical 域测；first-domain 形零变
- 件四 nomenclator map --concept：语义映射报告只报不判，四段即六态查得与既裁 code 形与近邻词与出泊指针；CLI 形，MCP 投影候令不并入
- 件五 lease stem 闸甲兜底：--new-stem 认领载荷增甲表即概念锚 zh 加既裁 code 形或显式无承申报加语素派生；裸认领拒；指称完整与派生对表机械核，填不圆即拒；执法面零 LLM 判词位不动；乙前注入即取名前拉 map 报告入上下文，使用侧纪律写进 DEC-017 修订六
- 件六 test_heartbeat_happy 根因修复：domaware 后 days_since_last_snapshot 返 None；夹具按域件新路径播快照或工具显式教形，禁弱断言

F 锚定：F1 裸 --new-stem 开约拒且教学载荷含甲表三件；F2 全甲表开约过且回执载甲表；F3 unknown 查词返回含立名摘要；F4 fixture canonical 域检词解析域包且 first-domain 中央包路径零变；F5 map 报告四段齐；F6 naming_guide 两面在役且计数 19 与 19；F7 mcpline 全套绿含 heartbeat；F8 lease 全套绿。

## 簇B anchorcover 批

批名 anchorcover，查册 unknown，--new-stem 认领，概念锚申报：zh 回锚覆盖越限告警，派生 anchor 即 attnanchor 既立语素、cover 即覆盖。语汇取 gchart 族既立即越限与告警，温故先例 recall-alarm.json 在 debtclear-parallel-materials。

- anchor.py chain_line 扩域覆盖段：中央登记册 sih-tools/mcpline/ledger/tokens.ndjson active 域根去重；逐域近 7 日 git 提交日对域链 trail 覆盖日；有提交日无链日即洞；洞出告警段「越限告警：域 日 N 笔提交查无此人」；无洞行保持纯读数形零变；册缺席或零在役即无域段零崩如实
- F 锚定：F1 fixture 域有提交无 trail 告警段出；F2 域链齐纯读数形零变；F3 册缺零段零崩；F4 attnanchor 全套绿；F5 只读约束即 git log 与文件读外零命令零网络

## 簇D tybound 批

批名 tybound，查册 unknown，--new-stem 认领，概念锚申报：zh 全形约束，派生 ty 即 ty_bounds 域词、bound 即界。正典输入：sih-engine/sih/event/plan/judouwire-solo-results.md 行 34 登记与 sih-tools/parser 源。

- parser ty_bounds 深阶：现形单取 type 位；深阶取 for 全形即 T for Foo 之 T 位解析绑定；旧单形回归钉死；深阶登记即兑现
- F 锚定：F1 全形用例先红后绿；F2 旧单形用例零回归；F3 parser 全套绿

## 簇C pathaudit 只读

- 范围：sih-engine/src 与 src/bin 与 sih-tools 各工具 src（排 target、.venv、node_modules、worktrees、dist）
- 猎类：绝对 /Users 路径；工作区名常量 SiHankor 与 SiMuseor；first-domain 共存假设即 sih-engine 与 sih-tools 并置 join；sih/ 布局假设无视 canonical 形；端口主机越 DES-015 正典位；env 回退序交叉域；测试夹具真根
- 基线排除：domaware 三件即 derive_root 与 layout_form 与 rootanchor 域感知与 gauge 域件为已修基线只验不重报；DES-015 正典常量 8765 与 127.0.0.1 非患
- 产出：分级真患、有界、误报三级，逐条 file 与行号加患义一笔加修法一笔；报告落 /tmp/sih-pathaudit-2026-09-11/report.md 且回执载 Top 发现

## 全簇约束

- 工地 lease worktree，主树零直写；identity 正身先行；lease 开约收约解锁全带 --identity；锁冲突 wait-turn 或 60 秒轮询至多 30 分钟仍阻即如实回报停手
- TDD 先红后绿；触 lease 的测试显式传 --ledger 与 --locks 并 ROOTANCHOR_DISABLE_SELF_BOOT=1 防串真册
- 文档产出管线化格、核阅、检词三步序固定；des-001 域只盖 sih-engine/doc，域外目标 exit-2 如实记不属违规
- settle 双仓加链 verify 加 reconcile 加主树重跑验收；CALL-LOG 追加；results 档落 sih-engine/sih/event/plan/<批名>-results.md；温故 recall 件随批入各自材料
- 禁区：不出本簇请求写入范围；bypass 仅既有通道且 reason 必填；工具 exit 2 停手如实报；不 push 远端；不动他簇文件；不改 stem 闸 drop 集与 DES-016 外部域只教不拒；first_domain 形逐字节零变
- 回执必含：批名、会话号、认证哈希、测试数、F 逐条结果、改动文件清单、未决项

## 请求写入

- 簇A：sih-tools/mcpline/src/mcpline、sih-tools/mcpline/tests、sih-tools/mcpline/README.md、sih-tools/mcpline/AI-MANUAL.md、sih-tools/mcpline/CALL-LOG.md、sih-tools/lease/src/lease、sih-tools/lease/tests、sih-tools/lease/CALL-LOG.md、sih-tools/lease/CONTRACT.md、sih-tools/nomenclator/src、sih-tools/nomenclator/packs/core、sih-tools/nomenclator/CONTRACT.md、sih-engine/doc/decision/017-wengu-naming.md、sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md
- 簇B：sih-tools/attnanchor、sih-tools/attnanchor/CALL-LOG.md
- 簇D：sih-tools/parser、sih-tools/parser/CALL-LOG.md
- 主线：sih-engine/sih/state/plan、sih-engine/sih/event/plan
- 公面随批写不入 allow：引擎 trail 与锁册与回执与绑定件

## 主线验收

F 逐条实跑、四套件主树重跑、链 verify、reconcile、T6 复验、campaign results 档、commit 全部。簇C 报告经主线阅后随 campaign 材料归档，如立修复批另走查册。
