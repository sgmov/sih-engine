"""生成 calibration 目录的 manifest.json。

承接 R-E1 决策: 走 manifest 加摘要哈希方案, 不走 Git LFS。
- 遍历 sih/state/calibration/ 全部文件
- 对每个文件计算 sha256 与字节大小
- 写 manifest.json: 文件名 (相对 calibration 根) + sha256 + size_bytes + mtime_iso

不修改任何原始文件, 仅写 manifest.json 自身。
"""
from __future__ import annotations

import hashlib
import json
import os
from datetime import datetime, timezone
from pathlib import Path

CALIBRATION_DIR = Path(__file__).resolve().parent
MANIFEST_PATH = CALIBRATION_DIR / "manifest.json"


def hash_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def main() -> None:
    entries = []
    file_count = 0
    total_bytes = 0
    for root, _dirs, files in os.walk(CALIBRATION_DIR):
        # 跳过 manifest.json 自身
        if Path(root) == CALIBRATION_DIR:
            files = [f for f in files if f != "manifest.json" and f != "_gen_manifest.py"]
        for name in sorted(files):
            p = Path(root) / name
            if not p.is_file():
                continue
            rel = p.relative_to(CALIBRATION_DIR).as_posix()
            st = p.stat()
            sha = hash_file(p)
            entries.append({
                "path": rel,
                "sha256": sha,
                "size_bytes": st.st_size,
                "mtime_iso": datetime.fromtimestamp(st.st_mtime, tz=timezone.utc).isoformat(),
            })
            file_count += 1
            total_bytes += st.st_size

    manifest = {
        "schema_version": "1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "calibration_dir": str(CALIBRATION_DIR.relative_to(Path.cwd())) if CALIBRATION_DIR.is_relative_to(Path.cwd()) else str(CALIBRATION_DIR),
        "file_count": file_count,
        "total_bytes": total_bytes,
        "files": entries,
    }

    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"manifest written: {MANIFEST_PATH}")
    print(f"  files: {file_count}")
    print(f"  total bytes: {total_bytes}")


if __name__ == "__main__":
    main()
