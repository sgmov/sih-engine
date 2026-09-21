# run-lifecycle 批结果档

- 批链：run-lifecycle（任务包 run-lifecycle.md），串行两簇施工（PR1、PR2），主线集成验收
- 仓：/Users/moc/workspaces/SiHankor/MiniMax-Code-Plugins；分支 community/run-lifecycle（PR1）与 community/rerun-lineage（PR2，栈于 PR1）
- 基线：origin/main @ 6481e4ae（含 #48 账本）；独立于 #52/#57
- 终头：PR1 af2d187；PR2 c421aaa（含两枚 win32 清理修复 cherry-pick，内容与 #57 的 14a7fd5/e1c976b 逐字节同源，git 同内容变更可自动收敛，任一先并另一免冲突）
- PR：https://github.com/MiniMax-AI/MiniMax-Code-Plugins/pull/58 与 /pull/59
- 日期：2026-09-21

## 提交清单

| 分支 | commit | 内容 |
|---|---|---|
| run-lifecycle | dcee394 | 墓碑软删/回收站/恢复/保留期/三面暴露（HTTP+MCP+Studio）+ trash 检查件 8 例 |
| run-lifecycle | af2d187 | 轮转压实/旁挂归档/manifestHash 校验/归档恢复/启动阈值 + archive-rotate 检查件 5 例 |
| rerun-lineage | 5633c91 | 复跑谱系（rerunOf/lineageRoot/rerunSeq、族谱 API、MCP workflow_rerun、Studio 对比面）+ 检查件 7 例 |
| rerun-lineage | 22def56/c421aaa | cross-reuse-mcp win32 清理两修复（同源 #57，main 上该检查件带病未修） |

## 设计落地要点

- events 链零物理删除红线守住：#48 账本只盖 events/repair，runs/steps 压实与链零冲突；archive.rotated 事件携 manifestHash 锚归档完整性；三路篡改（改归档体/伪造 rotations 哈希/删 rotations 记录）fail-closed。
- 复跑不改写原 run：字节级 before/after 钉死；requestId 合成避 UNIQUE 与幂等去重面，撞形后缀重试；复跑默认 reuseAcrossRuns=false（防假执行保对比真实性），显式旗可复用。
- 施工裁量记录：归档表 (rotationId,runId) 键控防再删再轮转改写旧行致旧 manifest 假败；needs_attention 不入可删集；retention=0 仅手动轮转；复跑机制面不限源状态（安全由 pending_review 闸承载）；谱系时长取自事件账本（归档员同有效）。

## 闸门矩阵

| 闸门 | PR1 (af2d187) | PR2 栈头 (c421aaa) |
|---|---|---|
| 本机 npm test | 130/130（含 13 新例独立亲跑） | 137/137（含 7 新例独立亲跑） |
| test:package / build / dist 幂等 / 树净 | 全绿 | 全绿 |
| siinfer 根门 | 绿 | 绿 |
| fork 预览 windows+codeql | —（栈头覆盖） | 绿（35602861103；首跑暴露 main 上 cross-reuse-mcp win32 清理病，同源修复后绿） |

## 检索申报

3 命中（T6 范式谱系、legacy 归档档、归档零损先例意图档），无冲突约束，存档 recall/run-lifecycle-recall-20260921.md。

## 追加：canvas-fullscreen（PR #60，用户需求驱动）

- 分支 community/canvas-fullscreen（基 55b8f48），commit b0b8849。
- 结构拓扑区前加全屏钮：Fullscreen API 优先、拒绝/受限落 CSS 覆盖层；进出双沿 zoom 自适应重算；数据刷交互态保（全屏态跨轮询/切 run 保持，空态释放）；Esc 对话框优先路由；i18n 双语。
- 状态机提纯 fullscreen-model.mjs + checks/canvas-fullscreen.check.mjs 6 例。
- egolite 交互实测：按钮位、进全屏（覆盖层 1426×751）、6 秒轮询保持、按钮翻转为退出、Esc 回内联（1128×327），全过。
- 闸门：npm test 148/148 亲跑；siinfer 绿（b0b88490）；fork 预览 35667369312 windows+codeql 双绿；PR https://github.com/MiniMax-AI/MiniMax-Code-Plugins/pull/60。
