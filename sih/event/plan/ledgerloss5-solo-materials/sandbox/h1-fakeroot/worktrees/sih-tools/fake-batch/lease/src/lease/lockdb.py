"""锁库：SQLite 现势锁表加只 INSERT 事件表，事务化取放锁判定门。

承 leaseopt-lockdb-solo（leaseopt 线批四，2026-09-05 用户租约模式定稿）：
锁状态入 SQLite WAL 现势表，acquire/release 单事务判定（BEGIN IMMEDIATE
即写者串行，插队机制性不可能）；事件表只 INSERT 不 UPDATE 即历史不可
篡改（工程基线四），现势态与历史账分置；ndjson 台账降为镜像面——每次
取放锁同步追加镜像行，引擎 lockgate 与既有读者照旧可见，判定语义退役
镜像语义保留。判定正典即 SQLite 事务，审计账即镜像 ndjson（入版控
append-only），两层面各司其职。

锁库原子性载体 ORD-020 全序资源分配与死锁自由（CONTRACT 载体引用节
既有挂点扩展），推导档 sih-math/docs/leaseopt-lockdb-derivation-2026-09-05.md。
"""

import json
import sqlite3
from datetime import datetime, timezone
from pathlib import Path

TOOL_NAME = "lease"

HEARTBEAT_STALE_SECONDS = 300  # 冻结登记：改它即改僵尸判定，清账路径指向批五载体批


def now_iso():
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def db_path_for(locks_ledger):
    """锁库路径：与 ndjson 台账同目录 locks.db。"""
    return Path(locks_ledger).parent / "locks.db"


def _connect(db):
    conn = sqlite3.connect(str(db), timeout=10.0)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA busy_timeout=10000")
    conn.row_factory = sqlite3.Row
    return conn


def _is_ancestor(ancestor, descendant):
    a = ancestor.rstrip("/") + "/"
    d = descendant.rstrip("/") + "/"
    return d.startswith(a) and a != "/"


def _conflicts(held_path, held_mode, new_path, new_mode):
    """锁冲突判定：与 lockcore._conflicts 同义（同路径或包含即冲突，双 append 共存除外）。

    双处同义由 test_lockdb.py 语义锚定测试锚定，语义变更须两处同批。
    """
    if held_path == new_path:
        return not (held_mode == "append" and new_mode == "append")
    if _is_ancestor(held_path, new_path) or _is_ancestor(new_path, held_path):
        return not (held_mode == "append" and new_mode == "append")
    return False


def ensure_db(locks_ledger):
    """建库建表；locks.db 缺而 ndjson 在即迁移（全史入事件表，现势重放入状态表）。幂等。

    迁移后 ndjson 冻结判定只作镜像（acquire/release 继续追加镜像行）。
    """
    db = db_path_for(locks_ledger)
    if db.exists():
        return db
    db.parent.mkdir(parents=True, exist_ok=True)
    conn = _connect(db)
    try:
        conn.executescript(
            """
            CREATE TABLE IF NOT EXISTS lock_state (
                path TEXT NOT NULL,
                session_id TEXT NOT NULL,
                mode TEXT NOT NULL DEFAULT 'exclusive',
                acquired_at TEXT NOT NULL,
                heartbeat_at TEXT,
                PRIMARY KEY (path, session_id)
            );
            CREATE TABLE IF NOT EXISTS lock_event (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts TEXT NOT NULL,
                event TEXT NOT NULL,
                path TEXT NOT NULL,
                session_id TEXT NOT NULL,
                mode TEXT,
                detail TEXT
            );
            CREATE TABLE IF NOT EXISTS lock_queue (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL,
                session_id TEXT NOT NULL,
                mode TEXT NOT NULL,
                enqueued_at TEXT NOT NULL
            );
            """
        )
        ledger = Path(locks_ledger)
        if ledger.exists():
            migrated = 0
            for line in ledger.read_text(encoding="utf-8").splitlines():
                line = line.strip()
                if not line:
                    continue
                try:
                    ev = json.loads(line)
                except ValueError:
                    continue
                if ev.get("event") not in ("acquired", "released"):
                    continue
                conn.execute(
                    "INSERT INTO lock_event(ts,event,path,session_id,mode,detail) VALUES(?,?,?,?,?,?)",
                    (ev.get("acquired_at") or ev.get("released_at") or now_iso(),
                     ev["event"], ev["path"], ev.get("session_id", ""),
                     ev.get("mode"), json.dumps(ev, ensure_ascii=False)),
                )
                migrated += 1
            # 现势重放：单遍事件序配对（released 弹同会话同路径）
            state = {}
            for row in conn.execute("SELECT * FROM lock_event ORDER BY id"):
                key = row["path"]
                if row["event"] == "acquired":
                    state.setdefault(key, []).append((row["session_id"], row["mode"] or "exclusive", row["ts"]))
                else:
                    holders = state.get(key)
                    if holders:
                        remaining = [h for h in holders if h[0] != row["session_id"]]
                        if remaining:
                            state[key] = remaining
                        else:
                            state.pop(key, None)
            for key, holders in state.items():
                for sid, mode, ts in holders:
                    conn.execute(
                        "INSERT OR REPLACE INTO lock_state(path,session_id,mode,acquired_at,heartbeat_at) VALUES(?,?,?,?,?)",
                        (key, sid, mode, ts, ts),
                    )
            conn.execute(
                "INSERT INTO lock_event(ts,event,path,session_id,mode,detail) VALUES(?,?,?,?,?,?)",
                (now_iso(), "migrated", str(ledger), "", None,
                 json.dumps({"migrated_lines": migrated}, ensure_ascii=False)),
            )
        conn.commit()
    finally:
        conn.close()
    return db


def _held_rows(conn):
    return {(r["path"], r["session_id"]): r for r in conn.execute("SELECT * FROM lock_state")}


def try_acquire(locks_ledger, path, session_id, mode, at=None):
    """事务化取锁：BEGIN IMMEDIATE 写者串行，冲突即拒不等待。

    返回 {"duplicate": bool}；冲突 raise LockConflict（detail 带 held 信息）。
    镜像行不在此写——调用方事务成功后自行 append ndjson 镜像（两步非原子
    即镜像可能落后事务，镜像只作可见性不作判定，如实语义）。
    """
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        conn.execute("BEGIN IMMEDIATE")
        rows = _held_rows(conn)
        duplicate = False
        for (held_path, held_sid), row in rows.items():
            if held_sid == session_id and held_path == path:
                duplicate = True
                continue
            if _conflicts(held_path, row["mode"], path, mode):
                conn.execute("ROLLBACK")
                raise LockConflict({
                    "path": path, "holder": held_sid,
                    "held_path": held_path, "held_mode": row["mode"],
                })
        ts = at if at is not None else now_iso()
        conn.execute(
            "INSERT OR REPLACE INTO lock_state(path,session_id,mode,acquired_at,heartbeat_at) VALUES(?,?,?,?,?)",
            (path, session_id, mode, ts, ts),
        )
        conn.execute(
            "INSERT INTO lock_event(ts,event,path,session_id,mode,detail) VALUES(?,?,?,?,?,?)",
            (ts, "acquired", path, session_id, mode, None),
        )
        conn.commit()
        return {"duplicate": duplicate}
    finally:
        conn.close()


def try_release(locks_ledger, path, session_id, at=None):
    """事务化放锁：持锁校验在事务内，非持者或未锁即拒。"""
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        conn.execute("BEGIN IMMEDIATE")
        all_rows = conn.execute(
            "SELECT * FROM lock_state WHERE path=?", (path,)
        ).fetchall()
        if not all_rows:
            conn.execute("ROLLBACK")
            raise LockConflict({"reason": "not_locked", "path": path})
        own = [r for r in all_rows if r["session_id"] == session_id]
        if not own:
            conn.execute("ROLLBACK")
            raise LockConflict({"reason": "not_holder", "path": path, "holder": all_rows[0]["session_id"]})
        conn.execute(
            "DELETE FROM lock_state WHERE path=? AND session_id=?",
            (path, session_id),
        )
        ts = at if at is not None else now_iso()
        conn.execute(
            "INSERT INTO lock_event(ts,event,path,session_id,mode,detail) VALUES(?,?,?,?,?,?)",
            (ts, "released", path, session_id, None, None),
        )
        conn.commit()
        return True
    finally:
        conn.close()


def heartbeat(locks_ledger, session_id, at=None):
    """心跳：刷新该会话全部锁行的 heartbeat_at，返回刷新行数。"""
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        ts = at if at is not None else now_iso()
        cur = conn.execute(
            "UPDATE lock_state SET heartbeat_at=? WHERE session_id=?",
            (ts, session_id),
        )
        conn.commit()
        return cur.rowcount
    finally:
        conn.close()


def holds(locks_ledger):
    """现势持锁映射：{path: [(session_id, mode)]}，语义同 lockcore.active_locks。"""
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        held = {}
        for r in conn.execute("SELECT * FROM lock_state"):
            held.setdefault(r["path"], []).append((r["session_id"], r["mode"]))
        return held
    finally:
        conn.close()


def stale_holds(locks_ledger, stale_seconds=HEARTBEAT_STALE_SECONDS, at=None):
    """停滞锁读数：heartbeat_at 距参照时超阈的现势行清单，视图告警数据源。"""
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        ref = at or now_iso()
        from datetime import datetime as _dt
        ref_t = _dt.fromisoformat(ref)
        out = []
        for r in conn.execute("SELECT * FROM lock_state"):
            hb = r["heartbeat_at"] or r["acquired_at"]
            try:
                age = (ref_t - _dt.fromisoformat(hb)).total_seconds()
            except ValueError:
                age = None
            if age is None or age > stale_seconds:
                out.append({"path": r["path"], "session_id": r["session_id"],
                            "mode": r["mode"], "stale_seconds": age})
        return out
    finally:
        conn.close()


class LockConflict(ValueError):
    """锁库冲突：detail 带 held 信息，调用方转 LockBlocked 语义。"""

    def __init__(self, detail):
        super().__init__("locked_elsewhere")
        self.detail = detail


def enqueue(locks_ledger, path, session_id, mode, at=None):
    """入队：撞锁 --wait 受理，返回位次（队尾+1，事务内计数）。"""
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        conn.execute("BEGIN IMMEDIATE")
        cur = conn.execute("SELECT COUNT(*) FROM lock_queue WHERE path=?", (path,))
        position = cur.fetchone()[0] + 1
        ts = at if at is not None else now_iso()
        conn.execute(
            "INSERT INTO lock_queue(path,session_id,mode,enqueued_at) VALUES(?,?,?,?)",
            (path, session_id, mode, ts),
        )
        conn.execute(
            "INSERT INTO lock_event(ts,event,path,session_id,mode,detail) VALUES(?,?,?,?,?,?)",
            (ts, "queued", path, session_id, mode,
             json.dumps({"position": position}, ensure_ascii=False)),
        )
        conn.commit()
        return position
    finally:
        conn.close()


def queue_head(locks_ledger, path):
    """队首会话：按入队序最早者；空队返回 None。"""
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        row = conn.execute(
            "SELECT session_id FROM lock_queue WHERE path=? ORDER BY id LIMIT 1",
            (path,),
        ).fetchone()
        return row["session_id"] if row else None
    finally:
        conn.close()


def dequeue(locks_ledger, path, session_id):
    """出队：acquire 成功即清自己在该路径的队列行，返回删除行数。"""
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        cur = conn.execute(
            "DELETE FROM lock_queue WHERE path=? AND session_id=?",
            (path, session_id),
        )
        conn.commit()
        return cur.rowcount
    finally:
        conn.close()


def queue_view(locks_ledger, path=None):
    """队列视图：path 缺省全队列。"""
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        if path:
            rows = conn.execute(
                "SELECT path,session_id,mode,enqueued_at FROM lock_queue WHERE path=? ORDER BY id",
                (path,),
            ).fetchall()
        else:
            rows = conn.execute(
                "SELECT path,session_id,mode,enqueued_at FROM lock_queue ORDER BY id"
            ).fetchall()
        return [dict(r) for r in rows]
    finally:
        conn.close()


def takeover_release(locks_ledger, session_id, at=None):
    """接管放锁：清指定会话全部现势锁行（人节点显式命令专用），返回清行清单。"""
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        conn.execute("BEGIN IMMEDIATE")
        rows = conn.execute(
            "SELECT path,mode FROM lock_state WHERE session_id=?", (session_id,)
        ).fetchall()
        cleared = []
        ts = at if at is not None else now_iso()
        for r in rows:
            conn.execute(
                "DELETE FROM lock_state WHERE path=? AND session_id=?",
                (r["path"], session_id),
            )
            conn.execute(
                "INSERT INTO lock_event(ts,event,path,session_id,mode,detail) VALUES(?,?,?,?,?,?)",
                (ts, "takeover_released", r["path"], session_id, r["mode"], None),
            )
            cleared.append({"path": r["path"], "mode": r["mode"]})
        conn.execute("DELETE FROM lock_queue WHERE session_id=?", (session_id,))
        conn.commit()
        return cleared
    finally:
        conn.close()


def record_stale_cleared(locks_ledger, path, session_id, detail, at=None):
    """stale_cleared 只 INSERT（openhyg-solo）：死 PID 残件机械自清留痕。

    现势锁表与队列零动作，事件表单笔 INSERT，detail 载原检验文件快照即审计账；
    判定语义变更（停滞残件由拒开索人接管改为机械自清留痕）经得一裁承载。
    """
    db = ensure_db(locks_ledger)
    conn = _connect(db)
    try:
        ts = at if at is not None else now_iso()
        cur = conn.execute(
            "INSERT INTO lock_event(ts,event,path,session_id,mode,detail) VALUES(?,?,?,?,?,?)",
            (ts, "stale_cleared", path, session_id or "", None,
             json.dumps(detail, ensure_ascii=False, sort_keys=True)),
        )
        conn.commit()
        return cur.lastrowid
    finally:
        conn.close()
