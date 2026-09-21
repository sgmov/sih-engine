# run-lifecycle 任务包（删除/恢复/归档 + 复跑谱系）

- 批名：run-lifecycle（程序两 PR：PR1 删除/恢复/轮转归档；PR2 复跑谱系，栈于 PR1）
- 形：串行两波（PR1 单施工簇先行；PR1 收口后 PR2 单施工簇；主线集成验收与闸门）
- 仓：/Users/moc/workspaces/SiHankor/MiniMax-Code-Plugins；分支：community/run-lifecycle（基 origin/main @ 6481e4ae，独立于 #52/#57，可任意序合并）
- 插件：plugins/hetaoBackend/mcode-dynamic-workflows
- 日期：2026-09-21
- 用户需求原文要点：跑完的脚本可删除、删除后可恢复；直接按二期（真回收存储）做；已完成任务可复跑；复跑不得覆盖原数据，须保留多次过程与结果供对比；可复跑无数次。

## 设计契约（两 PR 共守）

### 存储事实（已核实）

- runs(id PK, requestId UNIQUE, requestHash, body)、steps(runId,id,body)、events(seq,runId,body)。
- #48 账本只盖 events 与 repair_cache 两面（integrity_rows + integrity_<surface> 头），runs/steps 不在链上。
- 因此：**events 链永不删行**；runs/steps 的物理压实与账本零冲突。

### PR1：删除/恢复/回收站/轮转归档

1. 墓碑软删：run body 加 `deletedAt`/`deletedBy`/`purgeAfter`；仅终态 run（succeeded/failed/completed_with_gaps/cancelled/interrupted）可删，运行中/待审拒；重复删幂等（返回已是删除态）。事件面追加 `run.deleted`（含来源与时间）。
2. 查询面全过滤：run 列表、workflow_status、**复用候选扫描（#49 面）**、repair 源扫描，均跳过墓碑 run；回收站列表 `GET /api/runs?trash=1`；恢复 = 清墓碑 + 追加 `run.restored`，steps/events/result 原样保留（软删期间未离开）。
3. 保留期：`trashRetentionDays` 设置（store.setting 面，默认 30，0=仅手动轮转）；`purgeAfter=deletedAt+N天`。
4. 轮转压实（真回收存储）：
   - 触发：CLI `--rotate-archive`（main.mjs 旗标面）+ 服务启动时超阈值自动（墓碑 run 数 > 500 或 runs 表体积 > 100MB，阈值常量注释依据）；
   - 动作：到期墓碑 run 的 runs+steps 行导出到旁挂归档 `<data-dir>/archive.db`（同构三表 + rotations 表：rotationId/rotatedAt/manifestHash/runCount/bytes），然后从活库删除这些行；events 一行不动；
   - 审计：追加 `archive.rotated` 事件 {rotationId, runs:[ids], manifestHash, runCount, bytes}（manifestHash=导出行规范序哈希，防归档被篡改的锚）；
   - 验证：账本 verify 面（#48 件）加归档侧核实——integrity 语义不破（events 未动）；归档校验 = 按 manifestHash 重算比对（`--rotate-archive --verify` 与既有 verify 出口共用）。
   - 从归档恢复：`--restore <runId>`（或 API）从 archive.db 导回 runs/steps 行（id 幂等，OR REPLACE）+ 追加 `run.restored` {origin:'archive', rotationId}；墓碑清除。
5. 双面暴露：HTTP `DELETE /api/runs/:id`、`POST /api/runs/:id/restore`、`GET /api/runs?trash=1`；MCP `workflow_delete`/`workflow_restore`（含归档态如实返回）；Studio 回收站视图（列、恢复按钮、保留期剩余）。
6. 测试（checks 新件，真子进程/真库惯例）：删→隐→恢复→全量回（steps/events/result/审计事件）；复用不命中墓碑、恢复后重新候选；删运行中被拒；幂等删；事件链在删/恢复/轮转后仍 valid（复用 #48 verify 面）；轮转后活库行数下降、归档 manifestHash 校验过、篡改归档一行即校验败；归档恢复回活库且可再次删除/轮转；启动阈值自动轮转形态。

### PR2：复跑谱系（栈于 PR1 分支之上）

1. 复跑 = 以原 run 的 script+input 起新 run，**永不改写原 run**。新 run body：`rerunOf`（父 id）、`lineageRoot`（根 id）、`rerunSeq`（根下第 n 次）；requestId 合成唯一（`<orig>#rerun-<n>` 或调用方显式给新 id，避开 UNIQUE 约束与幂等去重面）。
2. 谱系族谱：同 lineageRoot 全体为族；`GET /api/runs/:id/lineage` 返回根 + 全体按 seq 排序（含各员 status/时长/时间戳/result 摘要）；墓碑/归档员如实标注（已删/已归档），不因删除断族。
3. 复跑与复用交互（关键）：**复跑默认 reuseAcrossRuns=false**（否则同上下文直接采纳原 run 成功节点，复跑变假执行、对比失真）；显式传 true 才允许复用（用于省钱重跑场景）。
4. 走既有 pending_review/approve 安全面（复跑执行 agent，与手建工作流同闸）；fail-loud 面不因复跑绕过（PR 独立于 #57，但不得破坏其行为——集成验收含 #57 检查件回归，若主线时 #57 已并则自然回归）。
5. 对比面：MCP `workflow_rerun`（runId，可选 input 覆盖、reuseAcrossRuns 显式旗）；Studio run 面板谱系列表 + 任两员 result 侧边 JSON diff（最小可行：两列只读对比）。
6. 测试：复跑一次→两 run 并存且原 run 字节不变（before/after hash 钉）；连跑 3 次→族谱 4 员有序；复跑默认不复用（真派发证据）/显式旗可复用；删原 run 后族谱标注、恢复后如初；复跑待审闸走通；requestId 合成不撞 UNIQUE。

## 共同不变量

- events 表零物理删除（全程序红线）；runs/steps 物理删仅经轮转且带 manifestHash 审计。
- 假成功禁令延续：删除/恢复/轮转/复跑任一失败不得报成功。
- 既有套件零回归；零新依赖；dist 同步提交；真实退出码验证；单 commit 单主题；不 push。
- 每步施工前亲读 store.mjs/engine.mjs 现状（尤其 #48 integrity 面与 #49 复用查询面）。

## F-锚

- L1 PR1 评审点全绿 + 新检查件全过（本机亲跑）。
- L2 PR2 同强度；族谱对比语义如契约。
- L3 集成后闸门流水线：本机全套件 → siinfer（远程门 + 插件套件）→ fork 预览（windows+codeql）→ 推 PR/PR 栈 → 回评说明。
- L4 结果档 + T6 + sih-engine 提交。

## 检索申报

retriever recall（topic 删除恢复/归档/复跑/谱系，2026-09-21）：待跑，结果随后补录。
