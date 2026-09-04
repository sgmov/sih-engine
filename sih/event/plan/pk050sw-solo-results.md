# pk050sw-solo 结果档

> 批：pk050sw-solo 检索缺省位切换批（recall 调用位缺省切确定性向量语义层，词面降为显式回退位）
> 会话：d70d406c04a2afa3（租约自生成）｜会话标识 sess-zcode-260904-pk050sw（ask3 双标识空间，各认各的）
> 日期：2026-09-04 ｜ 队形：单线形 solo，零子代理 ｜ 双仓工地 tools@integral-stage-build 与 engine@main
> 承接：用户预签设计「A/B 门过即切」加得一裁定材料 m-pk050-switch-1（boundary 九发→通道分解→确定性七项核对全绿）；判变件 pk-050 出泊 promoted
> 政策行：用户 2026-09-04 裁定「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——批内零新裁决点，机械链全绿自行收口
> 意图哈希：120646b2（intent 事件，meter 包裹；ask3 双门过：核阅 exit 0 零违规、重放 status ok 三锚，digest passed covered 5）
> 领取登记：lease claim 一笔在先（claimant sess-zcode-260904-pk050sw，ttl 480 分，claims 册在案）

## 一、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| 件一 缺省位切换 | 完成 | recall.py 调用位缺省切语义通道 K=3（DEFAULT_SEMANTIC_K，预登记 K 一次性登记值承 ab-criteria-preregistration.json），增 --word 显式回退旗标（切前词面缺省行为逐字节零改），--semantic 显式旗标语义不变（0 仍即关），recall() 函数级缺省不动；semantic.py 判定逻辑零触碰（git diff 零行）；测试先红（四红九绿）后绿（十三全绿） |
| 件二 金向量随冻 | 完成 | 新缺省行为金向量缺省族三场景（default-paraphrase/literal/broad，K=3）与旧词面行为回退族三场景（fallback-literal/cjk/empty）冻结于 fixtures/golden/，重放寻径约定承 V6（内联夹具临时仓重建、repo 字段剥除、禁绝对路径）；旧语义金向量三件重冻逐字节不变 |
| 件三 七项核对落据 | 完成 | switch_gate.py 七项 S1 至 S7 实跑全绿 verdict pass；tally 材料 check R1 至 R7 全过裁决通过、verify identical、sign 落据 crosscheck-m-pk050-switch-1 b0651374 在链 |
| 件四 pk-050 出泊记账 | 完成 | park exit 一笔 e4042c80（disposition promoted，ruling 载预签设计与得一裁定材料指引）；名册在泊七改六、历史住户十一改十二；泊材料件 pk-050.json 状态改 exited、pk-050-exit.json 落盘 |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 切换依据在案 | A/B 门过即切承预登记判据与用户预签设计，非裸切 | 过 | 预登记件 verdict pass（ΔIG 12.4523 比特对 0.5 比特判据线）；用户预签设计与政策行入 ruling；本批零新裁决点申报在档 |
| F-2 词面回退位行为零改 | --word 输出与切前缺省逐字节一致 | 过 | 实仓四查询 4/4 IDENTICAL（realcorpus-switch-evidence.txt）；夹具级 --word 对 git HEAD 版缺省计划逐字节一致（S7 证据） |
| F-3 semantic.py 判定逻辑零改 | 只切调用位缺省 | 过 | git diff semantic.py 零行；缺省对切前 --semantic 3 计划实仓 4/4 IDENTICAL；--semantic 显式旗标（含 0 即关）语义不变有测 |
| F-4 确定性可复现 | 判定路径零 LLM、双跑逐字节一致、零新增依赖 | 过 | S2 仅 stdlib 零禁词、S3 变更面 ⊆ 允许集零依赖清单、S4 实仓双跑 sha256 4/4 相等 |
| F-5 金向量纪律 | 随冻携带重放寻径约定、禁工地绝对路径、先红后绿 | 过 | 缺省族与回退族六件新冻结重放约定在 selftest 档头；红轮读数 red-run-preimpl.json 四红与绿轮 green-run-postimpl.json 十三绿在档；金向量全部内联夹具临时仓重建 |
| F-6 判变零静默 | 切换即判变，登记出泊全上链 | 过 | 入泊 0b3da7ec（批前在链）与出泊 e4042c80 与名册投影同步，泊材料状态 exited |
| F-7 机械链纪律 | 闸三 bypass 禁管道掩码与 meter 包裹等 | 过 | intent 与 append 全带 --session 加 --sessions；meter 包裹 2>/dev/null；退出码实取；一笔认证覆盖丢失即发现补签（见冲突样本节） |

## 三、七项核对落据节

- 跑件：switch_gate.py（随批入版控，零模型零网络，同输入逐字节同输出）；清单：switch-gate-checklist.json（sha256 前 16 位 382e308215762a14 装配时点）。
- S1 预登记 A/B pass：ab-readings.json verdict pass、三判据全真、k=3 与预登记 K 一致；双件 sha256 在案。
- S2 semantic.py 仅 stdlib 零 LLM：semantic.py 与 recall.py AST import 集 ⊆ 白名单（argparse/json/pathlib/re/sys/math/semantic），禁词十一类扫描零命中。
- S3 零新增依赖：tools 工地 wikirecall diff 加新文件集 ⊆ {recall.py, semantic_selftest.py, fixtures/golden/ 六件, CALL-LOG.md}，零依赖清单。
- S4 双跑逐字节一致：实仓两查询缺省计划双跑 canonical sha256 相等。
- S5 载体终签在链：crosscheck-m-pk037-idf-1 6b3723e8（PROB-017）与 crosscheck-m-pk037-cos-1 70da4959（ALG-011）当日链实存。
- S6 判变登记在链：parking_entered 0b3da7ec doc_id pk-050 当日链实存。
- S7 词面回退位设计在位：--word 计划对切前 HEAD 版缺省计划夹具逐字节一致，CLI 无旗标缺省出语义通道，回退计划零语义键。

## 四、终签节

- 装配：tally-check-input 材料按 assemble_material 字段口径确定性装配（topic_sha256 与 dc_fingerprint 复算，dc 指纹 9bcbf2944c85e148 与九发闸 assessment 记录一致），落 tools 工地 proposition/DES/m-pk050-switch-1/tally-material.json。
- 通道分解披露：九发闸 assessment 判 boundary（子判据 boundary_low 与 basis_consensus 挂，路由法层 refine）承 skill 修订一收尾条款与 m-sett001-crosscheck 先例落确定性核对通道不三跑，七项全绿后 refined 判定 stable_clear；飞轮记录原样保留不抹（九发含尾发 violate 原样在 trail）；披露字段径入材料本体。
- R5 位披露：材料不带 identity_hash——flywheel 九发为围堰 MiniMax-M2.7 席历史测量，无当日 MiniMax 席基线在册；本批零新采样零新标定（工程基线第五条），R5 走代码既有回退分支（双哈希任一缺席即按基线 verdict 判定），同日 zcode 基线 verdict 可用在案，读数「席位当日基线判定可用」。
- 三步：check exit 0 处置裁决通过方向 comply（R1 至 R7 全过零告警）；verify identical exit 0；sign 落据 crosscheck-m-pk050-switch-1 b0651374（scribe crosscheck 专属通道，meter 包裹位承 sign 内嵌调用）。

## 五、出泊记账节

- 出泊：pk-050 于 2026-09-04 经 scribe park exit 上链，事件哈希 e4042c80，disposition promoted，ruling 载用户预签设计「A/B 门过即切」与得一裁定材料 m-pk050-switch-1 指引（boundary 通道分解、确定性七项全绿）与政策行人节点退位声明；泊材料件状态改 exited 并落 pk-050-exit.json。
- 名册：doc/governance/PARKING-v1.md 第 25 行在泊七改六、历史住户十一改十二，pk-050 补行含出泊事件哈希 e4042c80 与机器终签 b0651374；一笔 C006 违规（本批新增行全角括号中文串）即时改写去括号后复读零违规，findings 亲读在案。

## 六、认证清单（认证时点）

| 事件哈希 | 对象 | 备注 |
|---|---|---|
| 120646b2 | 意图笔 scribe intent | meter 包裹，闸三 --session 加 --sessions 双带 |
| b0651374 | crosscheck-m-pk050-switch-1 | attractor sign 机器终签落据 |
| e4042c80 | park exit pk-050 | 出泊 promoted |
| c9ab0cfb | 认证：switch-gate-checklist | 已被并行批活写覆盖丢失，见冲突样本节 |
| 9a1bf670 | 认证：m-pk050-switch-1-tally-check | meter 包裹 append 主树活链 |
| a4c63bea | 认证：switch-gate-checklist（补签） | 覆盖丢失复读发现后重追加 |
| a3df50dd | 认证：pipeline 报告 | meter 包裹 append 主树活链 |
| e76fafa2 | 认证：changed-files 报告 | meter 包裹 append 主树活链 |

## 七、越线与误差申报

1. 证据生成重跑一笔：实仓对表证据文件首跑因脚本文件名基数混用（printf 与 echo 差一换行符）全部误判 DIFFER，即改判式统一命名重跑，4/4 IDENTICAL 与 4/4 EQUAL 如上；首跑误判属证据生成脚本缺陷非数据面差异，两轮原始数据同一。
2. 命名册行 C006 一笔：本批新增历史住户行全角括号含中文串违规，即改写去括号语义零改后复读零违规。
3. R5 回退位一笔：材料不带 identity_hash 走代码既有回退分支，缘由与披露见终签节，属显式申报非暗用。
4. 任务包与 dispatch 化格以 /tmp 拷贝承载读数：两件为主会 authored 输入件随批字节冻结（保 close 归并对表逐字节一致），化格读数拷贝 exit 0 零改动与原件语义等价，原件未触。
5. scribe/CALL-LOG 本批零改：主树该件有并行批未结算残改，worktree 改写将致 close 归并拒，本批 scribe 使用留痕以链事件与本档承载，scribe 调用册尾行随收口附记通道外回填（contribmath 整备先例同形）。

## 八、冲突样本节（pk-045 样本库）

- 认证覆盖丢失一笔：c9ab0cfb（switch-gate-checklist 认证）上链后遭并行批活写覆盖丢失——并行批在 121 至 135 事件窗口写入十二认证加两意图加一入泊加六读数，本批 9a1bf670 落链时复读发现前笔缺席，即补签 a4c63bea；链首哈希 05a8a75e 全程不变零分叉，scribe verify 全程 valid。承 facepark 99b85acc 与 pk-047 撞号先例同族，即 append 锁共存态下活写覆盖丢失样本第十三类实录：先认证后遭覆盖，靠上链复读（认证清单对表）发现加补签恢复，零静默。
- 锁面：本批七把 exclusive 施工面锁零撞零重试；trail append 锁即取即放五轮零等待；施工面与并行批零争用。

## 九、收口读数（认证时点）

- 链：sih-engine/sih/event/trail/2026-09-04.ndjson 认证时点 verify valid，批前 121 事件，本批八笔即 intent 120646b2、终签 b0651374、出泊 e4042c80、认证 c9ab0cfb（遭覆盖丢失）与 9a1bf670 与补签 a4c63bea 与认证 a3df50dd 与 e76fafa2，另有并行批十四笔交错（其窗口内零覆盖本批其余笔）。
- 锁：七把 exclusive 在持（wikirecall、parking、名册行、materials、results、任务包、DES cell），trail append 五轮即取即放已清。
- 泊界心跳（会话首动作）：双线 exit 0 零告警；tools 线 19 件主线 18 侧线 1（pk-042 名册行无泊材料件走侧线 P102 不判）；engine 线 17 件主线 16 废注 1（pk-013 泊材料缺名册现行行走废注 P101 与 P102 不判）——两笔为存量态如实转录，本批不越权处置。
- close 与 reconcile 与终态 verify 读数与双仓提交号见文末收口附记（close 后补记）。

## 收口附记（close 后补记）

- close 五跑实录：第一跑 trail append 锁残持被拒即放锁；第二跑 engine merge_diverge（trail，因并行批 leasewire-solo 归并在途提交了链快照使 base..main 集合扩大）；第三跑 merge-tree 内容冲突（trail 两边各改，我支已按 carrwire 整备形拷入活链超集 wip 70ca29d，main 侧有 leasewire 快照提交）；消解承 queueing 与 latexwire「主树活链通道外提交让位」先例即活链 138 事件超集快照通道外提交 ab0dcdf（--no-verify 加 bypass 登记，scribe verify valid 首哈希 05a8a75e 不变）；第四跑撞主树同名未跟踪件四件（tools DES cell 两件加 engine 任务包与 dispatch），按备份让位归并对表法四步：备份、让位、close 归并、逐件 diff 四件全数 IDENTICAL；第五跑成：双仓归并、双工地拆除、双支删除、会话吊销。
- 双仓提交号：sih-tools 段1 dbc9e560 与归并 62be0e3b 与回填 6cf28e63（scribe CALL-LOG 尾行通道外）；sih-engine 段1 9cf62f8 与链快照 wip 70ca29d 与让位整备 ab0dcdf 与归并 25d257b 与回填 c6bf228；sih-math 零触碰（HEAD 366298b 保持批前）。
- reconcile：sih-engine unrouted 0 与 unbypassed 0 与 cert_missing 0 与 session_orphan 0 全净；sih-tools unrouted 0 与 unbypassed 0 与 session_orphan 0 与 cert_missing 1（entryunique-solo 先前批既有旧账，批前即 1）——相比批前零新增，本批贡献零。
- 链 verify（close 后终态）：status valid 138 事件，first_hash 05a8a75e 与批前一致零分叉，last_hash e76fafa2 即本批 changed-files 认证；本批八笔在链即 intent 120646b2、终签 b0651374、出泊 e4042c80、认证 c9ab0cfb（遭覆盖丢失）与 9a1bf670 与补签 a4c63bea 与 a3df50dd 与 e76fafa2；并行批窗口内交错共笔如实共存。
- 净态终读：lease status active_sessions 0 与 held_locks 0。
- 主树复验：名册 doc/governance/PARKING-v1.md des-001 核阅 exit 0 零违规（归并后主树实跑）；金向量过已提交树复现关即主树 semantic_selftest --golden-only 与全套 semantic_selftest 十三用例与 selftest 六用例与 metrics_selftest 十一用例全 exit 0；缺省路径主树终验语义通道在场（不动点查询头部 ORD-003 与 TOP-003 与 ORD-015）与 --word 零语义键，同语料下 word 对切前缺省与缺省对切前 --semantic 3 双 IDENTICAL 复证（对照构造期读数的差异为 leasewire 归并后语料漂移即语料驱动差异非代码行为变化，承 pk037impl 主树复验同形判读）。
- checkcite 守卫（BATCH-FACE 十点五节）：批主题词首跑 cited 五件两件不在书单闭包如实记 fail，引用面收窄后 pass——本批载体引用 PROB-017 与 ALG-011 与 PROB-014 落书单闭包 exit 0；PROB-011 与 PROB-003 为 pk037impl 批 A/B 读数件内引用非本批新增书单消费，两轮读数件 2026-09-04-pk050sw-solo-checkcite.json 在案。
- 本笔回填提交：close 通道外 wip 形（--no-verify 加 lease bypass 登记），pk037impl 收口附记先例同形。

## 十、队形验证

单线形 solo 成立：本批全部写入由会话 d70d406c04a2afa3（sess-zcode-260904-pk050sw）亲写，零 Agent/Task 子代理调用；缺省判定由 semantic.py 确定性统计程序承载零 LLM，七项核对由 switch_gate.py 机械承载，终签由 attractor R1 至 R7 确定性核对承载，本批采样为零（承工程基线第五条减少 LLM 参与）；链写入全经引擎 scribe 闸三与 attractor sign 专属 crosscheck 通道，零直写链文件。
