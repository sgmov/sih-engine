//! SDD/TDD 完备度门：SDDG 四判据机械执法（SPEC-024 腿二，lease-commitlaw-parallel
//! 批）。判据定义单源在 DEC-024（sih-engine/doc/decision/024-sdd-completeness-gates.md），
//! 本模块只执法不修订；判定命令与反例指引照录 sddgate.py 单源常量禁改写。
//! 四判据全判定命令形零 LLM 零网络：正则匹配、时序比较、git 差分文件清单三件机械事。

use regex::Regex;
use serde_json::{json, Map, Value};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

pub(crate) const GATE_SINCE: &str = "2026-09-12";
pub(crate) const SDDG_VERSION: &str = "1.0.0";

pub(crate) const CRITERIA_COMMANDS: [(&str, &str); 4] = [
    (
        "SDDG-1",
        "取会话台账会话号，链面取意图笔时戳 t0 与首个实装落笔时戳 t1，查 t0 至 t1 \
         区间内或首个实装提交本身存在规格物变更（DEC/DES/SPEC 路径文件或任务包），\
         或意图记录显式申报规格消费面；三者有一即过，俱无即拒。",
    ),
    (
        "SDDG-2",
        "settle 差分文件清单交源码面，逐新增源码文件取头注前二十行，匹配正典指针\
         词形（SPEC、DES、DEC、正典）至少一处；零命中清单非空即拒。",
    ),
    (
        "SDDG-3",
        "结果档偏差申报逐条扫描，每条须命中承载词形（pk 编号、DEC、DES、SPEC、\
         候批）至少一处；偏差节缺席且差分非空且无零偏差显式申报即拒。",
    ),
    (
        "SDDG-4",
        "settle 差分文件清单匹配测试词形（tests 路径、test_ 前缀、_test 后缀）\
         至少一件，且验收材料或对表判词档在档；俱无即拒。",
    ),
];

pub(crate) const CRITERIA_COUNTEREXAMPLES: [(&str, &str); 4] = [
    (
        "SDDG-1",
        "实装提交先于任何规格物落笔且意图零申报。两通道：先立典（规格物落笔）或补申报（意图记录申报规格消费面）。",
    ),
    ("SDDG-2", "新模块头注无任何正典指针：生成时规格束缺位的实况残留。"),
    (
        "SDDG-3",
        "设计级偏差只落在会话散文与子代理报告，结果档偏差条目零承载词形。",
    ),
    ("SDDG-4", "实装笔落而测试件与对表材料俱缺。"),
];

const SOURCE_EXTS: [&str; 15] = [
    ".py", ".rs", ".js", ".ts", ".jsx", ".tsx", ".sh", ".go", ".c", ".h", ".cpp", ".hpp", ".java",
    ".rb", ".sql",
];

fn spec_path_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)(doc/decision/|doc/design/|doc/spec/|task-packages/|sih/state/plan/)|((^|/)(DEC|DES|SPEC)-[0-9][0-9A-Za-z-]*\.(md|adoc|json|txt)\b)",
        )
        .unwrap()
    })
}

fn intent_spec_decl_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"规格|消费面|正典|SPEC|DEC|DES").unwrap())
}

fn canon_pointer_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"SPEC|DES|DEC|正典").unwrap())
}

fn deviation_carrier_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"pk-\d+|DEC-\d+|DES-\d+|SPEC-\d+|候批").unwrap())
}

fn zero_deviation_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"零偏差").unwrap())
}

pub(crate) fn test_file_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(^|/)tests?(/|$)|(^|/)test_[^/]*\.py$|_test\.py$").unwrap()
    })
}

fn cross_repo_test_decl_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"测试件实体在工具仓|跨仓测试承载申报|test files carried cross-repo").unwrap()
    })
}

fn acceptance_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)acceptance|results|verdict").unwrap())
}

fn results_doc_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)results\.md$").unwrap())
}

fn git(repo: &Path, args: &[&str]) -> (i32, String, String) {
    match Command::new("git").arg("-C").arg(repo).args(args).output() {
        Ok(o) => (
            o.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string(),
        ),
        Err(e) => (-1, String::new(), e.to_string()),
    }
}

pub(crate) fn sddgate_applicable(session: &Value) -> bool {
    let ts = session
        .get("issued_at")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .or_else(|| session.get("opened_at").and_then(|v| v.as_str()))
        .unwrap_or("");
    // 时戳缺或不可解析按受检处理（fail-closed，与守卫家族同向）
    if ts.chars().count() < 10 {
        return true;
    }
    let prefix: String = ts.chars().take(10).collect();
    match chrono::NaiveDate::parse_from_str(&prefix, "%Y-%m-%d") {
        Ok(d) => d >= chrono::NaiveDate::from_ymd_opt(2026, 9, 12).unwrap(),
        Err(_) => true,
    }
}

fn parse_ts(ts: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    if ts.is_empty() {
        return None;
    }
    let t = ts.replace('Z', "+00:00");
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&t) {
        return Some(dt.with_timezone(&chrono::Utc));
    }
    if let Ok(d) = chrono::NaiveDate::parse_from_str(&t, "%Y-%m-%d") {
        return Some(d.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&t, "%Y-%m-%dT%H:%M:%S%.f") {
        return Some(dt.and_utc());
    }
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S%.f") {
        return Some(dt.and_utc());
    }
    None
}

pub(crate) fn branch_diff_files(repo: &Path, base: &str, branch: &str) -> Vec<Value> {
    if base.is_empty() || branch.is_empty() || base == branch {
        return Vec::new();
    }
    let (rc, out, _) = git(repo, &["diff", "--name-status", &format!("{}...{}", base, branch)]);
    if rc != 0 {
        return Vec::new();
    }
    let mut files = Vec::new();
    for line in out.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut parts = line.splitn(2, '\t');
        let status_raw = parts.next().unwrap_or("");
        let path_raw = parts.next().unwrap_or("").trim();
        let status: String = status_raw.chars().take(1).collect();
        let path = match path_raw.split_once(" -> ") {
            Some((_, after)) => after.trim().to_string(),
            None => path_raw.to_string(),
        };
        if !path.is_empty() {
            files.push(json!({"path": path, "status": status}));
        }
    }
    files
}

pub(crate) fn first_impl_commit(repo: &Path, base: &str, branch: &str) -> Option<Value> {
    if base.is_empty() || branch.is_empty() || base == branch {
        return None;
    }
    let (rc, out, _) = git(repo, &["rev-list", "--reverse", &format!("{}..{}", base, branch)]);
    if rc != 0 {
        return None;
    }
    let shas: Vec<&str> = out.split_whitespace().collect();
    let sha = *shas.first()?;
    let (rc_t, out_t, _) = git(repo, &["show", "-s", "--format=%cI", sha]);
    let ts = out_t.trim().lines().last().map(|s| s.trim().to_string());
    let (rc_f, out_f, _) = git(
        repo,
        &[
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            "--root",
            sha,
        ],
    );
    let files: Vec<String> = out_f
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    Some(json!({
        "committed_at": ts,
        "files": files,
        "repo": repo.display().to_string(),
        "sha": sha,
    }))
}

fn session_trail_events(session_id: &str, trail_paths: &[PathBuf]) -> Vec<Value> {
    let mut events = Vec::new();
    for tp in trail_paths {
        let text = match std::fs::read_to_string(tp) {
            Ok(t) => t,
            Err(_) => continue,
        };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let ev: Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if ev.get("session_id").and_then(|v| v.as_str()) == Some(session_id) {
                events.push(ev);
            }
        }
    }
    events
}

fn blob_text(repo: &Path, r: &str, path: &str, limit: Option<usize>) -> String {
    let (rc, out, _) = git(repo, &["show", &format!("{}:{}", r, path)]);
    if rc != 0 {
        return String::new();
    }
    let lines: Vec<&str> = out.lines().collect();
    match limit {
        Some(n) => lines.into_iter().take(n).collect::<Vec<_>>().join("\n"),
        None => lines.join("\n"),
    }
}

fn intent_declares_spec(session_events: &[Value]) -> bool {
    for ev in session_events {
        if ev.get("event_type").and_then(|v| v.as_str()) != Some("intent_refined") {
            continue;
        }
        let record_path = ev
            .get("details")
            .unwrap_or(&Value::Null)
            .get("record_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if record_path.is_empty() {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(record_path) {
            if intent_spec_decl_re().is_match(&text) {
                return true;
            }
        }
    }
    false
}

pub(crate) fn check_sddg1(session_events: &[Value], impl_commits: &[Option<Value>]) -> Value {
    let mut t0_dt: Option<chrono::DateTime<chrono::Utc>> = None;
    let mut t0: Option<String> = None;
    for ev in session_events {
        if ev.get("event_type").and_then(|v| v.as_str()) != Some("intent_refined") {
            continue;
        }
        let ts = ev.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
        if let Some(dt) = parse_ts(ts) {
            if t0_dt.is_none() || Some(dt) < t0_dt {
                t0_dt = Some(dt);
                t0 = Some(ts.to_string());
            }
        }
    }
    let mut t1_dt: Option<chrono::DateTime<chrono::Utc>> = None;
    let mut t1: Option<String> = None;
    let mut spec_hits: Vec<Value> = Vec::new();
    for c in impl_commits.iter().flatten() {
        let ts = c.get("committed_at").and_then(|v| v.as_str()).unwrap_or("");
        if let Some(dt) = parse_ts(ts) {
            if t1_dt.is_none() || Some(dt) < t1_dt {
                t1_dt = Some(dt);
                t1 = Some(ts.to_string());
            }
        }
        if let Some(files) = c.get("files").and_then(|v| v.as_array()) {
            for f in files {
                let f = f.as_str().unwrap_or("");
                if spec_path_re().is_match(f) {
                    spec_hits.push(json!({
                        "path": f,
                        "repo": c.get("repo").cloned().unwrap_or(Value::Null),
                    }));
                }
            }
        }
    }
    let mut window_hits: Vec<Value> = Vec::new();
    if let (Some(t0v), Some(t1v)) = (t0_dt, t1_dt) {
        for ev in session_events {
            let ts = ev.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
            let dt = match parse_ts(ts) {
                Some(d) => d,
                None => continue,
            };
            if dt < t0v || dt > t1v {
                continue;
            }
            let text = serde_json::to_string(ev).unwrap_or_default();
            if spec_path_re().is_match(&text) {
                window_hits.push(json!(ev.get("timestamp").cloned().unwrap_or(Value::Null)));
            }
        }
    }
    let declared = intent_declares_spec(session_events);
    let passed = !spec_hits.is_empty() || !window_hits.is_empty() || declared;
    json!({
        "gate": "SDDG-1",
        "verdict": if passed { "pass" } else { "reject" },
        "t0_intent": t0,
        "t1_first_impl": t1,
        "first_impl_spec_hits": spec_hits,
        "window_spec_hits": window_hits,
        "intent_spec_declared": declared,
    })
}

pub(crate) fn check_sddg2(repo_diffs: &[Value]) -> Value {
    let mut zero_hits: Vec<Value> = Vec::new();
    let mut checked = 0usize;
    for entry in repo_diffs {
        let repo = Path::new(entry.get("repo").and_then(|v| v.as_str()).unwrap_or(""));
        let branch = entry.get("branch").and_then(|v| v.as_str()).unwrap_or("");
        for f in entry.get("files").and_then(|v| v.as_array()).into_iter().flatten() {
            if f.get("status").and_then(|v| v.as_str()) != Some("A") {
                continue;
            }
            let path = f.get("path").and_then(|v| v.as_str()).unwrap_or("");
            if test_file_re().is_match(path) {
                continue;
            }
            let ext = std::path::Path::new(path)
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();
            if !SOURCE_EXTS.contains(&ext.as_str()) {
                continue;
            }
            checked += 1;
            let head = blob_text(repo, branch, path, Some(20));
            if !canon_pointer_re().is_match(&head) {
                zero_hits.push(json!({
                    "path": path,
                    "repo": repo.display().to_string(),
                }));
            }
        }
    }
    json!({
        "gate": "SDDG-2",
        "verdict": if zero_hits.is_empty() { "pass" } else { "reject" },
        "checked_new_source_files": checked,
        "zero_pointer_hits": zero_hits,
    })
}

fn deviation_entries(text: &str) -> Option<Vec<String>> {
    let lines: Vec<&str> = text.lines().collect();
    let mut start: Option<usize> = None;
    for (i, ln) in lines.iter().enumerate() {
        let s = ln.trim();
        if (s.starts_with('#') && s.contains("偏差")) || s.starts_with("偏差") {
            start = Some(i + 1);
            break;
        }
    }
    let start = start?;
    let mut entries = Vec::new();
    for ln in lines.iter().skip(start) {
        let s = ln.trim();
        if s.starts_with('#') {
            break;
        }
        if s.is_empty() {
            continue;
        }
        entries.push(s.trim_start_matches(['-', '*']).trim().to_string());
    }
    Some(entries)
}

pub(crate) fn check_sddg3(diff_nonempty: bool, results_docs: &[Value]) -> Value {
    if !diff_nonempty {
        return json!({
            "gate": "SDDG-3",
            "verdict": "pass",
            "reason": "diff_empty",
            "uncarried_entries": [],
        });
    }
    if results_docs.is_empty() {
        return json!({
            "gate": "SDDG-3",
            "verdict": "reject",
            "reason": "results_doc_absent",
            "uncarried_entries": [],
        });
    }
    let mut uncarried: Vec<Value> = Vec::new();
    let mut declared_zero = false;
    let mut has_section = false;
    for doc in results_docs {
        let text = doc.get("text").and_then(|v| v.as_str()).unwrap_or("");
        if zero_deviation_re().is_match(text) {
            declared_zero = true;
        }
        match deviation_entries(text) {
            None => continue,
            Some(entries) => {
                has_section = true;
                for e in entries {
                    if !deviation_carrier_re().is_match(&e) {
                        uncarried.push(json!({
                            "doc": doc.get("path").cloned().unwrap_or(Value::Null),
                            "entry": e,
                        }));
                    }
                }
            }
        }
    }
    if !uncarried.is_empty() {
        return json!({
            "gate": "SDDG-3",
            "verdict": "reject",
            "reason": "deviation_entry_uncarried",
            "uncarried_entries": uncarried,
        });
    }
    if has_section || declared_zero {
        return json!({
            "gate": "SDDG-3",
            "verdict": "pass",
            "deviation_section": has_section,
            "zero_deviation_declared": declared_zero,
            "uncarried_entries": [],
        });
    }
    json!({
        "gate": "SDDG-3",
        "verdict": "reject",
        "reason": "deviation_section_absent_and_no_zero_declaration",
        "uncarried_entries": [],
    })
}

pub(crate) fn check_sddg4(diff_paths: &[String], results_docs_hint: &[Value]) -> Value {
    let mut test_files: Vec<String> = diff_paths
        .iter()
        .filter(|p| test_file_re().is_match(p))
        .cloned()
        .collect();
    test_files.sort();
    test_files.dedup();
    let mut acceptance: Vec<String> = diff_paths
        .iter()
        .filter(|p| acceptance_re().is_match(p))
        .cloned()
        .collect();
    acceptance.sort();
    acceptance.dedup();
    if diff_paths.is_empty() {
        return json!({
            "gate": "SDDG-4",
            "verdict": "pass",
            "reason": "diff_empty",
            "test_files": [],
            "acceptance_files": [],
        });
    }
    let cross_repo_declared = results_docs_hint
        .iter()
        .any(|d| cross_repo_test_decl_re().is_match(d.get("text").and_then(|v| v.as_str()).unwrap_or("")));
    let passed = (!test_files.is_empty() && !acceptance.is_empty())
        || (cross_repo_declared && !acceptance.is_empty());
    let mut payload = json!({
        "gate": "SDDG-4",
        "verdict": if passed { "pass" } else { "reject" },
        "test_files": test_files,
        "acceptance_files": acceptance,
    });
    if cross_repo_declared && payload["test_files"].as_array().map(|a| a.is_empty()).unwrap_or(false) {
        payload["reason"] = json!("cross_repo_test_declaration");
    }
    payload
}

pub(crate) fn run_gate(session: &Value, trails: &[PathBuf], repos: &[Value]) -> Value {
    if !sddgate_applicable(session) {
        return json!({
            "checked": false,
            "verdict": "skip",
            "reason": "before_gate_since",
            "gate_since": GATE_SINCE,
        });
    }
    let trail_files: Vec<PathBuf> = trails.iter().filter(|t| t.is_file()).cloned().collect();
    if trail_files.is_empty() {
        return json!({
            "checked": false,
            "verdict": "skip",
            "reason": "workspace_chainless",
        });
    }
    let sid = session.get("session_id").and_then(|v| v.as_str()).unwrap_or("");
    let events = session_trail_events(sid, &trail_files);
    let mut repo_diffs: Vec<Value> = Vec::new();
    let mut impl_commits: Vec<Option<Value>> = Vec::new();
    for r in repos {
        let repo = Path::new(r.get("repo").and_then(|v| v.as_str()).unwrap_or(""));
        let base = r.get("base").and_then(|v| v.as_str()).unwrap_or("");
        let branch = r.get("branch").and_then(|v| v.as_str()).unwrap_or("");
        if base.is_empty() || branch.is_empty() || base == branch {
            continue;
        }
        repo_diffs.push(json!({
            "base": base,
            "branch": branch,
            "files": branch_diff_files(repo, base, branch),
            "repo": repo.display().to_string(),
        }));
        impl_commits.push(first_impl_commit(repo, base, branch));
    }
    let diff_paths: Vec<String> = repo_diffs
        .iter()
        .flat_map(|e| {
            e.get("files")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default()
        })
        .filter_map(|f| {
            f.get("path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .collect();
    let diff_nonempty = !diff_paths.is_empty();
    let mut results_docs: Vec<Value> = Vec::new();
    for e in &repo_diffs {
        let repo = Path::new(e.get("repo").and_then(|v| v.as_str()).unwrap_or(""));
        let branch = e.get("branch").and_then(|v| v.as_str()).unwrap_or("");
        for f in e.get("files").and_then(|v| v.as_array()).into_iter().flatten() {
            let path = f.get("path").and_then(|v| v.as_str()).unwrap_or("");
            if results_doc_re().is_match(path) {
                results_docs.push(json!({
                    "path": path,
                    "repo": repo.display().to_string(),
                    "text": blob_text(repo, branch, path, None),
                }));
            }
        }
    }
    let gates = json!({
        "SDDG-1": check_sddg1(&events, &impl_commits),
        "SDDG-2": check_sddg2(&repo_diffs),
        "SDDG-3": check_sddg3(diff_nonempty, &results_docs),
        "SDDG-4": check_sddg4(&diff_paths, &results_docs),
    });
    let rejected: Vec<String> = ["SDDG-1", "SDDG-2", "SDDG-3", "SDDG-4"]
        .iter()
        .filter(|k| gates[**k].get("verdict").and_then(|v| v.as_str()) == Some("reject"))
        .map(|k| k.to_string())
        .collect();
    let mut out = json!({
        "checked": true,
        "verdict": if rejected.is_empty() { "pass" } else { "reject" },
        "gates": gates,
        "gate_since": GATE_SINCE,
        "sddg_version": SDDG_VERSION,
    });
    if !rejected.is_empty() {
        let mut criteria = Map::new();
        let mut counter = Map::new();
        for k in &rejected {
            if let Some((_, cmd)) = CRITERIA_COMMANDS.iter().find(|(g, _)| g == k) {
                criteria.insert(k.clone(), json!(cmd));
            }
            if let Some((_, ce)) = CRITERIA_COUNTEREXAMPLES.iter().find(|(g, _)| g == k) {
                counter.insert(k.clone(), json!(ce));
            }
        }
        out["teaching"] = json!({
            "reason_code": "sddgate_rejected",
            "gate": rejected,
            "gate_verdicts": gates,
            "criteria_commands": Value::Object(criteria),
            "counterexamples": Value::Object(counter),
            "channels": "三通道：先立典（规格物落笔）、补申报（意图或结果档偏差承载）、\
                         显式绕行 close --bypass-sddgate <事由> 落 bypass 台账留痕",
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_repo(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "leg2-sdd-{}-{}",
            tag,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let gitout = |args: &[&str]| {
            let o = Command::new("git")
                .arg("-C")
                .arg(&dir)
                .args(args)
                .output()
                .unwrap();
            assert!(o.status.success(), "git {:?} failed: {}", args, String::from_utf8_lossy(&o.stderr));
        };
        let _ = Command::new("git").arg("init").arg("-b").arg("master").arg(&dir).output();
        gitout(&["config", "user.email", "t@t"]);
        gitout(&["config", "user.name", "t"]);
        std::fs::write(dir.join("README.md"), "base\n").unwrap();
        gitout(&["add", "-A"]);
        gitout(&["commit", "-m", "base"]);
        dir
    }

    fn add_branch_commit(dir: &Path, branch: &str, files: &[(&str, &str)]) {
        let o = Command::new("git").arg("-C").arg(dir).args(["checkout", "-b", branch]).output().unwrap();
        assert!(o.status.success(), "checkout: {}", String::from_utf8_lossy(&o.stderr));
        for (p, text) in files {
            let full = dir.join(p);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(full, text).unwrap();
        }
        let o = Command::new("git").arg("-C").arg(dir).args(["add", "-A"]).output().unwrap();
        assert!(o.status.success());
        let o = Command::new("git").arg("-C").arg(dir).args(["commit", "-m", "impl"]).output().unwrap();
        assert!(o.status.success(), "commit: {}", String::from_utf8_lossy(&o.stderr));
    }

    fn repo_entry(dir: &Path) -> Value {
        json!({"base": "master", "branch": "msh/t", "repo": dir.display().to_string()})
    }

    #[test]
    fn t6_sddg2_pointer_scan() {
        let dir = make_repo("s2a");
        add_branch_commit(
            &dir,
            "msh/t",
            &[("src/lib.rs", "// SPEC-024 腿二实装\nfn x() {}\n")],
        );
        let diffs = vec![json!({
            "branch": "msh/t",
            "files": branch_diff_files(&dir, "master", "msh/t"),
            "repo": dir.display().to_string(),
        })];
        let verdict = check_sddg2(&diffs);
        assert_eq!(verdict["verdict"], json!("pass"));
        assert_eq!(verdict["checked_new_source_files"], json!(1));

        let dir2 = make_repo("s2b");
        add_branch_commit(&dir2, "msh/t", &[("src/bare.rs", "fn y() {}\n")]);
        let diffs2 = vec![json!({
            "branch": "msh/t",
            "files": branch_diff_files(&dir2, "master", "msh/t"),
            "repo": dir2.display().to_string(),
        })];
        let verdict2 = check_sddg2(&diffs2);
        assert_eq!(verdict2["verdict"], json!("reject"));
        assert_eq!(
            verdict2["zero_pointer_hits"].as_array().unwrap().len(),
            1
        );
    }

    #[test]
    fn t6_sddg2_skips_tests_and_docs() {
        let dir = make_repo("s2c");
        add_branch_commit(
            &dir,
            "msh/t",
            &[
                ("tests/test_x.py", "def test_x():\n    pass\n"),
                ("docs/note.md", "纯文档无指针\n"),
            ],
        );
        let diffs = vec![json!({
            "branch": "msh/t",
            "files": branch_diff_files(&dir, "master", "msh/t"),
            "repo": dir.display().to_string(),
        })];
        let verdict = check_sddg2(&diffs);
        assert_eq!(verdict["verdict"], json!("pass"));
        assert_eq!(verdict["checked_new_source_files"], json!(0));
    }

    #[test]
    fn t6_sddg3_deviation_carriers() {
        let v = check_sddg3(false, &[]);
        assert_eq!(v["verdict"], json!("pass"));
        assert_eq!(v["reason"], json!("diff_empty"));

        let absent = check_sddg3(true, &[]);
        assert_eq!(absent["verdict"], json!("reject"));
        assert_eq!(absent["reason"], json!("results_doc_absent"));

        let doc_carried = json!({"path": "results.md", "text": "## 偏差申报 {#dev}\n\n- pk-090 stem 闸教学位复刻\n- 零偏差显式申报位：本批零偏差（pk-090 承载）\n"});
        let carried = check_sddg3(true, &[doc_carried]);
        assert_eq!(carried["verdict"], json!("pass"));

        let doc_uncarried = json!({"path": "results.md", "text": "## 偏差申报 {#dev}\n\n- 某处走了别的通道没有承载词\n"});
        let uncarried = check_sddg3(true, &[doc_uncarried]);
        assert_eq!(uncarried["verdict"], json!("reject"));
        assert_eq!(uncarried["reason"], json!("deviation_entry_uncarried"));
        assert_eq!(uncarried["uncarried_entries"].as_array().unwrap().len(), 1);

        let doc_zero = json!({"path": "results.md", "text": "零偏差显式申报位：无偏差节\n"});
        let zero = check_sddg3(true, &[doc_zero]);
        assert_eq!(zero["verdict"], json!("pass"));
        assert_eq!(zero["zero_deviation_declared"], json!(true));

        let nos = check_sddg3(true, &[json!({"path": "results.md", "text": "无偏差节的结果档\n"})]);
        assert_eq!(nos["verdict"], json!("reject"));
        assert_eq!(
            nos["reason"],
            json!("deviation_section_absent_and_no_zero_declaration")
        );
    }

    #[test]
    fn t6_sddg4_test_and_acceptance() {
        let pass = check_sddg4(
            &["tests/t.rs".into(), "results.md".into()],
            &[],
        );
        assert_eq!(pass["verdict"], json!("pass"));
        let no_acceptance = check_sddg4(&["tests/t.rs".into()], &[]);
        assert_eq!(no_acceptance["verdict"], json!("reject"));
        let cross = check_sddg4(
            &["src/x.rs".into(), "results.md".into()],
            &[json!({"text": "跨仓测试承载申报：测试件实体在工具仓 tests/"})],
        );
        assert_eq!(cross["verdict"], json!("pass"));
        assert_eq!(cross["reason"], json!("cross_repo_test_declaration"));
        let empty = check_sddg4(&[], &[]);
        assert_eq!(empty["verdict"], json!("pass"));
        assert_eq!(empty["reason"], json!("diff_empty"));
    }

    #[test]
    fn t6_applicability_and_chainless() {
        let fresh = json!({"issued_at": "2026-09-13T00:00:00+00:00"});
        assert!(sddgate_applicable(&fresh));
        let old = json!({"issued_at": "2026-09-01T00:00:00+00:00"});
        assert!(!sddgate_applicable(&old));
        let broken = json!({"issued_at": "garbage"});
        assert!(sddgate_applicable(&broken), "不可解析 fail-closed 受检");

        let session = json!({"issued_at": "2026-09-13T00:00:00+00:00", "session_id": "abc"});
        let out = run_gate(&session, &[], &[repo_entry(Path::new("/tmp/none"))]);
        assert_eq!(out["verdict"], json!("skip"));
        assert_eq!(out["reason"], json!("workspace_chainless"));
        assert_eq!(out["checked"], json!(false));
    }

    #[test]
    fn t6_run_gate_reject_teaching_shape() {
        let dir = make_repo("gate");
        add_branch_commit(&dir, "msh/t", &[("src/bare.rs", "fn z() {}\n")]);
        let trail = dir.join("trail.ndjson");
        std::fs::write(
            &trail,
            format!(
                "{}\n",
                json!({"event_type": "intent_refined", "session_id": "abc", "timestamp": "2026-09-13T01:00:00+00:00"})
            ),
        )
        .unwrap();
        let session = json!({"issued_at": "2026-09-13T00:00:00+00:00", "session_id": "abc"});
        let out = run_gate(&session, &[trail], &[repo_entry(&dir)]);
        assert_eq!(out["verdict"], json!("reject"));
        assert_eq!(out["checked"], json!(true));
        let teaching = &out["teaching"];
        assert_eq!(teaching["reason_code"], json!("sddgate_rejected"));
        assert!(teaching["gate"].as_array().unwrap().iter().any(|g| g == "SDDG-2"));
        assert!(teaching["criteria_commands"]["SDDG-2"].is_string());
        assert!(teaching["counterexamples"]["SDDG-2"].is_string());
    }
}
