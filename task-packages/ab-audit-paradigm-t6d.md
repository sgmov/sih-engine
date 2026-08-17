# ab-audit-paradigm-t6d：A/B 对照标准范式立文

> T6D-05 task-packages 治理任务
> 承接：facet-audit-t6d-results.md §八 教训 1（A/B 对照比单方审阅价值高）
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）： **本任务包偏离：立文类，主线直接写**
> 日期：2026-08-17

## 一、问题陈述 {#problem}

2026-08-17 facet-audit-t6d 实证：**A/B 对照审阅比单方审阅价值高**。

- 单 X1 审阅：13 条 + 7 盲点
- 单 X2 审阅：16 条 + 6 风险 + 6 工具盲点
- A/B 对照审阅：20 条独立发现（去重后），且揭示「立题命题覆盖」关键分歧（X1 测 ROADMAP 级完整 / X2 测 design 文档级 0%）

如果**不制度化**，下次审阅还会走「单方 + 后对照」弯路：A/B 价值是「设计时就并行 + 独立 + 锚定共享」实现的，后对照达不到同等张力。

## 二、关键设计 {#design}

### 2.1 范式骨架 {#2-1-范式骨架}

```
1. 主线立任务包（共享禁单 + 共享必读 + 共享 F 锚定）
2. 主线派双子代理（run_in_background=true 并行）：
   - X1 无工具对照组（纯阅读 + 直觉）
   - X2 有工具实验组（调全套新工具）
3. 主线读两份报告，对比发现清单
4. 主线写 results 文档判定工具有用性
5. 主线 commit 3 文件（X1 报告 / X2 报告 / results）
```

### 2.2 角色分工 {#2-2-角色分工}

| 角色 | 工具 | 报告字数 | 时间窗 |
|---|---|---|---|
| X1（无工具对照组） | 纯阅读（git log / read / grep / glob） | 1500-3000 字 | 25-35 min |
| X2（有工具实验组） | 阅读 + 跑全套新工具（至少 3 个） | 1500-3000 字 | 25-35 min |
| 主线 | 读两份报告 + 写 results | 不限 | 30-60 min |

### 2.3 共享 F 锚定（必立文） {#2-3-共享-f-锚定-必立文}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F1 X1 25-35 min 完成 | 元层工具 | 时间窗 + 报告非空 + 列至少 5 项发现 |
| F2 X2 25-35 min 完成 | 元层工具 | 时间窗 + 报告非空 + 列至少 5 项发现 |
| F3 X2 至少跑 3 个新工具 | 元层工具 | 报告引用 3+ 工具 + 列工具调用清单 |
| F4 X2 工具额外发现 ≥ 3 项 | 范式有用性 | 「X2 ∖ X1」去重后 ≥ 3 个独立发现项 |
| F5 X1 人类盲点 ≥ 1 项 | 范式有用性 | 「X1 ∖ X2」去重后 ≥ 1 个 X1 看到但 X2 工具漏掉的发现 |
| F6 报告锚定可追溯 | 元层工具 | 每条发现标注 (path, range) 或 commit hash |

### 2.4 共享禁单 {#2-4-共享禁单}

X1 禁止读 + 禁止跑（与 X2 工具组隔离）：

- `audit_pipeline.py` 全部 5 子脚本
- `check_three_proposition_audit.py`
- `v1-scan.py`（`task-packages/fix-governance-boundaries-t6d-v1-scan.py`）
- `contribution_metric.py`
- 任何 `check_/audit_/verify_` 类已 commit 脚本

X1 允许读：已 commit 的诊断数据（如 `v1-family-inventory.md`）： 因为这是产物不是工具

### 2.5 共享必读 {#2-5-共享必读}

- 任务包（含 F 锚定 + 共享禁单 + 共享必读）
- 审阅对象的 `ROADMAP.md`（全文 / 关键段）
- 审阅对象的 `plan/stage*-status.md`（如有）

X2 额外必读：新工具的 methodology.yaml + 5 脚本 README

### 2.6 主线对比方法学 {#2-6-主线对比方法学}

读两份报告后，主线做三件事：

1. **去重计算 X2 ∖ X1**（工具独有发现）
2. **去重计算 X1 ∖ X2**（阅读独有发现 / 人类盲点）
3. **合并 = 单方 vs A/B 的价值差**

主线的判定标准：

- 工具量化能力 vs 阅读语义能力 → 互补性
- 关键分歧（X1 vs X2 同议题不同结论）→ 揭示覆盖分层
- 工具盲点声明（X2 B1-B6 类）→ 工具下一阶段改进输入

### 2.7 范式适用边界 {#2-7-范式适用边界}

A/B 对照范式**适用**：

- 审阅对象 ≥ 1 仓
- 已有机械工具（X2 有东西可调）
- 审阅时长 ≥ 50 min（双子代理 + 主线）
- 审阅目标不是「修」（修 = T6D-08 类实施任务，不立本范式）

A/B 对照范式**不适用**：

- 审阅对象是单文件（双子代理审单文件浪费）
- 无工具可调（X2 退化为 X1）
- 紧急修复（时间窗不满足）

## 三、工作清单 {#work}

### 立文（无 Cluster 拆分，立文类主线写） {#立文-无-cluster-拆分-立文类主线写}

主线一次性写：

- [x] 范式骨架（§2.1）
- [x] 角色分工（§2.2）
- [x] 共享 F 锚定（§2.3）
- [x] 共享禁单（§2.4）
- [x] 共享必读（§2.5）
- [x] 主线对比方法学（§2.6）
- [x] 范式适用边界（§2.7）

主线验收（自我验证）：

- [x] 6 F 锚定可机械化（不依赖 LLM 解读）
- [x] 共享禁单有真禁止（不是「少调」是「零调」）
- [x] 主线对比方法学有去重操作

### 跨仓同步 {#跨仓同步}

- [ ] sih-tools/facet/ROADMAP.md §P5 加 1 段 cross-link 到本任务包

## 四、可证伪条件（跑前立文） {#falsifiable}

本任务包是「立文」类，F 锚定在「下次实际跑 A/B 对照」时验证，不是本次立文验证：

| F 锚定 | 判据 |
|---|---|
| F-A A/B 对照产生 X2 工具额外发现 ≥ 3 项 | 下次实际跑时计数 |
| F-B A/B 对照产生 X1 人类盲点 ≥ 1 项 | 下次实际跑时计数 |
| F-C 主线能在 60 min 内写完 results | 计时 |
| F-D 6 F 锚定 100% 机械化 | 不依赖 LLM 解读 |

## 五、必读文件 {#read}

立文时：

- `sih-engine/task-packages/facet-audit-t6d.md`（范式源）
- `sih-engine/task-packages/facet-audit-t6d-report-A.md`（X1 报告）
- `sih-engine/task-packages/facet-audit-t6d-report-B.md`（X2 报告）
- `sih-engine/task-packages/facet-audit-t6d-results.md`（results，对比方法学源头）
- `sih-engine/task-packages/README.md`（T6-D 命名约定）

## 六、约束 {#constraints}

1. **零 LLM 调用**（本任务包立文过程不调 LLM）
2. **6 F 锚定必须机械化**（不依赖 LLM 解读）
3. **共享禁单是「零调」不是「少调」**（X1 报告里有 1 个工具调用 = F 锚定失败）
4. **报告字数 1500-3000**（过短发现不足，过长淹没重点）
5. **锚定可追溯**（每条发现标注 path/line 或 commit hash）
6. **不重复 ROADMAP 既有结论**（要找 ROADMAP 未覆盖的真问题）

## 七、验收标准 {#acceptance}

本任务包验收 = 立文落地 + 跨仓同步：

- 本任务包 commit 入 `sih-engine/task-packages/`
- sih-tools/facet/ROADMAP.md §P5 加 1 段 cross-link
- 下次实际跑 A/B 对照时 F-A F-B F-C F-D 状态分项记录

## 八、范式偏离声明 {#deviation}

本任务包**偏离 T6-D 范式**的「双子代理实施」流程：

- 理由：立文类是确定性任务（写文档），不涉及「实施 / 修复 / 写代码」
- 偏离：双子代理由「实施 X1 + X2」改为「对照实验 X1 + X2」（X1 X2 都做审阅而非实施）
- 保留：T6-D 命名约定（`<主题>-t6d.md` 后缀）+ 主线串行验证 + F 锚定 + 跨仓同步

诚实记录：下次 A/B 对照实际跑时，如果出现「立文类 + 实施类并存」，应回退到 T6-D 标准流程（双子代理实施 + 主线验证）。

## 九、关联文件 {#related}

- 任务包源：`sih-engine/task-packages/facet-audit-t6d.md`（T6D-04 范式实例）
- results 源：`sih-engine/task-packages/facet-audit-t6d-results.md` §八 教训 1
- 范式文档：`sih-engine/task-packages/README.md`（T6-D 命名约定）
- 跨仓引用：sih-tools/facet/ROADMAP.md §P5（待加 cross-link）
