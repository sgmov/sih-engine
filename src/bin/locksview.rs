//! 引擎侧锁视图工具（locksview）命令行面 —— lease-mergeleg6-parallel 簇I 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/locks 0.2.0（cli.py、core.py），只读对表移植，
//! 围堰源码零改动。独立锁视图工具，与引擎 lease 内 locks 相关模块（lease.rs 的
//! lock_held_by 与锁台账参数面）无同名承载关系；bin 名取 locksview 避让引擎
//! lease 侧 locks 语义位，头注申报。
//!
//! CLI：`locksview <acquire|release|status|check> ...`
//! 退出码三值：0 = 过（取锁/放锁/查锁/净上游或不在册）、1 = 拦（五验失败、
//! locked_elsewhere、not_holder、not_locked、脏上游）、2 = 工具异常（文件不可读、
//! json 非法、级联不在、多在册无参、语料前缀外）。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. 缺省路径源不同：围堰 --root 缺省取源码文件位置向上推工作区根，--locks 缺省
//!    取 sih-tools/locks/ledger/locks.ndjson，--cascade-dir 缺省取 sih-tools/cascade；
//!    本移植三者缺省均按当前工作目录展开（root 缺省即 cwd，locks 台账缺省即
//!    <root>/sih-tools/locks/ledger/locks.ndjson，级联缺省即 <root>/sih-tools/cascade），
//!    显式传参行为逐字节同构。
//! 2. 级联子进程 120 秒超时未实装（std::process 无超时承载，围堰 subprocess
//!    timeout=120）；挂起形为无限等待，其余退出码语义同构。
//! 3. 绑定比对 str() 对 bool 形：Python str(True)="True"，移植为 "true"；
//!    hostname/user/boottime 三键在围堰消费面恒为字符串形，实际观测无差。
//! 4. argparse 用法错为自足解析，stderr 短句回报退出码 2，报文形不对齐。
//! 5. 锁台账行为 Python json.dumps(sort_keys=True, separators=(",", ":")) 紧凑形
//!    逐字节同构（ensure_ascii=False 原 UTF-8）；stdout 报告为 indent=2 同构。
//! 6. 乐观级联腿承载位变更（gap-locksview-cascade-engine-bin）：围堰 run_cascade_check
//!    经 `uv run --project <cascade-dir> cascade check` 子进程承载，本侧改直调引擎
//!    cascade bin（CASCADE_BIN_OVERRIDE 显式覆盖位优先——测试缝，围堰无此变量；
//!    缺省序当前 exe 同目录兄弟位优先，缺席回落 PATH 检索；SPEC-025 A2
//!    零 sih-tools spawn）；--cascade-dir 旗标仍收参（CLI 面兼容）但不再选 uv
//!    工地位，其余参形与判词消费面（targets/dirty/verdict）不变。

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

const TOOL_NAME: &str = "locks";
const TOOL_VERSION: &str = "0.2.0";
const BINDING_KEYS: [&str; 3] = ["hostname", "user", "boottime"];
const DOC_PREFIX: &str = "sih-engine/doc/";

// ---------- 错误面 ----------

/// Tool = 工具异常（退出码二）；Blocked = 拦截（退出码一，理由码承载）。
enum LErr {
    Blocked(String, Value),
    Tool(String),
}

fn blocked(reason: &str, detail: Value) -> LErr {
    LErr::Blocked(reason.to_string(), detail)
}

// ---------- Python json.dumps 同构序列化 ----------

fn json_quote(s: &str) -> String {
    serde_json::to_string(s).unwrap()
}

fn py_json(v: &Value, indent: Option<usize>, compact: bool) -> String {
    let mut out = String::new();
    py_rec(v, indent, compact, 0, &mut out);
    out
}

fn py_rec(v: &Value, indent: Option<usize>, compact: bool, depth: usize, out: &mut String) {
    match v {
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
            kvs.sort_by(|a, b| a.0.cmp(b.0));
            out.push('{');
            for (i, (k, val)) in kvs.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                match indent {
                    Some(n) => {
                        out.push('\n');
                        out.push_str(&" ".repeat((depth + 1) * n));
                    }
                    None => {
                        if !compact {
                            out.push(' ');
                        }
                    }
                }
                out.push_str(&json_quote(k));
                out.push_str(if compact { ":" } else { ": " });
                py_rec(val, indent, compact, depth + 1, out);
            }
            if let Some(n) = indent {
                out.push('\n');
                out.push_str(&" ".repeat(depth * n));
            }
            out.push('}');
        }
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, val) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                match indent {
                    Some(n) => {
                        out.push('\n');
                        out.push_str(&" ".repeat((depth + 1) * n));
                    }
                    None => {
                        if !compact {
                            out.push(' ');
                        }
                    }
                }
                py_rec(val, indent, compact, depth + 1, out);
            }
            if let Some(n) = indent {
                out.push('\n');
                out.push_str(&" ".repeat(depth * n));
            }
            out.push(']');
        }
        Value::String(s) => out.push_str(&json_quote(s)),
        Value::Number(n) => out.push_str(&n.to_string()),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Null => out.push_str("null"),
    }
}

fn emit(payload: &Value, code: i32) -> ! {
    println!("{}", py_json(payload, Some(2), false));
    exit(code)
}

/// Python str() 同构（绑定比对消费面，头注落差三）。
fn py_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "None".to_string(),
        Value::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        Value::Number(n) => n.to_string(),
        other => py_json(other, None, false),
    }
}

fn now_iso() -> String {
    chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S+00:00")
        .to_string()
}

fn tool_stamp() -> Value {
    json!({"name": TOOL_NAME, "version": TOOL_VERSION})
}

// ---------- 台账面（core.py 对表） ----------

fn load_events(path: &Path, kind: &str) -> Result<Vec<Value>, LErr> {
    if !path.exists() {
        return Ok(vec![]);
    }
    let text = fs::read_to_string(path)
        .map_err(|_| LErr::Tool(format!("{} unreadable: {}", kind, path.display())))?;
    let mut events = vec![];
    for (number, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let v: Value = serde_json::from_str(line).map_err(|e| {
            LErr::Tool(format!("{} line {} invalid: {}", kind, number + 1, e))
        })?;
        events.push(v);
    }
    Ok(events)
}

fn append_event(path: &Path, event: &Value) -> Result<(), LErr> {
    if let Some(p) = path.parent() {
        if !p.as_os_str().is_empty() {
            fs::create_dir_all(p).map_err(|_| {
                LErr::Tool(format!("locks ledger unwritable: {}", path.display()))
            })?;
        }
    }
    let f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|_| LErr::Tool(format!("locks ledger unwritable: {}", path.display())))?;
    let line = py_json(event, None, true) + "\n";
    let mut handle = &f;
    handle
        .write_all(line.as_bytes())
        .map_err(|_| LErr::Tool(format!("locks ledger unwritable: {}", path.display())))?;
    Ok(())
}

fn active_sessions(events: &[Value]) -> Vec<Value> {
    let mut sessions: BTreeMap<String, Value> = BTreeMap::new();
    for event in events {
        if event.get("event").and_then(|v| v.as_str()) == Some("issued") {
            if let Some(sid) = event.get("session_id").and_then(|v| v.as_str()) {
                sessions.insert(sid.to_string(), event.clone());
            }
        }
    }
    for event in events {
        if event.get("event").and_then(|v| v.as_str()) == Some("revoked") {
            if let Some(sid) = event.get("session_id").and_then(|v| v.as_str()) {
                sessions.remove(sid);
            }
        }
    }
    sessions.into_values().collect()
}

fn select_session(events: &[Value], session_id: Option<&str>) -> Result<Value, LErr> {
    let actives = active_sessions(events);
    match session_id {
        None => {
            if actives.is_empty() {
                return Err(blocked("no_active_session", json!({})));
            }
            if actives.len() > 1 {
                return Err(LErr::Tool(
                    "multiple active sessions, --session required".to_string(),
                ));
            }
            Ok(actives[0].clone())
        }
        Some(wanted) => actives
            .into_iter()
            .find(|s| s.get("session_id").and_then(|v| v.as_str()) == Some(wanted))
            .ok_or_else(|| blocked("session_not_active", json!({"wanted": wanted}))),
    }
}

fn scope_allows(allow: &[Value], path: &str) -> bool {
    for entry in allow {
        let entry = entry.as_str().unwrap_or("").trim_end_matches('/');
        if entry == path || path.starts_with(&format!("{entry}/")) {
            return true;
        }
    }
    false
}

fn load_identity_report(path: &str) -> Result<(Vec<Value>, Value, Option<String>), LErr> {
    let text = fs::read_to_string(path)
        .map_err(|_| LErr::Tool(format!("identity report unreadable: {path}")))?;
    let payload: Value = serde_json::from_str(&text)
        .map_err(|e| LErr::Tool(format!("identity report invalid: {e}")))?;
    if !payload.is_object() {
        return Err(LErr::Tool(
            "identity report top level must be an object".to_string(),
        ));
    }
    let anomalies = payload
        .get("anomalies")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let observed = payload
        .get("observed")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(|| json!({}));
    let identity_hash = payload
        .get("identity")
        .and_then(|i| i.get("hash"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    Ok((anomalies, observed, identity_hash))
}

fn load_binding(worktrees_root: &Path, session_id: &str) -> Result<Value, LErr> {
    let path = worktrees_root.join(".bindings").join(format!("{session_id}.json"));
    if !path.is_file() {
        return Err(blocked("binding_absent", json!({"expected": path.display().to_string()})));
    }
    let text = fs::read_to_string(&path)
        .map_err(|_| LErr::Tool(format!("binding sidecar invalid: {}", path.display())))?;
    let binding: Value = serde_json::from_str(&text)
        .map_err(|_| LErr::Tool(format!("binding sidecar invalid: {}", path.display())))?;
    if !binding.is_object() {
        return Err(LErr::Tool(format!(
            "binding sidecar not an object: {}",
            path.display()
        )));
    }
    Ok(binding)
}

fn verify_binding(binding: &Value, observed: &Value) -> Result<(), LErr> {
    for key in BINDING_KEYS {
        let obs = observed.get(key).cloned().unwrap_or(Value::Null);
        let exp = binding.get(key).cloned().unwrap_or(Value::Null);
        // Python: str(observed.get(key, "")) vs str(binding.get(key, ""))；
        // Null 即缺键，围堰 str("") = ""，此处同形为空串。
        let obs_s = if obs.is_null() { String::new() } else { py_str(&obs) };
        let exp_s = if exp.is_null() { String::new() } else { py_str(&exp) };
        if obs_s != exp_s {
            return Err(blocked(
                &format!("{key}_drift"),
                json!({"expected": exp, "observed": obs_s}),
            ));
        }
    }
    Ok(())
}

fn verify_five(
    session: &Value,
    path: &str,
    identity_path: &str,
    worktrees_root: &Path,
) -> Result<(), LErr> {
    let allow: Vec<Value> = session
        .get("allow")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if !scope_allows(&allow, path) {
        return Err(blocked(
            "scope_violation",
            json!({"allow": allow, "path": path}),
        ));
    }
    let (anomalies, observed, _hash) = load_identity_report(identity_path)?;
    if !anomalies.is_empty() {
        return Err(blocked("identity_anomaly", json!({"anomalies": anomalies})));
    }
    let session_id = session
        .get("session_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let binding = load_binding(worktrees_root, session_id)?;
    verify_binding(&binding, &observed)
}

/// 在锁读出：单遍事件序配对（basefix-solo 修后形，放后重取互斥不绕穿）。
fn active_locks(events: &[Value], root: Option<&str>) -> BTreeMap<String, String> {
    let mut held = BTreeMap::new();
    for event in events {
        match event.get("event").and_then(|v| v.as_str()) {
            Some("acquired") => {
                if let Some(p) = event.get("path").and_then(|v| v.as_str()) {
                    held.insert(normalize_path(p, root), sget(event, "session_id"));
                }
            }
            Some("released") => {
                if let Some(p) = event.get("path").and_then(|v| v.as_str()) {
                    let key = normalize_path(p, root);
                    if held.get(&key).map(|s| s.as_str()) == event.get("session_id").and_then(|v| v.as_str()) {
                        held.remove(&key);
                    }
                }
            }
            _ => {}
        }
    }
    held
}

fn sget(e: &Value, k: &str) -> String {
    e.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string()
}

fn normalize_path(path: &str, root: Option<&str>) -> String {
    let mut s = path.to_string();
    if let Some(r) = root {
        let root_s = r.trim_end_matches('/');
        if s.starts_with(&format!("{root_s}/")) {
            s = s[root_s.len() + 1..].to_string();
        }
    }
    if let Some(stripped) = s.strip_prefix("./") {
        s = stripped.to_string();
    }
    if s.ends_with('/') && s != "/" {
        s = s.trim_end_matches('/').to_string();
    }
    s
}

fn do_acquire_or_release(
    is_acquire: bool,
    path_arg: &str,
    identity: &str,
    session_id: Option<&str>,
    at: Option<&str>,
    root: &Path,
    locks_ledger: &Path,
    worktree_ledger: &Path,
) -> Result<Value, LErr> {
    let path = normalize_path(path_arg, Some(root.to_string_lossy().as_ref()));
    let events = load_events(worktree_ledger, "worktree ledger")?;
    let session = select_session(&events, session_id)?;
    let worktrees_root = root.join("worktrees");
    verify_five(&session, &path, identity, &worktrees_root)?;
    let lock_events = load_events(locks_ledger, "locks ledger")?;
    let held = active_locks(&lock_events, Some(root.to_string_lossy().as_ref()));
    let sid = sget(&session, "session_id");
    let holder = held.get(&path).cloned();
    if is_acquire {
        if let Some(h) = &holder {
            if h != &sid {
                return Err(blocked(
                    "locked_elsewhere",
                    json!({"path": path, "holder": h}),
                ));
            }
        }
        let line = json!({
            "acquired_at": at.map(|s| s.to_string()).unwrap_or_else(now_iso),
            "event": "acquired",
            "path": path,
            "session_id": sid,
            "tool": tool_stamp(),
        });
        append_event(locks_ledger, &line)?;
        Ok(json!({"duplicate": holder.as_deref() == Some(sid.as_str()), "line": line}))
    } else {
        if !held.contains_key(&path) {
            return Err(blocked("not_locked", json!({"path": path})));
        }
        if holder.as_deref() != Some(sid.as_str()) {
            return Err(blocked(
                "not_holder",
                json!({"path": path, "holder": holder.unwrap_or_default()}),
            ));
        }
        let line = json!({
            "event": "released",
            "path": path,
            "released_at": at.map(|s| s.to_string()).unwrap_or_else(now_iso),
            "session_id": sid,
            "tool": tool_stamp(),
        });
        append_event(locks_ledger, &line)?;
        Ok(json!({"line": line}))
    }
}

fn do_status(
    path_filter: Option<&str>,
    session_filter: Option<&str>,
    root: &Path,
    locks_ledger: &Path,
) -> Result<Value, LErr> {
    let events = load_events(locks_ledger, "locks ledger")?;
    let held = active_locks(&events, Some(root.to_string_lossy().as_ref()));
    let mut report: Vec<Value> = vec![];
    for line in &events {
        if line.get("event").and_then(|v| v.as_str()) != Some("acquired") {
            continue;
        }
        let key = normalize_path(sget(line, "path").as_str(), Some(root.to_string_lossy().as_ref()));
        if held.get(&key).map(|s| s.as_str()) != Some(sget(line, "session_id").as_str()) {
            continue;
        }
        if let Some(pf) = path_filter {
            if key != normalize_path(pf, Some(root.to_string_lossy().as_ref())) {
                continue;
            }
        }
        if let Some(sf) = session_filter {
            if sget(line, "session_id") != sf {
                continue;
            }
        }
        report.push(line.clone());
    }
    report.sort_by(|a, b| {
        let ka = (sget(a, "path"), sget(a, "acquired_at"));
        let kb = (sget(b, "path"), sget(b, "acquired_at"));
        ka.cmp(&kb)
    });
    let held_count = report.len();
    Ok(json!({
        "header": {"locks_ledger": locks_ledger.display().to_string(), "tool": tool_stamp()},
        "held": report,
        "summary": {"held": held_count},
    }))
}

// ---------- 乐观链（check） ----------

fn default_trails(root: &Path) -> Vec<String> {
    // 双居所并集：引擎新家加工具侧老家（pk-092，cli.py _default_trails 对表）。
    let homes = [
        root.join("sih-engine/sih/event/trail"),
        root.join("sih-tools/scribe/trail"),
    ];
    let mut out: Vec<String> = vec![];
    for home in homes {
        if home.is_dir() {
            let mut entries: Vec<PathBuf> = fs::read_dir(&home)
                .into_iter()
                .flatten()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("ndjson"))
                .collect();
            entries.sort();
            for p in entries {
                out.push(p.display().to_string());
            }
        }
    }
    out.sort();
    out
}

/// 引擎 cascade bin 解析（gap-locksview-cascade-engine-bin）：CASCADE_BIN_OVERRIDE
/// 显式覆盖位（设置即直用，测试缝，围堰无此变量）优先；缺省序为当前 exe 同目录
/// 兄弟位（开发形即 target/debug/cascade，发布形同级），缺席回落 PATH 检索。
fn resolve_cascade_bin() -> PathBuf {
    if let Ok(over) = std::env::var("CASCADE_BIN_OVERRIDE") {
        if !over.is_empty() {
            return PathBuf::from(over);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let cand = dir.join("cascade");
            if cand.is_file() {
                return cand;
            }
        }
    }
    PathBuf::from("cascade")
}

fn run_cascade_check(
    _cascade_dir: &Path,
    registry: &Path,
    doc_root: &Path,
    trails: &[String],
    reports_root: &Path,
) -> Result<Value, LErr> {
    // 围堰 uv run --project <cascade-dir> cascade check 已替换为引擎 bin 直调
    // （SPEC-025 A2 零 sih-tools spawn）；--cascade-dir 仅收参不承载选位。
    let mut cmd = Command::new(resolve_cascade_bin());
    cmd.args([
        "check",
        "--registry",
        registry.display().to_string().as_str(),
        "--root",
        doc_root.display().to_string().as_str(),
        "--reports-root",
        reports_root.display().to_string().as_str(),
    ]);
    for t in trails {
        cmd.arg("--trail").arg(t);
    }
    let out = cmd
        .output()
        .map_err(|e| LErr::Tool(format!("cascade invocation failed: {e}")))?;
    if out.status.code() == Some(2) {
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let head: String = stdout.chars().take(200).collect();
        return Err(LErr::Tool(format!("cascade check errored: {head}")));
    }
    serde_json::from_slice(&out.stdout)
        .map_err(|e| LErr::Tool(format!("cascade report unparsable: {e}")))
}

fn do_check(
    path_arg: &str,
    registry: Option<PathBuf>,
    doc_root: Option<PathBuf>,
    trails: Vec<String>,
    cascade_dir: Option<PathBuf>,
    reports_root: Option<PathBuf>,
    root: &Path,
    locks_ledger: &Path,
) -> Result<(Value, i32), LErr> {
    let path = normalize_path(path_arg, Some(root.to_string_lossy().as_ref()));
    if !path.starts_with(DOC_PREFIX) {
        return Err(LErr::Tool(format!(
            "path outside cascade corpus, expect {DOC_PREFIX} prefix: {path}"
        )));
    }
    let doc_rel = &path[DOC_PREFIX.len()..];
    let registry = registry.unwrap_or_else(|| root.join("sih-engine/doc/CASCADE.json"));
    let doc_root = doc_root.unwrap_or_else(|| root.join("sih-engine/doc"));
    let trails = if trails.is_empty() { default_trails(root) } else { trails };
    let cascade_dir = cascade_dir.unwrap_or_else(|| root.join("sih-tools/cascade"));
    let reports_root = reports_root.unwrap_or_else(|| root.join("sih-tools"));
    let report = run_cascade_check(&cascade_dir, &registry, &doc_root, &trails, &reports_root)?;
    let target = report
        .get("targets")
        .and_then(|t| t.get(doc_rel));
    let (verdict, dirty): (String, Vec<Value>) = match target {
        None => ("unknown_target".to_string(), vec![]),
        Some(t) => {
            let dirty = t.get("dirty").and_then(|v| v.as_array()).cloned().unwrap_or_default();
            if dirty.is_empty() {
                ("writable".to_string(), dirty)
            } else {
                ("blocked".to_string(), dirty)
            }
        }
    };
    let line = json!({
        "dirty": dirty,
        "event": "checked",
        "path": path,
        "tool": tool_stamp(),
        "verdict": verdict,
    });
    append_event(locks_ledger, &line)?;
    if verdict == "blocked" {
        return Ok((
            json!({"dirty": dirty, "error": "dirty_upstream", "verdict": verdict}),
            1,
        ));
    }
    Ok((json!({"dirty": dirty, "line": line, "verdict": verdict}), 0))
}

// ---------- CLI 解析 ----------

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: locksview <acquire|release|status|check> [旗标...]");
    exit(2);
}

struct Flags {
    single: BTreeMap<String, String>,
    trails: Vec<String>,
}

fn parse_flags(args: &[String], sub: &str) -> Flags {
    let allowed: &[&str] = match sub {
        "acquire" | "release" => &["path", "identity", "session", "at", "root", "locks", "ledger"],
        "status" => &["path", "session", "root", "locks"],
        "check" => &["path", "registry", "doc-root", "trail", "cascade-dir", "reports-root", "root", "locks"],
        _ => usage_fail(&format!("未知子命令: {sub}")),
    };
    let mut single = BTreeMap::new();
    let mut trails = vec![];
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].clone();
        let name = match a.strip_prefix("--") {
            Some(n) if !n.is_empty() => n.to_string(),
            _ => usage_fail(&format!("未知位置参数: {a}")),
        };
        if !allowed.contains(&name.as_str()) {
            usage_fail(&format!("子命令 {sub} 不接受 --{name}"));
        }
        i += 1;
        if i >= args.len() {
            usage_fail(&format!("--{name} 缺值"));
        }
        if name == "trail" {
            trails.push(args[i].clone());
        } else {
            single.insert(name, args[i].clone());
        }
        i += 1;
    }
    Flags { single, trails }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage_fail("缺子命令");
    }
    let sub = args[0].clone();
    let flags = parse_flags(&args[1..], &sub);
    let get = |k: &str| flags.single.get(k).map(|s| s.as_str());
    let root = PathBuf::from(get("root").unwrap_or("."));
    let locks_ledger = match get("locks") {
        Some(p) => PathBuf::from(p),
        None => root.join("sih-tools/locks/ledger/locks.ndjson"),
    };
    let result = match sub.as_str() {
        "acquire" | "release" => {
            let path_arg = match get("path") {
                Some(p) => p.to_string(),
                None => usage_fail("acquire/release 缺 --path（必填）"),
            };
            let identity = match get("identity") {
                Some(p) => p.to_string(),
                None => usage_fail("acquire/release 缺 --identity（必填）"),
            };
            let worktree_ledger = match get("ledger") {
                Some(p) => PathBuf::from(p),
                None => root.join("sih-tools/worktree/ledger/sessions.ndjson"),
            };
            do_acquire_or_release(
                sub == "acquire",
                &path_arg,
                &identity,
                get("session"),
                get("at"),
                &root,
                &locks_ledger,
                &worktree_ledger,
            )
            .map(|v| (v, 0))
        }
        "status" => do_status(get("path"), get("session"), &root, &locks_ledger)
            .map(|v| (v, 0)),
        "check" => {
            let path_arg = match get("path") {
                Some(p) => p.to_string(),
                None => usage_fail("check 缺 --path（必填）"),
            };
            do_check(
                &path_arg,
                get("registry").map(PathBuf::from),
                get("doc-root").map(PathBuf::from),
                flags.trails.clone(),
                get("cascade-dir").map(PathBuf::from),
                get("reports-root").map(PathBuf::from),
                &root,
                &locks_ledger,
            )
        }
        _ => usage_fail(&format!("未知子命令: {sub}")),
    };
    match result {
        Ok((payload, code)) => emit(&payload, code),
        Err(LErr::Blocked(reason, detail)) => {
            emit(&json!({"detail": detail, "error": reason}), 1)
        }
        Err(LErr::Tool(msg)) => emit(&json!({"error": msg}), 2),
    }
}
