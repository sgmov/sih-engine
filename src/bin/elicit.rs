//! 引擎侧叩问（elicit）命令行面 —— lease-mergeleg6-parallel 簇J 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/elicit 0.2.0（src/elicit/cli.py，契约 CONTRACT.md），
//! 只读对表移植，围堰源码零改动。构成性：零 LLM 零第三方依赖，同参双跑逐字节一致；
//! 三扳机即未登记词与召回零命中与矛盾候选，信号件 ndjson 五字段，digest 消化闸验
//! 契约逐信号处置，suspend 落挂起记录。载体语义承 ORD-008 引用图可达与孤悬判定：
//! 登记信号逐项被处置即可达，未处置信号即出度为零的孤悬节点不得上链，零信号直过
//! 即空图平凡可达。
//!
//! CLI：`elicit <check|digest|suspend> <子命令旗标>`；check 必填 --packs；
//! digest 必填 --signals 与 --contract；suspend 必填 --signals 与 --log。
//! 退出码三值：0 无信号 / 全处置 / 已挂起，1 有信号 / blocked，2 异常或用法错。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. 用法错（缺必填旗标、未知旗标、未知子命令）为自足解析退出码 2，stderr 短句
//!    回报；argparse 的 usage/epilog 帮助报文形与 -h 全文不对齐（-h 打一行用法）。
//! 2. 信号件或 terms.json 内行 JSON 不可解析、信号行缺 subject 字段：围堰为未捕
//!    traceback（退出码 1），移植按退出码 2 报错不静默。
//! 3. recall 面行的 matched 字段为容器形（dict/list）时 str() 表现不对齐（按 JSON
//!    形字符串化）；None/bool/数字标量按 Python str() 形对表。
//! 4. --version 在子命令旗标位也接受（围堰 argparse 仅主解析器位识别）。

use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::exit;

const TOOL: &str = "elicit";
/// 围堰版本锚：sih-tools/elicit/src/elicit/__init__.py __version__。
const VERSION: &str = "0.2.0";

// ---- Python 形 JSON 序列化（json.dumps 对表：sort_keys / separators / indent） ----

fn py_dumps(v: &Value, sort_keys: bool, tight: bool, indent: Option<usize>) -> String {
    let mut out = String::new();
    write_py(v, sort_keys, tight, indent, 0, &mut out);
    out
}

fn write_py(v: &Value, sort_keys: bool, tight: bool, indent: Option<usize>, level: usize, out: &mut String) {
    match v {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            out.push_str(&v.to_string())
        }
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            match indent {
                Some(w) => {
                    let inner = " ".repeat(w * (level + 1));
                    let close = " ".repeat(w * level);
                    out.push_str("[\n");
                    for (i, item) in a.iter().enumerate() {
                        if i > 0 {
                            out.push_str(",\n");
                        }
                        out.push_str(&inner);
                        write_py(item, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push('\n');
                    out.push_str(&close);
                    out.push(']');
                }
                None => {
                    let sep = if tight { "," } else { ", " };
                    out.push('[');
                    for (i, item) in a.iter().enumerate() {
                        if i > 0 {
                            out.push_str(sep);
                        }
                        write_py(item, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push(']');
                }
            }
        }
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
            if sort_keys {
                kvs.sort_by(|a, b| a.0.cmp(b.0));
            }
            match indent {
                Some(w) => {
                    let inner = " ".repeat(w * (level + 1));
                    let close = " ".repeat(w * level);
                    out.push_str("{\n");
                    for (i, (k, val)) in kvs.iter().enumerate() {
                        if i > 0 {
                            out.push_str(",\n");
                        }
                        out.push_str(&inner);
                        out.push_str(&Value::String((*k).clone()).to_string());
                        out.push_str(": ");
                        write_py(val, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push('\n');
                    out.push_str(&close);
                    out.push('}');
                }
                None => {
                    let (isep, ksep) = if tight { (",", ":") } else { (", ", ": ") };
                    out.push('{');
                    for (i, (k, val)) in kvs.iter().enumerate() {
                        if i > 0 {
                            out.push_str(isep);
                        }
                        out.push_str(&Value::String((*k).clone()).to_string());
                        out.push_str(ksep);
                        write_py(val, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push('}');
                }
            }
        }
    }
}

/// Python str() 形字符串化（matched 字段标量面对表，容器形浅对表见头注落差三）。
fn py_scalar_str(v: Option<&Value>) -> String {
    match v {
        None => String::new(),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Null) => "None".to_string(),
        Some(Value::Bool(true)) => "True".to_string(),
        Some(Value::Bool(false)) => "False".to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(other) => other.to_string(),
    }
}

// ---- 面件装载（_load_terms / _face_rows 对表） ----

fn err_stdout(message: String) -> i32 {
    println!("{}", py_dumps(&json!({"error": message}), false, false, None));
    2
}

/// 登记面装载：缺 terms.json 返回 None（对表 _load_terms 缺席形）。
fn load_terms(pack_dir: &str) -> (Option<HashSet<String>>, PathBuf) {
    let terms_path = Path::new(pack_dir).join("terms.json");
    if !terms_path.is_file() {
        return (None, terms_path);
    }
    let text = match fs::read_to_string(&terms_path) {
        Ok(t) => t,
        Err(_) => return (None, terms_path),
    };
    let data: Value = match serde_json::from_str(&text) {
        Ok(d) => d,
        Err(_) => return (None, terms_path),
    };
    let mut known = HashSet::new();
    if let Some(entries) = data.get("terms").and_then(|v| v.as_array()) {
        for entry in entries {
            for key in ["zh", "en", "code"] {
                if let Some(value) = entry.get(key).and_then(|v| v.as_str()) {
                    if !value.is_empty() {
                        known.insert(value.to_string());
                    }
                }
            }
        }
    }
    (Some(known), terms_path)
}

/// ndjson 面装载：空行跳过，逐行解析；缺席件返回 None。
fn load_rows(path: &str) -> (Option<Vec<Value>>, PathBuf) {
    let p = PathBuf::from(path);
    if !p.is_file() {
        return (None, p);
    }
    let text = match fs::read_to_string(&p) {
        Ok(t) => t,
        Err(_) => return (None, p),
    };
    let mut rows = vec![];
    for line in text.lines() {
        let stripped = line.trim();
        if stripped.is_empty() {
            continue;
        }
        match serde_json::from_str(stripped) {
            Ok(v) => rows.push(v),
            Err(e) => {
                err_stdout(format!("信号件行 JSON 不可解析：{e}"));
                return (None, p);
            }
        }
    }
    (Some(rows), p)
}

fn signal(signal_type: &str, subject: &str, weight: &str, source_ref: &str, at: &str) -> Value {
    json!({
        "signal_type": signal_type,
        "subject": subject,
        "weight": weight,
        "source_ref": source_ref,
        "at": at,
    })
}

fn rows_subjects(rows: &[Value]) -> Option<Vec<String>> {
    let mut out = vec![];
    for r in rows {
        match r.get("subject").and_then(|v| v.as_str()) {
            Some(s) => out.push(s.to_string()),
            None => return None,
        }
    }
    Some(out)
}

// ---- check：三扳机信号生成（cmd_check 对表） ----

struct CheckArgs {
    words: Vec<String>,
    topics: Vec<String>,
    conflict_words: Vec<String>,
    packs: String,
    recall_face: Option<String>,
    history_face: Option<String>,
    out: Option<String>,
    at: String,
    quiet: bool,
}

fn cmd_check(a: &CheckArgs) -> i32 {
    let (known, terms_path) = load_terms(&a.packs);
    let known = match known {
        Some(k) => k,
        None => return err_stdout(format!("目标不存在: {}", terms_path.display())),
    };
    let mut signals: Vec<Value> = vec![];
    // 载体锚点一（信号生成位）：ORD-008 引用图可达与孤悬判定，每条信号即链引用图
    // 一节点，登记信号逐项被后续处置即可达。
    for word in &a.words {
        if !known.contains(word) {
            signals.push(signal(
                "unregistered",
                word,
                "轻",
                &terms_path.to_string_lossy(),
                &a.at,
            ));
        }
    }
    if let Some(face) = &a.recall_face {
        let (rows, face_path) = load_rows(face);
        let rows = match rows {
            Some(r) => r,
            None => return err_stdout(format!("目标不存在: {}", face_path.display())),
        };
        for topic in &a.topics {
            let any_hit = rows
                .iter()
                .any(|r| py_scalar_str(r.get("matched")).contains(topic.as_str()));
            if !any_hit {
                signals.push(signal(
                    "recall_miss",
                    topic,
                    "轻",
                    &face_path.to_string_lossy(),
                    &a.at,
                ));
            }
        }
        for word in &a.conflict_words {
            for (index, r) in rows.iter().enumerate() {
                if r.get("matched").and_then(|v| v.as_str()) == Some(word.as_str()) {
                    signals.push(signal(
                        "recall_conflict",
                        word,
                        "重",
                        &format!("{}@{}", face_path.display(), index + 1),
                        &a.at,
                    ));
                }
            }
        }
    }
    if let Some(hist) = &a.history_face {
        let (rows, hist_path) = load_rows(hist);
        let rows = match rows {
            Some(r) => r,
            None => return err_stdout(format!("目标不存在: {}", hist_path.display())),
        };
        for word in &a.conflict_words {
            for (index, r) in rows.iter().enumerate() {
                if py_dumps(r, false, false, None).contains(word.as_str()) {
                    signals.push(signal(
                        "ruling_conflict",
                        word,
                        "重",
                        &format!("{}@{}", hist_path.display(), index + 1),
                        &a.at,
                    ));
                }
            }
        }
    }
    signals.sort_by(|x, y| {
        let kx = (
            x["signal_type"].as_str().unwrap_or(""),
            x["subject"].as_str().unwrap_or(""),
        );
        let ky = (
            y["signal_type"].as_str().unwrap_or(""),
            y["subject"].as_str().unwrap_or(""),
        );
        kx.cmp(&ky)
    });
    let text: String = signals
        .iter()
        .map(|s| py_dumps(s, false, true, None) + "\n")
        .collect();
    match &a.out {
        Some(p) => {
            if let Err(e) = fs::write(p, text.as_bytes()) {
                return err_stdout(format!("信号件写入失败: {e}"));
            }
        }
        None => {
            print!("{text}");
            let _ = std::io::stdout().flush();
        }
    }
    let code = if signals.is_empty() { 0 } else { 1 };
    if a.quiet {
        let summary = json!({
            "tool": TOOL,
            "command": "check",
            "code": code,
            "summary": {"signals": signals.len(), "out": a.out.clone().unwrap_or_else(|| "stdout".to_string())},
        });
        println!("{}", py_dumps(&summary, true, true, None));
    }
    code
}

// ---- digest：消化闸（cmd_digest 对表，孤悬判定位承 ORD-008） ----

struct DigestArgs {
    signals: String,
    contract: String,
    quiet: bool,
}

fn cmd_digest(a: &DigestArgs) -> i32 {
    let (rows, sig_path) = load_rows(&a.signals);
    let rows = match rows {
        Some(r) => r,
        None => return err_stdout(format!("目标不存在: {}", sig_path.display())),
    };
    let contract_path = PathBuf::from(&a.contract);
    if !contract_path.is_file() {
        return err_stdout(format!("目标不存在: {}", contract_path.display()));
    }
    let text = match fs::read_to_string(&contract_path) {
        Ok(t) => t,
        Err(e) => return err_stdout(format!("契约读取失败: {e}")),
    };
    // 载体锚点二（孤悬判定位）：缺处置标记的信号即非末端且出度为零的孤悬节点，
    // missing 非空即孤悬在场即 blocked 退出码一，不得上链。
    let subjects = match rows_subjects(&rows) {
        Some(s) => s,
        None => return err_stdout("信号件行缺 subject 字段".to_string()),
    };
    let missing: Vec<Value> = subjects
        .iter()
        .filter(|s| !text.contains(&format!("叩问处置[{}]", s)))
        .map(|s| json!(s))
        .collect();
    let (out, code): (Value, i32) = if !missing.is_empty() {
        (json!({"digest": "blocked", "missing": missing}), 1)
    } else {
        // 载体锚点三（空图平凡可达位）：零信号直过即空图平凡可达，全处置过闸即全图可达。
        (json!({"digest": "passed", "covered": rows.len()}), 0)
    };
    if a.quiet {
        let covered = out.get("covered").and_then(|v| v.as_u64()).unwrap_or(0);
        let summary = json!({
            "tool": TOOL,
            "command": "digest",
            "code": code,
            "summary": {"digest": out["digest"], "covered": covered},
        });
        println!("{}", py_dumps(&summary, true, true, None));
    } else {
        println!("{}", py_dumps(&out, false, false, None));
    }
    code
}

// ---- suspend：挂起记录（cmd_suspend 对表） ----

struct SuspendArgs {
    signals: String,
    log: String,
    at: String,
    quiet: bool,
}

fn cmd_suspend(a: &SuspendArgs) -> i32 {
    let (rows, sig_path) = load_rows(&a.signals);
    let rows = match rows {
        Some(r) => r,
        None => return err_stdout(format!("目标不存在: {}", sig_path.display())),
    };
    let subjects = match rows_subjects(&rows) {
        Some(s) => s,
        None => return err_stdout("信号件行缺 subject 字段".to_string()),
    };
    let record = json!({
        "logged_at": a.at,
        "state": "suspended",
        "signals": rows.len(),
        "subjects": subjects,
    });
    let line = py_dumps(&record, false, false, None) + "\n";
    let log_result = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&a.log)
        .and_then(|mut f| f.write_all(line.as_bytes()));
    if let Err(e) = log_result {
        return err_stdout(format!("挂起记录写入失败: {e}"));
    }
    if a.quiet {
        let summary = json!({
            "tool": TOOL,
            "command": "suspend",
            "code": 0,
            "summary": {"signals": rows.len(), "state": "suspended"},
        });
        println!("{}", py_dumps(&summary, true, true, None));
    } else {
        println!("{}", py_dumps(&json!({"suspend": "recorded"}), false, false, None));
    }
    0
}

// ---- 入口 ----

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: elicit <check|digest|suspend> <子命令旗标>（-h 打各子命令用法）");
    exit(2);
}

fn sub_usage(sub: &str) -> ! {
    let face = match sub {
        "check" => "用法: elicit check --packs <检词包目录> [--words 词]... [--topics 主题]... [--conflict-words 词]... [--recall-face ndjson] [--history-face ndjson] [--out 信号件] [--at 实日] [--quiet]",
        "digest" => "用法: elicit digest --signals <信号件 ndjson> --contract <契约文档> [--quiet]",
        "suspend" => "用法: elicit suspend --signals <信号件 ndjson> --log <挂起记录件> [--at 实日] [--quiet]",
        _ => "用法: elicit <check|digest|suspend> ...",
    };
    eprintln!("{face}");
    exit(0);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--version") {
        println!("{TOOL} {VERSION}");
        exit(0);
    }
    if args.iter().any(|a| a == "-h" || a == "--help") {
        let sub = args.iter().find(|a| ["check", "digest", "suspend"].contains(&a.as_str()));
        match sub {
            Some(s) => sub_usage(s),
            None => sub_usage(""),
        }
    }
    if args.is_empty() {
        usage_fail("缺子命令（check|digest|suspend）");
    }
    let sub = args[0].clone();
    if !["check", "digest", "suspend"].contains(&sub.as_str()) {
        usage_fail(&format!("未知子命令: {sub}"));
    }

    // 旗标自足解析（argparse 对表：append 形三旗标、单值旗标、store_true 一旗标）。
    let mut multi: HashMap<String, Vec<String>> = HashMap::new();
    let mut single: HashMap<String, String> = HashMap::new();
    let mut quiet = false;
    let mut i = 1usize;
    let append_flags = ["--words", "--topics", "--conflict-words"];
    let value_flags = [
        "--packs",
        "--recall-face",
        "--history-face",
        "--out",
        "--at",
        "--signals",
        "--contract",
        "--log",
    ];
    while i < args.len() {
        let a = args[i].clone();
        if append_flags.contains(&a.as_str()) {
            i += 1;
            if i >= args.len() {
                usage_fail(&format!("{a} 缺值"));
            }
            multi.entry(a).or_default().push(args[i].clone());
        } else if value_flags.contains(&a.as_str()) {
            i += 1;
            if i >= args.len() {
                usage_fail(&format!("{a} 缺值"));
            }
            single.insert(a, args[i].clone());
        } else if a == "--quiet" {
            quiet = true;
        } else {
            usage_fail(&format!("未知旗标或多余参数: {a}"));
        }
        i += 1;
    }
    let get = |k: &str| single.get(k).cloned();
    let code = match sub.as_str() {
        "check" => {
            let packs = match get("--packs") {
                Some(p) => p,
                None => usage_fail("check 缺 --packs（必填）"),
            };
            cmd_check(&CheckArgs {
                words: multi.get("--words").cloned().unwrap_or_default(),
                topics: multi.get("--topics").cloned().unwrap_or_default(),
                conflict_words: multi.get("--conflict-words").cloned().unwrap_or_default(),
                packs,
                recall_face: get("--recall-face"),
                history_face: get("--history-face"),
                out: get("--out"),
                at: get("--at").unwrap_or_default(),
                quiet,
            })
        }
        "digest" => {
            let signals = match get("--signals") {
                Some(p) => p,
                None => usage_fail("digest 缺 --signals（必填）"),
            };
            let contract = match get("--contract") {
                Some(p) => p,
                None => usage_fail("digest 缺 --contract（必填）"),
            };
            cmd_digest(&DigestArgs { signals, contract, quiet })
        }
        _ => {
            let signals = match get("--signals") {
                Some(p) => p,
                None => usage_fail("suspend 缺 --signals（必填）"),
            };
            let log = match get("--log") {
                Some(p) => p,
                None => usage_fail("suspend 缺 --log（必填）"),
            };
            cmd_suspend(&SuspendArgs {
                signals,
                log,
                at: get("--at").unwrap_or_default(),
                quiet,
            })
        }
    };
    exit(code);
}
