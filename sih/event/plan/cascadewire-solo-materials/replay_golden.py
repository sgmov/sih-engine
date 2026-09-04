#!/usr/bin/env python3
"""cascadewire-solo 金向量重放器：真实调用 cascade core 判定函数机械复算四场景。

重放寻径约定（冻结态携带，零绝对路径）：
  --cascade-src <cascade 源根，即含 cascade 包的 src 目录> 参数传入工具源路径；
  root 夹具由本重放器于 tempfile 临时目录自建，cases 零绝对路径；
  payload 只含场景名与三态读数与 verdict 与孤儿报告与边账计数，零运行时路径。

用法：
  python3 replay_golden.py golden_cases.json --cascade-src <src 目录>
"""
import argparse
import hashlib
import json
import sys
import tempfile
from pathlib import Path


def sha256_text(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def load_core(cascade_src):
    sys.path.insert(0, str(cascade_src))
    from cascade import core

    return core


def replay_case(core, case, tmp_root):
    """真实调用 cascade.core：建夹具、装配 registry、跑 check_targets 与 find_orphans 与 runtime_ledger。"""
    root = Path(tmp_root)
    for rel, text in case["fixture_files"].items():
        target = root / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8")
    # cert 哈希按参照内容换算成 file_hash 同形键（纯 hexdigest），对表 check_targets
    cert_hashes = {}
    for rel, ref_text in case["cert_refs"].items():
        disk = (root / rel).resolve()
        cert_hashes[str(disk)] = sha256_text(ref_text)
    registry = {"edges": case["edges"]}
    targets = core.check_targets(registry, root, cert_hashes)
    states = {}
    for t in targets.values():
        states.update(t["upstreams"])
    verdicts = {rel: t["verdict"] for rel, t in targets.items()}
    blocked = sum(1 for t in targets.values() if t["verdict"] == "blocked")
    # 孤儿侦查：registry edges 中指向缺席件的上游（find_orphans 对表形）
    orphans = [
        o["edge"] for o in core.find_orphans(registry, root)
    ]
    ledger = core.runtime_ledger(targets)
    exit_code = 1 if blocked else 0
    return {
        "case": case["name"],
        "states": {k: states[k] for k in sorted(states)},
        "verdicts": verdicts,
        "orphans": orphans,
        "ledger": ledger,
        "exit": exit_code,
        "expect": case["expect"],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("cases", help="golden_cases.json 路径")
    parser.add_argument("--cascade-src", required=True, help="cascade 源根（含 cascade 包的 src 目录）")
    args = parser.parse_args()
    core = load_core(args.cascade_src)
    cases = json.loads(Path(args.cases).read_text(encoding="utf-8"))["cases"]
    with tempfile.TemporaryDirectory() as tmp:
        results = [replay_case(core, case, Path(tmp) / f"fixture-{i}") for i, case in enumerate(cases)]
    # 期望对表：逐场景核 expect，一票不符即判 FAIL
    mismatch = []
    for r in results:
        exp = r["expect"]
        if r["states"] != exp.get("states", r["states"]):
            mismatch.append(r["case"] + ":states")
        if r["verdicts"] != exp.get("verdicts", r["verdicts"]):
            mismatch.append(r["case"] + ":verdicts")
        if "orphans" in exp and r["orphans"] != exp["orphans"]:
            mismatch.append(r["case"] + ":orphans")
        if "ledger" in exp and r["ledger"] != exp["ledger"]:
            mismatch.append(r["case"] + ":ledger")
        if r["exit"] != exp["exit"]:
            mismatch.append(r["case"] + ":exit")
    payload = {
        "formula_version": "cw-1",
        "carrier": "ORD-016",
        "carrier_batch": "cascadewire-solo",
        "expect_match": not mismatch,
        "mismatch": mismatch,
        "cases": results,
    }
    print(json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2))
    return 0 if not mismatch else 1


if __name__ == "__main__":
    sys.exit(main())
