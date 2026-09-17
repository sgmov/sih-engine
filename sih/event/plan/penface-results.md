# penface 批结果档：mcpline 写面修五件（gatefix-parallel 簇M）

> 正典输入：sih-engine/sih/state/plan/gatefix-parallel.md 簇M 任务书；pk-091 与 pk-092 与 pk-093 三账出泊条件
> 批机械链全序：sih-tools/BATCH-FACE.md；日期 2026-09-11

## 回执要件

- 批名：penface（查册 unknown，--new-stem 甲表认领，stem 闸 disposition new_coinage_acknowledged）
- 会话号：b8aa24a253bb7fd4（租约 lifecycle 完整：issued → settled 双仓 → revoked）
- 认证哈希：意图笔 1c6da2f7、红证 171b815c（exit 1）、绿证 ee07429d（exit 0），全在 2026-09-11 链（链 verify valid，54 笔）
- 测试数：全套 160 绿（基线 151 + 本批新增 9）；红先红 8 红 1 绿（绿者缺省零变回归卫）
- settle 提交：tools 段 08fbb0cf（归并 14bafd3e 入 integral-stage-build）、engine 段 a361f39（归并 fb20ba9 入 main），四验 session_active 与 staged_in_scope 与 cert_on_chain 全过
- reconcile：双仓 unrouted 0、cert_missing 0（较批前零新增）

## F1–F6 逐条实跑结果

- F1 MCP lease_commit 带 trail 到 CLI argv 单测先红：红（TypeError 缺 keyword）→ 绿。trail 逐条 --trail 透传（CLI action=append 同形，相对形按域根 anchored 归一）；root 覆写透传替换缺省 --root；缺省零变回归卫同绿（零 --trail 且 root 为域根）。settle 实跑即 CLI 直传 --trail --root 的 cert_on_chain 四验自证。
- F2 两错误路径 valid_params 非空且教学串钉死：红（valid_params 空与教学串缺）→ 绿。lease_open CLI 失败载荷教学钉「意图件路径」（ask3 或 plain 意图记录 JSON 文件，自由文本即 intent record unreadable 拒）；record_append CLI 失败载荷教学钉「JSON 报告文件路径」（md 与内联文本两连拒）；_finish 增 what/params 喂满机制，两调用点喂满。
- F3 fixture canonical 域根无令牌场景 runtime 缺省读数命中域 trail：红 → 绿。runtime 增 detect_layout_form（critsweep detect_layout 先例同构：first_domain 双仓标记俱在、canonical sih/ledger 面、余 None），trail_path 按 layout_form 解析，canonical 命中 <域根>/sih/event/trail，first_domain 与未知形回落第一域历史形（stdio 缺省行为零变）；scribe_bin 解析 profile 无关（debug 优先、release 兜底、俱缺回落 debug 位）；域令牌绑定分支（DomainLayout.trail）零变。
- F4 隐式绑定连接 close 收得先红：红（session_not_bound 拒，隐式会话成收不回孤儿）→ 绿。_binding_state 单点位三态（bound、implicit、unbound）为 open 与 close 与一切写工具同一绑定查找位；隐式占位态（连接级自动立会话未成留 auto_open_error 既成事实而零显式绑定）close 放行透传既有 CLI 收约（链闸与 closeguard 原位，零新增判定），收约成即隐式占位事实清零可显式重开；session_already_bound 与 session_not_bound 两处错误载荷教学互指隐式占位事实。
- F5 误教提示改正先红：红（提示语无点号形教学）→ 绿。httpface _binding_payload 按 label 分流：repo 合法形改教点号列表（"."，即所绑域仓集全容）与域内绝对路径（canonical 域 repo 集对表下唯此两形合法，实测行为红证同在）；其余路径参保持域根相对形或域内绝对形教学（anchored 归一原位，该教学对彼等参数为真）。
- F6 mcpline 全套绿基线 151：工地 160 绿（86s）与主树真跑 160 绿（81s）双读数在档；版本位 0.10.0 升 0.11.0 双点位（pyproject.toml 与 __init__.py）。

## 件一至件五落码位

- 件一：writeface/passthrough.py lease_commit_argv 增 trail 与 root 形参；writeface/tools.py tool_lease_commit 透传；server.py stdio 注册位增两参；httpface.py http_lease_commit 增 trail（逐条绑定验）而 root 零透传（DES-015 主防线，server 域锚定）。DES-014 修订三随件（写路径复用节增 commit trail 与 root 透传段）。
- 件二：writeface/tools.py _finish 增 what/params 喂满位，tool_lease_open 与 tool_record_append 两处参数教学更新且 CLI 失败路径喂满。
- 件三：runtime.py 增 detect_layout_form，trail_path 两形分形，scribe_bin debug 与 release 兜底序。
- 件四：writeface/tools.py _binding_state 单点位与 _precheck 归一重排与 close 隐式占位放行与收约清零与两处教学互指；DES-014 连接与会话映射节增隐式占位态段。
- 件五：httpface.py _binding_payload repo 分流改教真实合法形。

## 伴生修复（施工必要件，如实申报）

- lease_open_argv 增 --locks 显式全传（与 lock/unlock/close 诸 argv 同形同源）：此前 open 预检锁面走 CLI 根锚缺省解析，夹具形与 canonical 域形下漏进中央真锁册（隔而不离，本批持 DES-014 锁窗内夹具 open 实撞即红证在案）；显式传参后 stdio 第一域行为零变（同册）、canonical 域预检锁面归域内（隔离修正）。
- 差集闸目录声明形修正：本批任务包 penface.md 两目录声明补尾斜杠（目录路径以 / 结尾乃机械契约，无尾斜杠被按文件精确匹配判未提交）。

## 文档管线与对表

- DES-014 修订三：化格 rc0（targets_changed 0）→ 核阅 rc0（des-001 域内零违规，C006 两处全角括号违例即改半角先例形后复跑零 findings）→ 检词 rc0（零违例）。
- AI-MANUAL.md：版本行 0.11.0、lease_open repo 合法形、lease_commit trail 与 root（root 注记仅 stdio）、record_append JSON 报告文件路径、path_outside_domain 处置改点号形、剧本 C 第 6 步补 trail。
- README.md：测试族增 penface 修复锚定条目。

## 改动文件清单

- sih-tools 仓（08fbb0cf）：mcpline/src/mcpline/runtime.py、mcpline/src/mcpline/writeface/passthrough.py、mcpline/src/mcpline/writeface/tools.py、mcpline/src/mcpline/server.py、mcpline/src/mcpline/httpface.py、mcpline/src/mcpline/__init__.py、mcpline/tests/test_penface_fixes.py（新）、mcpline/pyproject.toml、mcpline/README.md、mcpline/AI-MANUAL.md
- sih-engine 仓（a361f39）：doc/design/DES-014-mcp-beta-security-model-v1.md
- 主树批面（候 campaign commit）：sih-engine/sih/state/plan/penface.md（本批簇任务包，开约机械承载）、sih-engine/sih/event/plan/penface-results.md（本档）、sih-engine/sih/event/plan/penface-materials/（红绿证 log 与 json、scrutinator 与检词报告、主树复跑 log）
- 公面随批写（不入 allow）：2026-09-11 链三笔、锁册六取六放、会话册 issued 与 revoked、bypass.ndjson 一笔（close --bypass-orphan 留痕）、CALL-LOG 三腿（mcpline 一笔）

## 收约通道申报

- close --bypass-orphan 一笔：无主闸三件皆批前既有脏件非本批产物（sih-tools/facet/probes/cascade_ng_probe.py 与 lightweight_mode_probe.py mtime 2026-09-10T04:11Z、sih-tools/mcpline/ledger/tokens.ndjson mtime 2026-09-10T08:50Z，俱早于本批开工且开工时 git status 已在案），不代清候人节点裁决。
- close --ack-uncommitted 三件：penface.md（协调面任务包载体，开约前立稿）与 penface-results.md（收约后交付件）与 penface-materials/（批材料），候主线 campaign commit，nomsupply 与 trailhome 先例同形。

## 未决项

- retriever_bin（server.py）仍绑死 target/debug：pk-092 真患二同款余患，不在本批五件范围，候后继批以同款 debug 与 release 兜底序处置。
- 其余写工具（record_intent、record_park、record_direct、lock 族、claim 族、close）CLI 失败路径 valid_params 仍走 _finish 缺省喂空：本批按任务书钉两错误路径，_finish what/params 机制已备，候后继批评审补齐。
- 无主闸三件批前遗留脏件候人节点裁决（见收约通道申报）。
- 甲表派生表机械承载形适配：stem 闸分段规则（[-_.] 分割，penface 单段）下任务书的 pen+face 双段派生形被机械拒（缺段 penface、表外段 face 与 pen），实开以单段 penface:new 认领；语素语义（pen 即书简笔、face 即面）由概念锚 zh 写面修与 penface.md 申报承载。此适配是词形机械规则与语义双段申报的载体分工，非语义改判。
- 零写证明测试（test_zero_write）在并联批共享活面下对群外活动敏感：主树基线首跑 150 绿 1 红（红者即他簇 identity 报告未跟踪件入窗，非本批写入），工地与主树终跑全绿；此敏感面候后继批评审（环境红隔离批范围）。

## 完工回显（当日五行锚）

```
[锚] 当前任务：无在飞——清账并联批 debtclear-parallel 收口（四簇全清……）……（.session-anchor.md 首行，锚件原文回显）
[在飞] 会话 0 在册 持锁 0 条
[泊界] 引擎线 主线59 侧7 废9 告警1 · 工具线 主线28 侧1 废1 告警0
[链] 今日 54 笔 末笔 ee07429d certification_completed · 越限告警：InferServer 四日五笔查无此人在案
[令] 新主题先三岔：属当下吸收／成形入队（泊界或待办）／未成形缓议；越界读须一句申报
```

注：锚件任务锚行仍显示 debtclear-parallel 收口态（.session-anchor.md 为并联批共享件归协调人改写，本簇不代写防互踩）；penface 批完工态以本档与链面三笔为准。
