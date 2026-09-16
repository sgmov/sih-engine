//! 判据扫引擎件（critsweep）：对话框内治理态回算器，v1.2.0 的引擎 bin 形。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//! 移植源：sih-tools/critsweep/sweep.py（lease-mergeall-parallel 簇A施工）。
//! 行为链：GOV-002 判据面（registry 判据单源，近窗 trail 事件命名空间字段
//! 扫令牌——record_path/report_path/package 禁全文散文匹配——加证据指针核对
//! 加三态判定）+ 泊界面（selector parking 包路由，单线超时十秒降级）
//! + 在飞面（会话与锁两账本）+ 散文对照面 + 裁判面封印（非缺省阈值缺事由即拒）。
//! 失效只降级不拦截且降级在输出内可见不静默。
//!
//! 落差申报（对表围堰 sweep.py，零粉饰）：
//! 1. registry 定位：围堰自脚本位取 registry.json；引擎 bin 无脚本位，解析序
//!    --registry 显式参 > env CRITSWEEP_REGISTRY > cwd 上溯首个
//!    sih-engine/critsweep/registry.json（引擎位，gap-packs-assets 落位）>
//!    cwd 上溯首个 sih-tools/critsweep/registry.json（围堰兼容候选）>
//!    root/sih-engine/critsweep/registry.json > root/sih-tools/critsweep/registry.json，
//!    全缺即 criteria 空与降级如实。判据单源是 registry.json 文件（引擎位与围堰位
//!    同内容），零内嵌拷贝。
//! 2. referee 台账定位：--referee-ledger 显式参 > env CRITSWEEP_REFEREE_LEDGER
//!    > cwd 上溯 sih-tools/critsweep/ledger/referee.ndjson。
//! 3. 根判别与中央码根：围堰自脚本位上溯，移植改自 cwd 上溯，语义同形
//!    （首个含账本目录者／首个双仓标记者），--root 显式参优先。
//! 4. selector 路由子进程超时以 try_wait 轮询十秒实现，等价 subprocess timeout=10；
//!    自 gap-parking-route-engine-selector 起泊界路由腿直调引擎 selector bin
//!    （本 bin 同目录兄弟位 + 引擎仓 packs/selector/parking），消对围堰
//!    uv run 的运行时依赖（iso-04 locks_read 直调先例同形）；materials 相对形
//!    归一绝对后传参，语义同围堰 cwd 锚定形。
//! 5. 出参 pretty 化（围堰 indent=1 sort_keys）：JSON 语义等价。
//! 6. 命名空间字段非字符串值的 str 化承 Python str() 形（Null→"None"、
//!    Bool→"True"/"False"、数与容器 serde 形与 Python repr 有微差）；
//!    路径与批名主形态是字符串，判定面不受影响。
//! 7. 账本坏行跳行如实（与围堰同）；registry 解析出非对象围堰 AttributeError
//!    裸崩，移植按不可读降级如实。
//! 退出码：0 回算成功（含降级可见与根不可判降级形）／1 裁判面封印拒／
//! 2 用法错（未知旗标、缺值、非整数值）。

use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const VERSION: &str = "1.2.0";
const DEFAULT_THRESHOLD: i64 = 3;
const NAMESPACE_FIELDS: [&str; 3] = ["record_path", "report_path", "package"];
const WINDOW_DAYS: i64 = 10;
const ROUTE_TIMEOUT_S: u64 = 10;

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

fn die(rc: i32, err: &str, detail: Value) -> ! {
    let mut m = Map::new();
    m.insert("error".into(), json!(err));
    if !detail.is_null() {
        m.insert("detail".into(), detail);
    }
    eprintln!("{}", emit(&Value::Object(m)));
    std::process::exit(rc);
}

fn today() -> chrono::NaiveDate {
    chrono::Local::now().date_naive()
}

/// JSON 值转 Python str() 风字符串（命名空间字段 str() 对表）
fn py_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "None".to_string(),
        Value::Bool(b) => {
            if *b {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        other => other.to_string(),
    }
}

struct Args {
    at: Option<String>,
    root: Option<String>,
    threshold: i64,
    threshold_reason: Option<String>,
    window: i64,
    criterion: Option<String>,
    registry: Option<String>,
    referee_ledger: Option<String>,
}

fn parse_args(argv: &[String]) -> Args {
    let mut a = Args {
        at: None,
        root: None,
        threshold: DEFAULT_THRESHOLD,
        threshold_reason: None,
        window: WINDOW_DAYS,
        criterion: None,
        registry: None,
        referee_ledger: None,
    };
    let mut i = 0;
    while i < argv.len() {
        let raw = &argv[i];
        let rest = match raw.strip_prefix("--") {
            Some(r) => r,
            None => die(2, &format!("意外位置参 {}", raw), json!(null)),
        };
        let (k, inline): (String, Option<String>) =
            match rest.split_once('=') {
                Some((k, v)) => (k.to_string(), Some(v.to_string())),
                None => (rest.to_string(), None),
            };
        let val = match inline {
            Some(v) => v,
            None => {
                i += 1;
                match argv.get(i) {
                    Some(v) => v.clone(),
                    None => die(2, &format!("旗标 --{} 缺值", k), json!(null)),
                }
            }
        };
        match k.as_str() {
            "at" => a.at = Some(val),
            "root" => a.root = Some(val),
            "threshold" => {
                a.threshold = val.parse().unwrap_or_else(|_| {
                    die(2, &format!("--threshold 非整数：{}", val), json!(null))
                })
            }
            "threshold-reason" => a.threshold_reason = Some(val),
            "window" => {
                a.window = val
                    .parse()
                    .unwrap_or_else(|_| die(2, &format!("--window 非整数：{}", val), json!(null)))
            }
            "criterion" => a.criterion = Some(val),
            "registry" => a.registry = Some(val),
            "referee-ledger" => a.referee_ledger = Some(val),
            _ => die(2, &format!("未知旗标 --{}", k), json!(null)),
        }
        i += 1;
    }
    a
}

fn resolve_root(explicit: Option<&str>) -> (Option<PathBuf>, &'static str) {
    if let Some(r) = explicit {
        if !r.is_empty() {
            let p = PathBuf::from(r);
            let p = std::fs::canonicalize(&p).unwrap_or(p);
            return (Some(p), "arg");
        }
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut cur = Some(cwd.as_path());
    while let Some(c) = cur {
        if c.join("sih-tools/lease/ledger/sessions.ndjson").is_file() {
            return (Some(c.to_path_buf()), "walk");
        }
        if c.join("sih/ledger").is_dir() && c.join("sih/event/trail").is_dir() {
            return (Some(c.to_path_buf()), "walk");
        }
        cur = c.parent();
    }
    for k in ["ZCODE_PROJECT_DIR", "CLAUDE_PROJECT_DIR"] {
        if let Ok(v) = std::env::var(k) {
            if !v.is_empty() {
                let p = PathBuf::from(v);
                let p = std::fs::canonicalize(&p).unwrap_or(p);
                return (Some(p), "env");
            }
        }
    }
    (None, "unresolved")
}

fn detect_layout(root: &Path) -> Option<&'static str> {
    if root.join("sih-engine/Cargo.toml").is_file()
        && root.join("sih-tools/pyproject.toml").is_file()
    {
        return Some("first_domain");
    }
    if root.join("sih/ledger").is_dir() {
        return Some("canonical");
    }
    None
}

/// 中央码根：自 cwd 上溯找首个 first_domain 布局工作区根（落差申报 3）
fn central_code_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut cur = Some(cwd.as_path());
    while let Some(c) = cur {
        if detect_layout(c) == Some("first_domain") {
            return c.to_path_buf();
        }
        cur = c.parent();
    }
    cwd
}

struct Faces {
    layout: &'static str,
    trail: PathBuf,
    sessions: PathBuf,
    locks: PathBuf,
    parking_jobs: Vec<(String, String)>,
    parking_cwd: PathBuf,
}

fn domain_faces(root: &Path) -> Faces {
    if detect_layout(root) == Some("canonical") {
        Faces {
            layout: "canonical",
            trail: root.join("sih/event/trail"),
            sessions: root.join("sih/ledger/sessions.ndjson"),
            locks: root.join("sih/ledger/locks.ndjson"),
            parking_jobs: vec![(
                "domain".to_string(),
                root.join("sih/state/parking/materials")
                    .display()
                    .to_string(),
            )],
            parking_cwd: central_code_root().join("sih-tools"),
        }
    } else {
        let layout = if detect_layout(root) == Some("first_domain") {
            "first_domain"
        } else {
            "unknown"
        };
        Faces {
            layout,
            trail: root.join("sih-engine/sih/event/trail"),
            sessions: root.join("sih-tools/lease/ledger/sessions.ndjson"),
            locks: root.join("sih-tools/lease/ledger/locks.ndjson"),
            parking_jobs: vec![
                (
                    "engine".to_string(),
                    "../sih-engine/sih/state/parking/materials".to_string(),
                ),
                ("tools".to_string(), "parking/materials".to_string()),
            ],
            parking_cwd: root.join("sih-tools"),
        }
    }
}

fn day_window(at: chrono::NaiveDate, window_days: i64) -> Vec<chrono::NaiveDate> {
    (0..window_days)
        .map(|i| at - chrono::Duration::days(window_days - 1 - i))
        .collect()
}

fn registry_path(explicit: Option<&str>, root: &Path) -> PathBuf {
    if let Some(p) = explicit {
        if !p.is_empty() {
            return PathBuf::from(p);
        }
    }
    if let Ok(p) = std::env::var("CRITSWEEP_REGISTRY") {
        if !p.is_empty() {
            return PathBuf::from(p);
        }
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut cur = Some(cwd.as_path());
    while let Some(c) = cur {
        // 引擎位默认候选首位（gap-packs-assets）：引擎树 registry 在前，
        // 围堰位降为后续兼容候选，显式 --registry/--env 覆盖形不变。
        let eng = c.join("sih-engine/critsweep/registry.json");
        if eng.is_file() {
            return eng;
        }
        let cand = c.join("sih-tools/critsweep/registry.json");
        if cand.is_file() {
            return cand;
        }
        cur = c.parent();
    }
    let eng = root.join("sih-engine/critsweep/registry.json");
    if eng.is_file() {
        return eng;
    }
    root.join("sih-tools/critsweep/registry.json")
}

fn load_registry(path: &Path) -> (Vec<Value>, Option<String>) {
    let err = |kind: &str| {
        (
            Vec::new(),
            Some(format!("registry 不可读（降级）：{} {}", path.display(), kind)),
        )
    };
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return err("OSError"),
    };
    let reg: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return err("JSONDecodeError"),
    };
    match reg.get("criteria").and_then(|v| v.as_array()) {
        Some(a) => (a.clone(), None),
        None => (Vec::new(), None),
    }
}

struct Scan {
    per_token_days: BTreeMap<String, BTreeMap<String, i64>>,
    per_token_events: BTreeMap<String, Vec<Value>>,
    prose_only_events: BTreeMap<String, Vec<Value>>,
    events_scanned: i64,
    missing_days: Vec<String>,
    unreadable_days: Vec<Value>,
    degradation: Option<String>,
    chain_events: Vec<Value>,
}

fn scan_trails(trail_dir: &Path, days: &[chrono::NaiveDate], criteria: &[Value]) -> Scan {
    let mut scan = Scan {
        per_token_days: BTreeMap::new(),
        per_token_events: BTreeMap::new(),
        prose_only_events: BTreeMap::new(),
        events_scanned: 0,
        missing_days: Vec::new(),
        unreadable_days: Vec::new(),
        degradation: None,
        chain_events: Vec::new(),
    };
    if !trail_dir.is_dir() {
        scan.missing_days = days.iter().map(|d| d.to_string()).collect();
        scan.degradation =
            Some(format!("trail 目录缺席（降级）：{}", trail_dir.display()));
        return scan;
    }
    let mut tokens: BTreeSet<String> = BTreeSet::new();
    for c in criteria {
        if let Some(ts) = c.get("token_scope").and_then(|v| v.as_array()) {
            for t in ts {
                if let Some(s) = t.as_str() {
                    tokens.insert(s.to_string());
                }
            }
        }
    }
    for t in &tokens {
        scan.per_token_days.insert(t.clone(), BTreeMap::new());
        scan.per_token_events.insert(t.clone(), Vec::new());
        scan.prose_only_events.insert(t.clone(), Vec::new());
    }
    for day in days {
        let path = trail_dir.join(format!("{}.ndjson", day.to_string()));
        if !path.is_file() {
            scan.missing_days.push(day.to_string());
            continue;
        }
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(_) => {
                scan.unreadable_days.push(json!({"day": day.to_string(), "error": "OSError"}));
                continue;
            }
        };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let event: Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(_) => continue,
            };
            scan.events_scanned += 1;
            let details = event
                .get("details")
                .cloned()
                .unwrap_or_else(|| json!({}));
            let namespace = NAMESPACE_FIELDS
                .iter()
                .map(|f| match details.get(*f) {
                    Some(v) => py_str(v),
                    None => String::new(),
                })
                .collect::<Vec<_>>()
                .join(" ");
            let fulltext = line;
            let probe = json!({
                "day": day.to_string(),
                "event_id": event.get("event_id").cloned().unwrap_or(json!("")),
                "event_type": event.get("event_type").cloned().unwrap_or(json!("")),
            });
            for token in &tokens {
                if namespace.contains(token.as_str()) {
                    *scan
                        .per_token_days
                        .get_mut(token)
                        .unwrap()
                        .entry(day.to_string())
                        .or_insert(0) += 1;
                    scan.per_token_events.get_mut(token).unwrap().push(probe.clone());
                } else if fulltext.contains(token.as_str()) {
                    scan.prose_only_events
                        .get_mut(token)
                        .unwrap()
                        .push(probe.clone());
                }
            }
        }
    }
    scan
}

fn opt_eq(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

fn criterion_status(
    crit: &Value,
    scan: &Scan,
    at: chrono::NaiveDate,
    threshold: i64,
    root: &Path,
) -> Value {
    let tokens: Vec<String> = crit
        .get("token_scope")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|t| t.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let mut last_hit: Option<String> = None;
    let mut total_hits: i64 = 0;
    for token in &tokens {
        if let Some(days_map) = scan.per_token_days.get(token) {
            for (day_str, count) in days_map {
                total_hits += count;
                if last_hit.as_ref().map_or(true, |l| day_str > l) {
                    last_hit = Some(day_str.clone());
                }
            }
        }
    }
    let registered_at = crit.get("registered_at").and_then(|v| v.as_str());
    let (gap, gap_basis): (Option<i64>, &str) = if let Some(lh) = &last_hit {
        match chrono::NaiveDate::parse_from_str(lh, "%Y-%m-%d") {
            Ok(d) => (Some((at - d).num_days()), "last_hit"),
            Err(_) => (None, "unavailable"),
        }
    } else if let Some(r) = registered_at {
        match chrono::NaiveDate::parse_from_str(r, "%Y-%m-%d") {
            Ok(d) => (Some((at - d).num_days()), "registered_at"),
            Err(_) => (None, "unavailable"),
        }
    } else {
        (None, "unavailable")
    };

    let empty_vec: Vec<Value> = Vec::new();
    if crit.get("status_kind").and_then(|v| v.as_str()) == Some("evidence") {
        let evidence = crit.get("evidence").cloned().unwrap_or_else(|| json!({}));
        let mut evidence_report: Vec<Value> = Vec::new();
        let mut files_ok = true;
        for spec in evidence
            .get("files")
            .and_then(|v| v.as_array())
            .unwrap_or(&empty_vec)
        {
            let path_str = spec
                .get("path")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let path = root.join(path_str);
            let mut present = path.is_file();
            let contains = spec.get("contains").and_then(|v| v.as_str());
            if present {
                if let Some(c) = contains {
                    present = match std::fs::read_to_string(&path) {
                        Ok(t) => t.contains(c),
                        Err(_) => false,
                    };
                }
            }
            files_ok = files_ok && present;
            evidence_report.push(json!({
                "path": path_str,
                "contains": contains.map(|c| json!(c)).unwrap_or(Value::Null),
                "ok": present,
            }));
        }
        let mut chain_ok = true;
        for spec in evidence
            .get("chain")
            .and_then(|v| v.as_array())
            .unwrap_or(&empty_vec)
        {
            let mut found = false;
            for probe in &scan.chain_events {
                let pd = probe
                    .get("details")
                    .cloned()
                    .unwrap_or_else(|| json!({}));
                if opt_eq(probe.get("event_type"), spec.get("event_type"))
                    && opt_eq(pd.get("entry_id"), spec.get("entry_id"))
                    && opt_eq(pd.get("disposition"), spec.get("disposition"))
                {
                    found = true;
                    break;
                }
            }
            chain_ok = chain_ok && found;
        }
        evidence_report.push(json!({"chain": Value::Array(
            evidence.get("chain").and_then(|v| v.as_array()).cloned().unwrap_or_default()
        ), "ok": chain_ok}));
        let status = if files_ok && chain_ok {
            "achieved"
        } else if gap.map_or(false, |g| g <= threshold) {
            "in_flight"
        } else {
            "sunk"
        };
        return json!({
            "id": crit.get("id").cloned().unwrap_or(Value::Null),
            "title": crit.get("title").cloned().unwrap_or(json!("")),
            "status_kind": "evidence",
            "status": status,
            "expected": "achieved",
            "evidence": evidence_report,
            "last_hit": last_hit,
            "gap_days": gap,
            "gap_basis": gap_basis,
            "namespace_hits": total_hits,
            "token_scope": tokens,
        });
    }
    let status = match gap {
        None => "unknown",
        Some(g) if g > threshold => "sunk",
        Some(_) => "in_flight",
    };
    let mut hit_days = Map::new();
    let mut prose_control: Vec<Value> = Vec::new();
    for t in &tokens {
        let days_list: Vec<String> = scan
            .per_token_days
            .get(t)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();
        hit_days.insert(t.clone(), json!(days_list));
        prose_control.push(json!(
            scan.prose_only_events.get(t).cloned().unwrap_or_default()
        ));
    }
    json!({
        "id": crit.get("id").cloned().unwrap_or(Value::Null),
        "title": crit.get("title").cloned().unwrap_or(json!("")),
        "status_kind": "activity",
        "status": status,
        "last_hit": last_hit,
        "gap_days": gap,
        "gap_basis": gap_basis,
        "namespace_hits": total_hits,
        "token_scope": tokens,
        "hit_days": Value::Object(hit_days),
        "prose_only_control": prose_control,
    })
}

fn scan_chain_events(trail_dir: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    if !trail_dir.is_dir() {
        return out;
    }
    let mut paths: Vec<PathBuf> = match std::fs::read_dir(trail_dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file() && p.extension().map(|x| x == "ndjson").unwrap_or(false))
            .collect(),
        Err(_) => return out,
    };
    paths.sort();
    for path in paths {
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(_) => continue,
        };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let event: Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let et = event.get("event_type").and_then(|v| v.as_str()).unwrap_or("");
            if et == "parking_entered" || et == "parking_exited" {
                out.push(json!({
                    "event_type": event.get("event_type").cloned().unwrap_or(Value::Null),
                    "details": event.get("details").cloned().unwrap_or_else(|| json!({})),
                }));
            }
        }
    }
    out
}

fn inflight_face(sessions_path: &Path, locks_path: &Path) -> (Value, Vec<String>) {
    let mut ledgers = Map::new();
    let mut degradations: Vec<String> = Vec::new();
    let mut state: BTreeMap<String, Value> = BTreeMap::new();
    match std::fs::read_to_string(sessions_path) {
        Ok(text) => {
            for line in text.lines() {
                let e: Value = match serde_json::from_str(line) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if let Some(sid) = e.get("session_id").and_then(|v| v.as_str()) {
                    state.insert(sid.to_string(), e.get("event").cloned().unwrap_or(Value::Null));
                }
            }
            let active = state
                .values()
                .filter(|ev| {
                    let s = ev.as_str().unwrap_or("");
                    s != "closed" && s != "revoked"
                })
                .count();
            ledgers.insert("active_sessions".to_string(), json!(active));
        }
        Err(_) => {
            ledgers.insert("active_sessions".to_string(), json!(null));
            degradations.push(format!("会话账本不可读（降级）：{}", sessions_path.display()));
        }
    }
    let mut lock_state: BTreeMap<String, Value> = BTreeMap::new();
    match std::fs::read_to_string(locks_path) {
        Ok(text) => {
            for line in text.lines() {
                let e: Value = match serde_json::from_str(line) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if let Some(sid) = e.get("session_id").and_then(|v| v.as_str()) {
                    let p = match e.get("path") {
                        Some(v) => py_str(v),
                        None => "None".to_string(),
                    };
                    lock_state.insert(
                        format!("{}|{}", sid, p),
                        e.get("event").cloned().unwrap_or(Value::Null),
                    );
                }
            }
            let held = lock_state
                .values()
                .filter(|ev| ev.as_str() == Some("acquired"))
                .count();
            ledgers.insert("held_locks".to_string(), json!(held));
        }
        Err(_) => {
            ledgers.insert("held_locks".to_string(), json!(null));
            degradations.push(format!("锁账本不可读（降级）：{}", locks_path.display()));
        }
    }
    (Value::Object(ledgers), degradations)
}

enum ProcErr {
    Timeout,
    Fail,
}

struct ProcOut {
    code: Option<i32>,
    stdout: String,
    #[allow(dead_code)]
    stderr: String,
}

fn run_with_timeout(mut cmd: Command, timeout: Duration) -> Result<ProcOut, ProcErr> {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => return Err(ProcErr::Fail),
    };
    let deadline = Instant::now() + timeout;
    let code;
    loop {
        match child.try_wait() {
            Ok(Some(st)) => {
                code = st.code();
                break;
            }
            Ok(None) => {}
            Err(_) => return Err(ProcErr::Fail),
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(ProcErr::Timeout);
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    let mut so = String::new();
    if let Some(mut h) = child.stdout.take() {
        let _ = Read::read_to_string(&mut h, &mut so);
    }
    let mut se = String::new();
    if let Some(mut h) = child.stderr.take() {
        let _ = Read::read_to_string(&mut h, &mut se);
    }
    Ok(ProcOut {
        code,
        stdout: so,
        stderr: se,
    })
}

/// 引擎 selector 二进制位（gap-parking-route-engine-selector）：本 bin 同目录
/// 兄弟位（cargo 同树 bins 同居 target/debug 或 target/release），缺位回落
/// PATH 裸名，spawn 失败即降级如实（iso-04 locks_read 直调先例同形，
/// mcpserver runtime lease_bin 兜底精神）。
fn selector_bin() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sib = dir.join("selector");
            if sib.is_file() {
                return sib;
            }
        }
    }
    PathBuf::from("selector")
}

/// 泊界包位：exe 派生引擎仓根 packs/selector/parking（gap-packs-assets 落位，
/// 与围堰 sih-tools/selector/packs/parking 内容同基），绝对形存在即用；
/// 缺位回落相对形 "parking" 交 selector resolve_pack_input 引擎位候选序自解析，
/// 全不中即 selector 既有 pack directory missing 路径如实报错（exit 2 降级可见）。
fn parking_pack_arg() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(root) = exe
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
        {
            let p = root.join("packs").join("selector").join("parking");
            if p.is_dir() {
                return p.display().to_string();
            }
        }
    }
    "parking".to_string()
}

fn parking_face(at: chrono::NaiveDate, faces: &Faces) -> (Value, Vec<String>) {
    let mut lines = Map::new();
    let mut degradations: Vec<String> = Vec::new();
    let pack_arg = parking_pack_arg();
    for (name, materials) in &faces.parking_jobs {
        // 泛在与围堰同参：materials 相对形原以 parking_cwd（<root>/sih-tools）为
        // 基准，归一绝对后传引擎 selector bin，子进程 cwd 不再锚定 sih-tools
        //（对围堰 uv run 的最后运行时依赖随之消除）。
        let mats = faces.parking_cwd.join(materials);
        let mut cmd = Command::new(selector_bin());
        cmd.args([
            "route",
            "--pack",
            &pack_arg,
            "--reference-time",
            &at.to_string(),
        ]);
        cmd.arg(&mats);
        match run_with_timeout(cmd, Duration::from_secs(ROUTE_TIMEOUT_S)) {
            Err(ProcErr::Timeout) => {
                lines.insert(
                    name.clone(),
                    json!({"route": "degraded", "reason": format!("路由超时 {} 秒", ROUTE_TIMEOUT_S)}),
                );
                degradations.push(format!("泊界路由超时（降级）：{}", name));
            }
            Err(ProcErr::Fail) => {
                lines.insert(
                    name.clone(),
                    json!({"route": "degraded", "reason": "OSError"}),
                );
                degradations.push(format!("泊界路由不可用（降级）：{} OSError", name));
            }
            Ok(out) => {
                let code_disp = out
                    .code
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "None".to_string());
                if out.code != Some(0) && out.code != Some(1) {
                    lines.insert(
                        name.clone(),
                        json!({
                            "route": "degraded",
                            "reason": format!("路由退出码 {}（0 与 1 外一律降级）", code_disp),
                        }),
                    );
                    degradations.push(format!(
                        "泊界路由异常退出（降级）：{} exit {}",
                        name, code_disp
                    ));
                    continue;
                }
                let parsed: Result<Value, _> = serde_json::from_str(out.stdout.trim());
                match parsed {
                    Err(_) => {
                        lines.insert(
                            name.clone(),
                            json!({"route": "degraded", "reason": "JSONDecodeError"}),
                        );
                        degradations
                            .push(format!("泊界路由不可用（降级）：{} JSONDecodeError", name));
                    }
                    Ok(v) => match v.get("summary") {
                        Some(s) if s.is_object() => {
                            lines.insert(
                                name.clone(),
                                json!({
                                    "route": "routed",
                                    "exit_code": out.code,
                                    "summary": s,
                                }),
                            );
                        }
                        _ => {
                            lines.insert(
                                name.clone(),
                                json!({"route": "degraded", "reason": "ValueError"}),
                            );
                            degradations
                                .push(format!("泊界路由不可用（降级）：{} ValueError", name));
                        }
                    },
                }
            }
        }
    }
    (Value::Object(lines), degradations)
}

fn enforce_threshold_seal(threshold: i64, reason: Option<&str>) -> Option<Value> {
    if threshold != DEFAULT_THRESHOLD && reason.map_or(true, |r| r.trim().is_empty()) {
        return Some(json!({
            "error": "非缺省阈值越窗缺事由申报（裁判面封印）",
            "reason_code": "threshold_reason_missing",
            "gate": "裁判面封印（m-referee-seal-r2）",
            "what_this_tool_does": "critsweep 判据扫回算器：--threshold 即沉底裁判窗口，越窗改变判定结果",
            "valid_params": ["--threshold <日数>", "--threshold-reason <事由>"],
            "suggested_action": "补 --threshold-reason <事由>（登记入 ledger/referee.ndjson），或用缺省阈值三",
        }));
    }
    None
}

fn referee_ledger_path(explicit: Option<&str>) -> PathBuf {
    if let Some(p) = explicit {
        if !p.is_empty() {
            return PathBuf::from(p);
        }
    }
    if let Ok(p) = std::env::var("CRITSWEEP_REFEREE_LEDGER") {
        if !p.is_empty() {
            return PathBuf::from(p);
        }
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut cur = Some(cwd.as_path());
    while let Some(c) = cur {
        let cand = c.join("sih-tools/critsweep/ledger/referee.ndjson");
        if cand.parent().map(|p| p.is_dir()).unwrap_or(false) {
            return cand;
        }
        cur = c.parent();
    }
    cwd.join("sih-tools/critsweep/ledger/referee.ndjson")
}

fn register_threshold_override(
    threshold: i64,
    reason: &str,
    at_iso: &str,
    ledger: &Path,
) -> std::io::Result<PathBuf> {
    if let Some(p) = ledger.parent() {
        std::fs::create_dir_all(p)?;
    }
    let row = json!({
        "at": at_iso,
        "reason": reason.trim(),
        "threshold": threshold,
        "tool": {"name": "critsweep", "version": VERSION},
    });
    use std::os::fd::AsRawFd;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(ledger)?;
    let fd = f.as_raw_fd();
    unsafe {
        libc::flock(fd, libc::LOCK_EX);
    }
    let wr = f.write_all(format!("{}\n", serde_json::to_string(&row).unwrap()).as_bytes());
    unsafe {
        libc::flock(fd, libc::LOCK_UN);
    }
    wr?;
    Ok(ledger.to_path_buf())
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = parse_args(&argv);

    if let Some(seal) = enforce_threshold_seal(args.threshold, args.threshold_reason.as_deref()) {
        let mut m = match seal {
            Value::Object(m) => m,
            _ => Map::new(),
        };
        m.insert("tool".to_string(), json!("critsweep"));
        m.insert("version".to_string(), json!(VERSION));
        print!("{}", emit(&Value::Object(m)));
        std::process::exit(1);
    }

    let mut degradations: Vec<String> = Vec::new();
    let at = match &args.at {
        Some(s) => match chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => {
                degradations.push(format!("--at 非法（降级取实日）：{}", s));
                today()
            }
        },
        None => today(),
    };

    let mut threshold_registered = false;
    if args.threshold != DEFAULT_THRESHOLD {
        let led = referee_ledger_path(args.referee_ledger.as_deref());
        match register_threshold_override(
            args.threshold,
            args.threshold_reason.as_deref().unwrap_or(""),
            &at.to_string(),
            &led,
        ) {
            Ok(_) => threshold_registered = true,
            Err(e) => degradations.push(format!("裁判面登记失败（降级可见）：{}", e)),
        }
    }

    let (root, root_basis) = resolve_root(args.root.as_deref());
    let root = match root {
        Some(r) => r,
        None => {
            degradations
                .push("工作区根不可判（降级）：上溯无账本目录且无环境回退".to_string());
            let out = json!({
                "tool": "critsweep",
                "version": VERSION,
                "at": at.to_string(),
                "root": Value::Null,
                "root_basis": root_basis,
                "degraded": true,
                "degradations": degradations,
            });
            print!("{}", emit(&out));
            return;
        }
    };

    let faces = domain_faces(&root);
    let (mut criteria, reg_err) = load_registry(&registry_path(args.registry.as_deref(), &root));
    if let Some(e) = reg_err {
        degradations.push(e);
    }
    if let Some(c) = &args.criterion {
        criteria.retain(|x| x.get("id").and_then(|v| v.as_str()) == Some(c.as_str()));
        if criteria.is_empty() {
            degradations.push(format!("--criterion 零命中：{}", c));
        }
    }

    let days = day_window(at, args.window.max(1));
    let mut scan = scan_trails(&faces.trail, &days, &criteria);
    scan.chain_events = scan_chain_events(&faces.trail);
    if !scan.missing_days.is_empty() {
        degradations.push(format!(
            "trail 近窗缺文件日（降级可见）：{}",
            scan.missing_days.join(",")
        ));
    }
    if !scan.unreadable_days.is_empty() {
        degradations.push(format!(
            "trail 读失败日（降级可见）：{}",
            serde_json::to_string(&Value::Array(scan.unreadable_days.clone())).unwrap()
        ));
    }
    if let Some(d) = &scan.degradation {
        degradations.push(d.clone());
    }

    let criteria_out: Vec<Value> = criteria
        .iter()
        .map(|c| criterion_status(c, &scan, at, args.threshold.max(0), &root))
        .collect();

    let (inflight, mut d1) = inflight_face(&faces.sessions, &faces.locks);
    degradations.append(&mut d1);
    let (parking, mut d2) = parking_face(at, &faces);
    degradations.append(&mut d2);

    let mut prose_control: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for crit in &criteria {
        let toks: Vec<String> = crit
            .get("token_scope")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|t| t.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        for token in toks {
            if let Some(events) = scan.prose_only_events.get(&token) {
                if !events.is_empty() {
                    prose_control.entry(token).or_default().extend(events.clone());
                }
            }
        }
    }

    let out = json!({
        "tool": "critsweep",
        "version": VERSION,
        "at": at.to_string(),
        "root": root.display().to_string(),
        "root_basis": root_basis,
        "root_form": faces.layout,
        "threshold_days": args.threshold.max(0),
        "threshold_registered": threshold_registered,
        "window_days": args.window.max(1),
        "window": [days.first().unwrap().to_string(), days.last().unwrap().to_string()],
        "namespace_fields": NAMESPACE_FIELDS,
        "criteria": criteria_out,
        "parking": parking,
        "inflight": inflight,
        "trail": {
            "events_scanned": scan.events_scanned,
            "missing_days": scan.missing_days,
            "unreadable_days": scan.unreadable_days,
        },
        "prose_control": {
            "note": "散文命中而命名空间零命中的事件即设计引用非程序活动，不计活动（pk-044 对照）",
            "events": prose_control,
        },
        "degraded": !degradations.is_empty(),
        "degradations": degradations,
        "replay": format!(
            "python3 sih-tools/critsweep/sweep.py --at {} --root {} [--threshold N] [--criterion <id>]",
            at, root.display()
        ),
    });
    print!("{}", emit(&out));
}
