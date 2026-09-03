# selwire-solo 金向量机械重放：selector 谓词划分两场景双跑。
# 场景一 same_feature_same_route：同特征材料（谓词过败全同）归同路即同类（推导档 §3.1/§3.2）。
# 场景二 boundary_predicate_localization：边界材料由首败谓词机械定位分界（推导档 §3.3）。
# 用法：python3 replay_golden.py golden_cases.json selwire-solo-golden-vector.json \
#           --selector-src <sih-tools 工地>/selector/src
# 判据：同参双跑逐字节一致（承 SPEC-015 双跑同参形条款），且与冻结金向量一致。
import argparse
import hashlib
import json
import sys
import tempfile
from pathlib import Path


def build_payload(cases_path, selector_src):
    sys.path.insert(0, str(selector_src))
    from selector.pack import load_pack
    from selector.route import route_batch

    cases = json.loads(Path(cases_path).read_text(encoding="utf-8"))
    with tempfile.TemporaryDirectory() as tmp:
        pack_dir = Path(tmp) / "pack"
        pack_dir.mkdir()
        (pack_dir / "manifest.toml").write_text(cases["pack"]["manifest"], encoding="utf-8")
        (pack_dir / "routes.toml").write_text(cases["pack"]["routes"], encoding="utf-8")
        pack = load_pack(pack_dir)
        materials = [m["record"] for m in cases["materials"]]
        result = route_batch(materials, pack)
    routed = {e["id"]: e for e in result["routed"]}
    checks_pass = [
        c["id"] for c in routed["mat-a"]["checks"] if c["pass"]
    ]
    boundary = routed["mat-c"]
    boundary_check = next(c for c in boundary["checks"] if c["id"] == "g_anchor")
    payload = {
        "carrier": "ALG-002",
        "formula_version": "sw-1",
        "routed": result["routed"],
        "summary": result["summary"],
        "scenario_1_same_feature_same_route": {
            "route_a": routed["mat-a"]["route"],
            "route_b": routed["mat-b"]["route"],
            "same_route": routed["mat-a"]["route"] == routed["mat-b"]["route"],
            "expect_route": cases["expectations"]["same_feature_same_route"]["route"],
            "failed_predicate": routed["mat-a"]["failed_predicate"],
            "checks_pass": checks_pass,
        },
        "scenario_2_boundary_predicate_localization": {
            "route": boundary["route"],
            "expect_route": cases["expectations"]["boundary_predicate_localization"]["route"],
            "failed_predicate": boundary["failed_predicate"],
            "expect_failed_predicate": cases["expectations"]["boundary_predicate_localization"]["failed_predicate"],
            "boundary_check_id": boundary_check["id"],
            "boundary_check_pass": boundary_check["pass"],
        },
    }
    return payload


def main():
    parser = argparse.ArgumentParser(description="selwire-solo golden replay")
    parser.add_argument("cases")
    parser.add_argument("frozen")
    parser.add_argument("--selector-src", required=True)
    args = parser.parse_args()
    payload = build_payload(args.cases, args.selector_src)
    frozen = json.loads(Path(args.frozen).read_text(encoding="utf-8"))
    computed_text = json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2)
    frozen_text = json.dumps(frozen["payload"], ensure_ascii=False, sort_keys=True, indent=2)
    verdict = "IDENTICAL" if computed_text == frozen_text else "DIVERGENT"
    scenarios_ok = (
        payload["scenario_1_same_feature_same_route"]["same_route"] is True
        and payload["scenario_1_same_feature_same_route"]["route_a"]
        == payload["scenario_1_same_feature_same_route"]["expect_route"]
        and payload["scenario_2_boundary_predicate_localization"]["route"]
        == payload["scenario_2_boundary_predicate_localization"]["expect_route"]
        and payload["scenario_2_boundary_predicate_localization"]["failed_predicate"]
        == payload["scenario_2_boundary_predicate_localization"]["expect_failed_predicate"]
        and payload["scenario_2_boundary_predicate_localization"]["boundary_check_pass"] is False
    )
    print(
        json.dumps(
            {
                "verdict_vs_frozen": verdict,
                "scenarios_ok": scenarios_ok,
                "payload_sha256": hashlib.sha256(computed_text.encode("utf-8")).hexdigest(),
                "payload": payload,
            },
            ensure_ascii=False,
            sort_keys=True,
            indent=2,
        )
    )
    return 0 if verdict == "IDENTICAL" and scenarios_ok else 1


if __name__ == "__main__":
    sys.exit(main())
