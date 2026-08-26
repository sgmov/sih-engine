# facet-audit-t6d：facet 仓 A/B 对照审阅

> T6-D 范式：双子代理并行 + 主线串行验证
> 主题：facet 仓现状审阅，对照实验 A vs B（新工具有用性判定）
> 日期：2026-08-17

## 一、问题陈述 {#problem}

facet 仓自阶段 1+2 累积（adc4d6d 684 文件 commit）以来，已搭起 5 机械脚本 + 三层命题审计 + 跨族诊断 + 贡献度指标等新工具。但**新工具是否真的有用**尚未实证：

- 之前 multi-agent-independent-audit-2026-08-16（8ee3e71）是同 LLM 厂商的「同源预训练风险」实验，未触及新工具有用性
- 之前 T6D-01 mechanism-scripts 抓了 11 真问题中 9 个（82%），但那是 A1-A5 各自跑的结果，没有「不用工具」的对照组
- 真有用 = 对照组能发现的盲点（无工具时漏掉的） ∖ 工具组（用工具时）有真发现

## 二、关键设计 {#design}

### 2.1 A/B 对照实验 {#2-1-a-b-对照实验}

| | A 子代理（X1） | B 子代理（X2） |
|---|---|---|
| 工具 | 纯阅读代码 + commit log + ROADMAP | 全部新工具 |
| 禁止 | audit_pipeline / check_three_proposition_audit / 5 mechanical scripts / v1-scan / contribution_metric | （无禁止） |
| 必须 | 独立写判定 + 列盲点 | 跑工具 + 列工具发现的额外项 |
| 产时 | 25-35 min | 25-35 min |

### 2.2 审阅对象 {#2-2-审阅对象}

`sih-tools/facet` 仓当前 head（44e8d0c 之后）：

- 99 probes 文件
- ROADMAP 1001 行 / plan/stage2-status 259 行
- 217 测试（阶段 1+2 全过）
- 阶段 1+2 完成度：cascade_ng / 锚点衰减 / rounds 边际 / 偏导 CI / v3 冻结 / 多角度 / 温度计 / 程序签 / knife-edge
- 阶段 2.5：贡献度指标（N + C 替代 3 个月观察期）

### 2.3 审阅维度（双子代理共用） {#2-3-审阅维度-双子代理共用}

四个维度，**双子代理各自决定权重**：

1. **代码质量**：ROADMAP 兑现度、commit 粒度、测试覆盖、文档完整度
2. **退出标准满足度**：6 条退出条件（特别是 #6 贡献度）
3. **治理覆盖度**：立题 / 应用 / 治理领域展开贡献三层是否对齐
4. **跨族治理现状**：v1-family 函数待迁规模（X2 用 v1-scan，X1 估数）

## 三、工作清单 {#work}

### Cluster 1：并行双子代理 {#cluster-1-并行双子代理}

- **X1**（无工具）：独立审阅 facet，写 facet-audit-t6d-report-A.md
- **X2**（有工具）：用全套新工具审阅 facet，写 facet-audit-t6d-report-B.md
- run_in_background = true（并行）

### Cluster 2：主线串行验证 {#cluster-2-主线串行验证}

- 读 X1 + X2 报告，对比发现清单
- 计算「X2 ∖ X1」的工具额外发现项数
- 计算「X1 ∖ X2」的人类直觉盲点数
- 写 facet-audit-t6d-results.md 判定新工具有用性

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F1** X1 报告 25-35 min 完成 | 元层工具 | 时间窗 + 报告非空 + 列至少 5 项发现 |
| **F2** X2 报告 25-35 min 完成 | 元层工具 | 时间窗 + 报告非空 + 列至少 5 项发现 |
| **F3** X2 至少跑 3 个新工具 | 元层工具 | 报告里引用 audit_pipeline + check_three_proposition_audit + 至少 1 个其他 |
| **F4** X2 工具额外发现 ≥ 3 项 | 范式有用性 | 「X2 ∖ X1」去重后 ≥ 3 个独立发现项 |
| **F5** X1 人类盲点 ≥ 1 项 | 范式有用性 | 「X1 ∖ X2」去重后 ≥ 1 个 X1 看到但 X2 工具漏掉的发现 |
| **F6** 报告锚定可追溯 | 元层工具 | 每条发现标注 (path, range) 或 commit hash |

## 五、必读文件 {#read}

X1 + X2 必读：

- `sih-tools/facet/ROADMAP.md`（1001 行）
- `sih-tools/facet/plan/stage2-status.md`（259 行）
- `sih-tools/facet/probes/contribution_metric.py`（236 行，最近立文）

X2 必读：

- `sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md`（A-1 inventory）
- `sih-engine/skills/sihankor-proposition-defense/probes/audit_pipeline.py`（B2 流水线）
- `sih-engine/skills/sihankor-proposition-defense/probes/check_three_proposition_audit.py`（三层审）

X1 禁止读：

- `sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md`
- `sih-engine/skills/sihankor-proposition-defense/probes/*.py`

## 六、约束 {#constraints}

1. **零 LLM 调用**（机械脚本层）： 工具组的 LLM 调用仅限调用 skill 内的 LLM 部分（proposition-defense skill 调 LLM），但 X2 子代理自己**不**调 LLM
2. **不实施修复**：只审阅，不动 facet 代码
3. **报告长度 1500-3000 字**：过短发现不足，过长淹没重点
4. **并行派发**：run_in_background=true 同时派 X1 + X2
5. **不预先告知对方报告**：独立审阅
6. **锚定可追溯**：每条发现标注 (path, range) 或 commit hash
7. **不重复 ROADMAP 既有结论**：要找的是 ROADMAP 未覆盖的真问题

## 七、验收标准 {#acceptance}

### Cluster 1 {#cluster-1}

- X1 报告：`sih-engine/task-packages/facet-audit-t6d-report-A.md`
- X2 报告：`sih-engine/task-packages/facet-audit-t6d-report-B.md`
- 两份报告都非空，长度 1500-3000 字

### Cluster 2 {#cluster-2}

- 主线对比结果：`sih-engine/task-packages/facet-audit-t6d-results.md`
- 至少列 3 个维度的对比（代码质量 / 退出标准 / 治理覆盖 / 跨族治理）
- 明确判定：新工具有用 / 有限有用 / 无用，附数据支撑
- F 锚定 F1-F6 状态分项记录

## 八、派发顺序（T6-D 范式） {#dispatch}

### 阶段 1：并行双子代理 {#阶段-1-并行双子代理}

```python
# X1：无工具审阅
task(
    description="facet-audit A 子代理 (无工具)",
    prompt=BRIEFING_A,
    agent_name="explore",  # 只读 + 独立审阅
    run_in_background=True
)

# X2：有工具审阅（同时）
task(
    description="facet-audit B 子代理 (有工具)",
    prompt=BRIEFING_B,
    agent_name="explore",  # 只读 + 调工具
    run_in_background=True
)
```

### 阶段 2：主线串行验证 {#阶段-2-主线串行验证}

- 等 X1 + X2 都完成（background 唤醒）
- 读两份报告，写 results 文档
- commit 3 个文件（X1 报告 / X2 报告 / results）

## 九、风险点 {#risks}

- **X1 / X2 报告雷同**：可能同源预训练导致结论相似。F 锚定 F4 + F5 显式比较「X2 ∖ X1」和「X1 ∖ X2」
- **X2 工具调用不充分**：可能 X2 偷懒不调工具全靠读代码。F 锚定 F3 强制 X2 至少跑 3 个工具
- **审阅范围过大**：facet 99 probes + ROADMAP 1001 行，子代理可能浅尝辄止。约束 7「不重复 ROADMAP 既有结论」保证深度

## 十、关联文件 {#related}

- 任务包：`task-packages/fix-governance-boundaries-t6d.md`（T6D-03 上游）
- 工具：`sih-engine/skills/sihankor-proposition-defense/probes/audit_pipeline.py`
- 工具：`sih-engine/task-packages/fix-governance-boundaries-t6d-v1-scan.py`
- 上游教训：`task-packages/fix-failures-t6d-results.md` §教训
- 范式文档：`task-packages/README.md`
