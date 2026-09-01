# guardhook-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 guardhook-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/guardhook-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。用户已批准守卫开权，裁决材料即 adjudisp-solo 的 adisp-guard-1（链事件 2ada7331）。

## 实装要点

1. **guardcore.py 纯函数**：`validate_commit_message(text) -> (ok, reason)`，合规两形——(a) 正文任意行以 `session:` 开头且任意行以 `cert:` 开头（lease commit wip/settle 形）；(b) 首行匹配 `merge: <批名> 副本归并`（close 归并形）。其余全拒。钩子脚本 hooks/pre-commit 薄壳即读 COMMIT_EDITMSG 调 python -c 内联或调 guardcore（以 python3 可用性为准，钩子内禁依赖 venv）。
2. **install-hooks / uninstall-hooks 子命令**：向指定仓写 `git config core.hooksPath <lease仓>/hooks`；uninstall 即 unset。--repo 可重复，缺省双仓。
3. **bypass 子命令**：`lease bypass --repo <仓> --sha <提交sha> --reason <事由> [--session <会话>]`，落 sih-tools/lease/ledger/bypass.ndjson（逐行 JSON：repo/sha/reason/session/at）。
4. **reconcile 增类**：无模板提交（既非 session/cert 形又非 merge 形）逐笔对 bypass 台账，已登记归 bypass 类不告警，未登记出 `unbypassed` 新类计数。既有 SEAL 逻辑零动。
5. **测试**：纯函数两形合规三形拒（无 session、无 cert、无 merge 前缀各一）加 reconcile bypass 对表加 unbypassed 告警，全部先红后绿留迹；`uv run --project . pytest tests/ -q` 全绿计数入档。
6. **双仓安装**：install-hooks 落 sih-tools 与 sih-engine。**狗粮首绕**：本批自身 lease 源码批的 settle 走 worktree 归并属合规形无需绕；若施工中出现必须直提主树的场景即走 `--no-verify` 加 `lease bypass` 登记，作为首绕实测，事由如实。
7. **CONTRACT 修订**：守卫语义（只拦信息形态不判内容即拦多不拦漏边界如实声明）、bypass 台账、install/uninstall、reconcile 新类、升 1.12.0 三源对齐（pyproject、__init__、CONTRACT）。
8. **BATCH-FACE 两处**：核阅腿命令勘误即 `--pack des-001` 裸名形（现文 `$ROOT/sih-tools/scrutinator/packs/des-001` 引擎件报未知包）；新增直提守卫与 bypass 登记坑位行入坑位速查表。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片：建议 08-on-settle 留痕条与工程基线四对应哲学源 07-on-assay 或 06-on-canon）→ 双门（scrutinator packs/ask3 + ask3repeater --root 绝对）→ elicit（--words 逐词重复）加 digest → 正身 → lease open --package guardhook-solo 双仓绝对路径 → 逐路径取锁（请求写入节全列）→ meter 包裹引擎 scribe intent 上链（--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-02.ndjson 绝对，链现 35 事件 valid 勿动前文）→ 工地施工（lease 源码改动在 worktrees/sih-tools/guardhook-solo 内改与测；BATCH-FACE 同；包档与材料在 engine 工地）→ 管线（化格工具件、核阅引擎件 --pack des-001 裸名、检词 core 包，findings 亲读，域外 exit-2 如实记）→ 认证逐件 meter 包裹 append → 双仓 settle --seq 1 --cert 链上哈希前八位 → 放锁 → close（备份让位归并对表法，diff identical）→ install-hooks 双仓 → reconcile 双仓（保持 unrouted 0 cert_missing 0；本批新增 bypass 或 unbypassed 类如实出数）→ 当日链 verify → 调用册留痕 → 结果档 F 表写全 → 全部产物含链尾随批入版控（settle 前后 wc -l 与末哈希对表留证）。

## inputlog 补录一笔

sih/event/inputlog/2026-09-02.ndjson seq 递增：`批准`（会话号 sess-zcode-260902-acceptor，note 注明即守卫开权令承 adisp-guard-1 裁决）。

## 红线

守卫只拦信息形态不判内容；引擎源码零改动（钩子装 config 不算源码）；历史存量直提笔不回溯；上链前必须等绿；findings 亲读；禁管道掩退出码；正规路径严禁 plain git commit（本批自身就是守卫批，直提即整批失败）；撞锁显式 --session；identity/reports 与既有存量 untracked 零收编（batch-materials、c006-sb3、viewimpl 系、08-30 inputlog、task-packages、sealwin3-solo-materials、adjudisp 无关件、leasepatch/proposition-defense 两 M）。

## 完工报告

意图哈希、守卫测试红转绿迹与计数、双仓安装态证据（git config core.hooksPath 实跑）、狗粮首绕实录或零绕声明、CONTRACT 修订要点、认证清单、双仓 commit 号、链 verify、reconcile 读数含新类、F 表、越线与误差申报。
