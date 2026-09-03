# locksplit-solo 结果档

> 批：lease 锁机制三洞修复批（路径同一化 + 追加态锁 + 路径包含判定）
> 会话：25d44cbbb8e3cb67
> 日期：2026-09-03
> 队形：单线形 solo，零子代理，**本批单飞不与任何批并发**（修锁的批不踩自己修的锁）
> 承接：用户裁定 2026-09-03 插队令原话「链上冲突，可以插队，并不存在太大的冲突点」与同日质询「锁竞态导致现在无法协作」；批群死复盘与 caswire 加 scriwire 实况坐实三洞；载体承 ORD-020 全序资源分配（ordwire-lease-solo 已接线），本批为其资源同一性与锁型语义的机械修正
> 意图哈希：1cf96cad2799b09da2baedf822260e6b8b557dbe98d18d2fba151639b0646dd4（会话台账 intent.record_sha256 对表一致，ask3 验证件 status ok 三锚，意图事件 0d4778456a36 与 c6a1a3faab36）

## 一、问题还原

其一，normalize_path 只去 ./ 前缀不折算绝对相对（lockcore.py:262-268），两会话以不同形态锁同一路径互斥被绕穿（实况 24 笔绝对与 16 笔相对混用）。其二，追加面（trail 文件、CALL-LOG、reports、counts、台账、core 包）被独占锁串行化，并发批互憋即用户所谓无法协作。其三，目录锁与文件锁无包含判定，同底层面可双持（mathclose 目录锁与 scriwire 文件锁共存实况）。

温故检索：materials/recall-locksplit.json 双档零命中如实记。

## 二、关键设计（三洞修复）

1. **路径同一化**：normalize_path 升级为工作区根相对规范形即绝对路径剥根前缀、去 ./ 前缀、尾斜杠统一；锁账本新笔一律落规范形；互斥判定对历史笔双侧规范化后比较（账本 append-only 零改写）。
2. **追加态锁**：lease lock 增 --mode exclusive|append 缺省 exclusive 向后兼容；判定规则即 append 与 append 可共存、exclusive 与任何形态互斥；unlock 按持位放；scribe lockgate_guard 对 trail 追加操作认 append 持位。任务包共享追加面声明追加态即机械承载插队裁定。
3. **路径包含判定**：取锁冲突判定增包含规则即他方所持路径为本路径祖先目录或后代即冲突，堵目录文件双持洞。
4. CONTRACT 修订二十五升 1.14.0（锁型语义与三洞修复记录）、ORD-020 推导档补 §3.4 资源同一性与 §3.5 锁型语义与 §3.6 路径包含判定、金向量四场景冻结。
5. 既有测试全绿即行为兼容面零破坏（缺省 exclusive 下旧调用形零变）。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 同一化** | 工程 | 异形态同路径互斥生效，账本新笔全规范形，历史笔比较零改写 | 过 | 金向量 normalization_same_path_conflict 机械重放；lockcore normalize_path 工作区根相对规范形，账本 append-only 零改写 |
| **F-2 追加态** | 工程 | 双 append 共存、append 与 exclusive 互斥、缺省形向后兼容 | 过 | 金向量 append_coexistence 与 append_exclusive_conflict 双场景；lease 56 测全绿证缺省 exclusive 旧调用形零破坏 |
| **F-3 包含判定** | 工程 | 目录锁与文件锁互斥生效 | 过 | 金向量 inclusion_dir_file_conflict 机械重放 |
| **F-4 行为兼容** | 工程 | lease 既有测试全绿加新测全绿 | 过 | lease 56 测全绿（51 既有 + 5 新：append_mode_coexistence/append_release_own_hold/exclusive_append_conflict/path_normalization_conflict/path_inclusion_conflict）；lockgate 10 测全绿（7 既有 + 3 新：g8_append_holder_allows/g9_exclusive_holder_blocks/g10_append_plus_exclusive_blocks） |
| **F-5 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 三仓工地 git status 对表未越 allow 冻结面，changed-files 报告逐件在 allow 面内 |

## 四、金向量读数（四场景）

- 场景一 **append_coexistence**：同路径双 append 锁共存各自持位，active_locks 多持位列表承载——追加面并发机械重放（F-2）。
- 场景二 **append_exclusive_conflict**：append 与 exclusive 双向互斥——锁型语义机械重放（F-2）。
- 场景三 **normalization_same_path_conflict**：绝对形与相对形锁同一路径即撞锁——路径同一化修复证（F-1）。
- 场景四 **inclusion_dir_file_conflict**：他方持目录锁即文件锁撞锁——路径包含判定修复证（F-3）。
- 复算：replay_golden.py golden_cases.json 双跑逐字节一致零漂移，与冻结向量 locksplit-solo-golden-vector.json IDENTICAL。

## 五、管线读数

- 化格：CONTRACT 与推导档 exit 0 零改动；py 变更件 general-v1 域外（只盖 md/json/yaml/toml）如实记不属违规。
- 核阅：des-001 两件均域外（非 sih-engine/doc 域）exit 2 如实记不属违规。
- 检词：CONTRACT 与推导档 exit 0 零 findings（合入主树后复验仍零）；py 变更件域外。
- 词债：本批无新增词债，检词零 findings。

## 六、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者。本批单飞零并发如实记，冲突点与响应逐条：

1. **单飞零并发**：本批不与任何批并发，修锁的批不踩自己修的锁，allow 七路取锁七路零撞锁。
2. **收约碰撞（本样本新增实测）**：close 首跑三仓中 math 归并成功，tools 与 engine 两仓 merge_failed——tools 侧 lease/CALL-LOG.md 与 lease/pyproject.toml 与 lease/src/lease/__init__.py 与 scribe/CALL-LOG.md 本地改动将被覆盖，engine 侧 trail 2026-09-03.ndjson 本地改动将被覆盖。处置为手工副本归并：tools c0c3fe72 与 engine ef83d7d 双亲 merge 形经 guardcore validate 通过，reconcile 判 routed_merge 零 unbypassed，无需 bypass 登记；重试 close 成功 revoked 14:22:12，三仓 worktree removed、分支 deleted。
3. **认证先落主树活链**：意图笔与五报告认证先落主树 2026-09-03.ndjson 活链，链文件 settle 前一次性拷工地，无工地链副本追加（承 pk-045 教训）。
4. **环境位移**：受 PYTHONHOME 污染风险，工具调用按 BATCH-FACE 坑位加 `env -u PYTHONHOME -u PYTHONPATH` 前缀规避 encodings 缺失。

## 七、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 0d4778456a36 | 意图笔（intent_refined） | ask3 记录 1cf96cad |
| c6a1a3faab36 | 意图笔（intent_refined） | ask3 记录 1cf96cad |
| 28fc28a38ca2 | 测试报告（2026-09-03-locksplit-solo-tests.json） | lease 56 测 + lockgate 10 测 |
| 7c05a8eca7e2 | 金向量报告（2026-09-03-locksplit-solo-golden.json） | 四场景双跑逐字节一致 |
| 25e6a465a3d3 | 管线报告（2026-09-03-locksplit-solo-pipeline.json） | 化格核阅检词三步读数 |
| b966a148927f | 推导档报告（2026-09-03-locksplit-solo-derivation.json） | CONTRACT 修订二十五 + ORD-020 推导档 §3.4-3.6 |
| 23b92e32f216 | 变更件报告（2026-09-03-locksplit-solo-changed-files.json） | 三仓工地变更清单逐件在 allow 面内 |

认证一律先落主树活链，链 verify 以批期链尾变更件认证笔为收（close 后读数为准，见对账对表节）。

## 八、收约碰撞两段式（本样本第 2 条）

close 首跑三仓：math 归并成功（23f342b），tools 与 engine 两仓 merge_failed（tools 侧 CALL-LOG/pyproject/__init__ 四件本地改动将被覆盖，engine 侧 trail 本地改动将被覆盖）。机械响应即手工副本归并：

1. tools 与 engine 两仓以双亲 merge 形手工归并（tools c0c3fe72、engine ef83d7d），merge 形信息经 guardcore validate 通过，非 --no-verify 直写故无需 bypass 登记。
2. 重试 close 成功：failed=[], 三仓 worktree removed、分支 deleted、revoked=true（14:22:12）。
3. 三仓 merge 均归 routed_merge，reconcile 零新增违规（见对账对表节）。

## 九、对账对表（close 后 reconcile 读数）

| 仓 | unrouted | unbypassed | cert_missing | 归因 |
|---|---|---|---|---|
| sih-math | 0 | 1 | 2 | 历史遗留（baseline c556abb、mathfix2 93c4f0b、fmtfix d561f17）非本批；本批 merge 23f342b 归 routed_merge |
| sih-tools | 0 | 0 | 1 | 历史遗留（entryunique-solo 526e2be）非本批；本批 merge c0c3fe72 归 routed_merge |
| sih-engine | 0 | 0 | 0 | 本批 merge ef83d7d 归 routed_merge |

当日链 verify（close 后）：status=valid，173 events，链尾 last_hash=23b92e32=批料变更件认证笔，收束正确。

## 十、验收

- [x] F-1 至 F-5 全过
- [x] 金向量四场景在档可重放
- [x] 认证入链，三仓结算收约，close 后 reconcile 零新增违规，当日链 verify valid
