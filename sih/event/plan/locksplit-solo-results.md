# locksplit-solo 结果档

> 批：lease 锁机制三洞修复批（路径同一化 + 追加态锁 + 路径包含判定）
> 会话：25d44cbbb8e3cb67
> 日期：2026-09-03
> 队形：单线形 solo，本批单飞不与任何批并发（修锁的批不踩自己修的锁）
> 承接：用户裁定 2026-09-03 插队令「链上冲突，可以插队，并不存在太大的冲突点」与质询「锁竞态导致现在无法协作了」；载体承 ORD-020 全序资源分配（ordwire-lease-solo 已接线），本批为其资源同一性与锁型语义的机械修正
> 意图事件：0d477845（scribe intent，ask3 记录三锚，双门校验 status ok）
> 温故检索：materials/recall-locksplit.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，本批单飞如实记零并发

## 一、问题还原

锁竞态致协作中断，三洞坐实：

1. **路径形态绕穿互斥**：normalize_path 只去 `./` 前缀不折算绝对相对，两会话以不同形态锁同一路径互斥被绕穿（实况 24 笔绝对与 16 笔相对混用）。
2. **共享追加面独占串行**：追加面（trail 文件、CALL-LOG、reports、counts、台账、core 包）被独占锁串行化，并发批互憋即用户所谓无法协作。
3. **目录与文件包含不判**：目录锁与文件锁无包含判定，同底层面可双持（mathclose 目录锁与 scriwire 文件锁共存）。

## 二、三洞修复实装

| 洞 | 修复 | 落位 |
|---|---|---|
| 路径同一化 | normalize_path 升级为工作区根相对规范形（绝对剥根、去 `./`、尾斜杠统一）；锁账本新笔一律落规范形；互斥判定对历史笔双侧规范化后比较（账本 append-only 零改写） | lockcore.py normalize_path / acquire |
| 追加态锁 | lease lock 增 `--mode exclusive|append` 缺省 exclusive 向后兼容；append 与 append 可共存、exclusive 与任何形态互斥；unlock 按持位放；scribe lockgate_guard 对 trail 追加操作认 append 持位 | lockcore.py acquire / cli.py --mode / lockgate.rs |
| 路径包含判定 | 取锁冲突判定增包含规则即他方所持路径为本路径祖先目录或后代即冲突，堵目录文件双持洞 | lockcore.py acquire（_conflicts/_is_ancestor） |

active_locks 升级为多持位（同 path 映射持锁会话与锁型列表），支持 append 共存与按持位放锁。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 同一化** | 工程 | 异形态同路径互斥生效，账本新笔全规范形，历史笔比较零改写 | 过 | test_path_normalization_conflict 绿；金向量 normalization_same_path_conflict 双跑逐字节一致 |
| **F-2 追加态** | 工程 | 双 append 共存、append 与 exclusive 互斥、缺省形向后兼容 | 过 | test_append_mode_coexistence / test_append_release_own_hold / test_exclusive_append_conflict 绿；金向量 append_coexistence 与 append_exclusive_conflict 双跑一致 |
| **F-3 包含判定** | 工程 | 目录锁与文件锁互斥生效 | 过 | test_path_inclusion_conflict 绿；金向量 inclusion_dir_file_conflict 双跑一致 |
| **F-4 行为兼容** | 工程 | lease 既有测试全绿加新测全绿 | 过 | lease 56 测全绿（既有 51 + 新测 5）；lockgate 10 测全绿（既有 7 + 新测 3） |
| **F-5 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 三仓工地 git status 对表未越 allow 冻结面 |

## 四、金向量读数（四场景）

- 场景一 **append_coexistence**：两会话同路径双 append 持锁共存，final_held 双持位逐字节一致。
- 场景二 **append_exclusive_conflict**：exclusive 持位下 append 取锁被拒（locked_elsewhere），互斥生效。
- 场景三 **normalization_same_path_conflict**：绝对与相对异形态同路径互斥生效，路径同一化修复证。
- 场景四 **inclusion_dir_file_conflict**：目录锁与文件锁同底层面互斥（locked_inclusion），包含判定修复证。

复算：replay_golden.py golden_cases.json 双跑逐字节一致零漂移，与冻结金向量 IDENTICAL（冻结于 materials/locksplit-solo-golden-vector.json）。

## 五、管线读数

- 化格：CONTRACT 与推导档两件零改动 exit 0。
- 核阅：des-001 两件均域外（sih-tools 与 sih-math 非 sih-engine/doc）exit 2 如实记不属违规。
- 检词：CONTRACT 与推导档两件零违例 exit 0。
- 词债：无新造词，既有术语沿用，findings 亲读零遗留。

## 六、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者。本批单飞零并发，冲突点与响应逐条：

1. **主树残留未认证草稿（本样本新增实测）**：close 收约前置态被阻 merge_diverge——主树工作目录（sih-tools/sih-engine/sih-math）存在未提交、未认证的锁修复草稿（dict 形 active_locks 与独立 locked_inclusion 错误码，缺 5 个新测），与已认证工地分支（list 形 active_locks，56 测全绿）分叉。响应：草稿先备份至 /tmp/locksplit-solo-backup/（三仓 patch 逐件在案，不丢失），分叉件回退基线，close 归并已认证分支版本。金向量 final_held 形（`[["A","append"],["B","append"]]`）与分支 list 形一致，证分支为认证权威版本。
2. **close 半程归并（merge 部分应用）**：首次 close 因 CALL-LOG/pyproject/__init__/trail 本地改动被 git 拒，且 merge 失败前已把无本地改动的 5 件更新为分支内容。响应：逐件备份后回退基线，重试 close 成功（failed=[]，三仓 worktree removed、分支 deleted、revoked=true）。
3. **锁残留**：close 前置检报 4 锁在册（施工面长持），先逐路径 unlock 清零再收约。
4. **环境位移**：受 PYTHONHOME 污染风险，工具调用按 BATCH-FACE 坑位加 `env -u PYTHONHOME -u PYTHONPATH` 前缀规避 encodings 缺失。

## 七、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 0d477845 | 意图笔（scribe intent） | ask3 记录三锚，双门校验 status ok |
| c6a1a3fa | 意图笔（validation） | ask3 验证件 |
| 28fc28a3 | 测试报告（2026-09-03-locksplit-solo-tests.json） | lease 56 测 + lockgate 10 测全绿 |
| 7c05a8ec | 金向量报告（2026-09-03-locksplit-solo-golden.json） | 四场景冻结向量 |
| 25e6a465 | 管线报告（2026-09-03-locksplit-solo-pipeline.json） | 化格/核阅/检词读数 |
| b966a148 | 推导档报告（2026-09-03-locksplit-solo-derivation.json） | ORD-020 推导档 §3.4-3.6 |
| 23b92e32 | 变更件报告（2026-09-03-locksplit-solo-changed-files.json） | 三仓变更件清单 |

认证一律先落主树活链，链 verify 以批期链尾变更件认证笔为收（close 后读数为准，见对表节）。

## 八、收约对表（close 后 reconcile 读数）

| 仓 | unrouted | unbypassed | cert_missing | 归因 |
|---|---|---|---|---|
| sih-tools | 0 | 0 | 1 | 历史遗留（entryunique-solo 段2 526e2be）非本批；本批 merge c0c3fe72 归 routed_merge |
| sih-engine | 0 | 0 | 0 | 本批 merge ef83d7d 归 routed_merge |
| sih-math | 0 | 1 | 2 | 历史遗留（baseline init c556abb、mathfix2/fmtfix 段2）非本批；本批 merge 23f342b 归 routed_merge |

当日链 verify（close 后）：status=valid，173 events，链尾 last_hash=23b92e32=批料变更件认证笔，收束正确。

## 九、验收

- [x] F-1 至 F-5 全过
- [x] 金向量四场景在档可重放（materials/locksplit-solo-golden-vector.json 冻结）
- [x] 认证入链，三仓结算收约，close 后 reconcile 零新增违规，当日链 verify valid
