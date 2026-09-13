# MANIFEST.md — leg2 domain2 金向量捕获总表

域：`work/leg2-fixtures/domain2`；捕获材料逐场景四件 `cmd.txt / stdout.json / stderr.txt / exit.txt`。
会话：goldc `b516664e0251a887`、goldd `2af46b28cdc6c2f4`（派生式见 NORMALIZATION.md）。
退出码约定：0 成功回执；1 执法拦截（CommitBlocked/StateError/LockBlocked）；2 用法错/工具异常。

## commit 面

| 场景 | 退出码 | 说明 |
|---|---|---|
| c05-commit-wip-success | 0 | wip 提交成功回执全形（checks、机械 message、base 标签、--note 入 message） |
| c06-commit-wip-nothing-staged | 1 | `nothing_staged` 拒绝（重复提交/零变更形） |
| c07-commit-settle-missing-seq | 2 | settle 缺 `--seq` 用法拒（CLI 首检形） |
| c08-commit-settle-missing-cert | 2 | settle 缺 `--cert` 用法拒（WorktreeError 形） |
| c09-commit-settle-fake-cert | 1 | `cert_not_on_chain` 拒绝全文（假哈希） |
| c14-commit-settle-success | 0 | settle 提交成功回执全形（真链 cert 前八位，message 挂接行 session+cert+base） |
| c10-commit-main-tree-rejected | 1 | `commit_must_target_worktree` 拒绝（主检出直提，回显副本路径） |
| c11-commit-repo-not-in-session | 1 | `repo_not_in_session` 拒绝（会话外仓） |
| c12-commit-session-not-active | 1 | `session_not_active` 拒绝（会话不在册） |
| c15-commit-staged-out-of-scope | 1 | `staged_out_of_scope` 拒绝（越 allow 面，回显 outside 清单） |

## sddgate 面（挂 close 链闸，链证守门后、CALL-LOG 跑步机前）

| 场景 | 退出码 | 说明 |
|---|---|---|
| c17-close-sddg-all-reject | 1 | SDDG 四判据全拒全文（教学载荷：判定命令+反例+三通道） |
| c22-close-sddg-1-only-reject | 1 | SDDG-1 单独拒（典在码前：t0/t1/first_impl_spec_hits/window_spec_hits/intent_spec_declared 全空） |
| c21-close-sddg-2-only-reject | 1 | SDDG-2 单独拒（新源码件无正典指针，zero_pointer_hits 列件） |
| c20-close-sddg-3-uncarried-reject | 1 | SDDG-3 单独拒（偏差条目缺承载词形，uncarried_entries 列件） |
| c18-close-sddg-3-4-reject | 1 | SDDG-3 `results_doc_absent` + SDDG-4 联合拒（结果档缺席形） |
| c19-close-sddg-4-only-reject | 1 | SDDG-4 单独拒（差分有 acceptance 无测试件） |
| c23-close-sddg-4-bypass-success | 0 | `--bypass-sddgate` 绕行：sddgate_gate verdict=bypass + bypass.ndjson 留痕行 + 成功收约全形（shadow 副本跑，见 NORMALIZATION F3） |

## guard/close 面

| 场景 | 退出码 | 说明 |
|---|---|---|
| c27-close-orphan-gate-reject | 1 | 无主闸拒绝全文（无主清单逐件 path/xy/mtime + 三通道指引） |
| c28-close-declaration-gate-reject | 1 | 差集闸拒绝全文（声明未提交清单，请求写入节对分支树） |
| c29-close-ack-mismatch-reject | 1 | 认领对表失败（认领差集外路径，unknown_acks 零粉饰） |
| c30-close-ack-uncommitted-success | 0 | `--ack-uncommitted 路径=事由` 放行 + 成功收约回执全形（acks 留档） |
| c31-bypass-record | 0 | `lease bypass` 登记行形（bypassed 事件：at/reason/repo/session/sha/tool） |
| c32-status-view | 0 | status 查册回执（会话历史 + 锁面 + summary） |

## 会话生命周期与链笔（配套全形）

| 场景 | 退出码 | 说明 |
|---|---|---|
| c01-open-goldc | 0 | open 签发回执全形（identity/intent/allow_effective/allow_derived/stem_check skip 形/gauge skip 形） |
| c02-lock-goldc-workfile | 0 | lock 取锁成功（auto→exclusive） |
| c03-lock-identity-drift | 1 | 锁五验身份拒绝（hostname_drift 绑定失配全形） |
| c04-scribe-intent-goldc | 0 | scribe intent 链笔回执（appended+event_hash） |
| c13-scribe-cert-goldc | 0 | scribe append 认证笔回执（cert 哈希来源） |
| c16-unlock-goldc-workfile | 0 | unlock 放锁回执 |
| c24-open-goldd | 0 | 第二会话 open（声明面单件形） |
| c25/c26 scribe intent/cert (goldd) | 0/0 | goldd 链笔（链证守门输入） |

## findings（行为发现，非回执）

| 件 | 说明 |
|---|---|
| findings/finding-dateonly-at-treadmill-crash/ | date-only `--at` 使 close 全路径在 calllog 闸 TypeError 崩（NORMALIZATION F1） |

配套文档：`NORMALIZATION.md`（非确定性字段归一化 + 八项行为发现 F1–F8）。
脚本：`../cap.sh`（捕获壳）、`../rungoldc.sh`、`../rungoldd.sh`（全序列复演）。
fixture 域：`../../leg2-fixtures/domain2/`（双 git 仓 + inputs + 台账 + 链 + shadow 副本）。
