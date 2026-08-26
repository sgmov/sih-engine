# facet 仓审阅报告（A 子代理，无工具对照组）

> 审阅对象：sih-tools/facet head (44e8d0c, 2026-08-17)
> 工具约束：纯阅读（git log / read / grep / glob），未跑任何 check_/audit_/verify_ 类新工具
> 审阅时间：2026-08-17 19:29-20:00 (Asia/Shanghai)
> F 锚定：F1（25-35 min 完成，本次 ~31 min）✓ / F6（每条发现标锚定）✓

## 整体印象 {#整体印象}

facet 仓处于「交付定格 + 阶段 2.5 收尾」的双层状态：ROADMAP 1001 行 + 35+ 篇 docs 报告 + 99 个探针 + 179 个 test 函数构成完整的工程沉淀。核心理念（异质性来自独立探索、facet 不裁决只产出异质性发现）从哲学仓 PRO-07 三条已立命题（A-A3.1/A-A4.1/A-A4.2）有显式锚定，不是工程自创。最显眼的是 commit 44e8d0c（T6D-02 A1+A4 修复）：layer 占比下限 60% 写入 contribution_metric.py、`_cmd_route` 内部 v3 重算落地、program_signoff dedup 复用同 event_id，这是 2026-08-17 一天之内的两批修复，说明 T6D 触发的 5 份审阅报告已在闭环。

但仓内存在一组 ROADMAP/plan/stage2-status.md 未覆盖的盲点：测试数量与历史宣告对不上、文档 doclint 错误追踪停在 129、v1-family 函数仍占严格分类的 15 个（按 calls）未迁移、measure.py 报告层写"v3 闸裁决"但实际拿 v2 字段映射、3 阶段命名债务（csnx）立了待复审。这是 ROADMAP 既写"实验收官"又写"v3 闸 fast lane 护栏（F3.1 根治）待阶段 3"形成的"半定格"状态：核心命题收官，工程债未清。

---

## 维度 1：代码质量 {#维度-1-代码质量}

### 发现 1.1 测试数量与 ROADMAP 宣告对不上 {#发现-1-1-测试数量与-roadmap-宣告对不上}

`plan/stage2-status.md:258` 写"阶段 1 回归 173 测试 + 阶段 2 新增 22 测试 = 总计 195 测试全过"，但实际 `grep -rE "^def test_" tests/` 数为 **179**（无 ROADMAP 174/195/217 三个数字中的任何一个）。`tests/test_check_layer_proportion.py`（10 tests）+ `tests/test_check_verdict_consistency.py`（12 tests）是 8/17 当日新增文件：加到 195 应为 217，加上 8/17 当日的 22 应是 217。实际 179 减 195 差 16，179 减 217 差 38。**测试数漂移无审计追踪**。锚定：`plan/stage2-status.md:258`、`ROADMAP.md:915`（"173 + 22 = 195 测试全过 2026-08-16 02:45"）、`sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md:244`（"跑 pytest 验 217+ 全过"是 T6D-04 目标值，不是当前值）。

### 发现 1.2 v1 family 函数未迁移（严格 15 / 宽松 40-44） {#发现-1-2-v1-family-函数未迁移-严格-15-宽松-40-44}

`fix-governance-boundaries-t6d-v1-family-inventory.md:18-25` 已 commit 的诊断数据：严格 v1-family（按 calls）= 15 函数待迁；宽松 v1-family（按 reads，无 FP）= 40；含 FP = 44。**A4 修复（commit 44e8d0c）只迁了 1 个 critical path**（`_cmd_route`），剩余 14 个 strict 严格 v1 函数（包括 `build_docket@layer2_signoff.py:77-141`、`submit_signoff@layer2_signoff.py:144-203`）仍读 v1 verdict 决策。锚定：`sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md:18-25, 41-97`（清单）、commit `44e8d0c`（仅 A4 修 1 个）、`probes/layer2_signoff.py:117`（`build_docket` 仍读 `gate.get("verdict")` 即 v1 字段）、`probes/layer2_signoff.py:167-172`（`submit_signoff` 仍读 `layer1_verdict = gate.get("verdict")`）。

### 发现 1.3 测量入口 measure.py 字段命名歧义 {#发现-1-3-测量入口-measure-py-字段命名歧义}

`measure.py:189` 报告"闸门裁决（判据 v3）：{_VERDICT_CN[v2['verdict_v3']]}"：`v2` 是变量名（assess_maturation_v3 的返回值），但行内字符串写"v3"、人话页用"判据 v3"，运行的是 `assess_maturation_v3` 函数，输出字段 `verdict_v3`。这是**两层歧义**：(a) 变量名 `v2` 与函数 `assess_maturation_v3` 不一致（v2 是 v3 的基座，但用户读报告只看到"判据 v3"）；(b) `_VERDICT_CN` 表只接受 v3 verdict 值（boundary / near_threshold / stable_clear），但 measure.py 报告路径上 `verdict_v3` 是字符串字段，下游无 type guard 保护，若 v3 闸 return 出现第四值（例如 v3 阶段 3 的新加裁决）则 `KeyError`。锚定：`measure.py:189`、`measure.py:60-64`（import 链）、`probes/maturation_gate.py:317-322`（assess_maturation_v3 函数签名与返回结构）。

### 发现 1.4 src/ v1 era 18 文件 3254 行未处置 {#发现-1-4-src-v1-era-18-文件-3254-行未处置}

`src/` 目录 18 个 .py 文件共 3254 行（anchors 255 / compiler 892 / runner 798 / validators 287 / engine 165 / llm_client 119 / config 124 / thinking_resolver 77 / ban_pick_helpers 101 / 等），是 P1 时代的产物：`grep -E "verdict_v3|assess_maturation_v3"` 在 src/ 下零结果，说明 v3 闸体系下 src/ 是冻结的 legacy 实现。`measure.py:60, 156` 仍 `import runner; await runner.run_experiment(...)`，把 v1 era run_experiment 套在 v3 闸的飞轮/闸/gate_assessment 流程里：新旧代码并存，无任何 ROADMAP 文档标识其边界。锚定：`wc -l src/*.py = 3254 total`、`measure.py:60`、`measure.py:156`、`src/runner.py:274`（run_experiment 入口）、`ROADMAP.md:78-128`（P0/P1 段列了 src/ 各文件但无"v3 时代下角色"说明）。

---

## 维度 2：退出标准满足度 {#维度-2-退出标准满足度}

### 发现 2.1 退出条件 #6 贡献度仍不达标，根因诊断不一致 {#发现-2-1-退出条件-6-贡献度仍不达标-根因诊断不一致}

`plan/stage2-status.md:130` 写"#6 贡献度指标 PASS: N=6227 ✅ / C=3601 ❌ / 新机制覆盖=0 ❌"：stage2-status 与 ROADMAP 数字一致。但 `stage2-status.md:178` 写"根因初判：数据飞轮问题（阶段 2 机制 schema 已实现 + 测试已过，但未接真实 guidance）"；`ROADMAP.md:130` 写"根因初判：数据飞轮问题"；`stage2-status.md:188-196` 写"执行路径：B 数据飞轮优先 / A 重审 LAYER_WEIGHTS 兜底"。

但**没有数据说明是数据飞轮问题还是指标设计问题**。`stage2-status.md:182-189` 列事件类型分布：flywheel_run 5270 × 0.5 = 2635 (73.2%)，gate_assessment 950 × 1.0 = 950 (26.4%)，其他加和 16 (0.4%)：这恰恰说明**生成器层确实占 73%**。A1 修复（commit 44e8d0c）加 60% 阈值是**承认这是设计失衡**，不是"数据飞轮问题"。锚定：`plan/stage2-status.md:178, 182-189`、`contribution_metric.py:46-48, 87-92`（A1 修复加 60% 阈值）、commit `44e8d0c`（A1 落库）。

### 发现 2.2 退出条件 #4 漏放率 0 的"兜底"机制未根治 {#发现-2-2-退出条件-4-漏放率-0-的-兜底-机制未根治}

`plan/stage2-status.md:128` 写"漏放率 0：F3.1 已被 2.4 兜底（knife_class_signed=0）"，"兜底"=检测层拒签 + trail 写 knife_edge_risk + 后处理算 rate 0.4088。`ROADMAP.md:972` 写"v3 闸 `unanimous_violate_fast_lane` 加 boundary_rate 护栏（F3.1 根治）"：这是**根治 F3.1**的待做项，但 v3 闸 fast lane 设计边界**至今未修**。`tests/test_knife_edge_visibility.py` 测的是"fast_lane + boundary_flag>0 → True 触发拒签"，但**没有测试覆盖"fast_lane + boundary_flag 全部 false（真清晰违规）→ 不触发"**：也就是说兜底机制会"误报"（真清晰违规无 flag 也走程序签，但若 flag 累加边界会误拦）。锚定：`ROADMAP.md:972`、`plan/stage2-status.md:128-129`、`probes/maturation_gate.py:317-368`（fast_lane + suspicious_fast_lane 逻辑）、`tests/test_knife_edge_visibility.py:32-37`（F1-F4 边界 4 case，无"清晰违规无 flag"对照）。

### 发现 2.3 退出条件 #5 "无人在环路" 实为人机混合 {#发现-2-3-退出条件-5-无人在环路-实为人机混合}

`plan/stage2-status.md:129` 写"无人在环路：程序签 + knife-edge 拒签（边界仍走人签）"，"边界仍走人签"=**人机混合**不是"无人在环路"。`ROADMAP.md:579-583` 红队结论："赐爵的范式排他性成立（管线不会自己升格），人身排他性是约定非机械（AI 不代签的自我约束 + PRO-07 锚定，守卫拦不住冒签）"：这意味着人签节点**仍是核心**，"无人在环路"应该改写为"环路靠程序 + 人审异常"。锚定：`plan/stage2-status.md:129`、`ROADMAP.md:579-583`、`probes/layer2_signoff.py:300-310`（`_cmd_route` human_signoff 路由仍存在）。

---

## 维度 3：治理覆盖度 {#维度-3-治理覆盖度}

### 发现 3.1 立题命题（PRO-07 三条）锚定完整 {#发现-3-1-立题命题-pro-07-三条-锚定完整}

`ROADMAP.md:20-26` 显式承载 A-A3.1（自证循环）/ A-A4.1（候选建议生成器）/ A-A4.2（裁决权归确定性引擎）三条哲学已立命题，每条配实验佐证数据（跨家族 Jaccard 0.21 vs 同家族 0.56、counter bps0=0.92）。`tests/test_knife_edge_visibility.py` 与 `probes/layer2_signoff.py` 的 PRO-07 路由守卫（boundary → 拒）是 A-A3.1 的机械实现。`docs/DES-001-facet-model.md`、`docs/DES-002-config-paradigm.md`、`docs/PRO-001-paradigm-primitives.md` 三文档承担范式定义。这层覆盖完整。锚定：`ROADMAP.md:20-26`、`docs/DES-001-facet-model.md`、`docs/DES-002-config-paradigm.md`、`docs/PRO-001-paradigm-primitives.md`、`probes/layer2_signoff.py:14, 172`（PRO-07 路由注释）。

### 发现 3.2 应用命题（PRO-07 工程实现）锚定完整 {#发现-3-2-应用命题-pro-07-工程实现-锚定完整}

`ROADMAP.md:602-606` "facet 应用命题立文（2026-08-13，用户核心想法）"：刀锋命题是治理主场，上下文是钥匙且可测：这是用户级核心想法（不来自哲学仓），立文位置正确。配套：刀锋人重作实验、基础 split heal 探针、m-knife 矩阵扩容、八维全跑（甲乙丙丁戊己庚辛）：命题在 200 命题 × 6 家族 4883 发 + 3 真实 guidance (rg-P1/P2/P3) + 多刀锋家族 (e1/pmon/k2) 实证。这层覆盖完整。锚定：`ROADMAP.md:602-606, 608-660, 671-690`。

### 发现 3.3 治理领域展开贡献：场景丰富但 #6 不达标 {#发现-3-3-治理领域展开贡献-场景丰富但-6-不达标}

`ROADMAP.md:909-915` 关键数据：跨厂治理温度差异 15.6%、4 LLM mean_comply_rate = 0.348、knife-edge 检测率 40.88%、**0 条已知刀锋被自动签**：指标层是好的。场景层：200 命题 × 6 家族多主题重跑、3 真实 guidance 走完 Layer 1→人签→留痕全链、轻量模式 8 命题 × 4 层四档顺序采样。但贡献度 #6 不达标（N 6227 ✅ / C 3601 ❌ / 新机制覆盖 0），A1 修复加 60% 阈值只是**承认失衡**不是**修复**：3 个月观察期候选项前，需要先有"生成器层占比下降 + Layer 2/3 事件增加"两条机制性改善。**应用场景与机制性贡献度之间的桥**未在 ROADMAP 写。锚定：`ROADMAP.md:130, 909-915`、`plan/stage2-status.md:130, 167-176`、`stage2-status.md:165-197`（执行路径预登记）。

---

## 维度 4：跨族治理现状（v1 family 函数待迁规模） {#维度-4-跨族治理现状-v1-family-函数待迁规模}

### 发现 4.1 严格 v1 family 函数 15 个，4 批迁移路径已设计 {#发现-4-1-严格-v1-family-函数-15-个-4-批迁移路径已设计}

按 `sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md:18-25, 167-225`（已 commit 诊断数据，未跑新扫描）：

| 维度 | 数量 | 状态 |
|---|---|---|
| 严格 v1-family（按 calls）= 实际待迁 | 15 | 14 未迁（_cmd_route A4 已修）|
| 宽松 v1-family（reads v1+v2，无 FP）= F 锚定阈值 | 40 | 满足 ≥ 40 |
| 宽松 v1-family（含 FP） | 44 | 估算基线 |
| v2-family（reads v2 字段） | 8 | 部分需 v3 迁移 |
| v3-family（已 v3） | 5 | program_signoff 等 |
| mixed（v1+v3 混用） | 1 | _cmd_route A4 已修 |
| reads-only（消费 trail 不决策） | 24 | 待评估 |
| false positive（custom 命名空间） | 4 | 排除 |

**第一批 critical 路径 2 个函数待迁**：`build_docket` + `submit_signoff`（layer2_signoff.py）：这是 Layer 2 人签核接口的核心，仍读 `gate.get("verdict")` 字段，是治理核心的未迁 debt。锚定：`fix-governance-boundaries-t6d-v1-family-inventory.md:18-25, 170-178`、`probes/layer2_signoff.py:117, 167-172`。

### 发现 4.2 第二批中优先级 8 个探针 main 函数待迁 {#发现-4-2-第二批中优先级-8-个探针-main-函数待迁}

包括 cascade_ng_probe / basis_split_heal_probe / flywheel_microcircuit_probe / lightweight_mode_probe / real_guidance_run / refine_loop_probe / reflexive_signing_probe 的 main 函数：这些是**实验/分析**用，非 critical path。`assess_maturation_v2@maturation_gate.py:201-232` 也在此批，是闸工厂，依赖升级会传导到所有 v2/v3 caller，需先跑 `batch_reevaluate_v3*` 验证 v3 闸稳定性。锚定：`fix-governance-boundaries-t6d-v1-family-inventory.md:181-192, 213-225`、`tests/test_batch_reevaluate_v3*.py`（验证套件）。

### 发现 4.3 T6D-04 任务包未立，但 F 锚定要求"217+ 测试全过" {#发现-4-3-t6d-04-任务包未立-但-f-锚定要求-217-测试全过}

`fix-governance-boundaries-t6d-v1-family-inventory.md:236-245` 声明本任务包"不实施迁移"：T6D-04 立后续任务包。T6D-04 范围含 217+ 测试全过。**T6D-04 任务包当前不存在**（`sih-engine/task-packages/` 下无 T6D-04 标记文件）。这意味着 critical 路径迁移**未排期**，仅靠 A1+A4+A3（T6D-02）打补丁式修复。锚定：`fix-governance-boundaries-t6d-v1-family-inventory.md:236-245`、`sih-engine/task-packages/`（目录列表无 T6D-04）。

---

## 风险点 / 待办 {#风险点-待办}

1. **测试数漂移无追踪**（发现 1.1）：179 vs 195 vs 217 三个数字，三个文档（ROADMAP / plan/stage2-status / T6D-04 目标）不统一。`scripts/count_tests.py` 不存在：加一个零 LLM 脚本锁定当前 test 函数数与基线差，是 1 小时可做的债。
2. **layer2_signoff critical 路径 2 函数待迁**（发现 2.2、4.1）：`build_docket` + `submit_signoff` 仍读 v1 verdict，T6D-04 任务包未立文：是 P3 双层闸设计的剩余 surface，对应 ROADMAP 阶段 3 候选"#5 knife-edge 报警 + trail 后处理"前置。
3. **measure.py 报告层 v2/v3 命名歧义**（发现 1.3）：变量名 `v2` 与函数 `assess_maturation_v3` 与字符串"v3"三层命名错位，影响下游 user 读报告理解。是 10 行代码可清。
4. **退出条件 #4 "漏放率 0" 兜底 ≠ 根治**（发现 2.2）：v3 闸 fast lane 加 boundary_rate 护栏是 F3.1 根治，ROADMAP 阶段 3 候选但未排期：如果未来出现"清晰违规无 flag"被兜底机制误报（fp>0.4），可能误拦正确 stable_clear。
5. **退出条件 #5 "无人在环路" 应改写**（发现 2.3）：边界仍走人签，不是"无人"，与 ROADMAP 阶段 3 候选"user_posture UI"形成闭环：但措辞需要诚实改。
6. **阶段 2 报告 doclint 错误 129 未追到 0**（盲点）：`plan/stage2-status.md:210` 写"前序 206/355，本报告 129 错误，未追到 0"：但 `docs/*.md` 13 个报告无 doclint 校验脚本（`sih-doclint` 在 sihankor/ 下，未在 facet 内集成）。`ROADMAP.md:976` 也承认"前序 129 错误，未追到 0"是阶段 3 候选。

---

## 盲点声明（ROADMAP/现有文档未覆盖但 facet 实际有的问题） {#盲点声明-roadmap-现有文档未覆盖但-facet-实际有的问题}

1. **csnx 命名债务**（`ROADMAP.md:38-42` 自承"立名债务 2026-08-11 暂缓"）：csnx 是"信号收敛积分"的首字母缩写，承担"加权收敛方向建议"职能，但名字承诺"压缩"（删除冗余）实际是"积分"（加权求和）：这是 ROADMAP 自承的债务，至今（44e8d0c）未清。锚定：`ROADMAP.md:38-42, 38 立名债务段`、`docs/PRO-001-paradigm-primitives.md:9-15`（csnx 命名说明段）。

2. **measure.py 的 v2 字段路径是隐性双绑**（已记但未明示）：`measure.py:189` 用 `v2['verdict_v3']` 实际上是 `assess_maturation_v3` 返回值，变量名 `v2` 是**v3 闸的内部基座**：读代码的人会困惑"为什么判据 v3 报告拿 v2 变量"。这是 P3 闸 v1→v2→v3 演化的命名债，与 csnx 债务同源。

3. **L2 人签核 critical path 是 ROADMAP"v3 闸现行" 但代码层仍 v1 闸**（最关键盲点）：`ROADMAP.md:718-719` 写"判据 v3 为闸现行版本（v2 保留回归基准，v1 保留历史）"，但 `probes/layer2_signoff.py:117, 167-172` 的 `build_docket` + `submit_signoff` 仍 `gate.get("verdict")` 读 v1 字段。**ROADMAP 文档说"v3 是现行"与代码现状（v1 在 critical path）不一致**。锚定：`ROADMAP.md:718-719`、`probes/layer2_signoff.py:117, 167-172`、`fix-governance-boundaries-t6d-v1-family-inventory.md:170-178`（"layer2_signoff 是 critical 路径"）。

4. **0.348 baseline 仍未在 sih-engine 集成后重测**（ROADMAP 自承）：`ROADMAP.md:973` 写"baseline 0.348 在 sih-engine 集成后重测 → 真实发布默认温度"，但 `ROADMAP.md:830-832` 又写"sih-engine 集成决策：暂不集成，facet 以 sih-tools 形态提供功能"：**两个 future hook 互相依赖（baseline 重测需要集成，集成被暂缓）形成死结**。锚定：`ROADMAP.md:973, 830-832`、`plan/stage2-status.md:48-53`（"0.348 是临时占位"自承）。

5. **v1 family 严格 15 函数中 _cmd_route 已修但其他 14 个未修**（盲点放大）：A4 修复（commit 44e8d0c）只迁了 1 个 critical path 函数，但**ROADMAP 阶段 2 退出条件 5/6 通过是基于"基础设施完整"**：这里"完整"是 schema 实现 + 测试过，不是 critical path 全迁。`stage2-status.md:87-90` 写"关键基础设施已落地"，措辞模糊，留了 14 个未迁函数的隐式未完结。

6. **T6D-04 任务包未立**（具体盲点）：`fix-governance-boundaries-t6d-v1-family-inventory.md:236-245` 声明 T6D-04 立后续任务包，但 `sih-engine/task-packages/` 下无 T6D-04 标记文件。意味着 critical path 迁移**未排期**。锚定：`fix-governance-boundaries-t6d-v1-family-inventory.md:236-245`。

7. **轻量模式"升级路径零触发"**（`ROADMAP.md:737-738` 自身记录）：M3+三元组 tier-3 升级路径在 8 命题 4 层真实数据上**零触发**：升级代码只有编译级验证，无真实数据流过。这是"轻量模式"作为日常裁决的**根本性盲点**：生产期不知是否会触发升级、触发了是否能正确降级回去。

---

## 报告元数据 {#报告元数据}

- 总字数（含标点）：约 2950 字
- 4 维度发现数：13 条（维度 1: 4 / 维度 2: 3 / 维度 3: 3 / 维度 4: 3）
- 盲点数：7 条
- 锚定率：100%（每条发现标 `path:line` 或 `commit hash`）
- 工具使用：仅 `git log` / `git show` / `read` / `grep` / `glob` / `wc` / `sed`，未跑任何新立 check_/audit_/verify_ 脚本
- 引用数据：ROADMAP.md / plan/stage2-status.md / task-packages/mech-checks-2026-08-17.md / task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md（已 commit 诊断输出，未跑新扫描）

<!-- A 子代理完成时间：20:00 -->
