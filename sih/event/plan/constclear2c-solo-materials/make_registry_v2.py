#!/usr/bin/env python3
"""constclear2c-solo 腿三生成器：constclear-registry v1 → v2 换版（md+json 同源）。

换版三面：
- 行 id 稳定不重编：env_params 四行与 pure_conventions 三行与 debts 三行逐字保留 v1 语义；
- 位列更新为批三注记落后行号（投影件 constants-annotations.json 机械读数）；
- 新增 §4 证据指针（C4 判据对齐）：处置三类与账面链证逐类落指针。
判定依据：A1 双通道（4073c5fa）、A2 类目不豁免取值（15f9a0ad）、A3 批界（1f03de72）、两态口径（41fb339c）。
"""
import json
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
MAT = ROOT / "worktrees/sih-engine/constclear2c-solo/sih/event/plan/constclear2c-solo-materials"
OUT_MD = ROOT / "worktrees/sih-math/constclear2c-solo/docs/constclear-registry-v2.md"
OUT_JSON = ROOT / "worktrees/sih-math/constclear2c-solo/docs/constclear-registry-v2.json"

proj = json.loads((MAT / "constants-annotations.json").read_text(encoding="utf-8"))
post = {it["name"] + "@" + it["site"].rsplit(":", 1)[0]: it["post_line"] for it in proj["items"]}


def site_of(name: str, path: str, pre: str) -> str:
    key = name + "@" + path
    line = post.get(key)
    return f"{path}:{line}" if line else pre


env_rows = [
    {"name": "MAX_ROUND", "site": site_of("MAX_ROUND", "sih-tools/facet/probes/eir_ecr_gate_probe.py", ":78"), "value": "3",
     "id": "r1（批一路由 §4，批二收编）",
     "two_state_basis": "工程实践态（承 A2 类目不豁免取值）：判定边界由闸判据承载不由轮数承载；证据即 eir_ecr_gate_probe 在役；回归义务：采样轮数-判据强度关系统计化时重估（PROB 族）"},
    {"name": "MAX_SHOTS", "site": site_of("MAX_SHOTS", "sih-tools/facet/probes/lightweight_mode_probe.py", ":146"), "value": "7",
     "id": "r2（批一路由 §4，批二收编）",
     "two_state_basis": "工程实践态：停采判定由 FLIP_STOP 众数规则承载，值只封采样预算；证据即 lightweight 档在役；回归义务：发射预算-置信度关系随置信度线重估（pk-073 系）"},
    {"name": "MAX_ROUND", "site": site_of("MAX_ROUND", "sih-tools/facet/probes/multiround_convergence_probe.py", ":108"), "value": "5",
     "id": "r3（批一路由 §4，批二收编）",
     "two_state_basis": "工程实践态：轨迹判定规则不由轮数承载（观测点 1/2/3/5 注记在源）；证据即 multiround 在役；回归义务：观测轮数-收敛评估关系统计化时重估"},
    {"name": "ROUTE_TIMEOUT_S", "site": site_of("ROUTE_TIMEOUT_S", "sih-tools/critsweep/sweep.py", ":24"), "value": "10",
     "id": "批二新账（环境参数）",
     "two_state_basis": "资源性（扫描器 native 判）：超时只降级不改变路由判定语义；证据即 critsweep 降级可见条款在档；回归义务：无（纯资源上限，维持申报）"},
]

pure_rows = [
    {"name": "EXIT_CLEAN / EXIT_TOOL_ERROR / EXIT_UNOWNED",
     "site": site_of("EXIT_CLEAN", "sih-tools/watchcheck/src/watchcheck/constants.py", "") + " 起",
     "id": "批二新账（纯约定）",
     "basis": "退出码枚举（0/1/2 三值语义约定），非判定数值；BATCH-FACE watch 节与 CONTRACT 判定式在档"},
    {"name": "DB_SCHEMA_VERSION", "site": site_of("DB_SCHEMA_VERSION", "sih-tools/calllog/src/calllog/core.py", ":30"),
     "id": "批二新账（纯约定）",
     "basis": "schema 版本标记（结构约定），派生可重建面版本位"},
    {"name": "LOCK_BILL_UNIT / EXPANSION_FREE_QUOTA / UNUSED_LOCK_MULTIPLIER",
     "site": site_of("LOCK_BILL_UNIT", "sih-tools/lease/src/lease/core.py", ":40") + " 起",
     "id": "批二新账（纯约定）",
     "basis": "锁面经济记账约定（先记账后算分，2026-09-06 用户裁定域）；billwire 与 leaseup 在役，账单台账 lockface-bills 双写一致判据在档；定价模型数学化评估义务随批三呈报"},
]

debts = [
    "gd-1 旧名族退场照录：CUSUM_B_DEFAULT/CUSUM_H_DEFAULT/K_SIGMA_DEFAULT/GD_BASELINE_N_DEFAULT/GD_P0_DEFAULT/GD_ALPHA_DEFAULT 六件自账面移除（GC_* 前缀继位与 gd-2 合同参数继位）",
    "rev3 再生与在档版 drift：lit_new 93 与 lit_gone 8（源码 09-04 后演进），exit 1 即 drift 旗非错误，双跑 IDENTICAL 确定性成立",
    "R2 债清偿：RECLASS_INHERIT 扩表三件，env-params 三行 overlay 转 native",
    "v2 换版 2026-09-08：位列自 v1 起全部更新为批三注记落后行号（行号位移即注记插入所致值零改动），行 id 稳定不重编；新增 §4 证据指针对齐 C4 判据文本",
]

c4_evidence = {
    "criterion_text_ref": "GOV-002-mainline-lock-v1.md 退出标准第四条（数学归因长尾收口即 pk-053 冻结件逐件清账完毕，处置三态即承载体落档或改判环境参数或维持冻结经用户裁，账面在链）",
    "carriers_documented": {
        "scope": "承载体落档：批一路由 §5 数学模型态 2 件（f11 DEFAULT_BOUNDARY_BASELINE 与 f12 BOUNDARY_RATE_THRESHOLD）与批二路由 §2 数学模型态 4 件（GC_K_SIGMA_DEFAULT 与 GC_CUSUM_B_DEFAULT 与 GC_CUSUM_H_DEFAULT 与 RHO_BOUND_DEFAULT），载体实文列在两路由表在档",
        "chain": "批一认证 414b4e8b、批二路由增补 json 认证 65260cd7（2026-09-08 当日链）",
    },
    "reclassified_env_params": {
        "scope": "改判环境参数：r1 与 r2 与 r3 批二收编 env-params native 三行即本登记面 §1；纯约定三行即 §2（A1 双通道登记面通道，非伪装数学）",
        "chain": "批二登记面 json 认证 c25dc8dd、批二内容清单件 f2bffd25（2026-09-08 当日链）",
    },
    "retired_void": {
        "scope": "退场核销（非归因态，显式核销申报）：f22 GD_ALPHA_DEFAULT 即 gd-1 推断式常数族退场红证在批一核验件；批二 retired 路径三件即 retired/ 路径核销标记",
        "chain": "批一认证 414b4e8b 随批红证、批二内容清单件 f2bffd25（2026-09-08 当日链）",
    },
    "ledger_on_chain": {
        "scope": "账面在链：批一 34 件路由与批二 25 件路由全数落档；批三源码注记 54 落码位与退场核销 3 件零触碰在投影件 constants-annotations.json；pk-049 联动呈报在批三结果档专节",
        "chain": "批一 ask3 认证 f4a89ffa 与批一认证 414b4e8b、批二 ask3 认证 58680ad9 族、pk-053 出泊笔 41fb339c、批三认证实录见 constclear2c-solo-results.md",
    },
    "generational_note": "旧文三态措辞之维持冻结经用户裁一态与 pk-053 出泊两态制令即冻结撤销零残留存在代际差：冻结件全数走承载体落档或改判登记面或退场核销，零维持冻结残留，由 GOV-002 v2.7 达成追记收口如实记档",
}

reg = {
    "batch": "constclear2c-solo",
    "date": "2026-09-08",
    "session": "sess-zcode-260908-main-constclear2c",
    "lease_session": "9b08f4627b93322b",
    "kind": "constclear-registry-v2（登记面换版，A1 承接地）",
    "supersedes": "constclear-registry-v1（constclear2b-solo 落位；行 id 稳定不重编）",
    "dual_channel_standard": "A1 双通道判定标准（席签 4073c5fa）：能数学化的裁决面必须挂数学载体与推导档（承 P1 67353ed9）；纯约定裁决面必须走登记面诚实登记且不得伪装成数学（承 P2 f4b8f93f）。登记面收环境参数类与纯约定类；数学态与工程三件套态见路由表（constclear2-routing 与 constclear2b-routing）不重复登记。",
    "env_params": env_rows,
    "pure_conventions": pure_rows,
    "debts_and_drifts": debts,
    "c4_evidence": c4_evidence,
}

OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
OUT_JSON.write_text(json.dumps(reg, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")

rm = [
    "# constclear 登记面 v2（A1 双通道承接地）", "",
    "> 批：constclear2c-solo（2026-09-08，登记面 v1 至 v2 换版）；本登记面是纯约定与环境参数类的常设诚实登记位，数学态与工程三件套态见路由表族",
    "> 双通道判定标准：" + reg["dual_channel_standard"],
    "> 换版说明：行 id 稳定不重编即 env 四行与纯约定三行与债三行逐字承 v1；位列更新为批三注记落后行号；v1 文件在档不删。", "",
    "## 1 环境参数类 {#env}", "",
    "| 常数 | 位 | 值 | 行 id | 两态依据（A2 类目不豁免取值） |",
    "|---|---|---|---|---|",
]
for it in env_rows:
    rm.append(f"| {it['name']} | {it['site']} | {it['value']} | {it['id']} | {it['two_state_basis']} |")
rm += ["", "## 2 纯约定类 {#pure}", ""]
for it in pure_rows:
    rm.append(f"- {it['name']} @ {it['site']}（行 id {it['id']}）：{it['basis']}")
rm += ["", "## 3 债与漂移照录 {#debts}", ""]
for d in debts:
    rm.append(f"- {d}")
rm += ["", "## 4 证据指针（C4 判据对齐）{#c4-evidence}", "",
       "- 判据文本：" + c4_evidence["criterion_text_ref"],
       "- 承载体落档：" + c4_evidence["carriers_documented"]["scope"] + "；链证 " + c4_evidence["carriers_documented"]["chain"],
       "- 改判环境参数：" + c4_evidence["reclassified_env_params"]["scope"] + "；链证 " + c4_evidence["reclassified_env_params"]["chain"],
       "- 退场核销：" + c4_evidence["retired_void"]["scope"] + "；链证 " + c4_evidence["retired_void"]["chain"],
       "- 账面在链：" + c4_evidence["ledger_on_chain"]["scope"] + "；链证 " + c4_evidence["ledger_on_chain"]["chain"],
       "- 代际差申报：" + c4_evidence["generational_note"], ""]
OUT_MD.write_text("\n".join(rm) + "\n", encoding="utf-8")
print(f"registry v2 written: {OUT_MD.name} + {OUT_JSON.name}")
print("env rows:", [(r['name'], r['site']) for r in env_rows])
