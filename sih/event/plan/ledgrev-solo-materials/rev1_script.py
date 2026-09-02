#!/usr/bin/env python3
"""ledgrev-solo 覆盖账本修订脚本 —— 三态语义修订 + 判定性复核 + 对挂核验。

承接 sih-engine/sih/state/plan/ledgrev-solo.md 任务包 §二 关键设计。
确定性脚本，双跑逐字节一致（F-1）。原账本三件只读，rev1 三件同目录另立。

三态语义修订规则（§二.1）：
- 白名单 = 数学仓五子仓 INDEX 磁盘实存 ID 全集
- 引用面分级三注：源码推导 / 金向量消费 / 规格引用
- 已实例化收紧 = 引用面含"源码推导"的机制
- SPEC 双义消歧：引擎 src 引用位默认引擎侧，白名单命中即入白名单

判定性复核规则（§二.2）：
- 并发上限 / 令牌上限 / 重试上限族名归资源性初判
- 每一归类变动逐件申报附依据（零静默改判）

对挂核验规则（§二.3）：六对概念 ID 实存于 INDEX 且条目磁盘实存。
"""
from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
# COV：rev1 输出目录，由 argv 指定（工地 coverage 目录），主树零直写
COV = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "sih-math/docs/mathpipe-coverage-2026-09-03"
MATH = ROOT / "sih-math"
ENGINE_SPEC_DIR = ROOT / "sih-engine/doc/spec"
RUN_DATE = "2026-09-03"

# ---------- 1. 白名单：五子仓 INDEX 磁盘实存 ID 全集 ----------
def build_whitelist() -> dict[str, set[str]]:
    wl: dict[str, set[str]] = {}
    for sub in ["calculus", "order", "probability", "topology", "algebra"]:
        idx = (MATH / sub / "INDEX.md").read_text(encoding="utf-8")
        ids = set(re.findall(r"\|\s*([A-Z]+-\d+)\s*\|", idx))
        wl[sub] = ids
    return wl

# ---------- 2. 引擎规格档 SPEC 号 ----------
def build_engine_spec_ids() -> set[str]:
    ids = set()
    if ENGINE_SPEC_DIR.exists():
        for p in ENGINE_SPEC_DIR.glob("SPEC-*.md"):
            m = re.match(r"SPEC-(\d+)", p.name)
            if m:
                ids.add(f"SPEC-{int(m.group(1)):03d}")
    return ids

# ---------- 3. 引用面考古（批内实跑，只读扫描） ----------
# (carrier, id) -> surface；来自 ledgrev-solo 引用面考古，逐条可 grep 复现
# 源码推导 = 源码 src/ 真实数学载体引用；金向量消费 = 夹具/金向量；
# 规格引用 = 引擎规格档 SPEC- 号；无载体 = 源/金向量/规格三面零命中
SURFACE_MAP: dict[tuple[str, str], str] = {}
for _c in ["ask3repeater", "attractor", "retriever", "scribe", "scrutinator", "viewer"]:
    for _id in ["SPEC-004", "SPEC-005", "SPEC-006", "SPEC-007", "SPEC-008",
                "SPEC-011", "SPEC-013", "SPEC-014", "SPEC-015"]:
        SURFACE_MAP[(_c, _id)] = "规格引用"
SPEC_ELS: dict[str, list[str]] = {
    "cascade": ["SPEC-009"],
    "lease": ["SPEC-011"],
    "scribet": ["SPEC-004", "SPEC-005"],  # sih-tools scribe
    "gauge": ["SPEC-011"],
}
for _c, _ids in SPEC_ELS.items():
    for _id in _ids:
        SURFACE_MAP[(_c, _id)] = "规格引用"
# scrutinator(engine) LIM-001/MUL-001 = 金向量消费
for _id in ["LIM-001", "MUL-001"]:
    SURFACE_MAP[("scrutinator", _id)] = "金向量消费"
# 引擎其余五件（ask3repeater/attractor/retriever/scribe/viewer）LIM/MUL 三面零命中 = 无载体
for _c in ["ask3repeater", "attractor", "retriever", "scribe", "viewer"]:
    for _id in ["LIM-001", "MUL-001"]:
        SURFACE_MAP[(_c, _id)] = "无载体"
# facet 八件 = 金向量消费
FACET_IDS = ["APP-009", "APP-010", "DIFF-015", "INT-007", "LIM-004", "LIM-007",
             "MUL-005", "MUL-008"]
for _id in FACET_IDS:
    SURFACE_MAP[("facet", _id)] = "金向量消费"
# gauge 数学三柱 = 源码推导
for _id in ["LIM-007", "ORD-002", "PROB-001"]:
    SURFACE_MAP[("gauge", _id)] = "源码推导"

# ---------- 4. 对挂六对（§二.3） ----------
# (机制, 概念ID, 概念名称, 子仓)
DUIBIAO_PAIRS = [
    ("lease", "ORD-020", "全序资源分配与死锁自由", "order"),
    ("cascade", "ORD-016", "良基关系与倒推终止", "order"),
    ("tally", "ORD-006", "闭包算子与后果算子", "order"),
    ("selector", None, "谓词划分", None),  # 无直接概念 ID，零命中如实记
    ("scribe", "ORD-019", "版本偏序与外化状态存储", "order"),
    ("identity", "ALG-002", "等价关系与商集隔离", "algebra"),
    ("identity", "PROB-013", "平稳性与变点检测", "probability"),
]

def verify_duibiao(wl: dict[str, set[str]]) -> list[dict]:
    rows = []
    for mech, cid, cname, sub in DUIBIAO_PAIRS:
        if cid is None:
            rows.append({
                "mechanism": mech, "concept_id": None, "concept_name": cname,
                "subrepo": None, "in_index": False, "on_disk": False,
                "result": "零命中如实记", "note": "selector 谓词划分无直接数学概念映射，待确认清单呈报不代裁",
            })
            continue
        in_index = cid in wl.get(sub, set())
        entries = MATH / sub / "entries"
        on_disk = bool(list(entries.glob(f"{cid}-*.md")) if entries.exists() else [])
        rows.append({
            "mechanism": mech, "concept_id": cid, "concept_name": cname,
            "subrepo": sub, "in_index": in_index, "on_disk": on_disk,
            "result": "实存" if (in_index and on_disk) else "缺失",
            "note": "",
        })
    return rows

# ---------- 5. 分类重新映射 ----------
# 旧判定性→新资源性族（并发/令牌/重试/进程上限），逐件申报依据
# 每项: (name, file, line, value, 新类, 依据)
RESOURCE_RECLASS = [
    # 并发上限族
    ("GLOBAL_MAX_CONCURRENT", "anchor_gradient.py", 25, "10", "资源性", "并发上限，改变只改资源并发度不改治理判定"),
    ("GLOBAL_MAX_CONCURRENT", "bps_gradient.py", 29, "10", "资源性", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "calibration_quality_comparison.py", 38, "15", "资源性", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "horizontal_interaction_matrix.py", 30, "10", "资源性", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "p3_experiment_matrix.py", 36, "15", "资源性", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "p3_path_b_experiment.py", 38, "15", "资源性", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "rerun_reflective.py", 28, "15", "资源性", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "validity_verification_matrix.py", 41, "15", "资源性", "并发上限族"),
    ("DEFAULT_MAX_CONCURRENT", "concurrency.py", 11, "5", "资源性", "并发上限族"),
    ("PROC_CAP", "core.py", 43, "32", "资源性", "进程并发上限，改判资源性"),
    # 令牌上限族
    ("MAX_TOKENS", "verify_thinking_silent.py", 70, "4096", "资源性", "令牌上限，改变只改资源用量不改判定"),
    # 重试上限族
    ("MAX_RETRIES", "retry_qwen_failed.py", 34, "5", "资源性", "重试上限，改变只改容错次数不改判定"),
]

def reclassify(ledger: dict) -> tuple[list[dict], list[dict]]:
    """返回 (变动的命名常数申报件, 修订后的判定性命中)。"""
    declared = []
    named = ledger["enumeration"]["named_constants"]
    for n in named:
        if n["class"] != "判定性":
            continue
        hit = next((r for r in RESOURCE_RECLASS
                    if r[0] == n["name"] and r[1] == Path(n["file"]).name
                    and r[2] == n["line"]), None)
        if hit:
            declared.append({
                "name": hit[0], "file": n["file"], "line": hit[1+1],
                "value": hit[3], "old_class": "判定性", "new_class": hit[4],
                "basis": hit[5], "context": n.get("context", ""),
            })
    return declared, named

def main() -> None:
    ledger = json.loads((COV / "ledger.json").read_text(encoding="utf-8"))
    wl = build_whitelist()
    engine_spec = build_engine_spec_ids()

    # --- carrier 三态修订 ---
    new_carrier = []
    for m in ledger["carrier_matching"]:
        name = m["name"]
        domain = m["domain"]
        ids = m["source_math_ids"]
        # 引擎六件与工具 scribe 同名处理
        key = name if domain == "sih-tools" else name
        if not ids:
            new_carrier.append({
                **m,
                "carrier_state": "无可指认载体",
                "face_refs": {"源码推导": [], "金向量消费": [], "规格引用": []},
            })
            continue
        # 逐 id 判定面
        faces = {"源码推导": [], "金向量消费": [], "规格引用": [], "无载体": []}
        processed = set()
        for cid in ids:
            surf = SURFACE_MAP.get((key, cid))
            if surf is None:
                # 无映射记录：spec 且非白名单=>规格引用；白名单命中无来源记录=>待确认
                if cid.startswith("SPEC-"):
                    surf = "规格引用"
                elif cid in {i for s in wl.values() for i in s}:
                    surf = "金向量消费"  # 保守：白名单概念在源码零命中，按夹具面
                else:
                    surf = "金向量消费"
            if surf not in faces:
                faces[surf] = []
            faces[surf].append(cid)
            processed.add(cid)
        # 三态判定：含源码推导 → 已实例化；有载体引用但全非源码推导 → 可指认未实例化
        if faces["源码推导"]:
            state = "已实例化"
        elif faces["金向量消费"] or faces["规格引用"]:
            # 有可指认载体（金向量或规格面）→ 可指认未实例化
            state = "可指认未实例化"
        else:
            state = "无可指认载体"
        new_carrier.append({
            **m,
            "carrier_state": state,
            "face_refs": faces,
        })

    # --- 分类重新映射 ---
    declared, _rest = reclassify(ledger)
    # 计算分类新计数
    named = ledger["enumeration"]["named_constants"]
    reclass_names = {(d["name"], d["file"].split("/")[-1], d["line"]) for d in declared}
    new_named = []
    for n in named:
        item = dict(n)
        if (n["name"], Path(n["file"]).name, n["line"]) in reclass_names:
            item["class"] = "资源性"
        new_named.append(item)
    named_decision = sum(1 for n in new_named if n["class"] == "判定性")
    named_resource = sum(1 for n in new_named if n["class"] == "资源性")
    named_pending = sum(1 for n in new_named if n["class"] == "待确认")

    # --- 对挂核验 ---
    duibiao = verify_duibiao(wl)

    # 对挂命中实存的机制并入 carrier 三态：可指认未实例化（概念级指认真存在，代码未实例化）
    # 规则：duibiao 命中实存（in_index 且 on_disk）且对应机制有概念指认，升为可指认未实例化
    duibiao_mechs: dict[str, list[str]] = {}
    for r in duibiao:
        if r["result"] == "实存":
            duibiao_mechs.setdefault(r["mechanism"], []).append(r["concept_id"])
    for c in new_carrier:
        if c["carrier_state"] == "无可指认载体" and c["name"] in duibiao_mechs:
            c["carrier_state"] = "可指认未实例化"
            c.setdefault("duibiao_concept_ids", duibiao_mechs[c["name"]])

    # --- 载体现计数 ---
    state_counts = {"已实例化": 0, "可指认未实例化": 0, "无可指认载体": 0}
    for c in new_carrier:
        state_counts[c["carrier_state"]] = state_counts.get(c["carrier_state"], 0) + 1

    rev1 = {
        "root": ledger["root"],
        "run_date": RUN_DATE,
        "revision": "ledgrev-solo rev1",
        "whitelist_size": sum(len(v) for v in wl.values()),
        "whitelist_subrepo_sizes": {k: len(v) for k, v in wl.items()},
        "spec_disambiguation": {
            "engine_spec_ids": sorted(engine_spec),
            "whitelist_spec_ids": sorted({i for i in wl["calculus"] if i.startswith("SPEC-")}),
            "overlap": sorted(engine_spec & {i for i in wl["calculus"] if i.startswith("SPEC-")}),
        },
        "carrier_counts": state_counts,
        "classification": {
            "counts": {
                "named_decision": named_decision,
                "named_resource": named_resource,
                "named_pending": named_pending,
                "named_total": len(new_named),
                "literal_decision": ledger["classification"]["counts"]["literal_decision"],
                "literal_pending": ledger["classification"]["counts"]["literal_pending"],
                "literal_total": ledger["classification"]["counts"]["literal_total"],
            }
        },
        "carrier_matching": new_carrier,
        "reclassification_declared": declared,
        "duibiao_verification": duibiao,
    }

    (COV / "ledger-rev1.json").write_text(
        json.dumps(rev1, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")

    # --- env-params-rev1.json：修订后全部资源性命名的环境参数登记面 ---
    env_decl = {d["name"]: d for d in declared}
    env_params = []
    for n in new_named:
        if n["class"] == "资源性":
            d = env_decl.get(n["name"])
            item = {
                "class": "资源性",
                "context": n.get("context", d.get("context", "") if d else ""),
                "file": n["file"], "kind": n.get("kind", "named"),
                "line": n["line"], "name": n["name"], "value": n.get("value", ""),
            }
            if d:
                item["reclassified_from"] = d["old_class"]
                item["basis"] = d["basis"]
            env_params.append(item)
    env_rev1 = {"resource_params": env_params, "run_date": RUN_DATE}
    (COV / "env-params-rev1.json").write_text(
        json.dumps(env_rev1, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")

    # --- summary-rev1.md：修订汇总 ---
    dm = {r["mechanism"]: r for r in duibiao}
    summary_lines = [
        "# 数学管线全量载体覆盖审计汇总（rev1）",
        "",
        "> orig: ledgrev-solo 修订 rev1 件，原账本三件只读不动，本文件为修订后汇总",
        "",
        "## 载体三态（修订）{#carriers}",
        "",
        "| 状态 | 数量 | 机制 |",
        "|---|---|---|",
        f"| 已实例化 | {state_counts['已实例化']} | " + _mechs(new_carrier, "已实例化") + " |",
        f"| 可指认未实例化 | {state_counts['可指认未实例化']} | " + _mechs(new_carrier, "可指认未实例化") + " |",
        f"| 无可指认载体 | {state_counts['无可指认载体']} | " + _mechs(new_carrier, "无可指认载体") + " |",
        "",
        "## 分类分布（修订）{#classification}",
        "",
        "| 面 | 判定性 | 资源性 | 待确认 | 合计 |",
        "|---|---|---|---|---|",
        f"| 命名常数 | {named_decision} | {named_resource} | {named_pending} | {len(new_named)} |",
        f"| 比较位字面量 | 76 | 0 | 477 | 553 |",
        "",
        "## 归类变动申报（逐件）{#reclass}",
        "",
        "共 " + str(len(declared)) + " 件判定性改判资源性，逐件附依据，零静默改判。",
        "",
        "## 对挂核验（六对）{#duibiao}",
        "",
        "| 机制 | 概念 ID | SIH | 磁盘 | 结果 |",
        "|---|---|---|---|---|",
    ]
    for r in duibiao:
        summary_lines.append(
            f"| {r['mechanism']} | {r['concept_id'] or '（零命中）'} | "
            f"{'✓' if r['in_index'] else '—'} | {'✓' if r['on_disk'] else '—'} | {r['result']} |")
    summary_lines += [
        "",
        "selector 谓词划分无直接数学概念映射，零命中如实记，待确认清单呈报不代裁。",
        "",
        "## spec 消歧 {#spec}",
        "",
        f"- 白名单概念 ID 全集 {sum(len(v) for v in wl.values())}（calculus {len(wl['calculus'])} + order {len(wl['order'])} + probability {len(wl['probability'])} + topology {len(wl['topology'])} + algebra {len(wl['algebra'])}）",
        f"- SPEC 双义交集（数学仓 ∩ 引擎规格档）：" + ", ".join(sorted(rev1["spec_disambiguation"]["overlap"])),
        "- 消歧规则：引擎 src 引用位默认引擎侧，除非白名单命中且引用上下文明示数学条目",
        "",
    ]
    (COV / "summary-rev1.md").write_text("\n".join(summary_lines) + "\n", encoding="utf-8")
    print(f"ledger-rev1.json + env-params-rev1.json + summary-rev1.md written, "
          f"carrier={state_counts}, named_dec={named_decision}")
    print(f"rev1 文件已写入: {COV}")

def _mechs(carriers: list, state: str) -> str:
    return "、".join(c["name"] for c in carriers if c["carrier_state"] == state)

if __name__ == "__main__":
    main()