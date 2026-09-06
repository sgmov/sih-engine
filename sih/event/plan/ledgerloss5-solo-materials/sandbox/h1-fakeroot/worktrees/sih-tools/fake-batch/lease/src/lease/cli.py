"""CLI 面：十三子命令 open、claim、unclaim、lock、check、status、unlock、close、commit、reconcile、bypass、install-hooks、uninstall-hooks，json 报告加退出码三值。"""

import argparse
import json
import os
import sqlite3
import subprocess
from datetime import datetime, timezone
from pathlib import Path

from lease import __version__
from lease.claimcore import (
    CLAIMANT_DEFAULT,
    ClaimBlocked,
    ClaimsError,
    PackageAlreadyClaimed,
    claim as claim_register,
    claims_view,
    default_claims_ledger,
    unclaim as claim_release,
)
from lease.commitcore import (
    HOOKS_DIR,
    CommitBlocked,
    commit_staged,
    default_bypass_ledger,
    install_hooks,
    record_bypass,
    reconcile,
    resolve_default_base,
    uninstall_hooks,
)
from lease.core import (
    resolve_root,
    StateError,
    PackageSessionActive,
    WorktreeError,
    close_session,
    open_execute,
    open_preflight,
    resolve_package,
    status,
    tool_dir,
)
from lease.lockcore import (
    LockBlocked,
    LocksError,
    acquire as lock_acquire,
    check_baseline,
    lock_status,
    release as lock_release,
)

_QUIET = False


def _emit(payload, code):
    if _QUIET:
        summary = payload.get("summary", {})
        if not summary:
            for key in ("error", "status", "event", "digest", "verdict"):
                if key in payload:
                    summary = {key: payload[key]}
                    break
        line = {
            "tool": "lease",
            "command": _LAST_COMMAND,
            "code": code,
            "summary": summary,
            "session_id": payload.get("session_id") or payload.get("session", {}).get("session_id"),
        }
        print(json.dumps(line, ensure_ascii=False, sort_keys=True, separators=(",", ":")))
        return code
    print(json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2))
    return code


_LAST_COMMAND = ""


def _emit_error(message, code=2, extra=None):
    payload = {"error": message}
    if extra:
        payload.update(extra)
    return _emit(payload, code)


def _default_trails(root):
    """缺省链枚举两居所并集即引擎新家加工具侧老家，迁链路标机械位承 pk-031。"""
    homes = [
        Path(root) / "sih-engine" / "sih" / "event" / "trail",
        Path(root) / "sih-tools" / "scribe" / "trail",
    ]
    seen, out = set(), []
    for home in homes:
        if home.is_dir():
            for p in sorted(home.glob("*.ndjson")):
                key = str(p.resolve())
                if key not in seen:
                    seen.add(key)
                    out.append(str(p))
    return sorted(out)




def _pid_alive(pid):
    """POSIX kill(pid,0) 探活（openhyg-solo T-4）：PermissionError 视为活（进程存在即活），
    ProcessLookupError 即死，其余 OS 异常判不可知返回 None 由调用方保守处理。"""
    try:
        os.kill(int(pid), 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True
    except (OSError, ValueError, OverflowError):
        return None


def _pid_started(pid):
    """进程启动时刻探针：ps lstart 串即跨启动 PID 复用对冲锚；
    平台取不到返回 None 如实申报（回落单判活且保守判活）。"""
    try:
        proc = subprocess.run(
            ["ps", "-o", "lstart=", "-p", str(int(pid))],
            capture_output=True, text=True, timeout=5,
        )
    except (OSError, ValueError, OverflowError, subprocess.SubprocessError):
        return None
    return proc.stdout.strip() or None


def _classify_window(data, ref):
    """检验文件窗口三态判（openhyg-solo PID 探针正典）：返回 (state, detail)。

    state 四值：pid_active 活窗拒开 / pid_dead 尸体可机械自清 /
    legacy_active 旧形心跳新鲜 / legacy_stale 旧形停滞。生死由进程说了算
    （用户裁定）：pid 探活加启动时刻对表，不符即跨启动复用判死；探针不可知
    保守判活防误删；无 pid 旧形回落心跳停滞判据（旧行为），detail 如实申报判定基准。
    """
    hb = data.get("heartbeat_at") or data.get("opened_at")
    try:
        age = (datetime.fromisoformat(ref) - datetime.fromisoformat(hb)).total_seconds() if hb else None
    except ValueError:
        age = None
    pid = data.get("pid")
    if pid is None:
        base = {"basis": "heartbeat-fallback", "package": data.get("package"),
                "heartbeat_at": hb, "window_session": data.get("session_id")}
        if age is not None and age <= HEARTBEAT_STALE_SECONDS:
            return "legacy_active", base
        base["stale_seconds"] = age
        return "legacy_stale", base
    alive = _pid_alive(pid)
    started = _pid_started(pid)
    rec_started = data.get("pid_started")
    start_match = None
    if rec_started and started:
        start_match = str(started) == str(rec_started)
    detail = {
        "basis": "pid-probe",
        "pid": pid,
        "pid_alive": alive,
        "pid_started_recorded": rec_started,
        "pid_started_probe": started,
        "start_match": start_match,
        "heartbeat_at": hb,
        "stale_seconds": age,
        "window_session": data.get("session_id"),
    }
    if alive is False or start_match is False:
        detail["verdict_reason"] = "process-gone" if alive is False else "start-time-mismatch-pid-reuse"
        return "pid_dead", detail
    if alive is None:
        detail["verdict_reason"] = "probe-unavailable-conservative-alive"
    return "pid_active", detail


def _self_clean_check_file(root, package, opened_at):
    """签发前失败自收桌（openhyg-solo T-3）：删自家检验文件，所有权核验防误删他窗。

    opened_at 与 pid 与己方一致才删（并发竞败场景文件已被他窗覆盖即让位）；
    返回 None 即清净或已让位，返回字符串即清理失败详情（调用方两错并报）。
    """
    stem, _ = resolve_package(package, root, must_exist=False)
    path = _checks_dir(root) / f"{stem}.json"
    if not path.exists():
        return None
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, ValueError) as exc:
        return f"检验文件不可读未清: {exc}"
    if not isinstance(data, dict) or data.get("opened_at") != opened_at or data.get("pid") != os.getpid():
        return None
    try:
        path.unlink()
    except OSError as exc:
        return f"检验文件删除失败: {exc}"
    return None


def _record_stale_cleared(root, stem, locks_ledger, window_session, at, reason, snapshot):
    """stale_cleared 只 INSERT 留痕（openhyg-solo T-4）：现势锁面零动作，
    detail 载原检验文件快照即审计账。锁库缺席不写，写败升 WorktreeError 不掩。"""
    if locks_ledger is None:
        return None
    from lease.lockdb import record_stale_cleared

    rel = Path(root) / "sih-tools" / "lease" / "ledger" / "checks" / f"{stem}.json"
    try:
        return record_stale_cleared(
            locks_ledger, str(rel), window_session or "",
            {"reason": reason, "file_snapshot": snapshot}, at=at,
        )
    except (OSError, sqlite3.Error) as exc:
        raise WorktreeError(f"stale_cleared recording failed: {exc}") from exc


HEARTBEAT_STALE_SECONDS = 300  # 冻结登记承 lockdb 同名常数


def _checks_dir(root):
    return Path(root) / "sih-tools" / "lease" / "ledger" / "checks"


def _check_key_gate(package, root, at, identity_core=None, locks_ledger=None):
    """正身检验钥匙闸（openhyg-solo 第六位·首个副作用位）：返回 None 放行，或 (error, reason, detail) 拒开。

    闸序重排后本闸是 open 首个副作用位：前五位（包校验、正身、意图、同包活跃、
    预检）全只读化已在调用方完成，任何后续失败不再必然留残件。检验文件出生即全形
    ——package 与 opened_at 与 heartbeat_at 与 identity_core 与 pid 与 pid_started 与
    session_id:null 一次落盘，无匿名窗；文件创建原子性（O_EXCL 竞败即拒）与写后
    回读校验照旧。读侧三态判：pid 活窗拒开（同正身误跑双开保护红证位）；pid 尸体
    机械自清（删残件加 stale_cleared 只 INSERT 留原文件快照）后正常续开；无 pid
    旧形回落心跳停滞判据（旧行为），detail 如实申报判定基准。
    """
    stem, _ = resolve_package(package, root, must_exist=False)
    checks = _checks_dir(root)
    checks.mkdir(parents=True, exist_ok=True)
    path = checks / f"{stem}.json"
    ref = at or datetime.now(timezone.utc).isoformat(timespec="seconds")
    if path.exists():
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except ValueError:
            data = {}
        if not isinstance(data, dict):
            data = {}
        state, detail = _classify_window(data, ref)
        if state == "pid_dead":
            snapshot = dict(data)
            try:
                path.unlink()
            except OSError as exc:
                return (
                    "尸体自清失败：检验文件 PID 已死但删除被拒",
                    "stale_clear_failed",
                    {"package": stem, "unlink_error": str(exc), **detail},
                )
            _record_stale_cleared(root, stem, locks_ledger, data.get("session_id"), ref,
                                  "open-gate-self-clear", snapshot)
        elif state == "pid_active":
            return (
                "同跑冲突：该租约已有活跃窗口（PID 探针判活）",
                "window_active",
                {"package": stem, **detail},
            )
        elif state == "legacy_active":
            return (
                "同跑冲突：该租约已有活跃窗口（检验文件心跳新鲜）",
                "same_package_active_window",
                {"package": stem, **detail},
            )
        else:
            return (
                "疑似僵尸遗留：检验文件心跳停滞超阈（旧形无 pid 字段），人节点显式接管",
                "stale_check_file",
                {"package": stem, **detail},
            )
    payload = {
        "package": stem,
        "opened_at": ref,
        "heartbeat_at": ref,
        "identity_core": identity_core,
        "pid": os.getpid(),
        "pid_started": _pid_started(os.getpid()),
        "session_id": None,
    }
    try:
        with open(path, "x", encoding="utf-8") as fh:
            fh.write(json.dumps(payload, ensure_ascii=False, indent=1))
    except FileExistsError:
        return (
            "同跑冲突：检验文件并发创建竞败（他窗口同刻开工）",
            "same_package_active_window",
            {"package": stem},
        )
    _exclusive_touch(path, payload)
    return None


def _exclusive_touch(path, payload):
    """创建竞态兜底：写后回读校验 opened_at 仍为己方。"""
    data = json.loads(path.read_text(encoding="utf-8"))
    if data.get("opened_at") != payload.get("opened_at"):
        raise FileExistsError(str(path))


def _stamp_check_file(root, package, session_id):
    """签发回填（openhyg-solo）：唯一保留的回填字段即 session_id——会话号只能签发时刻
    生成；正身与时间戳与 pid 出生即全（钥匙闸落盘形），本函数零他改。"""
    stem, _ = resolve_package(package, root, must_exist=False)
    path = _checks_dir(root) / f"{stem}.json"
    if not path.exists():
        return
    data = json.loads(path.read_text(encoding="utf-8"))
    data["session_id"] = session_id
    path.write_text(json.dumps(data, ensure_ascii=False, indent=1), encoding="utf-8")


def _write_close_receipt(package, root, report, at):
    """收约凭据：检验文件转写任务包同级为跑完证据，删除活件。"""
    stem, pkg_path = resolve_package(package, root, must_exist=False)
    path = _checks_dir(root) / f"{stem}.json"
    if not path.exists():
        return None
    data = json.loads(path.read_text(encoding="utf-8"))
    data["closed_at"] = at or datetime.now(timezone.utc).isoformat(timespec="seconds")
    data["close_session"] = report.get("session_id")
    receipt_path = Path(pkg_path).parent / f"{stem}.lease-check.json"
    receipt_path.parent.mkdir(parents=True, exist_ok=True)
    receipt_path.write_text(json.dumps(data, ensure_ascii=False, indent=1), encoding="utf-8")
    path.unlink()
    return {"receipt": str(receipt_path), "check_file_removed": str(path)}


def _cmd_heartbeat(args, root, session_ledger, locks_ledger):
    """心跳子命令：刷新锁库 heartbeat_at 与检验文件心跳位。"""
    from lease.lockdb import heartbeat as db_heartbeat
    stem, _ = resolve_package(args.package, root, must_exist=False)
    path = _checks_dir(root) / f"{stem}.json"
    if not path.exists():
        return _emit({"error": "检验文件缺位（未 open 或已收约）", "package": stem}, 1)
    data = json.loads(path.read_text(encoding="utf-8"))
    held = db_heartbeat(locks_ledger, args.session, at=args.at)
    ts = args.at or datetime.now(timezone.utc).isoformat(timespec="seconds")
    data["heartbeat_at"] = ts
    path.write_text(json.dumps(data, ensure_ascii=False, indent=1), encoding="utf-8")
    return _emit({"heartbeat": ts, "lock_rows": held, "package": stem}, 0)




def _cmd_takeover(args, root, locks_ledger):
    """接管：人节点显式清检验文件与僵尸锁（openhyg-solo 判据升级 PID 探针三态判）。

    pid 活窗拒（接管即同跑禁）；pid 尸体机械清（心跳新鲜亦清——死进程不是活窗，
    死文件时间戳被当活进程信号的工具自伤就此根治）；无 pid 旧形回落心跳停滞判据
    （旧行为）。清即删活件 + takeover_release 清该会话全部现势锁行与队列行
    （lock_event 记 takeover_released 只 INSERT），输出可重开。
    """
    from lease.lockdb import takeover_release
    stem, _ = resolve_package(args.package, root, must_exist=False)
    path = _checks_dir(root) / f"{stem}.json"
    if not path.exists():
        return _emit({"error": "检验文件缺位（无在营窗口，可直接 open）", "package": stem}, 1)
    data = json.loads(path.read_text(encoding="utf-8"))
    ref = args.at or datetime.now(timezone.utc).isoformat(timespec="seconds")
    state, detail = _classify_window(data, ref)
    if state in ("pid_active", "legacy_active"):
        return _emit({
            "error": f"接管被拒：窗口存活（判定基准 {detail.get('basis')}）",
            "reason": "window_active",
            "detail": {"package": stem, "window_state": state, **detail},
        }, 1)
    window_session = data.get("session_id")
    cleared = takeover_release(locks_ledger, window_session, at=args.at) if window_session else []
    path.unlink()
    return _emit({
        "takeover": True,
        "package": stem,
        "window_state": state,
        "detail": detail,
        "window_session": window_session,
        "released_locks": cleared,
        "reason": args.reason,
        "note": "检验活件已清，可重新 open；本接管动作请由人节点会话上链留痕",
    }, 0)




def _cmd_wait_turn(args, root, session_ledger, locks_ledger):
    """排队阻塞形态：确保入队，机械轮询至队首且锁空，取锁成功即返回零。

    等待期是普通进程的 sleep 查询（非 LLM 轮询，零 token 消耗）；
    保序由队列全序承保（非队首 acquire 必拒，不存在插队）；
    超时即出队退出码一并如实返回，调用方决定放弃或续等。
    """
    import time as _time
    from lease.lockdb import ensure_db, enqueue, queue_head, dequeue, LockConflict
    from lease.lockcore import LockBlocked
    path = args.path
    started = _time.monotonic()
    queued = False
    while True:
        head = queue_head(locks_ledger, path)
        if head is None or head == args.session:
            try:
                result = lock_acquire(
                    path, args.identity, args.session, root, locks_ledger,
                    session_ledger, at=args.at, mode=args.mode,
                )
            except StateError as exc:
                return _emit({"error": str(exc), "reason": "head_but_locked", "path": path}, 1)
            except Exception as exc:
                # 队首但锁仍被占（locked_elsewhere）即正常等待态：继续轮询；
                # 其他拒绝（五验失败等）如实退
                reason = getattr(exc, "reason", None)
                if reason is None or reason != "locked_elsewhere":
                    return _emit({"error": str(exc), "reason": reason or "acquire_rejected", "path": path}, 1)
                if args.timeout is not None and (_time.monotonic() - started) >= args.timeout:
                    dequeue(locks_ledger, path, args.session)
                    return _emit({
                        "error": "排队等待超时，已出队",
                        "reason": "wait_timeout",
                        "detail": {"path": path, "timeout": args.timeout},
                    }, 1)
                _time.sleep(max(args.interval, 0.1))
                continue
            dequeue(locks_ledger, path, args.session)
            result["waited"] = True
            result["queued_waited"] = queued
            return _emit(result, 0)
        if not queued:
            enqueue(locks_ledger, path, args.session, args.mode, at=args.at)
            queued = True
        if args.timeout is not None and (_time.monotonic() - started) >= args.timeout:
            dequeue(locks_ledger, path, args.session)
            return _emit({
                "error": "排队等待超时，已出队",
                "reason": "wait_timeout",
                "detail": {"path": path, "timeout": args.timeout},
            }, 1)
        _time.sleep(max(args.interval, 0.1))




SCOPE_SHARED_SURFACE = (
    "sih-engine/sih/event/trail",
    "sih-tools/scribe/reports",
    "sih-tools/identity/reports",
    "sih-tools/tally/reports",
    "sih-tools/meter/counts",
    "sih-tools/lease/ledger",
)  # 冻结登记：治理共享追加面白名单（架构常量非数值），交集预检豁免域


def _under_shared_surface(path):
    """路径是否落治理共享白名单（自身或祖先命中）。"""
    norm = path.rstrip("/")
    for shared in SCOPE_SHARED_SURFACE:
        s = shared.rstrip("/")
        if norm == s or norm.startswith(s + "/"):
            return True
    return False


def _precheck_gate(allow_paths, root, locks_ledger, at):
    """开工预检：返回 None 放行，或 (error, reason, detail) 拒开。

    只读探测：本批 allow 面逐路径与活跃会话独占持锁面（lock_state）交集
    判定，冲突谓词与锁库同义（同路径或包含）；治理共享白名单路径豁免，
    append 持位共存豁免。真锁仍由 agent 锁阶段取，本闸只做开工时感知。
    """
    from lease.lockdb import holds
    from lease.lockcore import normalize_path
    if not allow_paths:
        return None
    held = holds(locks_ledger)
    if not held:
        return None
    conflicts = []
    for mine in allow_paths:
        norm = normalize_path(mine, root)
        if _under_shared_surface(norm):
            continue
        for held_path, holders in held.items():
            for sid, mode in holders:
                if mode == "append":
                    continue
                hit = held_path == norm or _dir_ancestor(held_path, norm) or _dir_ancestor(norm, held_path)
                if hit:
                    conflicts.append({
                        "path": norm, "holder": sid, "held_path": held_path,
                        "hint": "wait-turn 排队或 takeover 接管或与持有批协调",
                    })
    if conflicts:
        return (
            "开工预检拦截：本批施工面与活跃会话独占持锁面存在交集",
            "open_precheck_conflict",
            {"conflicts": conflicts, "shared_surface_exempt": list(SCOPE_SHARED_SURFACE)},
        )
    return None


def _dir_ancestor(ancestor, descendant):
    a = ancestor.rstrip("/") + "/"
    d = descendant.rstrip("/") + "/"
    return d.startswith(a) and a != "/"





def _cmd_ledger_repair(args, root, session_ledger, locks_ledger):
    """台账补录：确定性导入恢复载荷。逐行校验、逐字节去重、repair 标记注入、原子追加。

    工程基线一：台账不由 LLM 手写，补录由本子命令承载；repair 标记显式即
    补录非伪装原笔，时间戳不可恢复者在标记中声明。幂等可重放。
    """
    from lease.lockcore import append_event, load_events
    src = Path(args.input)
    if not src.exists():
        return _emit({"error": "恢复载荷不存在", "detail": {"input": str(src)}}, 2)
    repaired, skipped_existing, rejected = [], [], []
    for lineno, raw in enumerate(src.read_text(encoding="utf-8").splitlines(), start=1):
        raw = raw.strip()
        if not raw:
            continue
        try:
            ev = json.loads(raw)
        except ValueError as exc:
            rejected.append({"lineno": lineno, "error": f"JSON 非法: {exc}"})
            continue
        if ev.get("event") not in ("issued", "revoked"):
            rejected.append({"lineno": lineno, "error": f"事件形态不在补录域: {ev.get('event')}"})
            continue
        existing = load_events(session_ledger, "sessions ledger")
        if any(_row_verbatim_match(e, ev) for e in existing):
            skipped_existing.append({"lineno": lineno, "session_id": ev.get("session_id", "")[:8]})
            continue
        line = dict(ev)
        source = getattr(args, "source", None) or "verbatim-from-vcs"
        line["repair"] = {
            "reason": args.reason,
            "repaired_at": args.at if args.at else datetime.now(timezone.utc).isoformat(timespec="seconds"),
            "source": source,
            "verbatim": source == "verbatim-from-vcs",
        }
        append_event(session_ledger, line)
        repaired.append({"lineno": lineno, "event": ev.get("event"),
                         "package": ev.get("package", ""), "session_id": ev.get("session_id", "")[:8]})
    result = {
        "repaired": len(repaired), "skipped_existing": len(skipped_existing),
        "rejected": rejected, "rows": repaired,
        "reason": args.reason,
        "source": getattr(args, "source", None) or "verbatim-from-vcs",
    }
    return _emit(result, 0 if not rejected else 1)


def _row_verbatim_match(existing, candidate):
    """逐字节同判：既有关键字段与候选原文关键字段全等即视为已在账。"""
    keys = ("event", "session_id", "package", "issued_at", "revoked_at")
    return all(existing.get(k) == candidate.get(k) for k in keys)


def main(argv=None):
    global _QUIET, _LAST_COMMAND
    parser = argparse.ArgumentParser(prog="lease")
    parser.add_argument("--quiet", action="store_true",
                        help="紧凑形即 stdout 只打一行摘要")
    sub = parser.add_subparsers(dest="command", required=True)

    open_cmd = sub.add_parser("open", help="立约开工：验意图起副本签发会话档")
    open_cmd.add_argument("--package", required=True, help="任务包 stem 或路径")
    open_cmd.add_argument("--identity", required=True, help="正身报告 json 文件")
    open_cmd.add_argument("--intent", required=True, help="意图记录 json，ask3 验收零发现方签发")
    open_cmd.add_argument("--repo", action="append", help="目标仓，可重复，缺省 sih-engine")
    open_cmd.add_argument("--allow", action="append", default=[], help="写入范围补充，可重复")
    open_cmd.add_argument("--claims", help="领取账本路径覆写，闸一联动警示读数位")
    open_cmd.add_argument("--at", help="ISO8601 时间戳覆写，复演用")
    open_cmd.add_argument("--root", help="工作区根覆写")
    open_cmd.add_argument("--ledger", help="会话台账路径覆写")
    open_cmd.add_argument("--locks", help="锁台账路径覆写（预检读数面，leaseopt-precheck-solo）")

    claim_cmd = sub.add_parser("claim", help="领取登记：同包未过期在领即拒，声明位非执法位")
    claim_cmd.add_argument("--package", required=True, help="任务包 stem 或路径")
    claim_cmd.add_argument("--ttl", required=True, type=int, help="领取时效分钟数，正整数")
    claim_cmd.add_argument("--claimant", help=f"领取人标识即会话预备号或委外自报标识，缺省 {CLAIMANT_DEFAULT}")
    claim_cmd.add_argument("--at", help="ISO8601 时间戳覆写，复演用")
    claim_cmd.add_argument("--root", help="工作区根覆写")
    claim_cmd.add_argument("--claims", help="领取账本路径覆写")

    unclaim_cmd = sub.add_parser("unclaim", help="领取释放：在领释放落 released 行，无在领即拦")
    unclaim_cmd.add_argument("--package", required=True, help="任务包 stem 或路径")
    unclaim_cmd.add_argument("--claimant", help="领取人标识，给参即验与在领相符")
    unclaim_cmd.add_argument("--at", help="ISO8601 时间戳覆写，复演用")
    unclaim_cmd.add_argument("--root", help="工作区根覆写")
    unclaim_cmd.add_argument("--claims", help="领取账本路径覆写")

    lock_cmd = sub.add_parser("lock", help="租内取锁即悲观链五验后落行")
    lock_cmd.add_argument("--path", required=True, help="工作区根相对路径")
    lock_cmd.add_argument("--identity", required=True, help="正身报告 json 文件")
    lock_cmd.add_argument("--session", help="会话号，缺省唯一在册")
    lock_cmd.add_argument("--mode", choices=["exclusive", "append"], default="exclusive",
                          help="锁型即独占或追加，缺省 exclusive 向后兼容")
    lock_cmd.add_argument("--at", help="ISO8601 时间戳覆写，复演用")
    lock_cmd.add_argument("--wait", action="store_true", help="撞锁入队受理返位次（leaseopt-lockqueue 排队调度）")
    lock_cmd.add_argument("--root", help="工作区根覆写")
    lock_cmd.add_argument("--locks", help="锁台账路径覆写")
    lock_cmd.add_argument("--ledger", help="会话台账路径覆写")

    unlock_cmd = sub.add_parser("unlock", help="放锁即同一五验加持锁验")
    unlock_cmd.add_argument("--path", required=True, help="工作区根相对路径")
    unlock_cmd.add_argument("--identity", required=True, help="正身报告 json 文件")
    unlock_cmd.add_argument("--session", help="会话号，缺省唯一在册")
    unlock_cmd.add_argument("--at", help="ISO8601 时间戳覆写，复演用")
    unlock_cmd.add_argument("--root", help="工作区根覆写")
    unlock_cmd.add_argument("--locks", help="锁台账路径覆写")
    unlock_cmd.add_argument("--ledger", help="会话台账路径覆写")

    status_cmd = sub.add_parser("status", help="查册即会话与在锁")
    status_cmd.add_argument("--package", help="按包名过滤会话")
    status_cmd.add_argument("--session", help="按会话号过滤锁")
    status_cmd.add_argument("--path", help="按路径过滤锁")
    status_cmd.add_argument("--history", action="store_true", help="会话全量事件")
    status_cmd.add_argument("--claims", action="store_true", help="附 claims 领取视图即在领与过期标注")
    status_cmd.add_argument("--at", help="ISO8601 时间戳覆写，claims 视图过期判定参照")
    status_cmd.add_argument("--root", help="工作区根覆写")
    status_cmd.add_argument("--ledger", help="会话台账路径覆写")
    status_cmd.add_argument("--locks", help="锁台账路径覆写")
    status_cmd.add_argument("--claims-ledger", help="领取账本路径覆写")

    check_cmd = sub.add_parser("check", help="乐观链验基线即上游洁净")
    check_cmd.add_argument("--path", required=True, help="工作区根相对路径")
    check_cmd.add_argument("--registry", help="边册路径，缺省引擎边册")
    check_cmd.add_argument("--doc-root", help="语料根，缺省引擎 doc")
    check_cmd.add_argument("--trail", action="append", help="书简 trail，可重复，缺省全链按日序")
    check_cmd.add_argument("--cascade-dir", help="级联工地目录，缺省同级 cascade")
    check_cmd.add_argument("--reports-root", help="报告根，缺省 sih-tools")
    check_cmd.add_argument("--root", help="工作区根覆写")
    check_cmd.add_argument("--locks", help="锁台账路径覆写")

    close_cmd = sub.add_parser("close", help="收约：锁清零、分支归并删支、拆本吊销")
    close_cmd.add_argument("--package", required=True, help="任务包 stem 或路径")
    close_cmd.add_argument("--force", action="store_true", help="脏工地强拆")
    close_cmd.add_argument("--reason", help="吊销事由")
    close_cmd.add_argument("--at", help="ISO8601 时间戳覆写，复演用")
    close_cmd.add_argument("--root", help="工作区根覆写")
    close_cmd.add_argument("--ledger", help="会话台账路径覆写")
    close_cmd.add_argument("--locks", help="锁台账路径覆写")

    commit_cmd = sub.add_parser("commit", help="提交即四验后机械 message 落笔，承 DEC-012 裁决三")
    commit_cmd.add_argument("--repo", required=True, help="目标仓")
    commit_cmd.add_argument("--session", help="会话号，缺省唯一在册")
    commit_cmd.add_argument("--stage", required=True, choices=["wip", "settle"], help="形态即 wip 或段结算")
    commit_cmd.add_argument("--subject", required=True, help="一段事述")
    commit_cmd.add_argument("--seq", type=int, help="段序，settle 必填")
    commit_cmd.add_argument("--cert", help="认证哈希，settle 必填即该段认证在链")
    commit_cmd.add_argument("--note", help="附注即偏离声明等")
    commit_cmd.add_argument("--trail", action="append", help="书简 trail，可重复，缺省全链按日序")
    commit_cmd.add_argument("--root", help="工作区根覆写")
    commit_cmd.add_argument("--ledger", help="会话台账路径覆写")

    reconcile_cmd = sub.add_parser("reconcile", help="对表即 git log 对台账对 trail 对 bypass 台账，绕行露形只报不拦")
    reconcile_cmd.add_argument("--repo", required=True, help="目标仓")
    reconcile_cmd.add_argument("--trail", action="append", help="书简 trail，可重复，缺省全链按日序")
    reconcile_cmd.add_argument("--base", help="起算 ref，缺省全史")
    reconcile_cmd.add_argument("--limit", type=int, default=0, help="尾部条数限，零即不限")
    reconcile_cmd.add_argument("--bypass-ledger", help="绕行台账路径覆写，缺省工具台账目录 bypass.ndjson")
    reconcile_cmd.add_argument("--root", help="工作区根覆写")
    reconcile_cmd.add_argument("--ledger", help="会话台账路径覆写")

    bypass_cmd = sub.add_parser("bypass", help="绕行留痕即 --no-verify 显式授权通道落 bypass 台账一笔")
    bypass_cmd.add_argument("--repo", required=True, help="被绕行提交所在仓")
    bypass_cmd.add_argument("--sha", required=True, help="被绕行提交 sha，全号或短号")
    bypass_cmd.add_argument("--reason", required=True, help="绕行事由")
    bypass_cmd.add_argument("--session", help="会话号")
    bypass_cmd.add_argument("--at", help="ISO8601 时间戳覆写，复演用")
    bypass_cmd.add_argument("--bypass-ledger", help="绕行台账路径覆写，缺省工具台账目录 bypass.ndjson")
    bypass_cmd.add_argument("--root", help="工作区根覆写")

    install_cmd = sub.add_parser("install-hooks", help="守卫安装即向仓写 core.hooksPath 指仓内 hooks 目录，缺省双仓")
    install_cmd.add_argument("--repo", action="append", help="目标仓，可重复，缺省 sih-tools 与 sih-engine")
    install_cmd.add_argument("--root", help="工作区根覆写")

    uninstall_cmd = sub.add_parser("uninstall-hooks", help="守卫拆卸即 unset core.hooksPath 可逆，缺省双仓")
    heartbeat_cmd = sub.add_parser("heartbeat", help="心跳：刷新锁库与检验文件活跃时间戳（leaseopt-lockdb-solo）")
    heartbeat_cmd.add_argument("--package", required=True)
    heartbeat_cmd.add_argument("--session", required=True)
    heartbeat_cmd.add_argument("--at", default=None)
    heartbeat_cmd.add_argument("--root")
    heartbeat_cmd.add_argument("--ledger")
    heartbeat_cmd.add_argument("--locks")
    takeover_cmd = sub.add_parser("takeover", help="接管：人节点显式清停滞检验文件与僵尸锁（leaseopt-lockqueue）")
    takeover_cmd.add_argument("--package", required=True)
    takeover_cmd.add_argument("--reason", required=True)
    takeover_cmd.add_argument("--at", default=None)
    takeover_cmd.add_argument("--root")
    takeover_cmd.add_argument("--ledger")
    takeover_cmd.add_argument("--locks")
    waitturn_cmd = sub.add_parser("wait-turn", help="排队等待：阻塞至轮到并取锁成功（leaseopt-lockqueue 排队阻塞形态）")
    waitturn_cmd.add_argument("--path", required=True)
    waitturn_cmd.add_argument("--identity", required=True)
    waitturn_cmd.add_argument("--session", required=True)
    waitturn_cmd.add_argument("--mode", default="exclusive")
    waitturn_cmd.add_argument("--timeout", type=float, default=None, help="等待上限秒，缺省无限")
    waitturn_cmd.add_argument("--interval", type=float, default=5.0, help="机械查询间隔秒（资源性参数）")
    waitturn_cmd.add_argument("--at", default=None)
    waitturn_cmd.add_argument("--root")
    waitturn_cmd.add_argument("--ledger")
    waitturn_cmd.add_argument("--locks")
    repair_cmd = sub.add_parser("ledger-repair", help="台账补录：确定性导入恢复载荷，repair 标记显式（leaseopt-ledger-repair）")
    repair_cmd.add_argument("--input", required=True, help="恢复载荷 ndjson（逐行 JSON 事件）")
    repair_cmd.add_argument("--reason", required=True, help="补录事由")
    repair_cmd.add_argument("--source", default="verbatim-from-vcs",
                            help="补录来源声明：逐字恢复缺省 verbatim-from-vcs；重建载荷显式声明来源，此时 verbatim 记 False")
    repair_cmd.add_argument("--at", default=None)
    repair_cmd.add_argument("--root")
    repair_cmd.add_argument("--ledger")
    uninstall_cmd.add_argument("--repo", action="append", help="目标仓，可重复，缺省 sih-tools 与 sih-engine")
    uninstall_cmd.add_argument("--root", help="工作区根覆写")

    args = parser.parse_args(argv)
    _QUIET = getattr(args, "quiet", False)
    _LAST_COMMAND = getattr(args, "command", "")
    root = resolve_root(getattr(args, "root", None))
    session_ledger = (
        Path(args.ledger)
        if getattr(args, "ledger", None)
        else tool_dir() / "ledger" / "sessions.ndjson"
    )
    locks_ledger = (
        Path(args.locks)
        if getattr(args, "locks", None)
        else tool_dir() / "ledger" / "locks.ndjson"
    )

    try:
        if args.command == "heartbeat":
            return _cmd_heartbeat(args, root, session_ledger, locks_ledger)
        if args.command == "takeover":
            return _cmd_takeover(args, root, locks_ledger)
        if args.command == "wait-turn":
            return _cmd_wait_turn(args, root, session_ledger, locks_ledger)
        if args.command == "ledger-repair":
            return _cmd_ledger_repair(args, root, session_ledger, locks_ledger)
        if args.command == "open":
            repos = args.repo if args.repo else [str(root / "sih-engine")]
            claims_ledger = (
                Path(args.claims) if getattr(args, "claims", None) else default_claims_ledger()
            )
            # openhyg-solo 闸序重排：前五位只读化（包校验、正身、意图、同包活跃在
            # open_preflight；预检在 _precheck_gate），检验钥匙闸降为第六位即首个
            # 副作用位——检验文件出生即全形，签发段失败自收桌，任何后续失败不再
            # 必然留残件（docmath-namefit 事故根因修复）。
            # leaseopt-precheck-solo 开工预检：allow 面对活跃会话独占持锁面
            # 交集只读探测，施工面真交集即拒开（共享白名单与 append 豁免）
            _locks_view = Path(args.locks) if getattr(args, "locks", None) else locks_ledger
            _ref_at = args.at if args.at is not None else datetime.now(timezone.utc).isoformat(timespec="seconds")
            _pre_flight = open_preflight(
                args.package,
                args.identity,
                repos,
                args.allow,
                _ref_at,
                root,
                session_ledger,
                intent_path=args.intent,
                claims_ledger=claims_ledger,
            )
            _pre = _precheck_gate(args.allow or [], root, _locks_view, args.at)
            if _pre is not None:
                return _emit({"error": _pre[0], "reason": _pre[1], "detail": _pre[2]}, 1)
            _key = _check_key_gate(
                args.package, root, _ref_at,
                identity_core=_pre_flight["identity"]["core_hash"], locks_ledger=_locks_view,
            )
            if _key is not None:
                return _emit({"error": _key[0], "reason": _key[1], "detail": _key[2]}, 1)
            try:
                line = open_execute(_pre_flight)
            except (WorktreeError, StateError) as exc:
                _note = _self_clean_check_file(root, args.package, _ref_at)
                if _note is not None:
                    exc.args = (f"{exc.args[0] if exc.args else exc}; 自收桌未竟: {_note}",)
                raise
            _stamp_check_file(root, args.package, line.get("session_id"))
            return _emit(line, 0)
        if args.command == "claim":
            stem, _ = resolve_package(args.package, root)
            result = claim_register(
                stem, args.claimant or CLAIMANT_DEFAULT, args.ttl, args.at,
                Path(args.claims) if args.claims else default_claims_ledger(),
            )
            return _emit(result, 0)
        if args.command == "unclaim":
            stem, _ = resolve_package(args.package, root)
            result = claim_release(
                stem, args.claimant, args.at,
                Path(args.claims) if args.claims else default_claims_ledger(),
            )
            return _emit(result, 0)
        if args.command == "lock":
            result = lock_acquire(
                args.path,
                args.identity,
                args.session,
                root,
                locks_ledger,
                session_ledger,
                at=args.at,
                mode=args.mode,
                wait=getattr(args, "wait", False),
            )
            if result.get("queued"):
                return _emit(result, 0)
            return _emit(result, 0)
        if args.command == "unlock":
            result = lock_release(
                args.path,
                args.identity,
                args.session,
                root,
                locks_ledger,
                session_ledger,
                at=args.at,
            )
            return _emit(result, 0)
        if args.command == "status":
            package_stem = None
            if args.package:
                package_stem, _ = resolve_package(args.package, root, must_exist=False)
            session_view = status(
                session_ledger, package_stem=package_stem, history=args.history
            )
            lock_view = lock_status(locks_ledger, path=args.path, session_id=args.session, root=root)
            report = {
                "header": {
                    "tool": {"name": "lease", "version": __version__},
                    "sessions_ledger": str(session_ledger),
                    "locks_ledger": str(locks_ledger),
                },
                "locks": lock_view,
                "sessions": session_view,
                "summary": {
                    "active_sessions": session_view["summary"]["active"],
                    "held_locks": lock_view["summary"]["held"],
                },
            }
            if getattr(args, "claims", False):
                claims_ledger_path = (
                    Path(args.claims_ledger)
                    if getattr(args, "claims_ledger", None)
                    else default_claims_ledger()
                )
                report["claims"] = claims_view(
                    claims_ledger_path,
                    at=getattr(args, "at", None),
                    package_stem=package_stem,
                )
            return _emit(report, 0)
        if args.command == "check":
            registry = Path(args.registry) if args.registry else root / "sih-engine" / "doc" / "CASCADE.json"
            doc_root = Path(args.doc_root) if args.doc_root else root / "sih-engine" / "doc"
            trails = args.trail if args.trail else _default_trails(root)
            cascade_dir = (
                Path(args.cascade_dir)
                if args.cascade_dir
                else Path(__file__).resolve().parents[3] / "cascade"
            )
            reports_root = Path(args.reports_root) if args.reports_root else root / "sih-tools"
            report = check_baseline(
                args.path, registry, doc_root, trails, cascade_dir, reports_root, locks_ledger
            )
            if report["verdict"] == "blocked":
                return _emit(
                    {"dirty": report["dirty"], "error": "dirty_upstream", "verdict": report["verdict"]},
                    1,
                )
            return _emit(report, 0)
        if args.command == "commit":
            if args.stage == "settle" and args.seq is None:
                return _emit_error("settle requires --seq", code=2)
            trails = args.trail if args.trail else _default_trails(root)
            report = commit_staged(
                args.repo,
                args.session,
                args.stage,
                args.subject,
                args.seq,
                args.cert,
                args.note,
                root,
                session_ledger,
                [Path(t) for t in trails],
            )
            if args.stage == "settle":
                from lease.core import gauge_summary

                report["gauge"] = gauge_summary(root)
            return _emit(report, 0)
        if args.command == "reconcile":
            trails = args.trail if args.trail else _default_trails(root)
            bypass_ledger = (
                Path(args.bypass_ledger) if getattr(args, "bypass_ledger", None) else default_bypass_ledger()
            )
            base, seal = resolve_default_base(args.repo, args.base)
            report = reconcile(
                args.repo,
                session_ledger,
                [Path(t) for t in trails],
                base=base,
                limit=args.limit,
                bypass_ledger=bypass_ledger,
            )
            if seal:
                report["base_source"] = {"kind": "seal_line", "boundary": seal, "range_base": base}
            code = 0 if report["summary"]["unrouted"] == 0 and report["summary"]["session_orphan"] == 0 and report["summary"]["cert_missing"] == 0 and report["summary"]["unbypassed"] == 0 else 1
            return _emit(report, code)
        if args.command == "bypass":
            ledger_path = (
                Path(args.bypass_ledger)
                if getattr(args, "bypass_ledger", None)
                else default_bypass_ledger()
            )
            line = record_bypass(
                args.repo, args.sha, args.reason, args.session, args.at, ledger_path
            )
            return _emit({"bypassed": line, "ledger": str(ledger_path)}, 0)
        if args.command in ("install-hooks", "uninstall-hooks"):
            repos = args.repo if args.repo else [str(root / "sih-tools"), str(root / "sih-engine")]
            if args.command == "install-hooks":
                results = install_hooks(repos)
                code = 0 if all(item["installed"] for item in results) else 1
            else:
                results = uninstall_hooks(repos)
                code = 0 if all(item["uninstalled"] for item in results) else 1
            return _emit({"hooks_dir": str(HOOKS_DIR), "repos": results}, code)
        report = close_session(
            args.package,
            args.force,
            args.reason,
            args.at,
            root,
            session_ledger,
            locks_ledger=locks_ledger,
        )
        if not report["revoked"]:
            return _emit_error(
                "worktree remove failed, session stays active",
                code=1,
                extra={"failed": report["failed"], "removed": report["removed"]},
            )
        # leaseopt-lockdb-solo 收约凭据：检验文件转写任务包同级为跑完证据
        _receipt = _write_close_receipt(args.package, root, report, args.at)
        if _receipt is not None:
            report["close_receipt"] = _receipt
        return _emit(report, 0)
    except LockBlocked as exc:
        return _emit({"detail": exc.detail, "error": exc.reason}, 1)
    except PackageSessionActive as exc:
        return _emit({
            "error": str(exc),
            "reason": "PackageSessionActive",
            "detail": {"package": exc.package, "session_id": exc.session_id},
        }, 1)
    except PackageAlreadyClaimed as exc:
        return _emit({
            "error": str(exc),
            "reason": "PackageAlreadyClaimed",
            "detail": {
                "claimant": exc.claimant,
                "expires_at": exc.expires_at,
                "package": exc.package,
            },
        }, 1)
    except ClaimBlocked as exc:
        return _emit({"detail": exc.detail, "error": exc.reason, "reason": exc.reason}, 1)
    except ClaimsError as exc:
        return _emit_error(str(exc), code=2)
    except CommitBlocked as exc:
        return _emit({"detail": exc.detail, "error": exc.reason}, 1)
    except LocksError as exc:
        return _emit({"error": str(exc)}, 2)
    except StateError as exc:
        return _emit_error(str(exc), code=1)
    except WorktreeError as exc:
        return _emit_error(str(exc), code=2)
