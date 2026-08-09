#!/usr/bin/env python3
"""Phase 0 v2 resumption scanner: isolated session mode.

Checks each document subdirectory for seq1.jsonl to seq10.jsonl.
Reports missing seq files per document.

Usage:
    python3 phase0-v2-resume-scan.py <phase0-v2-dir>

Exit codes:
    0 = all 6 docs × 10 seqs complete
    1 = missing files (printed)
    2 = bad usage
"""

import sys
from pathlib import Path

DOCS = [
    "DES-058",
    "DES-017",
    "DEC-008",
    "DEC-004",
    "sprint-plan",
    "INTENT-DRIFT",
]


def main():
    if len(sys.argv) != 2:
        print("Usage: phase0-v2-resume-scan.py <phase0-v2-dir>", file=sys.stderr)
        sys.exit(2)
    base = Path(sys.argv[1])

    all_complete = True
    for doc in DOCS:
        doc_dir = base / doc
        if not doc_dir.exists():
            print(f"MISSING dir: {doc_dir}")
            all_complete = False
            continue
        for seq in range(1, 11):
            f = doc_dir / f"{doc}-seq{seq}.jsonl"
            if not f.exists() or f.stat().st_size == 0:
                print(f"MISSING {doc} seq{seq}: {f}")
                all_complete = False
    if all_complete:
        print(f"OK: 6 docs × 10 seqs all present ({base})")
        sys.exit(0)
    sys.exit(1)


if __name__ == "__main__":
    main()
