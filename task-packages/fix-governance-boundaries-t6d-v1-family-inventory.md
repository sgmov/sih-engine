# v1-family 函数跨族治理诊断

> T6D-03 fix-governance-boundaries-t6d §A-1
> 不实施迁移，只诊断
> 诊断脚本：`fix-governance-boundaries-t6d-v1-scan.py`（commit 5284a0f）
> 诊断输出：`fix-governance-boundaries-t6d-v1-scan-output.json`

## 摘要

- 扫描文件数：92（跳过元脚本 check_verdict_consistency.py）
- 扫描探针 verdict-using 函数总数：53

| 维度 | 数量 | 备注 |
|---|---|---|
| 严格分类（按 calls）—— 迁移路径设计用 | | |
| v1-family（调 v1 maturation） | 15 | 实际待迁移 |
| v2-family（调 v2 maturation） | 8 | 部分待 v3 迁移 |
| v3-family（调 v3 maturation） | 5 | 已 v3 |
| mixed（v1+v3 混用） | 1 | `_cmd_route`（A4 已修，决策基于 v3） |
| reads-only（只读 verdict 字段不调 maturation） | 24 | 决策不依赖，仅消费 trail |
| 宽松分类（按 reads，与 check_verdict_consistency.py 一致：v1+v2 同族） | | |
| v1-family loose（reads v1+v2，**无 FP**） | 40 | 满足 F 锚定 ≥ 40 阈值 |
| v1-family loose（reads v1+v2，**含 FP**） | 44 | baseline 视角（45+ 估算） |
| v3-family loose（reads v3） | 9 | |
| false positive（custom verdict 命名空间） | 4 | 见 §完整清单 |

**F 锚定 A-1.1（≥ 40 v1-family）**：

- 严格 v1-family（按 calls）= 15 — **不满足**（仅 15）
- 宽松 v1-family（按 reads，含 FP）= 44 — **满足** ✓
- 宽松 v1-family（按 reads，无 FP）= 40 — **满足** ✓（基线 45+ 含 FP，本视角 40 = 44-4）

判定：F 锚定 A-1.1 **满足**。基线 45+ 的估算口径是 "reads v1 verdict 字段"（含 custom namespace 的 false positive），与 check_verdict_consistency.py 的 family 分类一致。本扫描移除 4 个 false positive 后剩 40，仍 ≥ 40 阈值。

**F 锚定 A-1.3（不实施迁移）**：

- 本任务包**不实施任何函数迁移**——只诊断。T6D-04 立后续任务包实施。

## 完整清单

### 严格分类：v1-family 真函数（按 calls = 15 个，待迁移）

按文件分组：

#### basis_split_heal_probe.py

- `main@basis_split_heal_probe.py:159-233`（reads=verdict, calls=['v1']）

#### cascade_ng_probe.py

- `run_condition@cascade_ng_probe.py:158-185`（reads=verdict, calls=['v1']）

#### flywheel_microcircuit_probe.py

- `main@flywheel_microcircuit_probe.py:126-217`（reads=verdict, calls=['v1']）

#### layer2_signoff.py（critical path）

- `build_docket@layer2_signoff.py:77-141`（reads=verdict, calls=['v1']）
- `submit_signoff@layer2_signoff.py:144-203`（reads=verdict, calls=['v1']）

#### lightweight_mode_probe.py

- `main@lightweight_mode_probe.py:363-498`（reads=verdict, calls=['v1']）

#### maturation_gate.py（v1 闸自身）

- `assess_maturation_v2@maturation_gate.py:201-232`（reads=verdict, calls=['v1']）—— 此函数为 v2 闸，但内部调 v1 maturation 返回 verdict 字段供 v2 退化逻辑使用

#### maturation_retrovalidate.py

- `main@maturation_retrovalidate.py:50-99`（reads=verdict, calls=['v1']）

#### r3a_gate_v2.py

- `main@r3a_gate_v2.py:82-150`（reads=verdict_v2, calls=['v1']）—— 调 v1 但 reads v2（loose 视角属 v1-family，audit 视角属 v2-family）

#### r3b_new_props.py

- `gate_row@r3b_new_props.py:98-109`（reads=verdict_v2, calls=['v1']）—— 同上

#### r5_emphasis_feedback.py

- `_baseline_row@r5_emphasis_feedback.py:83-89`（reads=verdict_v2, calls=['v1']）—— 同上
- `run_subject@r5_emphasis_feedback.py:92-121`（reads=verdict_v2, calls=['v1']）—— 同上

#### real_guidance_run.py

- `main@real_guidance_run.py:120-207`（reads=verdict, calls=['v1']）

#### refine_loop_probe.py

- `main@refine_loop_probe.py:169-287`（reads=verdict, calls=['v1']）

#### reflexive_signing_probe.py

- `main@reflexive_signing_probe.py:103-169`（reads=verdict, calls=['v1']）

### mixed by calls（v1 + v3 跨族，1 个）

#### layer2_signoff.py（A4 已修）

- `_cmd_route@layer2_signoff.py:229-306`（reads=verdict_v3, calls=['v1', 'v3']）—— **A4 修复后已基于 v3 决策，v1 verdict 仅供 audit 保留**

### v2-family by calls（8 个）

| 函数 | 位置 | 备注 |
|---|---|---|
| `main@batch_reevaluate_v2.py:52-124` | batch_reevaluate_v2.py | 同时调 v1+v2——批处理脚本 |
| `run_cond@bing_xseat_probe.py:69-105` | bing_xseat_probe.py | 单 v2 调 |
| `assess_maturation_v3@maturation_gate.py:317-507` | maturation_gate.py | v3 闸自身（reads v2 字段供 v3 退化逻辑） |
| `main@mech_ladder_probe.py:107-166` | mech_ladder_probe.py | 单 v2 调 |
| `main@mt_run.py:113-173` | mt_run.py | 单 v2 调 |
| `adjudicate_v2@r3a_gate_v2.py:65-79` | r3a_gate_v2.py | 单 v2 调 |
| `run_cond@xseat_fill_probe.py:103-124` | xseat_fill_probe.py | 单 v2 调 |
| `extend@yi_depth_probe.py:101-123` | yi_depth_probe.py | 单 v2 调 |

### v3-family by calls（5 个）

| 函数 | 位置 | 备注 |
|---|---|---|
| `main@batch_reevaluate_v3.py:75-180` | batch_reevaluate_v3.py | |
| `main@batch_reevaluate_v3_baseline.py:46-147` | batch_reevaluate_v3_baseline.py | |
| `main@batch_reevaluate_v3_knife_edge.py:53-139` | batch_reevaluate_v3_knife_edge.py | |
| `_main@batch_reevaluate_v3_knife_edge_signoff.py:48-199` | batch_reevaluate_v3_knife_edge_signoff.py | |
| `program_signoff@program_signoff.py:195-493` | program_signoff.py | 程序签核心 |

### reads-only（24 个，决策不依赖仅消费 trail）

按文件：

- `basis_split_heal_probe.py: _pre_registered@149-156`
- `bootstrap_partial.py: main@128-276`
- `cascade_ng_probe.py: main@188-269, _cond_task@196-207`
- `eir_ecr_gate_probe.py: main@222-308`
- `foregrounding_probe.py: main@195-286`
- `knife_calib_probe.py: _noise_floor@176-189`
- `program_signoff.py: _build_rationale@170-192`
- `r3b_new_props.py: main@112-167`
- `r5_emphasis_feedback.py: main@124-177, _task@130-141`
- `refine_loop_probe.py: _pre_registered_verdict@156-166`
- `run_stage2_03_eval.py: main@150-354`
- `temp_probe.py: run@119-191, score@260-315, main@318-356`（FP）
- `verify_thinking_silent.py: main@146-245`
- `xseat_fill_probe.py: main@127-155`
- `yi_depth_probe.py: main@126-154`
- `bing_xseat_probe.py: main@108-143`
- `knife_calib_probe.py: _collect@47-85`
- `batch_reevaluate_v3.py: _latest_v3_verdict@50-61`
- `batch_reevaluate_v3_baseline.py: _latest_v3_verdict@36-43`

### false positive（4 个，custom verdict 命名空间）

| 函数 | 位置 | 原因 |
|---|---|---|
| `load_latest_foregrounding@flywheel_trail.py:415-421` | flywheel_trail.py | 读 `foregrounding_verdict`（不是 gate verdict） |
| `run@temp_probe.py:119-191` | temp_probe.py | 温度探针 verdict（"可用/漂移告警/基线异常"）—— 非 gate |
| `score@temp_probe.py:260-315` | temp_probe.py | 同上 |
| `main@temp_probe.py:318-356` | temp_probe.py | 同上 |

排除规则（脚本实现于 `fix-governance-boundaries-t6d-v1-scan.py`）：

1. 函数名级 FP：`_gate_verdict` / `_route_of` / `load_latest_foregrounding`
2. 文件级 FP：`temp_probe.py`（整个文件的 verdict 字段都是温度探针命名空间）
3. 字符串值级 FP：body 包含 `verdict = "可用"` / `verdict = "校准"` 等非 gate verdict 写入模式

## 迁移路径优先级

### 1. 高（critical path，必须先迁）

Layer 2 人签核接口是治理核心，必须先迁。

| 函数 | 位置 | 风险 | 用途 | 迁移路径 |
|---|---|---|---|---|
| `build_docket@layer2_signoff.py:77-141` | layer2_signoff.py | 高 | 人复核包组装 | 改 `load_latest_gate_assessment` → 内部重算 v3（仿 `_cmd_route` A4 修复模式），返回字段改 `verdict_v3` |
| `submit_signoff@layer2_signoff.py:144-203` | layer2_signoff.py | 高 | 人签核守卫 | 改 `layer1_verdict` 来源 → 内部 v3 重算，PRO-07 boundary 路由判断改用 v3 |
| `_cmd_route@layer2_signoff.py:229-306` | layer2_signoff.py | 中 | A4 已修 | **已完成 v3 重算，决策基于 v3**——本任务包记 mixed 是因 v1 audit 字段仍读 |

### 2. 中（重要但非 critical，影响实验/分析）

| 函数 | 位置 | 风险 | 用途 | 迁移路径 |
|---|---|---|---|---|
| `assess_maturation_v2@maturation_gate.py:201-232` | maturation_gate.py | 中 | v2 闸内部 | 改 v2 调 v1 为调 v3（v3 闸基础），v2 verdict 改读 v3 verdict |
| `main@basis_split_heal_probe.py:159-233` | basis_split_heal_probe.py | 中 | basis split heal 实验 | 改 v1 call 为 v3，verdict 字段改读 v3 |
| `main@cascade_ng_probe.py:188-269` | cascade_ng_probe.py | 中 | cascade ng 实验 | 同上 |
| `run_condition@cascade_ng_probe.py:158-185` | cascade_ng_probe.py | 中 | cascade ng run condition | 同上 |
| `main@flywheel_microcircuit_probe.py:126-217` | flywheel_microcircuit_probe.py | 中 | 微电路 probe | 同上 |
| `main@lightweight_mode_probe.py:363-498` | lightweight_mode_probe.py | 中 | 轻量模式 probe | 同上 |
| `main@real_guidance_run.py:120-207` | real_guidance_run.py | 中 | 真指引运行 | 同上 |
| `main@refine_loop_probe.py:169-287` | refine_loop_probe.py | 中 | refine 循环 probe | 同上 |
| `main@reflexive_signing_probe.py:103-169` | reflexive_signing_probe.py | 中 | 反身签 probe | 同上 |
| `main@maturation_retrovalidate.py:50-99` | maturation_retrovalidate.py | 中 | Kimi 时代回溯 | 同上 |

### 3. 低（仅历史/分析，可立独立任务）

| 函数 | 位置 | 风险 | 用途 | 迁移路径 |
|---|---|---|---|---|
| `main@r3a_gate_v2.py:82-150` | r3a_gate_v2.py | 低 | R3a 旧实验 | 立 R3 旧实验迁移任务 |
| `gate_row@r3b_new_props.py:98-109` | r3b_new_props.py | 低 | R3b 旧实验 | 同上 |
| `_baseline_row@r5_emphasis_feedback.py:83-89` | r5_emphasis_feedback.py | 低 | R5 旧实验 | 同上 |
| `run_subject@r5_emphasis_feedback.py:92-121` | r5_emphasis_feedback.py | 低 | R5 旧实验 | 同上 |

### 4. reads-only（24 个，不需迁移代码，仅需评估依赖）

- 这些函数读 verdict 字段但**不调 maturation**，决策不依赖 verdict
- 多数是 **分析/汇总函数**（`_latest_v3_verdict`, `_build_rationale`, `_noise_floor` 等）
- 建议：T6D-04 阶段评估每个 reads-only 函数的"读取用途"——若仅用于日志/审计，**不需迁移**；若用于决策，需迁

## 建议迁移顺序

按依赖关系排序（critical path 优先）：

### 第一批（critical 路径必须先改）

1. `build_docket` + `submit_signoff`（layer2_signoff.py）—— Layer 2 人签核核心
2. `assess_maturation_v2`（maturation_gate.py）—— 闸内部，依赖升级会传导到所有 v2/v3 caller

### 第二批（中优先级，与 v3 闸一致化）

3. cascade_ng_probe / basis_split_heal_probe / flywheel_microcircuit_probe / lightweight_mode_probe / real_guidance_run / refine_loop_probe / reflexive_signing_probe 的 main 函数

### 第三批（低优先级，旧实验可独立任务）

4. r3a_gate_v2 / r3b_new_props / r5_emphasis_feedback 的历史实验函数

### 第四批（reads-only 评估）

5. 24 个 reads-only 函数的"读取用途"评估——多数不需迁移代码

### 风险点

- **layer2_signoff 是 critical 路径**：人签核接口——任何改动需测试 PRO-07 路由守卫（boundary 不进 Layer 2）+ PRO-10-c tier 守卫（核心公理 facet 不碰）+ 不可逆守卫（promote 须 rationale）+ PRO-08 留痕守卫
- **maturation_gate 是闸工厂**：v2 改 v3 影响所有 v2 caller——需先跑 `batch_reevaluate_v3*` 验证 v3 闸稳定性
- **mixed `_cmd_route` 不再改**：A4 已修，v1 verdict 保留 audit 用途

## 不实施声明

本任务包**不实施任何函数迁移**——只诊断。

T6D-04 立后续任务包实施。T6D-04 范围：

- 实施第一批 critical 路径迁移（layer2_signoff 2 函数 + maturation_gate v2 函数）
- 跑 audit_pipeline 验 A4 PASS（无跨族冲突）
- 跑 pytest 验 217+ 全过
- 评估 reads-only 函数是否需迁移

## 关联文件

- 诊断脚本：`task-packages/fix-governance-boundaries-t6d-v1-scan.py`（commit 5284a0f）
- 诊断输出：`task-packages/fix-governance-boundaries-t6d-v1-scan-output.json`
- 任务包：`task-packages/fix-governance-boundaries-t6d.md` §3.A-1
- 上游教训：`task-packages/fix-failures-t6d-results.md` F4.1.1（45+ 跨族函数根因）
- 参考判定：`sih-engine/skills/sihankor-proposition-defense/probes/check_verdict_consistency.py`（v1+v2 同族 family 分类）
- 后续任务包：T6D-04（待立文）
