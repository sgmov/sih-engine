# 任务包 T6D-01：通过机械脚本减少 LLM 调用

> 立项原因（2026-08-17）：多 agent 独立审阅实验（`doc/research/multi-agent-independent-audit-2026-08-16.md`）证明 11 个真问题中 5-6 个可被机械脚本抓到（不调 LLM）。本任务包落地"机械检查优先，LLM 真审只针对机械未过部分"流水线。

## 一、问题陈述 {#problem}

**当前状态**：每次治理工程审阅都全量调 LLM 做三层审（立题 / 应用 / 治理展开）：但 11 个真问题中：
- **机械可代**（不需 LLM）：6 个
  - 锚点覆盖（`check_three_proposition_audit.py` 已立）
  - N+C layer 占比（N+C terminology drift）
  - v1 vs v3 裁决分叉
  - test docstring vs 实际测试
  - methodology.yaml 文字 vs 代码事实
  - 决策权威（程序签 vs 人签 vs DES-011 路径）
- **必须 LLM 真审**：5 个
  - 立题合法性
  - 同源预训练风险
  - A-A3.1 跨家族 vs 独立
  - 错位批评
  - 跨任务包对位

**目标**：日常任务走机械检查（零 LLM），LLM 审只针对机械未过的部分：LLM 调用次数减少 80%。

## 二、关键设计 {#design}

### 2.1 流水线架构 {#2-1-流水线架构}

```
[commit / 任务完成]
  ↓
[机械检查 5 脚本并行跑（零 LLM）]
  ├─ 通过 → 跳过 LLM 审
  └─ 失败 → 触发 LLM 审"机械未过的部分"（不是全文）
```

### 2.2 5 个机械脚本设计原则 {#2-2-5-个机械脚本设计原则}

- **零 LLM 调用**：纯 grep / parse / 比较
- **可机械校验**：JSON 输出 + 退出码
- **跨仓可跑**：跑 facet / sih-engine / redteam 任一仓
- **启动时校验**（与 check_three_proposition_audit.py 同模式）：yaml schema / 依赖文件存在

### 2.3 T6-D 集群编排 {#2-3-t6-d-集群编排}

**Cluster 1: 5 脚本并行写**（5 子任务 A1-A5 独立）

- 子代理 X1：写 A1（layer_proportion）+ A2（yaml_factual_consistency）
- 子代理 X2：写 A3（test_coverage_intent）+ A4（verdict_consistency）
- 主线本会话：写 A5（decision_authority）： 因 A5 涉及多仓路径分析，最复杂

**Cluster 2: 流程改造**（依赖 Cluster 1 完成）

- 子代理 X1 后续：commit hook 配置
- 子代理 X2 后续：流水线脚本（机械检查 → 触发 LLM 审）

**主线（root session）**：

- 跑测试 / 复算贡献度 / 任务包验收 / 决策
- Cluster 1 收齐后做机械层回归（5 脚本全跑现有报告 + 现有代码）
- Cluster 2 收齐后做端到端测试（机械检查全过 → 确认 0 LLM 调用）

## 三、工作清单 {#work}

### Cluster 1：5 个机械脚本（独立可并行） {#cluster-1-5-个机械脚本-独立可并行}

#### A1. `check_layer_proportion.py`（优先：价值/成本最高） {#a1-check_layer_proportion-py-优先-价值-成本最高}

- 路径：`sih-engine/skills/sihankor-proposition-defense/probes/check_layer_proportion.py`
- 输入：trail root（默认 `sih-tools/proposition/DES/`）
- 检查：N+C 各 layer（0/1/2/3）贡献占比，**当前判定** = flywheel_run（生成器层）占比 < 60% 才算"治理深度 OK"
- 退出码：0 = 占比合理 / 1 = 占比失衡 / 2 = 文件不可读
- 当前 baseline：flywheel_run 2635/3601 = 73.2%（**失衡**）
- 阈值预登记：60%（代码内可调，但**用户不绕过**：按 F6.x 机械判定）
- 跨工具投影：`.agents/skills/sihankor-proposition-defense/probes/`

#### A2. `check_yaml_factual_consistency.py` {#a2-check_yaml_factual_consistency-py}

- 路径：同上目录
- 输入：methodology.yaml + 相关代码文件
- 检查：yaml 中的"反面教材" / "典型错位" / "must_check" 是否与代码事实一致
- 已知错位（baseline）：yaml L62 写"rationale 是 LLM 自动生成"：代码是确定性模板
- 输出：错位清单（yaml 文字 vs 代码事实）
- 退出码：0 = 一致 / 1 = 错位 / 2 = 文件不可读
- 跨工具投影：同上

#### A3. `check_test_coverage_intent.py` {#a3-check_test_coverage_intent-py}

- 路径：同上目录
- 输入：tests/ 目录 + test_*.py 文件
- 检查：test 函数 docstring / name vs 实际测什么
- 已知错位（baseline）：`test_f34_reproducibility` 名字说"复现性"但只比 dict 不验 trail 去重
- 输出：test 名字 vs 实际测试覆盖对照表
- 退出码：0 = 对应 / 1 = 错位 / 2 = 文件不可读
- 跨工具投影：同上

#### A4. `check_verdict_consistency.py` {#a4-check_verdict_consistency-py}

- 路径：同上目录
- 输入：layer2_signoff.py / program_signoff.py / maturation_gate.py
- 检查：trail 存的 verdict 版本（v1 / v2 / v3）vs 实际裁决用的版本
- 已知错位（baseline）：`layer2_signoff.py:236` 读 v1 verdict，`program_signoff.py:319` 内部判 v3
- 输出：v1 vs v3 决策分叉点清单
- 退出码：0 = 一致 / 1 = 分叉 / 2 = 文件不可读
- 跨工具投影：同上

#### A5. `check_decision_authority.py`（主线写，最复杂） {#a5-check_decision_authority-py-主线写-最复杂}

- 路径：同上目录
- 输入：facet program_signoff / layer2_signoff / DES-011 设计文档
- 检查：所有裁决路径（程序签 / 人签 / 引擎核对器）是否构成"双权威"或"唯一权威"
- 输出：裁决路径图（谁对什么有权）+ 重复或冲突点
- 退出码：0 = 唯一权威 / 1 = 双权威 / 2 = 文件不可读
- 跨工具投影：同上

### Cluster 2：流程改造（依赖 Cluster 1） {#cluster-2-流程改造-依赖-cluster-1}

#### B1. commit hook 配置 {#b1-commit-hook-配置}

- 路径：`.git/hooks/pre-commit` 或 git hooks 替代（pre-commit framework 优先）
- 内容：commit 前跑 5 个机械脚本（任一失败 = 阻断）
- 文档：`.githooks/README.md` 描述 hook 安装步骤
- 不改：sih-engine 治理范围内的 commit（DES/PRO/DEC）： 由 sih-doclint + 本脚本双重把关
- 不改：sih-tools/facet/ 的 commit（不受 sih-engine 治理约束）： 但本脚本可作为推荐（不强加）

#### B2. 流水线脚本：`pipeline_audit.sh` 或 `audit_pipeline.py` {#b2-流水线脚本-pipeline_audit-sh-或-audit_pipeline-py}

- 路径：`sih-engine/skills/sihankor-proposition-defense/probes/audit_pipeline.py`
- 输入：报告 / 代码仓
- 流程：
  1. 跑 5 机械脚本（并行）
  2. 全部通过 → 输出"机械层未漏" + 不调 LLM
  3. 任一失败 → 输出"机械漏层清单" + 可选调 LLM 审（默认不调，由用户决定）
- 输出：JSON 综合报告
- 退出码：0 / 1 / 2 语义同各机械脚本

### Cluster 3：文档与元层 {#cluster-3-文档与元层}

- `sih-engine/task-packages/t6d-01-mechanism-scripts.md`（本文件）
- `sih-engine/doc/research/reduce-llm-calls-results.md`（完成后写，N+C 减少前后对比）

## 四、可证伪条件（跑前立文） {#falsifiable}

每个脚本有 F1-F5 锚定：

- **F1.1** `check_layer_proportion.py` flywheel_run 占比 ≥ 60% → **撤销**（生成器层过度占比）
- **F1.2** `check_layer_proportion.py` 退出码与占比对应关系错误 → **撤销**
- **F2.1** `check_yaml_factual_consistency.py` 未抓到已知错位（yaml L62 rationale 注释）→ **撤销**
- **F2.2** `check_yaml_factual_consistency.py` 把 yaml 中文字误判为代码事实 → **撤销**
- **F3.1** `check_test_coverage_intent.py` 未抓到已知错位（test_f34 名字 vs 实际）→ **撤销**
- **F3.2** `check_test_coverage_intent.py` 对简单测试误报 → **撤销**
- **F4.1** `check_verdict_consistency.py` 未抓到已知分叉（layer2_signoff v1 vs program_signoff v3）→ **撤销**
- **F4.2** `check_verdict_consistency.py` 把"v1/v2 同一族"误判为分叉 → **撤销**
- **F5.1** `check_decision_authority.py` 未抓到程序签 + DES-011 双权威 → **撤销**
- **F5.2** `check_decision_authority.py` 路径图过简（漏掉人签路由）→ **撤销**
- **F6.1** commit hook 阻断正常 commit（误报）→ **撤销**
- **F6.2** commit hook 在机械脚本不通过时放行 → **撤销**
- **F7.1** 流水线脚本机械层全过仍触发 LLM 审 → **撤销**（LLM 调用 0 才对）
- **F7.2** 流水线脚本机械层失败不报告 → **撤销**

任一 F1.x-F7.x 触发 = 子任务失败。

## 五、必读文件 {#read}

子代理 X1 必读：
- `sih-engine/skills/sihankor-proposition-defense/probes/check_three_proposition_audit.py`（已立的脚本：同模式参考）
- `sih-engine/skills/sihankor-proposition-defense/methodology.yaml`（A2 必查）
- `sih-engine/doc/research/multi-agent-independent-audit-2026-08-16.md`（本次实验结论）
- `sih-tools/facet/docs/contribution-metric-design.md`（A1 baseline 来源）

子代理 X2 必读：
- `sih-tools/facet/probes/program_signoff.py:425-444`（A3 baseline）
- `sih-tools/facet/probes/layer2_signoff.py:236-256`（A4 baseline）
- `sih-tools/facet/tests/test_program_signoff.py:209-227`（A3 baseline，test_f34_reproducibility）

主线必读：
- `sih-engine/doc/design/DES-011-adjudication-baseline-check.md`（A5 必读）
- `sih-tools/facet/ROADMAP.md` §P5「sih-engine 集成路径选项」节（A5 双权威判断依据）

## 六、微积分知识包 {#六-微积分知识包}

查 `/Users/moc/workspaces/SiHankor/calculus/llm-friendly-build/`：
- `mapping.md` 检索"累计计算" / "分层积分"
- `entries/INT-007-fundamental-theorem-of-calculus.md`（FTC 第二部分 = N+C = ∫ w dN 数学基础）
- `entries/LIM-007-epsilon-delta-definition.md`（阈值 ± ε 的工程化）

缺则 web_search → 代理 → 先补仓再用于报告。

## 七、验收标准 {#acceptance}

### Cluster 1（A1-A5） {#cluster-1-a1-a5}
- 5 个脚本 + 5 个 `--help` 测试 + 5 个 baseline 复跑测试
- 退出码语义与文档一致
- 跨工具投影到 `.agents/skills/sihankor-proposition-defense/probes/`
- F1-F5 全部锚定不触发

### Cluster 2（B1-B2） {#cluster-2-b1-b2}
- commit hook 安装说明 + 实际跑通（可手动触发或 sample commit 测）
- 流水线脚本跑通：机械层全过 → exit 0 / 机械层失败 → exit 1 + 报告漏层

### Cluster 3（文档） {#cluster-3-文档}
- `t6d-01-mechanism-scripts.md`（本任务包）完成
- `sih-engine/doc/research/reduce-llm-calls-results.md` 写完：含：
  - 5 脚本在 facet 现有报告 / 代码上的 baseline 漏层数
  - 流水线跑一次 commit 的 LLM 调用数（应该 0）
  - vs 不跑机械检查时需要 LLM 审的次数

## 八、约束 {#constraints}

- **零 LLM 调用**：所有脚本不允许 `import openai` / `anthropic` / 类似
- **不绕过失败**：F1-F7 任一触发 = 任务失败，不许调参绕过
- **不写"建议"**：输出事实 + F 判定，不写"应该这样改"
- **不修已知缺陷**：脚本只检查 + 报告，不修 program_signoff 幂等问题 / v1v3 分叉 / 等
- **跨工具一致**：脚本行为在 sih-engine / sih-tools / .agents/skills 三处必须一致
- **不污染 trail**：脚本纯读，不写 trail / event

## 九、完成后回报（concise） {#report}

每个子任务完成后回报：

- A1: 当前 flywheel_run 占比 + 5 baseline 跑结果
- A2: 已知错位清单（yaml 文字 vs 代码事实）
- A3: 已知 test 名字 vs 实际覆盖错位清单
- A4: 已知 v1/v3 分叉点清单
- A5: 裁决路径图 + 唯一/双权威判定
- B1: commit hook 跑通证据（sample commit）
- B2: 流水线跑通证据（机械过/失败两种）
- 总：5 脚本 + 2 流程改造 commit + 文档 commit

## 十、派发顺序（按 T6-D 集群范式）{#dispatch}

### 阶段 1：并行派双子代理 + 主线写 A5 {#阶段-1-并行派双子代理-主线写-a5}

```bash
# 派子代理 X1（写 A1 + A2）
mavis task --description "T6D-01 A1+A2 机械脚本" \
  --prompt "任务: ... [A1 + A2 任务细节]"

# 派子代理 X2（写 A3 + A4）
mavis task --description "T6D-01 A3+A4 机械脚本" \
  --prompt "任务: ... [A3 + A4 任务细节]"

# 主线（root session）写 A5
# 完成后等 X1 + X2 报告
```

### 阶段 2：主线验收 + 派 B1 + B2 {#阶段-2-主线验收-派-b1-b2}

```bash
# 验收 X1 + X2 输出，跑 baseline 复测
# 然后派 B1 + B2 给原代理（X1 → B1, X2 → B2）
```

### 阶段 3：主线汇总 + commit {#阶段-3-主线汇总-commit}

- 跑流水线端到端测试
- commit 5 脚本 + 2 流程 + 2 文档
- 报告 root session

## 十一、任务包元数据 {#meta}

- **包名**：T6D-01-mechanism-scripts
- **作者**：Mavis（root session）
- **日期**：2026-08-17
- **关联**：
  - `doc/research/multi-agent-independent-audit-2026-08-16.md`（本次实验）
  - `skills/sihankor-proposition-defense/methodology.yaml`（方法学真源）
  - `skills/sihankor-proposition-defense/probes/check_three_proposition_audit.py`（已立脚本）
  - `sih-tools/facet/probes/contribution_metric.py`（N+C 基础）
- **下次更新**：本任务包完成后写 `t6d-01-results.md` 记录实际 LLM 调用减少百分比
