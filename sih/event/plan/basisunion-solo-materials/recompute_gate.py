#!/usr/bin/env python3
"""basisunion-solo T-3: watchcheck round1 gate verdict 只读复算（零 LLM 零写面）。

对象: gid m-watchcheck-judge-1（watchcheck-solo 批 facet round1，九发 3/3/3
依据族分散全落 baseline_1/4/5 合法集，旧判据 distinct_basis<=1 硬共识挂
basis_consensus 落 boundary 候裁）。

语义: 以 basisunion-solo T-1 新判据（集合成员语义，合法枚举集从合同各发
system_prompt 声明提取单源）复算 gate verdict，与 score 半同参（assess_maturation_v3
缺省参 + basis_allowed=declared_basis_enum(contract)）。

纪律: 原始 responses 与合同逐字节零改动零重采样（前后哈希对表在输出），
trail 只读消费（load_dc_list 不写），verdict 翻转与否俱如实输出，不翻转
即停批申报不回滚判据迁就。
"""
from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

WT = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-tools/basisunion-solo")
CONTRACT = WT / "facet" / "contracts" / "watchcheck-260905" / "contract.json"
RESPONSES_COPIES = [
    Path("/Users/moc/workspaces/SiHankor/sih-tools/facet/contracts/"
         "watchcheck-260905/responses.jsonl"),
    Path("/Users/moc/workspaces/SiHankor/sih-engine/sih/event/plan/"
         "watchcheck-solo-materials/facet-round1/responses.jsonl"),
]
CONTRACT_COPY = Path("/Users/moc/workspaces/SiHankor/sih-engine/sih/event/plan/"
                     "watchcheck-solo-materials/facet-round1/contract.json")
GID = "m-watchcheck-judge-1"

# 工地 facet 根须列 sys.path 首位（site-packages .pth 指主树 facet，不显式
# 前插即静默解析主树旧码——本批实测坑位，如实注记）
sys.path.insert(0, str(WT / "facet"))
sys.path.insert(1, str(WT / "facet" / "probes"))
sys.path.insert(2, str(WT / "facet" / "src"))

import contract_mode  # noqa: E402
import flywheel_trail as ft  # noqa: E402
from maturation_gate import assess_maturation_v3  # noqa: E402


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main() -> int:
    before = {str(p): sha(p) for p in RESPONSES_COPIES + [CONTRACT_COPY]}
    contract = contract_mode.load_contract(CONTRACT)
    allowed = contract_mode.declared_basis_enum(contract)
    dc_list = ft.load_dc_list(GID)
    old = assess_maturation_v3(dc_list)
    new = assess_maturation_v3(dc_list, basis_allowed=allowed)
    after = {str(p): sha(p) for p in RESPONSES_COPIES + [CONTRACT_COPY]}
    out = {
        "gid": GID,
        "recompute_batch": "basisunion-solo",
        "recompute_date": "2026-09-06",
        "verdict_before": old["verdict_v3"],
        "verdict_after": new["verdict_v3"],
        "criteria_after": new["criteria"],
        "near_flags_after": new["near_flags"],
        "basis_criterion": new["basis_criterion"],
        "basis_union_version": new["basis_union_version"],
        "basis_failure": new["basis_failure"],
        "distinct_basis": new["metrics"]["distinct_basis"],
        "basis_allowed": new["metrics"]["basis_allowed"],
        "basis_missing": new["metrics"]["basis_missing"],
        "basis_out_of_set": new["metrics"]["basis_out_of_set"],
        "boundary_test_p": new["metrics"].get("boundary_test_p"),
        "boundary_flag_sum": new["metrics"]["boundary_flag_sum"],
        "n_runs": len(dc_list),
        "responses_zero_modification": before == after,
        "hashes": after,
        "recompute_defaults":
            "assess_maturation_v3 缺省参 + basis_allowed=declared_basis_enum"
            "(contract)，与 score 半同参（T-1 后产线形）",
        "ruling": "用户 2026-09-06 裁定原话「watchcheck 改题文本不需要改，只要改"
                  "判定，依据只要是基线，无论是哪个，都是同一个语义的判决。」",
    }
    print(json.dumps(out, ensure_ascii=False, indent=1))
    return 0 if before == after else 2


if __name__ == "__main__":
    raise SystemExit(main())
