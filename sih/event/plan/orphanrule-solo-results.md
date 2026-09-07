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

- 化格：结果档 packs/general-v1 exit 0 无需改；topic.md 同。
- 核阅：引擎件 des-001 对结果档与 topic 均 exit 2 域外（des-001 include 只盖 sih-engine/doc，event/plan 与 sih-tools 命题文件不在域内），如实记入档不属违规（callloghyg 先例同形）。
- 检词：nomenclator check packs/core 对结果档与 topic 均 exit 0 零违例。
- 书单对表：拼接扫描形（merged-cited.md 单值 --cited）recall 书单 67 概念、cited 空、missing 空、verdict pass exit 0——批产出引用零数学概念 ID 与命题面一致。
- 管线逐命令退出码落 orphanrule-solo-materials/pipeline-readings.json。

## 认证清单

| 件 | 类型 | 状态 |
|---|---|---|
| ask3 记录 | 意图记录 | 书简认证 `035ab4f5`（event_hash 前8） |
| ask3 验证件 | 双门读数 | 书简认证 `20be9a28` |
| 正身件 | 身份报告 | 书简认证 `095857e9` |
| 一裁终签 | crosscheck_completed | `d099dd46`（sign 落链，direction comply） |

## 结算读数

（收约后回填补笔：双仓 settle 提交号、放锁收约、链 verify、reconcile、心跳复验。）
