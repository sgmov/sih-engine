#!/usr/bin/env python3
"""Full-scale checklist extraction for Phase 1 Round 1.

Scans all 263 .md documents in old repo, generates a checklist per doc.
Same extraction logic as phase0-v3, but operates on all docs.

Output: phase1-round1/checklists/{relative_path_sanitized}.json

Usage:
    python3 phase1-extract-all-checklists.py
"""

import json
import re
import sys
from pathlib import Path
from collections import defaultdict

REPO = Path("/Users/moc/workspaces/SiHankor")
OLD_DOC = REPO / "sihankor/doc"
OUT_BASE = REPO / "sih-engine/sih/event/experiment/phase1-round1/checklists"

REF_PATTERN = re.compile(r'\b(PRO|DEC|DES|GOV|KNOW|SPEC|ARC)-(\d{3})\b')

# Type prefix to directory mapping
PREFIX_TO_DIR = {
    "DES": "design",
    "DEC": "decision",
    "PRO": "proposal",
    "GOV": "governance",
    "KNOW": "knowledge",
    "SPEC": "spec",
    "ARC": "architecture",
}

LABEL_MAP = {
    "external-anchor": ["external-anchor", "外部锚定"],
    "empirical-hypothesis": ["empirical-hypothesis", "经验假设"],
    "tautology": ["tautology", "重言式"],
    "design-corollary": ["design-corollary", "设计推论"],
    "constructed-framework": ["constructed-framework", "构造框架"],
}


def find_section(text, heading_keywords):
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


def extract_refs(text):
    refs = REF_PATTERN.findall(text)
    seen = []
    for prefix, num in refs:
        ref = f"{prefix}-{num}"
        if ref not in seen:
            seen.append(ref)
    return [{"target": r} for r in seen]


def extract_epistemic(text):
    _, section = find_section(text, ["认识论立场", "epistemic-stance", "epistemic"])
    if not section:
        return {"has_epistemic_stance": False, "labels": [], "conditions": []}

    found_labels = []
    for canonical, variants in LABEL_MAP.items():
        for v in variants:
            if v in section:
                found_labels.append(canonical)
                break
    seen = set()
    labels = []
    for l in found_labels:
        if l not in seen:
            labels.append(l)
            seen.add(l)

    conditions = []
    for line in section.split("\n"):
        stripped = line.strip()
        if re.match(r'^\d+\.\s*可证伪', stripped) or re.match(r'^可证伪条件', stripped):
            conditions.append({"target": stripped})
        elif re.match(r'^\d+\.\s+(若|当)', stripped):
            conditions.append({"target": stripped})

    return {
        "has_epistemic_stance": True,
        "labels": [{"target": l} for l in labels],
        "conditions": conditions,
    }


def extract_terms(text):
    _, glossary = find_section(text, ["术语表", "词汇表", "glossary"])
    if not glossary:
        return []
    lines = glossary.split("\n")
    terms = []
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        if (
            line
            and not line.startswith("#")
            and not line.startswith(":")
            and not line.startswith("-")
            and i + 1 < len(lines)
            and lines[i + 1].strip().startswith(":")
        ):
            terms.append({"target": line})
        i += 1
    if not terms:
        for line in lines:
            stripped = line.strip()
            if stripped.startswith("- ") and stripped:
                content = stripped[2:]
                if ":" in content:
                    term = content.split(":")[0].strip().strip("*")
                    if term and len(term) <= 20:
                        terms.append({"target": term})
    return terms


def sanitize_path(rel_path):
    """Convert relative path to safe filename."""
    return rel_path.replace("/", "__").replace(".md", "")


def main():
    OUT_BASE.mkdir(parents=True, exist_ok=True)

    # Find all .md files
    all_docs = sorted(OLD_DOC.rglob("*.md"))
    print(f"发现 {len(all_docs)} 份文档")

    stats = {
        "total": 0,
        "has_refs": 0,
        "has_conditions": 0,
        "has_labels": 0,
        "has_terms": 0,
        "empty_checklists": 0,
        "total_check_objects": 0,
    }

    for doc_path in all_docs:
        rel = doc_path.relative_to(OLD_DOC)
        text = doc_path.read_text(encoding="utf-8")

        refs = extract_refs(text)
        epistemic = extract_epistemic(text)
        terms = extract_terms(text)

        # Determine doc_id from filename
        doc_id = sanitize_path(str(rel))

        checklist = {
            "doc_id": doc_id,
            "doc_path": str(doc_path),
            "doc_relative": str(rel),
            "承接关系有效性": refs,
            "可证伪条件存在性": epistemic["conditions"],
            "认识论标签合法性": epistemic["labels"],
            "术语一致性": terms,
            "meta": {
                "has_epistemic_stance": epistemic["has_epistemic_stance"],
                "n_refs": len(refs),
                "n_conditions": len(epistemic["conditions"]),
                "n_labels": len(epistemic["labels"]),
                "n_terms": len(terms),
            },
        }

        n_objects = len(refs) + len(epistemic["conditions"]) + len(epistemic["labels"]) + len(terms)

        stats["total"] += 1
        stats["total_check_objects"] += n_objects
        if refs:
            stats["has_refs"] += 1
        if epistemic["conditions"]:
            stats["has_conditions"] += 1
        if epistemic["labels"]:
            stats["has_labels"] += 1
        if terms:
            stats["has_terms"] += 1
        if n_objects == 0:
            stats["empty_checklists"] += 1

        out_file = OUT_BASE / f"{doc_id}.json"
        with open(out_file, "w", encoding="utf-8") as f:
            json.dump(checklist, f, ensure_ascii=False, indent=2)

    print(f"\n提取完成：")
    print(f"  总文档数: {stats['total']}")
    print(f"  总检查对象数: {stats['total_check_objects']}")
    print(f"  含承接关系引用: {stats['has_refs']}")
    print(f"  含可证伪条件: {stats['has_conditions']}")
    print(f"  含认识论标签: {stats['has_labels']}")
    print(f"  含术语: {stats['has_terms']}")
    print(f"  空清单（无检查对象）: {stats['empty_checklists']}")

    # Estimate session count
    # N=4 per object-set per doc. Each doc is one "unit" that gets 4 sessions.
    # Sessions = 263 * 4 = 1052 (base) + ~5% tie-trigger
    base_sessions = stats["total"] * 4
    est_extra = int(base_sessions * 0.05)
    print(f"\n  预估 session 数: {base_sessions} 基础 + {est_extra} 平局触发 = {base_sessions + est_extra}")

    # Write summary
    summary_file = OUT_BASE / "_extraction-summary.json"
    with open(summary_file, "w", encoding="utf-8") as f:
        json.dump(stats, f, ensure_ascii=False, indent=2)
    print(f"\n摘要写入 {summary_file}")


if __name__ == "__main__":
    main()
