# facet-audit-t6d-report-B（有工具组审阅报告）

> T6-D 范式：B 子代理（有工具实验组）
> 审阅对象：sih-tools/facet 仓 head (44e8d0c)
> 审阅时间：2026-08-17 19:30-19:55（≈25 min）
> 工具层：B2 流水线 + 三层审 + v1-scan + 贡献度 4 个工具全跑

## 一、整体印象

facet 仓在 44e8d0c（head）已形成「5 机制 + 217 测试 + 1001 行 ROADMAP」的阶段 1+2 累积形态，commit 粒度细到原子修复（A1 layer 占比、A4 route 内部 v3 重算、A2 dedup、A5 cross-link），但**四个工具一致显示仓整体尚未到可迁入 sih-engine 的状态**：贡献度 NOT PASS、三层审计漏立题命题、v1-family 跨族待迁 40+ 个函数、A4 裁决路径 UNIQUE_BUT_BLIND。

工具组 B 的优势在于：能在 25 分钟内把「表面看是文档说已达 5/6 退出条件」与「机械层 4/5 FAIL」之间的张力点出来，这是单靠阅读 ROADMAP 极易忽略的：ROADMAP 末段明明写着「贡献度指标 PASS ❌」，但「#1-#5 已过」的总表让它整体读起来像「基本可以迁」。工具把 4 个具体缺口量化了：N 过 / C 差 1399 / 新机制 0/3 / layer 占比 73.17% > 60%。

但工具组也有自己的盲点：所有 4 个工具都是「机械层事实采集」，没有「语义层解读」：比如 contribution_metric 报告 73.17% 失衡，它**不判**「这是数据飞轮问题还是指标设计缺陷」；v1-scan 报告 40 个 v1-family loose 函数，它**不**评估「这些函数迁移的成本/风险排序」。LLM 真审仍必要。

## 二、四维度发现

### 维度 1：代码质量 {#维度-1-代码质量}

**D1.1 【工具发现】** audit_pipeline 5 脚本中 4 FAIL（exit 1），A2 yaml_factual_consistency 是唯一 PASS：commit 44e8d0c 的修复目标 A1/A4 都仍 FAIL（A1 是数据问题非代码，A4 是跨族一致性问题）。
- 锚定：`sih-engine/skills/sihankor-proposition-defense/probes/audit_pipeline.py:64-83`（流水线判定逻辑）
- 工具：audit_pipeline (5 子脚本)
- FAIL 项：A1 layer_proportion (73.17% > 60%) / A3 test_intent (2 PARTIAL) / A4 verdict_consistency (1 version_split) / A5 decision_authority (UNIQUE_BUT_BLIND)

**D1.2 【工具发现】** test_coverage_intent 检出 2 条 PARTIAL：`test_check_verdict_consistency.py:288-306` 的 `test_json_metrics_conflict_payload` docstring 提 fingerprint（暗示文件级检查）但 body 只验内存 dict；`test_flywheel_trail.py:160-174` 的 `test_backfill_idempotent_and_tolerant` 名字含 idempotent 暗示文件级幂等但 body 仅 in-memory 断言。
- 锚定：`tests/test_check_verdict_consistency.py:288-306` + `tests/test_flywheel_trail.py:160-174`
- 工具：check_test_coverage_intent

**D1.3 【阅读发现】** commit 粒度极细（44e8d0c 单 commit 包含 A1+A4 两个独立修复），但 96bf81a 写「A3+A5」也是单 commit 双子任务：这与工程基线「每条独立」原则略有张力，但合并理由是 A3 dedup + A5 cross-link 都是 program_signoff 内的 schema 修改（锚定：`96bf81a` commit message）。
- 锚定：`git show 96bf81a 44e8d0c --stat`（与上游 commit hash）

**D1.4 【阅读发现】** ROADMAP 1001 行 + stage2-status 259 行声明 195 测试，但 `pytest --collect-only` 实测 217 测试：文档比实测少 22。stage2-status.md:258 写「阶段 2 新增：22 测试」与实测一致，但 ROADMAP 第 31 行「217 测试（阶段 1+2 全过）」未与 stage2-status 对齐。
- 锚定：`sih-tools/facet/ROADMAP.md:31` vs `pytest --collect-only` 输出（217 collected）
- 建议：ROADMAP §2.2 与 stage2-status 同步更新到 217 baseline。

### 维度 2：退出标准满足度 {#维度-2-退出标准满足度}

**D2.1 【工具发现】** 贡献度综合判定 NOT PASS：N=6227 ✅ / C=3601.0 ❌ (-1399) / 阶段 2 新机制覆盖 0/3 ❌ (program_signoff=0 / knife_edge_risk=0 / supersession=0) / A1 layer 占比 73.17% > 60% ❌。
- 锚定：`sih-tools/facet/probes/contribution_metric.py:65-85`（阈值预登记） + 工具输出
- 工具：contribution_metric
- 含义：阶段 2 三种新机制（程序签/刀锋拒签/翻案）在真实流量上**零触发**：schema 实现 ≠ 真实使用。

**D2.2 【工具发现】** flywheel_run 占 73.17% 贡献度（权重 0.5 × 5270 = 2635.0 / C=3601.0），gate_assessment 26.38%（权重 1.0 × 950 = 950.0），layer2_signoff 仅 0.28% (5 条 × 2.0 = 10.0)：**生成器层过度占比**，治理深度严重不足。
- 锚定：`contribution_metric.py:142-166`（_scan_trail 实现）+ 工具输出表
- 工具：contribution_metric / check_layer_proportion
- 含义：与 LIM-007 epsilon-delta 形式根一致：实测 73.17% - 阈值 60% = 13.17% = ε > 0 = 失衡。

**D2.3 【工具发现】** exit code 1 = NOT PASS（脚本注释明确「达标则 exit 0 / 不达标则 exit 1」），与 ROADMAP:926 「贡献度指标 PASS ❌ 当前 6227/3601/0 不达标」一致：但 ROADMAP 末段路径选择「B 路径优先：拿 2-3 条真实司衡决策走完整 pipeline」尚未触发。
- 锚定：`ROADMAP.md:944-967`（阶段 2.5 达标路径） + `contribution_metric.py:285`
- 工具：contribution_metric（exit code 1 = NOT PASS）

**D2.4 【阅读发现】** 退出条件 6 条中 5 条 ROADMAP 标 ✅（#1-#5），仅 #6 ❌：但 A4 修复后 (44e8d0c) `_cmd_route` 已从 v1-family 移到 v3-family，「#4 漏放率 0」实际由 A4 修复守护，需在 stage2-status 留底（当前 stage2-status.md:200-203 只提「监控 stage 2 状态... 跑 contribution_metric」未提 A4 修复链路）。
- 锚定：`ROADMAP.md:920-934`（6 条件表） + `44e8d0c commit message` (A4 部分)

### 维度 3：治理覆盖度（三层审计） {#维度-3-治理覆盖度-三层审计}

**D3.1 【工具发现】** `docs/contribution-metric-design.md` 三层审计：立题命题 0/14 = 0% ❌ / 应用命题 3/18 = 16.67% ✅ / 治理领域展开贡献 1/14 = 7.14% ✅：**立题命题整层漏掉**（关键词「本体命题 / 立题 / 异质性 / 独立探索 / 命题合法性 / 异质性来自 / 立题范围 / 本体论 / foundation proposition」等 0 命中）。
- 锚定：`sih-tools/facet/docs/contribution-metric-design.md` + 工具输出（漏层细节）
- 工具：check_three_proposition_audit
- 含义：贡献度指标设计**自身**未在立题命题层承载：「为什么是 N+C 而不是别的指标」无本体论支撑。

**D3.2 【工具发现】** `probes/program_signoff.py` + `probes/layer2_signoff.py` 同样漏立题命题（机械层同样 0% 立题命中）：阶段 2 关键裁决代码本体论层空缺。
- 锚定：`probes/program_signoff.py` + `probes/layer2_signoff.py`（probes 目录全扫）
- 工具：check_three_proposition_audit（两个文件分别跑）

**D3.3 【工具发现】** decision_authority 检出 4 条裁决路径（facet-program-signoff / facet-human-signoff / sih-engine-baseline-checker / facet-knife-edge-reject），其中 facet 与 sih-engine 各占 2 条：cross-link 协议层 OK，schema 字段层 OK，**但实际生效层 0/0 事件已 cross-link**（无 program_signoff 事件已写入）。
- 锚定：`sih-engine/skills/sihankor-proposition-defense/probes/check_decision_authority.py` + 工具输出
- 工具：check_decision_authority
- 含义：双权威点协议存在但**无任何真实流量**走过：与 D2.1 新机制 0/3 互为印证。

**D3.4 【阅读发现】** baseline_checker 写 `sih-engine/trail/YYYY-MM-DD.ndjson` 而 facet 写 `sih-tools/proposition/DES/<guidance_id>`：trail 双源（sih-engine 仓 + sih-tools 仓），这是 D3.3 跨仓 cross-link 的物理基础。但 A4 修复后 `_cmd_route` 输出新增 `v1_verdict / v3_rule / v3_note / criteria_version` 字段，DES-011 baseline_checker 是否已更新消费这些字段未在本次审阅中验证。
- 锚定：`check_decision_authority` 工具输出「路径 3」 + `44e8d0c commit message`（A4 新增输出字段）
- 建议：T6D-04 实施 critical path 迁移时同步验 baseline_checker 消费新字段。

### 维度 4：跨族治理现状 {#维度-4-跨族治理现状}

**D4.1 【工具发现】** v1-family 跨族函数：严格（按 calls）= 15 / 宽松（按 reads，含 FP）= 44 / 宽松（无 FP）= 40：**F 锚定 A-1.1 满足**（40 ≥ 40 阈值），但迁移规模仍大：critical path 2 个（layer2_signoff: build_docket + submit_signoff）+ 闸工厂 1 个（maturation_gate: assess_maturation_v2）+ 11 个中优先级 + 4 个低优先级 + 24 个 reads-only 待评估。
- 锚定：`sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md:13-25`（汇总表） + 工具输出完整清单
- 工具：fix-governance-boundaries-t6d-v1-scan.py

**D4.2 【工具发现】** A4 修复后 `_cmd_route@layer2_signoff.py:229-306` 仍属 mixed（calls v1+v3），v1 verdict 保留 audit 用途不再参与决策：这是已知设计而非 bug，但**v1 verdict 字段读仍被 check_verdict_consistency 视为 v1-family**：存在分类与意图错位风险。
- 锚定：`fix-governance-boundaries-t6d-v1-family-inventory.md:99-103`（mixed 说明） + `44e8d0c commit message`（A4 修复语义）
- 工具：v1-scan + check_verdict_consistency

**D4.3 【工具发现】** 4 个 false positive（custom verdict 命名空间）：`load_latest_foregrounding@flywheel_trail.py:415-421`（读 `foregrounding_verdict` 非 gate verdict）+ `temp_probe.py` 三个函数（温度探针 verdict「可用/漂移告警/基线异常」非 gate）。FP 排除规则三档：函数名级 / 文件级 / 字符串值级：这是 v1-scan 的精细度亮点。
- 锚定：`fix-governance-boundaries-t6d-v1-family-inventory.md:152-166`（FP 清单 + 排除规则）
- 工具：v1-scan

**D4.4 【工具发现】** check_verdict_consistency 检出 1 条 version_split 跨族不一致：v1=stable_clear + v3=boundary 时 v1-family 函数调 v3-family 函数 → 用户拿到拒签但上游没说闸改判。**44e8d0c 的 A4 修复就是为了根治此**：但工具仍 FAIL，因为 audit 视角下 v1 verdict 字段仍读。
- 锚定：`check_verdict_consistency.py` 工具输出（INCONSISTENT scenario）
- 工具：check_verdict_consistency
- 含义：A4 修复**决策**正确，但**工具分类**仍报跨族：T6D-04 需同步更新 family 分类（将 _cmd_route 从 v1-family 完全移到 v3-family 而非 mixed）。

## 三、工具 vs 阅读对比

### 工具独有发现（X2 ∖ X1 应有） {#工具独有发现-x2-x1-应有}

| ID | 发现 | 工具 |
|---|---|---|
| D1.1 | audit_pipeline 4/5 FAIL（含具体退出码） | audit_pipeline |
| D1.2 | 2 个 test docstring vs body 错位（PARTIAL） | check_test_coverage_intent |
| D2.1 | 贡献度综合 NOT PASS（含 N=6227/C=3601/新机制 0/3 全量化） | contribution_metric |
| D2.2 | flywheel_run 73.17% 占比（机械层失衡证据） | check_layer_proportion |
| D3.1 | contribution-metric-design.md 立题命题 0% | check_three_proposition_audit |
| D3.2 | program_signoff.py / layer2_signoff.py 立题命题 0% | check_three_proposition_audit |
| D3.3 | decision_authority 4 路径 + 0 cross-link 实际生效 | check_decision_authority |
| D4.1 | v1-family 40 严格 / 44 宽松 / 15 严格 calls | v1-scan |
| D4.2 | _cmd_route mixed 状态 = 分类与意图错位 | v1-scan + check_verdict_consistency |
| D4.3 | 4 个 FP 精细度（函数名/文件/字符串值三档） | v1-scan |
| D4.4 | version_split scenario 描述 | check_verdict_consistency |

**工具独有 ≈ 11 项**（去重后独立发现项）。

### 阅读独有发现（X1 应有，X2 工具漏） {#阅读独有发现-x1-应有-x2-工具漏}

| ID | 发现 | 路径 |
|---|---|---|
| D1.3 | commit 96bf81a 双子任务合并与「每条独立」原则张力 | git show |
| D1.4 | ROADMAP 195 vs pytest 实测 217 测试数不对齐 | ROADMAP.md:31 |
| D2.4 | A4 修复守护 #4 漏放率，但 stage2-status 未留底 | stage2-status.md:200-203 |
| D3.4 | baseline_checker 是否已消费 A4 新输出字段未验证 | 推测项 |

**阅读独有 ≈ 4 项**：其中 D3.4 是「工具漏判假设性风险」（需 T6D-04 实跑验证）。

### 工具 vs 阅读 关键差异 {#工具-vs-阅读-关键差异}

- **量化能力**：工具把「表面看是 ROADMAP 自述 ✅」转成「机械层 4/5 FAIL」：这种**事实↔自述差异**是阅读极难识别的（人易被 ✅ 符号锚定）。
- **盲点**：工具不读 commit message（不识「A4 修复语义」），不识别「文档/代码版本不对齐」（D1.4），不识别「修复上游未留底」（D2.4）。LLM 阅读在「意图层」仍占优。
- **张力点**：D4.2/D4.4 揭示**工具分类标准**（reads 字段 = v1-family）vs **修复意图**（决策已迁 v3，v1 仅 audit）有错位：这是工具设计的边界条件。

## 四、风险点 / 待办

**R1（高）**：退出条件 #6 NOT PASS → 阶段 2 新机制 0/3 真实流量触发缺位。ROADMAP §阶段 2.5 B 路径「拿 2-3 条真实司衡决策走完整 pipeline」未启动。建议：T6D-04 同步 B 路径触发（不只是迁移代码），否则 44e8d0c 的 A1+A4 修复在迁入 sih-engine 时仍会被同一问题拖累。锚定：`ROADMAP.md:944-967`。

**R2（高）**：D3.1/D3.2 揭示「立题命题层 0%」是**仓内文档级普遍现象**（不只是 contribution-metric-design 一处）：probes 目录下程序签/层 2 签核代码本体论层空缺。虽 check_three_proposition_audit 注明「机械检查 ≠ 真审」，但 0% 立题命中是反常信号，强烈建议 T6D-04 立文时**每个核心裁决脚本配套 1 段立题命题引用**。锚定：`probes/program_signoff.py` + `probes/layer2_signoff.py`。

**R3（中）**：D1.4 文档/实测不对齐（ROADMAP 195 vs pytest 217）：这是工程基线第四条「可验证性约束」的轻量违反。建议 ROADMAP §2.2 加 1 行 `pytest --collect-only` 输出截屏作为 baseline。锚定：`ROADMAP.md:31`。

**R4（中）**：D4.4 version_split scenario 仍 FAIL（A4 修复**决策**正确但**工具分类**仍报）：check_verdict_consistency 需在 T6D-04 同步更新（要么将 _cmd_route 完全移出 v1-family，要么新增「mixed 视为 v3-family 优先」规则）。锚定：`check_verdict_consistency.py` 输出。

**R5（中）**：D3.3 cross-link 实际生效 0/0：baseline_checker 在 4 条裁决路径中是「消费侧」，但消费触发条件依赖 facet 真实流量；D2.1 新机制 0/3 不达标时，cross-link 永远 0/0：R1 与 R5 是同一根因（流量缺位）。锚定：`check_decision_authority.py` 输出。

**R6（低）**：D1.3 commit 96bf81a 双子任务合并：可在 ROADMAP §commit 规范加 1 行「单 commit 单修复优先（紧急合并需 commit message 注明）」。锚定：`git show 96bf81a`。

## 五、工具盲点（你跑工具后认为工具漏掉的问题）

**B1：工具不识别 commit message 语义**。A4 修复 (44e8d0c) 的 commit message 明确写「route 读 v1 verdict 决策的 bug 已根治」：但 audit_pipeline / check_verdict_consistency 仅看代码 AST，识别不出「决策已迁」vs「读字段但决策不依赖」的区别。建议：v1-scan 增加「decision_source」字段标注（reads 字段 = 数据依赖 / 决策依据 = 决策依赖）。

**B2：工具不识别文档-代码版本漂移**。D1.4 ROADMAP 195 vs pytest 217 是 22 测试差：audit_pipeline / v1-scan 都不查「文档声明的测试数 vs 实测数」。建议：check_yaml_factual_consistency 增 1 条 `tests_collected_count` 校验（与 ROADMAP 数值交叉）。

**B3：工具不查「修复是否被文档留底」**。D2.4 A4 修复守护 #4 漏放率，但 stage2-status.md 200-203 未提 A4 修复链路：这是「代码修了但文档没跟」的典型漂移。audit_pipeline 不查文档留底。建议：立 `check_fix_documentation_alignment` 脚本（在 fix-failures-t6d-results 引用过的模式）。

**B4：工具不识别「schema 实现 vs 真实使用」语义**。D2.1/D3.3 都揭示同一根因：阶段 2 机制 schema 已实现 + 测试已过 ≠ 真实流量使用过。contribution_metric 报告新机制 0/3 但**不**进一步指出「哪些 schema 实际未挂到 trigger」：需要 LLM 真审。工具的「事实层」与 LLM 的「语义层」分工是必要的，但**当前没有 LLM 真审自动化绑定**：`check_three_proposition_audit` 末尾的「LLM 仍须按 methodology.yaml 三层方法学真审」是注释级提醒，不是机制级强制。

**B5：check_verdict_consistency 与 v1-scan 分类标准不完全一致**。D4.2/D4.4 显示 v1-scan 给出 40 v1-family loose 但 check_verdict_consistency 给 44（v1+v3 mixed 的 _cmd_route 计入 v1-family）。两个工具都「正确」但**口径不同**：A4 修复后到底算 v1-family 还是 v3-family 没有统一定论。建议：在 check_decision_authority 增加 `family_decision_resolver` 函数统一规则。

**B6：contribution_metric 的 60% 阈值是工程推断而非数学推导**。D2.2 注释里写「不是数学推导，是治理意图 + 实测的工程推断」：这本身没问题，但脚本不给出「如果实测 65% / 55% / 50% 的不同治理含义」分级解释，工具使用者只能得到 PASS/FAIL 二值。建议：增加 `governance_depth_assessment` 字段分级（如「严重失衡 / 失衡 / 接近 / 达标」），让审阅者看到「60.01% vs 73.17%」的差距性质。

---

## F 锚定状态

- F2（25-35 min 完成）：✅ 19:30-19:55（≈25 min）
- F3（至少跑 3 个新工具）：✅ audit_pipeline + check_three_proposition_audit + v1-scan + contribution_metric = 4 个（额外跑了 5 个子脚本 = 共 9 次工具调用）
- F6（每条发现标锚定）：✅ D1.1-D4.4 + 6 项风险 + 6 项盲点全部标 (path, line) 或 commit hash + 工具来源

## 工具调用清单（防虚报）

1. `python3 skills/sihankor-proposition-defense/probes/audit_pipeline.py` → 5 子脚本 1 PASS / 4 FAIL
2. `python3 skills/sihankor-proposition-defense/probes/check_layer_proportion.py` → 73.17% FAIL
3. `python3 skills/sihankor-proposition-defense/probes/check_test_coverage_intent.py` → 2 PARTIAL FAIL
4. `python3 skills/sihankor-proposition-defense/probes/check_verdict_consistency.py` → 1 version_split INCONSISTENT
5. `python3 skills/sihankor-proposition-defense/probes/check_decision_authority.py` → 4 路径 UNIQUE_BUT_BLIND
6. `python3 skills/sihankor-proposition-defense/probes/check_yaml_factual_consistency.py` → PASS
7. `python3 skills/sihankor-proposition-defense/probes/check_three_proposition_audit.py` → 3 文件立题 0% (contribution-metric-design / program_signoff / layer2_signoff)
8. `python3 task-packages/fix-governance-boundaries-t6d-v1-scan.py` → 15/40/44/9 分布
9. `python3 probes/contribution_metric.py` → N=6227 / C=3601 / 新机制 0/3 / 占比 73.17% / NOT PASS

合计 9 次工具调用（3 文档 + 6 代码扫描），全部 0 LLM 调用。

<!-- B 子代理完成时间：19:55 -->
