# packhyg-solo：出货包卫生三件（词表规范形、parser 测试包装、BATCH-FACE 协调纪律勘误）

- 承接：主会 2026-09-04 carrwire 追认验收两发现 + 协调误读教训；用户 2026-09-04 裁定「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——本批纯机械零裁决点。
- 日期：以开工实日为准（trail 用当日链，会话号 sess-zcode-<实日>-packhyg）｜ 温故检索：materials/recall-packhyg.json 零命中如实记 ｜ pk-045 参与者
- 队形：单线形 solo，零子代理；单飞（须台面净态：委外在途批收约后方可开工，开工前 lease status 双零为准）。

## 背景（主会已验的两发现一教训，实跑复核为准）

- 发现一：nomenclator `test_core_pack_shipped_canonical` 红——terms.json 经多批「并集收编」通道追加后未跑规范化，出货包非规范形态（内容语义零错，纯格式）。
- 发现二：parser `tests/test_engine.py` 收集错误 ModuleNotFoundError——测试对 `parser` 包的导入在主树失败（打包/conftest 配置缺角或依赖件未随批入版控），其原批次工地绿态，须查缺件补版控。
- 教训：主会协调误读——以「存在活动会话」推断「在途不动」而宣布回避，实际对方在让位重试中自行收约。纪律应为：回避判定看锁台账归属与对方最新写入时间戳，不看会话存在。

## F 清单

- F-1 terms.json 规范形：跑化格归一（内容零改），test_core_pack_shipped_canonical 转绿；并在 BATCH-FACE 坑位勘误增一条「词表并集收编后必须跑化格归一」预防条款。
- F-2 parser 测试包装：定位缺角（pyproject 打包声明或 conftest 或未入版控依赖件），补齐随批入版控，主树 pytest 全绿；parser 既有行为零改动。
- F-3 BATCH-FACE 增「主会协调纪律」勘误一条：回避判定依据为锁台账归属与对方最新写入时间戳，禁以会话存在性推断在途态。
- F-4 全工具线抽查：本批触达两工具测试全绿外加 formatter 与 meter 抽查绿（carrwire 后主树首查基线）。
- F-5 写入仅 allow。

## 管线与机械链

ask3 → 双门 → 叩问 → 正身 → lease open --package packhyg-solo → 锁（施工面 exclusive；共享追加面 append 短持）→ intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 → settle 前拷工地 → settle --cert → close → reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH；禁管道掩退出码；close 外提交 --no-verify + bypass。

## 请求写入（锁路径全集）

- sih-tools/nomenclator/packs/core/terms.json（仅规范形归一）
- sih-tools/parser/（pyproject 或 conftest 或测试依赖件补齐）
- sih-tools/BATCH-FACE.md（勘误两条）
- sih-engine/sih/event/plan/packhyg-solo-materials/ 与 packhyg-solo-results.md
- 当日链 trail（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

terms.json 内容语义零改（只归一格式）；parser 行为零改；四工具 CONTRACT/SPEC 载体节零触碰；主树零直写。
