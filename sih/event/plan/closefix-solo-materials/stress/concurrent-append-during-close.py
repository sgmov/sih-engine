#!/usr/bin/env python3
"""并发活写窗 close 压测（closefix-solo F-1 压测形）。

形：台账在仓内为跟踪件（生产形），close 进行期间他进程对主树台账活写追加
（append_row 带 flock），收约毕全部行俱在——盖版根除的并发面机械证明。

判据：
  stress 行（他进程追加 N 行）收约毕逐行在账；
  会话 issued/revoked 行俱在；
  最终行数 == 初态 + N + 1（revoked）；
  git stash list 为空（吞件根除）。
"""
import json
import subprocess
import sys
import time
from pathlib import Path

WT_SRC = "/Users/moc/workspaces/SiHankor/worktrees/sih-tools/closefix-solo/lease/src"
sys.path.insert(0, WT_SRC)

HERE = Path(__file__).resolve().parent
LEDGER_REL = "lease/ledger/sessions.ndjson"
INTENT = "/Users/moc/workspaces/SiHankor/sih-tools/lease/tests/fixtures-intent.json"
AT = "2026-09-06T13:00:00+00:00"
N_ROWS = 20
INTERVAL = 0.05

APPENDER = """
import sys, time, json
sys.path.insert(0, {src!r})
from lease.ledgerwrite import append_row
led = {led!r}
for i in range({n}):
    append_row(led, {{"event": "issued", "package": "stress-writer", "session_id": "SW%03d" % i, "issued_at": "2026-09-06T13:01:00+00:00"}})
    time.sleep({interval})
print("appended", {n})
"""


def g(repo, *args):
    r = subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True)
    if r.returncode != 0:
        raise RuntimeError(f"git {args}: {r.stderr}")
    return r.stdout


def rows(led):
    return [json.loads(ln) for ln in led.read_text(encoding="utf-8").splitlines() if ln.strip()]


def main():
    from lease.cli import main as lease_main

    work = HERE / "stress-repo"
    if work.exists():
        subprocess.run(["rm", "-rf", str(work)], check=True)
    repo = work / "main" / "sih-engine"
    repo.mkdir(parents=True)
    led = repo / LEDGER_REL
    led.parent.mkdir(parents=True)
    led.write_text("", encoding="utf-8")
    g(repo, "init", "-q", "-b", "main")
    g(repo, "config", "user.email", "t@t")
    g(repo, "config", "user.name", "t")
    g(repo, "add", "-A")
    g(repo, "commit", "-q", "-m", "ledger base")
    packages = repo / "sih" / "state" / "plan"
    packages.mkdir(parents=True)
    (packages / "demo-pkg.md").write_text("# demo\n", encoding="utf-8")
    identity = work / "identity.json"
    identity.write_text(json.dumps({
        "identity": {"hash": "a" * 64, "salt": "b" * 64, "string": "v3|"},
        "observed": {"pid": "99999", "ppid": "2", "hostname": "ZHQHOSTMARK",
                     "mac": "0123456789ab", "user": "u"},
    }), encoding="utf-8")
    locks = work / "locks.ndjson"
    import io
    from contextlib import redirect_stdout

    def run(*argv):
        out = io.StringIO()
        with redirect_stdout(out):
            code = lease_main(list(argv))
        return code, json.loads(out.getvalue())

    code, payload = run("open", "--package", "demo-pkg", "--identity", str(identity),
                        "--intent", INTENT, "--repo", str(repo), "--root", str(work / "main"),
                        "--ledger", str(led), "--at", AT)
    assert code == 0, payload
    wt = work / "main" / "worktrees" / "sih-engine" / "demo-pkg"
    (wt / "own-file.md").write_text("分支产物\n", encoding="utf-8")
    g(wt, "-c", "user.email=t@t", "-c", "user.name=t", "add", "-A")
    g(wt, "-c", "user.email=t@t", "-c", "user.name=t", "commit", "-q", "-m", "branch work")
    baseline = len(rows(led))

    appender = subprocess.Popen(
        [sys.executable, "-c", APPENDER.format(src=WT_SRC, led=str(led), n=N_ROWS,
                                               interval=INTERVAL, sec=0)],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    t0 = time.monotonic()
    code, payload = run("close", "--package", "demo-pkg", "--root", str(work / "main"),
                        "--ledger", str(led), "--locks", str(locks))
    close_dt = time.monotonic() - t0
    out, err = appender.communicate(timeout=30)
    assert appender.returncode == 0, err[-300:]

    final = rows(led)
    stress_rows = [r for r in final if r.get("package") == "stress-writer"]
    revoked = [r for r in final if r.get("event") == "revoked"]
    issued_demo = [r for r in final if r.get("event") == "issued" and r.get("package") == "demo-pkg"]
    stash_list = g(repo, "stash", "list").strip()
    result = {
        "scenario": "并发活写窗 close 压测：close 进行期间他进程 append_row 追加 20 行（无前置 sleep，活写窗与 close 全程交叠）",
        "baseline_rows": baseline,
        "final_rows": len(final),
        "stress_rows_present": len(stress_rows),
        "stress_rows_expected": N_ROWS,
        "revoked_present": len(revoked),
        "session_issued_present": len(issued_demo),
        "close_seconds": round(close_dt, 2),
        "appender_stdout": out.strip(),
        "stash_list_empty": stash_list == "",
        "PASS": (len(stress_rows) == N_ROWS and len(revoked) == 1
                 and len(issued_demo) == 1 and len(final) == baseline + N_ROWS + 1
                 and stash_list == ""),
    }
    print(json.dumps(result, ensure_ascii=False, indent=1))
    return 0 if result["PASS"] else 1


if __name__ == "__main__":
    sys.exit(main())
