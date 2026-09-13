//! 引擎侧检查器（checker）命令行面 —— lease-mergeleg6-parallel 簇I 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/checker 0.1.0（cli.py、engine.py），只读对表移植，
//! 围堰源码零改动。空腹判定机：规则全部来自格式包 check 机器字段，引擎只实现
//! 封闭操作词汇表（十六原子操作加一组合子）的通用求值，零文档形状知识。
//!
//! CLI：`checker --pack <包.json> <目标.md> [--reference <被引用文档.md>]`
//! 退出码三值：0 = 合规、1 = 违规、2 = 工具自身异常。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. 报告 stdout 为 Python json.dumps(sort_keys=True, indent=1) 同构形
//!    （自足序列化器逐字节对齐，含空容器行内形与缩进层进）。
//! 2. 正则方言边缘形不对齐：Python `$` 在非多行下匹配串尾或尾换行前，Rust regex
//!    非多行只匹配串尾；Python findall 对多捕获组返回组元组，reference_closure 的
//!    ref_pattern 多组形语义未对齐（单组形与无组形同构）。坏正则围堰以
//!    sre_constants.error 崩溃，移植按包非法退出码二。
//! 3. 包参数缺项（如缺 level）：围堰 KeyError 崩溃退出码一（traceback），移植按
//!    包非法 PackError 退出码二 fail-closed。
//! 4. 向量件与包 JSON 不可解析：围堰 JSONDecodeError 未接（traceback 退出码一），
//!    移植按包非法退出码二。
//! 5. argparse 用法错为自足解析，stderr 短句回报退出码 2，报文形不对齐。

use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::exit;

const ENGINE_VERSION: &str = "0.1.0";
const VOCAB: [&str; 16] = [
    "exists_readable",
    "heading_present",
    "sections_present",
    "section_nonempty",
    "sections_all_nonempty",
    "section_has_matches",
    "pattern_unique",
    "clause_body_has",
    "clauses_with_marker",
    "clause_has_children",
    "child_has_markers",
    "child_line_matches",
    "list_items_present",
    "list_items_match",
    "list_items_field",
    "reference_closure",
];
const COMBINATORS: [&str; 1] = ["any_of"];

/// 引擎错误：包非法 / 引用目标异常，均退出码二。
enum CErr {
    Pack(String),
    Target(String),
}

type OpResult = Result<(String, String), CErr>;

struct Ctx<'a> {
    text: Option<&'a str>,
    reference: Option<&'a str>,
}

// ---------- Python json.dumps 同构序列化（sort_keys / indent 对齐） ----------

fn json_quote(s: &str) -> String {
    serde_json::to_string(s).unwrap()
}

fn py_json(v: &Value, indent: Option<usize>, sort: bool) -> String {
    let mut out = String::new();
    py_rec(v, indent, sort, 0, &mut out);
    out
}

fn py_rec(v: &Value, indent: Option<usize>, sort: bool, depth: usize, out: &mut String) {
    match v {
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
            if sort {
                kvs.sort_by(|a, b| a.0.cmp(b.0));
            }
            out.push('{');
            for (i, (k, val)) in kvs.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                if let Some(n) = indent {
                    out.push('\n');
                    out.push_str(&" ".repeat((depth + 1) * n));
                }
                out.push_str(&json_quote(k));
                out.push_str(": ");
                py_rec(val, indent, sort, depth + 1, out);
            }
            if let Some(n) = indent {
                out.push('\n');
                out.push_str(&" ".repeat(depth * n));
            }
            out.push('}');
        }
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, val) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                if let Some(n) = indent {
                    out.push('\n');
                    out.push_str(&" ".repeat((depth + 1) * n));
                }
                py_rec(val, indent, sort, depth + 1, out);
            }
            if let Some(n) = indent {
                out.push('\n');
                out.push_str(&" ".repeat(depth * n));
            }
            out.push(']');
        }
        Value::String(s) => out.push_str(&json_quote(s)),
        Value::Number(n) => out.push_str(&n.to_string()),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Null => out.push_str("null"),
    }
}

// ---------- 正则承载 ----------

fn compile(pattern: &str, multiline: bool) -> Result<regex::Regex, CErr> {
    regex::RegexBuilder::new(pattern)
        .multi_line(multiline)
        .build()
        .map_err(|e| CErr::Pack(format!("正则不可编译：{pattern}（{e}）")))
}

/// Python re.split(lookahead) 同构块切分：以模式匹配起点切块，首前段弃。
fn blocks(text: &str, pattern: &str) -> Result<Vec<String>, CErr> {
    let re = compile(pattern, pattern.contains('^'))?;
    let starts: Vec<usize> = re.find_iter(text).map(|m| m.start()).collect();
    let mut out = vec![];
    for (i, &s) in starts.iter().enumerate() {
        let end = starts.get(i + 1).copied().unwrap_or(text.len());
        out.push(text[s..end].to_string());
    }
    Ok(out)
}

/// 模式在块首的匹配串（re.match(...).group(0) 同构，恒多行旗标与围堰一致）。
fn match_at_start(pattern: &str, text: &str) -> Result<String, CErr> {
    let re = compile(pattern, true)?;
    Ok(re
        .find_at(text, 0)
        .filter(|m| m.start() == 0)
        .map(|m| m.as_str().to_string())
        .unwrap_or_default()
        .trim()
        .to_string())
}

/// Python re.findall 字符串化同构：无组取整配，单组取组，多组逐组平铺
/// （未匹配组按空串，Python findall None→'' 同形）。
fn findall_strings(re: &regex::Regex, text: &str) -> Vec<String> {
    let ngroups = re.captures_len().saturating_sub(1);
    let mut out = vec![];
    for caps in re.captures_iter(text) {
        if ngroups == 0 {
            out.push(caps.get(0).unwrap().as_str().to_string());
        } else {
            for i in 1..=ngroups {
                out.push(caps
                    .get(i)
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .to_string());
            }
        }
    }
    out
}

// ---------- 章节承载（engine.py _section 对表） ----------

fn section(text: &str, name: &str, level: usize) -> Option<String> {
    let pat = regex::Regex::new(&format!(r"(?m)^#{{{}}}\s+(.+?)\s*$", level)).ok()?;
    let heads: Vec<(usize, String)> = pat
        .captures_iter(text)
        .map(|c| {
            let start = c.get(0).unwrap().start();
            (start, c.get(1).unwrap().as_str().trim().to_string())
        })
        .collect();
    for (i, (pos, title)) in heads.iter().enumerate() {
        if title == name {
            let end = heads.get(i + 1).map(|(p, _)| *p).unwrap_or(text.len());
            let body_start = text[*pos..].find('\n').map(|p| pos + p + 1).unwrap_or(0);
            return Some(text[body_start..end].to_string());
        }
    }
    None
}

// ---------- 参数取值 ----------

fn p_u64<'a>(op: &'a Value, key: &str) -> Result<u64, CErr> {
    op.get(key)
        .and_then(|v| v.as_u64())
        .ok_or_else(|| CErr::Pack(format!("包参数缺项：{key}")))
}

fn p_str<'a>(op: &'a Value, key: &str) -> Result<&'a str, CErr> {
    op.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| CErr::Pack(format!("包参数缺项：{key}")))
}

fn p_strlist(op: &Value, key: &str) -> Result<Vec<String>, CErr> {
    op.get(key)
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .map(|x| x.as_str().unwrap_or("").to_string())
                .collect()
        })
        .ok_or_else(|| CErr::Pack(format!("包参数缺项：{key}")))
}

fn p_truthy(op: &Value, key: &str) -> bool {
    match op.get(key) {
        None => false,
        Some(v) => match v {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => n.as_f64().map(|f| f != 0.0).unwrap_or(false),
            Value::String(s) => !s.is_empty(),
            _ => true,
        },
    }
}

fn trunc(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

// ---------- 操作词汇表（engine.py VOCAB 对表） ----------

fn run_op(op: &Value, ctx: &Ctx) -> OpResult {
    let name = op.get("op").and_then(|v| v.as_str()).unwrap_or("");
    if COMBINATORS.contains(&name) {
        let branches = op
            .get("branches")
            .and_then(|v| v.as_array())
            .ok_or_else(|| CErr::Pack("any_of 缺 branches".to_string()))?;
        let mut first_fail: Option<(String, String)> = None;
        for branch in branches {
            let (state, detail) = run_op(branch, ctx)?;
            if state == "pass" {
                return Ok(("pass".to_string(), String::new()));
            }
            if first_fail.is_none() {
                first_fail = Some((state, detail));
            }
        }
        return Ok(first_fail.unwrap_or(("pass".to_string(), String::new())));
    }
    if !VOCAB.contains(&name) {
        return Err(CErr::Pack(format!("词汇表外操作：{name}")));
    }
    let text = ctx.text;
    match name {
        "exists_readable" => {
            if text.is_some() {
                Ok(("pass".to_string(), String::new()))
            } else {
                Ok(("missing".to_string(), "目标缺席或不可读".to_string()))
            }
        }
        "heading_present" => {
            let level = p_u64(op, "level")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            let re = compile(&format!(r"(?m)^#{{{level}}}\s+(\S.+?)\s*$"), true)?;
            if re.is_match(text) {
                Ok(("pass".to_string(), String::new()))
            } else {
                Ok(("missing".to_string(), "该级标题缺席".to_string()))
            }
        }
        "sections_present" => {
            let level = p_u64(op, "level")? as usize;
            let names = p_strlist(op, "names")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            let missing: Vec<&str> = names
                .iter()
                .map(|n| n.as_str())
                .filter(|n| section(text, n, level).is_none())
                .collect();
            if !missing.is_empty() {
                return Ok(("missing".to_string(), format!("缺节位：{}", missing.join("、"))));
            }
            if p_truthy(op, "ordered") {
                // 围堰 str.find 未命中返 -1（如首行即节头无前导换行），序比较含 -1。
                let pos: Vec<i64> = names
                    .iter()
                    .map(|n| {
                        text.find(&format!("\n{} {}", "#".repeat(level), n))
                            .map(|p| p as i64)
                            .unwrap_or(-1)
                    })
                    .collect();
                let mut sorted_p = pos.clone();
                sorted_p.sort();
                if pos != sorted_p {
                    return Ok(("violation".to_string(), "节序错位".to_string()));
                }
            }
            Ok(("pass".to_string(), String::new()))
        }
        "section_nonempty" => {
            let level = p_u64(op, "level")? as usize;
            let name = p_str(op, "name")?;
            if let Some(body) = text.and_then(|t| section(t, name, level)) {
                if body.trim().is_empty() {
                    return Ok(("violation".to_string(), format!("空节：{name}")));
                }
            }
            Ok(("pass".to_string(), String::new()))
        }
        "sections_all_nonempty" => {
            let level = p_u64(op, "level")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            let pat = compile(&format!(r"(?m)^#{{{level}}}\s+(.+?)\s*$"), true)?;
            let mut empties: Vec<String> = vec![];
            let matches: Vec<regex::Match> = pat.find_iter(text).collect();
            for (i, m) in matches.iter().enumerate() {
                let start = m.end();
                let end = matches.get(i + 1).map(|n| n.start()).unwrap_or(text.len());
                let body = &text[start..end];
                if body.trim().is_empty() {
                    empties.push(
                        pat.captures_at(text, m.start())
                            .map(|c| c.get(1).unwrap().as_str().trim().to_string())
                            .unwrap_or_default(),
                    );
                }
            }
            if !empties.is_empty() {
                return Ok(("violation".to_string(), format!("空节：{}", empties.join("、"))));
            }
            Ok(("pass".to_string(), String::new()))
        }
        "section_has_matches" => {
            let sec = p_str(op, "section")?;
            let pattern = p_str(op, "pattern")?;
            let min = p_u64(op, "min")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            let Some(body) = section(text, sec, 2) else {
                return Ok(("missing".to_string(), format!("缺节位：{sec}")));
            };
            let re = compile(pattern, true)?;
            if re.find_iter(&body).count() >= min as usize {
                Ok(("pass".to_string(), String::new()))
            } else {
                Ok(("missing".to_string(), format!("节内匹配不足 {min}：{sec}")))
            }
        }
        "pattern_unique" => {
            let pattern = p_str(op, "pattern")?;
            let format = p_str(op, "format")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            let re = compile(pattern, true)?;
            let fre = compile(&format!(r"\A(?:{format})\z"), false)?;
            let flat = findall_strings(&re, text);
            let mut bad: BTreeSet<String> = BTreeSet::new();
            for x in &flat {
                if !fre.is_match(x) {
                    bad.insert(x.clone());
                }
            }
            if !bad.is_empty() {
                let head: Vec<String> = bad.iter().take(4).cloned().collect();
                return Ok(("violation".to_string(), format!("编号缺形：{}", head.join("、"))));
            }
            let mut dupes: Vec<String> = vec![];
            for (i, x) in flat.iter().enumerate() {
                if flat[i + 1..].contains(x) && !dupes.contains(x) {
                    dupes.push(x.clone());
                }
            }
            dupes.sort();
            if !dupes.is_empty() {
                return Ok(("violation".to_string(), format!("重号：{}", dupes.join("、"))));
            }
            Ok(("pass".to_string(), String::new()))
        }
        "clause_body_has" => {
            let clause_pattern = p_str(op, "clause_pattern")?;
            let token = p_str(op, "token")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            for block in blocks(text, clause_pattern)? {
                // 围堰 block.find("\n") 无换行时 -1 → block[-1:] 取末字（机械同构）。
                let body: String = match block.find('\n') {
                    Some(p) => block[p..].to_string(),
                    None => block.chars().last().map(|c| c.to_string()).unwrap_or_default(),
                };
                if !body.contains(token) {
                    let rid = match_at_start(clause_pattern, &block)?;
                    return Ok((
                        "violation".to_string(),
                        format!("缺强制词：条款「{}」正文缺 token", trunc(&rid, 40)),
                    ));
                }
            }
            Ok(("pass".to_string(), String::new()))
        }
        "clauses_with_marker" => {
            let clause_pattern = p_str(op, "clause_pattern")?;
            let marker = p_str(op, "marker")?;
            let in_sections = p_strlist(op, "in_sections")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            for sec in &in_sections {
                let Some(body) = section(text, sec, 2) else {
                    continue;
                };
                for block in blocks(&body, clause_pattern)? {
                    if !block.contains(marker) {
                        let rid = match_at_start(clause_pattern, &block)?;
                        return Ok((
                            "missing".to_string(),
                            format!("标记缺失：节「{sec}」条款「{}」", trunc(&rid, 40)),
                        ));
                    }
                }
            }
            Ok(("pass".to_string(), String::new()))
        }
        "clause_has_children" => {
            let clause_pattern = p_str(op, "clause_pattern")?;
            let child_pattern = p_str(op, "child_pattern")?;
            let min = p_u64(op, "min")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            for block in blocks(text, clause_pattern)? {
                let body = match block.find('\n') {
                    Some(p) => block[p..].to_string(),
                    None => String::new(),
                };
                let kids = blocks(&body, child_pattern)?;
                if kids.len() < min as usize {
                    let rid = match_at_start(clause_pattern, &block)?;
                    return Ok((
                        "missing".to_string(),
                        format!("无子块：条款「{}」子块数 {}", trunc(&rid, 40), kids.len()),
                    ));
                }
            }
            Ok(("pass".to_string(), String::new()))
        }
        "child_has_markers" => {
            let child_pattern = p_str(op, "child_pattern")?;
            let markers = p_strlist(op, "markers")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            for kid in blocks(text, child_pattern)? {
                let kid_id = trunc(kid.split('\n').next().unwrap_or(""), 40);
                let lacking: Vec<&str> = markers
                    .iter()
                    .map(|m| m.as_str())
                    .filter(|m| !kid.contains(m))
                    .collect();
                if !lacking.is_empty() {
                    return Ok((
                        "missing".to_string(),
                        format!("标记缺席：{kid_id} 缺 {}", lacking.join("、")),
                    ));
                }
            }
            Ok(("pass".to_string(), String::new()))
        }
        "child_line_matches" => {
            let child_pattern = p_str(op, "child_pattern")?;
            let line_prefix = p_str(op, "line_prefix")?;
            let must_match = p_strlist(op, "must_match")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            for kid in blocks(text, child_pattern)? {
                let kid_id = trunc(kid.split('\n').next().unwrap_or(""), 40);
                let line = kid
                    .lines()
                    .find(|ln| ln.trim().starts_with(line_prefix));
                let Some(line) = line else {
                    return Ok((
                        "missing".to_string(),
                        format!(
                            "行缺席：{kid_id} 缺「{}」行",
                            line_prefix.trim_matches(['：', ':'])
                        ),
                    ));
                };
                for pat in &must_match {
                    let re = compile(pat, false)?;
                    if !re.is_match(line) {
                        return Ok((
                            "missing".to_string(),
                            format!("要素缺项：{kid_id} 行缺「{pat}」"),
                        ));
                    }
                }
            }
            Ok(("pass".to_string(), String::new()))
        }
        "list_items_present" => {
            let marker = p_str(op, "marker")?;
            let min = p_u64(op, "min")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            let re = compile(&format!("{}\\s+(.+)$", marker.trim_end()), true)?;
            if re.find_iter(text).count() >= min as usize {
                Ok(("pass".to_string(), String::new()))
            } else {
                Ok(("missing".to_string(), format!("列表项不足 {min}")))
            }
        }
        "list_items_match" => {
            let marker = p_str(op, "marker")?;
            let token = p_str(op, "token")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            let re = compile(&format!("({})\\s+(.+)$", marker.trim_end()), true)?;
            let tre = compile(token, false)?;
            for caps in re.captures_iter(text) {
                let item = caps.get(2).unwrap().as_str();
                if !tre.is_match(item) {
                    return Ok((
                        "broken_ref".to_string(),
                        format!("标记缺席：列表项「{}…」缺 token", trunc(item, 24)),
                    ));
                }
            }
            Ok(("pass".to_string(), String::new()))
        }
        "list_items_field" => {
            let marker = p_str(op, "marker")?;
            let field = p_str(op, "field")?;
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            let re = compile(&format!("({})\\s+(.+)$", marker.trim_end()), true)?;
            for caps in re.captures_iter(text) {
                let item = caps.get(2).unwrap().as_str();
                let idx = item.find(field);
                let rest_ok = idx
                    .map(|i| {
                        !item[i + field.len()..]
                            .trim_matches(|c: char| "（）() ".contains(c))
                            .is_empty()
                    })
                    .unwrap_or(false);
                if !rest_ok {
                    return Ok((
                        "missing".to_string(),
                        format!("字段缺席：列表项「{}…」缺「{field}」", trunc(item, 24)),
                    ));
                }
            }
            Ok(("pass".to_string(), String::new()))
        }
        "reference_closure" => {
            let items_marker = p_str(op, "items_marker")?;
            let ref_pattern = p_str(op, "ref_pattern")?;
            let source = p_str(op, "source")?;
            let defined_pattern = p_str(op, "defined_pattern")?;
            if source != "@reference" {
                return Err(CErr::Pack("reference_closure source 仅支持 @reference".to_string()));
            }
            let Some(reference_text) = ctx.reference else {
                return Err(CErr::Target("--reference 被引用文档缺席".to_string()));
            };
            let Some(text) = text else {
                return Ok(("missing".to_string(), "目标缺席".to_string()));
            };
            let ire = compile(&format!("({})\\s+(.+)$", items_marker.trim_end()), true)?;
            let rre = compile(ref_pattern, false)?;
            let mut refs: BTreeSet<String> = BTreeSet::new();
            for caps in ire.captures_iter(text) {
                let item = caps.get(2).unwrap().as_str();
                // 头注落差二：ref_pattern 多组形语义未对齐，此按单组/无组同构承载。
                for r in findall_strings(&rre, item) {
                    if !r.is_empty() {
                        refs.insert(r);
                    }
                }
            }
            let dre = compile(defined_pattern, false)?;
            let ngroups = dre.captures_len().saturating_sub(1);
            let mut defined: BTreeSet<String> = BTreeSet::new();
            for caps in dre.captures_iter(reference_text) {
                if ngroups > 0 {
                    for i in 1..=ngroups {
                        if let Some(g) = caps.get(i) {
                            defined.insert(g.as_str().to_string());
                        }
                    }
                } else {
                    defined.insert(caps.get(0).unwrap().as_str().to_string());
                }
            }
            let hanging: Vec<String> = refs
                .iter()
                .filter(|r| !defined.contains(*r))
                .cloned()
                .collect();
            if !hanging.is_empty() {
                let head: Vec<String> = hanging.into_iter().take(6).collect();
                return Ok(("broken_ref".to_string(), format!("引用悬空：{}", head.join("、"))));
            }
            Ok(("pass".to_string(), String::new()))
        }
        _ => Err(CErr::Pack(format!("词汇表外操作：{name}"))),
    }
}

// ---------- 包校验与求值 ----------

fn validate_pack(pack: &Value) -> Result<(), CErr> {
    let obj = pack.as_object().ok_or_else(|| CErr::Pack("包非对象".to_string()))?;
    for key in ["pack", "version", "doc_class", "rules"] {
        if !obj.contains_key(key) {
            return Err(CErr::Pack(format!("包缺键：{key}")));
        }
    }
    let rules = obj["rules"]
        .as_array()
        .ok_or_else(|| CErr::Pack("包 rules 须为数组".to_string()))?;
    for rule in rules {
        let check = rule
            .get("check")
            .ok_or_else(|| {
                CErr::Pack(format!(
                    "规则缺机器字段：{}",
                    rule.get("id").and_then(|v| v.as_str()).unwrap_or("?")
                ))
            })?
            .as_object()
            .ok_or_else(|| CErr::Pack("check 须为对象".to_string()))?;
        let ops = check
            .get("ops")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for op in &ops {
            let name = op.get("op").and_then(|v| v.as_str()).unwrap_or("");
            if !VOCAB.contains(&name) && !COMBINATORS.contains(&name) {
                return Err(CErr::Pack(format!(
                    "词汇表外操作：{name}（{}）",
                    rule.get("id").and_then(|v| v.as_str()).unwrap_or("?")
                )));
            }
        }
    }
    Ok(())
}

fn evaluate(pack: &Value, target_text: &str, reference_text: Option<&str>) -> Result<Value, CErr> {
    validate_pack(pack)?;
    let ctx = Ctx { text: Some(target_text), reference: reference_text };
    let mut results: Vec<Value> = vec![];
    let mut findings: Vec<Value> = vec![];
    for rule in pack["rules"].as_array().unwrap() {
        let rid = rule.get("id").and_then(|v| v.as_str()).unwrap_or("?").to_string();
        let mut state = "pass".to_string();
        let mut detail = String::new();
        'ops: for op in rule["check"]
            .get("ops")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default()
        {
            let (s, d) = run_op(&op, &ctx)?;
            state = s;
            detail = d;
            if state != "pass" {
                break 'ops;
            }
        }
        results.push(json!({"rule": rid, "state": state}));
        if state != "pass" {
            let hint = rule
                .get("failure_location")
                .and_then(|f| f.get("hint"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            findings.push(json!({"rule": rid, "state": state, "detail": detail, "hint": hint}));
        }
    }
    Ok(json!({
        "engine": "checker",
        "engine_version": ENGINE_VERSION,
        "pack": pack["pack"],
        "pack_version": pack["version"],
        "doc_class": pack.get("doc_class").cloned().unwrap_or(Value::Null),
        "verdict": if findings.is_empty() { "pass" } else { "fail" },
        "results": results,
        "findings": findings,
    }))
}

// ---------- CLI ----------

fn fail2(payload: Value) -> ! {
    eprintln!("{}", py_json(&payload, None, false));
    exit(2)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut pack_path: Option<String> = None;
    let mut reference: Option<String> = None;
    let mut target: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].clone();
        match a.as_str() {
            "--pack" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("用法错: --pack 缺值");
                    exit(2);
                }
                pack_path = Some(args[i].clone());
            }
            "--reference" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("用法错: --reference 缺值");
                    exit(2);
                }
                reference = Some(args[i].clone());
            }
            s if s.starts_with("--") => {
                eprintln!("用法错: 未知旗标: {s}");
                exit(2);
            }
            _ => {
                if target.is_some() {
                    eprintln!("用法错: 目标至多一个");
                    exit(2);
                }
                target = Some(a);
            }
        }
        i += 1;
    }
    let pack_path = match pack_path {
        Some(p) => p,
        None => {
            eprintln!("用法错: --pack 必填");
            exit(2);
        }
    };
    let target_path = match target {
        Some(t) => t,
        None => {
            eprintln!("用法错: 缺受检文档");
            exit(2);
        }
    };

    let pack_text = match fs::read_to_string(Path::new(&pack_path)) {
        Ok(t) => t,
        Err(_) => fail2(json!({"error": "包文件缺席", "pack": pack_path})),
    };
    let pack: Value = match serde_json::from_str(&pack_text) {
        Ok(v) => v,
        Err(e) => fail2(json!({"error": "包 JSON 不可解析", "detail": e.to_string()})),
    };
    if !Path::new(&target_path).exists() {
        fail2(json!({"error": "目标缺席", "target": target_path}));
    }
    let target_text = match fs::read_to_string(Path::new(&target_path)) {
        Ok(t) => t,
        Err(e) => fail2(json!({"error": "目标不可读", "detail": e.to_string()})),
    };
    let reference_text = match &reference {
        Some(r) => {
            if !Path::new(r).exists() {
                fail2(json!({"error": "被引用文档缺席", "reference": r}));
            }
            match fs::read_to_string(Path::new(r)) {
                Ok(t) => Some(t),
                Err(e) => fail2(json!({"error": "目标不可读", "detail": e.to_string()})),
            }
        }
        None => None,
    };

    let report = match evaluate(&pack, &target_text, reference_text.as_deref()) {
        Ok(r) => r,
        Err(CErr::Pack(e)) => fail2(json!({"error": "包非法", "detail": e})),
        Err(CErr::Target(e)) => fail2(json!({"error": "引用目标异常", "detail": e})),
    };
    let code = if report["verdict"] == "pass" { 0 } else { 1 };
    println!("{}", py_json(&report, Some(1), true));
    exit(code);
}
