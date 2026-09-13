//! 孵化回路契约校验器（checker）引擎 bin —— lease-mergeleg6-parallel 簇H 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/incubation/checker（src/checker/：cli.py 封套、
//! engine.py 谓词引擎、pack.py 包装载），只读对表移植，围堰源码零改动。
//! 判定全是包数据驱动的谓词求值（正则、结构、行定位），零模型零语义裁量；
//! 锁步无状态，同参双跑 stdout 逐字节一致；只读零写入，落笔位仅 stdout。
//!
//! CLI：`incubation --pack <包文件或包目录> [--pack ...]
//!       [--reference <被引用文档> ...] <受检目标>...`
//! 退出码三值：0 = 合规、1 = 违规（发现带三态 missing/violation/broken_ref、
//! 行号与规则归属）、2 = 工具自身异常（目标缺席、包非法、被引用文档缺席或
//! 不可解码）。引擎版本戳取环境变量 SIH_CHECKER_ENGINE_VERSION，缺省
//! 0.1.0.workname。封闭操作词汇表 v0 十七操作与 D-4 出处必携位与包族目录
//! manifest 基名配对全移植；引用闭包语料禁自证（被引用文档加其余共验目标，
//! 本目标不入）。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. 正则方言：Rust regex crate 不承 Python 反向引用与环视；含此类模式的包
//!    按编译失败落包非法（退出码二），报文不对齐。
//! 2. 行界按 \n 与行尾 \r 处理，Python splitlines 的 Unicode 行界字
//!    （\u2028、\x85 等）不切行。
//! 3. argparse 用法错（缺 --pack、缺目标、未知旗标、缩写旗标）stderr 短句回报
//!    退出码二，报文形不对齐；围堰 argparse 允许旗标缩写，本件须全拼。
//! 4. 正则与非正则参数的类型宽严：参数须为字符串，非字符串围堰即 Python
//!    崩溃（退出码一），本件归包非法退出码二；level 的 Python isinstance(int)
//!    宽判（bool 兼容）改严格整数判。
//! 5. 报告面无浮点字段；包内 doc_class 非字符串时围堰原样透出，本件落 null。

use regex::Regex;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::exit;
use std::sync::OnceLock;

const DEFAULT_ENGINE_VERSION: &str = "0.1.0.workname";
const TRI_STATES: [&str; 3] = ["missing", "violation", "broken_ref"];

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!(
        "用法: incubation --pack <包文件或包目录> [--pack ...] \
         [--reference <被引用文档>...] <受检目标>..."
    );
    exit(2);
}

fn fail2(payload: Value) -> ! {
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());
    exit(2);
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn heading_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^(#{1,6})\s+(.*?)\s*$").unwrap())
}

/// 标题行解析：返回（级数、标题文本，两侧空白已由正则消化）。
fn heading(line: &str) -> Option<(usize, String)> {
    let c = heading_re().captures(line)?;
    Some((c[1].chars().count(), c[2].to_string()))
}

// ---------- 文本模型（engine.Doc 对表） ----------

struct Doc {
    lines: Vec<String>,
}

/// 字节解码：UTF-8（容 BOM）；不可解码回 None 由存在性规则承接。
fn doc_from_bytes(raw: &[u8]) -> Option<Doc> {
    let bytes = if raw.starts_with(&[0xEF, 0xBB, 0xBF]) { &raw[3..] } else { raw };
    let text = std::str::from_utf8(bytes).ok()?;
    let mut lines: Vec<&str> = text.split('\n').collect();
    if text.ends_with('\n') {
        lines.pop();
    }
    Some(Doc {
        lines: lines
            .into_iter()
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect(),
    })
}

// ---------- 结构基元（engine._heading_blocks / _pattern_blocks 对表） ----------

fn heading_blocks(doc: &Doc, level: Option<usize>, name: Option<&str>) -> Vec<(usize, usize, String)> {
    let mut blocks = vec![];
    let n = doc.lines.len();
    let mut i = 0usize;
    while i < n {
        let hit = heading(&doc.lines[i]).and_then(|(lvl, text)| {
            let lvl_ok = level.map_or(true, |l| l == lvl);
            let name_ok = name.map_or(true, |nm| nm == text);
            if lvl_ok && name_ok { Some((lvl, text)) } else { None }
        });
        if let Some((lvl, text)) = hit {
            let mut j = i + 1;
            while j < n {
                if let Some((l2, _)) = heading(&doc.lines[j]) {
                    if l2 <= lvl {
                        break;
                    }
                }
                j += 1;
            }
            blocks.push((i, j, text));
            i = j;
        } else {
            i += 1;
        }
    }
    blocks
}

/// 模式块：起于匹配行，止于次匹配行或级数不大于起始行标题级数的标题行或 hi。
/// pat 传锚定编译形（Python pattern.match 语义）。
fn pattern_blocks(doc: &Doc, pat: &Regex, lo: usize, hi: usize) -> Vec<(usize, usize)> {
    let mut blocks = vec![];
    let mut i = lo;
    while i < hi {
        if pat.is_match(&doc.lines[i]) {
            let lvl = heading(&doc.lines[i]).map(|(l, _)| l);
            let mut j = i + 1;
            while j < hi {
                if pat.is_match(&doc.lines[j]) {
                    break;
                }
                if let Some((l2, _)) = heading(&doc.lines[j]) {
                    if let Some(lvl) = lvl {
                        if l2 <= lvl {
                            break;
                        }
                    }
                }
                j += 1;
            }
            blocks.push((i, j));
            i = j;
        } else {
            i += 1;
        }
    }
    blocks
}

/// 取锚定模式对行匹配的第一捕获组作定位引用；无组或组未参与回 None。
fn cap(pat: &Regex, line: Option<&str>) -> Option<String> {
    let line = line?;
    let c = pat.captures(line)?;
    c.get(1).map(|m| m.as_str().to_string())
}

/// 条款定位引用：自身模式有组取组；无组时复用包内编号唯一型操作带组模式提取。
fn clause_ref(pack: &Pack, pat: &Regex, line: Option<&str>) -> Option<String> {
    if let Some(r) = cap(pat, line) {
        return Some(r);
    }
    if let Some(line) = line {
        for p in &pack.id_pats {
            if let Some(c) = p.captures(line) {
                if let Some(g) = c.get(1) {
                    if !g.is_empty() {
                        return Some(g.as_str().to_string());
                    }
                }
            }
        }
    }
    None
}

// ---------- 操作词汇表 v0（pack.VOCAB 对表） ----------

fn vocab_params(name: &str) -> Option<&'static [&'static str]> {
    Some(match name {
        "exists_readable" => &[],
        "heading_present" => &["level"],
        "sections_present" => &["level", "names"],
        "section_nonempty" => &["level", "name"],
        "sections_all_nonempty" => &["level"],
        "section_has_matches" => &["section", "pattern"],
        "pattern_unique" => &["pattern", "format"],
        "clause_body_has" => &["clause_pattern", "token"],
        "clauses_with_marker" => &["clause_pattern", "marker", "in_sections"],
        "clause_has_children" => &["clause_pattern", "child_pattern"],
        "child_has_markers" => &["child_pattern", "markers"],
        "child_line_matches" => &["child_pattern", "line_prefix", "must_match"],
        "list_items_present" => &["marker"],
        "list_items_match" => &["marker", "token"],
        "list_items_field" => &["marker", "field"],
        "reference_closure" => &["items_marker", "ref_pattern", "source", "defined_pattern"],
        "any_of" => &["branches"],
        _ => return None,
    })
}

// ---------- 操作树：装载即编译（pack._validate_op 对表） ----------

#[derive(Clone)]
enum RxKind {
    /// search：任意位置匹配（Python pattern.search）。
    Search,
    /// match：行首锚定（Python pattern.match）。
    Match,
    /// fullmatch：整行完全匹配（Python pattern.fullmatch）。
    Full,
}

#[allow(clippy::enum_variant_names)]
enum OpNode {
    ExistsReadable,
    HeadingPresent { level: usize },
    SectionsPresent { level: usize, names: Vec<String>, ordered: bool },
    SectionNonempty { level: usize, name: String },
    SectionsAllNonempty { level: usize },
    SectionHasMatches { section: String, pattern: Regex, min: i64 },
    PatternUnique { pattern: Regex, format: Regex },
    ClauseBodyHas { clause: Regex, token: String },
    ClausesWithMarker { clause: Regex, marker: String, in_sections: Vec<String> },
    ClauseHasChildren { clause: Regex, child: Regex, min: i64 },
    ChildHasMarkers { child: Regex, clause: Option<Regex>, markers: Vec<String> },
    ChildLineMatches { child: Regex, clause: Option<Regex>, prefix: String, musts: Vec<Regex> },
    ListItemsPresent { marker: Regex, min: i64 },
    ListItemsMatch { marker: Regex, token: Regex },
    ListItemsField { marker: Regex, field: String },
    ReferenceClosure { items: Regex, refs: Regex, defined: Regex },
    AnyOf { branches: Vec<Vec<OpNode>> },
}

fn compile_rx(
    kind: RxKind,
    name: &str,
    rid: &str,
    param: &str,
    pattern: &str,
    path: &str,
) -> Result<Regex, String> {
    let src = match kind {
        RxKind::Search => pattern.to_string(),
        RxKind::Match => format!("^(?:{pattern})"),
        RxKind::Full => format!("^(?:{pattern})$"),
    };
    Regex::new(&src)
        .map_err(|e| format!("规则 {rid} 操作 {name} 正则非法 {param}: {e}: {path}"))
}

fn get_level(obj: &Map<String, Value>, rid: &str, name: &str, path: &str) -> Result<usize, String> {
    match obj.get("level").and_then(|v| v.as_i64()) {
        Some(l) if (1..=6).contains(&l) => Ok(l as usize),
        _ => Err(format!("规则 {rid} 操作 {name} 参数 level 须为 1 至 6 整数: {path}")),
    }
}

fn get_min(obj: &Map<String, Value>, default: i64) -> i64 {
    obj.get("min").and_then(|v| v.as_i64()).unwrap_or(default)
}

fn get_strings(obj: &Map<String, Value>, key: &str) -> Vec<String> {
    obj.get(key)
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default()
}

fn get_regex_list(
    obj: &Map<String, Value>,
    key: &str,
    name: &str,
    rid: &str,
    path: &str,
) -> Result<Vec<Regex>, String> {
    let bad = || format!("规则 {rid} 操作 {name} 参数 {key} 须为字符串清单: {path}");
    let arr = obj.get(key).and_then(|v| v.as_array()).ok_or_else(bad)?;
    let mut out = vec![];
    for x in arr {
        let Some(s) = x.as_str() else { return Err(bad()) };
        out.push(compile_rx(RxKind::Search, name, rid, key, s, path)?);
    }
    Ok(out)
}

fn build_op(op: &Value, path: &str, rid: &str) -> Result<OpNode, String> {
    let Some(obj) = op.as_object() else {
        return Err(format!("规则 {rid} 操作须为对象: {path}"));
    };
    let Some(opname) = obj.get("op").and_then(|v| v.as_str()).filter(|s| vocab_params(s).is_some())
    else {
        let disp = match obj.get("op") {
            Some(Value::String(s)) => s.clone(),
            Some(Value::Null) | None => "None".to_string(),
            Some(v) => v.to_string(),
        };
        return Err(format!("规则 {rid} 词汇表外操作: {disp}: {path}"));
    };
    for p in vocab_params(opname).unwrap() {
        if !obj.contains_key(*p) {
            return Err(format!("规则 {rid} 操作 {opname} 缺参数 {p}: {path}"));
        }
    }
    if opname == "any_of" {
        let bad = || format!("规则 {rid} any_of 缺 branches: {path}");
        let arr = obj.get("branches").and_then(|v| v.as_array()).ok_or_else(bad)?;
        if arr.is_empty() {
            return Err(bad());
        }
        let mut branches = vec![];
        for b in arr {
            let subs: Vec<&Value> = if b.is_array() { b.as_array().unwrap().iter().collect() } else { vec![b] };
            let mut built = vec![];
            for s in subs {
                built.push(build_op(s, path, rid)?);
            }
            branches.push(built);
        }
        return Ok(OpNode::AnyOf { branches });
    }
    if opname == "reference_closure" && obj.get("source").and_then(|v| v.as_str()) != Some("@reference") {
        return Err(format!("规则 {rid} 引用闭包 source 仅承 @reference: {path}"));
    }
    // 任选正则参（子块类操作的 clause_pattern）缺席即跳过编译。
    let optional_clause = |obj: &Map<String, Value>| -> Result<Option<Regex>, String> {
        match obj.get("clause_pattern") {
            None => Ok(None),
            Some(v) if v.is_string() => {
                Ok(Some(compile_rx(RxKind::Match, opname, rid, "clause_pattern", v.as_str().unwrap(), path)?))
            }
            Some(_) => Err(format!(
                "规则 {rid} 操作 {opname} 正则非法 clause_pattern: 非字符串: {path}"
            )),
        }
    };
    let node = match opname {
        "exists_readable" => OpNode::ExistsReadable,
        "heading_present" => OpNode::HeadingPresent { level: get_level(obj, rid, opname, path)? },
        "sections_present" => OpNode::SectionsPresent {
            level: get_level(obj, rid, opname, path)?,
            names: get_strings(obj, "names"),
            ordered: obj.get("ordered").and_then(|v| v.as_bool()).unwrap_or(false),
        },
        "section_nonempty" => OpNode::SectionNonempty {
            level: get_level(obj, rid, opname, path)?,
            name: obj.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        },
        "sections_all_nonempty" => {
            OpNode::SectionsAllNonempty { level: get_level(obj, rid, opname, path)? }
        }
        "section_has_matches" => OpNode::SectionHasMatches {
            section: obj.get("section").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            pattern: match obj.get("pattern").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Search, opname, rid, "pattern", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 pattern: 非字符串: {path}")),
            },
            min: get_min(obj, 1),
        },
        "pattern_unique" => {
            let pat_src = obj.get("pattern").and_then(|v| v.as_str()).ok_or_else(|| {
                format!("规则 {rid} 操作 {opname} 正则非法 pattern: 非字符串: {path}")
            })?;
            let fmt_src = obj.get("format").and_then(|v| v.as_str()).ok_or_else(|| {
                format!("规则 {rid} 操作 {opname} 正则非法 format: 非字符串: {path}")
            })?;
            let pattern = compile_rx(RxKind::Search, opname, rid, "pattern", pat_src, path)?;
            let format = compile_rx(RxKind::Full, opname, rid, "format", fmt_src, path)?;
            if pattern.captures_len() - 1 != 1 {
                return Err(format!("规则 {rid} pattern_unique 须恰一捕获组: {path}"));
            }
            OpNode::PatternUnique { pattern, format }
        }
        "clause_body_has" => OpNode::ClauseBodyHas {
            clause: optional_clause(obj)?.expect("clause_pattern 必填已验"),
            token: obj.get("token").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        },
        "clauses_with_marker" => OpNode::ClausesWithMarker {
            clause: optional_clause(obj)?.expect("clause_pattern 必填已验"),
            marker: obj.get("marker").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            in_sections: get_strings(obj, "in_sections"),
        },
        "clause_has_children" => OpNode::ClauseHasChildren {
            clause: optional_clause(obj)?.expect("clause_pattern 必填已验"),
            child: match obj.get("child_pattern").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Match, opname, rid, "child_pattern", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 child_pattern: 非字符串: {path}")),
            },
            min: get_min(obj, 1),
        },
        "child_has_markers" => OpNode::ChildHasMarkers {
            child: match obj.get("child_pattern").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Match, opname, rid, "child_pattern", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 child_pattern: 非字符串: {path}")),
            },
            clause: optional_clause(obj)?,
            markers: get_strings(obj, "markers"),
        },
        "child_line_matches" => OpNode::ChildLineMatches {
            child: match obj.get("child_pattern").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Match, opname, rid, "child_pattern", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 child_pattern: 非字符串: {path}")),
            },
            clause: optional_clause(obj)?,
            prefix: obj.get("line_prefix").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            musts: get_regex_list(obj, "must_match", opname, rid, path)?,
        },
        "list_items_present" => OpNode::ListItemsPresent {
            marker: match obj.get("marker").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Match, opname, rid, "marker", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 marker: 非字符串: {path}")),
            },
            min: get_min(obj, 1),
        },
        "list_items_match" => OpNode::ListItemsMatch {
            marker: match obj.get("marker").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Match, opname, rid, "marker", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 marker: 非字符串: {path}")),
            },
            token: match obj.get("token").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Search, opname, rid, "token", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 token: 非字符串: {path}")),
            },
        },
        "list_items_field" => OpNode::ListItemsField {
            marker: match obj.get("marker").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Match, opname, rid, "marker", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 marker: 非字符串: {path}")),
            },
            field: obj.get("field").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        },
        "reference_closure" => {
            let items = match obj.get("items_marker").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Match, opname, rid, "items_marker", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 items_marker: 非字符串: {path}")),
            };
            let refs = match obj.get("ref_pattern").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Search, opname, rid, "ref_pattern", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 ref_pattern: 非字符串: {path}")),
            };
            let defined = match obj.get("defined_pattern").and_then(|v| v.as_str()) {
                Some(s) => compile_rx(RxKind::Match, opname, rid, "defined_pattern", s, path)?,
                None => return Err(format!("规则 {rid} 操作 {opname} 正则非法 defined_pattern: 非字符串: {path}")),
            };
            for (param, rx) in [("ref_pattern", &refs), ("defined_pattern", &defined)] {
                if rx.captures_len() - 1 > 1 {
                    return Err(format!("规则 {rid} reference_closure {param} 至多一捕获组: {path}"));
                }
            }
            OpNode::ReferenceClosure { items, refs, defined }
        }
        _ => unreachable!("词汇表已在入口过滤"),
    };
    // 数值参数纪律（对表 pack._validate_op 尾段：任意操作带 level/min 键即校验）。
    if obj.contains_key("level") && !matches!(node, OpNode::AnyOf { .. }) {
        get_level(obj, rid, opname, path)?;
    }
    if let Some(m) = obj.get("min") {
        match m.as_i64() {
            Some(v) if v >= 0 => {}
            _ => return Err(format!("规则 {rid} 操作 {opname} 参数 min 须为非负整数: {path}")),
        }
    }
    Ok(node)
}

// ---------- 包装载（pack.py 对表） ----------

struct Rule {
    id: String,
    default_state: String,
    hint: Option<String>,
    ops: Vec<OpNode>,
}

struct Pack {
    name: String,
    version: String,
    doc_class: Option<String>,
    rules: Vec<Rule>,
    d4_ok: bool,
    /// 编号唯一型操作的带组锚定模式集（clause_ref 机械提取位）。
    id_pats: Vec<Regex>,
    path: String,
    file_name: String,
}

struct Family {
    packs: Vec<Pack>,
    /// 目录形配对：目标基名 → 包文件基名（None 值即无配对，施全族）。
    pairing: Option<HashMap<String, Option<String>>>,
}

fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map(|f| f != 0.0).unwrap_or(false),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

/// 出处面 D-4 必携位（pack.Pack._check_d4 对表）。
fn check_d4(data: &Value) -> bool {
    let Some(slot) = data
        .get("provenance")
        .and_then(|p| p.get("d4_anchor_slot"))
        .and_then(|s| s.as_object())
    else {
        return false;
    };
    if slot.get("required").and_then(|v| v.as_bool()) != Some(true) {
        return true;
    }
    slot.get("vectors_home").map(truthy).unwrap_or(false)
}

fn load_pack_file(path: &Path) -> Result<Pack, String> {
    let disp = path.to_string_lossy().into_owned();
    if !path.is_file() {
        return Err(format!("包文件缺席: {disp}"));
    }
    let raw = fs::read(path).map_err(|e| format!("包 JSON 不可解析: {disp}: {e}"))?;
    let text = std::str::from_utf8(&raw)
        .map_err(|e| format!("包 JSON 不可解析: {disp}: {e}"))?;
    let data: Value =
        serde_json::from_str(text).map_err(|e| format!("包 JSON 不可解析: {disp}: {e}"))?;
    let Some(obj) = data.as_object() else {
        return Err(format!("包顶层须为对象: {disp}"));
    };
    let name = obj
        .get("pack")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("包缺 pack 名: {disp}"))?
        .to_string();
    let version = obj
        .get("version")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("包缺版本: {disp}"))?
        .to_string();
    let doc_class = obj.get("doc_class").and_then(|v| v.as_str()).map(|s| s.to_string());
    let rules_raw = obj
        .get("rules")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("包缺判定面 rules 清单: {disp}"))?;
    let mut rules: Vec<Rule> = vec![];
    let mut id_pats: Vec<Regex> = vec![];
    let mut seen_ids: HashSet<&str> = HashSet::new();
    for rule in rules_raw {
        let Some(robj) = rule.as_object() else {
            return Err(format!("规则条目须为对象: {disp}"));
        };
        let Some(rid) = robj.get("id").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) else {
            return Err(format!("规则缺 id: {disp}"));
        };
        if !seen_ids.insert(rid) {
            return Err(format!("规则 id 重号: {rid}: {disp}"));
        }
        let loc = robj
            .get("failure_location")
            .and_then(|v| v.as_object())
            .ok_or_else(|| format!("规则 {rid} 缺三态失败定位声明: {disp}"))?;
        let state = loc
            .get("state")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("规则 {rid} 缺三态失败定位声明: {disp}"))?;
        if !TRI_STATES.contains(&state) {
            return Err(format!("规则 {rid} 缺三态失败定位声明: {disp}"));
        }
        let check = robj
            .get("check")
            .and_then(|v| v.as_object())
            .ok_or_else(|| format!("规则 {rid} 缺 check.ops 机器字段: {disp}"))?;
        let ops_raw = check
            .get("ops")
            .and_then(|v| v.as_array())
            .filter(|o| !o.is_empty())
            .ok_or_else(|| format!("规则 {rid} 缺 check.ops 机器字段: {disp}"))?;
        let mut ops = vec![];
        for op in ops_raw {
            ops.push(build_op(op, &disp, rid)?);
            // id_pats 只取规则顶层 pattern_unique 操作的带组模式（对表 pack.Pack）。
            if let Some(o) = op.as_object() {
                if o.get("op").and_then(|v| v.as_str()) == Some("pattern_unique") {
                    if let Some(p) = o.get("pattern").and_then(|v| v.as_str()) {
                        if let Ok(rx) = compile_rx(RxKind::Match, "pattern_unique", rid, "pattern", p, &disp) {
                            id_pats.push(rx);
                        }
                    }
                }
            }
        }
        rules.push(Rule {
            id: rid.to_string(),
            default_state: state.to_string(),
            hint: loc.get("hint").and_then(|v| v.as_str()).map(|s| s.to_string()),
            ops,
        });
    }
    Ok(Pack {
        name,
        version,
        doc_class,
        rules,
        d4_ok: check_d4(&data),
        id_pats,
        path: disp,
        file_name: path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default(),
    })
}

fn load_family_dir(d: &Path) -> Result<Family, String> {
    let disp = d.to_string_lossy().into_owned();
    let mf = d.join("manifest.json");
    if !mf.is_file() {
        return Err(format!("包族目录缺 manifest.json: {disp}"));
    }
    let raw = fs::read(&mf).map_err(|e| format!("manifest.json 不可解析: {e}"))?;
    let text = std::str::from_utf8(&raw).map_err(|e| format!("manifest.json 不可解析: {e}"))?;
    let manifest: Value =
        serde_json::from_str(text).map_err(|e| format!("manifest.json 不可解析: {e}"))?;
    let pack_files: Vec<String> = manifest
        .get("packs")
        .and_then(|v| v.as_array())
        .filter(|a| !a.is_empty())
        .ok_or_else(|| format!("manifest 缺 packs 清单: {disp}"))?
        .iter()
        .map(|v| {
            v.as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| "manifest packs 清单含非字符串项".to_string())
        })
        .collect::<Result<_, _>>()?;
    let mut packs = vec![];
    for name in &pack_files {
        packs.push(load_pack_file(&d.join(name))?);
    }
    // 寻径配对：packs 与 fixtures 按序平行，基名对基名，先到先得。
    let mut pairing: HashMap<String, Option<String>> = HashMap::new();
    if let Some(fixtures) = manifest.get("fixtures").and_then(|v| v.as_array()) {
        for (i, fx) in fixtures.iter().enumerate() {
            let Some(fx) = fx.as_str() else { continue };
            let base = Path::new(fx)
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| fx.to_string());
            if !pairing.contains_key(&base) {
                pairing.insert(base, pack_files.get(i).cloned());
            }
        }
    }
    Ok(Family { packs, pairing: Some(pairing) })
}

fn load_pack_arg(arg: &str) -> Result<Family, String> {
    let p = Path::new(arg);
    if p.is_dir() {
        return load_family_dir(p);
    }
    if !p.is_file() {
        return Err(format!("包路径缺席: {arg}"));
    }
    Ok(Family { packs: vec![load_pack_file(p)?], pairing: None })
}

impl Family {
    /// 目标落包：目录形按基名配对，无配对即全族施检；单包形施于全部目标。
    fn assign<'a>(&'a self, target: &str) -> Vec<&'a Pack> {
        match &self.pairing {
            None => self.packs.iter().collect(),
            Some(map) => {
                let base = Path::new(target)
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| target.to_string());
                match map.get(&base) {
                    Some(Some(fname)) => match self.packs.iter().find(|p| &p.file_name == fname) {
                        Some(pk) => vec![pk],
                        None => self.packs.iter().collect(),
                    },
                    _ => self.packs.iter().collect(),
                }
            }
        }
    }
}

// ---------- 谓词求值（engine.py 对表） ----------

/// 命中三元组：（态覆盖或 None、行号或 None、编号引用或 None）。
type Hit = (Option<String>, Option<i64>, Option<String>);

fn child_blocks(doc: &Doc, clause: Option<&Regex>, child: &Regex) -> Vec<(usize, usize)> {
    match clause {
        None => pattern_blocks(doc, child, 0, doc.lines.len()),
        Some(cp) => {
            let mut res = vec![];
            for (s, e) in pattern_blocks(doc, cp, 0, doc.lines.len()) {
                res.extend(pattern_blocks(doc, child, s + 1, e));
            }
            res
        }
    }
}

fn eval_op(op: &OpNode, doc: &Doc, corpus: &[&Doc], pack: &Pack) -> Vec<Hit> {
    match op {
        // 可读性在目标预读层承接：文本在执即已存在可读，此操作恒过。
        OpNode::ExistsReadable => vec![],
        OpNode::HeadingPresent { level } => {
            for line in &doc.lines {
                if let Some((l, t)) = heading(line) {
                    if l == *level && !t.trim().is_empty() {
                        return vec![];
                    }
                }
            }
            vec![(None, None, None)]
        }
        OpNode::SectionsPresent { level, names, ordered } => {
            let mut pos: HashMap<String, usize> = HashMap::new();
            for (idx, line) in doc.lines.iter().enumerate() {
                if let Some((l, t)) = heading(line) {
                    if l == *level && names.iter().any(|nm| nm == &t) && !pos.contains_key(&t) {
                        pos.insert(t, idx);
                    }
                }
            }
            let mut out: Vec<Hit> = names
                .iter()
                .filter(|nm| !pos.contains_key(*nm))
                .map(|nm| (None, None, Some(nm.clone())))
                .collect();
            if *ordered && out.is_empty() {
                for w in names.windows(2) {
                    if pos[&w[0]] > pos[&w[1]] {
                        out.push((
                            Some("violation".to_string()),
                            Some(pos[&w[1]] as i64 + 1),
                            Some(w[1].clone()),
                        ));
                    }
                }
            }
            out
        }
        OpNode::SectionNonempty { level, name } => {
            let blocks = heading_blocks(doc, Some(*level), Some(name.as_str()));
            if blocks.is_empty() {
                // 节整段缺席已在在位规则申报，此处仍落一笔不静默。
                return vec![(None, None, Some(name.clone()))];
            }
            let (s, e, _) = blocks[0];
            if !doc.lines[s + 1..e].iter().any(|l| !l.trim().is_empty()) {
                vec![(None, Some(s as i64 + 1), Some(name.clone()))]
            } else {
                vec![]
            }
        }
        OpNode::SectionsAllNonempty { level } => {
            let mut out = vec![];
            for (s, e, text) in heading_blocks(doc, Some(*level), None) {
                if !doc.lines[s + 1..e].iter().any(|l| !l.trim().is_empty()) {
                    out.push((None, Some(s as i64 + 1), Some(text)));
                }
            }
            out
        }
        OpNode::SectionHasMatches { section, pattern, min } => {
            let mut count = 0usize;
            for (s, e, _) in heading_blocks(doc, None, Some(section.as_str())) {
                for i in s..e {
                    if pattern.is_match(&doc.lines[i]) {
                        count += 1;
                    }
                }
            }
            if count as i64 >= *min {
                vec![]
            } else {
                vec![(None, None, Some(section.clone()))]
            }
        }
        OpNode::PatternUnique { pattern, format } => {
            let mut out = vec![];
            let mut seen: HashMap<String, usize> = HashMap::new();
            for (idx, line) in doc.lines.iter().enumerate() {
                let Some(m) = pattern.captures(line) else { continue };
                let val = m.get(1).map(|g| g.as_str().to_string());
                match &val {
                    None => out.push((Some("violation".to_string()), Some(idx as i64 + 1), None)),
                    Some(v) => {
                        if !format.is_match(v) || seen.contains_key(v) {
                            out.push((
                                Some("violation".to_string()),
                                Some(idx as i64 + 1),
                                val.clone(),
                            ));
                        } else {
                            seen.insert(v.clone(), idx);
                        }
                    }
                }
            }
            out
        }
        OpNode::ClauseBodyHas { clause, token } => {
            let mut out = vec![];
            for (s, e) in pattern_blocks(doc, clause, 0, doc.lines.len()) {
                let body = doc.lines[s + 1..e].join("\n");
                if !body.contains(token.as_str()) {
                    let r = clause_ref(pack, clause, doc.lines.get(s).map(|s| s.as_str()));
                    out.push((None, Some(s as i64 + 1), r));
                }
            }
            out
        }
        OpNode::ClausesWithMarker { clause, marker, in_sections } => {
            let mut ranges: Vec<(usize, usize)> = vec![];
            for nm in in_sections {
                for (s, e, _) in heading_blocks(doc, None, Some(nm.as_str())) {
                    ranges.push((s, e));
                }
            }
            let mut out = vec![];
            for (s, e) in pattern_blocks(doc, clause, 0, doc.lines.len()) {
                if !ranges.iter().any(|(rs, re)| s >= *rs && s < *re) {
                    continue;
                }
                if !doc.lines[s..e].iter().any(|l| l.contains(marker.as_str())) {
                    let r = clause_ref(pack, clause, doc.lines.get(s).map(|s| s.as_str()));
                    out.push((None, Some(s as i64 + 1), r));
                }
            }
            out
        }
        OpNode::ClauseHasChildren { clause, child, min } => {
            let mut out = vec![];
            for (s, e) in pattern_blocks(doc, clause, 0, doc.lines.len()) {
                let n = doc.lines[s + 1..e].iter().filter(|l| child.is_match(l)).count();
                if (n as i64) < *min {
                    let r = clause_ref(pack, clause, doc.lines.get(s).map(|s| s.as_str()));
                    out.push((None, Some(s as i64 + 1), r));
                }
            }
            out
        }
        OpNode::ChildHasMarkers { child, clause, markers } => {
            let mut out = vec![];
            for (s, e) in child_blocks(doc, clause.as_ref(), child) {
                let text = doc.lines[s..e].join("\n");
                for mk in markers {
                    if !text.contains(mk.as_str()) {
                        let r = clause_ref(pack, child, doc.lines.get(s).map(|s| s.as_str()));
                        out.push((None, Some(s as i64 + 1), r));
                    }
                }
            }
            out
        }
        OpNode::ChildLineMatches { child, clause, prefix, musts } => {
            let mut out = vec![];
            for (s, e) in child_blocks(doc, clause.as_ref(), child) {
                let mut ok = false;
                for i in s..e {
                    let ln = &doc.lines[i];
                    if ln.starts_with(prefix.as_str()) && musts.iter().all(|rx| rx.is_match(ln)) {
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    let r = clause_ref(pack, child, doc.lines.get(s).map(|s| s.as_str()));
                    out.push((None, Some(s as i64 + 1), r));
                }
            }
            out
        }
        OpNode::ListItemsPresent { marker, min } => {
            let n = doc.lines.iter().filter(|l| marker.is_match(l)).count();
            if n as i64 >= *min {
                vec![]
            } else {
                vec![(None, None, None)]
            }
        }
        OpNode::ListItemsMatch { marker, token } => doc
            .lines
            .iter()
            .enumerate()
            .filter(|(_, l)| marker.is_match(l) && !token.is_match(l))
            .map(|(i, _)| (None, Some(i as i64 + 1), None))
            .collect(),
        OpNode::ListItemsField { marker, field } => {
            let mut out = vec![];
            for (i, l) in doc.lines.iter().enumerate() {
                if !marker.is_match(l) {
                    continue;
                }
                let empty_after = !l.contains(field.as_str())
                    || l.splitn(2, field.as_str())
                        .nth(1)
                        .map(|rest| rest.trim().is_empty())
                        .unwrap_or(true);
                if empty_after {
                    out.push((None, Some(i as i64 + 1), None));
                }
            }
            out
        }
        OpNode::ReferenceClosure { items, refs, defined } => {
            let mut defined_set: BTreeSet<String> = BTreeSet::new();
            for d in corpus {
                for line in &d.lines {
                    if let Some(m) = defined.captures(line) {
                        // 至多一捕获组：有组取组，无组取全匹配。
                        let g =
                            if defined.captures_len() - 1 == 1 { m.get(1) } else { m.get(0) };
                        if let Some(g) = g {
                            if !g.is_empty() {
                                defined_set.insert(g.as_str().to_string());
                            }
                        }
                    }
                }
            }
            let mut out = vec![];
            for (i, l) in doc.lines.iter().enumerate() {
                if !items.is_match(l) {
                    continue;
                }
                for m in refs.captures_iter(l) {
                    let g = if refs.captures_len() - 1 == 1 { m.get(1) } else { m.get(0) };
                    if let Some(g) = g {
                        let v = g.as_str().to_string();
                        if !defined_set.contains(&v) {
                            out.push((None, Some(i as i64 + 1), Some(v)));
                        }
                    }
                }
            }
            out
        }
        OpNode::AnyOf { branches } => {
            let mut first_fail: Option<Vec<Hit>> = None;
            for subs in branches {
                let mut hits = vec![];
                for sub in subs {
                    hits.extend(eval_op(sub, doc, corpus, pack));
                }
                if hits.is_empty() {
                    return vec![];
                }
                if first_fail.is_none() {
                    first_fail = Some(hits);
                }
            }
            first_fail.unwrap_or_default()
        }
    }
}

fn mk(rule: &Rule, hit: Hit) -> Value {
    let (override_state, line, ref_) = hit;
    json!({
        "rule": rule.id,
        "state": override_state.unwrap_or_else(|| rule.default_state.clone()),
        "line": line,
        "ref": ref_,
        "hint": rule.hint,
    })
}

fn eval_rule(rule: &Rule, doc: Option<&Doc>, corpus: &[&Doc], pack: &Pack) -> Vec<Value> {
    let Some(doc) = doc else {
        // 目标不可读：仅存在可读类规则落缺件，其余规则无文本可求值。
        if !rule.ops.is_empty() && rule.ops.iter().all(|o| matches!(o, OpNode::ExistsReadable)) {
            return vec![mk(rule, (None, None, None))];
        }
        return vec![];
    };
    let mut hits = vec![];
    for op in &rule.ops {
        hits.extend(eval_op(op, doc, corpus, pack));
    }
    hits.into_iter().map(|h| mk(rule, h)).collect()
}

fn evaluate_pack(pack: &Pack, doc: Option<&Doc>, corpus: &[&Doc]) -> Vec<Value> {
    let mut out = vec![];
    if !pack.d4_ok {
        out.push(json!({
            "rule": "D4-ANCHOR",
            "state": "violation",
            "line": null,
            "ref": null,
            "hint": "包出处面缺 D-4 基线向量挂锚位或挂锚位残缺（登记件 D-4 必携位）",
        }));
    }
    for rule in &pack.rules {
        out.extend(eval_rule(rule, doc, corpus, pack));
    }
    out
}

// ---------- CLI 封套（cli.py 对表） ----------

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut pack_args: Vec<String> = vec![];
    let mut ref_args: Vec<String> = vec![];
    let mut targets: Vec<String> = vec![];
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].clone();
        let (flag, inline) = match a.split_once('=') {
            Some((f, v)) if f.starts_with("--") => (f.to_string(), Some(v.to_string())),
            _ => (a.clone(), None),
        };
        match flag.as_str() {
            "--pack" => {
                if inline.is_some() {
                    pack_args.push(inline.unwrap());
                } else {
                    i += 1;
                    if i >= args.len() {
                        usage_fail("--pack 缺值");
                    }
                    pack_args.push(args[i].clone());
                }
            }
            "--reference" => {
                if inline.is_some() {
                    ref_args.push(inline.unwrap());
                } else {
                    i += 1;
                    if i >= args.len() {
                        usage_fail("--reference 缺值");
                    }
                    ref_args.push(args[i].clone());
                }
            }
            _ => {
                if flag.starts_with("--") {
                    usage_fail(&format!("未知旗标: {a}"));
                }
                targets.push(a);
            }
        }
        i += 1;
    }
    if pack_args.is_empty() {
        usage_fail("缺 --pack（必填，可重复）");
    }
    if targets.is_empty() {
        usage_fail("缺受检目标");
    }

    let engine_version = std::env::var("SIH_CHECKER_ENGINE_VERSION")
        .unwrap_or_else(|_| DEFAULT_ENGINE_VERSION.to_string());

    // 目标预检：缺席或非常规文件即工具自身异常（先于装包，对表 cli.py 序）。
    for t in &targets {
        if !Path::new(t).is_file() {
            fail2(json!({"error": "目标缺席或非常规文件", "target": t}));
        }
    }

    // 装包与被引用文档：包非法硬失败。
    let mut families: Vec<Family> = vec![];
    for a in &pack_args {
        match load_pack_arg(a) {
            Ok(f) => families.push(f),
            Err(e) => fail2(json!({"error": "包非法", "detail": e})),
        }
    }
    let mut ref_docs: Vec<Doc> = vec![];
    for r in &ref_args {
        let p = Path::new(r);
        if !p.is_file() {
            fail2(json!({"error": "被引用文档缺席", "reference": r}));
        }
        match fs::read(p) {
            Ok(raw) => match doc_from_bytes(&raw) {
                Some(d) => ref_docs.push(d),
                None => fail2(json!({
                    "error": "被引用文档不可读",
                    "detail": format!("被引用文档不可解码: {r}"),
                })),
            },
            Err(e) => fail2(json!({"error": "被引用文档缺席", "reference": r, "detail": e.to_string()})),
        }
    }

    // 目标预读：内容摘要入报告作凭据；不可解码落 None 由存在性规则承接。
    let mut target_docs: Vec<Option<Doc>> = vec![];
    let mut target_sha: Vec<String> = vec![];
    for t in &targets {
        let raw = fs::read(t).unwrap_or_default();
        target_sha.push(sha256_hex(&raw));
        target_docs.push(doc_from_bytes(&raw));
    }

    // 包清单去重（按包路径，装载序）。
    let mut packs_ordered: Vec<&Pack> = vec![];
    let mut seen_paths: HashSet<&str> = HashSet::new();
    for fam in &families {
        for pk in &fam.packs {
            if seen_paths.insert(pk.path.as_str()) {
                packs_ordered.push(pk);
            }
        }
    }

    let mut findings: Vec<Value> = vec![];
    let mut targets_report: Vec<Value> = vec![];
    for (ti, t) in targets.iter().enumerate() {
        let mut assigned: Vec<&Pack> = vec![];
        for fam in &families {
            assigned.extend(fam.assign(t));
        }
        let doc = target_docs[ti].as_ref();
        // 引用闭包语料：被引用文档加其余共验目标，本目标不入（禁自证）。
        let mut corpus: Vec<&Doc> = ref_docs.iter().collect();
        for (oj, _other) in targets.iter().enumerate() {
            if oj != ti {
                if let Some(d) = target_docs[oj].as_ref() {
                    corpus.push(d);
                }
            }
        }
        let mut results = vec![];
        for pk in &assigned {
            let hits = evaluate_pack(pk, doc, &corpus);
            let verdict = if hits.is_empty() { "pass" } else { "fail" };
            for item in hits {
                let mut f = Map::new();
                f.insert("target".to_string(), json!(t));
                f.insert("pack".to_string(), json!(pk.name));
                f.insert("pack_version".to_string(), json!(pk.version));
                if let Value::Object(m) = item {
                    for (k, v) in m {
                        f.insert(k, v);
                    }
                }
                findings.push(Value::Object(f));
            }
            results.push(json!({
                "pack": pk.name,
                "pack_version": pk.version,
                "verdict": verdict,
            }));
        }
        targets_report.push(json!({
            "path": t,
            "sha256": target_sha[ti],
            "results": results,
        }));
    }

    let report = json!({
        "engine_version": engine_version,
        "verdict": if findings.is_empty() { "pass" } else { "fail" },
        "packs": packs_ordered
            .iter()
            .map(|p| json!({"name": p.name, "version": p.version, "doc_class": p.doc_class}))
            .collect::<Vec<_>>(),
        "targets": targets_report,
        "findings": findings,
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    exit(if findings.is_empty() { 0 } else { 1 });
}
