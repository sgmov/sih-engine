#!/usr/bin/env python3
# mathclose-solo ask3 三锚程序切片：从哲学原文逐字节子串提取引文
import json, sys, os

ROOT = "/Users/moc/workspaces/SiHankor"
SPEC = [
    {
        "anchor_seq": 1,
        "source": "sih-philosophy/emanation/proodos/06-on-canon.md",
        "sub": "由松到紧，不可由紧到松",
    },
    {
        "anchor_seq": 2,
        "source": "sih-philosophy/emanation/proodos/07-on-assay.md",
        "sub": "镜的特点是纯粹映照——它不主动投射",
    },
    {
        "anchor_seq": 3,
        "source": "sih-philosophy/emanation/proodos/08-on-settle.md",
        "sub": "应而不藏，应辨当下，应几未来",
    },
]

out = []
for s in SPEC:
    p = os.path.join(ROOT, s["source"])
    text = open(p, encoding="utf-8").read()
    # 找到整句所在行：从 sub 起点对齐
    if s["sub"] not in text:
        raise SystemExit(f"SUBSTRING NOT FOUND in {s['source']}: {s['sub']}")
    idx = text.index(s["sub"])
    line_start = text.rfind("\n", 0, idx) + 1
    line_end = text.find("\n", idx)
    if line_end == -1:
        line_end = len(text)
    full_line = text[line_start:line_end]
    # 逐行定位行号（从 1 起）
    line_no = text[:idx].count("\n") + 1
    out.append({
        "anchor_seq": s["anchor_seq"],
        "source": s["source"],
        "substring_is_present": True,
        "line_no": line_no,
        "full_line": full_line,
    })
    # 校验 apple-to-apple：full_line 必须含 sub
    if s["sub"] not in full_line:
        raise SystemExit(f"SUB NOT WITHIN LINE: {s['sub']}")

print(json.dumps(out, ensure_ascii=False, indent=2))