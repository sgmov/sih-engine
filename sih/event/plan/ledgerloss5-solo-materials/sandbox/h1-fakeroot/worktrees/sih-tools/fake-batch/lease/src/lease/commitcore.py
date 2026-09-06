"""提交与对表：lease 1.1 承 DEC-012 裁决三与四即提交界与认证门同界、通道唯一。

commit 四验即会话在册、暂存全落 allow、该段认证在链、信息机械生成三形态，
模板单源于本模块，调用方不携自由 message。reconcile 三方对表即 git log 对
会话台账对 trail 对 bypass 台账，绕行与存量露形，只报不拦。
守卫件即 guardhook-solo 批增设：bypass 台账（--no-verify 显式绕行留痕）与
install-hooks/uninstall-hooks（core.hooksPath 守卫安装态）与对表新类
bypass（已登记不告警）/unbypassed（未登记告警）。
"""

import re
import subprocess
from pathlib import Path

from lease.core import (
    StateError,
    WorktreeError,
    _git,
    append_event,
    load_ledger,
    now_iso,
    tool_dir,
)
from lease.lockcore import LockBlocked, LocksError, scope_allows, select_session

MESSAGE_REASONS = ("session_not_active", "repo_not_in_session", "nothing_staged", "staged_out_of_scope", "cert_not_on_chain")
SESSION_LINE = re.compile(r"^session: ([0-9a-f]{16})\b", re.MULTILINE)
CERT_LINE = re.compile(r"\bcert: ([0-9a-f]+)")

# 内置封线即各仓对表起算界，承 audit-049 封字令 2026-08-27 与封窗裁定 2026-08-27 即修订十二
# 与 sealwin2 修订与 sealwin3 封窗令 2026-09-01。
# 起算取界线父即 range 含界线本身，日常对表免 --base 手输，改线走版本管理即此表变更须升版本。
# sealwin3 界线 2026-09-01 自 sealwin2 界线 22550a6 / d7f9338 前移至 scrutmerge-switch-solo 归并点
# 即 sih-engine 4146851 与 sih-tools 91d14d4f，封窗只移界线不碰历史。SEAL_EXEMPTS 追认表零改动。
SEAL_BASES = {
    "sih-engine": "4146851",
    "sih-tools": "91d14d4f",
}

# 追认表即界后存量钉死名单，承 2026-08-27 用户追认令即封字令修订一。
# 表内提交归 sealed 类不计违规不入尾单，只补身份不动历史；表变更须升版本；
# facet 两笔 3451c6f 与 1592b4a 为追认令后真增量，永不入表。
SEAL_EXEMPTS = {
    "sih-engine": {
        "dc31d77729e27f85ed7d22db89357a929d2370de",
        "1c86f5d3e0755cfb15db23475ee684ceb6914e7b",
        "86ce450520f38ebff0e85dd149328c6270600071",
    },
    "sih-tools": {
        "be75d70f8054f186bda1c1913182bcf4afec0146",
        "4adc597b8d6ac916c5211a08c45fc323c75b2fa0",
        "bb7a5490895cd1265d1052b6f75f6db4609b6b25",
        "5cb0c1d324720fccf05421db77d938a86b5d941e",
        "16faa748f1f61ca8f52e061c2fcba3502651f623",
        "568b4596b37984f4df4c51dbb01dc257c2852b56",
        "7a2128643050c345d6aebb8c1cc8bce23a8d9047",
        "18cdfe7096037669ee881ecd8e3d5f04d8bca64f",
        "d4a4d7f5c9feee3724d17b3b05fdbad4d26a73a7",
        "0f24b1c49d90463fcbd3e501326b122495212bd8",
        "6ef6a03974c4e08ffc0fbb569ae1088a864a2511",
        "7c695660f05fe21321b409cac06554c6696599de",
        "ec03e0ebfebb51e78e5583068fb682ff9d902820",
        "4e7f66a5f18c5ac37313c72aaa2598941626e8ee",
        "d7d38579d2f2ac5639bff695e6c8a1cf21eb5888",
        "2e888168b25db5c437cd1014003dfd3b5a979537",
        "c95fe11f71df43c4132c168a7208711835c83bf6",
        "b0bd556b81f33094671efe46c44194594ec3b253",
    },
}


def _repo_key(repo):
    """仓名键即路径组成件倒序首个命中，围堰副本路径含仓名组成件亦命中承 pk-031。"""
    parts = [p for p in Path(repo).resolve().parts[::-1]]
    return parts


def seal_exempt_set(repo):
    """按仓名取追认表，表外仓返空集即无豁免。"""
    for part in _repo_key(repo):
        if part in SEAL_EXEMPTS:
            return SEAL_EXEMPTS[part]
    return frozenset()


def resolve_default_base(repo, base=None):
    """缺 base 即按仓名套内置封线，返 base 加封线号。

    封线提交不在该仓即同名异源回落全史显式，不误拦不误报。
    """
    if base:
        return base, None
    seal = None
    for part in _repo_key(repo):
        if part in SEAL_BASES:
            seal = SEAL_BASES[part]
            break
    if seal is None:
        return None, None
    rc, _out, _stderr = _git(repo, "cat-file", "-e", seal)
    if rc != 0:
        return None, None
    return f"{seal}^", seal


class CommitBlocked(ValueError):
    """提交拦截即四验任一不过，退出码一。"""

    def __init__(self, reason, detail=None):
        super().__init__(reason)
        self.reason = reason
        self.detail = detail


def build_message(stage, stem, session_id, subject, seq=None, cert=None, base=None, note=None):
    """信息机械生成三形态即 wip 与段结算与收约归并，模板单源。"""
    if stage == "wip":
        return f"{stem} wip {subject}\n\nsession: {session_id}\n"
    if stage == "settle":
        head = f"{stem} 段{seq} {subject}"
        meta = f"session: {session_id} cert: {cert} base: {base}"
        parts = [head, "", meta]
        if note:
            parts.extend(["", note])
        return "\n".join(parts) + "\n"
    raise WorktreeError(f"unknown stage: {stage}")


def _load_trail_hashes(trails):
    """链上认证哈希集即 certification_completed 事件的 event_hash 全值。"""
    hashes = set()
    for trail in trails:
        for event in load_ledger(trail):
            if event.get("event_type") == "certification_completed":
                value = event.get("event_hash")
                if isinstance(value, str) and value:
                    hashes.add(value)
    return hashes


def _cert_on_chain(cert, trail_hashes):
    """cert 取八位短形或全值，前缀命一中即过。"""
    for value in trail_hashes:
        if value.startswith(cert):
            return True
    return False


def _resolve_repo_entry(session, repo_path):
    """仓条目解析：正身仓路径精确匹配，或本会话副本路径作别名。"""
    for entry in session.get("repos", []):
        if entry.get("repo") == str(repo_path):
            return entry
    for entry in session.get("repos", []):
        if entry.get("worktree") == str(repo_path):
            return entry
    return None


def _base_label(entry, repo_path):
    """base 标签：副本提交取基线分支加 merge-base 短号，主检出取分支名加 HEAD 短号。"""
    if entry.get("worktree") == str(repo_path):
        base_branch = entry.get("base_branch") or ""
        if not base_branch:
            return ""
        rc_mb, mb, _ = _git(repo_path, "merge-base", "HEAD", base_branch)
        if rc_mb != 0:
            return ""
        rc_s, short, _ = _git(repo_path, "rev-parse", "--short", mb.strip())
        return f"{base_branch}@{short.strip()}" if rc_s == 0 else ""
    rc, branch, _ = _git(repo_path, "rev-parse", "--abbrev-ref", "HEAD")
    rc2, head, _ = _git(repo_path, "rev-parse", "--short", "HEAD")
    return f"{branch.strip()}@{head.strip()}" if rc == 0 and rc2 == 0 else ""


def commit_staged(repo, session_id, stage, subject, seq, cert, note, root, ledger, trails):
    """四验后提交即手动期提交执行正身路径，承 DEC-011 修订四；副本路径作仓别名承编辑位迁移。"""
    events = load_ledger(ledger)
    session = select_session(events, session_id)
    repo_path = Path(repo).resolve()
    entry = _resolve_repo_entry(session, repo_path)
    if entry is None:
        raise CommitBlocked(
            "repo_not_in_session",
            {
                "repo": str(repo_path),
                "repos": [item.get("repo") for item in session.get("repos", [])],
                "worktrees": [item.get("worktree") for item in session.get("repos", [])],
            },
        )
    if entry.get("worktree") != str(repo_path):
        # commit 须指围堰承 pk-031 证据一，主检出直提拒并示副本路径
        raise CommitBlocked(
            "commit_must_target_worktree",
            {"repo": str(repo_path), "worktree": entry.get("worktree")},
        )
    rc, staged, stderr = _git(repo_path, "-c", "core.quotepath=off", "diff", "--cached", "--name-only")
    if rc != 0:
        raise WorktreeError(f"git diff failed: {stderr.strip()}")
    staged_files = [line for line in staged.splitlines() if line.strip()]
    if not staged_files:
        raise CommitBlocked("nothing_staged")
    try:
        repo_rel = Path(entry["repo"]).relative_to(Path(root).resolve())
    except ValueError as exc:
        raise WorktreeError(f"repo outside workspace root: {entry['repo']}") from exc
    outside = [
        f"{repo_rel}/{name}"
        for name in staged_files
        if not scope_allows(session.get("allow", []), f"{repo_rel}/{name}")
    ]
    if outside:
        raise CommitBlocked(
            "staged_out_of_scope",
            {"outside": outside, "allow": session.get("allow", [])},
        )
    if stage == "settle":
        if not cert:
            raise WorktreeError("settle requires --cert")
        trail_hashes = _load_trail_hashes(trails)
        if not _cert_on_chain(cert, trail_hashes):
            raise CommitBlocked("cert_not_on_chain", {"cert": cert, "trails": [str(t) for t in trails]})
    base = _base_label(entry, repo_path)
    message = build_message(
        stage,
        session.get("package", ""),
        session["session_id"],
        subject,
        seq=seq,
        cert=cert,
        base=base,
        note=note,
    )
    proc = subprocess.run(
        ["git", "-C", str(repo_path), "commit", "-m", message],
        capture_output=True,
        text=True,
        timeout=30,
    )
    if proc.returncode != 0:
        raise StateError(f"git commit failed: {proc.stderr.strip()[:200]}")
    rc3, sha, _ = _git(repo_path, "rev-parse", "--short", "HEAD")
    return {
        "base": base,
        "checks": ["session_active", "staged_in_scope"] + (["cert_on_chain"] if stage == "settle" else []),
        "commit": sha.strip() if rc3 == 0 else "",
        "message": message,
        "package": session.get("package", ""),
        "session_id": session["session_id"],
        "stage": stage,
    }


def _known_sessions(ledger):
    """台账已知即 issued 出现过的会话号集与包名集。"""
    session_ids = set()
    packages = set()
    for event in load_ledger(ledger):
        if event.get("event") == "issued":
            session_ids.add(event.get("session_id"))
            packages.add(event.get("package"))
    return session_ids, packages


BYPASS_LEDGER_NAME = "bypass.ndjson"
HOOKS_DIR = Path(__file__).resolve().parents[2] / "hooks"


def default_bypass_ledger():
    """绕行台账缺省位即工具台账目录下 bypass.ndjson。"""
    return tool_dir() / "ledger" / BYPASS_LEDGER_NAME


def record_bypass(repo, sha, reason, session, at, ledger_path):
    """绕行留痕即 --no-verify 显式授权通道的台账一笔，只增不改承工程基线四。"""
    line = {
        "at": at if at is not None else now_iso(),
        "event": "bypassed",
        "reason": reason,
        "repo": str(Path(repo).resolve()),
        "session": session,
        "sha": sha,
        "tool": {"name": "lease", "version": _version()},
    }
    append_event(ledger_path, line)
    return line


def load_bypass_entries(ledger_path):
    """绕行台账读入即 bypassed 事件列表，缺席即空册。"""
    return [event for event in load_ledger(ledger_path) if event.get("event") == "bypassed"]


def _bypass_hit(entries, repo, sha_full):
    """绕行对表即仓路径解析等值加 sha 前缀互含（登记短号或全号皆可命中）。"""
    try:
        repo_resolved = str(Path(repo).resolve())
    except OSError:
        repo_resolved = str(repo)
    for entry in entries:
        entry_repo = entry.get("repo") or ""
        try:
            entry_repo = str(Path(entry_repo).resolve())
        except OSError:
            pass
        if entry_repo != repo_resolved:
            continue
        entry_sha = (entry.get("sha") or "").strip()
        if entry_sha and (sha_full.startswith(entry_sha) or entry_sha.startswith(sha_full)):
            return True
    return False


def install_hooks(repos):
    """守卫安装即向仓写 core.hooksPath 指仓内 hooks 目录，config 属环境态不入版控。"""
    results = []
    for repo in repos:
        repo_path = str(Path(repo).resolve())
        rc, _out, stderr = _git(repo_path, "config", "core.hooksPath", str(HOOKS_DIR))
        results.append(
            {
                "hooks_path": str(HOOKS_DIR),
                "installed": rc == 0,
                "repo": repo_path,
                "detail": None if rc == 0 else stderr.strip()[:200],
            }
        )
    return results


def uninstall_hooks(repos):
    """守卫拆卸即 unset core.hooksPath 可逆，键缺席计 already_absent 照常收敛。"""
    results = []
    for repo in repos:
        repo_path = str(Path(repo).resolve())
        rc, _out, stderr = _git(repo_path, "config", "--unset", "core.hooksPath")
        results.append(
            {
                "repo": repo_path,
                "uninstalled": rc in (0, 5),
                "state": "unset" if rc == 0 else ("already_absent" if rc == 5 else "failed"),
                "detail": None if rc in (0, 5) else stderr.strip()[:200],
            }
        )
    return results


def _version():
    from lease import __version__

    return __version__


def reconcile(repo, ledger, trails, base=None, limit=0, bypass_ledger=None):
    """三方对表加绕行台账：git log 对会话台账对 trail，分类只报不拦。

    无模板提交（既非 session/cert 形又非 merge 形）逐笔对 bypass 台账，
    已登记归 bypass 类不告警，未登记出 unbypassed 告警类，守卫残余通道显形。
    既有 SEAL 逻辑零动即 exempts 与封线起算全承旧约。
    """
    session_ids, packages = _known_sessions(ledger)
    trail_hashes = _load_trail_hashes(trails)
    if bypass_ledger is None:
        bypass_ledger = default_bypass_ledger()
    bypass_entries = load_bypass_entries(bypass_ledger)
    args = ["log", "--format=%H%x1f%ad%x1f%B%x1e", "--date=short"]
    if base:
        args.append(f"{base}..HEAD")
    if limit:
        args.extend(["-n", str(limit)])
    rc, out, stderr = _git(repo, *args)
    if rc != 0:
        raise WorktreeError(f"git log failed: {stderr.strip()}")
    exempts = seal_exempt_set(repo)
    records = []
    for raw in out.split("\x1e"):
        raw = raw.strip("\n")
        if not raw.strip():
            continue
        parts = raw.split("\x1f", 2)
        if len(parts) != 3:
            continue
        sha, date, body = (part.strip() for part in parts)
        subject = body.splitlines()[0] if body else ""
        match = SESSION_LINE.search(body)
        detail = None
        if sha in exempts:
            classification = "sealed"
        elif match:
            sid = match.group(1)
            cert = CERT_LINE.search(body)
            if sid not in session_ids:
                classification = "session_orphan"
                detail = sid
            elif cert and not _cert_on_chain(cert.group(1), trail_hashes):
                classification = "cert_missing"
                detail = cert.group(1)
            else:
                classification = "routed"
        elif subject.startswith("merge: ") and any(
            subject == f"merge: {stem} 副本归并" for stem in packages
        ):
            classification = "routed_merge"
        elif _bypass_hit(bypass_entries, repo, sha):
            classification = "bypass"
        else:
            classification = "unbypassed"
        records.append(
            {
                "class": classification,
                "date": date,
                "detail": detail,
                "sha": sha[:7],
                "subject": subject[:80],
            }
        )
    summary = {
        "bypass": sum(1 for r in records if r["class"] == "bypass"),
        "cert_missing": sum(1 for r in records if r["class"] == "cert_missing"),
        "routed": sum(1 for r in records if r["class"] in ("routed", "routed_merge")),
        "sealed": sum(1 for r in records if r["class"] == "sealed"),
        "session_orphan": sum(1 for r in records if r["class"] == "session_orphan"),
        "total": len(records),
        "unbypassed": sum(1 for r in records if r["class"] == "unbypassed"),
        "unrouted": sum(1 for r in records if r["class"] == "unrouted"),
    }
    routed_shas = [r["sha"] for r in records if r["class"] in ("routed", "routed_merge")]
    return {
        "base": base or "",
        "first_routed": routed_shas[-1] if routed_shas else None,
        "repo": str(Path(repo).resolve()),
        "summary": summary,
        "unrouted_tail": [r for r in records if r["class"] not in ("routed", "sealed")][:200],
    }
