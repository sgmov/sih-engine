#!/usr/bin/env python3
"""constclear2b-solo 生成器：登记面 constclear-registry v1 与批二路由增补件（md+json 同源）。
判定依据：A1 双通道（4073c5fa）、A2 类目收编不豁免取值（15f9a0ad）、A3 批界（1f03de72）、两态口径（41fb339c）。"""
import json
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
MD = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/constclear2b-solo/docs")
OUT_MD = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/constclear2b-solo/docs/constclear2b-routing-2026-09-08.md")
OUT_JSON = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/constclear2b-solo/docs/constclear2b-routing-2026-09-08.json")
REG_MD = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/constclear2b-solo/docs/constclear-registry-v1.md")
REG_JSON = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/constclear2b-solo/docs/constclear-registry-v1.json")

PROB013_T1 = "PROB-013-stationarity-and-change-point.md L12 定理一（CUSUM 误报控制：阈值 h 下平均运行长度随 h 指数增长，误报率可预标定）"
PROB015_T2 = "PROB-015-littles-law-and-load-bound.md L16 定理二（ρ = λ/μ < 1 稳态必要条件；ρ ≥ 1 时 L 无界增长）"
PROB015_L61 = "PROB-015-littles-law-and-load-bound.md L61 工程注意事项三（窗口须远长于单件时长）"
PROB010_A1 = "PROB-010-hypothesis-testing-and-significance.md L24 公理一（水平先验）"

# ── 新账路由（25 件 = 扫描新增全集）──
NEW_ACCOUNT = {
    "math_model": [
        {"name": "GC_K_SIGMA_DEFAULT", "site": "sih-tools/gauge/src/gauge/cli.py:70", "value": "3.0",
         "carrier": f"{PROB013_T1}；{PROB010_A1}；gchart-derivation-2026-09-04.md L22 与 L44（缺省 3.0 与 α=erfc(k/√2) 随行声明）与 contribmath-derivation L51 实文载值",
         "note": "源码内联声明经条目与推导档实文逐件核验命中"},
        {"name": "GC_CUSUM_B_DEFAULT", "site": "sih-tools/gauge/src/gauge/cli.py:71", "value": "0.5",
         "carrier": f"{PROB013_T1}（容差 b 即累积统计量 S_t = max(0, S_(t-1) + X_t − b) 之 b）；gchart-derivation L36 与 contribmath L51 实文载值", "note": ""},
        {"name": "GC_CUSUM_H_DEFAULT", "site": "sih-tools/gauge/src/gauge/cli.py:72", "value": "5.0",
         "carrier": f"{PROB013_T1}（阈值 h 下 ARL 指数增长）；gchart-derivation L36 与 L46 金向量（S₂=9.58 ≥ 5 报信号）实文载值", "note": ""},
        {"name": "RHO_BOUND_DEFAULT", "site": "sih-tools/gauge/src/gauge/cli.py:75", "value": "1.0",
         "carrier": f"{PROB015_T2}——取值即定理临界值，约束直接实例化；queueing-derivation L34 实文（缺省 1.0 与前兆判据载定理引文）", "note": "本批最强数学态：值由定理临界承载"},
    ],
    "engineering_practice": [
        {"name": "GD_UNION_THRESHOLD_DEFAULT", "site": "sih-tools/gauge/src/gauge/cli.py:79", "value": "5",
         "evidence": "contribmath 实装与金向量 attains/not_attains 两态冻结在档；判定语义（union_count ≥ N 承重即过）由 contribmath-derivation L43 sustained/collapsed verdict 实文与源注承 P3.2.1 构造性可达性承载",
         "regress": "合同参数 N 值级锚义务：PROB-016 计数测度达标线语义化时重推（批三注记候选）"},
        {"name": "GD_SUSTAINED_MIN_DAYS_DEFAULT", "site": "sih-tools/gauge/src/gauge/cli.py:80", "value": "3",
         "evidence": "同上；sustained=连续窗无塌陷语义由 contribmath-derivation L43 实文承载",
         "regress": "合同参数 D 值级锚义务：连续窗长度的统计意义化（PROB-016/PROB-013 窗口对价）时重推"},
        {"name": "LOCKFACE_WIDE_THRESHOLD", "site": "sih-tools/watchcheck/src/watchcheck/constants.py", "value": "(在役值见源)",
         "evidence": "leaseup 线哨兵超宽三态只报不拦在役（GOV-002 v2.4 第五条证据面）；pk-074 残项登记在案",
         "regress": "锁面宽阈统计化评估义务（锁面形状文规治理不机器拦截先例同域）"},
        {"name": "HEARTBEAT_STALE_SECONDS", "site": "sih-tools/lease/src/lease/cli.py 与 lockdb.py（双位同值）", "value": "(在役值见源)",
         "evidence": "lease 心跳停滞判据在役（openhyg PID 探针勘误与检验文件生死判定承载消费位）",
         "regress": "停滞阈统计化评估义务（进程活性判据形式化归 PROB 族评估）"},
        {"name": "M_ACTIVE", "site": "sih-tools/confledger/src/confledger/constants.py", "value": "(在役值见源)",
         "evidence": "confledger v0 八子命令在役（置信度线阶段一批）",
         "regress": "置信度数学模型承接（pk-073 系；pk-078 残余载体 gate 联动）——M 形态参数由该模型重估"},
        {"name": "M_CLEAN", "site": "sih-tools/confledger/src/confledger/constants.py", "value": "(在役值见源)",
         "evidence": "同上", "regress": "同上"},
        {"name": "WINDOW_DAYS", "site": "sih-tools/critsweep/sweep.py（本批产物自登）", "value": "10",
         "evidence": "critsweep 在役（双跑 IDENTICAL 判据在档）；扫描窗 10 天覆盖沉底阈 3 天与批节律",
         "regress": f"{PROB015_L61} 约束实例化注记落位归批三（GQ_WINDOW 先例同形候推导档）"},
    ],
    "pure_convention": [
        {"name": "EXIT_CLEAN / EXIT_TOOL_ERROR / EXIT_UNOWNED", "site": "sih-tools/watchcheck/src/watchcheck/constants.py",
         "basis": "退出码枚举（0/1/2 三值语义约定），非判定数值；BATCH-FACE watch 节与 CONTRACT 判定式在档"},
        {"name": "DB_SCHEMA_VERSION", "site": "sih-tools/calllog/src/calllog/core.py",
         "basis": "schema 版本标记（结构约定），派生可重建面版本位"},
        {"name": "LOCK_BILL_UNIT / EXPANSION_FREE_QUOTA / UNUSED_LOCK_MULTIPLIER", "site": "sih-tools/lease/src/lease/core.py",
         "basis": "锁面经济记账约定（先记账后算分，2026-09-06 用户裁定域）；billwire 与 leaseup 在役，账单台账 lockface-bills 双写一致判据在档；定价模型数学化评估义务随批三呈报"},
    ],
    "retired_void": [
        {"name": "N_SHOTS / PACK_VERSION", "site": "sih-tools/facet/probes/retired/temp_probe.py",
         "basis": "温度探针退役路径在档（pk-044 出泊裁定 2026-09-07 与 baseinject 实装）；retired/ 路径即核销标记，无在役值可归因"},
        {"name": "REPEATS", "site": "sih-tools/facet/probes/retired/temp0_control.py",
         "basis": "同上 retired 路径"},
    ],
    "env_param": [
        {"name": "ROUTE_TIMEOUT_S", "site": "sih-tools/critsweep/sweep.py",
         "basis": "扫描器已判资源性 native 入 env-params 再生；单线路由十秒超时降级（CONTRACT 契约载）"},
    ],
}

reg = {
    "batch": "constclear2b-solo", "date": "2026-09-08",
    "session": "sess-zcode-20260908-fork2-constclear2b", "lease_session": "747ab81d85ac14f5",
    "kind": "constclear-registry-v1（登记面落位，A1 承接地）",
    "dual_channel_standard": "A1 双通道判定标准（席签 4073c5fa）：能数学化的裁决面必须挂数学载体与推导档（承 P1 67353ed9）；纯约定裁决面必须走登记面诚实登记且不得伪装成数学（承 P2 f4b8f93f）。登记面收环境参数类与纯约定类；数学态与工程三件套态见路由表（constclear2-routing 与 constclear2b-routing）不重复登记。",
    "env_params": [
        {"name": "MAX_ROUND", "site": "sih-tools/facet/probes/eir_ecr_gate_probe.py:78", "value": "3",
         "two_state_basis": "工程实践态（承 A2 类目不豁免取值）：判定边界由闸判据承载不由轮数承载；证据即 eir_ecr_gate_probe 在役；回归义务：采样轮数-判据强度关系统计化时重估（PROB 族）"},
        {"name": "MAX_SHOTS", "site": "sih-tools/facet/probes/lightweight_mode_probe.py:146", "value": "7",
         "two_state_basis": "工程实践态：停采判定由 FLIP_STOP 众数规则承载，值只封采样预算；证据即 lightweight 档在役；回归义务：发射预算-置信度关系随置信度线重估（pk-073 系）"},
        {"name": "MAX_ROUND", "site": "sih-tools/facet/probes/multiround_convergence_probe.py:108", "value": "5",
         "two_state_basis": "工程实践态：轨迹判定规则不由轮数承载（观测点 1/2/3/5 注记在源）；证据即 multiround 在役；回归义务：观测轮数-收敛评估关系统计化时重估"},
        {"name": "ROUTE_TIMEOUT_S", "site": "sih-tools/critsweep/sweep.py", "value": "10",
         "two_state_basis": "资源性（扫描器 native 判）：超时只降级不改变路由判定语义；证据即 critsweep 降级可见条款在档；回归义务：无（纯资源上限，维持申报）"},
    ],
    "pure_conventions": NEW_ACCOUNT["pure_convention"],
    "debts_and_drifts": [
        "gd-1 旧名族退场照录：CUSUM_B_DEFAULT/CUSUM_H_DEFAULT/K_SIGMA_DEFAULT/GD_BASELINE_N_DEFAULT/GD_P0_DEFAULT/GD_ALPHA_DEFAULT 六件自账面移除（GC_* 前缀继位与 gd-2 合同参数继位）",
        "rev3 再生与在档版 drift：lit_new 93 与 lit_gone 8（源码 09-04 后演进），exit 1 即 drift 旗非错误，双跑 IDENTICAL 确定性成立",
        "R2 债清偿：RECLASS_INHERIT 扩表三件，env-params 三行 overlay 转 native",
    ],
}

routing = {
    "batch": "constclear2b-solo", "date": "2026-09-08",
    "session": "sess-zcode-20260908-fork2-constclear2b",
    "caliber": "两态制续承（41fb339c）＋A1 双通道（4073c5fa）＋A2 类目不豁免取值（15f9a0ad）＋A3 批界（1f03de72）",
    "scope": "rev3 扫描新增 25 件两态路由（相对批一 34 件账本）；扫描双跑 IDENTICAL，exit 1 即 drift 旗如实申报",
    "distribution": {k: len(v) for k, v in NEW_ACCOUNT.items()},
    "items": NEW_ACCOUNT,
    "verify": "内联载体声明逐件对条目与推导档实文核验（PROB-013 L12、PROB-015 L16 与 L61、PROB-010 L24、gchart L22/L36/L44/L46、contribmath L43/L51、queueing L34 命中在案）；无实文承载即工程态不硬凑",
}

MD.mkdir(parents=True, exist_ok=True)
OUT_JSON.write_text(json.dumps(routing, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
REG_JSON.write_text(json.dumps(reg, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")

md = ["# constclear2b 路由增补：新账扫描 25 件两态路由", "",
      "> 批：constclear2b-solo（2026-09-08，会话 sess-zcode-20260908-fork2-constclear2b）",
      "> 口径：" + routing["caliber"], "",
      "## 1 分布 {#dist}", ""]
for k, v in routing["distribution"].items():
    md.append(f"- {k}: {v} 件")
md += ["", "## 2 数学模型态 5 件 {#math}", "",
       "| 常数 | 位 | 值 | 载体实文 |", "|---|---|---|---|"]
for it in NEW_ACCOUNT["math_model"]:
    md.append(f"| {it['name']} | {it['site']} | {it['value']} | {it['carrier']} |")
md += ["", "## 3 工程实践三件套态 7 件 {#eng}", "",
       "| 常数 | 位 | 证据 | 回归义务 |", "|---|---|---|---|"]
for it in NEW_ACCOUNT["engineering_practice"]:
    md.append(f"| {it['name']} | {it['site']} | {it['evidence'][:80]}… | {it['regress'][:80]}… |")
md += ["", "## 4 纯约定登记 7 件（A1 登记面）·退场核销 3 件·环境参数 1 件 {#rest}", ""]
for it in NEW_ACCOUNT["pure_convention"]:
    md.append(f"- [纯约定] {it['name']} @ {it['site']}：{it['basis'][:100]}")
for it in NEW_ACCOUNT["retired_void"]:
    md.append(f"- [退场核销] {it['name']} @ {it['site']}：{it['basis'][:80]}")
for it in NEW_ACCOUNT["env_param"]:
    md.append(f"- [环境参数] {it['name']} @ {it['site']}：{it['basis'][:80]}")
md += ["", "## 5 核验与漂移 {#verify}", "",
       "- " + routing["verify"],
       "- rev3 再生双跑三路 cmp IDENTICAL；exit 1 即 drift 旗（lit_new 93 与 lit_gone 8）如实申报不掩盖",
       "- gd-1 旧名六件退场照录（GC_* 与 gd-2 继位）", ""]
OUT_MD.write_text("\n".join(md) + "\n", encoding="utf-8")

rm = ["# constclear 登记面 v1（A1 双通道承接地）", "",
      "> 批：constclear2b-solo（2026-09-08）；本登记面是纯约定与环境参数类的常设诚实登记位，数学态与工程三件套态见路由表族",
      "> 双通道判定标准：" + reg["dual_channel_standard"], "",
      "## 1 环境参数类 {#env}", "",
      "| 常数 | 位 | 值 | 两态依据（A2 类目不豁免取值） |", "|---|---|---|---|"]
for it in reg["env_params"]:
    rm.append(f"| {it['name']} | {it['site']} | {it['value']} | {it['two_state_basis']} |")
rm += ["", "## 2 纯约定类 {#pure}", ""]
for it in reg["pure_conventions"]:
    rm.append(f"- {it['name']} @ {it['site']}：{it['basis']}")
rm += ["", "## 3 债与漂移照录 {#debts}", ""]
for d in reg["debts_and_drifts"]:
    rm.append(f"- {d}")
rm.append("")
REG_MD.write_text("\n".join(rm) + "\n", encoding="utf-8")
print("registry + routing addendum written | 分布:", routing["distribution"])
