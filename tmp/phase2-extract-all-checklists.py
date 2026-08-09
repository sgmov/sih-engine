#!/usr/bin/env python3
"""Full-scale checklist extraction for Phase 2.

Scans all .md documents in old repo, generates 3 semantic check items per doc:
1. 承接语义对应性: refs + claim context (800 char) + upstream overview (packed)
2. 论证自洽性: doc core claim (from overview section)
3. 归位充分性: doc claimed type (skip draft)

Output: phase2-full/checklists/{relative_path_sanitized}.json

Usage:
    python3 phase2-extract-all-checklists.py
"""

import json
import re
import sys
from pathlib import Path
from collections import defaultdict

REPO = Path("/Users/moc/workspaces/SiHankor")
OLD_DOC = REPO / "sihankor/doc"
OUT_BASE = REPO / "sih-engine/sih/event/experiment/phase2-full/checklists"

REF_PATTERN = re.compile(r'\b(PRO|DEC|DES|GOV|KNOW|SPEC)-(\d{3})\b')

# Type prefix to directory mapping
PREFIX_TO_DIR = {
    "DES": "design",
    "DEC": "decision",
    "PRO": "proposal",
    "GOV": "governance",
    "KNOW": "knowledge",
    "SPEC": "spec",
}

# Type labels for 归位充分性
DOC_TYPE_MAP = {
    "design": "design",
    "decision": "decision",
    "draft": "draft",
    "proposal": "proposal",
    "governance": "governance",
    "knowledge": "knowledge",
    "spec": "spec",
    "research": "draft",  # research docs are process outputs, treat as draft
}


def find_section(text, heading_keywords):
    """Find a section by heading keyword, return (line_index, content_until_next_heading)."""
    lines = text.split("\n")
    for i, line in enumerate(lines):
        if line.startswith("#"):
            for kw in heading_keywords:
                if kw in line:
                    level = len(line) - len(line.lstrip("#"))
                    end = len(lines)
                    for j in range(i + 1, len(lines)):
                        if lines[j].startswith("#"):
                            next_level = len(lines[j]) - len(lines[j].lstrip("#"))
                            if next_level <= level:
                                end = j
                                break
                    return i, "\n".join(lines[i:end])
    return None, None


def get_overview(text):
    """Extract the 概览 section from a document."""
    _, section = find_section(text, ["概览", "overview"])
    return section if section else "(无概览节)"


def get_core_claim(text):
    """Extract core claim from overview section."""
    overview = get_overview(text)
    if overview == "(无概览节)":
        lines = text.split("\n")
        for i, line in enumerate(lines):
            if line.startswith("# ") and i + 2 < len(lines):
                return lines[i + 2] if lines[i + 2].strip() else "(无法提取核心主张)"
        return "(无法提取核心主张)"
    lines = overview.split("\n")
    claim_lines = []
    for line in lines[1:]:
        if line.strip() and not line.startswith("#"):
            claim_lines.append(line.strip())
        if len(claim_lines) >= 3:
            break
    return " ".join(claim_lines) if claim_lines else overview[:200]


def find_upstream_doc(ref_prefix, ref_num, claim=None):
    """Find the upstream document file by prefix and number.

    旧仓命名不统一，两种格式都支持:
    - 带前缀: DES-015-engine-state-machine-implementation.md
    - 不带前缀: 015-engine-state-machine-implementation.md
    也支持目录形式如 DES-001-document-format/

    子文件匹配修复: 当上游是目录且 claim 提到子文件名（如 knowledge-spec）时，
    优先打包该子文件而非默认的 general.md。
    """
    doc_type = PREFIX_TO_DIR.get(ref_prefix)
    if not doc_type:
        return None

    type_dir = OLD_DOC / doc_type
    if not type_dir.exists():
        return None

    # 先试带前缀格式 {PREFIX}-{num}-*.md
    prefixed_pattern = f"{ref_prefix}-{ref_num}-"
    prefixed_matches = [m for m in type_dir.glob(f"{prefixed_pattern}*.md") if m.is_file()]
    if prefixed_matches:
        return prefixed_matches[0]

    # 再试不带前缀格式 {num}-*.md
    plain_pattern = f"{ref_num}-"
    plain_matches = [m for m in type_dir.glob(f"{plain_pattern}*.md") if m.is_file()]
    if plain_matches:
        return plain_matches[0]

    # 目录形式: {PREFIX}-{num}-*/ 或 {num}-*/
    for pat in [f"{ref_prefix}-{ref_num}-*/", f"{ref_num}-*/"]:
        dir_matches = list(type_dir.glob(pat))
        if dir_matches:
            # 子文件匹配修复: 从 claim 中解析子文件名
            # claim 写法如 "DES-001 knowledge-spec"，子文件名为 knowledge-spec.md
            subfile_match = None
            if claim:
                import re as _re
                # 提取 {ref}-{num} 之后紧跟的单词作为子文件名候选
                ref_token = f"{ref_prefix}-{ref_num}"
                subfile_pattern = _re.compile(
                    rf'{_re.escape(ref_token)}\s+([a-z][-a-z0-9]*)', _re.IGNORECASE
                )
                m = subfile_pattern.search(claim)
                if m:
                    subfile_slug = m.group(1)
                    for d in dir_matches:
                        candidate = d / f"{subfile_slug}.md"
                        if candidate.exists():
                            subfile_match = candidate
                            break
            if subfile_match:
                return subfile_match

            # 回退到默认顺序
            for d in dir_matches:
                for candidate in ["general.md", "design.md", "index.md"]:
                    f = d / candidate
                    if f.exists():
                        return f
            md_files = []
            for d in dir_matches:
                md_files.extend(list(d.glob("*.md")))
            if md_files:
                return md_files[0]

    return None


def extract_refs_with_claims(text, doc_id):
    """Extract refs with their surrounding context (claim)."""
    refs = []
    seen = set()
    lines = text.split("\n")

    for i, line in enumerate(lines):
        matches = REF_PATTERN.findall(line)
        for prefix, num in matches:
            ref = f"{prefix}-{num}"
            if ref == doc_id or ref in seen:
                continue
            seen.add(ref)

            # Get context: the line itself + 1 line before
            # 截断上限 800 字符，与 upstream_overview 的 1000 上限对齐
            # 历史问题：原 200 截断导致 DES-058 长承接段丢失工程仓承接清单
            context_parts = []
            if i > 0:
                context_parts.append(lines[i - 1].strip())
            context_parts.append(line.strip())
            claim = " ".join(context_parts)[:800]

            refs.append({"target": ref, "claim": claim})

    return refs


def extract_semantic_correspondence(text, doc_id):
    """Extract refs with upstream overviews packed."""
    refs = extract_refs_with_claims(text, doc_id)
    result = []
    for ref_data in refs:
        ref = ref_data["target"]
        prefix = ref.split("-")[0]
        num = ref.split("-")[1]

        upstream_file = find_upstream_doc(prefix, num, claim=ref_data["claim"])
        if upstream_file:
            upstream_text = upstream_file.read_text(encoding="utf-8")
            upstream_overview = get_overview(upstream_text)
            if len(upstream_overview) > 1000:
                upstream_overview = upstream_overview[:1000] + "...(截断)"
        else:
            upstream_overview = "(上游文档未找到)"

        result.append({
            "target": ref,
            "claim": ref_data["claim"],
            "upstream_overview": upstream_overview,
        })
    return result


def extract_self_consistency(text):
    """Extract core claim for self-consistency check."""
    core_claim = get_core_claim(text)
    return [{"target": "文档自身", "core_claim": core_claim}]


def extract_classification_adequacy(text, doc_path):
    """Extract claimed type for classification check."""
    rel = doc_path.relative_to(OLD_DOC)
    doc_type = rel.parts[0] if rel.parts else "unknown"
    claimed_type = DOC_TYPE_MAP.get(doc_type, doc_type)

    if claimed_type == "draft":
        return []

    return [{"target": "文档自身", "claimed_type": claimed_type}]


def sanitize_path(rel_path):
    """Convert relative path to safe filename."""
    return rel_path.replace("/", "__").replace(".md", "")


def main():
    OUT_BASE.mkdir(parents=True, exist_ok=True)

    all_docs = sorted(OLD_DOC.rglob("*.md"))
    print(f"发现 {len(all_docs)} 份文档")

    stats = {
        "total": 0,
        "total_correspondence": 0,
        "total_self_consistency": 0,
        "total_classification": 0,
        "total_upstream_found": 0,
        "empty_checklists": 0,
    }
    by_type = defaultdict(lambda: {"docs": 0, "corr": 0, "self": 0, "class": 0})

    for doc_path in all_docs:
        rel = doc_path.relative_to(OLD_DOC)
        text = doc_path.read_text(encoding="utf-8")
        doc_id = sanitize_path(str(rel))
        doc_type = rel.parts[0] if rel.parts else "unknown"

        correspondence = extract_semantic_correspondence(text, doc_id)
        self_consistency = extract_self_consistency(text)
        classification = extract_classification_adequacy(text, doc_path)

        n_upstream = sum(
            1 for c in correspondence if c["upstream_overview"] != "(上游文档未找到)"
        )

        checklist = {
            "doc_id": doc_id,
            "doc_path": str(doc_path),
            "doc_relative": str(rel),
            "承接语义对应性": correspondence,
            "论证自洽性": self_consistency,
            "归位充分性": classification,
            "meta": {
                "n_correspondence": len(correspondence),
                "n_self_consistency": len(self_consistency),
                "n_classification": len(classification),
                "n_upstream_found": n_upstream,
            },
        }

        n_objects = (
            len(correspondence) + len(self_consistency) + len(classification)
        )
        stats["total"] += 1
        stats["total_correspondence"] += len(correspondence)
        stats["total_self_consistency"] += len(self_consistency)
        stats["total_classification"] += len(classification)
        stats["total_upstream_found"] += n_upstream
        if n_objects == 0:
            stats["empty_checklists"] += 1

        by_type[doc_type]["docs"] += 1
        by_type[doc_type]["corr"] += len(correspondence)
        by_type[doc_type]["self"] += len(self_consistency)
        by_type[doc_type]["class"] += len(classification)

        out_file = OUT_BASE / f"{doc_id}.json"
        out_file.parent.mkdir(parents=True, exist_ok=True)
        with open(out_file, "w", encoding="utf-8") as f:
            json.dump(checklist, f, ensure_ascii=False, indent=2)

    print(f"\n提取完成：")
    print(f"  总文档数: {stats['total']}")
    print(
        f"  承接语义对应性 targets: {stats['total_correspondence']} "
        f"(上游找到 {stats['total_upstream_found']})"
    )
    print(f"  论证自洽性 targets: {stats['total_self_consistency']}")
    print(f"  归位充分性 targets: {stats['total_classification']}")
    print(f"  空清单: {stats['empty_checklists']}")
    print(f"\n按类型：")
    for t, v in sorted(by_type.items()):
        total = v["corr"] + v["self"] + v["class"]
        print(
            f"  {t}: {v['docs']} docs, "
            f"对应性={v['corr']}, 自洽性={v['self']}, 归位={v['class']}, "
            f"total={total}"
        )

    total_sessions = stats["total"] * 4
    print(f"\n  预估 session 数: {stats['total']} docs x 4 seq = {total_sessions}")

    summary_file = OUT_BASE / "_extraction-summary.json"
    with open(summary_file, "w", encoding="utf-8") as f:
        json.dump({"stats": stats, "by_type": dict(by_type)}, f, ensure_ascii=False, indent=2)
    print(f"摘要写入 {summary_file}")


if __name__ == "__main__":
    main()
