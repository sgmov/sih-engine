"""台账唯一写点（pk057fix-solo，pk-057 根因硬化）：flock 串行 + 原子整行追加。

纪律三条：
1. 所有台账行写入必须走 append_row，禁止任何整文件重写路径；
2. 合并窗口（备份→checkout→回补）必须持 ledger_lock，与追加互斥；
3. 行格式字节不变（sort_keys 紧凑形 + 换行），既有台账全史零迁移。

适用边界：flock 语义在本地文件系统（APFS/ext4）可靠，NFS 类文件系统不承诺。
"""

import json
import os
from contextlib import contextmanager
from pathlib import Path

try:
    import fcntl
except ImportError:  # pragma: no cover - 非 POSIX 环境显式失败
    fcntl = None


def _row_bytes(event) -> bytes:
    """行字节序列：与既有台账行格式逐字节一致（sort_keys 紧凑形 + 换行）。"""
    return json.dumps(
        event, ensure_ascii=False, sort_keys=True, separators=(",", ":")
    ).encode("utf-8") + b"\n"


@contextmanager
def ledger_lock(path):
    """台账互斥窗：对 path+".lock" 取排他 flock，跨进程串行。

    合并窗口（备份→checkout→回补）必须整体持本锁，与 append_row 的
    短锁互斥，窗口内并发追加被阻塞至窗口关闭——零交错零丢行。
    """
    if fcntl is None:  # pragma: no cover
        raise OSError("ledger flock 需 POSIX fcntl")
    lock_path = Path(str(path) + ".lock")
    lock_path.parent.mkdir(parents=True, exist_ok=True)
    fd = os.open(lock_path, os.O_RDWR | os.O_CREAT, 0o644)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX)
        yield
    finally:
        fcntl.flock(fd, fcntl.LOCK_UN)
        os.close(fd)


def append_row(path, event) -> dict:
    """唯一台账写点：flock 串行 + 单次 os.write 原子整行追加。

    O_APPEND 保证写指针恒在文件尾；flock 保证多进程串行；整行一次
    write 消除行内撕裂。返回 {"bytes": len(payload), "lines": 1}。
    """
    path = Path(path)
    payload = _row_bytes(event)
    with ledger_lock(path):
        fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_APPEND, 0o644)
        try:
            written = 0
            while written < len(payload):
                written += os.write(fd, payload[written:])
        finally:
            os.close(fd)
    return {"bytes": len(payload), "lines": 1}


def restore_missing(path, backup_path) -> int:
    """回补网（pk057fix）：备份行集中缺失于现文件的行逐行回补。

    合并窗口 checkout 覆盖后调用——备份（覆盖前快照）里有而现文件没有
    的行即窗口期活写行，按原字节序逐行 append_row 回补。返回回补行数。
    """
    path = Path(path)
    backup = Path(backup_path)
    backup_rows = [
        ln
        for ln in backup.read_text(encoding="utf-8").splitlines()
        if ln.strip()
    ]
    existing = set(
        Path(path).read_text(encoding="utf-8").splitlines()
    ) if Path(path).exists() else set()
    restored = 0
    for row in backup_rows:
        if row not in existing:
            append_row(path, json.loads(row))
            existing.add(row)
            restored += 1
    return restored
