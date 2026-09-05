#!/usr/bin/env python3
"""gateswitch-solo 段二：pk-048 冻结清单 114 件确定性复核。

纯确定性零 LLM。四步：
1. 载荷哈希闸：冻结件 sha256 前十六位须为 ca9dda80cfe8bbe8（pk-048 登记）。
2. 全量复算：调在档判变审计探针（facetmath_dual_audit.py，只读零 trail 写入）
   对现行在档材料双判据全量复算，与冻结清单逐件逐字段对表；冻结行必须是
   复算行的子集且零字段漂移。
3. 独立复算哨：每行 boundary_test_p 用本脚本内独立实现（math.comb 精确上尾和）
   复算，与行载值对表；占比判据规则（k/n < 0.34）独立复算；verdict_change
   标志与方向（由宽到紧/由严到宽）独立复核——任何由严到宽即 stop_flag。
4. 逐件账本：判变行逐件给判由与证据指针，落复核账本。

输出零时间戳零随机，同输入逐字节一致（双跑判据）。
用法：python3 recheck_114.py <输出.json>
"""
from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
FROZEN = ROOT / "sih-engine/sih/event/plan/facetmath-solo-materials/facetmath-golden-dual-audit.json"
FROZEN_SHA16 = "ca9dda80cfe8bbe8"
DUAL_AUDIT = ROOT / "sih-tools/facet/probes/facetmath_dual_audit.py"
DES_ROOT = ROOT / "sih-tools/proposition/DES"
PKGS_ROOT = ROOT / "sih-tools/facet/facet_task_packages"
ALPHA = 0.05
P_H0 = 0.004
RATIO_THRESHOLD = 0.34

TIGHTENING_FROM = {"stable_clear", "near_threshold"}


def exact_upper_tail_p(k: int, n: int, p0: float) -> float:
    """上尾精确二项 p 值独立实现：P(X >= k), X ~ Binomial(n, p0)。"""
    if n == 0:
        return 1.0
    if k <= 0:
        return 1.0
    from math import comb
    total = sum(comb(n, j) * (p0 ** j) * ((1.0 - p0) ** (n - j))
                for j in range(k, n + 1))
    return min(1.0, max(0.0, total))


def ratio_boundary_low(k: int, n: int) -> bool:
    """占比判据独立复算：V3-T 调整后率 < 0.34（单席同质载荷即 k/n）。"""
    if n == 0:
        return True
    return (k / n) < RATIO_THRESHOLD


def main() -> None:
    out_path = Path(sys.argv[1])

    # 1. 载荷哈希闸
    payload = FROZEN.read_bytes()
    sha = hashlib.sha256(payload).hexdigest()
    hash_gate = sha.startswith(FROZEN_SHA16)
    frozen = json.loads(payload)

    # 2. 全量复算（在档只读探针，子进程显式双根）
    proc = subprocess.run(
        [sys.executable, str(DUAL_AUDIT),
         "--des-root", str(DES_ROOT), "--pkgs-root", str(PKGS_ROOT),
         "--out", str(out_path.parent / (out_path.stem + "-recompute.json"))],
        capture_output=True, text=True, check=True)
    recompute = json.loads((out_path.parent / (out_path.stem + "-recompute.json")).read_text())

    rmap = {r["id"]: r for r in recompute["rows"]}
    frozen_rows = frozen["rows"]
    missing = [r["id"] for r in frozen_rows if r["id"] not in rmap]
    field_drift = []
    for r in frozen_rows:
        r2 = rmap.get(r["id"])
        if r2 is None:
            continue
        for key, val in r.items():
            if r2.get(key) != val:
                field_drift.append({"id": r["id"], "field": key,
                                    "frozen": val, "recomputed": r2.get(key)})

    # 3. 独立复算哨 + 方向复核（对冻结判变行逐件，另对复算新增判变行全查方向）
    sentinel_rows = []
    loosening = []
    for idx, r in enumerate(frozen_rows):
        if not r.get("verdict_change"):
            continue
        k, n = r["boundary_k"], r["boundary_n"]
        p_ind = exact_upper_tail_p(k, n, P_H0)
        p_ok = abs(p_ind - r["boundary_test_p"]) <= 1e-9
        ratio_ok = ratio_boundary_low(k, n)
        direction = ("tightening"
                     if r["verdict_ratio"] in TIGHTENING_FROM and r["verdict_test"] == "boundary"
                     else "loosening")
        if direction == "loosening":
            loosening.append(r["id"])
        sentinel_rows.append({
            "row_index": idx, "id": r["id"], "source": r["source"],
            "from": r["verdict_ratio"], "to": r["verdict_test"],
            "boundary_k": k, "boundary_n": n,
            "frozen_test_p": r["boundary_test_p"],
            "independent_test_p": p_ind, "test_p_match": p_ok,
            "independent_ratio_boundary_low": ratio_ok,
            "direction": direction,
            "判由": (
                "检验判据在役：k={k}/n={n} 上尾精确二项 p={p:.6g} <= α=0.05 判挂 "
                "boundary_low，占比判据 k/n={rate:.4f} < 0.34 判过——同一证据双判据 "
                "分叉，切换使该件由 {frm} 收紧为 boundary，判定基准由经验阈值变为 "
                "显式错误率上界的可复算检验".format(
                    k=k, n=n, p=r["boundary_test_p"], rate=(k / n if n else 0.0),
                    frm=r["verdict_ratio"])
                if direction == "tightening" else "由严到宽——违规漂移，须停批"),
            "证据指针": (
                "冻结载荷 sha256 前十六位 " + FROZEN_SHA16
                + "；row_index=" + str(idx)
                + "；源材料 " + ("sih-tools/facet/facet_task_packages/" + r["id"] + "/"
                                  if r["source"] == "facet_task_packages"
                                  else "sih-tools/proposition/DES/" + r["id"] + "/")),
        })

    # 复算新增判变行（冻结后新入档件）方向全查
    fids = {r["id"] for r in frozen_rows}
    new_changes = []
    for r in recompute["rows"]:
        if r["id"] in fids or not r.get("verdict_change"):
            continue
        k, n = r["boundary_k"], r["boundary_n"]
        direction = ("tightening"
                     if r["verdict_ratio"] in TIGHTENING_FROM and r["verdict_test"] == "boundary"
                     else "loosening")
        if direction == "loosening":
            loosening.append(r["id"])
        new_changes.append({"id": r["id"], "from": r["verdict_ratio"],
                            "to": r["verdict_test"], "boundary_k": k,
                            "boundary_n": n, "direction": direction})

    n_frozen_changes = len(sentinel_rows)
    stop_flag = bool(loosening or missing or field_drift or not hash_gate)

    out = {
        "probe": "gateswitch_recheck_114",
        "payload_sha256": sha,
        "hash_gate_pass": hash_gate,
        "frozen_rows": len(frozen_rows),
        "frozen_verdict_changes": n_frozen_changes,
        "recompute_rows": len(recompute["rows"]),
        "recompute_comparable": recompute["summary"]["comparable"],
        "recompute_archived_mismatch": recompute["summary"]["archived_mismatch_count"],
        "frozen_missing_in_recompute": missing,
        "frozen_field_drift": field_drift,
        "change_breakdown": {
            "near_threshold_to_boundary": sum(
                1 for s in sentinel_rows if s["from"] == "near_threshold"),
            "stable_clear_to_boundary": sum(
                1 for s in sentinel_rows if s["from"] == "stable_clear"),
        },
        "independent_p_all_match": all(s["test_p_match"] for s in sentinel_rows),
        "independent_ratio_all_pass": all(s["independent_ratio_boundary_low"] for s in sentinel_rows),
        "post_freeze_new_changes": new_changes,
        "loosening_ids": loosening,
        "stop_flag": stop_flag,
        "stop_reason": (
            None if not stop_flag else
            "由严到宽漂移或对表失败——停批上报" if loosening else
            "载荷哈希不符或冻结行缺失/漂移——停批上报"),
        "ledger": sentinel_rows,
    }
    out_path.write_text(json.dumps(out, ensure_ascii=False, indent=1, sort_keys=True) + "\n",
                        encoding="utf-8")
    print(json.dumps({k: out[k] for k in (
        "hash_gate_pass", "frozen_rows", "frozen_verdict_changes",
        "recompute_rows", "recompute_archived_mismatch", "frozen_field_drift_len",
        "independent_p_all_match", "loosening_ids", "stop_flag") if k != "frozen_field_drift_len"}
        | {"frozen_field_drift": len(field_drift)}, ensure_ascii=False))


if __name__ == "__main__":
    main()
