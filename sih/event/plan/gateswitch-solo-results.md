# gateswitch-solo 结果档

> 批：gateswitch-solo boundary 判据换闸、判变逐件复核与探针退役 A/B（任务包 sih-engine/sih/state/plan/gateswitch-solo.md）
> 会话：4bdcc62b5357c7a8（租约自生成）｜会话标识 sess-zcode-260905-gateswitch（ask3 双标识空间，各认各的）
> 日期：2026-09-05 ｜ 队形：单线形 solo，零子代理 ｜ 两仓工地 tools@integral-stage-build 与 engine@main
> 承接：pk-049（换闸挂起）、pk-048（判变预览清单 114 件）、pk-054（探针退役 A/B）、pk-058（判据观察联动停批条件）
> 意图哈希：630a3bd5（ask3 双门过：核阅 exit 0 零违规、重放 status ok 三锚；叩问七信号 digest 全 covered；意图事件 4257a09c）

## 一、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| gate-01 换闸命题重新立题与九发测量 | 完成 | 新 gid gateswitch-switch-2（谱系披露接 facetmath-switch-1，通道分解改写，改写次数 1），九发合同模式 stable_clear，材料在 facet/contracts/gateswitch-260905/ |
| gate-02 执契终签与缺省拨闸 | 完成 | attractor check 12 项全过、verify identical、sign 裁决通过（链 fb2e6d9d）；maturation_gate.py 两缺省位 ratio→test 拨闸；测试族 482 绿；金向量重放 archived_mismatch=0 |
| gate-03 114 件复核账本 | 完成 | 复核脚本双跑逐字节一致；构成 74+40 与 pk-048 登记逐件吻合；冻结 868 件零字段漂移；独立 p 值复算 114/114 全符；零由严到宽 |
| gate-04 探针退役 A/B 设计呈裁 | 完成 | pk054-ab-gate-design.md 在档（老路径 vs 上下文注入、四判据三切换条件、pk-044 硬前置、键不随件走）；实装以得一裁为门，批内零实装 |
| gate-05 三泊件出泊证据包 | 完成 | pk-048/049/054-exit-evidence.json 三件在档；出泊事件零代落呈人节点 |
| F-6 停批纪律 | 未触发 | 依据族分散签名未复现（依据族单源 baseline_4），pk-058 联动分析零需要，零加采零重跑 |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 换闸测量 | 新 gid 九发 stable_clear 或停批；材料哈希绑定在 facet/contracts/ | 过 | 合同 sha256 f8fa7765f0d0029b、响应 sha256 ab94ade3d8d97f15、计分材料 gate_verdict=stable_clear 三绑定在档；正身 hash d37658542bf872ef 配对 |
| F-2 执契拨闸 | check→verify→sign 全过；缺省 test 在役；测试零回归；金向量重放一致 | 过 | R1-R7 十二项 passed 零 failed；verify identical；七用例先红后绿历史与拨闸后新语义全绿；全测试族 482 绿（6 红为 worktree 环境态见偏差申报）；拨闸后金向量复算 archived_mismatch=0 |
| F-3 复核账本 | 114 件零漂移；核记入档；双跑逐字节一致 | 过 | run1/run2 cmp 全同；change_breakdown 74+40；independent_p_all_match true；loosening_ids 空；stop_flag false |
| F-4 A/B 门 | 设计呈裁在档；切换判据显式 | 过（呈裁态） | pk054-ab-gate-design.md；实装待一裁，批内零代落 |
| F-5 出泊证据 | 三件证据包在档；出泊零代落 | 过 | 三 evidence json 在 materials/；出泊事件零代落 |
| F-6 停批纪律 | 分散复现即停批含 pk-058 联动 | 未触发 | 前例签名（三值分散）未复现；零钓鱼重跑 |

## 三、段一测量纪要（gate-01/02）

**立题形**：机制规则类材料性断言——「二项检验判据使 boundary 判定基准成为第一类错误率上界显式在册的可机械校验对象（满足基线四），占比阈值判据（率 < 0.34）判定基准为工程经验值无错误率语义（不满足）」。前置分道自查：非裁断形、非祈使尾、非状态完成类、非跨文档比对类；事实面（函数族注册、探针 38 trail 对跑）归确定性核对不送采样。

**测量读数**：9 发全 comply，变卦 0%，谨慎信号 0/9，规约引用 1 类（baseline_4），闸判清晰稳定。**前例 pk-058 同源签名（依据族三值分散）未复现**——通道分解收窄规范面后依据族单源收敛，basis_consensus 过，basis_near 未触发。

**执契链**：围堰 tally assemble 零判断装配（当日席位基线 seat-baseline-zcode-2026-09-05-gateswitch.json 判定可用体温 0.0，core_hash 82f460c2 配对）→ 引擎件 attractor check（R1-R7 十二项全过）→ verify（identical）→ sign（机器终签，链 crosscheck 事件 fb2e6d9d-8d19-476e-ae6e-888ece9a3793，event_hash 304289ac 前十六位起）。agent 一字未签，终签主体是确定性脚本（修订六七）。**人类知晓无需复签；一票绝对反对经 append inconsistency 翻案；边界变更经 watch 浮出。**

**拨闸**：maturation_gate.py assess_maturation_v3_base 与 assess_maturation_v3 两缺省位 boundary_criterion "ratio"→"test"，文档串三处同步改写拨闸门据与历史。生产行为变更随批只向前生效，在档历史 verdict 零改写。占比判据退役为显式回退位（boundary_criterion="ratio" 显式传参位保留供审计对照）。

**测试与金向量**：test_boundary_switch.py 七用例两缺省位用例改新在役语义（1/9 旗 p≈0.0353 判挂 boundary）；test_maturation_gate.py 两例与 test_maturation_gate_strict.py 一例属拨闸波及（体温救援与封顶不挂是占比判据性质、strict 升格层与判据位独立），按检验层位锚定显式判据并补在役缺省形断言。全测试族 482 绿 6 红环境态（见偏差申报）。引擎侧内嵌判据查证：grep boundary_criterion 与 boundary_low 零命中，compiler.rs declared_boundary_rate≥0.5 为编译器 cell 路由位（阈值与语义均不同款），**两侧同改条件不成立，引擎零改动零重编**，如实记档。

**CONTRACT 版本行**：facet 采样合同契约无载闸位（contract_version 1 无判据字段），验收条不适用，如实记档。

## 四、段二复核纪要（gate-03）

复核脚本 recheck_114.py（纯确定性零 LLM，随批入档）：载荷哈希闸（ca9dda80cfe8bbe8 前十六位对表过）→ 在档判变审计探针全量复算（904 件）→ 冻结 868 件逐字段对表 → 每判变行独立复算哨（math.comb 精确上尾和独立实现 + 占比规则独立复算 + 方向复核）→ 逐件账本（判由 + 证据指针）。

- 判变构成：near_threshold→boundary 74 件、stable_clear→boundary 40 件，与 pk-048 泊件登记逐件吻合，**零由严到宽**；
- 独立 p 值复算 114/114 全符（1e-9 容差）；占比规则复算 114/114 全过；
- 冻结 868 件在复算语料中逐字段零漂移（金向量重放实质通过）；
- 双跑 run1/run2 逐字节一致（cmp 全 0），材料在 first-run-2026-09-05/。

## 五、段三 A/B 呈裁纪要（gate-04）与出泊证据（gate-05）

pk054-ab-gate-design.md 呈裁：A 路温度探针标定（模式一 agent 侧 + 模式二 MINIMAX 后台席）退役，B 路上下文注入（正身件与在档账本读数注入，零新鲜采样）；四判据（904 件 verdict 一致率 100%、体温逐位一致、漂移告警零、pk-035 哈希绑定不松动）、三切换条件（对跑过门、pk-044 独立性来源先裁为硬前置、实装批 T6 承接）。键位处置实情：.env MINIMAX 键不随件走，本批零触碰。实装以得一裁为门，批内零实装。

三泊件出泊条件证据包（pk-048/049/054-exit-evidence.json）在档，出泊事件批内零代落，呈人节点裁。

## 六、判变申报

| 判变 | 方向 | 证据 |
|---|---|---|
| maturation_gate.py 缺省 boundary_criterion ratio→test | 收紧（只向前生效，历史零改写） | 拨闸 diff 随批；判变预览 114 件全部由宽到紧（pk-048 复核账本）；拨闸后复算 archived_mismatch=0 |
| 测试缺省位两用例与波及三例期望更新 | 随判变语义 | 新在役语义断言与显式回退位锚定双写，红绿实录随批 |

## 七、词债与偏差申报

**词债**：叩问七词（换闸、拨闸、依据族、通道分解、复核账本、门控关闭形、上下文注入）全为轻信号未登记，本批零词表写入面，处置不登记申报，词面由任务包、A/B 呈裁件与本档行文承载。

**偏差申报**：
1. 复核脚本内计分材料路径基准归一：measure.py 计分材料以 facet/ 相对路径落盘，assemble 按仓根解析——归一为仓根相对形（facet/contracts/...），哈希绑定字段零动，装配后 R2 复算一致即证；
2. 开工预检拦截一回：idenlane-solo 批在途持五把独占锁（CALL-LOG 两笔、BATCH-FACE.md、identity、trail），排队等其收约后重开；期间本批首次 open 被拦回留下无会话僵尸窗（opened 09:54:50Z，session_id null 零锁），超 300 秒停滞阈后 lease takeover 机械自清（released_locks 空数组，零涉他批），如实留痕；
3. 全测试族 6 红：test_ng_assembler 读 worktrees/sih-tools/AGENTS.md（不入版控活文件，工地 checkout 无此件）——环境态非本批回归，主树同件 9/9 绿证实；
4. 冻结语料增长申报：pk-048 冻结 868 件，现行可复算 904 件（冻结后新入档 36 件，含本批 gateswitch-switch-2 与后继批件），新增判变 2 件（m-docmath-conflux-1 与 m-pk037-route-1，均 near→boundary 由宽到紧）；冻结 868 件为子集逐字段零漂移；
5. tally 材料路径为工地绝对形（assemble 按当时仓根解析），收约归并后重放按 sih-tools 根相对形（facet/contracts/gateswitch-260905/ 与 proposition/DES/gateswitch-switch-2/）解析，重放寻径约定随批。

## 八、CALL-LOG

- sih-tools/scribe/CALL-LOG.md：一笔（本批认证写入）
- sih-tools/lease/CALL-LOG.md：一笔（本批租约收约）
- facet 面随批 CALL-LOG.md 一笔（maturation_gate.py 拨闸）

## 九、认证清单与收约读数

（认证与收约读数随收尾链补录）

## 十、队形验证

单线形 solo 零子代理，委外代理亲写全程，T6-D 命名约定与 F 锚定与得一裁红线保留，范式零偏离。
