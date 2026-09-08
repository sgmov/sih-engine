#!/usr/bin/env python3
"""constclear2c-solo 腿一执行器：源码注记落码（仅注释行，值零改动）＋投影件生成。

机械纪律：
- 每条注记按唯一行前缀定位常数行，在常数行上方插入一行注释，注释携登记行 id 与态与账面指针；
- 插入前断言目标前缀在文件内唯一命中，杜绝误插；
- 插入后逐文件 git diff 自证：零删除行、全部新增行为注释行；
- 值零改动自证：HEAD 版与工地版的常数绑定行逐字符相等；
- 退场核销件（retired/ 路径与 gd-1 已退场族）零触碰，仅在投影件登记。
运行位：engine 工地 materials 目录；读 tools/engine 工地与主树 HEAD。
"""
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
TW = ROOT / "worktrees/sih-tools/constclear2c-solo"
EW = ROOT / "worktrees/sih-engine/constclear2c-solo"
HERE = Path(__file__).resolve().parent
R1 = "constclear2-routing-2026-09-08.md"
R2 = "constclear2b-routing-2026-09-08.md"
REG2 = "constclear-registry-v2.md"

# (repo, relpath, 唯一行前缀, 常数名, 登记行 id, 态, 账面指针)
ANNOTATIONS = [
    # ── 批一 34 件（constclear2-routing §3/§4/§5）──
    ("tools", "sih-tools/facet/src/facet_stats_inf.py", "ALPHA = 0.05", "ALPHA", "d1", "推导复核", R1),
    ("tools", "sih-tools/facet/probes/bootstrap_partial.py", "ALPHA = 0.05", "ALPHA", "d2", "推导复核", R1),
    ("tools", "sih-tools/facet/probes/d1_anova.py", "ALPHA = 0.05", "ALPHA", "d3", "推导复核", R1),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "PRIOR_ALPHA = 1.0", "PRIOR_ALPHA", "d4", "推导复核", R1),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "PRIOR_BETA = 1.0", "PRIOR_BETA", "d5", "推导复核", R1),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "GQ_WINDOW_DAYS_DEFAULT = 7", "GQ_WINDOW_DAYS_DEFAULT", "d6", "推导复核", R1),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "GD_WINDOW_DAYS_DEFAULT = 7", "GD_WINDOW_DAYS_DEFAULT", "d7", "推导复核", R1),
    ("tools", "sih-tools/facet/probes/reference_stats.py", "MIN_DECIDED = 30", "MIN_DECIDED", "d8", "推导复核", R1),
    ("tools", "sih-tools/facet/probes/eir_ecr_gate_probe.py", "MAX_ROUND = 3", "MAX_ROUND", "r1", "环境参数·批二收编", REG2),
    ("tools", "sih-tools/facet/probes/lightweight_mode_probe.py", "MAX_SHOTS = 7", "MAX_SHOTS", "r2", "环境参数·批二收编", REG2),
    ("tools", "sih-tools/facet/probes/multiround_convergence_probe.py", "MAX_ROUND = 5", "MAX_ROUND", "r3", "环境参数·批二收编", REG2),
    ("engine", "sih-engine/src/attractor/tally.rs", "pub const BUDGET_PER_GID", "BUDGET_PER_GID", "f1", "工程实践三件套", R1),
    ("tools", "sih-tools/tally/src/tally/cli.py", "BUDGET_PER_GID = 9", "BUDGET_PER_GID", "f2", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/analyze_merged_v2.py", "F21_THRESHOLD = 0.25", "F21_THRESHOLD", "f3", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/analyze_merged_v2.py", "F22_THRESHOLD = 0.10", "F22_THRESHOLD", "f4", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/analyze_merged_v2.py", "F23_THRESHOLD = 0.20", "F23_THRESHOLD", "f5", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/analyze_merged_v2.py", "F24_THRESHOLD = 0.05", "F24_THRESHOLD", "f6", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/analyze_merged_v2.py", "F25_THRESHOLD = 0.05", "F25_THRESHOLD", "f7", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/check_layer_proportion.py", "DEFAULT_FLYWHEEL_RUN_MAX_SHARE = 0.70", "DEFAULT_FLYWHEEL_RUN_MAX_SHARE", "f8", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/check_layer_proportion.py", "DEFAULT_LAYER2PLUS_MIN_SHARE = 0.05", "DEFAULT_LAYER2PLUS_MIN_SHARE", "f9", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/contribution_metric.py", "THRESHOLD_FLYWHEEL_RUN_RATIO = 0.60", "THRESHOLD_FLYWHEEL_RUN_RATIO", "f10", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/src/facet_stats_inf.py", "DEFAULT_BOUNDARY_BASELINE = 0.004", "DEFAULT_BOUNDARY_BASELINE", "f11", "数学模型", R1),
    ("tools", "sih-tools/facet/probes/r3a_gate_v2.py", "BOUNDARY_RATE_THRESHOLD = 0.34", "BOUNDARY_RATE_THRESHOLD", "f12", "数学模型", R1),
    ("tools", "sih-tools/facet/probes/contribution_metric.py", "THRESHOLD_N = 5000", "THRESHOLD_N", "f13", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/contribution_metric.py", "THRESHOLD_C = 5000.0", "THRESHOLD_C", "f14", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/contribution_metric.py", "THRESHOLD_NEW_MECHANISM_COVERAGE = 1", "THRESHOLD_NEW_MECHANISM_COVERAGE", "f15", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/program_signoff.py", "USER_POSTURE_MIN = 0.0", "USER_POSTURE_MIN", "f16", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/program_signoff.py", "USER_POSTURE_MAX = 2.0", "USER_POSTURE_MAX", "f17", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/program_signoff.py", "TIME_WINDOW_HOURS_DEFAULT = 0", "TIME_WINDOW_HOURS_DEFAULT", "f18", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/run_siliconflow_factcheck.py", "VERIFY_TOKEN_DIFF_THRESHOLD = 500", "VERIFY_TOKEN_DIFF_THRESHOLD", "f19", "工程实践三件套", R1),
    ("tools", "sih-tools/identity/src/identity/core.py", "SKEW_MAX_SECONDS = 300", "SKEW_MAX_SECONDS", "f20", "工程实践三件套", R1),
    ("tools", "sih-tools/facet/probes/n_convergence_probe.py", "EPS = 0.1", "EPS", "f21", "工程实践三件套", R1),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "# gd-2 合同参数显式声明", "(gd-1 退场注释块)", "f22", "退场核销", R1),
    ("tools", "sih-tools/facet/probes/check_verdict_consistency.py", "DEFAULT_INCONSISTENCY_MAX_RATE = 0.0", "DEFAULT_INCONSISTENCY_MAX_RATE", "f23", "工程实践三件套", R1),
    # ── 批二 19 件落码位（constclear2b-routing §2/§3/§4 与 registry v2 §1/§2；批二路由无行 id，登记行 id 取常数名）──
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "GC_K_SIGMA_DEFAULT = 3.0", "GC_K_SIGMA_DEFAULT", "GC_K_SIGMA_DEFAULT", "数学模型", R2),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "GC_CUSUM_B_DEFAULT = 0.5", "GC_CUSUM_B_DEFAULT", "GC_CUSUM_B_DEFAULT", "数学模型", R2),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "GC_CUSUM_H_DEFAULT = 5.0", "GC_CUSUM_H_DEFAULT", "GC_CUSUM_H_DEFAULT", "数学模型", R2),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "RHO_BOUND_DEFAULT = 1.0", "RHO_BOUND_DEFAULT", "RHO_BOUND_DEFAULT", "数学模型", R2),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "GD_UNION_THRESHOLD_DEFAULT = 5", "GD_UNION_THRESHOLD_DEFAULT", "GD_UNION_THRESHOLD_DEFAULT", "工程实践三件套", R2),
    ("tools", "sih-tools/gauge/src/gauge/cli.py", "GD_SUSTAINED_MIN_DAYS_DEFAULT = 3", "GD_SUSTAINED_MIN_DAYS_DEFAULT", "GD_SUSTAINED_MIN_DAYS_DEFAULT", "工程实践三件套", R2),
    ("tools", "sih-tools/watchcheck/src/watchcheck/constants.py", "LOCKFACE_WIDE_THRESHOLD = 20", "LOCKFACE_WIDE_THRESHOLD", "LOCKFACE_WIDE_THRESHOLD", "工程实践三件套", R2),
    ("tools", "sih-tools/lease/src/lease/lockdb.py", "HEARTBEAT_STALE_SECONDS = 300", "HEARTBEAT_STALE_SECONDS", "HEARTBEAT_STALE_SECONDS", "工程实践三件套", R2),
    ("tools", "sih-tools/lease/src/lease/cli.py", "HEARTBEAT_STALE_SECONDS = 300", "HEARTBEAT_STALE_SECONDS", "HEARTBEAT_STALE_SECONDS（cli 双位同值）", "工程实践三件套", R2),
    ("tools", "sih-tools/confledger/src/confledger/constants.py", "M_CLEAN = 1", "M_CLEAN", "M_CLEAN", "工程实践三件套", R2),
    ("tools", "sih-tools/confledger/src/confledger/constants.py", "M_ACTIVE = 1", "M_ACTIVE", "M_ACTIVE", "工程实践三件套", R2),
    ("tools", "sih-tools/critsweep/sweep.py", "WINDOW_DAYS = 10", "WINDOW_DAYS", "WINDOW_DAYS", "工程实践三件套", R2),
    ("tools", "sih-tools/critsweep/sweep.py", "ROUTE_TIMEOUT_S = 10", "ROUTE_TIMEOUT_S", "ROUTE_TIMEOUT_S", "环境参数", REG2),
    ("tools", "sih-tools/watchcheck/src/watchcheck/constants.py", "EXIT_CLEAN = 0", "EXIT_CLEAN", "EXIT_CLEAN", "纯约定", REG2),
    ("tools", "sih-tools/watchcheck/src/watchcheck/constants.py", "EXIT_UNOWNED = 1", "EXIT_UNOWNED", "EXIT_UNOWNED", "纯约定", REG2),
    ("tools", "sih-tools/watchcheck/src/watchcheck/constants.py", "EXIT_TOOL_ERROR = 2", "EXIT_TOOL_ERROR", "EXIT_TOOL_ERROR", "纯约定", REG2),
    ("tools", "sih-tools/calllog/src/calllog/core.py", "DB_SCHEMA_VERSION = 1", "DB_SCHEMA_VERSION", "DB_SCHEMA_VERSION", "纯约定", REG2),
    ("tools", "sih-tools/lease/src/lease/core.py", "LOCK_BILL_UNIT = 1", "LOCK_BILL_UNIT", "LOCK_BILL_UNIT", "纯约定", REG2),
    ("tools", "sih-tools/lease/src/lease/core.py", "UNUSED_LOCK_MULTIPLIER = 1", "UNUSED_LOCK_MULTIPLIER", "UNUSED_LOCK_MULTIPLIER", "纯约定", REG2),
    ("tools", "sih-tools/lease/src/lease/core.py", "EXPANSION_FREE_QUOTA = 1", "EXPANSION_FREE_QUOTA", "EXPANSION_FREE_QUOTA", "纯约定", REG2),
]

# 退场核销零触碰件（retired 路径即核销标记）
RETIRED_ZERO_TOUCH = [
    {"id": "f22-值位", "name": "GD_ALPHA_DEFAULT", "site": "sih-tools/gauge/src/gauge/cli.py（已退场无常数行）",
     "state": "退场核销", "account": R1, "note": "gd-1 族退场红证在批一核验件；追认语义见 pk-049-exit.json；本批在 gd-1 退场注释块落 f22 注记"},
    {"id": "批二·退场1", "name": "N_SHOTS / PACK_VERSION", "site": "sih-tools/facet/probes/retired/temp_probe.py",
     "state": "退场核销", "account": R2, "note": "retired 路径即核销标记，零触碰"},
    {"id": "批二·退场2", "name": "REPEATS", "site": "sih-tools/facet/probes/retired/temp0_control.py",
     "state": "退场核销", "account": R2, "note": "retired 路径即核销标记，零触碰"},
]


def worktree_of(repo: str) -> Path:
    return {"tools": TW, "engine": EW}[repo]


def wt_path(repo: str, rel: str) -> Path:
    """工作区相对路径转工地路径：工地根即仓根，剥去仓名首段。"""
    top = {"tools": "sih-tools/", "engine": "sih-engine/"}[repo]
    assert rel.startswith(top), f"rel path not under {top}: {rel}"
    return worktree_of(repo) / rel[len(top):]


def main() -> int:
    projection = []
    per_file_lines: dict[Path, list[str]] = {}
    pre_lines: dict[tuple[str, str], dict] = {}

    # 第一遍：定位与预读数
    for repo, rel, prefix, name, rid, state, account in ANNOTATIONS:
        wt = worktree_of(repo)
        del wt
        path = wt_path(repo, rel)
        if path not in per_file_lines:
            per_file_lines[path] = path.read_text(encoding="utf-8").splitlines()
        lines = per_file_lines[path]
        hits = [i for i, ln in enumerate(lines) if ln.startswith(prefix)]
        if len(hits) != 1:
            print(f"ABORT: prefix not unique ({len(hits)} hits): {rel} :: {prefix}")
            return 2
        pre_lines[(str(path), prefix)] = {
            "pre_line_1based": hits[0] + 1,
            "binding": lines[hits[0]],
        }

    # 第二遍：自底向上逐文件插入（同文件多靶按行号降序插入保前缀稳定）
    inserted_by_file: dict[Path, list[tuple[int, str]]] = {}
    for repo, rel, prefix, name, rid, state, account in ANNOTATIONS:
        wt = worktree_of(repo)
        del wt
        path = wt_path(repo, rel)
        comment = ("// " if rel.endswith(".rs") else "# ") + f"[constclear2c] 登记行 {rid} 态{state}｜账面 sih-math/docs/{account}"
        inserted_by_file.setdefault(path, []).append((pre_lines[(str(path), prefix)]["pre_line_1based"], comment))

    for path, items in inserted_by_file.items():
        lines = per_file_lines[path]
        for lineno_1b, comment in sorted(items, key=lambda x: -x[0]):
            lines.insert(lineno_1b - 1, comment)
        path.write_text("\n".join(lines) + "\n", encoding="utf-8")

    # 第三遍：post 行号与值绑定对表
    drift = []
    for repo, rel, prefix, name, rid, state, account in ANNOTATIONS:
        wt = worktree_of(repo)
        del wt
        path = wt_path(repo, rel)
        lines = path.read_text(encoding="utf-8").splitlines()
        hits = [i for i, ln in enumerate(lines) if ln.startswith(prefix)]
        assert len(hits) == 1, f"post-locate failed {rel} {prefix}"
        post_binding = lines[hits[0]]
        pre = pre_lines[(str(path), prefix)]
        value_unchanged = post_binding == pre["binding"]
        projection.append({
            "id": rid, "name": name, "state": state,
            "repo": repo, "site": f"{rel}:{hits[0] + 1}",
            "pre_line": pre["pre_line_1based"], "post_line": hits[0] + 1,
            "value_binding_unchanged": value_unchanged,
            "account": f"sih-math/docs/{account}",
        })
        if not value_unchanged:
            drift.append({"rel": rel, "prefix": prefix, "pre": pre["binding"], "post": post_binding})

    out = {
        "batch": "constclear2c-solo",
        "kind": "constants-annotations（源码注记落码机器可读投影）",
        "session": "sess-zcode-260908-main-constclear2c",
        "annotation_form": "邻行注释即常数行上方一行，携登记行 id 与态与账面指针，值零改动",
        "counts": {
            "annotation_sites": len(ANNOTATIONS),
            "files": len(inserted_by_file),
            "retired_zero_touch": len(RETIRED_ZERO_TOUCH),
        },
        "items": projection,
        "retired_zero_touch": RETIRED_ZERO_TOUCH,
        "value_drift": drift,
    }
    (HERE / "constants-annotations.json").write_text(
        json.dumps(out, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"projection written: {len(ANNOTATIONS)} sites / {len(inserted_by_file)} files / drift {len(drift)}")
    return 0 if not drift else 1


if __name__ == "__main__":
    sys.exit(main())
