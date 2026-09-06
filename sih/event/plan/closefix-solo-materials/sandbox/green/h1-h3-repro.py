#!/usr/bin/env python3
"""H-1 与 H-3 沙盒复现。

H-1:tool_dir() 随工地源码定位前提验证(台账缺省位随执行体所在源码树漂移),
     并核 resolve_root 双仓祖先搜上仍回真根——前提为真,但对第五笔判证伪。
H-3:SPEC-020 让位归并 checkout 不持 ledger_lock——他进程持锁时 checkout 畅通
     即机械证明该写点绕开 pk057 互斥网;对照组 append_row 同刻被阻塞。
"""
import json
import multiprocessing
import subprocess
import sys
import time
from pathlib import Path

PROD_LEASE_SRC = "/Users/moc/workspaces/SiHankor/worktrees/sih-tools/closefix-solo/lease/src"
sys.path.insert(0, PROD_LEASE_SRC)
HERE = Path(__file__).resolve().parent


def h1_tool_dir():
    """H-1:复制 lease 源到假工地位,证明 tool_dir 与缺省台账位随工地漂移。"""
    import shutil
    fake_root = HERE / "h1-fakeroot"
    if fake_root.exists():
        shutil.rmtree(fake_root)
    wt = fake_root / "worktrees" / "sih-tools" / "fake-batch" / "lease"
    (fake_root / "sih-tools").mkdir(parents=True)
    (fake_root / "sih-tools" / "pyproject.toml").write_text("", encoding="utf-8")
    (fake_root / "sih-engine").mkdir(parents=True)
    (fake_root / "sih-engine" / "Cargo.toml").write_text("", encoding="utf-8")
    shutil.copytree(Path(PROD_LEASE_SRC).parent, wt,
                    ignore=shutil.ignore_patterns("__pycache__", ".venv", "tests"))
    code = (
        "import sys, json;"
        f"sys.path.insert(0, r'{wt / 'src'}');"
        "from lease.core import tool_dir, resolve_root;"
        "print(json.dumps({'tool_dir': str(tool_dir()), 'resolved_root': str(resolve_root(None))}))"
    )
    r = subprocess.run([sys.executable, "-c", code], capture_output=True, text=True)
    d = json.loads(r.stdout)
    prod_ledger = "/Users/moc/workspaces/SiHankor/sih-tools/lease/ledger/sessions.ndjson"
    sandbox_ledger = str(wt / "ledger" / "sessions.ndjson")
    return {
        "scenario": "H-1 前提验证:工地副本内执行 tool_dir/缺省台账位",
        "tool_dir_from_worktree": d["tool_dir"],
        "resolve_root_from_worktree": d["resolved_root"],
        "default_ledger_in_worktree_copy": sandbox_ledger,
        "production_ledger": prod_ledger,
        "mislocated": sandbox_ledger != prod_ledger,
        "note": "前提为真:tool_dir 随源码树漂移;resolve_root 双仓祖先搜上仍回真根。",
    }


def hold_lock(path, seconds, q):
    from lease.ledgerwrite import ledger_lock
    with ledger_lock(path):
        q.put("held")
        time.sleep(seconds)
    q.put("released")


def h3_lock_bypass():
    """H-3:他进程持 ledger_lock 时,allow_and_merge 的 git checkout 畅通无阻。"""
    from lease.core import allow_and_merge
    repo = HERE / "h3-repo"
    if repo.exists():
        subprocess.run(["rm", "-rf", str(repo)], check=True)
    repo.mkdir(parents=True)
    def g(*args):
        r = subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True)
        if r.returncode != 0:
            raise RuntimeError(r.stderr)
        return r.stdout
    g("init", "-q", "-b", "main")
    g("config", "user.email", "s@s"); g("config", "user.name", "s")
    led = repo / "lease" / "ledger" / "sessions.ndjson"
    led.parent.mkdir(parents=True)
    base_rows = [{"event": "issued", "package": "p1", "session_id": "S1", "issued_at": "2026-09-06T00:00:00+00:00"}]
    led.write_text("".join(json.dumps(r, sort_keys=True, separators=(",", ":")) + "\n" for r in base_rows), encoding="utf-8")
    g("add", "-A"); g("commit", "-q", "-m", "base")
    g("checkout", "-q", "-b", "msh/beta")
    (repo / "doc.txt").write_text("branch file\n", encoding="utf-8")
    g("add", "-A"); g("commit", "-q", "-m", "branch change")
    g("checkout", "-q", "main")
    # 主树台账活写一行(锁窗内追加者类比)
    from lease.ledgerwrite import append_row
    append_row(led, {"event": "issued", "package": "p2", "session_id": "S2", "issued_at": "2026-09-06T00:01:00+00:00"})
    q = multiprocessing.Queue()
    p = multiprocessing.Process(target=hold_lock, args=(led, 4.0, q))
    p.start()
    assert q.get() == "held"  # 锁已在他进程手中
    t0 = time.monotonic()
    result = allow_and_merge(str(repo), "main", "msh/beta", ["lease/ledger/sessions.ndjson"])
    dt = time.monotonic() - t0
    lock_respected = dt >= 4.0
    after = led.read_text(encoding="utf-8").splitlines()
    p.join()
    # 对照:锁在持时 append_row 应阻塞至释放
    q2 = multiprocessing.Queue()
    p2 = multiprocessing.Process(target=hold_lock, args=(led, 2.0, q2))
    p2.start()
    assert q2.get() == "held"
    t1 = time.monotonic()
    append_row(led, {"event": "issued", "package": "p3", "session_id": "S3", "issued_at": "2026-09-06T00:02:00+00:00"})
    dt2 = time.monotonic() - t1
    p2.join()
    return {
        "scenario": "H-3 SPEC-020 checkout 绕锁验证",
        "checkout_completed_in_seconds_under_foreign_lock": round(dt, 3),
        "checkout_waited_for_lock": lock_respected,
        "rows_after_checkout": len(after),
        "s2_live_row_survived_checkout": any('"S2"' in l for l in after),
        "control_append_row_blocked_seconds": round(dt2, 3),
        "control_append_waited": dt2 >= 1.5,
        "note": "checkout 持锁窗内即时完成且活写行被盖即机械证明让位归并写点绕开互斥网;对照组 append_row 阻塞至锁释放。",
    }


if __name__ == "__main__":
    out = {"h1": h1_tool_dir(), "h3": h3_lock_bypass(),
           "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S%z")}
    print(json.dumps(out, ensure_ascii=False, indent=1))
