# lease 收约执法面 Python 行为规格笔记（腿二 Rust 移植取材，簇 A）

基准：sih-tools/lease/src/lease/{commitcore.py, sddgate.py, guardcore.py, cli.py, core.py, ledgerwrite.py} + CONTRACT.md 修订四十九至六十二。所有报错文案为源码逐字节复制（含中文标点）。日期基准 2026-09-13。

---

## 0. 共享基础设施（core.py / ledgerwrite.py）

### 0.1 异常类与退出码映射（cli.py main 尾部 except 链，序即匹配序）

| 异常 | 输出载荷 | 退出码 |
|---|---|---|
| `LockBlocked` | `{"detail": exc.detail, "error": exc.reason}` | 1 |
| `PackageSessionActive` | `{"error": str(exc), "reason": "PackageSessionActive", "detail": {"package": exc.package, "session_id": exc.session_id}}` | 1 |
| `PackageAlreadyClaimed` | `{"error": str(exc), "reason": "PackageAlreadyClaimed", "detail": {"claimant","expires_at","package"}}` | 1 |
| `ClaimBlocked` | `{"detail": exc.detail, "error": exc.reason, "reason": exc.reason}` | 1 |
| `ClaimsError` | `_emit_error(str(exc), 2)` → `{"error": str(exc)}` | 2 |
| `CommitBlocked` | `{"detail": exc.detail, "error": exc.reason}` | 1 |
| `LocksError` | `{"error": str(exc)}` | 2 |
| `StateError` | `_emit_error(str(exc), 1)` | 1 |
| `WorktreeError`（含子类 `StemGateToolError`） | `_emit_error(str(exc), 2)` | 2 |

注意：`CommitBlocked(ValueError)` 定义于 commitcore.py：`__init__(reason, detail=None)`，`str(exc)` 即 reason 码（如 `repo_not_in_session`），error 键取 reason、detail 键取 detail dict（可为 None → JSON null）。

### 0.2 JSON 输出形（cli.py `_emit`）

- 正常：`print(json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2))`（print 尾随 `\n`）。键序 = 字典键的 sort_keys 排序（ASCII 序）。
- `_emit_error(message, code=2, extra=None)`：`{"error": message}` + extra.update。
- `--quiet`：单行 `json.dumps(line, ensure_ascii=False, sort_keys=True, separators=(",", ":"))`，line = `{tool:"lease", command:_LAST_COMMAND, code, summary, session_id}`；summary 取 payload["summary"]，否则依序找 `("error","status","event","digest","verdict")` 第一命中包成单键 dict。
- dogfood 留痕（`SIHANKOR_CALLLOG_DOGFOOD=1` 才开）：失败仅 stderr 一行 `f"dogfood_calllog: {exc}\n"` 不阻断；工地 CWD（路径 split 后含 `worktrees` 段）缺显式根时 stderr 一行 `"跨根卫拒：工地 dogfood 禁直写真根\n"`。Rust 移植可判为非正典面（env 门控）但为对等须保留 env 语义。

### 0.3 台账写点（ledgerwrite.append_row —— 唯一写点）

- 行字节 = `json.dumps(event, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode("utf-8") + b"\n"`（紧凑形、键排序、单行、尾换行）。
- 写法：先对 `path + ".lock"` 取 flock（O_RDWR|O_CREAT 0o644），再 `os.open(path, O_WRONLY|O_CREAT|O_APPEND, 0o644)` 循环 write 整行。同线程同路径可重入（计数表 `_LOCK_HELD` 键 = (线程 id, lock 路径 abs)）。fcntl 缺席 raise `OSError("ledger flock 需 POSIX fcntl")`。
- 上层 `core.append_event` 捕 OSError → `WorktreeError(f"ledger unwritable: {ledger}")`。

### 0.4 读侧与路径解析（core.py）

- `load_ledger(path)`：缺席返 `[]`；逐行 `json.loads`，坏行 → `WorktreeError(f"ledger line {number} invalid: {exc}")`（number 从 1 起，空行跳过）；读败 → `WorktreeError(f"ledger unreadable: {ledger}")`。
- `now_iso()` = `datetime.now(timezone.utc).isoformat(timespec="seconds")` → 形如 `2026-09-13T08:15:09+00:00`。
- `core._git(repo, *args)`：`["git","-C",repo,*args]`，timeout=30；FileNotFoundError → `WorktreeError("git not available")`；SubprocessError → `WorktreeError(f"git invocation failed: {exc}")`。sddgate.py 与 guardcore.py 各有**自己的** `_git`（sddgate 版无 timeout；guardcore 的 `_central_ledger_via_common_dir` 用 timeout=10）——移植时注意三处 git 出口参数不同。
- `resolve_root(explicit)`：显式优先；否则从 `Path(__file__).resolve()` 及其 parents 上溯找同时含 `sih-engine/Cargo.toml` 与 `sih-tools/pyproject.toml` 的目录；回落 `here.parents[4]`。
- `tool_dir()` = `Path(__file__).resolve().parents[2]`（即 `.../sih-tools/lease`）。
- `detect_domain_context(explicit_root)`：显式 root 优先否则 cwd 上溯；每候选先检 canonical（`path/sih/ledger` 为目录 → `(cur,"canonical")`），再检双仓标记 → `(cur,"first_domain")`；零命中 `(None,"none")`。
- CLI 缺省台账：`--ledger`/`--locks` 显式优先；canonical 域 → `<域根>/sih/ledger/{sessions,locks}.ndjson`；否则 `tool_dir()/ledger/{sessions,locks}.ndjson`。
- `_default_trails(root)`（cli.py）：三居所并集 `<root>/sih-engine/sih/event/trail`、`<root>/sih-tools/scribe/trail`、`<root>/sih/event/trail`，每目录 `sorted(glob("*.ndjson"))`，按 resolved 绝对路径去重，最终 `sorted(out)`（字符串排序）。

### 0.5 退出码三值总纲（CONTRACT）

0=成；1=拦（五验失败/锁冲突/脏上游/收约三检失败/commit 四验拦/SDDG 拒/闸拦）；2=工具异常（文件非法/git 不在/多在册无参/用法错 argparse 自身 exit 2）。

---

## 1. commitcore.py —— commit 四验与 reconcile 对表

### 1.1 冻结常量

```python
MESSAGE_REASONS = ("session_not_active", "repo_not_in_session", "nothing_staged", "staged_out_of_scope", "cert_not_on_chain")
SESSION_LINE = re.compile(r"^session: ([0-9a-f]{16})\b", re.MULTILINE)
CERT_LINE = re.compile(r"\bcert: ([0-9a-f]+)")
SEAL_BASES = {"sih-engine": "4146851", "sih-tools": "91d14d4f"}
BYPASS_LEDGER_NAME = "bypass.ndjson"
HOOKS_DIR = Path(__file__).resolve().parents[2] / "hooks"
```
`SEAL_EXEMPTS`：sih-engine 3 个全 sha；sih-tools 17 个全 sha（源码 47–72 行逐字节照抄移植，sha 清单是冻结数据面）。

### 1.2 调用图

- `lease commit`（cli）→ settle 且 `--seq is None` → `_emit_error("settle requires --seq", code=2)`（payload 恰为 `{"error": "settle requires --seq"}`，**退出码 2 非人工拦截 1**）→ `commit_staged(...)` → settle 时回执附 `report["gauge"] = gauge_summary(root)` → `_emit(report, 0)`。
- `lease reconcile` → `resolve_default_base` → `reconcile(...)` → 退出码：`0 if summary.unrouted==0 and summary.session_orphan==0 and summary.cert_missing==0 and summary.unbypassed==0 else 1`。seal 命中时回执附 `report["base_source"] = {"kind": "seal_line", "boundary": seal, "range_base": base}`。
- `lease bypass` → `record_bypass(...)` → `_emit({"bypassed": line, "ledger": str(ledger_path)}, 0)`。
- `lease install-hooks`/`uninstall-hooks` → install 全 `installed` 真 → 0 否则 1；uninstall 全 `uninstalled` 真 → 0 否则 1；回执 `{"hooks_dir": str(HOOKS_DIR), "repos": results}`。缺省双仓 `[root/"sih-tools", root/"sih-engine"]`。

### 1.3 commit_staged 四验（顺序、失败即停）

签名：`commit_staged(repo, session_id, stage, subject, seq, cert, note, root, ledger, trails)`。

1. **会话在册**：`load_ledger(ledger)` + `select_session(events, session_id)`（lockcore；session 缺省唯一在册，多在册无参 → LocksError/StateError 族走 CLI 尾 except）。`_resolve_repo_entry`：先 `entry["repo"] == str(repo_path)` 精确匹配，再 `entry["worktree"] == str(repo_path)` 副本别名；都不中 → `CommitBlocked("repo_not_in_session", {"repo": str(repo_path), "repos": [...各entry repo...], "worktrees": [...各entry worktree...]})`。
2. **必须指副本**：`entry.get("worktree") != str(repo_path)` → `CommitBlocked("commit_must_target_worktree", {"repo": str(repo_path), "worktree": entry.get("worktree")})`。
3. **暂存非空**：`_git(repo, "-c", "core.quotepath=off", "diff", "--cached", "--name-only")`；rc≠0 → `WorktreeError(f"git diff failed: {stderr.strip()}")`；空 → `CommitBlocked("nothing_staged")`（detail None）。
4. **范围验**：`repo_rel = Path(entry["repo"]).relative_to(Path(root).resolve())`，ValueError → `WorktreeError(f"repo outside workspace root: {entry['repo']}")`。`rel_prefix = "" if str(repo_rel)=="." else f"{repo_rel}/"`；逐 staged 件 `checked = f"{rel_prefix}{name}"` 交 `scope_allows(session["allow"], checked)`；越界集非空 → `CommitBlocked("staged_out_of_scope", {"outside": outside, "allow": session.get("allow", [])})`。
5. **认证在链（仅 settle）**：`cert` 缺 → `WorktreeError("settle requires --cert")`（退出码 2）。`_load_trail_hashes(trails)` 收集全部 trail 中 `event_type=="certification_completed"` 的 `event_hash`（str 非空）；`_cert_on_chain(cert, hashes)` = 任一 hash `startswith(cert)`（八位短形或全值皆可）。不中 → `CommitBlocked("cert_not_on_chain", {"cert": cert, "trails": [str(t) for t in trails]})`。
6. **提交**：message 由 `build_message` 生成（见下），`subprocess.run(["git","-C",repo,"commit","-m",message], capture_output=True, text=True, timeout=30)`——**不加 hooksPath 禁用**（与 close 预收提交不同）；rc≠0 → `StateError(f"git commit failed: {proc.stderr.strip()[:200]}")`。随后 `rev-parse --short HEAD` 取短号。

### 1.4 build_message 三形态（模板单源，逐字节）

先 `_dedup_subject_prefix(stem, subject)`：循环剥前缀至稳定，剥序 `^{stem} wip\s+` → `^{stem} 段\d+\s+` → `^{stem}\s+`（re.escape(stem)；stem 空或 subject 空原样返回；`{stem}` 非整词前缀如 pkgx 不误剥因 pattern 以 `\s`/`wip`/`段` 收尾）。

- wip：`"{stem} wip {subject}"`, `""`, `"session: {session_id}"`，note 给定时再追加 `"", note`；`"\n".join(parts) + "\n"`。无 note 时字节零变（修订六十一）。
- settle：head = `"{stem} 段{seq} {subject}"`；meta 行 = `"session: {session_id} cert: {cert} base: {base}"`（**同一行三挂接**）；note 同 wip。seq/cert/base 可能 None → Python f-string 出 `None` 字面（settle 时 CLI 已强制 seq，cert 已强制）。
- 其他 stage → `WorktreeError(f"unknown stage: {stage}")`。

`base` 取自 `_base_label(entry, repo_path)`：副本（entry.worktree==repo_path）→ `git merge-base HEAD base_branch` + `rev-parse --short` → `"{base_branch}@{short}"`，任一步 rc≠0 返 `""`；主检出 → `"{branch}@{HEAD_short}"`。

### 1.5 commit 回执 JSON（键序经 sort_keys）

```json
{"base": ..., "checks": [...], "commit": "<短sha或空>", "gauge": {...settle时}, "message": "<全文>", "package": ..., "session_id": ..., "stage": ...}
```
`checks` = `["session_active","staged_in_scope"] + (["cert_on_chain"] if settle else [])`。

### 1.6 reconcile 分类器（git log 对 台账 对 trail 对 bypass 台账）

- `git log --format=%H%x1f%ad%x1f%B%x1e --date=short [f"{base}..HEAD"] [-n limit]`；rc≠0 → `WorktreeError(f"git log failed: {stderr.strip()}")`。逐记录 split `\x1e` 再 split `\x1f`(maxsplit 2)，三段不足即跳过；sha/date/body strip；subject = body 首行。
- 已知集：`_known_sessions(ledger)` 取 issued 行的 session_id 集与 package 集。
- 分类判序（首中即停）：
  1. `sha in SEAL_EXEMPTS[仓名]` → `sealed`（`seal_exempt_set` 按路径组成件倒序首个命中仓键；表外仓空集）。
  2. `SESSION_LINE` 命中：sid 不在 session_ids → `session_orphan`(detail=sid)；cert 行命中且 `_cert_on_chain` 不中 → 补录绑定 `_backfill_binding`：`"cert_prefix"` → `cert_backfilled`(detail=cert)；`_bypass_hit` → `cert_writtenoff`(detail=cert)；`"binding_mismatch"` → `cert_missing`(detail=f"{cert} binding_mismatch")；无绑定 → `cert_missing`(detail=cert)。cert 在链或无 cert 行 → `routed`。
  3. `subject.startswith("merge: ")` 且 `subject == f"merge: {stem} 副本归并"`（stem ∈ packages）→ `routed_merge`。
  4. `_direct_pen_on_chain(body, all_event_hashes)` 非 None → `routed_direct`(detail=命中前缀)。`DIRECT_PEN`（guardcore 单源）= `\b直改链笔[：:]\s*([0-9a-f]{8})\b`；在链 = 前缀 `startswith` 任一**任意事件**的 event_hash（`_load_all_event_hashes`，非认证专属）。
  5. `_bypass_hit(bypass_entries, repo, sha)` → `bypass`。
  6. 否则 `unbypassed`。
- `_bypass_hit`：entry repo resolve 等值（OSError 原样字符串）+ sha **前缀互含**（`sha_full.startswith(entry_sha) or entry_sha.startswith(sha_full)`，短号全号皆命中）。
- 补录绑定 `_load_backfill_bindings(trails)`：certification_completed 且 `(details or {}).get("content_hashes")` 为 dict 且 `ch.get("backfill")` 真值 → `{"original_event_hash": str(ch.get("original_event_hash") or ""), "referencing_settle_commits": ch.get("referencing_settle_commits") or []}`。`_backfill_binding`：cert 前缀逐字节命中任一 original_event_hash → `"cert_prefix"`；否则任一 binding 的 referencing_settle_commits 存在 `ref["sha"] == sha_full` 且 `_repo_name_hit(ref["repo"], repo)` → `"binding_mismatch"`（绑定不符仍 missing 加告警）。`_repo_name_hit`：entry strip 去 `/`；等于 repo resolved 字符串，或 entry 是 repo resolved 路径组成件，或 `Path(entry).resolve()` 等值（OSError → False）。
- 记录行键：`{"class","date","detail","sha"([:7]),"subject"([:80])}`。
- summary 键（sort_keys 序）：`bypass, cert_backfilled, cert_missing, cert_writtenoff, routed, routed_direct, sealed, session_orphan, total, unbypassed, unrouted`。**quirk：`unrouted` 恒 0**（sum 条件 class=="unrouted" 无此分类，兼容保留）。`routed` 计数含 routed/routed_merge/routed_direct 三类。
- 返回：`{"base": base or "", "first_routed": routed_shas[-1] 或 None, "repo": str(Path(repo).resolve()), "summary": summary, "unrouted_tail": [非 routed 非 sealed 记录][:200]}`。注意 `routed_direct` 与 `routed_merge` **入 unrouted_tail**（可见性非告警）但不影响退出码。
- `resolve_default_base(repo, base=None)`：base 显式给 → `(base, None)`；否则按仓路径组成件命中 SEAL_BASES → `git cat-file -e seal` rc=0 时返 `(f"{seal}^", seal)`，否则 `(None, None)`（全史显式回落）。

### 1.7 bypass 台账（bypass 子命令写行形）

`record_bypass(repo, sha, reason, session, at, ledger_path)` 行形（经 append_event → flock + sort_keys 紧凑）：
```json
{"at": "<at或now_iso>", "event": "bypassed", "reason": ..., "repo": "<Path(repo).resolve()字符串>", "session": ..., "sha": ..., "tool": {"name": "lease", "version": "<lease.__version__>"}}
```
`load_bypass_entries` = load_ledger 过滤 `event=="bypassed"`。

### 1.8 install/uninstall hooks 行形

install 结果项 `{"hooks_path": str(HOOKS_DIR), "installed": rc==0, "repo": <resolved>, "detail": None 或 stderr[:200]}`；uninstall 结果项 `{"repo": ..., "uninstalled": rc in (0,5), "state": "unset"|"already_absent"|"failed", "detail": ...}`（rc=5 = config 键缺席）。

---

## 2. sddgate.py —— SDDG 四判据（DEC-024 B 层，close 链闸）

### 2.1 冻结常量（逐字节移植）

```python
GATE_SINCE = "2026-09-12"
SDDG_VERSION = "1.0.0"
SPEC_PATH_RE = re.compile(r"(doc/decision/|doc/design/|doc/spec/|task-packages/|sih/state/plan/)|((^|/)(DEC|DES|SPEC)-[0-9][0-9A-Za-z-]*\.(md|adoc|json|txt)\b)", re.IGNORECASE)
INTENT_SPEC_DECL_RE = re.compile(r"规格|消费面|正典|SPEC|DEC|DES")   # 大小写敏感
CANON_POINTER_RE = re.compile(r"SPEC|DES|DEC|正典")
DEVIATION_CARRIER_RE = re.compile(r"pk-\d+|DEC-\d+|DES-\d+|SPEC-\d+|候批")
ZERO_DEVIATION_RE = re.compile(r"零偏差")
TEST_FILE_RE = re.compile(r"(^|/)tests?(/|$)|(^|/)test_[^/]*\.py$|_test\.py$")
CROSS_REPO_TEST_DECL_RE = re.compile(r"测试件实体在工具仓|跨仓测试承载申报|test files carried cross-repo")
ACCEPTANCE_RE = re.compile(r"acceptance|results|verdict", re.IGNORECASE)
RESULTS_DOC_RE = re.compile(r"results\.md$", re.IGNORECASE)
SOURCE_EXTS = {".py",".rs",".js",".ts",".jsx",".tsx",".sh",".go",".c",".h",".cpp",".hpp",".java",".rb",".sql"}
```
`CRITERIA_COMMANDS`（四判据判定命令教学载荷）与 `CRITERIA_COUNTEREXAMPLES`（四反例）为**单源照录 DEC-024 的中文字符串常量**——Rust 须逐字节复制（源码 57–83 行），reject teaching 直接引用。

### 2.2 run_gate 入口与跳过态

`run_gate(session, trails, repos)`，repos 元素 `{"repo","base","branch"}`：

1. `sddgate_applicable(session)`：`ts = session.get("issued_at") or session.get("opened_at") or ""`；`datetime.fromisoformat(str(ts)).date() >= date.fromisoformat("2026-09-12")`；**ValueError（缺/不可解析）→ True（fail-closed 受检）**。不适用 → `{"checked": False, "verdict": "skip", "reason": "before_gate_since", "gate_since": GATE_SINCE}`。
2. trails 过滤 `Path(t).is_file()`；空 → `{"checked": False, "verdict": "skip", "reason": "workspace_chainless"}`。
3. `session_trail_events(sid, files)`：逐 trail 逐行 json.loads（坏行/读败/缺席静默跳），过滤 `ev.get("session_id") == session_id`（**字段全等**）。
4. 逐 repo：base/branch 缺或相等 → 跳；否则 `branch_diff_files`（`git diff --name-status {base}...{branch}`，rc≠0 → 空单；逐行 split `\t` maxsplit 1，status 取首字符，rename 形 ` -> ` 取右半）与 `first_impl_commit`（`rev-list --reverse {base}..{branch}` 首个 sha；`show -s --format=%cI` 取 committed_at（取 stdout 非空行最后一行 strip）；`diff-tree --no-commit-id --name-only -r --root` 取件单；无提交 → None）。→ `repo_diffs`、`impl_commits`。
5. `diff_paths` = 全部 diff path 平铺；`results_docs` = diff 文件中 `RESULTS_DOC_RE`（`results\.md$`）命中者，text = `_blob_text(repo, branch, path)`（`git show {branch}:{path}`，rc≠0 → 空串）。
6. 四判据各一函数；verdict=reject 集合非空 → 总判词 reject + teaching。

### 2.3 SDDG-1 典在码前（check_sddg1）

- t0 = 所有 `intent_refined` 事件 timestamp 归一（`_parse_ts`：`replace("Z","+00:00")` → fromisoformat，naive 补 UTC；不可解析 None）后的**最小**者（原串入 t0 键）。
- t1 = impl_commits 中 committed_at 最小者。逐 impl commit 的 files 中 `SPEC_PATH_RE` 命中 → `first_impl_spec_hits`（`{"repo","path"}`）。
- window hits：session_events 中时戳落 `[t0, t1]` 闭区间的事件，`json.dumps(ev, ensure_ascii=False)` 全文匹配 `SPEC_PATH_RE` → 收 timestamp。
- declared：`intent_declares_spec`——`intent_refined` 事件的 `(details or {}).get("record_path")` 非空则读该文件文本（OSError 跳过），`INTENT_SPEC_DECL_RE.search` 命中即 True。
- pass = `spec_hits or window_hits or declared`。输出键：`gate, verdict, t0_intent, t1_first_impl, first_impl_spec_hits, window_spec_hits, intent_spec_declared`。

### 2.4 SDDG-2 规格束齐备（check_sddg2）

逐 repo_diffs 逐文件：`status == "A"`（新增）才受检；`TEST_FILE_RE` 命中跳过；`Path(path).suffix not in SOURCE_EXTS` 跳过；取 `git show {branch}:{path}` 前 20 行（`_blob_text(limit=20)`，splitlines 后 join——**末行无换行**），`CANON_POINTER_RE.search` 零命中 → 入 `zero_pointer_hits`（`{"repo": str(repo), "path": path}`）。verdict = pass iff zero_hits 空。输出键：`gate, verdict, checked_new_source_files, zero_pointer_hits`。

### 2.5 SDDG-3 偏差有承载（check_sddg3）

- diff 空 → `{"gate","verdict":"pass","reason":"diff_empty","uncarried_entries":[]}`。
- results_docs 空 → reject，`reason: "results_doc_absent"`。
- `_deviation_entries(text)`：逐行；标题行 = strip 后 `startswith("#") and "偏差" in s` **或** `s.startswith("偏差")`，此后至下一 `#` 开头行止；条目 = 非空行 `lstrip("-*").strip()`。无标题行 → None。
- 全文档 `ZERO_DEVIATION_RE`（零偏差）命中 → declared_zero。
- 逐条目 `DEVIATION_CARRIER_RE`（`pk-\d+|DEC-\d+|DES-\d+|SPEC-\d+|候批`）零命中 → `uncarried_entries` 加 `{"doc": doc["path"], "entry": e}`。
- uncarried 非空 → reject，`reason: "deviation_entry_uncarried"`。
- 有偏差节（has_section）或 declared_zero → pass，输出含 `deviation_section`（bool）与 `zero_deviation_declared`（bool）。
- 否则 → reject，`reason: "deviation_section_absent_and_no_zero_declaration"`。

### 2.6 SDDG-4 测试同批（check_sddg4）

- diff_paths 空 → pass，`reason: "diff_empty"`，`test_files/acceptance_files` 空。
- `test_files = sorted({p for p in diff_paths if TEST_FILE_RE.search(p)})`；`acceptance_files = sorted({p ... ACCEPTANCE_RE.search(p)})`。
- 跨仓通道：`CROSS_REPO_TEST_DECL_RE.search(d["text"])` 任一 results doc 命中 → cross_repo_declared。
- pass = `(test_files and acceptance_files) or (cross_repo_declared and acceptance_files)`。
- cross declared 且 test_files 空时 payload 附 `"reason": "cross_repo_test_declaration"`。

### 2.7 总出口与 reject teaching

```json
{"checked": true, "verdict": "pass|reject", "gates": {"SDDG-1":..., "SDDG-2":..., "SDDG-3":..., "SDDG-4":...}, "gate_since": "2026-09-12", "sddg_version": "1.0.0"}
```
reject 时附 `teaching`：`{"reason_code": "sddgate_rejected", "gate": [被拒判据名列表], "gate_verdicts": gates(全量), "criteria_commands": {被拒者}, "counterexamples": {被拒者}, "channels": "三通道：先立典（规格物落笔）、补申报（意图或结果档偏差承载）、显式绕行 close --bypass-sddgate <事由> 落 bypass 台账留痕"}`。

### 2.8 close 接线（core.py close_session 内，链证守门后、CALL-LOG 跑步机前）

- 门模块自身任何异常 → `raise WorktreeError(f"SDDG 门工具异常（fail-closed）：{exc}")` → 退出码 2。
- verdict=="reject" 且 `--bypass-sddgate <事由>` 给定：写 bypass 台账（**注意：此处不是 append_event/append_row，是裸 `open(path,"a")` 直写**，键序非 sort_keys）：
```json
{"at": "<now_iso()，不受 --at 冻结>", "event": "bypassed_sddgate", "reason": "<事由>", "package": "<stem>", "session": "<session_id>", "gates": ["<teaching.gate 列表>"]}
```
  写位 = `tool_dir()/ledger/bypass.ndjson`。报告态 `sddgate_gate = {**_sdd_verdict, "verdict": "bypass", "reason": bypass_sddgate}`。
- reject 无 bypass → `StateError("SDDG 门拦截：SDD/TDD 完备度判据未过，整批拒零动作\n（DEC-024 B 层逐批执法，判定命令形机械执法零 LLM 判词位）。\n" + json.dumps(_sdd_verdict.get("teaching", _sdd_verdict), ensure_ascii=False, indent=2))`（退出码 1）。
- pass/skip → `sddgate_gate = _sdd_verdict` 原样入 revoked 行 detail 与 close 回执。

---

## 3. guardcore.py —— commit-msg 守卫与锁面投影（钩子用）

GUARD_VERSION = "1.17.0"。模块零 venv 依赖（钩子 sys.path 直入 src）。

### 3.1 正则（逐字节）

```python
SESSION_LINE = re.compile(r"^session: ([0-9a-f]{16})\b", re.MULTILINE)
CERT_LINE = re.compile(r"\bcert: ([0-9a-f]+)")
MERGE_LINE = re.compile(r"^merge: \S+ 副本归并$")
BATCH_PREFIX = re.compile(r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)+(?=[\s(\u3001\u3002\u4e00-\u9fff])")
DIRECT_PEN = re.compile(r"\b直改链笔[：:]\s*([0-9a-f]{8})\b")   # 全半角冒号同形
REASONS = ("no_session", "no_cert", "no_merge_hanger", "batch_prefix_no_session")
```

### 3.2 validate_commit_message(text) 判序

1. `DIRECT_PEN.search(text)` → `(True, "ok_direct")`（首位判定，真实在链由钩子层 grep 当日 trail 核验，纯函数不读文件）。
2. has_session 且 has_cert → `(True, "ok")`（settle 形）。
3. `MERGE_LINE.match(first)` → `(True, "ok")`（merge 形，首行精确）。
4. `first.startswith("merge:")` → `(False, "no_merge_hanger")`。
5. has_session：`" wip " in first` → `(True, "ok")`（wip 形）；否则 `(False, "no_cert")`。
6. `BATCH_PREFIX.match(first)` → `(False, "batch_prefix_no_session")`（首记号连字符批名形且无 session 行）。
7. 否则 `(False, "no_session")`。

### 3.3 staged_enforce（pre-commit 文件面）

- `allow_no_verify=True` → 立即 `{"ok": True, "rejected": [], "guidance": "", "allow_no_verify": True}`。
- 逐件 `_staged_covered`：① 活跃锁面（`ws_path == lp or ws_path.startswith(lp.rstrip("/") + "/")`）→ ② `SCOPE_SHARED_SURFACE` startswith（ImportError 静默跳过）→ ③ `DIRECT_LANE_FILE_WHITELIST`（kind=path → startswith；kind=class 且 untracked-disposal → 本函数跳过，留 commit-msg 位）。
- 返回 `{"ok": len(rejected)==0, "rejected": [...], "guidance": STAGED_REJECT_GUIDANCE, "allow_no_verify": False}`。
- `STAGED_REJECT_GUIDANCE` 与 `GUIDANCE`（commit-msg 拦指引）两中文字符串常量逐字节照抄（源码 87–95、154–159 行，含全角冒号与换行）。

### 3.4 锁账本解析与投影（钩子装配位）

- `resolve_locks_ledger(explicit, root_dir, explicit_db, cwd)` 优先序：explicit 原样 → `<root_dir>/sih-tools/lease/ledger/locks.ndjson` 在盘 → git common dir 锚定（`git -C cwd rev-parse --git-common-dir` timeout=10，相对形按 cwd 归一；`<common 的父目录>/lease/ledger/locks.ndjson` 在盘）→ explicit_db → `<root_dir>/.../locks.db` → 相对缺省 `Path("sih-tools/lease/ledger/locks.ndjson")`。
- `parse_active_locks(path)`：非 .ndjson 后缀或不存在 → `[]`；逐行 json.loads（坏行跳）；`(path, session_id)` 键 acquired 置 `mode or "exclusive"`、released 弹；输出 `[{"path","mode"}]`（无路径行不入）。

---

## 4. close_session（core.py）——收约闸序全序（含 closeguard 家族）

闸序（顺序即语义，失败即停整批零动作）：

1. **会话定位**：`resolve_package(package_value, root, must_exist=False)`；active_sessions 按 package 过滤；零 → `StateError(f"no active session for package: {stem}")`；多 → `StateError(f"ambiguous active sessions for package: {stem}")`。
2. **锁清零**：`active_locks(load_lock_events(locks_ledger, "locks ledger"), root)` 中本会话持锁路径排序非空 → `StateError(f"locks held by session, unlock first: {sid_held}")`（sid_held 为 sorted 路径 list 的 repr）。
3. **工地卫生**（逐仓 worktree）：`check_worktree_clean` —— `git status --porcelain`，非 `??` 行 → issue `{"type":"modified","path":...,"action":"git add <path> && git commit"}`；`??` 行 → `{"type":"untracked","path":...,"action":"git add <path> && git commit or remove file"}`。非空 → `StateError("工地卫生检查失败：存在未提交修改或未跟踪文件，提交归代理责任，工具不代提交。" + json.dumps({"issues": issues}, ensure_ascii=False))`。worktree 缺席 → clean。
4. **前置机械对表三态**：逐仓 `detect_merge_conflicts`（三点式 `_merge_files` = merge-base..branch diff --name-only，交真实脏位与未跟踪命中面；未跟踪命中含目录前缀命中 `_hits_untracked`；净态差集件退场）+ `is_pure_append_conflict`（same：ondisk==branch 字节；append：仅 `.ndjson` 且 ondisk 以 base 行集为严格前缀且追加行不在 branch 集，附 live_only；diverged：其余；未跟踪碰撞同内容= same+untracked、异内容=diverged）。diverged → `StateError("收约被阻：存在真分叉冲突（非纯追加形），整批拒零动作。" + json.dumps({"repo":..., "diverged_files":[...]}, ensure_ascii=False))`。append/same 走 `backup_conflict_files`（备份至 `<worktrees根>/.close-backups/<stem>/<session_id>/<仓名>/`）+ `allow_and_merge`（台账追加面 `LEASE_LEDGER_APPEND_FACES` 四面走 `_union_append_face` 并集追加；其余 `git checkout <branch> -- <f>`；same 未跟踪件先 unlink）+ `generate_re_certify_hashes`（live_only 行对合并件差集，sha256(line)[:16]）。
5. **预收提交最小化**：三点归并面 ∩ 真脏位（排除 `??`），`git add` + `git -c core.hooksPath=/dev/null commit -m f"closeguard-solo: pre-close working tree commit ({stem})"`。
6. **前置态探针**：`_close_precondition`（MERGE_HEAD/rebase-merge/rebase-apply/CHERRY_PICK_HEAD/REVERT_HEAD/BISECT_LOG 标记 + 归并面真分叉件 `_merge_file_diverged`，ndjson 纯追加增长豁免）非空 → `StateError("收约前置态被阻：目标仓合并态或共享面脏即整批拒零部分动作，详情 " + json.dumps(blockers, ensure_ascii=False))`。
7. **merge-tree 预检**：`git merge-tree --write-tree base branch` 逐仓，非零解析 `CONFLICT ... Merge conflict in <file>` 行 → `StateError("收约预检被阻（merge-tree 本场归并有内容冲突即整批拒零动作半程不可达被堵）：详情 " + json.dumps(mt_precheck, ensure_ascii=False))`。支已删（rev-parse 失败）跳过。
8. **链证守门**：trails 过滤在盘文件；有文件则 `chain_gate_check(session_id, files)`——须有 `intent_refined` 与 `certification_completed` 两笔（session_id 全等）；缺 → `StateError("链证守门拦截：本会话正典链缺笔，补链笔后收约" + json.dumps({"session_id":..., "missing": [...]}, ensure_ascii=False))`；零链文件 → `_chain_gate_note = {"checked": False, "reason": "workspace_chainless"}` 空转放行。
9. **SDDG 门**（见 §2.8）。
10. **CALL-LOG 跑步机收编**：`calllog_treadmill_applicable`（issued_at **严格晚于** `CALLLOG_TREADMILL_EFFECTIVE_AT="2026-09-08T08:00:00+00:00"`；不可解析 → True fail-closed）；逐仓 `calllog_treadmill_collect`：纯追加（untracked 视为 born 追加；modified 需 numstat deletes==0 且 adds>0）脏收编面（`calllog_treadmill_faces(repo)` 单源取 guardcore 白清单 CALL-LOG.md/calls.ndjson 条目，传 repo 时补仓相对形）→ `git add` + `git -c core.hooksPath=/dev/null commit -m f"lease projection-collect: CALL-LOG treadmill appends ({stem})"`；非纯追加不碰。
11. **无主闸**：trails 过滤在盘，非空才检（否则空转零注记仅由回执形态体现）。`no_master_check(root, trails[0], locks_ledger)`（**只取首个在盘 trail 文件**）——watchcheck `unowned_list` 不可导入 → `StateError("无主闸谓词不可用：watchcheck 不可导入（%s）。fail-closed 拦截。\n指引：修复 watchcheck 安装与 sys.path 后重试，或 lease --bypass-orphan <事由> 显式绕行。\n" % exc)`；trail 缺席（理论不可达，前面已过滤）→ `StateError("无主闸前置条件：当日 trail 缺席（挂点序保证 trail 先在）：%s\n指引：先 scribe record 落链或 pen 写当日 trail 后重试。\n" % trail)`。unowned 非零：`--bypass-orphan <事由>` 给定 → 裸 open 直写 bypass.ndjson 一行 `{"at": now_iso(), "event": "bypassed_orphan", "reason": 事由, "package": stem, "session": session_id, "unowned": 清单}`；否则 `StateError("无主闸拦截：主树存在无主 tracked 改件（脏文件集 − 锁面 ∪ 链笔声明面 ∪ 豁免面 = 无主清单，共 N 件），整批拒零动作。\n三通道：\n  1) 走司衡治理通道开租约持锁后再改（lease open + lease lock --path <路径>）\n  2) 走直改链笔申报（scribe pen 落 direct_edit_completed 声明）\n  3) 显式绕行：close --bypass-orphan <事由> 落 bypass.ndjson 留痕\n清单：" + json.dumps(unowned, ensure_ascii=False, indent=2))`。
12. **CALL-LOG 随批检查**：`calllog_guard_applicable`（issued_at **严格晚于** `CALLLOG_GUARD_EFFECTIVE_AT="2026-09-08T00:01:35+00:00"`；不可解析 → True）；`calllog_face_check` 逐仓扫 `CALL-LOG.md` 脏面（`git status --porcelain -uall`），released = 盘上字节 == `git show {branch}:{path}`（released_by=settle）；排除已收编面（跑步机 collected 交集）后 unreleased 非空：bypass 给定 → 裸写 `{"at": now_iso(), "event": "bypassed_calllog", "reason":..., "package":..., "session":..., "faces":[...]}`，calllog_gate = `{"checked": True, "released_by": "bypass", "unreleased": faces}`；否则 `StateError(calllog_gate_message(faces))`（报文常量见 core.py 465–479 行，含 `（共 N 件）` 与三通道指引）。净态 calllog_gate = `{"checked": True, "unreleased": 0}`。
13. **差集闸**：`declared_uncommitted_diff(package_path, session, ack_map)`——声明面 = 任务包 `## 请求写入` 节逐列表项（`parse_declared_writes`：`- `/`* ` 起首、strip、剥反引号；`conditional = "若" in item`（DECLARATION_CONDITIONAL_MARK）；`paths = DECLARATION_PATH_TOKEN.findall(item)`，正则 `(?:[A-Za-z0-9_.\-]+/)+[A-Za-z0-9_.\-]*`）。逐 path token：条件行入 `exempt_conditional`；无仓前缀命中（`<仓名>/` 前缀，仓名 = entry repo 的 Path.name）入 `unparsed`；命中 SCOPE_SHARED_SURFACE 入 `shared_surface_exempt`；`git ls-tree -r --name-only <branch> -- <rel>` 判提交在树（目录形带尾 `/` 需子路径命中）；未提交且 ack_map 有该 token → `acks` 加 `{"path","reason"}` 并弹出；否则入 `uncommitted`。**残留 ack_map（认领差集外路径）** → `StateError("差集闸认领对表失败：认领路径不在声明未提交差集内（与实态不符零粉饰）" + json.dumps({"unknown_acks": sorted(ack_map)}, ensure_ascii=False))`。uncommitted 非空 → `StateError("差集闸拦截：声明未提交（请求写入节对分支提交树差集非空，整批拒零动作；补提交或 close --ack-uncommitted 逐路径带事由放行）" + json.dumps(declaration_gate 全报告, ensure_ascii=False))`。CLI 层 ack 解析（close 分支前）：
    - 无 `=` → `_emit({"error": "差集闸认领形非法：须 路径=事由（事由零粉饰留档）", "reason": "ack_uncommitted_invalid", "item": item}, 1)`；
    - path 或 reason strip 后空 → `_emit({"error": "差集闸认领形非法：路径与事由俱必填", "reason": "ack_uncommitted_invalid", "item": item}, 1)`；
    - `item.split("=", 1)`（只切首个等号）。
14. **归并/删支/拆本**：ahead = `rev-list --count {base}..{branch}`；非 "0" 先 union 让位（`_chain_union_yield`）再 `git merge --no-ff branch -m f"merge: {stem} 副本归并"`；失败 → record `result: "merge_failed"` 入 failed；`worktree remove [--force]`；支在（rev-parse --verify refs/heads）则 `branch -d`，失败 `branch_delete_failed`；支缺席 `already_gone`；worktree 缺席 `result: "missing"`。record 字段含 ahead 无（ahead 只用于判定）；record 是 entry 复制 + worktree_result/branch_result/result/detail/chain_union。
15. **失败收约**：failed 非空 → append_event 行 `{"event":"close_failed","package","reason","failed","removed","session_id","failed_at": <at or now_iso()>,"tool":{...}}`，返回 `{"failed","removed","revoked": False,"session_id"}`；CLI `_emit_error("worktree remove failed, session stays active", code=1, extra={"failed","removed"})`。
16. **成功吊销**：append_event 行 `{"event":"revoked","package","reason","removed","revoked_at": <at or now_iso()>,"session_id","tool":{...},"detail":{"declaration_gate","calllog_gate","calllog_treadmill","sddgate_gate","gates_skipped"}}`。`gates_skipped = collect_gates_skipped(bypass_orphan, bypass_calllog, declaration_gate)` = `{"orphan_bypass": 事由或null, "calllog_bypass": ..., "ack_uncommitted": [ack 路径列表], "count": 非空计数}`。返回值另附 `declaration_gate, chain_gate, calllog_gate, calllog_treadmill, sddgate_gate, gates_skipped`。CLI 成功后 `_write_close_receipt`：checks 文件（`<root>/sih-tools/lease/ledger/checks/<stem>.json`）加 `closed_at`（at or now）与 `close_session`，转写 `<root>/sih-tools/lease/ledger/receipts/<stem>.json` 后删活件；回执附 `close_receipt: {"receipt":..., "check_file_removed":...}`。

**bypass 直写行 vs append_event 行的差异（移植要点）**：close 内三闸（bypassed_orphan / bypassed_calllog / bypassed_sddgate）用裸 `open(..., "a")` + `json.dumps(entry, ensure_ascii=False)`（**无 sort_keys、无 separators 紧凑、插字典序**）+ `\n`；而 `lease bypass` 子命令走 `append_event`（flock + sort_keys 紧凑）。Rust 侧须分别复刻两种字节形。

---

## 5. cli.py 接线要点（执法面相关子命令）

- `lease commit`：`--repo/--stage(wip|settle)/--subject` 必填；`--seq type=int`；settle 缺 seq → exit 2 `{"error": "settle requires --seq"}`；trails 缺省 `_default_trails(root)`。
- `lease reconcile`：`--repo` 必填；`--base/--limit/--bypass-ledger`；seal 自动起算。
- `lease bypass`：`--repo/--sha/--reason` 必填，`--session` 可选。
- `lease close`：`--package` 必填；`--force`；`--ack-uncommitted` 可重复等号形；`--bypass-orphan/--bypass-calllog/--bypass-sddgate` 给事由即绕行留痕、零给即闸拦；`--trail` 可重复（缺省 `_default_trails`）。
- `lease install-hooks/uninstall-hooks`：`--repo` 可重复缺省双仓。
- 自举闸（所有命令前）：src 或 cwd 落 `worktrees/` 段 → 必须三参（--ledger/--locks/--bills）全显式，否则 exit 2 payload `{"error": "self_boot_rejected", "reason": "工位（worktrees/ 下）跑 CLI 必须三参显式全传：--ledger + --locks + --bills", "self_boot_msg": ..., "action": "传三参即可放行", "resolved_defaults": {...}}`（env `ROOTANCHOR_DISABLE_SELF_BOOT=1` 豁免）。

---

## 6. 非确定性源清单（金向量归一用）

| # | 位置 | 源 | 冻结旗标 | 归一建议 |
|---|---|---|---|---|
| 1 | `core.now_iso()` | `datetime.now(utc).isoformat(timespec="seconds")` | `--at`（open/claim/unclaim/lock/unlock/close/bypass/heartbeat/takeover 等） | `--at` 覆盖处可复演 |
| 2 | commitcore.record_bypass `"at"` | now_iso() | `--at` 可冻结 | 同上 |
| 3 | close 三闸 bypass 直写行 `"at"`（bypassed_orphan/calllog/sddgate） | **now_iso() 无条件，不受 --at 冻结** | 无 | 金向量须整 `"at":"..."` 值归一（精确跨度：该键值串） |
| 4 | close `revoked_at` / `close_failed.failed_at` | at or now_iso() | `--at` | 可复演 |
| 5 | `_write_close_receipt` 的 `closed_at` | at or now_iso() | `--at` | 可复演 |
| 6 | `make_session_id` | sha256(package|issued_at|file_sha|repos)[:16] | 随 issued_at 冻结 | 派生确定 |
| 7 | cli `_pid_started` / 检验文件 `pid`/`pid_started` | `ps -o lstart=`、`os.getpid()` | 无 | 工具异常面非正典输出，金向量避开 open/takeover 或归一 detail |
| 8 | `gauge_summary` | 子进程 gauge read，`at=(at or now_iso())[:10]`；失败 → `{"skipped": true, "reason": ...}`（reason 载 exc 类名） | settle 回执随 --at | settle 金向量归一 gauge 节或造缺省跳过环境（缺 gauge cli/trail → reason "missing" 确定形） |
| 9 | reconcile / _base_label / sddgate `first_impl_commit` | git 输出（sha/日期/%cI） | 由夹具仓决定 | 夹具仓固定即确定 |
| 10 | 路径依赖：`tool_dir()`（__file__ 派生）、`resolve_root`（__file__ 上溯）、`Path.resolve()`（/var→/private/var macOS 符号链接差异）、`repo`/`trails` 绝对路径入回执与 bypass 行 | 文件系统 | `--root` 部分覆盖；resolve 的 symlink 展开不可控 | 金向量统一 resolve 后串或归一 |
| 11 | 错误文案内嵌动态片段：`git stderr[:200]`、`exc` 字符串、`unowned` 清单、deviation entry 全文、conflicts json | 环境 | 无 | 归一自由文本段精确跨度：StateError str 的 stderr 部分、no_master 清单 json、SDDG uncarried_entries |
| 12 | `_emit` 键序 | sort_keys=ASCII 序 | 确定 | 移植按 ASCII 字典序输出 |
| 13 | argparse 用法错 | argparse stderr + exit 2 | — | Rust 侧自行对齐 usage 文案（逐字节对等通常不要求，簇 B 实测定界） |

---

## 7. CONTRACT.md 修订节速查（执法面相关）

- 修订四十（closefix-solo）：归并三点式 + 台账追加面冻结豁免（`LEASE_LEDGER_APPEND_FACES` 四面）+ 预收提交最小化 + 未跟踪件照单不清场。
- 修订四十九~五十（closegate/precommit）：无主闸（bypassed_orphan 行形）+ pre-commit 文件面 `staged_enforce`。
- 修订五十一（orphanexec）：CALL-LOG 随批检查位（bypassed_calllog 行形，CALLLOG_GUARD_EFFECTIVE_AT 过渡条款）。
- 修订五十二（hygspots）：CALL-LOG 跑步机收编位（CALLLOG_TREADMILL_EFFECTIVE_AT 过渡条款）。
- 修订五十四（recclsf）：reconcile 认 cert_backfilled/cert_writtenoff 两形 + 跑步机收编面路径形态归一（根相对与仓相对两形）。
- 修订五十九（domaware）：域感知台账缺省 + `gates_skipped` 归集字段。
- 修订六十一（scoperoot）：scope 归一共形（normalize 后精确等或目录前缀；allow 点号=全容）+ wip message 纳 note。
- 修订六十二（sddgate-solo）：SDDG 四判据 + close 接线 + bypassed_sddgate 行形；成功路径链面认证笔 details.sddgate 标记（该标记由 scribe 侧承载，不在本四模块写面内）。

## 8. 移植陷阱清单（逐字节对等的已知难点）

1. close 三闸 bypass 行**无 sort_keys**、bypass 子命令行有——两写形并存。
2. `summary["unrouted"]` 恒 0 的兼容键必须保留。
3. `unrouted_tail` 含 routed_merge/routed_direct 但退出码只看 unrouted/session_orphan/cert_missing/unbypassed。
4. commit settle 缺 `--seq` 是 **exit 2**，缺 `--cert` 是 WorktreeError 也是 **exit 2**——两者都不是执法 1。
5. `build_message` settle 形 session/cert/base 同一行；`base` 可为空串。
6. `_cert_on_chain` 是 `startswith` 前缀匹配（八位短形即可）；`_bypass_hit` 是**双向** startswith。
7. SDDG `_parse_ts` naive 视 UTC；`sddgate_applicable` 用 `date()` 比较且不可解析 → **受检**（fail-closed），而 GATE_SINCE 用 `>=`（同日受检）。
8. SDDG-2 头注取 `git show` 前 20 行后 join，末尾无换行——正则 `$` 语义差异要注意（Rust regex 默认 multi-line off，Python search 单次命中语义）。
9. `_deviation_entries` 标题行判定两种形态（`#...偏差` 或裸 `偏差` 起）；条目剥 `-`/`*` 前缀用 `lstrip`（剥字符集非前缀串）。
10. SDDG-3 reason 字段只在部分分支出现（diff_empty/results_doc_absent/uncarried/absent），pass-with-section 分支无 reason 键。
11. SDDG-4 `reason` 键仅 cross 通道且零测试件时出现。
12. Python `re` 与 Rust `regex` 语法差异：`(?:...)`、`\b` 对中文边界的语义、`(?=...)` 前瞻（BATCH_PREFIX 用到，Rust regex crate **不支持前瞻**，需手写判定）。
13. `no_master_check` 只取 `trails[0]`（过滤在盘后的第一个，即 `_default_trails` 排序后的字典序首个）。
14. watchcheck `unowned_list` 是外部依赖（sys.path 导入 `watchcheck.core`），Rust 侧需对应实现或接口。
15. `scope_allows`（lockcore）为 commit 范围验单源，移植需读 lockcore.py 该函数（归一后精确等或目录前缀、点号/空条目全容）。
