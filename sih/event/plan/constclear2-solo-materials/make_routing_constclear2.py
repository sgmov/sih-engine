#!/usr/bin/env python3
"""constclear2-solo 路由表生成器：34 件两态归属，md+json 双形态同源产出。
路由判定依据三层对账：constclear-solo 2026-09-04 三分账、constmodel-solo 2026-09-05 模型化处置、
pk-053 出泊笔 41fb339c 两态口径令（2026-09-08）。数值零改动，账面归因。"""
import json
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
OUT_MD = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/constclear2-solo/docs/constclear2-routing-2026-09-08.md")
OUT_JSON = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/constclear2-solo/docs/constclear2-routing-2026-09-08.json")

CALIBER = "两态制：数学模型态（载体条目或在档推导档实文承载判定语义与取值锚，过 M-4）优先；工程实践三件套态（引擎验证证据＋回归数学模型路径＋登记行）；冻结态撤销零残留。退场核销非归因态，系对象消亡的显式核销记录。"

# 冻结 23 件路由（id 与 verify_constants.py 对齐；constmodel 处置承 constmodel-solo-results.md §1）
FROZEN = {
    "f1": {"route": "engineering_practice", "cm": "定义化（锚 PROB-015 在途界形态）",
           "evidence": "attractor tally R6 与 tally cli R6 双形在役，历次 facet 九发测量与执契终签在链",
           "regress": "置信度线样本量-在途界模型立项时由功效与在途界理论重估九发数（载体族 PROB，pk-073 系与 pk-078 gate）"},
    "f2": {"route": "engineering_practice", "cm": "定义化（同 f1，围堰形同源）",
           "evidence": "tally cli R6 在役，与引擎形同参双跑先例在档",
           "regress": "同 f1"},
    "f3": {"route": "engineering_practice", "cm": "定义化（锚 PROB-010 比率判据形态）",
           "evidence": "阶段二标定批在案（修订放宽 15pp 注记），analyze_merged_v2 计分在役",
           "regress": "误报率与容许差异分布化后按分位数重推标定点（PROB-010 族）"},
    "f4": {"route": "engineering_practice", "cm": "定义化（同族）", "evidence": "同 f3 在役闸判据",
           "regress": "同 f3"},
    "f5": {"route": "engineering_practice", "cm": "定义化（同族）", "evidence": "同 f3 在役闸判据",
           "regress": "同 f3"},
    "f6": {"route": "engineering_practice", "cm": "定义化（同族）", "evidence": "同 f3 在役闸判据",
           "regress": "同 f3"},
    "f7": {"route": "engineering_practice", "cm": "定义化（同族）", "evidence": "同 f3 在役闸判据（429 容忍修订注记在案）",
           "regress": "同 f3"},
    "f8": {"route": "engineering_practice", "cm": "定义化（实测基线预登记）",
           "evidence": "check_layer_proportion 实测 73.2% 基线预登记在档，探针在役",
           "regress": "比例界后验化：PROB-005 Beta-Bernoulli 后验分位数形重推上界"},
    "f9": {"route": "engineering_practice", "cm": "定义化（实测基线预登记）",
           "evidence": "check_layer_proportion 实测 0.44% 基线预登记在档",
           "regress": "同 f8"},
    "f10": {"route": "engineering_practice", "cm": "定义化（锚 PROB-016，源码自承治理意图+实测）",
            "evidence": "contribution_metric 在役，自承非推导注记在案",
            "regress": "贡献度比值判据分布化（PROB-016 计数测度比值检验形）"},
    "f11": {"route": "math_model", "cm": "承接（PROB-010 + facetmath-derivation §2 检验零假设 p_h0 实文）",
            "evidence": "constmodel-solo 2026-09-05 承接裁定；_BASELINE_FLIP_RATES_FROZEN 冻结机制在役；PROB-010 L12/L16/L24 引文逐字节复核命中（2026-09-08）",
            "regress": "已承载体；平台或模型迁移时基线重标定并声明（冻结机制既载）"},
    "f12": {"route": "math_model", "cm": "承接（PROB-010 + facetmath-derivation 在役判挂线与 near 带与 B2 红线实文）",
            "evidence": "constmodel-solo 2026-09-05 承接裁定；r3a_gate_v2 在役判据；注记：切换命题挂起 pk-049 不阻归因（facet ALPHA 先例同形）",
            "regress": "已承载体；pk-049 重启时挂线重裁走 PROB-010 水平先验语义"},
    "f13": {"route": "engineering_practice", "cm": "定义化（锚 PROB-016）",
            "evidence": "contribution_metric 贡献度判定在役（引擎承重三判据规格在档）",
            "regress": "达标判定的估计精度与功效语义化（PROB-003/PROB-016 族）"},
    "f14": {"route": "engineering_practice", "cm": "定义化（锚 PROB-016）", "evidence": "同 f13", "regress": "同 f13"},
    "f15": {"route": "engineering_practice", "cm": "定义化（锚 PROB-016）",
            "evidence": "contribution_metric 新机制覆盖判定在役",
            "regress": "覆盖度量化评估义务：转率或比例形则归 PROB 族；维持纯计数约定则后继批申报依据"},
    "f16": {"route": "engineering_practice", "cm": "定义化（用户裁决量纲域 ±100% 围绕 1.0）",
            "evidence": "任务包用户裁决在链，program_signoff 在役",
            "regress": "姿态尺度进加权聚合公式时归 ORD/ALG 载体评估"},
    "f17": {"route": "engineering_practice", "cm": "定义化（同 f16）", "evidence": "同 f16", "regress": "同 f16"},
    "f18": {"route": "engineering_practice", "cm": "定义化（特征关闭缺省）",
            "evidence": "program_signoff 缺省不过滤显式在案",
            "regress": "特性启用时按 PROB-013 窗口对价公理取值（GD_WINDOW 同族先例）"},
    "f19": {"route": "engineering_practice", "cm": "定义化（启发式分档）",
            "evidence": "run_siliconflow_factcheck 分档在役",
            "regress": "token 差分布统计化（PROB-010/PROB-003 语义）；或维持启发式并申报非统计量"},
    "f20": {"route": "engineering_practice", "cm": "定义化（量纲容差约定）",
            "evidence": "identity verify 历批 anomalies 零（0.4.0 在役）",
            "regress": "时钟偏移误差概率模型化评估（PROB 族）"},
    "f21": {"route": "engineering_practice", "cm": "定义化（ε 实例化约定，锚 LIM-007 概念非值级）",
            "evidence": "n_convergence probe 收敛判定在役；09-04 档自承不硬挂 LIM-007",
            "regress": "收敛容差检验形转化与 pk-049 联动（LIM-007/PROB 族）；值级锚义务登记不硬凑"},
    "f22": {"route": "retired_void", "cm": "呈裁后对象退场（gd-1 推断式常数族 α/p0/k_sigma/cusum 已退场，源码注释在役）",
            "evidence": "gauge cli.py 现源：gd-1 已退场 PROB-010/013 仅作理论史登记不参与新判据面（2026-09-08 核验脚本 found=false 红证）；constmodel 呈裁记录在档",
            "regress": "无在役值可归因；退场核销非第三归因态，显式核销申报（应而不藏）；gd 判据面现由 gd-2 合同参数承载（承 P3.2.1 与 PROB-016）"},
    "f23": {"route": "engineering_practice", "cm": "定义化（定性公理，锚 PROB-016）",
            "evidence": "check_verdict_consistency 零容忍判定在役（bug 性质非统计噪声注记在案）",
            "regress": "噪声成分检出时转 PROB-010 检验形；否则维持定性零容忍并申报非统计量"},
}

verify_raw = json.loads((Path(__file__).parent / "verify-constants-2026-09-08.json").read_text(encoding="utf-8"))
verify = {r["id"]: r for r in verify_raw["items"]}

def vrow(iid):
    v = verify[iid]
    return {"found": v["found"], "value_match": v["value_match"],
            "actual_lines": [h["line"] for h in v.get("hits", [])], "doc_line": v["doc_line"]}

out = {
    "batch": "constclear2-solo", "date": "2026-09-08",
    "session": "sess-zcode-20260908-fork2-constclear2",
    "caliber": CALIBER,
    "order_sources": ["constclear-solo 2026-09-04 三分账（sih-math/docs/constclear-derivation-2026-09-04.md）",
                      "constmodel-solo 2026-09-05 模型化处置（sih-math/docs/constmodel-derivation-2026-09-05.md 与结果档）",
                      "pk-053 出泊笔 41fb339c 两态口径令（2026-09-08 当日链）"],
    "verify_evidence": "sih-engine/sih/event/plan/constclear2-solo-materials/verify-constants-2026-09-08.json（33/34 found 且 value_match，f22 退场红证，d7 行漂移照录）",
    "distribution": {},
    "items": []
}
derived_ids = ["d1","d2","d3","d4","d5","d6","d7","d8"]
reclass_ids = ["r1","r2","r3"]
for iid in derived_ids:
    out["items"].append({"id": iid, "state_0904": "推导", "new_state": "derived_confirmed",
                         "note": "载体引文逐字节复核命中，归因维持 09-04 档", "verify": vrow(iid)})
for iid in reclass_ids:
    out["items"].append({"id": iid, "state_0904": "改判", "new_state": "reclassified_pending_ruling",
                         "note": "env-params overlay 三行在账已核验；RECLASS_INHERIT 未收编债照录；呈候人节点裁", "verify": vrow(iid)})
for iid, r in FROZEN.items():
    out["items"].append({"id": iid, "state_0904": "冻结", "new_state": r["route"],
                         "constmodel_disposition": r["cm"], "engine_evidence": r["evidence"],
                         "regression_path": r["regress"], "verify": vrow(iid)})

from collections import Counter
cnt = Counter(i["new_state"] for i in out["items"])
out["distribution"] = dict(cnt)

# MD 渲染
md = ["# constclear2 路由表：判定性命名常数 34 件两态清账（批一·账面盘点）", "",
      "> 批：constclear2-solo（2026-09-08，会话 sess-zcode-20260908-fork2-constclear2）",
      "> 口径：" + CALIBER, "",
      "## 1 三层令源 {#sources}", ""]
for s in out["order_sources"]:
    md.append(f"- {s}")
md += ["", "## 2 分布总览 {#distribution}", ""]
md.append("| 态 | 件数 |")
md.append("|---|---|")
label = {"derived_confirmed": "推导复核归档（维持 09-04）", "reclassified_pending_ruling": "改判呈裁（候人节点）",
         "math_model": "数学模型态", "engineering_practice": "工程实践三件套态", "retired_void": "退场核销（非归因态）"}
for k, n in sorted(cnt.items()):
    md.append(f"| {label.get(k,k)} | {n} |")
md += ["", f"合计 34 件；核验证据件：{out['verify_evidence']}", "",
       "## 3 推导八件（复核归档）{#derived}", "",
       "| id | 常数位 | 现值 | 复核 |", "|---|---|---|---|"]
for i in out["items"]:
    if i["id"].startswith("d"):
        v = verify[i["id"]]
        site = f"{v['file']}:{v['doc_line']}"
        md.append(f"| {i['id']} | {site} {v['name']} | {v['doc_value']} | 引文逐字节命中，值同（现行号 {i['verify']['actual_lines']}） |")
md += ["", "## 4 改判三件（呈候人裁）{#reclassified}", "",
       "| id | 常数位 | 现值 | 呈裁事项 |", "|---|---|---|---|"]
for i in out["items"]:
    if i["id"].startswith("r"):
        v = verify[i["id"]]
        md.append(f"| {i['id']} | {v['file']}:{v['doc_line']} {v['name']} | {v['doc_value']} | env-params overlay 三行在账已核验；RECLASS_INHERIT 未收编债照录候裁 |")
md += ["", "## 5 冻结廿三件两态路由 {#frozen}", "",
       "| id | 常数位 | 现值 | 新态 | constmodel 处置 | 引擎验证证据 | 回归数学模型路径 |", "|---|---|---|---|---|---|---|"]
for i in out["items"]:
    if i["id"].startswith("f"):
        v = verify[i["id"]]
        site = f"{v['file']}:{i['verify']['actual_lines'][0] if i['verify']['actual_lines'] else '退场'}"
        md.append(f"| {i['id']} | {site} {v['name']} | {v['doc_value']} | {label.get(i['new_state'],i['new_state'])} | {i['constmodel_disposition']} | {i['engine_evidence']} | {i['regression_path']} |")
md += ["", "## 6 账面漂移申报 {#drift}", "",
       "- f22 GD_ALPHA_DEFAULT 源码退场（gd-1 推断式常数族 α/p0/k_sigma/cusum 已退场，PROB-010/013 转理论史登记）：核验红证在档，按退场核销记录不强行两态归属",
       "- d7 GD_WINDOW_DAYS_DEFAULT 行漂移 77→78（值 7 不变，后续注释插入所致）",
       "- 账面外新增常数族在役（GC_K_SIGMA/GC_CUSUM_B/GC_CUSUM_H/RHO_BOUND/GD_UNION/GD_SUSTAINED/GD_MONOTONICITY 等，承 gchart 与 queueing 与 contribmath 批及引擎承重三判据规格）：rev3 34 件账本未覆盖，候后继清账批扫描收编", "",
       "## 7 范畴排除 {#exclusions}", "",
       "- 本表是账面归因记录，零数值变更零代码注记；后继批拆分见结果档；改判裁与拆分裁候人节点", ""]
OUT_MD.write_text("\n".join(md) + "\n", encoding="utf-8")
OUT_JSON.write_text(json.dumps(out, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print("written", OUT_MD.name, OUT_JSON.name, "| 分布:", dict(cnt))
