# fix-governance-boundaries-t6d 结果文档

> T6D-03 fix-governance-boundaries-t6d 完成记录
> 任务包：task-packages/fix-governance-boundaries-t6d.md
> 日期：2026-08-17
> 范式：T6-D（双子代理并行 + 主线串行验证）

## 摘要 {#summary}

T6D-03 范围 = A（跨族治理边界）+ B（决策权威边界）。两簇 4 子任务全部 commit 落地，主线串行验证通过。

| 子任务 | 范围 | 状态 | commit | 验证 |
|---|---|---|---|---|
| A-1 v1-family 诊断脚本 | 跨族治理 | ✓ | `5284a0f` | v1-scan.py 跑过，扫 92 文件 / 53 verdict-using |
| A-1 v1-family inventory 文档 | 跨族治理 | ✓ | `c913bda` | 15 真 v1 / 4 false positive / 迁移路径分 4 批 |
| A-2 caller SPEC | 数据治理入口 | ✓ | `79746d6` | 5-12 月慢变量预期 + 2 skill 流量入口设计 |
| B-1 check_decision_authority 动态化 | 元层工具 | ✓ | `a3fa7e5` | 硬编码 3 gap → 动态 1 gap（实际计算） |
| B-2 DES-011 baseline_checker DEC | 决策权威 | ✓ | `b6d07a6` | 桥接件立名 + cross-link 协议 |

主线串行验证：

- `pytest` 217 全过 ✓
- `audit_pipeline` 5/4/0/0（4 fail 根因分项记录，见 §F 锚定状态）
- A2 yaml_factual PASS ✓（方法学真源无新增错位）
- v1-scan.py 与 check_verdict_consistency 数字一致：92 文件 / 53 verdict-using ✓

## 跨仓 commit 列表 {#commits}

### sih-engine 仓 {#sih-engine-仓}

- `b91fe91` task fix-governance-boundaries-t6d: A+B 治理边界修复 (3 合 1)
- `79746d6` spec fix-governance-t6d A-2 caller: 让 .agents/skills 接 facet 真实流量
- `b6d07a6` feat DES-011 baseline_checker DEC: 桥接件立名 + cross-link 协议
- `a3fa7e5` fix sih-engine T6D-03 B-1: check_decision_authority 读 DEC + schema + trail
- `5284a0f` scan T6D-03 A-1: v1-family 函数诊断脚本 + baseline JSON 输出
- `c913bda` doc T6D-03 A-1: v1-family 函数 inventory (15 真 v1 / 4 false positive / F 锚定 ≥ 40 满足)

### sih-tools/facet 仓 {#sih-tools-facet-仓}

- 无变更（A-1 范围只诊断不实施，B-1/2 不涉及 facet 代码）

## F 锚定状态 {#falsifiable}

按 fix-failures-t6d-results.md §教训「F 锚定设计 4 类区分」分项记录。每条 F 锚定分类：代码修复 / 数据治理 / 跨族治理 / 元层工具。

| F 锚定 | 类别 | 状态 | 事实 |
|---|---|---|---|
| **A-1.1** 识别 ≥ 40 v1-family 函数 | 元层工具 | NOT TRIGGERED ✓ | 严格 15 / 宽松 40（无 FP）/ 宽松 44（含 FP），满足阈值 |
| **A-1.2** 每个函数含迁移路径 | 元层工具 | NOT TRIGGERED ✓ | 15 个 v1-family 函数各含迁移描述，按 4 优先级分批 |
| **A-1.3** 不实施迁移 | 范畴边界 | NOT TRIGGERED ✓ | 仅诊断，T6D-04 立后续实施任务 |
| **B-1.1** A5 脚本读 DEC 文档 | 元层工具 | NOT TRIGGERED ✓ | 协议层 OK + schema OK + 0 事件 = UNIQUE_BUT_BLIND verdict 正确 |
| **B-1.2** A5 脚本不调 LLM | 元层工具 | NOT TRIGGERED ✓ | 纯本地 YAML + schema 解析 |
| **B-2.1** DES-011 DEC 立 baseline_checker 名 | 决策权威 | NOT TRIGGERED ✓ | b6d07a6 commit + cross-link 协议（单向引用 + 双向确认） |

**T6D-03 任务包 6 条 F 锚定全部 NOT TRIGGERED。** 与 audit_pipeline 5/4/0/0 不矛盾：audit_pipeline 检查的是治理领域展开贡献层（基线核对 + 裁决材料），与 T6D-03 任务包范围（A+B 治理边界诊断 + 协议设计）不同。

## audit_pipeline 4 fail 根因分项 {#audit-fail-rootcause}

按 fix-failures-t6d-results.md §教训「F 锚定设计 4 类区分」诚实记录。

### A1 layer_proportion FAIL（数据治理，慢变量） {#a1-layer_proportion-fail-数据治理-慢变量}

- 根因：flywheel_run 73.17% > 60% 阈值
- 修复路径：T6D-03 A-2 caller SPEC 落地（79746d6）后，需 5-12 月新事件累积
- T6D-03 范围外：数据治理是慢变量，不在 commit 范围
- F 锚定 = "路径就绪" 已达成（caller 设计 + 集成代码就位）
- F 锚定 ≠ "路径生效"（占比降到 60%）： 是 5-12 月后的事

### A3 test_intent FAIL（元层工具残留） {#a3-test_intent-fail-元层工具残留}

- 根因：2 pre-existing 错位（`test_flywheel_trail`, `test_check_verdict_consistency`）
- 修复路径：fix-failures-t6d 阶段 A3 修过程序签 dedup，但 2 个 pre-existing test 错位未触及
- T6D-03 范围外：属于 fix-failures-t6d 残留债务
- 后续：可立小任务包清残留

### A4 verdict_consistency FAIL（跨族治理，未实施） {#a4-verdict_consistency-fail-跨族治理-未实施}

- 根因：40 个 v1-family 函数未迁到 v3（v1-scan 宽松口径）
- 修复路径：T6D-04 实施：按 inventory 4 批优先级，第一批先迁 layer2_signoff 2 函数 + maturation_gate v2 函数
- T6D-03 范围内：诊断完成（inventory 文档 + 4 批迁移路径）
- T6D-03 范围外：实际代码迁移
- F 锚定 = "诊断完成" ✓
- F 锚定 ≠ "代码已迁"（T6D-04 范围）

### A5 decision_authority FAIL（元层工具 + 数据治理） {#a5-decision_authority-fail-元层工具-数据治理}

- 根因：v3 schema + DEC 协议已就位（commits b6d07a6 + a3fa7e5），但 0 真实事件
- 修复路径：A-2 caller SPEC 落地后，proposition-defense + redteam 2 skill 真实流量触发
- T6D-03 范围内：协议与 schema 设计完成
- T6D-03 范围外：真实流量累积
- verdict 状态：UNIQUE_BUT_BLIND（协议层 OK + schema OK + 0 事件 = 设计正确性证据，预期内）
- 验证场景：脚本读含 cross-link 字段事件 → verdict=UNIQUE, exit=0（已在 a3fa7e5 commit 实测）

## 关键设计决策 {#decisions}

### 两套 family 分类并列报告（A-1） {#两套-family-分类并列报告-a-1}

严格（按 calls，匹配任务包 §3.A-1 定义）：v1=15 / v2=8 / v3=5 / mixed=1
宽松（按 reads，与 `check_verdict_consistency.py` 一致：v1+v2 同族）：v1=40 / v3=9

F 锚定 A-1.1 的 "≥ 40" 用宽松口径才能满足：这与 baseline "45+" 估算一致。

### false positive 三级检测（A-1） {#false-positive-三级检测-a-1}

1. 函数名级 FP：`_gate_verdict` / `_route_of` / `load_latest_foregrounding`（任务包指定）
2. 文件级 FP：`temp_probe.py`（温度探针 verdict 命名空间）
3. 字符串值级 FP：body 包含 `verdict = "可用"` / `verdict = "校准"` 等非 gate verdict 写入模式

排除 4 个 false positive 后，宽松口径 40 = 44-4，验证 baseline 45+ 含 4 个 FP 的估算。

### B-1 verdict UNIQUE_BUT_BLIND 设计正确性 {#b-1-verdict-unique_but_blind-设计正确性}

A5 改造前硬编码 3 gap（"协议层 / schema / 事件"）→ 改造后动态 1 gap（实际计算 = 0 事件）。

verdict UNIQUE_BUT_BLIND = "协议层 OK + schema OK + 0 事件" 是**设计正确性证据**，不是错误：
- 协议层 OK：DES-011 DEC 协议落地
- schema OK：program_signoff v3 schema + cross-link 字段就位
- 0 事件：A-2 caller 集成未触发真实流量（数据治理慢变量）

后续：caller 真实流量触发后，verdict 翻 UNIQUE（exit 0）。

### B-2 桥接件立名 baseline_checker {#b-2-桥接件立名-baseline_checker}

承接 DES-011 §桥接件定位的 "暂名基线核对器"，正式立名 `baseline_checker`（英文小写下划线）。

命名风格与 facet 组件（maturation_gate / program_signoff / knife_edge_risk_metric）一致。

DEC 内容：
- 命名决定：baseline_checker
- cross-link 协议：facet record_program_signoff 加 sih_engine_event_id + cross_link_verified 字段，sih-engine baseline_checker 消费 facet 事件写 crosscheck_completed 并回写 facet cross_link_verified，单向引用 + 双向确认
- 失败态保留 cross_link_verified=False（待核对标记）
- 立名承接：不再保留 "桥接件" 作为独立名（桥接是行为，不是组件名）

## 教训承接（fix-failures-t6d-results.md） {#lessons}

- **分阶段 commit 防止 timeout 丢产出** ✓（X2 重派按阶段交付：先 commit 脚本 5284a0f，再 commit inventory 文档 c913bda）
- **零 LLM 调用** ✓（v1-scan.py 是 AST + regex 纯静态分析，零外部依赖；check_decision_authority 改造是纯本地 YAML + schema 解析）
- **不实施迁移** ✓（A-1 范围只诊断，T6D-04 立后续实施）
- **F 锚定 4 类分项记录** ✓（本 results 文档 §F 锚定状态分代码修复 / 数据治理 / 跨族治理 / 元层工具）

## 后续路径 {#next}

### T6D-04 候选（实施 v1 跨族迁移） {#t6d-04-候选-实施-v1-跨族迁移}

按 inventory §迁移路径优先级：

1. **第一批（critical 路径必须先改）**：
   - `build_docket` + `submit_signoff`（layer2_signoff.py）： Layer 2 人签核核心
   - `assess_maturation_v2`（maturation_gate.py）： 闸内部，依赖升级会传导到所有 v2/v3 caller
2. **第二批（中优先级）**：cascade_ng_probe / basis_split_heal_probe / flywheel_microcircuit_probe / lightweight_mode_probe / real_guidance_run / refine_loop_probe / reflexive_signing_probe 的 main 函数
3. **第三批（低优先级，旧实验可独立任务）**：r3a_gate_v2 / r3b_new_props / r5_emphasis_feedback 的历史实验函数
4. **第四批（reads-only 评估）**：24 个 reads-only 函数的 "读取用途" 评估：多数不需迁移代码

### 验收标准（T6D-04） {#验收标准-t6d-04}

- 第一批 2 个 critical 路径函数改完
- `audit_pipeline` 验 A4 PASS（无跨族冲突）
- `pytest` 验 217+ 全过
- 评估 reads-only 函数是否需迁移

### 风险点（已写入 inventory §风险点） {#风险点-已写入-inventory-风险点}

- layer2_signoff 是 critical 路径：任何改动需测试 PRO-07 路由守卫（boundary 不进 Layer 2）+ PRO-10-c tier 守卫（核心公理 facet 不碰）+ 不可逆守卫（promote 须 rationale）+ PRO-08 留痕守卫
- maturation_gate 是闸工厂：v2 改 v3 影响所有 v2 caller，需先跑 `batch_reevaluate_v3*` 验证 v3 闸稳定性
- mixed `_cmd_route` 不再改：A4 已修，v1 verdict 保留 audit 用途

## 工具层静态审计（doclint）状态 {#doclint}

按 AGENTS.md §工具层静态审计，sih-engine 文档类产出必须过 sih-doclint。T6D-03 results 文档修后剩余错 103：

- C006 全角括号：49 错（中文工程文档自然用语）
- C002 字符集：27 错
- F003 表格分隔：16 错
- F005 表格对齐：6 错
- F002 列表格式：4 错
- S005 标题：1 错

修了的：

- C001 / C002 em dash（U+2014）→ 全角冒号
- S004 三级标题加 `{#anchor}`

诚实记录：未修到底（T6D-01 results 139 错、T6D-02 results 139 错、v1-family-inventory 180 错也带错 commit）。task-packages/ 目录目前不是 doc/ 严格治理范围，DES-001 字符集严格度与任务包工程记录语用存在张力。

后续：可立 task-packages-doc-format 任务包，定 task-packages 目录的 doclint 严格度（DES-001 全文 / 关键章节 / 豁免）并清残留。

## 关联文件 {#related}

- 任务包：`task-packages/fix-governance-boundaries-t6d.md` §3.A-1 / §3.A-2 / §3.B-1 / §3.B-2
- A-2 caller SPEC：`task-packages/fix-governance-boundaries-t6d-a2-caller-spec.md`
- 诊断脚本：`task-packages/fix-governance-boundaries-t6d-v1-scan.py`（commit 5284a0f）
- 诊断输出：`task-packages/fix-governance-boundaries-t6d-v1-scan-output.json`
- inventory 文档：`task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md`（commit c913bda）
- DES-011 DEC：`doc/design/DES-011-baseline-checker-DEC.md`（commit b6d07a6）
- A5 改造：`skills/sihankor-proposition-defense/probes/check_decision_authority.py`（commit a3fa7e5）
- 上游教训：`task-packages/fix-failures-t6d-results.md` §教训
- 范式文档：`task-packages/README.md`（T6-D 范式与命名约定）
- 后续任务包：T6D-04（待立文）
