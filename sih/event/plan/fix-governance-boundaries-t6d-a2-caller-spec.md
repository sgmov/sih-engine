# A-2 数据治理 caller SPEC（让 facet 接真实流量）

> 文档类型：SPEC（设计）
> 承接：fix-governance-boundaries-t6d §A-2
> 日期：2026-08-17
> 关联：A-1 v1-family 诊断、A-3 caller 集成、T6D-04 实施

## 一、问题陈述 {#problem}

### 1.1 当前数据状态 {#1-1-当前数据状态}

- 6227 个 trail 事件，flywheel_run 5270 / gate_assessment 950 / layer2_signoff 5 / program_signoff 0 / knife_edge_risk 0 / supersession 0
- 阶段 2 新机制（program_signoff / knife_edge_risk / supersession）零触发
- 占比 flywheel_run 73.17% > 60% 阈值 → A1 check_layer_proportion FAIL
- 数据治理目标：让新机制接真实流量，让占比降到 ≤ 60%

### 1.2 当前 .agents/skills/ 治理操作 {#1-2-当前-agents-skills-治理操作}

现有 6 个 skill：

| skill | 当前 LLM 调用模式 | 是否该走 facet 飞轮 |
|---|---|---|
| sihankor-calculus-trigger | 单 LLM 查概念 | 否（不治理判断）|
| sihankor-facet-measure | 调 facet 测温 | 部分（已有 facet 集成）|
| sihankor-intent-refine | 单 LLM 提炼意图 | 否（生成任务）|
| sihankor-pre-output-self-check | 单 LLM 自检 | 否（单实例不需多视角）|
| sihankor-proposition-defense | 三层审（多 LLM 投票）| **是**（多 LLM → 飞轮）|
| sihankor-redteam | 多 LLM 对抗 | **是**（多 LLM → 飞轮）|
| sihankor-slash-trigger | 路由 | 否（不判断）|

**2 个 skill 天然该走 facet 飞轮**：
1. **sihankor-proposition-defense** ： 三层审本质是多 LLM 投票，**应该是** facet 飞轮的 N 个 actor
2. **sihankor-redteam** ： 多 LLM 对抗，**应该是** facet 飞轮的 N 个 actor

### 1.3 流量入口识别 {#1-3-流量入口识别}

数据治理 = **让 .agents/skills/ 关键治理操作走 facet pipeline**：

- 每次 `sihankor-proposition-defense` 调三层审 = 1 个 flywheel_run 事件 + 1 个 gate_assessment 事件
- 每次 `sihankor-redteam` 跑对抗 = 1 个 flywheel_run + 1 个 gate_assessment
- 稳定 clear 时 = 1 个 program_signoff 事件
- 触发刀锋指纹 = 1 个 knife_edge_risk 事件
- 翻案（supersession）= 1 个 supersession 事件

按"每天每个 skill 触发 1-3 次"估算（实际看使用频次）：

| skill | 频次估计 | 单次事件 | 日事件累计 |
|---|---|---|---|
| proposition-defense | 1-3/天 | flywheel + gate | 2-6 |
| redteam | 1-2/天 | flywheel + gate | 2-4 |
| **合计** | | | 4-10/天 |

按 60% 占比倒推：
- 当前 73% = 5270 / 7205（N+C = 3601 估算）
- 目标 60%：新增 C 必须让 flywheel_run 占比从 73% 降到 60%
- 假设新事件全为 Layer 2（权重 2）：每条贡献 2
- 所需新事件：约 (60% × current_C) / (2 × 0.4) ≈ ~1500 条新 Layer 2 事件
- 按 4-10/天 估算：约 150-375 天 ≈ 5-12 个月

**A-2 数据治理是慢变量**：需要长期接入，不能立竿见影。

## 二、caller 设计 {#design}

### 2.1 总体架构 {#2-1-总体架构}

```
.agents/skills/{proposition-defense | redteam}
  ↓ (现有 LLM 调用)
caller wrapper (新组件)
  ├─ 准备：构造 facet scheme.yaml + topic.md
  ├─ 调 facet runner.py 跑 differential_integral 范式
  ├─ 读闸裁决 + trail
  └─ 返回 (verdict, evidence, 治理动作建议)
```

### 2.2 caller 接口设计 {#2-2-caller-接口设计}

```python
# sih-engine/skills/sihankor-proposition-defense/caller.py

def audit_with_facet(
    proposition: str,  # 待审命题
    audit_framework: str = "methodology.yaml",
    n_actors: int = 4,  # 4 模型跨家族
    *,
    trail_root: Path | None = None,
) -> AuditResult:
    """通过 facet 飞轮做治理审阅。
    
    流程：
    1. 构造 facet scheme.yaml（topic=proposition, NG=audit_framework）
    2. 调 facet runner 跑 differential_integral 范式
    3. 闸裁决 → record_flywheel_run + record_gate_assessment
    4. stable_clear + 程序签路径 → record_program_signoff
    5. 刀锋指纹触发 → record_knife_edge_risk
    6. 返回 (verdict, evidence, audit_trail_path)
    """
    ...
```

### 2.3 集成到现有 skill {#2-3-集成到现有-skill}

**SihankorPropositionDefenseSkill 改造**：

```python
# 之前 (LLM 真审)
def run_audit(report_path):
    return llm_audit(report_path)  # 3 layer LLM call

# 之后 (caller 走 facet)
def run_audit(report_path):
    return audit_with_facet(
        proposition=f"审 {report_path}",
        audit_framework="methodology.yaml",
    )
```

**SihankorRedteamSkill 改造**：

```python
# 之前 (LLM 对抗)
def run_redteam(proposition):
    return llm_redteam(proposition)

# 之后 (caller 走 facet)
def run_redteam(proposition):
    return audit_with_facet(
        proposition=proposition,
        audit_framework="redteam-protocol",
    )
```

### 2.4 反向兼容 {#2-4-反向兼容}

caller 设计要保留"非 facet 路径"作为 fallback：

```python
def run_audit(report_path, use_facet: bool = True):
    if use_facet and facet_available():
        return audit_with_facet(report_path)
    else:
        return llm_audit(report_path)  # 旧路径，fallback
```

**fallback 触发条件**：
- facet 不可用（runner 缺失 / 异常）
- 用户显式 --no-facet
- 测试 / dev 环境

### 2.5 流量归一化 {#2-5-流量归一化}

caller 跑的每个 audit 调用 = 1 个 flywheel_run + 1 个 gate_assessment 事件。

要确保：
- 1 个 audit 调用不展开为 10 个 flywheel_run（避免流量放大）
- 1 个 audit 调用真用 N 个 actor（4 模型跨家族）
- 闸裁决是 v3 闸（不是 v1）

## 三、不实施声明 {#out-of-scope}

A-2 caller SPEC **不实施**：
- ❌ 不写 `caller.py` 实际代码
- ❌ 不改 sihankor-proposition-defense / sihankor-redteam skill 集成
- ❌ 不立即在真流量上跑

实施路径：T6D-04 立任务包 ： caller 实现 + skill 集成 + 真实流量验证。

## 四、依赖与配套 {#dependencies}

| 依赖 | 状态 | 备注 |
|---|---|---|
| facet runner.py | ✅ 已存在 | `sih-tools/facet/runner.py` |
| facet differential_integral 范式 | ✅ 已存在 | atom-chain.yaml + runner |
| facet maturation_gate v3 | ✅ 已存在（v3-family）| maturation_gate.py |
| facet program_signoff schema | ✅ 已存在（T6D-02 X1）| A5 schema 加 cross-link |
| baseline_checker 实际代码 | ❌ DEC 已立，代码未实 | DES-011 §边界 §66 |
| caller.py 实际实现 | ❌ 本 SPEC 不实施 | T6D-04 |
| skill 集成（proposition-defense + redteam）| ❌ 本 SPEC 不实施 | T6D-04 |

## 五、F 锚定（设计阶段） {#falsifiable}

- **A-2.1 caller 设计文档化** ： 本 SPEC 落地 ✓
- **A-2.2 流量入口识别** ： proposition-defense + redteam 2 个 skill 识别 ✓

**实施阶段 F 锚定（T6D-04 立）**：
- A-2.3 caller 实际写完 + 单元测试过
- A-2.4 skill 集成后真实流量触发新机制事件 ≥ 10 条
- A-2.5 占比从 73% 降到 ≤ 60%（数据治理见效）
- A-2.6 baseline_checker 实际代码落地 + cross-link 协议生效

## 六、关联 {#relation}

- 承接：fix-failures-t6d §A-2 范围
- 依赖：T6D-01 mechanism-scripts-t6d（5 机械脚本监控）
- 依赖：T6D-02 fix-failures-t6d（4 修复代码层）
- 依赖：DES-011-baseline-checker-DEC（cross-link 协议）
- 实施：T6D-04 立任务包
- 平行：T6D-03 §A-1 v1-family 诊断

## 七、自检 {#self-check}

### 形式合规 {#形式合规}

- 一级标题无锚点，仅一个
- 二级标题均带锚点
- 首个二级标题命名为"问题陈述"
- 无破折号、无装饰符号、无 Unicode Emoji
- 引用使用半角双冒号加锚点链接格式

### 内容合规 {#内容合规}

- 现状数据明确（6227 / 73% / 5 PASS / 4 FAIL）
- 流量入口识别（2 个 skill：proposition-defense + redteam）
- caller 接口签名完整（参数 / 返回 / 异常 / fallback）
- 反向兼容路径明确（fallback 触发条件）
- 不实施声明显式（4 条不做的）
- 流量估算（5-12 个月达 60%）： 数据治理是慢变量，不立竿见影
- 依赖表清晰（4 ✅ + 3 ❌）
- 关联文档链回 fix-failures-t6d / DES-011 DEC / T6D-04
