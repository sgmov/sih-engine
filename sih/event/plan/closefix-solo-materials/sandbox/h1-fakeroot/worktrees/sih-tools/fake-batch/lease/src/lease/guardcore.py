"""直提守卫纯函数：提交信息形态判定单源，guardhook-solo 批承 adisp-guard-1 裁决与用户 2026-09-02 批准令。

只拦信息形态不判内容。合规形覆盖 lease 全部 message 形态：
settle 形即正文含 session: 十六位挂接行与 cert: 挂接行（build_message settle 形同一行挂接），
wip 形即首行含 wip 标记加 session: 挂接行（build_message wip 形无认证门故无 cert），
merge 形即首行 merge: <批名> 副本归并（close 归并形）。
拒类四值即批名前缀形无 session 行显式独立立类承 guardrail-solo 护栏三：
subject 首记号连字符批名模式（如 aea1768 同形即 mathpipe-a3-solo 冒头）且无 session 挂接行即拒，
这是对无 session 兜底的显式命名，非新增拦截即无 session 本就拒。
两形之外皆拒并出指引即走 lease commit 或 --no-verify 加 bypass 登记留痕。
本模块零依赖即钩子薄壳以 sys.path 直入 src 导入，禁依赖 venv。
"""

import re

GUARD_VERSION = "1.15.0"

SESSION_LINE = re.compile(r"^session: ([0-9a-f]{16})\b", re.MULTILINE)
CERT_LINE = re.compile(r"\bcert: ([0-9a-f]+)")
MERGE_LINE = re.compile(r"^merge: \S+ 副本归并$")
BATCH_PREFIX = re.compile(r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)+(?=[\s(\u3001\u3002\u4e00-\u9fff])")
# 直改车道放行（idenlane-solo iden-02）：提交信息携带直改链笔引用即
# 「直改链笔: <事件哈希前八位>」。本纯函数只认形态，真实在链由钩子层
# 对当日 trail grep 核验（不核引用即形同虚设，承 idenlane 任务包风险条）。
DIRECT_PEN = re.compile(r"\b直改链笔[：:]\s*([0-9a-f]{8})\b")

# 轻车道文件类白名单（idenlane-guard-solo T-9 承用户 2026-09-06 扩面令，冻结架构常量）：
# 直改车道合法提交的文件类枚举，起步宁窄勿宽即只收文书类。kind=path 即工作区根相对
# 路径前缀（目录带尾斜杠），kind=class 即处置类（untracked-disposal 即未跟踪件处置）。
# 源代码与引擎件不入选，仍走工地全仪式。扩面走 CONTRACT 修订，禁裸奔增删。
DIRECT_LANE_FILE_WHITELIST = (
    {"kind": "path", "pattern": "sih-tools/lease/ledger/"},
    {"kind": "path", "pattern": "sih-tools/lease/CALL-LOG.md"},
    {"kind": "path", "pattern": "sih-tools/scribe/CALL-LOG.md"},
    {"kind": "path", "pattern": "sih-tools/scribe/reports/"},
    {"kind": "path", "pattern": "sih-tools/identity/reports/"},
    {"kind": "path", "pattern": "sih-tools/tally/reports/"},
    {"kind": "path", "pattern": "sih-engine/sih/state/parking/materials/"},
    {"kind": "path", "pattern": "sih-engine/doc/governance/PARKING-v1.md"},
    {"kind": "path", "pattern": "sih-tools/PARKING-v1.md"},
    {"kind": "class", "pattern": "untracked-disposal"},
)

REASONS = ("no_session", "no_cert", "no_merge_hanger", "batch_prefix_no_session")

GUIDANCE = (
    "直提守卫拦截：提交信息不合 lease 模板形态。"
    "正规路径：lease commit --repo <仓> --session <会话号> --stage wip|settle --subject <事述>"
    "（settle 加 --seq 与 --cert）；"
    "显式绕行：git commit --no-verify 后 lease bypass --repo <仓> --sha <提交号> --reason <事由> 登记留痕。"
)


def validate_commit_message(text):
    """判 (ok, reason)。reason 为 ok 即合规，否则 REASONS 三值之一。

    合规三形即 settle 形与 wip 形与 merge 形，覆盖 lease 全部 message 形态；
    合规边界如实声明即形态模仿者不在此拦（持真实会话号手写 wip 形可通过），
    残余通道由 reconcile 对账兜底即 cert 缺席与段结算认证门。
    """
    lines = text.splitlines()
    first = lines[0].strip() if lines else ""
    # 直改车道放行首位判定：携带直改链笔引用即形态合规（ok_direct），
    # 真实在链由钩子层对当日 trail grep 核验，本纯函数不落文件读。
    if DIRECT_PEN.search(text):
        return True, "ok_direct"
    has_session = bool(SESSION_LINE.search(text))
    has_cert = bool(CERT_LINE.search(text))
    if has_session and has_cert:
        return True, "ok"
    if MERGE_LINE.match(first):
        return True, "ok"
    if first.startswith("merge:"):
        return False, "no_merge_hanger"
    if has_session:
        if " wip " in first:
            return True, "ok"
        return False, "no_cert"
    if BATCH_PREFIX.match(first):
        return False, "batch_prefix_no_session"
    return False, "no_session"
