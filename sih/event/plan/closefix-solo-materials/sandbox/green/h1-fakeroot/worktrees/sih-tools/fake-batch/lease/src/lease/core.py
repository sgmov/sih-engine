"""租约核心：任务包立约开工、副本生命周期与会话档台账。

会话档即租约凭证，立约签发收约吊销。close 三检即租内锁清零、
分支归并删支、拆本吊销。承 GOV-003 第 1 段与 DEC-011。
"""

import hashlib
import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

from lease.claimcore import claims_warning, default_claims_ledger

# 冻结架构常量：任务包合法落位登记表（openhyg-solo 批）。起步三目录宁窄勿宽，
# 扩面走 CONTRACT 修订不裸奔；裸 stem 按表逐目录搜，解析语义见 resolve_package。
TASK_PACKAGE_DIRS = (
    "sih-engine/sih/state/plan",
    "sih-engine/task-packages",
    "sih-math/sih/event/plan",
)
# 冻结架构常量：台账追加面登记表（closefix-solo 病一，用户已裁方向）。收约归并
# 对这些面永不整文件 checkout 盖版，只走并集追加通道（_union_append_face）。
# 起步四面即会话册加锁册加绕行册加检验目录，扩面走 CONTRACT 修订不裸奔。
LEASE_LEDGER_APPEND_FACES = (
    "lease/ledger/sessions.ndjson",
    "lease/ledger/locks.ndjson",
    "lease/ledger/bypass.ndjson",
    "lease/ledger/checks",
)
TOOL_NAME = "lease"
BRANCH_PREFIX = "msh"


def _is_ledger_append_face(path):
    """台账追加面冻结判定（closefix-solo 病一）：路径命中冻结常量（自身或子路径）。

    命中即收约归并机械对这件永不整文件 checkout 盖版——台账活行只可并集
    追加保全，分支版仅作并集输入不作替换源。
    """
    norm = str(path).rstrip("/")
    for face in LEASE_LEDGER_APPEND_FACES:
        s = face.rstrip("/")
        if norm == s or norm.startswith(s + "/"):
            return True
    return False


class WorktreeError(ValueError):
    """工具异常即输入非法或 git 不在或台账不可写，退出码二。"""


class StateError(ValueError):
    """状态违例即重复开工或收工无在册或工地脏或 git 拒绝，退出码一。"""


class PackageSessionActive(StateError):
    """闸一同包唯一：同包已有活跃会话即拒，载会话号与包名，详情非空。

    承 guardrail2-solo 闸一：lease open 不查同包活跃会话即同包双跑畅行，
    本例外把同包查位从裸 StateError 强化为独立异常载包名加会话号。
    """

    def __init__(self, package, session_id):
        self.package = package
        self.session_id = session_id
        super().__init__(f"active session exists for package {package}: {session_id}")


def default_root():
    """工作区根即双仓祖先搜上，围堰副本内亦命中；回落旧式上溯四级承 pk-031。"""
    return resolve_root(None)


def resolve_root(explicit):
    """显式给参优先；缺省双仓祖先搜上即围堰相对根不再主线绿围堰红承 pk-031。"""
    if explicit:
        return Path(explicit)
    here = Path(__file__).resolve()
    for candidate in (here, *here.parents):
        if (candidate / "sih-engine" / "Cargo.toml").is_file() and (
            candidate / "sih-tools" / "pyproject.toml"
        ).is_file():
            return candidate
    return here.parents[4]


def _gate_root():
    """意图闸门根即含 sih-engine 与 sih-tools 的祖先，副本内上溯到真根。

    找不到双仓祖先即回落旧式 parents[3].parent，保持显式环境下的可预测失败。
    """
    here = Path(__file__).resolve().parents[3]
    for candidate in (here, *here.parents):
        if (candidate / "sih-engine" / "Cargo.toml").is_file() and (candidate / "sih-tools" / "pyproject.toml").is_file():
            return candidate
    return here.parent


def tool_dir():
    """工具目录即台账缺省父位。"""
    return Path(__file__).resolve().parents[2]


def resolve_package(value, root, must_exist=True):
    """包值解析（openhyg-solo 按表解析）：返回 stem 加路径。

    裸 stem 按登记表 TASK_PACKAGE_DIRS 逐目录搜——唯一命中即用；歧义列全部
    命中径拒开不猜；零命中报文列全搜索面。带分隔符路径（含相对与绝对）一律
    按 root 拼接后校验，不按调用 cwd（constmodel 首跑缺陷收编）。must_exist=False
    零命中回落登记表首目录确定性占位，供 stem 键位操作与收约凭据定位。
    """
    candidate = Path(value)
    if candidate.is_absolute() or "/" in value or str(candidate.parent) != ".":
        if not candidate.is_absolute():
            candidate = Path(root) / candidate
        if must_exist and not candidate.is_file():
            raise WorktreeError(f"package file not found: {candidate}")
        return candidate.stem, candidate
    hits = [
        Path(root) / d / f"{value}.md"
        for d in TASK_PACKAGE_DIRS
        if (Path(root) / d / f"{value}.md").is_file()
    ]
    if len(hits) == 1:
        return hits[0].stem, hits[0]
    if len(hits) > 1:
        listed = "\n  ".join(str(h) for h in hits)
        raise WorktreeError(
            f"ambiguous package stem {value!r}, refuse to guess, hits:\n  {listed}"
        )
    faces = "\n  ".join(str(Path(root) / d / f"{value}.md") for d in TASK_PACKAGE_DIRS)
    if must_exist:
        raise WorktreeError(
            f"package file not found for stem {value!r}, searched faces:\n  {faces}"
        )
    return value, Path(root) / TASK_PACKAGE_DIRS[0] / f"{value}.md"


def now_iso():
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def sha256_file(path):
    """身份件字节哈希，恒入账。"""
    try:
        data = Path(path).read_bytes()
    except OSError as exc:
        raise WorktreeError(f"identity file unreadable: {path}") from exc
    return hashlib.sha256(data).hexdigest()


def load_identity(path):
    """身份件入账形态即双哈希，绑定子集另行抽取，非法 json 即异常。

    core_hash 增读承 idcore-solo：报告 identity 块带核哈希即入台账，
    缺席即空缺不判败；identity_hash 与 file_sha256 与会话号派生全不动。
    """
    file_sha = sha256_file(path)
    try:
        payload = json.loads(Path(path).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise WorktreeError(f"identity file invalid: {path}") from exc
    identity_hash = None
    core_hash = None
    if isinstance(payload, dict) and isinstance(payload.get("identity"), dict):
        value = payload["identity"].get("hash")
        if isinstance(value, str) and value:
            identity_hash = value
        core_value = payload["identity"].get("core_hash")
        if isinstance(core_value, str) and core_value:
            core_hash = core_value
    observed = payload.get("observed") if isinstance(payload, dict) else None
    observed = observed if isinstance(observed, dict) else {}
    binding = {key: str(observed.get(key, "")) for key in ("hostname", "user", "boottime")}
    return {"file_sha256": file_sha, "identity_hash": identity_hash, "core_hash": core_hash}, binding


def validate_intent(intent_path, root):
    """意图件验收即 ask3 子进程，退出码零零发现方过，无意图无会话。"""
    scrutinator_dir = Path(__file__).resolve().parents[3] / "scrutinator"
    record_sha = sha256_file(intent_path)
    try:
        payload = json.loads(Path(intent_path).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise WorktreeError(f"intent record invalid: {intent_path}") from exc
    try:
        proc = subprocess.run(
            [
                "uv", "run", "--project", str(scrutinator_dir),
                "python", "-m", "scrutinator", "--pack", "packs/ask3", str(Path(intent_path).resolve()),
            ],
            capture_output=True,
            text=True,
            timeout=120,
            cwd=str(scrutinator_dir),
        )
    except (OSError, subprocess.SubprocessError) as exc:
        raise WorktreeError(f"ask3 invocation failed: {exc}") from exc
    if proc.returncode != 0:
        detail = proc.stdout.strip()[:200] or proc.stderr.strip()[:200]
        raise WorktreeError(f"intent validation failed: {detail}")
    try:
        report = json.loads(proc.stdout)
        if report.get("findings"):
            raise WorktreeError("intent validation has findings")
    except json.JSONDecodeError as exc:
        raise WorktreeError(f"ask3 report unparsable: {exc}") from exc
    workspace_root = _gate_root()
    engine_dir = workspace_root / "sih-engine"
    try:
        gate = subprocess.run(
            [
                "cargo", "run", "--quiet", "--manifest-path", str(engine_dir / "Cargo.toml"),
                "--bin", "ask3repeater", str(Path(intent_path).resolve()),
                "--root", str(workspace_root),
            ],
            capture_output=True,
            text=True,
            timeout=180,
        )
    except (OSError, subprocess.SubprocessError) as exc:
        raise WorktreeError(f"ask3gate invocation failed: {exc}") from exc
    if gate.returncode != 0:
        detail = gate.stdout.strip()[:200] or gate.stderr.strip()[:200]
        raise WorktreeError(f"intent lineage validation failed: {detail}")
    anchors = payload.get("anchors") if isinstance(payload, dict) else None
    return {
        "anchor_count": len(anchors) if isinstance(anchors, list) else 0,
        "record_sha256": record_sha,
        "validated_by": "scrutinator-ask3",
    }


def parse_requested_writes(package_path):
    """任务包请求写入段即机器读取位，列表项即工作区根相对路径。"""
    try:
        text = Path(package_path).read_text(encoding="utf-8")
    except OSError as exc:
        raise WorktreeError(f"package unreadable: {package_path}") from exc
    paths = []
    in_section = False
    for line in text.splitlines():
        if line.startswith("## "):
            in_section = "请求写入" in line.split("{", 1)[0]
            continue
        if not in_section:
            continue
        stripped = line.strip()
        if stripped.startswith("- ") or stripped.startswith("* "):
            item = stripped[2:].strip().strip("`")
            if item:
                paths.append(item)
    return paths


def write_binding(worktrees_root, session_id, binding):
    """绑定侧档即工地根下 .bindings，载稳定子集三件，不入版控不进台账。"""
    path = Path(worktrees_root) / ".bindings" / f"{session_id}.json"
    try:
        path.parent.mkdir(parents=True, exist_ok=True)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(json.dumps(
                {"boottime": binding["boottime"], "hostname": binding["hostname"],
                 "session_id": session_id, "user": binding["user"]},
                ensure_ascii=False, sort_keys=True,
            ))
    except OSError as exc:
        raise WorktreeError(f"binding sidecar unwritable: {path}") from exc


def _git(repo, *args):
    """git 单点出口，git 不在即工具异常，返回码加双流。"""
    try:
        proc = subprocess.run(
            ["git", "-C", str(repo), *args],
            capture_output=True,
            text=True,
            timeout=30,
        )
    except FileNotFoundError as exc:
        raise WorktreeError("git not available") from exc
    except subprocess.SubprocessError as exc:
        raise WorktreeError(f"git invocation failed: {exc}") from exc
    return proc.returncode, proc.stdout, proc.stderr


def load_ledger(ledger):
    """台账逐行读，缺席即空册，坏行即工具异常。"""
    path = Path(ledger)
    if not path.exists():
        return []
    events = []
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise WorktreeError(f"ledger unreadable: {ledger}") from exc
    for number, line in enumerate(text.splitlines(), start=1):
        if not line.strip():
            continue
        try:
            events.append(json.loads(line))
        except json.JSONDecodeError as exc:
            raise WorktreeError(f"ledger line {number} invalid: {exc}") from exc
    return events


def append_event(ledger, event):
    """台账追加即唯一写点，单行紧凑形。（pk057fix: 委托 ledgerwrite flock 串行）"""
    from lease.ledgerwrite import append_row

    try:
        append_row(Path(ledger), event)
    except OSError as exc:
        raise WorktreeError(f"ledger unwritable: {ledger}") from exc


def active_sessions(events):
    """在册即按事件序配对：issued 置位、revoked 仅弹在册同号。

    同号重开不被旧 revoked 误抹承 pk-031 证据二，单遍事件序即时序语义。
    """
    sessions = {}
    for event in events:
        sid = event.get("session_id")
        if event.get("event") == "issued":
            sessions[sid] = event
        elif event.get("event") == "revoked" and sid in sessions:
            sessions.pop(sid)
    return sessions


def make_session_id(package, issued_at, file_sha, repos):
    """会话号即 SHA-256 前十六位，输入即包名加签发时刻加身份件哈希加仓序列。"""
    payload = f"{package}|{issued_at}|{file_sha}|{'|'.join(repos)}"
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()[:16]


def _current_branch(repo_path):
    """开工时记录基线分支即归并目标位。"""
    rc, out, _ = _git(repo_path, "rev-parse", "--abbrev-ref", "HEAD")
    return out.strip() if rc == 0 else ""



def gauge_summary(root, at=None, window_days=30):
    """三维读数摘要：调秤星 read 三维各一出值，只报不拦。

    承 SPEC-011 消费侧声明：租约入口与段结算附读数。秤星缺席、证据件缺席或
    读数败折成 skipped 注记不阻断租约主功能，零拦截承 SPEC-011 F-6。
    """
    at = (at or now_iso())[:10]
    root = Path(root)
    try:
        cli = root / "sih-tools" / "gauge" / "src" / "gauge" / "cli.py"
        trail_dir = root / "sih-engine" / "sih" / "event" / "trail"
        ledger = root / "sih-tools" / "lease" / "ledger" / "sessions.ndjson"
        trails = sorted(trail_dir.glob("*.ndjson")) if trail_dir.is_dir() else []
        missing = [
            str(p)
            for p in (cli, ledger, root / "sih-engine", root / "sih-tools")
            if not p.exists()
        ]
        if not trails:
            missing.append(str(trail_dir))
        if missing:
            return {"skipped": True, "reason": "missing", "missing": missing}
        values = {}
        formula = None
        for dim in ("convergence", "adoption", "mergeback"):
            cmd = [
                sys.executable, str(cli), "read", "--dimension", dim,
                "--at", at, "--window-days", str(window_days),
                *[_pair for t in trails for _pair in ("--trail", str(t))],
                "--sessions-ledger", str(ledger),
                "--src-root", str(root / "sih-engine"),
                "--tools-root", str(root / "sih-tools"),
            ]
            proc = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
            if proc.returncode != 0:
                return {
                    "skipped": True,
                    "reason": "gauge_error",
                    "dimension": dim,
                    "detail": (proc.stderr or proc.stdout).strip()[:200],
                }
            out = json.loads(proc.stdout)
            formula = out["reading"]["formula_version"]
            values[dim] = {
                "value": out["reading"]["value"],
                "sequence": out["sequence"],
            }
        return {"at": at, "formula_version": formula, "values": values, "window_days": window_days}
    except (OSError, subprocess.SubprocessError, json.JSONDecodeError, KeyError) as exc:
        return {"skipped": True, "reason": f"unavailable: {exc.__class__.__name__}"}


def open_preflight(package_value, identity_path, repos, allow, at, root, ledger, worktrees_root=None, intent_path=None, claims_ledger=None):
    """开工前四位只读化（openhyg-solo 闸序重排）：本函数零副作用，失败即拒不落盘面。

    闸序：一包校验（按表解析加 must_exist 加请求写入段）、二正身读取、三意图校验、
    四同包活跃会话查，另附只读的副本路径与分支探测。检验钥匙闸（首个副作用位）
    与签发段归调用方编排：五预检、六钥匙闸、七 open_execute。
    """
    if claims_ledger is None:
        claims_ledger = default_claims_ledger()
    stem, package_path = resolve_package(package_value, root)
    requested = parse_requested_writes(package_path)
    allow = requested + [item for item in allow if item not in requested]
    identity, binding = load_identity(identity_path)
    if intent_path is None:
        raise WorktreeError("intent record required, no intent no session")
    intent = validate_intent(intent_path, root)
    events = load_ledger(ledger)
    for active in active_sessions(events).values():
        if active.get("package") == stem:
            raise PackageSessionActive(stem, active["session_id"])
    issued_at = at if at is not None else now_iso()
    if worktrees_root is None:
        worktrees_root = Path(root) / "worktrees"
    entries = []
    branch_exists = []
    for repo in repos:
        repo_p = Path(repo)
        if repo_p.is_absolute():
            repo_path = repo_p.resolve()
        else:
            # 根相对解析：相对路径先试工作区根下，存在则用之
            candidate = (Path(root) / repo_p).resolve()
            if candidate.exists():
                repo_path = candidate
            else:
                repo_path = repo_p.resolve()
        worktree_path = Path(worktrees_root) / repo_path.name / stem
        if worktree_path.exists():
            raise StateError(f"worktree path exists: {worktree_path}")
        base_branch = _current_branch(repo_path)
        branch = f"{BRANCH_PREFIX}/{stem}"
        rc, _, _ = _git(repo_path, "rev-parse", "--verify", "--quiet", f"refs/heads/{branch}")
        branch_exists.append(rc == 0)
        entries.append(
            {
                "base_branch": base_branch,
                "branch": branch,
                "repo": str(repo_path),
                "worktree": str(worktree_path),
            }
        )
    return {
        "allow": allow,
        "binding": binding,
        "branch_exists": branch_exists,
        "claims_ledger": str(claims_ledger),
        "entries": entries,
        "identity": identity,
        "intent": intent,
        "issued_at": issued_at,
        "ledger": str(ledger),
        "package": stem,
        "package_path": package_path,
        "root": str(root),
        "scope_source": "package" if requested else "explicit",
        "worktrees_root": str(worktrees_root),
    }


def open_execute(preflight):
    """开工第七位（openhyg-solo）：检验文件已由钥匙闸出生即全落盘，本段签发入台账。

    起工地、生成会话号、台账行追加、绑定侧档。本段任何失败由调用方自收桌
    （检验文件所有权核验后删除，两错并报）；台账行字段与本函数外形态零变更。
    """
    created = []
    for entry, exists in zip(preflight["entries"], preflight["branch_exists"]):
        if exists:
            rc, _, stderr = _git(
                entry["repo"], "worktree", "add", entry["worktree"], entry["branch"]
            )
        else:
            rc, _, stderr = _git(
                entry["repo"], "worktree", "add", "-b", entry["branch"], entry["worktree"]
            )
        if rc != 0:
            for done in created:
                _git(done["repo"], "worktree", "remove", "--force", done["worktree"])
            raise StateError(f"git worktree add failed: {stderr.strip()}")
        created.append(entry)
    session_id = make_session_id(
        preflight["package"], preflight["issued_at"], preflight["identity"]["file_sha256"],
        [entry["repo"] for entry in preflight["entries"]],
    )
    line = {
        "allow": list(preflight["allow"]),
        "event": "issued",
        "identity": preflight["identity"],
        "intent": preflight["intent"],
        "issued_at": preflight["issued_at"],
        "package": preflight["package"],
        "package_path": str(preflight["package_path"].resolve()),
        "repos": preflight["entries"],
        "session_id": session_id,
        "scope_source": preflight["scope_source"],
        "tool": {"name": TOOL_NAME, "version": _version()},
        "gauge": gauge_summary(preflight["root"], preflight["issued_at"]),
    }
    open_claims_warning = claims_warning(
        Path(preflight["claims_ledger"]), preflight["package"], session_id, preflight["issued_at"]
    )
    if open_claims_warning:
        line["claims_warning"] = open_claims_warning
    append_event(preflight["ledger"], line)
    write_binding(preflight["worktrees_root"], session_id, preflight["binding"])
    return line


def _merge_file_diverged(repo, base, branch, f):
    """归并件落盘是否真分叉：既不等 branch 固件也不等 base 原样即拦。

    merge 将把这件写成 branch 内容。落盘已是 branch 内容即清净，或仍是
    base 原样即由 merge 清净更新；两向皆非即真实并发分叉（ordwire 共享
    追加件事故样本）需人工并入即整批拒。纯追加形 ndjson 共享活面除外，
    由 close 并集复查闸自备份让位承载（basefix-solo 批 F-3）。
    """
    if f.endswith(".ndjson") and _chain_append_grown(repo, base, branch, f) is not None:
        return False
    path = Path(repo) / f
    if not path.is_file():
        return False
    try:
        ondisk = path.read_text(encoding="utf-8")
    except OSError:
        return True
    rc_b, blob_b, _ = _git(repo, "show", f"{branch}:{f}")
    if rc_b == 0 and ondisk == blob_b:
        return False
    if base:
        rc_a, blob_a, _ = _git(repo, "show", f"{base}:{f}")
        if rc_a == 0 and ondisk == blob_a:
            return False
    return True


def _merge_files(repo, base, branch):
    """归并面即三点式差集件清单（closefix-solo 病一）。

    以 merge-base 为基准取 branch 侧改动（diff merge-base..branch）——
    两端点差集会把主干侧前进误入归并面（分支台账与 merge-base 逐字相同
    也中弹，ledgerloss5 第五笔根因第一环），三点式使主干侧前进零入面，
    「分支无新内容可归并」的文件零触碰。merge-base 解算失败回落 base
    两点形保守。
    """
    mb = base
    rc_mb, out_mb, _ = _git(repo, "merge-base", base, branch)
    if rc_mb == 0 and out_mb.strip():
        mb = out_mb.strip()
    rc, out, _ = _git(repo, "diff", "--name-only", mb, branch)
    if rc != 0:
        return []
    return [f for f in out.splitlines() if f.strip()]


def _union_append_face(repo_path, branch, f):
    """并集追加通道（closefix-solo 病一，台账面冻结豁免）：永不整文件盖版。

    单锁窗内：重读现行活面 → 取分支版行集 → 活面保持原序原行不动，分支
    独有行按序经 append_row 补尾（锁由本窗持有且可重入）——落盘结果为
    活面∪分支的超集，活写行零丢失。分支版仅作并集输入不作替换源。
    返回 {"live_rows", "branch_only_appended"}。
    """
    from lease.ledgerwrite import ledger_lock

    src = Path(repo_path) / f
    with ledger_lock(src):
        rc_b, blob_b, err_b = _git(repo_path, "show", f"{branch}:{f}")
        if rc_b != 0:
            raise StateError(
                f"union face branch content unreadable: {f}: {err_b.strip()[:120]}"
            )
        live_lines = [
            ln for ln in (src.read_text(encoding="utf-8").splitlines() if src.exists() else [])
            if ln.strip()
        ]
        live_set = set(live_lines)
        branch_only = [ln for ln in blob_b.splitlines() if ln.strip() and ln not in live_set]
        from lease.ledgerwrite import append_row

        for ln in branch_only:
            append_row(src, json.loads(ln))
        return {"live_rows": len(live_lines), "branch_only_appended": len(branch_only)}


def _chain_append_grown(repo, base, branch, f):
    """并集复查分类（basefix-solo 批 F-3）：ndjson 归并件落盘是否纯追加增长。

    落盘行序以 base 全行集为严格前缀且多出尾行即纯追加增长，返回
    {"live_only": 落盘独有行清单}；否则 None 交回真分叉判位。leasewire
    事故根因即 close 复跑用旧快照 clobber 并行批追加一笔，本判位使
    close 在归并前重读现行链重算并集超集，不信任早前快照。
    """
    path = Path(repo) / f
    if not f.endswith(".ndjson") or not path.is_file():
        return None
    try:
        ondisk = path.read_text(encoding="utf-8")
    except OSError:
        return None
    if not base:
        return None
    rc_a, blob_a, _ = _git(repo, "show", f"{base}:{f}")
    if rc_a != 0:
        return None
    ondisk_lines = ondisk.splitlines()
    base_lines = blob_a.splitlines()
    if len(ondisk_lines) <= len(base_lines) or ondisk_lines[: len(base_lines)] != base_lines:
        return None
    rc_b, blob_b, _ = _git(repo, "show", f"{branch}:{f}")
    branch_set = set(blob_b.splitlines()) if rc_b == 0 else set()
    base_set = set(base_lines)
    live_only = [line for line in ondisk_lines[len(base_lines):] if line not in branch_set and line not in base_set]
    return {"live_only": live_only}


def _chain_union_yield(repo, base, branch, files, backup_dir):
    """并集复查让位（basefix-solo 批 F-3）：纯追加 ndjson 备份让位归并。

    落盘内容先备份至 deterministic 路径再让位，归并后重读合并件算补笔清单
    即备份独有行哈希，账目随 removed 记录入台账事件可对账，链语义修复归
    书简补笔通道不归 close 直写。closefix-solo 病一：台账追加面（冻结常量
    命中）永不 checkout 让位，走并集追加通道即盖版不可达、活行零丢失；
    其余 ndjson 仍 checkout 让位加回补网（锁可重入后窗内回补零死锁）。
    """
    report = {}
    backup_dir.mkdir(parents=True, exist_ok=True)
    for f in files:
        info = _chain_append_grown(repo, base, branch, f)
        if info is None:
            continue
        src = Path(repo) / f
        dest = backup_dir / f
        dest.parent.mkdir(parents=True, exist_ok=True)
        from lease.ledgerwrite import ledger_lock, restore_missing

        # pk057fix：合并窗口持台账互斥锁，与并发追加零交错；
        # closefix-solo 病三：锁可重入，窗内回补经 append_row 不再自死锁。
        with ledger_lock(src):
            dest.write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
            if _is_ledger_append_face(f):
                merged = _union_append_face(repo, branch, f)
                restored = merged["branch_only_appended"]
            else:
                # 从 branch 复制到 working tree（不是从 HEAD 还原）
                rc_c, _, err_c = _git(repo, "checkout", branch, "--", f)
                if rc_c != 0:
                    raise StateError(f"chain union checkout failed: {f}: {err_c.strip()[:120]}")
                restored = restore_missing(src, dest)
        report[f] = {"backup_path": str(dest), "live_only": info["live_only"], "restored": restored}
    return report


def _chain_union_recertify(repo, f, union_info):
    """归并后重读合并件算补笔清单：备份独有行不在合并件即出哈希工单。"""
    merged = (Path(repo) / f).read_text(encoding="utf-8").splitlines()
    merged_set = set(merged)
    hashes = [
        hashlib.sha256(line.encode("utf-8")).hexdigest()[:16]
        for line in union_info["live_only"]
        if line not in merged_set
    ]
    return hashes


def _close_precondition(session):
    """收约前置态整批探针：逐仓报合并态标记与归并面真分叉件。

    承 guardrail-solo 护栏二：目标仓停置合并态或归并面脏即整批拒绝零部分
    动作，错误详情全量透出禁空串，半程不可达即要么全归并要么零动作。归并
    面脏是 base..branch 差集件落盘内容既不等 branch 固件也不等 base 原样；
    纯运行账本（锁册会话册）或他批在线文件扩充的全部跟踪脏态不是 merge 带
    入面，构成不了半程合并风险，不入前置态即不催收合封死正常收约。
    """
    issues = []
    for entry in session["repos"]:
        repo = entry["repo"]
        base = entry.get("base_branch") or ""
        branch = entry.get("branch") or ""
        rc0, out0, _ = _git(repo, "rev-parse", "--git-dir")
        git_dir = Path(out0.strip()) if rc0 == 0 and out0.strip() else Path(repo) / ".git"
        if not git_dir.is_absolute():
            git_dir = Path(repo) / git_dir
        markers = [
            name
            for name in (
                "MERGE_HEAD",
                "rebase-merge",
                "rebase-apply",
                "CHERRY_PICK_HEAD",
                "REVERT_HEAD",
                "BISECT_LOG",
            )
            if (git_dir / name).exists()
        ]
        merge_files = _merge_files(repo, base, branch)
        rc2, stat, _ = _git(repo, "status", "--porcelain")
        stat_lines = stat.splitlines() if rc2 == 0 else []
        untracked_set = {
            l[3:].strip().rstrip("/")
            for l in stat_lines
            if l.strip().startswith("??")
        }
        diverge = [f for f in merge_files if _merge_file_diverged(repo, base, branch, f)]
        untracked_collisions = [f for f in diverge if f.rstrip("/") in untracked_set]
        if markers or diverge:
            issues.append(
                {
                    "repo": str(repo),
                    "merge_state": markers,
                    "merge_diverge": diverge,
                    "untracked_collisions": untracked_collisions,
                }
            )
    return issues


def _merge_tree_conflicts(repo, base, branch):
    """闸四收约预检：git merge-tree --write-tree 读预检本场归并，内容冲突即返文件清单。

    在现有前置态探针（_close_precondition）后、真归并前逐仓执行。merge-tree 只
    读不写，按各仓 merge-base 自动解算本场 merge --no-ff 的内容冲突；退出码零即
    净过放行，非零即内容冲突整批拒零动作。冲突文件清单从 CONFLICT (content)
    行解析文件面，堵 close 半程不可达三例（mathrefmt2/facepark/idwire）。
    """
    rc, out, _ = _git(repo, "merge-tree", "--write-tree", base, branch)
    if rc == 0:
        return {"repo": str(repo), "clean": True, "conflicts": []}
    files = []
    for line in out.splitlines():
        stripped = line.strip()
        if stripped.startswith("CONFLICT") and "Merge conflict in " in stripped:
            files.append(stripped.split("Merge conflict in ")[-1].strip())
    return {
        "repo": str(repo),
        "clean": False,
        "conflicts": sorted(set(files)),
        "exit_code": rc,
        "detail": (out.strip() or "").splitlines()[:6],
    }


def _merge_tree_precheck(session):
    """闸四预检逐仓：本场真归并前先于 merge-tree 探测内容冲突，任一仓冲突即整批拒。

    只对活在的副本支预检：支已删即 rev-parse 失败，close 归并段同样跳过即无本场
    归并可冲突，不误拦残局收束（test_close_branch_already_gone 复演残留）。
    """
    precheck = []
    for entry in session["repos"]:
        base = entry.get("base_branch") or ""
        branch = entry.get("branch") or ""
        if not base or not branch or base == branch:
            continue
        rc_v, _, _ = _git(entry["repo"], "rev-parse", "--verify", "--quiet", f"refs/heads/{branch}")
        if rc_v != 0:
            continue
        result = _merge_tree_conflicts(entry["repo"], base, branch)
        if not result["clean"]:
            precheck.append(result)
    return precheck


def close_session(package_value, force, reason, at, root, ledger, locks_ledger=None):
    """收约三检：租内锁清零、前置态整批拒、分支归并删支、拆本吊销。
    
    SPEC-020 租约收约硬化：前置机械对表三步骤：
    1. 枚举主树脏位（本地修改加未跟踪）中与合并目标路径相交的逐件清单
    2. 脏位内容与分支对应内容逐件 diff——同内容即让位放行；纯追加形让位归并；非纯追加形整批拒
    3. 工地卫生检查——工地存在未提交修改或未跟踪文件即拦收约
    """
    stem, _ = resolve_package(package_value, root, must_exist=False)
    events = load_ledger(ledger)
    actives = [
        event
        for event in active_sessions(events).values()
        if event.get("package") == stem
    ]
    if not actives:
        raise StateError(f"no active session for package: {stem}")
    if len(actives) > 1:
        raise StateError(f"ambiguous active sessions for package: {stem}")
    session = actives[0]
    from lease.lockcore import active_locks, load_events as load_lock_events

    if locks_ledger is None:
        locks_ledger = tool_dir() / "ledger" / "locks.ndjson"
    held = active_locks(load_lock_events(locks_ledger, "locks ledger"), root)
    sid_held = sorted(path for path, holders in held.items() if any(s == session["session_id"] for s, _ in holders))
    if sid_held:
        raise StateError(f"locks held by session, unlock first: {sid_held}")
    
    # SPEC-020 步骤三：工地卫生检查（在前置态探针之前执行）
    for entry in session["repos"]:
        worktree_path = Path(entry["worktree"])
        clean_result = check_worktree_clean(str(worktree_path))
        if not clean_result["clean"]:
            raise StateError(
                "工地卫生检查失败：存在未提交修改或未跟踪文件，提交归代理责任，工具不代提交。"
                + json.dumps({"issues": clean_result["issues"]}, ensure_ascii=False)
            )
    
    # 步骤一与步骤二：前置机械对表（detect_merge_conflicts + is_pure_append_conflict）
    # 对每个仓检测冲突并判别三态
    merge_conflicts = []
    for entry in session["repos"]:
        repo = entry["repo"]
        base = entry.get("base_branch") or ""
        branch = entry.get("branch") or ""
        if not base or not branch or base == branch:
            continue
        conflicts = detect_merge_conflicts(repo, base, branch)
        conflict_type_result = is_pure_append_conflict(conflicts)
        merge_conflicts.append({
            "repo": repo,
            "conflicts": conflicts,
            "conflict_type": conflict_type_result["conflict_type"],
            "files": conflict_type_result["files"],
        })
    
    # 步骤二：根据三态判别执行不同处置
    for mc in merge_conflicts:
        repo = mc["repo"]
        base = None
        branch = None
        for entry in session["repos"]:
            if entry["repo"] == repo:
                base = entry.get("base_branch") or ""
                branch = entry.get("branch") or ""
                break
        
        if base is None or branch is None:
            continue
        
        conflict_type = mc["conflict_type"]
        files = mc["files"]
        
        # 纯追加形：让位归并（backup + checkout + merge）
        if conflict_type == "append":
            # 计算 backup_dir 路径（在 close_session 函数作用域内）
            backup_dir = (
                Path(entry["worktree"]).parent.parent
                / ".close-backups" / stem / session["session_id"] / Path(repo).name
            )
            append_files = [f for f in files if files[f].get("type") == "append"]
            backup_result = backup_conflict_files(repo, append_files, backup_dir)
            allow_result = allow_and_merge(repo, base, branch, append_files)
            # 补笔清单生成：以 live_only 为源避免依赖合并时序
            live_only_lines = {
                f: files[f].get("live_only", []) for f in append_files
            }
            re_certify = generate_re_certify_hashes(
                backup_dir, append_files, repo_path=repo,
                live_only_lines=live_only_lines,
            )
            # 记录让位归并结果
            mc["allow_and_merge"] = {
                "backup_result": backup_result,
                "allow_result": allow_result,
                "re_certify": re_certify,
            }
            # 记录 chain_union 信息供台账
            mc["chain_union"] = {
                f: {
                    "backup_path": backup_result["backed_up"].get(f),
                    "live_only_count": len(files[f].get("live_only", [])),
                    "re_certify_hashes": re_certify["re_certify_hashes"].get(f, []),
                }
                for f in files if files[f].get("type") == "append"
            }
        
        # 同内容：让位放行（checkout 让位）
        elif conflict_type == "same":
            checkout_files = [f for f in files if files[f].get("type") == "same"]
            untracked_same_files = [f for f in files if files[f].get("type") == "same" and files[f].get("untracked", False)]
            if checkout_files:
                allow_result = allow_and_merge(repo, base, branch, checkout_files, untracked_same_files)
                mc["allow_and_merge"] = {
                    "allow_result": allow_result,
                    "same_files": checkout_files,
                    "untracked_same_files": untracked_same_files,
                }
        
        # 非纯追加（真分叉）：整批拒
        elif conflict_type == "diverged":
            # 整批拒并回显事由
            raise StateError(
                "收约被阻：存在真分叉冲突（非纯追加形），整批拒零动作。"
                + json.dumps({"repo": repo, "diverged_files": [f for f in files if files[f].get("type") == "diverged"]}, ensure_ascii=False)
            )

    # 预收提交最小化（closefix-solo 病一）：add -A 收窄为三点归并面∩真脏位——
    # 台账追加面活行不入预收提交即不固化（活面留待 append-only 追加）；面外
    # 脏件（分支未触碰）merge 零写入即无碰撞，保持未提交态不动。
    # 未跟踪件照单登记不清场（closefix-solo 病二结构性替代）：stash 舞步根除，
    # 收约全路径（成功/失败/拒）主树未跟踪件零离盘——面内未跟踪件已由步骤二
    # same 让位（unlink 后 merge 写回同内容字节）或 diverged 整批拒处置，
    # 面外未跟踪件零风险零触碰。
    for entry in session["repos"]:
        repo = entry["repo"]
        base = entry.get("base_branch") or ""
        branch = entry.get("branch") or ""
        if not base or not branch or base == branch:
            continue
        face = set(_merge_files(repo, base, branch))
        rc_st, out_st, _ = _git(repo, "status", "--porcelain")
        if rc_st != 0:
            continue
        staged_candidates = []
        for line in out_st.splitlines():
            if not line.strip():
                continue
            code, path = line[:2], line[3:].strip().rstrip("/")
            if code.strip() == "??":
                continue  # 未跟踪件零卷入预收提交（照单不清场）
            if " -> " in path:
                staged_candidates.extend(
                    p.strip().rstrip("/") for p in path.split(" -> ") if p.strip().rstrip("/") in face
                )
            elif path in face:
                staged_candidates.append(path)
        if staged_candidates:
            _git(repo, "add", "--", *sorted(set(staged_candidates)))
            _git(repo, "-c", "core.hooksPath=/dev/null", "commit", "-m",
                 f"closeguard-solo: pre-close working tree commit ({stem})")
    
    # 原有前置态探针（合并态与共享面脏）
    blockers = _close_precondition(session)
    if blockers:
        raise StateError(
            "收约前置态被阻：目标仓合并态或共享面脏即整批拒零部分动作，详情 "
            + json.dumps(blockers, ensure_ascii=False)
        )
    mt_precheck = _merge_tree_precheck(session)
    if mt_precheck:
        raise StateError(
            "收约预检被阻（merge-tree 本场归并有内容冲突即整批拒零动作半程不可达被堵）：详情 "
            + json.dumps(mt_precheck, ensure_ascii=False)
        )
    
    # 将 merge_conflicts 中的 chain_union 信息存入字典供后续使用
    merge_chain_union = {}
    for mc in merge_conflicts:
        if "chain_union" in mc:
            merge_chain_union[mc["repo"]] = mc["chain_union"]
    
    removed = []
    failed = []
    for entry in session["repos"]:
        worktree_path = Path(entry["worktree"])
        record = dict(entry)
        base = entry.get("base_branch") or ""
        branch = entry.get("branch") or ""
        ahead = "0"
        if base and branch:
            rc_a, out_a, _ = _git(
                entry["repo"], "rev-list", "--count", f"{base}..{branch}"
            )
            ahead = out_a.strip() if rc_a == 0 else "0"
        if base and branch and ahead != "0":
            # 并集复查让位（basefix-solo 批 F-3）：纯追加 ndjson 活面先备份
            # 让位再归并，归并后重算补笔清单随 record 入台账，链语义修复
            # 归书简补笔通道。
            union_files = [
                f for f in _merge_files(entry["repo"], base, branch)
                if _chain_append_grown(entry["repo"], base, branch, f) is not None
            ]
            chain_union = {}
            if union_files:
                backup_dir = (
                    Path(entry["worktree"]).parent.parent
                    / ".close-backups" / stem / session["session_id"] / Path(entry["repo"]).name
                )
                union_report = _chain_union_yield(
                    entry["repo"], base, branch, union_files, backup_dir
                )
                chain_union = {
                    f: {"backup_path": info["backup_path"], "live_only_count": len(info["live_only"])}
                    for f, info in union_report.items()
                }
            # 使用前置机械对表中的 chain_union 信息（如果已处理）
            if entry["repo"] in merge_chain_union:
                chain_union = merge_chain_union[entry["repo"]]
                record["chain_union"] = chain_union
            rc_m, _, err_m = _git(
                entry["repo"], "merge", "--no-ff", branch,
                "-m", f"merge: {stem} 副本归并",
            )
            if rc_m != 0:
                record["result"] = "merge_failed"
                record["detail"] = err_m.strip()[:200]
                record["chain_union"] = chain_union
                failed.append(record)
                continue
            if union_files:
                record["chain_union"] = {
                    f: {
                        "backup_path": info["backup_path"],
                        "live_only_count": len(info["live_only"]),
                        "re_certify_hashes": _chain_union_recertify(entry["repo"], f, info),
                    }
                    for f, info in union_report.items()
                }
        if worktree_path.exists():
            args = ["worktree", "remove"]
            if force:
                args.append("--force")
            rc, _, stderr = _git(entry["repo"], *args, entry["worktree"])
            if rc != 0:
                record["result"] = "failed"
                record["detail"] = stderr.strip()[:200]
                failed.append(record)
                continue
            record["worktree_result"] = "removed"
            # closefix-solo 病二：stash 舞步已根除（照单不清场），无 pop 位
        else:
            record["result"] = "missing"
            record["worktree_result"] = "missing"
        if branch:
            rc_v, _, _ = _git(
                entry["repo"], "rev-parse", "--verify", "--quiet",
                f"refs/heads/{branch}",
            )
            if rc_v == 0:
                rc_d, _, err_d = _git(entry["repo"], "branch", "-d", branch)
                if rc_d != 0:
                    record["result"] = "branch_delete_failed"
                    record["detail"] = err_d.strip()[:200]
                    failed.append(record)
                    continue
                record["branch_result"] = "deleted"
            else:
                record["branch_result"] = "already_gone"
        if record.get("result") != "missing":
            record["result"] = "removed"
        removed.append(record)
    if failed:
        append_event(
            ledger,
            {
                "event": "close_failed",
                "package": stem,
                "reason": reason,
                "failed": failed,
                "removed": removed,
                "session_id": session["session_id"],
                "failed_at": at if at is not None else now_iso(),
                "tool": {"name": TOOL_NAME, "version": _version()},
            },
        )
        return {"failed": failed, "removed": removed, "revoked": False, "session_id": session["session_id"]}
    line = {
        "event": "revoked",
        "package": stem,
        "reason": reason,
        "removed": removed,
        "revoked_at": at if at is not None else now_iso(),
        "session_id": session["session_id"],
        "tool": {"name": TOOL_NAME, "version": _version()},
    }
    append_event(ledger, line)
    return {"failed": [], "removed": removed, "revoked": True, "session_id": session["session_id"]}


def status(ledger, package_stem=None, history=False):
    """查册：在册为主，history 出全量事件。"""
    events = load_ledger(ledger)
    actives = sorted(active_sessions(events).values(), key=lambda item: item.get("issued_at", ""))
    if package_stem is not None:
        actives = [item for item in actives if item.get("package") == package_stem]
    revoked = sum(1 for event in events if event.get("event") == "revoked")
    report = {
        "active": actives,
        "header": {
            "events": len(events),
            "ledger": str(ledger),
            "tool": {"name": TOOL_NAME, "version": _version()},
        },
        "summary": {"active": len(actives), "revoked": revoked},
    }
    if history:
        report["events"] = events
    return report


def _version():
    from lease import __version__

    return __version__


def detect_merge_conflicts(repo_path, base_branch, branch):
    """detect_merge_conflicts：枚举主树脏位中与合并目标路径相交的逐件清单。

    步骤一：枚举主树脏位（本地修改加未跟踪）中与合并目标路径相交的逐件清单。
    归并面取三点式（closefix-solo 病一）：以 merge-base 为基准取 branch 侧
    改动，主干侧前进零入面即「分支无新内容可归并」的台账活面零触碰。
    返回：冲突文件清单，含本地修改与未跟踪文件。
    """
    conflicts = []

    # 获取归并面（三点式差集，closefix-solo 病一）
    merge_files = _merge_files(repo_path, base_branch, branch)

    # 获取真实脏位与未跟踪文件（leaseopt-fixguard-solo 修复）：对 HEAD 的
    # 修改行与暂存行与未跟踪行俱入面；判定域从差集全集收缩为差集交真实脏位，
    # 净态差集件（批期内基线前进、盘上==HEAD）退场交 git 三方合并仲裁，
    # 真内容冲突由 merge-tree 闸四预检拦（leaseopt-audit 批一病灶一）
    rc2, stat, _ = _git(repo_path, "status", "--porcelain")
    stat_lines = stat.splitlines() if rc2 == 0 else []
    untracked_files = []
    dirty_set = set()
    for line in stat_lines:
        if not line.strip():
            continue
        code, path = line[:2], line[3:].strip().rstrip("/")
        if code.strip() == "??":
            untracked_files.append(path)
        elif " -> " in path:
            dirty_set.update(p.strip().rstrip("/") for p in path.split(" -> "))
        else:
            dirty_set.add(path)

    def _hits_untracked(f):
        """未跟踪命中：路径自身或落在未跟踪目录内（目录 vs 树形态，closefix 补）。"""
        norm = f.rstrip("/")
        for u in untracked_files:
            us = u.rstrip("/")
            if norm == us or norm.startswith(us + "/"):
                return True
        return False

    # 收集真脏位且在归并面中的文件；净态差集件直接退场（fixguard F-1 修复核心）。
    # 未跟踪面内件不再跳过（closefix-solo 病二配套）：同内容形入 conflicts 由
    # same+untracked 处置让位（unlink 后 merge 写回同内容字节），异内容即真分叉
    # 整批拒——旧跳过形依赖 stash 舞步善后即吞件通道，stash 根除后必须显式处置。
    for f in merge_files:
        if f not in dirty_set and not _hits_untracked(f):
            continue
        path = Path(repo_path) / f
        if path.is_file():
            try:
                ondisk = path.read_text(encoding="utf-8")
            except OSError:
                ondisk = None
        else:
            ondisk = None

        # 检查是否在分支内容中（同内容即让位）
        rc_b, blob_b, _ = _git(repo_path, "show", f"{branch}:{f}")
        branch_content = blob_b if rc_b == 0 else None

        rc_a, blob_a, _ = _git(repo_path, "show", f"{base_branch}:{f}")
        base_content = blob_a if rc_a == 0 else None

        untracked_collision = _hits_untracked(f)

        # 未跟踪碰撞：未跟踪文件与 branch 路径碰撞（同内容由 is_pure_append_conflict
        # 判 same+untracked 后让位放行，异内容判 diverged 整批拒）
        if untracked_collision:
            conflicts.append({
                "path": f,
                "local_content": ondisk,
                "branch_content": branch_content,
                "base_content": base_content,
                "untracked_collision": True,
            })
            continue

        # 本地没有修改（ondisk 为 None 或与 branch 一致）→ 正常分支提交，不视为冲突
        if ondisk is None or ondisk == branch_content:
            continue

        conflicts.append({
            "path": f,
            "local_content": ondisk,
            "branch_content": branch_content,
            "base_content": base_content,
            "untracked_collision": False,
        })

    # 未跟踪且不在归并面的文件零触碰（closefix-solo 病一收编）：三点面外即
    # branch 未改动该路径，merge 零写入即无覆盖风险——旧尾环把面外未跟踪件
    # 误判碰撞（目录 vs 树形态旁证即 ledgerloss5 §九），废止。
    return {"repo": repo_path, "conflicts": conflicts}


def is_pure_append_conflict(conflict_files):
    """is_pure_append_conflict：判别纯追加形。
    
    步骤二：脏位内容与分支对应内容逐件 diff
    - 同内容：本地内容与 branch 内容一致或与 base 内容一致（让位放行）
    - 纯追加形：本地文件是 base 内容 + 尾部新增（让位归并）
    - 非纯追加（真分叉）：其他情况（整批拒）
    - 未跟踪碰撞：未跟踪文件内容与 branch 内容一致（让位放行），否则视为冲突
    
    返回：{"conflict_type": "same"|"append"|"diverged", "files": {...}}
    """
    result = {"conflict_type": "diverged", "files": {}}
    
    for cf in conflict_files.get("conflicts", []):
        path = cf["path"]
        ondisk = cf.get("local_content")
        branch_content = cf.get("branch_content")
        base_content = cf.get("base_content")
        untracked_collision = cf.get("untracked_collision", False)
        
        # 未跟踪碰撞：未跟踪文件与 branch 路径碰撞
        if untracked_collision:
            # 未跟踪文件内容与 branch 内容一致，让位放行
            if branch_content is not None and ondisk == branch_content:
                result["files"][path] = {"type": "same", "untracked": True}
                continue
            # 未跟踪文件内容与 branch 内容不一致，视为真分叉
            result["conflict_type"] = "diverged"
            result["files"][path] = {"type": "diverged", "untracked": True}
            continue
        
        # 同内容：本地内容与 branch 一致（SPEC-020 严义：不看 base，避免误判 base==ondisk 形 fast-forward）
        is_same = False
        if ondisk is not None and branch_content is not None and ondisk == branch_content:
            is_same = True
        
        if is_same:
            result["files"][path] = {"type": "same"}
            continue
        
        # 纯追加形：仅适用于 .ndjson 文件
        # 只有当 base_content 存在时，才考虑纯追加（文件在 base 中存在）
        if path.endswith(".ndjson") and ondisk and base_content:
            ondisk_lines = ondisk.splitlines()
            base_lines = base_content.splitlines()
            if len(ondisk_lines) > len(base_lines) and ondisk_lines[:len(base_lines)] == base_lines:
                # 追加内容不在 branch 中
                branch_lines = branch_content.splitlines() if branch_content else []
                branch_set = set(branch_lines)
                base_set = set(base_lines)
                live_only = [line for line in ondisk_lines[len(base_lines):] 
                            if line not in branch_set and line not in base_set]
                result["files"][path] = {"type": "append", "live_only": live_only}
                continue
        
        # 其他情况为真分叉
        result["conflict_type"] = "diverged"
        result["files"][path] = {"type": "diverged"}
    
    # 如果所有文件都是 same 或 append，才返回 append 类型
    if result["conflict_type"] == "diverged":
        all_same_or_append = True
        has_append = False
        has_same = False
        
        for cf in conflict_files.get("conflicts", []):
            path = cf["path"]
            file_info = result["files"].get(path, {})
            file_type = file_info.get("type", "diverged")
            
            if file_type == "diverged":
                all_same_or_append = False
                break
            elif file_type == "append":
                has_append = True
            elif file_type == "same":
                has_same = True
        
        if all_same_or_append:
            if has_append:
                result["conflict_type"] = "append"
            else:
                result["conflict_type"] = "same"
    
    return result


def check_worktree_clean(worktree_path):
    """check_worktree_clean：工地卫生检查。
    
    工地存在未提交修改或未跟踪文件即拦收约并回显可行动清单。
    提交归代理责任，工具不代提交。
    
    返回：{"clean": bool, "issues": [...]}
    """
    issues = []
    wt_path = Path(worktree_path)
    
    if not wt_path.exists():
        return {"clean": True, "issues": [], "worktree_missing": True}
    
    # 检查未提交修改
    rc, out, _ = _git(str(wt_path), "status", "--porcelain")
    if rc == 0:
        for line in out.splitlines():
            stripped = line.strip()
            if stripped and not stripped.startswith("??"):
                # 已跟踪但有修改
                path = stripped[3:].strip().rstrip("/")
                issues.append({
                    "type": "modified",
                    "path": path,
                    "action": "git add " + path + " && git commit",
                })
            elif stripped.startswith("??"):
                # 未跟踪文件
                path = stripped[3:].strip().rstrip("/")
                issues.append({
                    "type": "untracked",
                    "path": path,
                    "action": "git add " + path + " && git commit or remove file",
                })
    
    return {"clean": len(issues) == 0, "issues": issues}


def backup_conflict_files(repo_path, files_to_backup, backup_dir):
    """backup_conflict_files：备份保全逻辑。
    
    备份冲突文件内容至 deterministic 路径。
    
    返回：{"backed_up": {path: backup_path}, "errors": [...]}
    """
    backup_dir = Path(backup_dir)
    backup_dir.mkdir(parents=True, exist_ok=True)
    backed_up = {}
    errors = []
    
    for f in files_to_backup:
        src = Path(repo_path) / f
        dest = backup_dir / f
        try:
            dest.parent.mkdir(parents=True, exist_ok=True)
            if src.is_file():
                dest.write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
                backed_up[f] = str(dest)
            else:
                # 未跟踪文件，从 git checkout
                rc, blob, _ = _git(repo_path, "show", f"{f}")
                if rc == 0:
                    dest.write_text(blob, encoding="utf-8")
                    backed_up[f] = str(dest)
        except Exception as e:
            errors.append({"path": f, "error": str(e)})
    
    return {"backed_up": backed_up, "errors": errors}


def allow_and_merge(repo_path, base_branch, branch, files_to_merge, untracked_same_files=None):
    """allow_and_merge：让位归并逻辑（closefix-solo 病一：台账面冻结豁免）。

    台账追加面（冻结常量命中）永不 checkout 让位，走并集追加通道——活面
    原序原行保持，分支独有行补尾，写点持 ledger_lock（H-3 绕锁根除）。
    其余文件备份后 checkout 让位（git checkout <branch> -- <f>，不是从
    HEAD 还原）。同内容未跟踪文件需要先移除再 merge（内容字节由 branch
    写回保全）。

    参数：
    - repo_path: 仓库路径
    - base_branch: 基线分支
    - branch: 副本分支
    - files_to_merge: 需要 checkout 的文件列表
    - untracked_same_files: 同内容未跟踪文件列表（需先移除）

    返回：{"merged": [...], "untracked_removed": [...], "errors": [...],
           "union_appended": {path: {"live_rows", "branch_only_appended"}}}
    """
    merged = []
    untracked_removed = []
    errors = []
    union_appended = {}

    # 先处理同内容未跟踪文件（需要先移除）
    if untracked_same_files:
        for f in untracked_same_files:
            try:
                file_path = Path(repo_path) / f
                if file_path.exists():
                    file_path.unlink()
                    untracked_removed.append(f)
            except Exception as e:
                errors.append({"path": f, "error": str(e), "action": "remove_untracked"})

    for f in files_to_merge:
        try:
            if _is_ledger_append_face(f):
                # 冻结豁免：台账追加面永不整文件盖版，并集追加保全活行
                info = _union_append_face(repo_path, branch, f)
                union_appended[f] = info
                merged.append(f)
                continue
            # 从 branch 复制到 working tree（不是从 HEAD 还原）
            rc, _, err = _git(repo_path, "checkout", branch, "--", f)
            if rc == 0:
                merged.append(f)
            else:
                errors.append({"path": f, "error": err.strip()[:200], "action": "checkout"})
        except Exception as e:
            errors.append({"path": f, "error": str(e), "action": "checkout"})

    return {
        "merged": merged,
        "untracked_removed": untracked_removed,
        "errors": errors,
        "union_appended": union_appended,
    }


def generate_re_certify_hashes(backup_dir, files, repo_path=None, live_only_lines=None):
    """generate_re_certify_hashes：补笔清单生成逻辑。
    
    归并后重读合并件算 re_certify_hashes（live_only 独有行哈希）。live_only 是
    「ondisk 截断 base 后、且不在 branch 集的行」——即实际需补笔的行，与合并时序
    解耦：传入 live_only 时直接以 live_only 为源对合并件做差集，未传时回退
    「backup 全行 - 合并件」旧语义（仅在合并已完成前提下成立）。
    
    参数：
    - backup_dir: 备份目录路径
    - files: 文件名列表
    - repo_path: 仓库路径（用于读取合并后文件）
    - live_only_lines: {path: [line, ...]}，可选
    
    返回：{"re_certify_hashes": {path: [hash1, hash2, ...]}, "backup_content": {...}}
    """
    backup_path = Path(backup_dir)
    result = {"re_certify_hashes": {}, "backup_content": {}}
    live_only_lines = live_only_lines or {}
    
    for f in files:
        backup_file = backup_path / f
        if backup_file.is_file():
            try:
                backup_content = backup_file.read_text(encoding="utf-8")
                result["backup_content"][f] = backup_content
                
                # live_only 模式：以 live_only 为源，避免依赖合并时序
                if f in live_only_lines:
                    source_lines = live_only_lines[f]
                else:
                    source_lines = backup_content.splitlines()
                
                if repo_path:
                    merged_file = Path(repo_path) / f
                    if merged_file.is_file():
                        merged_content = merged_file.read_text(encoding="utf-8")
                        merged_lines = set(merged_content.splitlines())
                        hashes = [
                            hashlib.sha256(line.encode("utf-8")).hexdigest()[:16]
                            for line in source_lines
                            if line not in merged_lines
                        ]
                        result["re_certify_hashes"][f] = hashes
            except Exception:
                pass
    
    return result
