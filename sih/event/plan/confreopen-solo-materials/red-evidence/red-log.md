# confreopen-solo 首跑红证归档（先红留痕纪律，2026-09-08）

| # | 时点 | 红证 | 退出码 | 处置 |
|---|---|---|---|---|
| R1 | 例扫 | rev3_script.py 双跑各 exit 1，findings 两笔（scrutinator 语料命中未被载体映射覆盖 ALG-012；ALG-002/ORD-019/PROB-010/PROB-011/TOP-008 等），声明滞后灰项族 | 1/1 | checkmath exit 0 零红灰 6，四路 cmp 全 IDENTICAL，如实转述不停批（run1.stdout.txt 与 run2.stdout.txt 在档） |
| R2 | watch 对表首跑 | 管道接 head 掩退出码违坑位禁令，首读误得 RC 0 | 0（假） | 立即重跑输出重定向真捕 RC=1，无主清单十件（CALL-LOG 族九册加 calls.ndjson 候清项）如实呈报 |
| R3 | tools 工地归并首笔 | git merge 手写提交信息被直提守卫拦截：batch_prefix_no_session（报文全文见本表下方） | 1 | 归并已暂存未提交，转 lease commit 正规通道完成 |
| R4 | lease commit 工地 cwd 首跑 | self_boot_rejected：工位跑 CLI 必须三参显式全传（JSON 全文见 lease-commit-engine-nothing-staged-red.json 同形报文；tools 侧同款） | 2 | 补 --locks 后发现 commit 子命令旗标集无 --locks（exit 2 unrecognized arguments 红证同族），正形为主树 cwd 跑 CLI 指工地 --repo，即改即过 |
| R5 | engine 承接笔 lease commit | nothing_staged：engine 侧旧支内容已全数在 main（五材料件 blob 逐字节 IDENTICAL），空归并零内容可提交 | 1 | git merge --abort 撤空归并，改走任务包「对表确认旧提交可直接延续形」，零提交承接，理由入结果档 |
| R6 | engine 空归并尝试 | merge msh/confpreempt-solo 进入 merging 态零暂存 diff，守卫拦 no_session 未落提交 | 1 | 同 R5 合并处置 |

## R3 守卫报文原文

```
直提守卫拦截：提交信息不合 lease 模板形态。正规路径：lease commit --repo <仓> --session <会话号> --stage wip|settle --subject <事述>（settle 加 --seq 与 --cert）；显式绕行：git commit --no-verify 后 lease bypass --repo <仓> --sha <提交号> --reason <事由> 登记留痕。
拒因：batch_prefix_no_session
```

## R4 self-boot 报文原文（tools 侧）

```json
{"action": "传三参即可放行", "error": "self_boot_rejected", "reason": "工位（worktrees/ 下）跑 CLI 必须三参显式全传：--ledger + --locks + --bills", "self_boot_msg": "src=…/sih-tools/lease/src/lease/core.py cwd=…/worktrees/sih-tools/confreopen-solo in_worktree_src=False in_worktree_cwd=True"}
```

## R4 续红（commit --locks 旗标不存在）

```
lease: error: unrecognized arguments: --locks /Users/moc/workspaces/SiHankor/sih-tools/lease/ledger/locks.ndjson
```

## 收约期续红（R7 至 R10，先红留痕纪律）

| # | 时点 | 红证 | 退出码 | 处置 |
|---|---|---|---|---|
| R7 | checkcite 首跑 | verdict fail：结果档散文裸写载体编号被提取为引用不在书单 | 1 | F-1 材料修复通道改引用形复跑 pass，红证件与逐字读数零改动 |
| R8 | 投影件路径 | lease settle staged_out_of_scope 拒（readout 在 allow 面外）＋补锁 scope_violation 拒 | 1/1 | 移入 materials 面内重认证 0a012960，内容字节零改 |
| R9 | unlock 首跑 | 漏传 --identity 即 argparse 用法错 | 2×12 | 补参全过零副作用 |
| R10 | close 三跑 | 一跑真分叉拒（未跟踪目录启发式判分支独有七件 diverged）；二跑无主闸拦十六件 CALL-LOG 族候清项 | 1/1 | 七件同内容字节复制主树让位＋run2 证据件补笔入支（14fe97c）；三跑双旗 bypass 收约成，事由留痕不代收 |
