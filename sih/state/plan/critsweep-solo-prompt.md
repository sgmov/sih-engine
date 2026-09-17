# critsweep-solo 委外提示词

> 用途：由主会拉起的执行子代理直接按本提示词执行。任务包在 sih-engine/sih/state/plan/critsweep-solo.md，本提示词是执行入口。

---

你是司衡工作区的委外执行代理，单线 solo 队形、零子代理（不得再派生任何子代理或后台任务）。工作区根：/Users/moc/workspaces/SiHankor。今天是 2026-09-08，当日链文件 sih-engine/sih/event/trail/2026-09-08.ndjson（不存在则由首笔 scribe 写入创建）。

## 第一步：读三件再动手

1. 任务包：sih-engine/sih/state/plan/critsweep-solo.md（判据与 F 表与约束与请求写入节，全部对你有约束力）
2. 命令面正典：sih-tools/BATCH-FACE.md（批机械链 verbatim 全序与坑位勘误，特别是 2026-09-04 至 09-07 各勘误节）
3. 家族先例：sih-tools/attnanchor/CONTRACT.md 与 sih-tools/attnanchor/anchor.py（根判据上溯、严格 JSON 单键、降级可见、退出码恒零），及 sih-engine/sih/event/plan/anchorskill-solo-results.md（最近的委外链全形先例：内容哈希清单认证形、bypass-orphan、收约补笔）

## 第二步：会话启动节律（批前置读数）

- 例行读数：cd sih-tools/gauge 后 env -u PYTHONHOME -u PYTHONPATH PYTHONPATH=src python3 -m gauge.cli record --at 2026-09-08 --trail ../../sih-engine/sih/event/trail/2026-09-08.ndjson --sessions-ledger ../lease/ledger/sessions.ndjson --src-root ../../sih-engine --tools-root .. --scribe ../../sih-engine/target/debug/scribe --record-trail ../../sih-engine/sih/event/trail/2026-09-08.ndjson --locks ../lease/ledger/locks.ndjson --session sess-zcode-260908-critsweep
- watch 对表：cd sih-tools/watchcheck 后 env -u PYTHONHOME -u PYTHONPATH PYTHONPATH=src python3 -m watchcheck.cli check --at 2026-09-08 --root <根>（已知无主二件 anchor.py 权限位与 calls.ndjson：如实呈报不代清不触碰）
- 泊界心跳两线：cd sih-tools 后 uv run --project ./selector selector route --pack selector/packs/parking --reference-time 2026-09-08 parking/materials 与 ../sih-engine/sih/state/parking/materials 各一次，告警如实转述

## 第三步：意图治理

- ask3 记录生成器仿 sih-tools/scribe/reports/make_ask3_anchorskill-solo.py：哲学引文必须从 sih-philosophy/emanation/proodos 原文程序切片逐字节内嵌禁手打，三锚自选与本批相干（建议方向：确定性程序承载检验、应而不藏、只列事实），session_id 用 sess-zcode-260908-critsweep
- 双门：cd sih-tools/scrutinator && uv run scrutinator --pack packs/ask3 <记录>；sih-engine/target/debug/ask3repeater <记录> --root <根>；验证件落 scribe/reports
- 叩问 elicit check 加 digest（--words 逐词重复传）；正身 identity verify 落 identity/reports

## 第四步：租约与施工

- lease open --package critsweep-solo --identity <正身件> --intent <ask3记录> --repo <根>/sih-tools --repo <根>/sih-engine --allow 按任务包请求写入节逐路径传（目录带尾斜杠；漏仓即写入面无工地通道），取 session_id
- lease lock 逐路径（--identity --session --root --locks --ledger 全参）
- scribe intent 经 meter 包裹：cd sih-tools/meter && uv run --project . meter run -- sih-engine/target/debug/scribe intent --record <记录> --validation <验证件> --trail <当日链> --locks <锁账本> --sessions <会话账本> --session <会话号>（闸三必带 --sessions）
- 施工全在工地 worktrees/sih-tools/critsweep-solo 与 worktrees/sih-engine/critsweep-solo 内改（lease open 自动建工地）；任务包与报告件等批输入件先在主树落位者镜像入工地；AGENTS.md 无仓版控主树原地改加归档件落引擎工地

## 第五步：工具本体要求（判据扫 v1）

- sih-tools/critsweep/sweep.py：纯 Python 标准库，零 LLM 零网络，退出码恒零；调用形 python3 sih-tools/critsweep/sweep.py --at 2026-09-08 --root <根> [--threshold N]（N 默认 3）；根判据承 anchor.py 自脚本位逐级上溯首个含账本目录者；输出严格 JSON 单对象 stdout
- 回算三面（详见任务包关键设计节）：判据面按 registry.json 批名令牌扫近 10 日 trail 事件 details 的 record_path 与 report_path 与 package 字段（禁全文散文匹配——pk-044 出泊裁定散文含 measure-poly 字样属设计引用非程序活动，须作对照读数落材料）；泊界面经 subprocess 调 selector parking 包路由（超时 10 秒降级）；在飞面读两账本
- registry.json 空腹五判据：自 sih-engine/doc/governance/GOV-002-mainline-lock-v1.md v2.4 概览与版本史提取五条判据文本与证据指针；①leaseopt 达成（SETTLEMENT-LEASEOPT-2026-09-05.md）⑤租约升级线达成（GOV-002 v2.4 追记与 lease CONTRACT 修订四十一至四十三）②视图在飞（viewline 族令牌）③measure-poly 沉底（令牌 measure-poly/measurepoly，registered_at 2026-09-04，零批记录）④数学归因长尾（令牌自 pk-053 与 mathclose/mathreg 等链面核实）；每条派生依据落批材料，宁窄勿宽
- tests/：pytest 加 fixture 链切片加金向量，含缺 trail 降级用例、双跑一致用例、pk-044 对照用例
- sih-tools/pyproject.toml members 收编 critsweep；主树裸调验收算 F-1 硬性项（工地条件验证不算接线）

## 第六步：链收口

- 管线三步（化格 packs/general-v1 --write → 核阅 target/debug/scrutinator --pack des-001 裸名 → 检词 nomenclator check --pack packs/core）对引擎域文档；sih-tools 域件与无仓控件跑核阅检词、化格不越域如实申报
- checkcite（认证前必跑）：cd sih-tools/wikirecall 后 python3 recall.py --repo <根>/sih-math --query <批主题词> --aliases aliases-seed.json --out /tmp/plan.json；python3 checkcite.py --plan /tmp/plan.json --cited <合并单件> --out scribe/reports/…（--cited 是单值参数，多件 cat 合并）
- 认证经 meter 包裹 scribe append（--report 只认 JSON；md 件走内容哈希清单件一件绑定，anchorskill 先例；--sessions 必带）
- 双仓 settle：工地内 git add -A 后 cd sih-tools/lease && uv run --project . lease commit --repo <工地绝对路径> --session <会话号> --stage settle --seq 1 --cert <ask3 记录认证前八位> --subject "<事述>" --trail <当日链> --root <根> --ledger <会话账本>
- 放锁 11 路径（正身件保持在场直至 unlock 毕）；close 前把 shell cwd 移出工地；close 被无主闸拦（已知 anchor.py 权限位与 calls.ndjson 二件非本批活面）即 --bypass-orphan 载事由；被包不可读拒即任务包须在主树在场
- 收约后：结果档结算读数回填经 --no-verify 加 lease bypass --repo <仓> --sha <提交号> --reason <事由> 入版控；CALL-LOG 走 uv run --project lease lease call-log append（禁徒手）
- 对账对表：lease reconcile 双仓（unrouted 较批前零新增）；target/debug/scribe verify --trail <当日链> status valid
- 温故检索：结果档起草前 cd sih-engine 后 target/debug/retriever recall --event <批事件标识> --since 2026-09-08 --until 2026-09-08 --at 2026-09-08 --out <批材料>，零命中如实记

## 纪律红线（违一即批失败）

1. 零子代理零后台任务；anchor.py 与 .zcode/config.json 与 .session-anchor.md 与 confpreempt 面、confidence 面、calls.ndjson 活面零触碰
2. 主树零直写（repo 件经工地）；禁 plain git commit；禁管道掩退出码（退出码即时捕获）；先红留痕禁清洗重跑
3. 退出码 2 一律先处置工具异常不得当违规硬闯；撞锁不绕行
4. 结果档必含：意图锚定、前置读数、件读数、管线读数、认证清单、F 表逐条实态、越线与误差申报（含对己不利读数）、结算读数、完工回显即 python3 sih-tools/critsweep/sweep.py --at 2026-09-08 --root <根> 的首跑读数（本工具自证）
5. 完工报告以任务包第七节验收标准逐项自答，零项不得虚报；受阻即停批上报不硬凑
