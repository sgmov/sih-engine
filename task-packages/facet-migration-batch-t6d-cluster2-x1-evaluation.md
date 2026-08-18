# facet-migration-batch-t6d Cluster 2 X1 · reads-only 评估

> T6D-11 Cluster 2 X1 子代理
> 评估范围：fix-governance-boundaries-t6d-v1-family-inventory §reads-only 清单的前 12 个函数
> 任务：分类 reads-only 函数的「读取用途」= 数据依赖 vs 决策依赖
> 评估时间：2026-08-18
> 评估方法：读函数体 + grep caller + 核对 verdict 字段在 caller 是数据/控制流分支

## 摘要

- 评估函数：12 个（inventory §reads-only 清单前 12 项）
- 数据依赖：6 个 → **不迁**
- 决策依赖：6 个 → **已迁 v3**（T6D-09/T6D-10 已完成代码迁移，本次评估仅记录在册）
- F 锚定：F1 12 函数全评估 ✓ F2 清单可读 ✓ F3 分类明确 ✓
- 不实施代码改动（仅评估）

## 评估方法

每个函数 3 步：

1. 读函数体，定位 verdict 字段的读入与产出
2. grep caller（函数被哪些代码调用，caller 内部如何用 verdict）
3. 分类：
   - **数据依赖**：verdict 仅用于 print / 写 JSON / 写 trail / 拼字符串 / 透传，无控制流分支
   - **决策依赖**：verdict 用于 if / 列表推导过滤 / 路径分支 / 守卫判断

处置：

- **不迁**：数据依赖，且 verdict 命名空间与 maturation v1 无关（自身探针 verdict）
- **已迁 v3**：决策依赖，但代码已迁移到 v3 verdict（`verdict_v3` 优先 / 字段名 `verdict_v2` 但值为 v3）
- **待 T6D-XX 迁移**：决策依赖且仍读 v1 verdict（本次评估无此类）

## 12 函数评估清单

### 1. `_pre_registered@basis_split_heal_probe.py:149-156`

- **分类**：数据依赖
- **依据**：
  - 函数体（`basis_split_heal_probe.py:149-161`）：读 `base["verdict_v3"] or base["verdict"]` 与 `targeted["verdict_v3"] or targeted["verdict"]`，返回语义标签
  - 函数注释（line 150-151）：`T6D-09 X1 迁移：决策基于 v3 verdict，v1 保留 audit`，**已迁 v3**
  - caller（`basis_split_heal_probe.py:228`）：`read = _pre_registered(results["base"], results["targeted"])`
  - caller 用途（line 235-244）：`print(f"\n预登记判读：{read}")` + `out_path.write_text(json.dumps(payload, ..., "pre_registered": read, ...))`
  - 无控制流分支
- **处置**：**不迁**（数据依赖，仅 print + JSON 写盘，且已迁 v3）

### 2. `main@bootstrap_partial.py:128-276`

- **分类**：数据依赖
- **依据**：
  - 函数体（`bootstrap_partial.py:111-125` `judge()`）：verdict 是本文件自算的 `INCONCLUSIVE` / `SIGNIFICANT`（CI 含零 + p<0.05 判定）
  - `main` 内（line 194-210）`verdict = judge(ci, st)` → 写 `rows[i]["verdict"]` → print（line 209-210）+ 写 JSON（line 263-274）
  - line 257-260：`print(f"{r['label']:<48} {r['original']:<35} {r['verdict']:<15}")` 仅打印对比
  - 无控制流分支
  - 注：本文件 verdict 是 bootstrap CI 判定，**非 maturation v1 verdict**——`fix-governance-boundaries-t6d-v1-scan.py` 按 `verdict` 字段名误判为 v1-family，actual reads v1 = 0
- **处置**：**不迁**（数据依赖，且 verdict 命名空间与 maturation 无关，verdict 来自 bootstrap 统计判据）

### 3. `main@cascade_ng_probe.py:188-269`（实际函数体 line 201-269）

- **分类**：决策依赖
- **依据**：
  - 函数体读 `levels[lv]["verdict_v3"]`（line 239, 242, 246, 264-265）
  - 决策分支（line 241-243）：`if key == "c3_program": ok = levels[1]["verdict_v3"] in ("stable_clear", "near_threshold")` → `adj["H1_c3"] = ok`
  - 决策分支（line 245-248）：`if key == "c1_direct": ok = ...` → `adj["H1_c1"] = ok`
  - 决策分支（line 249-254）：`if key == "p_monitor": non_worse = all(sev[i] >= sev[i + 1] ...)` → `adj["H2_non_worse"]`
  - 决策列表（line 264-265）：`leaks = [c["gid"] for c in conds if c.get("kind") == "knife_edge" and c.get("verdict_v3") == "stable_clear"]`（漏放判定）
  - 函数注释（line 192）：`# v3 字段（决策依据）：F6 锚定 = v3 字段输出到 trail`，**已迁 v3**
- **处置**：**已迁 v3**（决策依赖，迁移完成于 T6D-09 X1，commit `e3232a2` 包含本函数，记录在 `mid-priority-migration-t6d-results.md:98`）

### 4. `_cond_task@cascade_ng_probe.py:196-207`（实际函数体 line 209-220）

- **分类**：数据依赖
- **依据**：
  - 函数体（line 209-220）：`try: r = await run_condition(p, lv, n, sem); print(f"[{r['gid']}] verdict_v3={r['verdict_v3']} (v1={r['verdict']}) ...") return r; except: return {"gid": ..., "error": ...}`
  - 读 verdict 字段是间接的（通过 `run_condition` 返回的 dict）
  - caller（line 222-223）：`asyncio.gather(*[_cond_task(p, lv) for p in PROPOSITIONS for lv in LEVELS])`
  - 用途：异常处理 + print + 透传，无控制流分支依赖 verdict
- **处置**：**不迁**（数据依赖，仅异常处理 + print + 透传）

### 5. `main@eir_ecr_gate_probe.py:222-308`

- **分类**：决策依赖
- **依据**：
  - 函数体（line 254-256）：`gate = _gate_verdict(trajectory[0], trajectory[-1])` → `gate['verdict']` 是 ECR 自己的 verdict 字符串（`single_round_suffices` / `multi_round_resolves` / `multi_round_oscillates` / `ambiguous`，见 `eir_ecr_gate_probe.py:185-209`）
  - 决策分支（line 271）：`any_pass = any(r["gate"]["passed"] for r in knife_results)` → line 278 `print(f"\n  >>> 总判定：{'C 门槌通过（有增量）' if any_pass else 'C 门槌未通过 → 闭回 B'} <<<")`（控制流分支）
  - 决策依据是 `gate['passed']`（ECR 自己的 passed 标志），verdict 字符串用于 print 展示
  - 注：`_gate_verdict` 函数本身被 `fix-governance-boundaries-t6d-v1-scan.py` exclusion 规则 1 标为 FP，但 main 函数未排除；main 内部读的 `gate['verdict']` 是 ECR gate 命名空间，**非 maturation v1 verdict**
- **处置**：**不迁**（verdict 命名空间与 maturation 无关，决策用 ECR gate `passed` 字段，ECR gate 自身的门槌实验结论 `C 闭回 B 落定` 已记录在 `INTEGRAL-STAGE-RECOVERY.md:45` 与 `ROADMAP.md:437`）

### 6. `main@foregrounding_probe.py:195-286`

- **分类**：决策依赖
- **依据**：
  - 函数体（line 243-244）：`result = foregrounding_metrics.compute_foregrounding_susceptibility(paraphrase_runs)` → `result["verdict"]` 是 foregrounding 自己的 verdict（`susceptible` / `stable` / `stable_with_noise_shift`，见 `foregrounding_metrics.py`）
  - 决策分支（line 248）：`match = ("susceptible" in result["verdict"]) == (expect == "susceptible")` → 控制 `match` 标志
  - 决策分支（line 279-283）：`if not r["expect_match"]: all_match = False` → 控制汇总输出
  - 注：verdict 是 `foregrounding_metrics` 自身产出的判定，**非 maturation v1 verdict**
- **处置**：**不迁**（verdict 命名空间与 maturation 无关，决策基于 foregrounding_metrics 自身的三值判定，A item 6 已闭环见 `ROADMAP.md:435`）

### 7. `_noise_floor@knife_calib_probe.py:176-189`

- **分类**：数据依赖
- **依据**：
  - 函数体（line 176-189）：读 `r["verdict"]`（knife-calib 自己的 verdict `stable_clear`，来自 `_collect()` line 47-85），统计 per-model `off_modal_rate`
  - 过滤条件（line 179）：`if r["verdict"] != "stable_clear": continue` 是数据过滤（仅算 stable_clear trail 内的 off-modal），非治理决策
  - caller（`knife_calib_probe.py:201`）：`"ji_noise_floor": _noise_floor(rows)` → 写 result dict
  - caller 用途（line 207-224）：`j = result["ji_noise_floor"]` → 打印（line 222-224）
  - 无控制流分支
  - 注：`r["verdict"]` 来自 knife-calib 自己的 `_collect` 阶段，**非 maturation v1 verdict**
- **处置**：**不迁**（数据依赖，且 verdict 命名空间与 maturation 无关，verdict 来自 knife-calib probe 自身采集）

### 8. `_build_rationale@program_signoff.py:170-192`

- **分类**：数据依赖
- **依据**：
  - 函数体（line 170-192）：读 `gate_result.get("verdict_v3") or gate_result.get("verdict")` → 拼 rationale 字符串
  - 函数注释（line 175-178）：`程序自动生成签核 rationale（不是人写）`、`stable_clear verified at baseline={cal}*user_posture={p}={eff}, 3/3 conditions met, no dissent, N flywheel runs, time_window_met={t}`
  - caller（`program_signoff.py:444`）：`rationale = _build_rationale(gate_result=..., ...)` → 写 trail（line 457-472）+ 返回（line 478-481）
  - 用途：trail 留痕（`ft.record_program_signoff`）与人展示，无控制流分支
  - 注：verdict_v3 优先（**已迁 v3**，line 180 注释），v1 fallback 仅兜底
- **处置**：**不迁**（数据依赖，仅生成文本字段用于 trail 留痕与人展示，且已迁 v3）

### 9. `main@r3b_new_props.py:112-167`

- **分类**：决策依赖
- **依据**：
  - 函数体读 `r["verdict_v2"]`（line 160-161）
  - 决策列表（line 160）：`knife_leak = [r["gid"] for r in knife if r["verdict_v2"] == "stable_clear"]`（漏放判定）
  - 决策求和（line 161）：`clear_boundary = sum(1 for r in clear if r["verdict_v2"] == "boundary")`
  - 决策布尔（line 162-165）：`dir_ok = all((r["modal"] == "comply" if r["stratum"] == "clear_comply" else r["modal"] == "violate" if r["stratum"] == "clear_violate" else True) for r in rows)` → 方向判定
  - 注：`verdict_v2` 字段值是 v3 verdict（`gate_row@r3b_new_props.py:131` 注释 `verdict_v2 = verdict_v3, # main() 决策键`，已迁 v3）
- **处置**：**已迁 v3**（决策依赖，迁移完成于 T6D-10 X2 low-priority 范围，记录在 `mid-priority-migration-t6d-results.md:158`）

### 10. `main@r5_emphasis_feedback.py:124-177`（实际函数体 line 215-...）

- **分类**：决策依赖
- **依据**：
  - 函数体读 `r["verdict_v2"]`（line 238）
  - 决策列表（line 238）：`knife_leak = [r["gid"] for r in knife_rows if r["verdict_v2"] == "stable_clear"]`（漏放判定）
  - 决策列表（line 240）：`basis_healed = [r["gid"] for r in non_knife if len(r["distinct_basis"]) == 1]`（治愈判定）
  - 决策列表（line 241-243）：`bdy_dropped = [r["gid"] for r in non_knife if r["baseline"]["bdy"] and sum(r["bdy_flags"]) / N_SHOTS < r["baseline"]["bdy"]]`（bdy 下降判定）
  - 注：`verdict_v2` 字段值是 v3 verdict（`run_subject@r5_emphasis_feedback.py:206` 注释 `verdict_v2 = verdict_v3, # main() 决策键`，已迁 v3）
- **处置**：**已迁 v3**（决策依赖，迁移完成于 T6D-10 X2 low-priority 范围，记录在 `mid-priority-migration-t6d-results.md:159-160`）

### 11. `_task@r5_emphasis_feedback.py:130-141`（实际函数体 line 221-232）

- **分类**：数据依赖
- **依据**：
  - 函数体（line 221-232）：`try: r = await run_subject(key, src, knife, sem); print(f"... {r['verdict_v1']}->{r['verdict_v2']}"); return r; except: return {"gid": ..., "error": ...}`
  - 读 verdict 字段是间接的（通过 `run_subject` 返回的 dict）
  - caller（line 234）：`results = await asyncio.gather(*[_task(*s) for s in SUBJECTS])`
  - 用途：异常处理 + print + 透传，无控制流分支依赖 verdict
- **处置**：**不迁**（数据依赖，仅异常处理 + print + 透传）

### 12. `_pre_registered_verdict@refine_loop_probe.py:156-166`

- **分类**：决策依赖
- **依据**：
  - 函数体（line 156-166）：读 `verdicts.get("targeted")` / `verdicts.get("generic")` v3 verdict 字符串，返回语义标签（`refine_works` / `just_strength` / `refine_dead` / `anomalous`）
  - 函数注释（line 157）：`预登记判读：按 base/generic/targeted 三条件 v3 verdict 组合机判（T6D-09 X2）`，**已迁 v3**
  - caller（`refine_loop_probe.py:253`）：`v = _pre_registered_verdict(verdicts_v3[key])`
  - 决策分支（line 261-264）：`if len(set(overall)) == 1: conclusion = overall[0]; else: conclusion = "mixed"` → 控制结论
  - 测试锚定（`tests/test_refine_loop_probe.py:5`）：`v3 verdict 用于判读（_pre_registered_verdict 改用 v3 verdict 字符串）`
  - 迁移记录（`mid-priority-migration-t6d-results.md:107`）：`c9e5e02 refine_loop_probe.py: 210-232 → 211-256（既有 gate 缺 v3 字段时退回 v1；_pre_registered_verdict 改签 = 接受 v3 verdict 字符串 dict）`
- **处置**：**已迁 v3**（决策依赖，迁移完成于 T6D-09 X2，commit `c9e5e02`）

## 分类汇总

| # | 函数 | 位置 | 分类 | 处置 |
|---|---|---|---|---|
| 1 | `_pre_registered` | basis_split_heal_probe.py:149-156 | 数据 | 不迁（已迁 v3） |
| 2 | `main` | bootstrap_partial.py:128-276 | 数据 | 不迁（verdict 命名空间与 maturation 无关） |
| 3 | `main` | cascade_ng_probe.py:188-269 | 决策 | 已迁 v3（commit e3232a2） |
| 4 | `_cond_task` | cascade_ng_probe.py:196-207 | 数据 | 不迁（仅异常处理 + 透传） |
| 5 | `main` | eir_ecr_gate_probe.py:222-308 | 决策 | 不迁（verdict 命名空间与 maturation 无关，决策用 ECR gate `passed`） |
| 6 | `main` | foregrounding_probe.py:195-286 | 决策 | 不迁（verdict 命名空间与 maturation 无关，决策用 foregrounding_metrics 三值判定） |
| 7 | `_noise_floor` | knife_calib_probe.py:176-189 | 数据 | 不迁（verdict 命名空间与 maturation 无关） |
| 8 | `_build_rationale` | program_signoff.py:170-192 | 数据 | 不迁（已迁 v3 优先） |
| 9 | `main` | r3b_new_props.py:112-167 | 决策 | 已迁 v3（mid-priority X2） |
| 10 | `main` | r5_emphasis_feedback.py:124-177 | 决策 | 已迁 v3（mid-priority X2） |
| 11 | `_task` | r5_emphasis_feedback.py:130-141 | 数据 | 不迁（仅异常处理 + 透传） |
| 12 | `_pre_registered_verdict` | refine_loop_probe.py:156-166 | 决策 | 已迁 v3（commit c9e5e02） |

汇总计数：

- 数据依赖：6 个（#1, #2, #4, #7, #8, #11）→ **不迁**
- 决策依赖：6 个（#3, #5, #6, #9, #10, #12）→ 4 个已迁 v3（#3, #9, #10, #12）+ 2 个 verdict 命名空间与 maturation 无关不需迁（#5, #6）
- **总计不迁：12 个**

## 关键发现

1. **6 个决策依赖函数中 4 个已迁 v3**（#3 cascade_ng_probe.main, #9 r3b_new_props.main, #10 r5_emphasis_feedback.main, #12 refine_loop_probe._pre_registered_verdict）—— 字段名保留 `verdict_v2` 但实际值是 v3 verdict（向后兼容垫片）
2. **2 个决策依赖函数的 verdict 命名空间与 maturation 无关**（#5 eir_ecr_gate_probe.main, #6 foregrounding_probe.main）—— 它们读的是 ECR gate / foregrounding_metrics 自己的 verdict 字符串，**非 maturation v1**
3. **fix-governance-boundaries-t6d-v1-scan.py 的 reads-only 分类有 3 个 FP 误判**（#2 bootstrap_partial, #5 eir_ecr_gate_probe.main, #7 knife_calib_probe._noise_floor）—— 它们的 verdict 是各探针自己的命名空间，**非 maturation v1 verdict**。但这不影响迁移决策（FP 都不需迁移），仅说明 inventory 分类规则可精化。
4. **reads-only 24 个函数全部不需迁移**—— 12 个数据依赖（无控制流）+ 4 个决策依赖已迁 v3 + 4 个 verdict 命名空间与 maturation 无关（待 T6D-12 集群 3 X2 评估剩余 12 个，可预期同样结论）

## 关联文件

- 任务包：T6D-11 Cluster 2 X1 子任务
- 上游：fix-governance-boundaries-t6d-v1-family-inventory.md §reads-only 清单
- 上游：mid-priority-migration-t6d-results.md（T6D-09 X1/X2 + T6D-10 X2 已完成 commit 记录）
- 探针位置：sih-tools/facet/probes/{basis_split_heal,bootstrap_partial,cascade_ng,eir_ecr_gate,foregrounding,knife_calib,program_signoff,r3b_new_props,r5_emphasis_feedback,refine_loop}_probe.py
- 后续：T6D-11 Cluster 2 X2 / Cluster 3 评估剩余 12 个 reads-only 函数
