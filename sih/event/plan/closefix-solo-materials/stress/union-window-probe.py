#!/usr/bin/env python3
"""并集窗探针：分支改台账面（settle 形）加 close 期间他进程活写——零丢失不变式读数。"""
import io, json, subprocess, sys, time
from contextlib import redirect_stdout
from pathlib import Path

WT_SRC = "/Users/moc/workspaces/SiHankor/worktrees/sih-tools/closefix-solo/lease/src"
sys.path.insert(0, WT_SRC)
HERE = Path(__file__).resolve().parent
LEDGER_REL = "lease/ledger/sessions.ndjson"
INTENT = "/Users/moc/workspaces/SiHankor/sih-tools/lease/tests/fixtures-intent.json"

def g(repo, *a):
    r = subprocess.run(["git", "-C", str(repo), *a], capture_output=True, text=True)
    assert r.returncode == 0, (a, r.stderr)
    return r.stdout

def rows(led):
    return [json.loads(x) for x in led.read_text(encoding="utf-8").splitlines() if x.strip()]

work = HERE / "union-probe-repo"
if work.exists():
    subprocess.run(["rm", "-rf", str(work)], check=True)
repo = work / "main" / "sih-engine"; repo.mkdir(parents=True)
led = repo / LEDGER_REL; led.parent.mkdir(parents=True)
for i in range(3):
    led.open("a", encoding="utf-8").write(json.dumps({"event":"issued","package":f"base{i}","session_id":f"B{i}","issued_at":"2026-09-06T12:00:00+00:00"}, sort_keys=True, separators=(",",":"))+"\n")
g(repo, "init", "-q", "-b", "main"); g(repo,"config","user.email","t@t"); g(repo,"config","user.name","t")
g(repo, "add", "-A"); g(repo, "commit", "-q", "-m", "base 3 rows")
(repo / "sih" / "state" / "plan").mkdir(parents=True)
(repo / "sih" / "state" / "plan" / "demo-pkg.md").write_text("# demo\n", encoding="utf-8")
identity = work / "identity.json"
identity.write_text(json.dumps({"identity":{"hash":"a"*64,"salt":"b"*64,"string":"v3|"},"observed":{"pid":"99999","ppid":"2","hostname":"ZHQHOSTMARK","mac":"0123456789ab","user":"u"}}), encoding="utf-8")
from lease.cli import main as lease_main
def run(*argv):
    out = io.StringIO()
    with redirect_stdout(out):
        code = lease_main(list(argv))
    return code, json.loads(out.getvalue())
code, _ = run("open","--package","demo-pkg","--identity",str(identity),"--intent",INTENT,"--repo",str(repo),"--root",str(work/"main"),"--ledger",str(led),"--at","2026-09-06T12:30:00+00:00")
assert code == 0
wt = work / "main" / "worktrees" / "sih-engine" / "demo-pkg"
# 分支 settle 形改台账：工地提交 2 行新台账行
branch_led = wt / LEDGER_REL
branch_led.write_text(led.read_text(encoding="utf-8") + "".join(json.dumps(r, sort_keys=True, separators=(",",":"))+"\n" for r in [{"event":"issued","package":"branch-add","session_id":"BR1","issued_at":"2026-09-06T12:40:00+00:00"},{"event":"issued","package":"branch-add","session_id":"BR2","issued_at":"2026-09-06T12:41:00+00:00"}]), encoding="utf-8")
g(wt, "-c","user.email=t@t","-c","user.name=t","add","-A"); g(wt, "-c","user.email=t@t","-c","user.name=t","commit","-q","-m","branch settle ledger rows")
# 主树活写 2 行（未提交）
from lease.ledgerwrite import append_row
append_row(led, {"event":"issued","package":"live-writer","session_id":"LV1","issued_at":"2026-09-06T12:50:00+00:00"})
append_row(led, {"event":"issued","package":"live-writer","session_id":"LV2","issued_at":"2026-09-06T12:51:00+00:00"})
before = {r["session_id"] for r in rows(led)}
code, payload = run("close","--package","demo-pkg","--root",str(work/"main"),"--ledger",str(led),"--locks",str(work/"locks.ndjson"))
after = rows(led)
after_ids = {r["session_id"] for r in after}
stash_empty = g(repo, "stash", "list").strip() == ""
result = {
    "scenario": "并集窗探针：分支改台账（settle 形 2 行）加主树活写 2 行，close 处置",
    "close_exit": code,
    "close_error_head": (payload.get("error","")[:160] if code != 0 else ""),
    "rows_before_close_ids": sorted(before),
    "rows_after_ids": sorted(after_ids),
    "base_rows_survived": before.issubset(after_ids),
    "branch_rows_in_working_tree": {"BR1","BR2"}.issubset(after_ids),
    "revoked_appended": any(r.get("event")=="revoked" for r in after),
    "stash_list_empty": stash_empty,
    "zero_loss_invariant": before.issubset(after_ids) and stash_empty,
}
print(json.dumps(result, ensure_ascii=False, indent=1))
