# 任务包：fix-governance-boundaries-t6d（A+B 治理边界修复）

> 范式：T6-D（双子代理 + 主线串行验证）
> 立项原因（2026-08-17）：fix-failures-t6d 5 修复代码层完成，但 4 F 锚定 TRIGGERED 暴露 3 类治理边界：
> - **A 数据治理**（A1 数据 73.17% > 60% 不能代码改）
> - **A 跨族治理**（A4 _cmd_route 移 v3-family 但 45+ 其他 v1-family 函数未改）
> - **B 元层工具更新**（A5 audit 脚本不读 DEC 文档）
>
> 本任务包（A+B）落地这 3 类治理边界，让 audit_pipeline 真正 5 全 PASS。

## 一、问题陈述 {#problem}

T6D-02 fix-failures-t6d 完成 5 修复代码层，但 audit_pipeline 仍 1 PASS / 4 FAIL（数量未变，根因已转）：

| F 锚定 | 治理范畴 | 修复路径 |
|---|---|---|
| F1.1.1 A1 数据 73.17% > 60% | **A 数据治理** | 新机制接真实流量让占比下降 |
| F3.1.1 A3 剩 2 pre-existing 错位 | 元层 + 测试 | 修 test_flywheel_trail + test_check_verdict_consistency |
| F4.1.1 A4 45+ 跨族函数 | **A 跨族治理** | 多文件 v1 → v3 迁移 |
| F5.1.1 A5 audit 不读 DEC | **B 元层工具更新** | 改 `check_decision_authority.py` 读 DEC 文档 + schema 字段 |

**目标**：A+B 完成后 audit_pipeline 5 全 PASS。

## 二、关键设计 {#design}

### 2.1 范围切分（A vs B）

**A = 跨族治理 + 数据治理**（大工程，多文件，跨多版本）

- A-1 跨族治理：识别 45+ v1-family 函数 + 设计迁移路径 + 实施第一阶段
- A-2 数据治理：让 .agents/skills/ 关键路径走 facet pipeline（制造真实 program_signoff / knife_edge_risk / supersession 事件）

**B = 元层工具更新**（小工程，单文件，立竿见影）

- B-1 改 `check_decision_authority.py` 读 DEC 文档 + schema 字段
- B-2 让 audit A5 能基于"实际代码 + DEC 文档"判定 cross-link gap

### 2.2 T6-D 编排（双子代理 + 主线）

**Cluster 1：双子代理并行（X1 写 B / X2 写 A-1 诊断）**

- X1：写 B（audit 脚本更新，单文件，1-2 小时）
- X2：写 A-1（v1-family 函数识别 + 迁移路径设计，**不实施**——只诊断）
- 主线：写 A-2 数据治理路径（caller 设计，让 .agents/skills/ 关键路径走 facet）

**Cluster 2：主线验收集齐**

- 跑 audit_pipeline.py 验 5 PASS（A2 路径 + B 路径）
- 跑 pytest 验 217+ 全过
- 主线 commit

### 2.3 B 优先级

B 立即能跑出 A5 PASS（脚本改完即可）——X1 完成后立刻见效。

A-1 跨族治理是大工程（45+ 函数迁移）——本任务包只做"诊断 + 路径设计"，实际迁移留 T6D-04。

A-2 数据治理是"工具设计 + 流程改造"——本任务包做 caller 设计，实际跑流量留阶段 2.5。

## 三、工作清单 {#work}

### Cluster 1

#### B-1. audit_pipeline A5 脚本读 DEC（X1）

**位置**：`sih-engine/skills/sihankor-proposition-defense/probes/check_decision_authority.py`

**当前行为**：A5 静态提取 4 路径 + 硬编码"3 cross-link gap"（UNIQUE_BUT_BLIND）

**目标行为**：A5 改读：
1. `sih-engine/doc/design/DES-011-baseline-checker-DEC.md` 解析 cross-link 协议
2. `sih-tools/facet/probes/flywheel_trail.py:record_program_signoff` 检查 `sih_engine_event_id` + `cross_link_verified` 字段是否真写
3. 综合判定：UNIQUE（协议存在 + 字段真写）/ UNIQUE_BUT_BLIND（协议存在但字段未写）/ UNKNOWN

**F 锚定（B.x）**：
- B.1.1 改后 audit A5 立即转 PASS
- B.1.2 改后 audit A5 不报硬编码"3 gap"——而是根据实际 schema 字段动态计算
- B.1.3 pytest 217+ 不倒退

#### A-1. v1-family 函数跨族诊断（X2）

**目标**：识别 45+ v1-family 函数 + 提迁移路径

**位置**：
- `sih-tools/facet/probes/layer2_signoff.py`（除 _cmd_route 外的函数）
- `sih-tools/facet/probes/maturation_gate.py`（v1/v2 旧版本族函数）
- `sih-tools/facet/probes/batch_reevaluate_v2.py`（旧版）
- `sih-tools/facet/probes/cascade_ng_probe.py` / `temp_probe.py` / 等

**输出**：
- `sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md`（v1-family 函数清单 + 迁移路径）
- 含：函数名 / 文件:行 / 读什么 verdict 字段 / 用什么 verdict 决策 / 迁移路径（v1 → v3）

**F 锚定（A-1.x）**：
- A-1.1 识别 ≥ 40 个 v1-family 函数（baseline 已知 45+）
- A-1.2 每个函数含迁移路径（"改 X 行"或"立 X 任务"）
- A-1.3 不实施迁移（**只诊断**——T6D-04 才实施）

#### A-2. 数据治理 caller 设计（主线）

**目标**：让 .agents/skills/ 关键路径走 facet pipeline，制造真实 program_signoff / knife_edge_risk / supersession 事件

**位置**：`sih-engine/skills/sihankor-proposition-defense/`（自动触发机制设计）

**当前行为**：cron 跑 contribution_metric.py 监控 — 但 contribution_metric.py 是**指标计算**，不是真"治理操作"

**目标行为**：
- 关键治理决策（如"接受这个 prompt"）走 facet pipeline
- 跑 differential_integral 范式 → 闸裁决 → program_signoff / knife_edge_risk 事件真写
- 监控"飞轮跑频次"（用 cron）

**F 锚定（A-2.x）**：
- A-2.1 caller 设计文档化（不实施，但写 SPEC）
- A-2.2 流量入口识别（哪些治理操作该走 facet）

### Cluster 2

#### C-1. 跨仓 commit

- sih-engine 仓：B 改的 audit 脚本
- facet 仓：（如 B 涉及 flywheel_trail 字段改名）

#### C-2. 任务包结果文档

- `sih-engine/task-packages/fix-governance-boundaries-t6d-results.md`
- 内容：B 改完验 audit A5 PASS + A-1 跨族函数清单 + A-2 caller SPEC

## 四、可证伪条件（跑前立文） {#falsifiable}

### B 部分

- **B.1.1** B 改后 audit A5 PASS → 必须达成
- **B.1.2** B 改后 audit A5 不报硬编码"3 gap" → 必须达成
- **B.1.3** B 改后 pytest 217+ 不倒退 → 必须达成

### A-1 部分

- **A-1.1** 识别 ≥ 40 个 v1-family 函数 → 必须达成
- **A-1.2** 每个函数含迁移路径 → 必须达成
- **A-1.3** 不实施迁移（**只诊断**）→ 必须达成

### A-2 部分

- **A-2.1** caller 设计文档化 → 必须达成
- **A-2.2** 流量入口识别 → 必须达成

任一 F 触发 = 子任务失败。

## 五、必读文件 {#read}

X1 必读：
- `sih-engine/task-packages/fix-failures-t6d.md`（上游任务包）
- `sih-engine/task-packages/fix-failures-t6d-results.md`（F 锚定触发状态）
- `sih-engine/doc/design/DES-011-baseline-checker-DEC.md`（B 要读的 DEC）
- `sih-engine/skills/sihankor-proposition-defense/probes/check_decision_authority.py`（B 改的脚本）
- `sih-tools/facet/probes/flywheel_trail.py:238-299`（B 改的 schema 部分）

X2 必读：
- `sih-engine/task-packages/fix-failures-t6d.md`（上游）
- `sih-tools/facet/probes/check_verdict_consistency.py`（A-1 调研参考）
- `sih-tools/facet/probes/layer2_signoff.py`（除 _cmd_route 外的函数）
- `sih-tools/facet/probes/maturation_gate.py`（v1/v2 旧版本族）
- `sih-tools/facet/probes/batch_reevaluate_v2.py`（旧版）

主线必读：
- 全部 + A-2 caller 设计

## 六、微积分知识包

查 `/Users/moc/workspaces/SiHankor/calculus/llm-friendly-build/`：
- `mapping.md` 检索"约束 / 限制 / 边界"
- `entries/LIM-007-epsilon-delta-definition.md`（A-1 跨族函数识别的"边界"判据）
- `entries/INT-007-fundamental-theorem-of-calculus.md`（A-2 caller 流量的"累积"语义）

## 七、验收标准 {#acceptance}

### Cluster 1
- B 改完（X1）
- A-1 v1-family 函数清单（X2）
- A-2 caller SPEC（主线）

### Cluster 2
- audit_pipeline.py 跑 → exit 0（5 全 PASS）
- pytest 217+ 全过
- 跨仓 commit
- results 文档

## 八、约束 {#constraints}

- **零 LLM 调用**（脚本不调 LLM）
- **A-1 不实施迁移**（只诊断，不改代码）
- **B 不改 schema**（只改判定逻辑）
- **A-2 不实施**（只写 SPEC）
- **跨工具一致**（sih-engine / .agents/skills 双处都能跑）
- **不绕过失败**（F 锚定触发 = 子任务失败）

## 九、派发顺序（T6-D 范式） {#dispatch}

### 阶段 1：并行双子代理

```bash
# 派 X1 写 B
mavis task --description "fix-governance-t6d X1 B 部分" --prompt "..."

# 派 X2 写 A-1
mavis task --description "fix-governance-t6d X2 A-1 诊断" --prompt "..."

# 主线写 A-2
# 写 + commit A-2 caller SPEC
```

### 阶段 2：主线验收集齐

- 跑 audit_pipeline.py 验 5 PASS
- 跑 pytest 验 217+ 全过
- 主线 commit
- 写 results 文档

## 十、任务包元数据 {#meta}

- **包名**：fix-governance-boundaries-t6d
- **范式**：T6-D
- **作者**：Mavis（root session）
- **日期**：2026-08-17
- **依赖**：
  - T6D-01 mechanism-scripts-t6d（5 机械脚本）
  - T6D-02 fix-failures-t6d（4 修复代码层）
  - DES-011-baseline-checker-DEC（A5 DEC 立文）
- **关联**：
  - `task-packages/README.md` 范式命名约定
  - `skills/sihankor-proposition-defense/methodology.yaml` v1.1
  - `doc/design/DES-011-adjudication-baseline-check.md` + DEC

## 十一、不做的（明确边界）{#out-of-scope}

- ❌ A-1 不实施跨族函数迁移（45+ 函数改是 T6D-04 范围）
- ❌ A-2 不实施 caller 实际跑流量（只写 SPEC）
- ❌ 数据治理（让占比从 73% 降到 60%）—— 阶段 2.5 流量工程
- ❌ pre-existing 错位（test_flywheel_trail / test_check_verdict_consistency）—— 元层测试更新
- ❌ baseline_checker 实际代码实现（DES-011 §边界 §66）—— 后续 SPEC

## 十二、教训承接 {#lessons}

承接 fix-failures-t6d-results.md 教训：
- F 锚定设计要区分"代码修复" / "数据治理" / "跨族治理" / "元层工具"
- 本任务包 B 部分 = 元层工具更新（修即可见效）
- A-1 跨族治理 = 诊断阶段（不实施，留 T6D-04）
- A-2 数据治理 = SPEC 阶段（不实施，留阶段 2.5）
