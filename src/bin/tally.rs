//! 引擎侧执契（Tally）命令行面 —— lease-mergeleg23-parallel 簇F 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/tally 1.0.0（src/tally/cli.py 524 行），只读对表移植，
//! 围堰源码零改动。核心行为链：check 对 tally-check-input 材料做 R1-R9 逐条核对与
//! DES-011 三态映射四值处置、verify 同输入重放逐字节比对、sign 仅裁决通过时外调
//! scribe crosscheck 落据、watch 对签署报告批量重放浮异常视图。R1 由实现本身承载。
//!
//! CLI：`tally <check|verify|assemble|watch|sign> ...`
//! 退出码三值：0 = 全过或一致；1 = 失败或不一致；2 = 用法或环境错误。
//!
//! 落差申报（相对围堰）：
//! 1. [已清偿 gap-tally-parity] assemble 确定性装配已移植：glob 择件（responses 哈希
//!    对表、按 (n_shots, responses_sha256) 择最大、平局取先现）、合同 topic 哈希发现
//!    （cell topic.md 先行、topics-dir 逐个 glob *.md，围堰 glob 顺序未定处按全径
//!    字典序，sha 命中至多一件时同判）、正身 core_hash 透传、基线与日期透传；
//!    argparse 的无歧义前缀缩写形未移植（全名形为准）。
//! 2. [已清偿 gap-tally-parity] Python json 规范形（fingerprint 与 canonical）的浮点
//!    最短表示已对齐 CPython repr 排版：decpt ≤ -4 或 > 16 走科学计数、指数带符号
//!    且至少两位（1e+16 / 1e-05）、整值浮点带 .0、负零保留符号；NaN/Infinity 的
//!    json 文本形 serde 解析面不认，不在对表范围。
//! 3. 异常报文内文不对齐（FileNotFoundError/JSONDecodeError 的 Python 内文形以近似
//!    形出），异常类名与退出码对齐。
//! 4. watch 异常视图 error 字段类名以近似 Python 名出（KeyError/ValueError/
//!    FileNotFoundError/JSONDecodeError），内文不对齐。
//! 5. 「基线判定值不可读」呈报中非字符串判定值按近似 Python str() 出（对象/数组走
//!    紧凑 JSON 形），字符串与 None 对齐。

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::Path;
use std::process::{exit, Command};

/// 围堰版本锚：sih-tools/tally/src/tally/cli.py VERSION。
const VERSION: &str = "1.0.0";
const GATE_VALUES: [&str; 3] = ["stable_clear", "near_threshold", "boundary"];
const CRITERIA_VERSION: &str = "v3";
const BUDGET_PER_GID: usize = 9;
const PER_ACTOR_FIELDS: [&str; 7] = [
    "actor_id",
    "model_id",
    "family",
    "decision",
    "basis_regulation",
    "boundary_flag",
    "reason",
];
const DECISION_VALUES: [&str; 2] = ["comply", "violate"];
const RULES_VERSION_R8: &str = "des-011-r2";

// ---------- Python 异常近似 ----------

struct ToolFailure {
    kind: &'static str,
    msg: String,
}
impl ToolFailure {
    fn value(msg: impl Into<String>) -> Self {
        ToolFailure { kind: "ValueError", msg: msg.into() }
    }
    fn not_found(path: &str) -> Self {
        ToolFailure {
            kind: "FileNotFoundError",
            msg: format!("[Errno 2] No such file or directory: '{path}'"),
        }
    }
    fn json_decode(msg: impl Into<String>) -> Self {
        ToolFailure { kind: "JSONDecodeError", msg: msg.into() }
    }
    fn key(name: &str) -> Self {
        ToolFailure { kind: "KeyError", msg: format!("'{name}'") }
    }
}

// ---------- Python json 规范形（见头注落差二） ----------

fn py_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn py_scalar(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        Value::Number(n) => {
            if n.is_f64() {
                py_float_repr(n.as_f64().unwrap_or(0.0))
            } else {
                n.to_string()
            }
        }
        Value::String(s) => py_escape(s),
        _ => String::new(),
    }
}

fn write_line(v: &Value, sorted: bool, item_sep: &str, kv_sep: &str, out: &mut String) {
    match v {
        Value::Object(m) => {
            out.push('{');
            let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
            if sorted {
                kvs.sort_by(|a, b| a.0.cmp(b.0));
            }
            for (i, (k, val)) in kvs.iter().enumerate() {
                if i > 0 {
                    out.push_str(item_sep);
                }
                out.push_str(&py_escape(k));
                out.push_str(kv_sep);
                write_line(val, sorted, item_sep, kv_sep, out);
            }
            out.push('}');
        }
        Value::Array(a) => {
            out.push('[');
            for (i, val) in a.iter().enumerate() {
                if i > 0 {
                    out.push_str(item_sep);
                }
                write_line(val, sorted, item_sep, kv_sep, out);
            }
            out.push(']');
        }
        other => out.push_str(&py_scalar(other)),
    }
}

fn py_line(v: &Value, sorted: bool) -> String {
    // json.dumps 默认分隔符 (', ', ': ')
    let mut s = String::new();
    write_line(v, sorted, ", ", ": ", &mut s);
    s
}

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

fn write_pretty(v: &Value, depth: usize, out: &mut String) {
    match v {
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push_str("{\n");
            let n = m.len();
            for (i, (k, val)) in m.iter().enumerate() {
                out.push_str(&"  ".repeat(depth + 1));
                out.push_str(&py_escape(k));
                out.push_str(": ");
                write_pretty(val, depth + 1, out);
                if i + 1 < n {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&"  ".repeat(depth));
            out.push('}');
        }
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push_str("[\n");
            for (i, val) in a.iter().enumerate() {
                out.push_str(&"  ".repeat(depth + 1));
                write_pretty(val, depth + 1, out);
                if i + 1 < a.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&"  ".repeat(depth));
            out.push(']');
        }
        other => out.push_str(&py_scalar(other)),
    }
}

/// json.dumps(v, ensure_ascii=False, sort_keys=True, indent=2)
fn py_canonical(v: &Value) -> String {
    let mut s = String::new();
    write_pretty(&sort_value(v.clone()), 0, &mut s);
    s
}

/// Python str() 近似（见头注落差五）。
fn py_display(v: &Value) -> String {
    match v {
        Value::Null => "None".to_string(),
        Value::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        Value::String(s) => s.clone(),
        other => py_line(other, true),
    }
}

// ---------- Python 浮点 repr 与 pathlib 展示形（gap-tally-parity 清偿，见头注落差一、二） ----------

/// Python repr(float)（json.dumps 浮点规范形）：最短往返数字串按 CPython repr 排版——
/// decpt ≤ -4 或 > 16 走科学计数、指数带符号且至少两位（1e+16 / 1e-05）、整值浮点带 .0。
fn py_float_repr(x: f64) -> String {
    if x.is_nan() {
        return "NaN".to_string();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let neg = x.is_sign_negative();
    // Rust {:e} 即最短往返 d[.ddd]e±exp 形，取其数字与十进指数
    let s = format!("{:e}", x.abs());
    let (mant, exp_s) = s.split_once('e').expect("LowerExp 形必带 e");
    let exp: i32 = exp_s.parse().expect("十进指数");
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let decpt = exp + 1; // 值 = 0.digits × 10^decpt
    let ndigits = digits.len();
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    if decpt <= -4 || decpt > 16 {
        out.push_str(&digits[..1]);
        if ndigits > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        let e = decpt - 1;
        if e < 0 {
            out.push_str(&format!("e-{:02}", -e));
        } else {
            out.push_str(&format!("e+{:02}", e));
        }
    } else if decpt <= 0 {
        out.push_str("0.");
        for _ in 0..-decpt {
            out.push('0');
        }
        out.push_str(&digits);
    } else if decpt as usize >= ndigits {
        out.push_str(&digits);
        for _ in 0..(decpt as usize - ndigits) {
            out.push('0');
        }
        out.push_str(".0");
    } else {
        out.push_str(&digits[..decpt as usize]);
        out.push('.');
        out.push_str(&digits[decpt as usize..]);
    }
    out
}

/// pathlib PurePath 解析形：（是否绝对、组件列）。空组件与点组件剥除，`..` 保留。
fn py_norm(p: &str) -> (bool, Vec<&str>) {
    let abs = p.starts_with('/');
    let comps: Vec<&str> = p
        .split('/')
        .filter(|c| !c.is_empty() && *c != ".")
        .collect();
    (abs, comps)
}

fn py_render(abs: bool, comps: &[&str]) -> String {
    let mut s = comps.join("/");
    if abs {
        s.insert(0, '/');
    }
    if s.is_empty() {
        s.push('.');
    }
    s
}

/// pathlib str() 形（Path("") 归 "."）。
fn py_path_str(p: &str) -> String {
    let (abs, comps) = py_norm(p);
    py_render(abs, &comps)
}

/// pathlib parent 语义（Path("a").parent == "."，Path("/").parent == "/"）。
fn py_parent_str(p: &str) -> String {
    let (abs, comps) = py_norm(p);
    if comps.is_empty() {
        return if abs { "/".to_string() } else { ".".to_string() };
    }
    py_render(abs, &comps[..comps.len() - 1])
}

/// pathlib join 后的 str() 形（rel 绝对即取 rel 本身）。
fn py_join_str(base: &str, rel: &str) -> String {
    if rel.starts_with('/') {
        return py_path_str(rel);
    }
    let (abs, base_comps) = py_norm(base);
    let (_, rel_comps) = py_norm(rel);
    let mut comps = base_comps;
    comps.extend(rel_comps);
    py_render(abs, &comps)
}

// ---------- 基础工具 ----------

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn sha256_file(path: &Path) -> Result<String, ToolFailure> {
    let bytes = fs::read(path).map_err(|_| ToolFailure::not_found(&path.to_string_lossy()))?;
    Ok(sha256_hex(&bytes))
}

fn read_text(path: &Path) -> Result<String, ToolFailure> {
    fs::read_to_string(path).map_err(|_| ToolFailure::not_found(&path.to_string_lossy()))
}

fn parse_json(text: &str) -> Result<Value, ToolFailure> {
    serde_json::from_str(text).map_err(|e| ToolFailure::json_decode(e.to_string()))
}

fn py_truthy(v: Option<&Value>) -> bool {
    match v {
        None | Some(Value::Null) | Some(Value::Bool(false)) => false,
        Some(Value::Bool(true)) => true,
        Some(Value::Number(n)) => n.as_f64().map(|f| f != 0.0).unwrap_or(true),
        Some(Value::String(s)) => !s.is_empty(),
        Some(Value::Array(a)) => !a.is_empty(),
        Some(Value::Object(m)) => !m.is_empty(),
    }
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: tally <check|verify|assemble|watch|sign> <子命令参数>");
    exit(2);
}

/// 单值旗标严格解析（未知旗标即用法错，与 argparse 退出码对齐）。
fn parse_flags(args: &[String], allowed: &[&str], required: &[&str]) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        let (name, inline) = match a.split_once('=') {
            Some((n, v)) if n.starts_with("--") => (n.to_string(), Some(v.to_string())),
            _ => (a.clone(), None),
        };
        if !name.starts_with("--") {
            usage_fail(&format!("意外位置参数: {a}"));
        }
        if !allowed.contains(&name.as_str()) {
            usage_fail(&format!("未知旗标: {name}"));
        }
        let val = match inline {
            Some(v) => v,
            None => {
                i += 1;
                if i >= args.len() {
                    usage_fail(&format!("{name} 缺值"));
                }
                args[i].clone()
            }
        };
        out.insert(name, val);
        i += 1;
    }
    for r in required {
        if !out.contains_key(*r) {
            usage_fail(&format!("缺 {r}"));
        }
    }
    out
}

// ---------- trail 装载与派生 ----------

/// trail → dc_list 与 flywheel_run 原始行。格式错行跳过计数（围堰同形跳过）。
fn load_dc_list(trail_path: &Path) -> Result<(Vec<Value>, Vec<Value>), ToolFailure> {
    let text = read_text(trail_path)?;
    let mut dc_list = vec![];
    let mut runs = vec![];
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let r: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if r.get("trail_type").and_then(|v| v.as_str()) == Some("flywheel_run") {
            runs.push(r.clone());
            if let Some(dc) = r.get("decision_convergence") {
                if dc.is_object() && dc.get("per_actor").is_some() {
                    dc_list.push(dc.clone());
                }
            }
        }
    }
    Ok((dc_list, runs))
}

/// json.dumps(dc_list, ensure_ascii=False, sort_keys=True) 的 sha256 前 16 位。
fn fingerprint(dc_list: &[Value]) -> String {
    let payload = py_line(&Value::Array(dc_list.to_vec()), true);
    sha256_hex(payload.as_bytes())[..16].to_string()
}

/// Counter.most_common 语义：计数最大者，平局取先现者。
fn modal_direction(dc_list: &[Value]) -> Option<String> {
    let mut order: Vec<String> = vec![];
    let mut counts: HashMap<String, usize> = HashMap::new();
    for dc in dc_list {
        if let Some(pas) = dc.get("per_actor").and_then(|v| v.as_array()) {
            for pa in pas {
                if let Some(d) = pa.get("decision").and_then(|v| v.as_str()) {
                    if !counts.contains_key(d) {
                        order.push(d.to_string());
                    }
                    *counts.entry(d.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    if order.is_empty() {
        return None;
    }
    let mut best = &order[0];
    let mut best_n = counts[best];
    for d in &order {
        let n = counts[d];
        if n > best_n {
            best = d;
            best_n = n;
        }
    }
    Some(best.clone())
}

// ---------- check：R1-R9 核对与四值处置 ----------

fn check_material(material_path: &str) -> Result<Value, ToolFailure> {
    let text = read_text(Path::new(material_path))?;
    let material = parse_json(&text)?;
    if material.get("kind").and_then(|v| v.as_str()) != Some("tally-check-input") {
        let got = material.get("kind").cloned().unwrap_or(Value::Null);
        return Err(ToolFailure::value(format!("材料 kind 不符：{}", py_display(&got))));
    }

    let mut passed: Vec<Value> = vec![];
    let mut failed: Vec<Value> = vec![];
    let mut alarms: Vec<String> = vec![];

    let gid = material.get("gid").cloned().unwrap_or(Value::Null);
    let verdict = material.get("gate_verdict").cloned().unwrap_or(Value::Null);

    // R1 执行者归属：版本署名于此；材料全程只读
    passed.push(Value::String(format!("R1: 核对程序版本署名 tally/{VERSION}，材料只读消费")));

    // R2 来源可追溯
    for field in ["gid", "topic_sha256", "dc_fingerprint"] {
        if material.get(field).is_none() {
            failed.push(json!({"rule": "R2", "where": format!("材料缺字段 {field}")}));
        }
    }
    let topic_path = match material.get("topic_path").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => Some(s.to_string()),
        _ => None,
    };
    let topic_is_file = topic_path
        .as_ref()
        .map(|p| Path::new(p).is_file())
        .unwrap_or(false);
    if topic_path.is_none() || !topic_is_file {
        failed.push(json!({"rule": "R2", "where": "topic.md 不存在"}));
    } else {
        let want = material.get("topic_sha256").and_then(|v| v.as_str());
        let got = sha256_file(Path::new(topic_path.as_ref().unwrap()))?;
        if want != Some(got.as_str()) {
            failed.push(json!({"rule": "R2", "where": "topic_sha256 与 topic.md 复算不符"}));
        } else {
            passed.push(json!("R2: topic_sha256 复算一致"));
        }
    }
    let trail_path = match material.get("trail_path").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => Some(s.to_string()),
        _ => None,
    };
    let trail_is_file = trail_path
        .as_ref()
        .map(|p| Path::new(p).is_file())
        .unwrap_or(false);
    if trail_path.is_none() || !trail_is_file {
        failed.push(json!({"rule": "R2", "where": "trail 文件不存在"}));
    } else {
        passed.push(json!("R2: trail 文件在场"));
    }
    for (key, path_key) in [("contract_sha256", "contract_path"), ("responses_sha256", "responses_path")] {
        if py_truthy(material.get(path_key)) {
            let pf = material.get(path_key).and_then(|v| v.as_str()).unwrap_or("");
            let p = Path::new(pf);
            if !p.is_file() {
                failed.push(json!({"rule": "R2", "where": format!("{path_key} 不存在")}));
            } else {
                let got = sha256_file(p)?;
                let want = material.get(key).and_then(|v| v.as_str());
                if want != Some(got.as_str()) {
                    failed.push(json!({"rule": "R2", "where": format!("{key} 复算不符")}));
                } else {
                    passed.push(json!(format!("R2: {key} 复算一致")));
                }
            }
        }
    }

    // R3 结果可机械校验
    let verdict_str = verdict.as_str();
    if !GATE_VALUES.iter().any(|g| Some(*g) == verdict_str) {
        failed.push(json!({"rule": "R3", "where": format!("闸裁决不在三值集合：{}", py_display(&verdict))}));
    } else {
        passed.push(json!(format!("R3: 闸裁决 {} 在三值集合", verdict_str.unwrap_or(""))));
    }
    let criteria = material
        .get("criteria_version")
        .and_then(|v| v.as_str())
        .unwrap_or(CRITERIA_VERSION);
    if criteria != CRITERIA_VERSION {
        failed.push(json!({"rule": "R3", "where": format!("criteria_version 非 {CRITERIA_VERSION}")}));
    } else {
        passed.push(json!(format!("R3: criteria_version {CRITERIA_VERSION}")));
    }
    let mut dc_list: Vec<Value> = vec![];
    let mut runs: Vec<Value> = vec![];
    if trail_path.is_some() && trail_is_file {
        let (d, r) = load_dc_list(Path::new(trail_path.as_ref().unwrap()))?;
        dc_list = d;
        runs = r;
        if material.get("dc_fingerprint").and_then(|v| v.as_str()) != Some(fingerprint(&dc_list).as_str()) {
            failed.push(json!({"rule": "R3", "where": "dc_fingerprint 复算不符"}));
        } else {
            passed.push(json!("R3: dc_fingerprint 复算一致"));
        }
        let mut bad_actor: BTreeSet<usize> = BTreeSet::new();
        for (i, dc) in dc_list.iter().enumerate() {
            if let Some(pas) = dc.get("per_actor").and_then(|v| v.as_array()) {
                for pa in pas {
                    let keys_ok = pa.as_object().map(|o| {
                        o.len() == PER_ACTOR_FIELDS.len()
                            && PER_ACTOR_FIELDS.iter().all(|f| o.contains_key(*f))
                    }).unwrap_or(false);
                    let dec_ok = pa
                        .get("decision")
                        .and_then(|v| v.as_str())
                        .map(|d| DECISION_VALUES.contains(&d))
                        .unwrap_or(false);
                    if !keys_ok || !dec_ok {
                        bad_actor.insert(i);
                    }
                }
            }
        }
        if !bad_actor.is_empty() {
            let head: Vec<String> = bad_actor.iter().take(3).map(|i| i.to_string()).collect();
            failed.push(json!({"rule": "R3", "where": format!("per_actor 字段或 decision 枚举不符：行 [{}]", head.join(", "))}));
        } else {
            passed.push(json!("R3: per_actor 七字段与方向枚举全符"));
        }
    }

    // R5 席位有效性：当日基线三态联动
    let r5_values = |raw: Option<&Value>| -> &'static str {
        match raw {
            Some(Value::String(s)) => match s.as_str() {
                "可用" => "ok",
                "漂移告警" => "suspend",
                "基线异常" => "abnormal",
                _ => "",
            },
            _ => "",
        }
    };
    let mut r5_state = "missing";
    let bp = material.get("seat_baseline_path").and_then(|v| v.as_str());
    if bp.map(|s| s.is_empty()).unwrap_or(true) {
        failed.push(json!({"rule": "R5", "where": "席位当日基线缺席"}));
    } else {
        let bf = Path::new(bp.unwrap());
        if !bf.is_file() {
            failed.push(json!({"rule": "R5", "where": "席位当日基线文件不存在"}));
        } else {
            let base = parse_json(&read_text(bf)?)?;
            // raw = base.get("verdict") or base.get("判定")
            let raw = match base.get("verdict") {
                Some(v) if py_truthy(Some(v)) => Some(v.clone()),
                _ => base.get("判定").cloned(),
            };
            let m_core = material.get("core_hash").cloned();
            let b_core = base.get("core_hash").cloned();
            let m_hash = material.get("identity_hash").cloned();
            let b_hash = base.get("identity_hash").cloned();
            let both_truthy = |a: &Option<Value>, b: &Option<Value>| {
                py_truthy(a.as_ref()) && py_truthy(b.as_ref())
            };
            if both_truthy(&m_core, &b_core) {
                if m_core == b_core {
                    r5_state = r5_values(raw.as_ref());
                    match r5_state {
                        "ok" => passed.push(json!("R5: 席位当日基线判定可用且身份核哈希一致")),
                        "suspend" | "abnormal" => passed.push(json!(format!(
                            "R5: 身份核哈希一致、基线判定 {}（处置走优先级映射）", py_display(raw.as_ref().unwrap_or(&Value::Null))
                        ))),
                        _ => failed.push(json!({"rule": "R5", "where": format!("基线判定值不可读：{}", py_display(raw.as_ref().unwrap_or(&Value::Null)))})),
                    }
                } else {
                    r5_state = "suspend";
                    passed.push(json!("R5: 席位身份核哈希与基线不一致（处置走优先级映射）"));
                }
            } else if both_truthy(&m_hash, &b_hash) {
                if m_hash == b_hash {
                    r5_state = r5_values(raw.as_ref());
                    match r5_state {
                        "ok" => passed.push(json!("R5: 席位当日基线判定可用且身份哈希一致")),
                        "suspend" | "abnormal" => passed.push(json!(format!(
                            "R5: 身份哈希一致、基线判定 {}（处置走优先级映射）", py_display(raw.as_ref().unwrap_or(&Value::Null))
                        ))),
                        _ => failed.push(json!({"rule": "R5", "where": format!("基线判定值不可读：{}", py_display(raw.as_ref().unwrap_or(&Value::Null)))})),
                    }
                } else {
                    r5_state = "suspend";
                    passed.push(json!("R5: 席位身份哈希与基线不一致（处置走优先级映射）"));
                }
            } else {
                r5_state = r5_values(raw.as_ref());
                match r5_state {
                    "ok" => passed.push(json!("R5: 席位当日基线判定可用")),
                    "suspend" | "abnormal" => passed.push(json!(format!(
                        "R5: 席位当日基线判定 {}（处置走优先级映射）", py_display(raw.as_ref().unwrap_or(&Value::Null))
                    ))),
                    _ => failed.push(json!({"rule": "R5", "where": format!("基线判定值不可读：{}", py_display(raw.as_ref().unwrap_or(&Value::Null)))})),
                }
            }
        }
    }

    // R6 预算上限
    let gid_runs = runs
        .iter()
        .filter(|r| r.get("guidance_id").map(|v| v == &gid).unwrap_or(false))
        .count();
    if gid_runs > BUDGET_PER_GID {
        failed.push(json!({"rule": "R6", "where": format!("同 gid 累计 {gid_runs} 发超预算 {BUDGET_PER_GID}")}));
    } else {
        passed.push(json!(format!("R6: 同 gid 累计 {gid_runs} 发未超预算")));
    }

    // R7 谱系完整
    if runs
        .iter()
        .any(|r| !r.get("guidance_id").map(|v| v == &gid).unwrap_or(false))
    {
        failed.push(json!({"rule": "R7", "where": "trail 内存在异 gid 判定行"}));
    } else {
        passed.push(json!("R7: trail 判定流同 gid 一致"));
    }
    let rewrite_of = material.get("rewrite_of").cloned();
    if !rewrite_of.as_ref().map(|v| v.is_null()).unwrap_or(true) {
        let rewrite_str = py_display(rewrite_of.as_ref().unwrap());
        let mut authored = String::new();
        if topic_path.is_some() && topic_is_file {
            let head_text = read_text(Path::new(topic_path.as_ref().unwrap()))?;
            let parts: Vec<&str> = head_text.splitn(3, "---").collect();
            if parts.len() >= 2 {
                authored = parts[1].to_string();
            }
        }
        if authored.contains(&rewrite_str) {
            passed.push(json!("R7: 改写谱系披露在场"));
        } else {
            failed.push(json!({"rule": "R7", "where": "改写重测缺前置 gid 谱系披露"}));
        }
        let chain_len = material
            .get("rewrite_chain")
            .and_then(|v| v.as_array())
            .map(|a| a.len())
            .unwrap_or(1);
        if chain_len > 3 {
            alarms.push("R7: 同命题改写超三次，钓邻域风险告警".to_string());
        }
    }

    // R8 推导档机械复核
    let mut r8_state = "ok";
    let rules_version = material
        .get("rules_version")
        .and_then(|v| v.as_str())
        .unwrap_or("des-011-r1");
    if rules_version == RULES_VERSION_R8 {
        let dp = material.get("derivation_path").and_then(|v| v.as_str());
        match dp {
            None => {
                r8_state = "suspend";
                passed.push(json!("R8: 推导档指针缺席（处置走优先级映射）"));
            }
            Some(p) if !Path::new(p).is_file() => {
                r8_state = "suspend";
                passed.push(json!(format!("R8: 推导档所指文件不存在 {p}（处置走优先级映射）")));
            }
            Some(_) => {
                passed.push(json!("R8: 推导档指针在场且所指文件实存"));
            }
        }
    }

    // R9 证据登记闸
    if let Some(evidence) = material.get("evidence") {
        if !evidence.is_null() {
            match evidence.as_array() {
                None => failed.push(json!({"rule": "R9", "where": "evidence 登记非数组"})),
                Some(items) => {
                    let mut missing: Vec<String> = vec![];
                    for (idx, item) in items.iter().enumerate() {
                        if !item.is_object() {
                            missing.push(format!("idx{idx}(malformed)"));
                        } else if item.get("required") == Some(&Value::Bool(true))
                            && item.get("status").and_then(|v| v.as_str()) != Some("provided")
                        {
                            let id = match item.get("id") {
                                Some(Value::String(s)) if !s.is_empty() => s.clone(),
                                other if !py_truthy(other) => format!("idx{idx}"),
                                Some(v) => py_display(v),
                                None => format!("idx{idx}"),
                            };
                            let status = match item.get("status") {
                                Some(Value::String(s)) if !s.is_empty() => s.clone(),
                                other if !py_truthy(other) => "status 缺席".to_string(),
                                Some(v) => py_display(v),
                                None => "status 缺席".to_string(),
                            };
                            missing.push(format!("{id}({status})"));
                        }
                    }
                    if !missing.is_empty() {
                        failed.push(json!({"rule": "R9", "where": format!("缺证：{}", missing.join("、"))}));
                    } else {
                        passed.push(json!(format!("R9: 证据登记 {} 项无缺证", items.len())));
                    }
                }
            }
        }
    }

    // 三态映射落处置（DES-011 优先级）
    let direction = modal_direction(&dc_list);
    let disposition = if r5_state == "suspend" || r5_state == "abnormal" || r8_state == "suspend" {
        "挂起"
    } else if !failed.is_empty() {
        "材料退回"
    } else if verdict_str == Some("boundary") {
        "打回重作"
    } else if verdict_str == Some("near_threshold") {
        "挂起"
    } else {
        "裁决通过"
    };

    let has_failed = !failed.is_empty();
    let mut rep = Map::new();
    rep.insert("tool".into(), json!("tally"));
    rep.insert("version".into(), json!(VERSION));
    rep.insert("material".into(), json!(material_path));
    rep.insert("gid".into(), gid);
    rep.insert("gate_verdict".into(), verdict);
    rep.insert(
        "direction".into(),
        direction.map(Value::String).unwrap_or(Value::Null),
    );
    rep.insert("disposition".into(), json!(disposition));
    rep.insert("rules_version".into(), json!(rules_version));
    rep.insert("passed".into(), Value::Array(passed));
    rep.insert("failed".into(), Value::Array(failed));
    rep.insert(
        "alarms".into(),
        Value::Array(alarms.into_iter().map(Value::String).collect()),
    );
    rep.insert("verdict".into(), json!(if has_failed { "fail" } else { "pass" }));
    Ok(Value::Object(rep))
}

// ---------- assemble：确定性装配（gap-tally-parity 移植，围堰 assemble_material） ----------

/// 围堰 _resolve：缺席/空值原样回（None/空串），绝对保持，相对按仓库根拼；
/// 非字符串真值围堰走 Path(v) 崩溃，此处近似串化（病态输入，见头注落差）。
fn py_resolve(repo_root: &str, v: Option<&Value>) -> Option<String> {
    match v {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => {
            if s.is_empty() {
                Some(String::new())
            } else {
                Some(py_join_str(repo_root, s))
            }
        }
        Some(other) => Some(py_display(other)),
    }
}

/// 计分材料的 responses 哈希对表（围堰 _load_sc 的 _m 位）：响应文件在场且
/// responses_sha256 与当前文件哈希一致才可择。
fn score_matches_responses(repo_root: &str, sc: &Value) -> Result<bool, ToolFailure> {
    let rp = match py_resolve(repo_root, sc.get("responses_path")) {
        Some(s) if !s.is_empty() => s,
        _ => return Ok(false),
    };
    if !Path::new(&rp).is_file() {
        return Ok(false);
    }
    let want = sc.get("responses_sha256").and_then(|v| v.as_str());
    let got = sha256_file(Path::new(&rp))?;
    Ok(want == Some(got.as_str()))
}

fn assemble_material(
    gid: &str,
    des_root: &str,
    topics_dirs: &[String],
    baseline: Option<&str>,
    date: Option<&str>,
    out_path: &str,
    identity_report: Option<&str>,
) -> Result<Value, ToolFailure> {
    let repo_root = py_parent_str(&py_parent_str(des_root));
    let cell_str = py_join_str(des_root, gid);
    if !Path::new(&cell_str).is_dir() {
        return Err(ToolFailure::value(format!("命题区目录不存在：{cell_str}")));
    }
    // glob *-score-material.json（隐藏件排除，全径字典序）
    let mut scores: Vec<String> = vec![];
    let rd = fs::read_dir(Path::new(&cell_str)).map_err(|_| ToolFailure::not_found(&cell_str))?;
    for ent in rd.flatten() {
        let name = ent.file_name().to_string_lossy().into_owned();
        if !name.starts_with('.') && name.ends_with("-score-material.json") {
            scores.push(py_join_str(&cell_str, &name));
        }
    }
    scores.sort();
    if scores.is_empty() {
        return Err(ToolFailure::value(format!(
            "无计分材料：{cell_str}/*-score-material.json"
        )));
    }
    let mut matched: Vec<Value> = vec![];
    for sp in &scores {
        let sc = parse_json(&read_text(Path::new(sp))?)?;
        if !sc.is_object() {
            return Err(ToolFailure::value(format!("计分材料非对象：{sp}")));
        }
        if score_matches_responses(&repo_root, &sc)? {
            matched.push(sc);
        }
    }
    if matched.is_empty() {
        return Err(ToolFailure::value(
            "无计分材料与当前响应文件哈希一致（响应已变而未重计分？）".to_string(),
        ));
    }
    // max by (n_shots, responses_sha256)：严格大于才替换即平局取先现（Python max 语义）
    let n_shots_key = |sc: &Value| -> f64 {
        match sc.get("n_shots") {
            Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
            _ => 0.0,
        }
    };
    let sha_key = |sc: &Value| -> String {
        sc.get("responses_sha256")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    let mut best_idx = 0usize;
    for i in 1..matched.len() {
        let best = (n_shots_key(&matched[best_idx]), sha_key(&matched[best_idx]));
        let cand = (n_shots_key(&matched[i]), sha_key(&matched[i]));
        if cand > best {
            best_idx = i;
        }
    }
    let score = matched[best_idx].clone();
    let responses_resolved = py_resolve(&repo_root, score.get("responses_path"));
    let trail_resolved = py_resolve(&repo_root, score.get("trail_path"));
    let contract_resolved = py_resolve(&repo_root, score.get("contract_path"));
    // Python：Path(score["contract_path"])，键缺席即 KeyError('contract_path')
    let contract_str = match &contract_resolved {
        Some(s) if !s.is_empty() => s.clone(),
        Some(_) => ".".to_string(), // Python Path("") == PosixPath(".")
        None => return Err(ToolFailure::key("contract_path")),
    };
    let contract = parse_json(&read_text(Path::new(&contract_str))?)?;
    let topic_sha = contract
        .get("proposition")
        .and_then(|p| p.as_object())
        .and_then(|o| o.get("topic_sha256"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    // trail：resolved 真值即用，否则 cell 内默认名
    let trail_str = match &trail_resolved {
        Some(s) if !s.is_empty() => s.clone(),
        _ => py_join_str(&cell_str, "flywheel-trail.jsonl"),
    };
    // topic 发现：cell topic.md 先行，topics_dirs 逐个 glob *.md；围堰 glob 顺序未定，
    // 此处按全径字典序（sha 命中至多一件时同判），命中即取
    let mut cands: Vec<String> = vec![py_join_str(&cell_str, "topic.md")];
    for td in topics_dirs {
        let dir_str = py_path_str(td);
        let mut hits: Vec<String> = vec![];
        if let Ok(rd) = fs::read_dir(Path::new(&dir_str)) {
            for ent in rd.flatten() {
                let name = ent.file_name().to_string_lossy().into_owned();
                if !name.starts_with('.') && name.ends_with(".md") {
                    hits.push(py_join_str(&dir_str, &name));
                }
            }
        }
        hits.sort();
        cands.extend(hits);
    }
    let mut topic_path: Option<String> = None;
    for cand in &cands {
        let p = Path::new(cand);
        if p.is_file() && sha256_file(p)? == topic_sha {
            topic_path = Some(cand.clone());
            break;
        }
    }
    let topic_path = match topic_path {
        Some(t) => t,
        None => {
            let head: String = topic_sha.chars().take(12).collect();
            return Err(ToolFailure::value(format!(
                "topic 按哈希无匹配：{head}（topics_dirs 覆盖不足）"
            )));
        }
    };
    let (dc_list, _runs) = load_dc_list(Path::new(&trail_str))?;
    let mut material = Map::new();
    material.insert("kind".into(), json!("tally-check-input"));
    material.insert("gid".into(), json!(gid));
    material.insert("topic_path".into(), json!(topic_path));
    material.insert("topic_sha256".into(), json!(topic_sha));
    material.insert("trail_path".into(), json!(trail_str));
    material.insert("dc_fingerprint".into(), json!(fingerprint(&dc_list)));
    material.insert(
        "gate_verdict".into(),
        score.get("gate_verdict").cloned().unwrap_or(Value::Null),
    );
    material.insert("criteria_version".into(), json!(CRITERIA_VERSION));
    material.insert("contract_path".into(), json!(contract_str));
    material.insert(
        "contract_sha256".into(),
        score.get("contract_sha256").cloned().unwrap_or(Value::Null),
    );
    material.insert(
        "responses_path".into(),
        responses_resolved.map(Value::String).unwrap_or(Value::Null),
    );
    material.insert(
        "responses_sha256".into(),
        score.get("responses_sha256").cloned().unwrap_or(Value::Null),
    );
    material.insert(
        "n_shots".into(),
        score.get("n_shots").cloned().unwrap_or(Value::Null),
    );
    material.insert(
        "voids".into(),
        match score.get("voids") {
            Some(v) => v.clone(), // 键在即原样（含 null），缺席才缺省空数组（围堰 get(v, []) 语义）
            None => json!([]),
        },
    );
    material.insert("rules_version".into(), json!("des-011-r1"));
    if py_truthy(score.get("identity_hash")) {
        material.insert(
            "identity_hash".into(),
            score.get("identity_hash").cloned().unwrap_or(Value::Null),
        );
    }
    // 正身透传承 pendsweep-solo：读 identity.core_hash 载入材料，缺席空缺不判败
    if let Some(irp) = identity_report {
        let irp_str = py_path_str(irp);
        let p = Path::new(&irp_str);
        if !p.is_file() {
            return Err(ToolFailure::value(format!("identity 报告不存在：{irp_str}")));
        }
        let report = parse_json(&read_text(p)?)?;
        let core = report
            .get("identity")
            .filter(|v| py_truthy(Some(v)))
            .and_then(|v| v.as_object())
            .and_then(|o| o.get("core_hash"))
            .cloned();
        if let Some(core) = core {
            if py_truthy(Some(&core)) {
                material.insert("core_hash".into(), core);
            }
        }
    }
    if let Some(bp) = baseline {
        material.insert("seat_baseline_path".into(), json!(py_path_str(bp)));
    }
    if let Some(d) = date {
        material.insert("date".into(), json!(d));
    }
    // 落盘：json.dumps(material, ensure_ascii=False, sort_keys=True, indent=2) + "\n"
    let out_str = py_path_str(out_path);
    let op = Path::new(&out_str);
    if let Some(parent) = op.parent() {
        let ps = parent.to_string_lossy();
        if !ps.is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| ToolFailure { kind: "OSError", msg: e.to_string() })?;
        }
    }
    fs::write(op, format!("{}\n", py_canonical(&Value::Object(material.clone()))))
        .map_err(|e| ToolFailure { kind: "OSError", msg: e.to_string() })?;
    Ok(Value::Object(material))
}

// ---------- verify / watch / sign ----------

fn verify_material(material_path: &str, report_path: &str) -> Result<i32, ToolFailure> {
    let fresh = check_material(material_path)?;
    let text = read_text(Path::new(report_path))?;
    let recorded = parse_json(&text)?;
    if py_canonical(&fresh) == py_canonical(&recorded) {
        println!(
            "{}",
            py_line(
                &json!({"verify": "identical", "gid": fresh["gid"], "disposition": fresh["disposition"]}),
                true
            )
        );
        Ok(0)
    } else {
        eprintln!(
            "{}",
            py_line(
                &json!({"verify": "divergent", "gid": recorded.get("gid").cloned().unwrap_or(Value::Null), "note": "复算与既报不符，整包作废"}),
                false
            )
        );
        Ok(1)
    }
}

fn sign_material(
    material_path: &str,
    out_dir: &str,
    trail: &str,
    scribe_binary: &str,
    session: &str,
    locks: &str,
) -> Result<i32, ToolFailure> {
    let report = check_material(material_path)?;
    let disposition = report["disposition"].as_str().unwrap_or("");
    let gid_disp = py_display(&report["gid"]);
    if disposition != "裁决通过" {
        println!(
            "{}",
            py_line(
                &json!({
                    "sign": "refused",
                    "reason": format!("处置为 {disposition}，仅裁决通过可落据"),
                    "gid": report["gid"],
                }),
                true
            )
        );
        return Ok(1);
    }
    fs::create_dir_all(out_dir).map_err(|e| ToolFailure {
        kind: "OSError",
        msg: e.to_string(),
    })?;
    let report_path = Path::new(out_dir).join(format!("{gid_disp}-signcheck.json"));
    fs::write(&report_path, format!("{}\n", py_canonical(&report))).map_err(|e| ToolFailure {
        kind: "OSError",
        msg: e.to_string(),
    })?;
    let done = Command::new(scribe_binary)
        .args([
            "crosscheck",
            "--report",
            &report_path.to_string_lossy(),
            "--material",
            material_path,
            "--trail",
            trail,
            "--locks",
            locks,
            "--session",
            session,
        ])
        .status();
    let code = match done {
        Ok(st) => match st.code() {
            Some(c) => c,
            None => {
                use std::os::unix::process::ExitStatusExt;
                -st.signal().unwrap_or(1)
            }
        },
        Err(e) => {
            return Err(ToolFailure { kind: "OSError", msg: e.to_string() });
        }
    };
    if code != 0 {
        println!(
            "{}",
            py_line(
                &json!({
                    "sign": "failed",
                    "reason": "scribe crosscheck 非零退出，落据未发生链上无事件，唯退出码与此输出为准",
                    "gid": report["gid"],
                    "scribe_exit": code,
                }),
                true
            )
        );
        return Ok(code);
    }
    println!(
        "{}",
        py_line(
            &json!({
                "signed": report["gid"],
                "disposition": report["disposition"],
                "direction": report["direction"],
                "replay_anchor": report_path.to_string_lossy(),
                "note": "人类知晓无需复签；一票绝对反对经 append inconsistency 翻案；边界变更经 watch 浮出",
            }),
            true
        )
    );
    Ok(0)
}

fn watch_reports(reports_dir: &str) -> Result<i32, ToolFailure> {
    let dir = Path::new(reports_dir);
    let mut reports: Vec<String> = vec![];
    if let Ok(rd) = fs::read_dir(dir) {
        for ent in rd.flatten() {
            let p = ent.path();
            if p.to_string_lossy().ends_with("-signcheck.json") {
                reports.push(p.to_string_lossy().into_owned());
            }
        }
    }
    reports.sort();
    if reports.is_empty() {
        println!(
            "{}",
            py_line(&json!({"watch": "empty", "dir": reports_dir}), true)
        );
        return Ok(0);
    }
    let mut anomalies: Vec<Value> = vec![];
    let mut ok = 0usize;
    for rp in &reports {
        let recorded_res = parse_json(&read_text(Path::new(rp))?);
        let recorded = match recorded_res {
            Ok(v) => v,
            Err(e) => {
                anomalies.push(json!({"report": rp, "error": format!("{}: {}", e.kind, e.msg)}));
                continue;
            }
        };
        let gid = recorded.get("gid").cloned().unwrap_or(Value::Null);
        let material_path = match recorded.get("material").and_then(|v| v.as_str()) {
            Some(m) => m.to_string(),
            None => {
                anomalies.push(json!({"report": rp, "error": format!("{}: {}", ToolFailure::key("material").kind, ToolFailure::key("material").msg)}));
                continue;
            }
        };
        match check_material(&material_path) {
            Err(e) => anomalies.push(json!({"report": rp, "error": format!("{}: {}", e.kind, e.msg)})),
            Ok(fresh) => {
                if py_canonical(&fresh) == py_canonical(&recorded) {
                    ok += 1;
                } else {
                    anomalies.push(json!({
                        "report": rp,
                        "gid": gid,
                        "note": "边界条件变更：复算与既报不符（异常视图，人重命题或令 agent 重跑）",
                    }));
                }
            }
        }
    }
    let has_anomalies = !anomalies.is_empty();
    let mut rep = Map::new();
    rep.insert("watch".into(), json!("done"));
    rep.insert("ok".into(), json!(ok));
    rep.insert("anomalies".into(), Value::Array(anomalies));
    println!("{}", py_canonical(&Value::Object(rep)));
    Ok(if has_anomalies { 1 } else { 0 })
}

// ---------- main ----------

fn dispatch(cmd: &str, rest: &[String]) -> Result<i32, ToolFailure> {
    let rest: Vec<String> = rest.to_vec();
    match cmd {
        "check" => {
            let f = parse_flags(&rest, &["--material"], &["--material"]);
            let report = check_material(f["--material"].as_str())?;
            println!("{}", py_canonical(&report));
            Ok(if report["verdict"] == "pass" { 0 } else { 1 })
        }
        "verify" => {
            let f = parse_flags(&rest, &["--material", "--report"], &["--material", "--report"]);
            verify_material(f["--material"].as_str(), f["--report"].as_str())
        }
        "assemble" => {
            // argparse 形：--gid/--des-root/--out 必带，--topics-dir 可重复累积，
            // --baseline/--identity-report/--date 空串同缺席（围堰 if 值语义）
            let mut flags: HashMap<String, String> = HashMap::new();
            let mut topics: Vec<String> = vec![];
            let mut i = 0usize;
            while i < rest.len() {
                let a = &rest[i];
                let (name, inline) = match a.split_once('=') {
                    Some((n, v)) if n.starts_with("--") => (n.to_string(), Some(v.to_string())),
                    _ => (a.clone(), None),
                };
                if !name.starts_with("--") {
                    usage_fail(&format!("意外位置参数: {a}"));
                }
                let val = match inline {
                    Some(v) => v,
                    None => {
                        i += 1;
                        if i >= rest.len() {
                            usage_fail(&format!("{name} 缺值"));
                        }
                        rest[i].clone()
                    }
                };
                match name.as_str() {
                    "--topics-dir" => topics.push(val),
                    "--gid" | "--des-root" | "--baseline" | "--identity-report" | "--date"
                    | "--out" => {
                        flags.insert(name, val);
                    }
                    other => usage_fail(&format!("未知旗标: {other}")),
                }
                i += 1;
            }
            for req in ["--gid", "--des-root", "--out"] {
                if !flags.contains_key(req) {
                    usage_fail(&format!("缺 {req}"));
                }
            }
            let baseline = flags.get("--baseline").filter(|s| !s.is_empty()).cloned();
            let date = flags.get("--date").filter(|s| !s.is_empty()).cloned();
            let identity = flags
                .get("--identity-report")
                .filter(|s| !s.is_empty())
                .cloned();
            let material = assemble_material(
                flags["--gid"].as_str(),
                flags["--des-root"].as_str(),
                &topics,
                baseline.as_deref(),
                date.as_deref(),
                flags["--out"].as_str(),
                identity.as_deref(),
            )?;
            println!(
                "{}",
                py_line(
                    &json!({
                        "assembled": py_path_str(flags["--out"].as_str()),
                        "gid": material["gid"],
                        "gate_verdict": material["gate_verdict"],
                        "n_shots": material["n_shots"],
                    }),
                    true
                )
            );
            Ok(0)
        }
        "watch" => {
            let f = parse_flags(&rest, &["--reports"], &["--reports"]);
            watch_reports(f["--reports"].as_str())
        }
        "sign" => {
            let f = parse_flags(
                &rest,
                &["--material", "--out", "--trail", "--scribe-binary", "--session", "--locks"],
                &["--material", "--out", "--trail", "--scribe-binary", "--session", "--locks"],
            );
            sign_material(
                f["--material"].as_str(),
                f["--out"].as_str(),
                f["--trail"].as_str(),
                f["--scribe-binary"].as_str(),
                f["--session"].as_str(),
                f["--locks"].as_str(),
            )
        }
        other => usage_fail(&format!("未知子命令: {other}")),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage_fail("缺子命令");
    }
    let cmd = args[0].clone();
    let rest: Vec<String> = args[1..].to_vec();
    match dispatch(&cmd, &rest) {
        Ok(c) => exit(c),
        Err(e) => {
            eprintln!(
                "{}",
                py_line(&json!({"error": format!("{}: {}", e.kind, e.msg)}), false)
            );
            exit(2);
        }
    }
}
