# guardrail-solo 结果档

> 批：冲突处理机制加固批（防分叉护栏 + 收约原子性 + 守卫补漏 + 余件入库）
> 会话：941409ce491354b4
> 日期：2026-09-03
> 队形：单线形 solo，零子代理
> 承接：guardrail-solo 执行指令（冲突模式，承 pk-045）
> 意图哈希：e80597952548ff2cf6d962679b001e5ef5190a94e964a4e2c47854aeafe9e2f5（会话台账 intent.record_sha256 对表一致，ask3 三锚，意图事件 7a9448e0）

## 一、批概览

本批为冲突处理机制加固批，四项：防分叉护栏拒工地链副本追加、收约原子性前置态整批拒、守卫补漏拒批名前缀无 session 行、余件入库。前三项为机制加固不承载金融数学向量，第四项把 ordwire-lease-solo 主树活写残迹纳入版控。

温故检索：recall-guardrail.json count 0 零命中如实记。

## 二、护栏落实

### 护栏一 防分叉（scribe 工地链副本禁追加）

scribe 四写入入口（append/intent/park/record）加 `worktree_trail_guard`：`--trail` 物理路径含 `worktrees/` 段即拒 exit 2 载错文「工地链副本禁追加即认证先落主链」，显式 `--allow-worktree-trail` 留应急位默认关。落 src/bin/scribe.rs，单测四件（ga1 append / ga2 record / ga3 intent / ga4 park）全红后全绿。SPEC-006 修订三载明，USAGE 同步旗标。

### 护栏二 收约原子性（close 前置态整批拒）

lease close 增前置态整批探针 `_close_precondition`：逐仓查合并态标记（MERGE_HEAD 与 rebase-merge/rebase-apply 与 CHERRY_PICK_HEAD 与 REVERT_HEAD 与 BISECT_LOG）与共享面脏（已跟踪改动非零或未跟踪件覆盖分支 tip 落地路径即碰撞判定），命中即整批拒绝零部分动作，错误详情 JSON 全量透出禁空串（ordwire 空详情事故反训）。未碰撞未跟踪件不入前置态。半程不可达即要么全归并要么零动作。两反例测即脏共享面拒与合并态拒，均断言支未删、工地未拆、会话在册、台账无 revoked/close_failed。

### 护栏三 守卫补漏（commit-msg 拒批名前缀无 session 行）

guardcore.validate_commit_message 拒因三值扩四值：BATCH_PREFIX 正则识别 subject 首记号连字符批名模式且无 session 挂接行即拒 `batch_prefix_no_session`（aea1768 同形必拦 F-3 硬判据，属对无 session 兜底的显式命名非新增拦截）。合法三形（settle/wip/merge）带批名前缀全放行回归。单测含纯函数反例与实际钩子端到端反例（回放 aea1768 同形消息退出一、提交零落成）。

### 余件入库

- sih-engine/sih/event/plan/ordwire-lease-solo-results.md 第 5 条行（收约碰撞两段式样本）随批入版控。
- sih-tools/lease/CALL-LOG.md 更正行（ordwire 收约更正）随批入版控，strip 手工并合并残留的 `+` 标记修正为规范行首 `- `。
- guardrail-solo 任务包、材料目录（dispatch + recall）与结果档随批入版控。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 防分叉护栏** | 工程 | 四写入入口工地链路径拒 exit 2 载错文，覆写旗标默认关显式开放行 | 过 | ga1-ga4 四测先红后绿；scribe 141 测全绿 |
| **F-2 收约原子性** | 工程 | 合并态或共享面脏即整批拒零部分动作，详情禁空串 | 过 | 脏共享面残区块链与合并态两反例测，零部分动作断言过 |
| **F-3 守卫补漏** | 治理 | aea1768 同形必拦，合法三形全放行 | 过 | 纯函数反例加钩子端到端双拦证；合法三形带批名前缀回归全放 |
| **F-4 余件入库** | 治理 | ordwire 第 5 条行与 CALL-LOG 更正行随批入版控 | 过 | 双仓 settle 内含两残迹逐字节对表 |
| **F-5 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 双仓工地 staged 清单对表未越 allow 冻结面 |

## 四、冲突样本节（pk-045 参与声明）

本批声明冲突模式：认证一律先落主树活链、链文件 settle 前一次性拷工地、严禁工地链副本追加、收约让位对表法。

1. **主树活写共享面施工**：护栏代码在主树活链工作面施工经 cargo/pytest 全绿后一次性拷工地 settle，工地零追加链副本，承 pk-045 防分叉护栏自证。
2. **护栏一工地拒证（F-1 红→绿）**：ga1-ga4 先红即无护栏时工地链路径可追加，后绿即 `--trail` 含 worktrees/ 段拒 2 载错文，显式 `--allow-worktree-trail` 放行；单测认证材质逐机器断言。
3. **收约前置态现场照**：本批 close 前置态探针即对目标仓合并态与共享面脏整批拒，ordwire 事故残迹为并行批先并入 main 的共享只追加文件碰撞样本（trail/CALL-LOG/terms 并发追加），本批在入口拦截而非半程留残。

## 五、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 7a9448e0 | 意图笔（scribe intent） | ask3 记录 e8059795 |
| 94102182 | 管线报告（2026-09-03-guardrail-solo-pipeline.json） | 域内全过 py 域外如实记 |
| 02118133 | 变更件报告（2026-09-03-guardrail-solo-changed-files.json） | 三护栏一余件文件清单 |

认证先落主树活链 2026-09-03.ndjson，链文件 settle 前一次性拷工地，链 verify 以批期链尾为收（close 后读数为准）。

## 六、管线读数

- 化格：SPEC-006 exit 0 零待改、CONTRACT exit 0 零待改、pyproject.toml exit 0；py 变更件 general-v1 域外 exit-2 如实记不属违规。
- 核阅：SPEC-006 des-001 exit 0 零发现（域内）；CONTRACT 与 py 件 des-001 域外（盖 sih-engine/doc）exit-2 如实记。
- 检词：SPEC-006 与 CONTRACT exit 0 零违例；py 件 core 词包域外 exit-2 如实记。
- 测试：scribe cargo 141 测全绿（含 ga1-ga4）、lease pytest 66 测全绿（含两收约反例与守卫反例）。

## 七、验收

- [x] F-1 至 F-5 全过
- [x] 三护栏各自红绿（工地路径拒证、close 反例拒证、守卫反例拦证）
- [x] 余件入库（ordwire 第 5 条行与 CALL-LOG 更正行）
- [x] 认证入链先落主树活链
- [ ] close 后 reconcile 读数与链 verify 对表（见收约附表，close 后补记）
