# retrline 批结果档

## 验收判词 {#verdict}

- rl-01 档案面盘点在册：`retrline-materials/rl01-inventory.md` 逐件清点带域标。
- rl-02 扩容成立：legacy md 263 与 setsp md 199 入引擎温故 archives 面，实测 files=462 entries=39281 parse_errors=0，条目带 `source_domain` 标注（legacy-sihankor 与 setsp），既有域行七字段逐字节零变单测钉死；检索可见面非引用权威面承纪律。
- rl-03 语义通道成立：确定性统计向量对齐 wikirecall semantic.py，五组 Python 实测金值逐位相等，缺省语义 K=3，词面 `--lexical` 显式回退位行为与切前逐字节一致（词轴 1049 行实测），`--semantic K` 显式给参，不做向量库承 pk-037 既裁。
- rl-04 对表成立：八查询 K=3 同语料双跑判词 8/8 一致，score_max_delta 2.22e-16 在 1e-9 判据阈内，覆盖词面命中型三、语义近邻型三、零命中型二；判词口径机械可查在 dualrun-summary.json。
- 消费面零破坏：CLI 既有十参数全兼容，NDJSON 行形零变，MCP retriever_recall 零改码；事件与时间轴两模式零变。

## 偏差申报 {#deviations}

- 主会补笔：`src/retriever/mod.rs` 测试模块 `retr-env-<pid>` 掺名位按 memfix 同款追加线程 id（memfix 批偏差申报移交的顺带处置点），复跑 lib retriever 25 绿。
- 子代理存量陈旧测试修复一件：miss_log 两测试零命中选词已被 wenguobs-resolo 批结果档引录进语料而假命中（HEAD 上即红，非本批引入），换新词并留换词理由注释。
- 语料面对价：wikirecall 用 title+triggers 两面而引擎 locator 条目无此两面，取 entry.text 与词面通道同一文本面对价公平承原口径。
- MCP retriever_recall 缺省结果面随 CLI 缺省承语义通道（pk-050 形的必然传导），参数形与退出码与行形零变。
- 过程事故全量回收：中途 cargo fmt 波及全 crate 153 文件已逐文件回滚，终态 diff 只含本批件；locator_bridge.rs 与 mcpserver 复验零触碰。

## 处置记录 {#dispositions}

承 2026-09-18 用户令「全都推进」候令簿第三件，承载 pk-039 与 pk-056 出泊开工令形（立项包 retrieverline.md，docwave-solo 批入档）。子代理施工、主会结算。SETSP yaml 五件关系边集指向域外 @philosophy 目标在册未入索引，盘点册有载。
