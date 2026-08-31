//! 规则引擎：解析规则 + 6 类 text 谓词 + 5 类 json 谓词。
//!
//! 承 SPEC-013 § 接口对表 + § 验收判据 A1-A6。
//! 零规则知识：具体领域字段名不内嵌，规则全在数据。

use serde::Deserialize;
use toml::Value;

/// 标题尾部 anchor `{#...}` 剥离，承工具件 ANCHOR_RE
static ANCHOR_RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
fn anchor_re() -> &'static regex::Regex {
    ANCHOR_RE.get_or_init(|| regex::Regex::new(r"\{#([^}]+)\}\s*$").unwrap())
}

/// 规则条目（解自 rules.toml）
#[derive(Debug, Clone)]
pub struct RuleEntry {
    pub id: String,
    pub kind: RuleKind,
    pub params: toml::value::Table,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleKind {
    // text 类
    CharsetForbid,
    CharsetAllow,
    ForbidPattern,
    HeaderStructure,
    LineFlag,
    NavFormat,
    // json 类
    JsonField,
    JsonArraySchema,
    JsonNumberRange,
    JsonFieldCompare,
    JsonParse,
}

impl RuleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RuleKind::CharsetForbid => "charset_forbid",
            RuleKind::CharsetAllow => "charset_allow",
            RuleKind::ForbidPattern => "forbid_pattern",
            RuleKind::HeaderStructure => "header_structure",
            RuleKind::LineFlag => "line_flag",
            RuleKind::NavFormat => "nav_format",
            RuleKind::JsonField => "json_field",
            RuleKind::JsonArraySchema => "json_array_schema",
            RuleKind::JsonNumberRange => "json_number_range",
            RuleKind::JsonFieldCompare => "json_field_compare",
            RuleKind::JsonParse => "json_parse",
        }
    }
}

/// 解析 rules.toml
pub fn parse_rules(toml_text: &str) -> Result<Vec<RuleEntry>, String> {
    let value: Value = toml::from_str(toml_text).map_err(|e| format!("rules.toml 解析失败: {e}"))?;
    let arr = value
        .as_array()
        .or_else(|| value.get("rules").and_then(|v| v.as_array()))
        .ok_or_else(|| "rules.toml 缺 [[rules]] 数组".to_string())?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let t = item
            .as_table()
            .ok_or_else(|| "[[rules]] 项非 table".to_string())?;
        let id = t
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "规则缺 id".to_string())?
            .to_string();
        let kind_str = t
            .get("kind")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("规则 {id} 缺 kind"))?;
        let kind = match kind_str {
            "charset_forbid" => RuleKind::CharsetForbid,
            "charset_allow" => RuleKind::CharsetAllow,
            "forbid_pattern" => RuleKind::ForbidPattern,
            "header_structure" => RuleKind::HeaderStructure,
            "line_flag" => RuleKind::LineFlag,
            "nav_format" => RuleKind::NavFormat,
            "json_field" => RuleKind::JsonField,
            "json_array_schema" => RuleKind::JsonArraySchema,
            "json_number_range" => RuleKind::JsonNumberRange,
            "json_field_compare" => RuleKind::JsonFieldCompare,
            "json_parse" => RuleKind::JsonParse,
            other => return Err(format!("规则 {id} 未知 kind {other}")),
        };
        let params = t
            .get("params")
            .and_then(|v| v.as_table())
            .cloned()
            .unwrap_or_default();
        let message = t
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        out.push(RuleEntry {
            id,
            kind,
            params,
            message,
        });
    }
    Ok(out)
}

/// 解析 manifest.toml 提取 include/exclude 域
pub fn parse_manifest_domains(toml_text: &str) -> Result<DomainSpec, String> {
    let value: Value = toml::from_str(toml_text).map_err(|e| format!("manifest.toml 解析失败: {e}"))?;
    let domain = value
        .get("domain")
        .ok_or_else(|| "manifest.toml 缺 [domain]".to_string())?;
    let include = domain
        .get("include")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let exclude = domain
        .get("exclude")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let material = value
        .get("material")
        .and_then(|v| v.as_str())
        .map(String::from);
    let version = value
        .get("version")
        .and_then(|v| v.as_str())
        .map(String::from)
        .unwrap_or_else(|| "0.0.0".to_string());
    Ok(DomainSpec {
        include,
        exclude,
        material,
        version,
    })
}

#[derive(Debug, Clone)]
pub struct DomainSpec {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub material: Option<String>,
    pub version: String,
}

/// domain include/exclude glob 匹配
/// 承工具件 Python `(?:^|/)` 锚定语义：双星跨目录段，单星段内任意，模式可自任一路径边界锚定
pub fn domain_match(spec: &DomainSpec, path: &str) -> bool {
    if spec.exclude.iter().any(|p| glob_match(p, path)) {
        return false;
    }
    spec.include.iter().any(|p| glob_match(p, path))
}

fn glob_match(pattern: &str, path: &str) -> bool {
    if pattern == "**" || pattern == "**/*" {
        return true;
    }
    // 编译模式为正则：(?:^|/) 前缀 + 模式体
    // 模式体：`**/` → `(?:[^/]+/)*`、`*` → `[^/]*`、`?` → `[^/]`、其他字面量
    let mut body = String::new();
    let mut i = 0;
    let n = pattern.len();
    let bytes = pattern.as_bytes();
    while i < n {
        if i + 3 <= n && &bytes[i..i + 3] == b"**/" {
            body.push_str("(?:[^/]+/)*");
            i += 3;
        } else if bytes[i] == b'*' {
            body.push_str("[^/]*");
            i += 1;
        } else if bytes[i] == b'?' {
            body.push_str("[^/]");
            i += 1;
        } else {
            // ASCII 字面量用 regex::escape，非 ASCII 直接 push
            let c = pattern[i..].chars().next().unwrap();
            let clen = c.len_utf8();
            // escape 只对 ASCII 安全字母元字符；非 ASCII 视为字面
            body.push_str(&regex::escape(&pattern[i..i + clen]));
            i += clen;
        }
    }
    let full = format!("(?:^|/){body}");
    match regex::Regex::new(&full) {
        Ok(re) => re.is_match(path),
        Err(_) => false,
    }
}

/// 规则对单条文本检查，返回 findings
pub fn check_text(rule: &RuleEntry, text: &str) -> Vec<TextFinding> {
    let mut out = Vec::new();
    match rule.kind {
        RuleKind::CharsetForbid => {
            let strings: Vec<String> = rule
                .params
                .get("strings")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            for (lineno, line) in text.lines().enumerate() {
                for s in &strings {
                    if line.contains(s.as_str()) {
                        out.push(TextFinding {
                            rule_id: rule.id.clone(),
                            line: lineno + 1,
                            message: rule.message.clone(),
                            excerpt: line.to_string(),
                        });
                    }
                }
            }
        }
        RuleKind::CharsetAllow => {
            let ranges: Vec<(u32, u32)> = rule
                .params
                .get("ranges")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str())
                        .filter_map(|s| {
                            let mut parts = s.split('-');
                            let a = u32::from_str_radix(parts.next()?, 16).ok()?;
                            let b = u32::from_str_radix(parts.next()?, 16).ok()?;
                            Some((a, b))
                        })
                        .collect()
                })
                .unwrap_or_default();
            for (lineno, line) in text.lines().enumerate() {
                for (_col, ch) in line.chars().enumerate() {
                    let cp = ch as u32;
                    if !ranges.iter().any(|(a, b)| cp >= *a && cp <= *b) {
                        let msg = rule.message.replace("{char}", &format!("{:04X}", cp));
                        out.push(TextFinding {
                            rule_id: rule.id.clone(),
                            line: lineno + 1,
                            message: msg,
                            excerpt: line.to_string(),
                        });
                        break;
                    }
                }
            }
        }
        RuleKind::ForbidPattern => {
            if let Some(pat) = rule.params.get("pattern").and_then(|v| v.as_str()) {
                if let Ok(re) = regex::Regex::new(pat) {
                    for (lineno, line) in text.lines().enumerate() {
                        if let Some(m) = re.find(line) {
                            out.push(TextFinding {
                                rule_id: rule.id.clone(),
                                line: lineno + 1,
                                message: format!("{} [match={}]", rule.message, m.as_str()),
                                excerpt: line.to_string(),
                            });
                        }
                    }
                }
            }
        }
        RuleKind::HeaderStructure => {
            let check = rule
                .params
                .get("check")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let mut h1_count = 0;
            let mut prev_level: i32 = 0;
            let mut first_h2: Option<(usize, String)> = None;
            for (lineno, line) in text.lines().enumerate() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("# ") {
                    h1_count += 1;
                    prev_level = 1;
                } else if trimmed.starts_with("## ") {
                    let raw_title = trimmed[3..].trim().to_string();
                    if first_h2.is_none() {
                        first_h2 = Some((lineno + 1, raw_title.clone()));
                    }
                    let name = rule
                        .params
                        .get("name")
                        .and_then(|v| match v {
                            Value::String(s) => Some(vec![s.clone()]),
                            Value::Array(a) => Some(
                                a.iter()
                                    .filter_map(|x| x.as_str().map(String::from))
                                    .collect(),
                            ),
                            _ => None,
                        })
                        .unwrap_or_default();
                    if check == "first_h2_name" && !name.is_empty() {
                        if let Some((first_line, first_title)) = &first_h2 {
                            if lineno + 1 == *first_line {
                                let bare = anchor_re().replace(first_title, "").trim().to_string();
                                if !name.iter().any(|n| n == &bare) {
                                    out.push(TextFinding {
                                        rule_id: rule.id.clone(),
                                        line: lineno + 1,
                                        message: rule.message.replace("{title}", &bare),
                                        excerpt: line.to_string(),
                                    });
                                }
                            }
                        }
                    }
                    if prev_level > 2 {
                        out.push(TextFinding {
                            rule_id: rule.id.clone(),
                            line: lineno + 1,
                            message: format!("标题层级跳级:上一级为 {prev_level} 级,当前为 2 级"),
                            excerpt: line.to_string(),
                        });
                    }
                    prev_level = 2;
                } else if trimmed.starts_with("#") {
                    let level = trimmed.chars().take_while(|c| *c == '#').count() as i32;
                    if prev_level > 0 && level > prev_level + 1 {
                        out.push(TextFinding {
                            rule_id: rule.id.clone(),
                            line: lineno + 1,
                            message: format!("标题层级跳级:上一级为 {prev_level} 级,当前为 {level} 级"),
                            excerpt: line.to_string(),
                        });
                    }
                    prev_level = level;
                }
            }
            if check == "single_h1" && h1_count > 1 {
                out.push(TextFinding {
                    rule_id: rule.id.clone(),
                    line: 0,
                    message: "全文仅允许一个一级标题".to_string(),
                    excerpt: format!("h1_count={h1_count}"),
                });
            }
        }
        RuleKind::LineFlag => {
            let flag = rule
                .params
                .get("flag")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            for (lineno, line) in text.lines().enumerate() {
                match flag {
                    "blockquote" => {
                        if line.trim_start().starts_with("> ") || line.trim_start() == ">" {
                            out.push(TextFinding {
                                rule_id: rule.id.clone(),
                                line: lineno + 1,
                                message: rule.message.clone(),
                                excerpt: line.to_string(),
                            });
                        }
                    }
                    "bold" => {
                        if line.contains("**") && line.matches("**").count() >= 2 {
                            out.push(TextFinding {
                                rule_id: rule.id.clone(),
                                line: lineno + 1,
                                message: rule.message.clone(),
                                excerpt: line.to_string(),
                            });
                        }
                    }
                    "table" => {
                        if line.trim_start().starts_with('|') && line.contains('|') {
                            out.push(TextFinding {
                                rule_id: rule.id.clone(),
                                line: lineno + 1,
                                message: rule.message.clone(),
                                excerpt: line.to_string(),
                            });
                        }
                    }
                    "fence_open" => {
                        if line.trim_start().starts_with("```") {
                            let lang = line.trim_start().trim_start_matches("```").trim();
                            let allow: Vec<String> = rule
                                .params
                                .get("allow_langs")
                                .and_then(|v| v.as_array())
                                .map(|a| {
                                    a.iter()
                                        .filter_map(|x| x.as_str().map(String::from))
                                        .collect()
                                })
                                .unwrap_or_default();
                            if !lang.is_empty() && !allow.iter().any(|a| lang.starts_with(a)) {
                                out.push(TextFinding {
                                    rule_id: rule.id.clone(),
                                    line: lineno + 1,
                                    message: rule.message.clone(),
                                    excerpt: line.to_string(),
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        RuleKind::NavFormat => {
            // 简化：检查每行是否形如「...::[...](#...)」
            for (lineno, line) in text.lines().enumerate() {
                if !line.contains("::[") || !line.contains("](#") {
                    continue;
                }
                if !looks_like_nav_line(line) {
                    out.push(TextFinding {
                        rule_id: rule.id.clone(),
                        line: lineno + 1,
                        message: rule.message.clone(),
                        excerpt: line.to_string(),
                    });
                }
            }
        }
        _ => {}
    }
    out
}

fn looks_like_nav_line(line: &str) -> bool {
    // 承工具件 NAV_FULL_RE: ^\s*(?:[-*+]\s*)?[^:\n]+::\[[^\]]+\]\(#?[^)]*\)\s*$
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r"^\s*(?:[-*+]\s*)?[^:\n]+::\[[^\]]+\]\(#?[^)]*\)\s*$").unwrap()
    });
    re.is_match(line)
}

#[derive(Debug, Clone)]
pub struct TextFinding {
    pub rule_id: String,
    pub line: usize,
    pub message: String,
    pub excerpt: String,
}

/// 解析 manifest 加载规则集
pub fn load_pack(pack_name: &str) -> Result<(DomainSpec, Vec<RuleEntry>), String> {
    let manifest_text = crate::scrutinator::asset::manifest(pack_name);
    if manifest_text.is_empty() {
        return Err(format!("未知名包: {pack_name}"));
    }
    let domain = parse_manifest_domains(manifest_text)?;
    let rules_text = crate::scrutinator::asset::rules(pack_name);
    let rules = parse_rules(rules_text)?;
    Ok((domain, rules))
}

/// 解析 JSON 文本（json_parse 谓词用）
pub fn try_parse_json(text: &str) -> Result<serde_json::Value, String> {
    serde_json::from_str(text).map_err(|e| format!("JSON 解析失败: {e}"))
}

/// 简易单值查找（json_field 谓词支持点分路径 + 数组下标 + 通配）
pub fn json_get<'a>(
    value: &'a serde_json::Value,
    path: &str,
) -> Option<&'a serde_json::Value> {
    let mut cur = value;
    for part in path.split('.') {
        // 数组下标如 foo[0]
        if let Some(idx) = part.find('[') {
            let key = &part[..idx];
            let rest = &part[idx..];
            if !key.is_empty() {
                cur = cur.get(key)?;
            }
            // 解析 [n] 段
            let mut s = rest;
            while let Some(open) = s.find('[') {
                let close = s.find(']')?;
                let n: usize = s[open + 1..close].parse().ok()?;
                cur = cur.get(n)?;
                s = &s[close + 1..];
            }
        } else {
            cur = cur.get(part)?;
        }
    }
    Some(cur)
}

/// 检查 json_field 谓词
pub fn check_json_field(rule: &RuleEntry, value: &serde_json::Value) -> Vec<JsonFinding> {
    let mut out = Vec::new();
    let path = rule
        .params
        .get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let expected_type = rule
        .params
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let enum_v: Option<Vec<String>> = rule.params.get("enum").and_then(|v| match v {
        Value::String(s) => Some(vec![s.clone()]),
        Value::Array(a) => Some(
            a.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect(),
        ),
        _ => None,
    });
    let optional = rule
        .params
        .get("optional")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let v = json_get(value, path);
    match (v, optional) {
        (None, true) => return out,
        (None, false) => {
            out.push(JsonFinding {
                rule_id: rule.id.clone(),
                pointer: path.to_string(),
                message: format!("缺字段 {path}"),
            });
            return out;
        }
        (Some(val), _) => {
            let actual_type = match val {
                serde_json::Value::Null => "null",
                serde_json::Value::Bool(_) => "bool",
                serde_json::Value::Number(n) => {
                    if n.is_i64() || n.is_u64() {
                        "int"
                    } else {
                        "number"
                    }
                }
                serde_json::Value::String(_) => "str",
                serde_json::Value::Array(_) => "array",
                serde_json::Value::Object(_) => "object",
            };
            if !expected_type.is_empty() && actual_type != expected_type {
                out.push(JsonFinding {
                    rule_id: rule.id.clone(),
                    pointer: path.to_string(),
                    message: format!("类型 {actual_type} ≠ {expected_type}"),
                });
                return out;
            }
            if let Some(enums) = &enum_v {
                let s = val.as_str().unwrap_or_default();
                if !enums.iter().any(|e| e == s) {
                    out.push(JsonFinding {
                        rule_id: rule.id.clone(),
                        pointer: path.to_string(),
                        message: format!("值 {s} 不在枚举 {enums:?}"),
                    });
                }
            }
        }
    }
    out
}

#[derive(Debug, Clone)]
pub struct JsonFinding {
    pub rule_id: String,
    pub pointer: String,
    pub message: String,
}

/// 字段比较（json_field_compare）
pub fn check_json_field_compare(rule: &RuleEntry, value: &serde_json::Value) -> Vec<JsonFinding> {
    let mut out = Vec::new();
    let left_path = rule
        .params
        .get("left")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let op = rule
        .params
        .get("op")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let right = rule.params.get("right");
    let lv = json_get(value, left_path);
    let rv: Option<serde_json::Value> = right.and_then(|r| match r {
        Value::Integer(n) => Some(serde_json::json!(*n)),
        Value::Float(f) => Some(serde_json::json!(*f)),
        Value::String(s) => Some(serde_json::Value::String(s.clone())),
        _ => None,
    });
    if let (Some(ln), Some(rn)) = (lv.and_then(|v| v.as_f64()), rv.and_then(|v| v.as_f64())) {
        let ok = match op {
            "<=" => ln <= rn,
            ">=" => ln >= rn,
            "<" => ln < rn,
            ">" => ln > rn,
            "==" => ln == rn,
            "!=" => ln != rn,
            _ => true,
        };
        if !ok {
            out.push(JsonFinding {
                rule_id: rule.id.clone(),
                pointer: left_path.to_string(),
                message: format!("{left_path} {op} {rn} 不成立 (实际 {ln})"),
            });
        }
    }
    out
}

/// 数值闭区间（json_number_range）
pub fn check_json_number_range(rule: &RuleEntry, value: &serde_json::Value) -> Vec<JsonFinding> {
    let mut out = Vec::new();
    let path = rule
        .params
        .get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let min = rule
        .params
        .get("min")
        .and_then(|v| v.as_float())
        .unwrap_or(f64::MIN);
    let max = rule
        .params
        .get("max")
        .and_then(|v| v.as_float())
        .unwrap_or(f64::MAX);
    if let Some(v) = json_get(value, path).and_then(|v| v.as_f64()) {
        if !(min..=max).contains(&v) {
            out.push(JsonFinding {
                rule_id: rule.id.clone(),
                pointer: path.to_string(),
                message: format!("{path} = {v} 不在 {min}..={max}"),
            });
        }
    }
    out
}

/// 数组 schema 简化：检查数组存在与每元素 type（json_array_schema）
pub fn check_json_array_schema(rule: &RuleEntry, value: &serde_json::Value) -> Vec<JsonFinding> {
    let mut out = Vec::new();
    let path = rule
        .params
        .get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let min_items = rule
        .params
        .get("min_items")
        .and_then(|v| v.as_integer())
        .unwrap_or(0) as usize;
    if let Some(arr) = json_get(value, path).and_then(|v| v.as_array()) {
        if arr.len() < min_items {
            out.push(JsonFinding {
                rule_id: rule.id.clone(),
                pointer: path.to_string(),
                message: format!("数组长度 {} < {min_items}", arr.len()),
            });
        }
        // fields 简化：每元素若是 object 检查给定 fields
        if let Some(fields) = rule.params.get("fields").and_then(|v| v.as_table()) {
            for (i, el) in arr.iter().enumerate() {
                for (fname, ftype) in fields.iter() {
                    let expected = match ftype {
                        Value::String(s) => s.clone(),
                        _ => continue,
                    };
                    let sub = el.get(fname);
                    let actual = match sub {
                        Some(serde_json::Value::String(_)) => "str",
                        Some(serde_json::Value::Number(n)) => {
                            if n.is_i64() || n.is_u64() { "int" } else { "number" }
                        }
                        Some(serde_json::Value::Bool(_)) => "bool",
                        Some(serde_json::Value::Array(_)) => "array",
                        Some(serde_json::Value::Object(_)) => "object",
                        Some(serde_json::Value::Null) | None => "null",
                    };
                    if actual != expected {
                        out.push(JsonFinding {
                            rule_id: rule.id.clone(),
                            pointer: format!("{path}[{i}].{fname}"),
                            message: format!("类型 {actual} ≠ {expected}"),
                        });
                    }
                }
            }
        }
    }
    out
}
