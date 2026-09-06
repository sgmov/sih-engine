#!/usr/bin/env python3
"""H-4/H-2 沙盒复现:close SPEC-020 让位归并整文件盖版倒灌 + 回补网不可达 + 回补网自死锁。

三景:
  甲(H-4 主证):生产 SPEC-020 子序列直调重放 2026-09-05 第五笔——工作面 10 行
    盖版后 3 行,7 行丢失,预收提交固化,merge 终局固化,_chain_append_grown 回 None。
  乙(H-2 深证):同一初态走 basefix _chain_union_yield,恢复非空时进程在
    restore_missing→append_row→ledger_lock 二次取锁处自死锁(子进程带超时机械固定)。
  丙(修复方向证明):同一初态按「单锁窗 + 窗内差集追加」语义(沙盒自实现,不动生产码),
    行零丢失。

生产代码只读导入,git 动作限沙盒仓。
"""
import json
import subprocess
import sys
import time
from pathlib import Path

PROD_LEASE_SRC = "/Users/moc/workspaces/SiHankor/sih-tools/lease/src"
sys.path.insert(0, PROD_LEASE_SRC)

from lease.core import (  # noqa: E402
    detect_merge_conflicts, is_pure_append_conflict, backup_conflict_files,
    allow_and_merge, _chain_append_grown, _chain_union_yield,
)
from lease.ledgerwrite import append_row, restore_missing, ledger_lock  # noqa: E402

HERE = Path(__file__).resolve().parent
LEDGER_REL = "lease/ledger/sessions.ndjson"


def row(sid, pkg, ts):
    return {"event": "issued", "package": pkg, "session_id": sid, "issued_at": ts}


def dump(rows):
    return "".join(json.dumps(r, sort_keys=True, separators=(",", ":")) + "\n" for r in rows)


def rd(repo):
    p = Path(repo) / LEDGER_REL
    return [json.loads(l) for l in p.read_text(encoding="utf-8").splitlines() if l.strip()]


def g(repo, *args):
    r = subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True)
    if r.returncode != 0:
        raise RuntimeError(f"git {args}: {r.stderr}")
    return r.stdout


def setup(name):
    repo = HERE / name
    if repo.exists():
        subprocess.run(["rm", "-rf", str(repo)], check=True)
    repo.mkdir(parents=True)
    led = repo / LEDGER_REL
    led.parent.mkdir(parents=True)
    led.write_text(dump(BASE), encoding="utf-8")
    g(repo, "init", "-q", "-b", "main")
    g(repo, "config", "user.email", "sandbox@sih")
    g(repo, "config", "user.name", "sandbox")
    g(repo, "add", "-A")
    g(repo, "commit", "-q", "-m", "base R1..R3")
    wt = HERE / (name + "-wt")
    if wt.exists():
        subprocess.run(["rm", "-rf", str(wt)], check=True)
    g(repo, "worktree", "add", "-q", "-b", "msh/beta", str(wt), "main")
    write_ledger = repo / LEDGER_REL
    write_ledger.write_text(dump(BASE + ADV), encoding="utf-8")
    g(repo, "add", "-A")
    g(repo, "commit", "-q", "-m", "main advanced 6 rows (other batches pre-close)")
    append_row(write_ledger, LIVE)
    return repo


BASE = [row("R1", "batch-one", "2026-09-05T10:00:00+00:00"),
        row("R2", "batch-two", "2026-09-05T11:00:00+00:00"),
        row("R3", "batch-three", "2026-09-05T12:00:00+00:00")]
ADV = [row("R4", "docmath-analog", "2026-09-05T15:13:52+00:00"),
       row("R5", "idenlane-analog", "2026-09-05T15:14:57+00:00"),
       row("R6", "docmath-rv-analog", "2026-09-05T15:17:43+00:00"),
       row("R7", "wc1-issued-analog", "2026-09-05T15:25:45+00:00"),
       row("R8", "wc1-revoked-analog", "2026-09-05T15:26:53+00:00"),
       row("R9", "wc2-issued-analog", "2026-09-05T15:27:05+00:00")]
LIVE = {"event": "revoked", "package": "wc2-issued-analog", "revoked_at": "2026-09-05T15:57:47+00:00", "session_id": "R9"}


def scenario_a():
    """甲:生产 SPEC-020 子序列重放。"""
    repo = setup("h4-a")
    ondisk = len(rd(repo))
    conflicts = detect_merge_conflicts(str(repo), "main", "msh/beta")
    ctype = is_pure_append_conflict(conflicts)
    append_files = [f for f in ctype["files"] if ctype["files"][f].get("type") == "append"]
    backup = backup_conflict_files(str(repo), append_files, HERE / "h4-a-backups")
    allow = allow_and_merge(str(repo), "main", "msh/beta", append_files)
    after = len(rd(repo))
    g(repo, "add", "-A")
    g(repo, "commit", "-q", "-m", "closeguard pre-close commit (beta-analog)")
    committed = len(g(repo, "show", f"HEAD:{LEDGER_REL}").splitlines())
    grown = _chain_append_grown(str(repo), "main", "msh/beta", LEDGER_REL)
    g(repo, "merge", "-q", "--no-ff", "msh/beta", "-m", "merge: beta-analog")
    final = len(g(repo, "show", f"HEAD:{LEDGER_REL}").splitlines())
    return {
        "scenario": "甲 生产 SPEC-020 子序列(detect→classify→backup→allow_and_merge→预收提交→merge)",
        "conflict_type": ctype["conflict_type"],
        "classified_files": {f: ctype["files"][f].get("type") for f in ctype["files"]},
        "before_rows": ondisk, "after_checkout_rows": after,
        "rows_lost_working_tree": ondisk - after,
        "backup_rows": len((HERE / "h4-a-backups" / LEDGER_REL).read_text(encoding="utf-8").splitlines()),
        "committed_after_preclose": committed,
        "chain_append_grown_post_clobber": "None" if grown is None else grown,
        "final_merged_rows": final,
        "RED": (after == 3 and ondisk - after == 7 and committed == 3 and final == 3 and grown is None),
    }


def _b_child(repo):
    """乙子进程:直调 _chain_union_yield,非空恢复预期自死锁。"""
    _chain_union_yield(str(repo), "main", "msh/beta", [LEDGER_REL], HERE / "h4-b-backups")


def scenario_b():
    """乙:回补网可达时的自死锁(ledger_lock 不可重入)。"""
    repo = setup("h4-b")
    before = len(rd(repo))
    try:
        r = subprocess.run([sys.executable, str(HERE / "h4-b-child.py"), str(repo)],
                           capture_output=True, text=True, timeout=12)
        deadlocked = False
        detail = {"returncode": r.returncode, "stdout_tail": r.stdout[-200:], "stderr_tail": r.stderr[-200:]}
    except subprocess.TimeoutExpired as e:
        deadlocked = True
        detail = {"timeout_seconds": 12, "stderr_tail": (e.stderr or b"")[-200:] if isinstance(e.stderr, bytes) else str(e.stderr)[-200:]}
    return {
        "scenario": "乙 basefix _chain_union_yield 子进程直调(恢复非空,12 秒超时探针)",
        "before_rows": before,
        "self_deadlock_reproduced": deadlocked,
        "mechanism": "_chain_union_yield 持 ledger_lock → restore_missing → append_row → ledger_lock 同路径二次 flock,flock 按 open file description 判定,同进程同线程亦阻塞",
        "RED": deadlocked,
        "detail": detail,
    }


def scenario_c():
    """丙:修复方向证明——单锁窗内差集追加(沙盒自实现,零生产码改动)。"""
    repo = setup("h4-c")
    led = repo / LEDGER_REL
    before = len(rd(repo))
    dest = HERE / "h4-c-backups" / LEDGER_REL
    dest.parent.mkdir(parents=True, exist_ok=True)
    with ledger_lock(led):
        dest.write_text(led.read_text(encoding="utf-8"), encoding="utf-8")
        g(repo, "checkout", "msh/beta", "--", LEDGER_REL)
        # 窗内差集追加:不经 append_row(避重入),直接 O_APPEND 写整行,锁由本窗持有
        existing = set(led.read_text(encoding="utf-8").splitlines())
        import os as _os
        fd = _os.open(led, _os.O_WRONLY | _os.O_APPEND)
        for line in dest.read_text(encoding="utf-8").splitlines():
            if line.strip() and line not in existing:
                _os.write(fd, (line + "\n").encode("utf-8"))
        _os.close(fd)
    after = len(rd(repo))
    return {
        "scenario": "丙 修复方向:单锁窗 + 窗内差集 O_APPEND(语义与 restore_missing 同,锁不重入)",
        "before_rows": before, "after_rows": after, "rows_lost": before - after,
        "RED": before - after == 0,
    }


def h2_unit():
    work = HERE / "h2-unit"
    if work.exists():
        subprocess.run(["rm", "-rf", str(work)], check=True)
    work.mkdir(parents=True)
    live, backup = work / "sessions.ndjson", work / "backup.ndjson"
    backup.write_text(dump(BASE + ADV), encoding="utf-8")
    live.write_text(dump(BASE), encoding="utf-8")
    n = restore_missing(live, backup)
    return {"scenario": "H-2 单元 restore_missing(无外持锁,缺 6 行回补)", "restored": n, "RED": n == 6}


if __name__ == "__main__":
    out = {"h4_a": scenario_a(), "h2_unit": h2_unit(), "h4_b": scenario_b(), "h4_c": scenario_c(),
           "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S%z")}
    print(json.dumps(out, ensure_ascii=False, indent=1))
