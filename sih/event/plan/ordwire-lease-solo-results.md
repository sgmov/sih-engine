# ordwire-lease-solo 结果档

> 批：lease 锁机制载体接线批（mathpipe-full 程序批四起首件）
> 会话：986cc687835e8a42
> 日期：2026-09-03
> 队形：单线形 solo，零子代理
> 承接：mathpipe-full-program-v1.md 批四节对挂表与 ledger-rev1 可指认未实例化件；用户令委外即冲突模式并发启动（pk-045 样本库参与者）
> 意图哈希：d6eb72057553a363afc45e09fdc59829e2121d6a23c85bed679af35ae3a892dfa（会话台账 intent.record_sha256 对表一致，ask3 验证件 status ok 三锚，意图事件 5fe990c5）

## 一、问题还原

rev1 账本坐实 lease 为可指认未实例化件：程序档对挂 ORD-020 全序资源分配与死锁自由在数学仓在册而 lease 源码零引用。按 M-1 判定性常数与判据须携带载体引用与推导档，锁机制的判定面即撞锁拒绝与等待重试语义当前裸奔。本批将序判定语义与 ORD-020 载体接线。

温故检索：materials/recall-ordwire.json 双档零命中如实记。

## 二、载体四件套

| 件 | 落位 | 证据 |
|---|---|---|
| 载体引用 | mapping.md:206「多文件锁的死锁防护 ORD-020 全序资源分配与死锁自由 已建」 | sih-math/order/entries/ORD-020-total-order-resource-allocation-deadlock-freedom.md 磁盘实存 |
| 推导档 | sih-math/docs/ordwire-lease-derivation-2026-09-03.md | 承载锁判定语义形式化 §3.1 互斥 §3.2 死锁不自由 §3.3 等待终止性，随批入版控 |
| 接线 | lease CONTRACT 增载体引用节（## 载体引用 {#carrier}）+ 源码判定位注释锚点 | lockcore.py active_locks(§3.1) acquire(§3.2) release(§3.3) normalize_path(§3.2) |
| 金向量 | sih-engine/sih/event/plan/ordwire-lease-solo-materials/ordwire-lease-solo-golden-vector.json | 撞锁与让路两场景机械重放逐字节一致（F-3） |

消费面验收线：mapping.md 全读通过，ORD-020 锚点磁盘实存，零命中不成立未触发 M-3。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 载体四件套** | 工程 | 载体引用与推导档与接线与金向量四件齐，ORD-020 锚点磁盘实存 | 过 | 二节四件逐件在案，mapping.md:206 命中，entry 磁盘实存 |
| **F-2 行为零变更** | 工程 | lease 既有测试全绿，判定行为 diff 为零或偏差即停批申报 | 过 | lease 62 测全绿，changed-files 报告 change_type=wiring_only_no_behavior_change，本批只加引用注释与契约节不改判定谓词 |
| **F-3 金向量重放** | 工程 | 撞锁与让路两场景机械重放逐字节一致 | 过 | replay_golden.py 双跑逐字节 IDENTICAL，金向量事件认证 ea53cd8a 承 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 三仓工地 git status 对表未越 allow 冻结面 |

## 四、金向量读数（撞锁与让路）

- 场景一 **collision_locked_elsewhere**：会话 A 锁 CONTRACT（granted），会话 B 撞锁所拒 locked_elsewhere，A 仍持——互斥语义机械重放。
- 场景二 **yield_release_then_grant**：A 锁（granted）→ A 放锁 released（让路）→ B 锁（granted）——等待终止性让步良基机械重放。
- 复算：replay_golden.py golden_cases.json 双跑逐字节一致零漂移，与冻结金向量 IDENTICAL。

## 五、管线读数

- 化格：推导档与 CONTRACT md 目标 exit 0 无需改；py 变更件 general-v1 域外（只盖 md/json/yaml/toml）exit 2 如实记不属违规。
- 核阅：des-001 推导档与 CONTRACT 与 py 变更件均域外（非 sih-engine/doc 域）exit 2 如实记不属违规；ask3 双门第一门 exit 0 零违规。
- 检词：推导档与 CONTRACT 与 py 变更件 exit 0 零违例（工地 core 包含五新词）。
- 词债：撞锁、让路、载体接线、金向量、全序资源分配五词 established 登记入工地 core 包随批入版控。

## 六、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者。冲突模式声明即撞锁有限重试计数、认证先落主树活链、串文件 settle 前一次性拷工地、收约让位走备份对表。实测冲突点与响应逐条如下：

1. **撞锁竞态（本样本第 1 条）**：本批三仓 allow 十五路取锁，9 路径与 resarch-solo 会话 37e3bbc2ffd0f6c7 在途锁并发重叠（trail 链文件、双仓五册 CALL-LOG、meter/counts、nomenclator 包、scribe/reports 九路）。第 1 次取锁 locked_elsewhere 拒不绕行，有限重试；resarch-solo 于 00:09:02–03 放锁让位后，本批第 2 次于 00:09:36–37 获放行全部九路，重试计数 1。锁账本 acquired/released 时间戳对表在案。
2. **认证先落主树活链**：本批认证事件先落主树 2026-09-03.ndjson 活链，链文件 settle 前一次性拷入引擎工地，无工地链副本追加（承 pk-045 教训）。
3. **串文件收约让位对表**：收约时主树同名未跟踪件阻挡归并走备份让位法，diff 备份与归并结果须 identical，非 identical 即停批上报。
4. **环境位移**：受 PYTHONHOME 污染风险，工具调用按 BATCH-FACE 坑位加 `env -u PYTHONHOME -u PYTHONPATH` 前缀规避 encodings 缺失。

## 七、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 5fe990c5 | 意图笔（scribe intent） | ask3 记录 d6eb7205 |
| 4756a52e 加 28da55fa（重复挂两笔） | 管线报告（2026-09-03-ordwire-lease-solo-pipeline.json） | 8f62a6fd |
| 0680c3f0 | 推导档报告（2026-09-03-ordwire-lease-solo-derivation.json） | da88e38a |
| ea53cd8a | 金向量报告（2026-09-03-ordwire-lease-solo-golden.json） | d4d23b4a |
| 99d390e0 | 变更件报告（2026-09-03-ordwire-lease-solo-changed-files.json） | 3c1a5ac2 |

认证一律先落主树活链，链 verify 以批期链尾变更件认证笔为收（close 后读数为准，见对表节）。

## 八、验收

- [x] F-1 至 F-4 全过
- [x] 冲突样本节在结果档（第 1 条撞锁竞态含 retry 计数与让位时间戳）
- [x] 认证入链，多仓结算收约，对表读数在档（close 后 reconcile 读数见收约附表）+5. **收约碰撞两段式（本样本第 5 条）**：close 于 engine 与 tools 两仓 merge_failed——并行 resarch-solo 先并入 main 致 5 册调用册、terms.json、引擎 trail 共享只追加文件左右各追加 git 拒并（math 仓归并 cef0f16 成功）。机械响应即两仓工地并集手工归并：引擎 trail 77 事件 scribegate verify valid（main 71 事件为其前缀、ordwire 6 事件纯追加尾）、工具 5 册调用册两批行并存、terms.json 并 8 词（修正 ordinate-lease-solo 笔误）；engine 归并 cc5908c 与 tools 归并 dfe569f0 均成 main 后裔可快进。终态归并采"收约让位"延至并发批（leasepatch/algidx/autoflow）让位后再行，本批不越界强并主树脏树活写，重试计数或让位时刻见收约附表与工具调用册更正行。
