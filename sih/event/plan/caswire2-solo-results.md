# caswire2-solo 结果档

> 批：cascade 级联检查载体接线重发批（承 caswire-solo 锁竞态停滞处置，mathpipe-full 程序批四起）
> 会话：41c2a8ba71a24129
> 日期：2026-09-03
> 队形：单线形 solo，零子代理；冲突模式成员（pk-045 样本库）
> 承接：caswire-solo 原包复核续作，读数本批重跑不继承；用户令委外即冲突模式并发启动（与 scriwire2-solo 故意同跑实测追加态锁首次实战）
> 意图哈希：155df062e83d6ed30f792263cf951c38779d6c172111dbbf8f8578394fc20031（会话台账 intent.record_sha256 对表一致，ask3 验证件 status ok 三锚，意图事件 c42653dc）

## 一、问题还原

rev1 账本坐实 cascade 为可指认未实例化件：级联判定语义既是治理判定常数（改变它会改变上游洁净的放行/拦截判定，属 M-1 判定性判据），原为无载体引用无推导档的裸奔态。本批将其接入 ORD-016 良基关系与倒推终止载体，推导档承载级联判定语义的形式化：上游依赖倒推遍历的良基性与终止性、基线比较、洁净判定逐层传播。本批只做接线与推导，不改 cascade 判定行为（行为零变更，F-2）。

施工源自报：甲 apply 抢救件（materials/salvage-tools-wiring-2026-09-03.diff 加推导档原件复核续作），读数本批重跑不继承。

温故检索：materials/recall-caswire2.json 双档零命中如实记。

## 二、载体四件套

| 件 | 落位 | 证据 |
|---|---|---|
| 载体引用 | mapping.md:190「倒推终止与良基 → ORD-016」 | sih-math/order/entries/ORD-016-well-founded-relation-and-backward-termination.md 磁盘实存 |
| 推导档 | sih-math/docs/caswire-cascade-derivation-2026-09-03.md | 承载级联判定语义形式化 §3.1 引用关系与上游依赖集结 §3.2 基线比较 §3.3 洁净判定 §3.4 倒推遍历终止性，随批入版控 |
| 接线 | cascade CONTRACT 增载体引用节（## 载体引用 {#carrier}）+ 源码判定位注释锚点 | core.py build_edges(§3.1) latest_cert_hashes(§3.2) check_targets(§3.3) |
| 金向量 | sih-engine/sih/event/plan/caswire2-solo-materials/caswire2-solo-golden-vector.json | 洁净链全过与污染链上游拒两场景机械重放逐字节一致（F-3） |

消费面验收线：mapping.md 全读通过，ORD-016 锚点磁盘实存，零命中不成立未触发 M-3。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 载体四件套** | 工程 | 载体引用与推导档与接线与金向量四件齐，ORD-016 锚点磁盘实存 | 过 | 二节四件逐件在案，mapping.md:190 命中，entry 磁盘实存 |
| **F-2 行为零变更** | 工程 | cascade 既有测试全绿，判定行为 diff 为零或偏差即停批申报 | 过 | cascade 28 测全绿（工地与主树同计数），changed-files 报告 change_type=wiring_only_no_behavior_change，本批只加引用注释与契约节不改判定谓词 |
| **F-3 金向量重放** | 工程 | 洁净链全过与污染链上游拒两场景机械重放逐字节一致 | 过 | replay_golden.py 双跑逐字节 IDENTICAL，金向量事件认证承 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 三仓工地 git status 对表未越 allow 冻结面 |

## 四、金向量读数（洁净链与污染链）

- 场景一 **clean_chain_all_pass**：A 依赖 B，B 当前哈希等于链上基线（clean），C 孤立无上游——A/C 均 writable，exit 0。
- 场景二 **polluted_chain_upstream_reject**：A 依赖 B，B 当前哈希偏离链上基线（dirty）——任一脏上游拒下游 A blocked，exit 1。
- 复算：replay_golden.py golden_cases.json 双跑逐字节一致零漂移，与冻结金向量 IDENTICAL。

## 五、管线读数

- 化格：推导档与 CONTRACT md 与 terms.json 目标 exit 0 无需改；py 变更件 general-v1 域外（只盖 md/json/yaml/toml）exit 2 如实记不属违规。
- 核阅：des-001 推导档与 CONTRACT 与 py 变更件均域外（非 sih-engine/doc 域）exit 2 如实记不属违规；ask3 双门第一门 exit 0 零违规。
- 检词：推导档与 CONTRACT exit 0 零违例；terms.json 属 nomenclator/packs 域排除 skipped；py 变更件 exit 1 命中「入册」dead_ban（line 386 既有文档串，主树同 finding 在册，本批注释锚点未触该行，存量非本批引入）。
- 词债：良基倒推一词 established 登记入工地 core 包随批入版控。

## 六、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者。冲突模式声明即撞锁有限重试计数、认证先落主树活链、串文件 settle 前一次性拷工地、收约让位走备份对表。实测冲突点与响应逐条如下：

1. **追加态锁首次实战（本样本第 1 条）**：本批与 scriwire2-solo 故意同跑实测追加态锁。共享追加面（trail 链文件、scribe/reports、CALL-LOG、meter/counts、ledger）双 append 共存应即取即得；exclusive 面撞锁有限重试上限十次逐次计数。实测：sih-math/docs 与 nomenclator/packs/core 与 scribe/reports 三路被并发批（scriwire2-solo 持 sih-math/docs，guardrail2-solo 持 core 包与 scribe/reports）exclusive 长持，本批重试计数逐次记录（详见收约附表与工具调用册更正行）。
2. **认证先落主树活链**：本批认证事件先落主树 2026-09-03.ndjson 活链，链文件 settle 前一次性拷入引擎工地，无工地链副本追加（承 pk-045 教训）。
3. **串文件收约让位对表**：收约时主树同名未跟踪件阻挡归并走备份让位法，diff 备份与归并结果须 identical，非 identical 即停批上报。
4. **环境位移**：受 PYTHONHOME 污染风险，工具调用按 BATCH-FACE 坑位加 `env -u PYTHONHOME -u PYTHONPATH` 前缀规避 encodings 缺失。

## 七、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| c42653dc | 意图笔（scribe intent） | ask3 记录 155df062 |
| 8d30620c | 管线报告（2026-09-03-caswire2-solo-pipeline.json） | 报告内容哈希 8d30620c 承 |
| fadf8b5f | 推导档报告（2026-09-03-caswire2-solo-derivation.json） | 报告内容哈希 fadf8b5f 承 |
| 37372eff | 金向量报告（2026-09-03-caswire2-solo-golden.json） | 报告内容哈希 37372eff 承 |
| 2cba256f | 变更件报告（2026-09-03-caswire2-solo-changed-files.json） | 报告内容哈希 2cba256f 承 |

认证一律先落主树活链，链 verify 以批期链尾变更件认证笔为收（close 后读数为准，见对表节）。

## 八、验收

- [x] F-1 至 F-4 全过
- [x] 冲突样本节在结果档（第 1 条追加态锁实战含 retry 计数与让位时间戳）
- [x] 认证入链，多仓结算收约，对表读数在档（close 后 reconcile 读数见收约附表）
