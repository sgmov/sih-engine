//! validators——PRO-001 内置检验器函数库（融回自围堰 facet/src/validators.py）。
//!
//! atom.yaml 的 validator 字段引用函数名 + 参数。注册表四件：anchor_whitelist、
//! filter_selection、markdown_output、json_schema。纯解析校验，无采样依赖。

use std::collections::HashSet;
use std::path::Path;

use serde_json::{json, Map, Value};

use super::anchors::{load_whitelist, validate as anchor_validate};
use super::jsonc::{verr, PyError};

fn fence_re() -> &'static regex::Regex {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r"(?s)```(?:[a-zA-Z]*)\s*(.*?)\s*```").unwrap())
}

fn fenced_text(output: &str) -> String {
    match fence_re().captures(output) {
        Some(c) => c.get(1).map(|m| m.as_str().to_string()).unwrap_or_else(|| output.to_string()),
        None => output.to_string(),
    }
}

/// 花括号平衡切片后 parse，坏形返回 None。
fn brace_slice_parse(output: &str) -> Option<Value> {
    let text = fenced_text(output);
    let start = text.find('{')?;
    let mut depth = 0i32;
    for (i, ch) in text.char_indices().skip_while(|(i, _)| *i < start) {
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            if depth == 0 {
                let obj: Value = serde_json::from_str(&text[start..=i]).ok()?;
                return Some(obj);
            }
        }
    }
    None
}

/// 从 LLM output 解析 facets JSON。
pub fn parse_facets(output: &str) -> Vec<Value> {
    match brace_slice_parse(output) {
        Some(obj) => obj
            .get("facets")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default(),
        None => Vec::new(),
    }
}

/// 从 LLM output 解析 filter_selection 的 selected 列表。
pub fn parse_filter_output(output: &str) -> Option<Vec<Value>> {
    let obj = brace_slice_parse(output)?;
    match obj.get("selected") {
        Some(Value::Array(a)) => Some(a.clone()),
        _ => None,
    }
}

pub type ValidatorFn = fn(&str, &Value, &Path, &Path) -> Result<Value, PyError>;

/// anchor_whitelist 检验器：anchor_ref 在白名单内 + parent_facet_ids 引用
/// 真实存在。params 承 atom.yaml validator_params（strict 缺省 true），
/// runner 预加载注入 _whitelist 与 _prior_facet_ids。
pub fn anchor_whitelist(output: &str, params: &Value, topic_path: &Path, project_root: &Path) -> Result<Value, PyError> {
    let strict = params.get("strict").and_then(|v| v.as_bool()).unwrap_or(true);
    let whitelist: HashSet<(String, i64)> = match params.get("_whitelist") {
        Some(Value::Array(a)) => {
            let mut s = HashSet::new();
            for pair in a {
                if let Some(arr) = pair.as_array() {
                    if arr.len() >= 2 {
                        s.insert((
                            arr[0].as_str().unwrap_or_default().to_string(),
                            arr[1].as_i64().unwrap_or(0),
                        ));
                    }
                }
            }
            s
        }
        _ => load_whitelist(topic_path, project_root)?,
    };
    let prior_facet_ids: Option<HashSet<String>> = match params.get("_prior_facet_ids") {
        Some(Value::Array(a)) => Some(
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
        ),
        Some(Value::Null) | None => None,
        _ => None,
    };

    let facets = parse_facets(output);
    if facets.is_empty() {
        return Ok(json!({
            "valid": !strict, "passed": 0, "total": 0, "dropped": [], "dropped_parents": [],
        }));
    }
    let mut passed = 0i64;
    let mut dropped = Vec::new();
    let mut dropped_parents = Vec::new();
    for f in &facets {
        let ref_ = f.get("anchor_ref").and_then(|v| v.as_str()).unwrap_or("");
        if ref_.is_empty() {
            continue;
        }
        let validation = anchor_validate(ref_, &whitelist, project_root);
        if validation.result == "Keep" {
            passed += 1;
        } else {
            dropped.push(json!({
                "anchor_ref": ref_,
                "result": validation.result,
                "reason": validation.reason.unwrap_or_default(),
            }));
        }
        if let Some(prior) = &prior_facet_ids {
            let parents = f
                .get("parent_facet_ids")
                .or_else(|| f.get("parent_ids"))
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            for pid in parents {
                let pid_s = pid.as_str().unwrap_or_default().to_string();
                if !prior.contains(&pid_s) {
                    dropped_parents.push(json!({
                        "facet_id": f.get("id").cloned().unwrap_or(Value::String(String::new())),
                        "parent_facet_id": pid_s,
                        "reason": format!("parent_facet_id '{}' 不在前序产出 id 集合内", pid_s),
                    }));
                }
            }
        }
    }
    let total = facets.len() as i64;
    let valid = if strict { passed == total } else { true };
    Ok(json!({
        "valid": valid, "passed": passed, "total": total,
        "dropped": dropped, "dropped_parents": dropped_parents,
    }))
}

/// filter_selection 检验器：筛选输出格式与 max_selected 约束。
pub fn filter_selection(output: &str, params: &Value, _topic_path: &Path, _project_root: &Path) -> Result<Value, PyError> {
    let max_selected = params.get("max_selected").and_then(|v| v.as_i64()).unwrap_or(2);
    let candidate_ids: Option<HashSet<String>> = match params.get("_candidate_actor_ids") {
        Some(Value::Array(a)) => Some(
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
        ),
        _ => None,
    };

    let parsed = parse_facets(output);
    if parsed.is_empty() {
        return Ok(json!({"valid": false, "passed": 0, "total": 0, "dropped": []}));
    }
    // filter_selection 输出 key 是 selected 不是 facets
    let selected = match parse_filter_output(output) {
        Some(s) => s,
        None => return Ok(json!({"valid": false, "passed": 0, "total": 0, "dropped": []})),
    };

    let mut dropped = Vec::new();
    let mut valid_ids: Vec<String> = selected
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_string())
        .collect();
    if let Some(candidates) = &candidate_ids {
        let ids_snapshot = valid_ids.clone();
        for sid in ids_snapshot {
            if !candidates.contains(&sid) {
                dropped.push(json!({
                    "selected_id": sid,
                    "reason": format!("selected id '{}' 不在候选 actor_id 集合中", sid),
                }));
                valid_ids.retain(|x| *x != sid);
            }
        }
    }
    if selected.len() as i64 > max_selected {
        return Ok(json!({
            "valid": false, "passed": valid_ids.len() as i64,
            "total": selected.len() as i64, "dropped": dropped,
        }));
    }
    let passed = valid_ids.len() as i64;
    let total = selected.len() as i64;
    Ok(json!({
        "valid": passed > 0, "passed": passed, "total": total, "dropped": dropped,
    }))
}

/// markdown_output 检验器：有效 markdown 且达到最小长度。
pub fn markdown_output(output: &str, params: &Value, _topic_path: &Path, _project_root: &Path) -> Result<Value, PyError> {
    let min_length = params.get("min_length").and_then(|v| v.as_i64()).unwrap_or(50);
    let length = output.trim().chars().count() as i64;
    let passed = length >= min_length;
    Ok(json!({
        "valid": passed,
        "passed": if passed { 1 } else { 0 },
        "total": 1,
        "dropped": if passed { vec![] } else { vec![json!({"reason": format!("输出长度 {} < min_length {}", length, min_length)})] },
    }))
}

/// json_schema 检验器：顶层 JSON 对象校验（required_fields + schema.type +
/// enum 约束）。
pub fn json_schema(output: &str, params: &Value, _topic_path: &Path, _project_root: &Path) -> Result<Value, PyError> {
    let schema = params.get("schema").cloned().unwrap_or(json!({}));
    let required_fields: Vec<String> = params
        .get("required_fields")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    let text = fenced_text(output);
    let start = match text.find('{') {
        Some(s) => s,
        None => {
            return Ok(json!({
                "valid": false, "passed": 0, "total": 1,
                "dropped": [{"reason": "输出中无 JSON 对象"}],
            }))
        }
    };
    let mut depth = 0i32;
    let mut parsed: Option<Value> = None;
    for (i, ch) in text.char_indices().skip_while(|(i, _)| *i < start) {
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            if depth == 0 {
                match serde_json::from_str::<Value>(&text[start..=i]) {
                    Ok(v) => {
                        parsed = Some(v);
                        break;
                    }
                    Err(e) => {
                        return Ok(json!({
                            "valid": false, "passed": 0, "total": 1,
                            "dropped": [{"reason": format!("JSON 解析失败: {}", e)}],
                        }));
                    }
                }
            }
        }
    }
    let obj = match parsed {
        Some(v) if v.is_object() => v,
        _ => {
            return Ok(json!({
                "valid": false, "passed": 0, "total": 1,
                "dropped": [{"reason": "输出无有效 JSON 对象"}],
            }))
        }
    };

    let mut dropped: Vec<Value> = Vec::new();
    for field in &required_fields {
        if obj.get(field).is_none() {
            dropped.push(json!({"reason": format!("缺少必需字段 '{}'", field)}));
        }
    }
    let py_type = |v: &Value| -> &'static str {
        match v {
            Value::String(_) => "str",
            Value::Bool(_) => "bool",
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    "int"
                } else {
                    "float"
                }
            }
            Value::Array(_) => "list",
            Value::Object(_) => "dict",
            Value::Null => "NoneType",
        }
    };
    if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
        for (key, type_expected) in props {
            if let Some(actual) = obj.get(key) {
                if let Some(expected_type) = type_expected.get("type").and_then(|v| v.as_str()) {
                    let expected_py = match expected_type {
                        "string" => "str",
                        "boolean" => "bool",
                        "integer" => "int",
                        "number" => "float",
                        other => other,
                    };
                    let actual_type = py_type(actual);
                    let compatible =
                        actual_type == expected_py || (expected_py == "float" && actual_type == "int");
                    if !compatible {
                        dropped.push(json!({
                            "reason": format!("字段 '{}' 类型期望 {}，实际 {}", key, expected_type, actual_type)
                        }));
                    }
                }
            }
        }
        for (key, prop_schema) in props {
            let allowed = prop_schema.get("enum").and_then(|v| v.as_array()).cloned();
            if let (Some(allowed), Some(actual)) = (allowed, obj.get(key)) {
                if !allowed.contains(actual) {
                    dropped.push(json!({
                        "reason": format!("字段 '{}' 值 '{}' 不在允许值 {} 中", key, actual, Value::Array(allowed.clone()))
                    }));
                }
            }
        }
    }
    let valid = dropped.is_empty();
    Ok(json!({
        "valid": valid,
        "passed": if valid { 1 } else { 0 },
        "total": 1,
        "dropped": dropped,
    }))
}

/// 检验器注册表（atom.yaml 的 func 名必须在这里）。返回 None 即名不在册。
pub fn validator(func_name: &str) -> Option<ValidatorFn> {
    match func_name {
        "anchor_whitelist" => Some(anchor_whitelist),
        "filter_selection" => Some(filter_selection),
        "markdown_output" => Some(markdown_output),
        "json_schema" => Some(json_schema),
        _ => None,
    }
}

/// 注册表名单（paradigm_loader 校验用，序对齐围堰 dict 字面）。
pub fn valid_names() -> Vec<&'static str> {
    vec!["anchor_whitelist", "filter_selection", "markdown_output", "json_schema"]
}

/// 供范式校验的名单形（String 化）。
pub fn valid_names_str() -> Vec<String> {
    valid_names().into_iter().map(|s| s.to_string()).collect()
}

/// 断言注册表可经 validator() 取回（registry 完整性自检）。
pub fn registry_complete() -> bool {
    valid_names().iter().all(|n| validator(n).is_some())
}

// Map 引用保持（避免未用告警的合法占位：dropped 构造经由 json! 宏，不直接
// 用 Map 类型）。
#[allow(dead_code)]
fn _map_ref(_m: &Map<String, Value>) {}

// verr 保留信封一致性（错误路径统一 ValueError 形）。
#[allow(dead_code)]
fn _verr_ref() -> PyError {
    verr("unused")
}
