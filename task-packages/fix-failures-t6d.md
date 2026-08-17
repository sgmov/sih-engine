# 任务包：修 T6D-01 抓到的 4 个真修复

> 范式：T6-D（双子代理 + 主线串行验证）
> 立项原因（2026-08-17）：T6D-01 任务包（mechanism-scripts-t6d）跑完 5 机械脚本，抓到 4 个真修复。机械脚本只检查不修（任务包 §8 约束）。本任务包落地这 4 个真修复，让 audit_pipeline 从 1 PASS / 4 FAIL → 5 PASS / 0 FAIL。

## 一、问题陈述 {#problem}

T6D-01 audit_pipeline baseline（c3f98b7 / f0eca04 后）：

| 脚本 | 状态 | 根因 | 修复方向 |
|---|---|---|---|
| A1 check_layer_proportion | FAIL | flywheel_run 73.17% > 60% | contribution_metric 加"layer 占比下限"约束 |
| A2 check_yaml_factual | PASS | (v1.1 已修) | — |
| A3 check_test_intent | FAIL | test_f34_reproducibility 名字 vs 实际（3 mismatch） | 加 assert trail 不被重复写 + program_signoff 加 dedup |
| A4 check_verdict_consistency | FAIL | 1 v1/v3 version split | layer2_signoff route 内部重算 v3 |
| A5 check_decision_authority | FAIL | UNIQUE_BUT_BLIND + 3 cross-link gap | DES-011 立 DEC + program_signoff schema 加 cross-link |

**目标**：修完后 audit_pipeline 跑 5 全 PASS，可切到 STRICT commit hook 模式。

## 二、关键设计 {#design}

### 2.1 T6-D 编排（双子代理 + 主线）

**Cluster 1：双子代理并行（facet 代码改）**

- 子代理 X1：写 A3（test_f34 + program_signoff dedup）+ A4（layer2_signoff route 内部重算 v3）
- 子代理 X2：写 A1（contribution_metric layer 占比下限）+ A5（program_signoff schema 加 cross-link）
- 主线（本会话）：验收 + 立 DES-011 DEC

**Cluster 2：跨仓 commit + 任务包结果**

- 主线：cluster 1 收齐后 commit + 写 fix-failures-t6d-results.md

### 2.2 修复顺序依赖

- A3 依赖 program_signoff dedup（要先改 program_signoff，再加 test assert）
- A4 依赖 layer2_signoff route 改（独立）
- A1 依赖 contribution_metric 改（独立）
- A5 依赖 program_signoff schema 改（与 A3 共享 program_signoff 改动）

**约束**：A3 + A5 都改 program_signoff.py——必须协调，否则合并冲突。建议 X1 改完 program_signoff 主体，X2 在 X1 基础上加 cross-link 字段。

或：让 X1 改 A3 + A4（程序签路由层），X2 改 A1 + A5（指标层 + 设计层），X2 用 DES-011 DEC 立文 + program_signoff 增量改动（不冲突 X1 的 dedup 部分）。

### 2.3 验收标准

- 跑 audit_pipeline.py → exit 0（5 全 PASS）
- 跑 pytest tests/ -q → 195+ 全过
- 跑 contribution_metric.py → NOT PASS（仍是真实流量 0 触发，但 N+C 指标逻辑不能倒退）

## 三、工作清单 {#work}

### Cluster 1

#### A1. contribution_metric 加 layer 占比下限（X2）

**位置**：`sih-tools/facet/probes/contribution_metric.py`

**改动**：
- 加 `LAYER_PROPORTION_MIN` 阈值（默认 `flywheel_run 占比 ≤ 0.60`）
- 加 `THRESHOLD_LAYER_PROPORTION = 0.60` 常量
- 在 `compute_contribution()` 末尾加 layer 占比判定
- 报告输出加 `layer_proportions: dict` 字段
- exit code 含义扩展：当前 0=PASS / 1=NOT PASS / 2=ERROR，新增：
  - `meets_layer_proportion: bool` 字段
  - 总体 exit 改为：0=全过 (N+C+新机制+占比) / 1=任一不达

**F 锚定（F1.x）**：
- F1.1 实测 flywheel_run 占比 ≥ 60% → 撤销（已抓）→ 修后 = PASS ✓
- F1.3 占比阈值与 A1 机械脚本一致（不能 < 0.60，否则 A1 永远 FAIL）

#### A3. program_signoff 加 dedup + test_f34 修（X1）

**位置 1**：`sih-tools/facet/probes/program_signoff.py`

**改动**：
- 入口加 `if ft.load_latest_program_signoff(guidance_id, trail_root) is not None: return existing event`
- 不写新 event，复用已有

**位置 2**：`sih-tools/facet/tests/test_program_signoff.py:209-227` `test_f34_reproducibility`

**改动**：
- 测试名保留 / docstring 加 "并验 trail 不被重复写"
- 加 `assert ft.load_program_signoff_count(gid, ft_root) == 1`

**F 锚定（F3.x）**：
- F3.1 修后 A3 check_test_intent 不再 PARTIAL test_f34 → PASS
- F3.2 简单测试不误报（已有） → 仍 PASS

#### A4. layer2_signoff route 内部重算 v3（X1）

**位置**：`sih-tools/facet/probes/layer2_signoff.py:228-267`

**改动**：
- `_cmd_route` 调 `ft.load_dc_list(guidance_id, trail_root)` 而非只读 v1 verdict
- 调 `mg.assess_maturation_v3(dc_list, ...)` 重算 v3
- 用 v3 verdict 决定 route（boundary/near 走人签 / stable_clear 走 program_signoff）

**F 锚定（F4.x）**：
- F4.1 修后 A4 check_verdict_consistency 不再 INCONSISTENT → PASS
- F4.2 v1/v2 同一族不误判（已有）→ 仍 PASS

#### A5. program_signoff schema 加 cross-link + DES-011 DEC（X2）

**位置 1**：`sih-tools/facet/probes/flywheel_trail.py:238-291` `record_program_signoff` 事件 schema

**改动**：
- 加 `sih_engine_event_id: str | None` 字段（cross-link 到 sih-engine event_stream）
- 加 `cross_link_verified: bool` 字段（DES-011 核对通过标记）

**位置 2**：`sih-engine/doc/design/` 新建 `DES-011-baseline-checker-DEC.md`（组件立名 DEC）

**改动**：
- 立名 `baseline_checker`（暂名 → DEC）
- 定义 cross-link 事件类型（spec-004 扩展）
- 描述 facet program_signoff 与 sih-engine baseline_checker 之间的 cross-link 协议

**F 锚定（F5.x）**：
- F5.1 修后 A5 check_decision_authority 不再 UNIQUE_BUT_BLIND → UNIQUE（cross-link 建立后）
- F5.2 路径图含 4 条（人签路由 + cross-link 到 sih-engine）

### Cluster 2

#### B1. 跨仓 commit

- facet 仓：3 commit（A1 / A3+A4 / A5 schema）
- sih-engine 仓：1 commit（DES-011 DEC）

#### B2. 任务包结果文档

- `sih-engine/task-packages/fix-failures-t6d-results.md`
- 内容：4 修复的 commit hash + 修后 audit_pipeline 实测 (5 PASS) + 教训

## 四、可证伪条件（跑前立文） {#falsifiable}

- **F1.1.1** A1 修后 audit_pipeline A1 仍 FAIL → **撤销**（修复无效）
- **F1.1.2** A1 修后 pytest 195+ 倒退 → **撤销**（破坏现有测试）
- **F3.1.1** A3 修后 audit_pipeline A3 仍 FAIL → **撤销**
- **F3.1.2** A3 修后 test_f34_reproducibility 测试失败 → **撤销**
- **F4.1.1** A4 修后 audit_pipeline A4 仍 FAIL → **撤销**
- **F4.1.2** A4 修后 v1 trail 数据不消费 → **撤销**（v1 verdict 应仍可查）
- **F5.1.1** A5 修后 audit_pipeline A5 仍 UNIQUE_BUT_BLIND → **撤销**
- **F5.1.2** A5 修后 cross-link 字段不写 → **撤销**

任一 F1.x.1-F5.x.2 触发 = 子任务失败。

## 五、必读文件 {#read}

子代理 X1 必读：
- `sih-engine/task-packages/mechanism-scripts-t6d.md` §3.A3/A4 + §4 F 锚定
- `sih-engine/task-packages/mechanism-scripts-t6d-results.md` 中期可做清单
- `sih-tools/facet/probes/program_signoff.py`（A3 必读全文）
- `sih-tools/facet/probes/layer2_signoff.py:228-267`（A4 必读）
- `sih-tools/facet/tests/test_program_signoff.py:209-227`（A3 必读）

子代理 X2 必读：
- 同上 + `sih-tools/facet/probes/contribution_metric.py`（A1 必读全文）
- `sih-tools/facet/probes/flywheel_trail.py:238-291`（A5 schema 必读）
- `sih-engine/doc/design/DES-011-adjudication-baseline-check.md`（A5 必读全文）

主线必读：
- 全部
- DES-011 立 DEC 决策（用户已立任务包，需要 root session 决策 DEC 接受度）

## 六、微积分知识包

查 `/Users/moc/workspaces/SiHankor/calculus/llm-friendly-build/`：
- `mapping.md` 检索"约束 / 限制 / 边界"
- `entries/LIM-007-epsilon-delta-definition.md`（A1 占比阈值 ± ε 论证）
- `entries/INT-007-fundamental-theorem-of-calculus.md`（A1 占比 ∫ w dN）

缺则 web_search → 代理 → 补仓。

## 七、验收标准 {#acceptance}

### Cluster 1（A1 + A3 + A4 + A5）
- 4 个 commit（facet 仓 + sih-engine 仓）
- 修后 audit_pipeline.py 跑 → exit 0（5 全 PASS）
- 修后 pytest tests/ -q → 195+ 全过
- F1.x.1-F5.x.2 全部锚定不触发

### Cluster 2（B1 + B2）
- 跨仓 commit 完成
- fix-failures-t6d-results.md 写完
- 5 脚本 baseline 报告

## 八、约束 {#constraints}

- **零 LLM 调用**（如果脚本/测试改用 LLM call 必须立即停止）
- **不绕过失败**（F 锚定触发 = 任务失败）
- **不写"建议"**（输出事实 + F 判定）
- **跨仓协调**：A3 + A5 都改 program_signoff.py，必须协调（X1 改主体，X2 加 cross-link 字段）
- **A1 占比阈值论证**：不能拍脑袋（必须用 LIM-007 epsilon-delta 或 INT-007 论证）
- **DES-011 DEC 是设计变更**：主线验收后 commit，不能 X2 自行 commit

## 九、派发顺序（T6-D 范式） {#dispatch}

### 阶段 1：并行双子代理

```bash
# 派 X1 写 A3 + A4
mavis task --description "fix-failures A3+A4" --prompt "..."

# 派 X2 写 A1 + A5
mavis task --description "fix-failures A1+A5" --prompt "..."
```

### 阶段 2：主线验收 + 跨仓 commit

- 跑 audit_pipeline.py 验 5 PASS
- 跑 pytest 验 195+ 全过
- 主线 commit
- 写 fix-failures-t6d-results.md

## 十、任务包元数据 {#meta}

- **包名**：fix-failures-t6d
- **范式**：T6-D
- **作者**：Mavis（root session）
- **日期**：2026-08-17
- **依赖**：T6D-01 任务包（mechanism-scripts-t6d）完成
- **关联**：
  - `task-packages/mechanism-scripts-t6d.md` + results
  - `task-packages/README.md` 范式命名约定
  - `skills/sihankor-proposition-defense/methodology.yaml` v1.1
