#!/usr/bin/env python3
"""mathquote-solo 枚举：扫四子仓已建条目，判哲学桥接缺「原文「」」逐字引文锚。"""
import json
import re
import sys
from pathlib import Path

MATH_ROOT = Path("/Users/moc/workspaces/SiHankor/sih-math")
SUBREPOS = ["topology", "probability", "order", "algebra"]

# 完整引文锚：原文「...」见 <文件> 正本 L<行号>
FULL_ANCHOR = re.compile(r"原文「.+?」见\s+\S+\s+正本\s+L\d+")
# 桥接节标题
BRIDGE = re.compile(r"#+\s*哲学桥接")

def main():
    out = []
    for sub in SUBREPOS:
        subdir = MATH_ROOT / sub / "entries"
        for md in sorted(subdir.glob("*.md")):
            text = md.read_text(encoding="utf-8")
            has_bridge = BRIDGE.search(text) is not None
            anchors = FULL_ANCHOR.findall(text)
            entry_id = md.stem.split("-")[0] + "-" + md.stem.split("-")[1]
            out.append({
                "subrepo": sub,
                "file": str(md),
                "entry": entry_id,
                "stem": md.stem,
                "has_bridge": has_bridge,
                "quote_anchors": len(anchors),
            })

    print(f"{'subrepo':<12}{'entry':<18}{'bridge':<7}{'anchors':<8}{'stem'}")
    for r in out:
        print(f"{r['subrepo']:<12}{r['entry']:<18}{str(r['has_bridge']):<7}{r['quote_anchors']:<8}{r['stem']}")

    missing = [r for r in out if r["has_bridge"] and r["quote_anchors"] == 0]
    missing.sort(key=lambda r: (r["subrepo"], r["stem"]))
    print("\n=== 缺引文锚(有桥接且零引文)计数:", len(missing), "===")
    for r in missing:
        print(f"{r['subrepo']}/{r['stem']}")

    with open(MATH_ROOT.parent / "sih-engine/sih/event/plan/mathquote-solo-materials/queue.json", "w", encoding="utf-8") as f:
        json.dump({
            "scope": "four sub-repos (topology/probability/order/algebra), calculus excluded",
            "total_examined": len(out),
            "bridge_total": sum(1 for r in out if r["has_bridge"]),
            "missing_anchor": [r["stem"] for r in missing],
            "missing_anchor_count": len(missing),
        }, f, ensure_ascii=False, indent=2)
    print("\nqueue.json written.")

if __name__ == "__main__":
    sys.exit(main())