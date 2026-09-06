"""领取账本核心：任务包领取登记声明位，append-only 台账与 ttl 过期判定。

领取登记补领取到立约之间的碰撞真空即提前避撞与可见性，声明位拒绝非执法
拒绝，同包唯一硬执法仍归闸一。账本追加即唯一写点零改写；过期读时派生即
判定时自动标注，僵尸领取不靠人清。承 PRO-07 纯粹映照零投射与 PRO-06 治理
力度由松到紧与 PRO-08 应而不藏。
"""

import json
from datetime import datetime, timedelta, timezone
from pathlib import Path

TOOL_NAME = "lease"
CLAIMANT_DEFAULT = "anon"


class ClaimsError(ValueError):
    """工具异常即账本不可读或 json 非法或参数非法，退出码二。"""


class PackageAlreadyClaimed(ValueError):
    """声明位拒绝：同包已有未过期领取即拒，载领取人与到期时刻，详情非空。

    承 claimgate-solo 任务包关键设计二：声明位拒绝 exit 一详情非空，
    硬执法归闸一即 PackageSessionActive，两拒不同位不互相替代。
    """

    def __init__(self, package, claimant, expires_at):
        self.package = package
        self.claimant = claimant
        self.expires_at = expires_at
        super().__init__(
            f"package already claimed: {package} by {claimant} until {expires_at}"
        )


class ClaimBlocked(ValueError):
    """拦截即无在领可释放或领取人不符，退出码一，理由码承载。"""

    def __init__(self, reason, detail=None):
        super().__init__(reason)
        self.reason = reason
        self.detail = detail or {}


def tool_dir():
    """工具目录即台账缺省父位，与 core.tool_dir 同位独立防循环引用。"""
    return Path(__file__).resolve().parents[2]


def default_claims_ledger():
    """领取账本缺省位即工具台账目录下 claims.ndjson。"""
    return tool_dir() / "ledger" / "claims.ndjson"


def now_iso():
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def _version():
    from lease import __version__

    return __version__


def parse_ts(value):
    """ISO8601 时戳解析，非法即工具异常。"""
    try:
        return datetime.fromisoformat(value)
    except (TypeError, ValueError) as exc:
        raise ClaimsError(f"invalid ISO8601 timestamp: {value}") from exc


def reference_time(at):
    """参照时刻：显式给参优先，缺省读钟；复演与金向量靠显式给参确定性。"""
    return parse_ts(at) if at is not None else datetime.now(timezone.utc)


def load_claims(path):
    """领取账本逐行读，缺席即空册，坏行即工具异常。"""
    path = Path(path)
    if not path.exists():
        return []
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise ClaimsError(f"claims ledger unreadable: {path}") from exc
    events = []
    for number, line in enumerate(text.splitlines(), start=1):
        if not line.strip():
            continue
        try:
            events.append(json.loads(line))
        except json.JSONDecodeError as exc:
            raise ClaimsError(f"claims ledger line {number} invalid: {exc}") from exc
    return events


def append_claim_event(path, event):
    """领取账本追加即唯一写点，单行紧凑形，历史笔原文保留零改写。"""
    path = Path(path)
    try:
        path.parent.mkdir(parents=True, exist_ok=True)
        with open(path, "a", encoding="utf-8") as handle:
            handle.write(
                json.dumps(event, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
                + "\n"
            )
    except OSError as exc:
        raise ClaimsError(f"claims ledger unwritable: {path}") from exc


def resolve_package_claims(events):
    """按包归并领取态：单遍事件序配对，claimed 置位、released 仅弹在领同包。

    同包重领不被旧 released 误抹，重复事件幂等，与 lockcore.active_sessions
    同款单遍事件序时序语义。
    """
    state = {}
    for event in events:
        package = event.get("package")
        kind = event.get("event")
        if kind == "claimed":
            state[package] = {"claim": event, "released": False}
        elif kind == "released" and package in state:
            state[package]["released"] = True
    return state


def effective_status(claim_event, released, at_dt):
    """实效状态三值：released 即释放、过参照时刻即 expired、否则 claimed。

    expired 读时派生不落笔，账本零改写承 append-only，判定时自动标注即
    僵尸领取不靠人清。
    """
    if released:
        return "released"
    expires_at = claim_event.get("expires_at")
    if at_dt is not None and expires_at and at_dt >= parse_ts(expires_at):
        return "expired"
    return "claimed"


def active_claim(events, package, at_dt):
    """同包在领取件：存在且实效状态为 claimed 即返回，否则 None。"""
    entry = resolve_package_claims(events).get(package)
    if not entry:
        return None
    if effective_status(entry["claim"], entry["released"], at_dt) == "claimed":
        return entry["claim"]
    return None


def claim(package, claimant, ttl_minutes, at, claims_ledger):
    """领取登记：同包未过期再领即拒 PackageAlreadyClaimed，过验落 claimed 行。"""
    if not isinstance(claimant, str) or not claimant.strip():
        raise ClaimsError("claimant must be a non-empty string")
    claimant = claimant.strip()
    if not isinstance(ttl_minutes, int) or ttl_minutes <= 0:
        raise ClaimsError(f"ttl_minutes must be a positive integer, got {ttl_minutes!r}")
    at_dt = reference_time(at)
    claimed_at = at if at is not None else now_iso()
    expires_at = (at_dt + timedelta(minutes=ttl_minutes)).isoformat(timespec="seconds")
    events = load_claims(claims_ledger)
    existing = active_claim(events, package, at_dt)
    if existing is not None:
        raise PackageAlreadyClaimed(
            package, existing.get("claimant", ""), existing.get("expires_at", "")
        )
    line = {
        "claimant": claimant,
        "claimed_at": claimed_at,
        "event": "claimed",
        "expires_at": expires_at,
        "package": package,
        "status": "claimed",
        "tool": {"name": TOOL_NAME, "version": _version()},
        "ttl_minutes": ttl_minutes,
    }
    append_claim_event(claims_ledger, line)
    return {"line": line, "package": package}


def unclaim(package, claimant, at, claims_ledger):
    """领取释放：无在领即拦 no_active_claim，领取人不符即拦 not_claimant。"""
    events = load_claims(claims_ledger)
    entry = resolve_package_claims(events).get(package)
    at_dt = reference_time(at)
    status_now = (
        effective_status(entry["claim"], entry["released"], at_dt) if entry else None
    )
    if entry is None or status_now != "claimed":
        raise ClaimBlocked(
            "no_active_claim",
            {"package": package, "status": status_now or "unclaimed"},
        )
    holder = entry["claim"].get("claimant", "")
    if claimant is not None and claimant.strip() and claimant.strip() != holder:
        raise ClaimBlocked(
            "not_claimant",
            {"package": package, "holder": holder, "wanted": claimant.strip()},
        )
    line = {
        "claimant": holder,
        "event": "released",
        "package": package,
        "released_at": at if at is not None else now_iso(),
        "status": "released",
        "tool": {"name": TOOL_NAME, "version": _version()},
    }
    append_claim_event(claims_ledger, line)
    return {"line": line, "package": package}


def claims_view(claims_ledger, at=None, package_stem=None):
    """claims 视图：每包最新领取实效状态三值标注，在领与过期即标注可见。"""
    at_dt = reference_time(at)
    events = load_claims(claims_ledger)
    state = resolve_package_claims(events)
    rows = []
    for package in sorted(state):
        if package_stem is not None and package != package_stem:
            continue
        entry = state[package]
        claim_event = entry["claim"]
        rows.append(
            {
                "claimant": claim_event.get("claimant", ""),
                "claimed_at": claim_event.get("claimed_at", ""),
                "expires_at": claim_event.get("expires_at", ""),
                "package": package,
                "status": effective_status(claim_event, entry["released"], at_dt),
                "ttl_minutes": claim_event.get("ttl_minutes"),
            }
        )
    summary = {
        "claimed": sum(1 for row in rows if row["status"] == "claimed"),
        "expired": sum(1 for row in rows if row["status"] == "expired"),
        "packages": len(rows),
        "released": sum(1 for row in rows if row["status"] == "released"),
    }
    return {
        "claims": rows,
        "header": {
            "claims_ledger": str(claims_ledger),
            "tool": {"name": TOOL_NAME, "version": _version()},
        },
        "summary": summary,
    }


def claims_warning(claims_ledger, package, session_id, at=None):
    """闸一联动警示：同包有他人未过期领取即返回警示行清单，零在领即空表零噪声。

    他人即 claimant 异于本会话号；claimant 以会话预备号自报，与签发后
    会话号不同形即如实警示，开工人自认领者自辨。判定不拒即硬执法归闸一。
    """
    at_dt = reference_time(at)
    events = load_claims(claims_ledger)
    warnings = []
    for row_claim in [active_claim(events, package, at_dt)]:
        if row_claim is None:
            continue
        holder = row_claim.get("claimant", "")
        if session_id is not None and holder == session_id:
            continue
        warnings.append(
            {
                "claimant": holder,
                "claimed_at": row_claim.get("claimed_at", ""),
                "expires_at": row_claim.get("expires_at", ""),
                "package": package,
            }
        )
    return warnings
