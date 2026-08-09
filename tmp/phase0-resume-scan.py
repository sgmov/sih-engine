#!/usr/bin/env python3
"""Phase 0 resumption scanner.

Reads a phase0 output jsonl file, extracts only (check_item, sample_seq)
pairs, compares against the full 4x10=40 matrix, prints missing pairs.

Does NOT read judgment or reason fields — resumption subagents must not
see prior judgments (independence protection).

Usage:
    python3 phase0-resume-scan.py <output.jsonl>

Exit codes:
    0 = all 40 present, document complete
    1 = missing pairs found (printed to stdout)
    2 = file unreadable or malformed
"""

import json
import sys

CHECK_ITEMS = [
    "承接关系有效性",
    "可证伪条件存在性",
    "认识论标签合法性",
    "术语一致性",
]
SAMPLES = range(1, 11)  # 1..10
FULL_MATRIX = {(c, s) for c in CHECK_ITEMS for s in SAMPLES}


def scan(path):
    seen = set()
    try:
        with open(path, "r", encoding="utf-8") as f:
            for lineno, line in enumerate(f, 1):
                line = line.strip()
                if not line:
                    continue
                try:
                    rec = json.loads(line)
                except json.JSONDecodeError as e:
                    print(f"ERROR line {lineno}: invalid JSON: {e}", file=sys.stderr)
                    return None
                # Extract only identity fields, never judgment/reason
                ci = rec.get("check_item")
                ss = rec.get("sample_seq")
                if ci is None or ss is None:
                    print(
                        f"WARN line {lineno}: missing check_item or sample_seq, skipped",
                        file=sys.stderr,
                    )
                    continue
                try:
                    ss = int(ss)
                except (TypeError, ValueError):
                    print(
                        f"WARN line {lineno}: sample_seq not int ({ss!r}), skipped",
                        file=sys.stderr,
                    )
                    continue
                seen.add((ci, ss))
    except FileNotFoundError:
        print(f"ERROR: file not found: {path}", file=sys.stderr)
        return None
    except OSError as e:
        print(f"ERROR: cannot read {path}: {e}", file=sys.stderr)
        return None
    return seen


def main():
    if len(sys.argv) != 2:
        print("Usage: phase0-resume-scan.py <output.jsonl>", file=sys.stderr)
        sys.exit(2)
    path = sys.argv[1]
    seen = scan(path)
    if seen is None:
        sys.exit(2)
    missing = FULL_MATRIX - seen
    if not missing:
        print(f"OK: 40/40 complete ({path})")
        sys.exit(0)
    # Print missing in matrix order for stable diff/re-run
    print(f"MISSING {len(missing)} pairs in {path}:")
    for c in CHECK_ITEMS:
        for s in SAMPLES:
            if (c, s) in missing:
                print(f"{c} {s}")
    sys.exit(1)


if __name__ == "__main__":
    main()
