#!/usr/bin/env python3
"""乙子进程:对已就绪沙盒仓直调生产 _chain_union_yield(恢复非空,预期自死锁)。"""
import sys
from pathlib import Path

sys.path.insert(0, "/Users/moc/workspaces/SiHankor/sih-tools/lease/src")
from lease.core import _chain_union_yield

repo = sys.argv[1]
backup_dir = Path(__file__).resolve().parent / "h4-b-backups"
report = _chain_union_yield(repo, "main", "msh/beta", ["lease/ledger/sessions.ndjson"], backup_dir)
print("returned:", report)
