#!/usr/bin/env python3
"""critsweep registry.json 构建器：五判据文本自 GOV-002 §退出标准程序切片逐字节内嵌，禁手打。

运行位：critsweep-solo 批 tools 工地。输出 registry.json 落 critsweep/ 工具目录。
每条派生依据（token_scope 窄化理由与证据指针）同脚本内声明，镜像文落批材料。
"""
import json
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
GOV = ROOT / "sih-engine/doc/governance/GOV-002-mainline-lock-v1.md"
OUT = ROOT / "worktrees/sih-tools/critsweep-solo/critsweep/registry.json"

# GOV-002-mainline-lock-v1.md §退出标准 五判据行号（v2.4 现版实文）
BULLETS = [23, 24, 25, 26, 27]


def slice_line(lineno: int) -> str:
    return GOV.read_text(encoding="utf-8").splitlines()[lineno - 1]


texts = {ln: slice_line(ln) for ln in BULLETS}
for ln, val in texts.items():
    assert val.startswith("- "), f"line {ln} not a bullet"

registry = {
    "registry": "critsweep",
    "version": "1.0.0",
    "source": {
        "doc": "sih-engine/doc/governance/GOV-002-mainline-lock-v1.md",
        "section": "退出标准（v2.4 现版）",
        "line_span": "23-27",
        "note": "五判据文本为该节五行程序切片逐字节内嵌，构建器 make_registry.py 随批落档可重放",
    },
    "semantics": {
        "status_kinds": {
            "evidence": "达成以证据指针在档为准：证据面（文件在场与文本指纹与链事件）全过即 achieved，任一缺席即按活动面回落并降级可见",
            "activity": "在飞与沉底按活动回算：last_hit 为近窗命名空间命中最新日，gap 为 at 减 last_hit 日数；零批记录自 registered_at 起计 gap；gap 严格大于阈值即 sunk，gap 不大于阈值即 in_flight",
        },
        "namespace_fields": ["record_path", "report_path", "package"],
        "namespace_note": "活动扫描只认批名命名空间字段，禁全文散文匹配；散文引用属设计引用非程序活动（pk-044 出泊裁定对照例）",
        "threshold_default_days": 3,
        "threshold_note": "沉底阈值三日：gap 超过三日即沉底，恰等于三日仍算在飞；--threshold 可覆调",
        "window_default_days": 10,
    },
    "criteria": [
        {
            "id": "GOV2-C1-leaseopt",
            "seq": 1,
            "title": "leaseopt 线级验收达成",
            "criterion_text": texts[23],
            "status_kind": "evidence",
            "evidence": {
                "files": [
                    {"path": "sih-engine/doc/governance/SETTLEMENT-LEASEOPT-2026-09-05.md"}
                ],
                "chain": [
                    {"event_type": "parking_exited", "entry_id": "pk-045", "disposition": "promoted"}
                ],
            },
            "token_scope": ["leaseopt"],
            "registered_at": "2026-09-05",
            "derivation": "达成态承 GOV-002 v2.4 追记：三项判据全数在档，结算单 SETTLEMENT-LEASEOPT-2026-09-05.md 承载，pk-045 出泊事件在链；令牌 leaseopt 取线程序族批名共根，宁窄勿宽",
        },
        {
            "id": "GOV2-C2-viewline",
            "seq": 2,
            "title": "视图组件在役",
            "criterion_text": texts[24],
            "status_kind": "activity",
            "token_scope": ["viewline"],
            "registered_at": "2026-09-04",
            "derivation": "令牌取线程序包 viewline-line-v1.md 与 viewline-solo 族批名共根 viewline；viewer 一词泛化过宽（散文易撞）不收，宁窄勿宽；2026-09-08 链面核实命名空间命中仅 2026-09-05 viewline-solo 批",
        },
        {
            "id": "GOV2-C3-measure-poly",
            "seq": 3,
            "title": "measure-poly 程序级验收达成",
            "criterion_text": texts[25],
            "status_kind": "activity",
            "token_scope": ["measure-poly", "measurepoly"],
            "registered_at": "2026-09-04",
            "derivation": "程序源 measure-poly-rev1-progdoc.md 立约 2026-09-04；令牌取程序名连字与无连字两形；pk-044 出泊裁定（2026-09-07）散文明引 measure-poly-rev1 §7 属设计引用非程序活动，全文匹配会误计活动，故禁散文匹配并设对照读数；2026-09-08 链面核实命名空间零批记录，gap 自 registered_at 起算",
        },
        {
            "id": "GOV2-C4-math-attribution",
            "seq": 4,
            "title": "数学归因长尾收口",
            "criterion_text": texts[26],
            "status_kind": "activity",
            "token_scope": ["pk-053", "mathclose", "mathreg"],
            "registered_at": "2026-09-04",
            "derivation": "判据本体挂 pk-053 冻结件清账；令牌自链面核实取三件：泊位号 pk-053 与清账批名 mathclose-solo 与账本批名 mathreg-solo 之共根；泛数学词（math 前缀族）过宽不收，宁窄勿宽；2026-09-08 链面核实 last_hit 为 2026-09-03 mathclose-solo",
        },
        {
            "id": "GOV2-C5-lease-line",
            "seq": 5,
            "title": "租约修复升级线级验收达成",
            "criterion_text": texts[27],
            "status_kind": "evidence",
            "evidence": {
                "files": [
                    {"path": "sih-engine/doc/governance/SETTLEMENT-LEASEUP-2026-09-07.md"},
                    {"path": "sih-tools/lease/CONTRACT.md", "contains": "修订四十一"},
                    {"path": "sih-tools/lease/CONTRACT.md", "contains": "修订四十二"},
                    {"path": "sih-tools/lease/CONTRACT.md", "contains": "修订四十三"},
                ],
                "chain": [
                    {"event_type": "parking_exited", "entry_id": "pk-072", "disposition": "promoted"}
                ],
            },
            "token_scope": ["leaseup"],
            "registered_at": "2026-09-06",
            "derivation": "达成态承 GOV-002 v2.4 追记：五判据全数在档即 CONTRACT 修订四十一至四十三（1.28.0 至 1.30.0）在役承载、线程序包 leaseup-line-v1.md 迁 event/plan 归档、结算单 SETTLEMENT-LEASEUP-2026-09-07.md；pk-072 出泊链事件由 leaseupclose-solo 补落在链；令牌 leaseup 取线族批名共根，与 leaseopt 互斥不重叠，宁窄勿宽",
        },
    ],
}

OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(registry, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(f"written {OUT}")
for c in registry["criteria"]:
    print(c["id"], "|", c["criterion_text"][:40], "| tokens:", c["token_scope"])
