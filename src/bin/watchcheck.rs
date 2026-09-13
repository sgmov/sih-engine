//! 引擎侧稽（Watchcheck）命令行面 —— lease-mergeleg23-parallel 簇F 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/watchcheck 0.2.0（src/watchcheck/：core.py 274 行、
//! constants.py 65 行、cli.py 38 行），只读对表移植，围堰源码零改动。核心行为链：
//! git status --porcelain -uall -z --no-renames 双仓脏文件集解析、锁面镜像 ndjson
//! acquired/released 配对、当日 trail 直改链笔声明面、豁免面冻结登记，判定式
//! 脏文件集 −（租约锁面 ∪ 链笔声明面 ∪ 豁免面）= 无主清单，只呈报不代裁。
//!
//! CLI：`watchcheck check --at <YYYY-MM-DD> --root <工作区根> [--locks <路径>] [--trail <路径>]...`
//! 退出码三值：0 = 净态；1 = 有无主修改（呈报位）；2 = 工具自身异常（fail-closed）。
//!
//! 落差申报（相对围堰）：
//! 1. 围堰 _lockface_wide_alarms 内含多处 pass 体死分支，本件按其有效行为移植：
//!    repo_root_lock（norm 去尾斜杠恰为仓根）、whole_repo_lock（norm == 仓根/仓根
//!    罕见形）、session_over_threshold（单会话持锁超 20，按 sid 排序）。
//! 2. 无主件为已删除路径时 stat 失败：围堰未捕获 FileNotFoundError 栈回退退出 1，
//!    本件以「(stat 失败)」占位继续呈报其余件。
//! 3. git 二进制缺席：围堰 FileNotFoundError 未捕获退出 1，本件按 ToolError 退出 2
//!    （fail-closed 同向，码值对齐异常路径）。
//! 4. 锁面 session_id 缺席（null）时呈报按 "None" 显示串参与排序（围堰混排 None 与
//!    str 会 TypeError，此为近似承载）。
//! 5. core.py 的库函数 unowned_list 不在围堰 CLI 面上，未单列（判定式与 check 同源）。
//! 6. stderr 报文与呈报文本形逐字对齐；异常内文形近似（见落差三、四）。

use serde_json::{json, Map, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use chrono::{DateTime, Utc};

/// 围堰版本锚：sih-tools/watchcheck/src/watchcheck/__init__.py VERSION。
const VERSION: &str = "0.2.0";
const REPOS: [&str; 2] = ["sih-tools", "sih-engine"];
const JUDGMENT_FORMULA: &str = "脏文件集 −（租约锁面 ∪ 直改链笔声明面 ∪ 豁免面）= 无主清单";
const EXIT_CLEAN: i32 = 0;
const EXIT_UNOWNED: i32 = 1;
const EXIT_TOOL_ERROR: i32 = 2;
const LOCKFACE_WIDE_THRESHOLD: usize = 20;
/// 豁免面冻结登记（constants.py EXEMPTION_FACES 逐字对表，16 面）。
const EXEMPTION_FACES: [&str; 16] = [
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
];

struct ToolError(String);

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: watchcheck check --at <YYYY-MM-DD> --root <工作区根> [--locks <路径>] [--trail <路径>]...");
    exit(2);
}

// ---------- 脏文件集：porcelain -uall -z --no-renames 对表 ----------

/// 单仓脏文件清单：[(工作区相对路径, XY 码)]。
fn git_dirty_entries(repo_root: &Path) -> Result<Vec<(String, String)>, ToolError> {
    let out = Command::new("git")
        .args([
            "-C",
            &repo_root.to_string_lossy(),
            "status",
            "--porcelain",
            "-uall",
            "-z",
            "--no-renames",
        ])
        .output()
        .map_err(|e| ToolError(format!("git status 失败（{}）：{e}", repo_root.display())))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(ToolError(format!(
            "git status 失败（{}）：{stderr}",
            repo_root.display()
        )));
    }
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let mut entries = vec![];
    for rec in stdout.split('\0') {
        if rec.is_empty() {
            continue;
        }
        let bytes = rec.as_bytes();
        let xy: String = String::from_utf8_lossy(&bytes[..bytes.len().min(2)]).into_owned();
        let rel: String = if bytes.len() >= 3 {
            String::from_utf8_lossy(&bytes[3..]).into_owned()
        } else {
            String::new()
        };
        entries.push((rel, xy));
    }
    Ok(entries)
}

/// 双仓脏文件集：工作区根相对路径，按 (路径, XY) 排序。
fn dirty_set(root: &Path) -> Result<Vec<(String, String)>, ToolError> {
    let mut dirty = vec![];
    for name in REPOS {
        let repo = root.join(name);
        for (rel, xy) in git_dirty_entries(&repo)? {
            dirty.push((format!("{name}/{rel}"), xy));
        }
    }
    dirty.sort();
    Ok(dirty)
}

// ---------- 三对照面 ----------

/// 锁面：镜像 ndjson acquired/released 配对，路径 → [mode, session_id]（Value 数组）。
fn lock_face(locks_path: &Path) -> Map<String, Value> {
    let mut held: Vec<((String, Value), String)> = vec![];
    let text = match fs::read_to_string(locks_path) {
        Ok(t) => t,
        Err(_) => return Map::new(),
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
        let path = py_display_key(ev.get("path").unwrap_or(&Value::Null));
        let sid = ev.get("session_id").cloned().unwrap_or(Value::Null);
        match ev.get("event").and_then(|v| v.as_str()) {
            Some("acquired") => {
                let mode = match ev.get("mode") {
                    Some(Value::String(m)) if !m.is_empty() => m.clone(),
                    _ => "exclusive".to_string(),
                };
                held.push(((path, sid), mode));
            }
            Some("released") => {
                held.retain(|((p, s), _)| !(*p == path && *s == sid));
            }
            _ => {}
        }
    }
    // 围堰 dict 推导：同路径后写覆盖、位置保持（IndexMap 语义）
    let mut held_map: Map<String, Value> = Map::new();
    for ((p, s), mode) in held {
        held_map.insert(p, json!([mode, s]));
    }
    held_map
}

fn py_display_key(v: &Value) -> String {
    match v {
        Value::Null => "None".to_string(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        other => other.to_string(),
    }
}

/// 声明面：当日 trail 的 direct_edit_completed 事件 files 并集。
fn pen_face(trail_paths: &[String]) -> Result<BTreeSet<String>, ToolError> {
    let mut declared = BTreeSet::new();
    for tp in trail_paths {
        let path = Path::new(tp);
        if !path.exists() {
            return Err(ToolError(format!(
                "当日 trail 缺席（挂点序保证 gauge record 落链先行）：{path:?}"
            )));
        }
        let text = fs::read_to_string(path)
            .map_err(|e| ToolError(format!("当日 trail 读取失败（{path:?}）：{e}")))?;
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let ev: Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if ev.get("event_type").and_then(|v| v.as_str()) == Some("direct_edit_completed") {
                if let Some(files) = ev.get("details").and_then(|d| d.get("files")).and_then(|f| f.as_array()) {
                    for f in files {
                        if let Some(s) = f.as_str() {
                            declared.insert(s.to_string());
                        }
                    }
                }
            }
        }
    }
    Ok(declared)
}

/// 覆盖判定：声明笔精确 → 租约锁精确或目录前缀 → 豁免面前缀；余集即无主。
fn covered(ws_path: &str, locks: &Map<String, Value>, declared: &BTreeSet<String>) -> Option<&'static str> {
    if declared.contains(ws_path) {
        return Some("声明笔");
    }
    for (lp, _) in locks {
        if lp == ws_path {
            return Some("租约锁");
        }
        if lp.ends_with('/') && format!("{ws_path}/").starts_with(lp.as_str()) {
            return Some("租约锁");
        }
    }
    for ef in EXEMPTION_FACES {
        if format!("{ws_path}/").starts_with(ef) {
            return Some("豁免面");
        }
    }
    None
}

// ---------- 呈报 ----------

fn status_label(xy: &str) -> String {
    let base = |k: &str| match k {
        "??" => Some("未跟踪新件"),
        "M" => Some("已跟踪修改"),
        "A" => Some("已暂存新增"),
        "D" => Some("已删除"),
        _ => None,
    };
    if let Some(b) = base(xy) {
        return b.to_string();
    }
    let stripped = xy.trim();
    let k = if stripped.is_empty() { xy } else { stripped };
    if let Some(b) = base(k) {
        return b.to_string();
    }
    format!("git 态 {k}")
}

/// UTC ISO8601 秒级 mtime（围堰 timespec seconds 同形）。
fn mtime_iso(abspath: &Path) -> String {
    match fs::metadata(abspath).and_then(|m| m.modified()) {
        Ok(t) => DateTime::<Utc>::from(t).to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        Err(_) => "(stat 失败)".to_string(),
    }
}

/// 锁面超宽面哨兵（leaseup-solo T-4）有效行为移植（见头注落差一）。只报不拦。
fn lockface_wide_alarms(locks: &Map<String, Value>) -> Vec<(String, String)> {
    let mut alarms = vec![];
    for (path, v) in locks {
        let sid = v.get(1).map(py_display_key).unwrap_or_default();
        let norm = path.trim_end_matches('/');
        if REPOS.contains(&norm) {
            alarms.push(("repo_root_lock".to_string(), format!("path={path} session={sid}")));
        }
    }
    for (path, v) in locks {
        let sid = v.get(1).map(py_display_key).unwrap_or_default();
        let norm = path.trim_end_matches('/');
        for root in REPOS {
            if norm == format!("{root}/{root}") {
                alarms.push(("whole_repo_lock".to_string(), format!("path={path} session={sid}")));
            }
        }
    }
    let mut counts: Vec<(String, usize)> = vec![];
    for (_, v) in locks {
        let sid = v.get(1).map(py_display_key).unwrap_or_else(|| "None".to_string());
        if let Some(e) = counts.iter_mut().find(|(s, _)| *s == sid) {
            e.1 += 1;
        } else {
            counts.push((sid, 1));
        }
    }
    counts.sort();
    for (sid, count) in counts {
        if count > LOCKFACE_WIDE_THRESHOLD {
            alarms.push((
                "session_over_threshold".to_string(),
                format!("session={sid} locks={count} threshold={LOCKFACE_WIDE_THRESHOLD}"),
            ));
        }
    }
    alarms
}

/// 对表判定：返回 (退出码, 呈报文本)。只呈报不代裁。
fn judge(root: &Path, at: &str, trail_paths: &[String], locks_path: &Path) -> Result<(i32, String), ToolError> {
    let dirty = dirty_set(root)?;
    let locks = lock_face(locks_path);
    let declared = pen_face(trail_paths)?;
    let mut unowned: Vec<(String, String)> = vec![];
    for (ws_path, xy) in &dirty {
        if covered(ws_path, &locks, &declared).is_none() {
            unowned.push((ws_path.clone(), xy.clone()));
        }
    }
    let wide_alarms = lockface_wide_alarms(&locks);
    let trails_joined = trail_paths.join(" 与 ");
    let mut lines = vec![
        format!("稽 watchcheck 对表读数 {at}（v{VERSION}）"),
        format!("判定式：{JUDGMENT_FORMULA}"),
        format!(
            "输入面：双仓 git status --porcelain -uall；锁面镜像 {}（现势 {} 面）；声明面 {}（声明 {} 件）；豁免面冻结登记 {} 面（constants.py）",
            locks_path.to_string_lossy(),
            locks.len(),
            trails_joined,
            declared.len(),
            EXEMPTION_FACES.len(),
        ),
        format!("扫描：脏文件 {} 件", dirty.len()),
    ];
    if unowned.is_empty() {
        lines.push("结论：净态，无主修改零处。人节点零动作（工程基线三：注意力只投异常信号）。".to_string());
        if !wide_alarms.is_empty() {
            lines.push(format!(
                "锁面超宽面哨兵呈报（leaseup-solo T-4）：{} 项，只报不拦——超阈值 LOCKFACE_WIDE_THRESHOLD={LOCKFACE_WIDE_THRESHOLD}：",
                wide_alarms.len()
            ));
            for (kind, payload) in &wide_alarms {
                lines.push(format!("  - {kind}: {payload}"));
            }
        }
        return Ok((EXIT_CLEAN, lines.join("\n") + "\n"));
    }
    lines.push(format!(
        "结论：无主修改 {} 件，呈人节点二值裁决——「我的」：走司衡治理通道（域内管线三步加直改链笔）；「不是我的」：机械回滚（git restore 或对未跟踪件删除，操作见 BATCH-FACE 处置协议节；本工具零代行）。",
        unowned.len()
    ));
    lines.push("无主清单：".to_string());
    for (ws_path, xy) in &unowned {
        let mt = mtime_iso(&root.join(ws_path));
        lines.push(format!("- {ws_path} | mtime {mt} | {}", status_label(xy)));
    }
    if !wide_alarms.is_empty() {
        lines.push(format!(
            "\n锁面超宽面哨兵呈报（leaseup-solo T-4）：{} 项，只报不拦：",
            wide_alarms.len()
        ));
        for (kind, payload) in &wide_alarms {
            lines.push(format!("  - {kind}: {payload}"));
        }
    }
    Ok((EXIT_UNOWNED, lines.join("\n") + "\n"))
}

// ---------- CLI ----------

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--version" => {
                println!("watchcheck {VERSION}");
                exit(0);
            }
            a if a.starts_with("--") => usage_fail(&format!("未知旗标: {a}")),
            _ => break,
        }
    }
    if i >= args.len() {
        usage_fail("缺子命令");
    }
    let sub = args[i].clone();
    let rest: Vec<String> = args[i + 1..].to_vec();
    if sub != "check" {
        usage_fail(&format!("未知子命令: {sub}"));
    }
    let mut at: Option<String> = None;
    let mut root: Option<String> = None;
    let mut locks: Option<String> = None;
    let mut trails: Vec<String> = vec![];
    let mut j = 0usize;
    while j < rest.len() {
        match rest[j].as_str() {
            "--at" => {
                j += 1;
                if j >= rest.len() {
                    usage_fail("--at 缺值");
                }
                at = Some(rest[j].clone());
            }
            "--root" => {
                j += 1;
                if j >= rest.len() {
                    usage_fail("--root 缺值");
                }
                root = Some(rest[j].clone());
            }
            "--locks" => {
                j += 1;
                if j >= rest.len() {
                    usage_fail("--locks 缺值");
                }
                locks = Some(rest[j].clone());
            }
            "--trail" => {
                j += 1;
                if j >= rest.len() {
                    usage_fail("--trail 缺值");
                }
                trails.push(rest[j].clone());
            }
            a if a.starts_with("--") => usage_fail(&format!("未知旗标: {a}")),
            a => usage_fail(&format!("意外位置参数: {a}")),
        }
        j += 1;
    }
    let at = match at {
        Some(a) => a,
        None => usage_fail("缺 --at（必填）"),
    };
    let root = match root {
        Some(r) => PathBuf::from(r),
        None => usage_fail("缺 --root（必填）"),
    };
    let locks_path = locks
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("sih-tools/lease/ledger/locks.ndjson"));
    let trail_paths: Vec<String> = if trails.is_empty() {
        vec![root
            .join("sih-engine/sih/event/trail")
            .join(format!("{at}.ndjson"))
            .to_string_lossy()
            .into_owned()]
    } else {
        trails
    };
    match judge(&root, &at, &trail_paths, &locks_path) {
        Ok((code, report)) => {
            print!("{report}");
            exit(code);
        }
        Err(ToolError(msg)) => {
            eprintln!("稽 watchcheck 工具异常：{msg}");
            exit(EXIT_TOOL_ERROR);
        }
    }
}
