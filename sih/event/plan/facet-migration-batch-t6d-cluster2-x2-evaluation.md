# facet-migration-batch-t6d Cluster 2 X2 · reads-only 评估

> T6D-11 Cluster 2 X2 子代理
> 评估范围：fix-governance-boundaries-t6d-v1-family-inventory §reads-only 清单的剩余 12 个函数（与 X1 错位分工，X1 评前 12 个 = JSON items 1, 5, 6, 7, 8, 10, 12, 13, 14, 15, 16, 17；X2 评剩余 12 个 = JSON items 2, 3, 4, 9, 11, 18, 19, 20, 21, 22, 23, 24）
> 任务：分类 reads-only 函数的「读取用途」= 数据依赖 vs 决策依赖
> 评估时间：2026-08-18
> 评估方法：读函数体 + grep caller + 核对 verdict 字段在 caller 是数据/控制流分支

## 摘要

- 评估函数：12 个（X1 评估后剩余 reads-only 清单项）
- 数据依赖：10 个 → **不迁**
- 决策依赖：2 个 → **待 T6D-XX 迁移**（X2 错位分工无已迁 v3 案例，与 X1 不同）
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
- **待 T6D-XX 迁移**：决策依赖且仍读 v2 verdict（本次评估命中 2 个：bing_xseat_probe.main + yi_depth_probe.main）

## 错位分工说明

X1 与 X2 错位分工原因：X1 评估时按函数名/功能区段分组（含跨段）评前 12 个；X2 评剩余 12 个。本次 X2 评估项 JSON 序：

| JSON 序 | 函数 | family 分类 | FP 状态 |
|---|---|---|---|
| 2 | `_latest_v3_verdict@batch_reevaluate_v3.py:50-61` | reads-v3-family | non-FP |
| 3 | `_latest_v3_verdict@batch_reevaluate_v3_baseline.py:36-43` | reads-v3-family | non-FP |
| 4 | `main@bing_xseat_probe.py:108-143` | reads-v2-family | non-FP |
| 9 | `load_latest_foregrounding@flywheel_trail.py:415-421` | reads-v1-family | **FP**（函数名 `load_latest_foregrounding` 在 FALSE_POSITIVE_NAMES）|
| 11 | `_collect@knife_calib_probe.py:47-85` | reads-v2-family | non-FP |
| 18 | `main@run_stage2_03_eval.py:150-354` | reads-v1-family | non-FP |
| 19 | `run@temp_probe.py:119-191` | reads-v1-family | **FP**（文件 `temp_probe.py` 在 FALSE_POSITIVE_FILES）|
| 20 | `score@temp_probe.py:260-315` | reads-v1-family | **FP**（同上）|
| 21 | `main@temp_probe.py:318-356` | reads-v1-family | **FP**（同上）|
| 22 | `main@verify_thinking_silent.py:146-245` | reads-v1-family | non-FP（verdict 局部计算，非 gate verdict）|
| 23 | `main@xseat_fill_probe.py:127-155` | reads-v2-family | non-FP |
| 24 | `main@yi_depth_probe.py:126-154` | reads-v2-family | non-FP |

## 12 函数评估清单

### 1. `_latest_v3_verdict@batch_reevaluate_v3.py:50-61`

- **分类**：数据依赖
- **依据**：
  - 函数体（`batch_reevaluate_v3.py:50-61`）：从 `trail` 倒序遍历找 `trail_type == "gate_assessment"` 且 `criteria_version == "v3"` 的记录，返回 `(verdict_v3, dc_fingerprint, family_temperature_version)` 三元组；无则 `(None, None, None)`
  - 函数 docstring（line 51-54）：`从 trail 提取最后一条 v3 闸记录的 (verdict_v3, dc_fingerprint, ftv)`
  - 函数无控制流分支，纯数据读取 + 透传
  - caller（`batch_reevaluate_v3.py:93`）：`old_v3, old_fp, old_ftv = _latest_v3_verdict(trail)`
  - caller 用途（line 94-115）：`if old_v3 is None: skipped_no_v3 += 1; rows.append({"diff": "no_prior_v3", ...})` + `diff = "match" if v3_frozen["verdict_v3"] == old_v3 else "verdict_changed"`
  - 注：函数读取 v3 verdict（`verdict_v3` 字段），返回结构化三元组；判据 `criteria_version == "v3"` 已在函数内硬编码
  - caller 的 match/verdict_changed 判定属 batch_reevaluate_v3 主线的整体一致性验证，非本函数职责
- **处置**：**不迁**（数据依赖，函数已显式读 v3 verdict 字段；本身无控制流）

### 2. `_latest_v3_verdict@batch_reevaluate_v3_baseline.py:36-43`

- **分类**：数据依赖
- **依据**：
  - 函数体（`batch_reevaluate_v3_baseline.py:36-43`）：从 `trail` 倒序遍历找 `trail_type == "gate_assessment"` 且 `criteria_version == "v3"` 且 `family_temperature_version == DEFAULT_FTV`（`v1-2026-08-15-frozen`）的记录，返回 `(verdict_v3, baseline_flip_rate_version)` 二元组
  - 函数 docstring（line 37）：`从 trail 取最后一条 v3 闸 (frozen) 记录`
  - 函数无控制流分支，纯数据读取 + 透传
  - caller（`batch_reevaluate_v3_baseline.py:67`）：`old_verdict, old_bfv = _latest_v3_verdict(trail)`
  - caller 用途（line 68-79）：`if old_verdict is None: old_verdict = "(no_frozen_record)"; diff = "match" if new_verdict == old_verdict else "verdict_changed"`
  - 注：与第 1 项 `_latest_v3_verdict@batch_reevaluate_v3.py` 同模式，函数本身只读 v3 verdict 字段
- **处置**：**不迁**（数据依赖，函数已显式读 v3 verdict 字段；本身无控制流）

### 3. `main@bing_xseat_probe.py:108-143`

- **分类**：决策依赖
- **依据**：
  - 函数体读 `r["verdict_v2"]`（line 117, 122）
  - 决策 print（line 117）：`print(f"[{c['key']}@{tag}] modal={r['modal']} flip={r['flip']} bdy={r['bdy_rate']} v2={r['verdict_v2']}", flush=True)` —— 仅 print，非决策
  - **决策列表（line 122）**：`leak = [r for r in rows if r["verdict_v2"] == "stable_clear"]`（漏放判定）
  - 决策输出（line 131）：`summary["leak_stable_clear"] = [r["xgid"] for r in leak]` —— filter 结果写入 summary
  - 其他决策（line 121）：`blunts = [r for r in rows if (r["bdy_rate"] or 0) < 0.34]` 与 line 123-125 `modal_flips` —— 与 verdict 无关，不算 verdict 决策
  - 注：上游 `run_cond@bing_xseat_probe.py:69-105` line 99 仍调 `assess_maturation_v2`（**未迁 v3**），line 105 `verdict_v2` 字段值是真实 v2 verdict（非 v3 verdict 兼容垫片）
- **处置**：**待 T6D-XX 迁移**（决策依赖，filter 表达式 `r["verdict_v2"] == "stable_clear"`；上游 `run_cond` 仍 v2-family by calls 调 v2 maturation，未迁 v3，main 的 filter 仍基于真实 v2 verdict 决策）

### 4. `load_latest_foregrounding@flywheel_trail.py:415-421`（**FP**）

- **分类**：数据依赖
- **依据**：
  - 函数体（`flywheel_trail.py:415-421`）：从 `load_trail(guidance_id, trail_root)` 遍历找 `trail_type == "foregrounding_measurement"` 的记录，返回 `r.get("verdict")`
  - 函数 docstring（line 416）：`最新 foregrounding 判定（多次测量取最新；无返回 None）`
  - 函数无控制流分支
  - caller（`flywheel_trail.py:429`）：`load_latest_foregrounding(guidance_id, trail_root)` —— 由 `load_for_gate` 便利函数包装后喂 `assess_maturation`
  - **注**：函数读 `trail_type == "foregrounding_measurement"` 类型的 `verdict` 字段（**foregrounding 命名空间**，非 maturation gate verdict）；`fix-governance-boundaries-t6d-v1-scan.py` exclusion 规则 1（`FALSE_POSITIVE_NAMES`）已标 FP
  - verdict 字符串值形如 `foregrounding_susceptible` / `foregrounding_stable`（见 `foregrounding_metrics.py`），**与 maturation 闸 verdict 命名空间正交**
- **处置**：**不迁**（FP，verdict 命名空间与 maturation 无关；函数读 `foregrounding_measurement` 命名空间）

### 5. `_collect@knife_calib_probe.py:47-85`

- **分类**：数据依赖
- **依据**：
  - 函数体（`knife_calib_probe.py:47-85`）：遍历 `TRAIL_ROOT` 目录收集每条 trail 的展平 shots + trail 级指标
  - 读 verdict 字段（line 74-82）：`gate = _latest_gate(trail)`（line 42-44 取 `trail_type == "gate_assessment"` 最新一条）→ `rows.append({..., "verdict": (gate or {}).get("verdict_v2") or (gate or {}).get("verdict"), ...})`
  - 函数无控制流分支
  - caller（`knife_calib_probe.py:193`）：`rows = _collect()` —— 由 main 包装
  - caller 用途：`result = {"probe": ..., "wu_calibration": _calibration(rows), "xin_reason_consistency": _reason_consistency(rows), "ji_noise_floor": _noise_floor(rows)}` —— 三阶段分析
  - 注：`_collect` 把 verdict 字段透传到 `rows[i]["verdict"]`，下游 `_noise_floor` 据此过滤；过滤行为发生在 `_noise_floor`（已在 X1 评估，判定为数据依赖）
- **处置**：**不迁**（数据依赖，仅 verdict 透传到 row；filter 行为在 `_noise_floor`，已在 X1 评估为不迁）

### 6. `main@run_stage2_03_eval.py:150-354`

- **分类**：数据依赖
- **依据**：
  - 函数体读 `result.get("verdict")`（line 187）
  - 数据存储（line 182-193）：`eval_results.append({..., "verdict": result.get("verdict"), "reason": result.get("reason"), ...})` —— verdict 仅存入 eval_results
  - **关键**：line 198-255 所有 F3.x 判定都用 `signed` 字段（`r["signed"]`），**不用 verdict 字段**：
    - F3.1（line 198-200）：`knife_edge_signed = [r for r in eval_results if r["category"] == "knife_edge" and r["signed"]]`
    - F3.2（line 203-209）：`clear_total` / `clear_unsigned` 都用 `r["signed"]`
    - F3.3（line 213-222）：`sample = next((r for r in eval_results if r["signed"]), None)`
    - F3.6（line 255）：`f36_pass = all(bool(r.get("flywheel_run_ids")) for r in eval_results if r["signed"])`
  - 函数无 `if r["verdict"] == ...` 形式的条件
  - 注：verdict 字段被存入 eval_results 但**未被任何 F3.x 条件使用**；上报 JSON 时也透传
  - upstream `program_signoff`（line 444）调 `_build_rationale` 生成 rationale，verdict 来源是 `result.get("verdict")` 即 v3 verdict
- **处置**：**不迁**（数据依赖，verdict 字段仅存储未参与决策；F3.x 全用 `signed` 字段）

### 7. `run@temp_probe.py:119-191`（**FP**）

- **分类**：数据依赖
- **依据**：
  - 函数体（`temp_probe.py:119-191`）：跑 N 次 shot + 收 trail + 计算 `temperature` / `accuracy_ok` / `drift_alarm`
  - 局部计算 verdict（line 189-190）：`result["verdict"] = ("可用" if result["accuracy_ok"] and not drift_hit else "漂移告警" if drift_hit else "基线异常（violate 未全票判违）")` —— verdict 是**温度探针自身的三值判定**（"可用"/"漂移告警"/"基线异常"），**非 maturation gate verdict**
  - 函数无控制流分支
  - caller（`temp_probe.py:342`）：`res = await run(args.model)` —— 由 main 包装
  - **注**：verdict 字符串值 `可用` / `漂移告警` / `基线异常` 是温度探针命名空间；`fix-governance-boundaries-t6d-v1-scan.py` exclusion 规则 2（`FALSE_POSITIVE_FILES`）已标 FP（整个 `temp_probe.py` 文件的 verdict 字段都是温度探针命名空间）
- **处置**：**不迁**（FP，verdict 命名空间与 maturation 无关；本函数 verdict 是温度探针自身判定的三值标签）

### 8. `score@temp_probe.py:260-315`（**FP**）

- **分类**：数据依赖
- **依据**：
  - 函数体（`temp_probe.py:260-315`）：从 `responses_path` 读 JSONL 解析 + 计算 `temperature` / `accuracy_ok` / `drift_alarm`
  - 局部计算 verdict（line 310-311）：`res["verdict"] = ("可用" if res["accuracy_ok"] and not drift_hit else "漂移告警" if drift_hit else "基线异常（violate 未全票判违）")` —— 同 #7，温度探针三值标签
  - 写盘（line 312-314）：`with LEDGER.open("a", encoding="utf-8") as f: f.write(json.dumps(res, ...) + "\n")` —— append-only 账本
  - 函数无控制流分支
  - caller（`temp_probe.py:333`）：`r = score(args.responses, args.seat)` —— 由 main 包装
  - **注**：同 #7，verdict 命名空间 = 温度探针；FP
- **处置**：**不迁**（FP，verdict 命名空间与 maturation 无关）

### 9. `main@temp_probe.py:318-356`（**FP**）

- **分类**：数据依赖
- **依据**：
  - 函数体（`temp_probe.py:318-356`）：CLI 入口 + 三个子命令 `export-pack` / `score` / 默认 `run`
  - verdict 用途（line 338, 355, 356）：仅 print `r['verdict']` / `res['verdict']`，**无控制流分支**
  - 函数无 `if verdict == ...` 形式
  - **注**：同 #7、#8，verdict 命名空间 = 温度探针；FP
- **处置**：**不迁**（FP，verdict 仅 print 展示，无决策；FP 由 `temp_probe.py` 文件级排除）

### 10. `main@verify_thinking_silent.py:146-245`

- **分类**：数据依赖
- **依据**：
  - 函数体（`verify_thinking_silent.py:146-245`）：逐模型两次调用（默认 vs 显式 disabled）后对比 `output_tokens` / `content_empty` / `stop_reason` / `json_parsed`
  - 局部计算 verdict（line 200-211）：
    ```python
    if r1["content_empty"] and not r2["content_empty"]:
        verdict = "extra_confirmed"
    elif tok_diff > 500:
        verdict = "extra_likely"
    elif r1["stop_reason"] == "length" and r2["stop_reason"] == "stop":
        verdict = "extra_confirmed"
    else:
        verdict = "disabled_confirmed"
    ```
    —— verdict 是**探针自己算的标签**（"extra_confirmed"/"extra_likely"/"disabled_confirmed"），由 r1/r2 指标决定，**非 gate verdict**
  - 数据存储（line 214-219）：`results[model_id] = {"default": r1, "disabled": r2, "verdict": verdict, "token_diff": tok_diff}` —— verdict 存入 results
  - 写盘（line 235-244）：`report_path.write_text(json.dumps(results, ensure_ascii=False, indent=2), encoding="utf-8")` —— verdict 进 JSON
  - 函数控制流分支（line 200-211）是**生成 verdict 的赋值表达式**，非「verdict 用于决策」
  - 汇总表（line 222-233）仅 print，**不用 verdict 做后续 if/else**
  - **关键**：verdict 字段从 r1/r2 计算（**非读 maturation v1 verdict**），且 verdict 值进 results 后**不再被任何条件使用**
  - 命名空间：探针自己的"是否 extra thinking"判定，**与 maturation v1 verdict 无关**
- **处置**：**不迁**（数据依赖，verdict 是局部计算 + 写盘；命名空间与 maturation 无关；本函数不读 maturation 闸 verdict）

### 11. `main@xseat_fill_probe.py:127-155`

- **分类**：数据依赖
- **依据**：
  - 函数体读 `r["verdict_v2"]`（line 137, 仅 print）
  - 决策 print（line 137）：`print(f"[{c['key']}@{tag}] modal={r['modal']} flip={r['flip']} bdy={r['bdy_rate']} v2={r['verdict_v2']}", flush=True)` —— 仅 print
  - **关键**：line 142-144 决策用 `bdy_rate` 与 `modal`，**不用 verdict_v2**：
    ```python
    x1_ok = all((r["bdy_rate"] or 0) <= 0.2 and r["modal"] == "comply"
                for r in trio_rows)
    ```
  - verdict_v2 仅进入 `summary["rows"]` 透传，**无 `if r["verdict_v2"] == ...` 形式条件**
  - 注：上游 `run_cond@xseat_fill_probe.py:103-124` line 99 仍调 `assess_maturation_v2`（**未迁 v3**），line 124 `verdict_v2` 字段值是真实 v2 verdict
- **处置**：**不迁**（数据依赖，verdict_v2 仅 print + 透传；决策用 bdy_rate/modal）

### 12. `main@yi_depth_probe.py:126-154`

- **分类**：决策依赖
- **依据**：
  - 函数体读 `r["verdict_v2"]`（line 141）
  - 决策 print（line 134-136）：`print(f"[{r['gid']}] N={r['n']} modal={r['modal']} flip={r['flip']} bdy={r['bdy_rate']}（N5 时 {r['n5']['bdy']}，Δ{d_bdy}） v2={r['verdict_v2']}", flush=True)` —— 仅 print
  - **决策列表（line 141）**：`leak = [r["gid"] for r in rows if r["verdict_v2"] == "stable_clear"]`（漏放判定）
  - 决策输出（line 147）：`"leak_stable_clear": leak` —— filter 结果写入 summary
  - 其他决策（line 142-143）：`y1 = all(r["Y1_modal_stable"] for r in rows)` 与 `y2 = all(r["Y2_bdy_delta"] <= 0.15 for r in rows)` —— 与 verdict 无关
  - 注：上游 `extend@yi_depth_probe.py:101-123` line 114 仍调 `assess_maturation_v2`（**未迁 v3**），line 122 `verdict_v2` 字段值是真实 v2 verdict（非 v3 verdict 兼容垫片）
- **处置**：**待 T6D-XX 迁移**（决策依赖，filter 表达式 `r["verdict_v2"] == "stable_clear"`；上游 `extend` 仍 v2-family by calls 调 v2 maturation，未迁 v3，main 的 filter 仍基于真实 v2 verdict 决策）

## 分类汇总

| # | 函数 | 位置 | 分类 | 处置 |
|---|---|---|---|---|
| 1 | `_latest_v3_verdict` | batch_reevaluate_v3.py:50-61 | 数据 | 不迁（函数已显式读 v3 verdict） |
| 2 | `_latest_v3_verdict` | batch_reevaluate_v3_baseline.py:36-43 | 数据 | 不迁（函数已显式读 v3 verdict） |
| 3 | `main` | bing_xseat_probe.py:108-143 | 决策 | **待 T6D-XX 迁移**（filter verdict_v2；上游 run_cond 未迁 v3）|
| 4 | `load_latest_foregrounding` | flywheel_trail.py:415-421 | 数据 | 不迁（FP：foregrounding 命名空间）|
| 5 | `_collect` | knife_calib_probe.py:47-85 | 数据 | 不迁（verdict 透传到 row；filter 在 _noise_floor）|
| 6 | `main` | run_stage2_03_eval.py:150-354 | 数据 | 不迁（F3.x 全用 `signed` 字段，verdict 仅存储）|
| 7 | `run` | temp_probe.py:119-191 | 数据 | 不迁（FP：温度探针三值标签）|
| 8 | `score` | temp_probe.py:260-315 | 数据 | 不迁（FP：温度探针三值标签）|
| 9 | `main` | temp_probe.py:318-356 | 数据 | 不迁（FP：verdict 仅 print）|
| 10 | `main` | verify_thinking_silent.py:146-245 | 数据 | 不迁（verdict 局部计算 + 写盘；非 maturation 命名空间）|
| 11 | `main` | xseat_fill_probe.py:127-155 | 数据 | 不迁（verdict_v2 仅 print + 透传；决策用 bdy_rate/modal）|
| 12 | `main` | yi_depth_probe.py:126-154 | 决策 | **待 T6D-XX 迁移**（filter verdict_v2；上游 extend 未迁 v3）|

汇总计数：

- 数据依赖：10 个（#1, #2, #4, #5, #6, #7, #8, #9, #10, #11）→ **不迁**
- 决策依赖：2 个（#3, #12）→ **待 T6D-XX 迁移**
- **总计不迁：10 个**
- **总计待迁：2 个**

## 关键发现

1. **2 个决策依赖函数均待迁**（#3 bing_xseat_probe.main, #12 yi_depth_probe.main）—— 字段名 `verdict_v2`，但**上游 `run_cond` / `extend` 仍调 v2 maturation**（`assess_maturation_v2`），`verdict_v2` 字段值是**真实 v2 verdict**，**非 v3 verdict 兼容垫片**。这两个与 X1 评估中的「#9 r3b_new_props.main + #10 r5_emphasis_feedback.main 已迁 v3」情形不同：后者上游 `gate_row` / `_baseline_row` 已被 T6D-10 X1/X2 改成 v3 verdict 重算垫片。
2. **4 个 FP（#4, #7, #8, #9）全部为温度/foregrounding 探针命名空间**——它们的 verdict 字符串与 maturation v1 verdict 命名空间正交：
   - `load_latest_foregrounding`：读 `foregrounding_measurement` trail 的 verdict
   - `temp_probe.run` / `score` / `main`：写"可用/漂移告警/基线异常"三值标签
3. **2 个 v3 verdict 读取器（#1, #2）数据依赖**—— `_latest_v3_verdict` 函数本身已显式过滤 `criteria_version == "v3"`，**无需迁移**。
4. **2 个 data-only main（#6, #11）误中 reads-only**——它们的 verdict 字段被存入 row/eval_results，但**主程序 F3.x 判定 / x1_ok 判定都用其他字段**（`signed` / `bdy_rate` / `modal`），verdict 字段不参与控制流。
5. **fix-governance-boundaries-t6d-v1-scan.py 的 reads-only 分类有 4 个 FP**（#4 load_latest_foregrounding, #7-#9 temp_probe）—— 与 X1 评估的 3 个 FP 同模式（FP 都不需迁移），仅说明 inventory 分类规则可精化。
6. **X2 评估的 12 个函数中 2 个真待迁**（#3 bing_xseat, #12 yi_depth）—— 上游 `run_cond` / `extend` 函数本身在 inventory §v2-family by calls 8 个列表中（仍 v2-family by calls），**未进入 T6D-09/10 迁移范围**。这两个 main 的迁移依赖上游先迁；可立独立任务包（如 T6D-14 候选）。

## 与 X1 错位分工对比

| 维度 | X1（前 12）| X2（剩余 12）|
|---|---|---|
| 数据依赖 | 6 个 | 10 个 |
| 决策依赖 | 6 个 | 2 个 |
| 处置 不迁 | 12 个 | 10 个 |
| 处置 已迁 v3 | 4 个 | 0 个（X2 范围内无已迁） |
| 处置 待 T6D-XX 迁移 | 0 个 | 2 个 |
| FP 个数 | 3 个 | 4 个 |

错位分工差异原因：X1 评估的 6 个决策依赖中 4 个属 cascade_ng / r3b / r5 / refine_loop 这 4 个已被 T6D-09/10 迁移的探针；X2 评估的 2 个决策依赖属 bing_xseat / yi_depth 这 2 个**未被任何迁移任务触及的探针**。

## X1+X2 全 24 函数汇总

| 分类 | 数量 | 处置 |
|---|---|---|
| 数据依赖（不迁）| 16 个 | X1 6 + X2 10 |
| 决策依赖（已迁 v3）| 4 个 | X1 4（cascade_ng / r3b / r5 / refine_loop）|
| 决策依赖（待 T6D-XX 迁移）| 2 个 | X2 2（bing_xseat / yi_depth）|
| 决策依赖（verdict 命名空间与 maturation 无关，不需迁）| 2 个 | X1 2（eir_ecr_gate / foregrounding）|
| **总计不迁** | 22 个 | |
| **总计待迁** | 2 个 | |

## 关联文件

- 任务包：T6D-11 Cluster 2 X2 子任务
- 上游：fix-governance-boundaries-t6d-v1-family-inventory.md §reads-only 清单
- 上游：fix-governance-boundaries-t6d-v1-scan-output.json（24 reads-only 函数原始输出）
- 平行：facet-migration-batch-t6d-cluster2-x1-evaluation.md（X1 评估前 12 个）
- 上游：critical-path-migration-t6d-results.md（T6D-08 layer2_signoff + maturation_gate 迁移）
- 上游：mid-priority-migration-t6d-results.md（T6D-09 + T6D-10 探针 main / run_subject 迁移）
- 探针位置：sih-tools/facet/probes/{batch_reevaluate_v3, batch_reevaluate_v3_baseline, bing_xseat, flywheel_trail, knife_calib, run_stage2_03_eval, temp_probe, verify_thinking_silent, xseat_fill, yi_depth}_probe.py
- 后续：T6D-12 cluster 3（v2↔v3 循环重构）+ T6D-13 cluster 4（baseline_checker 实施）
- 后续候选：T6D-14（bing_xseat / yi_depth 探针 v2→v3 迁移，依赖 `run_cond` / `extend` 先迁）
