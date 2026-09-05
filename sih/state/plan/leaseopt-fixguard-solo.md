# leaseopt-fixguard-solo：收约守卫假阳性修复（批二）

> 批：leaseopt-fixguard-solo（leaseopt 线批二，收约守卫脏位判定修复）
> 承接：leaseopt-line-v1.md 批二节；批一账本 census-ledger.json（local-changes 112 + worktree-dirty-remove 37 + unmerged-index 2 = 151 条 64.5% 根因）；2026-09-04 晚 mainord-solo 与 gov002v2-solo 四笔 bypass 绕行实录
> 范式：T6-D 单线 solo，主会亲写，零子代理

## 一、问题陈述 {#problem}

`lease/src/lease/core.py` `detect_merge_conflicts`（981 行起）以「盘上内容 ≠ 分支内容」判脏位（1040 行锚句），不核对 HEAD 的真实脏位；修订二十九「严义不看 base」后，凡批期内基线前进过的文件（主树对 HEAD 净态）即被误判 diverged 整批拒。实证：纯盘点批零源码改动收约被拦（批一自撞实录）；2026-09-04 晚四笔 bypass 皆此因；`git merge` 三方合并本可干净仲裁这些件（branch 未动或同向时无冲突）。

## 二、修法定稿 {#design}

冲突判定域收缩：从「base..branch 差集全集」改为「差集 ∩ 真实脏位」。真实脏位 = `git status --porcelain` 的修改行与已暂存行（含 `??` 未跟踪，现实现只取 `??` 弃修改行，一并修）∪ 对 HEAD 的 diff 名单。差集中不在脏位集合的文件退场，交 git 三方合并仲裁；真内容冲突（branch 与 HEAD 双改不同向）由既有 `_merge_tree_conflicts` 闸四预检拦住，链路闭合。三态判别（same/append/diverged）与未跟踪碰撞语义零改动；净态收约语义零变化（净态且无碰撞本来就过）。

## 三、工作清单 {#work}

- [ ] 红证夹具三件：基线前进净态件应过（今拒）、branch 与 HEAD 双改不同向应拦（merge-tree 拦）、真脏位真分叉仍拒
- [ ] 修复 detect_merge_conflicts 判定域
- [ ] 绿证：三夹具过 + 既有全族零回归 + 金向量双跑
- [ ] CONTRACT 1.19.0 修订三十 + CALL-LOG 尾行
- [ ] 得一裁：判定语义变更测量九发，stable_clear 过执契终签
- [ ] 推导档 sih-math/docs/leaseopt-fixguard-derivation-2026-09-05.md（载体以 mapping.md 实命中为准，零命中如实申报）

## 四、可证伪条件 {#falsifiable}

| F | 判据 |
|---|---|
| F-1 | 夹具一：基线前进 + 主树净态 + branch 改件，修复前红（diverged 拒）修复后绿（收约过） |
| F-2 | 夹具二：branch 与 HEAD 双改同件不同向，merge-tree CONFLICT 拦，零半程动作 |
| F-3 | 夹具三：真脏位真分叉，diverged 拒语义保留 |
| F-4 | 既有 lease 测试全族零回归；金向量双跑逐字节一致 |
| F-5 | 得一裁 stable_clear 执契终签；boundary 即批失败呈人 |
| F-6 | 判定性常数零新增；untracked 面与三态判别零改 |

## 五、必读 {#read}

- lease/src/lease/core.py 981-1140（detect_merge_conflicts / is_pure_append_conflict / _merge_file_diverged）
- sih-engine/doc/spec/SPEC-020-close-merge-harden-v1.md
- sih-engine/sih/event/plan/leaseopt-audit-solo-results.md 第三节病灶一与第八节自撞实录
- sih-tools/lease/tests/test_lease.py 1237-1326（close 三测试搭法）

## 六、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/core.py
- sih-tools/lease/tests/test_lease.py
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/lease/pyproject.toml
- sih-tools/lease/src/lease/__init__.py
- sih-math/docs/leaseopt-fixguard-derivation-2026-09-05.md
- sih-tools/facet/contracts/leaseopt-fixguard-260905/
- sih-tools/proposition/DES/leaseopt-fixguard-solo/
- sih-engine/sih/event/plan/leaseopt-fixguard-solo-results.md
- sih-engine/sih/event/trail/
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- sih-tools/tally/reports/
- sih-tools/meter/counts/
