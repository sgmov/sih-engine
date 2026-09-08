# release09-solo 批结果档：0.9.0 首发布工程三件收约

> 令源：用户 2026-09-09 令派 release09-solo 批，任务包 sih-engine/sih/state/plan/release09-solo.md
> 会话：租约 2dc048ef101dbf00（lease 1.39.0，三仓登记 sih-tools 与 sih-engine 与 sih-math，十锁）
> 判定：三件全落，管线全绿，认证上链，零执行零发布（不打 tag 不推送不发布红线守住）

## 一、三件终值与路径

- 件一 SemVer 晋升判据 DEC：`sih-engine/doc/decision/022-semver-release-v1.md`。编号申报：任务包原载 015，地面实况核对即 decision 目录数字序列已至 021 且 015 已被 015-quality-baseline.md 占用，任务包自令「编号先对地面实况核对顺延」，故顺延取 022。载三节即 0.9.0 语义（首发布即测试味版本，alpha 相只读 MCP 面在列）与 1.0.0 机械晋升判据（判据一冷 agent MCP 面零辅助跑通、判据二陌生人零帮助指南走查、判据三发布后修复节奏建立，逐条载判定命令或材料指针，判定读数以晋升批当批重放为准）与治理语义 SemVer 定约（MAJOR 即治理语义破坏、MINOR 即判定语义增量、PATCH 即执法面修补）与版本双载体（git tag 与 Cargo version 位同批对表）。管线三步全绿。
- 件二 0.9.0 发布清单：`sih-engine/doc/plan/release-0.9.0-v1.md`。发布内容面五项即引擎五件套（scribe 与 scrutinator 与 attractor 与 ask3repeater 与 retriever，viewer 与 snapline 归视图组件线随带不在枚举）与 sih-tools 工具带与 mcpline 服务器与 SPEC-023 契约与视图仓指针；执行步骤九步，第七至第九步即打 tag 与推送与发布宣告标注归主窗终验后执行；已知边界四项即 beta 写面未开与 GOV-002 v3 候裁与出参超集申明与帮助面漂移；回滚法四腿。管线三步全绿。
- 件三 数学仓推送卫生：`sih-math/.gitignore` 只增不删。现行三行 .DS_Store 与 *.swp 与 tmp/ 零改动，新增九行即缓存与运行残留类（__pycache__/ 与 *.py[cod] 与 .pytest_cache/ 与 .ruff_cache/ 与 .mypy_cache/ 与 .venv/ 与 .coverage 与 htmlcov/ 加注释行），实证残留即 mathpipe-coverage 目录下 rev3_script.cpython-314.pyc 增后归 ignore 面。只增证明：materials/gitignore-addonly.diff 形态 3a4,12 纯追加零删改。math 仓随批 settle 留痕。

## 二、链笔清单（当日链 sih-engine/sih/event/trail/2026-09-09.ndjson）

- 例行读数三笔：convergence 9ec8942f、adoption 05690d8e、mergeback 309b3412（index 84 至 86，scribe confirm 复证在档）
- 书简意图笔：fed6ede6（intent_refined，record sha256 09092ac0，anchor 三）
- 认证五笔：pipeline-report aedccef1、first-red-scrut-c002 2542594d、first-red-nomen-covenant 0a9c5940、gitignore-addonly-evidence 30c24bc7、本结果档见末笔

## 三、红证与误差申报

- 核阅首跑红：两文档 C002 希腊字母字符集违例共六笔（alpha 与 beta 相名），处置即拉丁转写加转写声明（承 SPEC-023 先例），判定文本零改动，复跑绿；红证 materials/first-red-scrut-c002.json，先红留痕纪律 2026-09-06 承办
- 检词首跑红：DEC-022 锚名 covenant 命死档禁用词表（PRO-007 宗教重载）二笔，处置即锚名改 agreement，复跑绿；红证 materials/first-red-nomen-covenant.json
- scribe append 首试 gitignore-addonly.diff 报 ReportNotJson（报告须 JSON 形），处置即包 JSON 证据件重试成功，原始 diff 字节件同目录在档
- watchcheck 会话启动对表出两件无主修改清单（sih-tools/attnanchor/CALL-LOG.md 与 sih-tools/attractor/CALL-LOG.md，mtime 2026-09-08，已跟踪修改，非本批产物），按处置协议机械呈人节点二值裁决，本批零代行零回滚，如实转述
- 判据扫启动读数：degraded 假，五判据俱 achieved，泊界双线零告警，如实转述

## 四、免跑与豁免申报

- 书单对表守卫（BATCH-FACE 10.5）免跑申报：批引用件 DEC-022 与 release-0.9.0-v1 零数学仓推导档引用 ID（引用面限 SPEC-023 与 mcpline 结算件与指南与 governance 件与 Cargo.toml），不触发先 recall 后 checkcite 条件
- proposition/DES/m-release09-1 在 allow 清单在册但本批零测量零写入：本批三件俱文档与卫生件，任务包未立 facet 测量腿，锁未取未占

## 五、settle 与收约

- 三仓 settle：sih-tools 与 sih-engine 与 sih-math 各 lease commit 一笔，提交号见完工回报
- 收约后链 verify 与 reconcile 双仓读数见完工回报；收约后回锚一次
