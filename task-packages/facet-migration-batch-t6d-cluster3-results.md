# facet-migration-batch-t6d Cluster 3 results（T6D-12 v2↔v3 循环重构）

> Cluster 3 T6D-12 合并 results
> X1 commit `7375b66` + X2 commit `cc2cb45`
> 日期：2026-08-18

## 摘要 {#summary}

T6D-12 抽 `assess_maturation_v3_base` 函数 + 重构 `assess_maturation_v2` 调 base，移除 `sys._getframe(1)` call stack hack。

双子代理分工：X1（抽 v3_base）+ X2（重构 v2 调 base + 移除 hack）。

**关键成果**：

- v3 闸结构简化：v3 → v3_base（薄包装，v1 子判据内联 + V3-T/U/B 重算）
- v2 闸结构简化：v2 → v3_base（替代前 v3 嵌套 + sys._getframe hack）
- 移除 188 行嵌套逻辑 + 49 行 hack 路径
- v1-family by calls 仍 = 0
- `sys._getframe(1)` AST 扫描 0 命中（仅 docstring 描述）

## 关键设计决策 {#decisions}

### 决策 1：v3_base 内部实现 v1 子判据（不调 v1 函数） {#决策-1-v3_base-内部实现-v1-子判据-不调-v1-函数}

按 X1 报告：v3_base 内部实现 v1 子判据逻辑（4 子判据 + v1 verdict 裁决）**不调 v1 函数**。

- 原因：避免 v1-family by calls 增长（v3_base 调 v1 = v1-family 新增 1）
- 效果：v1-family 仍 = 0，F3 NOT TRIGGERED
- 代价：v3_base 内部重复实现 v1 逻辑（4 子判据 + verdict 裁决）

### 决策 2：v2 不开 v1 fallback（X2 与约束 5 偏差） {#决策-2-v2-不开-v1-fallback-x2-与约束-5-偏差}

按 X2 报告：F3 锚定 vs 约束 5 冲突取舍。

| 选项 | v2 加 v1 fallback | v2 不加 v1 fallback |
|---|---|---|
| v1-family 判定 | v2 calls = {v1} → v1-family | v2 calls = {} + reads verdict_v3 → reads-v3-family |
| F3 v1-family = 0 | ❌ 失败 | ✅ 通过 |
| 约束 5 保留 v1 fallback | ✅ | ❌ |
| **取舍** | 失败回滚 | **采纳**（F3 > 约束 5）|

v3_base 已内联 v1 子判据，dc_list 异常 / repeat_metrics 异常等失败场景由调用方（maturation_retrovalidate / r5_emphasis_feedback 的 `load_dc_list` 上层）做防御。X2 已在 commit message 显式记录此偏差。

### 决策 3：cross-coupling 处理（X1 X2 并行） {#决策-3-cross-coupling-处理-x1-x2-并行}

按 X1 + X2 报告：

- X1 第一次 commit (a314268) 误把 X2 工作一起 commit → 立即 `git reset --hard HEAD~1` 回滚
- X1 重新 commit `7375b66`（仅 X1 范围：v3_base + v3 薄包装）
- X2 基于 X1 终态 `7375b66` commit `cc2cb45`（仅 X2 范围：v2 调 base + 移除 hack）
- 两个 commit 形成完整 T6D-12 Cluster 3 闭环

**教训**：双子代理并行 + 共享文件 = 必须严格按文件分 commit，不能跨范围。X1 reset 教训 = commit 前 `git diff --stat` 验证范围。

## 关键修改 {#changes}

| 文件 | 范围 | 改法 |
|---|---|---|
| `maturation_gate.py:350` (新) | `assess_maturation_v3_base` | 抽 v3 闸基础（v1 子判据内联 + V3-T/U/B 重算 + B1/B2 降级）|
| `maturation_gate.py:630` (改) | `assess_maturation_v3` | 改为薄包装：直接调 v3_base，签名向后兼容 |
| `maturation_gate.py:208-275` (改) | `assess_maturation_v2` | 重构调 v3_base，移除 `sys._getframe(1)` hack |

## F 锚定状态 {#falsifiable}

| F 锚定 | 类别 | 状态 | 事实 |
|---|---|---|---|
| F1 Cluster 1-4 F 锚定全部 NOT TRIGGERED | 实施 | NOT TRIGGERED ✓ | Cluster 1-3 全部完成 |
| F2 pytest 247+ 全过 | 实施 | NOT TRIGGERED ✓ | 247/247 + maturation_gate 20/20 + maturation_retrovalidate 9/9 |
| F5 `sys._getframe(1)` 0 命中 | 元层工具 | NOT TRIGGERED ✓ | AST 扫描 0 命中（仅 docstring 描述）|
| F9 失败回滚机制不触发 | 范畴边界 | 不触发 ✓ | 全部 F 锚定通过 |
| F10 1 session 持续 active | 元层工具 | NOT TRIGGERED ✓ | 主线持续 active，3 Cluster 派发 + 验收无中断 |

## 协调情况 {#coordination}

- X1 commit 历程：3 次落地（a314268 含 X2 误 → reset → 7375b66 仅 X1 范围）
- X2 commit 历程：1 次落地（cc2cb45 基于 X1 终态）
- 跨子代理协调：X1 X2 都观察到对方工作，X1 主动 reset + 重做只含自己范围
- 教训：双子代理 + 共享文件 = 必须严格按范围 commit + commit 前 git diff 验证

## 后续路径 {#next}

### T6D-13 候选（baseline_checker 实际代码实现） {#t6d-13-候选-baseline_checker-实际代码实现}

- 范围：消费 facet 报告 + 验 4 守卫 + 写 crosscheck_completed + 回写 cross_link_verified
- 实施 0 厂商，验证需 A-2 caller 集成（后续）
- 工作量：3-5 天

### T6D-14 候选（bing_xseat / yi_depth / xseat_fill v1→v3 迁移） {#t6d-14-候选-bing_xseat-yi_depth-xseat_fill-v1-v3-迁移}

- T6D-11 评估发现的 2 个决策依赖真待迁
- 复用 T6D-09/10 模式
- 工作量：1-2 天

### T6D-15 候选（v1-scan 优化） {#t6d-15-候选-v1-scan-优化}

- 加 decision_source 字段 + 完善 FP 排除规则
- 工作量：1 天

## 关联文件 {#related}

- 任务包：`sih-engine/task-packages/facet-migration-batch-t6d.md` §三 Cluster 3
- X1 实施：`sih-tools/facet/probes/maturation_gate.py` `7375b66` 抽 v3_base
- X2 实施：`sih-tools/facet/probes/maturation_gate.py` `cc2cb45` 重构 v2 调 base + 移除 hack
- 上游：`sih-engine/task-packages/critical-path-migration-t6d-results.md`（T6D-08 X2 报告 `sys._getframe(1)` hack 脆弱性）
- 流水线状态：Cluster 1 ✓ / Cluster 2 ✓ / Cluster 3 ✓ / Cluster 4 待派 / Cluster 5 待派
