//! route——路择谓词机（融回自 sih-tools/selector，承 SPEC-015 谓词融回落差
//! 规格与 GOV-002 判据二）。
//!
//! 判定知识全在包不在代码：空腹零内置谓词零内置路由，manifest.toml 加
//! routes.toml 成对的谓词包承载全部判定面。三路即 mainline 加 siding 加
//! scrap_track，首败定路，逐材料输出全部谓词过败记录。零网络零 LLM 零
//! key 读取零目标仓写入，判在别处。
//!
//! 谓词机对表基准：sih-tools/selector/src/selector/{predicates,route,pack,cli}.py
//! 现行文，十一 kinds 与 fail-closed 语义零漂移；包纯数据随迁
//! src/attractor/packs/{core,parking}/ 与围堰逐字节一致（同参形构成条件）。
//! 有余告警豁免语义随 parktune-solo 批对表围堰修订七：[alarms] 节可选键
//! siding_surplus_exempt_kinds 声明豁免清单即首败谓词 kind 属清单的侧轨件
//! 不计入积压计数，缺省零豁免，summary.siding 仍全量不隐藏。
//! 错误信封承围堰 f-string 原文形，其中 toml 解析失败与 json 解析失败的
//! 异常报文是运行时差异面（CPython 异常文本不逐字节复刻，拒收退出码与
//! 信封形态对表，字节冻结面只钉包校验可枚举类），此为显式范畴排除。

use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use serde_json::{json, Map, Value};

use super::jsonc::{canonical_json, parse_json, py_compact_sorted, verr, PyError};
use crate::snapline::glob::match_fnmatch_glob;

/// 围堰契约面版本字面：报告头 tool.version 承围堰 __version__，承 tally
/// 先例即工件工具名字段是契约面标识非二进制身份。
pub const SELECTOR_VERSION: &str = "0.4.0";
pub const TOOL_KEY: &str = "selector";

/// 三路枚举（Python 元组 repr 形用于错误信封）。
pub const ROUTES: [&str; 3] = ["mainline", "siding", "scrap_track"];
const ROUTES_REPR: &str = "('mainline', 'siding', 'scrap_track')";

/// 十一谓词 kinds：单件材料判定族四件、按轮判定族四件、时间维度族三件。
pub const KINDS: [&str; 4] = ["schema_required", "state_annotation", "anchor_whitelist", "write_boundary"];
pub const ROUND_KINDS: [&str; 4] = ["anchor_density", "standing_constraints", "assertion_lists", "registry_conflict"];
pub const PARKING_KINDS: [&str; 3] = ["time_deadline", "parking_aging", "gate_hold"];

const ALL_KINDS: [&str; 11] = [
    "schema_required",
    "state_annotation",
    "anchor_whitelist",
    "write_boundary",
    "anchor_density",
    "standing_constraints",
    "assertion_lists",
    "registry_conflict",
    "time_deadline",
    "parking_aging",
    "gate_hold",
];
const ALL_KINDS_REPR: &str = "('schema_required', 'state_annotation', 'anchor_whitelist', 'write_boundary', 'anchor_density', 'standing_constraints', 'assertion_lists', 'registry_conflict', 'time_deadline', 'parking_aging', 'gate_hold')";

/// route_on_fail 缺省四件即缺省 siding。
const ROUTE_ON_FAIL_DEFAULT: [&str; 4] = ["anchor_density", "standing_constraints", "assertion_lists", "time_deadline"];

// ---------------------------------------------------------------------------
// 谓词与包
// ---------------------------------------------------------------------------

/// kind 专属参数（承围堰 Predicate.params）。
#[derive(Debug, Clone)]
pub enum Params {
    Fields(Vec<String>),
    Allowed(Vec<String>),
    Whitelist(Vec<String>),
    AllowedRoots(Vec<String>),
    AnchorDensity { density_threshold: f64, resolve_root: String },
    RequiredReceipts(Vec<String>),
    /// 阈值保形承载即 toml 原始数值形（整数 45 报告出 45 非浮点 45.0）。
    AgingThresholdDays(Value),
    None,
}

/// 一条已校验谓词。
#[derive(Debug, Clone)]
pub struct Predicate {
    pub id: String,
    pub kind: String,
    pub route_on_fail: Option<String>,
    pub params: Params,
}

/// 已校验谓词包。
#[derive(Debug, Clone)]
pub struct Pack {
    pub name: String,
    pub version: String,
    pub domain: Value,
    pub predicates: Vec<Predicate>,
    pub pass_route: String,
    pub siding_surplus_threshold: i64,
    pub mainline_starvation_threshold: i64,
    /// 有余告警豁免清单（承围堰修订七 [alarms] 可选键，缺省空即零豁免）。
    pub siding_surplus_exempt_kinds: Vec<String>,
}

/// 读包目录下 manifest.toml 与 routes.toml，全量校验后返回 Pack。
///
/// 错误信封消息承围堰 PackError f-string 原文形（对表 pack.py）。
pub fn load_pack(pack_dir: &Path) -> Result<Pack, PyError> {
    if !pack_dir.is_dir() {
        return Err(verr(format!("pack directory missing: {}", pack_dir.display())));
    }
    let manifest = load_toml_file(&pack_dir.join("manifest.toml"), "manifest.toml")?;
    let routes = load_toml_file(&pack_dir.join("routes.toml"), "routes.toml")?;
    let (name, version, domain) = parse_manifest(&manifest, pack_dir)?;
    let predicates = parse_predicates(&routes, pack_dir)?;
    let (pass_route, siding, starvation, exempt_kinds) = parse_tail(&routes, pack_dir)?;
    Ok(Pack { name, version, domain, predicates, pass_route, siding_surplus_threshold: siding, mainline_starvation_threshold: starvation, siding_surplus_exempt_kinds: exempt_kinds })
}

fn load_toml_file(path: &Path, label: &str) -> Result<toml::Value, PyError> {
    if !path.is_file() {
        return Err(verr(format!("{label} missing: {}", path.display())));
    }
    let bytes = fs::read(path).map_err(|e| PyError {
        kind: "OSError",
        msg: format!("{label} unreadable: {}: {}", path.display(), e),
    })?;
    let text = String::from_utf8(bytes).map_err(|e| {
        // CPython UnicodeDecodeError 报文形对表：'utf-8' codec can't decode ...
        let pos = e.utf8_error().valid_up_to();
        let bad = e.as_bytes().get(pos).copied().unwrap_or(0);
        PyError {
            kind: "UnicodeDecodeError",
            msg: format!(
                "{label} not utf-8 decodable: {}: 'utf-8' codec can't decode byte 0x{bad:02x} in position {pos}: invalid start byte",
                path.display()
            ),
        }
    })?;
    text.parse::<toml::Value>().map_err(|e| PyError {
        kind: "TOMLDecodeError",
        msg: format!("{label} not valid toml: {}: {e}", path.display()),
    })
}

/// require 字符串：`type(value) is not str or not value`（围堰 _require_str）。
fn require_str(table: &toml::Value, key: &str, ctx: &str) -> Result<String, PyError> {
    let value = table.get(key);
    match value {
        Some(toml::Value::String(s)) if !s.is_empty() => Ok(s.clone()),
        _ => Err(verr(format!("{ctx}: {key} must be a non-empty string"))),
    }
}

/// require 字符串数组：`isinstance(value, list)` 且逐项 `type(item) is str`
/// 且非空（围堰 _require_str_list，allow_empty 只对 exclude 开）。
fn require_str_list(table: &toml::Value, key: &str, ctx: &str, allow_empty: bool) -> Result<Vec<String>, PyError> {
    let Some(toml::Value::Array(items)) = table.get(key) else {
        return Err(verr(format!("{ctx}: {key} must be a non-empty string array")));
    };
    if items.is_empty() && !allow_empty {
        return Err(verr(format!("{ctx}: {key} must be a non-empty string array")));
    }
    let mut out = Vec::new();
    for item in items {
        match item {
            toml::Value::String(s) if !s.is_empty() => out.push(s.clone()),
            _ => return Err(verr(format!("{ctx}: {key} entries must be non-empty strings"))),
        }
    }
    Ok(out)
}

/// require 非负整数：`type(value) is not int or value < 0`（布尔拒收）。
fn require_threshold(table: &toml::Value, key: &str, ctx: &str) -> Result<i64, PyError> {
    match table.get(key) {
        Some(toml::Value::Integer(v)) if *v >= 0 => Ok(*v),
        _ => Err(verr(format!("{ctx}: {key} must be a non-negative integer"))),
    }
}

/// Python f-string 值显示：字符串原样、True/False、数值 repr。
fn py_display(v: &toml::Value) -> String {
    match v {
        toml::Value::String(s) => s.clone(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => format_float_py(*f),
        toml::Value::Boolean(b) => if *b { "True".into() } else { "False".into() },
        toml::Value::Datetime(d) => d.to_string(),
        toml::Value::Table(_) => "{}".into(),
        toml::Value::Array(_) => "[]".into(),
    }
}

/// Python float repr 最小面：整数值浮点补 .0，其余经 ryu 通道（serde_json）。
fn format_float_py(f: f64) -> String {
    let s = serde_json::to_string(&json!(f)).unwrap_or_default();
    s
}

fn parse_manifest(data: &toml::Value, directory: &Path) -> Result<(String, String, Value), PyError> {
    let ctx = format!("{}/manifest.toml", directory.display());
    let name = require_str(data, "name", &ctx)?;
    let version = require_str(data, "version", &ctx)?;
    let Some(domain_raw) = data.get("domain") else {
        return Err(verr(format!("{ctx}: domain must be a table")));
    };
    let toml::Value::Table(_) = domain_raw else {
        return Err(verr(format!("{ctx}: domain must be a table")));
    };
    let include = require_str_list(domain_raw, "include", &format!("{ctx} domain"), false)?;
    let exclude = require_str_list(domain_raw, "exclude", &format!("{ctx} domain"), true)?;
    let domain = json!({"include": include, "exclude": exclude});
    Ok((name, version, domain))
}

fn parse_predicates(data: &toml::Value, directory: &Path) -> Result<Vec<Predicate>, PyError> {
    let ctx = format!("{}/routes.toml", directory.display());
    // 围堰形：data.get("predicates", [])，键缺省即空包合法（F6 空包全走
    // pass_route），非数组才拒
    let raw: Vec<toml::Value> = match data.get("predicates") {
        None => Vec::new(),
        Some(toml::Value::Array(items)) => items.clone(),
        Some(_) => return Err(verr(format!("{ctx}: predicates must be an array of tables"))),
    };
    let mut predicates = Vec::new();
    for (index, table) in raw.iter().enumerate() {
        let pctx = format!("{ctx} predicates[{index}]");
        let toml::Value::Table(_) = table else {
            return Err(verr(format!("{pctx}: must be a table")));
        };
        let predicate_id = require_str(table, "id", &pctx)?;
        let kind = require_str(table, "kind", &pctx)?;
        if !ALL_KINDS.contains(&kind.as_str()) {
            return Err(verr(format!("{pctx}: kind must be one of {ALL_KINDS_REPR}, got {kind}")));
        }
        let route_on_fail = parse_route_on_fail(&kind, table, &pctx)?;
        let params = parse_params(&kind, table, &pctx)?;
        predicates.push(Predicate { id: predicate_id, kind, route_on_fail, params });
    }
    Ok(predicates)
}

fn parse_route_on_fail(kind: &str, table: &toml::Value, pctx: &str) -> Result<Option<String>, PyError> {
    let value = table.get("route_on_fail");
    if kind == "registry_conflict" || kind == "parking_aging" {
        if value.is_some() {
            return Err(verr(format!("{pctx}: route_on_fail is not allowed for {kind}")));
        }
        return Ok(None);
    }
    let parsed = match value {
        None => {
            if ROUTE_ON_FAIL_DEFAULT.contains(&kind) {
                "siding".to_string()
            } else {
                // 围堰形：None not in ROUTES → 报文带 got None
                return Err(verr(format!("{pctx}: route_on_fail must be one of {ROUTES_REPR}, got None")));
            }
        }
        Some(v) => {
            let toml::Value::String(s) = v else {
                return Err(verr(format!("{pctx}: route_on_fail must be one of {ROUTES_REPR}, got {}", py_display(v))));
            };
            if !ROUTES.contains(&s.as_str()) {
                return Err(verr(format!("{pctx}: route_on_fail must be one of {ROUTES_REPR}, got {s}")));
            }
            s.clone()
        }
    };
    Ok(Some(parsed))
}

fn parse_params(kind: &str, table: &toml::Value, pctx: &str) -> Result<Params, PyError> {
    match kind {
        "anchor_density" => {
            let threshold = match table.get("density_threshold") {
                None => 1.0,
                Some(toml::Value::Float(f)) => *f,
                Some(toml::Value::Integer(i)) => *i as f64,
                Some(_) => {
                    return Err(verr(format!("{pctx}: density_threshold must be a number")))
                }
            };
            let resolve_root = require_str(table, "resolve_root", pctx)?;
            Ok(Params::AnchorDensity { density_threshold: threshold, resolve_root })
        }
        "standing_constraints" => {
            let list = require_str_list(table, "required_receipts", pctx, false)?;
            Ok(Params::RequiredReceipts(list))
        }
        "parking_aging" => {
            let threshold = match table.get("aging_threshold_days") {
                Some(toml::Value::Float(f)) if *f > 0.0 => json!(f),
                Some(toml::Value::Integer(i)) if *i > 0 => json!(i),
                _ => return Err(verr(format!("{pctx}: aging_threshold_days must be a positive number"))),
            };
            Ok(Params::AgingThresholdDays(threshold))
        }
        "assertion_lists" | "registry_conflict" | "time_deadline" | "gate_hold" => Ok(Params::None),
        other => {
            // 四单件族：schema_required/state_annotation/anchor_whitelist/write_boundary
            let key = match other {
                "schema_required" => "fields",
                "state_annotation" => "allowed",
                "anchor_whitelist" => "whitelist",
                "write_boundary" => "allowed_roots",
                _ => unreachable!("kind 已在上游校验"),
            };
            let list = require_str_list(table, key, pctx, false)?;
            Ok(match key {
                "fields" => Params::Fields(list),
                "allowed" => Params::Allowed(list),
                "whitelist" => Params::Whitelist(list),
                _ => Params::AllowedRoots(list),
            })
        }
    }
}

fn parse_tail(data: &toml::Value, directory: &Path) -> Result<(String, i64, i64, Vec<String>), PyError> {
    let ctx = format!("{}/routes.toml", directory.display());
    let Some(defaults) = data.get("defaults") else {
        return Err(verr(format!("{ctx}: defaults must be a table")));
    };
    let toml::Value::Table(_) = defaults else {
        return Err(verr(format!("{ctx}: defaults must be a table")));
    };
    let pass_route = match defaults.get("pass_route") {
        Some(toml::Value::String(s)) if ROUTES.contains(&s.as_str()) => s.clone(),
        Some(v) => {
            return Err(verr(format!(
                "{ctx} defaults: pass_route must be one of {ROUTES_REPR}, got {}",
                py_display(v)
            )))
        }
        None => {
            return Err(verr(format!(
                "{ctx} defaults: pass_route must be one of {ROUTES_REPR}, got None"
            )))
        }
    };
    let Some(alarms) = data.get("alarms") else {
        return Err(verr(format!("{ctx}: alarms must be a table")));
    };
    let toml::Value::Table(_) = alarms else {
        return Err(verr(format!("{ctx}: alarms must be a table")));
    };
    let siding = require_threshold(alarms, "siding_surplus_threshold", &format!("{ctx} alarms"))?;
    let starvation = require_threshold(alarms, "mainline_starvation_threshold", &format!("{ctx} alarms"))?;
    let exempt = parse_exempt_kinds(alarms, &format!("{ctx} alarms"))?;
    Ok((pass_route, siding, starvation, exempt))
}

/// siding_surplus_exempt_kinds 解析（对表围堰 _parse_exempt_kinds）：
/// 可选键缺省空即零豁免，值须为字符串数组且逐项合法谓词 kind，typo 拒包。
fn parse_exempt_kinds(alarms: &toml::Value, ctx: &str) -> Result<Vec<String>, PyError> {
    let raw = match alarms.get("siding_surplus_exempt_kinds") {
        None => return Ok(Vec::new()),
        Some(toml::Value::Array(items)) => items.clone(),
        Some(_) => {
            return Err(verr(format!("{ctx}: siding_surplus_exempt_kinds must be a string array")))
        }
    };
    let mut out = Vec::new();
    for item in raw {
        let Some(s) = item.as_str() else {
            return Err(verr(format!("{ctx}: siding_surplus_exempt_kinds must be a string array")));
        };
        if !ALL_KINDS.contains(&s) {
            return Err(verr(format!(
                "{ctx}: siding_surplus_exempt_kinds entries must be one of {ALL_KINDS_REPR}, got {s}"
            )));
        }
        out.push(s.to_string());
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// 谓词求值（对表 predicates.py，fail-closed 语义零漂移）
// ---------------------------------------------------------------------------

fn all_match(values: Option<&Value>, patterns: &[String]) -> bool {
    let Some(Value::Array(items)) = values else {
        return false;
    };
    for value in items {
        let Some(s) = value.as_str() else {
            return false;
        };
        if !patterns.iter().any(|p| match_fnmatch_glob(s, p)) {
            return false;
        }
    }
    true
}

fn is_round_record(material: &Value) -> bool {
    material.get("round").map(|v| v.is_object()).unwrap_or(false)
}

fn is_parking_record(material: &Value) -> bool {
    material.get("parking").map(|v| v.is_object()).unwrap_or(false)
}

/// round 键缺省或非对象返回 None，轮级谓词一律判败。
fn round_data<'a>(material: &'a Value) -> Option<&'a Map<String, Value>> {
    material.get("round").and_then(|v| v.as_object())
}

/// round 内数组装载：缺省空数组，在场非数组即结构非法 None。
fn round_list<'a>(round: &'a Map<String, Value>, key: &str) -> Option<Option<&'a Vec<Value>>> {
    match round.get(key) {
        None => Some(None),
        Some(Value::Array(a)) => Some(Some(a)),
        Some(_) => None,
    }
}

fn evidence_path(entry: &str) -> &str {
    // 条目形「路径」或「路径:行」：冒号尾段为行号时取路径前缀
    match entry.rsplit_once(':') {
        Some((head, tail)) if !head.is_empty() && !tail.is_empty() && tail.bytes().all(|b| b.is_ascii_digit()) => head,
        _ => entry,
    }
}

fn has_valid_anchor(assertion: &Value, resolve_root: &str) -> bool {
    let Some(Value::Array(evidence)) = assertion.get("evidence") else {
        return false;
    };
    for entry in evidence {
        let Some(text) = entry.as_str() else { continue };
        let path_text = evidence_path(text);
        if path_text.is_empty() || path_text.starts_with('/') {
            continue;
        }
        if Path::new(resolve_root).join(path_text).exists() {
            return true;
        }
    }
    false
}

/// 参照时间 ISO 日期解析，非字符串或非法格式返回 None（对表 _parse_date）。
fn parse_iso_date(value: Option<&Value>) -> Option<NaiveDate> {
    let text = value?.as_str()?;
    NaiveDate::parse_from_str(text, "%Y-%m-%d").ok()
}

fn check_schema_required(material: &Value, params: &Params) -> bool {
    let Params::Fields(fields) = params else { return false };
    fields.iter().all(|f| material.get(f).map(|v| v.is_string()).unwrap_or(false))
}

fn check_state_annotation(material: &Value, params: &Params) -> bool {
    let Params::Allowed(allowed) = params else { return false };
    match material.get("state").and_then(|v| v.as_str()) {
        Some(state) => allowed.iter().any(|a| a == state),
        None => false,
    }
}

fn check_anchor_whitelist(material: &Value, params: &Params) -> bool {
    let Params::Whitelist(whitelist) = params else { return false };
    all_match(material.get("anchors"), whitelist)
}

fn check_write_boundary(material: &Value, params: &Params) -> bool {
    let Params::AllowedRoots(roots) = params else { return false };
    all_match(material.get("requested_writes"), roots)
}

fn check_anchor_density(material: &Value, params: &Params) -> bool {
    let Params::AnchorDensity { density_threshold, resolve_root } = params else { return false };
    let Some(round) = round_data(material) else { return false };
    let Some(assertions) = round_list(round, "assertions") else { return false };
    let Some(assertions) = assertions else { return true };
    if assertions.is_empty() {
        return true;
    }
    let rooted = assertions.iter().filter(|a| has_valid_anchor(a, resolve_root)).count();
    (rooted as f64) / (assertions.len() as f64) >= *density_threshold
}

fn check_standing_constraints(material: &Value, params: &Params) -> bool {
    let Params::RequiredReceipts(required) = params else { return false };
    let Some(round) = round_data(material) else { return false };
    let Some(receipts) = round_list(round, "receipts") else { return false };
    let Some(receipts) = receipts else { return true };
    let tools: Vec<&str> = receipts
        .iter()
        .filter_map(|r| r.get("tool").and_then(|t| t.as_str()))
        .collect();
    required.iter().all(|req| tools.iter().any(|t| *t == req.as_str()))
}

fn check_assertion_lists(material: &Value) -> bool {
    let Some(round) = round_data(material) else { return false };
    let Some(assertions) = round_list(round, "assertions") else { return false };
    let Some(assertions) = assertions else { return true };
    for assertion in assertions {
        let Some(Value::Array(evidence)) = assertion.get("evidence") else {
            return false;
        };
        if evidence.is_empty() {
            return false;
        }
    }
    true
}

/// 到期判定：参照时间越过 entered_at 加 ttl_days 即败，到期日当日即败。
/// ttl 浮点按 timedelta 归一日数截断（Python date + timedelta 语义）。
fn check_time_deadline(material: &Value, context: Option<&Value>) -> bool {
    let Some(parking) = material.get("parking").and_then(|v| v.as_object()) else {
        return false;
    };
    let Some(entered) = parse_iso_date(parking.get("entered_at")) else {
        return false;
    };
    let ttl_value = parking.get("ttl_days");
    let ttl = match ttl_value {
        Some(Value::Number(n)) if n.is_i64() || n.is_u64() => n.as_f64(),
        Some(Value::Number(n)) => n.as_f64(),
        _ => None,
    };
    let Some(ttl) = ttl else { return false };
    if ttl <= 0.0 {
        return false;
    }
    // 布尔在 serde_json 非数值天然排除
    let Some(reference) = parse_iso_date(context.and_then(|c| c.get("reference_time"))) else {
        return false;
    };
    let deadline = entered + chrono::Duration::days(ttl.trunc() as i64);
    reference < deadline
}

/// 搁置判定（对表围堰 check_gate_hold）：gate 键为对象且 fired_at 缺省或
/// 非字符串即未点火判败走 route_on_fail，fired_at 在场即已点火判过交后续
/// 谓词定路。gate 键缺省或非对象即非搁置件判过零影响。只查结构不解读
/// 触发器语义，点火判定在别处。
fn check_gate_hold(material: &Value) -> bool {
    let Some(gate) = material.get("gate") else {
        return true;
    };
    if !gate.is_object() {
        return true;
    }
    gate.get("fired_at").map(|v| v.is_string()).unwrap_or(false)
}

// ---------------------------------------------------------------------------
// 路由与批装配（对表 route.py）
// ---------------------------------------------------------------------------

/// 按 kind 分发求值。kind 已在包装载时校验。
pub fn evaluate(material: &Value, predicate: &Predicate, context: Option<&Value>) -> bool {
    match predicate.kind.as_str() {
        "schema_required" => check_schema_required(material, &predicate.params),
        "state_annotation" => check_state_annotation(material, &predicate.params),
        "anchor_whitelist" => check_anchor_whitelist(material, &predicate.params),
        "write_boundary" => check_write_boundary(material, &predicate.params),
        "anchor_density" => check_anchor_density(material, &predicate.params),
        "standing_constraints" => check_standing_constraints(material, &predicate.params),
        "assertion_lists" => check_assertion_lists(material),
        "registry_conflict" => true,
        "time_deadline" => check_time_deadline(material, context),
        "parking_aging" => true,
        "gate_hold" => check_gate_hold(material),
        _ => false,
    }
}

/// 单材料路由：谓词按包内声明顺序评估，首败定路，全过走 pass_route。
pub fn route_material(material: &Value, pack: &Pack, context: Option<&Value>) -> Value {
    let mut checks = Vec::new();
    let mut failed_predicate: Option<String> = None;
    let mut route = pack.pass_route.clone();
    for predicate in &pack.predicates {
        let passed = evaluate(material, predicate, context);
        checks.push(json!({"id": predicate.id, "pass": passed}));
        if failed_predicate.is_none() && !passed {
            failed_predicate = Some(predicate.id.clone());
            route = predicate.route_on_fail.clone().unwrap_or_else(|| pack.pass_route.clone());
        }
    }
    let mut entry = json!({
        "id": material.get("id").cloned().unwrap_or(Value::Null),
        "path": material.get("path").cloned().unwrap_or(Value::Null),
        "route": route,
        "failed_predicate": failed_predicate,
        "checks": checks,
    });
    if is_round_record(material) {
        entry["round"] = json!(true);
    }
    if is_parking_record(material) {
        entry["parking"] = json!(true);
    }
    entry
}

/// 逐结论查 establishes_term 撞回执 flagged_terms 产告警（对表 registry_conflicts）。
fn registry_conflict_alarms(material: &Value) -> Vec<Value> {
    let mut alarms = Vec::new();
    let Some(round) = round_data(material) else { return alarms };
    let (Some(conclusions), Some(receipts)) = (round_list(round, "conclusions"), round_list(round, "receipts")) else {
        return alarms;
    };
    let conclusions = conclusions.unwrap_or(&Vec::new()).clone();
    let receipts = receipts.unwrap_or(&Vec::new()).clone();
    let mut flagged: Vec<String> = Vec::new();
    for receipt in &receipts {
        if let Some(Value::Array(terms)) = receipt.get("flagged_terms") {
            for term in terms {
                if let Some(t) = term.as_str() {
                    if !flagged.iter().any(|f| f == t) {
                        flagged.push(t.to_string());
                    }
                }
            }
        }
    }
    for conclusion in &conclusions {
        if conclusion.get("kind").and_then(|k| k.as_str()) != Some("establishes_term") {
            continue;
        }
        if let Some(term) = conclusion.get("term").and_then(|t| t.as_str()) {
            if flagged.iter().any(|f| f == term) {
                alarms.push(json!({"kind": "registry_conflict", "term": term, "conclusion": "establishes_term"}));
            }
        }
    }
    alarms
}

/// 参照时间与 entered_at 差值达阈值逐件产告警（对表 parking_agings）。
/// 未点火搁置件即 gate 键为对象且 fired_at 非字符串时零告警即停计
/// （围堰 parkgate 修订六语义，引擎随 parktune-solo 批追平）。
fn parking_aging_alarms(material: &Value, threshold: &Value, context: Option<&Value>) -> Vec<Value> {
    let mut alarms = Vec::new();
    if let Some(gate) = material.get("gate") {
        if gate.is_object() && !gate.get("fired_at").map(|v| v.is_string()).unwrap_or(false) {
            return alarms;
        }
    }
    let Some(threshold_days) = threshold.as_f64() else {
        return alarms;
    };
    let Some(parking) = material.get("parking").and_then(|v| v.as_object()) else {
        return alarms;
    };
    let Some(entered) = parse_iso_date(parking.get("entered_at")) else {
        return alarms;
    };
    let Some(reference) = parse_iso_date(context.and_then(|c| c.get("reference_time"))) else {
        return alarms;
    };
    let age_days = (reference - entered).num_days();
    if (age_days as f64) >= threshold_days {
        alarms.push(json!({
            "kind": "parking_aging",
            "id": material.get("id").cloned().unwrap_or(Value::Null),
            "age_days": age_days,
            "threshold": threshold,
        }));
    }
    alarms
}

/// 批路由加批级告警。返回 routed 条目数组与 summary 对象。
pub fn route_batch(materials: &[Value], pack: &Pack, reference_time: Option<&str>) -> (Vec<Value>, Value) {
    let context = json!({"reference_time": reference_time});
    let context_ref = Some(&context);
    let routed: Vec<Value> = materials.iter().map(|m| route_material(m, pack, context_ref)).collect();
    let mut counts: std::collections::HashMap<&str, i64> = std::collections::HashMap::new();
    for r in ROUTES {
        counts.insert(r, 0);
    }
    for item in &routed {
        let r = item["route"].as_str().unwrap_or_default();
        *counts.entry(r).or_insert(0) += 1;
    }
    let mut alarms: Vec<Value> = Vec::new();
    if !routed.is_empty() {
        // 有余告警积压计数排除首败谓词 kind 属豁免清单的侧轨件（对表围堰
        // 修订七 siding_surplus_exempt_kinds 声明面），summary.siding 仍全量。
        let exempt_ids: std::collections::HashSet<&str> = pack
            .predicates
            .iter()
            .filter(|p| pack.siding_surplus_exempt_kinds.iter().any(|k| k == &p.kind))
            .map(|p| p.id.as_str())
            .collect();
        let siding_countable = routed
            .iter()
            .filter(|item| {
                item["route"].as_str() == Some("siding")
                    && !item["failed_predicate"]
                        .as_str()
                        .map(|fp| exempt_ids.contains(fp))
                        .unwrap_or(false)
            })
            .count() as i64;
        if siding_countable >= pack.siding_surplus_threshold {
            alarms.push(json!({"kind": "siding_surplus", "count": siding_countable, "threshold": pack.siding_surplus_threshold}));
        }
        let mainline = counts.get("mainline").copied().unwrap_or(0);
        if mainline <= pack.mainline_starvation_threshold {
            alarms.push(json!({"kind": "mainline_starvation", "count": mainline, "threshold": pack.mainline_starvation_threshold}));
        }
    }
    let mut conflicts: Vec<Value> = Vec::new();
    if pack.predicates.iter().any(|p| p.kind == "registry_conflict") {
        for material in materials {
            conflicts.extend(registry_conflict_alarms(material));
        }
    }
    let conflicts_len = conflicts.len();
    alarms.extend(conflicts);
    let mut agings: Vec<Value> = Vec::new();
    let aging_predicate = pack.predicates.iter().find(|p| p.kind == "parking_aging");
    if let Some(p) = aging_predicate {
        if let Params::AgingThresholdDays(threshold) = &p.params {
            for material in materials {
                agings.extend(parking_aging_alarms(material, threshold, context_ref));
            }
        }
    }
    let agings_len = agings.len();
    alarms.extend(agings);
    let mut summary = json!({
        "total": routed.len(),
        "mainline": counts.get("mainline").copied().unwrap_or(0),
        "siding": counts.get("siding").copied().unwrap_or(0),
        "scrap_track": counts.get("scrap_track").copied().unwrap_or(0),
        "alarms": alarms,
    });
    if conflicts_len > 0 {
        summary["conflicts"] = json!(conflicts_len);
    }
    if agings_len > 0 {
        summary["agings"] = json!(agings_len);
    }
    (routed, summary)
}

/// 报告装配（对表 cli.py report 形）。
pub fn build_report(pack: &Pack, routed_summary: &(Vec<Value>, Value), reference_time: Option<&str>) -> Value {
    let mut header = json!({
        "tool": {"name": TOOL_KEY, "version": SELECTOR_VERSION},
        "pack": {"name": pack.name, "version": pack.version},
        "domain": pack.domain,
    });
    if let Some(rt) = reference_time {
        header["reference_time"] = json!(rt);
    }
    json!({
        "header": header,
        "routed": routed_summary.0,
        "summary": routed_summary.1,
        "status": "routed",
    })
}

/// 报告落盘形：sort_keys 加 indent 2 加非 ASCII 原样（不含尾换行）。
pub fn report_json(report: &Value) -> String {
    canonical_json(report)
}

/// 材料装载：json 对象校验，错误信封消息承围堰原文形。
pub fn parse_material(text: &str, raw_path: &str) -> Result<Value, PyError> {
    let material = parse_json(text).map_err(|e| {
        PyError { kind: "JSONDecodeError", msg: format!("material json invalid: {raw_path}: {e}") }
    })?;
    if !material.is_object() {
        return Err(verr(format!("material json invalid: {raw_path}: top level must be an object")));
    }
    Ok(material)
}

/// route 执行入口（对表 cli.py main 流程序：装载包 → 校验参照时间 →
/// 展开材料 → 批路由）。返回报告文本与退出码，或错误信封文本与退出码二。
pub fn run_route(
    pack_dir: &Path,
    reference_time: Option<&str>,
    material_paths: &[String],
) -> Result<(String, i32), (String, i32)> {
    let pack = load_pack(pack_dir).map_err(|e| {
        // 围堰形：str(exc) 只取消息文不挂异常类型前缀
        (py_compact_sorted(&json!({"error": format!("pack invalid: {}", e.message())})), 2)
    })?;
    if let Some(rt) = reference_time {
        if parse_iso_date(Some(&json!(rt))).is_none() {
            return Err((
                py_compact_sorted(&json!({"error": format!("reference_time invalid ISO date: {rt}")})),
                2,
            ));
        }
    }
    let mut materials = Vec::new();
    for raw_path in material_paths {
        let path = Path::new(raw_path);
        if path.is_dir() {
            let mut entries: Vec<PathBuf> = fs::read_dir(path)
                .map_err(|e| (py_compact_sorted(&json!({"error": format!("material file missing: {raw_path}: {e}")})), 2))?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().map(|x| x == "json").unwrap_or(false))
                .collect();
            entries.sort();
            for p in entries {
                push_material(&mut materials, &p.to_string_lossy())?;
            }
        } else if path.is_file() {
            push_material(&mut materials, raw_path)?;
        } else {
            return Err((py_compact_sorted(&json!({"error": format!("material file missing: {raw_path}")})), 2));
        }
    }
    let (routed, summary) = route_batch(&materials, &pack, reference_time);
    let report = build_report(&pack, &(routed, summary), reference_time);
    let exit = if report["summary"]["alarms"].as_array().map(|a| !a.is_empty()).unwrap_or(false) { 1 } else { 0 };
    Ok((format!("{}\n", report_json(&report)), exit))
}

fn push_material(materials: &mut Vec<Value>, raw_path: &str) -> Result<(), (String, i32)> {
    let path = Path::new(raw_path);
    if !path.is_file() {
        return Err((py_compact_sorted(&json!({"error": format!("material file missing: {raw_path}")})), 2));
    }
    let text = fs::read_to_string(path).map_err(|e| {
        (py_compact_sorted(&json!({"error": format!("material json invalid: {raw_path}: {e}")})), 2)
    })?;
    let material = parse_material(&text, raw_path).map_err(|e| {
        (py_compact_sorted(&json!({"error": e.msg})), 2)
    })?;
    materials.push(material);
    Ok(())
}

// ---------------------------------------------------------------------------
// T4 谓词机语义单测（先红后绿红态：本模块未建时集成测试编译失败）
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn pack_of(predicates: serde_json::Value) -> Pack {
        // 测试辅助：直接构造已校验包（包数据校验由 load_pack 集成路径承载）
        let arr = predicates.as_array().unwrap();
        let mut out = Vec::new();
        for p in arr {
            let kind = p["kind"].as_str().unwrap().to_string();
            let params = match kind.as_str() {
                "schema_required" => Params::Fields(p["fields"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect()),
                "state_annotation" => Params::Allowed(p["allowed"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect()),
                "anchor_whitelist" => Params::Whitelist(p["whitelist"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect()),
                "write_boundary" => Params::AllowedRoots(p["allowed_roots"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect()),
                "anchor_density" => Params::AnchorDensity {
                    density_threshold: p["density_threshold"].as_f64().unwrap_or(1.0),
                    resolve_root: p["resolve_root"].as_str().unwrap_or(".").to_string(),
                },
                "standing_constraints" => Params::RequiredReceipts(p["required_receipts"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect()),
                "parking_aging" => Params::AgingThresholdDays(p["aging_threshold_days"].clone()),
                _ => Params::None,
            };
            out.push(Predicate {
                id: p["id"].as_str().unwrap().to_string(),
                kind: kind.clone(),
                route_on_fail: p.get("route_on_fail").and_then(|v| v.as_str()).map(|s| s.to_string()).or_else(|| {
                    if ROUTE_ON_FAIL_DEFAULT.contains(&kind.as_str()) {
                        Some("siding".to_string())
                    } else {
                        None
                    }
                }),
                params,
            });
        }
        Pack {
            name: "test".into(),
            version: "0.0.0".into(),
            domain: json!({"include": ["**/**"], "exclude": []}),
            predicates: out,
            pass_route: "mainline".into(),
            siding_surplus_threshold: 99,
            mainline_starvation_threshold: 0,
            siding_surplus_exempt_kinds: Vec::new(),
        }
    }

    #[test]
    fn t4_schema_required() {
        let pack = pack_of(json!([{"id": "R1", "kind": "schema_required", "route_on_fail": "scrap_track", "fields": ["id", "state"]}]));
        let good = json!({"id": "a", "state": "draft"});
        let bad = json!({"id": "a", "state": 1});
        let bad2 = json!({"id": "a"});
        assert!(evaluate(&good, &pack.predicates[0], None));
        assert!(!evaluate(&bad, &pack.predicates[0], None));
        assert!(!evaluate(&bad2, &pack.predicates[0], None));
    }

    #[test]
    fn t4_state_annotation_enum() {
        let pack = pack_of(json!([{"id": "R2", "kind": "state_annotation", "route_on_fail": "siding", "allowed": ["process", "draft"]}]));
        assert!(evaluate(&json!({"state": "draft"}), &pack.predicates[0], None));
        assert!(!evaluate(&json!({"state": "signed"}), &pack.predicates[0], None));
        assert!(!evaluate(&json!({}), &pack.predicates[0], None));
    }

    #[test]
    fn t4_anchor_whitelist_and_boundary() {
        let pack = pack_of(json!([
            {"id": "R3", "kind": "anchor_whitelist", "route_on_fail": "siding", "whitelist": ["doc/**", "skills/**"]},
            {"id": "R4", "kind": "write_boundary", "route_on_fail": "scrap_track", "allowed_roots": ["sih-tools/**"]}
        ]));
        assert!(evaluate(&json!({"anchors": ["doc/a.md", "skills/b.md"]}), &pack.predicates[0], None));
        assert!(evaluate(&json!({"anchors": []}), &pack.predicates[0], None), "空数组过");
        assert!(!evaluate(&json!({"anchors": ["../etc/passwd"]}), &pack.predicates[0], None), "白名单外判败");
        assert!(!evaluate(&json!({}), &pack.predicates[0], None), "anchors 缺席判败");
        assert!(evaluate(&json!({"requested_writes": ["sih-tools/x/y.py"]}), &pack.predicates[1], None));
        assert!(!evaluate(&json!({"requested_writes": ["/etc/passwd"]}), &pack.predicates[1], None));
    }

    #[test]
    fn t4_round_fail_closed() {
        let pack = pack_of(json!([
            {"id": "R101", "kind": "assertion_lists", "route_on_fail": "siding"},
            {"id": "R102", "kind": "anchor_density", "route_on_fail": "siding", "density_threshold": 1.0, "resolve_root": "/Users/moc/workspaces/SiHankor/sih-engine"},
            {"id": "R103", "kind": "standing_constraints", "route_on_fail": "siding", "required_receipts": ["nomenclator"]}
        ]));
        // round 键缺省：轮级谓词一律判败
        assert!(!evaluate(&json!({"id": "x"}), &pack.predicates[0], None));
        assert!(!evaluate(&json!({"id": "x"}), &pack.predicates[1], None));
        assert!(!evaluate(&json!({"id": "x"}), &pack.predicates[2], None));
        // round 非对象同败
        assert!(!evaluate(&json!({"round": "no"}), &pack.predicates[0], None));
        // 空断言数组：anchor_density 判过、assertion_lists 判过
        let empty_round = json!({"round": {"assertions": [], "conclusions": [], "receipts": []}});
        assert!(evaluate(&empty_round, &pack.predicates[0], None));
        assert!(evaluate(&empty_round, &pack.predicates[1], None));
        // 缺回执 standing_constraints 判败
        assert!(!evaluate(&empty_round, &pack.predicates[2], None));
        // 断言缺 evidence 数组判败；evidence 空数组判败
        assert!(!evaluate(&json!({"round": {"assertions": [{"claim": "x"}]}}), &pack.predicates[0], None));
        assert!(!evaluate(&json!({"round": {"assertions": [{"claim": "x", "evidence": []}]}}), &pack.predicates[0], None));
        // 有效锚密度：真实存在路径
        let rooted = json!({"round": {"assertions": [
            {"claim": "a", "evidence": ["Cargo.toml"]},
            {"claim": "b", "evidence": ["src/lib.rs:1"]}
        ]}});
        assert!(evaluate(&rooted, &pack.predicates[1], None), "全有效锚密度 1.0 过");
        let half = json!({"round": {"assertions": [
            {"claim": "a", "evidence": ["Cargo.toml"]},
            {"claim": "b", "evidence": ["no/such/file.md"]}
        ]}});
        assert!(!evaluate(&half, &pack.predicates[1], None), "半有效锚密度 0.5 不过");
        // 回执齐备 standing_constraints 过
        let receipted = json!({"round": {"assertions": [], "conclusions": [], "receipts": [{"tool": "nomenclator"}]}});
        assert!(evaluate(&receipted, &pack.predicates[2], None));
    }

    #[test]
    fn t4_time_deadline_fail_closed() {
        let pack = pack_of(json!([{"id": "P102", "kind": "time_deadline", "route_on_fail": "siding"}]));
        let live = json!({"parking": {"entered_at": "2026-08-30", "ttl_days": 60}});
        let expired = json!({"parking": {"entered_at": "2026-06-01", "ttl_days": 14}});
        let ctx_ok = json!({"reference_time": "2026-09-02"});
        assert!(evaluate(&live, &pack.predicates[0], Some(&ctx_ok)));
        assert!(!evaluate(&expired, &pack.predicates[0], Some(&ctx_ok)), "到期判败");
        // 到期日当日即败
        let boundary = json!({"parking": {"entered_at": "2026-08-19", "ttl_days": 14}});
        assert!(!evaluate(&boundary, &pack.predicates[0], Some(&ctx_ok)));
        // 缺参照时间 fail-closed
        assert!(!evaluate(&live, &pack.predicates[0], None));
        // 缺键或字段非法 fail-closed
        assert!(!evaluate(&json!({"parking": {"ttl_days": 60}}), &pack.predicates[0], Some(&ctx_ok)));
        assert!(!evaluate(&json!({"parking": {"entered_at": "bad", "ttl_days": 60}}), &pack.predicates[0], Some(&ctx_ok)));
        assert!(!evaluate(&json!({"parking": {"entered_at": "2026-08-30", "ttl_days": 0}}), &pack.predicates[0], Some(&ctx_ok)));
        assert!(!evaluate(&json!({"parking": {"entered_at": "2026-08-30", "ttl_days": true}}), &pack.predicates[0], Some(&ctx_ok)));
        assert!(!evaluate(&json!({"parking": "no"}), &pack.predicates[0], Some(&ctx_ok)));
    }

    #[test]
    fn t4_first_fail_route_and_flags() {
        let mut pack = pack_of(json!([
            {"id": "R001", "kind": "schema_required", "route_on_fail": "scrap_track", "fields": ["id", "path", "state"]},
            {"id": "R002", "kind": "state_annotation", "route_on_fail": "siding", "allowed": ["process", "draft"]}
        ]));
        pack.pass_route = "mainline".into();
        let material = json!({"id": "m", "path": "doc/m.md", "state": "parked", "parking": {"entered_at": "2026-08-30", "ttl_days": 60}});
        let entry = route_material(&material, &pack, None);
        assert_eq!(entry["route"], "siding", "首败 R002 走 siding");
        assert_eq!(entry["failed_predicate"], "R002");
        assert_eq!(entry["checks"].as_array().unwrap().len(), 2, "全部谓词过败记录在场");
        assert_eq!(entry["parking"], json!(true), "停泊材料携带 parking 真");
        let plain = route_material(&json!({"id": "m", "path": "doc/m.md", "state": "draft"}), &pack, None);
        assert!(plain.get("parking").is_none(), "普通材料不携带 parking 标记");
        assert!(plain.get("round").is_none());
    }

    #[test]
    fn t4_alarms_and_summary_keys() {
        let mut pack = pack_of(json!([{"id": "R1", "kind": "schema_required", "route_on_fail": "scrap_track", "fields": ["id"]}]));
        pack.siding_surplus_threshold = 5;
        pack.mainline_starvation_threshold = 1;
        let mats = vec![json!({"n": 1}), json!({"n": 2})];
        let (routed, summary) = route_batch(&mats, &pack, None);
        assert_eq!(routed.len(), 2);
        assert_eq!(summary["total"], 2);
        assert_eq!(summary["scrap_track"], 2);
        let alarms = summary["alarms"].as_array().unwrap();
        assert_eq!(alarms.len(), 1, "全弃批产饥饿告警");
        assert_eq!(alarms[0]["kind"], "mainline_starvation");
        // 有 id 材料全过主线零告警
        let good = vec![json!({"id": "a"}), json!({"id": "b"})];
        let (_, good_summary) = route_batch(&good, &pack, None);
        assert_eq!(good_summary["mainline"], 2);
        assert_eq!(good_summary["alarms"].as_array().unwrap().len(), 0);
        // 空批零告警
        let (_, empty_summary) = route_batch(&[], &pack, None);
        assert_eq!(empty_summary["alarms"].as_array().unwrap().len(), 0);
        assert!(empty_summary.get("conflicts").is_none(), "无产出不增键");
        assert!(empty_summary.get("agings").is_none());
    }

    #[test]
    fn t4_registry_conflict_alarm() {
        let mut pack = pack_of(json!([
            {"id": "R101", "kind": "assertion_lists", "route_on_fail": "siding"},
            {"id": "R104", "kind": "registry_conflict"}
        ]));
        pack.mainline_starvation_threshold = 0;
        let material = json!({"round": {
            "assertions": [],
            "conclusions": [{"kind": "establishes_term", "term": "裁行泊"}],
            "receipts": [{"tool": "nomenclator", "flagged_terms": ["裁行泊"]}]
        }});
        let (_, summary) = route_batch(&[material], &pack, None);
        let alarms = summary["alarms"].as_array().unwrap();
        assert_eq!(alarms.len(), 1);
        assert_eq!(alarms[0]["kind"], "registry_conflict");
        assert_eq!(alarms[0]["term"], "裁行泊");
        assert_eq!(summary["conflicts"], 1, "有产出增 conflicts 键");
        // 包未声明该谓词即零告警零键（告警以包声明为门）
        let mut pack2 = pack_of(json!([{"id": "R101", "kind": "assertion_lists", "route_on_fail": "siding"}]));
        pack2.mainline_starvation_threshold = 0;
        let material2 = json!({"round": {
            "assertions": [],
            "conclusions": [{"kind": "establishes_term", "term": "x"}],
            "receipts": [{"tool": "t", "flagged_terms": ["x"]}]
        }});
        let (_, s2) = route_batch(&[material2], &pack2, None);
        assert_eq!(s2["alarms"].as_array().unwrap().len(), 0);
        assert!(s2.get("conflicts").is_none());
    }

    #[test]
    fn t4_parking_aging_alarm() {
        let mut pack = pack_of(json!([
            {"id": "P101", "kind": "schema_required", "route_on_fail": "scrap_track", "fields": ["id", "state"]},
            {"id": "P102", "kind": "time_deadline", "route_on_fail": "siding"},
            {"id": "P103", "kind": "parking_aging", "aging_threshold_days": 45}
        ]));
        pack.mainline_starvation_threshold = 0;
        pack.siding_surplus_threshold = 99;
        let material = json!({"id": "pk-x", "state": "parked", "parking": {"entered_at": "2026-06-01", "ttl_days": 365}});
        let ctx = json!({"reference_time": "2026-09-02"});
        let (routed, summary) = route_batch(&[material], &pack, Some("2026-09-02"));
        assert_eq!(routed[0]["route"], "mainline", "未到期走主线");
        let alarms = summary["alarms"].as_array().unwrap();
        assert_eq!(alarms.len(), 1);
        assert_eq!(alarms[0]["kind"], "parking_aging");
        assert_eq!(alarms[0]["age_days"], 93);
        assert_eq!(summary["agings"], 1);
        let _ = ctx;
    }

    #[test]
    fn t4_gate_hold_hold_and_release() {
        let pack = pack_of(json!([{"id": "P104", "kind": "gate_hold", "route_on_fail": "siding"}]));
        // 未点火（fired_at 缺省与显式 null 两形）判败走 siding
        let unfired = json!({"id": "pk-070", "gate": {"trigger": "MCP 实装结算件落 event/plan/", "declared_at": "2026-09-07"}});
        assert!(!evaluate(&unfired, &pack.predicates[0], None));
        let null_fired = json!({"id": "pk-077", "gate": {"trigger": "t", "declared_at": "2026-09-07", "fired_at": null}});
        assert!(!evaluate(&null_fired, &pack.predicates[0], None), "显式 null 即未点火判败");
        // 点火在场判过
        let fired = json!({"id": "g", "gate": {"trigger": "t", "fired_at": "2026-09-09"}});
        assert!(evaluate(&fired, &pack.predicates[0], None));
        // gate 缺省或非对象判过零影响
        assert!(evaluate(&json!({"id": "m"}), &pack.predicates[0], None));
        assert!(evaluate(&json!({"id": "m", "gate": "not-a-object"}), &pack.predicates[0], None));
    }

    #[test]
    fn t4_surplus_exempt_kinds_parktune() {
        // 豁免语义（对表围堰修订七）：首败谓词 kind 属豁免清单的侧轨件不计入
        // 有余告警积压计数；真积压（time_deadline 首败）仍告警；缺省零豁免。
        let mut pack = pack_of(json!([
            {"id": "P104", "kind": "gate_hold", "route_on_fail": "siding"},
            {"id": "P102", "kind": "time_deadline", "route_on_fail": "siding"}
        ]));
        pack.siding_surplus_threshold = 2;
        pack.mainline_starvation_threshold = 0;
        pack.pass_route = "mainline".into();
        let gated = |id: &str| json!({"id": id, "path": "doc/governance/PARKING-v1.md", "state": "parked",
            "parking": {"entered_at": "2026-09-01", "ttl_days": 60},
            "gate": {"trigger": "MCP 实装结算件落 event/plan/", "declared_at": "2026-09-07"}});
        let expired = |id: &str| json!({"id": id, "path": "doc/governance/PARKING-v1.md", "state": "parked",
            "parking": {"entered_at": "2026-08-01", "ttl_days": 30}});
        // 缺省零豁免：两件 gate 落 siding 计入计数即旧行为
        let (_, s_old) = route_batch(&[gated("pk-070"), gated("pk-077")], &pack, Some("2026-09-08"));
        let surplus_old: Vec<&Value> = s_old["alarms"].as_array().unwrap().iter().filter(|a| a["kind"] == "siding_surplus").collect();
        assert_eq!(surplus_old.len(), 1);
        assert_eq!(surplus_old[0]["count"], 2, "缺省零豁免即全量计数");
        // 声明豁免：同批零有余告警而 summary.siding 全量不隐藏
        pack.siding_surplus_exempt_kinds = vec!["gate_hold".into()];
        let (routed, s_ex) = route_batch(&[gated("pk-070"), gated("pk-077")], &pack, Some("2026-09-08"));
        assert_eq!(s_ex["siding"], 2, "全量侧轨数不隐藏");
        assert!(s_ex["alarms"].as_array().unwrap().iter().all(|a| a["kind"] != "siding_surplus"), "豁免后零有余告警");
        assert!(routed.iter().all(|r| r["failed_predicate"] == "P104"));
        // 真积压：三件到期侧轨仍告警 count 三
        let (_, s_real) = route_batch(&[expired("e1"), expired("e2"), expired("e3")], &pack, Some("2026-09-08"));
        let surplus_real: Vec<&Value> = s_real["alarms"].as_array().unwrap().iter().filter(|a| a["kind"] == "siding_surplus").collect();
        assert_eq!(surplus_real.len(), 1);
        assert_eq!(surplus_real[0]["count"], 3, "真积压仍告警");
        // 点火后到期不豁免：fired_at 在场即 P104 过 P102 首败计入
        let fired_expired = json!({"id": "g9", "path": "doc/governance/PARKING-v1.md", "state": "parked",
            "parking": {"entered_at": "2026-08-01", "ttl_days": 30},
            "gate": {"trigger": "t", "fired_at": "2026-09-06"}});
        let (routed_fe, s_fe) = route_batch(&[fired_expired, expired("e2")], &pack, Some("2026-09-08"));
        assert!(routed_fe.iter().all(|r| r["failed_predicate"] == "P102"));
        let surplus_fe: Vec<&Value> = s_fe["alarms"].as_array().unwrap().iter().filter(|a| a["kind"] == "siding_surplus").collect();
        assert_eq!(surplus_fe.len(), 1);
        assert_eq!(surplus_fe[0]["count"], 2, "点火后到期是真事件即计入");
    }

    #[test]
    fn t4_aging_suspended_for_unfired_gate() {
        let pack = pack_of(json!([{"id": "P103", "kind": "parking_aging", "aging_threshold_days": 45}]));
        let unfired = json!({"id": "pk-x", "parking": {"entered_at": "2026-01-01", "ttl_days": 365},
            "gate": {"trigger": "t", "declared_at": "2026-09-07"}});
        let (_, summary) = route_batch(&[unfired.clone()], &pack, Some("2026-09-08"));
        assert!(summary["alarms"].as_array().unwrap().iter().all(|a| a["kind"] != "parking_aging"), "未点火搁置件停计零老化告警");
        let mut fired = unfired.clone();
        fired["gate"]["fired_at"] = json!("2026-09-06");
        let (_, s2) = route_batch(&[fired], &pack, Some("2026-09-08"));
        assert!(s2["alarms"].as_array().unwrap().iter().any(|a| a["kind"] == "parking_aging"), "点火后复计");
    }

    #[test]
    fn t4_pack_validation_rejects() {
        let dir = std::env::temp_dir().join(format!("route-pack-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("p1")).unwrap();
        // manifest 缺 domain
        fs::write(dir.join("p1").join("manifest.toml"), "name = \"x\"\nversion = \"1\"\n").unwrap();
        fs::write(dir.join("p1").join("routes.toml"), "[defaults]\npass_route = \"mainline\"\n\n[alarms]\nsiding_surplus_threshold = 5\nmainline_starvation_threshold = 1\n").unwrap();
        let err = load_pack(&dir.join("p1")).unwrap_err();
        assert!(err.message().contains("domain must be a table"), "{}", err);
        // 未知 kind
        fs::write(dir.join("p1").join("manifest.toml"), "name = \"x\"\nversion = \"1\"\n\n[domain]\ninclude = [\"**/**\"]\nexclude = []\n").unwrap();
        fs::write(dir.join("p1").join("routes.toml"), "[[predicates]]\nid = \"X\"\nkind = \"zzz\"\n\n[defaults]\npass_route = \"mainline\"\n\n[alarms]\nsiding_surplus_threshold = 5\nmainline_starvation_threshold = 1\n").unwrap();
        let err = load_pack(&dir.join("p1")).unwrap_err();
        assert!(err.message().contains("kind must be one of"), "{}", err);
        assert!(err.message().ends_with("got zzz"), "{}", err);
        // 告警位禁配 route_on_fail
        fs::write(dir.join("p1").join("routes.toml"), "[[predicates]]\nid = \"X\"\nkind = \"registry_conflict\"\nroute_on_fail = \"siding\"\n\n[defaults]\npass_route = \"mainline\"\n\n[alarms]\nsiding_surplus_threshold = 5\nmainline_starvation_threshold = 1\n").unwrap();
        let err = load_pack(&dir.join("p1")).unwrap_err();
        assert!(err.message().contains("route_on_fail is not allowed for registry_conflict"), "{}", err);
        // pass_route 非法（无 predicates 键即空包合法，错误面在 defaults）
        fs::write(dir.join("p1").join("routes.toml"), "[defaults]\npass_route = \"nowhere\"\n\n[alarms]\nsiding_surplus_threshold = 5\nmainline_starvation_threshold = 1\n").unwrap();
        let err = load_pack(&dir.join("p1")).unwrap_err();
        assert!(err.message().contains("pass_route must be one of"), "{}", err);
        // 空包合法（F6）：零谓词全走 pass_route
        fs::write(dir.join("p1").join("routes.toml"), "[defaults]\npass_route = \"siding\"\n\n[alarms]\nsiding_surplus_threshold = 5\nmainline_starvation_threshold = 0\n").unwrap();
        let empty_pack = load_pack(&dir.join("p1")).unwrap();
        assert!(empty_pack.predicates.is_empty());
        let (routed, _) = route_batch(&[json!({"id": "x"})], &empty_pack, None);
        assert_eq!(routed[0]["route"], "siding", "空包全走 pass_route");
        // 阈值负数
        fs::write(dir.join("p1").join("routes.toml"), "[defaults]\npass_route = \"mainline\"\n\n[alarms]\nsiding_surplus_threshold = -1\nmainline_starvation_threshold = 1\n").unwrap();
        let err = load_pack(&dir.join("p1")).unwrap_err();
        assert!(err.message().contains("siding_surplus_threshold must be a non-negative integer"), "{}", err);
        // 豁免清单非法 kind 拒包（typo 不静默失效）
        fs::write(dir.join("p1").join("routes.toml"), "[defaults]\npass_route = \"mainline\"\n\n[alarms]\nsiding_surplus_threshold = 2\nmainline_starvation_threshold = 0\nsiding_surplus_exempt_kinds = [\"gate_hod\"]\n").unwrap();
        let err = load_pack(&dir.join("p1")).unwrap_err();
        assert!(err.message().contains("siding_surplus_exempt_kinds entries must be one of"), "{}", err);
        assert!(err.message().ends_with("got gate_hod"), "{}", err);
        // routes.toml 缺席
        fs::remove_file(dir.join("p1").join("routes.toml")).unwrap();
        let err = load_pack(&dir.join("p1")).unwrap_err();
        assert!(err.message().contains("routes.toml missing"), "{}", err);
        // 包目录缺席
        let err = load_pack(&dir.join("nope")).unwrap_err();
        assert_eq!(err.message(), format!("pack directory missing: {}", dir.join("nope").display()));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn t4_error_envelope_shapes() {
        // 未知 kind 拒包信封与金向量同形（@PACKDIR@ token 对表）
        let dir = std::env::temp_dir().join(format!("route-env-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("pack")).unwrap();
        fs::write(dir.join("pack").join("manifest.toml"), "name = \"badpack\"\nversion = \"0.1.0\"\n\n[domain]\ninclude = [\"**/**\"]\nexclude = []\n").unwrap();
        fs::write(dir.join("pack").join("routes.toml"), "[[predicates]]\nid = \"X001\"\nkind = \"no_such_kind\"\nroute_on_fail = \"siding\"\n\n[defaults]\npass_route = \"mainline\"\n\n[alarms]\nsiding_surplus_threshold = 5\nmainline_starvation_threshold = 1\n").unwrap();
        let result = run_route(&dir.join("pack"), None, &[]);
        let (envelope, code) = result.unwrap_err();
        assert_eq!(code, 2);
        let expected_template = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/attractor/fixtures/route/golden/badpack-kind/expected.json"),
        )
        .unwrap()
        .replace("@PACKDIR@", &dir.join("pack").to_string_lossy());
        // 信封文本加换行即二进制 stdout 形（围堰 print 形）
        assert_eq!(format!("{envelope}\n"), expected_template, "拒包信封须与冻结形逐字节一致");
        let _ = fs::remove_dir_all(&dir);
    }
}

// PathBuf 引用保持（run_route 材料展开用）。
#[allow(dead_code)]
fn _pathbuf_ref(_p: PathBuf) {}

use std::path::PathBuf;
