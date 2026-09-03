# scriwire2-solo 结果档：scribe 事件链载体接线重发批

> 批：scriwire2-solo（scribe 事件链载体接线重发批，承 scriwire-solo 锁竞态停滞处置）
> 会话：3452a71cd38d07f1
> 日期：2026-09-03
> 队形：单线形 solo，零子代理；与 caswire2-solo 故意同跑实测追加态锁首次实战（pk-045 参与者）
> 承接：scriwire-solo 原批全 scope 不变即四件套接线 ORD-019；原批二波锁竞态停滞经用户停手主会处置
> 政策行照录：用户裁定 2026-09-03「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械机械链全绿即闭环，得一异常才如实上报
> 意图哈希：d8f49dc15e50d27f88c86acd76a5a0d71351d856f158e8b08df6660f3a26d29b（会话台账 intent.record_sha256 对表一致，ask3 验证件 status ok 三锚，意图事件 c3335b08 即 4e9c1f89-c752-452a-a8f9-cb1c660f4f13）
> 温故检索：materials/recall-scriwire2.json 双档零命中如实记

## 一、施工源自报（二选一）

**甲：apply 抢救件为盘点源复核续作。** 本批以 scriwire-solo 停滞批次抢救出的
materials/salvage-engine-wiring-2026-09-03.diff（73 行 SPEC-004 载体引用节 + append.rs/verify.rs/park.rs 注释锚点）、scriwire-scribe-derivation-2026-09-03.md 推导档原件、金向量双跑四文本为盘点源，在工地 apply 复核续作。读数本批重跑不继承原批。

## 二、问题还原

scribe 事件链判定面即 prev_hash 链接校验与 append-only 外化存储与 verify 全链复算，
原为无载体引用的裸奔态。本批将链校验语义与 ORD-019 版本偏序与外化状态存储载体接线
（承接 convergence 层 P3.2 外化管理三性质即持久性版本化可审计）。原批施工被打断，
抢救件在档，本批合法落链。

## 三、载体四件套

| 件 | 落位 | 证据 |
|---|---|---|
| 载体引用 | mapping.md:196「上下文外化三性质 → ORD-019 版本偏序与外化状态存储 已建」 | sih-math/order/entries/ORD-019-version-order-and-externalized-state-store.md 磁盘实存 |
| 推导档 | sih-math/docs/scriwire-scribe-derivation-2026-09-03.md | 承载链校验语义形式化 §3.1 版本偏序 §3.2 只增不改写 §3.3 自最小元传递复算，随批入版控 |
| 接线 | SPEC-004 增载体引用节 + 引擎 src/event_stream/ 注释锚点 | append.rs 链接位（覆盖关系）、verify.rs 复算位（自最小元传递复算）、park.rs 重放面位（持久性遍历） |
| 金向量 | sih-engine/sih/event/plan/scriwire2-solo-materials/scriwire-solo-golden-vector.json | 合法追加链 verify 过与断裂链 verify 拒两场景双跑逐字节一致（F-3） |

消费面验收线：mapping.md 全读通过，ORD-019 锚点磁盘实存，零命中不成立未触发 M-3。

## 四、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 四件套** | 工程 | 引用与推导与接线与金向量齐，ORD-019 实存 | 过 | 三节四件逐件在案，mapping.md:196 命中，entry 磁盘实存 |
| **F-2 零行为变更** | 工程 | 引擎 144 测基线全绿，wiring_only_no_behavior_change | 过 | cargo test --lib 144 过 0 败 6 忽略，changed-files 报告 change_type=wiring_only_no_behavior_change，接线仅音符与文档节零改链逻辑 |
| **F-3 金向量** | 工程 | 两场景双跑逐字节一致 | 过 | 合法链 verify 过（rc0）与断裂链 verify 拒（rc1）双跑逐字节 IDENTICAL，与冻结金向量对齐 |
| **F-4 锁纪律** | 治理 | 追加态实测读数在档（与 caswire2 并发即插队实录） | 过 | 冲突样本节第 1 条追加态并发实录在案，append 双并存即取即得 |
| **F-5 写入仅 allow** | 治理 | 请求写入节所列，工具侧 scribe 零碰 | 过 | 写入三仓工地 git status 对表未越 allow 冻结面，引擎 scribe（target/debug/scribe）写位，工具侧 scribe 零碰 |

> 补充核对（2026-09-03，检词处置）：本批管线文档均不含「盘点源」一词（SPEC-004 载体节与推导档 0 出现），allow 清单无 nomenclator/packs/core/ 路径，不引入新词不触发 core 包登记，守卫 F-5。

## 五、金向量读数（合法追加链与断裂链）

- 场景一 **legal_append_chain_verify_passes**：构造段合法链各事件 prev_hash 衔接，走 verify 全量 → 应过 rc0，返回首末哈希。实测 rc0，输出「链完整…共 1 事件，首 94f1dd00… 末 94f1dd00…」。
- 场景二 **broken_chain_verify_rejected**：构造 prev_hash 不衔接链，走 verify 全量 → 应拒 rc1 返回断裂事件 ID。实测 rc1，输出「链断裂…共 2 处违规」并报 prev_hash 与 event_hash 不衔接/不一致。
- 复算：teardown 于 engine 工地 target/debug/examples/verify_external_trail 同参形双跑逐字节一致零漂移，与冻结金向量四文本对齐，本批双跑证据入 materials（scriwire2-golden-run1/2-valid/broken.txt）。

## 六、管线读数（笔在核前）

- 化格：SPEC-004 目标 exit 0 无需改（域内）；rs 三件 general-v1 域外（不盖 .rs）如实记；推导档 md 目标 exit 1 已归一格式。
- 核阅：SPEC-004 域内 des-001 经图内暂存路径复验 exit 0 零违规（补救后）；rs 三件与推导档均域外 exit-2 如实记不属违规；ask3 双门第一门 exit 0 零违规。
- C002/C006 修复录：抢救件载体引用节初检有 3 处 des-001 违例（C002 箭头符号 U+2192 一处于映射行、C006 全角括号内容非法两处于映射行与对应关系行），复核续作已修正文案至合规（映射行去箭头与全角括号、对应关系行进独立分句）后复验 exit 0。抢救件是盘点源不是引用源，接线为交付物须合规。
- 检词：SPEC-004 与 rs 三件与推导档 exit 0 零违例；检词经 core 包。
- 词债：叩问得「盘点源」轻信号一，本金向量处置核对确认本批管线文档均不含该词（SPEC-004 载体节与推导档 0 出现），且 allow 清单无 nomenclator/packs/core/ 路径，不引入新词不触发 core 包登记（守卫 F-5）；解读见叩问处置节。

## 七、冲突样本节（pk-045 样本库，追加态首实战）

本批为 pk-045 样本库参与者，与 caswire2-solo 故意同跑实测追加态锁首次实战。共享面双 append 共存应即取即得，如实录插队读数；exclusive 面撞锁有限重试上限十次逐次计数。实测如下：

1. **追加态并发实录（本样本第 1 条）**：trail 共享追加面以 `lease lock --mode append` 短持取放，意图笔与四认证共五次取放全部即取即得（acquired_at 与 caswire2 交错在 2026-09-03 UTC 14:5x–15:0x 期间），未遇撞锁，追加态 append 双并存不互斥即插队合法。exclusive 施工面六路（event_stream/、SPEC-004、sih-math/docs/、三批文件）两次取锁均首跑即得 retries=1 零重试。重试计数合计 0 次，零撞锁如实记零。
2. **认证先落主树活链**：认证事件先落主树 2026-09-03.ndjson 活链，链文件 settle 前一次性拷工地，无工地链副本追加（承 pk-045 教训，严禁非书简通道改写链文件）。
3. **allow 占位解析缺陷（本样本第 2 条，申报）**：任务包 requested-writes 中 trail 以字面 `<开工实日>.ndjson` 承载，lease open 自任务包 parse_requested_writes 原样取用不作日期替换（core.py parse_requested_writes 逐行取列表项）。首次取 trail append 锁报 scope_violation。处置：收空租约以 `--allow sih-engine/sih/event/trail/2026-09-03.ndjson` 补充实日路径重开，allow 清单兼含占位与实日两径（占位路径无人锁取，无副作用）。教训登记：任务包 trail 占位需解析实日再 open。
4. **环境位移**：受 PYTHONHOME 污染风险，工具调用按 BATCH-FACE 坑位加 `env -u PYTHONHOME -u PYTHONPATH` 前缀规避 encodings 缺失。

## 八、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| c3335b08 | 意图笔（scribe intent） | ask3 记录 d8f49dc15e50d27f…（record_sha256 对表一致，意图事件 4e9c1f89） |
| 92bf0ca0 | 管线报告（2026-09-03-scriwire2-solo-pipeline.json） | 接线版 SPEC-004 域内全过 |
| dd5c6440 | 变更件报告（2026-09-03-scriwire2-solo-changed-files.json） | change_type=wiring_only_no_behavior_change，144 测全绿 |
| 78f57ec3 | 金向量报告（2026-09-03-scriwire2-solo-golden.json） | 两场景双跑逐字节一致 |
| 021b22b3 | 推导档报告（2026-09-03-scriwire2-solo-derivation.json） | 推导档承载三类链语义形式化 |

认证一律先落主树活链，链 verify 以批期链尾变更件认证笔为收（close 后读数为准，见收约对表节）。

## 九、三仓 commit

| 仓 | 分支 | commit | 承 |
|---|---|---|---|
| sih-engine | msh/scriwire2-solo | <待回填> | SPEC-004 载体引用节 + event_stream 三锚点 + results + materials |
| sih-math | msh/scriwire2-solo | <待回填> | 推导档 scriwire-scribe-derivation-2026-09-03.md |
| sih-tools | msh/scriwire2-solo | <待回填> | 五册 CALL-LOG 留痕（scriblet/lease/formatter/scrutinator/nomenclator） |

## 十、链 verify 前后对表

- 认证后 close 前：trail 2026-09-03.ndjson 以引擎 scribe verify 复核 status valid（本期认证事件五笔全在链）。
- close 后 reconcile 对表读数（unrouted 与 cert_missing 相比批前零新增，见收约附表）。

## 十一、越线与误差申报

无越线直写；工具侧 scribe 零碰；identity/reports 与既有存量 untracked 零收编；findings 亲读；无管道掩退出码（各步退出码单独捕获）。申报项：allow 占位解析缺陷（冲突样本节第 3 条）已如实记载并处置；C002/C006 三违例补录（管线节）已修复合规。