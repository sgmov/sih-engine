# 工程基线 v1：sih-engine 工程层操作约束正典

本文件是 sih-engine 工程层操作约束的正典档，承 pk-070 出泊由 AGENTS.md「工程基线与禁止条款」节整体迁入，甲案落据即 m-hygp070-1 九发 stable_clear 终签 f6263808。正典在此，AGENTS.md 同名节改指针为入口投影。正文自前像程序切片逐字节迁入零语义改动，diff 自证随批材料可复算。

## 概览 {#overview}

- 本节承载 sih-engine 的工程层操作约束，每条约束标注来源类型，区分已有哲学支撑与待哲学锚定，来源类型五类::[基线正文](#baseline-body)
- 工程基线五条即确定性程序与信息洪流与异常信号与可验证性与减少 LLM 参与::[工程基线五条](#engineering-baseline-five)
- 工程禁止条款五条由工程基线与失败复盘直接推导，agent 必须遵守::[工程禁止条款](#engineering-prohibitions)
- 待哲学锚定条目的地位两类比照，同时是哲学仓后续演化的候选输入::[待哲学锚定条目的地位](#pending-philosophy-anchor)
- v1 于 2026-09-08 承 pk-070 出泊迁入，三层固定即 git 版本化加哈希入 trail 加结果档在链::[版本](#version)

## 基线正文 {#baseline-body}

本节承载 sih-engine 的工程层操作约束。每条约束标注来源类型，区分已有哲学支撑与待哲学锚定。

来源类型五类：

- 哲学命题：承接 sih-philosophy 流衍段或复归段或链外补充的命题条目，标注命题 ID。
- convergence 层对照命题：承接 sih-philosophy convergence 层单盲推导体系的工程命题，标注命题编号与原文定位。单盲推导体系是与流衍链对照验证的自底向上骨架，认识论地位是对照证立，不是流衍命题本身。
- 铁律：承接 convergence 层铁律，标注铁律编号，标注其精确指称域。
- 工程实证经验：来自工程落地失败复盘，哲学仓无对应命题，标注待哲学锚定。
- 工程基线：本工作区对话共识，无哲学命题直接支撑。

### 工程基线五条 {#engineering-baseline-five}

第一条，确定性程序是治理操作的唯一执行者。LLM 只生成符号材料，不拥有写入知识包、事件流、意图锚定的权限。治理操作的执行归确定性程序，LLM 产出仅作为待校验的符号材料。来源类型：工程基线，即对话共识。哲学相邻命题：PRO-07 鉴，检验由可重复程序承载。

第二条，信息洪流是旧仓失败根因。LLM 的生成速度与人类审查注意力之间存在结构性不对称，不是工程失误，是结构性约束。治理系统必须假设人类注意力是稀缺资源，将人类介入点限制在确定性程序无法自动处理的异常范围内。来源类型：工程实证经验，来自旧 sihankor 仓失败复盘。哲学对照基础：convergence 层单盲推导体系 P3.1 退化机制论证了注意力预算的结构性稀释，P3.2 外化管理论证了状态外化的必然性，均与信息洪流同源。哲学相邻命题：PRO-03 道二、PRO-08 应，为相邻但未精确覆盖。说明：单盲推导体系在 convergence 层有对照基础，但 llm-friendly-build 检索路径暂未覆盖，须回到 convergence 原文核验。

第三条，人类注意力只投向异常信号。治理系统的设计须确保确定性程序自动处理绝大多数操作，人类只在视图告警时介入，不看原始日志。来源类型：工程基线，由第二条直接推论。

第四条，可验证性约束。所有写入操作须满足：操作来源可追溯、操作结果可机械校验、操作历史不可篡改。LLM 的不可复现输出不得作为治理决策的最终依据。来源类型：工程实证经验。哲学对照基础：convergence 层单盲推导体系 P3.2 外化管理论证了外化存储的持久性、版本化、可审计性三性质，是本条的对照证立。说明一：本条与 convergence 层铁律 #6 方法学可重现原则同源精神，但指称域不同。铁律 #6 约束的是核心洞察层收敛率算法的可重现，本条约束的是工程写入操作的可验证，不构成对铁律 #6 的承接。说明二：单盲推导体系在 convergence 层有对照基础，但 llm-friendly-build 检索路径暂未覆盖，须回到 convergence 原文核验。

第五条，治理延伸是减少 LLM 参与，而非增加。用更多 LLM 调用对抗 LLM 不确定性的模式，在治理核心中不采用。治理靠确定性程序，LLM 只作为符号材料的生成器，但生成过程受哲学命题约束，使产出对齐意图。不生成不等于不引导：司衡不产出实质语义内容，但通过前置约束注入、意图锚定、上下文加载锁定生成方向。来源类型：工程基线，即对话共识。

### 工程禁止条款 {#engineering-prohibitions}

以下条款由工程基线与失败复盘直接推导，agent 必须遵守：

- 禁止 LLM 直接修改知识包或意图锚定。信息洪流的根因之一是 LLM 的不可复现写入使审查者无法追溯变更来源。
- 禁止用不可复现的多 Agent 交互产出作为治理决策依据。注意：多主体协作本身是鉴层打破自证循环的哲学机制（PRO-07），不是禁止对象。禁止的是不可复现的交互实现与用 LLM 调用堆叠替代确定性验证。
- 禁止用 LLM 调用堆叠替代确定性验证。多 Agent 协同、红蓝对抗等模式若用于治理决策的执行环节，属于此类。
- 所有治理操作必须写入事件流，留痕不可篡改。承接 PRO-08 应而不藏，留痕是应鉴循环的构成性条件。
- 人类只通过视图介入，不看原始日志。

### 待哲学锚定条目的地位 {#pending-philosophy-anchor}

本节标注的条目分两类。第一类是信息洪流与可验证性约束，当前状态为工程实证经验的提炼，在哲学仓 convergence 层单盲推导体系中有对照基础，即 P3.1 退化机制与 P3.2 外化管理，但未被流衍链立为独立命题。它们不假装已是流衍命题，也不因为未立为流衍命题就放弃工程约束力。第二类是其他工程基线条目，来自本工作区对话共识，无哲学命题直接支撑。

这些条目同时是哲学仓后续演化的候选输入。若工程实践反复验证其有效性，哲学仓可按自身演化程序即 PRO 流衍段推导链，决定是否新增独立命题或扩展现有命题。工程层不越俎代庖，不自行将其升格为哲学命题。


## 版本 {#version}

v1 于 2026-09-08 承 pk-070 出泊迁入即本文件首版。令源两笔照录：用户 2026-09-06 裁定原话「这个工程基线应该是司衡引擎的运行机制的基石，不应该放在AGENTS里。你这个记一笔，未来要清的。」与用户 2026-09-08 测量令「PK-070 过得一」经 m-hygp070-1 九发 stable_clear 终签 f6263808 裁甲案即迁入 sih-engine/doc/governance/ 治理决策档族新立 BASELINE 正典档，AGENTS.md 改指针，乙向界族与丙宪法档不采。正文自 AGENTS.md 同名节程序切片逐字迁入零语义改动，diff 自证在批材料 migration-diff-proof.json 可复算。三层固定即本文件 git 版本化随批 settle 归并、内容哈希入 trail 认证、出入泊笔与结果档在链。
# baselineexit-solo 任务包：pk-070 迁移实装批（工程基线正典落位）

> 令源：pk-070 出泊条件经用户 2026-09-08 令「PK-070 过得一」测量落据——m-hygp070-1 九发 stable_clear 终签 f6263808 裁甲案（迁入 sih-engine/doc/governance/ 治理决策档族新立 BASELINE 正典档，AGENTS.md 改指针；乙向界族与丙宪法档不采）。本批即迁移实装。会话号 sess-zcode-260908-forkA-baselineexit。

## 范围 {#scope}

1. 正典档落位：新立 sih-engine/doc/governance/BASELINE-v1.md，承载 AGENTS.md「工程基线与禁止条款」全部内容（五条基线＋禁止条款＋待哲学锚定条目地位节）逐条迁入，结构照治理档族形（概览锚节＋版本节；来源类型标注逐条保留），文走化格核阅检词管线，内容哈希认证上链。
2. AGENTS.md 改指针：工程基线与禁止条款两大节正文替换为指针（一句指向 BASELINE-v1.md 路径加「正典在此，本文件为入口投影」），其余节零触碰。AGENTS.md 不在 git 仓，改动即落盘留 diff 申报（这也是 pk-070 病灶本身，正典入仓后指针残留属可接受态，如实记档）。
3. pk-070 出泊：pk-070-exit.json 落位（照 pk-044-exit 形，ruling 载甲案测量令源与终签 f6263808），PARKING-v1.md 名册行移历史住户（附出泊链笔），出入泊笔经引擎 scribe 落链。
4. 三层固定：BASELINE-v1.md 入 git 版本化（随批 settle 归并）＋哈希入 trail 认证。

## 产出 {#outputs}

任务包本件、BASELINE-v1.md、AGENTS.md 指针改后全档、pk-070-exit.json、名册更新、结果档 sih-engine/sih/event/plan/baselineexit-solo-results.md、ask3 记录与验证件、正身件、租约与锁实录、意图链笔与出泊笔、认证清单、大白话节、机器可读投影 baseline-exit.json 认证上链、越线与误差申报、双仓 settle 提交号、链 verify 与 reconcile 读数。

## 红线 {#redlines}

- 迁移零语义改动：基线五条与禁止条款正文逐字保真（diff 自证），结构词（节标题锚点）可适配治理档族形。
- AGENTS.md 其余节（概览、会话启动、文件索引等）零触碰；锚文件 .session-anchor.md 零触碰（并行窗共用）。
- 主树零直写（本任务包主窗预落除外），链文件只经引擎 scribe 写位，禁管道掩退出码，误差红证如实记档。
- 并行窗面零触碰：entrydocs（doc/guide 与 README 与 CASCADE.json）、confreopen（confledger 与置信度面）、scrutpath（src 与核阅器源码）。trail 为共享追加面照常写。
# baselineexit-solo 结果档：pk-070 迁移实装批（工程基线正典落位）

> 承接：任务包 baselineexit-solo.md 与用户 2026-09-08 令「完整执行 baselineexit-solo 批直至双仓 close 收约与 reconcile」；出泊裁决由 m-hygp070-1 九发 stable_clear 终签 f6263808 承载即甲案，本批只实装不裁。
> 队形单线形 solo，日期 2026-09-08，会话 sess-zcode-260908-forkA-baselineexit（租约 session_id c25525d869cec4e3，双标识空间各认各的）。

## 意图锚定

- 意图事件：intent_refined event_hash 前 8 `16ae7d33`
- record：sih-tools/scribe/reports/2026-09-08-ask3-baselineexit-solo-record.json；validation 同目录 status ok anchor_count 3
- 三锚引文程序切片（01-ontology-of-names.md L18 承诺不撤回、08-on-settle.md L110 应而不藏、07-on-assay.md L55 鉴只列事实）于生成器 make_ask3_baselineexit-solo.py，禁手打承契约。

## 前置读数

- 叩问：六轻信号俱未注册即正典档与指针化与入口投影与名册补记与迁移实装与逐字保真，处置后 digest passed covered 6。
- 正身：identity verify anomalies 0（identity.hash 1a4d7ec3）。
- 回锚：五行锚取读在案；任务锚首行按本批红线零触碰未改写（.session-anchor.md 并行窗共用，如实申报）。
- 判据扫：degraded 假零降级，C1/C3/C4/C5 达成、C2 在飞，两线泊界告警双零即净态零动作。
- watch 对表：无主 10 件俱 CALL-LOG 投影腿与 calls.ndjson 候清项，前窗批遗留非本批活面不代清。
- 例行读数：convergence 0.125 与 adoption 0.777778 与 mergeback 0.029412 三维落链。
- 泊界心跳：两线告警双零（引擎 51/2/9、工具 24/1/0）。
- 协调面：现势活跃锁零，entrydocs 与 confreopen 与 scrutpath 并行窗面全程零触碰。

## 迁移读数：四节实态

- 正典档落位：sih-engine/doc/governance/BASELINE-v1.md 新立，正文自 AGENTS.md 前像程序切片逐字节迁入 37 行（body_sha256 前 16 `9eab24de82779e31`），结构照治理档族形即概览锚节加基线正文加版本节，来源类型五类与逐条来源类型标注原样保留；生成器 make_baseline_v1.py 随批落档禁手打。
- AGENTS.md 指针化：工程基线与禁止条款两节正文 40 行替换为指针一句即「正典在此，本文件为入口投影」，183 行改 146 行，节外逐字节不变机械自证为真；AGENTS.md 无仓版控，改动落盘留 diff 申报即 AGENTS-pointer-diff.patch，指针残留属可接受态如实记档。
- pk-070 出泊：出泊材料按 pk-044-exit 形落 sih/state/parking/materials/pk-070-exit.json，ruling 载两笔令源照录与终签 f6263808，出泊链笔 87e85363（parking_exited promoted）；原在泊材料件 pk-070.json 零触碰真相在链。
- 名册更新：PARKING-v1.md 在泊十四项改十三项，pk-070 散文段移历史住户二十四项首位附出泊链笔，补记句照 constclear2c 先例形；投影与链不符时以链为准。
- 三层固定：git 版本化随批 settle 归并、内容哈希入 trail 认证、出入泊笔与结果档在链。

## 迁移零语义改动自证

- 方法：正文不由人手写，由 make_baseline_v1.py 从前像切片生成，再以独立读数复算即源节 37 行与 BASELINE-v1.md 正文区逐行全等，body_sha256 与 doc_sha256 落 migration-diff-proof.json。
- 源节字符集与规则靶面预检：C001 破折号零、C002 字符集零越界、C006 全角括号仅（PRO-07）合法 slug、C007 与 C009 判定性常数声明形零、粗体与表格与块引用零，逐字保真与管线闸无冲突。

## 管线读数

- 化格：读数随收约回填。
- 核阅：des-001 对 BASELINE-v1.md 与 PARKING-v1.md 域内读数与对任务包与结果档域外 exit-2 如实记档，主树归并后复验读数随回填。
- 检词：nomenclator packs/core 读数随收约回填。
- checkcite：topic 锚引读数随收约回填。

## 认证清单

| 件 | 认证哈希前 8 |
|---|---|
| 待收约回填 | 待收约回填 |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 正典档落位 | 数据治理 | BASELINE-v1.md 新立走管线认证上链，正文逐字保真 | 通过（37 行切片迁移，diff 自证在档，管线读数随回填） |
| F-2 AGENTS.md 指针化 | 治理 | 两节正文替换为指针，其余节与锚文件零触碰 | 通过（节外逐字节不变自证为真；.session-anchor.md 零触碰） |
| F-3 pk-070 出泊在链 | 数据治理 | parking_exited promoted ruling 照录终签入据 | 通过（87e85363，f6263808 入 ruling，重入拒零触发） |
| F-4 写入仅 allow | 治理 | 写入仅 allow 面所列路径，并行窗面零触碰 | 通过（写入面即 allow 九路径；原泊材料零触碰） |
| F-5 链面全绿 | 治理 | 双仓 settle、close、verify valid、reconcile 零新增 | 待收约回填 |

## 越线与误差申报

- 叩问 elicit check 首跑退出码被管道掩蔽一笔即 tail 掩 RC 违坑位禁令，即重跑捕真退出码 1 六轻信号，红证即本申报与首跑信号文件俱在档。
- 任务锚首行未按会话启动节律改写，因本批红线明令 .session-anchor.md 零触碰（并行窗共用），两约束冲突取红线从严，完工回显五行锚补偿，如实申报。
- AGENTS.md 指针残留态：同名节留指针即 pk-070 病灶本身的可接受残留，正典入仓后按任务包如实记档。
- 其余误差零申报。

## 结算读数

- 待收约回填。

## 大白话节

- 工程基线五条搬家了：从前住在 AGENTS.md 这本工作手册里，手册不在任何版本库里、改了没人审、丢了没处找；现在搬进引擎治理文档区立了正典档案，走格式、核阅、检词三道检查，内容一个字没变，机器 diff 可以逐行对证；手册里原地留了一行指路牌写明正典在哪。
- pk-070 这张停泊了 30 天时限的欠条销账了：当初记账是因为用户说工程基线是引擎的基石不该放在手册里；现在测量过了、搬家做完了、销账笔迹在链上，泊位名册同步划到历史住户。
