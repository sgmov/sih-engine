#!/usr/bin/env python3
"""Deterministic extraction of check targets for Phase 0 v3.

Generates fixed checklists per document so that judgment sessions
do zero extraction — they only judge pre-specified targets.

Output: phase0-v3/checklists/{doc}.json

Structure per checklist:
{
  "doc_id": "...",
  "doc_path": "...",
  "承接关系有效性": [
    {"target": "DEC-001", "context": "引用位置说明"},
    ...
  ],
  "可证伪条件存在性": [
    {"target": "可证伪条件一 ...", "label": "external-anchor"},
    ...
  ] or [] if no epistemic stance,
  "认识论标签合法性": [
    {"target": "design-corollary", "raw_text": "设计推论(design-corollary)"}
  ] or [{"target": "(无认识论立场节)"}],
  "术语一致性": [
    {"target": "NPC", "defined_in": "术语表" or null}
  ]
}
"""

import json
import re
import sys
from pathlib import Path

REPO = Path("/Users/moc/workspaces/SiHankor")
OLD_DOC = REPO / "sihankor/doc"
OUT_BASE = REPO / "sih-engine/sih/event/experiment/phase0-v3/checklists"

DOCS = [
    ("DES-058", OLD_DOC / "design/DES-058-multi-agent-collaboration-framework.md"),
    ("DES-017", OLD_DOC / "design/DES-017-brain-borrowing-mechanism.md"),
    ("DEC-008", OLD_DOC / "decision/DEC-008-know-sediment.md"),
    ("DEC-004", OLD_DOC / "decision/DEC-004-review-progression-plan.md"),
    ("sprint-plan", OLD_DOC / "draft/sprint-plan-setsp-reuse-2026-07-21.md"),
    ("INTENT-DRIFT", OLD_DOC / "draft/INTENT-DRIFT-INVENTORY-V2.md"),
]

REF_PATTERN = re.compile(r'\b(PRO|DEC|DES|GOV|KNOW|SPEC|ARC)-(\d{3})\b')


def extract_refs(text, doc_id):
    """Extract engineering references, exclude self-reference."""
    refs = REF_PATTERN.findall(text)
    seen = []
    for prefix, num in refs:
        ref = f"{prefix}-{num}"
        if ref == doc_id:  # exclude self
            continue
        if ref not in seen:
            seen.append(ref)
    return [{"target": r} for r in seen]


def find_section(text, heading_keywords):
    """Find a section by heading keyword, return (start_line, content_until_next_heading)."""
    lines = text.split("\n")
    for i, line in enumerate(lines):
        if line.startswith("#"):
            for kw in heading_keywords:
                if kw in line:
                    # Find end (next heading of same or higher level)
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


def extract_epistemic(text):
    """Extract epistemic stance: labels and falsification conditions."""
    _, section = find_section(text, ["认识论立场", "epistemic-stance", "epistemic"])
    if not section:
        return {"has_epistemic_stance": False, "labels": [], "conditions": []}

    # Extract labels: look for known label values or Chinese equivalents
    label_map = {
        "external-anchor": ["external-anchor", "外部锚定"],
        "empirical-hypothesis": ["empirical-hypothesis", "经验假设"],
        "tautology": ["tautology", "重言式"],
        "design-corollary": ["design-corollary", "设计推论"],
        "constructed-framework": ["constructed-framework", "构造框架"],
    }
    found_labels = []
    for canonical, variants in label_map.items():
        for v in variants:
            if v in section:
                found_labels.append(canonical)
                break
    # Deduplicate preserving order
    seen = set()
    labels = []
    for l in found_labels:
        if l not in seen:
            labels.append(l)
            seen.add(l)

    # Extract falsification conditions
    conditions = []
    lines = section.split("\n")
    for line in lines:
        stripped = line.strip()
        # Match numbered conditions like "1. 可证伪一:..." or "可证伪条件一..."
        if re.match(r'^\d+\.\s*可证伪', stripped) or re.match(r'^可证伪条件', stripped):
            conditions.append({"target": stripped})
        # Also match lines with 若...则 or 当...则 pattern as conditions
        elif re.match(r'^\d+\.\s+(若|当)', stripped):
            conditions.append({"target": stripped})

    return {
        "has_epistemic_stance": True,
        "labels": [{"target": l} for l in labels],
        "conditions": conditions,
    }


def extract_terms(text, doc_id):
    """Extract terms from glossary section only.

    If no glossary exists, return empty list — the check is skipped
    (you can't check term consistency without a defined term set).
    Only DES-017 has a glossary among the 6 docs.
    """
    _, glossary = find_section(text, ["术语表", "词汇表", "glossary"])
    if not glossary:
        return []

    # Extract term entries from definition list format: "term\n: definition"
    # Or simple "term\n: text"
    lines = glossary.split("\n")
    terms = []
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        # Definition list: next line starts with ":"
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

    # Fallback: if no definition-list terms found, try bullet format
    if not terms:
        for line in lines:
            stripped = line.strip()
            if stripped.startswith("- ") and stripped:
                # "- **Term**: definition" or "- Term: definition"
                content = stripped[2:]
                # Extract term before colon
                if ":" in content:
                    term = content.split(":")[0].strip().strip("*")
                    if term and len(term) <= 20:
                        terms.append({"target": term})

    return terms


def main():
    OUT_BASE.mkdir(parents=True, exist_ok=True)

    for doc_id, doc_path in DOCS:
        text = doc_path.read_text(encoding="utf-8")

        refs = extract_refs(text, doc_id)
        epistemic = extract_epistemic(text)
        terms = extract_terms(text, doc_id)

        checklist = {
            "doc_id": doc_id,
            "doc_path": str(doc_path),
            "承接关系有效性": refs,
            "可证伪条件存在性": epistemic["conditions"],
            "认识论标签合法性": epistemic["labels"],
            "术语一致性": terms,
            "meta": {
                "has_epistemic_stance": epistemic["has_epistemic_stance"],
                "has_glossary": any(t.get("has_glossary") for t in terms),
                "n_refs": len(refs),
                "n_conditions": len(epistemic["conditions"]),
                "n_labels": len(epistemic["labels"]),
                "n_terms": len(terms),
            },
        }

        out_file = OUT_BASE / f"{doc_id}.json"
        with open(out_file, "w", encoding="utf-8") as f:
            json.dump(checklist, f, ensure_ascii=False, indent=2)
        print(f"{doc_id}: refs={len(refs)} conditions={len(epistemic['conditions'])} labels={len(epistemic['labels'])} terms={len(terms)} epistemic={epistemic['has_epistemic_stance']}")

    print(f"\n清单已生成到 {OUT_BASE}/")


if __name__ == "__main__":
    main()
