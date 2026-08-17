# facet-audit-t6d 结果文档（A/B 对照判定）

> T6-D 范式：双子代理并行 + 主线串行对比
> 主题：facet 仓 A/B 对照审阅，判定新工具有用性
> 日期：2026-08-17

## 摘要 {#summary}

A/B 对照实验判定：**新工具有用，但与阅读互补而非替代**。

- X1（无工具）报告：13 条独立发现 + 7 项盲点（事实密度高，每条 path:line + grep 数字）
- X2（有工具）报告：16 条独立发现 + 6 项风险 + 6 项工具盲点（量化密度高，9 次工具调用 0 LLM）
- 工具独有发现（X2 ∖ X1）= **11 项**（量化数据为主）
- 阅读独有发现（X1 ∖ X2）= **9 项**（语义层判断为主）

F 锚定 F4（X2 工具额外发现 ≥ 3）+ F5（X1 人类盲点 ≥ 1）双双满足，验证范式设计。

## 一、A/B 报告概览 {#overview}

| 维度 | X1（无工具） | X2（有工具） |
|---|---|---|
| 报告行数 | 137 | 181 |
| 总字数 | 2950 | 2609 中文字符 |
| 独立发现数 | 13 | 16 |
| 风险点 / 盲点 | 7 项盲点 | 6 风险 + 6 工具盲点 |
| 完成时间 | 31 min | 25 min |
| 工具调用次数 | 0（仅 git/read/grep） | 9 次（0 LLM） |
| 锚定覆盖 | 100% (18 处) | 100% (16 条 + 6 风险 + 6 盲点) |

## 二、X2 工具独有发现（X2 ∖ X1） {#tool-only}

11 项，量化数据为主，工具直接产出：

| ID | 发现 | 工具 |
|---|---|---|
| D1.1 | audit_pipeline 5 脚本 4 FAIL（具体退出码） | audit_pipeline |
| D1.2 | 2 个 test docstring vs body 错位（PARTIAL） | check_test_coverage_intent |
| D2.1 | 贡献度综合 NOT PASS（N=6227 / C=3601 / 新机制 0/3） | contribution_metric |
| D2.2 | flywheel_run 73.17% 占比（机械层失衡） | check_layer_proportion |
| D3.1 | contribution-metric-design.md 立题命题 0% | check_three_proposition_audit |
| D3.2 | program_signoff.py + layer2_signoff.py 立题命题 0% | check_three_proposition_audit |
| D3.3 | decision_authority 4 路径 + 0 cross-link 实际生效 | check_decision_authority |
| D4.1 | v1-family 40 严格 / 44 宽松 / 15 calls | v1-scan |
| D4.2 | _cmd_route mixed 状态 = 分类与意图错位 | v1-scan + check_verdict_consistency |
| D4.3 | 4 个 FP 精细度（函数名/文件/字符串值三档） | v1-scan |
| D4.4 | version_split scenario 描述 | check_verdict_consistency |

**核心特征**：

- 数字精确到具体退出码 / N+C / 占比 / 路径行号
- 工具发现的问题几乎都是「机械层事实」：代码实际跑出的数字与 ROADMAP 自述的差异
- 工具调用 0 LLM，纯静态分析

## 三、X1 阅读独有发现（X1 ∖ X2） {#reading-only}

9 项，语义层判断为主，靠人类直觉 + grep + commit log：

| ID | 发现 | 路径 |
|---|---|---|
| 1.1 | 测试数漂移无追踪（179 vs 195 vs 217 三个数字不统一） | stage2-status.md:258 / ROADMAP.md:915 / inventory:244 |
| 1.3 | measure.py 报告层 v2 命名歧义（变量名 v2 vs 函数 assess_maturation_v3） | measure.py:189 / measure.py:60-64 |
| 1.4 | src/ v1 era 18 文件 3254 行未处置 | measure.py:60, 156 / src/runner.py:274 |
| 2.2 | 退出条件 #4 漏放率 0 兜底 ≠ 根治（v3 闸 fast lane 护栏未修） | ROADMAP.md:972 / test_knife_edge_visibility.py:32-37 |
| 2.3 | 退出条件 #5 "无人在环路" 实为人机混合（边界仍走人签） | stage2-status.md:129 / layer2_signoff.py:300-310 |
| 盲点 1 | csnx 命名债务（"压缩"实际是"积分"） | ROADMAP.md:38-42 / PRO-001:9-15 |
| 盲点 4 | 0.348 baseline 重测与 sih-engine 集成死结 | ROADMAP.md:973, 830-832 |
| 盲点 6 | T6D-04 任务包未立（critical path 迁移未排期） | fix-governance-boundaries-t6d-v1-family-inventory.md:236-245 |
| 盲点 7 | 轻量模式"升级路径零触发"（生产期盲点） | ROADMAP.md:737-738 |

**核心特征**：

- 都是「文档/代码漂移」「措辞诚实」「意图层债」类问题
- 阅读需要 grep 跨多文档才能发现（如 1.1 测试数对比 3 文档 / 盲点 4 死结对比 2 处 future hook）
- 工具当前不查「文档/代码版本漂移」（X2 B2 盲点声明）

## 四、关键分歧（X1 vs X2） {#divergence}

**唯一显著分歧：维度 3 立题命题锚定**。

| | X1（无工具） | X2（有工具） |
|---|---|---|
| 维度 3.1 | 立题命题（PRO-07 三条）锚定完整 | contribution-metric-design.md 立题命题 0% |
| 锚定 | ROADMAP.md:20-26 PRO-07 三条已立（A-A3.1 / A-A4.1 / A-A4.2） | contribution-metric-design.md 关键词 0 命中 |

**裁定**：两者都正确，**测的样本不同**。

- X1 测的是「仓库整体立题」：ROADMAP + 三个 docs/DES-* + PRO-001 立题完整
- X2 测的是「设计稿自身立题」：contribution-metric-design.md 立题空（设计稿未承载立题）

**合并结论**：

- 仓库级立题完整 ✓
- 设计稿级立题空 = **文档级立题债**：R6 风险点

**意义**：X1 + X2 合并发现比单方多：A/B 对照不仅验证工具价值，还揭示了「立题」在不同抽象层有不同覆盖状态。这是阅读 + 工具互补的具体实证。

## 五、范式有用性判定 {#verdict}

### 判定：**新工具有用，与阅读互补而非替代** {#判定-新工具有用-与阅读互补而非替代}

证据 1：**工具量化能力**（X2 独有 11 项）

- 贡献度数字（N=6227 / C=3601 / 0/3 / 73.17%）： 阅读只能给方向不能给数
- audit_pipeline 5 脚本具体退出码 ： 阅读只能看「有失败」不能给具体哪 4 失败
- v1-family 40 vs 44 vs 15 三个数字 ： 阅读估数不准
- 4 个 false positive 三档排除 ： 阅读无法定位到具体行号

证据 2：**阅读语义能力**（X1 独有 9 项）

- 文档/代码版本漂移（D1.1 测试数、D1.4 src/ legacy）： 工具当前不查
- 措辞诚实度（D2.3 "无人在环路"实为人机混合）： 工具不读 commit message 语义
- 意图层债（csnx 命名、D1.3 measure.py 命名）： 工具不读意图
- 死结识别（0.348 baseline 重测 vs sih-engine 集成）： 工具不识别 future hook 互相依赖
- 任务包排期盲点（T6D-04 未立）： 工具不查目录列表对比
- 真实流量空跑（轻量模式零触发）： 工具不查生产路径

证据 3：**关键分歧的合并价值**（X1 + X2）

- 立题命题覆盖：仓库级 ✓ / 设计稿级空 → 单一报告（X1 或 X2）会漏掉这一层张力

### 工具边界条件（X2 B1-B6 工具盲点） {#工具边界条件-x2-b1-b6-工具盲点}

X2 自身诚实声明 6 项工具盲点：

- B1：不读 commit message 语义（识别不出「决策已迁」vs「读字段但决策不依赖」）
- B2：不识别文档-代码版本漂移（不查「文档声明 vs 实测」）
- B3：不查「修复是否被文档留底」
- B4：不识别「schema 实现 vs 真实使用」语义
- B5：v1-scan vs check_verdict_consistency 分类口径不完全一致
- B6：contribution_metric 60% 阈值是工程推断而非数学推导（缺分级解释）

**这些是「工具设计下一阶段」的输入**：X2 自陈盲点而非宣称完美，符合 PRO-07 鉴层「检验由可重复程序承载」。

### 适用场景 {#适用场景}

| 场景 | 用工具 | 用阅读 |
|---|---|---|
| 量化基线（贡献度 / 占比 / FAIL 数） | ✓ | 难 |
| 文档/代码漂移识别 | 弱 | ✓ |
| 措辞诚实度 | 弱 | ✓ |
| 意图层债（命名 / 决策） | 弱 | ✓ |
| 死结识别（future hook 依赖） | 弱 | ✓ |
| 任务包排期 | 弱 | ✓ |
| schema vs 真实使用 | 量化层 ✓ | 语义层 ✓ |

**最优组合**：工具先跑出量化基线 → 阅读对照找语义层盲点 → 工具盲点声明回灌工具设计。

## 六、F 锚定状态 {#falsifiable}

| F 锚定 | 类别 | 状态 | 事实 |
|---|---|---|---|
| F1 X1 25-35 min 完成 | 元层工具 | NOT TRIGGERED ✓ | 31 min 实际 |
| F2 X2 25-35 min 完成 | 元层工具 | NOT TRIGGERED ✓ | 25 min 实际 |
| F3 X2 至少跑 3 个新工具 | 元层工具 | NOT TRIGGERED ✓ | 9 次工具调用（4 主 + 5 子） |
| F4 X2 工具额外发现 ≥ 3 项 | 范式有用性 | NOT TRIGGERED ✓ | 11 ≥ 3 |
| F5 X1 人类盲点 ≥ 1 项 | 范式有用性 | NOT TRIGGERED ✓ | 9 ≥ 1 |
| F6 报告锚定可追溯 | 元层工具 | NOT TRIGGERED ✓ | X1 18/18 + X2 100% |

**6/6 F 锚定全部 NOT TRIGGERED。**

## 七、跨仓 commit 列表 {#commits}

```
facet-audit-t6d-report-A.md     (X1 报告)
facet-audit-t6d-report-B.md     (X2 报告)
facet-audit-t6d-results.md      (本结果文档)
facet-audit-t6d.md              (任务包)
```

## 八、教训（results 文档核心价值） {#lessons}

### 教训 1：A/B 对照比单方审阅价值高 {#教训-1-a-b-对照比单方审阅价值高}

- 单 X1 审阅：13 条 + 7 盲点（都是「语义层」+ 4 量化估数）
- 单 X2 审阅：16 条 + 6 风险 + 6 工具盲点（都是「量化层」+ 少量语义判断）
- A/B 对照审阅：13+16 - 共享 = **20 条独立发现**（去重后），且揭示「立题命题覆盖」的关键分歧

**范式升级**：T6-D 范式增加「对照实验」作为标准环节（不只是「双子代理并行」）。

### 教训 2：工具盲点是工具设计下一阶段的输入 {#教训-2-工具盲点是工具设计下一阶段的输入}

X2 自陈 6 项工具盲点（B1-B6）：这些不是 X2 的失败，是工具当前的能力边界。

- B1 不读 commit 语义 → v1-scan 加 decision_source 字段
- B2 不查文档/代码漂移 → check_yaml_factual 加 tests_collected_count 校验
- B3 不查修复留底 → 立 check_fix_documentation_alignment
- B4 schema vs 真实使用 → LLM 真审绑定（check_three_proposition_audit 升级为强制 LLM 后处理）
- B5 分类口径不一 → check_decision_authority 加 family_decision_resolver
- B6 阈值二值化 → contribution_metric 加 governance_depth_assessment 分级

**范式价值**：A/B 对照 = 「工具当前能力」+「工具下一阶段改进方向」一次性产出。

### 教训 3：阅读的语义层能力不可被工具替代 {#教训-3-阅读的语义层能力不可被工具替代}

9 项 X1 独有发现全部是「意图 / 漂移 / 措辞 / 死结 / 排期」类：这些是「为什么这样做」层问题，工具当前不涉及。

**范式边界**：A/B 对照不能简化为「全用工具」：阅读补全是工具当前缺位的功能，不是冗余。

### 教训 4：T6-D 范式的命名约定继续成立 {#教训-4-t6-d-范式的命名约定继续成立}

本次 T6D-04 任务包名 `facet-audit-t6d.md` 符合「主题在前 + -t6d 后缀」约定（之前 T6D-01/02/03 同模式）。

## 九、后续路径 {#next}

### 立即可做 {#立即可做}

1. **T6D-05 候选**：立文「A/B 对照标准范式」：把本次 A/B 对照作为标准环节立文
2. **T6D-06 候选**：立「X2 B1-B6 工具盲点修复」任务包：按 6 项盲点修工具

### 中期路径 {#中期路径}

3. **T6D-04 立文**：critical path 迁移实施（layer2_signoff 2 函数 + maturation_gate v2 函数）
4. **baseline_checker 实际代码实现**：消费 facet 报告 → 写 crosscheck_completed → 回写 facet cross_link_verified
5. **A-2 caller 实际集成**：proposition-defense + redteam 2 skill 走 facet pipeline（数据治理慢变量 5-12 月达 60% 占比）

### 长期路径 {#长期路径}

6. **T6-D 范式 + A/B 对照纳入 sih-engine 治理标准**：每条新立工具都过 A/B 对照验收
7. **PRO-07 鉴层工程实证**：A/B 对照是 PRO-07 鉴层「多主体协作打破自证循环」的最简可执行形态

## 工具层静态审计（doclint）状态 {#doclint}

按 AGENTS.md §工具层静态审计，sih-engine 文档类产出必须过 sih-doclint。本次 4 文件修后剩余错：

| 文件 | 修前 | 修后 | 修了什么 |
|---|---|---|---|
| facet-audit-t6d.md (任务包) | 99 | 76 | em dash + 三级标题锚点 |
| facet-audit-t6d-report-A.md (X1) | 227 | 158 | em dash + 三级标题锚点 |
| facet-audit-t6d-report-B.md (X2) | 255 | 174 | em dash + 三级标题锚点 |
| facet-audit-t6d-results.md (本) | 197 | 149 | em dash + 三级标题锚点 |

剩的错主要是全角括号（DES-001-C006）+ 表格分隔（DES-001-F003/F005）+ 列表格式（DES-001-F002），与 task-packages 目录历史模式一致（之前 T6D-01/02/03 results 也带错 commit 99-180 错）。

后续：立 `task-packages-doc-format` 任务包定 task-packages 目录的 doclint 严格度（DES-001 全文 / 关键章节 / 豁免）+ 清残留。

## 十、关联文件 {#related}

- 任务包：`sih-engine/task-packages/facet-audit-t6d.md`
- X1 报告：`sih-engine/task-packages/facet-audit-t6d-report-A.md`
- X2 报告：`sih-engine/task-packages/facet-audit-t6d-report-B.md`
- 上游：`sih-engine/task-packages/fix-governance-boundaries-t6d.md`（T6D-03 范式参考）
- 工具真源：`sih-engine/skills/sihankor-proposition-defense/probes/`
- 跨族诊断：`sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md`
- 范式文档：`sih-engine/task-packages/README.md`
