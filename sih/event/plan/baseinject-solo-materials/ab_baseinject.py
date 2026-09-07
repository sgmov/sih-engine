#!/usr/bin/env python3
"""baseinject-solo A/B 同场对跑夹具（零 LLM 零网络零新鲜采样）。

枚举承 gateswitch-solo recheck 先例（facetmath_dual_audit 同通道）：
facet_task_packages 合同包逐 actor 件 + proposition/DES flywheel trail 逐 gid 件。

A 路 = assess_maturation_v3(dc_list)（冻结表缺省源，在役形）
B 路 = assess_maturation_v3(dc_list, family_temperatures=注入表)
       （注入表由 facet/probes/baseline_inject.py 从标定账本在档读数构建）

四判据（任务包 F-1 至 F-4）：
- F-1 判零变：逐件 verdict_v3 与 near_flags 与 basis 指标全同，100% 才过
- F-2 数值逐位：注入表对冻结表逐键逐位相等（构建器内建 + 本夹具独立复核）
- F-3 漂移告警零：注入 seat-baseline 的 drift_alarm 空且两路输出零告警位
- F-4 哈希绑定不松动：注入 provenance 每家族携 identity_hash，基线件同

输出零时间戳零随机，同输入双跑逐字节一致。
用法：python3 ab_baseinject.py <输出.json>
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

FACET_ROOT = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-tools/baseinject-solo/facet")
_SYS = [FACET_ROOT / "src", FACET_ROOT / "probes"]
for _p in _SYS:
    if str(_p) not in sys.path:
        sys.path.insert(0, str(_p))

import contract_mode  # noqa: E402
import flywheel_trail as ft  # noqa: E402
import baseline_inject  # noqa: E402
from maturation_gate import assess_maturation_v3  # noqa: E402

TOOLROOT = Path("/Users/moc/workspaces/SiHankor/sih-tools")
DES_ROOT = TOOLROOT / "proposition" / "DES"
PKGS_ROOT = FACET_ROOT / "facet_task_packages"
LEDGER = FACET_ROOT / "probes" / "calibration" / "ledger.jsonl"


def iter_dc_items():
    """与 facetmath_dual_audit 同枚举：产出 (source, id, dc_list)。"""
    for pkg_dir in sorted(PKGS_ROOT.iterdir()) if PKGS_ROOT.is_dir() else []:
        if not pkg_dir.is_dir():
            continue
        contract_path = pkg_dir / "contract.json"
        responses_path = pkg_dir / "responses.jsonl"
        if not (contract_path.exists() and responses_path.exists()):
            continue
        try:
            contract = contract_mode.load_contract(contract_path)
            responses = contract_mode.load_responses(responses_path, contract)
            per_actor, _voids = contract_mode.dc_from_responses(
                responses, contract["seat"])
            for i, pa in enumerate(per_actor):
                yield ("facet_task_packages", f"{pkg_dir.name}#{i}",
                       [{"per_actor": [pa]}])
        except Exception as exc:  # noqa: BLE001
            yield ("facet_task_packages", f"{pkg_dir.name}#ERROR",
                   {"error": f"{type(exc).__name__}: {exc}"})
    for gid_dir in sorted(DES_ROOT.iterdir()) if DES_ROOT.is_dir() else []:
        if not gid_dir.is_dir():
            continue
        trail_path = gid_dir / ft.TRAIL_FILENAME
        if not trail_path.exists():
            continue
        try:
            has_runs = False
            with trail_path.open(encoding="utf-8") as f:
                for line in f:
                    if not line.strip():
                        continue
                    rec = json.loads(line)
                    if (rec.get("trail_type") == "flywheel_run"
                            and rec.get("decision_convergence", {}).get("per_actor")):
                        has_runs = True
                        break
            if not has_runs:
                continue
            dc_list = ft.load_dc_list(gid_dir.name, trail_root=DES_ROOT)
            if dc_list:
                yield ("proposition_DES", gid_dir.name, dc_list)
        except Exception as exc:  # noqa: BLE001
            yield ("proposition_DES", f"{gid_dir.name}#ERROR",
                   {"error": f"{type(exc).__name__}: {exc}"})


def _cmp_item(dc_list, injected):
    a = assess_maturation_v3(dc_list)
    b = assess_maturation_v3(dc_list, family_temperatures=injected)
    fields = {
        "verdict_v3": (a.get("verdict_v3"), b.get("verdict_v3")),
        "near_flags": (a.get("near_flags"), b.get("near_flags")),
        "basis_consensus": (a.get("metrics", {}).get("basis_consensus"),
                            b.get("metrics", {}).get("basis_consensus")),
        "boundary_k": (a.get("metrics", {}).get("boundary_k"),
                       b.get("metrics", {}).get("boundary_k")),
        "boundary_n": (a.get("metrics", {}).get("boundary_n"),
                       b.get("metrics", {}).get("boundary_n")),
        "boundary_test_p": (a.get("metrics", {}).get("boundary_test_p"),
                            b.get("metrics", {}).get("boundary_test_p")),
        "adjusted": (a.get("metrics", {}).get("adjusted"),
                     b.get("metrics", {}).get("adjusted")),
    }
    item_identical = all(x == y for x, y in fields.values())
    return item_identical, fields


def main() -> int:
    out_path = Path(sys.argv[1])
    injected = baseline_inject.build_family_temperatures(LEDGER)
    frozen = baseline_inject.frozen_table()
    # F-2 独立复核：冻结键逐位相等
    temps_verbatim = all(injected[k] == v for k, v in frozen.items())
    injected_extra = {k: injected[k] for k in injected if k not in frozen}
    # F-3/F-4：基线件与出处面
    baseline = baseline_inject.build_seat_baseline(LEDGER)
    prov = baseline_inject.provenance(LEDGER)
    drift_alarm_zero = (baseline.get("drift_alarm") == []
                        and baseline.get("parse_fail") == 0)
    identity_bound = (bool(baseline.get("identity_hash"))
                      and all(v.get("identity_hash")
                              for k, v in prov.items()
                              if not k.startswith("_")))

    n_items = 0
    n_identical = 0
    mismatches = []
    errors = []
    for source, ident, dc in iter_dc_items():
        if isinstance(dc, dict) and "error" in dc:
            errors.append({"source": source, "id": ident, "error": dc["error"]})
            continue
        n_items += 1
        try:
            ok, fields = _cmp_item(dc, injected)
        except Exception as exc:  # noqa: BLE001
            errors.append({"source": source, "id": ident,
                           "error": f"{type(exc).__name__}: {exc}"})
            continue
        if ok:
            n_identical += 1
        else:
            mismatches.append({"source": source, "id": ident,
                               "fields": {k: list(v) for k, v in fields.items()
                                          if v[0] != v[1]}})
    doc = {
        "probe": "baseinject_ab_run",
        "n_items": n_items,
        "n_identical": n_identical,
        "identity_rate": (round(n_identical / n_items, 6)
                          if n_items else None),
        "criteria": {
            "F1_verdict_identity_100pct": (n_items > 0
                                           and n_identical == n_items),
            "F2_injected_temps_verbatim": temps_verbatim,
            "F3_drift_alarm_zero": drift_alarm_zero,
            "F4_identity_binding_intact": identity_bound,
        },
        "injected_extra_families": injected_extra,
        "frozen_table_version": baseline_inject.frozen_version(),
        "baseline_date": baseline.get("date"),
        "mismatches": mismatches,
        "enum_errors": errors,
    }
    out_path.write_text(json.dumps(doc, ensure_ascii=False, sort_keys=True,
                                   indent=1) + "\n", encoding="utf-8")
    print(json.dumps(doc["criteria"], ensure_ascii=False))
    print(f"items={n_items} identical={n_identical} "
          f"mismatches={len(mismatches)} errors={len(errors)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
