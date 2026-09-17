//! 收约执法面：commit 四验、bypass 留痕、reconcile 三方对表、gauge 摘要（SPEC-024
//! 腿二，lease-commitlaw-parallel 批）。融回基准权威 sih-tools/lease/src/lease/
//! commitcore.py 与 cli.py 接线，行为对等判据 SPEC-024 A2/A4 与金向量同参形条款。

use crate::{append_row, emit, now_utc, one, read_jsonl, tool};
use regex::Regex;
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

// 内置封线与追认表逐字节承 commitcore.py SEAL_BASES 与 SEAL_EXEMPTS，表变更须升版本
const SEAL_BASES: [(&str, &str); 2] = [("sih-engine", "4146851"), ("sih-tools", "91d14d4f")];
const SEAL_EXEMPTS_ENGINE: [&str; 3] = [
    "dc31d77729e27f85ed7d22db89357a929d2370de",
    "1c86f5d3e0755cfb15db23475ee684ceb6914e7b",
    "86ce450520f38ebff0e85dd149328c6270600071",
];
const SEAL_EXEMPTS_TOOLS: [&str; 18] = [
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
];

pub(crate) fn fail(rc: i32, payload: Value) -> ! {
    print!("{}", emit(&payload));
    std::process::exit(rc);
}

pub(crate) fn py_resolve(p: &Path) -> PathBuf {
    match std::fs::canonicalize(p) {
        Ok(v) => v,
        Err(_) => {
            if p.is_absolute() {
                p.to_path_buf()
            } else {
                std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join(p)
            }
        }
    }
}

pub(crate) fn git(repo: &Path, args: &[&str]) -> (i32, String, String) {
    match Command::new("git").arg("-C").arg(repo).args(args).output() {
        Ok(o) => (
            o.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string(),
        ),
        Err(e) => (-1, String::new(), e.to_string()),
    }
}

pub(crate) fn normalize_scope_path(p: &str) -> String {
    let mut s = p.trim().to_string();
    while s.starts_with("./") {
        s = s[2..].to_string();
    }
    if s != "/" {
        s = s.trim_end_matches('/').to_string();
    }
    s
}

pub(crate) fn scope_allows(allow: &[Value], path: &str) -> bool {
    let target = normalize_scope_path(path);
    for entry in allow {
        let e = normalize_scope_path(entry.as_str().unwrap_or(""));
        if e.is_empty() || e == "." {
            return true;
        }
        if target == e || target.starts_with(&format!("{}/", e)) {
            return true;
        }
    }
    false
}

pub(crate) fn default_trails(root: &Path) -> Vec<PathBuf> {
    let homes = [
        root.join("sih-engine/sih/event/trail"),
        root.join("sih-tools/scribe/trail"),
        root.join("sih/event/trail"),
    ];
    let mut seen: BTreeSet<PathBuf> = BTreeSet::new();
    let mut out = Vec::new();
    for home in homes {
        if !home.is_dir() {
            continue;
        }
        let mut entries: Vec<PathBuf> = std::fs::read_dir(&home)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().map(|x| x == "ndjson").unwrap_or(false))
            .collect();
        entries.sort();
        for p in entries {
            let key = py_resolve(&p);
            if seen.insert(key.clone()) {
                out.push(key);
            }
        }
    }
    out
}

fn trail_hashes_of(trails: &[PathBuf], all_events: bool) -> BTreeSet<String> {
    let mut hashes = BTreeSet::new();
    for trail in trails {
        for event in read_jsonl(trail) {
            if !all_events
                && event.get("event_type").and_then(|v| v.as_str()) != Some("certification_completed")
            {
                continue;
            }
            if let Some(h) = event.get("event_hash").and_then(|v| v.as_str()) {
                if !h.is_empty() {
                    hashes.insert(h.to_string());
                }
            }
        }
    }
    hashes
}

fn cert_on_chain(cert: &str, hashes: &BTreeSet<String>) -> bool {
    hashes.iter().any(|h| h.starts_with(cert))
}

fn dedup_subject_prefix(stem: &str, subject: &str) -> String {
    if stem.is_empty() || subject.is_empty() {
        return subject.to_string();
    }
    let esc = regex::escape(stem);
    let pats = [
        Regex::new(&format!("^{} wip\\s+", esc)).unwrap(),
        Regex::new(&format!("^{} 段\\d+\\s+", esc)).unwrap(),
        Regex::new(&format!("^{}\\s+", esc)).unwrap(),
    ];
    let mut text = subject.to_string();
    loop {
        let mut changed = false;
        for p in &pats {
            let stripped = p.replace(&text, "").to_string();
            if stripped != text {
                text = stripped;
                changed = true;
                break;
            }
        }
        if !changed {
            break;
        }
    }
    text
}

pub(crate) fn build_message(
    stage: &str,
    stem: &str,
    session_id: &str,
    subject: &str,
    seq: Option<i64>,
    cert: Option<&str>,
    base: &str,
    note: Option<&str>,
) -> Result<String, String> {
    let subject = dedup_subject_prefix(stem, subject);
    match stage {
        "wip" => {
            let mut parts = vec![
                format!("{} wip {}", stem, subject),
                String::new(),
                format!("session: {}", session_id),
            ];
            if let Some(n) = note {
                if !n.is_empty() {
                    parts.push(String::new());
                    parts.push(n.to_string());
                }
            }
            Ok(format!("{}\n", parts.join("\n")))
        }
        "settle" => {
            let seq = seq.ok_or_else(|| "settle requires --seq".to_string())?;
            let head = format!("{} 段{} {}", stem, seq, subject);
            let meta = format!(
                "session: {} cert: {} base: {}",
                session_id,
                cert.unwrap_or(""),
                base
            );
            let mut parts = vec![head, String::new(), meta];
            if let Some(n) = note {
                if !n.is_empty() {
                    parts.push(String::new());
                    parts.push(n.to_string());
                }
            }
            Ok(format!("{}\n", parts.join("\n")))
        }
        other => Err(format!("unknown stage: {}", other)),
    }
}

fn base_label(entry: &Value, repo_path: &Path) -> String {
    let base_branch = entry.get("base_branch").and_then(|v| v.as_str()).unwrap_or("");
    if base_branch.is_empty() {
        return String::new();
    }
    let (rc, out, _) = git(repo_path, &["merge-base", "HEAD", base_branch]);
    if rc != 0 {
        return String::new();
    }
    let mb = out.trim();
    let (rc2, out2, _) = git(repo_path, &["rev-parse", "--short", mb]);
    if rc2 != 0 {
        String::new()
    } else {
        format!("{}@{}", base_branch, out2.trim())
    }
}

pub(crate) fn gauge_summary(root: &Path) -> Value {
    let at_full = now_utc();
    let at: String = at_full.chars().take(10).collect();
    let cli = root.join("sih-tools/gauge/src/gauge/cli.py");
    let trail_dir = root.join("sih-engine/sih/event/trail");
    let ledger = root.join("sih-tools/lease/ledger/sessions.ndjson");
    let mut trails: Vec<PathBuf> = Vec::new();
    if trail_dir.is_dir() {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(&trail_dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().map(|x| x == "ndjson").unwrap_or(false))
            .collect();
        entries.sort();
        trails = entries;
    }
    let mut missing: Vec<String> = [
        cli.clone(),
        ledger.clone(),
        root.join("sih-engine"),
        root.join("sih-tools"),
    ]
    .iter()
    .filter(|p| !p.exists())
    .map(|p| p.display().to_string())
    .collect();
    if trails.is_empty() {
        missing.push(trail_dir.display().to_string());
    }
    if !missing.is_empty() {
        return json!({"missing": missing, "reason": "missing", "skipped": true});
    }
    let mut values = Map::new();
    let mut formula = String::new();
    for dim in ["convergence", "adoption", "mergeback"] {
        let mut cmd = Command::new("python3");
        cmd.arg(&cli)
            .args(["read", "--dimension", dim, "--at", &at, "--window-days", "30"]);
        for t in &trails {
            cmd.arg("--trail").arg(t);
        }
        cmd.arg("--sessions-ledger").arg(&ledger);
        cmd.arg("--src-root").arg(root.join("sih-engine"));
        cmd.arg("--tools-root").arg(root.join("sih-tools"));
        let out = match cmd.output() {
            Ok(o) => o,
            Err(e) => {
                return json!({"skipped": true, "reason": format!("unavailable: {}", e)})
            }
        };
        if !out.status.success() {
            let detail = if out.stderr.is_empty() {
                String::from_utf8_lossy(&out.stdout).trim().to_string()
            } else {
                String::from_utf8_lossy(&out.stderr).trim().to_string()
            };
            let detail: String = detail.chars().take(200).collect();
            return json!({
                "detail": detail,
                "dimension": dim,
                "reason": "gauge_error",
                "skipped": true,
            });
        }
        let parsed: Value = match serde_json::from_slice(&out.stdout) {
            Ok(v) => v,
            Err(e) => return json!({"skipped": true, "reason": format!("unavailable: {}", e)}),
        };
        match (
            parsed.pointer("/reading/formula_version").and_then(|v| v.as_str()),
            parsed.pointer("/reading/value"),
            parsed.get("sequence"),
        ) {
            (Some(fv), Some(val), Some(seqv)) => {
                formula = fv.to_string();
                values.insert(
                    dim.to_string(),
                    json!({"sequence": seqv, "value": val}),
                );
            }
            _ => return json!({"skipped": true, "reason": "unavailable: KeyError"}),
        }
    }
    json!({
        "at": at,
        "formula_version": formula,
        "values": Value::Object(values),
        "window_days": 30,
    })
}

fn repo_name_parts_hit(name: &str, repo: &Path) -> bool {
    py_resolve(repo)
        .components()
        .any(|c| c.as_os_str().to_string_lossy() == name)
}

fn seal_exempt_set(repo: &Path) -> Vec<String> {
    for name in ["sih-engine", "sih-tools"] {
        if repo_name_parts_hit(name, repo) {
            return if name == "sih-engine" {
                SEAL_EXEMPTS_ENGINE.iter().map(|s| s.to_string()).collect()
            } else {
                SEAL_EXEMPTS_TOOLS.iter().map(|s| s.to_string()).collect()
            };
        }
    }
    Vec::new()
}

fn resolve_default_base(repo: &Path, base: Option<&str>) -> (Option<String>, Option<String>) {
    if let Some(b) = base {
        return (Some(b.to_string()), None);
    }
    let mut seal: Option<&str> = None;
    for (name, boundary) in SEAL_BASES {
        if repo_name_parts_hit(name, repo) {
            seal = Some(boundary);
            break;
        }
    }
    let seal = match seal {
        Some(s) => s,
        None => return (None, None),
    };
    let (rc, _, _) = git(repo, &["cat-file", "-e", seal]);
    if rc != 0 {
        return (None, None);
    }
    (Some(format!("{}^", seal)), Some(seal.to_string()))
}

pub(crate) fn cmd_commit(m: &BTreeMap<String, Vec<String>>) {
    let root = py_resolve(Path::new(one(m, "root").unwrap_or(".")));
    let stage = one(m, "stage").unwrap_or("").to_string();
    if stage != "wip" && stage != "settle" {
        fail(
            2,
            json!({"error": format!(
                "argument --stage: invalid choice: '{}' (choose from 'wip', 'settle')",
                stage
            )}),
        );
    }
    let seq = one(m, "seq").and_then(|s| s.parse::<i64>().ok());
    if stage == "settle" && seq.is_none() {
        fail(2, json!({"error": "settle requires --seq"}));
    }
    let repo_arg = one(m, "repo").unwrap_or("").to_string();
    let subject = one(m, "subject").unwrap_or("").to_string();
    let cert = one(m, "cert").map(|s| s.to_string());
    let note = one(m, "note").map(|s| s.to_string());
    let sid_opt = one(m, "session").map(|s| s.to_string());
    let ledger = match one(m, "ledger") {
        Some(l) => PathBuf::from(l),
        None => root.join("sih-tools/lease/ledger/sessions.ndjson"),
    };
    let trails: Vec<PathBuf> = match m.get("trail") {
        Some(v) => v.iter().map(PathBuf::from).collect(),
        None => default_trails(&root),
    };

    let actives = crate::active_sessions(&ledger);
    let session = match &sid_opt {
        Some(sid) => actives
            .iter()
            .find(|(s, _)| s == sid)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| {
                fail(
                    1,
                    json!({"detail": {"wanted": sid}, "error": "session_not_active"}),
                )
            }),
        None => {
            if actives.is_empty() {
                fail(1, json!({"detail": null, "error": "no_active_session"}));
            }
            if actives.len() > 1 {
                fail(
                    2,
                    json!({"error": "multiple active sessions, --session required"}),
                );
            }
            actives[0].1.clone()
        }
    };

    let repo_path = py_resolve(Path::new(&repo_arg));
    let repo_str = repo_path.display().to_string();
    let repos = session
        .get("repos")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let entry = repos
        .iter()
        .find(|e| e.get("repo").and_then(|v| v.as_str()) == Some(repo_str.as_str()))
        .or_else(|| {
            repos
                .iter()
                .find(|e| e.get("worktree").and_then(|v| v.as_str()) == Some(repo_str.as_str()))
        })
        .cloned()
        .unwrap_or_else(|| {
            let empty: Value = Value::Null;
            fail(
                1,
                json!({"detail": {
                    "repo": repo_str,
                    "repos": repos.iter().map(|e| e.get("repo").unwrap_or(&empty).clone()).collect::<Vec<_>>(),
                    "worktrees": repos.iter().map(|e| e.get("worktree").unwrap_or(&empty).clone()).collect::<Vec<_>>(),
                }, "error": "repo_not_in_session"}),
            )
        });
    let worktree_str = entry
        .get("worktree")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if worktree_str != repo_str {
        fail(
            1,
            json!({"detail": {"repo": repo_str, "worktree": worktree_str},
                   "error": "commit_must_target_worktree"}),
        );
    }
    let (rc, out, err) = git(
        &repo_path,
        &["-c", "core.quotepath=off", "diff", "--cached", "--name-only"],
    );
    if rc != 0 {
        fail(2, json!({"error": format!("git diff failed: {}", err.trim())}));
    }
    let staged_files: Vec<String> = out
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect();
    if staged_files.is_empty() {
        fail(1, json!({"detail": null, "error": "nothing_staged"}));
    }
    let entry_repo = entry
        .get("repo")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let repo_rel = match Path::new(&entry_repo).strip_prefix(&root) {
        Ok(r) => r.to_path_buf(),
        Err(_) => fail(
            2,
            json!({"error": format!("repo outside workspace root: {}", entry_repo)}),
        ),
    };
    let rel_prefix = if repo_rel.as_os_str() == "." {
        String::new()
    } else {
        format!("{}/", repo_rel.display())
    };
    let allow: Vec<Value> = session
        .get("allow")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let mut outside: Vec<String> = Vec::new();
    for name in &staged_files {
        let checked = format!("{}{}", rel_prefix, name);
        if !scope_allows(&allow, &checked) {
            outside.push(checked);
        }
    }
    if !outside.is_empty() {
        fail(
            1,
            json!({"detail": {"allow": allow, "outside": outside},
                   "error": "staged_out_of_scope"}),
        );
    }
    if stage == "settle" {
        let cert_val = match &cert {
            Some(c) => c.clone(),
            None => fail(2, json!({"error": "settle requires --cert"})),
        };
        let hashes = trail_hashes_of(&trails, false);
        if !cert_on_chain(&cert_val, &hashes) {
            fail(
                1,
                json!({"detail": {
                    "cert": cert_val,
                    "trails": trails.iter().map(|t| t.display().to_string()).collect::<Vec<_>>(),
                }, "error": "cert_not_on_chain"}),
            );
        }
    }
    let base = base_label(&entry, &repo_path);
    let package = session
        .get("package")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let sid = session
        .get("session_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let message = match build_message(
        &stage,
        &package,
        &sid,
        &subject,
        seq,
        cert.as_deref(),
        &base,
        note.as_deref(),
    ) {
        Ok(msg) => msg,
        Err(e) => fail(2, json!({"error": e})),
    };
    let commit_out = Command::new("git")
        .arg("-C")
        .arg(&repo_path)
        .args(["commit", "-m", &message])
        .output();
    match commit_out {
        Ok(o) if o.status.success() => {}
        Ok(o) => {
            let errtxt = String::from_utf8_lossy(&o.stderr).trim().to_string();
            let cut: String = errtxt.chars().take(200).collect();
            fail(1, json!({"error": format!("git commit failed: {}", cut)}));
        }
        Err(e) => fail(1, json!({"error": format!("git commit failed: {}", e)})),
    }
    let (rc3, sha_out, _) = git(&repo_path, &["rev-parse", "--short", "HEAD"]);
    let mut checks = vec![json!("session_active"), json!("staged_in_scope")];
    if stage == "settle" {
        checks.push(json!("cert_on_chain"));
    }
    let mut receipt = json!({
        "base": base,
        "checks": checks,
        "commit": if rc3 == 0 { sha_out.trim().to_string() } else { String::new() },
        "message": message,
        "package": package,
        "session_id": sid,
        "stage": stage,
    });
    if stage == "settle" {
        receipt["gauge"] = gauge_summary(&root);
    }
    print!("{}", emit(&receipt));
}

/// 工作区根上溯发现（pathfix 批）：自起点向上找 sih-engine 与 sih-tools 双目录在位者。
/// 无果即 None，调用位 fail-closed 拒绝，不落 cwd 缺省。
pub(crate) fn discover_workspace_root(start: &Path) -> Option<PathBuf> {
    let mut cur = py_resolve(start);
    loop {
        if cur.join("sih-engine").is_dir() && cur.join("sih-tools").is_dir() {
            return Some(cur);
        }
        if !cur.pop() {
            return None;
        }
    }
}

/// 台账面基座（pathfix 批承 pk-089）：root 下 sih-tools/lease/ledger 在位即中央工作区形，
/// 否则即新城域正典形 sih/ledger。禁止以 root 直拼第一域常量致域树沉积假骨架。
pub(crate) fn ledger_surface(root: &Path) -> PathBuf {
    if root.join("sih-tools/lease/ledger").is_dir() {
        root.join("sih-tools/lease/ledger")
    } else {
        root.join("sih/ledger")
    }
}

pub(crate) fn cmd_bypass(m: &BTreeMap<String, Vec<String>>) {
    let repo_arg = one(m, "repo").unwrap_or("").to_string();
    let sha = one(m, "sha").unwrap_or("").to_string();
    let reason = one(m, "reason").unwrap_or("").to_string();
    if repo_arg.is_empty() || sha.is_empty() || reason.is_empty() {
        fail(
            2,
            json!({"error": "bypass 缺参即拒：--repo 与 --sha 与 --reason 俱必填，防空行污染台账"}),
        );
    }
    let root = match one(m, "root") {
        Some(r) => py_resolve(Path::new(r)),
        None => match discover_workspace_root(Path::new(".")) {
            Some(r) => r,
            None => fail(
                2,
                json!({"error": "bypass 工作区根发现失败：未传 --root 且自 cwd 上溯无 sih-engine 与 sih-tools 双目录在位者，fail-closed 拒落 cwd 缺省"}),
            ),
        },
    };
    let session = one(m, "session").map(|s| s.to_string());
    let at = one(m, "at").map(|s| s.to_string()).unwrap_or_else(now_utc);
    let ledger = match one(m, "bypass-ledger") {
        Some(l) => PathBuf::from(l),
        None => root.join("sih-tools/lease/ledger/bypass.ndjson"),
    };
    let line = json!({
        "at": at,
        "event": "bypassed",
        "reason": reason,
        "repo": py_resolve(Path::new(&repo_arg)).display().to_string(),
        "session": session,
        "sha": sha,
        "tool": tool(),
    });
    append_row(&ledger, &line);
    print!("{}", emit(&json!({"bypassed": line, "ledger": ledger.display().to_string()})));
}

fn known_sessions(ledger: &Path) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut sids = BTreeSet::new();
    let mut packages = BTreeSet::new();
    for event in read_jsonl(ledger) {
        if event.get("event").and_then(|v| v.as_str()) == Some("issued") {
            if let Some(s) = event.get("session_id").and_then(|v| v.as_str()) {
                sids.insert(s.to_string());
            }
            if let Some(p) = event.get("package").and_then(|v| v.as_str()) {
                packages.insert(p.to_string());
            }
        }
    }
    (sids, packages)
}

fn load_bypass_entries(ledger_path: &Path) -> Vec<Value> {
    read_jsonl(ledger_path)
        .into_iter()
        .filter(|e| e.get("event").and_then(|v| v.as_str()) == Some("bypassed"))
        .collect()
}

fn bypass_hit(entries: &[Value], repo: &Path, sha_full: &str) -> bool {
    let repo_resolved = py_resolve(repo).display().to_string();
    for entry in entries {
        let entry_repo = entry.get("repo").and_then(|v| v.as_str()).unwrap_or("");
        if entry_repo.is_empty() {
            continue;
        }
        let entry_resolved = py_resolve(Path::new(entry_repo)).display().to_string();
        if entry_resolved != repo_resolved {
            continue;
        }
        let entry_sha = entry.get("sha").and_then(|v| v.as_str()).unwrap_or("").trim();
        if !entry_sha.is_empty()
            && (sha_full.starts_with(entry_sha) || entry_sha.starts_with(sha_full))
        {
            return true;
        }
    }
    false
}

fn direct_pen_prefix(body: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"\b直改链笔[：:]\s*([0-9a-f]{8})\b").unwrap());
    let caps = re.captures(body)?;
    Some(caps.get(1).unwrap().as_str().to_string())
}

fn direct_pen_on_chain(body: &str, all_hashes: &BTreeSet<String>) -> Option<String> {
    let prefix = direct_pen_prefix(body)?;
    all_hashes
        .iter()
        .find(|h| h.starts_with(&prefix))
        .map(|_| prefix)
}

fn repo_name_hit(entry_repo: &str, repo: &Path) -> bool {
    let entry = entry_repo.trim().trim_end_matches('/');
    if entry.is_empty() {
        return false;
    }
    let repo_resolved = py_resolve(repo).display().to_string();
    if entry == repo_resolved
        || py_resolve(repo)
            .components()
            .any(|c| c.as_os_str().to_string_lossy() == entry)
    {
        return true;
    }
    py_resolve(Path::new(entry)).display().to_string() == repo_resolved
}

fn backfill_bindings(trails: &[PathBuf]) -> Vec<Value> {
    let mut bindings = Vec::new();
    for trail in trails {
        for event in read_jsonl(trail) {
            if event.get("event_type").and_then(|v| v.as_str()) != Some("certification_completed") {
                continue;
            }
            let ch = event.get("details").unwrap_or(&Value::Null);
            let ch = ch.get("content_hashes").unwrap_or(&Value::Null);
            if !ch.is_object() || ch.get("backfill").and_then(|v| v.as_bool()) != Some(true) {
                continue;
            }
            bindings.push(json!({
                "original_event_hash": ch.get("original_event_hash").and_then(|v| v.as_str()).unwrap_or(""),
                "referencing_settle_commits": ch.get("referencing_settle_commits").cloned().unwrap_or(json!([])),
            }));
        }
    }
    bindings
}

fn backfill_binding(
    cert: &str,
    sha_full: &str,
    repo: &Path,
    bindings: &[Value],
) -> Option<&'static str> {
    let mut cert_hit = false;
    let mut sha_claim_only = false;
    for b in bindings {
        let original = b
            .get("original_event_hash")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let cert_ok = !cert.is_empty() && !original.is_empty() && original.starts_with(cert);
        if cert_ok {
            cert_hit = true;
            continue;
        }
        if let Some(refs) = b.get("referencing_settle_commits").and_then(|v| v.as_array()) {
            for r in refs {
                let ref_sha = r.get("sha").and_then(|v| v.as_str()).unwrap_or("");
                if !ref_sha.is_empty()
                    && ref_sha == sha_full
                    && repo_name_hit(r.get("repo").and_then(|v| v.as_str()).unwrap_or(""), repo)
                {
                    sha_claim_only = true;
                }
            }
        }
    }
    if cert_hit {
        Some("cert_prefix")
    } else if sha_claim_only {
        Some("binding_mismatch")
    } else {
        None
    }
}

pub(crate) fn cmd_reconcile(m: &BTreeMap<String, Vec<String>>) {
    let root = py_resolve(Path::new(one(m, "root").unwrap_or(".")));
    let repo_arg = one(m, "repo").unwrap_or("").to_string();
    let repo = py_resolve(Path::new(&repo_arg));
    let ledger = match one(m, "ledger") {
        Some(l) => PathBuf::from(l),
        None => root.join("sih-tools/lease/ledger/sessions.ndjson"),
    };
    let trails: Vec<PathBuf> = match m.get("trail") {
        Some(v) => v.iter().map(PathBuf::from).collect(),
        None => default_trails(&root),
    };
    let base_arg = one(m, "base").map(|s| s.to_string());
    let limit = one(m, "limit").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let bypass_ledger = match one(m, "bypass-ledger") {
        Some(l) => PathBuf::from(l),
        None => root.join("sih-tools/lease/ledger/bypass.ndjson"),
    };

    let (session_ids, packages) = known_sessions(&ledger);
    let trail_hashes = trail_hashes_of(&trails, false);
    let all_event_hashes = trail_hashes_of(&trails, true);
    let bindings = backfill_bindings(&trails);
    let bypass_entries = load_bypass_entries(&bypass_ledger);
    let (base, seal) = resolve_default_base(&repo, base_arg.as_deref());

    let mut log_args: Vec<String> = vec![
        "log".into(),
        "--format=%H%x1f%ad%x1f%B%x1e".into(),
        "--date=short".into(),
    ];
    if let Some(b) = &base {
        log_args.push(format!("{}..HEAD", b));
    }
    if limit != 0 {
        log_args.push("-n".into());
        log_args.push(limit.to_string());
    }
    let log_args_ref: Vec<&str> = log_args.iter().map(|s| s.as_str()).collect();
    let (rc, out, err) = git(&repo, &log_args_ref);
    if rc != 0 {
        fail(2, json!({"error": format!("git log failed: {}", err.trim())}));
    }
    let exempts: BTreeSet<String> = seal_exempt_set(&repo).into_iter().collect();

    let mut records: Vec<Value> = Vec::new();
    for raw in out.split('\x1e') {
        let raw = raw.trim_matches('\n');
        if raw.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = raw.splitn(3, '\x1f').collect();
        if parts.len() != 3 {
            continue;
        }
        let sha = parts[0].trim();
        let date = parts[1].trim();
        let body = parts[2].trim();
        let subject = body.lines().next().unwrap_or("").to_string();

        static SESSION_LINE: OnceLock<Regex> = OnceLock::new();
        let session_re = SESSION_LINE.get_or_init(|| {
            Regex::new(r"(?m)^session: ([0-9a-f]{16})\b").unwrap()
        });
        static CERT_LINE: OnceLock<Regex> = OnceLock::new();
        let cert_re = CERT_LINE.get_or_init(|| Regex::new(r"\bcert: ([0-9a-f]+)").unwrap());

        let session_cap = session_re.captures(body);
        let mut detail: Value = Value::Null;
        let classification: String;
        if exempts.contains(sha) {
            classification = "sealed".into();
        } else if let Some(cap) = session_cap {
            let sid = cap.get(1).unwrap().as_str().to_string();
            let cert_cap = cert_re.captures(body);
            if !session_ids.contains(&sid) {
                classification = "session_orphan".into();
                detail = json!(sid);
            } else if let Some(cc) = cert_cap {
                let cert = cc.get(1).unwrap().as_str().to_string();
                if !cert_on_chain(&cert, &trail_hashes) {
                    let sha_full = sha.to_string();
                    match backfill_binding(&cert, &sha_full, &repo, &bindings) {
                        Some("cert_prefix") => {
                            classification = "cert_backfilled".into();
                            detail = json!(cert);
                        }
                        _ if bypass_hit(&bypass_entries, &repo, &sha_full) => {
                            classification = "cert_writtenoff".into();
                            detail = json!(cert);
                        }
                        Some("binding_mismatch") => {
                            classification = "cert_missing".into();
                            detail = json!(format!("{} binding_mismatch", cert));
                        }
                        _ => {
                            classification = "cert_missing".into();
                            detail = json!(cert);
                        }
                    }
                } else {
                    classification = "routed".into();
                }
            } else {
                classification = "routed".into();
            }
        } else if subject.starts_with("merge: ")
            && packages
                .iter()
                .any(|p| subject == format!("merge: {} 副本归并", p))
        {
            classification = "routed_merge".into();
        } else if let Some(prefix) = direct_pen_on_chain(body, &all_event_hashes) {
            classification = "routed_direct".into();
            detail = json!(prefix);
        } else if bypass_hit(&bypass_entries, &repo, sha) {
            classification = "bypass".into();
        } else {
            classification = "unbypassed".into();
        }
        let sha7: String = sha.chars().take(7).collect();
        let subj80: String = subject.chars().take(80).collect();
        records.push(json!({
            "class": classification,
            "date": date,
            "detail": detail,
            "sha": sha7,
            "subject": subj80,
        }));
    }

    let count = |cls: &[&str]| -> usize {
        records
            .iter()
            .filter(|r| {
                cls.iter()
                    .any(|c| r.get("class").and_then(|v| v.as_str()) == Some(*c))
            })
            .count()
    };
    let summary = json!({
        "bypass": count(&["bypass"]),
        "cert_backfilled": count(&["cert_backfilled"]),
        "cert_missing": count(&["cert_missing"]),
        "cert_writtenoff": count(&["cert_writtenoff"]),
        "routed": count(&["routed", "routed_merge", "routed_direct"]),
        "routed_direct": count(&["routed_direct"]),
        "sealed": count(&["sealed"]),
        "session_orphan": count(&["session_orphan"]),
        "total": records.len(),
        "unbypassed": count(&["unbypassed"]),
        "unrouted": count(&["unrouted"]),
    });
    let routed_shas: Vec<String> = records
        .iter()
        .filter(|r| {
            matches!(
                r.get("class").and_then(|v| v.as_str()),
                Some("routed") | Some("routed_merge")
            )
        })
        .filter_map(|r| r.get("sha").and_then(|v| v.as_str()).map(|s| s.to_string()))
        .collect();
    let unrouted_tail: Vec<Value> = records
        .iter()
        .filter(|r| {
            !matches!(
                r.get("class").and_then(|v| v.as_str()),
                Some("routed") | Some("sealed")
            )
        })
        .take(200)
        .cloned()
        .collect();
    let mut report = json!({
        "base": base.clone().unwrap_or_default(),
        "first_routed": routed_shas.last().cloned().map(Value::String).unwrap_or(Value::Null),
        "repo": repo.display().to_string(),
        "summary": summary,
        "unrouted_tail": unrouted_tail,
    });
    if let Some(seal) = seal {
        report["base_source"] = json!({
            "kind": "seal_line",
            "boundary": seal,
            "range_base": base.unwrap_or_default(),
        });
    }
    let s = &report["summary"];
    let code = if s.get("unrouted").and_then(|v| v.as_u64()) == Some(0)
        && s.get("session_orphan").and_then(|v| v.as_u64()) == Some(0)
        && s.get("cert_missing").and_then(|v| v.as_u64()) == Some(0)
        && s.get("unbypassed").and_then(|v| v.as_u64()) == Some(0)
    {
        0
    } else {
        1
    };
    print!("{}", emit(&report));
    std::process::exit(code);
}

/// Python json.dumps 缺省分隔符（", " 与 ": "）紧凑渲染，插入序保留（ensure_ascii=False 等价）
pub(crate) fn py_compact(v: &Value) -> String {
    match v {
        Value::Object(m) => {
            let items: Vec<String> = m
                .iter()
                .map(|(k, val)| {
                    format!("{}: {}", serde_json::to_string(k).unwrap(), py_compact(val))
                })
                .collect();
            format!("{{{}}}", items.join(", "))
        }
        Value::Array(a) => {
            let items: Vec<String> = a.iter().map(py_compact).collect();
            format!("[{}]", items.join(", "))
        }
        other => serde_json::to_string(other).unwrap(),
    }
}

/// Python json.dumps(indent=W) 渲染，插入序保留；空容器内联形
pub(crate) fn py_pretty(v: &Value, level: usize, width: usize) -> String {
    match v {
        Value::Object(m) if !m.is_empty() => {
            let pad = " ".repeat(level * width);
            let inner = " ".repeat((level + 1) * width);
            let items: Vec<String> = m
                .iter()
                .map(|(k, val)| {
                    format!(
                        "{}{}: {}",
                        inner,
                        serde_json::to_string(k).unwrap(),
                        py_pretty(val, level + 1, width)
                    )
                })
                .collect();
            format!("{{\n{}\n{}}}", items.join(",\n"), pad)
        }
        Value::Array(a) if !a.is_empty() => {
            let pad = " ".repeat(level * width);
            let inner = " ".repeat((level + 1) * width);
            let items: Vec<String> = a
                .iter()
                .map(|val| format!("{}{}", inner, py_pretty(val, level + 1, width)))
                .collect();
            format!("[\n{}\n{}]", items.join(",\n"), pad)
        }
        other => serde_json::to_string(other).unwrap(),
    }
}

#[cfg(test)]
mod pathfix_tests {
    // pathfix 批（pk-089 承接）：台账基座双分支与工作区根上溯发现单测。
    use super::{discover_workspace_root, ledger_surface};
    use std::path::PathBuf;

    fn tmp_ws(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("pathfix-unit-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        root
    }

    #[test]
    fn ledger_surface_central_form() {
        let ws = tmp_ws("central");
        std::fs::create_dir_all(ws.join("sih-tools/lease/ledger")).unwrap();
        assert_eq!(
            ledger_surface(&ws),
            ws.join("sih-tools/lease/ledger"),
            "中央形零改即既有路径逐字节不变"
        );
        let _ = std::fs::remove_dir_all(&ws);
    }

    #[test]
    fn ledger_surface_domain_form() {
        let dom = tmp_ws("domain");
        std::fs::create_dir_all(dom.join("sih/ledger")).unwrap();
        assert_eq!(
            ledger_surface(&dom),
            dom.join("sih/ledger"),
            "新城域形落域内正典位，不沉积 sih-tools 假骨架"
        );
        assert!(!dom.join("sih-tools").exists());
        let _ = std::fs::remove_dir_all(&dom);
    }

    #[test]
    fn discover_root_found_and_not_found() {
        let ws = tmp_ws("discover");
        std::fs::create_dir_all(ws.join("sih-engine")).unwrap();
        std::fs::create_dir_all(ws.join("sih-tools")).unwrap();
        let deep = ws.join("sih-engine/src/bin/lease");
        std::fs::create_dir_all(&deep).unwrap();
        let ws_canon = std::fs::canonicalize(&ws).unwrap_or_else(|_| ws.clone());
        assert_eq!(discover_workspace_root(&deep).as_deref(), Some(ws_canon.as_path()));
        let bare = tmp_ws("bare");
        std::fs::create_dir_all(&bare).unwrap();
        assert_eq!(
            discover_workspace_root(&bare).as_deref(),
            None,
            "无双目录标记即 None，调用位 fail-closed"
        );
        let _ = std::fs::remove_dir_all(&ws);
        let _ = std::fs::remove_dir_all(&bare);
    }
}
