//! 引擎侧检词（Nomenclator）命令行面 —— lease-mergeall-parallel 簇B 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/nomenclator 0.3.0（src/nomenclator/：cli.py、pack.py、
//! check.py、matching.py、query.py、map.py），只读对表移植，围堰源码零改动。
//!
//! CLI 子命令（空腹只报不改）：
//! - `nomenclator check --pack <术语包目录> [--quiet] <目标文档>...`
//! - `nomenclator query --pack <术语包目录> --word <待查词> [--quiet]`
//! - `nomenclator map --pack <术语包目录> --concept <概念锚词> [--quiet]`
//! 退出码三值：0 = 零违例或查询出、1 = 有违例（仅 check）、2 = 工具异常。
//!
//! 落差申报（相对围堰）：
//! 1. register 写面未实装（申报位）：调用即出错误 JSON 退出码 2；围堰的登记前
//!    校验四查与规范化写入（register.py）未移植。
//! 2. check 行切分按 \n 切并去尾 \r（围堰 splitlines 另含 \r 与 Unicode 行界），
//!    行界边缘形或有位差。
//! 3. 目标存在但不可读时归一为退出码 2 错误 JSON（围堰为未捕获 OSError traceback）。
//! 4. 用法错（缺子命令、缺 --pack、缺 --word 等）为自足解析 stderr 短句，
//!    退出码 2 与围堰 argparse 对齐，报文形不对齐；错误信封内文不含 errno 形。
//! 5. 英文整词大小写折叠按 ASCII 范围（Unicode 特例折叠如 U+212A 未对齐）。
//! 6. envelope_version 判等按 JSON 整数 1 严格判（Python 1.0 == 1 宽判未对齐）。

use serde_json::{json, Map, Value};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::exit;

/// 围堰版本锚：sih-tools/nomenclator/src/nomenclator/__init__.py __version__。
const TOOL_VERSION: &str = "0.3.0";

/// 包允许的 family 全集（envelope 共享形，pack.py 逐字对表）。
const FAMILIES: [&str; 4] = ["scrutinator", "attractor", "formatter", "nomenclator"];

/// 六态条目键集（query.py 逐字对表，缺键补 null）。
const QUERY_KEYS: [&str; 11] = [
    "zh",
    "en",
    "code",
    "definition",
    "word",
    "cause",
    "class",
    "note",
    "signed",
    "since",
    "source",
];

/// map 出泊指针（map.py POINTERS 逐字对表）。
const POINTERS: [&str; 4] = [
    "立名程序：.agents/skills/sihankor-naming/SKILL.md（本质段材料制备、推导段纪律加载、裁决登记三段）",
    "常设纪律 DEC-017：sih-engine/doc/decision/017-wengu-naming.md 修订四（工程命名动作前查命名集与检词）与修订五（执行位 stem 查册闸与检词两具上 MCP 面）与修订六（乙前注入语义映射报告加 --new-stem 甲表认领）",
    "stem 闸认领：lease open --new-stem 带甲表三件（--claim-zh 概念锚 zh、--claim-code 既裁 code 形或字面 无承、--claim-derivation 语素派生表 段:形），裸认领拒，填不圆即拒（机械核对零 LLM）",
    "新词登记：nomenclator register --pack <术语包> --entry <登记 JSON>（立名程序收敛后入口；登记即立名入口非本报告职能）",
];

/// map 免责声明（map.py DISCLAIMER 逐字对表）。
const DISCLAIMER: &str = "只报不判：本报告零语义裁决，六态与近邻俱机械子串对表，语义忠实终裁归立名程序人节点（pk-090 甲乙结合定案）";

enum LoadErr {
    Pack(String),
    JsonParse(String),
}

struct Pack {
    manifest: Value,
    terms: Vec<Value>,
    lazy: Vec<Value>,
    dead: Vec<Value>,
    candidates: Vec<Value>,
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: nomenclator check|query|map --pack <术语包目录> [...]；详见围堰 CONTRACT.md");
    exit(2);
}

fn fail_pack(e: String) -> ! {
    println!("{}", json!({"error": format!("术语包非法: {e}")}));
    exit(2);
}

fn fail_json(e: String) -> ! {
    println!("{}", json!({"error": format!("JSON 解析失败: {e}")}));
    exit(2);
}

/// envelope 校验（pack.py load_pack 前半逐字对表）。
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
    let bodies: Vec<&str> = env
        .get("bodies")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|b| b.as_str()).collect())
        .unwrap_or_default();
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
    for b in bodies {
        if !dir.join(b).is_file() {
            return Err(format!("envelope body missing: {}: {}", dir.display(), b));
        }
    }
    Ok(())
}

/// 状态文件名映射（pack.py STATE_FILES 对表）。
fn state_file(name: &str) -> &'static str {
    match name {
        "terms" => "terms.json",
        "lazy" => "lazy.json",
        "dead" => "dead.json",
        "candidates" => "candidates.json",
        _ => unreachable!(),
    }
}

fn load_list(root: &Path, name: &str) -> Result<Vec<Value>, LoadErr> {
    let fname = state_file(name);
    let path = root.join(fname);
    if !path.exists() {
        return Err(LoadErr::Pack(format!("术语包缺文件: {fname}")));
    }
    let text = fs::read_to_string(&path).map_err(|e| LoadErr::JsonParse(e.to_string()))?;
    let data: Value = serde_json::from_str(&text).map_err(|e| LoadErr::JsonParse(e.to_string()))?;
    match data.get(name).and_then(|v| v.as_array()) {
        Some(rows) => Ok(rows.clone()),
        None => Err(LoadErr::Pack(format!("术语包文件结构非法: {fname}"))),
    }
}

/// 术语包装载（pack.py load_pack 对表）：envelope 加 manifest 加四状态文件加死档 class 校验。
fn load_pack(root: &Path) -> Result<Pack, LoadErr> {
    validate_envelope(root).map_err(LoadErr::Pack)?;
    let manifest_path = root.join("manifest.json");
    if !manifest_path.exists() {
        return Err(LoadErr::Pack(format!("术语包缺清单: {}", root.display())));
    }
    let manifest_text =
        fs::read_to_string(&manifest_path).map_err(|e| LoadErr::JsonParse(e.to_string()))?;
    let manifest: Value =
        serde_json::from_str(&manifest_text).map_err(|e| LoadErr::JsonParse(e.to_string()))?;
    for key in ["name", "version", "domain"] {
        if manifest.get(key).is_none() {
            return Err(LoadErr::Pack(format!("术语包清单缺字段: {key}")));
        }
    }
    let terms = load_list(root, "terms")?;
    let lazy = load_list(root, "lazy")?;
    let dead = load_list(root, "dead")?;
    let candidates = load_list(root, "candidates")?;
    for e in &dead {
        let class = e.get("class").and_then(|v| v.as_str()).unwrap_or("");
        if class != "strict" && class != "name_only" {
            let word = match e.get("word") {
                Some(Value::String(s)) => s.clone(),
                Some(other) => other.to_string(),
                None => "None".to_string(),
            };
            return Err(LoadErr::Pack(format!("死档条目 class 非法: {word}")));
        }
    }
    Ok(Pack {
        manifest,
        terms,
        lazy,
        dead,
        candidates,
    })
}

/// fnmatch 译形（Python fnmatch.translate 对表）：`*` 任意串、`?` 单字、`[...]` 字符类。
fn fnmatch_regex(pat: &str) -> String {
    let chars: Vec<char> = pat.chars().collect();
    let n = chars.len();
    let mut res = String::new();
    let mut i = 0usize;
    while i < n {
        let c = chars[i];
        i += 1;
        if c == '*' {
            res.push_str(".*");
        } else if c == '?' {
            res.push('.');
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
                res.push_str("\\[");
            } else {
                let stuff: String = chars[i..j].iter().collect();
                let stuff = stuff.replace('\\', "\\\\");
                let inner = if let Some(rest) = stuff.strip_prefix('!') {
                    format!("^{rest}")
                } else {
                    stuff
                };
                res.push('[');
                res.push_str(&inner);
                res.push(']');
                i = j + 1;
            }
        } else {
            res.push_str(&regex::escape(&c.to_string()));
        }
    }
    format!("(?s:{res})\\z")
}

/// 术语包排除判定（pack.py Pack.excluded 对表）：
/// posix 形与斜杠前缀形与文件名三候选逐模式 fnmatch。
fn excluded(pack: &Pack, path: &Path) -> bool {
    let patterns: Vec<String> = pack
        .manifest
        .get("domain")
        .and_then(|d| d.get("exclude"))
        .and_then(|e| e.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let posix = path.to_string_lossy().replace('\\', "/");
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let candidates = [posix.clone(), format!("/{posix}"), name];
    for p in &patterns {
        let re = match regex::Regex::new(&fnmatch_regex(p)) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for c in &candidates {
            if re.is_match(c) {
                return true;
            }
        }
    }
    false
}

fn is_cjk(ch: char) -> bool {
    ('\u{4e00}'..='\u{9fff}').contains(&ch)
}

fn is_ascii_word(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_ascii_alphabetic())
}

fn find_sub(hay: &[char], needle: &[char]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    if needle.len() > hay.len() {
        return None;
    }
    for i in 0..=hay.len() - needle.len() {
        if hay[i..i + needle.len()] == *needle {
            return Some(i);
        }
    }
    None
}

/// 匹配粒度三条（matching.py iter_spans 对表，字符索引即码点偏移）：
/// 中文字词子串、中文单字两侧非 CJK、英文整词大小写不敏感。
fn iter_spans(chars: &[char], word: &str) -> Vec<(usize, usize)> {
    let wchars: Vec<char> = word.chars().collect();
    let mut spans = vec![];
    if is_ascii_word(word) {
        for i in 0..chars.len() {
            if i + wchars.len() > chars.len() {
                break;
            }
            if !chars[i..i + wchars.len()]
                .iter()
                .zip(wchars.iter())
                .all(|(a, b)| a.eq_ignore_ascii_case(b))
            {
                continue;
            }
            let before_ok = i == 0 || !chars[i - 1].is_ascii_alphabetic();
            let after = i + wchars.len();
            let after_ok = after >= chars.len() || !chars[after].is_ascii_alphabetic();
            if before_ok && after_ok {
                spans.push((i, after));
            }
        }
        return spans;
    }
    if wchars.len() == 1 && is_cjk(wchars[0]) {
        let w = wchars[0];
        for i in 0..chars.len() {
            if chars[i] != w {
                continue;
            }
            let before_cjk = i > 0 && is_cjk(chars[i - 1]);
            let after_cjk = i + 1 < chars.len() && is_cjk(chars[i + 1]);
            if !before_cjk && !after_cjk {
                spans.push((i, i + 1));
            }
        }
        return spans;
    }
    let mut i = 0usize;
    while i + wchars.len() <= chars.len() {
        if chars[i..i + wchars.len()] == wchars[..] {
            spans.push((i, i + wchars.len()));
            i += wchars.len();
        } else {
            i += 1;
        }
    }
    spans
}

/// 命中定位（matching.py locate 对表）：行号、摘录（超 60 字按命中位截 60）。
fn locate(chars: &[char], start: usize, end: usize, matched: &[char]) -> (usize, String) {
    let line = chars[..start].iter().filter(|&&c| c == '\n').count() + 1;
    let line_start = chars[..start]
        .iter()
        .rposition(|&c| c == '\n')
        .map(|i| i + 1)
        .unwrap_or(0);
    let line_end = chars[end..]
        .iter()
        .position(|&c| c == '\n')
        .map(|i| end + i)
        .unwrap_or(chars.len());
    let mut excerpt: Vec<char> = chars[line_start..line_end].to_vec();
    let b = excerpt
        .iter()
        .position(|c| !c.is_whitespace())
        .unwrap_or(excerpt.len());
    excerpt.drain(..b);
    let e = excerpt
        .iter()
        .rposition(|c| !c.is_whitespace())
        .map(|i| i + 1)
        .unwrap_or(0);
    excerpt.truncate(e);
    if excerpt.len() > 60 {
        let cut = find_sub(&excerpt, matched).unwrap_or(0);
        let lo = cut.saturating_sub(20);
        let hi = (lo + 60).min(excerpt.len());
        excerpt = excerpt[lo..hi].to_vec();
    }
    (line, excerpt.into_iter().collect())
}

fn entry_str<'a>(e: &'a Value, key: &str) -> Option<&'a str> {
    e.get(key).and_then(|v| v.as_str())
}

/// 文档核查两规则（check.py run_check 对表）：禁字级死档禁用、懒波词违例，只报不改。
fn run_check(pack: &Pack, targets: &[String]) -> Result<(Vec<String>, Vec<String>, Vec<Value>), String> {
    let mut checked: Vec<String> = vec![];
    let mut skipped: Vec<String> = vec![];
    let mut findings: Vec<Value> = vec![];
    let mut seen: HashSet<(String, String, String, i64)> = HashSet::new();
    for raw in targets {
        let path = Path::new(raw);
        if excluded(pack, path) {
            skipped.push(path.to_string_lossy().replace('\\', "/"));
            continue;
        }
        if !path.exists() {
            return Err(format!("__notfound__{raw}"));
        }
        let text = fs::read_to_string(path).map_err(|e| format!("__readfail__{e}"))?;
        let chars: Vec<char> = text.chars().collect();
        let lines: Vec<String> = text
            .split('\n')
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect();
        checked.push(path.to_string_lossy().replace('\\', "/"));
        let posix = path.to_string_lossy().replace('\\', "/");
        let dead_strict: Vec<&Value> = pack
            .dead
            .iter()
            .filter(|e| entry_str(e, "class") == Some("strict"))
            .collect();
        let rules: Vec<(&str, &str, Vec<&Value>)> = vec![
            ("dead_ban", "high", dead_strict),
            ("lazy_in_doc", "medium", pack.lazy.iter().collect()),
        ];
        for (rule, severity, entries) in &rules {
            for entry in entries {
                let word = match entry_str(entry, "word") {
                    Some(w) if !w.is_empty() => w,
                    _ => continue,
                };
                for (s, e) in iter_spans(&chars, word) {
                    let matched: Vec<char> = chars[s..e].to_vec();
                    let (line, excerpt) = locate(&chars, s, e, &matched);
                    let line_text = if line > 0 && line <= lines.len() {
                        lines[line - 1].as_str()
                    } else {
                        ""
                    };
                    let exempt = entry.get("exempt").and_then(|v| v.as_array());
                    if let Some(pats) = exempt {
                        if pats
                            .iter()
                            .filter_map(|p| p.as_str())
                            .any(|p| line_text.contains(p))
                        {
                            continue;
                        }
                    }
                    let key = (rule.to_string(), word.to_string(), posix.clone(), line as i64);
                    if seen.contains(&key) {
                        continue;
                    }
                    seen.insert(key);
                    let class = entry_str(entry, "class").unwrap_or("lazy");
                    findings.push(json!({
                        "rule": rule,
                        "severity": severity,
                        "word": word,
                        "class": class,
                        "file": posix,
                        "line": line,
                        "excerpt": excerpt,
                        "cause": entry.get("cause").cloned().unwrap_or(Value::Null),
                        "source": entry.get("source").cloned().unwrap_or(Value::Null),
                    }));
                }
            }
        }
    }
    findings.sort_by(|a, b| {
        let fa = a["file"].as_str().unwrap_or("");
        let fb = b["file"].as_str().unwrap_or("");
        let la = a["line"].as_i64().unwrap_or(0);
        let lb = b["line"].as_i64().unwrap_or(0);
        let ra = a["rule"].as_str().unwrap_or("");
        let rb = b["rule"].as_str().unwrap_or("");
        let wa = a["word"].as_str().unwrap_or("");
        let wb = b["word"].as_str().unwrap_or("");
        (fa, la, ra, wa).cmp(&(fb, lb, rb, wb))
    });
    Ok((checked, skipped, findings))
}

/// 状态查询（query.py run_query 对表）：返回六态与出处指针，不返回该不该。
fn run_query(pack: &Pack, word: &str) -> Value {
    let mut hits: Vec<(String, &Value)> = vec![];
    for t in &pack.terms {
        if ["zh", "en", "code"]
            .iter()
            .any(|k| entry_str(t, k) == Some(word))
        {
            hits.push(("established".to_string(), t));
        }
    }
    for e in &pack.lazy {
        if entry_str(e, "word") == Some(word) {
            hits.push(("lazy".to_string(), e));
        }
    }
    for e in &pack.dead {
        if entry_str(e, "word") == Some(word) {
            let class = entry_str(e, "class").unwrap_or("");
            hits.push((format!("dead/{class}"), e));
        }
    }
    for e in &pack.candidates {
        if entry_str(e, "word") == Some(word) {
            hits.push(("candidate".to_string(), e));
        }
    }
    if hits.is_empty() {
        return json!({"word": word, "state": "unknown", "hits": []});
    }
    let mut entries = vec![];
    let mut states = vec![];
    for (state, entry) in &hits {
        states.push(json!(state));
        let mut o = Map::new();
        o.insert("state".to_string(), json!(state));
        for k in QUERY_KEYS {
            o.insert(k.to_string(), entry.get(k).cloned().unwrap_or(Value::Null));
        }
        entries.push(Value::Object(o));
    }
    json!({
        "word": word,
        "state": states[0].clone(),
        "states": states,
        "hits": entries,
    })
}

/// 条目名形集（map.py _names_of 对表）：established 用 zh 与 en 与 code，余态用 word。
fn names_of(state: &str, entry: &Value) -> Vec<String> {
    if state == "established" {
        ["zh", "en", "code"]
            .iter()
            .filter_map(|k| entry_str(entry, k))
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    } else {
        entry_str(entry, "word")
            .filter(|s| !s.is_empty())
            .map(String::from)
            .into_iter()
            .collect()
    }
}

/// 语义映射报告（map.py run_map 对表）：概念锚查词典只报不判，四段齐。
fn run_map(pack: &Pack, concept_raw: &str) -> Value {
    let concept = concept_raw.trim();
    let six = run_query(pack, concept);
    let mut exact_names: Vec<String> = vec![];
    let mut code_forms = vec![];
    for t in &pack.terms {
        let names = names_of("established", t);
        if !concept.is_empty() && names.iter().any(|n| n == concept) {
            for n in &names {
                if !exact_names.contains(n) {
                    exact_names.push(n.clone());
                }
            }
            code_forms.push(json!({
                "zh": t.get("zh").cloned().unwrap_or(Value::Null),
                "en": t.get("en").cloned().unwrap_or(Value::Null),
                "code": t.get("code").cloned().unwrap_or(Value::Null),
                "definition": t.get("definition").cloned().unwrap_or(Value::Null),
            }));
        }
    }
    let mut neighbors = vec![];
    for (state, rows) in [
        ("established", &pack.terms),
        ("lazy", &pack.lazy),
        ("dead", &pack.dead),
        ("candidate", &pack.candidates),
    ] {
        for e in rows {
            let resolved = if state == "dead" {
                format!("dead/{}", entry_str(e, "class").unwrap_or(""))
            } else {
                state.to_string()
            };
            let names = names_of(state, e);
            let overlap =
                !concept.is_empty() && names.iter().any(|n| n.contains(concept) || concept.contains(n));
            if !overlap {
                continue;
            }
            if names.iter().all(|n| exact_names.contains(n)) {
                continue;
            }
            neighbors.push(json!({
                "state": resolved,
                "name": names[0],
                "names": names,
            }));
        }
    }
    json!({
        "concept": concept,
        "six_states": six,
        "code_forms": code_forms,
        "neighbors": neighbors,
        "pointers": POINTERS,
        "disclaimer": DISCLAIMER,
    })
}

fn header_value(pack: &Pack) -> Value {
    json!({
        "tool": {"name": "nomenclator", "version": TOOL_VERSION},
        "pack": {
            "name": pack.manifest.get("name").cloned().unwrap_or(Value::Null),
            "version": pack.manifest.get("version").cloned().unwrap_or(Value::Null),
        },
        "domain": pack.manifest.get("domain").cloned().unwrap_or(json!({})),
    })
}

/// 对象键递归排序（json.dumps sort_keys=True 语义）。
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

fn emit(report: Map<String, Value>) {
    println!(
        "{}",
        serde_json::to_string_pretty(&sort_value(Value::Object(report))).unwrap()
    );
}

/// 紧凑形摘要行（cli.py _emit_quiet 对表）。
fn emit_quiet(command: &str, report: &Value, code: i32) {
    let mut summary = Map::new();
    if let Some(s) = report.get("summary").and_then(|v| v.as_object()) {
        for (k, v) in s {
            summary.insert(k.clone(), v.clone());
        }
    }
    match report.get("status") {
        Some(st) => {
            summary.entry("status".to_string()).or_insert_with(|| st.clone());
        }
        None => {
            let derived = if code == 2 {
                "error"
            } else if code == 1 {
                "violation"
            } else {
                "clean"
            };
            summary.insert("status".to_string(), json!(derived));
        }
    }
    let mut line = Map::new();
    line.insert("tool".to_string(), json!("nomenclator"));
    line.insert("command".to_string(), json!(command));
    line.insert("code".to_string(), json!(code));
    line.insert("summary".to_string(), Value::Object(summary));
    println!("{}", sort_value(Value::Object(line)).to_string());
}

fn arg_value<'a>(args: &'a [String], i: &mut usize, name: &str) -> &'a str {
    if *i < args.len() {
        let v = args[*i].as_str();
        *i += 1;
        v
    } else {
        usage_fail(&format!("{name} 缺值"))
    }
}

/// 通用旗标解析：--pack 必填，--quiet 旗标，rest 收集（check 位置目标用）。
struct CommonArgs<'a> {
    pack: Option<&'a str>,
    quiet: bool,
    word: Option<&'a str>,
    concept: Option<&'a str>,
    positionals: Vec<&'a str>,
}

fn parse_common(args: &[String]) -> CommonArgs<'_> {
    let mut out = CommonArgs {
        pack: None,
        quiet: false,
        word: None,
        concept: None,
        positionals: vec![],
    };
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--pack" {
            i += 1;
            out.pack = Some(arg_value(args, &mut i, "--pack"));
        } else if let Some(v) = a.strip_prefix("--pack=") {
            out.pack = Some(v);
            i += 1;
        } else if a == "--quiet" {
            out.quiet = true;
            i += 1;
        } else if a == "--word" {
            i += 1;
            out.word = Some(arg_value(args, &mut i, "--word"));
        } else if let Some(v) = a.strip_prefix("--word=") {
            out.word = Some(v);
            i += 1;
        } else if a == "--concept" {
            i += 1;
            out.concept = Some(arg_value(args, &mut i, "--concept"));
        } else if let Some(v) = a.strip_prefix("--concept=") {
            out.concept = Some(v);
            i += 1;
        } else if a.starts_with("--") {
            usage_fail(&format!("未知旗标: {a}"));
        } else {
            out.positionals.push(a);
            i += 1;
        }
    }
    out
}

fn load_or_fail(pack_dir: &str) -> Pack {
    match load_pack(Path::new(pack_dir)) {
        Ok(p) => p,
        Err(LoadErr::Pack(e)) => fail_pack(e),
        Err(LoadErr::JsonParse(e)) => fail_json(e),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage_fail("缺子命令（check|query|map）");
    }
    let sub = args[0].clone();
    let rest = &args[1..];
    let parsed = parse_common(rest);
    match sub.as_str() {
        "register" => {
            // 落差申报一：register 写面未实装（校验四查与规范化写入未移植）。
            println!(
                "{}",
                json!({"error": "register 写面未实装（移植落差申报：校验四查与规范化写入未移植）"})
            );
            exit(2);
        }
        "check" => {
            let pack_dir = match parsed.pack {
                Some(p) => p,
                None => usage_fail("check 缺 --pack"),
            };
            if parsed.positionals.is_empty() {
                usage_fail("check 缺目标文档");
            }
            let pack = load_or_fail(pack_dir);
            let targets: Vec<String> = parsed.positionals.iter().map(|s| s.to_string()).collect();
            let (checked, skipped, findings) = match run_check(&pack, &targets) {
                Ok(r) => r,
                Err(e) if e.starts_with("__notfound__") => {
                    println!(
                        "{}",
                        json!({"error": format!("目标不存在: {}", &e["__notfound__".len()..])})
                    );
                    exit(2);
                }
                Err(e) if e.starts_with("__readfail__") => {
                    println!(
                        "{}",
                        json!({"error": format!("读取失败: {}", &e["__readfail__".len()..])})
                    );
                    exit(2);
                }
                Err(_) => unreachable!(),
            };
            let code: i32 = if findings.is_empty() { 0 } else { 1 };
            let mut report = Map::new();
            report.insert("header".to_string(), header_value(&pack));
            report.insert("checked".to_string(), json!(checked));
            report.insert("skipped".to_string(), json!(skipped));
            report.insert("findings".to_string(), Value::Array(findings.clone()));
            report.insert(
                "summary".to_string(),
                json!({
                    "checked": checked.len(),
                    "skipped": skipped.len(),
                    "findings": findings.len(),
                }),
            );
            if parsed.quiet {
                emit_quiet("check", &Value::Object(report.clone()), code);
            } else {
                emit(report);
            }
            exit(code);
        }
        "query" => {
            let pack_dir = match parsed.pack {
                Some(p) => p,
                None => usage_fail("query 缺 --pack"),
            };
            let word = match parsed.word {
                Some(w) => w,
                None => usage_fail("query 缺 --word"),
            };
            let pack = load_or_fail(pack_dir);
            let result = run_query(&pack, word);
            let mut report = Map::new();
            report.insert("header".to_string(), header_value(&pack));
            if let Some(obj) = result.as_object() {
                for (k, v) in obj {
                    report.insert(k.clone(), v.clone());
                }
            }
            if parsed.quiet {
                emit_quiet("query", &Value::Object(report.clone()), 0);
            } else {
                emit(report);
            }
            exit(0);
        }
        "map" => {
            let pack_dir = match parsed.pack {
                Some(p) => p,
                None => usage_fail("map 缺 --pack"),
            };
            let concept = match parsed.concept {
                Some(c) => c,
                None => usage_fail("map 缺 --concept"),
            };
            let pack = load_or_fail(pack_dir);
            let result = run_map(&pack, concept);
            let mut report = Map::new();
            report.insert("header".to_string(), header_value(&pack));
            if let Some(obj) = result.as_object() {
                for (k, v) in obj {
                    report.insert(k.clone(), v.clone());
                }
            }
            if parsed.quiet {
                emit_quiet("map", &Value::Object(report.clone()), 0);
            } else {
                emit(report);
            }
            exit(0);
        }
        other => usage_fail(&format!("未知子命令: {other}")),
    }
}
