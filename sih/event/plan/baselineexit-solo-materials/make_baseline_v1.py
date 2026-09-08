#!/usr/bin/env python3
"""BASELINE-v1.md 生成器：正文自 AGENTS.md 前像程序切片逐字节迁入，零手打，禁改语义。"""
import hashlib
import json
import sys
from pathlib import Path

MAT = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-engine/baselineexit-solo/sih/event/plan/baselineexit-solo-materials")
PRE = MAT / "AGENTS-preimage-2026-09-08.md"
OUT = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-engine/baselineexit-solo/doc/governance/BASELINE-v1.md")

pre_lines = PRE.read_text(encoding="utf-8").splitlines()
i1 = pre_lines.index("## 工程基线与禁止条款 {#engineering-baseline}")
i2 = pre_lines.index("## 产出前自检核心 {#pre-output-self-check}")
body = pre_lines[i1 + 1:i2]
while body and not body[0].strip():
    body.pop(0)
while body and not body[-1].strip():
    body.pop()

header = """# 工程基线 v1：sih-engine 工程层操作约束正典

本文件是 sih-engine 工程层操作约束的正典档，承 pk-070 出泊由 AGENTS.md「工程基线与禁止条款」节整体迁入，甲案落据即 m-hygp070-1 九发 stable_clear 终签 f6263808。正典在此，AGENTS.md 同名节改指针为入口投影。正文自前像程序切片逐字节迁入零语义改动，diff 自证随批材料可复算。

## 概览 {#overview}

- 本节承载 sih-engine 的工程层操作约束，每条约束标注来源类型，区分已有哲学支撑与待哲学锚定，来源类型五类::[基线正文](#baseline-body)
- 工程基线五条即确定性程序与信息洪流与异常信号与可验证性与减少 LLM 参与::[工程基线五条](#engineering-baseline-five)
- 工程禁止条款五条由工程基线与失败复盘直接推导，agent 必须遵守::[工程禁止条款](#engineering-prohibitions)
- 待哲学锚定条目的地位两类比照，同时是哲学仓后续演化的候选输入::[待哲学锚定条目的地位](#pending-philosophy-anchor)
- v1 于 2026-09-08 承 pk-070 出泊迁入，三层固定即 git 版本化加哈希入 trail 加结果档在链::[版本](#version)

## 基线正文 {#baseline-body}

"""

version = """

## 版本 {#version}

v1 于 2026-09-08 承 pk-070 出泊迁入即本文件首版。令源两笔照录：用户 2026-09-06 裁定原话「这个工程基线应该是司衡引擎的运行机制的基石，不应该放在AGENTS里。你这个记一笔，未来要清的。」与用户 2026-09-08 测量令「PK-070 过得一」经 m-hygp070-1 九发 stable_clear 终签 f6263808 裁甲案即迁入 sih-engine/doc/governance/ 治理决策档族新立 BASELINE 正典档，AGENTS.md 改指针，乙向界族与丙宪法档不采。正文自 AGENTS.md 同名节程序切片逐字迁入零语义改动，diff 自证在批材料 migration-diff-proof.json 可复算。三层固定即本文件 git 版本化随批 settle 归并、内容哈希入 trail 认证、出入泊笔与结果档在链。
"""

doc = header + "\n".join(body) + "\n" + version
OUT.write_text(doc, encoding="utf-8")

body_sha = hashlib.sha256(("\n".join(body) + "\n").encode("utf-8")).hexdigest()
doc_sha = hashlib.sha256(doc.encode("utf-8")).hexdigest()
proof = {
    "source": "sih-engine/sih/event/plan/baselineexit-solo-materials/AGENTS-preimage-2026-09-08.md",
    "source_section": "## 工程基线与禁止条款 {#engineering-baseline} 至 ## 产出前自检核心 前exclusive",
    "body_lines": len(body),
    "body_sha256": body_sha,
    "doc_sha256": doc_sha,
    "method": "程序切片逐字节迁入即 make_baseline_v1.py，正文零手打零改写",
    "structural_additions": ["H1 题行", "导言节", "概览锚节", "基线正文包装节标题", "版本节"],
    "semantic_change": "none",
}
(MAT / "migration-diff-proof.json").write_text(
    json.dumps(proof, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(f"written {OUT}")
print(f"body_lines={len(body)} body_sha256={body_sha[:16]} doc_sha256={doc_sha[:16]}")
