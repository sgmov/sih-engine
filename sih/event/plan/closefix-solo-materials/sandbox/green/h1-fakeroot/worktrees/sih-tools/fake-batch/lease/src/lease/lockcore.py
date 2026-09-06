"""锁本体核心：悲观锁验人验范围，乐观锁验基线。

悲观链五验即会话档在册加范围加身份加绑定，吃工地外壳台账与
正身侧档。乐观链吃级联边册与书简链。锁台账追加即唯一写点。
"""

import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path

TOOL_NAME = "lease"
BINDING_KEYS = ("hostname", "user", "boottime")
DOC_PREFIX = "sih-engine/doc/"


class LocksError(ValueError):
    """工具异常即文件不可读或 json 非法或级联不在或多在册无参，退出码二。"""


class LockBlocked(ValueError):
    """拦截即五验失败或锁态冲突或脏上游，退出码一，理由码承载。"""

    def __init__(self, reason, detail=None):
        super().__init__(reason)
        self.reason = reason
        self.detail = detail or {}


def now_iso():
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def _version():
    from lease import __version__

    return __version__


def load_events(path, kind):
    """ndjson 台账逐行读，缺席即空册。"""
    path = Path(path)
    if not path.exists():
        return []
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise LocksError(f"{kind} unreadable: {path}") from exc
    events = []
    for number, line in enumerate(text.splitlines(), start=1):
        if not line.strip():
            continue
        try:
            events.append(json.loads(line))
        except json.JSONDecodeError as exc:
            raise LocksError(f"{kind} line {number} invalid: {exc}") from exc
    return events


def append_event(path, event):
    """锁台账追加即唯一写点。（pk057fix: 委托 ledgerwrite flock 串行）"""
    from lease.ledgerwrite import append_row

    try:
        append_row(Path(path), event)
    except OSError as exc:
        raise LocksError(f"locks ledger unwritable: {path}") from exc


def active_sessions(events):
    """在册会话即按事件序配对：issued 置位、revoked 仅弹在册同号。

    同号重开不因旧 revoked 误抹承 pk-031 证据二与 leasepatch-solo 缺陷一，
    单遍事件序即时序语义，重复 issued 或 revoked 均幂等。
    """
    sessions = {}
    for event in events:
        sid = event.get("session_id")
        if event.get("event") == "issued":
            sessions[sid] = event
        elif event.get("event") == "revoked" and sid in sessions:
            sessions.pop(sid)
    return sessions


def select_session(events, session_id=None):
    """会话择定：显式号或在册唯一，零在册即拦，多在册无参即异常。"""
    actives = list(active_sessions(events).values())
    if session_id is None:
        if not actives:
            raise LockBlocked("no_active_session")
        if len(actives) > 1:
            raise LocksError("multiple active sessions, --session required")
        return actives[0]
    for active in actives:
        if active.get("session_id") == session_id:
            return active
    raise LockBlocked("session_not_active", {"wanted": session_id})


def scope_allows(allow, path):
    """范围验：条目精确等于或目录前缀。"""
    for entry in allow:
        entry = str(entry).rstrip("/")
        if entry == path or path.startswith(entry + "/"):
            return True
    return False


def load_identity_report(path):
    """正身报告读三样：异常点计数与 observed 与身份哈希。"""
    try:
        payload = json.loads(Path(path).read_text(encoding="utf-8"))
    except OSError as exc:
        raise LocksError(f"identity report unreadable: {path}") from exc
    except json.JSONDecodeError as exc:
        raise LocksError(f"identity report invalid: {exc}") from exc
    if not isinstance(payload, dict):
        raise LocksError("identity report top level must be an object")
    anomalies = payload.get("anomalies")
    anomalies = anomalies if isinstance(anomalies, list) else []
    observed = payload.get("observed")
    observed = observed if isinstance(observed, dict) else {}
    identity = payload.get("identity")
    identity_hash = None
    if isinstance(identity, dict) and isinstance(identity.get("hash"), str):
        identity_hash = identity["hash"]
    return anomalies, observed, identity_hash


def load_binding(worktrees_root, session_id):
    """绑定侧档即工地根 .bindings 下按会话号。"""
    path = Path(worktrees_root) / ".bindings" / f"{session_id}.json"
    if not path.is_file():
        raise LockBlocked("binding_absent", {"expected": str(path)})
    try:
        binding = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise LocksError(f"binding sidecar invalid: {path}") from exc
    if not isinstance(binding, dict):
        raise LocksError(f"binding sidecar not an object: {path}")
    return binding


def verify_binding(binding, observed):
    """绑定验：三稳定子集与报告 observed 全等，失配即漂移拦。"""
    for key in BINDING_KEYS:
        if str(observed.get(key, "")) != str(binding.get(key, "")):
            raise LockBlocked(f"{key}_drift", {
                "expected": binding.get(key, ""),
                "observed": str(observed.get(key, "")),
            })


def verify_five(session, path, identity_path, worktrees_root):
    """悲观链五验：在册已由会话择定承，余四验即范围加身份加绑定。"""
    if not scope_allows(session.get("allow", []), path):
        raise LockBlocked("scope_violation", {
            "allow": session.get("allow", []),
            "path": path,
        })
    anomalies, observed, _ = load_identity_report(identity_path)
    if anomalies:
        raise LockBlocked("identity_anomaly", {"anomalies": anomalies})
    binding = load_binding(worktrees_root, session["session_id"])
    verify_binding(binding, observed)
    return observed


def active_locks(events, root=None):
    """在锁即 acquired 无 released 配对，按 path 映射持锁会话列表。

    单遍事件序配对即 acquired 追加、released 仅弹同号同路径，同路径重取
    不被旧 released 误抹承 leasepatch-solo 缺陷一修法；路径双侧规范化后
    比较承 locksplit-solo 路径同一化。锁型缺省 exclusive 向后兼容；双
    append 共存即同 path 多持位，exclusive 持位下同 path 至多一持位。

    ORD-020 互斥语义在锁映射（同一 path 至多一 exclusive 持锁会话，append
    可多持），推导档 sih-math/docs/ordwire-lease-derivation-2026-09-03.md §3.1。
    """
    held = {}
    for event in events:
        if event.get("event") == "acquired":
            key = normalize_path(event["path"], root)
            sid = event["session_id"]
            mode = event.get("mode", "exclusive")
            holders = held.setdefault(key, [])
            if not any(s == sid for s, _ in holders):
                holders.append((sid, mode))
        elif event.get("event") == "released":
            key = normalize_path(event["path"], root)
            holders = held.get(key)
            if holders:
                remaining = [(s, m) for (s, m) in holders if s != event.get("session_id")]
                if remaining:
                    held[key] = remaining
                else:
                    held.pop(key, None)
    return held


def acquire(path, identity_path, session_id, root, locks_ledger, worktree_ledger, at=None, worktrees_root=None, mode="exclusive", wait=False):
    """悲观锁取锁：五验后落 acquired 行，他持即拦，同会话幂等记行。

    ORD-020 互斥+死锁不自由：撞锁所拒 locked_elsewhere 即等待不绕行，
    推导档 sih-math/docs/ordwire-lease-derivation-2026-09-03.md §3.2。
    """
    path = normalize_path(path, root)
    session = select_session(load_events(worktree_ledger, "worktree ledger"), session_id)
    if worktrees_root is None:
        worktrees_root = Path(root) / "worktrees"
    verify_five(session, path, identity_path, worktrees_root)
    # leaseopt-lockdb-solo：判定正典换 SQLite 事务（BEGIN IMMEDIATE 写者串行，
    # 插队机制性不可能），互斥谓词与 1.14.0 语义同义；本函数旧读判逻辑退役
    # 为 ndjson 镜像追加（引擎 lockgate 与既有读者照旧可见）。
    from lease.lockdb import LockConflict, try_acquire, enqueue, queue_head, dequeue
    try:
        result = try_acquire(locks_ledger, path, session["session_id"], mode, at=at)
    except LockConflict as exc:
        # leaseopt-lockqueue-solo 排队调度：--wait 撞锁入队受理返位次；
        # 队列存在时非队首的无 wait 撞锁拒 queued_not_your_turn（保序公平，
        # 后来者不再反复撞墙）；队首者锁真被占仍 locked_elsewhere。
        head = queue_head(locks_ledger, path)
        if wait:
            position = enqueue(locks_ledger, path, session["session_id"], mode, at=at)
            return {"queued": True, "position": position, "path": path,
                    "line": {"event": "queued", "path": path,
                             "session_id": session["session_id"], "position": position}}
        if head is not None and head != session["session_id"] and exc.detail.get("path") == path:
            raise LockBlocked("queued_not_your_turn", {
                "path": path, "queue_head": head, "holder": exc.detail.get("holder"),
            })
        raise LockBlocked("locked_elsewhere", exc.detail)
    dequeue(locks_ledger, path, session["session_id"])
    duplicate = result["duplicate"]
    line = {
        "acquired_at": at if at is not None else now_iso(),
        "event": "acquired",
        "path": path,
        "session_id": session["session_id"],
        "mode": mode,
        "tool": {"name": TOOL_NAME, "version": _version()},
    }
    append_event(locks_ledger, line)
    return {"duplicate": duplicate, "line": line}


def release(path, identity_path, session_id, root, locks_ledger, worktree_ledger, at=None, worktrees_root=None):
    """放锁：同一五验加持锁验，非持者拦。

    ORD-020 等待终止性让步良基：放锁落 released 行即让路，等待方获放行，
    推导档 sih-math/docs/ordwire-lease-derivation-2026-09-03.md §3.3。
    """
    path = normalize_path(path, root)
    session = select_session(load_events(worktree_ledger, "worktree ledger"), session_id)
    if worktrees_root is None:
        worktrees_root = Path(root) / "worktrees"
    verify_five(session, path, identity_path, worktrees_root)
    # leaseopt-lockdb-solo：放锁判定正典同换 SQLite 事务，持锁校验在事务内；
    # ndjson 镜像追加保留
    from lease.lockdb import LockConflict, try_release
    try:
        try_release(locks_ledger, path, session["session_id"], at=at)
    except LockConflict as exc:
        reason = exc.detail.get("reason", "locked_elsewhere")
        raise LockBlocked(reason, exc.detail)
    line = {
        "event": "released",
        "path": path,
        "released_at": at if at is not None else now_iso(),
        "session_id": session["session_id"],
        "tool": {"name": TOOL_NAME, "version": _version()},
    }
    append_event(locks_ledger, line)
    return {"line": line}


def lock_status(locks_ledger, path=None, session_id=None, root=None):
    """查锁：在锁按 path 与会话过滤。

    ORD-020 全序读出（leasewire-solo 增注）：held 按（path, acquired_at）
    字典序排序输出，即资源全序的一次字典序展开，全序性可机械对表，
    推导档 sih-math/docs/leasewire-derivation-2026-09-04.md lease 节全序
    读出小节。
    """
    events = load_events(locks_ledger, "locks ledger")
    held = active_locks(events, root)
    lines = [event for event in events if event.get("event") == "acquired"]
    report = []
    for line in lines:
        key = normalize_path(line["path"], root)
        if not any(s == line["session_id"] for s, _ in held.get(key, [])):
            continue
        if path and key != normalize_path(path, root):
            continue
        if session_id and line["session_id"] != session_id:
            continue
        report.append(line)
    return {
        "header": {"locks_ledger": str(locks_ledger), "tool": {"name": TOOL_NAME, "version": _version()}},
        "held": sorted(report, key=lambda item: (item["path"], item["acquired_at"])),
        "summary": {"held": len(report)},
    }


def normalize_path(path, root=None):
    """路径同一化：绝对路径剥工作区根前缀，去 ./ 前缀，尾斜杠统一。

    ORD-020 全序资源标识统一：归一化路径即资源集 R 上的一致标识，
    推导档 sih-math/docs/ordwire-lease-derivation-2026-09-03.md §3.2。
    """
    s = str(path)
    if root is not None:
        root_s = str(root).rstrip("/")
        if s.startswith(root_s + "/"):
            s = s[len(root_s) + 1:]
    if s.startswith("./"):
        s = s[2:]
    if s.endswith("/") and s != "/":
        s = s.rstrip("/")
    return s


def _is_ancestor(ancestor, descendant):
    """祖先判定：ancestor 为 descendant 的目录前缀。

    ORD-020 互斥闭包（leasewire-solo 增注）：祖先与后代即包含偏序，
    互斥关系在资源包含序上闭包，目录文件双持洞不绕穿，推导档
    sih-math/docs/leasewire-derivation-2026-09-04.md lease 节互斥闭包小节。
    """
    a = ancestor.rstrip("/")
    d = descendant.rstrip("/")
    return d.startswith(a + "/")


def _conflicts(held_path, held_mode, new_path, new_mode):
    """锁冲突判定：同路径或包含关系即冲突，双 append 共存除外。

    承 locksplit-solo 路径包含判定：目录锁与文件锁同底层面双持即互斥，
    堵目录文件双持洞；追加态锁即 append 与 append 可共存、exclusive 与
    任何形态互斥。

    ORD-020 载体（leasewire-solo 增注）：冲突判定即资源全序分配上的
    互斥谓词，conflict 关系在同路径与包含偏序上闭包，推导档
    sih-math/docs/leasewire-derivation-2026-09-04.md lease 节互斥闭包小节。
    """
    if held_path == new_path:
        return not (held_mode == "append" and new_mode == "append")
    if _is_ancestor(held_path, new_path) or _is_ancestor(new_path, held_path):
        return not (held_mode == "append" and new_mode == "append")
    return False


def run_cascade_check(cascade_dir, registry, doc_root, trails, reports_root):
    """子进程调级联 check，--trail 可重复按参序接续。"""
    command = [
        "uv", "run", "--project", str(cascade_dir), "cascade", "check",
        "--registry", str(registry),
        "--root", str(doc_root),
        "--reports-root", str(reports_root),
    ]
    for trail in trails:
        command.extend(["--trail", str(trail)])
    try:
        proc = subprocess.run(command, capture_output=True, text=True, timeout=120)
    except (OSError, subprocess.SubprocessError) as exc:
        raise LocksError(f"cascade invocation failed: {exc}") from exc
    if proc.returncode == 2:
        raise LocksError(f"cascade check errored: {proc.stdout.strip()[:200]}")
    try:
        return json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        raise LocksError(f"cascade report unparsable: {exc}") from exc


def check_baseline(path, registry, doc_root, trails, cascade_dir, reports_root, locks_ledger):
    """乐观锁验基线：脏上游拦，unverified 不拦，不在册过注记。"""
    path = normalize_path(path)
    if not path.startswith(DOC_PREFIX):
        raise LocksError(f"path outside cascade corpus, expect {DOC_PREFIX} prefix: {path}")
    doc_rel = path[len(DOC_PREFIX):]
    report = run_cascade_check(cascade_dir, registry, doc_root, trails, reports_root)
    targets = report.get("targets", {})
    target = targets.get(doc_rel)
    if target is None:
        verdict = "unknown_target"
        dirty = []
    else:
        dirty = target.get("dirty", [])
        verdict = "blocked" if dirty else "writable"
    line = {
        "dirty": dirty,
        "event": "checked",
        "path": path,
        "tool": {"name": TOOL_NAME, "version": _version()},
        "verdict": verdict,
    }
    append_event(locks_ledger, line)
    return {"dirty": dirty, "line": line, "verdict": verdict}
