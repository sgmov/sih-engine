//! 引擎侧路择（Selector）命令行面 —— lease-mergeleg23-parallel 簇E 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/selector 0.4.0（src/selector/：cli.py、pack.py、
//! predicates.py、route.py），只读对表移植，围堰源码零改动。
//!
//! CLI：`selector route --pack <谓词包目录> [--reference-time <ISO日期>] [MATERIAL...]`
//! 退出码三值：0 = 路由毕无告警、1 = 路由毕有告警、2 = 输入非法。
//! 核心行为链：pack（envelope.json + manifest.toml + routes.toml）驱动谓词按声明
//! 序逐件机械求值，首败定路（route_on_fail）全过走 pass_route；轮记录携带 round
//! 真、停泊材料携带 parking 真；批级告警 siding_surplus 与 mainline_starvation
//! 只记不改路（豁免 kinds 不计入有余积压、summary.siding 保持全量计数）；
//! registry_conflict 与 parking_aging 两告警位恒过不定路，只在包内声明时产
//! 批级冲突与超期告警；时间谓词参照时间显式给参不读系统钟，缺参即判败
//! fail-closed。输出 json 报告 sort_keys + indent 2。
//!
//! 落差申报（相对围堰）：
//! 1. ISO 日期解析只认 YYYY-MM-DD 与 YYYYMMDD 两基形；围堰 Python 3.11+
//!    date.fromisoformat 的宽 ISO 形（带时间后缀串、周日期形）未对齐。
//! 2. time_deadline 的浮点 ttl_days 按秒级舍入对表（围堰 timedelta 微秒舍入
//!    未对齐）；整型 ttl 两边一致（到期日当日即败语义保留）。
//! 3. JSON 浮点最短表示边缘形（Python repr 与 serde 不同）未逐字对齐；
//!    批级告警门槛与计数均整型不受影响。
//! 4. 用法错（缺子命令、缺 --pack、未知旗标）为自足解析，stderr 短句回报，
//!    退出码 2 与围堰 argparse 对齐，报文形不对齐；argparse 帮助面未移植。
//! 5. fnmatch 按 Python fnmatch.translate 标准语义移植（*、?、[seq]、[!seq]），
//!    非法模式编译失败即 panic（围堰 re.error 崩溃形等价，报文不对齐）。

use serde_json::{json, Map, Value};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

/// 围堰版本锚：sih-tools/selector/src/selector/__init__.py __version__。
const ENGINE_VERSION: &str = "0.4.0";
const TOOL_KEY: &str = "selector";

const KINDS: [&str; 4] = [
    "schema_required",
    "state_annotation",
    "anchor_whitelist",
    "write_boundary",
];
const ROUND_KINDS: [&str; 4] = [
    "anchor_density",
    "standing_constraints",
    "assertion_lists",
    "registry_conflict",
];
const PARKING_KINDS: [&str; 3] = ["time_deadline", "parking_aging", "gate_hold"];
const ROUTES: [&str; 3] = ["mainline", "siding", "scrap_track"];
const ROUTE_ON_FAIL_DEFAULT: [(&str, &str); 4] = [
    ("anchor_density", "siding"),
    ("standing_constraints", "siding"),
    ("assertion_lists", "siding"),
    ("time_deadline", "siding"),
];
const KIND_PARAM: [(&str, &str); 4] = [
    ("schema_required", "fields"),
    ("state_annotation", "allowed"),
    ("anchor_whitelist", "whitelist"),
    ("write_boundary", "allowed_roots"),
];

fn is_valid_kind(k: &str) -> bool {
    KINDS.contains(&k) || ROUND_KINDS.contains(&k) || PARKING_KINDS.contains(&k)
}

#[derive(Default)]
struct Params {
    fields: Vec<String>,
    allowed: Vec<String>,
    whitelist_re: Vec<regex::Regex>,
    allowed_roots_re: Vec<regex::Regex>,
    density_threshold: f64,
    resolve_root: String,
    required_receipts: Vec<String>,
    aging_threshold_days: f64,
    aging_threshold_json: Value,
}

struct Predicate {
    id: String,
    kind: String,
    route_on_fail: Option<String>,
    params: Params,
}

struct Pack {
    name: String,
    version: String,
    domain_include: Vec<String>,
    domain_exclude: Vec<String>,
    predicates: Vec<Predicate>,
    pass_route: String,
    siding_surplus_threshold: i64,
    mainline_starvation_threshold: i64,
    siding_surplus_exempt_kinds: Vec<String>,
}

/// 对象键递归排序（json.dumps sort_keys=True 语义，serde_json preserve_order 下重建）。
fn sort_value(v: Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut kvs: Vec<(String, Value)> = m.into_iter().collect();
            kvs.sort_by(|a, b| a.0.cmp(&b.0));
            let mut out = Map::new();
            for (k, val) in kvs {
                out.insert(k, sort_value(val));
            }
            Value::Object(out)
        }
        Value::Array(a) => Value::Array(a.into_iter().map(sort_value).collect()),
        other => other,
    }
}

fn emit_error(msg: &str) -> ! {
    println!("{}", json!({"error": msg}));
    exit(2);
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!(
        "用法: selector route --pack <谓词包目录> [--reference-time <ISO日期>] [MATERIAL...]"
    );
    exit(2);
}

/// envelope 校验（pack.py load_pack 前半逐字对表，与先例 formatter.rs 同形）。
fn validate_envelope(dir: &Path) -> Result<(), String> {
    let env_path = dir.join("envelope.json");
    if !env_path.is_file() {
        return Err(format!("envelope missing: {}", dir.display()));
    }
    let text = fs::read_to_string(&env_path)
        .map_err(|e| format!("envelope invalid: {}: {}", dir.display(), e))?;
    let env: Value = serde_json::from_str(&text)
        .map_err(|e| format!("envelope invalid: {}: {}", dir.display(), e))?;
    if env.get("envelope_version").and_then(|v| v.as_i64()) != Some(1) {
        return Err(format!(
            "envelope invalid: {}: envelope_version must be 1",
            dir.display()
        ));
    }
    let id_ok = env
        .get("id")
        .and_then(|v| v.as_str())
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    if !id_ok {
        return Err(format!(
            "envelope invalid: {}: id must be non-empty string",
            dir.display()
        ));
    }
    let family = env.get("family").and_then(|v| v.as_str()).unwrap_or("");
    const FAMILIES: [&str; 4] = ["scrutinator", "attractor", "formatter", "nomenclator"];
    if !FAMILIES.contains(&family) {
        return Err(format!("envelope invalid: {}: family illegal", dir.display()));
    }
    let body_type = env.get("body_type").and_then(|v| v.as_str()).unwrap_or("");
    if body_type != "config" && body_type != "doc_spec" {
        return Err(format!(
            "envelope invalid: {}: body_type illegal",
            dir.display()
        ));
    }
    let bodies_all_str = env
        .get("bodies")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty() && a.iter().all(|b| b.is_string()))
        .unwrap_or(false);
    if !bodies_all_str {
        return Err(format!(
            "envelope invalid: {}: bodies must be non-empty string list",
            dir.display()
        ));
    }
    let bodies: Vec<String> = env["bodies"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b.as_str().unwrap().to_string())
        .collect();
    for b in bodies {
        if !dir.join(&b).is_file() {
            return Err(format!("envelope body missing: {}: {}", dir.display(), b));
        }
    }
    Ok(())
}

fn load_toml_file(path: &Path, label: &str) -> Result<toml::Value, String> {
    if !path.is_file() {
        return Err(format!("{label} missing: {}", path.display()));
    }
    let text = fs::read_to_string(path)
        .map_err(|e| format!("{label} not utf-8 decodable: {}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("{label} not valid toml: {}: {e}", path.display()))
}

fn require_str_toml(table: &toml::Value, key: &str, ctx: &str) -> Result<String, String> {
    match table.get(key) {
        Some(toml::Value::String(s)) if !s.is_empty() => Ok(s.clone()),
        _ => Err(format!("{ctx}: {key} must be a non-empty string")),
    }
}

fn require_str_list_toml(
    table: &toml::Value,
    key: &str,
    ctx: &str,
    allow_empty: bool,
) -> Result<Vec<String>, String> {
    let Some(list) = table.get(key).and_then(|v| v.as_array()) else {
        return Err(format!("{ctx}: {key} must be a non-empty string array"));
    };
    if list.is_empty() && !allow_empty {
        return Err(format!("{ctx}: {key} must be a non-empty string array"));
    }
    let mut out = Vec::new();
    for item in list {
        match item.as_str() {
            Some(s) if !s.is_empty() => out.push(s.to_string()),
            _ => return Err(format!("{ctx}: {key} entries must be non-empty strings")),
        }
    }
    Ok(out)
}

fn require_threshold(table: &toml::Value, key: &str, ctx: &str) -> Result<i64, String> {
    match table.get(key) {
        Some(toml::Value::Integer(i)) if *i >= 0 => Ok(*i),
        _ => Err(format!("{ctx}: {key} must be a non-negative integer")),
    }
}

/// Python fnmatch.translate 对表：* → .*、? → .、[seq] → 类、[!seq] → 否定类，
/// 余字符逐字转义；整串全匹配锚定。
fn compile_fnmatch(pattern: &str) -> regex::Regex {
    let chars: Vec<char> = pattern.chars().collect();
    let n = chars.len();
    let mut out = String::new();
    let mut i = 0usize;
    while i < n {
        let c = chars[i];
        i += 1;
        if c == '*' {
            out.push_str(".*");
        } else if c == '?' {
            out.push('.');
        } else if c == '[' {
            let mut j = i;
            if j < n && chars[j] == '!' {
                j += 1;
            }
            if j < n && chars[j] == ']' {
                j += 1;
            }
            while j < n && chars[j] != ']' {
                j += 1;
            }
            if j >= n {
                out.push_str("\\[");
            } else {
                let raw: String = chars[i..j].iter().collect();
                let mut stuff = raw.replace('\\', "\\\\");
                i = j + 1;
                if stuff.starts_with('!') {
                    stuff = format!("^{}", &stuff[1..]);
                } else if stuff.starts_with('^') {
                    stuff = format!("\\{stuff}");
                }
                out.push('[');
                out.push_str(&stuff);
                out.push(']');
            }
        } else {
            out.push_str(&regex::escape(&c.to_string()));
        }
    }
    regex::Regex::new(&format!("\\A(?:{out})\\z")).expect("fnmatch pattern compile")
}

fn parse_route_on_fail(
    kind: &str,
    table: &toml::Value,
    pctx: &str,
) -> Result<Option<String>, String> {
    let value = table.get("route_on_fail");
    if kind == "registry_conflict" || kind == "parking_aging" {
        if value.is_some() {
            return Err(format!("{pctx}: route_on_fail is not allowed for {kind}"));
        }
        return Ok(None);
    }
    let Some(v) = value else {
        if let Some((_, d)) = ROUTE_ON_FAIL_DEFAULT.iter().find(|(k, _)| *k == kind) {
            return Ok(Some(d.to_string()));
        }
        return Err(format!(
            "{pctx}: route_on_fail must be one of {ROUTES:?}, got None"
        ));
    };
    match v.as_str() {
        Some(s) if ROUTES.contains(&s) => Ok(Some(s.to_string())),
        _ => Err(format!(
            "{pctx}: route_on_fail must be one of {ROUTES:?}, got {v}"
        )),
    }
}

fn parse_params(kind: &str, table: &toml::Value, pctx: &str) -> Result<Params, String> {
    match kind {
        "anchor_density" => {
            let threshold = match table.get("density_threshold") {
                None => 1.0,
                Some(toml::Value::Integer(i)) => *i as f64,
                Some(toml::Value::Float(f)) => *f,
                _ => return Err(format!("{pctx}: density_threshold must be a number")),
            };
            let resolve_root = require_str_toml(table, "resolve_root", pctx)?;
            Ok(Params {
                density_threshold: threshold,
                resolve_root,
                ..Default::default()
            })
        }
        "standing_constraints" => Ok(Params {
            required_receipts: require_str_list_toml(table, "required_receipts", pctx, false)?,
            ..Default::default()
        }),
        "parking_aging" => {
            let (f, j) = match table.get("aging_threshold_days") {
                Some(toml::Value::Integer(i)) if *i > 0 => {
                    (*i as f64, json!(i))
                }
                Some(toml::Value::Float(f)) if *f > 0.0 => (*f, json!(f)),
                _ => {
                    return Err(format!(
                        "{pctx}: aging_threshold_days must be a positive number"
                    ))
                }
            };
            Ok(Params {
                aging_threshold_days: f,
                aging_threshold_json: j,
                ..Default::default()
            })
        }
        "assertion_lists" | "registry_conflict" | "time_deadline" | "gate_hold" => {
            Ok(Params::default())
        }
        _ => {
            let key = KIND_PARAM
                .iter()
                .find(|(k, _)| *k == kind)
                .map(|(_, v)| *v)
                .unwrap();
            let list = require_str_list_toml(table, key, pctx, false)?;
            Ok(match key {
                "fields" => Params {
                    fields: list,
                    ..Default::default()
                },
                "allowed" => Params {
                    allowed: list,
                    ..Default::default()
                },
                "whitelist" => Params {
                    whitelist_re: list.iter().map(|p| compile_fnmatch(p)).collect(),
                    ..Default::default()
                },
                _ => Params {
                    allowed_roots_re: list.iter().map(|p| compile_fnmatch(p)).collect(),
                    ..Default::default()
                },
            })
        }
    }
}

fn parse_manifest(data: &toml::Value, dir: &Path) -> Result<(String, String, Vec<String>, Vec<String>), String> {
    let ctx = format!("{}/manifest.toml", dir.display());
    let name = require_str_toml(data, "name", &ctx)?;
    let version = require_str_toml(data, "version", &ctx)?;
    let domain = match data.get("domain") {
        Some(d) if d.is_table() => d,
        _ => return Err(format!("{ctx}: domain must be a table")),
    };
    let include = require_str_list_toml(domain, "include", &format!("{ctx} domain"), false)?;
    let exclude = require_str_list_toml(domain, "exclude", &format!("{ctx} domain"), true)?;
    Ok((name, version, include, exclude))
}

fn parse_predicates(data: &toml::Value, dir: &Path) -> Result<Vec<Predicate>, String> {
    let ctx = format!("{}/routes.toml", dir.display());
    let raw = match data.get("predicates") {
        None => vec![],
        Some(v) => match v.as_array() {
            Some(a) if a.iter().all(|t| t.is_table()) => a.clone(),
            _ => return Err(format!("{ctx}: predicates must be an array of tables")),
        },
    };
    let mut predicates = Vec::new();
    for (index, table) in raw.iter().enumerate() {
        let pctx = format!("{ctx} predicates[{index}]");
        let t = table.as_table().unwrap();
        let predicate_id = require_str_toml(&toml::Value::Table(t.clone()), "id", &pctx)?;
        let kind = require_str_toml(&toml::Value::Table(t.clone()), "kind", &pctx)?;
        if !is_valid_kind(&kind) {
            return Err(format!(
                "{pctx}: kind must be one of {:?}, got {kind}",
                [KINDS.as_slice(), ROUND_KINDS.as_slice(), PARKING_KINDS.as_slice()].concat()
            ));
        }
        let route_on_fail = parse_route_on_fail(&kind, &toml::Value::Table(t.clone()), &pctx)?;
        let params = parse_params(&kind, &toml::Value::Table(t.clone()), &pctx)?;
        predicates.push(Predicate {
            id: predicate_id,
            kind,
            route_on_fail,
            params,
        });
    }
    Ok(predicates)
}

fn parse_tail(
    data: &toml::Value,
    dir: &Path,
) -> Result<(String, i64, i64, Vec<String>), String> {
    let ctx = format!("{}/routes.toml", dir.display());
    let defaults = match data.get("defaults") {
        Some(d) if d.is_table() => d,
        _ => return Err(format!("{ctx}: defaults must be a table")),
    };
    let pass_route = match defaults.get("pass_route").and_then(|v| v.as_str()) {
        Some(s) if ROUTES.contains(&s) => s.to_string(),
        other => {
            return Err(format!(
                "{ctx} defaults: pass_route must be one of {ROUTES:?}, got {}",
                other.map(|o| o.to_string()).unwrap_or_else(|| "None".into())
            ))
        }
    };
    let alarms = match data.get("alarms") {
        Some(a) if a.is_table() => a,
        _ => return Err(format!("{ctx}: alarms must be a table")),
    };
    let siding = require_threshold(alarms, "siding_surplus_threshold", &format!("{ctx} alarms"))?;
    let starvation = require_threshold(
        alarms,
        "mainline_starvation_threshold",
        &format!("{ctx} alarms"),
    )?;
    let exempt_raw = match alarms.get("siding_surplus_exempt_kinds") {
        Some(v) => match v.as_array() {
            Some(a) => a.clone(),
            None => {
                return Err(format!(
                    "{ctx} alarms: siding_surplus_exempt_kinds must be a string array"
                ))
            }
        },
        None => vec![],
    };
    let mut exempt = Vec::new();
    for kind in &exempt_raw {
        match kind.as_str() {
            Some(s) if is_valid_kind(s) => exempt.push(s.to_string()),
            Some(s) => {
                return Err(format!(
                    "{ctx} alarms: siding_surplus_exempt_kinds entries must be one of {:?}, got {s}",
                    [KINDS.as_slice(), ROUND_KINDS.as_slice(), PARKING_KINDS.as_slice()].concat()
                ))
            }
            None => {
                return Err(format!(
                    "{ctx} alarms: siding_surplus_exempt_kinds must be a string array"
                ))
            }
        }
    }
    Ok((pass_route, siding, starvation, exempt))
}

/// 谓词包装载（pack.py load_pack 对表）。
fn load_pack(pack_dir: &Path) -> Result<Pack, String> {
    if !pack_dir.is_dir() {
        return Err(format!("pack directory missing: {}", pack_dir.display()));
    }
    validate_envelope(pack_dir)?;
    let manifest = load_toml_file(&pack_dir.join("manifest.toml"), "manifest.toml")?;
    let routes = load_toml_file(&pack_dir.join("routes.toml"), "routes.toml")?;
    let (name, version, include, exclude) = parse_manifest(&manifest, pack_dir)?;
    let predicates = parse_predicates(&routes, pack_dir)?;
    let (pass_route, siding, starvation, exempt) = parse_tail(&routes, pack_dir)?;
    Ok(Pack {
        name,
        version,
        domain_include: include,
        domain_exclude: exclude,
        predicates,
        pass_route,
        siding_surplus_threshold: siding,
        mainline_starvation_threshold: starvation,
        siding_surplus_exempt_kinds: exempt,
    })
}

// ---------- 谓词机械求值（predicates.py 对表，空腹零模型调用） ----------

fn round_data(m: &Value) -> Option<&Value> {
    m.get("round").filter(|v| v.is_object())
}

/// 轮内数组装载：缺省按空数组（Ok(None)），在场非数组即结构非法（Err）。
fn round_list<'a>(r: &'a Value, key: &str) -> Result<Option<&'a Vec<Value>>, ()> {
    match r.get(key) {
        None => Ok(None),
        Some(Value::Array(a)) => Ok(Some(a)),
        Some(_) => Err(()),
    }
}

fn check_schema_required(m: &Value, p: &Params) -> bool {
    p.fields
        .iter()
        .all(|f| m.get(f).map(|x| x.is_string()).unwrap_or(false))
}

fn check_state_annotation(m: &Value, p: &Params) -> bool {
    m.get("state")
        .and_then(|v| v.as_str())
        .map(|s| p.allowed.iter().any(|a| a == s))
        .unwrap_or(false)
}

fn all_match(values: Option<&Value>, patterns: &[regex::Regex]) -> bool {
    let Some(list) = values.and_then(|v| v.as_array()) else {
        return false;
    };
    for value in list {
        let Some(s) = value.as_str() else {
            return false;
        };
        if !patterns.iter().any(|re| re.is_match(s)) {
            return false;
        }
    }
    true
}

fn check_gate_hold(m: &Value) -> bool {
    match m.get("gate") {
        Some(Value::Object(g)) => g.get("fired_at").map(|v| v.is_string()).unwrap_or(false),
        _ => true,
    }
}

fn has_valid_anchor(assertion: &Value, resolve_root: &str) -> bool {
    let Some(evidence) = assertion
        .as_object()
        .and_then(|o| o.get("evidence"))
        .and_then(|v| v.as_array())
    else {
        return false;
    };
    let root = Path::new(resolve_root);
    for entry in evidence {
        let Some(s) = entry.as_str() else { continue };
        let path_text = evidence_path(s);
        if path_text.is_empty() || path_text.starts_with('/') {
            continue;
        }
        if root.join(path_text).exists() {
            return true;
        }
    }
    false
}

/// 条目形如「路径」或「路径:行」，冒号尾段为行号时取路径前缀。
fn evidence_path(entry: &str) -> &str {
    match entry.rfind(':') {
        Some(idx) => {
            let head = &entry[..idx];
            let tail = &entry[idx + 1..];
            if !head.is_empty() && !tail.is_empty() && tail.chars().all(|c| c.is_ascii_digit()) {
                head
            } else {
                entry
            }
        }
        None => entry,
    }
}

fn check_anchor_density(m: &Value, p: &Params) -> bool {
    let Some(rd) = round_data(m) else {
        return false;
    };
    let assertions = match round_list(rd, "assertions") {
        Err(_) => return false,
        Ok(a) => a,
    };
    let Some(assertions) = assertions else {
        return true; // 缺省按空数组，零断言判过
    };
    if assertions.is_empty() {
        return true;
    }
    let rooted = assertions
        .iter()
        .filter(|a| has_valid_anchor(a, &p.resolve_root))
        .count();
    rooted as f64 / assertions.len() as f64 >= p.density_threshold
}

fn check_standing_constraints(m: &Value, p: &Params) -> bool {
    let Some(rd) = round_data(m) else {
        return false;
    };
    let receipts = match round_list(rd, "receipts") {
        Err(_) => return false,
        Ok(a) => a,
    };
    let Some(receipts) = receipts else {
        return true; // 缺省按空数组
    };
    let tools: Vec<&Value> = receipts
        .iter()
        .filter_map(|r| r.as_object().and_then(|o| o.get("tool")))
        .collect();
    p.required_receipts
        .iter()
        .all(|req| tools.iter().any(|t| t.as_str() == Some(req.as_str())))
}

fn check_assertion_lists(m: &Value) -> bool {
    let Some(rd) = round_data(m) else {
        return false;
    };
    let assertions = match round_list(rd, "assertions") {
        Err(_) => return false,
        Ok(a) => a,
    };
    let Some(assertions) = assertions else {
        return true; // 缺省按空数组
    };
    assertions.iter().all(|a| {
        a.as_object()
            .and_then(|o| o.get("evidence"))
            .and_then(|v| v.as_array())
            .map(|e| !e.is_empty())
            .unwrap_or(false)
    })
}

/// ISO 日期解析基形（落差一：宽 ISO 形未对齐）。
fn parse_iso_date(s: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .ok()
        .or_else(|| chrono::NaiveDate::parse_from_str(s, "%Y%m%d").ok())
}

fn check_time_deadline(m: &Value, _p: &Params, context: Option<&str>) -> bool {
    let Some(pk) = m.get("parking").filter(|v| v.is_object()) else {
        return false;
    };
    let entered = pk
        .get("entered_at")
        .and_then(|v| v.as_str())
        .and_then(parse_iso_date);
    let Some(entered) = entered else {
        return false;
    };
    let ttl = match pk.get("ttl_days").and_then(|v| v.as_f64()) {
        Some(f) if !pk.get("ttl_days").unwrap().is_boolean() && f > 0.0 => f,
        _ => return false,
    };
    let Some(reference) = context.and_then(parse_iso_date) else {
        return false;
    };
    if ttl.fract() == 0.0 {
        match entered.checked_add_signed(chrono::Duration::days(ttl as i64)) {
            Some(deadline) => reference < deadline,
            None => false,
        }
    } else {
        let deadline = entered
            .and_hms_opt(0, 0, 0)
            .unwrap()
            + chrono::Duration::seconds((ttl * 86400.0).round() as i64);
        reference.and_hms_opt(0, 0, 0).unwrap() < deadline
    }
}

fn parking_agings(m: &Value, p: &Params, context: Option<&str>) -> Vec<Value> {
    if let Some(Value::Object(g)) = m.get("gate") {
        if !g.get("fired_at").map(|v| v.is_string()).unwrap_or(false) {
            return vec![];
        }
    }
    let Some(pk) = m.get("parking").filter(|v| v.is_object()) else {
        return vec![];
    };
    let entered = pk
        .get("entered_at")
        .and_then(|v| v.as_str())
        .and_then(parse_iso_date);
    let reference = context.and_then(parse_iso_date);
    let (Some(entered), Some(reference)) = (entered, reference) else {
        return vec![];
    };
    let age_days = (reference - entered).num_days();
    if age_days as f64 >= p.aging_threshold_days {
        vec![json!({
            "kind": "parking_aging",
            "id": m.get("id").cloned().unwrap_or(Value::Null),
            "age_days": age_days,
            "threshold": p.aging_threshold_json,
        })]
    } else {
        vec![]
    }
}

fn registry_conflicts(m: &Value) -> Vec<Value> {
    let Some(rd) = round_data(m) else {
        return vec![];
    };
    let conclusions = match round_list(rd, "conclusions") {
        Err(_) => return vec![],
        Ok(a) => a,
    };
    let receipts = match round_list(rd, "receipts") {
        Err(_) => return vec![],
        Ok(a) => a,
    };
    let (Some(conclusions), Some(receipts)) = (conclusions, receipts) else {
        return vec![];
    };
    let mut flagged: BTreeSet<&str> = BTreeSet::new();
    for receipt in receipts {
        if let Some(terms) = receipt
            .as_object()
            .and_then(|o| o.get("flagged_terms"))
            .and_then(|v| v.as_array())
        {
            for t in terms {
                if let Some(s) = t.as_str() {
                    flagged.insert(s);
                }
            }
        }
    }
    let mut alarms = Vec::new();
    for conclusion in conclusions {
        let Some(o) = conclusion.as_object() else {
            continue;
        };
        if o.get("kind").and_then(|v| v.as_str()) != Some("establishes_term") {
            continue;
        }
        if let Some(term) = o.get("term").and_then(|v| v.as_str()) {
            if flagged.contains(term) {
                alarms.push(json!({
                    "kind": "registry_conflict",
                    "term": term,
                    "conclusion": o.get("kind").cloned().unwrap_or(Value::Null),
                }));
            }
        }
    }
    alarms
}

fn evaluate(m: &Value, pred: &Predicate, context: Option<&str>) -> bool {
    match pred.kind.as_str() {
        "schema_required" => check_schema_required(m, &pred.params),
        "state_annotation" => check_state_annotation(m, &pred.params),
        "anchor_whitelist" => all_match(m.get("anchors"), &pred.params.whitelist_re),
        "write_boundary" => all_match(m.get("requested_writes"), &pred.params.allowed_roots_re),
        "anchor_density" => check_anchor_density(m, &pred.params),
        "standing_constraints" => check_standing_constraints(m, &pred.params),
        "assertion_lists" => check_assertion_lists(m),
        "registry_conflict" => true,
        "time_deadline" => check_time_deadline(m, &pred.params, context),
        "parking_aging" => true,
        "gate_hold" => check_gate_hold(m),
        _ => true,
    }
}

// ---------- 逐材料路由与批级告警（route.py 对表，首败定路判在别处） ----------

fn route_material(m: &Value, pack: &Pack, context: Option<&str>) -> Value {
    let mut checks: Vec<Value> = Vec::new();
    let mut failed: Option<&Predicate> = None;
    let mut route = pack.pass_route.clone();
    for pred in &pack.predicates {
        let passed = evaluate(m, pred, context);
        checks.push(json!({"id": pred.id, "pass": passed}));
        if failed.is_none() && !passed {
            failed = Some(pred);
            route = pred.route_on_fail.clone().unwrap_or_default();
        }
    }
    let mut entry = Map::new();
    entry.insert("id".into(), m.get("id").cloned().unwrap_or(Value::Null));
    entry.insert("path".into(), m.get("path").cloned().unwrap_or(Value::Null));
    entry.insert("route".into(), Value::String(route));
    entry.insert(
        "failed_predicate".into(),
        failed
            .map(|p| Value::String(p.id.clone()))
            .unwrap_or(Value::Null),
    );
    entry.insert("checks".into(), Value::Array(checks));
    if m.get("round").map(|v| v.is_object()).unwrap_or(false) {
        entry.insert("round".into(), Value::Bool(true));
    }
    if m.get("parking").map(|v| v.is_object()).unwrap_or(false) {
        entry.insert("parking".into(), Value::Bool(true));
    }
    Value::Object(entry)
}

fn route_batch(materials: &[Value], pack: &Pack, context: Option<&str>) -> (Vec<Value>, Value) {
    let mut routed: Vec<Value> = Vec::new();
    let mut counts: HashMap<String, i64> = HashMap::new();
    for r in ROUTES {
        counts.insert(r.to_string(), 0);
    }
    for m in materials {
        let entry = route_material(m, pack, context);
        *counts
            .entry(entry["route"].as_str().unwrap_or_default().to_string())
            .or_insert(0) += 1;
        routed.push(entry);
    }
    let mut alarms: Vec<Value> = Vec::new();
    if !routed.is_empty() {
        let exempt_ids: BTreeSet<&str> = pack
            .predicates
            .iter()
            .filter(|p| pack.siding_surplus_exempt_kinds.iter().any(|k| *k == p.kind))
            .map(|p| p.id.as_str())
            .collect();
        let siding_countable = routed
            .iter()
            .filter(|e| {
                e["route"] == "siding"
                    && !e["failed_predicate"]
                        .as_str()
                        .map(|f| exempt_ids.contains(f))
                        .unwrap_or(false)
            })
            .count();
        if siding_countable as i64 >= pack.siding_surplus_threshold {
            alarms.push(json!({
                "kind": "siding_surplus",
                "count": siding_countable,
                "threshold": pack.siding_surplus_threshold,
            }));
        }
        if counts["mainline"] <= pack.mainline_starvation_threshold {
            alarms.push(json!({
                "kind": "mainline_starvation",
                "count": counts["mainline"],
                "threshold": pack.mainline_starvation_threshold,
            }));
        }
    }
    let mut conflicts: Vec<Value> = Vec::new();
    if pack
        .predicates
        .iter()
        .any(|p| p.kind == "registry_conflict")
    {
        for m in materials {
            conflicts.extend(registry_conflicts(m));
        }
    }
    alarms.extend(conflicts.iter().cloned());
    let mut agings: Vec<Value> = Vec::new();
    if let Some(ap) = pack.predicates.iter().find(|p| p.kind == "parking_aging") {
        for m in materials {
            agings.extend(parking_agings(m, &ap.params, context));
        }
    }
    alarms.extend(agings.iter().cloned());
    let mut summary = Map::new();
    summary.insert("total".into(), json!(routed.len()));
    summary.insert("mainline".into(), json!(counts["mainline"]));
    summary.insert("siding".into(), json!(counts["siding"]));
    summary.insert("scrap_track".into(), json!(counts["scrap_track"]));
    summary.insert("alarms".into(), Value::Array(alarms));
    if !conflicts.is_empty() {
        summary.insert("conflicts".into(), json!(conflicts.len()));
    }
    if !agings.is_empty() {
        summary.insert("agings".into(), json!(agings.len()));
    }
    (routed, Value::Object(summary))
}

// ---------- 材料展开与读取（cli.py 对表） ----------

fn expand_material_paths(raw_paths: &[String]) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    for raw in raw_paths {
        let p = Path::new(raw);
        if p.is_dir() {
            let mut kids: Vec<PathBuf> = Vec::new();
            for entry in fs::read_dir(p).map_err(|e| format!("material file missing: {raw}: {e}"))? {
                let entry = entry.map_err(|e| format!("material file missing: {raw}: {e}"))?;
                let name = entry.file_name();
                let name = name.to_string_lossy().into_owned();
                // Python glob("*.json") 语义：不匹配隐藏件，后缀大小写敏感。
                if name.starts_with('.') || !name.ends_with(".json") {
                    continue;
                }
                kids.push(entry.path());
            }
            kids.sort();
            paths.extend(kids);
        } else if p.is_file() {
            paths.push(p.to_path_buf());
        } else {
            return Err(format!("material file missing: {raw}"));
        }
    }
    Ok(paths)
}

fn read_material(path: &Path) -> Result<Value, String> {
    let disp = path.to_string_lossy();
    if !path.is_file() {
        return Err(format!("material file missing: {disp}"));
    }
    let bytes = fs::read(path).map_err(|e| format!("material json invalid: {disp}: {e}"))?;
    let text = String::from_utf8(bytes).map_err(|e| format!("material json invalid: {disp}: {e}"))?;
    let v: Value =
        serde_json::from_str(&text).map_err(|e| format!("material json invalid: {disp}: {e}"))?;
    if !v.is_object() {
        return Err(format!(
            "material json invalid: {disp}: top level must be an object"
        ));
    }
    Ok(v)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage_fail("缺子命令（route）");
    }
    if args[0] != "route" {
        usage_fail(&format!("未知子命令: {}", args[0]));
    }
    let mut pack_dir: Option<String> = None;
    let mut reference_time: Option<String> = None;
    let mut raw_paths: Vec<String> = Vec::new();
    let mut i = 1usize;
    while i < args.len() {
        let a = args[i].clone();
        if a == "--pack" {
            i += 1;
            if i >= args.len() {
                usage_fail("--pack 缺值");
            }
            pack_dir = Some(args[i].clone());
        } else if let Some(v) = a.strip_prefix("--pack=") {
            pack_dir = Some(v.to_string());
        } else if a == "--reference-time" {
            i += 1;
            if i >= args.len() {
                usage_fail("--reference-time 缺值");
            }
            reference_time = Some(args[i].clone());
        } else if let Some(v) = a.strip_prefix("--reference-time=") {
            reference_time = Some(v.to_string());
        } else if a.starts_with("--") {
            usage_fail(&format!("未知旗标: {a}"));
        } else {
            raw_paths.push(a);
        }
        i += 1;
    }
    let Some(pack_dir) = pack_dir else {
        usage_fail("缺 --pack（必填）");
    };

    let pack = match load_pack(Path::new(&pack_dir)) {
        Ok(p) => p,
        Err(e) => emit_error(&format!("pack invalid: {e}")),
    };
    if let Some(rt) = &reference_time {
        if parse_iso_date(rt).is_none() {
            emit_error(&format!("reference_time invalid ISO date: {rt}"));
        }
    }
    let paths = match expand_material_paths(&raw_paths) {
        Ok(p) => p,
        Err(e) => emit_error(&e),
    };
    let mut materials: Vec<Value> = Vec::new();
    for p in &paths {
        match read_material(p) {
            Ok(m) => materials.push(m),
            Err(e) => emit_error(&e),
        }
    }

    let (routed, summary) = route_batch(&materials, &pack, reference_time.as_deref());
    let has_alarms = summary["alarms"]
        .as_array()
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    let mut header = Map::new();
    header.insert(
        "tool".into(),
        json!({"name": TOOL_KEY, "version": ENGINE_VERSION}),
    );
    header.insert(
        "pack".into(),
        json!({"name": pack.name, "version": pack.version}),
    );
    header.insert(
        "domain".into(),
        json!({"include": pack.domain_include, "exclude": pack.domain_exclude}),
    );
    if let Some(rt) = &reference_time {
        header.insert("reference_time".into(), Value::String(rt.clone()));
    }
    let report = json!({
        "header": Value::Object(header),
        "routed": Value::Array(routed),
        "summary": summary,
        "status": "routed",
    });
    println!("{}", serde_json::to_string_pretty(&sort_value(report)).unwrap());
    exit(if has_alarms { 1 } else { 0 });
}
