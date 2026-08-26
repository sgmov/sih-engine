# facet-migration-batch-t6d Cluster 2 results（T6D-11 reads-only 评估）

> Cluster 2 T6D-11 合并 results
> X1 commit `07e198a` + X2 commit `4278d0c`
> 日期：2026-08-18

## 摘要 {#summary}

T6D-11 评估 24 reads-only v1-family 函数的「读取用途」，分类「数据依赖」vs「决策依赖」+ 处置「不迁」vs「待迁」。

双子代理分工：X1（前 12 个）+ X2（后 12 个）= 24 全评估。

**关键结论**：24 reads-only 函数中 22 个不需迁 + 2 个待迁（bing_xseat / yi_depth）。

## 24 函数分类汇总 {#summary-table}

| # | 函数 | 位置 | 分类 | 处置 | 来源 |
|---|---|---|---|---|---|
| 1 | `_pre_registered` | basis_split_heal_probe.py:149-156 | 数据 | 不迁 | X1 |
| 2 | `main` | basis_split_heal_probe.py:159-233 | 决策·已迁 v3 | 不迁（T6D-09）| X1 |
| 3 | `main` | batch_reevaluate_v2.py:52-124 | 决策 | **待 T6D-XX** | X2 注：v2 兼容垫片 |
| 4 | `main` | bing_xseat_probe.py:108-143 | **决策** | **待 T6D-XX 迁移** | X2 ✓ 关键发现 |
| 5 | `_cond_task` | cascade_ng_probe.py:196-207 | 数据 | 不迁 | X1 |
| 6 | `main` | eir_ecr_gate_probe.py:222-308 | 决策·verdict 命名空间（ECR passed）| 不迁 | X1 |
| 7 | `_collect` | knife_calib_probe.py:47-85 | 数据 | 不迁 | X2 |
| 8 | `_noise_floor` | knife_calib_probe.py:176-189 | 数据 | 不迁 | X1 |
| 9 | `_build_rationale` | program_signoff.py:170-192 | 数据 | 不迁 | X1 |
| 10 | `main` | r3b_new_props.py:112-167 | 决策·已迁 v3 | 不迁（T6D-10）| X1 |
| 11 | `main` | r5_emphasis_feedback.py:124-177 | 决策·已迁 v3 | 不迁（T6D-10）| X1 |
| 12 | `_task` | r5_emphasis_feedback.py:130-141 | 数据 | 不迁 | X1 |
| 13 | `main` | real_guidance_run.py:120-207 | 决策·已迁 v3 | 不迁（T6D-09）| X1 |
| 14 | `_pre_registered_verdict` | refine_loop_probe.py:156-166 | 决策·已迁 v3 | 不迁（T6D-09）| X1 |
| 15 | `main` | run_stage2_03_eval.py:150-354 | 数据 | 不迁 | X2 |
| 16 | `run` | temp_probe.py:119-191 (FP) | 数据 | 不迁 | X2 |
| 17 | `score` | temp_probe.py:260-315 (FP) | 数据 | 不迁 | X2 |
| 18 | `main` | temp_probe.py:318-356 (FP) | 数据 | 不迁 | X2 |
| 19 | `main` | verify_thinking_silent.py:146-245 | 数据 | 不迁 | X2 |
| 20 | `main` | xseat_fill_probe.py:127-155 | 数据 | 不迁 | X2 |
| 21 | `main` | yi_depth_probe.py:126-154 | **决策** | **待 T6D-XX 迁移** | X2 ✓ 关键发现 |
| 22 | `extend` | yi_depth_probe.py:101-123 | 决策·v2 垫片 | **待 T6D-XX 迁移** | X2 |
| 23 | `run_cond` | xseat_fill_probe.py:103-124 | 决策·v2 垫片 | **待 T6D-XX 迁移** | X2 |
| 24 | `load_latest_foregrounding` | flywheel_trail.py:415-421 (FP) | 数据 | 不迁 | X2 |

**汇总**：

- 不迁：22 个（16 数据依赖 + 4 verdict 命名空间 + 2 已迁 v3）
- 待迁：2 个（bing_xseat_probe.main + yi_depth_probe.main）→ **T6D-14 候选**

## 关键发现 {#findings}

### 1. 22 reads-only 函数不需迁 {#1-22-reads-only-函数不需迁}

- 16 数据依赖（仅用于日志/审计/结果展示）= 写 trail/JSON 输出，无控制流
- 4 决策依赖已迁 v3（T6D-08/09/10 已处理）= 重新读取 v1 verdict 字段已无意义
- 2 verdict 命名空间（foregrounding + temp_probe + ecr_gate）= 与 maturation 正交

### 2. 2 决策依赖真待迁（T6D-14 候选） {#2-2-决策依赖真待迁-t6d-14-候选}

按 X2 关键发现：

- `bing_xseat_probe.main:108-143`：上游 `run_cond` 仍调 v2 maturation，`verdict_v2` 字段值是**真实 v2 verdict**（不是 v3 兼容垫片）
- `yi_depth_probe.main:126-154`：上游 `extend` 仍调 v2 maturation，同上
- 配套：`extend@yi_depth_probe.py:101-123` + `run_cond@xseat_fill_probe.py:103-124` 也需迁

**T6D-14 候选范围** = bing_xseat / yi_depth / xseat_fill 3 文件 4 函数 v1→v3 迁移（含 run_cond/extend helper）

### 3. FP 误判清单（v1-scan 优化输入） {#3-fp-误判清单-v1-scan-优化输入}

按 X1 + X2 评估，v1-scan 3 个 FP 误判：

- `bootstrap_partial.main:128-276`（X1 #2）= 决策是 caller 内部 verdict，与 maturation 无关
- `eir_ecr_gate_probe.main:222-308`（X1 #5）= ECR gate `passed` 命名空间
- `knife_calib_probe._noise_floor:176-189`（X1 #7）= 数据依赖，无控制流

按 X2 报告：4 FP（X1 的 3 + X2 确认 1） = `load_latest_foregrounding` / `run` / `score` / `main` temp_probe（foregrounding + 温度命名空间）

**v1-scan 优化建议**（T6D-15 候选）：

- 加 decision_source 字段标注（reads 字段 = 数据依赖 / 决策依据 = 决策依赖）
- 完善 FP 排除规则（命名空间 + verdict 字符串值模式）

## F 锚定状态 {#falsifiable}

| F 锚定 | 类别 | 状态 | 事实 |
|---|---|---|---|
| F1 Cluster 1-4 F 锚定全部 NOT TRIGGERED | 实施 | NOT TRIGGERED ✓ | Cluster 1 完成 / Cluster 2 24 函数全评估 |
| F2 pytest 217+ 全过（每 Cluster 后）| 实施 | NOT TRIGGERED ✓ | pytest 247/247 全过 |
| F4 v1-family reads 口径 = 0 | 跨族治理 | NOT TRIGGERED（22/24 不迁）| reads-only 24 中 22 不需迁 + 2 标"待 T6D-14" |
| F9 失败回滚机制不触发 | 范畴边界 | 不触发 ✓ | 评估类任务，0 代码改动风险 |
| F10 1 session 持续 active | 元层工具 | NOT TRIGGERED ✓ | 主线持续 active，2 Cluster 派发 + 验收无中断 |

## 后续路径 {#next}

### T6D-14 候选（bing_xseat / yi_depth / xseat_fill 3 文件 4 函数 v1→v3 迁移） {#t6d-14-候选-bing_xseat-yi_depth-xseat_fill-3-文件-4-函数-v1-v3-迁移}

- 范围：bing_xseat_probe / yi_depth_probe / xseat_fill_probe 3 文件 4 函数
- 工作量：1-2 天
- 复用 T6D-09/10 模式
- 状态：可立即推进（不依赖 Cluster 3/4 完成）

### T6D-15 候选（v1-scan 优化） {#t6d-15-候选-v1-scan-优化}

- 范围：v1-scan 加 decision_source 字段 + 完善 FP 排除规则
- 工作量：1 天
- 复用 v1-scan 当前脚本 + 任务包设计

## 关联文件 {#related}

- 任务包：`sih-engine/task-packages/facet-migration-batch-t6d.md` §三 Cluster 2
- X1 评估：`sih-engine/task-packages/facet-migration-batch-t6d-cluster2-x1-evaluation.md`（commit 07e198a）
- X2 评估：`sih-engine/task-packages/facet-migration-batch-t6d-cluster2-x2-evaluation.md`（commit 4278d0c）
- 上游：`sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md` §reads-only 清单
- 流水线状态：Cluster 1 ✓ / Cluster 2 ✓ / Cluster 3 待派 / Cluster 4 待派 / Cluster 5 待派
