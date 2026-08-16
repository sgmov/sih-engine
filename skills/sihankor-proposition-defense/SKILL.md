---
name: "sihankor-proposition-defense"
description: "Defend SiHankor propositions and reports from three-layer audit gaps. Invoke when reviewing a report/proposition/design, when the user asks to 'audit' / 'stress-check' / 'defend' a claim, or when a facet/audit/decision-output report may have missed the 立题命题 / 应用命题 / 治理领域展开贡献 三层覆盖. Use the embedded script to mechanically flag missing layers, then complete the audit with LLM reasoning."
---

# SiHankor Proposition Defense Skill

司衡命题防御：防止司衡工程产物（报告 / 命题 / 审计 / 决策产出）在外部审阅中漏掉三个命题层次的角度。

## 背景：为什么需要这个 skill {#background}

司衡工程的很多外部审阅缺角度，根源是审阅者**没意识到要查三个层次**：

- **立题命题层**（foundation proposition）：被审对象是否在立题范围内？审阅者是否把超出立题的事混进来？
- **应用命题层**（application proposition，PRO-07 锚定）：被审对象是否真被应用命题覆盖？A-A3.1 鉴层破自证循环 / A-A4.1 候选建议生成器 / A-A4.2 裁决权归确定性引擎——三命题都被审到吗？
- **治理领域展开贡献层**（governance contribution）：被审对象对司衡治理领域**真贡献**是什么？N + C 工具成熟度 ≠ 治理贡献 = 上下文风洞 / 权责归一 / 信息洪流的实际展开。

漏任一层 = 审阅不充分，**不管工程数据多好看**。

skill 设计原则：**方法学注入（LLM 主动调）+ 机械漏层检查（脚本兜底）双层防御**。

## 触发时机 {#trigger}

用户表达以下意图时触发

- "审一下这份报告"、"audit 一下"、"review 一下"
- "这个命题站得住吗"、"这个设计决策对不对"
- 收到 facet / redteam / 任何 LLM 工具产出的报告，要求真判断
- 起草 / 提交 / commit 任何 PRO / DEC / DES 文档前自检
- 阶段交付前的"外部审阅"

不触发

- 单纯事实查询
- 不涉及命题 / 报告 / 设计的纯讨论
- 已明确由 sihankor-redteam / sihankor-facet-measure 处理的窄场景（这两个 skill 内部已含方法学）

## 前置检查 {#precheck}

1. `sih-engine/skills/sihankor-proposition-defense/probes/check_three_proposition_audit.py` 在
2. `.agents/skills/sihankor-proposition-defense/` 投影存在（AGENTS.md 约定权威源在 sih-engine/skills/）
3. 报告 / 命题文件路径已确定

## 执行流程 {#flow}

按以下顺序执行。

### 1. 跑机械漏层检查 {#step-script}

```bash
python sih-engine/skills/sihankor-proposition-defense/probes/check_three_proposition_audit.py <report.md> --json
```

输出 JSON：

```json
{
  "file_path": "...",
  "layers": [
    {"name": "立题命题", "matched_keywords": [...], "coverage_ratio": 0.3, "covered": true},
    {"name": "应用命题", "matched_keywords": [...], "coverage_ratio": 0.0, "covered": false},
    {"name": "治理领域展开贡献", "matched_keywords": [...], "coverage_ratio": 0.0, "covered": false}
  ],
  "all_covered": false,
  "missing_layers": ["应用命题", "治理领域展开贡献"]
}
```

`all_covered: true` = 三层都覆盖（机械检查通过），但**仍须 LLM 真审**。
`missing_layers` 非空 = 报告漏层，须 LLM 在漏层补审。

### 2. LLM 真审三层 {#step-llm}

不管脚本结果如何，LLM 必须按方法学真审三个命题层。每层**至少给一段实质判断**，不能只引关键词。

#### 立题命题层

问：被审对象是否在立题命题范围内？审阅者是否把 X 当作超出立题的事来审？

例：审 facet "错误方向相关性盲区"批评——这个批评是否在 facet 立题"异质性来自独立探索"范围内？错位批评 = 审阅者把"测语义对错"（超出立题）当 facet 该做的事。

判据：

- X 是否触及立题命题的核心词（异质性 / 独立探索 / 命题合法性）
- 审阅者是否在立题范围内讨论
- 错位批评 = 审阅者把 X 当成超出立题的事来审

#### 应用命题层

问：X 是否真被 PRO-07 / A-A3.1 / A-A4.1 / A-A4.2 覆盖？四个锚定都被审到吗？

- A-A3.1 自证循环：X 是否触及"多模型独立审阅真打破自证循环"？
- A-A4.1 候选建议生成器：X 是否触及"facet 不裁决只产报告"？
- A-A4.2 裁决权归确定性引擎：X 是否触及"裁决权不能下放给 LLM"？

判据：四个锚定中任一未审 = 漏层。

#### 治理领域展开贡献层

问：X 对司衡治理领域**真贡献**是什么？是工具成熟度（事件数 / 测试数）还是治理贡献（上下文风洞 / 权责归一 / 信息洪流的实际展开）？

判据：

- X 是否区分"工具成熟度"和"治理贡献"？
- X 是否测了"上下文风洞"在真实数据上的表现？
- X 是否触及"权责归一"原则？

### 3. 综合产出审计报告 {#step-report}

按以下结构输出

- **机械漏层检查结果**（脚本 JSON 摘要）
- **LLM 真审三层结果**（每层一段实质判断）
- **最终判定**：
  - 三层都过 + 工程数据合理 → ✅ 通过
  - 任一层未审或实质错位 → ❌ 报告不通过，列缺什么
  - 多次修正仍不通过 → 输出"需要人类指引"，附未通过项

### 4. 通知用户 {#step-notify}

给用户一份最终审计报告，**不替用户做工程决策**——审计结果由用户在自己上下文里做。

## 工具关键设计 {#design}

双层防御。方法学（LLM 主动调）+ 机械检查（脚本兜底）—— 两者各防一边：
- 方法学防"审阅者没意识到要查这层"
- 机械检查防"审阅者写了报告但漏了某层"

锚点表可扩展。`LAYER_ANCHORS` 字典是开源的，工程师可按领域加新锚点（不要删旧锚点，避免漏判）。新锚点加进去后**必须**回退跑一次现有报告，确认覆盖率不倒退。

不替代 LLM。脚本查"是否含某锚点"≠ "是否真审过"。真判断仍是 LLM + 人。脚本退出码 0 也不等于审计通过——是"机械层未漏"，LLM 层仍须按方法学真审。

退出码语义：

- 0 = 三层都覆盖（机械层未漏）
- 1 = 漏层（须 LLM 补审）
- 2 = 文件不可读

不预判内容。脚本只查锚点覆盖，不替 LLM 给"应该审什么"。漏什么列什么。

## 常见错误处理 {#errors}

锚点表误判。某关键词在报告里出现但语境不是真审（如 "应用命题" 在脚注里提一句）。LLM 须区分"真审" vs "提到"。脚本无法判断，由 LLM 处理。

锚点表漏列。新领域报告可能用新术语，脚本查不到锚点。LLM 应基于方法学真审，**即使脚本说 covered=false，LLM 仍须独立判断**——脚本是辅助不是判据。

strict 模式。`--strict` 标记只表示"用严格态度审"，不改变覆盖逻辑（已要求 ≥ 1 命中）。严格语义留给 LLM + 用户定。

跨仓使用。本 skill 在 sih-engine/skills/ 是权威源，跨工具投影到 .agents/skills/。两处内容须同步。

## 与其他 skill 的分工 {#relation}

- **sihankor-pre-output-self-check**：每条输出的 6 项自检（意图 / 范畴 / 责任 / 精简 / 哲学相容）—— **不重复**，本 skill 不做这 6 项
- **sihankor-redteam**：LLM 红蓝紫对抗审查（窄工具调用）—— **本 skill 是元层**，redteam 输出仍是报告，仍可被本 skill 审
- **sihankor-facet-measure**：facet 测温工具调用（窄工具）—— 同 redteam
- **sihankor-intent-refine**：意图提炼（生成前）—— 本 skill 是**生成后审阅**
- **sihankor-calculus-trigger**：微积分概念检索（推理工具）—— 本 skill 借它做数学正确性支撑

## 当前状态 {#status}

- 1 commit 落地：skill + 内嵌脚本（2026-08-16）
- 锚点表初始 3 层 × 中英文 5-10 关键词 / 层
- 测试：脚本 `--json` 退出码可机械校验
- 跨工具投影：待 .agents/skills/sihankor-proposition-defense/ 创建（AGENTS.md 约定）

### Baseline 漏层率（2026-08-16 跑）

跑 `probes/check_three_proposition_audit.py --json` 在现有 facet 报告上：

| 报告 | 立题命题 | 应用命题 | 治理领域展开贡献 | 漏层 |
|---|---|---|---|---|
| stage2-01-multiview-report.md | 14% ✅ | **0% ❌** | 7% ✅ | 应用命题 |
| stage2-02-baseline-report.md | **0% ❌** | **0% ❌** | 7% ✅ | 立题 + 应用 |
| stage2-03-program-signoff-report.md | **0% ❌** | **0% ❌** | 21% ✅ | 立题 + 应用 |
| stage2-04-knife-edge-report.md | **0% ❌** | **0% ❌** | 14% ✅ | 立题 + 应用 |
| stage2-04-knife-edge-retro.md | **0% ❌** | **0% ❌** | 14% ✅ | 立题 + 应用 |
| contribution-metric-design.md | **0% ❌** | 17% ✅ | 7% ✅ | 立题 |

**几乎所有报告都漏立题命题 + 应用命题层**——skill 设计的"反复防御"价值已实证。

## 下一步 {#next}

- **优先**：把每份现有报告按本 skill 三层方法学补审，输出补审报告
- 立项补充锚点表（用户提到"反复要遇到的防御挑战"——意味着新领域出现新锚点要加）
- 远期：redteam 工具接本 skill 做"对立锚定"对抗
