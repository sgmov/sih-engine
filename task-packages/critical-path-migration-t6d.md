# critical-path-migration-t6d：critical path 跨族迁移实施

> T6D-08 task-packages 治理任务
> 承接：fix-governance-boundaries-t6d-v1-family-inventory.md §迁移路径优先级 §1. 高（critical path）
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）
> 日期：2026-08-17

## 一、问题陈述 {#problem}

facet 仓 v1-family 严格分类（按 calls）= 15 个函数待迁。其中 3 个是 **critical path**：

1. `build_docket@layer2_signoff.py:77-141`：人复核包组装（治理核心 UI 入口）
2. `submit_signoff@layer2_signoff.py:144-203`：人签核守卫（PRO-07 / PRO-10-c / 不可逆 / PRO-08 四守卫核心）
3. `assess_maturation_v2@maturation_gate.py:201-232`：v2 闸内部（闸工厂）

3 个函数当前都读 `gate.get("verdict")` 字段（v1 字段），决策仍依赖 v1 verdict：与 A4 修复后 `_cmd_route@layer2_signoff.py:229-306` 已基于 v3 决策的现状不一致。

A4 修复的副作用是**critical path 内部 verdict 字段来源分裂**：

- `_cmd_route` 已 v3 决策
- `build_docket` / `submit_signoff` 仍 v1 决策
- 同文件内不同函数读不同 verdict 来源 = 治理核心的内部矛盾

inventory §迁移路径 §1 已立，但实际迁移未实施（v1-family-inventory.md 明确「不实施迁移」）。T6D-08 立文实施第 1 批 3 函数。

## 二、关键设计 {#design}

### 2.1 改法（仿 _cmd_route A4 修复模式） {#2-1-改法-仿-_cmd_route-a4-修复模式}

每个函数都遵循 4 步：

1. 读 v1 gate_assessment 保留 audit（不依赖决策）
2. 读 dc_list（飞轮历史 = v3 闸输入）
3. 调 `mg.assess_maturation_v3(dc_list, ...)` 重算 v3
4. 用 v3 verdict 决策，v1 verdict 保留 audit 字段

### 2.2 build_docket 改法 {#2-2-build_docket-改法}

`layer2_signoff.py:117` `layer1_verdict = gate.get("verdict")` → 改 v3 重算。

docket 输出 payload 改：
- 增 `layer1_verdict_v3` 字段（决策依据）
- 保留 `layer1_verdict` 字段（v1 audit，标 `v1_audit` key）
- `layer1_note` / `layer1_criteria` / `layer1_near_flags` / `layer1_metrics` 来自 v3 重算（v3 闸输出）

### 2.3 submit_signoff 改法 {#2-3-submit_signoff-改法}

`layer2_signoff.py:167-172` PRO-07 路由守卫 → 改 v3 verdict。

守卫链改：
- 增 v3 重算步骤
- `if layer1_verdict == "boundary"` 改 `if layer1_verdict_v3 == "boundary"`
- v1 verdict 仍入 payload（标 `v1_audit` key）供 trail 历史保留
- payload 增 `v3_rule` / `v3_note` / `criteria_version` 字段

### 2.4 assess_maturation_v2 改法 {#2-4-assess_maturation_v2-改法}

`maturation_gate.py:201-232` 当前 v2 闸内部调 v1 maturation。

改：
- 内部调 `assess_maturation_v3`（v3 闸基础）
- v2 verdict 字段改读 v3 verdict 输出
- v1 verdict 字段保留 audit 用途

### 2.5 双源 trail（v1 audit + v3 决策） {#2-5-双源-trail-v1-audit-v3-决策}

3 函数改完后，trail 同时有 v1 字段（审计）和 v3 字段（决策）：

```
payload = {
    "decision": "promote",
    "layer1_verdict": "stable_clear",     # v1 audit（保留 trail 历史）
    "v1_audit": {                         # v1 完整快照供审计
        "verdict": "stable_clear",
        "note": "...",
        "criteria": "...",
    },
    "layer1_verdict_v3": "stable_clear",  # v3 决策（PRO-07 守卫用）
    "v3_rule": "...",
    "v3_note": "...",
    "criteria_version": "v3.x",
    "tier": "...",
    ...
}
```

## 三、工作清单 {#work}

### Cluster 1：并行双子代理 {#cluster-1-并行双子代理}

- **X1**（独立 + 互不依赖）：改 `layer2_signoff.py` 2 函数（build_docket + submit_signoff）
- **X2**（独立 + 互不依赖）：改 `maturation_gate.py` 1 函数（assess_maturation_v2）
- run_in_background=true 并行

每个 X 必须做完：

- [ ] 代码改完
- [ ] 跑 `pytest tests/test_layer2_signoff.py` + `pytest tests/test_maturation_gate.py` 全过
- [ ] 跑 `pytest` 全 217+ 过
- [ ] 写 commit message 含「迁移 v1→v3」
- [ ] commit 进 facet 仓

### Cluster 2：主线串行验证 {#cluster-2-主线串行验证}

- [ ] 跑 `pytest` 全 217+ 过
- [ ] 跑 `audit_pipeline` 验 A4 跨族治理（v1-family 严格分类应减 2：layer2_signoff 2 函数迁完）
- [ ] 跑 `v1-scan` 验 v1-family 清单（严格 calls 应从 15 → 12 / 14 - 2 layer2_signoff + 1 maturation_gate = 12）
- [ ] 跑 `check_three_proposition_audit` 验程序签 + layer2_signoff 文档
- [ ] 写 `task-packages/critical-path-migration-t6d-results.md`
- [ ] 跨仓 commit（sih-tools/facet 仓 + sih-engine 仓 results）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F1** X1 改完 2 函数 + X2 改完 1 函数 | 实施 | git diff 显示改 + 单元测试过 |
| **F2** pytest 217+ 全过 | 实施 | `pytest` 退出码 0 |
| **F3** audit_pipeline A4 v1-family 严格 ≤ 13 | 跨族治理 | `check_verdict_consistency` 报告严格 v1-family ≤ 13（15-2 layer2_signoff）|
| **F4** v1-scan 严格 v1-family ≤ 12 | 跨族治理 | `fix-governance-boundaries-t6d-v1-scan.py` 严格 calls ≤ 12（15-2-1）|
| **F5** PRO-07 守卫测试全过 | 实施 | `pytest tests/test_layer2_signoff.py` PRO-07 路由守卫全过 |
| **F6** PRO-10-c 守卫测试全过 | 实施 | `pytest tests/test_layer2_signoff.py` PRO-10-c tier 守卫全过 |
| **F7** 不可逆守卫 + PRO-08 守卫全过 | 实施 | `pytest tests/test_layer2_signoff.py` 不可逆 + PRO-08 全过 |
| **F8** v3 字段输出到 trail | 实施 | 模拟测试写 layer2_signoff 后读 trail，含 `layer1_verdict_v3` / `v3_rule` / `v3_note` 字段 |
| **F9** 双源 trail 保留 v1 audit | 实施 | trail payload 含 `v1_audit` 子对象（v1 完整快照）|
| **F10** 失败 F 锚定触发 = 回滚 | 范畴边界 | F1-F9 任一失败，主线停止 + 写 results 文档记失败根因 + git revert |

**F1-F9 任意触发** = 任务失败

**F10 失败处理**：不重试，立刻回滚到 44e8d0c 状态 + 写 results 文档诚实记录失败根因

## 五、必读文件 {#read}

双子代理必读：

- `sih-tools/facet/ROADMAP.md` §P5 §P6 上下文
- `sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md` §迁移路径优先级 §1. 高
- `sih-engine/task-packages/fix-failures-t6d-results.md` §A4 修复模式（commit 44e8d0c）
- `sih-tools/facet/probes/layer2_signoff.py`（line 77-141 + 144-203 + 229-306 _cmd_route 范例）
- `sih-tools/facet/probes/maturation_gate.py`（line 201-232 + 317-507 assess_maturation_v3 范例）
- `sih-tools/facet/tests/test_layer2_signoff.py`（现有 PRO-07/10-c/不可逆/PRO-08 守卫测试）
- `sih-tools/facet/tests/test_maturation_gate.py`（现有 v2/v3 闸测试）

主线必读：

- `sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md` §风险点
- `sih-engine/skills/sihankor-proposition-defense/probes/check_verdict_consistency.py`（v1 分类判定）

## 六、约束 {#constraints}

1. **零 LLM 调用**（机械脚本层）
2. **不删 v1 verdict 字段**（保留 audit 用途，标 `v1_audit` key）
3. **PRO-07 / PRO-10-c / 不可逆 / PRO-08 四守卫必须全过**（F5-F7 强制）
4. **不改 A4 修复**（`_cmd_route` 已 v3 决策，不动）
5. **不改 v3 闸基础**（`assess_maturation_v3` 不动）
6. **不改其他 12 个 v1-family 函数**（中优先级 8 个 + 低优先级 4 个，留 T6D-09+）
7. **F 锚定触发 = 失败回滚**（F10 强制）
8. **commit 粒度**：X1 1 commit（layer2_signoff 2 函数）/ X2 1 commit（maturation_gate 1 函数）
9. **commit message 引用本任务包 + inventory §迁移路径 §1**

## 七、验收标准 {#acceptance}

本任务包验收 = 7 项 Cluster 1 + 5 项 Cluster 2 + 1 项 F 锚定 10/10：

### Cluster 1（双子代理实施） {#cluster-1-双子代理实施}

- [ ] X1 改 layer2_signoff 2 函数 + commit
- [ ] X2 改 maturation_gate 1 函数 + commit
- [ ] X1 X2 各自跑 pytest 全过
- [ ] X1 X2 各自 commit message 含「迁移 v1→v3 + inventory §1」
- [ ] X1 X2 各自 commit hash 回报给主线
- [ ] X1 X2 各自跑 commit hook（B1 默认 WARN）
- [ ] 3 commit 全进 facet 仓

### Cluster 2（主线串行验证） {#cluster-2-主线串行验证}

- [ ] 跑 pytest 217+ 全过
- [ ] 跑 audit_pipeline 验 A4 v1-family 严格 ≤ 13
- [ ] 跑 v1-scan 验严格 calls ≤ 12
- [ ] 写 results 文档
- [ ] 跨仓 commit

### F 锚定 10/10 {#f-锚定-10-10}

- F1-F9 全部 NOT TRIGGERED
- F10 失败回滚机制就位

## 八、风险点 {#risks}

### 风险 1：layer2_signoff 是 critical 路径（高风险） {#风险-1-layer2_signoff-是-critical-路径-高风险}

任何改动需测试 PRO-07 路由守卫（boundary 不进 Layer 2）+ PRO-10-c tier 守卫（核心公理 facet 不碰）+ 不可逆守卫（promote 须 rationale）+ PRO-08 留痕守卫（F5-F7 强制）。

**缓解**：X1 改完先跑 `test_layer2_signoff.py` 单元测试，全过才 commit。F5-F7 触发 = 任务失败。

### 风险 2：maturation_gate 是闸工厂（高风险） {#风险-2-maturation_gate-是闸工厂-高风险}

v2 改 v3 影响所有 v2 caller：需先跑 `batch_reevaluate_v3*` 验证 v3 闸稳定性。

**缓解**：X2 改完先跑 `test_maturation_gate.py` + `test_batch_reevaluate_v3*.py`，全过才 commit。F2 触发 = 任务失败。

### 风险 3：v1 audit + v3 决策双源 trail 增加 payload 大小 {#风险-3-v1-audit-v3-决策双源-trail-增加-payload-大小}

docket + signoff payload 增 ~30% 字段（v1_audit + v3_xxx）。

**缓解**：接受成本（trail 持久化是核心要求，工程基线四）。F8 强制 payload 含 v3 字段。

### 风险 4：双子代理实施可能引入 cross-coupling {#风险-4-双子代理实施可能引入-cross-coupling}

X1 改 layer2_signoff 用 `assess_maturation_v3`（与 maturation_gate 共享），X2 改 maturation_gate 是 v3 闸基础：两边可能引入签名不一致。

**缓解**：X1 X2 各自独立 commit 后，主线跑 pytest 全套验 cross-coupling（217+ 全过 + audit_pipeline 全跑）。F1-F2 触发 = 回滚。

### 风险 5：失败回滚成本高 {#风险-5-失败回滚成本高}

10 F 锚定任一触发 = 回滚到 44e8d0c 状态 = 3 commit revert + 文档修正。

**缓解**：双子代理各自 commit 前先跑单元测试（不让 F 锚定积累）。F10 失败回滚机制就位。

## 九、关联文件 {#related}

- 上游：sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md §迁移路径 §1. 高
- 上游：sih-engine/task-packages/fix-failures-t6d-results.md §A4 修复模式
- 现状：sih-tools/facet/probes/layer2_signoff.py:117, 167, 172（v1 verdict 读）
- 现状：sih-tools/facet/probes/maturation_gate.py:201-232（v2 闸）
- 范例：sih-tools/facet/probes/layer2_signoff.py:229-306 `_cmd_route` A4 修复
- 工具：sih-engine/skills/sihankor-proposition-defense/probes/check_verdict_consistency.py
- 工具：sih-engine/task-packages/fix-governance-boundaries-t6d-v1-scan.py
- 工具：sih-engine/skills/sihankor-proposition-defense/probes/audit_pipeline.py
- 跨仓：sih-tools/facet/ROADMAP.md §P5
- 范式文档：sih-engine/task-packages/README.md
