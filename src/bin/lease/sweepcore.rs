//! 租约扫残留（sweep）：五类协调态残留全机械普查三态输出，--fix 自清件径走
//! 既有通道。对表面：围堰 sweepcore.py 455 行（sweepimpl-solo 正典）。
//!
//! 五类：幻影活跃会话（四证取二）与僵尸锁（属主经类一判死）与停滞检验文件
//! （pid 探针四态，会话活跃或持锁即候裁防搁浅）与散位收据（带 closed_at，
//! git 跟踪件只列候裁）与无主工地（零属主且结果档已归档，只列不删）。
//! 落差申报：围堰现势锁面从 sqlite lockdb 投影读，引擎从 ndjson 锁册
//! acquired/released 事件序推导（语义同构）；僵尸锁 fix 围堰走
//! lockdb.takeover_release INSERT，引擎落锁册 released 行（引擎无 sqlite，
//! A9 依赖零新增）。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};

use crate::attachments::detect_domain_context;
use crate::commitlaw::{git, py_resolve};
use crate::{die, emit, now_utc, one};

const SWEEP_REPAIR_MARK: &str = "sweep:phantom-session";
const RESULTS_FACES: &[&str] = &["sih-engine/sih/event/plan", "sih-math/sih/event/plan"];
const ORPHAN_REPOS: &[&str] = &["sih-tools", "sih-engine"];
const HEARTBEAT_STALE_SECONDS: f64 = 300.0;
const CLASS_ORDER: &[&str] = &[
    "phantom_sessions",
    "zombie_locks",
    "stale_check_files",
    "stray_receipts",
    "orphan_worksites",
];

fn read_jsonl(path: &Path) -> Vec<Value> {
    let mut rows = Vec::new();
    if let Ok(text) = std::fs::read_to_string(path) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                rows.push(v);
            }
        }
    }
    rows
}

/// 活跃会话视图（core.active_sessions 对表）：issued 置位、revoked 弹同号。
fn active_sessions(events: &[Value]) -> BTreeMap<String, Value> {
    let mut sessions = BTreeMap::new();
    for e in events {
        match e.get("event").and_then(|v| v.as_str()) {
            Some("issued") => {
                if let (Some(sid), Some(row)) = (
                    e.get("session_id").and_then(|v| v.as_str()),
                    e.as_object(),
                ) {
                    sessions.insert(sid.to_string(), Value::Object(row.clone()));
                }
            }
            Some("revoked") => {
                if let Some(sid) = e.get("session_id").and_then(|v| v.as_str()) {
                    sessions.remove(sid);
                }
            }
            _ => {}
        }
    }
    sessions
}

/// 现势锁面按会话归并（引擎 ndjson 推导形，落差见模块注）。
fn held_by_session(locks_ledger: &Path) -> BTreeMap<String, Vec<(String, String)>> {
    let mut held: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for e in read_jsonl(locks_ledger) {
        match e.get("event").and_then(|v| v.as_str()) {
            Some("acquired" | "reacquired") => {
                if let (Some(sid), Some(p)) = (
                    e.get("session_id").and_then(|v| v.as_str()),
                    e.get("path").and_then(|v| v.as_str()),
                ) {
                    let mode = e.get("mode").and_then(|v| v.as_str()).unwrap_or("exclusive");
                    held.entry(sid.to_string()).or_default().push((p.to_string(), mode.to_string()));
                }
            }
            Some("released") => {
                if let (Some(sid), Some(p)) = (
                    e.get("session_id").and_then(|v| v.as_str()),
                    e.get("path").and_then(|v| v.as_str()),
                ) {
                    if let Some(rows) = held.get_mut(sid) {
                        rows.retain(|(rp, _)| rp != p);
                    }
                }
            }
            _ => {}
        }
    }
    held.retain(|_, rows| !rows.is_empty());
    held
}

fn load_json(path: &Path) -> Value {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .filter(|v| v.is_object())
        .unwrap_or_else(|| json!({}))
}

fn results_archived(root: &Path, stem: &str) -> bool {
    RESULTS_FACES
        .iter()
        .any(|face| root.join(face).join(format!("{stem}-results.md")).is_file())
}

fn worktree_and_branch_absent(session: &Value) -> bool {
    let repos = match session.get("repos").and_then(|v| v.as_array()) {
        Some(r) if !r.is_empty() => r,
        _ => return false,
    };
    for entry in repos {
        let worktree = entry.get("worktree").and_then(|v| v.as_str()).unwrap_or("");
        let branch = entry.get("branch").and_then(|v| v.as_str()).unwrap_or("");
        if !worktree.is_empty() && Path::new(worktree).exists() {
            return false;
        }
        if !branch.is_empty() {
            let repo = entry.get("repo").and_then(|v| v.as_str()).unwrap_or(".");
            let (rc, _, _) = git(Path::new(repo), &["rev-parse", "--verify", "--quiet", &format!("refs/heads/{branch}")]);
            if rc == 0 {
                return false;
            }
        }
    }
    true
}

fn evidence_for_session(sid: &str, session: &Value, events: &[Value], held_by: &BTreeMap<String, Vec<(String, String)>>, root: &Path) -> Value {
    json!({
        "revoked_row_present": events.iter().any(|e| {
            e.get("event").and_then(|v| v.as_str()) == Some("revoked")
                && e.get("session_id").and_then(|v| v.as_str()) == Some(sid)
        }),
        "locks_all_released": !held_by.contains_key(sid),
        "worktree_and_branch_absent": worktree_and_branch_absent(session),
        "results_archived": results_archived(root, session.get("package").and_then(|v| v.as_str()).unwrap_or("")),
    })
}

fn pid_alive(pid: i64) -> Option<bool> {
    // POSIX kill(pid,0) 探活（cli._pid_alive 对表）：ESRCH 死、EPERM 视活、
    // 其余不可知 None 保守判活。
    let r = unsafe { libc::kill(pid as libc::pid_t, 0) };
    if r == 0 {
        Some(true)
    } else {
        match std::io::Error::last_os_error().raw_os_error() {
            Some(libc::ESRCH) => Some(false),
            Some(libc::EPERM) => Some(true),
            _ => None,
        }
    }
}

fn pid_started(pid: i64) -> Option<String> {
    let out = std::process::Command::new("ps")
        .args(["-o", "lstart=", "-p", &pid.to_string()])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

fn parse_ts(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&chrono::Utc))
        .ok()
}

/// 检验文件窗口四态（cli._classify_window 对表）：pid_active/pid_dead/
/// legacy_active/legacy_stale；探针不可知保守判活。
fn classify_window(data: &Value, at: &str) -> (String, Value) {
    let hb = data
        .get("heartbeat_at")
        .and_then(|v| v.as_str())
        .or_else(|| data.get("opened_at").and_then(|v| v.as_str()));
    let age = match (hb, parse_ts(at), parse_ts(hb.unwrap_or(""))) {
        (Some(_), Some(a), Some(h)) => Some((a - h).num_seconds() as f64),
        _ => None,
    };
    let pid = data.get("pid").and_then(|v| v.as_i64());
    let Some(pid) = pid else {
        let mut base = json!({
            "basis": "heartbeat-fallback",
            "package": data.get("package"),
            "heartbeat_at": hb,
            "window_session": data.get("session_id"),
        });
        if let Some(a) = age {
            if a <= HEARTBEAT_STALE_SECONDS {
                return ("legacy_active".into(), base);
            }
            base["stale_seconds"] = json!(a);
        }
        return ("legacy_stale".into(), base);
    };
    let alive = pid_alive(pid);
    let started = pid_started(pid);
    let rec_started = data.get("pid_started").and_then(|v| v.as_str());
    let start_match = match (rec_started, &started) {
        (Some(r), Some(s)) => Some(r == s.as_str()),
        _ => None,
    };
    let mut detail = json!({
        "basis": "pid-probe",
        "pid": pid,
        "pid_alive": alive,
        "pid_started_recorded": rec_started,
        "pid_started_probe": started,
        "start_match": start_match,
        "heartbeat_at": hb,
        "stale_seconds": age,
        "window_session": data.get("session_id"),
    });
    if alive == Some(false) {
        detail["verdict_reason"] = json!("process-gone");
        return ("pid_dead".into(), detail);
    }
    if start_match == Some(false) {
        detail["verdict_reason"] = json!("start-time-mismatch-pid-reuse");
        return ("pid_dead".into(), detail);
    }
    if alive.is_none() {
        detail["verdict_reason"] = json!("probe-unavailable-conservative-alive");
    }
    ("pid_active".into(), detail)
}

fn default_branch(repo: &Path) -> String {
    let (rc, _, _) = git(repo, &["rev-parse", "--verify", "--quiet", "refs/heads/main"]);
    if rc == 0 {
        return "main".into();
    }
    let (rc, out, _) = git(repo, &["rev-parse", "--abbrev-ref", "HEAD"]);
    if rc == 0 && !out.trim().is_empty() {
        out.trim().to_string()
    } else {
        "main".into()
    }
}

fn empty_classes() -> Map<String, Value> {
    let mut m = Map::new();
    for name in CLASS_ORDER {
        m.insert((*name).to_string(), json!([]));
    }
    m
}

/// 五类普查（sweepcore.scan 对表）：只读零副作用，返回三态报告。
pub(crate) fn scan(root: &Path, session_ledger: &Path, locks_ledger: &Path, at: &str) -> Value {
    let root = py_resolve(&root.to_path_buf());
    let events = read_jsonl(session_ledger);
    let actives = active_sessions(&events);
    let held_by = held_by_session(locks_ledger);
    let mut classes = empty_classes();

    // 类一：幻影活跃会话（四证取二即自清件；证据不足其二零触碰不列报）。
    for (sid, session) in &actives {
        let evidence = evidence_for_session(sid, session, &events, &held_by, &root);
        let count = evidence.as_object().unwrap().values().filter(|v| v.as_bool() == Some(true)).count();
        if count >= 2 {
            classes["phantom_sessions"].as_array_mut().unwrap().push(json!({
                "class": "phantom_session",
                "object": sid,
                "state": "auto_clean",
                "criteria": format!("四证取二（{count}/4）"),
                "channel": "ledger-repair",
                "detail": {"package": session.get("package"), "evidence": evidence, "evidence_count": count},
            }));
        }
    }

    // 类二：僵尸锁（属主会话已被类一判死即自清件；属主活即零触碰不列报）。
    let phantom_sids: Vec<String> = classes["phantom_sessions"]
        .as_array().unwrap()
        .iter()
        .map(|i| i["object"].as_str().unwrap_or("").to_string())
        .collect();
    for sid in &phantom_sids {
        if let Some(locks) = held_by.get(sid) {
            for (path, mode) in locks {
                classes["zombie_locks"].as_array_mut().unwrap().push(json!({
                    "class": "zombie_lock",
                    "object": format!("{sid}:{path}"),
                    "state": "auto_clean",
                    "criteria": "属主会话经类一判死",
                    "channel": "takeover",
                    "detail": {"session_id": sid, "path": path, "mode": mode},
                }));
            }
        }
    }

    // 类三：停滞检验文件（pid 探针四态；会话活跃或持锁即候裁防搁浅）。
    let checks = root.join("sih-tools/lease/ledger/checks");
    if checks.is_dir() {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(&checks)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().map(|e| e == "json").unwrap_or(false))
            .collect();
        entries.sort();
        for cf in entries {
            let data = load_json(&cf);
            let (state, detail) = classify_window(&data, at);
            if state != "pid_dead" && state != "legacy_stale" {
                continue;
            }
            let window_session = data.get("session_id").and_then(|v| v.as_str()).unwrap_or("");
            let session_active = !window_session.is_empty() && actives.contains_key(window_session);
            let session_holds = !window_session.is_empty() && held_by.contains_key(window_session);
            let mut item = json!({
                "class": "stale_check_file",
                "object": cf.display().to_string(),
                "state": "auto_clean",
                "criteria": format!("pid 探针四态={state}"),
                "channel": "takeover",
                "detail": {"window_state": state, "window_session": window_session, "probe": detail},
            });
            if session_active || session_holds {
                item["state"] = json!("awaiting");
                item["criteria"] = json!(format!(
                    "pid 探针四态={state} 而窗会话在册活跃或仍持锁（防搁浅守卫）"
                ));
                item["channel"] = json!("human-takeover");
            }
            classes["stale_check_files"].as_array_mut().unwrap().push(item);
        }
    }

    // 类四：散位收据（带 closed_at 即存量；git 跟踪件只列候裁指令）。
    for rel_dir in ["sih-engine/sih/state/plan", "sih-math/sih/event/plan"] {
        let src_dir = root.join(rel_dir);
        if !src_dir.is_dir() {
            continue;
        }
        let repo_root = root.join(rel_dir.split('/').next().unwrap_or("sih-engine"));
        let mut srcs: Vec<PathBuf> = std::fs::read_dir(&src_dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.file_name().map(|n| n.to_string_lossy().ends_with(".lease-check.json")).unwrap_or(false))
            .collect();
        srcs.sort();
        for src in srcs {
            let data = load_json(&src);
            let closed_at = data.get("closed_at").and_then(|v| v.as_str()).unwrap_or("");
            if closed_at.is_empty() {
                continue;
            }
            let stem = src
                .file_name()
                .map(|n| n.to_string_lossy().trim_end_matches(".lease-check.json").to_string())
                .unwrap_or_default();
            let rel = src.strip_prefix(&repo_root).map(|p| p.display().to_string()).unwrap_or_default();
            let (rc, _, _) = git(&repo_root, &["ls-files", "--error-unmatch", &rel]);
            let tracked = rc == 0;
            classes["stray_receipts"].as_array_mut().unwrap().push(json!({
                "class": "stray_receipt",
                "object": src.display().to_string(),
                "state": if tracked { "awaiting" } else { "auto_clean" },
                "criteria": format!("散位收据带 closed_at（git {}）", if tracked { "跟踪" } else { "未跟踪" }),
                "channel": if tracked { "instruction-only" } else { "receipts-home" },
                "detail": {
                    "stem": stem, "closed_at": closed_at, "tracked": tracked,
                    "home": root.join("sih-tools/lease/ledger/receipts").join(format!("{stem}.json")).display().to_string(),
                },
            }));
        }
    }

    // 类五：无主工地（msh 分支与工地目录项对活跃名册零属主且结果档已归档；
    // 只列清单零删除）。
    for repo_name in ORPHAN_REPOS {
        let repo = root.join(repo_name);
        let mut stems: BTreeMap<String, (Option<String>, Option<String>)> = BTreeMap::new();
        let (rc, out, _) = git(&repo, &["branch", "--format", "%(refname:short)"]);
        if rc == 0 {
            for line in out.lines() {
                let branch = line.trim();
                if let Some(stem) = branch.strip_prefix("msh/") {
                    stems.entry(stem.to_string()).or_insert((Some(branch.to_string()), None));
                }
            }
        }
        let wt_root = root.join("worktrees").join(repo_name);
        if wt_root.is_dir() {
            let mut children: Vec<PathBuf> = std::fs::read_dir(&wt_root)
                .into_iter().flatten().flatten().map(|e| e.path()).collect();
            children.sort();
            for child in children {
                if child.is_dir() {
                    if let Some(name) = child.file_name().map(|n| n.to_string_lossy().to_string()) {
                        if !name.starts_with('.') {
                            let e = stems.entry(name).or_insert((None, None));
                            e.1 = Some(child.display().to_string());
                        }
                    }
                }
            }
        }
        for (stem, (branch, worktree_dir)) in &stems {
            let owned = actives.values().any(|session| {
                session.get("package").and_then(|v| v.as_str()) == Some(stem.as_str())
                    || session.get("repos").and_then(|v| v.as_array()).map(|rs| {
                        rs.iter().any(|r| r.get("branch").and_then(|v| v.as_str()) == Some(&format!("msh/{stem}")))
                    }).unwrap_or(false)
            });
            if owned || !results_archived(&root, stem) {
                continue;
            }
            let mut unmerged: Option<i64> = None;
            if let Some(b) = branch {
                let base = default_branch(&repo);
                let (rc_c, out_c, _) = git(&repo, &["rev-list", "--count", &format!("{base}..{b}")]);
                if rc_c == 0 {
                    unmerged = out_c.trim().parse::<i64>().ok();
                }
            }
            let (state, channel) = if unmerged.unwrap_or(0) > 0 {
                ("awaiting", "list-only")
            } else {
                ("info", "list-only")
            };
            classes["orphan_worksites"].as_array_mut().unwrap().push(json!({
                "class": "orphan_worksite",
                "object": if branch.is_some() { format!("{repo_name}:msh/{stem}") } else { format!("{repo_name}:{stem}") },
                "state": state,
                "criteria": "零属主且结果档已归档（git 写不属本工具只列不删）",
                "channel": channel,
                "detail": {"repo": repo_name, "stem": stem, "branch": branch,
                           "worktree_dir": worktree_dir, "unmerged_commits": unmerged},
            }));
        }
    }

    let mut auto_n = 0usize;
    let mut await_n = 0usize;
    let mut info_n = 0usize;
    for name in CLASS_ORDER {
        for item in classes[*name].as_array().unwrap() {
            match item["state"].as_str().unwrap_or("") {
                "auto_clean" => auto_n += 1,
                "awaiting" => await_n += 1,
                "info" => info_n += 1,
                _ => {}
            }
        }
    }
    json!({
        "command": "sweep",
        "at": at,
        "root": root.display().to_string(),
        "classes": Value::Object(classes),
        "summary": {"auto_cleanable": auto_n, "awaiting": await_n, "info": info_n,
                    "evaluated_active_sessions": actives.len()},
        "verdict": if auto_n == 0 && await_n == 0 { "clean" } else { "residue" },
    })
}

fn fix_phantom(item: &Value, session_ledger: &Path, at: &str) -> Value {
    // 修正性 revoked 终笔（ledger-repair 通道对表）：repair 标记显式非伪装原笔。
    let detail = &item["detail"];
    let evidence = detail["evidence"].as_object().unwrap();
    let names: Vec<&str> = evidence.iter().filter(|(_, v)| v.as_bool() == Some(true)).map(|(k, _)| k.as_str()).collect();
    let package = detail["package"].as_str().unwrap_or("");
    let reason = format!(
        "{SWEEP_REPAIR_MARK} evidence={}; 令源=用户2026-09-07残留直清令与sweepimpl-solo任务包§2.2; package={package}",
        names.join("+")
    );
    let row = json!({
        "event": "revoked",
        "package": package,
        "reason": reason,
        "removed": [],
        "revoked_at": at,
        "session_id": item["object"],
    });
    match std::fs::OpenOptions::new().create(true).append(true).open(session_ledger) {
        Ok(mut f) => {
            use std::io::Write;
            let line = serde_json::to_string(&row).unwrap_or_default();
            match writeln!(f, "{line}") {
                Ok(_) => json!({"result": "repaired", "channel_output": {"repaired": 1}}),
                Err(e) => json!({"result": "failed", "error": e.to_string()}),
            }
        }
        Err(e) => json!({"result": "failed", "error": e.to_string()}),
    }
}

fn fix_zombie(item: &Value, locks_ledger: &Path, at: &str) -> Value {
    // 僵尸锁释放（takeover 通道对表）：落锁册 released 行留痕（引擎无 sqlite
    // 投影，落差已在模块注申报）。
    let detail = &item["detail"];
    let row = json!({
        "event": "released",
        "path": detail["path"],
        "session_id": detail["session_id"],
        "mode": detail["mode"],
        "released_at": at,
        "reason": "sweep 扫残留僵尸锁释放（属主会话经类一判死）",
    });
    match std::fs::OpenOptions::new().create(true).append(true).open(locks_ledger) {
        Ok(mut f) => {
            use std::io::Write;
            let line = serde_json::to_string(&row).unwrap_or_default();
            match writeln!(f, "{line}") {
                Ok(_) => json!({"result": "released", "released": 1}),
                Err(e) => json!({"result": "failed", "error": e.to_string()}),
            }
        }
        Err(e) => json!({"result": "failed", "error": e.to_string()}),
    }
}

fn fix_stale_check(item: &Value, locks_ledger: &Path, at: &str) -> Value {
    // 停滞检验文件清除（takeover 通道对表）：删活件加锁释放，输出留档。
    let path = PathBuf::from(item["object"].as_str().unwrap_or(""));
    let mut outcome = json!({});
    match std::fs::remove_file(&path) {
        Ok(_) => outcome["cleared"] = json!(path.display().to_string()),
        Err(e) => return json!({"result": "failed", "error": e.to_string()}),
    }
    if let Some(sid) = item["detail"]["window_session"].as_str() {
        if !sid.is_empty() {
            let held = held_by_session(locks_ledger);
            if held.contains_key(sid) {
                let row = json!({
                    "event": "released",
                    "path": held[sid].iter().map(|(p, _)| json!(p)).collect::<Vec<_>>(),
                    "session_id": sid,
                    "released_at": at,
                    "reason": "sweep 扫残留停滞检验文件清除",
                });
                if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(locks_ledger) {
                    use std::io::Write;
                    let _ = writeln!(f, "{}", serde_json::to_string(&row).unwrap_or_default());
                }
            }
        }
    }
    json!({"result": "cleared", "channel_output": outcome})
}

fn fix_stray_receipt(item: &Value) -> Value {
    // 收据归家：原子迁入 receipts 正典位（幂等让位）。
    let src = PathBuf::from(item["object"].as_str().unwrap_or(""));
    let dest = PathBuf::from(item["detail"]["home"].as_str().unwrap_or(""));
    if dest.exists() {
        return json!({"result": "skipped", "detail": {"reason": "destination exists", "home": dest.display().to_string()}});
    }
    if let Some(p) = dest.parent() {
        let _ = std::fs::create_dir_all(p);
    }
    match std::fs::rename(&src, &dest) {
        Ok(_) => json!({"result": "moved", "detail": {"from": src.display().to_string(), "to": dest.display().to_string()}}),
        Err(e) => json!({"result": "failed", "error": e.to_string()}),
    }
}

/// 普查入口（sweepcore.run 对表）：fix 即先普查再逐自清件走既有通道清除并
/// 复普查定退出码；净即零，有残留即一。
pub(crate) fn run(root: &Path, session_ledger: &Path, locks_ledger: &Path, fix: bool, at: &str) -> Value {
    let root = &detect_domain_context(Some(root.to_str().unwrap_or(".")));
    let mut report = scan(root, session_ledger, locks_ledger, at);
    let mut actions: Vec<Value> = Vec::new();
    if fix {
        for name in CLASS_ORDER {
            let items: Vec<Value> = report["classes"][name].as_array().cloned().unwrap_or_default();
            for item in &items {
                if item["state"].as_str() != Some("auto_clean") {
                    continue;
                }
                let outcome = match item["class"].as_str().unwrap_or("") {
                    "phantom_session" => fix_phantom(item, session_ledger, at),
                    "zombie_lock" => fix_zombie(item, locks_ledger, at),
                    "stale_check_file" => fix_stale_check(item, locks_ledger, at),
                    "stray_receipt" => fix_stray_receipt(item),
                    _ => continue,
                };
                actions.push(json!({
                    "class": item["class"],
                    "object": item["object"],
                    "criteria": item["criteria"],
                    "channel": item["channel"],
                    "result": outcome.get("result").cloned().unwrap_or(json!("failed")),
                    "channel_output": outcome.get("channel_output").cloned().unwrap_or(Value::Null),
                    "error": outcome.get("error").cloned().unwrap_or(Value::Null),
                }));
            }
        }
        report["actions"] = json!(actions);
        let post = scan(root, session_ledger, locks_ledger, at);
        report["post_fix"] = json!({
            "summary": post["summary"], "verdict": post["verdict"], "classes": post["classes"],
        });
        report["exit_code"] = json!(if post["verdict"].as_str() == Some("clean") { 0 } else { 1 });
    } else {
        report["exit_code"] = json!(if report["verdict"].as_str() == Some("clean") { 0 } else { 1 });
    }
    report
}

fn render_lines(report: &Value) -> Vec<String> {
    let mut lines = vec![format!(
        "sweep: root={} at={} evaluated_active_sessions={}",
        report["root"].as_str().unwrap_or(""),
        report["at"].as_str().unwrap_or(""),
        report["summary"]["evaluated_active_sessions"],
    )];
    for name in CLASS_ORDER {
        for item in report["classes"][name].as_array().unwrap_or(&Vec::new()) {
            lines.push(format!(
                "[{}] {} {} criteria={} channel={}",
                item["state"].as_str().unwrap_or(""),
                item["class"].as_str().unwrap_or(""),
                item["object"].as_str().unwrap_or(""),
                item["criteria"].as_str().unwrap_or(""),
                item["channel"].as_str().unwrap_or(""),
            ));
        }
    }
    for action in report["actions"].as_array().unwrap_or(&Vec::new()) {
        lines.push(format!(
            "fix: {} {} via {} criteria={} -> {}",
            action["class"].as_str().unwrap_or(""),
            action["object"].as_str().unwrap_or(""),
            action["channel"].as_str().unwrap_or(""),
            action["criteria"].as_str().unwrap_or(""),
            action["result"].as_str().unwrap_or(""),
        ));
    }
    if let Some(tail) = report.get("post_fix") {
        lines.push(format!(
            "post_fix: verdict={} auto={} awaiting={} info={}",
            tail["verdict"].as_str().unwrap_or(""),
            tail["summary"]["auto_cleanable"],
            tail["summary"]["awaiting"],
            tail["summary"]["info"],
        ));
    }
    lines.push(format!(
        "verdict: {} auto={} awaiting={} info={} exit={}",
        report["verdict"].as_str().unwrap_or(""),
        report["summary"]["auto_cleanable"],
        report["summary"]["awaiting"],
        report["summary"]["info"],
        report["exit_code"],
    ));
    lines
}

pub(crate) fn cmd_sweep(m: &BTreeMap<String, Vec<String>>) -> ! {
    let root_flag = one(m, "root").map(|s| s.to_string()).filter(|s| !s.is_empty());
    let root = detect_domain_context(root_flag.as_deref());
    let session_ledger = PathBuf::from(one(m, "ledger").unwrap_or(""));
    let locks_ledger = PathBuf::from(one(m, "locks").unwrap_or(""));
    if session_ledger.as_os_str().is_empty() || locks_ledger.as_os_str().is_empty() {
        die(2, "sweep 缺 --ledger 与 --locks 显式传参", json!(null));
    }
    let at = one(m, "at").unwrap_or(&now_utc()).to_string();
    let fix = m.contains_key("fix");
    let report = run(&root, &session_ledger, &locks_ledger, fix, &at);
    if m.contains_key("quiet") {
        for line in render_lines(&report) {
            println!("{line}");
        }
    } else {
        print!("{}", emit(&report));
    }
    std::process::exit(report["exit_code"].as_i64().unwrap_or(1) as i32);
}
