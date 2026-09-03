"""mathclose-solo 件一 C006 冲突处置：状态行全角括注转半角括注。

背景：VERSION 规定「（<波名> 增补）」中文全角括注形被 des-001-mathe C006 否决
（全角括号仅允英文 slug/缩写/单一治理编号）。用户跳过裁定，采最小偏离方案 opt1，
保留增补语义，仅将状态行该对全角括注 （ ） 转为半角 ( )（C006 只扫全角，实测 exit 0）。

纪律：只改以「状态：」开头的状态行，恰一对括注，其余行零改。
"""

import pathlib
import subprocess

M = pathlib.Path("/Users/moc/workspaces/SiHankor/worktrees/sih-math/mathclose-solo")
ROOT = "/Users/moc/workspaces/SiHankor"

def changed_md():
    r = subprocess.run(
        ["git", "-C", str(M), "diff", "--name-only", "--", "*.md"],
        capture_output=True, text=True, check=True,
    )
    return [line for line in r.stdout.splitlines() if line.strip()]

def main():
    files = changed_md()
    converted = []
    skipped = []
    for rel in files:
        p = M / rel
        raw = p.read_bytes()
        text = raw.decode("utf-8")
        lines = text.split("\n")
        done = False
        for i, line in enumerate(lines):
            if line.startswith("状态："):
                op = line.find("（")
                cl = line.find("）")
                if op == -1 or cl == -1 or cl < op:
                    skipped.append(rel)
                    break
                new_line = line[:op] + "(" + line[op+1:cl] + ")" + line[cl+1:]
                if new_line != line:
                    lines[i] = new_line
                    converted.append(rel)
                    done = True
                break
        if done:
            p.write_text("\n".join(lines), encoding="utf-8", newline="")
    print(f"converted={len(converted)} skipped={len(skipped)} total={len(files)}")
    for s in skipped:
        print("SKIP:", s)

if __name__ == "__main__":
    main()