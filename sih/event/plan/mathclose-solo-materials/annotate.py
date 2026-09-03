#!/usr/bin/env python3
# mathclose-solo 件一施工：147 件状态行括注 + 逐件 diff 验证
import json, re, sys, os, hashlib

M = "/Users/moc/workspaces/SiHankor/worktrees/sih-math/mathclose-solo"
E = "/Users/moc/workspaces/SiHankor/sih-engine/sih/event/plan/mathclose-solo-materials/2026-09-03-mathclose-enumerate.json"

d = json.load(open(E))
entries = d["entries"]

status_pat = re.compile(r"^(状态：已建，哲学到工程桥梁条目)。$")

report = {"total": len(entries), "changed": [], "errors": [], "skipped": []}

for rel, waves in entries.items():
    p = os.path.join(M, rel)
    if not os.path.exists(p):
        report["errors"].append((rel, "file_missing"))
        continue
    with open(p, encoding="utf-8") as f:
        text = f.read()
    lines = text.split("\n")
    # 状态行唯一性
    st_idx = [i for i, l in enumerate(lines) if l.startswith("状态：")]
    if len(st_idx) != 1:
        report["errors"].append((rel, f"status_lines={len(st_idx)}"))
        continue
    idx = st_idx[0]
    line = lines[idx]
    m = status_pat.match(line)
    wave_label = "加".join(waves)
    if m:
        # 无括注基态：已建，哲学到工程桥梁条目。
        new_status = f"状态：已建，哲学到工程桥梁条目（{wave_label} 增补）。"
    else:
        # 既有括注形态：已建，哲学到工程桥梁条目，复归段补强波簇X 增补。
        # 用户 2026-09-03 裁定：并置保留两者，追加本批波名括注，不覆盖既有簇括注。
        legacy_pat = re.compile(
            r"^(状态：已建，哲学到工程桥梁条目，复归段补强波簇([ABC]) 增补)。$")
        lm = legacy_pat.match(line)
        if not lm:
            report["errors"].append((rel, f"status_alt: {line.rstrip()}"))
            continue
        cluster = lm.group(2)
        # 保留既有「复归段补强波簇X 增补」信息，并置追加本批波名括注
        new_status = f"状态：已建，哲学到工程桥梁条目，复归段补强波簇{cluster} 增补（{wave_label} 增补）。"
    lines[idx] = new_status
    new_text = "\n".join(lines)
    with open(p, "w", encoding="utf-8") as f:
        f.write(new_text)

    # diff 验证：只动状态行一行，正文零改
    old_lines = text.split("\n")
    diff_lines = [i for i in range(max(len(old_lines), len(lines)))
                  if i >= len(old_lines) or i >= len(lines) or old_lines[i] != lines[i]]
    report["changed"].append({
        "rel": rel,
        "waves": waves,
        "wave_label": wave_label,
        "diff_line_indices": diff_lines,
        "olds": [text.split("\n")[i] for i in diff_lines],
        "news": [new_text.split("\n")[i] for i in diff_lines],
        "sha_old": hashlib.sha256(text.encode()).hexdigest()[:12],
        "sha_new": hashlib.sha256(new_text.encode()).hexdigest()[:12],
    })

print(json.dumps(report, ensure_ascii=False, indent=2))