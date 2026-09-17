//! 租约引擎件：锁核腿 fixture 对等实装（SPEC-024 腿一，lease-lockcore-solo 批）。
//! 覆盖域：fixture 域根 ceremony 即 open 与 lock 与 unlock 与 close 与 status 五子命令，
//! 金向量逐字节对等（归一 session_id 与域根两域）。chained workspace 门族归腿一后继批。
//! 腿二收约执法面（lease-commitlaw-parallel 批）：commit 与 bypass 与 reconcile 三子命令
//! 与 SDDG 四判据门与直提守卫纯函数，子模块 lease/{commitlaw,sddgate,guardlaw}.rs。
//! open repos 面单一源化（openface-solo 批）：repos 推导自任务包声明面仓前缀
//!（closegate repo_entries 前缀解析形），--repo 旗标保留为显式扩写位并集。

#[path = "lease/closegate.rs"]
mod closegate;
#[path = "lease/commitlaw.rs"]
mod commitlaw;
#[path = "lease/guardlaw.rs"]
mod guardlaw;
#[path = "lease/sddgate.rs"]
mod sddgate;
#[path = "lease/attachments.rs"]
mod attachments;
#[path = "lease/calllogface.rs"]
mod calllogface;
#[path = "lease/sweepcore.rs"]
mod sweepcore;

use chrono::Utc;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

// 工件 tool.version 承契约面字面（SPEC-015 先例：工件工具名版本字段是契约面标识非二进制身份）
const VERSION: &str = "1.46.0";
const FACES: [&str; 4] = [
    "sih-engine/sih/state/plan",
    "sih-engine/task-packages",
    "sih-math/sih/event/plan",
    "sih/state/plan",
];

fn die(rc: i32, err: &str, detail: Value) -> ! {
    let mut m = Map::new();
    m.insert("error".into(), json!(err));
    if !detail.is_null() {
        m.insert("detail".into(), detail);
    }
    eprintln!("{}", emit(&Value::Object(m)));
    std::process::exit(rc);
}

fn sort_json(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let bt: BTreeMap<&String, &Value> = m.iter().collect();
            Value::Object(
                bt.into_iter()
                    .map(|(k, val)| (k.clone(), sort_json(val)))
                    .collect(),
            )
        }
        Value::Array(a) => Value::Array(a.iter().map(sort_json).collect()),
        other => other.clone(),
    }
}

fn emit(v: &Value) -> String {
    let mut s = serde_json::to_string_pretty(&sort_json(v)).unwrap();
    s.push('\n');
    s
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn now_utc() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%S+00:00").to_string()
}

fn tool() -> Value {
    json!({"name": "lease", "version": VERSION})
}

fn append_row(path: &Path, v: &Value) {
    use std::fs::OpenOptions;
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p).ok();
    }
    let mut f = OpenOptions::new().create(true).append(true).open(path).unwrap();
    writeln!(f, "{}", serde_json::to_string(&sort_json(v)).unwrap()).ok();
}

fn read_jsonl(path: &Path) -> Vec<Value> {
    match std::fs::read_to_string(path) {
        Ok(t) => t
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).unwrap_or(Value::Null))
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn parse_args(args: &[String]) -> (String, BTreeMap<String, Vec<String>>) {
    let mut sub = String::new();
    let mut m: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if let Some(k) = a.strip_prefix("--") {
            let repeat = k == "allow" || k == "repo";
            if i + 1 < args.len() && (repeat || !args[i + 1].starts_with("--")) {
                i += 1;
                m.entry(k.to_string()).or_default().push(args[i].clone());
            } else {
                m.entry(k.to_string()).or_default().push(String::new());
            }
        } else if sub.is_empty() {
            sub = a.clone();
        }
        i += 1;
    }
    (sub, m)
}

fn one<'a>(m: &'a BTreeMap<String, Vec<String>>, k: &str) -> Option<&'a str> {
    m.get(k).and_then(|v| v.first()).map(|s| s.as_str())
}

/// allow 条目抽取（gap-lease-allow-parse，recognize-solo 双跑偏差第二处清偿）：
/// 对表围堰 declguard-solo 冻结启发族（core.py parse_requested_writes 列表项
/// 语义，CONTRACT 修订六十一其四）——遇「——」截断取路径部，理由散文不入
/// allow，剥首尾反引号；抽后空串即不入面。
fn extract_allow_item(raw: &str) -> Option<String> {
    let p = raw.split("——").next().unwrap_or("").trim().trim_matches('`');
    if p.is_empty() {
        None
    } else {
        Some(p.to_string())
    }
}

/// 包档显式 allow 键与包档抽取面组合（gap-lease-allow-parse）：显式键同经
/// 散文不入 allow 抽取，再对包档面去重并入（组合形对表围堰 core.py
/// open_preflight：requested + 显式不在 requested 者原序保留）。
fn compose_allow(requested: &[String], explicit: &[String]) -> Vec<String> {
    let mut allow = requested.to_vec();
    for a in explicit {
        if let Some(p) = extract_allow_item(a) {
            if !requested.contains(&p) {
                allow.push(p);
            }
        }
    }
    allow
}

/// 域内已知仓名面（pk-104 openface-solo）：open 推导位的仓前缀集合，
/// closegate 声明差集闸 repo_entries 的仓名来源在会话 repos，open 时
/// 会话未立故以已知仓面锚定。
const KNOWN_REPOS: [&str; 2] = ["sih-engine", "sih-tools"];

/// 声明面仓前缀推导（pk-104 openface-solo）：复用 closegate.rs 声明差集闸
/// repo_entries 前缀解析形（token.starts_with("<repo>/")）——声明路径以
/// sih-engine/ 开头即挂引擎仓、sih-tools/ 开头即挂工具仓，域外件
/// （AGENTS.md 与 远期备忘录.md 形）不推导仓；推导面空（声明全无仓前缀）
/// 回落缺省 sih-engine（腿一金向量兼容形）。
fn derive_repo_names(declared: &[String]) -> Vec<String> {
    let mut repos: Vec<String> = KNOWN_REPOS
        .iter()
        .filter(|r| {
            let prefix = format!("{}/", r);
            declared.iter().any(|p| p.starts_with(&prefix))
        })
        .map(|r| r.to_string())
        .collect();
    if repos.is_empty() {
        repos.push("sih-engine".into());
    }
    repos
}

/// repos 组合形（pk-104 openface-solo，组合序对表 compose_allow）：推导面
/// 原序保留，--repo 显式旗标不在面者按传入序并入（显式扩写位并集，面不重账）。
fn compose_repo_names(derived: &[String], explicit: &[String]) -> Vec<String> {
    let mut names = derived.to_vec();
    for r in explicit {
        if !names.contains(r) {
            names.push(r.clone());
        }
    }
    names
}

fn parse_requested_writes(text: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut in_section = false;
    for line in text.lines() {
        if line.starts_with("## ") {
            in_section = line.split('{').next().unwrap_or("").contains("请求写入");
            continue;
        }
        if !in_section {
            continue;
        }
        let s = line.trim();
        if s.starts_with("- ") || s.starts_with("* ") {
            if let Some(p) = extract_allow_item(&s[2..]) {
                paths.push(p);
            }
        }
    }
    paths
}

fn resolve_package(root: &Path, stem: &str) -> PathBuf {
    for face in FACES {
        let p = root.join(face).join(format!("{}.md", stem));
        if p.is_file() {
            return p;
        }
    }
    die(
        1,
        &format!("package file not found for stem {:?}, searched faces", stem),
        json!({"faces": FACES.iter().map(|f| root.join(f).join(format!("{}.md", stem))).collect::<Vec<_>>()}),
    );
}

pub(crate) fn resolve_package_quiet(root: &Path, stem: &str) -> PathBuf {
    for face in FACES {
        let p = root.join(face).join(format!("{}.md", stem));
        if p.is_file() {
            return p;
        }
    }
    root.join(FACES[0]).join(format!("{}.md", stem))
}

fn active_sessions(ledger: &Path) -> Vec<(String, Value)> {
    let mut last: BTreeMap<String, Value> = BTreeMap::new();
    for row in read_jsonl(ledger) {
        if let Some(sid) = row.get("session_id").and_then(|v| v.as_str()) {
            last.insert(sid.to_string(), row);
        }
    }
    last.into_iter()
        .filter(|(_, row)| row.get("event").and_then(|v| v.as_str()) == Some("issued"))
        .collect()
}

fn stem_check(
    root: &Path,
    stem: &str,
    m: &BTreeMap<String, Vec<String>>,
) -> Value {
    attachments::stem_check_full(
        root,
        stem,
        m.contains_key("new-stem"),
        one(m, "claim-zh").unwrap_or(""),
        one(m, "claim-code").unwrap_or(""),
        one(m, "claim-derivation").unwrap_or(""),
    )
}

fn cmd_open(m: &BTreeMap<String, Vec<String>>) {
    // pk-103 收口：root 缺省自 cwd 上溯（canonical 先检域界即停再双仓标记），
    // 词典包路径随 root 锚定与 cwd 无关。
    let root_flag = one(m, "root").map(|s| s.to_string()).filter(|s| !s.is_empty());
    let root = attachments::detect_domain_context(root_flag.as_deref());
    let stem = one(m, "package").unwrap_or("").to_string();
    if stem.is_empty() {
        die(2, "open 缺 --package", json!(null));
    }
    let identity_path = PathBuf::from(one(m, "identity").unwrap_or(""));
    let intent_path = PathBuf::from(one(m, "intent").unwrap_or(""));
    let at = one(m, "at")
        .map(|s| s.to_string())
        .unwrap_or_else(now_utc);

    let ledger = PathBuf::from(one(m, "ledger").unwrap_or(""));
    let locks = PathBuf::from(one(m, "locks").unwrap_or(""));
    let bills = PathBuf::from(one(m, "bills").unwrap_or(""));
    if ledger.as_os_str().is_empty() || locks.as_os_str().is_empty() || bills.as_os_str().is_empty() {
        die(2, "open 须三台账显式全传（fixture scope）", json!(null));
    }
    // 目录骨架先行：域内 sih-tools/lease/ledger 面（与围堰行为对表）
    for d in [
        ledger.parent().unwrap_or(Path::new("/")),
        locks.parent().unwrap_or(Path::new("/")),
        bills.parent().unwrap_or(Path::new("/")),
        root.join("sih-tools/lease/ledger/checks").as_path(),
        root.join("sih-tools/lease/ledger/receipts").as_path(),
    ] {
        std::fs::create_dir_all(d).ok();
    }

    let pkg_path = resolve_package(&root, &stem);
    let pkg_text = std::fs::read_to_string(&pkg_path).unwrap_or_default();
    let requested = parse_requested_writes(&pkg_text);
    let explicit: Vec<String> = m.get("allow").cloned().unwrap_or_default();
    let allow = compose_allow(&requested, &explicit);

    let sc = stem_check(&root, &stem, m);

    let id_bytes = std::fs::read(&identity_path).unwrap_or_default();
    let id_json: Value = serde_json::from_slice(&id_bytes).unwrap_or(Value::Null);
    let core_hash = id_json
        .pointer("/identity/core_hash")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let identity_hash = id_json
        .pointer("/identity/identity_hash")
        .or_else(|| id_json.pointer("/identity/hash"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let intent_bytes = std::fs::read(&intent_path).unwrap_or_default();
    let intent_json: Value = serde_json::from_slice(&intent_bytes).unwrap_or(Value::Null);
    let anchor_count = intent_json
        .get("anchors")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    // gauge 底座探针（三固定件位）
    let probe = [
        root.join("sih-tools/gauge/src/gauge/cli.py"),
        root.join("sih-tools/lease/ledger/sessions.ndjson"),
        root.join("sih-engine/sih/event/trail"),
    ];
    let missing: Vec<String> = probe
        .iter()
        .filter(|p| !p.exists())
        .map(|p| p.display().to_string())
        .collect();
    let gauge_block = if !missing.is_empty() {
        json!({"missing": missing, "reason": "missing", "skipped": true})
    } else {
        json!({"missing": [], "reason": "ok", "skipped": false})
    };

    // repos 面单一源化（pk-104 openface-solo）：推导自任务包声明面仓前缀，
    // --repo 旗标保留为显式扩写位与推导面并集。
    let explicit_repos: Vec<String> = m.get("repo").cloned().unwrap_or_default();
    let repo_names = compose_repo_names(&derive_repo_names(&requested), &explicit_repos);
    let mut repos = Vec::new();
    for rn in &repo_names {
        let repo_dir = commitlaw::py_resolve(&root.join(rn));
        let base = Command::new("git")
            .args(["-C", &repo_dir.display().to_string(), "rev-parse", "--abbrev-ref", "HEAD"])
            .output();
        let base_branch = match base {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            _ => die(2, "repo 面非 git 仓", json!({"repo": repo_dir})),
        };
        let branch = format!("msh/{}", stem);
        let worktree_raw = root.join("worktrees").join(rn).join(&stem);
        let wa = Command::new("git")
            .args([
                "-C",
                &repo_dir.display().to_string(),
                "worktree",
                "add",
                "-b",
                &branch,
                &worktree_raw.display().to_string(),
            ])
            .output();
        match wa {
            Ok(o) if o.status.success() => {}
            Ok(o) => die(
                2,
                "git worktree add failed",
                json!({"stderr": String::from_utf8_lossy(&o.stderr)}),
            ),
            Err(e) => die(2, "git worktree add failed", json!({"err": e.to_string()})),
        }
        repos.push(json!({
            "base_branch": base_branch,
            "branch": branch,
            "repo": repo_dir.display().to_string(),
            "worktree": commitlaw::py_resolve(&worktree_raw).display().to_string(),
        }));
    }

    // 会话号即确定性派生（core.make_session_id 对表）：sha256(包名|签发时刻|身份件哈希|仓序列)[:16]
    let repo_paths: Vec<String> = repos
        .iter()
        .map(|r| r.get("repo").and_then(|v| v.as_str()).unwrap_or("").to_string())
        .collect();
    let sid_payload = format!(
        "{}|{}|{}|{}",
        stem,
        at,
        sha256_hex(&id_bytes),
        repo_paths.join("|")
    );
    let sid: String = sha256_hex(sid_payload.as_bytes())[..16].to_string();
    let receipt = json!({
        "allow": allow,
        "allow_derived": [],
        "allow_effective": allow,
        "event": "issued",
        "gauge": gauge_block,
        "identity": {
            "core_hash": core_hash,
            "file_sha256": sha256_hex(&id_bytes),
            "identity_hash": identity_hash,
        },
        "intent": {
            "anchor_count": anchor_count,
            "record_sha256": sha256_hex(&intent_bytes),
            "validated_by": if anchor_count > 0 { "lease-ask3" } else { "lease-structural" },
        },
        "issued_at": at,
        "package": stem,
        "package_path": pkg_path.display().to_string(),
        "repos": repos,
        "scope_source": "package",
        "session_id": sid,
        "stem_check": sc,
        "tool": tool(),
    });
    append_row(&ledger, &receipt);
    // 乐观基线 check 件占位（close 时移除，路径面与围堰对表）
    let check_file = root.join(format!("sih-tools/lease/ledger/checks/{}.json", stem));
    std::fs::write(&check_file, "{}\n").ok();
    print!("{}", emit(&receipt));
}

fn lock_held_by(locks: &Path, path: &str) -> Option<String> {
    let mut held: BTreeMap<String, String> = BTreeMap::new();
    for row in read_jsonl(locks) {
        let ev = row.get("event").and_then(|v| v.as_str()).unwrap_or("");
        let p = row.get("path").and_then(|v| v.as_str()).unwrap_or("");
        let sid = row.get("session_id").and_then(|v| v.as_str()).unwrap_or("");
        if p != path {
            continue;
        }
        match ev {
            "acquired" => held.insert(p.to_string(), sid.to_string()),
            "released" => held.remove(p),
            _ => None,
        };
    }
    held.get(path).cloned()
}

fn cmd_lock(m: &BTreeMap<String, Vec<String>>) {
    let locks = PathBuf::from(one(m, "locks").unwrap_or(""));
    let ledger = PathBuf::from(one(m, "ledger").unwrap_or(""));
    let path = one(m, "path").unwrap_or("").to_string();
    let at = one(m, "at").map(|s| s.to_string()).unwrap_or_else(now_utc);
    let mode = one(m, "mode").unwrap_or("exclusive").to_string();
    let sid = match one(m, "session") {
        Some(s) => s.to_string(),
        None => match active_sessions(&ledger).len() {
            1 => active_sessions(&ledger)[0].0.clone(),
            _ => die(
                1,
                "session_not_active",
                json!({"reason": "缺 --session 且活跃会话不唯一"}),
            ),
        },
    };
    let active: Vec<(String, Value)> = active_sessions(&ledger)
        .into_iter()
        .filter(|(s, _)| *s == sid)
        .collect();
    if active.is_empty() {
        die(1, "session_not_active", json!({"wanted": sid}));
    }
    if let Some(holder) = lock_held_by(&locks, &path) {
        if holder != sid {
            die(1, "locked_elsewhere", json!({"path": path, "held_by": holder}));
        }
        // 同会话重入：duplicate 形，不追加行
        let receipt = json!({
            "duplicate": true,
            "line": {"acquired_at": at, "event": "acquired", "mode": mode, "path": path, "session_id": sid, "tool": tool()},
        });
        print!("{}", emit(&receipt));
        return;
    }
    let line = json!({
        "acquired_at": at,
        "event": "acquired",
        "mode": mode,
        "path": path,
        "session_id": sid,
        "tool": tool(),
    });
    append_row(&locks, &line);
    let receipt = json!({"duplicate": false, "line": line});
    print!("{}", emit(&receipt));
}

fn cmd_unlock(m: &BTreeMap<String, Vec<String>>) {
    let locks = PathBuf::from(one(m, "locks").unwrap_or(""));
    let ledger = PathBuf::from(one(m, "ledger").unwrap_or(""));
    let path = one(m, "path").unwrap_or("").to_string();
    let at = one(m, "at").map(|s| s.to_string()).unwrap_or_else(now_utc);
    let sid = match one(m, "session") {
        Some(s) => s.to_string(),
        None => match active_sessions(&ledger).len() {
            1 => active_sessions(&ledger)[0].0.clone(),
            _ => die(1, "session_not_active", json!({"reason": "缺 --session"})),
        },
    };
    match lock_held_by(&locks, &path) {
        Some(h) if h == sid => {}
        Some(h) => die(1, "locked_elsewhere", json!({"path": path, "held_by": h})),
        None => die(1, "not_held", json!({"path": path})),
    }
    let line = json!({
        "event": "released",
        "path": path,
        "released_at": at,
        "session_id": sid,
        "tool": tool(),
    });
    append_row(&locks, &line);
    print!("{}", emit(&json!({"line": line})));
}

fn cmd_status(m: &BTreeMap<String, Vec<String>>) {
    let ledger = PathBuf::from(one(m, "ledger").unwrap_or("sessions.ndjson"));
    let locks = PathBuf::from(one(m, "locks").unwrap_or("locks.ndjson"));
    let held: Vec<Value> = Vec::new();
    let status = json!({
        "header": {
            "locks_ledger": locks.display().to_string(),
            "sessions_ledger": ledger.display().to_string(),
            "tool": tool(),
        },
        "locks": {
            "header": {"locks_ledger": locks.display().to_string(), "tool": tool()},
            "held": held,
            "summary": {"held": 0},
        },
        "sessions": {"active": []},
    });
    print!("{}", emit(&status));
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        die(2, "用法 lease <open|lock|unlock|close|status> [flags]", json!(null));
    }
    let (sub, m) = parse_args(&args);
    match sub.as_str() {
        "open" => cmd_open(&m),
        "lock" => cmd_lock(&m),
        "unlock" => cmd_unlock(&m),
        "close" => closegate::cmd_close(&m),
        "status" => cmd_status(&m),
        "commit" => commitlaw::cmd_commit(&m),
        "bypass" => commitlaw::cmd_bypass(&m),
        "reconcile" => commitlaw::cmd_reconcile(&m),
        "sweep" => sweepcore::cmd_sweep(&m),
        "call-log" => calllogface::cmd_calllog_import(&m),
        "install-hooks" => attachments::cmd_install_hooks(&m),
        "uninstall-hooks" => attachments::cmd_uninstall_hooks(&m),
        other => die(
            2,
            &format!("子命令 {} 未在融回对等域：{} 覆盖", other, "open/lock/unlock/close/status/commit/bypass/reconcile/sweep/call-log/install-hooks/uninstall-hooks"),
            json!(null),
        ),
    }
}

#[cfg(test)]
mod allow_parse_tests {
    use super::*;

    /// recognize-solo 双跑偏差第二处回归钉：反引号路径附理由散文的包档行，
    /// 旧形整行收纳（反引号在面致锁范围验必拒），围堰为路径抽取。
    #[test]
    fn backticked_path_with_prose_reason_extracts_path() {
        let pkg = "# 包\n\n## 九、请求写入 {#requested-writes}\n\n\
                   - `sih-engine/src/mcpserver/httpface.rs`——识别四元化与三闸与教学载荷\n\
                   - sih-engine/src/bin/lease.rs——锁范围解析位修复\n";
        assert_eq!(
            parse_requested_writes(pkg),
            vec![
                "sih-engine/src/mcpserver/httpface.rs".to_string(),
                "sih-engine/src/bin/lease.rs".to_string(),
            ]
        );
    }

    /// 「——」后散文不入面；星号列表项同读；纯净路径行原样。
    #[test]
    fn dash_truncation_and_star_marker() {
        let pkg = "## 请求写入 {#rw}\n\n\
                   * sih-engine/src/a.rs——理由散文不入\n\
                   - sih-engine/src/b.rs\n\
                   - 理由前置散文无截断符\n";
        assert_eq!(
            parse_requested_writes(pkg),
            vec![
                "sih-engine/src/a.rs".to_string(),
                "sih-engine/src/b.rs".to_string(),
                "理由前置散文无截断符".to_string(),
            ]
        );
    }

    /// 节界检测：请求写入节外列表项不入；节题带 {#anchor} 前缀判定同围堰
    /// line.split("{",1)[0] 含「请求写入」语义。
    #[test]
    fn section_scoping_excludes_other_sections() {
        let pkg = "## 二、关键设计 {#design}\n\n\
                   - sih-engine/src/wrong.rs\n\n\
                   ## 九、请求写入 {#requested-writes}\n\n\
                   - sih-engine/src/right.rs\n";
        assert_eq!(parse_requested_writes(pkg), vec!["sih-engine/src/right.rs".to_string()]);
    }

    /// 占位符与裸词对表围堰现行文原样保留（core.py 无 '<' 过滤、无斜杠词形
    /// 不猜——declguard 冻结启发族在 allow 面只做「——」截断加剥引）。
    #[test]
    fn placeholder_and_bare_word_kept_for_weizhan_parity() {
        let pkg = "## 请求写入 {#rw}\n\n\
                   - <占位符不入>\n\
                   - materials\n";
        assert_eq!(
            parse_requested_writes(pkg),
            vec!["<占位符不入>".to_string(), "materials".to_string()]
        );
    }

    /// 显式 allow 键同经散文不入 allow 抽取，并对包档抽取面去重（组合形
    /// 对表围堰 core.py open_preflight）；requested 面序在前原样保留。
    #[test]
    fn explicit_allow_extracted_and_deduped_against_requested() {
        let requested = parse_requested_writes("## 请求写入 {#rw}\n\n- sih-engine/src/a.rs——包档理由\n");
        let composed = compose_allow(
            &requested,
            &[
                "`sih-engine/src/b.rs`——显式补面散文".to_string(),
                "sih-engine/src/a.rs——包档已载重复键".to_string(),
            ],
        );
        assert_eq!(
            composed,
            vec![
                "sih-engine/src/a.rs".to_string(),
                "sih-engine/src/b.rs".to_string(),
            ]
        );
    }

    /// 抽取后空串（裸「——」行）不入面。
    #[test]
    fn empty_extraction_dropped() {
        assert_eq!(extract_allow_item("——纯理由行"), None);
        assert_eq!(extract_allow_item("``"), None);
        assert_eq!(
            extract_allow_item("`sih-engine/src/c.rs`——注"),
            Some("sih-engine/src/c.rs".to_string())
        );
    }
}
