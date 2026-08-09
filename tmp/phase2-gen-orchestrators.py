#!/usr/bin/env python3
"""Generate 8 orchestrator prompt files for Phase 2 full-scale experiment.

One per type: governance, research, draft, knowledge, spec, decision, proposal, design.
Each orchestrator launches N×4 isolated subagents for its type.
"""

import json
from pathlib import Path

REPO = Path("/Users/moc/workspaces/SiHankor")
CL_BASE = REPO / "sih-engine/sih/event/experiment/phase2-full/checklists"
OUT_BASE = REPO / "sih-engine/sih/event/experiment/phase2-full"
PROMPT_DIR = REPO / "sih-engine/tmp/prompts/phase2"
JUDGE_PKG = "/Users/moc/workspaces/SiHankor/sih-engine/tmp/experiment-phase2-judge-package.md"

# 按规模从小到大排序，烟雾测试优先
TYPE_ORDER = ["governance", "research", "draft", "knowledge", "spec", "decision", "proposal", "design"]

TYPE_DOC_COUNT = {
    "governance": 2,
    "research": 8,
    "draft": 28,
    "knowledge": 46,
    "spec": 35,
    "decision": 36,
    "proposal": 50,
    "design": 58,
}


def gen_orchestrator(type_name):
    """Generate one orchestrator prompt for a type."""
    # 收集该类型的所有 checklist
    prefix = type_name + "__"
    checklists = sorted(CL_BASE.glob(f"{prefix}*.json"))

    lines = []
    lines.append(f"你是语义验证实验阶段二全量的编排者，负责 {type_name} 类型的 {len(checklists)} 份文档。")
    lines.append("你不亲自做语义判断，你只负责启动隔离子代理并验证产出完整性。")
    lines.append("")
    lines.append("## 任务规模")
    lines.append("")
    total_sessions = len(checklists) * 4
    lines.append(f"{len(checklists)} 份文档，每份 4 个采样序号即 N 等于 4，共 {total_sessions} 个子代理。")
    lines.append(f"你每次启动 4 个子代理并行，等全部完成后再启动下一批，共约 {len(checklists)} 批。")
    lines.append("")
    lines.append("## 隔离要求")
    lines.append("")
    lines.append("每个子代理必须是独立 session，子代理之间零状态共享。")
    lines.append("你不把一个子代理的产出传给另一个子代理。")
    lines.append("你不读取子代理产出的 jsonl 文件内容。你只通过文件是否存在且非空来判断子代理是否完成。")
    lines.append("")
    lines.append("## 子代理提示词模板")
    lines.append("")
    lines.append("每个子代理的提示词由两部分构成。")
    lines.append("")
    lines.append(f"第一部分：")
    lines.append(f"读取文件 {JUDGE_PKG}。这是你的完整判断协议，定义了 3 项语义检查项、判断方法、输出格式。你严格按协议执行。")
    lines.append("")
    lines.append("第二部分，把大括号内变量替换为具体值：")
    lines.append(f"你的检查对象清单在 {CL_BASE}/{{checklist_file}}，你读取它获取检查对象。清单的承接语义对应性对象含 claim 字段即承接声明上下文与 upstream_overview 字段即上游文档概览节全文，你不需要自行查找上游文档。你负责检查的文档路径是 {{doc_path}}。你的判断记录输出到 {{output_path}}。你的采样序号是 {{seq}}，全部记录的 sample_seq 填 {{seq}}。")
    lines.append("")
    lines.append("## 平局触发逻辑")
    lines.append("")
    lines.append("本实验采用 N 等于 4。4 个 session 的多数投票规则如下。")
    lines.append("4 票一致或 3 比 1，该对象自动判定，无需第 5 次。")
    lines.append("2 比 2 平局，你需要为该文档加采第 5 个 session，seq 填 5，output_path 的文件名用 seq5。")
    lines.append("但平局检测需要在 4 个 session 的产出全部完成后，比较判断结果。")
    lines.append("由于你不读取 jsonl 内容，平局检测由后处理脚本执行，不在编排阶段处理。")
    lines.append("你的编排只负责：为每份文档启动 4 个 session，验证 4 个文件都存在且非空。")
    lines.append("")
    lines.append(f"## {type_name} 类型 {len(checklists)} 份文档清单")
    lines.append("")

    for idx, cl_file in enumerate(checklists, 1):
        cl = json.loads(cl_file.read_text())
        doc_id = cl["doc_id"]
        doc_path = cl["doc_path"]
        checklist_file = cl_file.name
        meta = cl["meta"]
        n_objects = meta["n_correspondence"] + meta["n_self_consistency"] + meta["n_classification"]

        lines.append(f"### 文档 {idx}: {doc_id}")
        lines.append("")
        lines.append(f"doc_path: {doc_path}")
        lines.append(f"checklist_file: {checklist_file}")
        lines.append(f"检查对象数: {n_objects} (对应性={meta['n_correspondence']}, 自洽性={meta['n_self_consistency']}, 归位={meta['n_classification']})")
        lines.append("")
        lines.append("4 个子代理参数:")
        lines.append("")

        out_type_dir = OUT_BASE / type_name / doc_id
        for seq in range(1, 5):
            out_path = out_type_dir / f"{doc_id}-seq{seq}.jsonl"
            lines.append(f"seq {seq}: output_path {out_path}")
        lines.append("")

    lines.append("## 完整性验证")
    lines.append("")
    lines.append(f"全部 {total_sessions} 个子代理完成后，你检查每份文档的 4 个 seq 文件是否存在且非空。")
    lines.append("缺哪个重启哪个，按 4 个一批。全齐后报告完成。")
    lines.append("")
    lines.append("## 你不做的事")
    lines.append("")
    lines.append("你不亲自做语义判断。你不读取子代理的 jsonl 产出内容。你不修改任务包或清单文件。")
    lines.append("你不产出治理决策。你只编排子代理与验证文件完整性。")
    lines.append("")

    return "\n".join(lines)


def main():
    PROMPT_DIR.mkdir(parents=True, exist_ok=True)

    for type_name in TYPE_ORDER:
        content = gen_orchestrator(type_name)
        out_file = PROMPT_DIR / f"orchestrator-{type_name}.txt"
        out_file.write_text(content, encoding="utf-8")

        # 统计
        prefix = type_name + "__"
        n_docs = len(list(CL_BASE.glob(f"{prefix}*.json")))
        n_sessions = n_docs * 4
        print(f"  {type_name}: {n_docs} docs, {n_sessions} sessions → {out_file.name}")

    print(f"\n8 份编排提示词已生成到 {PROMPT_DIR}/")


if __name__ == "__main__":
    main()
