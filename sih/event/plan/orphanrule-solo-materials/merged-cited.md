# orphanrule-solo 结果档：无主件处置与留痕规则三件得一裁批（只裁不改）

> 承接：任务包 orphanrule-solo.md 与用户 2026-09-07 令「得一裁」即无主件处置与留痕规则面不落人节点，承 DEC-021 灰区从严过得一与既有路由先例（m-confrule-1 同形）。单件：命题 m-orphanrule-1 完整得一裁，三件裁定值随档落据；两无主件本体与推导面与在役条款零改动，处置执行归后继面。
> 队形：串行 serial（主会起草与开约后验收，串行子代理持全量上下文连续执行链，零再下级子代理）。日期：立包 2026-09-07，实跑 2026-09-08 本地（UTC 2026-09-07 22:33 起链沿 09-07 活跃链，见误差申报一）。会话 sess-zcode-260907-orphanrule（session_id 677fe2d1a120c4b7，双仓租约 sih-tools＋sih-engine，十一路径锁）。

## 意图锚定

- 意图事件：intent_refined（event_hash `a610be20...`）
- record：sih-tools/scribe/reports/2026-09-07-ask3-orphanrule-solo-record.json（三锚引文程序切片：07-on-assay.md L55 鉴只列事实、08-on-settle.md L110 应而不藏、01-ontology-of-names.md L18 承载不撤回）
- validation：sih-tools/scribe/reports/2026-09-07-ask3-orphanrule-solo-validation.json（status ok，anchor_count 3）

## 三件裁定值（本批核心产出）

| 判定点 | 裁定值 | 裁定依据（材料内可机械核验） |
|---|---|---|
| P1 attnanchor/anchor.py 权限位件处置 | **入账**（按治理通道提交，mode 变更随提交落版控） | 四判据逐件核齐全向入账：①内容完整性——复核命令 `git diff --summary` 裁定时刻实核仅 mode change 100644 => 100755，零内容变化无实质分歧面；②归属可溯——attnanchor-solo 会话 b2540a947d8b6484 sessions 台账 revoked 2026-09-07T02:39:03Z 在档，无在飞争议方；③效用品——可执行位服务回锚调用形（AGENTS.md 会话启动节指名直跑，本批实跑 anchor.py 读数在案），回滚即丧失直跑且无对价收益；④协议座——watchcheck 处置协议回滚分支以「非工作产物」为前提，本件系 attnanchor-solo 批工作产物且在役，前提不满足，回滚分支不可启 |
| P2 提案一活数据文件节奏 | **B 案（批节奏提交）**，A 案（gitignore 化）不可立 | ①可追溯链闭合——rebuild 重建件在役性实核四证据在档（`lease call-log rebuild` CLI 在役、calllog/core.py `rebuild_db` L281 实现、calllog/tests/test_calllog.py:96 `test_rebuild_db_deterministic` 双跑逐字节一致绿、calllog-solo 结果档 F-2/F-5 读数绿加 m-calllog-dual-1 终签），但 rebuild 是账本→索引派生，A 案使权威腿 calls.ndjson 本体脱版控，账本历史在版控面断裂，与既裁在役条款（BATCH-FACE calllog 写点纪律：权威腿 calls.ndjson git 版控）及工程基线第四条可验证性三性质相抵，承 PRO-08 应而不藏即 A 不可裁；②无主闸复发面——B 案堵「追加未入版控」行为缺口与 A 案出清 tracked 态同向构造性减少；③注意力预算——B 案随批 settle 节奏提交属批机械链既有环节扩展，非人介面；④处置联动——立 B 即当前脏态按通道入账且后续随批节奏提交。判定权重如实披露：判据①压倒③，未取 A 的零纪律便利 |
| P3 提案二 CALL-LOG 追加强制随批检查位 | **立则**（实施归后继批） | 四判据全向立则：①事实基座——16 件无主清单中 14 件为 CALL-LOG 投影腿（callloghyg-solo 结果档 §四与 F-1 在档）即类问题非孤例；②基线兼容——检查位落 closeguard 属收约检查扩展，确定性程序承载零新增 LLM 参与，不改既有子命令退出码语义，新增拦截条件语义实施批显式申报；③负载收益——检查位使此类无主闸拦截构造性清零，对比 callloghyg 29 行补笔 bypass 兜底的实证重复成本；④重复立法检查——与既有 closeguard 脏位对表是细分扩展非重叠（彼查未提交脏面，此对 CALL-LOG 追加行与权威腿对表） |

落据：stable_clear 加机器终签即三件落据成规则；P1 的提交执行、P2 的形态转换（脏态入账加随批节奏）、P3 的检查位实施归后继执行面，本批零改写零数值断言零代码。boundary 件：无（三件皆 stable_clear 内判定，零转诊断）。

## 一裁读数（gid m-orphanrule-1）

- facet：emit-contract 零 LLM 零网络从工地 facet 跑（cwd 置工地 facet，绝对路径传参；seat ZCode:GLM-5.3-Flash，9 shots，basis 枚举声明 baseline_1 或 baseline_4 或 baseline_5）；回填九发同席即席作答（谱系披露双声明载 frontmatter authored 行：同席起草与同席采样，判对席有利方向即成本加重）；score 9/9 comply、变卦 0%、谨慎信号 0/9、规约引用单类 baseline_4、闸门裁决 stable_clear、空转 0。
- seat-baseline：ledger 尾行为 09-07 席位行，按先例正形制基线（本批当日正身 identity ef2253cc／core e1c9a7fa 配对，props 沿尾行，date 标实跑日 2026-09-08）。
- tally assemble：stable_clear，材料 kind tally-check-input，criteria v3，voids 空。
- attractor check：verdict pass、disposition 裁决通过（passed 12／failed 0／alarms 0）。
- attractor verify：identical（同参形）。
- attractor sign：signed，crosscheck-m-orphanrule-1，event `d099dd46`（event_hash 前8），direction comply，重放锚 proposition/DES/m-orphanrule-1/sign-reports/m-orphanrule-1-signcheck.json。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 命题与源零偏差 | 数据治理 | 命题六处切片与源文件逐字节比对全过（生成器 make_topic_orphanrule.py 在档可重放） | 通过（首核绿）：生成器重放（重放件落 /tmp 零主树写）与在档命题 cmp 逐字节一致 exit 0，六切片（prop1 250／prop2 171／protocol 122／settle110 29／assay55 73／calls_head 198 字符）零漂移；核形三笔（frontmatter 补齐、主标题归位待裁命题段、anchors 段补齐）按勘误条款材料修复完成，修复声明嵌入头注，正文逐行 IDENTICAL 验证在档，parse_topic 解析过（gid／ng medium／n 9／anchors 3） |
| F-2 采样携正身 | 治理 | 九发计分材料携 identity_hash，从工地 facet 跑，席位与正身不混同 | 通过（计分材料 identity_hash ef2253cc 与席位串 ZCode:GLM-5.3-Flash:self-reported 分立；基线件当日正身配对；emit-contract 与 score 两半 cwd 均置工地 facet） |
| F-3 三态终签 | 跨族治理 | check 十二项全过、verify identical、sign 三态如实（refused 透传不掩） | 通过（check passed 12 failed 0 alarms 0、verify identical、sign signed 透传；零 refused 零 boundary） |
| F-4 只裁不改 | 治理 | 推导面与两无主件本体零改动；本批零数值断言零实施；boundary 不重采样凑共识 | 通过（写入面即命题区材料＋结果档批材料＋链笔；attnanchor/anchor.py 与 calllog/calls.ndjson 全程只读复核零触碰；裁定值是规则面结论非数值断言；三件 stable_clear 零 boundary 即无凑共识面） |

## 越线与误差申报

- **误差申报一（跨日实跑与读数日期）**：实跑跨 UTC 日界——本地 2026-09-08 开工，UTC 2026-09-07 22:33 起，链沿任务包请求写入节指定的 2026-09-07 活跃链（该链至当日实跑时点仍为在飞活跃链）。受此影响：gauge record 例行读数 --at 传 2026-09-08 落 09-07 链；泊界心跳首跑 --reference-time 传 2026-09-08；watchcheck 首跑 --at 2026-09-08 报「当日 trail 缺席」工具异常输出在案（fail-closed 设计生效），改按活跃链日期 --at 2026-09-07 复跑成功。首跑 watchcheck 与心跳的退出码经管道 tail 被掩未直读（违「退出码直读禁管道掩码」一次，即时发现），复跑全部直读退出码，首跑读数（无主 5 件／工具线零告警）与复跑一致。
- **误差申报二（P1 复核输出格式）**：复核命令原生输出行首含 git 前导空格一位，与命题照录（剥格式形）非逐字节同形；strip 后逐字节一致，mode change 实质内容零差异。非状态漂移，git 输出格式所致，以裁定时刻只读复核为准如实记档。
- **误差申报三（calls.ndjson 行数漂移）**：命题补充事实载 725 行为起草时刻快照；裁定时刻实核 728 行（后续批正常追加 3 行），tracked 且 M 态追加式与权威腿地位不变，首行 header 与命题照录 IDENTICAL。P2 判定不受影响（判据锚在形态与协议非行数）。
- **误差申报四（基线件制形）**：ledger 尾行为 09-07 席位行（identity 1f44f626／core c0110e40），按先例正形制本批基线：identity/core 换配本批当日正身（ef2253cc／e1c9a7fa），props 沿尾行，date 标实跑日 2026-09-08。
- **对己不利申报（谱系与判定权重）**：其一，同席裁自席起草的规则面——九发回填为同席自答，判对席有利方向即成本加重，谱系披露载 frontmatter authored 行；其二，P1 裁入账对同席前身批 attnanchor-solo 产出有利，但四判据逐条机械核过（复核命令输出、台账 revoked 行、回锚调用实跑、协议前提语义），非偏好导向；其三，P2 裁 B 与工程基线五「减少 LLM 参与」的便利方向相反（A 案零纪律成本），判定依据是判据①可追溯链闭合（既裁权威腿 git 版控条款＋基线四）压倒③注意力预算，权重如实披露候人节点核。
- 其余零越线零申报。

## 管线与对表读数

- 化格：结果档 packs/general-v1（读数随实录补记）；topic.md 同。
- 核阅：引擎件 des-001 对结果档与 topic（工地路径形，先例 exit 0 形）。
- 检词：nomenclator check packs/core 对结果档与 topic（读数随实录补记）。
- 书单对表：拼接扫描形（多 --cited 单值坑勘误在案）扫结果档加 topic 加 responses 合并文（读数随实录补记）。
- 管线逐命令退出码落 orphanrule-solo-materials/pipeline-readings.json。

## 认证清单

| 件 | 类型 | 状态 |
|---|---|---|
| ask3 记录 | 意图记录 | 书简认证（读数随实录补记） |
| ask3 验证件 | 双门读数 | 书简认证（读数随实录补记） |
| 正身件 | 身份报告 | 书简认证（读数随实录补记） |
| 一裁终签 | crosscheck_completed | `d099dd46`（sign 落链，direction comply） |

## 结算读数

（收约后回填补笔：双仓 settle 提交号、放锁收约、链 verify、reconcile、心跳复验。）
---
title: "无主件处置与留痕规则三件一裁：anchor.py 权限位件处置、活数据节奏二择、CALL-LOG 随批检查位"
authored: orphanrule-solo 批命题起草（sess-zcode-260907-orphanrule），2026-09-07，谱系披露：同席起草与同席采样，判对席有利方向即成本加重
ng: medium
n: 9
gid: m-orphanrule-1
---

# 待裁命题

> 待裁命题（自包含，可独立裁定）。令源：用户 2026-09-07 令「得一裁」即无主件处置与留痕规则面不落人节点，承 DEC-021 灰区从严过得一与既有路由先例（m-confrule-1 同形）。
> 元数据：gid m-orphanrule-1，n 9，ng medium，authored 2026-09-07（sess-zcode-260907-orphanrule）。
> 规范源：sih-engine/sih/event/plan/callloghyg-solo-results.md（§四两提案）与 sih-tools/BATCH-FACE.md watchcheck 处置协议节。材料引文程序切片逐字节照录。（orphanrule-solo 批 F-1 核形修复：frontmatter 补齐、主标题归位待裁命题段、anchors 段补齐三笔形修复，判定文本与切片零改，生成器 make_topic_orphanrule.py 重放 cmp 逐字节一致在档。）

## 裁定总则

裁规则与处置不裁数值。判定依据只认五类材料：一即在役协议与既裁条款（BATCH-FACE 处置协议、工程基线与禁止条款）；二即归账批在档证据（callloghyg-solo-results.md 与 attribution-disclosure）；三即两无主件的机械事实（git 只读命令可复核，复核命令随件列明）；四即哲学锚（引文切片如下）；五即提案原文照录。不引外部偏好与便利性论证。每件独立裁定，三值结论即认可立则／不立则／boundary（不发明确结论即 boundary）。

哲学锚切片（逐字节）：

- 08-on-settle.md L110：- 应而不藏：治理诚信——不藏 bug、不藏失效、不藏错误
- 07-on-assay.md L55：司衡的鉴要求：检验时保持"虚"（不预设立场）和"静"（不急于下判断）。验证报告应只列事实，不列"应该怎样"的建议——建议是法的范畴，不是鉴的范畴。

## P1 attnanchor/anchor.py 处置（入账 vs 回滚）

**机械事实**（复核命令：`git -C sih-tools diff --summary attnanchor/anchor.py`，预期输出逐字节一致）：

```
mode change 100644 => 100755 attnanchor/anchor.py
```

即改动仅权限位翻转，内容零字节变化。归属：attnanchor-solo（sessions 台账 revoked 在档即已收约，无在飞争议方）。该脚本是会话启动回锚调用件（AGENTS.md 会话启动节指名调用）。

**裁定判据**：①内容完整性——零内容变化即无实质分歧面；②归属可溯——归属批已结算且其结果档在档；③效用品——可执行位服务回锚调用形，回滚即丧失直跑能力且无对价收益；④协议座——watchcheck 处置协议正典为人节点二值裁决，本裁系用户路由令委托机器执行，协议的回滚分支以「非工作产物」为前提，本件是否满足该前提即裁定核心。四条全向入账即裁入账（按治理通道提交，mode 变更随提交落版控）；任一条反转即裁回滚（git restore 恢复权限位）。

## P2 提案一：活数据文件节奏（A gitignore 化 vs B 批节奏提交）

**材料照录**（callloghyg-solo-results.md §四，逐字节）：

```
### 提案一：活数据文件节奏（候选二择）

- 触发：`calls.ndjson` 类累积面（权威腿）逐笔追加不入版控，与投影腿 CALL-LOG 并行靠补笔兜底，反复撞无主闸。
- 候选 A｜gitignore 化：calls.ndjson 类累积面断言活数据不入版控，快照/基线走确定性重建（`call-log rebuild` 肉身）承载可追溯。
- 候选 B｜批节奏提交：权威腿追加随调用批自身节奏提交入版控，堵补笔兜底缺口。
- 候裁点：A 与 B 二择，裁归得一裁或人节点，本批零实施。
```

**补充事实**：calls.ndjson 现 tracked 且 M 态（725 行追加式），是 callloghyg 归账对表的权威腿（29/29 对表以其为据，attribution-disclosure 在档）；其首行为结构化 header（复核：`head -1 sih-tools/calllog/calls.ndjson`）：

```
{"at":"2026-09-07T03:28:31+00:00","commands":"","event_id":"clog-import-6209e97b8878","exit":"","kind":"header","note":"","occasion":"","session":"","tool":"cascade","verbatim":"# cascade 调用留痕（级联）"}
```

**裁定判据**：①可追溯链闭合——A 案依赖「确定性重建件」（call-log rebuild）的在役存在性与确定性（同参双跑逐字节一致），裁定须核其在役证据；重建件缺席或不确定则 A 的可追溯链开口，承 PRO-08 应而不藏与可验证性三性质即不可裁 A；②无主闸复发面——两案各自使无主闸构造性减少的程度（A 案出清 tracked 态，B 案堵追加缺口）；③注意力预算——工程基线第二三条即人类注意力稀缺，机械自动优于人介面；④处置联动——立 A 即 calls.ndjson 转 A 形态处置（gitignore 加按 A 重建面），立 B 即当前脏态按通道入账且后续随批节奏提交。逐条据材料裁形，不据实现便利。

## P3 提案二：CALL-LOG 追加强制随批入版控（收约检查位）

**材料照录**（callloghyg-solo-results.md §四，逐字节）：

```
### 提案二：CALL-LOG 追加强制随批入版控（收约检查位）

- 触发：16 件无主清单中 14 件为 CALL-LOG 投影腿追加未随批入版控，收约补笔/solo 收约 bypass-orphan 反复兜底。
- 拟则：每批 CALL-LOG 投影腿追加强制随该批 settle 提交入版控，收约检查位对追加行与权威腿对表，缺即拦。
```

**裁定判据**：①事实基座——16 件无主清单中 14 件为 CALL-LOG 投影腿（callloghyg F-1 前态在档）即类问题非孤例；②基线兼容——检查位落 closeguard 属收约检查扩展，不得新增 LLM 参与、不得改既有子命令退出码语义，语义变化须显式申报；③负载收益——检查位使此类无主闸拦截构造性清零，对比每批补笔兜底的重复成本；④重复立法检查——与既有 closeguard 脏位对表及 2026-09-07 勘误四条的关系是扩展不是重叠。三条向立则即立则（实施归后继批），矛盾即不立则或 boundary。

## 锚与落据

单锚 baseline_4 可验证性：P1 复核命令输出可机械比对，P2 重建件在役性可机械核，P3 事实基座与既有条款可机械对表。stable_clear 加机器终签即三件落据成规则；boundary 件转人诊断呈报，唯人节点残余仅此。处置执行（P1 提交或回滚、P2 形态转换、P3 检查位实施）归后继执行面，本命题只裁不改零数值断言。

## anchors

- path: sih-engine/sih/event/plan/callloghyg-solo-results.md
  range: 全文（§四两提案为 P2/P3 材料源）
  note: 规范源（提案照录母本，F-1 逐字节对表基准）
- path: sih-tools/BATCH-FACE.md
  range: watch 对表挂点与处置协议节
  note: P1 协议座（处置协议正典与回滚分支前提即裁定核心）
- path: sih-engine/sih/state/plan/orphanrule-solo.md
  range: 全文
  note: 本批任务包（F 表与红线与裁定路由依据）
{"key": "m-orphanrule-1#r1", "shot": 1, "raw": "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"三件判据逐条核过：P1 零内容变化且归属批已收约、协议回滚分支以非工作产物为前提不满足；P2 重建件虽在役但 gitignore 化使权威腿本体历史脱版控，与工程基线四可验证性三性质冲突故立 B 案批节奏提交；P3 检查位四判据全向立则且与 closeguard 是扩展非重叠。\", \"boundary_flag\": false}"}
{"key": "m-orphanrule-1#r2", "shot": 2, "raw": "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"P1 复核命令实核仅权限位翻转且 attnanchor-solo 已收约，四条全向入账；P2 的 A 案使权威腿退出版控令可追溯链在账本历史处开口，rebuild 只重建索引腿不救账本历史，裁 B；P3 类问题非孤例且检查位是确定性扩展，立则。\", \"boundary_flag\": false}"}
{"key": "m-orphanrule-1#r3", "shot": 3, "raw": "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"权限位件内容零变化无实质分歧面而回锚调用形依赖可执行位，裁入账；活数据节奏案中 A 案与既裁权威腿 git 版控条款相抵，注意力预算虽利 A 而可追溯性优先，立 B 案；CALL-LOG 随批检查位落 closeguard 不新增 LLM 参与且使拦截构造性清零，立则。\", \"boundary_flag\": false}"}
{"key": "m-orphanrule-1#r4", "shot": 4, "raw": "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"P1 四判据全向入账，mode 变更随治理通道提交落版控；P2 判据一可追溯链闭合不满足于 A 案，承应而不藏裁 B 批节奏提交；P3 事实基座在档即 14 件投影腿先例，负载收益对补笔兜底成本占优，三条向立则。\", \"boundary_flag\": false}"}
{"key": "m-orphanrule-1#r5", "shot": 5, "raw": "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"零内容变化即无分歧面，归属可溯效用品在役，回滚分支前提不满足，入账；重建件在役只解索引重建不解权威腿版控历史，A 案可追溯链开口不可裁，B 案随批提交堵缺口成立；检查位扩展 closeguard 非重叠立法，立则。\", \"boundary_flag\": false}"}
{"key": "m-orphanrule-1#r6", "shot": 6, "raw": "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"P1 逐字节复核仅 mode 翻转，归属批 revoked 在档，直跑能力有效用品，入账；P2 的 B 案保持版控历史连续且借既有 settle 节奏零新增人介面，A 案 gitignore 与基线四冲突，裁 B；P3 收约检查位使无主闸对此类构造性清零，立则。\", \"boundary_flag\": false}"}
{"key": "m-orphanrule-1#r7", "shot": 7, "raw": "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"三件均承可验证性单锚：P1 复核命令机械比对过且协议前提核过，入账；P2 权威腿历史不可篡改性要求版控承载，A 不可裁立 B；P3 与既有检查的关系核过为扩展，立则。\", \"boundary_flag\": false}"}
{"key": "m-orphanrule-1#r8", "shot": 8, "raw": "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"P1 入账——四条判据无一反转；P2 立 B——A 案即便 rebuild 在役其确定性重建也不覆盖账本本体历史，链开口即不可裁；P3 立则——四判据全过，实施归后继批。\", \"boundary_flag\": false}"}
{"key": "m-orphanrule-1#r9", "shot": 9, "raw": "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"依裁定总则五类材料裁：P1 机械事实与归属台账与效用品三面齐向入账；P2 提案照录与既裁条款对表 A 案冲突基线四故立 B 批节奏提交；P3 事实基座与基线兼容与负载收益三判据向立则，检查位实施归后继。\", \"boundary_flag\": false}"}
