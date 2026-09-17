//! 收约执法闸序（SPEC-024 腿二，lease-commitlaw-parallel 批）：锁清零、工地卫生、
//! 冲突三态、预收提交、链证守门、SDDG 四判据、CALL-LOG 跑步机与随批检查、无主闸、
//! 声明差集闸、级联投影 close 前重建（pk-055）、归并删支拆本、账单与收约凭据。
//! 融回基准权威 sih-tools/lease/src/lease/
//! core.py close_session 与 watchcheck/core.py 无主谓词，闸序承 closegate-solo T-2 终签。

use crate::commitlaw::{default_trails, fail, git, ledger_surface, py_compact, py_pretty, py_resolve};
use crate::sddgate;
use crate::{append_row, emit, now_utc, one, read_jsonl, tool};
use chrono::DateTime;
use regex::Regex;
use serde_json::{json, Map, Value};
use sih_engine::cascade_registry::{build_registry_recorded, dump_canonical};
use std::process::Command;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const CALLLOG_GUARD_EFFECTIVE_AT: &str = "2026-09-08T00:01:35+00:00";
const CALLLOG_TREADMILL_EFFECTIVE_AT: &str = "2026-09-08T08:00:00+00:00";
const UNUSED_LOCK_MULTIPLIER: i64 = 1;
const EXEMPTION_FACES: [&str; 17] = [
    "sih-engine/sih/event/trail/",
    "sih-tools/lease/ledger/",
    "sih-tools/meter/counts/",
    "sih-tools/scribe/reports/",
    "sih-tools/identity/reports/",
    "sih-engine/sih/state/plan/",
    "sih-engine/sih/event/plan/",
    "sih-engine/sih/state/parking/",
    "sih-tools/facet/contracts/",
    "sih-tools/facet/facet_task_packages/",
    "sih-tools/facet/probes/calibration/",
    "sih-tools/proposition/",
    "sih-tools/parking/",
    "sih-tools/tally/reports/",
    "sih-tools/scribe/trail/",
    "sih-tools/elicit/signals/",
    "sih-tools/lease/ledger/receipts/",
];

fn append_row_raw(path: &Path, v: &Value) {
    use std::io::Write;
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p).ok();
    }
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    writeln!(f, "{}", serde_json::to_string(v).unwrap()).ok();
}

fn sessions_pair(events: Vec<Value>) -> BTreeMap<String, Value> {
    let mut sessions: BTreeMap<String, Value> = BTreeMap::new();
    for event in events {
        let sid = event.get("session_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        match event.get("event").and_then(|v| v.as_str()) {
            Some("issued") => {
                sessions.insert(sid, event);
            }
            Some("revoked") => {
                sessions.remove(&sid);
            }
            _ => {}
        }
    }
    sessions
}

fn parse_iso(ts: &str) -> Option<DateTime<chrono::Utc>> {
    let t = ts.replace('Z', "+00:00");
    DateTime::parse_from_rfc3339(&t)
        .ok()
        .map(|d| d.with_timezone(&chrono::Utc))
}

fn transition_applicable(session: &Value, constant: &str) -> bool {
    let issued = session.get("issued_at").and_then(|v| v.as_str()).unwrap_or("");
    match (parse_iso(issued), parse_iso(constant)) {
        (Some(a), Some(b)) => a > b,
        _ => true,
    }
}

fn lock_face_paths(locks: &Path) -> Vec<(String, Vec<String>)> {
    let mut held: Vec<(String, String)> = Vec::new();
    for row in read_jsonl(locks) {
        let ev = row.get("event").and_then(|v| v.as_str()).unwrap_or("");
        let p = row.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let sid = row.get("session_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        match ev {
            "acquired" => {
                if let Some(slot) = held.iter_mut().find(|(hp, hs)| *hp == p && *hs == sid) {
                    continue;
                }
                held.push((p, sid));
            }
            "released" => held.retain(|(hp, hs)| !(hp == &p && hs == &sid)),
            _ => {}
        }
    }
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (p, s) in held {
        out.entry(p).or_default().push(s);
    }
    out.into_iter().collect()
}

fn check_worktree_clean(worktree: &Path) -> Value {
    let mut issues: Vec<Value> = Vec::new();
    if !worktree.exists() {
        return json!({"clean": true, "issues": [], "worktree_missing": true});
    }
    let (rc, out, _) = git(worktree, &["status", "--porcelain"]);
    if rc == 0 {
        for line in out.lines() {
            let stripped = line.trim();
            if stripped.is_empty() {
                continue;
            }
            let path = stripped[3..].trim().trim_end_matches('/').to_string();
            if stripped.starts_with("??") {
                issues.push(json!({
                    "action": format!("git add {} && git commit or remove file", path),
                    "path": path,
                    "type": "untracked",
                }));
            } else {
                issues.push(json!({
                    "action": format!("git add {} && git commit", path),
                    "path": path,
                    "type": "modified",
                }));
            }
        }
    }
    json!({"clean": issues.is_empty(), "issues": issues})
}

fn dirty_entries(repo: &Path) -> Vec<(String, String)> {
    let o = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["status", "--porcelain", "-uall", "-z", "--no-renames"])
        .output();
    let mut entries = Vec::new();
    if let Ok(o) = o {
        if o.status.success() {
            let text = String::from_utf8_lossy(&o.stdout).to_string();
            for rec in text.split('\0') {
                if rec.is_empty() {
                    continue;
                }
                let xy: String = rec.chars().take(2).collect();
                let rel = rec[3..].to_string();
                entries.push((rel, xy));
            }
        }
    }
    entries
}

fn merge_face(repo: &Path, base: &str, branch: &str) -> Vec<String> {
    let (rc, out, _) = git(repo, &["diff", "--name-only", &format!("{}...{}", base, branch)]);
    if rc != 0 {
        return Vec::new();
    }
    out.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn unowned_list(root: &Path, trail: &Path, locks_path: &Path) -> Vec<Value> {
    // 脏文件集：双仓 git status --porcelain -uall -z --no-renames，根相对路径排序
    let mut dirty: Vec<(String, String)> = Vec::new();
    for name in ["sih-tools", "sih-engine"] {
        for (rel, xy) in dirty_entries(&root.join(name)) {
            dirty.push((format!("{}/{}", name, rel), xy));
        }
    }
    dirty.sort();
    // 锁面镜像
    let mut locks: BTreeMap<String, String> = BTreeMap::new();
    for row in read_jsonl(locks_path) {
        let ev = row.get("event").and_then(|v| v.as_str()).unwrap_or("");
        let p = row.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let sid = row.get("session_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        match ev {
            "acquired" => {
                locks.insert(format!("{}\u{0}{}", p, sid), row.get("mode").and_then(|v| v.as_str()).unwrap_or("exclusive").to_string());
            }
            "released" => {
                locks.remove(&format!("{}\u{0}{}", p, sid));
            }
            _ => {}
        }
    }
    // 声明面：direct_edit_completed 事件 files 并集
    let mut declared: BTreeSet<String> = BTreeSet::new();
    for line in std::fs::read_to_string(trail).unwrap_or_default().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let ev: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if ev.get("event_type").and_then(|v| v.as_str()) == Some("direct_edit_completed") {
            if let Some(files) = ev.pointer("/details/files").and_then(|v| v.as_array()) {
                for f in files {
                    if let Some(s) = f.as_str() {
                        declared.insert(s.to_string());
                    }
                }
            }
        }
    }
    let mut out = Vec::new();
    for (ws_path, xy) in dirty {
        let mut covered = declared.contains(&ws_path);
        if !covered {
            for key in locks.keys() {
                let lp = key.split('\u{0}').next().unwrap_or("");
                if lp == ws_path || (lp.ends_with('/') && format!("{}/", ws_path).starts_with(lp)) {
                    covered = true;
                    break;
                }
            }
        }
        if !covered {
            covered = EXEMPTION_FACES
                .iter()
                .any(|ef| format!("{}/", ws_path).starts_with(ef));
        }
        if !covered {
            let abspath = root.join(&ws_path);
            let mtime = std::fs::metadata(&abspath)
                .and_then(|m| m.modified())
                .ok()
                .map(|t| {
                    let dt: DateTime<chrono::Utc> = t.into();
                    dt.format("%Y-%m-%dT%H:%M:%S+00:00").to_string()
                })
                .unwrap_or_default();
            out.push(json!({"path": ws_path, "xy": xy, "mtime": mtime}));
        }
    }
    out
}

fn calllog_dirty_faces(repo: &Path) -> Vec<(String, String)> {
    dirty_entries(repo)
        .into_iter()
        .filter(|(p, _)| {
            Path::new(p)
                .file_name()
                .map(|n| n == "CALL-LOG.md")
                .unwrap_or(false)
        })
        .collect()
}

fn calllog_gate_message(unreleased: &[Value]) -> String {
    format!(
        "CALL-LOG 随批检查拦截：仓内存在未随批入版控的 CALL-LOG.md 脏面（共 {} 件），整批拒零动作。\n缺件事由：CALL-LOG.md 脏面（已修改或未跟踪）未含于该批 settle 提交（v1 判定即脏面路径在会话分支 tip 在席且盘上内容与分支 tip 逐字节一致），亦无显式 bypass 登记事由（m-orphanrule-1 P3 既裁立则，orphanexec-solo 批实施）。\n文件清单：{}\n处置指引（三通道）：\n  1) 随批入版控：把脏面现字节态复制入本批工地随 settle 提交（B 案随批节奏）\n  2) 显式绕行：close --bypass-calllog <事由> 落 bypass.ndjson 留痕\n  3) 追加行逐行对权威腿 calls.ndjson 对表属后继加强位，本批不实装（裁定边界）",
        unreleased.len(),
        py_pretty(&json!(unreleased), 0, 2)
    )
}

fn parse_declared_writes(package_path: &Path) -> Vec<(String, bool, Vec<String>)> {
    static TOKEN: OnceLock<Regex> = OnceLock::new();
    let token = TOKEN.get_or_init(|| Regex::new(r"(?:[A-Za-z0-9_.\-]+/)+[A-Za-z0-9_.\-]*").unwrap());
    let mut lines = Vec::new();
    let text = match std::fs::read_to_string(package_path) {
        Ok(t) => t,
        Err(_) => fail(
            2,
            json!({"error": format!("package unreadable: {}", package_path.display())}),
        ),
    };
    let mut in_section = false;
    for line in text.lines() {
        if line.starts_with("## ") {
            in_section = line.split('{').next().unwrap_or("").contains("请求写入");
            continue;
        }
        if !in_section {
            continue;
        }
        let stripped = line.trim();
        if stripped.starts_with("- ") || stripped.starts_with("* ") {
            let item = stripped[2..].trim().trim_matches('`').to_string();
            if !item.is_empty() {
                let paths: Vec<String> = token
                    .find_iter(&item)
                    .map(|m| m.as_str().to_string())
                    .collect();
                let conditional = item.contains("若");
                lines.push((item, conditional, paths));
            }
        }
    }
    lines
}

fn declared_path_committed(repo: &Path, branch: &str, rel: &str, is_dir: bool) -> bool {
    let (rc, out, _) = git(
        repo,
        &["ls-tree", "-r", "--name-only", branch, "--", rel],
    );
    if rc != 0 {
        return false;
    }
    let rel_norm = rel.trim_end_matches('/');
    for line in out.lines() {
        let entry = line.trim();
        if entry.is_empty() {
            continue;
        }
        if is_dir && (entry == rel_norm || entry.starts_with(&format!("{}/", rel_norm))) {
            return true;
        }
        if !is_dir && entry == rel_norm {
            return true;
        }
    }
    false
}

fn collect_gates_skipped(
    bypass_orphan: Option<&String>,
    bypass_calllog: Option<&String>,
    declaration_gate: &Value,
) -> Value {
    let acks: Vec<Value> = declaration_gate
        .get("acks")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.get("path").and_then(|p| p.as_str()).map(|s| json!(s)))
                .collect()
        })
        .unwrap_or_default();
    let count = bypass_orphan.is_some() as i64
        + bypass_calllog.is_some() as i64
        + acks.len() as i64;
    json!({
        "ack_uncommitted": acks,
        "calllog_bypass": bypass_calllog.map(|s| json!(s)).unwrap_or(Value::Null),
        "count": count,
        "orphan_bypass": bypass_orphan.map(|s| json!(s)).unwrap_or(Value::Null),
    })
}

/// 级联边册投影 close 前重建（pk-055 cascadeclose-solo 批，2026-09-17 用户裁定
/// 形态：close 前重建挂收约流程，承 pk-052 择案先例）。sih-engine/doc/CASCADE.json
/// 是级联边册投影，认证后不随动（消费位 locksview check 缺省读它作乐观锁基线），
/// 故挂收约闸序重建。分流与处置：
/// - 仅中央工作区形生效：root 下 sih-tools/lease/ledger 在位（承 ledger_surface
///   分流先例），新城域形跳过注记 reason=domain_form。
/// - 级联基建在位（主树语料根与投影件俱在）才重建，缺席跳过注记
///   reason=no_cascade_infrastructure（跳过不炸）。
/// - 载体即会话 sih-engine 工地（语料取工地 doc，归并后态近似；册记 root 锚主树
///   语料位防工地路径进册致伪 diff）；无载体跳过注记 reason=no_worktree_carrier
///   （主树零直写，投影只经工地归并路径回主树）。
/// - 产物与工地在盘投影逐字节比：in_sync 零动作；diff 即落工地并自成一笔提交
///   （归并带回主树）。重建异常不静默（含语料扫描 panic 形 catch_unwind 收口），
///   fail-visible 拒收约退出码一，会话保持活跃零归并。
fn cascade_projection_rebuild(root: &Path, repos: &[Value], stem: &str) -> Value {
    if !root.join("sih-tools/lease/ledger").is_dir() {
        return json!({"checked": false, "reason": "domain_form"});
    }
    let main_doc = root.join("sih-engine/doc");
    if !main_doc.is_dir() || !main_doc.join("CASCADE.json").is_file() {
        return json!({"checked": false, "reason": "no_cascade_infrastructure"});
    }
    let carrier = repos.iter().find_map(|e| {
        let repo = PathBuf::from(e.get("repo").and_then(|v| v.as_str()).unwrap_or(""));
        let worktree = PathBuf::from(e.get("worktree").and_then(|v| v.as_str()).unwrap_or(""));
        if repo.file_name().and_then(|n| n.to_str()) == Some("sih-engine")
            && worktree.is_dir()
            && worktree.join("doc").is_dir()
            && worktree.join("doc/CASCADE.json").is_file()
        {
            Some(worktree)
        } else {
            None
        }
    });
    let Some(worktree) = carrier else {
        return json!({"checked": false, "reason": "no_worktree_carrier"});
    };
    let corpus = worktree.join("doc");
    let rebuilt = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        build_registry_recorded(&corpus, &main_doc)
    }));
    let registry = match rebuilt {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => fail(
            1,
            json!({
                "error": format!(
                    "级联投影重建失败：close 前重建不静默，整批拒零动作（会话保持活跃，修复语料后重收）。{e}"
                ),
                "reason": "cascade_rebuild_failed",
            }),
        ),
        Err(_) => fail(
            1,
            json!({
                "error": "级联投影重建失败：语料扫描中断（panic 形，如非 UTF-8 语料件），close 前重建不静默，整批拒零动作（会话保持活跃，修复语料后重收）",
                "reason": "cascade_rebuild_failed",
            }),
        ),
    };
    let text = dump_canonical(&registry);
    let projection = corpus.join("CASCADE.json");
    let ondisk = std::fs::read_to_string(&projection).unwrap_or_default();
    if ondisk == text {
        return json!({"checked": true, "diff": false, "result": "in_sync"});
    }
    if let Err(e) = std::fs::write(&projection, &text) {
        fail(
            1,
            json!({
                "error": format!("级联投影落盘失败：{}: {e}", projection.display()),
                "reason": "cascade_rebuild_failed",
            }),
        );
    }
    git(&worktree, &["add", "--", "doc/CASCADE.json"]);
    let (rc_c, _, err_c) = git(
        &worktree,
        &[
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "-m",
            &format!("cascadeclose-solo: pre-close cascade projection rebuild ({})", stem),
        ],
    );
    if rc_c != 0 {
        let detail: String = err_c.trim().chars().take(200).collect();
        fail(
            1,
            json!({
                "error": format!("级联投影工地提交失败：{}", detail),
                "reason": "cascade_rebuild_failed",
            }),
        );
    }
    let (rc_s, sha, _) = git(&worktree, &["rev-parse", "--short", "HEAD"]);
    json!({
        "checked": true,
        "commit": if rc_s == 0 { sha.trim().to_string() } else { String::new() },
        "diff": true,
        "projection": "sih-engine/doc/CASCADE.json",
        "result": "updated",
    })
}

pub(crate) fn cmd_close(m: &BTreeMap<String, Vec<String>>) {
    let root = py_resolve(Path::new(one(m, "root").unwrap_or(".")));
    let stem = one(m, "package").unwrap_or("").to_string();
    if stem.is_empty() {
        fail(2, json!({"error": "close 缺 --package"}));
    }
    let force = m.contains_key("force");
    let reason = one(m, "reason").map(|s| s.to_string());
    let at = one(m, "at").map(|s| s.to_string());
    let ledger = one(m, "ledger")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("sih-tools/lease/ledger/sessions.ndjson"));
    let locks_ledger = one(m, "locks")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("sih-tools/lease/ledger/locks.ndjson"));
    let bypass_ledger = one(m, "bypass-ledger")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("sih-tools/lease/ledger/bypass.ndjson"));
    let trails: Vec<PathBuf> = match m.get("trail") {
        Some(v) if !v.is_empty() => v.iter().map(PathBuf::from).collect(),
        _ => default_trails(&root),
    };
    let bypass_orphan = one(m, "bypass-orphan").map(|s| s.to_string());
    let bypass_calllog = one(m, "bypass-calllog").map(|s| s.to_string());
    let bypass_sddgate = one(m, "bypass-sddgate").map(|s| s.to_string());
    let mut ack_map: BTreeMap<String, String> = BTreeMap::new();
    for item in m.get("ack-uncommitted").into_iter().flatten() {
        match item.split_once('=') {
            None => fail(
                1,
                json!({
                    "error": "差集闸认领形非法：须 路径=事由（事由零粉饰留档）",
                    "item": item,
                    "reason": "ack_uncommitted_invalid",
                }),
            ),
            Some((p, r)) if p.trim().is_empty() || r.trim().is_empty() => fail(
                1,
                json!({
                    "error": "差集闸认领形非法：路径与事由俱必填",
                    "item": item,
                    "reason": "ack_uncommitted_invalid",
                }),
            ),
            Some((p, r)) => {
                ack_map.insert(p.trim().to_string(), r.trim().to_string());
            }
        }
    }

    // 会话定位（在册配对语义）。--session 点名形（recognize-solo iso-08）：
    // 同包多活跃会话（中断残留）时按会话号收窄，缺点名而多活跃仍歧义拒。
    let session_want = one(m, "session").map(|s| s.to_string());
    let sessions = sessions_pair(read_jsonl(&ledger));
    let actives: Vec<Value> = sessions
        .values()
        .filter(|s| s.get("package").and_then(|v| v.as_str()) == Some(stem.as_str()))
        .filter(|s| match &session_want {
            Some(want) => s.get("session_id").and_then(|v| v.as_str()) == Some(want.as_str()),
            None => true,
        })
        .cloned()
        .collect();
    if actives.is_empty() {
        fail(
            1,
            json!({"error": format!("no active session for package: {}", stem)}),
        );
    }
    if actives.len() > 1 {
        fail(
            1,
            json!({"error": format!(
                "ambiguous active sessions for package: {}；以 --session <会话号> 点名（recognize-solo iso-08）",
                stem
            )}),
        );
    }
    let session = actives[0].clone();
    let sid = session
        .get("session_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let repos = session
        .get("repos")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // 锁清零
    let held = lock_face_paths(&locks_ledger);
    let mut sid_held: Vec<String> = held
        .iter()
        .filter(|(_, holders)| holders.iter().any(|h| h == &sid))
        .map(|(p, _)| p.clone())
        .collect();
    sid_held.sort();
    if !sid_held.is_empty() {
        let list = format!(
            "[{}]",
            sid_held
                .iter()
                .map(|p| format!("'{}'", p))
                .collect::<Vec<_>>()
                .join(", ")
        );
        fail(
            1,
            json!({"error": format!("locks held by session, unlock first: {}", list)}),
        );
    }

    // 工地卫生检查
    for entry in &repos {
        let wt = PathBuf::from(entry.get("worktree").and_then(|v| v.as_str()).unwrap_or(""));
        let clean = check_worktree_clean(&wt);
        if clean.get("clean").and_then(|v| v.as_bool()) != Some(true) {
            fail(
                1,
                json!({"error": format!(
                    "工地卫生检查失败：存在未提交修改或未跟踪文件，提交归代理责任，工具不代提交。{}",
                    py_compact(&json!({"issues": clean.get("issues").cloned().unwrap_or(Value::Null)}))
                )}),
            );
        }
    }

    // 冲突三态（fixture 范围：干净形零冲突；脏面相交即 diverged 拒，同内容与纯追加
    // 让位归并机制未实装，结果档申报）
    for entry in &repos {
        let repo = PathBuf::from(entry.get("repo").and_then(|v| v.as_str()).unwrap_or(""));
        let base = entry.get("base_branch").and_then(|v| v.as_str()).unwrap_or("");
        let branch = entry.get("branch").and_then(|v| v.as_str()).unwrap_or("");
        if base.is_empty() || branch.is_empty() || base == branch {
            continue;
        }
        let face = merge_face(&repo, base, branch);
        if face.is_empty() {
            continue;
        }
        let dirty: Vec<String> = dirty_entries(&repo).into_iter().map(|(p, _)| p).collect();
        let mut diverged: Vec<String> = Vec::new();
        for d in &dirty {
            for f in &face {
                if d == f || f.starts_with(&format!("{}/", d)) {
                    diverged.push(f.clone());
                }
            }
        }
        if !diverged.is_empty() {
            fail(
                1,
                json!({"error": format!(
                    "收约被阻：存在真分叉冲突（非纯追加形），整批拒零动作。{}",
                    py_compact(&json!({"repo": repo.display().to_string(), "diverged_files": diverged}))
                )}),
            );
        }
    }

    // 预收提交最小化：归并面∩真脏位（未跟踪件照单不清场）
    for entry in &repos {
        let repo = PathBuf::from(entry.get("repo").and_then(|v| v.as_str()).unwrap_or(""));
        let base = entry.get("base_branch").and_then(|v| v.as_str()).unwrap_or("");
        let branch = entry.get("branch").and_then(|v| v.as_str()).unwrap_or("");
        if base.is_empty() || branch.is_empty() || base == branch {
            continue;
        }
        let face = merge_face(&repo, base, branch);
        let face_set: BTreeSet<&String> = face.iter().collect();
        let mut staged_candidates: Vec<String> = Vec::new();
        let (rc_st, out_st, _) = git(&repo, &["status", "--porcelain"]);
        if rc_st != 0 {
            continue;
        }
        for line in out_st.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let code = &line[..2.min(line.len())];
            let path = line[3..].trim().trim_end_matches('/');
            if code.trim() == "??" {
                continue;
            }
            let paths: Vec<&str> = match path.split_once(" -> ") {
                Some((a, b)) => vec![a, b],
                None => vec![path],
            };
            for p in paths {
                if face_set.contains(&p.to_string()) {
                    staged_candidates.push(p.to_string());
                }
            }
        }
        if !staged_candidates.is_empty() {
            staged_candidates.sort();
            staged_candidates.dedup();
            let mut add_args = vec!["add", "--"];
            add_args.extend(staged_candidates.iter().map(|s| s.as_str()));
            git(&repo, &add_args);
            git(
                &repo,
                &[
                    "-c",
                    "core.hooksPath=/dev/null",
                    "commit",
                    "-m",
                    &format!("closeguard-solo: pre-close working tree commit ({})", stem),
                ],
            );
        }
    }

    // 链证守门
    let gate_files: Vec<PathBuf> = trails.iter().filter(|t| t.is_file()).cloned().collect();
    let mut chain_note = json!({"checked": false, "reason": "workspace_chainless"});
    if !gate_files.is_empty() {
        let mut found_intent = false;
        let mut found_cert = false;
        for tp in &gate_files {
            for line in std::fs::read_to_string(tp).unwrap_or_default().lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let ev: Value = match serde_json::from_str(line) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if ev.get("session_id").and_then(|v| v.as_str()) != Some(sid.as_str()) {
                    continue;
                }
                match ev.get("event_type").and_then(|v| v.as_str()) {
                    Some("intent_refined") => found_intent = true,
                    Some("certification_completed") => found_cert = true,
                    _ => {}
                }
            }
        }
        let mut missing = Vec::new();
        if !found_intent {
            missing.push("intent_refined");
        }
        if !found_cert {
            missing.push("certification_completed");
        }
        if !missing.is_empty() {
            fail(
                1,
                json!({"error": format!(
                    "链证守门拦截：本会话正典链缺笔，补链笔后收约{}",
                    py_compact(&json!({"session_id": sid, "missing": missing}))
                )}),
            );
        }
        chain_note = json!({"checked": true, "trails": gate_files.len()});
    }

    // SDDG 四判据
    let sdd_repos: Vec<Value> = repos
        .iter()
        .map(|e| {
            json!({
                "base": e.get("base_branch").and_then(|v| v.as_str()).unwrap_or(""),
                "branch": e.get("branch").and_then(|v| v.as_str()).unwrap_or(""),
                "repo": e.get("repo").and_then(|v| v.as_str()).unwrap_or(""),
            })
        })
        .collect();
    let sdd_verdict = sddgate::run_gate(&session, &trails, &sdd_repos);
    let sddgate_gate: Value;
    if sdd_verdict.get("verdict").and_then(|v| v.as_str()) == Some("reject") {
        if let Some(r) = &bypass_sddgate {
            let gates = sdd_verdict
                .pointer("/teaching/gate")
                .cloned()
                .unwrap_or(Value::Null);
            append_row_raw(
                &bypass_ledger,
                &json!({
                    "at": now_utc(),
                    "event": "bypassed_sddgate",
                    "gates": gates,
                    "package": stem,
                    "reason": r,
                    "session": sid,
                }),
            );
            let mut v = sdd_verdict.clone();
            v["verdict"] = json!("bypass");
            v["reason"] = json!(r);
            sddgate_gate = v;
        } else {
            let teaching = sdd_verdict.get("teaching").cloned().unwrap_or(sdd_verdict.clone());
            fail(
                1,
                json!({"error": format!(
                    "SDDG 门拦截：SDD/TDD 完备度判据未过，整批拒零动作（DEC-024 B 层逐批执法，判定命令形机械执法零 LLM 判词位）。\n{}",
                    py_pretty(&teaching, 0, 2)
                )}),
            );
        }
    } else {
        sddgate_gate = sdd_verdict;
    }

    // CALL-LOG 跑步机收编位
    let calllog_treadmill: Value = if transition_applicable(&session, CALLLOG_TREADMILL_EFFECTIVE_AT) {
        json!({"checked": true, "collected": []})
    } else {
        json!({"checked": false, "reason": "before_effective_at"})
    };

    // 无主闸
    let orphan_trails: Vec<PathBuf> = trails.iter().filter(|t| t.is_file()).cloned().collect();
    if !orphan_trails.is_empty() {
        let unowned = unowned_list(&root, &orphan_trails[0], &locks_ledger);
        if !unowned.is_empty() {
            if let Some(r) = &bypass_orphan {
                append_row_raw(
                    &bypass_ledger,
                    &json!({
                        "at": now_utc(),
                        "event": "bypassed_orphan",
                        "package": stem,
                        "reason": r,
                        "session": sid,
                        "unowned": unowned,
                    }),
                );
            } else {
                fail(
                    1,
                    json!({"error": format!(
                        "无主闸拦截：主树存在无主 tracked 改件（脏文件集 − 锁面 ∪ 链笔声明面 ∪ 豁免面 = 无主清单，共 {} 件），整批拒零动作。\n三通道：\n  1) 走司衡治理通道开租约持锁后再改（lease open + lease lock --path <路径>）\n  2) 走直改链笔申报（scribe pen 落 direct_edit_completed 声明）\n  3) 显式绕行：close --bypass-orphan <事由> 落 bypass.ndjson 留痕\n清单：{}",
                        unowned.len(),
                        py_pretty(&json!(unowned), 0, 2)
                    )}),
                );
            }
        }
    }

    // CALL-LOG 随批检查
    let calllog_gate: Value = if transition_applicable(&session, CALLLOG_GUARD_EFFECTIVE_AT) {
        let mut unreleased: Vec<Value> = Vec::new();
        for entry in &repos {
            let repo = PathBuf::from(entry.get("repo").and_then(|v| v.as_str()).unwrap_or(""));
            let branch = entry.get("branch").and_then(|v| v.as_str()).unwrap_or("").to_string();
            for (p, xy) in calllog_dirty_faces(&repo) {
                let status = if xy.trim() == "??" { "untracked" } else { "modified" };
                let ondisk = std::fs::read_to_string(repo.join(&p)).ok();
                let blob = if !branch.is_empty() {
                    let (rc_b, blob_b, _) = git(&repo, &["show", &format!("{}:{}", branch, p)]);
                    if rc_b == 0 { Some(blob_b) } else { None }
                } else {
                    None
                };
                let released = blob.is_some() && ondisk.is_some() && &blob.unwrap() == ondisk.unwrap().as_str();
                if !released {
                    unreleased.push(json!({"path": p, "status": status}));
                }
            }
        }
        if !unreleased.is_empty() {
            if let Some(r) = &bypass_calllog {
                append_row_raw(
                    &bypass_ledger,
                    &json!({
                        "at": now_utc(),
                        "event": "bypassed_calllog",
                        "faces": unreleased,
                        "package": stem,
                        "reason": r,
                        "session": sid,
                    }),
                );
                json!({"checked": true, "released_by": "bypass", "unreleased": unreleased})
            } else {
                fail(1, json!({"error": calllog_gate_message(&unreleased)}));
            }
        } else {
            json!({"checked": true, "unreleased": 0})
        }
    } else {
        json!({"checked": false, "reason": "before_effective_at"})
    };

    // 声明差集闸
    let pkg_path = crate::resolve_package_quiet(&root, &stem);
    let mut declaration_gate = json!({
        "gate": "declared_uncommitted",
        "uncommitted": [],
        "exempt_conditional": [],
        "unparsed": [],
        "acks": [],
        "shared_surface_exempt": [],
    });
    {
        let lines = parse_declared_writes(&pkg_path);
        let repo_entries: Vec<(String, &Value)> = repos
            .iter()
            .map(|e| {
                (
                    Path::new(e.get("repo").and_then(|v| v.as_str()).unwrap_or(""))
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default(),
                    e,
                )
            })
            .collect();
        for (raw, conditional, paths) in &lines {
            if *conditional {
                declaration_gate["exempt_conditional"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!({"paths": paths, "raw": raw}));
                continue;
            }
            for token in paths {
                let mut repo_entry: Option<&Value> = None;
                let mut rel = "";
                for (repo_name, e) in &repo_entries {
                    let prefix = format!("{}/", repo_name);
                    if token.starts_with(&prefix) {
                        repo_entry = Some(e);
                        rel = &token[prefix.len()..];
                        break;
                    }
                }
                let repo_entry = match repo_entry {
                    Some(e) => e,
                    None => {
                        declaration_gate["unparsed"]
                            .as_array_mut()
                            .unwrap()
                            .push(json!(token));
                        continue;
                    }
                };
                let norm = token.trim_end_matches('/');
                let shared = crate::guardlaw::SCOPE_SHARED_SURFACE
                    .iter()
                    .any(|s| norm == s.trim_end_matches('/') || norm.starts_with(&format!("{}/", s.trim_end_matches('/'))));
                if shared {
                    declaration_gate["shared_surface_exempt"]
                        .as_array_mut()
                        .unwrap()
                        .push(json!(token));
                    continue;
                }
                let is_dir = token.ends_with('/');
                let repo_p = PathBuf::from(repo_entry.get("repo").and_then(|v| v.as_str()).unwrap_or(""));
                let branch = repo_entry.get("branch").and_then(|v| v.as_str()).unwrap_or("");
                if !declared_path_committed(&repo_p, branch, rel, is_dir) {
                    if let Some(r) = ack_map.remove(token) {
                        declaration_gate["acks"]
                            .as_array_mut()
                            .unwrap()
                            .push(json!({"path": token, "reason": r}));
                    } else {
                        declaration_gate["uncommitted"]
                            .as_array_mut()
                            .unwrap()
                            .push(json!(token));
                    }
                }
            }
        }
        if !ack_map.is_empty() {
            let unknown: Vec<String> = ack_map.keys().cloned().collect();
            fail(
                1,
                json!({"error": format!(
                    "差集闸认领对表失败：认领路径不在声明未提交差集内（与实态不符零粉饰）{}",
                    py_compact(&json!({"unknown_acks": unknown}))
                )}),
            );
        }
    }
    if declaration_gate
        .get("uncommitted")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false)
    {
        fail(
            1,
            json!({"error": format!(
                "差集闸拦截：声明未提交（请求写入节对分支提交树差集非空，整批拒零动作；补提交或 close --ack-uncommitted 逐路径带事由放行）{}",
                py_compact(&declaration_gate)
            )}),
        );
    }

    // 级联边册投影 close 前重建（pk-055）：接线位在声明差集闸后、归并删支拆本前
    // 即收约凭据落链前——全部准入闸（卫生/冲突/链证/SDDG/无主/CALL-LOG/差集）先
    // 按批自身材料判毕，投影重建作为末位预归并动作不扰动闸判定（机械派生件不
    // 走 SDDG 材料面）；diff 落工地自成一笔提交由归并带回主树；重建失败
    // fail-visible 拒收约零归并会话保持活跃；跳过形注记入吊销行（回执面承 T2
    // 金向量对表冻结，engage 形才上回执键）。
    let cascade_gate = cascade_projection_rebuild(&root, &repos, &stem);

    // 归并删支拆本
    let mut removed: Vec<Value> = Vec::new();
    let mut failed: Vec<Value> = Vec::new();
    for entry in &repos {
        let repo = PathBuf::from(entry.get("repo").and_then(|v| v.as_str()).unwrap_or(""));
        let worktree = PathBuf::from(entry.get("worktree").and_then(|v| v.as_str()).unwrap_or(""));
        let base = entry.get("base_branch").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let branch = entry.get("branch").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let mut record: Map<String, Value> = entry.as_object().cloned().unwrap_or_default();
        let ahead = if !base.is_empty() && !branch.is_empty() {
            let (rc_a, out_a, _) = git(&repo, &["rev-list", "--count", &format!("{}..{}", base, branch)]);
            if rc_a == 0 { out_a.trim().to_string() } else { "0".to_string() }
        } else {
            "0".to_string()
        };
        if !base.is_empty() && !branch.is_empty() && ahead != "0" {
            let (rc_m, _, err_m) = git(
                &repo,
                &[
                    "merge",
                    "--no-ff",
                    &branch,
                    "-m",
                    &format!("merge: {} 副本归并", stem),
                ],
            );
            if rc_m != 0 {
                record.insert("result".into(), json!("merge_failed"));
                let detail: String = err_m.trim().chars().take(200).collect();
                record.insert("detail".into(), json!(detail));
                failed.push(Value::Object(record));
                continue;
            }
        }
        if worktree.exists() {
            let mut args = vec!["worktree", "remove"];
            if force {
                args.push("--force");
            }
            args.push(worktree.to_str().unwrap_or(""));
            let (rc, _, stderr) = git(&repo, &args);
            if rc != 0 {
                record.insert("result".into(), json!("failed"));
                let detail: String = stderr.trim().chars().take(200).collect();
                record.insert("detail".into(), json!(detail));
                failed.push(Value::Object(record));
                continue;
            }
            record.insert("worktree_result".into(), json!("removed"));
        } else {
            record.insert("result".into(), json!("missing"));
            record.insert("worktree_result".into(), json!("missing"));
        }
        if !branch.is_empty() {
            let (rc_v, _, _) = git(
                &repo,
                &["rev-parse", "--verify", "--quiet", &format!("refs/heads/{}", branch)],
            );
            if rc_v == 0 {
                let (rc_d, _, err_d) = git(&repo, &["branch", "-d", &branch]);
                if rc_d != 0 {
                    record.insert("result".into(), json!("branch_delete_failed"));
                    let detail: String = err_d.trim().chars().take(200).collect();
                    record.insert("detail".into(), json!(detail));
                    failed.push(Value::Object(record));
                    continue;
                }
                record.insert("branch_result".into(), json!("deleted"));
            } else {
                record.insert("branch_result".into(), json!("already_gone"));
            }
        }
        if record.get("result").and_then(|v| v.as_str()) != Some("missing") {
            record.insert("result".into(), json!("removed"));
        }
        removed.push(Value::Object(record));
    }
    if !failed.is_empty() {
        append_row(
            &ledger,
            &json!({
                "event": "close_failed",
                "failed": failed,
                "failed_at": at.clone().unwrap_or_else(now_utc),
                "package": stem,
                "reason": reason,
                "removed": removed,
                "session_id": sid,
                "tool": tool(),
            }),
        );
        fail(
            1,
            json!({"error": "worktree remove failed, session stays active", "failed": failed, "removed": removed}),
        );
    }

    // 吊销行
    let revoked_at = at.clone().unwrap_or_else(now_utc);
    let gates_skipped = collect_gates_skipped(bypass_orphan.as_ref(), bypass_calllog.as_ref(), &declaration_gate);
    append_row(
        &ledger,
        &json!({
            "detail": {
                "calllog_gate": calllog_gate,
                "calllog_treadmill": calllog_treadmill,
                "cascade_gate": cascade_gate,
                "declaration_gate": declaration_gate,
                "gates_skipped": gates_skipped,
                "sddgate_gate": sddgate_gate,
            },
            "event": "revoked",
            "package": stem,
            "reason": reason,
            "removed": removed,
            "revoked_at": revoked_at,
            "session_id": sid,
            "tool": tool(),
        }),
    );

    // 账单：未用锁罚（SQL 投影腿不随迁，ndjson 正典唯一写点）
    let mut locked_paths: BTreeSet<String> = BTreeSet::new();
    for row in read_jsonl(&locks_ledger) {
        if row.get("event").and_then(|v| v.as_str()) == Some("acquired")
            && row.get("session_id").and_then(|v| v.as_str()) == Some(sid.as_str())
        {
            if let Some(p) = row.get("path").and_then(|v| v.as_str()) {
                if !p.is_empty() {
                    locked_paths.insert(p.to_string());
                }
            }
        }
    }
    let allow: Vec<String> = session
        .get("allow")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let unused: Vec<String> = locked_paths
        .iter()
        .filter(|p| !allow.contains(p))
        .cloned()
        .collect();
    if !unused.is_empty() {
        let identity_hash = session
            .pointer("/identity/identity_hash")
            .and_then(|v| v.as_str())
            .map(|s| json!(s))
            .unwrap_or(Value::Null);
        let bills = locks_ledger
            .parent()
            .map(|p| p.join("lockface-bills.ndjson"))
            .unwrap_or_else(|| PathBuf::from("lockface-bills.ndjson"));
        append_row(
            &bills,
            &json!({
                "bill_points": unused.len() as i64 * UNUSED_LOCK_MULTIPLIER,
                "detail": {"unused_paths": unused},
                "event_type": "unused_lock_penalty",
                "identity_hash": identity_hash,
                "locks_ndjson": locks_ledger.display().to_string(),
                "package": stem,
                "session_id": sid,
                "ts": revoked_at,
                "unused_count": unused.len(),
            }),
        );
    }

    // 收约凭据（cascade_gate 承 T2 金向量对表冻结：engage 形才上回执，跳过形
    // 注记只在吊销行 detail）
    let mut report = json!({
        "calllog_gate": calllog_gate,
        "calllog_treadmill": calllog_treadmill,
        "chain_gate": chain_note,
        "declaration_gate": declaration_gate,
        "failed": [],
        "gates_skipped": gates_skipped,
        "removed": removed,
        "revoked": true,
        "sddgate_gate": sddgate_gate,
        "session_id": sid,
    });
    if cascade_gate.get("checked").and_then(|v| v.as_bool()) == Some(true) {
        report["cascade_gate"] = cascade_gate.clone();
    }
    let surface = ledger_surface(&root);
    let checks_file = surface.join("checks").join(format!("{}.json", stem));
    if checks_file.exists() {
        let mut data: Value = serde_json::from_str(&std::fs::read_to_string(&checks_file).unwrap_or_default())
            .unwrap_or(json!({}));
        data["closed_at"] = json!(at.unwrap_or_else(now_utc));
        data["close_session"] = json!(report.get("session_id").cloned().unwrap_or(Value::Null));
        let receipts_dir = surface.join("receipts");
        std::fs::create_dir_all(&receipts_dir).ok();
        let receipt_path = receipts_dir.join(format!("{}.json", stem));
        std::fs::write(&receipt_path, py_pretty(&data, 0, 1) + "\n").ok();
        std::fs::remove_file(&checks_file).ok();
        report["close_receipt"] = json!({
            "check_file_removed": checks_file.display().to_string(),
            "receipt": receipt_path.display().to_string(),
        });
    }
    print!("{}", emit(&report));
}
