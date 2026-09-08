#!/usr/bin/env python3
"""constclear2-solo 核验脚本：34 件源码常数位现值对表（只读，零写入源码）。
输出 JSON 证据件：每件 found/line_actual/line_doc/value_match/value_drift。
匹配规则：常数名出现且赋值语义行内取值解析相等（int/float），不按行号硬断，漂移单列。"""
import json
import re
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")

ITEMS = [
    # (id, file, doc_line, name, value_str, group, state_0904)
    ("d1", "sih-tools/facet/src/facet_stats_inf.py", 42, "ALPHA", "0.05", "derived", "推导"),
    ("d2", "sih-tools/facet/probes/bootstrap_partial.py", 34, "ALPHA", "0.05", "derived", "推导"),
    ("d3", "sih-tools/facet/probes/d1_anova.py", 52, "ALPHA", "0.05", "derived", "推导"),
    ("d4", "sih-tools/gauge/src/gauge/cli.py", 66, "PRIOR_ALPHA", "1.0", "derived", "推导"),
    ("d5", "sih-tools/gauge/src/gauge/cli.py", 67, "PRIOR_BETA", "1.0", "derived", "推导"),
    ("d6", "sih-tools/gauge/src/gauge/cli.py", 74, "GQ_WINDOW_DAYS_DEFAULT", "7", "derived", "推导"),
    ("d7", "sih-tools/gauge/src/gauge/cli.py", 77, "GD_WINDOW_DAYS_DEFAULT", "7", "derived", "推导"),
    ("d8", "sih-tools/facet/probes/reference_stats.py", 30, "MIN_DECIDED", "30", "derived", "推导"),
    ("r1", "sih-tools/facet/probes/eir_ecr_gate_probe.py", 78, "MAX_ROUND", "3", "reclassified", "改判"),
    ("r2", "sih-tools/facet/probes/lightweight_mode_probe.py", 146, "MAX_SHOTS", "7", "reclassified", "改判"),
    ("r3", "sih-tools/facet/probes/multiround_convergence_probe.py", 108, "MAX_ROUND", "5", "reclassified", "改判"),
    ("f1", "sih-engine/src/attractor/tally.rs", 34, "BUDGET_PER_GID", "9", "frozen", "冻结"),
    ("f2", "sih-tools/tally/src/tally/cli.py", 32, "BUDGET_PER_GID", "9", "frozen", "冻结"),
    ("f3", "sih-tools/facet/probes/analyze_merged_v2.py", 29, "F21_THRESHOLD", "0.25", "frozen", "冻结"),
    ("f4", "sih-tools/facet/probes/analyze_merged_v2.py", 30, "F22_THRESHOLD", "0.10", "frozen", "冻结"),
    ("f5", "sih-tools/facet/probes/analyze_merged_v2.py", 31, "F23_THRESHOLD", "0.20", "frozen", "冻结"),
    ("f6", "sih-tools/facet/probes/analyze_merged_v2.py", 32, "F24_THRESHOLD", "0.05", "frozen", "冻结"),
    ("f7", "sih-tools/facet/probes/analyze_merged_v2.py", 33, "F25_THRESHOLD", "0.05", "frozen", "冻结"),
    ("f8", "sih-tools/facet/probes/check_layer_proportion.py", 48, "DEFAULT_FLYWHEEL_RUN_MAX_SHARE", "0.70", "frozen", "冻结"),
    ("f9", "sih-tools/facet/probes/check_layer_proportion.py", 53, "DEFAULT_LAYER2PLUS_MIN_SHARE", "0.05", "frozen", "冻结"),
    ("f10", "sih-tools/facet/probes/contribution_metric.py", 85, "THRESHOLD_FLYWHEEL_RUN_RATIO", "0.60", "frozen", "冻结"),
    ("f11", "sih-tools/facet/src/facet_stats_inf.py", 48, "DEFAULT_BOUNDARY_BASELINE", "0.004", "frozen", "冻结"),
    ("f12", "sih-tools/facet/probes/r3a_gate_v2.py", 63, "BOUNDARY_RATE_THRESHOLD", "0.34", "frozen", "冻结"),
    ("f13", "sih-tools/facet/probes/contribution_metric.py", 65, "THRESHOLD_N", "5000", "frozen", "冻结"),
    ("f14", "sih-tools/facet/probes/contribution_metric.py", 66, "THRESHOLD_C", "5000.0", "frozen", "冻结"),
    ("f15", "sih-tools/facet/probes/contribution_metric.py", 68, "THRESHOLD_NEW_MECHANISM_COVERAGE", "1", "frozen", "冻结"),
    ("f16", "sih-tools/facet/probes/program_signoff.py", 99, "USER_POSTURE_MIN", "0.0", "frozen", "冻结"),
    ("f17", "sih-tools/facet/probes/program_signoff.py", 100, "USER_POSTURE_MAX", "2.0", "frozen", "冻结"),
    ("f18", "sih-tools/facet/probes/program_signoff.py", 107, "TIME_WINDOW_HOURS_DEFAULT", "0", "frozen", "冻结"),
    ("f19", "sih-tools/facet/probes/run_siliconflow_factcheck.py", 47, "VERIFY_TOKEN_DIFF_THRESHOLD", "500", "frozen", "冻结"),
    ("f20", "sih-tools/identity/src/identity/core.py", 47, "SKEW_MAX_SECONDS", "300", "frozen", "冻结"),
    ("f21", "sih-tools/facet/probes/n_convergence_probe.py", 31, "EPS", "0.1", "frozen", "冻结"),
    ("f22", "sih-tools/gauge/src/gauge/cli.py", 79, "GD_ALPHA_DEFAULT", "0.05", "frozen", "冻结"),
    ("f23", "sih-tools/facet/probes/check_verdict_consistency.py", 48, "DEFAULT_INCONSISTENCY_MAX_RATE", "0.0", "frozen", "冻结"),
]

ASSIGN = re.compile(r"^\s*(?:pub\s+)?(?:const\s+)?([A-Z_][A-Z0-9_]*)\s*[:=]")

def to_num(s):
    try:
        f = float(s)
        return int(f) if f == int(f) and "." not in s and "e" not in s.lower() else f
    except ValueError:
        return None

results = []
for iid, rel, doc_line, name, val_s, group, st in ITEMS:
    p = ROOT / rel
    rec = {"id": iid, "file": rel, "doc_line": doc_line, "name": name,
           "doc_value": val_s, "group": group, "state_0904": st}
    if not p.exists():
        rec.update(found=False, error="file missing")
        results.append(rec)
        continue
    expected = to_num(val_s)
    hits = []
    for i, line in enumerate(p.read_text(encoding="utf-8").splitlines(), 1):
        m = ASSIGN.match(line)
        if m and m.group(1) == name:
            tail = line.split("=", 1)[1] if "=" in line else ""
            tail = tail.split("#")[0].split(";")[0].strip()
            actual = to_num(tail)
            hits.append({"line": i, "text": line.strip()[:100], "value": actual})
    val_match = any(h["value"] is not None and expected is not None and abs(float(h["value"]) - float(expected)) < 1e-12 for h in hits)
    rec.update(found=bool(hits), hits=hits, value_match=val_match,
               line_drift=([h["line"] for h in hits] != [doc_line]) if hits else None)
    results.append(rec)

summary = {
    "total": len(results),
    "found": sum(1 for r in results if r.get("found")),
    "value_match": sum(1 for r in results if r.get("value_match")),
    "line_drift": sum(1 for r in results if r.get("line_drift")),
    "missing": [r["id"] for r in results if not r.get("found")],
    "value_drift": [r["id"] for r in results if r.get("found") and not r.get("value_match")],
}
out = {"at": "2026-09-08", "batch": "constclear2-solo", "summary": summary, "items": results}
dest = Path(__file__).parent / "verify-constants-2026-09-08.json"
dest.write_text(json.dumps(out, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(json.dumps(summary, ensure_ascii=False))
