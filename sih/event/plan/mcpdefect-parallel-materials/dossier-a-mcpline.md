# mcpdefect-parallel 取材簇A：mcpline 泊界缺陷件修缮材料卷

- 批：mcpdefect-parallel（候批取材）
- 簇：簇A（mcpline 线）
- 取材日：2026-09-13
- 取材角色：只读取材。除本卷外零写入；未改 sih-tools 源码、sih-engine 源码、任何 .ndjson 台账、任何 trail 链文件。
- 取材范围：pk-091、pk-092、pk-093、pk-094、pk-095、pk-099 逐件对表现行源码。
- 事实纪律：每条判词附现行源码行号证据；源码与缺陷描述不符时以源码为准并注明。

## 逐缺陷条目

### pk-091 域写面闸组四缺陷

- 判词：**已被前批核销**（gatefix-parallel 四子项逐一现行源码核实在役；出泊记录 pk-091-exit.json disposition=promoted 与源码相符）。
- 现行源码证据：
  - 其一（commit 根锚定仓范围验失效）：已修。`sih-tools/lease/src/lease/commitcore.py:351-361`——`rel_prefix = "" if str(repo_rel) == "." else f"{repo_rel}/"`，点号 repo_rel 不再拼 `./` 前缀病形；比对单源 `scope_allows`（条目同剥，双保险）。
  - 其二（lock 与 commit 归一互斥）：已修。`sih-tools/lease/src/lease/lockcore.py:102-115` `normalize_scope_path`（剥前导 `./` 与尾斜杠，点号条目语义定为仓根递归全容）与 `lockcore.py:117-133` `scope_allows`（commit 与 lock 两闸共用本函数，scoperoot 批归一共形，CONTRACT 修订六十一）。
  - 其三（MCP lease_commit trail 与 root 透传缺位）：已修。`sih-tools/mcpline/src/mcpline/writeface/passthrough.py:267-298` `lease_commit_argv` 逐条 `--trail` 透传（相对形按域根锚定归一）与 `--root` 覆写透传，缺席即 CLI 缺省发现语义零变；`writeface/tools.py:483-501` `tool_lease_commit` 签名纳 `trail: list | None` 与 `root: str | None`，教学文载明「cert 四验收敛域内链须显式传」。
  - 其四（写面参数教学缺位）：已修。`writeface/tools.py:145-168` `_finish` CLI 失败路径 `what` 与 `params` 喂满（pk-091 其四注记在位，禁喂空）；`writeface/errors.py:185-214` `error_payload` 四字段（error、what_this_tool_does、valid_params、canonical_pointers）加理由码与闸位扩展。
- 最小复现形式（修前病形，现测应绿）：根锚定仓（repo 即域根）会话 stage 暂存根下文件后 `lease commit`——修前 staged_out_of_scope，修后过闸。
- 建议修法与红测草案：已修无需修法。回归钉位即 green 测试形态：fixture 根造 repo==root 会话，allow 传点号条目，断言 commit 过闸且 lock 侧同条目同过（两闸同源单函数 `scope_allows` 的等价性测试）。

### pk-092 写死路径排查真患两件

- 判词：**已被前批核销**（真患一两居所、真患二读面分形均现行源码核实在役；余患 retriever_bin 如实注候后继，与 gatefix-parallel-results.md 未决节相符）。
- 现行源码证据：
  - 真患一（locks 缺省链册单居所漏拦）：已修。`sih-tools/locks/src/locks/cli.py:26-42` `_default_trails` 双居所并集（`sih-engine/sih/event/trail` + `sih-tools/scribe/trail`），docstring 载 pk-092 修与 lease 先例对齐；`cli.py:114` `trails = args.trail if args.trail else _default_trails(root)` 在役。
  - 真患二（mcpline runtime 读面缺省未分形）：已修。`sih-tools/mcpline/src/mcpline/runtime.py:116-124` `detect_layout_form`（first_domain / canonical / None 三态，critsweep detect_layout 先例同构）；`runtime.py:127-136` `trail_path` canonical 域根命中域内 `sih/event/trail`，余回落第一域历史形；`runtime.py:139-150` `scribe_bin` debug 至 release 兜底序（旧形绑死 debug 档已除，两档俱缺回落 debug 位）。
  - 余患（如实注）：`server.py:586-589` `retriever_bin` 仍单绑 `sih-engine/target/debug/retriever`，无 release 兜底——gatefix-parallel-results.md:25 已申报「候后继批，修法即 runtime 件三同款兜底序」。属已申报未决项，非新缺陷。
- 最小复现形式（修前病形，现测应绿）：SIH_ROOT 指 canonical 域根（`sih/ledger` 在）部署形下 stdio 读面 `chain_query`——修前 trail 默认路落 `sih-engine/sih/event/trail` 错位，修后 `trail_path` 命中域内链。
- 建议修法与红测草案：真患二已修。retriever_bin 余患红测可写：`monkeypatch` 造 code_root 下仅 `target/release/retriever` 存在的树，断言 `retriever_bin()` 返回 release 位（与 `runtime.scribe_bin` 兜底序同形对表）。

### pk-093 司梦域 MCP 实测缺陷第二批（四患）

- 判词：**已被前批核销**（四患逐一现行源码核实在役；出泊记录 pk-093-exit.json disposition=promoted 与源码相符）。
- 现行源码证据：
  - 其一（隐式占位会话互斥）：已修。`writeface/tools.py:60-71` `_binding_state` 三态单点位（bound / implicit / unbound），open 与 close 与一切写工具同源（docstring 载 pk-093 其一归一位）；`tools.py:118-122` 隐式占位态放行 close 透传；`tools.py:564-568` 收约成即 `conn.auto_open_error = None` 生命周期闭环；`session.py:136-159` `auto_open_from_config` 承载隐式占位事实。两处错误载荷教学互指隐式占位见 `tools.py:100-104` 与 `tools.py:106-115`。
  - 其二（wip 形丢 note）：已修。`sih-tools/lease/src/lease/commitcore.py:147-168` `build_message` wip 形 `if note: parts.extend(["", note])`（docstring 载 scoperoot 批 pk-093 其二）；settle 形零变。
  - 其三（请求写入解析污染）：已修。`sih-tools/lease/src/lease/core.py:861-874` `parse_requested_writes` 列表项 `split("——", 1)[0]` 截断取路径部理由剥离（注记载 scoperoot 批 pk-093 其三，走 lease CONTRACT 修订六十一不裸奔）。
  - 其四（repo 校验误教）：已修。`httpface.py:236-247` `_binding_payload` repo 位提示语改教真实合法形——「合法形即点号列表（`.`，即所绑域仓集全容）与域内绝对路径」（注记载 pk-093 其四）。
- 最小复现形式（修前病形，现测应绿）：纯 MCP 连接首笔 `lease_claim` 后调 `lease_open`——修前 session_already_bound 死锁且 close 报 session_not_bound，修后 close 按包收隐式会话可重开。
- 建议修法与红测草案：已修。回归钉位形态：连接级 `auto_open_error` 非零时 `_precheck` 对 `lease_close` 返 None（放行）而对其他写工具返 session_not_bound 教学；close 成功后 `auto_open_error` 清零断言。现行测试树 `tests/test_pkfix_parallel.py` 与 `tests/test_cross_domain.py` 已有同族覆盖（后者当前环境红，见 pytest 基线节）。

### pk-094 司梦域 MCP/CLI 实测缺陷第三批（六件）

- 判词：**活缺陷件（部分）**。件无出泊记录（materials 目录无 pk-094-exit.json），仍 in-parked。六件分态：其一其二已被 pkfix-parallel 前批核销（源码核实）；其三其四其五为活缺陷（现行源码与缺陷描述一致，未修）；其六为已申报边界呈现弱残余（护栏在位，候后继小改）。
- 逐件源码证据：
  - 其一（commit repo 同传不同解）：已核销。`writeface/tools.py:513-525` repo 形参归一（str 与列表双形接受，列表归一单仓，多条教学拒 fail-visible）；`writeface/passthrough.py:252-264` `commit_repo_single`（兜底禁静默取头）；`passthrough.py:267-298` argv 构造经归一。回归测试 `tests/test_pkfix_parallel.py:34-91` 五件在役（列表透传、str 零变、多条教学拒、点号形教学、MCP 与 HTTP schema 双形）。
  - 其二（record 教学缺位二度实锤）：已核销。`writeface/tools.py:40-48` scribe 缺件报文静态短语集与路径形教学补位（`_CLI_FILE_MISSING_PHRASES` + `_CLI_FILE_PATH_HINT`）；`tools.py:145-168` `_finish` 失败路径命中短语即缀教学补位；record_intent / record_append / record_park / record_direct 四调用点 params 均载 json 文件路径教学（tools.py:222-229、251-258、281-284、306-311）。回归测试 `tests/test_pkfix_parallel.py:117-176` 六件在役。
  - 其三（停泊模板与校验器脱节）：**活缺陷**。`mcpline/init.py:124-134` `PARKING_TEMPLATE` 键集为 action、entry_id、context、ruling、state、parking{entered_at、ttl_days}——无 title、无 exit_condition、ttl_days 嵌 parking 子对象；而引擎侧校验器 `sih-engine/src/event_stream/park.rs:87-95` 要求顶层 title 与 exit_condition 与 ttl_days 且 `ttl_days <= 0` 即 `InvalidTtl` 拒，工具侧 `sih-tools/scribe/src/scribe/cli.py:209-214` 同要求（`for field in ("entry_id", "title", "exit_condition")`）。照模板写停泊材料必被双校验器拒，两典分叉未合。
  - 其四（lease CLI self_boot 缺省指引缺）：**活缺陷**。`sih-tools/lease/src/lease/core.py:214-224` `self_boot_check` 仍按路径含 `worktrees/` 子串判自举（`in_worktree_src or in_worktree_cwd`），工地路径不匹配即闸不触发；缺省指引（CLI 缺 --ledger/--locks/--bills 时错误不告知缺省路径）未见补位。
  - 其五（域装 gauge 底座缺席）：**活缺陷**。`sih-tools/gauge/src/gauge/cli.py:125-127` `_die_missing` 报「底座件缺席」退出码二在役；`mcpline/init.py` 全文 grep `gauge|底座` 零命中——域自举落地五步（init.py:136-141 WHAT_THIS_TOOL_DOES）不含 gauge CLI 源与会话台账与 sih-engine 证据底座，域内读数必死未愈。
  - 其六（close 归并呈现弱）：**已申报边界，护栏核实无害**。`writeface/tools.py:539-545` 注记申报 light_trail 恒 None、canonical 域收约链闸空转 workspace_chainless 如实注记（mcpwrite-solo 实测在档）；归并护栏在 lease 侧核实在位——`sih-tools/lease/src/lease/core.py:2596-2633` close 输出含 `"merged"` 字段，MCP 面经 `passthrough.py:336-353` `passthrough_result` stdout 原样透出（归并事实不静默）。残余仅教学面归并记录显式透出欠，候后继小改，非新增执法缺陷。
- 最小复现形式：
  - 其三：照 `PARKING_TEMPLATE` 填一份停泊材料件喂 `scribe park`——引擎与工具侧校验器双拒（缺 title/exit_condition、ttl_days 非顶层正数）。
  - 其五：新装域内跑 `gauge record`——`_die_missing` 退出码二底座件缺席。
- 建议修法与红测草案：
  - 其三：单一源裁定二择一——模板平铺改顶层 title/exit_condition/ttl_days（对表校验器），或校验器兼容嵌套形。红测：照现行模板逐字生成件喂 park 校验器断言通过（现测必红）。
  - 其四：self_boot 闸路径启发窄化加缺省指引——闸未触发且显式参缺席时错误文载缺省路径三件（--ledger/--locks/--bills 各自缺省位）。红测：非 worktrees 工地调 close 缺显式参，断言错误载荷含缺省路径教学。
  - 其五：域 init 扩装 gauge 底座（CLI 源、sessions 台账、sih-engine 证据底座三件映射）。红测：init 落地后域内 `gauge read` 退出码零。
  - 其六：close 成功载荷教学面显式载归并记录摘要（merged 条目数）。

### pk-095 lease CLI settle 域内链缺省发现两居所缺 canonical 域家

- 判词：**已被前批核销**（pkfix-parallel；出泊记录 pk-095-exit.json disposition=promoted；现行源码核实第三居所在役）。
- 现行源码证据：`sih-tools/lease/src/lease/cli.py:214-229` `_default_trails` 三居所并集——`sih-engine/sih/event/trail` + `sih-tools/scribe/trail` + `sih/event/trail`（canonical 域家），docstring 载「pkfix-parallel 批（pk-095）：补第三居所……既有两居所枚举序与去重零变」；`cli.py:1278` `trails = args.trail if args.trail else _default_trails(root)` 在役。两好转 field（allow 解析净化 core.py:869 截断、claim 隐式会话冲突教学 tools.py:100-115）亦与 pk-093 核验证据互证在役。
- 最小复现形式（修前病形，现测应绿）：canonical 域内既有链文件时 `lease settle` 缺省（零 --trail）——修前 trails=[] 致 cert_not_on_chain，修后域家链入缺省发现。
- 建议修法与红测草案：已修。回归钉位形态：fixture 根造 `sih/event/trail/<日>.ndjson`，断言 `_default_trails(root)` 含第三居所路径且去重序稳定。

### pk-099 8765 握手自报版本串 1.30.0 错位

- 判词：**已被前批核销**（pkfix-parallel；出泊记录 pk-099-exit.json disposition=promoted；现行源码与回归测试俱在役）。
- 现行源码证据：
  - 单源化：`mcpline/__init__.py` `__version__ = "0.11.0"` 与 `pyproject.toml:3` `version = "0.11.0"` 同步，docstring 载包版本单点位。
  - 注入位：`runtime.py:14-34` `pkg_version()` 与 `apply_server_info_version`（把 mcpline 包版本注入低层 `Server.version`，stdio 与 HTTP 两面同函同源，注记载 pk-099 修与 1.30.0 错位实锤位）；`server.py:66-69` stdio 面构造后即注入；`httpface.py:714-715` HTTP 面同函注入。`web.py` 版本面零第二源（grep 零命中）。
  - 回归钉位：`tests/test_pkfix_parallel.py:180-201` 四件——`test_pkg_version_matches_pyproject`、`test_stdio_mcp_server_info_version`、`test_http_mcp_server_info_version`、`test_server_info_version_not_sdk_fallback`。
- 最小复现形式（修前病形，现测应绿）：8765 initialize 应答 serverInfo.version——修前落 mcp SDK 包版本 1.30.0，修后落 0.11.0。
- 建议修法与红测草案：已修无需修法。红测即现行 `test_server_info_version_not_sdk_fallback`（版本串不得等于 mcp SDK 版本）。

## pytest 基线节

- 命令：`cd /Users/moc/workspaces/SiHankor/sih-tools/mcpline && uv run pytest -q`
- 读数：**9 failed, 141 passed, 28 errors in 78.26s**（复跑 70.89s，读数一致）。
- 失败名清单（原样记录）：
  - FAILED tests/test_ai_manual.py::test_manual_route_serves_markdown
  - FAILED tests/test_ai_manual.py::test_manual_route_missing_file_teaches
  - FAILED tests/test_ai_manual.py::test_console_pages_link_manual
  - FAILED tests/test_bootstrap_domain.py::test_c4_console_confirm_open_writes_card
  - FAILED tests/test_bootstrap_domain.py::test_c5_live_server_tools_list_same_shape
  - FAILED tests/test_domain_layout.py::test_critsweep_first_form_full_and_canonical_degraded
  - FAILED tests/test_stdio_smoke.py::test_stdio_smoke_alpha_five_regression
  - FAILED tests/test_tools_unit.py::test_critsweep_happy
  - FAILED tests/test_web_smoke.py::test_http_smoke_panel_console_and_mcp_face
- ERROR 名清单（28 件，原样记录）：test_cross_domain.py 七件、test_http_identity.py 七件、test_http_write_loop.py 一件、test_scope_readonly.py 五件、test_token_console.py 八件（逐名见前两次运行输出，均为 RuntimeError 级 fixture/环境错，非断言失败）。
- 失败因归类（抽样实勘）：
  - 环境红（静态资产缺席）：web.py:427 `RuntimeError: 静态目录缺席（只读引用形，零创建）: /Users/moc/workspaces/SiHankor/sih-visual/assets/viewer-dashboard-2026-09-06`——抽样命中 test_manual_route_serves_markdown 与 test_c4_console_confirm_open_writes_card；28 件 ERROR 与 web smoke 与 ai_manual 失败大概率同根（sih-visual 资产目录本机缺席）。
  - 测试陈旧（判据数漂移）：test_critsweep_happy 与 test_stdio_smoke_alpha_five_regression 与 test_domain_layout 一件断言 `len(ids) == 5`，实得 6——GOV2 判据新增 `GOV2-C6-sddgate`（实钥清单：GOV2-C1-leaseopt、GOV2-C2-viewline、GOV2-C3-measure-determinism、GOV2-C4-math-attribution、GOV2-C5-lease-line、GOV2-C6-sddgate），测试期望未跟。
- 如实注：本基线环境红（静态资产缺席）与测试陈旧（判据数漂移）两类均非本簇六缺陷件引入；候批立批时以本节为前基线，修后对表增量。

## 检索先例节

- debtclear-parallel-results.md:26（在档）：「立：pk-091 域写面闸组四缺陷、pk-092 写死路径真患两件、pk-093 司梦 MCP 实测四患，三账合组候批即域面闸组与 MCP 写面修，出泊条件互引，mcpline 与 lease 树锁已随簇A收约让出可开」——即 pk-091/092/093 曾由 debtclear-parallel 合组候批（锁已让出）。
- 后续兑现对表：三件已由 **gatefix-parallel** 批兑现出泊（pk-091/092/093-exit.json 俱 disposition=promoted，归并 ecfac044 与 14bafd3e 与 12540dff；gatefix-parallel-results.md:21 载三件俱 promoted 与余患如实注四条：retriever_bin 绑 debug、其余写工具 valid_params 喂空、stem 单段甲表双段拒机械适配、零写证明对并联批群外活动敏感）。本卷逐件源码核实与出泊判词相符。
- pk-095 与 pk-099 与 pk-094 其一其二由 **pkfix-parallel** 批兑现（results 档 sih-engine/sih/event/plan/pkfix-parallel/pkfix-parallel-results.md；pk-095/099 出泊记录在 materials，pk-094 本件无出泊记录仍 in-parked，仅其一其二子项随批核销）。
- 出泊条件中「批名先查册」「租约候簇A nomsupply 让出 sih-tools/lease 锁后开」等批名纪律条款：本卷为只读取材未立批未取锁，立批时仍须走查册与租约程序。

## 判词一行总表

| pk 号 | 判词 | 一句话依据 |
|---|---|---|
| pk-091 | 已被前批核销（gatefix-parallel） | commitcore.py:351 点号归一、lockcore.py:102/117 两闸单源、passthrough.py:267 trail/root 透传、tools.py:145 valid_params 喂满俱在役 |
| pk-092 | 已被前批核销（gatefix-parallel），余患 retriever_bin 候后继 | locks cli.py:26 双居所、runtime.py:116-150 分形与兜底序在役；server.py:589 retriever 仍单绑 debug（已申报未决） |
| pk-093 | 已被前批核销（gatefix-parallel） | tools.py:60 三态绑定单点位、commitcore.py:147 wip note、core.py:869 截断、httpface.py:236 误教改正俱在役 |
| pk-094 | 部分活缺陷（件仍 in-parked） | 其一其二已核销（test_pkfix_parallel.py:34-176 在役）；其三（init.py:124 对 park.rs:87 双典分叉）、其四（core.py:214 子串启发）、其五（init.py 零 gauge 底座）现行源码未修；其六护栏在位候教学面小改 |
| pk-095 | 已被前批核销（pkfix-parallel） | lease cli.py:214-229 三居所并集在役，canonical 域家第三居所在册 |
| pk-099 | 已被前批核销（pkfix-parallel） | runtime.py:30 版本单源注入两面同函，__init__ 与 pyproject 同步 0.11.0，回归四测试钉住 |

## pytest 基线读数（复述）

`uv run pytest -q`（cwd sih-tools/mcpline）：**9 failed, 141 passed, 28 errors**。失败两类根因：sih-visual 静态资产目录缺席（web.py:427 RuntimeError，环境红）与 GOV2 判据数 5→6 漂移（测试陈旧）。均非本簇六件引入，如实记为前基线。
