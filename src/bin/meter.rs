//! 引擎侧计量（Meter）命令行面 —— lease-mergeleg23-parallel 簇F 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/meter 0.2.0（src/meter/cli.py 269 行），只读对表移植，
//! 围堰源码零改动。核心行为链：run 包裹执行被包裹命令并按日 ndjson 落计数册
//! （时间戳、命令、工具归因、退出码、耗时，归因为命令记号扫描已知工具名的
//! 启发式，如实声明）、count 计数汇总（total 与 by_tool 与 by_date 三方对表）、
//! crosscheck 链上工具事件对册面逐日对表漏计检出。
//!
//! CLI：`meter <run|count|crosscheck> ...`
//! 退出码三值：0 = 净；1 = 漏计旗在（crosscheck）；2 = 用法或工具自身异常。
//!
//! 落差申报（相对围堰）：
//! 1. --counts 缺省目录语义不同：围堰缺省为包相邻 counts/ 目录，引擎件缺省为
//!    cwd 相对 "counts"（引擎安装形无包相邻语义）。
//! 2. aware/naive 时间戳混比：围堰 datetime 比较抛 TypeError 未捕获栈回退退出 1，
//!    本件 naive 按 UTC 归一后继续参与比较。
//! 3. 围堰未捕获异常路径（counts 行坏 JSON、trail 事件缺 timestamp、ts 解析失败）
//!    为栈回退退出 1，本件 stderr 单行报文退出 1，报文形不对齐、退出码对齐。
//! 4. Python json 浮点最短表示边缘形未对齐（serde_json vs repr）。
//! 5. argparse REMAINDER 的冷门排序形（旗标夹在命令记号之间）按「首个非旗标记号
//!    起全部转命令」近似，常规形（前置 --counts 与前置 --）逐字对齐。
//! 6. spawn 失败 stderr 报文内文不对齐（退出码 127 与落册行为对齐）。

use serde_json::{json, Map, Value};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::Path;
use std::process::{exit, Command};
use std::time::Instant;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};

/// 围堰版本锚：sih-tools/meter/src/meter/__init__.py __version__。
const VERSION: &str = "0.2.0";
const KNOWN_TOOLS: [&str; 6] = ["scribe", "scrutinator", "formatter", "nomenclator", "selector", "meter"];
const TRAIL_TOOL_EVENTS: [&str; 2] = ["certification_completed", "intent_refined"];

// ---------- Python json 规范形（见头注落差四） ----------

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
        Value::Number(n) => n.to_string(),
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

fn py_line_ordered(v: &Value) -> String {
    // json.dumps(v, ensure_ascii=False)：默认分隔符 (', ', ': ')，插入序
    let mut s = String::new();
    write_line(v, false, ", ", ": ", &mut s);
    s
}

fn py_line_compact_sorted(v: &Value) -> String {
    // json.dumps(v, ensure_ascii=False, sort_keys=True, separators=(',', ':'))
    let mut s = String::new();
    write_line(v, true, ",", ":", &mut s);
    s
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

fn py_pretty_ordered(v: &Value) -> String {
    // json.dumps(v, ensure_ascii=False, indent=2)：插入序
    let mut s = String::new();
    write_pretty(v, 0, &mut s);
    s
}

/// json.dumps 非字符串 dict 键的转换形（None→"null"、bool、数）。
fn py_key(v: Option<&Value>) -> String {
    match v {
        None | Some(Value::Null) => "null".to_string(),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Bool(b)) => if *b { "true" } else { "false" }.to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(other) => py_line_compact_sorted(other),
    }
}

// ---------- 时间 ----------

/// 围堰 datetime.now(timezone.utc).isoformat()（timespec auto：micro==0 省略微秒）。
fn utcnow_iso() -> String {
    let now = chrono::Utc::now();
    let micro = now.timestamp_subsec_micros();
    if micro == 0 {
        now.format("%Y-%m-%dT%H:%M:%S+00:00").to_string()
    } else {
        now.format("%Y-%m-%dT%H:%M:%S%.6f+00:00").to_string()
    }
}

/// datetime.fromisoformat 近似：aware 保留其偏移，naive 按 UTC 归一（落差二）。
#[derive(Clone)]
enum Ts {
    Aware(DateTime<FixedOffset>),
    Naive(NaiveDateTime),
}

impl Ts {
    fn parse(text: &str) -> Option<Ts> {
        if let Ok(dt) = DateTime::parse_from_rfc3339(text) {
            return Some(Ts::Aware(dt));
        }
        for fmt in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%d %H:%M:%S%.f"] {
            if let Ok(nd) = NaiveDateTime::parse_from_str(text, fmt) {
                return Some(Ts::Naive(nd));
            }
        }
        None
    }
    /// 比较键：aware 折 UTC，naive 按 UTC（落差二）。
    fn epoch(&self) -> i64 {
        match self {
            Ts::Aware(dt) => dt.timestamp(),
            Ts::Naive(nd) => nd.and_utc().timestamp(),
        }
    }
    fn date(&self) -> NaiveDate {
        match self {
            Ts::Aware(dt) => dt.date_naive(),
            Ts::Naive(nd) => nd.date(),
        }
    }
    fn iso(&self) -> String {
        match self {
            Ts::Aware(dt) => {
                let micro = dt.timestamp_subsec_micros();
                if micro == 0 {
                    dt.format("%Y-%m-%dT%H:%M:%S%:z").to_string()
                } else {
                    dt.format("%Y-%m-%dT%H:%M:%S%.6f%:z").to_string()
                }
            }
            Ts::Naive(nd) => nd.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: meter <run|count|crosscheck> <子命令参数>");
    exit(2);
}

/// 围堰未捕获异常路径的近似承载（落差三）：stderr 单行、退出 1。
fn die_uncaught(kind: &str, msg: &str) -> ! {
    eprintln!("meter: {kind}: {msg}");
    exit(1);
}

// ---------- 计数册 ----------

/// 按日分册全载：sorted(glob("*.ndjson")) 逐行 JSON。坏行围堰栈回退退出 1（落差三）。
fn load_counts(counts_dir: &str) -> Vec<Value> {
    let mut records = vec![];
    let dir = Path::new(counts_dir);
    if !dir.is_dir() {
        return records;
    }
    let mut files: Vec<String> = vec![];
    if let Ok(rd) = fs::read_dir(dir) {
        for ent in rd.flatten() {
            let p = ent.path();
            if p.to_string_lossy().ends_with(".ndjson") {
                files.push(p.to_string_lossy().into_owned());
            }
        }
    }
    files.sort();
    for f in files {
        let text = match fs::read_to_string(&f) {
            Ok(t) => t,
            Err(e) => die_uncaught("OSError", &format!("{f}: {e}")),
        };
        for (lineno, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            match serde_json::from_str::<Value>(line) {
                Ok(v) => records.push(v),
                Err(e) => die_uncaught("JSONDecodeError", &format!("{f} line {}: {e}", lineno + 1)),
            }
        }
    }
    records
}

fn append_record(counts_dir: &str, record: &Value) {
    let dir = Path::new(counts_dir);
    let _ = fs::create_dir_all(dir);
    let ts = record.get("ts").and_then(|v| v.as_str()).unwrap_or("");
    let day = ts.get(..10).unwrap_or("");
    let path = dir.join(format!("{day}.ndjson"));
    use std::io::Write;
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "{}", py_line_ordered(record));
    }
}

// ---------- run ----------

fn cmd_run(counts: &str, mut command: Vec<String>) -> i32 {
    if !command.is_empty() && command[0] == "--" {
        command.remove(0);
    }
    if command.is_empty() {
        println!(
            "{}",
            py_line_ordered(&json!({"status": "error", "error": "command_missing"}))
        );
        return 2;
    }
    let tool = command.iter().find(|t| KNOWN_TOOLS.contains(&t.as_str())).cloned();
    let start = Instant::now();
    let exit_code;
    match Command::new(&command[0]).args(&command[1..]).status() {
        Ok(st) => {
            exit_code = match st.code() {
                Some(c) => c,
                None => {
                    use std::os::unix::process::ExitStatusExt;
                    -st.signal().unwrap_or(1)
                }
            };
        }
        Err(e) => {
            exit_code = 127;
            eprintln!("命令不可执行: {e}");
        }
    }
    let duration_ms = start.elapsed().as_millis() as i64;
    let mut record = Map::new();
    record.insert("ts".into(), json!(utcnow_iso()));
    record.insert(
        "command".into(),
        Value::Array(command.into_iter().map(Value::String).collect()),
    );
    record.insert("tool".into(), tool.map(Value::String).unwrap_or(Value::Null));
    record.insert("exit_code".into(), json!(exit_code));
    record.insert("duration_ms".into(), json!(duration_ms));
    append_record(counts, &Value::Object(record));
    exit_code
}

// ---------- count ----------

fn cmd_count(counts: &str, tool: Option<&str>, date: Option<&str>, quiet: bool) -> i32 {
    let mut records = load_counts(counts);
    if let Some(t) = tool {
        records.retain(|r| r.get("tool").and_then(|v| v.as_str()) == Some(t));
    }
    if let Some(d) = date {
        records.retain(|r| {
            r.get("ts")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .starts_with(d)
        });
    }
    let mut by_tool: Map<String, Value> = Map::new();
    let mut by_date: Map<String, Value> = Map::new();
    let mut tool_counts: HashMap<String, usize> = HashMap::new();
    let mut tool_order: Vec<String> = vec![];
    let mut date_counts: HashMap<String, usize> = HashMap::new();
    let mut date_order: Vec<String> = vec![];
    for rec in &records {
        let tk = py_key(rec.get("tool"));
        if !tool_order.contains(&tk) {
            tool_order.push(tk.clone());
        }
        *tool_counts.entry(tk).or_insert(0) += 1;
        let day = rec.get("ts").and_then(|v| v.as_str()).unwrap_or("");
        let day = day.get(..10).unwrap_or("").to_string();
        if !date_order.contains(&day) {
            date_order.push(day.clone());
        }
        *date_counts.entry(day).or_insert(0) += 1;
    }
    for k in tool_order {
        let n = tool_counts[&k];
        by_tool.insert(k, json!(n));
    }
    for k in date_order {
        let n = date_counts[&k];
        by_date.insert(k, json!(n));
    }
    let total = records.len();
    if quiet {
        let mut line = Map::new();
        line.insert("tool".into(), json!("meter"));
        line.insert("command".into(), json!("count"));
        line.insert("code".into(), json!(0));
        let mut summary = Map::new();
        summary.insert("total".into(), json!(total));
        summary.insert("by_tool".into(), Value::Object(by_tool));
        summary.insert("by_date".into(), Value::Object(by_date));
        line.insert("summary".into(), Value::Object(summary));
        println!("{}", py_line_compact_sorted(&Value::Object(line)));
        return 0;
    }
    let mut rep = Map::new();
    rep.insert(
        "engine".into(),
        json!({"name": "meter", "version": VERSION}),
    );
    rep.insert("counts_dir".into(), json!(counts));
    rep.insert("total".into(), json!(total));
    rep.insert("by_tool".into(), Value::Object(by_tool));
    rep.insert("by_date".into(), Value::Object(by_date));
    println!("{}", py_pretty_ordered(&Value::Object(rep)));
    0
}

// ---------- crosscheck ----------

fn implied_tools(event: &Value) -> BTreeSet<String> {
    let mut implied: BTreeSet<String> = BTreeSet::new();
    implied.insert("scribe".to_string());
    if event.get("event_type").and_then(|v| v.as_str()) == Some("intent_refined") {
        implied.insert("scrutinator".to_string());
    }
    let report_path = event
        .get("details")
        .and_then(|d| d.get("report_path"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    for name in KNOWN_TOOLS {
        if name != "meter" && report_path.contains(name) {
            implied.insert(name.to_string());
        }
    }
    implied
}

fn cmd_crosscheck(trails: &[String], counts: &str, quiet: bool) -> i32 {
    let records = load_counts(counts);
    let mut tool_days: HashMap<String, BTreeSet<NaiveDate>> = HashMap::new();
    let mut first_ts: Option<Ts> = None;
    for rec in &records {
        let ts_str = match rec.get("ts").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => die_uncaught("KeyError", "'ts'"),
        };
        let ts = match Ts::parse(ts_str) {
            Some(t) => t,
            None => die_uncaught("ValueError", &format!("Invalid isoformat string: '{ts_str}'")),
        };
        if first_ts.as_ref().map(|f| ts.epoch() < f.epoch()).unwrap_or(true) {
            first_ts = Some(ts.clone());
        }
        if let Some(t) = rec.get("tool").and_then(|v| v.as_str()) {
            if !t.is_empty() {
                tool_days.entry(t.to_string()).or_default().insert(ts.date());
            }
        }
    }
    let mut checked = 0usize;
    let mut pre_meter = 0usize;
    let mut flagged: Vec<Value> = vec![];
    let mut trails_out: Vec<Value> = vec![];
    for trail in trails {
        let path = Path::new(trail);
        trails_out.push(json!(trail));
        if !path.is_file() {
            println!(
                "{}",
                py_line_ordered(&json!({"status": "error", "error": "trail_not_found", "detail": trail}))
            );
            return 2;
        }
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => die_uncaught("OSError", &format!("{trail}: {e}")),
        };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let event: Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "{}",
                        py_line_ordered(&json!({
                            "status": "error", "error": "trail_invalid_json",
                            "detail": format!("{trail}: {e}"),
                        }))
                    );
                    return 2;
                }
            };
            let ev_type = event.get("event_type").and_then(|v| v.as_str()).unwrap_or("");
            if !TRAIL_TOOL_EVENTS.contains(&ev_type) {
                continue;
            }
            let ev_ts_str = match event.get("timestamp").and_then(|v| v.as_str()) {
                Some(s) => s,
                None => die_uncaught("KeyError", "'timestamp'"),
            };
            let ev_ts = match Ts::parse(ev_ts_str) {
                Some(t) => t,
                None => die_uncaught("ValueError", &format!("Invalid isoformat string: '{ev_ts_str}'")),
            };
            if first_ts.as_ref().map(|f| ev_ts.epoch() < f.epoch()).unwrap_or(true) {
                pre_meter += 1;
                continue;
            }
            checked += 1;
            let d0 = ev_ts.date();
            let d1 = d0.pred_opt();
            let days_hit = |tool: &str| -> bool {
                tool_days
                    .get(tool)
                    .map(|set| set.contains(&d0) || d1.map(|dd| set.contains(&dd)).unwrap_or(false))
                    .unwrap_or(false)
            };
            for tool in implied_tools(&event) {
                if !days_hit(&tool) {
                    flagged.push(json!({
                        "event_id": event.get("event_id").cloned().unwrap_or(Value::Null),
                        "event_type": event.get("event_type").cloned().unwrap_or(Value::Null),
                        "tool": tool,
                        "event_date": d0.to_string(),
                        "reason": "no_meter_record",
                    }));
                }
            }
        }
    }
    let has_flagged = !flagged.is_empty();
    let code = if has_flagged { 1 } else { 0 };
    if quiet {
        let mut line = Map::new();
        line.insert("tool".into(), json!("meter"));
        line.insert("command".into(), json!("crosscheck"));
        line.insert("code".into(), json!(code));
        let mut summary = Map::new();
        summary.insert("meter_records".into(), json!(records.len()));
        summary.insert("checked_events".into(), json!(checked));
        summary.insert("pre_meter_events".into(), json!(pre_meter));
        summary.insert("flagged_count".into(), json!(flagged.len()));
        line.insert("summary".into(), Value::Object(summary));
        println!("{}", py_line_compact_sorted(&Value::Object(line)));
        return code;
    }
    let mut rep = Map::new();
    rep.insert(
        "engine".into(),
        json!({"name": "meter", "version": VERSION}),
    );
    rep.insert("trails".into(), Value::Array(trails_out));
    rep.insert("counts_dir".into(), json!(counts));
    rep.insert(
        "first_meter_ts".into(),
        first_ts.map(|t| Value::String(t.iso())).unwrap_or(Value::Null),
    );
    rep.insert("meter_records".into(), json!(records.len()));
    rep.insert("checked_events".into(), json!(checked));
    rep.insert("pre_meter_events".into(), json!(pre_meter));
    rep.insert("flagged".into(), Value::Array(flagged.clone()));
    rep.insert("flagged_count".into(), json!(flagged.len()));
    println!("{}", py_pretty_ordered(&Value::Object(rep)));
    code
}

// ---------- main ----------

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--version" => {
                println!("{VERSION}");
                exit(0);
            }
            "--" => {
                i += 1;
                break;
            }
            a if a.starts_with("--") => usage_fail(&format!("未知旗标: {a}")),
            _ => break,
        }
    }
    if i >= args.len() {
        usage_fail("缺子命令");
    }
    let sub = args[i].clone();
    let rest: Vec<String> = args[i + 1..].to_vec();
    let code = match sub.as_str() {
        "run" => {
            let mut counts = "counts".to_string();
            let mut command: Vec<String> = vec![];
            let mut j = 0usize;
            let mut in_command = false;
            while j < rest.len() {
                if in_command {
                    command.push(rest[j].clone());
                    j += 1;
                    continue;
                }
                if rest[j] == "--counts" {
                    j += 1;
                    if j >= rest.len() {
                        usage_fail("--counts 缺值");
                    }
                    counts = rest[j].clone();
                } else if let Some(v) = rest[j].strip_prefix("--counts=") {
                    counts = v.to_string();
                } else if rest[j] == "--" {
                    in_command = true;
                } else if rest[j].starts_with("--") {
                    usage_fail(&format!("未知旗标: {}", rest[j]));
                } else {
                    in_command = true;
                    continue;
                }
                j += 1;
            }
            cmd_run(&counts, command)
        }
        "count" => {
            let mut counts = "counts".to_string();
            let mut tool: Option<String> = None;
            let mut date: Option<String> = None;
            let mut quiet = false;
            let mut j = 0usize;
            while j < rest.len() {
                match rest[j].as_str() {
                    "--counts" => {
                        j += 1;
                        if j >= rest.len() {
                            usage_fail("--counts 缺值");
                        }
                        counts = rest[j].clone();
                    }
                    "--tool" => {
                        j += 1;
                        if j >= rest.len() {
                            usage_fail("--tool 缺值");
                        }
                        tool = Some(rest[j].clone());
                    }
                    "--date" => {
                        j += 1;
                        if j >= rest.len() {
                            usage_fail("--date 缺值");
                        }
                        date = Some(rest[j].clone());
                    }
                    "--quiet" => quiet = true,
                    a if a.starts_with("--") => usage_fail(&format!("未知旗标: {a}")),
                    a => usage_fail(&format!("意外位置参数: {a}")),
                }
                j += 1;
            }
            cmd_count(&counts, tool.as_deref(), date.as_deref(), quiet)
        }
        "crosscheck" => {
            let mut trails: Vec<String> = vec![];
            let mut counts = "counts".to_string();
            let mut quiet = false;
            let mut j = 0usize;
            while j < rest.len() {
                match rest[j].as_str() {
                    "--trail" => {
                        j += 1;
                        while j < rest.len() && !rest[j].starts_with("--") {
                            trails.push(rest[j].clone());
                            j += 1;
                        }
                        continue;
                    }
                    "--counts" => {
                        j += 1;
                        if j >= rest.len() {
                            usage_fail("--counts 缺值");
                        }
                        counts = rest[j].clone();
                    }
                    "--quiet" => quiet = true,
                    a if a.starts_with("--") => usage_fail(&format!("未知旗标: {a}")),
                    a => usage_fail(&format!("意外位置参数: {a}")),
                }
                j += 1;
            }
            if trails.is_empty() {
                usage_fail("缺 --trail（必填，可多个）");
            }
            cmd_crosscheck(&trails, &counts, quiet)
        }
        other => usage_fail(&format!("未知子命令: {other}")),
    };
    exit(code);
}
