//! 秤星引擎件（gauge）：治理态读数计算核，ga-2 的引擎 bin 形。
//! 移植源：sih-tools/gauge/src/gauge/cli.py（lease-mergeall-parallel 簇A施工）。
//! 行为链：read（按维度出七字段读数与 history 序列回看）与 record（算并落链，
//! 维度缺省三维全出即例行快照形），两动作共用算半 _compute 逐字节同值；
//! 公式 ga-2 三维 convergence（sih-engine）／adoption（agents）／mergeback
//! （tools-wheel）；底座件中央回退（pk-094 其五，回退事实出参三字段明示）
//! 与跨域卫 fail-closed 承载。只报不判，退出码仅指自身。
//!
//! 落差申报（对表围堰 cli.py，零粉饰）：
//! 1. gchart／gqueue／contrib 三只读子命令未移植（gc-1／gq-1／gd-2 公式面缺席）。
//! 2. sddgate 只读第四维未移植：read --dimension sddgate 报维度违例
//!    （DEC-024 D 层完备度趋势读维缺席申报）。
//! 3. --domain-root 一旗域件形未移植（canonical 域展开缺席）：显式证据旗标形
//!    与中央回退形承载；传 --domain-root 即 exit 2 如实拒。
//! 4. 链文件坏行围堰 json.loads 裸崩，移植跳行如实；_in_window 坏时间戳围堰
//!    ValueError 裸崩，移植按出窗如实；会话账本坏行同跳行。
//! 5. 缺 --sessions-ledger 等旗标时围堰 _guard_same_domain 对 None 面
//!    Path(None) TypeError 裸崩（exit 1），移植按 unresolved 域标记计，
//!    走缺旗标 exit 2 干净报。
//! 6. record 落链回执坏 JSON 围堰裸崩，移植 exit 1「落链回执解析失败」如实报。
//! 7. convergence 的 blob 匹配面 JSON 序列化分隔符与围堰（Python json.dumps
//!    默认带空格）略异，组件名子串匹配语义等价。
//! 8. 出参 pretty 化（围堰 compact sort_keys）：JSON 语义等价；reading 本体
//!    键序按 sort_keys 排布落链内容可对表。
//! 退出码：0 成功／1 维度违例或落链拒／2 用法错或缺底座件或混域拒。

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

const FORMULA: &str = "ga-2";
const PRIOR_ALPHA: f64 = 1.0;
const PRIOR_BETA: f64 = 1.0;
const Z_95: f64 = 1.96;
const DIMENSIONS: [&str; 3] = ["convergence", "adoption", "mergeback"];
/// sddgate 只读第四维未移植（落差申报 2），READ 枚举不入
const READ_DIMENSIONS: [&str; 3] = ["convergence", "adoption", "mergeback"];
const READ_FALLBACK_PIECES: [&str; 3] = ["sessions_ledger", "src_root", "tools_root"];
const FALLBACK_PIECES: [&str; 4] = ["sessions_ledger", "src_root", "tools_root", "scribe"];

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

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

/// Python round(x, 6) 等价：十进制六位舍入
fn round6(x: f64) -> f64 {
    format!("{:.6}", x).parse().unwrap_or(x)
}

/// Python %g 等价（本核常数面 1.0／1.96 形）：整数去小数尾，余按 Display
fn py_g(x: f64) -> String {
    if x == x.trunc() && x.abs() < 1e16 {
        format!("{}", x as i64)
    } else {
        format!("{}", x)
    }
}

fn out_window(at: &str, days: i64) -> String {
    let end = chrono::NaiveDate::parse_from_str(at, "%Y-%m-%d").unwrap_or_default();
    let start = end - chrono::Duration::days(days - 1);
    format!("{}/{}", start, at)
}

fn in_window(ts: Option<&str>, at: &str, days: i64) -> bool {
    let Some(ts) = ts else { return false };
    if ts.len() < 10 {
        return false;
    }
    let Ok(d) = chrono::NaiveDate::parse_from_str(&ts[..10], "%Y-%m-%d") else {
        return false;
    };
    let Ok(end) = chrono::NaiveDate::parse_from_str(at, "%Y-%m-%d") else {
        return false;
    };
    let gap = (end - d).num_days();
    0 <= gap && gap < days
}

fn truthy(v: Option<&Value>) -> bool {
    match v {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_f64().map(|f| f != 0.0).unwrap_or(true),
        Some(Value::String(s)) => !s.is_empty(),
        Some(Value::Array(a)) => !a.is_empty(),
        Some(Value::Object(o)) => !o.is_empty(),
    }
}

fn read_jsonl(path: &Path) -> Vec<Value> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

fn load_window(trails: &[PathBuf], at: &str, days: i64) -> (Vec<Value>, Vec<String>) {
    let mut events = Vec::new();
    let mut hashes = Vec::new();
    for tp in trails {
        let Ok(text) = std::fs::read_to_string(tp) else {
            continue;
        };
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let Ok(ev) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            if in_window(ev.get("timestamp").and_then(|v| v.as_str()), at, days) {
                hashes.push(
                    ev.get("event_hash")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                );
                events.push(ev);
            }
        }
    }
    (events, hashes)
}

fn history(trails: &[PathBuf], dim: &str, subject: &str) -> Vec<Value> {
    let mut out = Vec::new();
    for tp in trails {
        let Ok(text) = std::fs::read_to_string(tp) else {
            continue;
        };
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let Ok(ev) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            if ev.get("event_type").and_then(|v| v.as_str()) != Some("reading_recorded") {
                continue;
            }
            let d = ev.get("details").cloned().unwrap_or_else(|| json!({}));
            if d.get("dimension").and_then(|v| v.as_str()) == Some(dim)
                && d.get("subject").and_then(|v| v.as_str()) == Some(subject)
            {
                out.push(json!({
                    "computed_at": d.get("computed_at").cloned().unwrap_or(Value::Null),
                    "value": d.get("value").cloned().unwrap_or(Value::Null),
                    "event_hash": ev.get("event_hash").cloned().unwrap_or(Value::Null),
                }));
            }
        }
    }
    out
}

/// 算半：read 与 record 共用，改这里即改两处读数值
fn compute(
    dim: &str,
    at: &str,
    days: i64,
    events: &[Value],
    hashes: &[String],
    sessions_ledger: &str,
    src_root: &str,
    tools_root: &str,
    merged: &str,
) -> (Value, Vec<String>) {
    let mut extra: Map<String, Value> = Map::new();
    let subject: &str;
    let inputs: Vec<String>;
    match dim {
        "convergence" => {
            let src = Path::new(src_root).join("src");
            let mut components: Vec<String> = Vec::new();
            if src.is_dir() {
                if let Ok(rd) = std::fs::read_dir(&src) {
                    let mut names: Vec<String> = rd
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .filter_map(|e| Some(e.file_name().to_string_lossy().to_string()))
                        .collect();
                    names.sort();
                    components = names;
                }
            }
            let blob = serde_json::to_string(&Value::Array(events.to_vec())).unwrap_or_default();
            let hit: Vec<&String> = components
                .iter()
                .filter(|c| blob.contains(c.as_str()))
                .collect();
            let value = if !components.is_empty() {
                round6(hit.len() as f64 / components.len() as f64)
            } else {
                0.0
            };
            let mut ins = Vec::new();
            for (h, ev) in hashes.iter().zip(events.iter()) {
                let evs = serde_json::to_string(ev).unwrap_or_default();
                if hit.iter().any(|c| evs.contains(c.as_str())) {
                    ins.push(h.clone());
                }
            }
            inputs = ins;
            subject = "sih-engine";
            return finish_reading(dim, subject, value, at, days, extra, inputs);
        }
        "adoption" => {
            let mut issued: Vec<Value> = Vec::new();
            for r in read_jsonl(Path::new(sessions_ledger)) {
                if r.get("event").and_then(|v| v.as_str()) == Some("issued")
                    && in_window(r.get("issued_at").and_then(|v| v.as_str()), at, days)
                {
                    issued.push(r);
                }
            }
            let n = issued.len();
            let k = issued
                .iter()
                .filter(|r| truthy(r.get("identity").and_then(|i| i.get("identity_hash"))))
                .count();
            let value = if n > 0 {
                round6(k as f64 / n as f64)
            } else {
                0.0
            };
            if n > 0 {
                let p = k as f64 / n as f64;
                let half = Z_95 * (p * (1.0 - p) / n as f64).sqrt();
                extra.insert(
                    "confidence_band".to_string(),
                    json!({
                        "lower": round6((p - half).max(0.0)),
                        "upper": round6((p + half).min(1.0)),
                    }),
                );
                extra.insert(
                    "posterior_mean".to_string(),
                    json!(round6(
                        (PRIOR_ALPHA + k as f64) / (PRIOR_ALPHA + PRIOR_BETA + n as f64)
                    )),
                );
            }
            inputs = vec![
                format!(
                    "sessions-ledger:{}",
                    sha256_hex(&std::fs::read(sessions_ledger).unwrap_or_default())
                ),
                format!("prior:Beta({},{})", py_g(PRIOR_ALPHA), py_g(PRIOR_BETA)),
                format!("z:{}", py_g(Z_95)),
            ];
            subject = "agents";
            return finish_reading(dim, subject, value, at, days, extra, inputs);
        }
        _ => {
            let mut tools: Vec<String> = Vec::new();
            if let Ok(rd) = std::fs::read_dir(tools_root) {
                for e in rd.filter_map(|e| e.ok()) {
                    let p = e.path();
                    let name = e.file_name().to_string_lossy().to_string();
                    if p.is_dir() && !name.starts_with('.') {
                        tools.push(name);
                    }
                }
            }
            tools.sort();
            let merged_set: BTreeSet<&str> = merged.split(',').filter(|m| !m.is_empty()).collect();
            let value = if !tools.is_empty() {
                round6(merged_set.len() as f64 / tools.len() as f64)
            } else {
                0.0
            };
            let merged_sorted: Vec<&str> = merged_set.iter().copied().collect();
            inputs = vec![
                format!(
                    "tools-dirs:{}",
                    sha256_hex(tools.join(",").as_bytes())
                ),
                format!(
                    "merged:{}",
                    sha256_hex(merged_sorted.join(",").as_bytes())
                ),
            ];
            subject = "tools-wheel";
            return finish_reading(dim, subject, value, at, days, extra, inputs);
        }
    }
}

/// reading 载体组装：键序按围堰 sort_keys 形排布（落差申报 8）
fn finish_reading(
    dim: &str,
    subject: &str,
    value: f64,
    at: &str,
    days: i64,
    extra: Map<String, Value>,
    inputs: Vec<String>,
) -> (Value, Vec<String>) {
    let digest = {
        let mut sorted = inputs.clone();
        sorted.sort();
        sha256_hex(sorted.join("").as_bytes())
    };
    let mut m = Map::new();
    m.insert("computed_at".to_string(), json!(at));
    if let Some(v) = extra.get("confidence_band") {
        m.insert("confidence_band".to_string(), v.clone());
    }
    m.insert("dimension".to_string(), json!(dim));
    m.insert("formula_version".to_string(), json!(FORMULA));
    m.insert("inputs_digest".to_string(), json!(digest));
    if let Some(v) = extra.get("posterior_mean") {
        m.insert("posterior_mean".to_string(), v.clone());
    }
    m.insert("subject".to_string(), json!(subject));
    m.insert("value".to_string(), json!(value));
    m.insert("window".to_string(), json!(out_window(at, days)));
    (Value::Object(m), inputs)
}

fn subject_of(dim: &str) -> &'static str {
    match dim {
        "convergence" => "sih-engine",
        "adoption" => "agents",
        _ => "tools-wheel",
    }
}

struct Faces {
    given: BTreeSet<String>,
    trail: Vec<PathBuf>,
    sessions_ledger: Option<String>,
    src_root: Option<String>,
    tools_root: Option<String>,
    merged: String,
    record_trail: Option<String>,
    scribe: Option<String>,
    locks: Option<String>,
    session: Option<String>,
    dimension: Option<String>,
    at: String,
    window_days: i64,
    evidence_basis: String,
    evidence_missing: Vec<String>,
    evidence_fallback: BTreeMap<String, String>,
}

impl Default for Faces {
    fn default() -> Self {
        Faces {
            given: BTreeSet::new(),
            trail: Vec::new(),
            sessions_ledger: None,
            src_root: None,
            tools_root: None,
            merged: "scribe".to_string(),
            record_trail: None,
            scribe: None,
            locks: None,
            session: None,
            dimension: None,
            at: String::new(),
            window_days: 1,
            evidence_basis: "domain".to_string(),
            evidence_missing: Vec::new(),
            evidence_fallback: BTreeMap::new(),
        }
    }
}

/// 数据面域标记上溯：canonical（sih/ledger 面）或 first_domain（双仓标记）先到先得
fn domain_marker(path: &Path) -> Option<PathBuf> {
    let mut cur = if path.is_file() {
        path.parent().unwrap_or(path).to_path_buf()
    } else {
        path.to_path_buf()
    };
    if let Ok(c) = std::fs::canonicalize(&cur) {
        cur = c;
    }
    loop {
        if cur.join("sih/ledger").is_dir() {
            return Some(cur.clone());
        }
        if cur.join("sih-engine/Cargo.toml").is_file()
            && cur.join("sih-tools/pyproject.toml").is_file()
        {
            return Some(cur.clone());
        }
        match cur.parent() {
            Some(p) if p != cur => cur = p.to_path_buf(),
            _ => return None,
        }
    }
}

fn guard_same_domain(f: &Faces) -> bool {
    let mut faces: Vec<PathBuf> = f.trail.clone();
    if let Some(s) = &f.sessions_ledger {
        faces.push(PathBuf::from(s));
    }
    if let Some(s) = &f.record_trail {
        faces.push(PathBuf::from(s));
    }
    if let Some(s) = &f.locks {
        faces.push(PathBuf::from(s));
    }
    let mut marks: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for p in &faces {
        let m = domain_marker(p);
        let key = m
            .map(|x| x.display().to_string())
            .unwrap_or_else(|| "unresolved".to_string());
        marks.entry(key).or_default().push(p.display().to_string());
    }
    if marks.len() > 1 {
        let out = json!({
            "error": "跨域数据面混域拒（fail-closed，M2 病灶根治位）",
            "domains": marks,
        });
        eprintln!("{}", emit(&out));
        return false;
    }
    true
}

/// 中央码根：自 cwd 上溯找首个双仓根（落差申报 3：码面仍供给中央）
fn central_code_root() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let mut cur = Some(cwd.as_path());
    while let Some(c) = cur {
        if c.join("sih-engine/Cargo.toml").is_file()
            && c.join("sih-tools/pyproject.toml").is_file()
        {
            return Some(c.to_path_buf());
        }
        cur = c.parent();
    }
    None
}

fn evidence_missing(f: &Faces) -> Vec<String> {
    let mut v = Vec::new();
    for t in &f.trail {
        if !t.exists() {
            v.push(t.display().to_string());
        }
    }
    for p in [&f.sessions_ledger, &f.src_root, &f.tools_root] {
        if let Some(s) = p {
            if !Path::new(s).exists() {
                v.push(s.clone());
            }
        }
    }
    v
}

fn face_get(f: &Faces, piece: &str) -> Option<String> {
    match piece {
        "sessions_ledger" => f.sessions_ledger.clone(),
        "src_root" => f.src_root.clone(),
        "tools_root" => f.tools_root.clone(),
        "scribe" => f.scribe.clone(),
        _ => None,
    }
}

fn face_set(f: &mut Faces, piece: &str, val: String) {
    match piece {
        "sessions_ledger" => f.sessions_ledger = Some(val),
        "src_root" => f.src_root = Some(val),
        "tools_root" => f.tools_root = Some(val),
        "scribe" => f.scribe = Some(val),
        _ => {}
    }
}

/// 底座件中央回退解析（pk-094 其五）：显式位缺席不代换，回退事实记三字段不静默
fn resolve_central_fallback(f: &mut Faces, pieces: &[&str], include_scribe: bool) -> Vec<String> {
    f.evidence_basis = "domain".to_string();
    f.evidence_missing.clear();
    f.evidence_fallback.clear();
    let central = central_code_root();
    let mut still = evidence_missing(f);
    if let Some(c) = central {
        let basis: BTreeMap<&str, PathBuf> = BTreeMap::from([
            ("sessions_ledger", c.join("sih-tools/lease/ledger/sessions.ndjson")),
            ("src_root", c.join("sih-engine")),
            ("tools_root", c.join("sih-tools")),
            ("scribe", c.join("sih-engine/target/debug/scribe")),
        ]);
        let mut fallback: Vec<(String, String)> = Vec::new();
        for piece in pieces {
            let val = face_get(f, piece);
            if let Some(v) = &val {
                if Path::new(v).exists() {
                    continue;
                }
                if f.given.contains(*piece) {
                    continue;
                }
            }
            let Some(fb) = basis.get(piece) else { continue };
            if fb.exists() {
                if let Some(v) = val {
                    f.evidence_missing.push(v);
                }
                face_set(f, piece, fb.display().to_string());
                fallback.push((piece.to_string(), fb.display().to_string()));
            }
        }
        if !fallback.is_empty() {
            f.evidence_basis = "central-fallback".to_string();
            f.evidence_fallback = fallback.into_iter().collect();
            still = evidence_missing(f);
        }
    }
    if include_scribe {
        if let Some(s) = &f.scribe {
            if !Path::new(s).exists() {
                still.push(s.clone());
            }
        }
    }
    still
}

fn disclose_evidence(out: &mut Map<String, Value>, f: &Faces) {
    if f.evidence_basis == "central-fallback" {
        out.insert(
            "evidence_basis".to_string(),
            json!(f.evidence_basis.clone()),
        );
        out.insert(
            "evidence_missing".to_string(),
            json!(f.evidence_missing.clone()),
        );
        out.insert(
            "evidence_fallback".to_string(),
            json!(f.evidence_fallback.clone()),
        );
    }
}

fn cmd_read(f: &mut Faces) -> i32 {
    if !guard_same_domain(f) {
        return 2;
    }
    if f.trail.is_empty()
        || f.sessions_ledger.is_none()
        || f.src_root.is_none()
        || f.tools_root.is_none()
    {
        die(
            2,
            "缺必填证据旗标（--trail --sessions-ledger --src-root --tools-root，或 --domain-root 一旗域件形）",
            json!(null),
        );
    }
    let still = resolve_central_fallback(f, &READ_FALLBACK_PIECES, false);
    if !still.is_empty() {
        die(2, "底座件缺席", json!({"missing": still}));
    }
    let dim = f.dimension.clone().unwrap_or_default();
    if !READ_DIMENSIONS.contains(&dim.as_str()) {
        die(1, "维度违例", json!({"dimension": dim}));
    }
    let at = f.at.clone();
    let days = f.window_days;
    let sessions = f.sessions_ledger.clone().unwrap_or_default();
    let src = f.src_root.clone().unwrap_or_default();
    let tools = f.tools_root.clone().unwrap_or_default();
    let merged = f.merged.clone();
    let (events, hashes) = load_window(&f.trail, &at, days);
    let (reading, inputs) = compute(
        &dim, &at, days, &events, &hashes, &sessions, &src, &tools, &merged,
    );
    let hist = history(&f.trail, &dim, subject_of(&dim));
    let mut sorted_inputs = inputs.clone();
    sorted_inputs.sort();
    let sources: Vec<String> = f
        .trail
        .iter()
        .map(|p| p.display().to_string())
        .collect();
    let mut out = Map::new();
    out.insert("history".to_string(), json!(hist));
    out.insert(
        "inputs".to_string(),
        json!({
            "events": sorted_inputs,
            "sources": sources,
        }),
    );
    out.insert("reading".to_string(), reading);
    out.insert(
        "sequence".to_string(),
        json!(if hist.is_empty() { "insufficient" } else { "ok" }),
    );
    disclose_evidence(&mut out, f);
    print!("{}", emit(&Value::Object(out)));
    0
}

fn cmd_record(f: &mut Faces) -> i32 {
    if !guard_same_domain(f) {
        return 2;
    }
    if f.trail.is_empty()
        || f.sessions_ledger.is_none()
        || f.src_root.is_none()
        || f.tools_root.is_none()
        || f.record_trail.is_none()
        || f.scribe.is_none()
    {
        die(
            2,
            "缺必填证据旗标（--trail --sessions-ledger --src-root --tools-root --record-trail --scribe，或 --domain-root 一旗域件形）",
            json!(null),
        );
    }
    if let Some(d) = &f.dimension {
        if !DIMENSIONS.contains(&d.as_str()) {
            die(1, "维度违例", json!({"dimension": d}));
        }
    }
    let still = resolve_central_fallback(f, &FALLBACK_PIECES, true);
    if !still.is_empty() {
        die(2, "底座件缺席", json!({"missing": still}));
    }
    let dims: Vec<String> = match &f.dimension {
        Some(d) => vec![d.clone()],
        None => DIMENSIONS.iter().map(|s| s.to_string()).collect(),
    };
    let at = f.at.clone();
    let days = f.window_days;
    let sessions = f.sessions_ledger.clone().unwrap_or_default();
    let src = f.src_root.clone().unwrap_or_default();
    let tools = f.tools_root.clone().unwrap_or_default();
    let merged = f.merged.clone();
    let (events, hashes) = load_window(&f.trail, &at, days);
    let mut recorded: Vec<Value> = Vec::new();
    for dim in &dims {
        let (reading, _inputs) = compute(
            dim, &at, days, &events, &hashes, &sessions, &src, &tools, &merged,
        );
        let tmp = std::env::temp_dir().join(format!(
            "gauge-reading-{}.json",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(&tmp, serde_json::to_string(&reading).unwrap()).ok();
        let tmp_s = tmp.to_string_lossy().to_string();
        let record_trail_s = f.record_trail.clone().unwrap_or_default();
        let mut cmd = Command::new(f.scribe.clone().unwrap_or_default());
        cmd.args(["record", "--reading", tmp_s.as_str(), "--trail", record_trail_s.as_str()]);
        if let Some(l) = &f.locks {
            cmd.args(["--locks", l]);
        }
        if let Some(s) = &f.session {
            cmd.args(["--session", s]);
        }
        let out = cmd.output();
        let _ = std::fs::remove_file(&tmp);
        let out = match out {
            Ok(o) => o,
            Err(e) => die(
                1,
                "落链拒",
                json!({"dimension": dim, "scribe_stderr": format!("{}", e)}),
            ),
        };
        if !out.status.success() {
            let se: String = String::from_utf8_lossy(&out.stderr).trim().chars().take(400).collect();
            let so: String = String::from_utf8_lossy(&out.stdout).trim().chars().take(400).collect();
            die(
                1,
                "落链拒",
                json!({"dimension": dim, "scribe_stderr": se, "scribe_stdout": so}),
            );
        }
        let stdout_str = String::from_utf8_lossy(&out.stdout);
        let reply: Value = match serde_json::from_str(stdout_str.trim()) {
            Ok(v) => v,
            Err(_) => die(
                1,
                "落链回执解析失败",
                json!({"dimension": dim, "scribe_stdout": stdout_str.chars().take(400).collect::<String>()}),
            ),
        };
        recorded.push(json!({
            "dimension": dim,
            "event_hash": reply.get("event_hash").cloned().unwrap_or(Value::Null),
            "subject": reading.get("subject").cloned().unwrap_or(Value::Null),
            "value": reading.get("value").cloned().unwrap_or(Value::Null),
        }));
    }
    let mut out = Map::new();
    out.insert("formula_version".to_string(), json!(FORMULA));
    out.insert("record_trail".to_string(), json!(f.record_trail.clone()));
    out.insert("recorded".to_string(), json!(recorded));
    disclose_evidence(&mut out, f);
    print!("{}", emit(&Value::Object(out)));
    0
}

fn parse_args() -> (String, Faces) {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if argv.is_empty() {
        die(2, "用法 gauge <read|record> [flags]", json!(null));
    }
    let sub = argv[0].clone();
    let mut f = Faces::default();
    let mut at: Option<String> = None;
    let mut i = 1;
    while i < argv.len() {
        let raw = &argv[i];
        let rest = match raw.strip_prefix("--") {
            Some(r) => r,
            None => die(2, &format!("意外位置参 {}", raw), json!(null)),
        };
        let (k, inline): (String, Option<String>) = match rest.split_once('=') {
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
            "dimension" => f.dimension = Some(val),
            "at" => at = Some(val),
            "window-days" => {
                f.window_days = val
                    .parse()
                    .unwrap_or_else(|_| die(2, &format!("--window-days 非整数：{}", val), json!(null)))
            }
            "trail" => f.trail.push(PathBuf::from(val)),
            "sessions-ledger" => {
                f.sessions_ledger = Some(val.clone());
                f.given.insert("sessions_ledger".to_string());
            }
            "src-root" => {
                f.src_root = Some(val.clone());
                f.given.insert("src_root".to_string());
            }
            "tools-root" => {
                f.tools_root = Some(val.clone());
                f.given.insert("tools_root".to_string());
            }
            "merged" => f.merged = val,
            "scribe" => {
                f.scribe = Some(val.clone());
                f.given.insert("scribe".to_string());
            }
            "record-trail" => f.record_trail = Some(val),
            "locks" => f.locks = Some(val),
            "session" => f.session = Some(val),
            "domain-root" => die(
                2,
                "--domain-root 一旗域件形未移植（落差申报：canonical 域展开缺席，用显式证据旗标形）",
                json!(null),
            ),
            _ => die(2, &format!("未知旗标 --{}", k), json!(null)),
        }
        i += 1;
    }
    f.at = match at {
        Some(a) => a,
        None => die(2, "缺 --at（显式给参，禁 now）", json!(null)),
    };
    (sub, f)
}

fn main() {
    let (sub, mut f) = parse_args();
    let rc = match sub.as_str() {
        "read" => {
            if f.dimension.is_none() {
                die(2, "read 缺 --dimension", json!(null));
            }
            cmd_read(&mut f)
        }
        "record" => cmd_record(&mut f),
        other => die(
            2,
            &format!("子命令 {} 未在融回对等域：read/record 覆盖（gchart/gqueue/contrib 未移植，落差申报）", other),
            json!(null),
        ),
    };
    std::process::exit(rc);
}
