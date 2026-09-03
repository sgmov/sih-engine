# idwire-solo 结果档

> 批：identity 正身载体接线批（mathpipe-full 程序批四起）
> 会话：77cee4a30db0d163
> 日期：2026-09-03
> 队形：单线形 solo，零子代理
> 承接：mathpipe-full-program-v1.md 批四节即 identity 对挂 ALG-002 商集隔离加 PROB-013 变点检测；批一账本 rev1 可指认未实例化件；四件套形制全承 ordwire-lease-solo 先例
> 意图哈希：797a01587abb36937208419732d916b38f284e1e2f8e1dc1e2c4a7fb45703e16（会话台账 intent.record_sha256 对表一致，ask3 验证件 status ok 三锚，意图事件 10c92979）

## 一、问题还原

identity 即身份串 v3 十二件组件加盐 SHA-256，判定面即身份等价判定（canonical 代表）与漂移监控（跨时点身份串比对），当前裸奔无载体引用。按 M-1 判定性常数与判据须携带载体引用与推导档。本批将等价判定面对接 ALG-002（商集隔离/换序同串），漂移监控面对接 PROB-013（一变点检测/平稳基线）。

温故检索：materials/recall-idwire.json 双档零命中如实记。

## 二、载体四件套（双载体分面）

| 件 | 落位 | 证据 |
|---|---|---|
| 载体引用 | mapping.md:197「选择性隔离与防污 → ALG-002」、mapping.md:203「未形之变的微兆检测 → PROB-013」 | 双 entry 磁盘实存：algebra/entries/ALG-002 与 probability/entries/PROB-013 |
| 推导档 | sih-math/docs/idwire-identity-derivation-2026-09-03.md | §3.1 等价类 canonical 代表与换序不变性（ALG-002）；§3.2 平稳基线变点检测（PROB-013）；随批入版控 |
| 接线 | identity CONTRACT 增载体引用节 + 源码判定位注释锚点 | core.py：COMPONENT_ORDER(组件序归一位)/identity_string(等价类代表)/identity_hash(哈希合成位)/compare_claims(漂移比对位) |
| 金向量 | sih-engine/sih/event/plan/idwire-solo-materials/idwire-identity-solo-golden-vector.json | 换序同串与变点定位两场景机械重放逐字节一致（F-3） |

消费面验收线：mapping.md 双锚点磁盘实存，identity 判定常数接线无零命中。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 载体四件套** | 工程 | 双载体引用与推导档与接线与金向量四件齐，ALG-002 与 PROB-013 锚点磁盘实存 | 过 | 二节四件逐件在案，mapping.md:197 与 :203 命中，双 entry 磁盘实存 |
| **F-2 行为零变更** | 工程 | identity 既有测试全绿，判定行为零改动 | 过 | identity 49 测全绿，changed-files 报告 change_type=wiring_only_no_behavior_change，只加注释与契约节 |
| **F-3 金向量重放** | 工程 | 换序同串与变点定位两场景机械重放逐字节一致 | 过 | replay_golden.py 双跑逐字节 IDENTICAL，且在盘冻结向量与新鲜确定性输出一致，金向量认证 e521522e |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 三仓工地 git status 对表未越 allow 冻结面 |

## 四、金向量读数（换序同串与变点定位）

- 场景一 **shuffle_invariance_same_string**：同一组件集两种录入顺序 identity_string 输出字节级一致、同盐同哈希——等价类代表稳定，商集隔离机械重放（ALG-002）。
- 场景二 **drift_change_point_located**：hostname 异值致身份串异串，compare_claims 逐键唯一 mismatch 落 hostname 位——变点定位机械重放（PROB-013）。
- 复算：replay_golden.py golden_cases.json 双跑逐字节一致零漂移，与冻结金向量 IDENTICAL。

## 五、管线读数

- 化格：推导档 exit 1 已修改归一 1 行；CONTRACT exit 0 无需改；py 变更件 general-v1 域外（只盖 md/json/yaml/toml）如实记不属违规。
- 核阅：des-001 推导档与 CONTRACT 与 py 变更件均域外（非 sih-engine/doc 域）exit 2 如实记不属违规。
- 检词：推导档与 CONTRACT exit 0 零 findings；py 变更件域外。
- 词债：载体接线、商集隔离、变点检测 established 登记入 core 包随批入版控。

## 六、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者。冲突点与响应逐条：

1. **共享锁多批双持（本样本新增实测）**：mathclose/caswire/scriwire 多并行批与 idwire 在同一共享锁集（trail、sih-math/docs/、双仓调用册、meter/counts、nomenclator 包、scribe/reports）撞锁。scriwire-solo 于 12:33 重发以**绝对路径**拼写整组重锁，按在册判定后来者居上，idwire 相对路径取锁被超越，scribe 以「链路径在他会话锁下」拒 idwire 写 trail。响应：不绕行强写，改绝对路径拼写续取时证 idwire 会话 allow 面为相对拼写致 scope_violation，遂退回相对拼写。trail 锁经确认后被 mathclose 让出（12:32:59），scriwire 续放全部共享锁（12:58:17 逐路径 released），idwire 于 13:0x 前台账定补齐 11/11 锁（含 docs 补取）后重试 intent 上链成功。锁账本 acquired/released 时间戳对表在案。
2. **认证先落主树活链（插队）**：idwire intent（10c92979）与四报告认证（86d86d9d/49ed5d98/e521522e/0ea65d90）先落主树 2026-09-03.ndjson 活链，链文件 settle 前一次性拷工地，无工地链副本追加（承 pk-045 教训）。
3. **trail 双拼写归一**：相对与绝对路径拼写在册判定归一为同一物理文件，scriwire 绝对持锁在 idwire 相对取锁后按后来者居上覆盖，暴露双拼写同指一文件致锁册竞争者判定；处置为以与 allow 面一致的相对拼写持有，待他批让锁后立续。
4. **环境位移**：受 PYTHONHOME 污染风险，工具调用按 BATCH-FACE 坑位加 `env -u PYTHONHOME -u PYTHONPATH` 前缀规避 encodings 缺失。

## 七、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 10c92979 | 意图笔（scribe intent） | ask3 记录 797a0158 |
| 86d86d9d | 管线报告（2026-09-03-idwire-solo-pipeline.json） | 双载体分面 + 四锚点 |
| 49ed5d98 | 推导档报告（2026-09-03-idwire-solo-derivation.json） | derivation 件承 |
| e521522e | 金向量报告（2026-09-03-idwire-solo-golden.json） | golden 向量承 |
| 0ea65d90 | 变更件报告（2026-09-03-idwire-solo-changed-files.json） | change_type=wiring_only_no_behavior_change |

认证一律先落主树活链，链 verify 以批期链尾变更件认证笔为收（close 后读数为准，见对表节）。

## 八、验收

- [x] F-1 至 F-4 全过
- [x] 冲突样本节在结果档（本样本第 1 条共享锁多批双持含 scriwire 绝对拼写超越与 mathclose/scriwire 让锁时间戳）
- [x] 认证入链，多仓结算收约，对表读数在档（close 后 reconcile 读数见收约附表）