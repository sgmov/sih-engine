# mcpbeta-solo 批首跑红证留痕（先红留痕纪律，2026-09-09）

> 令源：BATCH-FACE.md 先红留痕纪律（gvecmath 验收直改笔增，2026-09-09 适用新批）
> 本批测试先红后绿共九发，全量如实记档；每发红证均未清洗未删除，复跑在红证修证后。

## 红证一：test_tool_descriptions_frozen_static_no_instruction_face

- 首跑断言：`assert "DES-014" in t.description` 对全量十七工具；
- 红：alpha 五工具描述正典指针是 SPEC-023（alpha 契约），DES-014 只载 beta 工具；
- 判：断言错，工具面行为对；修为仅对 MATRIX_ROWS 内工具断言 DES-014 指针。

## 红证二：test_zero_write_guard_writeface_no_direct_disk_write（两发）

- 首跑断言：`open\([^)]*['\"][wax]` 直写盘守卫 grep writeface 源码；
- 红：session.py 正身暂存位 `os.fdopen(fd, "w", ...)` 误中 `open(` 子串（fdopen 词尾）；
- 判：守卫正则缺词边界，误中白名单单点（tempfile.mkstemp + os.fdopen，DES-014 允许的
  正身报告暂存位）；修为 `\bopen\(` 词边界形。首替未中（源文件引号前有反斜杠转义），
  次替精确命中，替后绿。

## 红证三：fixture git commit 失败（conftest 工场构造）

- 红：`git commit -m fixture genesis` 对 sih-tools fixture 仓退出码一（空树无件可提交，
  git 报文走 stdout）；
- 判：工场构造缺占位件；补 `tl/README.md` 占位后绿。

## 红证四：test_park_passthrough——停泊拒 ActionMissing

- 红：scribe park 对夹具停泊记录报 `停泊拒 ActionMissing`（退出码一）；
- 判：夹具记录缺 `action` 字段；照 park.rs 记录 schema（action=enter 加 entry_id 与
  title 与 exit_condition 与 ttl_days）补齐后绿。server 侧零改动——拒定来自既有程序，
  正是透传设计预期。

## 红证五：test_direct_passthrough_local_only——会话在册验需 --sessions

- 红：scribe direct 闸三报 `会话在册验需 --sessions <会话台账路径>`（退出码二）；
- 判：scribe_direct_argv 漏传 --sessions（2026-09-04 BATCH-FACE 勘误坑位：闸类写入必带
  --sessions）；补参后绿。属实装坑位非设计缺口。

## 红证六：test_locked_elsewhere_conflict_surfaces——同包活跃闸（两发）

- 首跑红：两会话同包 open，第二会话被既有同包活跃闸拒（PackageSessionActive）；
- 判：DES-014 第一节一对一与 lease 同包活跃闸联动的预期行为，测试须走双任务包形；
  fixture 增 mcpbeta-fixture-2 任务包，两会话先开后锁（开工预检只对持锁面，双开窗口
  预检净）。
- 次跑红：两会话对 trail 径取锁俱成（exit 0）；
- 判：trail 属共享追加面，lock 缺省 mode=auto 命中 SCOPE_SHARED_SURFACE 落 append 模式
  双持合规——既有 lease 语义正确，测试选径错；改用 doc 面独占径后 locked_elsewhere
  照现。此红证顺带实证：共享追加面 append 双持与独占面 locked_elsewhere 拒两语义俱
  原位生效。

## 红证七：test_commit_wip_passthrough——nothing_staged 与 staged_out_of_scope（两发）

- 首跑红：`repo_not_in_session`——commit 的 --repo 相对形经 Path.resolve() 按 lease 子
  进程 cwd（/）解析失配；
- 判：commit 须指围堰（commit_must_target_worktree 既有闸）且 lease 无根锚解析位；
  实装增 anchored() 参数根锚定归一（相对形按数据根拼绝对，静态归一非判定），测试指
  围堰路径。
- 次跑红：`nothing_staged`——lease commit 只提交已暂存件（既有批次流 git add -A 在
  commit 前）；
  判：测试补 git add -A。
- 三跑红：`staged_out_of_scope`——暂存件不在会话 allow 面；
  判：四验范围验原位生效的预期行为；wip 件改落 allow 面内（trail 面）后绿。

## 红证八：test_stdio_smoke_beta_write_path_verify_loop——events 计数断言

- 红：`assert out["chain_verify"]["events"] >= 3` 得 2；
- 判：闭环序为验链在认证笔后停泊笔前，verify 时点链面恰两笔（intent_refined +
  certification_completed）；断言修为 == 2 并注明时点。工具面零改动。

## 红证九（非测试）：叩问 digest 首跑 blocked

- 红：elicit digest 对 ask3 记录报 blocked（缺叩问处置标记）；
- 判：ask3 记录缺 `叩问处置[词]` 标记十二枚；补 elicit_dispositions 字段后双门重跑
  绿、digest passed covered 12。ask3 记录改动在链前（意图笔未落），双门重跑合规。

## 判据

九发红证俱是测试工场与夹具与调用参形缺陷，实装代码红证仅两发（红证五 --sessions
漏参、红证七 anchored 缺位），俱以补参与归一修复，判定语义零改动、DES-014 零触碰；
既有工具代码零改动全程保持（红证俱证既有执法按设计原位生效）。
