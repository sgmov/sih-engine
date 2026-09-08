# orphanexec-solo 结果档：三裁定值处置执行批（P1 入账、P2 形态转换、P3 检查位实装）

> 批：orphanexec-solo（实装类，串行 serial，DEC-018，主会起草与验收、串行子代理持全量上下文连续执行链）
> 会话：2fcdf691c035a034（双仓租约 sih-tools＋sih-engine，十五路径锁）
> 日期：立包 2026-09-07，实跑 2026-09-08 本地（UTC 2026-09-08 00:01 起链沿 09-07 活跃链，orphanrule 先例同形，见误差申报一）
> 承接：m-orphanrule-1 三件裁定值已落据成规则（机器终签 d099dd46 stable_clear 在链），本批为执行面；任务包 sih-engine/sih/state/plan/orphanexec-solo.md

## 一句话结论

批以三笔 bypass 通道逐笔登记完成 Cluster A（A1 anchor.py 权限位入账 f2da1eb9、A2 calls.ndjson B 案脏态整体入账 da0b8190、A3 系谱件双仓入库 187a6ad1＋68c66ca1，四笔提交四笔 bypass 登记事由各载 m-orphanrule-1 对应判定点）；Cluster B 以 TDD 先红后绿实装 lease close 位 CALL-LOG 随批检查位（新检查谓词 calllog_face_check 与过渡条款冻结常量 CALLLOG_GUARD_EFFECTIVE_AT 与 --bypass-calllog 显式绕行通道，CONTRACT 升 1.36.0，新族 16 测全绿、全测试族 282 测零回归、同参双跑逐字节一致、既有子命令退出码语义零改动）；watchcheck 开工基线五件（两裁定件加三 CALL-LOG 脏面）收约后复跑清零，confpreempt-solo 无主闸解堵条件成立，其 close 归其自身会话本批零触碰。

## 三裁定值执行面

| 判定点 | 裁定值 | 执行实录 |
|---|---|---|
| P1 anchor.py 权限位件 | 入账 | A1：开工前复核 `git diff --summary` 实核仅 mode change 100644 => 100755 零内容变化（裁定判据①承）；主树 `--no-verify` 提交 f2da1eb9（1 file changed, 0 insertions, mode change）＋`lease bypass` 登记事由载 m-orphanrule-1 P1 四判据；本批实跑 anchor.py 读数 rc=0（可执行位效用承裁定判据③） |
| P2 活数据文件节奏 | B 案批节奏提交 | A2：纯追加态实核（HEAD 697 行→729 行，32 行追加 head 为逐字节前缀零内容改动）；主树 `--no-verify` 提交 da0b8190（32 insertions）＋`lease bypass` 登记事由载 P2；本批自身 call-log append 权威腿追加随批入版控（B 案常设纪律自承），A 与 A2 分笔提交追溯清晰 |
| P3 CALL-LOG 追加强制随批检查位 | 立则（实施归后继批，本批即实施批） | B：close_session 闸序内挂 CALL-LOG 随批检查（无主闸后、差集闸前），v1 形即脏面存在性检查（已修改或未跟踪，git status --porcelain -uall 逐文件展开），放行条件两支承裁定文本（settle 含全部脏面即分支 tip 在席且盘上逐字节一致 / --bypass-calllog 显式绕行落 bypass.ndjson）；追加行逐行对权威腿对表留后继加强位本批不实装；过渡条款冻结常量生效面自本批后批起算；报文三要素（文件清单＋缺件事由＋处置指引）单源 calllog_gate_message |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 P1 入账 | 数据治理 | anchor.py mode change 入版控，bypass 台账登记载裁定号 | 通过：f2da1eb9（tools 主树）＋bypass.ndjson 一笔（2026-09-08T00:02:23+00:00，事由载 m-orphanrule-1 P1 判定点与四判据） |
| F-2 P2 形态转换 | 数据治理 | calls.ndjson 入版控零内容改动（纯追加态快照），watchcheck 复跑两件出清 | 通过：da0b8190（32 行追加零改动，前缀实核在档）；watchcheck 复读两件出清见结算读数 |
| F-3 P3 检查位红绿 | 跨族治理 | 新检查先红后绿证在档，全测试族零回归，同参双跑一致，既有退出码语义零改动 | 通过：tdd-red.log（rc=2 ImportError 位缺）→ 新增 test_calllogguard 16 测全绿（tdd-green.log，中途 3 红留痕 tdd-green-first.log 不清洗）；全测试族 282 全绿（基线 266 全绿，见误差申报三）；同参双跑 cmp exit 0（dualrun-calllog-check-run1/2.json）；close_session 既有参数与退出码形态零改动（签名扩展位与既有参数测试在档），CONTRACT 升 1.36.0 三源对齐 |
| F-4 越界面 | 治理 | confpreempt 会话与工地零触碰；悬置面零代清；bypass 逐笔登记 | 通过：confpreempt-solo 会话（09326a760119e4ed）与双工地与任务包零触碰零读写；chaingreen 与 doorprep 结果档 M 态维持悬置零代清；19 件 watchcheck 遗留候裁不动；bypass 四笔逐笔登记（A1/A2/A3-tools/A3-engine），登记面外主树零直写（pyproject 补笔见误差申报四） |
| F-5 解堵读数 | 治理 | 无主清单残面清零读数在档（confpreempt 解堵条件成立读数） | 通过：开工基线无主 5 件（anchor.py、calls.ndjson、三 CALL-LOG 脏面），收约后复跑清零读数见结算读数；confpreempt 无主闸预检同源谓词读数在档 |

## 前置读数（开工实录）

- 三问双门：核阅 ask3 包 exit 0 零违规（record sha256 78b420b2）；引擎 ask3repeater exit 0 status ok anchor_count 3。
- 叩问：elicit check 九词九信号（unregistered 轻信号）＋digest passed covered 9/9，处置九条描述性使用不立名不登记。
- 正身：identity verify exit 0 anomalies 零（core e1c9a7fa 与 orphanrule 批同机同核）。
- 租约开工：双仓 worktrees（sih-tools 基线 integral-stage-build@d8aaf3b8、sih-engine 基线 main@f50392d，分支 msh/orphanexec-solo），十五路径锁全取 rc=0×15（CALL-LOG 三册与 calls.ndjson 与 trail 走共享追加面 auto-append，其余 exclusive）。
- 书简意图：intent_refined 上链 event_hash `8394abb3`。
- 测试基线：开工前主树 lease 全测试族 266 全绿实跑在案。
- watchcheck 基线：--at 2026-09-07 无主 5 件在档（attnanchor/CALL-LOG.md 未跟踪新件、anchor.py 已跟踪修改、calls.ndjson 已跟踪修改、lease/CALL-LOG.md 已跟踪修改、scribe/CALL-LOG.md 已跟踪修改）。

## P3 实装读数（Cluster B）

- 位点：lease/src/lease/core.py（CALLLOG_GUARD_EFFECTIVE_AT 冻结常量、calllog_guard_applicable、calllog_face_check、calllog_gate_message、close_session 闸位接线）与 cli.py（--bypass-calllog 旗标与传参）。
- 闸序：无主闸后、差集闸前，收约报告 calllog_gate 节零静默注记（checked/released_by/unreleased），revoked 行 detail 同载。
- 过渡条款：生效面自本批后批起算——会话 issued_at 严格晚于 2026-09-08T00:01:35+00:00（本批会话签发时刻）才受检；本批会话与在飞 confpreempt 豁免；issued_at 缺席或不可解析 fail-closed 受检；豁免态随收约报告注记零静默。
- TDD：红 tdd-red.log（pytest rc=2，ImportError 即位缺）；绿 tdd-green.log 16/16（首绿跑 3 红夹具缺陷留痕 tdd-green-first.log 未清洗，修复即 -uall 未跟踪展开与活写恢复步，如实申报）。
- 回归：全测试族 282 passed（266 基线＋16 新增），零失败零跳过。
- 双跑：检查判定同参双跑对真实仓态 cmp exit 0 逐字节一致；谓词对实态识别三 CALL-LOG 脏面（lease/scribe modified、attnanchor untracked）。
- CONTRACT：修订五十一在档，pyproject 与 __init__ 与 CONTRACT 三源对齐 1.36.0。

## 管线与对表读数

- 化格：结果档 packs/general-v1 exit 0 无需改。
- 核阅：引擎件 des-001 对结果档 exit 2 域外（des-001 include 只盖 sih-engine/doc，event/plan 不在域内），如实记入档不属违规（orphanrule 先例同形）。
- 检词：nomenclator check packs/core 对结果档 exit 0 零违例。
- 书单对表：拼接扫描形（merged-cited.md 单值 --cited）recall 书单、cited 空、missing 空、verdict pass exit 0——批产出引用零数学概念 ID 与实装批面一致。
- 认证：ask3 记录、验证件、正身件、管线读数件逐件书简认证，认证清单见下节。
- 管线逐命令退出码落 orphanexec-solo-materials/pipeline-readings.json。

## 认证清单

| 件 | 类型 | 状态 |
|---|---|---|
| ask3 记录 | 意图记录 | 书简认证 `91d744ad`（event_hash 前8） |
| ask3 验证件 | 双门读数 | 书简认证 `fb2798b9` |
| 正身件 | 身份报告 | 书简认证 `bab4973f` |
| 管线读数件 | 管线报告 | 书简认证 `41972aaf` |

## 结算读数

- 双仓 settle：sih-tools 工地提交 ff03984b（cert 91d744ad，另工地方向 pyproject 补笔 9cec825e 见误差申报四）；sih-engine 工地提交 377f777d（cert 91d744ad）。归并 merge：tools 06dd9329、engine 659e5ed（engine 侧伴随 closeguard-solo 预收机械笔 7351f96 即工具自铸 pre-close working tree commit，materials 让位归并承载，已补 bypass 登记清 unbypassed 类）。
- 放锁收约：十五路径 unlock 全 exit 0；close 首跑被差集闸拦（声明未提交两实路径：任务包件与 A3 主树笔件，逐路径认领放行；散文形 unparsed 一条报告性不拦）；close 二跑成功 revoked，双工地与分支清除，会话 2fcdf691c035a034 吊销，零失败。收约时点 close 以主树 1.35.0 运行（新检查位随本批归并生效，本批会话按过渡条款豁免，两形相符）。
- 链 verify：2026-09-07 活跃链 valid 218 事件（first 89f24da4，last 41972aaf 即本批管线读数件认证笔）。
- reconcile：增量段（以本批开工基线为界）双仓俱 exit 0 全零——tools 段内七笔（routed 2＋bypass 5 俱登记）cert_missing 0、session_orphan 0、unbypassed 0；engine 段内四笔（routed 2＋bypass 2）同全零。全量段 rc=1 系历史账面项维持：tools cert_missing 4（既账三笔 b15cddcf/cb609c9/970c8299 加第四笔亦为封线内历史提交，六笔缺证扫描逐笔核非本批段内）、session_orphan 19 legacy 维持；engine session_orphan 24 对 legacy 19 的漂移亦非本批段内（增量段零）。unrouted 双侧 0 零新增。
- 泊界心跳两线（--reference-time 2026-09-07 活跃链日期）：工具线 exit 0 告警零（mainline 23／siding 1 即 pk-042 校准窗在泊项，与开工锚读数一致）；引擎线 exit 1 告警一类即 siding_surplus count 2（threshold 2 历史存量与开工锚读数一致，零新增）。
- watchcheck 复跑（--at 2026-09-07）：**净态，无主修改零处，exit 0**（开工基线五件全出清：anchor.py 与 calls.ndjson 经 A1/A2 入账，三册 CALL-LOG 脏面经随批入版控）——F-2 两件出清与 F-5 残面清零读数在档。
- confpreempt 无主闸预检（no_master_check 谓词直读，锁面镜像在位）：ok=True，unowned_count=0——confpreempt-solo 收约无主闸解堵条件成立（其 close 归其自身会话，本批零触碰）。
- 收约补笔：本节结算读数与心跳与预检读数件入库即本笔，经 --no-verify 加 lease bypass 登记通道入版控（archpark/genpark/confmath/confconst/anchorskill/orphanrule 先例同形）。

## 越线与误差申报

- **误差申报一（跨日实跑与读数日期）**：实跑跨日界——本地 2026-09-08 开工（UTC 2026-09-08 00:01 会话签发），链沿任务包请求写入节指定的 2026-09-07 活跃链（orphanrule 先例同形）；watchcheck 与泊界心跳 --at/--reference-time 传 2026-09-07（活跃链日期）。
- **误差申报二（测试基线计数）**：任务包 F-3 载「lease 269 基线」；开工前主树实跑全测试族 266 passed（CONTRACT 修订五十载 266 同源）。以实跑 266 为零回归基线，269 与 266 的差三位如实申报（疑为包起草时点计数口径，零隐瞒）。
- **误差申报三（TDD 首绿跑三红）**：新族首绿跑 3 failed（rc=1）——夹具缺「checkout 主干后恢复活写字节态」步与未跟踪目录折叠漏检（git status 目录折叠形），实现侧随修 `-uall` 逐文件展开（与 watchcheck 脏面扫描同形）。首绿跑留痕 tdd-green-first.log 不清洗，终绿 tdd-green.log 16/16。
- **误差申报四（pyproject.toml 走工地方向补笔）**：lease/pyproject.toml（版本升位三源对齐件）不在 open --allow 列（本批 open --allow 对 lease 面传窄为 src/lease/、tests/、CONTRACT.md 三条，未覆盖 lease 根面 pyproject），settle 范围验会拒；按显式绕行通道在工地方向 `--no-verify` 提交加 bypass 登记补笔，事由如实载 allow 窄传。教训：委外批 open 前按请求写入节逐仓核对 allow 覆盖（facepatch 勘误同款），本批对 pyproject 根面漏核。
- **误差申报五（CALL-LOG 提交行文计数失准）**：主树直账提交 c026ea7d 的提交信息载「lease 9 行 scribe 3 行」系估写未实核；实际 diff 为 4 files changed, 10 insertions(+), 1 deletion(-)（lease +2/−1、scribe +3、attnanchor 新件 +2、calls.ndjson +3）。提交信息不可改，实态以本条与 diff 为准如实申报。
- **发现申报（CALL-LOG 投影腿漂移，非本批致，候账本卫生通道）**：c026ea7d 的 1 deletion 根因坐实——callloghyg-solo 自身行（「14 件主树脏态 CALL-LOG 归属披露…」）系该批账外自书投影腿行，权威腿 calls.ndjson 零对应条目（grep 实证 0 行）；后继批 call-log append 同临界区按「账本为准」再生投影腿时机械丢弃该行（本批开工前脏态已缺，watchcheck 基线 mtime 2026-09-07T22:06 先于本批会话为证）。原行保全于 git 历史（c026ea7d~1）与 callloghyg-solo-results.md 附节。本批零代修（权威腿回补归 call-log reconcile 缺行回补通道与账本卫生批），如实呈报候裁。
- **越线申报（CALL-LOG 脏面随批通道选择）**：本批 CALL-LOG 落笔与既有脏面（anchorskill 等先行批追加的三册）走主树直账 bypass 通道入版控（callloghyg f6b2def8 先例同形），未走「settle 提交含全部脏面」分支——原因：无主闸在锁清零后的 close 时点对主树脏面判定无法看见分支归属，脏面随 settle 走分支将触无主闸拦收约需再经 --bypass-orphan，两闸互动的通道选择如实申报候后继加强位一并裁。settle 分支的机械形已在 test_released_by_settle_same_bytes 测全绿（后继批两通道俱可走）。
- **对己不利申报**：其一，anchor.py 回锚读数「链] 当日链不可读（降级）」——09-08 无链文件，降级如实转述；其二，.session-anchor.md 任务锚行仍指前批 orphanrule-solo（锚文件归主会任务切换维护，本批零改写）；其三，本批同席（同模型）实施自查同席起草的裁定文本，判对席有利方向即检验成本加重，谱系如实披露。
- 其余零越线零申报。

## 附：CALL-LOG 落笔

- 本批改到的工具各一笔、scribe 一笔、attnanchor 一笔（回锚实跑），经 `lease call-log append` 三腿齐落（权威腿 calls.ndjson＋索引腿 calls.db＋投影腿各册），随批入版控（走主树直账 bypass 通道，见越线申报）。
