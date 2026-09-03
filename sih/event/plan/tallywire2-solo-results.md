# tallywire2-solo 结果档

> 批：tally 规则族载体接线批（mathpipe-full 程序批四起收官件一，批四起六件至此收官）
> 会话：be02815afcd3149f
> 日期：2026-09-04
> 队形：单线形 solo，零子代理
> 承接：mathpipe-full-program-v1.md 批四节即 tally 规则族对挂闭包算子与良基；四件套形制全承 ordwire 与 caswire2 先例
> 意图哈希：80f45479700b7688e283778c507bf4838815bd5884622153e80412fe6cb02f10（ask3 记录内容哈希，会话台账对表一致，ask3 验证件 status ok 三锚，意图事件 7415d87e）

## 一、问题还原

rev1 账本坐实 tally 为可指认未实例化件：tally 即 R1-R7 核对与三态映射四值处置，判定面即规则族对材料的闭包式核对（每条核对是后果算子一步）与归约终止性（良基），原为无载体引用无推导档的裸奔态。本批双载体分面接线：ORD-006 承核对规则族的闭包语义，ORD-011 承四值处置归约的良基终止。

温故检索：materials/recall-tallywire.json 双档零命中如实记（envelope count 0）。

## 二、载体四件套（双载体分面）

| 件 | 落位 | 证据 |
|---|---|---|
| 载体引用 | mapping.md:182「闭包算子是否保序外延 ORD-006 闭包算子与后果算子 已建」与 mapping.md:78「递归是否必然终止 ORD-011 良基归纳与递归终止 已建」 | 两 entry 磁盘实存：sih-math/order/entries/ORD-006-closure-and-consequence-operator.md 与 ORD-011-well-founded-recursion-termination.md |
| 推导档 | sih-math/docs/tallywire-tally-derivation-2026-09-04.md | 承载 R1-R7 即后果算子逐步应用（核对集对材料集的闭包，p_i ∈ Cn(P_i ∪ A)）与四值处置归约良基性（有限支集测度严格下降无无穷降链），随批入版控 |
| 接线 | tally CONTRACT 增 ## 载体引用 {#carrier} 节（双载体分面）+ 源码三锚注释 | cli.py 规则装载位（常数块即锚集预给定固定）、逐条核对位（check_material 即 p_i ∈ Cn(P_i ∪ A) 成员判定）、归约终止位（三态映射块即良基归约） |
| 金向量 | sih-engine/sih/event/plan/tallywire2-solo-materials/tallywire2-solo-golden-vector.json | 两场景双跑逐字节一致（F-3），replay_golden.py exit 0 |

消费面验收线：mapping.md 双锚行命中，双 entry 磁盘实存，零命中不成立（recall 零命中属温故检索申报非载体缺席）。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 载体四件套** | 工程 | 双载体引用与推导与接线与金向量齐，双条目磁盘实存 | 过 | 二节四件逐件在案，mapping.md:182 与 :78 双行命中，双 entry 磁盘实存 |
| **F-2 零行为变更** | 工程 | tally 既有测试全绿，wiring_only_no_behavior_change | 过 | 工地 pytest 20 测全绿（0.45s），本批只加注释与契约节零改判定谓词，changed-files 报告 change_type=wiring_only_no_behavior_change |
| **F-3 金向量** | 工程 | 两场景双跑逐字节一致 | 过 | pass_all_closure 与 suspend_r5_drift 双跑逐字节一致零漂移，replay_golden.py exit 0，与冻结金向量 IDENTICAL |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 三仓工地 git status 对表未越 allow 冻结面（tools 四改四增、engine 四增一改、math 一增，全在册） |

## 四、金向量读数（两场景）

- 场景一 **pass_all_closure**（全过材料核对闭合）：R1-R7 七条核对全入闭包，处置 = 裁决通过，方向 comply（席位众数），exit 0——闭包闭合语义机械重放。
- 场景二 **suspend_r5_drift**（拒材料在某 R 条目挂起定位）：R5 席位当日基线漂移告警，p_5 出闭包按 DES-011 优先级映射落挂起，定位行即「R5: 席位当日基线判定 漂移告警（处置走优先级映射）」，exit 0（核对零失败项、处置挂起为映射语义非核对失败）——挂起定位语义机械重放。
- 复算：replay_golden.py 双跑逐字节一致，stdout 归一化后与冻结金向量逐字节一致，退出码一致，double_run_all_identical=true。

## 五、管线读数（笔在核前、判在书简前）

- 化格：CONTRACT.md 与推导档与 terms.json exit 0 无需改；cli.py general-v1 域外（只盖 md/json/yaml/toml）exit 2 如实记不属违规。
- 核阅：des-001 域只盖 sih-engine/doc，四件均域外 exit 2 如实记不属违规；ask3 双门第一门 exit 0 零违规。
- 检词：四件 exit 0 零违例（工地 core 包含闭包算子与良基终止两新词）。
- 词债：载体接线（ordwire 已立）本批复用零信号；闭包算子、良基终止两词 established 登记入工地 core 包随批入版控，叩问 digest passed covered 2。

## 六、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者，与 selwire-solo（施工面 sih-tools/selector/）同时在跑。实测冲突点与响应逐条如下：

1. **共享活链并发追加（本样本第 1 条）**：本批意图笔 16:29:38（7415d87e）与认证四笔（16:41 前后）落主树 2026-09-04.ndjson 活链期间，selwire-solo 意图笔 16:31:12（f061c9b0）插入本批意图与认证之间——共享追加面 --mode append 短持即取即得，零撞锁零重试，链 verify valid 为唯一放行形（9 事件哈希连续）。
2. **exclusive 施工面零撞锁如实记**：本批五施工面锁（tally、packs/core、results、materials、sih-math/docs）一射取获重试计数 0；selwire 施工面 selector/ 与本批 tally/ 不同面，零重叠。
3. **闸三在役实录**：scribe intent 首试缺 --sessions 会话台账路径参数被 gate 拒（exit 2，报错文「会话在册验需 --sessions <会话台账路径>」，链零写入留痕核实三笔不动），补参后过——参数面误差非绕行，见越线与误差申报节。
4. **认证先落主树活链**：认证事件先落主树活链，链文件 settle 前一次性拷入引擎工地（9 事件整链拷贝），无工地链副本追加（承 pk-045 教训）。
5. **收约让位**：本批收约时零主树同名未跟踪件阻挡，备份让位法未触发；若 close 撞 selwire 先并 main 致 trail 冲突，取事件数并集超集且 scribe verify 过为唯一放行形（收约附表回填实测）。

## 七、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 7415d87e | 意图笔（scribe intent，meter 包裹） | ask3 记录 80f45479 |
| c0880b21 | 管线报告（2026-09-04-tallywire2-solo-pipeline.json） | 报告件内容哈希 |
| 3f44e1d9 | 推导档报告（2026-09-04-tallywire2-solo-derivation.json） | 报告件内容哈希 |
| ae63ea70 | 金向量报告（2026-09-04-tallywire2-solo-golden.json） | 报告件内容哈希 |
| a5450400 | 变更件报告（2026-09-04-tallywire2-solo-changed-files.json） | 报告件内容哈希 |

认证一律先落主树活链，四笔全 meter 包裹、闸三 --sessions 带、追加态 append 短持锁即取即放。

## 八、越线与误差申报

1. 闸三首试缺 --sessions 参数被拒 exit 2 一笔：链零写入（事前三笔事后三笔核实），补参即过，属参数面误差非门绕行；BATCH-FACE scribe intent 形缺 --sessions 旗标注记，已入 scribe CALL-LOG 坑位实录，建议漂移登记。
2. 零停批事件：全程无不可解释的门与闸拒绝，无工具 exit 2 异常（formatter 对 py 件与 scrutinator 对四件的 exit 2 均为域外如实记非工具异常）。
3. 政策行在役：机械链全绿即自行收口推进，结果档不设「等你令」节。

## 九、验收

- [x] F-1 至 F-4 全过
- [x] 冲突样本节含追加态实录（第 1 条 selwire 插队读数，第 2 条零撞锁如实记零）
- [x] 认证入链，多仓结算收约对表在档（close 后 reconcile 读数见收约附表）
- [x] 任务包与 dispatch 两件随批入版控（主会 d034799 批前收编在案）

## 十、队形验证

单线形 solo 零子代理，全链由会话 be02815afcd3149f 亲写，零 Agent/Task 派生。

## 十一、收约附表（close 后回填）

- **close 两段式**：试一 math 归并移除成功（911adff），tools 与 engine merge_failed——tools 撞主树同名未跟踪件（四报告件）、engine 撞主树活链脏态（链相对 HEAD 三笔基线后追加未提交）；按备份让位归并对表法：备份主树四报告与活链、让位删除、close 试二归并成功 revoked true（tools 与 engine 归并移除，math already_gone）。
- **三仓 commit 号**：settle 即 math 04a21d5、tools ce426c4d、engine 53301af（--cert a5450400，base 即 integral-stage-build@eeda306c / main@d034799 / main@ddb6f7d）；归并即 math 911adff、tools a53d7164、engine 39b21b0。
- **备份让位对表**：备份与归并结果五件 cmp 逐字节 IDENTICAL（四报告件 + 活链 9 事件），非 identical 即停批条件未触发。
- **reconcile 读数（close 后）**：tools unrouted_tail 46（cert_missing 1 即 entryunique-solo 526e2be 09-03 既有）、engine unrouted_tail 70（cert_missing 0、bypass 11 全登记）、math unrouted_tail 47（cert_missing 2 即 08-30 mathfix2 与 fmtfix 既有、unbypassed 1 即 08-30 基线 c556abb 既有）——三仓 unrouted 与 cert_missing 较批前零新增，本批提交零入尾。
- **链 verify 前后对表**：批期 verify valid 9 事件（首哈希 05a8a75e、末哈希 a5450400），close 后 verify valid 9 事件同首末哈希，前后一致零漂移；与 selwire-solo 并发追加态下活链哈希连续。
- **本笔回填**：本节由收约后 wip 提交回填，--no-verify 提交 + lease bypass 登记随行（close 通道以外提交的既定通道）。
