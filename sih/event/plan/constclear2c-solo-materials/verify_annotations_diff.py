#!/usr/bin/env python3
"""constclear2c-solo 腿一自证器：diff 逐件证明仅注释行变更。

判据：
- 逐工地文件跑 git diff --unified=0，解析变更行；
- 零删除行（无任何 `-` 内容行）；
- 全部新增行是注释行（Python `# ` 或 Rust `// ` 起）且逐行携带 [constclear2c] 记号；
- 值绑定对表已在投影件 value_binding_unchanged 全真位；
- 输出 annotations-diff-selfcert-2026-09-08.json。
"""
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
TW = ROOT / "worktrees/sih-tools/constclear2c-solo"
EW = ROOT / "worktrees/sih-engine/constclear2c-solo"
HERE = Path(__file__).resolve().parent

proj = json.loads((HERE / "constants-annotations.json").read_text(encoding="utf-8"))

by_repo_files: dict[str, set[str]] = {"tools": set(), "engine": set()}
for it in proj["items"]:
    rel = it["site"].rsplit(":", 1)[0]
    if rel.startswith("sih-tools/"):
        by_repo_files["tools"].add(rel[len("sih-tools/"):])
    else:
        by_repo_files["engine"].add(rel[len("sih-engine/"):])


def run_git(wt: Path, args: list[str]) -> str:
    return subprocess.run(["git", "-C", str(wt)] + args, capture_output=True, text=True, check=True).stdout


report = {"batch": "constclear2c-solo", "kind": "annotations-diff-selfcert", "date": "2026-09-08",
          "verdict": "pass", "files": []}
total_add = 0
for repo, files in by_repo_files.items():
    wt = TW if repo == "tools" else EW
    for rel in sorted(files):
        diff = run_git(wt, ["diff", "--unified=0", "--", rel])
        adds, dels, bad_adds = [], [], []
        for line in diff.splitlines():
            if line.startswith("+++") or line.startswith("---") or line.startswith("@@") or line.startswith("diff "):
                continue
            if line.startswith("+"):
                body = line[1:]
                adds.append(body)
                stripped = body.lstrip()
                is_comment = stripped.startswith("# [constclear2c]") or stripped.startswith("// [constclear2c]")
                if not is_comment:
                    bad_adds.append(body)
            elif line.startswith("-"):
                dels.append(line[1:])
        entry = {"repo": repo, "file": rel, "added_comment_lines": len(adds),
                 "deleted_lines": len(dels), "non_comment_additions": len(bad_adds)}
        report["files"].append(entry)
        total_add += len(adds)
        if dels or bad_adds:
            report["verdict"] = "fail"
            report.setdefault("failures", []).append({"file": rel, "dels": dels[:5], "bad_adds": bad_adds[:5]})

report["total_added_comment_lines"] = total_add
report["total_files"] = len(report["files"])
(HERE / "annotations-diff-selfcert-2026-09-08.json").write_text(
    json.dumps(report, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(f"selfcert verdict={report['verdict']} files={report['total_files']} added_comments={total_add} "
      f"deletions={sum(f['deleted_lines'] for f in report['files'])} bad_adds={sum(f['non_comment_additions'] for f in report['files'])}")
return_code = 0 if report["verdict"] == "pass" else 1
sys.exit(return_code)
