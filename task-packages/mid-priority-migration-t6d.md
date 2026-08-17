# mid-priority-migration-t6d：中优先级 v1-family 跨族迁移

> T6D-09 task-packages 治理任务
> 承接：fix-governance-boundaries-t6d-v1-family-inventory.md §迁移路径优先级 §2. 中（重要但非 critical）
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）
> 日期：2026-08-17
> 上游：T6D-08 critical-path-migration-t6d 模式复用

## 一、问题陈述 {#problem}

T6D-08 完成后 v1-family 严格 calls 15 → 12。剩 12 个里：

- **中优先级 8 个**（inventory §2）：cascade_ng_probe / basis_split_heal_probe / flywheel_microcircuit_probe / lightweight_mode_probe / real_guidance_run / refine_loop_probe / reflexive_signing_probe 的 main 函数
- **低优先级 4 个**（inventory §3，留 T6D-10）：r3a_gate_v2 / r3b_new_props / r5_emphasis_feedback 的历史实验函数
- **额外 1 个**（v2 闸但 calls v1）：maturation_retrovalidate（Kimi 时代回溯）

T6D-09 范围 = 中优先级 8 个 + maturation_retrovalidate 1 个 = **9 函数**。本次迁完 v1-family 严格 calls 应从 12 → 3（剩 4 个 R3a/R3b/R5 留 T6D-10）。

中优先级是**探针类**（实验 / 分析用），非 critical path：风险低于 T6D-08，但每个探针有独立测试 + 独立数据流，需要分别验证。

## 二、关键设计 {#design}

### 2.1 改法（沿用 T6D-08 仿 _cmd_route 模式） {#2-1-改法-沿用-t6d-08-仿-_cmd_route-模式}

每个 main 函数 4 步：

1. 读 v1 `gate_assessment` 保留 audit
2. 读 `dc_list`（飞轮历史 = v3 闸输入）
3. 调 `mg.assess_maturation_v3(dc_list, ...)` 重算 v3
4. 用 v3 verdict 决策，v1 verdict 保留 `v1_audit` key

### 2.2 9 函数清单 {#2-2-9-函数清单}

| # | 函数 | 文件 | 风险 |
|---|---|---|---|
| 1 | `main@cascade_ng_probe.py:188-269` | cascade_ng_probe.py | 中（实验探针）|
| 2 | `run_condition@cascade_ng_probe.py:158-185` | cascade_ng_probe.py | 中（实验探针）|
| 3 | `main@basis_split_heal_probe.py:159-233` | basis_split_heal_probe.py | 中（实验探针）|
| 4 | `main@flywheel_microcircuit_probe.py:126-217` | flywheel_microcircuit_probe.py | 中（实验探针）|
| 5 | `main@lightweight_mode_probe.py:363-498` | lightweight_mode_probe.py | 中（实验探针）|
| 6 | `main@real_guidance_run.py:120-207` | real_guidance_run.py | 中（实验探针）|
| 7 | `main@refine_loop_probe.py:169-287` | refine_loop_probe.py | 中（实验探针）|
| 8 | `main@reflexive_signing_probe.py:103-169` | reflexive_signing_probe.py | 中（实验探针）|
| 9 | `main@maturation_retrovalidate.py:50-99` | maturation_retrovalidate.py | 中（Kimi 时代回溯）|

### 2.3 双子代理分工（按文件独立） {#2-3-双子代理分工-按文件独立}

- **X1**（4 函数 + 2 文件）：cascade_ng_probe.py（2 函数）+ basis_split_heal_probe.py（1 函数）+ flywheel_microcircuit_probe.py（1 函数）+ lightweight_mode_probe.py（1 函数）= 5 函数 / 4 文件
- **X2**（4 函数 + 4 文件）：real_guidance_run.py（1 函数）+ refine_loop_probe.py（1 函数）+ reflexive_signing_probe.py（1 函数）+ maturation_retrovalidate.py（1 函数）= 4 函数 / 4 文件

按文件分 = 每个 X 独立 commit，互不依赖。

### 2.4 探针类特殊处理 {#2-4-探针类特殊处理}

与 T6D-08 critical path 不同，探针类：

- **不守卫 PRO-07 / PRO-10-c / 不可逆 / PRO-08**（探针只读不写 trail，无守卫链）
- **输出是实验数据 + 报告**（不是裁决事件）
- **失败影响 = 实验数据偏差，不是治理崩溃**：风险低

但仍要：

- 跑每个探针的测试（`test_<probe>.py`）
- 验证 verdict 字段来源迁移（v1 → v3）
- 验证 v1 audit key 保留

## 三、工作清单 {#work}

### Cluster 1：并行双子代理 {#cluster-1-并行双子代理}

- **X1**（5 函数 / 4 文件）：cascade_ng_probe + basis_split_heal + flywheel_microcircuit + lightweight_mode
- **X2**（4 函数 / 4 文件）：real_guidance_run + refine_loop + reflexive_signing + maturation_retrovalidate
- run_in_background=true 并行

每个 X 必须做完：

- [ ] 改完 5/4 个 main 函数
- [ ] 跑 `pytest tests/test_<probe>.py` 全过（每个探针）
- [ ] 跑 `pytest` 全 217+ 过
- [ ] 跑 `python3 probes/<probe>.py --help` 验 CLI 不破
- [ ] 写 commit message 含「T6D-09 探针 v1→v3 迁移」
- [ ] commit 进 facet 仓

### Cluster 2：主线串行验证 {#cluster-2-主线串行验证}

- [ ] 跑 `pytest` 全 217+ 过
- [ ] 跑 `v1-scan` 验严格 calls ≤ 3（12 - 9 = 3，剩 4 个 R3a/R3b/R5 留 T6D-10）
- [ ] 跑 `check_verdict_consistency` 验 reads 口径下降（探针 reads v1 verdict 字段会减少）
- [ ] 写 `task-packages/mid-priority-migration-t6d-results.md`
- [ ] 跨仓 commit（sih-tools/facet 仓 + sih-engine 仓 results）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F1** X1 改完 5 函数 + X2 改完 4 函数 | 实施 | git diff 显示 9 函数改 + 单元测试过 |
| **F2** pytest 217+ 全过 | 实施 | `pytest` 退出码 0 |
| **F3** v1-scan 严格 calls ≤ 3 | 跨族治理 | `fix-governance-boundaries-t6d-v1-scan.py` 严格 calls ≤ 3（12-9）|
| **F4** 9 探针测试全过 | 实施 | 9 个 `test_<probe>.py` 全过 |
| **F5** CLI 不破 | 实施 | `python3 probes/<probe>.py --help` 退出码 0 |
| **F6** v3 字段输出到 trail | 实施 | 模拟测试 + 探针输出含 v3 verdict 字段 |
| **F7** 双源 trail 保留 v1 audit | 实施 | 输出含 v1_audit key 或保留 v1 verdict 字段 |
| **F8** cross-coupling 验证 | 元层工具 | 主线跑 audit_pipeline 验 9 函数改完不破坏 v3 闸基础 |
| **F9** 失败 F 锚定触发 = 回滚 | 范畴边界 | F1-F8 任一失败，主线停止 + 写 fail 文档 + git revert |

**F1-F8 任意触发** = 任务失败

**F9 失败处理**：不重试，立刻回滚到 T6D-08 commit 状态 + 写 results 文档诚实记录失败根因

## 五、必读文件 {#read}

双子代理必读：

- `sih-engine/task-packages/critical-path-migration-t6d.md`（T6D-08 任务包，复用模式）
- `sih-engine/task-packages/critical-path-migration-t6d-results.md`（T6D-08 results，看 4 步模式 + 关键设计）
- `sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md` §迁移路径 §2. 中
- `sih-tools/facet/probes/layer2_signoff.py:229-306` _cmd_route（A4 修复范例）
- `sih-tools/facet/probes/maturation_gate.py:201-265` assess_maturation_v2（X2 改范例）
- `sih-tools/facet/probes/maturation_gate.py:317-507` assess_maturation_v3（v3 闸基础）
- 各探针对应 test 文件 `tests/test_<probe>.py`

主线必读：

- `sih-engine/task-packages/fix-failures-t6d-results.md` §教训 1（F 锚定要分类型）
- `sih-engine/task-packages/critical-path-migration-t6d-results.md` §四 关键设计（双源 trail）

## 六、约束 {#constraints}

1. **零 LLM 调用**（机械代码修改）
2. **不删 v1 verdict 字段**（保留 audit 用途，标 `v1_audit` key 或保留原字段）
3. **不改 _cmd_route / assess_maturation_v2 / assess_maturation_v3**（T6D-08 + A4 已修）
4. **不改其他 3 个低优先级函数**（r3a_gate_v2 / r3b_new_props / r5_emphasis_feedback，留 T6D-10）
5. **不改 24 个 reads-only 函数**（不决策，迁完自然下降）
6. **commit 粒度**：每个 X 每文件 1 commit（X1 = 4 commits / X2 = 4 commits = 8 commits total）
7. **F 锚定触发 = 失败回滚**（F9 强制）
8. **不改 PRO-07 / PRO-10-c / 不可逆 / PRO-08 守卫**（探针不涉及，迁移探针不引入新守卫）

## 七、验收标准 {#acceptance}

本任务包验收 = 9 项 Cluster 1 + 5 项 Cluster 2 + 9/9 F 锚定：

### Cluster 1（双子代理实施） {#cluster-1-双子代理实施}

- [ ] X1 改 5 函数 + 4 commit
- [ ] X2 改 4 函数 + 4 commit
- [ ] X1 X2 各自跑 pytest 全过
- [ ] X1 X2 各自 commit message 含「T6D-09 探针 v1→v3 迁移 + inventory §2」
- [ ] X1 X2 各自 commit hash 回报给主线
- [ ] X1 X2 各自跑 commit hook（B1 默认 WARN）
- [ ] 8 commit 全进 facet 仓

### Cluster 2（主线串行验证） {#cluster-2-主线串行验证}

- [ ] 跑 pytest 217+ 全过
- [ ] 跑 v1-scan 验严格 calls ≤ 3
- [ ] 跑 check_verdict_consistency 验 reads 下降
- [ ] 写 results 文档
- [ ] 跨仓 commit

### F 锚定 9/9 {#f-锚定-9-9}

- F1-F8 全部 NOT TRIGGERED
- F9 失败回滚机制就位

## 八、风险点 {#risks}

### 风险 1：探针独立测试多（中风险） {#风险-1-探针独立测试多-中风险}

9 个探针每个有独立 test 文件 + 独立数据流 + 独立 CLI：9 处都要验。

**缓解**：X1 X2 各自 commit 前先跑对应探针的 `test_<probe>.py` + `--help` CLI 验。F4 F5 触发 = 任务失败。

### 风险 2：v1 audit key 在探针里可能不通用 {#风险-2-v1-audit-key-在探针里可能不通用}

T6D-08 critical path 的 v1 audit key 是「裁决事件 payload 字段」。探针的 v1 audit key 可能是「实验数据字段」：两种语义不同。

**缓解**：探针 v1 audit 不强制用 `v1_audit` key，可保留原 v1 verdict 字段 + 加 v3 verdict 字段 = 双字段共存。F7 强制双源保留。

### 风险 3：maturation_retrovalidate 是 Kimi 时代回溯（历史） {#风险-3-maturation_retrovalidate-是-kimi-时代回溯-历史}

`maturation_retrovalidate.py:50-99` 是 Kimi 时代（早期）回溯脚本：v3 闸可能未覆盖所有历史 case。

**缓解**：保留 v1 verdict fallback（dc_list 空时用 v1），与 critical path 一致。

### 风险 4：cross-coupling（9 函数 vs v3 闸基础） {#风险-4-cross-coupling-9-函数-vs-v3-闸基础}

9 函数改完都调 `assess_maturation_v3`：v3 闸基础稳定性是关键。T6D-08 X2 已证 v3 基础稳，但 T6D-09 9 函数批量调用需再验。

**缓解**：主线 Cluster 2 跑 audit_pipeline 验 cross-coupling（F8 强制）。

### 风险 5：失败回滚成本高（8 commit + 1 task = 9 commit） {#风险-5-失败回滚成本高-8-commit-1-task-9-commit}

F1-F8 任一触发 = 9 commit revert + 文档修正。

**缓解**：X1 X2 各自 commit 前先跑单元测试（不让 F 锚定积累）。F9 强制。

## 九、关联文件 {#related}

- 上游：`sih-engine/task-packages/critical-path-migration-t6d.md`（T6D-08 模式）
- 上游：`sih-engine/task-packages/critical-path-migration-t6d-results.md`（T6D-08 results + 4 步模式）
- 上游：`sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md` §迁移路径 §2. 中
- 现状：sih-tools/facet/probes/cascade_ng_probe.py / basis_split_heal_probe.py / flywheel_microcircuit_probe.py / lightweight_mode_probe.py / real_guidance_run.py / refine_loop_probe.py / reflexive_signing_probe.py / maturation_retrovalidate.py
- 工具：`sih-engine/skills/sihankor-proposition-defense/probes/check_verdict_consistency.py`
- 工具：`sih-engine/task-packages/fix-governance-boundaries-t6d-v1-scan.py`
- 跨仓：sih-tools/facet/ROADMAP.md §P5
- 范式文档：`sih-engine/task-packages/README.md`
