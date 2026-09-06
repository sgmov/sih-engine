#!/usr/bin/env python3
"""sdd-v1 自携校验脚本（sddpacks-solo 批）。

对包族 manifest、五包 JSON、样例五件按各包判定面条款逐条校验。
判定面实现与包判定面条款一一对应；三态失败定位编码 missing、violation、broken_ref。
确定性输出：无时间戳无随机，双跑逐字节一致。退出码 0 全过，1 有 findings。
退役声明：文规检查器实装后本脚本退役由真机接管（manifest self_validator）。
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent

FAIL_STATES = {"missing", "violation", "broken_ref"}


def load(path):
    return Path(path).read_text(encoding="utf-8")


def section(text, name, level=2):
    """取 level 级标题 name 的节正文（到下一个同级或更高级标题）。"""
    pat = re.compile(r"^#{%d}\s+(.+?)\s*$" % level, re.M)
    heads = [(m.start(), m.group(1)) for m in pat.finditer(text)]
    for i, (pos, title) in enumerate(heads):
        if title.strip() == name:
            end = heads[i + 1][0] if i + 1 < len(heads) else len(text)
            return text[text.find("\n", pos) + 1:end]
    return None


def h1(text):
    m = re.search(r"^#\s+(.+?)\s*$", text, re.M)
    return m.group(1).strip() if m else None


class Check:
    def __init__(self):
        self.findings = []

    def fail(self, rule, state, detail):
        self.findings.append({"rule": rule, "state": state, "detail": detail})


def check_change_proposal(text, c):
    if text is None:
        c.fail("CP-001", "missing", "变更提案文件缺席或不可读")
        return
    if not h1(text):
        c.fail("CP-002", "missing", "一级标题缺席")
    order = ["问题", "意图", "范畴排除", "令源"]
    bodies = []
    for name in order:
        body = section(text, name)
        if body is None:
            c.fail("CP-003", "missing", f"缺节位：{name}")
            bodies.append(None)
        else:
            bodies.append(body.strip())
    seen = [i for i, b in enumerate(bodies) if b is not None]
    if seen != sorted(seen) and seen:
        c.fail("CP-003", "violation", "节序错位（缺节导致的空位不算错位）")
    if bodies[2] is not None and not bodies[2]:
        c.fail("CP-004", "violation", "范畴排除节空即无范畴排除不成提案")
    if bodies[3] is not None and not bodies[3]:
        c.fail("CP-005", "violation", "令源缺席即无令不开工")
    for name, body in zip(order, bodies):
        if body is not None and not body:
            c.fail("CP-006", "violation", f"空节：{name}")


def check_spec_delta(text, c):
    if text is None:
        c.fail("SD-001", "missing", "规格差分文件缺席或不可读")
        return
    if not h1(text):
        c.fail("SD-002", "missing", "一级标题缺席")
    added = section(text, "ADDED Requirements") or ""
    zero = section(text, "零增量申报")
    has_clause = "### Requirement:" in added
    has_zero = zero is not None and zero.strip()
    if not (has_clause or has_zero):
        c.fail("SD-003", "missing", "零增量未申报即不合格：零规格差分的代码变更判不合格")
    ids = re.findall(r"^### Requirement:\s*(R-\d+)", text, re.M)
    if any(not re.fullmatch(r"R-\d+", i) for i in ids):
        c.fail("SD-004", "violation", "编号缺形报行")
    if len(ids) != len(set(ids)):
        c.fail("SD-004", "violation", f"编号重号：{sorted(i for i in ids if ids.count(i) > 1)}")
    for m in re.finditer(r"^### Requirement:\s*(R-\d+)", text, re.M):
        head = text[:m.start()]
        kind = "ADDED"
        if "## MODIFIED" in head and ("## ADDED" not in head or head.rfind("## MODIFIED") > head.rfind("## ADDED")):
            kind = "MODIFIED"
        if "## REMOVED" in head and head.rfind("## REMOVED") > head.rfind("## ADDED"):
            kind = "REMOVED"
        if kind in ("MODIFIED", "REMOVED"):
            nxt = text.find("### Requirement:", m.end())
            seg = text[m.end():nxt if nxt != -1 else len(text)]
            if "归因：" not in seg:
                c.fail("SD-005", "missing", f"归因路径缺席报条款 {m.group(1)}（D-1 唯一归因）")
    for m in re.finditer(r"### Requirement:\s*(R-\d+)", text):
        nxt = text.find("### Requirement:", m.end())
        seg = text[m.end():nxt if nxt != -1 else len(text)]
        if not seg.strip() or "SHALL" not in seg:
            c.fail("SD-006", "violation", f"条款正文空或缺 SHALL 报 {m.group(1)}")


def check_scenario_list(text, c):
    if text is None:
        c.fail("SL-001", "missing", "场景清单文件缺席或不可读")
        return
    if not h1(text):
        c.fail("SL-002", "missing", "一级标题缺席")
    r_ids = re.findall(r"^### Requirement:\s*(R-\d+)", text, re.M)
    s_ids = re.findall(r"^#### Scenario:\s*(S-\d+)", text, re.M)
    if any(not re.fullmatch(r"[RS]-\d+", i) for i in r_ids + s_ids):
        c.fail("SL-003", "violation", "编号缺形报行")
    if len(r_ids) != len(set(r_ids)):
        c.fail("SL-003", "violation", f"R- 重号：{sorted(i for i in r_ids if r_ids.count(i) > 1)}")
    if len(s_ids) != len(set(s_ids)):
        c.fail("SL-003", "violation", f"S- 重号：{sorted(i for i in s_ids if s_ids.count(i) > 1)}")
    blocks = re.split(r"(?=^### Requirement:)", text, flags=re.M)
    for block in blocks:
        m = re.match(r"### Requirement:\s*(R-\d+)", block)
        if not m:
            continue
        rid = m.group(1)
        body = block[block.find("\n"):]
        if "SHALL" not in body:
            c.fail("SL-004", "violation", f"主体句缺席或缺 SHALL 报 {rid}")
        scenes = re.split(r"(?=^#### Scenario:)", body, flags=re.M)[1:]
        if not scenes:
            c.fail("SL-005", "missing", f"无场景的条款报 {rid}")
        for scene in scenes:
            sm = re.match(r"#### Scenario:\s*(S-\d+)", scene)
            sid = sm.group(1) if sm else "?"
            if "**WHEN**" not in scene:
                c.fail("SL-006", "missing", f"WHEN 缺席报 {sid}")
            if "**THEN**" not in scene:
                c.fail("SL-006", "missing", f"THEN 缺席报 {sid}")
            jm = re.search(r"^- 判据：(.+)$", scene, re.M)
            if not jm:
                c.fail("SL-007", "missing", f"判据行缺席报 {sid}")
            else:
                j = jm.group(1)
                prog = j.split("期望退出码")[0].strip()
                if not prog:
                    c.fail("SL-007", "missing", f"检验程序缺项报 {sid}")
                if not re.search(r"期望退出码\s*\d+", j):
                    c.fail("SL-007", "missing", f"期望退出码缺项或缺数字报 {sid}")
                pm = re.search(r"期望产物\s*(\S+)", j)
                if not pm or (pm.group(1) not in ("无",) and not pm.group(1)):
                    c.fail("SL-007", "missing", f"期望产物缺项报 {sid}")


def check_tech_design(text, c, r_ids):
    if text is None:
        c.fail("TD-001", "missing", "技术方案文件缺席或不可读")
        return
    if not h1(text):
        c.fail("TD-002", "missing", "一级标题缺席")
    items = re.findall(r"^- (.+)$", text, re.M)
    if not items:
        c.fail("TD-003", "missing", "技术选择缺席即 HOW 层无承载")
    linked = []
    for it in items:
        ms = re.findall(r"回链\s*(R-\d+(?:\s*,\s*R-\d+)*)", it)
        if not ms:
            c.fail("TD-004", "broken_ref", f"回链标记缺席报列表项：{it[:24]}…")
        else:
            for grp in ms:
                linked += re.findall(r"R-\d+", grp)
    for rid in linked:
        if rid not in r_ids:
            c.fail("TD-005", "broken_ref", f"回链悬空报 {rid}")


def check_task_list(text, c, r_ids, s_ids):
    if text is None:
        c.fail("TL-001", "missing", "任务清单文件缺席或不可读")
        return
    if not h1(text):
        c.fail("TL-002", "missing", "一级标题缺席")
    items = re.findall(r"^(\d+\.)\s+(.+)$", text, re.M)
    if not items:
        c.fail("TL-003", "missing", "任务项缺席")
    for num, it in items:
        refs = re.findall(r"(?:引|回链)\s*((?:[SR]-\d+)(?:\s*,\s*[SR]-\d+)*)", it)
        flat = []
        for grp in refs:
            flat += re.findall(r"[SR]-\d+", grp)
        if not flat:
            c.fail("TL-004", "broken_ref", f"引用标记缺席报任务项 {num}")
        for rid in flat:
            pool = s_ids if rid.startswith("S-") else r_ids
            if rid not in pool:
                c.fail("TL-005", "broken_ref", f"引用悬空报 {rid}")
        em = re.search(r"证据：(.+)$", it)
        if not em or not em.group(1).strip("（）() "):
            c.fail("TL-006", "missing", f"证据位缺席报任务项 {num}")


def main():
    report = {"pack_family": "sdd-v1", "verdict": "pass", "packs": {}, "fixtures": {}, "findings": []}

    # manifest 结构校验
    man_path = ROOT / "manifest.json"
    if not man_path.exists():
        report["findings"].append({"rule": "MANIFEST", "state": "missing", "detail": "manifest.json 缺席"})
    else:
        man = json.loads(load(man_path))
        for k in ("pack_family", "version", "packs", "fixtures", "self_validator"):
            if k not in man:
                report["findings"].append({"rule": "MANIFEST", "state": "missing", "detail": f"manifest 键缺席：{k}"})
        for p in man.get("packs", []):
            pp = ROOT / p
            if not pp.exists():
                report["findings"].append({"rule": "PACK-FILE", "state": "missing", "detail": f"包文件缺席：{p}"})
                continue
            d = json.loads(load(pp))
            for k in ("recognition", "rules", "provenance"):
                if k not in d:
                    report["findings"].append({"rule": "PACK-SCHEMA", "state": "missing", "detail": f"{p} 缺面：{k}"})
            rids = [r.get("id") for r in d.get("rules", [])]
            if len(rids) != len(set(rids)):
                report["findings"].append({"rule": "PACK-SCHEMA", "state": "violation", "detail": f"{p} 规则 id 重号"})
            for r in d.get("rules", []):
                st = (r.get("failure_location") or {}).get("state")
                if st not in FAIL_STATES:
                    report["findings"].append({"rule": "PACK-SCHEMA", "state": "violation", "detail": f"{p}:{r.get('id')} 失败定位态非法：{st}"})
            report["packs"][p] = {"rules": len(d.get("rules", [])), "ok": True}
        for f in man.get("fixtures", []):
            if not (ROOT / f).exists():
                report["findings"].append({"rule": "FIXTURE", "state": "missing", "detail": f"样例缺席：{f}"})
        sv = man.get("self_validator", {})
        if sv.get("script") != "validate.py" or not (ROOT / sv.get("script", "")).exists():
            report["findings"].append({"rule": "SELF-VALIDATOR", "state": "missing", "detail": "自携校验脚本缺席"})

    def rd(rel):
        p = ROOT / rel
        return load(p) if p.exists() else None

    proposal = rd("fixtures/proposal.md")
    spec_delta = rd("fixtures/spec-delta.md")
    scenarios = rd("fixtures/scenarios.md")
    tech = rd("fixtures/tech-design.md")
    tasks = rd("fixtures/tasks.md")

    c1 = Check(); check_change_proposal(proposal, c1)
    c2 = Check(); check_spec_delta(spec_delta, c2)
    r_in_scen = re.findall(r"^### Requirement:\s*(R-\d+)", scenarios or "", re.M)
    c3 = Check(); check_scenario_list(scenarios, c3)
    s_in_scen = re.findall(r"^#### Scenario:\s*(S-\d+)", scenarios or "", re.M)
    c4 = Check(); check_tech_design(tech, c4, set(r_in_scen))
    c5 = Check(); check_task_list(tasks, c5, set(r_in_scen), set(s_in_scen))

    report["fixtures"] = {
        "proposal.md": c1.findings,
        "spec-delta.md": c2.findings,
        "scenarios.md": c3.findings,
        "tech-design.md": c4.findings,
        "tasks.md": c5.findings,
    }
    report["findings"] = report["findings"] + sum(report["fixtures"].values(), [])
    if report["findings"]:
        report["verdict"] = "fail"
    out = json.dumps(report, ensure_ascii=False, indent=1, sort_keys=True) + "\n"
    sys.stdout.write(out)
    return 0 if report["verdict"] == "pass" else 1


if __name__ == "__main__":
    sys.exit(main())
