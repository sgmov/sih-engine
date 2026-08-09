#!/usr/bin/env python3
"""Extract semantic check targets for Phase 2 calibration.

For each of 6 anchor docs, generates 3 check items:
1. 承接语义对应性: refs + claim context + upstream overview (packed)
2. 论证自洽性: doc core claim (from overview section)
3. 归位充分性: doc claimed type

Output: phase2-calibration/checklists/{doc_id}.json

Usage:
    python3 phase2-extract-checklists.py
"""

import json
import re
import sys
from pathlib import Path

REPO = Path("/Users/moc/workspaces/SiHankor")
OLD_DOC = REPO / "sihankor/doc"
OUT_BASE = REPO / "sih-engine/sih/event/experiment/phase2-calibration/checklists"

ANCHOR_DOCS = [
    ("DES-058", OLD_DOC / "design/DES-058-multi-agent-collaboration-framework.md"),
    ("DES-017", OLD_DOC / "design/DES-017-brain-borrowing-mechanism.md"),
    ("DEC-008", OLD_DOC / "decision/DEC-008-know-sediment.md"),
    ("DEC-004", OLD_DOC / "decision/DEC-004-review-progression-plan.md"),
    ("sprint-plan", OLD_DOC / "draft/sprint-plan-setsp-reuse-2026-07-21.md"),
    ("INTENT-DRIFT", OLD_DOC / "draft/INTENT-DRIFT-INVENTORY-V2.md"),
]

REF_PATTERN = re.compile(r'\b(PRO|DEC|DES|GOV|KNOW|SPEC|ARC)-(\d{3})\b')

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
    """Find a section by heading keyword, return content until next heading."""
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
                    return "\n".join(lines[i:end])
    return None


def get_overview(text):
    """Extract the 概览 section from a document."""
    section = find_section(text, ["概览", "overview"])
    return section if section else "(无概览节)"


def get_core_claim(text):
    """Extract core claim from overview section."""
    overview = get_overview(text)
    if overview == "(无概览节)":
        # Fallback: use first paragraph after title
        lines = text.split("\n")
        for i, line in enumerate(lines):
            if line.startswith("# ") and i + 2 < len(lines):
                return lines[i + 2] if lines[i + 2].strip() else "(无法提取核心主张)"
        return "(无法提取核心主张)"
    # First 3 bullet points or first paragraph of overview
    lines = overview.split("\n")
    claim_lines = []
    for line in lines[1:]:  # skip heading
        if line.strip() and not line.startswith("#"):
            claim_lines.append(line.strip())
        if len(claim_lines) >= 3:
            break
    return " ".join(claim_lines) if claim_lines else overview[:200]


def find_upstream_doc(ref_prefix, ref_num):
    """Find the upstream document file by prefix and number."""
    doc_type = PREFIX_TO_DIR.get(ref_prefix)
    if not doc_type:
        return None

    type_dir = OLD_DOC / doc_type
    if not type_dir.exists():
        return None

    # Try to find file starting with the number
    pattern = f"{ref_num}-"
    matches = list(type_dir.glob(f"{pattern}*.md"))
    if matches:
        # Filter out directories, prefer .md files
        file_matches = [m for m in matches if m.is_file()]
        if file_matches:
            return file_matches[0]

    # Also try with prefix
    pattern2 = f"{ref_prefix}-{ref_num}"
    matches2 = list(type_dir.glob(f"*{pattern2}*"))
    if matches2:
        file_matches2 = [m for m in matches2 if m.is_file()]
        if file_matches2:
            return file_matches2[0]

    # Try subdirectories (e.g. DES-001-document-format/ is a dir)
    dir_matches = [m for m in matches if m.is_dir()]
    if dir_matches:
        # Look for a main file inside the directory
        for d in dir_matches:
            # Prefer general.md or design.md as the "main" doc
            for candidate in ["general.md", "design.md", "overview.md", "index.md"]:
                f = d / candidate
                if f.exists():
                    return f
            # Fallback: first .md file in directory
            md_files = list(d.glob("*.md"))
            if md_files:
                return md_files[0]

    return None


def extract_refs_with_claims(text, doc_id):
    """Extract refs with their surrounding context (claim)."""
    refs = []
    seen = set()
    lines = text.split("\n")

    # Find all ref mentions with context
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
                context_parts.append(lines[i-1].strip())
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

        upstream_file = find_upstream_doc(prefix, num)
        if upstream_file:
            upstream_text = upstream_file.read_text(encoding="utf-8")
            upstream_overview = get_overview(upstream_text)
            # Truncate overview to reasonable length
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
    # Determine type from path
    rel = doc_path.relative_to(OLD_DOC)
    doc_type = rel.parts[0] if rel.parts else "unknown"
    claimed_type = DOC_TYPE_MAP.get(doc_type, doc_type)

    if claimed_type == "draft":
        return []  # Skip draft docs for this check

    return [{"target": "文档自身", "claimed_type": claimed_type}]


def main():
    OUT_BASE.mkdir(parents=True, exist_ok=True)

    for doc_id, doc_path in ANCHOR_DOCS:
        text = doc_path.read_text(encoding="utf-8")

        correspondence = extract_semantic_correspondence(text, doc_id)
        self_consistency = extract_self_consistency(text)
        classification = extract_classification_adequacy(text, doc_path)

        checklist = {
            "doc_id": doc_id,
            "doc_path": str(doc_path),
            "承接语义对应性": correspondence,
            "论证自洽性": self_consistency,
            "归位充分性": classification,
            "meta": {
                "n_correspondence": len(correspondence),
                "n_self_consistency": len(self_consistency),
                "n_classification": len(classification),
                "n_upstream_found": sum(1 for c in correspondence if c["upstream_overview"] != "(上游文档未找到)"),
            },
        }

        out_file = OUT_BASE / f"{doc_id}.json"
        out_file.parent.mkdir(parents=True, exist_ok=True)
        with open(out_file, "w", encoding="utf-8") as f:
            json.dump(checklist, f, ensure_ascii=False, indent=2)

        print(f"{doc_id}: 对应性={len(correspondence)}(上游找到{checklist['meta']['n_upstream_found']}) "
              f"自洽性={len(self_consistency)} 归位={len(classification)}")

    print(f"\n清单已生成到 {OUT_BASE}/")


if __name__ == "__main__":
    main()
