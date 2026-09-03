# mathpipe-a3-solo 结果档

> 批：gauge 期票清偿批（mathpipe-full 程序批三）
> 会话：fa66d4838b2d3224
> 日期：2026-09-03
> 队形：单线形 solo，零子代理
> 承接：mathpipe-full-program-v1.md 批三节与 pk-041 路线档第一档，用户令委外即冲突模式并发启动（pk-045 样本库参与者）
> 意图哈希：ea846a7478a86a4c3574324b34a8f137ae9be6f230469c5a569ef3ba67faed5d（会话台账 intent.record_sha256 对表一致，ask3 验证件 status ok 三锚，意图事件 aead347c）

## 一、问题还原

gauge 秤星契约登记两张期票即 PROB-003 置信带与 PROB-005 贝叶斯语义，读数事件 schema 待增量扩展即公式版本升 ga-2。载体已在数学仓在册即 rev1 账本可指认未实例化件，属程序批三与路线档第一档三项中的秤星控制图前置。本批将两张期票接线 gauge 计算核并升 schema，ga-1 旧读数回放兼容。

温故检索：materials/recall-a3.json 双档零命中如实记。

## 二、载体定位结论

| 件 | 落位 | 证据 |
|---|---|---|
| 载体引用 | mapping.md 概率近似与信念更新节命中 PROB-003 与 PROB-005 | sih-math/probability/entries/PROB-003-central-limit-theorem.md 与 PROB-005-bayesian-updating.md 磁盘实存 |
| 推导档 | sih-math/docs/mathpipe-a3-derivation-2026-09-03.md（内容哈希 04aa4861…） | 承载置信带公式（CLT 95% 双侧）与贝叶斯更新语义（Beta-Bernoulli 共轭先验后验均值），随批入版控 |
| 接线 | gauge 计算核 ga-2：adoption 维增 confidence_band 与 posterior_mean 两扩展字段，模型参数 α=1 β=1 z=1.96 入 inputs_digest | gauge/src/gauge/cli.py 与 CONTRACT 修订，reading.rs 守卫扩展 |
| 金向量 | materials/golden/ga2-{convergence,adoption,mergeback}.json | 三维 ga-2 输出冻结复算逐字节一致（F-2） |

消费面验收线：mapping.md 全读通过，PROB-003 与 PROB-005 锚点磁盘实存，零命中不成立未触发 M-3。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 载体推导在案** | 工程 | PROB-003 与 PROB-005 条目实存，推导档在册锚点磁盘实存 | 过 | 二节逐件在案，mapping.md 命中，两 entry 磁盘实存，推导档 sih-math/docs/ 在册随批入版控 |
| **F-2 金向量一致** | 工程 | ga-2 输出冻结复算逐字节一致，ga-1 旧读数回放同判 | 过 | golden-report 双跑逐字节一致；ga-1 回放 18/18 全过守卫，ga-2 金向量 3/3 过守卫，守卫负测四件全拒 |
| **F-3 判变零静默** | 治理 | 判变逐件申报或如实记零 | 过 | 零判变：ga-2 与 ga-1 三维基础值公式逐字节同式，ga-2 只增 adoption 维两统计摘要扩展字段，基础值不回改，判变面为零如实记 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 三仓工地 settle 提交文件全落 allow 十六路（gauge/、sih-math/docs/、plan、results、materials、trail、scribe/reports、五册 CALL-LOG、meter/counts、nomenclator/packs/core、reading.rs、SPEC-011） |

## 四、金向量读数（ga-2 三维）

- **convergence**：value 0.285714，无扩展字段，inputs_digest 3d684654。
- **adoption**：value 0.985849，confidence_band {lower 0.969949, upper 1.0}，posterior_mean 0.981308，inputs_digest 8335dc97（含 sessions-ledger 哈希与 prior:Beta(1,1) 与 z:1.96）。
- **mergeback**：value 0.043478，无扩展字段，inputs_digest 3a512cbb。
- 复算：三维金向量双跑逐字节一致零漂移；ga-1 回放 18 条旧读数全过 ga-2 守卫（18/18，failed 0）；守卫负测四件即 confidence_band 非 adoption 维、lower 大于 upper、越零一界、posterior_mean 越界全拒。

## 五、管线读数

- 化格：SPEC-011 与推导档与 CONTRACT 三 md 目标 exit 0 无需改；py/rs 变更件 general-v1 域外（只盖 md/json/yaml/toml）exit 2 如实记不属违规。
- 核阅：des-001 SPEC-011 工地路径 exit 2 域外经 corpus 域形副本重跑 exit 0 findings 0 双证；推导档与 CONTRACT 与 py/rs 变更件域外即 math docs 与 sih-tools 与 src 非 des-001 域 exit 2 如实记不属违规；ask3 双门第一门 exit 0 零违规。
- 检词：SPEC-011 与推导档与 CONTRACT 与 py/rs 变更件六目标 exit 0 零违例（工地 core 包含三新词）。
- 词债：置信带、贝叶斯语义、后验均值三词 established 登记入工地 core 包随批入版控（词表 112→115，先同步主仓已提交态再登记）。
- 测试：gauge 12 passed 3 skipped。

## 六、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者。冲突模式声明即撞锁有限重试逐次计数、认证先落主树活链、串文件 settle 前一次性拷工地、收约让位走备份对表。实测冲突点与响应逐条如下：

1. **撞锁竞态（本样本第 1 条）**：本批三仓 allow 十六路取锁，首轮 6 路径（gauge/、reading.rs、SPEC-011、plan、results、materials）于 00:09:24 无争用获锁；其余 10 共享路径（sih-math/docs/、trail、scribe/reports、五册 CALL-LOG、meter/counts、nomenclator/packs/core）与 ordwire-lease-solo 会话 986cc687835e8a42 在途锁并发重叠。ordwire 于 00:09:36–37 获锁，本批撞锁 locked_elsewhere 拒不绕行，冲突模式 7 轮有限重试累计 70 次冲突计数（10 路径 × 7 轮）；ordwire 于 00:22:19 放锁让位后，本批于 00:22:48–49 获放行全部 10 路径，重试计数 7。锁账本 acquired/released 时间戳对表在案。
2. **认证先落主树活链**：本批意图笔与认证笔共五事件（aead347c 加 5d0d9218 加 8e181f68 加 38274002 加 878c3466）最终落主树 2026-09-03.ndjson 活链，链尾 878c3466。执行机制如实记：链文件 settle 前一次性拷入引擎工地，四笔认证事件追加于工地 trail 副本（report_path 指向 worktrees/...），意图笔记录路径指向主树；收约归并时工地副本与主树链同位分叉，经第 5 条手动合链解决，非零冲突（承 pk-045 链分叉教训，本批为链分叉样本续例）。
3. **串文件收约让位对表**：收约时主树同名未跟踪件阻挡归并走备份让位法，diff 备份与归并结果须 identical，非 identical 即停批上报。
4. **环境位移**：受 PYTHONHOME 污染风险，工具调用按 BATCH-FACE 坑位加 `env -u PYTHONHOME -u PYTHONPATH` 前缀规避 encodings 缺失。
5. **链分叉合并冲突（本样本第 2 条）**：收约归并引擎仓时，工地 trail 副本（msh/mathpipe-a3-solo 分支）在 0c911046 之后追加本批五事件，主树链同位无追加，git 报 2026-09-03.ndjson 冲突。机械响应：不取任一侧，逐事件核 prev_hash 连续性（99d390e0→aead347c→5d0d9218→8e181f68→38274002→878c3466 全连续）后保留分支侧五事件、清除三处冲突标记，`scribe verify` 82 事件 valid 链尾 878c3466。解决路径：手动合链，非重放非截断，链完整性由 verify 对表。

## 七、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| aead347c | 意图笔（scribe intent） | ask3 记录 ea846a74 |
| 5d0d9218 | 管线报告（2026-09-03-mathpipe-a3-pipeline.json） | — |
| 8e181f68 | 金向量报告（golden-report.json） | — |
| 38274002 | 推导档报告（2026-09-03-mathpipe-a3-derivation.json） | 04aa4861… |
| 878c3466 | 变更件报告（2026-09-03-mathpipe-a3-changed-files.json） | 415e9c00… / f2d1f234… / 8f871ce0… / 5239190d… / aaaac326… |

认证一律先落主树活链，链 verify 以批期链尾变更件认证笔 878c3466 为收（close 后读数为准，见对表节）。

## 八、验收

- [x] F-1 至 F-4 全过
- [x] ga-2 在案且 ga-1 回放兼容（18/18 全过守卫）
- [x] 冲突样本节在结果档（第 1 条撞锁竞态含 7 轮 70 次冲突计数与让位时间戳，第 5 条链分叉合并冲突含合链路径与 verify 对表）
- [x] 认证入链，多仓结算收约，对表读数在档（close 后 reconcile 读数见收约附表）
