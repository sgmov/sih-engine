# closeguard-solo：租约收约硬化 - 前置机械对表与确定性处置

> closeguard-solo 租约收约硬化批
> 承接：SPEC-020 close-merge-harden-v1
> 范式：T6-D 范式（单线实施）—— 立文类，零子代理确定性实装

## 一、问题陈述 {#problem}

- **问题 1**：收约失败自 2026-08-29 dose01-solo 至 2026-09-04 measure-poly-rev1-progdoc 持续发生，共 184 件失败记录（sih-tools/lease/ledger/sessions.ndjson）
- **问题 2**：根因为机制缺陷 - 治理架构存在合法批中主树活写（书简认证追加链文件、泊界材料、CALL-LOG 尾行、台账），而 close 假设批写入只在工地，主树必然脏，merge 拒绝覆盖即死
- **问题 3**：既有实现为裸 git merge 碰运气，无前置机械对表与确定性处置

## 二、关键设计 {#design}

### 2.1 三态判别逻辑

- **同内容即让位放行**：脏位内容与分支对应内容完全一致 → 备份保全后 checkout 让位，再 merge
- **纯追加形走让位归并**：文件为 ndjson 且落盘行序以 base 全行集为严格前缀且多出尾行 → 备份落盘内容后 checkout 让位，merge 后重读合并件算 re_certify_hashes 补笔清单
- **非纯追加形即整批拒**：真分叉（既不等 branch 固件也不等 base 原样）→ 整批拒零动作

### 2.2 机制函数清单

- `detect_merge_conflicts(repo_path, branch, base_branch)`：枚举主树脏位中与合并目标路径相交的逐件清单
- `is_pure_append_conflict(conflict_files, repo, base, branch)`：判别纯追加形
- `check_worktree_clean(worktree_path)`：工地卫生检查
- `allow_and_merge(repo_path, branch, base_branch, conflict_files)`：让位归并逻辑
- `backup_conflict_files(repo_path, conflict_files, backup_dir)`：备份保全
- `generate_re_certify_hashes(backup_dir)`：补笔清单生成

### 2.3 夹具样本清单

- **样本一：local-changes（形态 a）**：session_id e8faac477f48c608，package dose01-solo，failed_files ["scribe/CALL-LOG.md"]
- **样本二：untracked（形态 b）**：session_id bde05bc3711f576f，package tallywire-solo，failed_files ["sih/event/trail/2026-08-30.ndjson"]
- **样本三：contains-modified（形态 c）**：session_id e8faac477f48c608，package dose01-solo，失败 detail 工地脏

## 三、工作清单 {#work}

### Cluster 1：实装与夹具

- [x] closeguard-02：实装报告 closeguard-solo-impl.md（SPEC 先行，机制加夹具）
- [x] closeguard-03：契约同步 closeguard-solo-contract-sync.md（CONTRACT 版本行与 CALL-LOG 尾行、BATCH-FACE 勘误）
- [ ] closeguard-04：活体收约验证 closeguard-solo-live-verification.md（本批收约并以活体收约结果为验证件）
- [ ] closeguard-05：pk-045 样本账面补记

### Cluster 2：验收测试

- [ ] 三类失败签名夹具齐，修复前红后绿，双跑逐字节一致
- [ ] 既有 lease 测试全绿，净态收约语义零变化
- [ ] 新增判定性常数零裸奔（载体或冻结登记在账）
- [ ] CONTRACT 版本行与 CALL-LOG 尾行在档，BATCH-FACE 勘误随批
- [ ] pk-045 样本账面有本批补记

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 三类失败签名夹具齐 | 夹具准备 | local-changes/untracked/contains-modified 三类失败样本在 lease/tests/fixtures/close-failures/ |
| **F-2** 修复前红证 | 红证验证 | close_failed 复现可重放（同参双跑逐字节一致） |
| **F-3** 修复后绿证 | 绿证验证 | 按策略让位/补笔/拒可重放（同参双跑逐字节一致） |
| **F-4** 净态收约语义零变化 | 回归测试 | 既有 lease 测试全绿（test_lease.py） |
| **F-5** 新增判定性常数零裸奔 | 载体登记 | ORD-019 版本偏序与外化存储挂载，或冻结登记在账 |
| **F-6** 合约与面单勘误 | 契约同步 | CONTRACT 版本行（1.18.0）与 CALL-LOG 尾行、BATCH-FACE 勘误随批 |

## 五、必读文件 {#read}

- SPEC-020：/Users/moc/workspaces/SiHankor/sih-engine/doc/spec/SPEC-020-close-merge-harden-v1.md
- CONTRACT 1.17.0：sih-tools/lease/CONTRACT.md 修订二十八（basefix-solo）
- CONTRACT 1.15.0：sih-tools/lease/CONTRACT.md 修订二十五（guardrail2-solo）
- 机制设计：sih-tools/proposition/DES/closeguard-solo/closeguard-solo-mechanism.md
- 旧失败史：sih-tools/lease/ledger/sessions.ndjson
- 既有测试：sih-tools/lease/tests/test_lease.py

## 六、约束 {#constraints}

1. **零 LLM 并行执行**：前置机械对表全程无 LLM，仅确定性 git 操作与文件内容比对
2. **判定语义保守**：净态路径的收约语义零变化，只有撞线路径获得新处置
3. **数学锚纪律**：三分界（同/追加/异）新常数与边界逐件登记（ORD-019 版本偏序与外化存储为首选）

## 七、验收标准 {#acceptance}

本任务包验收 = 6 项：

- [ ] F-1：三类失败签名夹具齐（local-changes/untracked/contains-modified）
- [ ] F-2：修复前红证（close_failed 复现）→ F-3：修复后绿证（按策略让位/补笔/拒）
- [ ] 同参双跑逐字节一致（既有 lease 测试全绿零回归）
- [ ] 新增判定性常数零裸奔（载体或冻结登记在账）
- [ ] CONTRACT 版本行与 CALL-LOG 尾行在档，BATCH-FACE 勘误随批
- [ ] pk-045 样本账面有本批补记

## 八、风险点 {#risks}

- 无新增副作用：本批是机制硬化批，只硬化 close 内部路径，不改八子命令既有命令面语义
- 工地脏位缺省：拦并报（推荐，提交归代理责任）或自动提交
- 历史样本回归范围：三签名各一件起，或全量 close_failed 史入夹具

## 九、范式偏离声明 {#deviation}

本任务包**偏离 T6-D 范式**的「双子代理并行」流程：

- 理由：本批是确定性任务，零子代理
- 偏离：单线 solo 实装，零子代理
- 保留：T6-D 命名约定 + F 锚定 + 跨仓同步

## 十、关联文件 {#related}

- 规格档：sih-engine/doc/spec/SPEC-020-close-merge-harden-v1.md
- 机制设计：sih-tools/proposition/DES/closeguard-solo/closeguard-solo-mechanism.md
- 实装报告：sih-tools/proposition/DES/closeguard-solo/closeguard-solo-impl.md（待 closeguard-02 产出）
- 契约同步：sih-tools/proposition/DES/closeguard-solo/closeguard-solo-contract-sync.md（待 closeguard-03 产出）
- pk-045：sih-engine/doc/governance/PARKING-v1.md

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/core.py（close_session 函数）
- sih-tools/lease/tests/test_lease.py（新增测试）
- sih-tools/lease/CONTRACT.md（修订 1.18.0）
- sih-tools/BATCH-FACE.md（勘误随批）
- sih-tools/lease/ledger/sessions.ndjson（活体收约事件）
- sih-tools/lease/ledger/locks.ndjson（活体收约锁事件）
