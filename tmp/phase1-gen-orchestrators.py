#!/usr/bin/env python3
"""Generate per-type orchestrator prompts for Phase 1 Round 1.

Each orchestrator handles one document type from the old repo.
7 types: design, decision, draft, proposal, governance, knowledge, spec, research
(skips types with 0 docs or empty-only docs).

Each orchestrator prompt is self-contained:
- Reads the judge package
- Has a checklist lookup pattern
- Starts N=4 sessions per doc, 4 parallel, batches of 4
- Tie-break logic
- Completeness check

Output: phase1-round1/orchestrators/{type}.txt
"""

import json
from pathlib import Path

REPO = Path("/Users/moc/workspaces/SiHankor")
CHECKLIST_DIR = REPO / "sih-engine/sih/event/experiment/phase1-round1/checklists"
JUDGE_PKG = REPO / "sih-engine/tmp/experiment-phase0-v3-judge-package.md"
OUTPUT_BASE = REPO / "sih-engine/sih/event/experiment/phase1-round1"
PROMPT_DIR = REPO / "sih-engine/tmp/prompts/phase1"

TYPES = [
    ("design", 58),
    ("decision", 36),
    ("draft", 28),
    ("proposal", 50),
    ("governance", 2),
    ("knowledge", 46),
    ("spec", 35),
    ("research", 8),
]


def load_checklists(doc_type):
    """Load all checklists for a given type."""
    prefix = f"{doc_type}__"
    files = sorted(CHECKLIST_DIR.glob(f"{prefix}*.json"))
    docs = []
    for f in files:
        data = json.loads(f.read_text(encoding="utf-8"))
        n_objs = (
            data["meta"]["n_refs"]
            + data["meta"]["n_conditions"]
            + data["meta"]["n_labels"]
            + data["meta"]["n_terms"]
        )
        if n_objs == 0:
            continue  # skip empty checklists
        docs.append(data)
    return docs


def generate_prompt(doc_type, docs):
    n_sessions = len(docs) * 4
    n_batches = (n_sessions + 3) // 4  # ceil division

    lines = []
    lines.append(f"你是语义验证实验阶段一轮一的编排者，负责 {doc_type} 类型的 {len(docs)} 份文档。")
    lines.append(f"你不亲自做语义判断，你只负责启动隔离子代理并验证产出完整性。")
    lines.append(f"")
    lines.append(f"## 任务规模")
    lines.append(f"")
    lines.append(f"{len(docs)} 份文档，每份 4 个采样序号即 N 等于 4，共 {n_sessions} 个子代理。")
    lines.append(f"你每次启动 4 个子代理并行，等全部完成后再启动下一批，共约 {n_batches} 批。")
    lines.append(f"")
    lines.append(f"## 隔离要求")
    lines.append(f"")
    lines.append(f"每个子代理必须是独立 session，子代理之间零状态共享。")
    lines.append(f"你不把一个子代理的产出传给另一个子代理。")
    lines.append(f"你不读取子代理产出的 jsonl 文件内容。你只通过文件是否存在且非空来判断子代理是否完成。")
    lines.append(f"")
    lines.append(f"## 子代理提示词模板")
    lines.append(f"")
    lines.append(f"每个子代理的提示词由两部分构成。")
    lines.append(f"")
    lines.append(f"第一部分：")
    lines.append(f"读取文件 {JUDGE_PKG}。这是你的完整判断协议，定义了 4 项检查项、判断方法、输出格式。你严格按协议执行。")
    lines.append(f"")
    lines.append(f"第二部分，把大括号内变量替换为具体值：")
    lines.append(f"你的检查对象清单在 {CHECKLIST_DIR}/{{checklist_file}}，你读取它获取检查对象。你负责检查的文档路径是 {{doc_path}}。你的判断记录输出到 {{output_path}}。你的采样序号是 {{seq}}，全部记录的 sample_seq 填 {{seq}}。")
    lines.append(f"")
    lines.append(f"## 平局触发逻辑")
    lines.append(f"")
    lines.append(f"本实验采用 N 等于 4。4 个 session 的多数投票规则如下。")
    lines.append(f"4 票一致或 3 比 1，该对象自动判定，无需第 5 次。")
    lines.append(f"2 比 2 平局，你需要为该文档加采第 5 个 session，seq 填 5，output_path 的文件名用 seq5。")
    lines.append(f"")
    lines.append(f"但平局检测需要在 4 个 session 的产出全部完成后，比较判断结果。")
    lines.append(f"由于你不读取 jsonl 内容，平局检测由后处理脚本执行，不在编排阶段处理。")
    lines.append(f"你的编排只负责：为每份文档启动 4 个 session，验证 4 个文件都存在且非空。")
    lines.append(f"")
    lines.append(f"## {doc_type} 类型 {len(docs)} 份文档清单")
    lines.append(f"")

    for i, doc in enumerate(docs):
        doc_id = doc["doc_id"]
        doc_path = doc["doc_path"]
        checklist_file = f"{doc_id}.json"
        n_objs = (
            doc["meta"]["n_refs"]
            + doc["meta"]["n_conditions"]
            + doc["meta"]["n_labels"]
            + doc["meta"]["n_terms"]
        )

        # Output directory per doc
        out_dir = OUTPUT_BASE / doc_type / doc_id

        lines.append(f"### 文档 {i+1}: {doc_id}")
        lines.append(f"")
        lines.append(f"doc_path: {doc_path}")
        lines.append(f"checklist_file: {checklist_file}")
        lines.append(f"检查对象数: {n_objs}")
        lines.append(f"")
        lines.append(f"4 个子代理参数:")
        lines.append(f"")
        for seq in range(1, 5):
            out_path = out_dir / f"{doc_id}-seq{seq}.jsonl"
            lines.append(f"seq {seq}: output_path {out_path}")
        lines.append(f"")

    lines.append(f"## 完整性验证")
    lines.append(f"")
    lines.append(f"全部 {n_sessions} 个子代理完成后，你检查每份文档的 4 个 seq 文件是否存在且非空。")
    lines.append(f"缺哪个重启哪个，按 4 个一批。全齐后报告完成。")
    lines.append(f"")
    lines.append(f"## 你不做的事")
    lines.append(f"")
    lines.append(f"你不亲自做语义判断。你不读取子代理的 jsonl 产出内容。你不修改任务包或清单文件。")
    lines.append(f"你不产出治理决策。你只编排子代理与验证文件完整性。")

    return "\n".join(lines)


def main():
    PROMPT_DIR.mkdir(parents=True, exist_ok=True)

    total_sessions = 0
    for doc_type, expected_count in TYPES:
        docs = load_checklists(doc_type)
        if not docs:
            print(f"{doc_type}: 跳过（无有效文档）")
            continue

        prompt = generate_prompt(doc_type, docs)
        prompt_file = PROMPT_DIR / f"orchestrator-{doc_type}.txt"
        prompt_file.write_text(prompt, encoding="utf-8")

        n_sessions = len(docs) * 4
        total_sessions += n_sessions
        print(f"{doc_type}: {len(docs)} 份文档, {n_sessions} session, 提示词 {len(prompt)} 字符")

    print(f"\n总计: {total_sessions} session")
    print(f"提示词目录: {PROMPT_DIR}/")


if __name__ == "__main__":
    main()
