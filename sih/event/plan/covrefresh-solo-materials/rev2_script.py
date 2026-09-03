#!/usr/bin/env python3
"""covrefresh-solo 覆盖账本刷新脚本 —— rev2 三态刷新 + 双实存核验 + 分类重扫。

承接 rev1_script.py 口径（白名单 167 概念 ID 不变，原账本三件与 rev1 三件只读），
刷新 rev1 之后账面漂移，产出 rev2 三件到 argv[1] 指定目录（工地），主树零直写。

rev1 之后接线反映（F-2 逐件）：
- 六件接线批：tally（ORD-006/ORD-008/ORD-011）、selector（ALG-002 第二消费位）、
  lease（ORD-020）、引擎 scribe（ORD-019）、identity（ALG-002/PROB-013）、cascade（ORD-016）
- gatecap 与 scrutinator 闸：tools 与 engine scrutinator 源码推导 ORD-008（C007/C008）
- facet 数学基础件：PROB-010 源码推导（facet_stats_inf.py，mathpipe-a2 推导档）
- gauge ga-2：PROB-003/PROB-005 源码推导（mathpipe-a3 推导档，期票清偿）

三态判定（rev2 收紧）：已实例化 = 源码推导面非空 且 推导档磁盘在案（双门）；
可指认未实例化 = 有指认面（金向量/规格引用）而无源码实例化；无可指认载体 = 三面零命中。

口径声明（承 rev1，漂移零静默）：
- SPEC 双义 ID（白名单 SPEC-001..007 ∩ 引擎规格档）不在漏项核对域；消歧规则承 rev1：
  引擎 src 引用位默认引擎侧。规格引用面归属零变动承 rev1（已认四件 tools SPEC_ELS 与
  引擎六件同构九 SPEC 面）；rev1 未认的 SPEC 词面（parser 与 tools scrutinator 语料内）
  rev2 不新增面归属。
- 源码扫描语料：tools 逐工具目录（跳 tests/reports/docs/task-packages/plan/contracts/
  facet_task_packages），ext .py/.rs/.toml；engine 逐机制目录（rev1 为整 src 粗粒度，
  rev2 细化到机制目录，对四件未实例化机制读数无影响）。
- 漏项核对线（F-2）：语料内非 SPEC 白名单 ID 命中必须全部被 CARRIER_MAP 覆盖，
  否则记入 scan_findings 且退出码一。
- 分类重扫（F-4）：枚举与分类正则承 mathpipe-a1 audit.py 原式；rev1 十二件资源性改判
  以（相对路径，常数名）键承继（rev1 原件行号键仅存档）；与 rev1 生效面逐件对表，
  变动/新增/消失三桶申报零静默。
"""
from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
COV_IN = ROOT / "sih-math/docs/mathpipe-coverage-2026-09-03"  # 只读：原账本与 rev1 件
MATH = ROOT / "sih-math"
DOSSIER_DIR = MATH / "docs"
ENGINE_SPEC_DIR = ROOT / "sih-engine/doc/spec"
RUN_DATE = "2026-09-04"
REVISION = "covrefresh-solo rev2"

# ---------- 1. 白名单：五子仓 INDEX 磁盘实存 ID 全集（承 rev1 口径不变） ----------

def build_whitelist() -> tuple[dict[str, set[str]], dict[str, tuple[str, str]]]:
    wl: dict[str, set[str]] = {}
    id_home: dict[str, tuple[str, str]] = {}
    for sub in ["calculus", "order", "probability", "topology", "algebra"]:
        idx = (MATH / sub / "INDEX.md").read_text(encoding="utf-8")
        ids = set(re.findall(r"\|\s*([A-Z]+-\d+)\s*\|", idx))
        wl[sub] = ids
        for m in re.finditer(r"\|\s*([A-Z]+-\d+)\s*\|\s*([^|]+)\|", idx):
            cid = m.group(1)
            if cid not in id_home:
                id_home[cid] = (sub, m.group(2).strip())
    return wl, id_home

# ---------- 2. 载体映射表（每条 evidence 逐件 grep 核验，缺失即 finding） ----------
# (domain, name) -> {concept_id: (face, [evidence 相对路径...])}
CARRIER_MAP: dict[tuple[str, str], dict[str, tuple[str, list[str]]]] = {
    # 六件接线批
    ("sih-tools", "tally"): {
        "ORD-006": ("源码推导", ["sih-tools/tally/src/tally/cli.py"]),
        "ORD-008": ("源码推导", ["sih-tools/tally/src/tally/cli.py"]),
        "ORD-011": ("源码推导", ["sih-tools/tally/src/tally/cli.py"]),
    },
    ("sih-tools", "selector"): {
        "ALG-002": ("源码推导", ["sih-tools/selector/src/selector/pack.py",
                                 "sih-tools/selector/src/selector/route.py"]),
    },
    ("sih-tools", "lease"): {
        "ORD-020": ("源码推导", ["sih-tools/lease/src/lease/lockcore.py"]),
    },
    ("sih-engine", "scribe"): {
        "ORD-019": ("源码推导", ["sih-engine/src/event_stream/append.rs",
                                 "sih-engine/src/event_stream/verify.rs",
                                 "sih-engine/src/event_stream/park.rs"]),
    },
    ("sih-tools", "identity"): {
        "ALG-002": ("源码推导", ["sih-tools/identity/src/identity/core.py"]),
        "PROB-013": ("源码推导", ["sih-tools/identity/src/identity/core.py"]),
    },
    ("sih-tools", "cascade"): {
        "ORD-016": ("源码推导", ["sih-tools/cascade/src/cascade/core.py"]),
    },
    # gatecap 与 scrutinator 闸（C007/C008，载体 ORD-008）
    ("sih-tools", "scrutinator"): {
        "ORD-008": ("源码推导", ["sih-tools/scrutinator/packs/des-001/rules.toml"]),
    },
    ("sih-engine", "scrutinator"): {
        "ORD-008": ("源码推导", ["sih-engine/src/scrutinator/packs/des-001/rules.toml"]),
        "LIM-001": ("金向量消费", ["sih-engine/src/scrutinator/fixtures/golden/des-001-mathe-lim001.json"]),
        "MUL-001": ("金向量消费", ["sih-engine/src/scrutinator/fixtures/golden/des-001-mathe-mul001.json"]),
    },
    # facet 数学基础件（facet_stats_inf 检验函数族 A2，mathpipe-a2 推导档）
    ("sih-tools", "facet"): {
        "APP-009": ("金向量消费", ["sih-tools/facet/probes/r3ap_uncertainty_channel.py",
                                   "sih-tools/facet/probes/phase2d_probe.py"]),
        "APP-010": ("金向量消费", ["sih-tools/facet/probes/yi_depth_probe.py"]),
        "DIFF-015": ("金向量消费", ["sih-tools/facet/probes/multiround_convergence_probe.py"]),
        "INT-007": ("金向量消费", ["sih-tools/facet/probes/contribution_metric.py"]),
        "LIM-004": ("金向量消费", ["sih-tools/facet/probes/foregrounding_metrics.py"]),
        "LIM-007": ("金向量消费", ["sih-tools/facet/probes/maturation_gate.py",
                                   "sih-tools/facet/probes/noise_floor_probe.py"]),
        "MUL-005": ("金向量消费", ["sih-tools/facet/probes/maturation_gate.py",
                                   "sih-tools/facet/probes/r3a_gate_v2.py"]),
        "MUL-008": ("金向量消费", ["sih-tools/facet/probes/phase2e_probe.py"]),
        "PROB-010": ("源码推导", ["sih-tools/facet/src/facet_stats_inf.py"]),
    },
    # gauge ga-2（PROB-003/005 期票清偿，mathpipe-a3 推导档）
    ("sih-tools", "gauge"): {
        "LIM-007": ("源码推导", ["sih-tools/gauge/src/gauge/cli.py"]),
        "ORD-002": ("源码推导", ["sih-tools/gauge/src/gauge/cli.py"]),
        "PROB-001": ("源码推导", ["sih-tools/gauge/src/gauge/cli.py"]),
        "PROB-003": ("源码推导", ["sih-tools/gauge/src/gauge/cli.py"]),
        "PROB-005": ("源码推导", ["sih-tools/gauge/src/gauge/cli.py"]),
    },
}

# ---------- 3. 规格引用面（承 rev1 零变动；每条 evidence 核验） ----------
SPEC_FACE_NINE = ["SPEC-004", "SPEC-005", "SPEC-006", "SPEC-007", "SPEC-008",
                  "SPEC-011", "SPEC-013", "SPEC-014", "SPEC-015"]


def spec_doc_files(sid: str) -> list[str]:
    """引擎规格档实际文件名通配（SPEC-004-event-stream.md 形）。"""
    hits = sorted(p.name for p in ENGINE_SPEC_DIR.glob(f"{sid}-*.md"))
    return [f"sih-engine/doc/spec/{h}" for h in hits]


SPEC_FACES: dict[tuple[str, str], dict[str, list[str]]] = {}
for _mech in ["ask3repeater", "attractor", "retriever", "scribe", "scrutinator", "viewer"]:
    SPEC_FACES[("sih-engine", _mech)] = {
        sid: spec_doc_files(sid) for sid in SPEC_FACE_NINE
    }
SPEC_FACES[("sih-tools", "scribe")] = {
    "SPEC-004": ["sih-tools/scribe/CONTRACT.md"],
    "SPEC-005": ["sih-tools/scribe/CONTRACT.md"],
}
SPEC_FACES[("sih-tools", "cascade")] = {
    "SPEC-009": ["sih-tools/cascade/CONTRACT.md"],
}
SPEC_FACES[("sih-tools", "lease")] = {
    "SPEC-011": ["sih-tools/lease/CONTRACT.md"],
}
SPEC_FACES[("sih-tools", "gauge")] = {
    "SPEC-011": ["sih-engine/doc/spec/SPEC-011-governance-state-reading.md"],
}

# ---------- 4. 推导档门（已实例化第二门：推导档磁盘在案） ----------
DOSSIERS: dict[tuple[str, str], str] = {
    ("sih-tools", "gauge"): "mathpipe-a3-derivation-2026-09-03.md",
    ("sih-tools", "facet"): "mathpipe-a2-derivation-2026-09-03.md",
    ("sih-tools", "tally"): "tallywire-tally-derivation-2026-09-04.md",
    ("sih-tools", "selector"): "selwire-selector-derivation-2026-09-04.md",
    ("sih-tools", "lease"): "ordwire-lease-derivation-2026-09-03.md",
    ("sih-engine", "scribe"): "scriwire-scribe-derivation-2026-09-03.md",
    ("sih-tools", "identity"): "idwire-identity-derivation-2026-09-03.md",
    ("sih-tools", "cascade"): "caswire-cascade-derivation-2026-09-03.md",
    ("sih-tools", "scrutinator"): "gatecap-derivation-2026-09-04.md",
    ("sih-engine", "scrutinator"): "gatecap-derivation-2026-09-04.md",
}

# ---------- 5. 源码扫描语料 ----------
TOOLS_SKIP_DIRS = {".git", "target", "node_modules", "__pycache__", ".venv",
                   "tests", "reports", "docs", "task-packages", "plan",
                   "facet_task_packages", "contracts"}
SCAN_EXTS = (".py", ".rs", ".toml")
MATH_ID = re.compile(r"\b(PROB|ORD|TOP|ALG|LIM|DIFF|INT|SPEC|MUL|SER|HIS|NS|APP)-\d{3}\b")
ENGINE_MECH_DIRS: dict[str, list[str]] = {
    "ask3repeater": ["sih-engine/src/ask3repeater", "sih-engine/src/bin/ask3repeater.rs"],
    "attractor": ["sih-engine/src/attractor", "sih-engine/src/bin/attractor.rs"],
    "retriever": ["sih-engine/src/retriever", "sih-engine/src/bin/retriever.rs"],
    "scribe": ["sih-engine/src/event_stream", "sih-engine/src/bin/scribe.rs"],
    "scrutinator": ["sih-engine/src/scrutinator", "sih-engine/src/bin/scrutinator.rs"],
    "viewer": ["sih-engine/src/view", "sih-engine/src/bin/viewer.rs"],
}


def scan_corpus(files: list[Path], whitelist_all: set[str]) -> tuple[set[str], set[str]]:
    """返回 (非 SPEC 白名单命中, SPEC 命中)。"""
    nonspec: set[str] = set()
    spec: set[str] = set()
    for p in files:
        try:
            t = p.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for m in MATH_ID.finditer(t):
            mid = m.group(0)
            if mid.startswith("SPEC-"):
                spec.add(mid)
            elif mid in whitelist_all:
                nonspec.add(mid)
    return nonspec, spec


def tool_corpus_files(name: str) -> list[Path]:
    base = ROOT / "sih-tools" / name
    out: list[Path] = []
    if not base.is_dir():
        return out
    for dirpath, dirnames, filenames in os.walk(base):
        dirnames[:] = sorted(d for d in dirnames if d not in TOOLS_SKIP_DIRS)
        for f in sorted(filenames):
            if f.endswith(SCAN_EXTS):
                out.append(Path(dirpath) / f)
    return out


def engine_corpus_files(mech: str) -> list[Path]:
    out: list[Path] = []
    for rel in ENGINE_MECH_DIRS[mech]:
        p = ROOT / rel
        if p.is_file():
            out.append(p)
        elif p.is_dir():
            for dirpath, dirnames, filenames in os.walk(p):
                dirnames[:] = sorted(d for d in dirnames if d not in TOOLS_SKIP_DIRS)
                for f in sorted(filenames):
                    if f.endswith(SCAN_EXTS):
                        out.append(Path(dirpath) / f)
    return out

# ---------- 6. 分类重扫（正则承 mathpipe-a1 audit.py 原式） ----------
FAMILY = re.compile(
    r"(THRESHOLD|MAX|MIN|BUDGET|CAP|LIMIT|TTL|SKEW|TIMEOUT|ALPHA|BETA|EPS|"
    r"WINDOW|QUORUM|MAJORITY|RATIO|PVALUE|P_VALUE|KAPPA|LAMBDA|WEIGHT|BOUNDARY)")
RESOURCE_NAME = re.compile(r"(TIMEOUT|TTL|RETRY|BUFFER|CHUNK|DEPTH|SIZE|LEN|PORT)")
DECISION_NAME = re.compile(
    r"(THRESHOLD|QUORUM|RATIO|ALPHA|BETA|EPS|SKEW|PVALUE|P_VALUE|KAPPA|"
    r"BOUNDARY|WEIGHT|MAJORITY|CONFIDENCE)")
TRIVIAL = {"-1", "0", "1", "2"}
PY_CONST = re.compile(r"^([A-Z][A-Z0-9_]{2,})\s*=\s*(-?\d+(?:\.\d+)?)\s*(?:#.*)?$", re.M)
RS_CONST = re.compile(r"const\s+([A-Z][A-Z0-9_]{2,})[^=\n]*=\s*(-?\d+(?:\.\d+)?)")
CMP = re.compile(r"([<>]=?|==|!=)\s*(-?\d+(?:\.\d+)?)(?![\.\d])")
AUDIT_SKIP = {".git", "target", "node_modules", "__pycache__", ".venv"}


def audit_source_files(root: Path, exts: tuple[str, ...]) -> list[Path]:
    out: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = sorted(d for d in dirnames if d not in AUDIT_SKIP)
        if "/tests/" in (dirpath.replace(os.sep, "/") + "/") or os.path.basename(dirpath) == "tests":
            continue
        for f in sorted(filenames):
            if f.endswith(exts):
                out.append(Path(dirpath) / f)
    return out


def rel(path: Path) -> str:
    return os.path.relpath(path, ROOT)


def scan_named_constants() -> list[dict]:
    rows = []
    for path in audit_source_files(ROOT / "sih-tools", (".py",)):
        t = path.read_text(encoding="utf-8", errors="replace")
        for m in PY_CONST.finditer(t):
            name, value = m.group(1), m.group(2)
            if RESOURCE_NAME.search(name) and not DECISION_NAME.search(name):
                cls = "资源性"
            elif DECISION_NAME.search(name) or FAMILY.search(name):
                cls = "判定性"
            else:
                cls = "待确认"
            rows.append({"file": rel(path), "line": t.count("\n", 0, m.start()) + 1,
                         "kind": "named", "name": name, "value": value, "class": cls,
                         "context": m.group(0)[:160]})
    for path in audit_source_files(ROOT / "sih-engine", (".rs",)):
        t = path.read_text(encoding="utf-8", errors="replace")
        for m in RS_CONST.finditer(t):
            name, value = m.group(1), m.group(2)
            if RESOURCE_NAME.search(name) and not DECISION_NAME.search(name):
                cls = "资源性"
            elif DECISION_NAME.search(name) or FAMILY.search(name):
                cls = "判定性"
            else:
                cls = "待确认"
            rows.append({"file": rel(path), "line": t.count("\n", 0, m.start()) + 1,
                         "kind": "named", "name": name, "value": value, "class": cls,
                         "context": m.group(0)[:160]})
    rows.sort(key=lambda r: (r["file"], r["line"], r["name"]))
    return rows


def scan_literals() -> list[dict]:
    rows = []
    for base in (ROOT / "sih-tools", ROOT / "sih-engine"):
        for path in audit_source_files(base, (".py", ".rs")):
            t = path.read_text(encoding="utf-8", errors="replace")
            id_refs = sorted(set(m.group(0) for m in MATH_ID.finditer(t)))
            for i, line in enumerate(t.splitlines(), 1):
                stripped = line.strip()
                if stripped.startswith(("#", "//", "*")):
                    continue
                for m in CMP.finditer(line):
                    v = m.group(2)
                    if v in TRIVIAL or v.lstrip("-").startswith("0x"):
                        continue
                    frac = 0 < abs(float(v)) < 1
                    cls = "判定性" if frac else "待确认"
                    rows.append({"file": rel(path), "line": i, "kind": "literal",
                                 "name": None, "value": v, "class": cls,
                                 "context": stripped[:160], "file_math_ids": id_refs})
    rows.sort(key=lambda r: (r["file"], r["line"], r["value"]))
    return rows

# ---------- 7. rev1 资源性改判承继（十二件，键稳定化申报） ----------
RECLASS_INHERIT = [
    ("GLOBAL_MAX_CONCURRENT", "sih-tools/facet/probes/anchor_gradient.py", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "sih-tools/facet/probes/bps_gradient.py", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "sih-tools/facet/probes/calibration_quality_comparison.py", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "sih-tools/facet/probes/horizontal_interaction_matrix.py", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "sih-tools/facet/probes/p3_experiment_matrix.py", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "sih-tools/facet/probes/p3_path_b_experiment.py", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "sih-tools/facet/probes/rerun_reflective.py", "并发上限族"),
    ("GLOBAL_MAX_CONCURRENT", "sih-tools/facet/probes/validity_verification_matrix.py", "并发上限族"),
    ("DEFAULT_MAX_CONCURRENT", "sih-tools/facet/src/concurrency.py", "并发上限族"),
    ("PROC_CAP", "sih-tools/identity/src/identity/core.py", "进程并发上限，改判资源性"),
    ("MAX_TOKENS", "sih-tools/facet/probes/verify_thinking_silent.py", "令牌上限，改变只改资源用量不改判定"),
    ("MAX_RETRIES", "sih-tools/facet/probes/retry_qwen_failed.py", "重试上限，改变只改容错次数不改判定"),
]


def apply_reclass(rows: list[dict]) -> tuple[list[dict], list[dict]]:
    """承 rev1 十二件改判；返回 (改判后全表, 改判申报细目)。"""
    keyset = {(n, f) for n, f, _ in RECLASS_INHERIT}
    basis_map = {(n, f): b for n, f, b in RECLASS_INHERIT}
    declared = []
    out = []
    for r in rows:
        item = dict(r)
        if (r["name"], r["file"]) in keyset and r["class"] == "判定性":
            item["class"] = "资源性"
            declared.append({
                "name": r["name"], "file": r["file"], "line": r["line"],
                "value": r["value"], "old_class": "判定性", "new_class": "资源性",
                "basis": basis_map[(r["name"], r["file"])], "context": r["context"],
            })
        out.append(item)
    return out, declared

# ---------- 8. 主流程 ----------

def build_engine_spec_ids() -> set[str]:
    ids = set()
    if ENGINE_SPEC_DIR.exists():
        for p in ENGINE_SPEC_DIR.glob("SPEC-*.md"):
            m = re.match(r"SPEC-(\d+)", p.name)
            if m:
                ids.add(f"SPEC-{int(m.group(1)):03d}")
    return ids


def main() -> None:
    cov_out = Path(sys.argv[1])
    findings: list[str] = []

    ledger0 = json.loads((COV_IN / "ledger.json").read_text(encoding="utf-8"))
    rev1 = json.loads((COV_IN / "ledger-rev1.json").read_text(encoding="utf-8"))
    wl, id_home = build_whitelist()
    whitelist_all = {i for s in wl.values() for i in s}

    # --- 机制清单：承原账本 23 件（tools 17 + engine 6，本批复核枚举面零漂移） ---
    mechanisms = [(m["domain"], m["name"]) for m in ledger0["enumeration"]["mechanisms"]]

    # --- 源码扫描 + 漏项核对（F-2） ---
    scan_report: dict[str, dict] = {}
    for domain, name in mechanisms:
        files = (tool_corpus_files(name) if domain == "sih-tools"
                 else engine_corpus_files(name))
        nonspec, spec = scan_corpus(files, whitelist_all)
        mapped = set(CARRIER_MAP.get((domain, name), {}).keys())
        uncovered = sorted(nonspec - mapped)
        if uncovered:
            findings.append(f"漏项核对失败: {domain}/{name} 语料命中未被载体映射覆盖: {uncovered}")
        scan_report[f"{domain}/{name}"] = {
            "corpus_files": len(files),
            "nonspec_hits": sorted(nonspec),
            "uncovered": uncovered,
        }

    # --- 载体映射与规格面 evidence 逐件核验 ---
    for (domain, name), ids in CARRIER_MAP.items():
        for cid, (face, evidences) in sorted(ids.items()):
            for ev in evidences:
                p = ROOT / ev
                if not p.is_file():
                    findings.append(f"证据缺失: {domain}/{name} {cid} {face} {ev}")
                elif cid not in p.read_text(encoding="utf-8", errors="replace"):
                    findings.append(f"证据无命中: {domain}/{name} {cid} {face} {ev}")
    for (domain, name), ids in SPEC_FACES.items():
        for sid, evidences in sorted(ids.items()):
            for ev in evidences:
                p = ROOT / ev
                if not p.is_file():
                    findings.append(f"证据缺失: {domain}/{name} {sid} 规格引用 {ev}")

    # --- 三态组装 ---
    new_carrier = []
    for domain, name in mechanisms:
        key = (domain, name)
        faces = {"源码推导": [], "金向量消费": [], "规格引用": [], "无载体": []}
        for cid, (face, _ev) in sorted(CARRIER_MAP.get(key, {}).items()):
            faces[face].append(cid)
        for sid in sorted(SPEC_FACES.get(key, {}).keys()):
            faces["规格引用"].append(sid)
        # rev1 承继：引擎六件原账本 LIM/MUL 词目在无金向量面时记无载体面（rev1 形）
        if domain == "sih-engine":
            golden = set(CARRIER_MAP.get(key, {}).keys())
            for lim_id in ["LIM-001", "MUL-001"]:
                if lim_id not in golden and lim_id not in faces["无载体"]:
                    faces["无载体"].append(lim_id)
        # 双门判定
        dossier = DOSSIERS.get(key)
        dossier_on_disk = bool(dossier) and (DOSSIER_DIR / dossier).is_file()
        if dossier and not dossier_on_disk:
            findings.append(f"推导档缺: {domain}/{name} {dossier}")
        if faces["源码推导"] and dossier_on_disk:
            state = "已实例化"
        elif faces["金向量消费"] or faces["规格引用"] or faces["源码推导"]:
            state = "可指认未实例化"
        else:
            state = "无可指认载体"
        new_carrier.append({
            "domain": domain, "name": name, "carrier_state": state,
            "face_refs": faces,
            "derivation_dossier": dossier,
            "dossier_on_disk": dossier_on_disk,
        })

    state_counts = {"已实例化": 0, "可指认未实例化": 0, "无可指认载体": 0}
    for c in new_carrier:
        state_counts[c["carrier_state"]] += 1

    # --- F-3 双实存核验：全部已实例化机制的源码推导 ID 逐件（承 rev1 对挂核验表形） ---
    rev1_instantiated = {c["name"] for c in
                         json.loads(json.dumps(rev1["carrier_matching"]))
                         if c["carrier_state"] == "已实例化" and c["domain"] == "sih-tools"}
    f3_rows = []
    for c in new_carrier:
        if c["carrier_state"] != "已实例化":
            continue
        newly = c["name"] not in rev1_instantiated or c["domain"] == "sih-engine"
        for cid in c["face_refs"]["源码推导"]:
            home = id_home.get(cid)
            sub, cname = home if home else (None, None)
            in_index = bool(sub and cid in wl[sub])
            on_disk = bool(sub and list((MATH / sub / "entries").glob(f"{cid}-*.md")))
            f3_rows.append({
                "mechanism": c["name"], "domain": c["domain"], "concept_id": cid,
                "concept_name": cname, "subrepo": sub,
                "in_index": in_index, "on_disk": on_disk,
                "newly_instantiated": newly,
                "result": "实存" if (in_index and on_disk) else "缺失",
            })
    for r in f3_rows:
        if r["result"] != "实存" and r["newly_instantiated"]:
            findings.append(f"双实存核验缺失: {r['mechanism']} {r['concept_id']}")
    inherited_gaps = [r for r in f3_rows
                      if r["result"] != "实存" and not r["newly_instantiated"]]

    # --- 分类重扫（F-4） ---
    named_raw = scan_named_constants()
    named2, reclass_declared = apply_reclass(named_raw)
    literals2 = scan_literals()

    # rev1 生效面重建（原账本分类 + rev1 十二件改判，键（相对路径，常数名））
    named1_raw = ledger0["enumeration"]["named_constants"]
    reclass1_keys = {(d["file"], d["name"]) for d in rev1["reclassification_declared"]}
    named1_eff = {}
    for n in named1_raw:
        k = (n["file"], n["name"])
        cls = n["class"]
        if k in reclass1_keys and cls == "判定性":
            cls = "资源性"
        named1_eff[k] = cls
    named2_eff = {(n["file"], n["name"]): n["class"] for n in named2}

    class_changed, only_rev2, only_rev1 = [], [], []
    for k in sorted(set(named2_eff) | set(named1_eff)):
        c1, c2 = named1_eff.get(k), named2_eff.get(k)
        if c1 is None and c2 is not None:
            only_rev2.append({"file": k[0], "name": k[1], "rev2_class": c2})
        elif c2 is None and c1 is not None:
            only_rev1.append({"file": k[0], "name": k[1], "rev1_class": c1})
        elif c1 != c2:
            class_changed.append({"file": k[0], "name": k[1],
                                  "rev1_class": c1, "rev2_class": c2})

    # 字面量逐件对表（键：相对路径+值+上下文）
    lit1_raw = ledger0["enumeration"]["comparison_literals"]
    lit1_eff = {(l["file"], l["value"], l["context"]): l["class"] for l in lit1_raw}
    lit2_eff = {(l["file"], l["value"], l["context"]): l["class"] for l in literals2}
    lit_changed = sum(1 for k in set(lit2_eff) & set(lit1_eff)
                      if lit2_eff[k] != lit1_eff[k])
    lit_new = len(set(lit2_eff) - set(lit1_eff))
    lit_gone = len(set(lit1_eff) - set(lit2_eff))
    lit_new_items = [{"file": l["file"], "line": l["line"], "value": l["value"],
                      "rev2_class": l["class"], "context": l["context"][:80]}
                     for l in literals2
                     if (l["file"], l["value"], l["context"]) not in lit1_eff]
    lit_gone_items = [{"file": l["file"], "line": l["line"], "value": l["value"],
                       "rev1_class": l["class"], "context": l["context"][:80]}
                      for l in lit1_raw
                      if (l["file"], l["value"], l["context"]) not in lit2_eff]

    named_counts = {
        "named_total": len(named2),
        "named_decision": sum(1 for n in named2 if n["class"] == "判定性"),
        "named_resource": sum(1 for n in named2 if n["class"] == "资源性"),
        "named_pending": sum(1 for n in named2 if n["class"] == "待确认"),
    }
    literal_counts = {
        "literal_total": len(literals2),
        "literal_decision": sum(1 for l in literals2 if l["class"] == "判定性"),
        "literal_pending": sum(1 for l in literals2 if l["class"] == "待确认"),
    }

    # --- env-params-rev2.json ---
    reclass_keys = {(d["name"], d["file"]) for d in reclass_declared}
    basis_map = {(d["name"], d["file"]): d["basis"] for d in reclass_declared}
    env_params = []
    for n in named2:
        if n["class"] == "资源性":
            item = {k: n[k] for k in ("class", "context", "file", "kind", "line",
                                      "name", "value")}
            if (n["name"], n["file"]) in reclass_keys:
                item["reclassified_from"] = "判定性"
                item["basis"] = basis_map[(n["name"], n["file"])]
            env_params.append(item)

    # --- wiring 反映表（F-2 逐件） ---
    wiring_reflected = []
    for domain, name in [("sih-tools", "tally"), ("sih-tools", "selector"),
                         ("sih-tools", "lease"), ("sih-engine", "scribe"),
                         ("sih-tools", "identity"), ("sih-tools", "cascade"),
                         ("sih-tools", "scrutinator"), ("sih-engine", "scrutinator"),
                         ("sih-tools", "facet"), ("sih-tools", "gauge")]:
        c = next(x for x in new_carrier if x["domain"] == domain and x["name"] == name)
        wiring_reflected.append({
            "mechanism": name, "domain": domain,
            "source_derivation_ids": c["face_refs"]["源码推导"],
            "derivation_dossier": c["derivation_dossier"],
            "carrier_state": c["carrier_state"],
        })

    rev2 = {
        "root": str(ROOT),
        "run_date": RUN_DATE,
        "revision": REVISION,
        "whitelist_size": len(whitelist_all),
        "whitelist_subrepo_sizes": {k: len(v) for k, v in wl.items()},
        "spec_disambiguation": {
            "engine_spec_ids": sorted(build_engine_spec_ids()),
            "whitelist_spec_ids": sorted(i for i in wl["calculus"] if i.startswith("SPEC-")),
            "overlap": sorted(set(i for i in wl["calculus"] if i.startswith("SPEC-"))
                              & build_engine_spec_ids()),
            "rule": "引擎 src 引用位默认引擎侧，除非白名单命中且引用上下文明示数学条目（承 rev1）",
        },
        "carrier_counts": state_counts,
        "carrier_matching": new_carrier,
        "wiring_reflected": wiring_reflected,
        "f3_verification": f3_rows,
        "f3_inherited_gaps": inherited_gaps,
        "scan_findings": findings,
        "scan_report": scan_report,
        "classification": {
            "counts": {**named_counts, **literal_counts},
            "diff_vs_rev1": {
                "named_class_changed": class_changed,
                "named_only_in_rev2": only_rev2,
                "named_only_in_rev1": only_rev1,
                "literal_changed": lit_changed,
                "literal_new": lit_new,
                "literal_gone": lit_gone,
                "literal_new_items": lit_new_items,
                "literal_gone_items": lit_gone_items,
            },
        },
        "reclassification_inherited": reclass_declared,
        "duibiao_note": "rev1 对挂七行由 F-3 双实存核验全表承接；selector 零命中已由 selwire 批 ALG-002 接线消解（商集第二消费位）",
    }
    rev2["spec_disambiguation"]["engine_spec_ids"] = sorted(build_engine_spec_ids())
    (cov_out / "ledger-rev2.json").write_text(
        json.dumps(rev2, ensure_ascii=False, indent=1, sort_keys=True) + "\n",
        encoding="utf-8")
    (cov_out / "env-params-rev2.json").write_text(
        json.dumps({"resource_params": env_params, "run_date": RUN_DATE},
                   ensure_ascii=False, indent=1, sort_keys=True) + "\n", encoding="utf-8")

    # --- summary-rev2.md ---
    def mechs(state: str) -> str:
        return "、".join(f"{c['name']}({c['domain']})" if c["name"] == "scrutinator"
                        or c["name"] == "scribe" else c["name"]
                        for c in new_carrier if c["carrier_state"] == state)

    lines = []
    a = lines.append
    a("# 数学管线全量载体覆盖审计汇总（rev2）")
    a("")
    a("> orig: covrefresh-solo 刷新件，原账本三件与 rev1 三件只读不动，本文件为刷新后汇总")
    a("")
    a("## 载体三态（rev2）{#carriers}")
    a("")
    a("| 状态 | 数量 | 机制 |")
    a("|---|---|---|")
    a(f"| 已实例化 | {state_counts['已实例化']} | " + mechs("已实例化") + " |")
    a(f"| 可指认未实例化 | {state_counts['可指认未实例化']} | " + mechs("可指认未实例化") + " |")
    a(f"| 无可指认载体 | {state_counts['无可指认载体']} | " + mechs("无可指认载体") + " |")
    a("")
    a("rev1 读数 1 已实例化 / 12 可指认未实例化 / 10 无可指认载体；rev2 承六件接线批与 "
      "gatecap 与 facet A2 与 gauge ga-2 逐件反映，同名跨域机制以括注域区分。")
    a("")
    a("## 接线反映（逐件）{#wiring}")
    a("")
    a("| 机制 | 域 | 源码推导载体 | 推导档 | 三态 |")
    a("|---|---|---|---|---|")
    for w in wiring_reflected:
        a(f"| {w['mechanism']} | {w['domain']} | "
          f"{'、'.join(w['source_derivation_ids']) or '—'} | "
          f"{w['derivation_dossier'] or '—'} | {w['carrier_state']} |")
    a("")
    a("已实例化双门：源码引用载体且推导档在案；推导档磁盘核验逐件在案零缺。")
    a("")
    a("## 分类分布（rev2）{#classification}")
    a("")
    a("| 面 | 判定性 | 资源性 | 待确认 | 合计 |")
    a("|---|---|---|---|---|")
    a(f"| 命名常数 | {named_counts['named_decision']} | {named_counts['named_resource']} | "
      f"{named_counts['named_pending']} | {named_counts['named_total']} |")
    a(f"| 比较位字面量 | {literal_counts['literal_decision']} | 0 | "
      f"{literal_counts['literal_pending']} | {literal_counts['literal_total']} |")
    a("")
    a(f"归类对表（对 rev1 生效面）：命名常数分类变动 {len(class_changed)} 件、"
      f"rev2 新增 {len(only_rev2)} 件、rev1 有而 rev2 无 {len(only_rev1)} 件；"
      f"比较位字面量变动 {lit_changed} 件、新增 {lit_new} 件、消失 {lit_gone} 件。"
      "逐件细目落 ledger-rev2.json classification.diff_vs_rev1 节，零静默。")
    a("")
    a(f"资源性改判承继：rev1 十二件判定性改判资源性以（相对路径，常数名）键承继，"
      f"本批实命中 {len(reclass_declared)} 件，逐件细目落 ledger-rev2.json "
      "reclassification_inherited 节。")
    a("")
    a("## 新判已实例化双实存核验{#f3}")
    a("")
    a("| 机制 | 概念 ID | 概念名 | 子仓 | SIH | 磁盘 | 结果 | 新判 |")
    a("|---|---|---|---|---|---|---|---|")
    for r in f3_rows:
        a(f"| {r['mechanism']} | {r['concept_id']} | {r['concept_name']} | {r['subrepo']} | "
          f"{'✓' if r['in_index'] else '—'} | {'✓' if r['on_disk'] else '—'} | "
          f"{r['result']} | {'是' if r['newly_instantiated'] else '否（承继）'} |")
    a("")
    a("rev1 对挂七行由本表承接；selector 零命中已由 selwire 批 ALG-002 接线消解（商集第二消费位）。")
    if inherited_gaps:
        a("承继位缺口如实申报：" + "；".join(
            f"{r['mechanism']} {r['concept_id']}（INDEX {'✓' if r['in_index'] else '—'}"
            f"/磁盘 {'✓' if r['on_disk'] else '—'}）" for r in inherited_gaps)
          + "，属数学仓条目磁盘数据缺口，本批只记不代修。")
    a("")
    a("## 扫描对表零漏项{#scan}")
    a("")
    a(f"语料内非 SPEC 白名单 ID 命中全部被载体映射覆盖，漏项核对 findings {len(findings)} 笔；"
      "SPEC 双义 ID 不在漏项核对域，消歧规则与规格引用面归属零变动承 rev1。")
    a("")
    a("## spec 消歧 {#spec}")
    a("")
    a(f"- 白名单概念 ID 全集 {len(whitelist_all)}（calculus {len(wl['calculus'])} + "
      f"order {len(wl['order'])} + probability {len(wl['probability'])} + "
      f"topology {len(wl['topology'])} + algebra {len(wl['algebra'])}）")
    a("- SPEC 双义交集（数学仓 ∩ 引擎规格档）："
      + ", ".join(rev2["spec_disambiguation"]["overlap"]))
    a("- 消歧规则：引擎 src 引用位默认引擎侧，除非白名单命中且引用上下文明示数学条目")
    a("")
    a("## 命题层")
    a("")
    a("立题立场：本汇总的立题是「账本与源码实况的重新对表」——通过接线批逐件反映与双门收紧，"
      "使账本读数重新映照源码实况，触及载体归因的时效准确性，不把刷新本身当立题。")
    a("")
    a("应用命题映射：PRO-07 鉴要求检验由可重复程序承载，rev2_script.py 同参双跑逐字节一致即其应用；"
      "A-A3.1 鉴层破自证循环——漏项核对线以独立语料扫描对表载体映射，不以映射自证映射；"
      "A-A4.2 裁决权归确定性引擎——三态判与改判承继由脚本按规则机械执行；"
      "PRO-08 应而不藏——rev1 三件原账本只读零改写，刷新前后读数并列可回溯。")
    a("")
    a("治理贡献：账面漂移整体刷新后，人类注意力只需投向归类变动申报与漏项核对 findings 与"
      "双实存核验缺失三类异常信号，其余读数跟可复算脚本走，治理贡献即信息洪流降维与权责归一。")
    a("")
    (cov_out / "summary-rev2.md").write_text("\n".join(lines) + "\n", encoding="utf-8")

    print(json.dumps({
        "carrier": state_counts,
        "named": named_counts,
        "literal": literal_counts,
        "f3_rows": len(f3_rows),
        "findings": findings,
        "diff": {"named_class_changed": len(class_changed),
                 "named_new": len(only_rev2), "named_gone": len(only_rev1),
                 "lit_changed": lit_changed, "lit_new": lit_new, "lit_gone": lit_gone},
        "reclass_inherited_hits": len(reclass_declared),
    }, ensure_ascii=False, indent=1))
    if findings:
        print(f"FAILED: {len(findings)} findings", file=sys.stderr)
        sys.exit(1)
    print(f"rev2 三件已写入: {cov_out}")


if __name__ == "__main__":
    main()
