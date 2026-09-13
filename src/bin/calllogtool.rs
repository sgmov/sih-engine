//! 引擎侧行式账本（Calllog）命令行面 —— lease-mergeleg23-parallel 簇F 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 选名申报：bin 名取 calllogtool，避开首战批（lease-mergeall-parallel）已入引擎
//! src/lease 的 calllog 模块名，防目标名混淆。
//!
//! 行为面移植自围堰 sih-tools/calllog 0.1.0（src/calllog/：core.py 362 行、cli.py
//! 77 行），只读对表移植，围堰源码零改动。核心行为链：append 三腿齐落（ndjson
//! 权威腿 flock + O_APPEND 原子整行落账、该 tool 的 markdown 投影腿同临界区
//! verbatim 重组再生）与 render 单 tool 投影再生与 reconcile 权威腿对账。
//!
//! CLI：`calllogtool --root <工作区根> <append|render|reconcile|rebuild|import> ...`
//! 退出码：0 = 成；2 = 用法错或未实装位（与围堰 argparse 及 import 占位对齐）。
//!
//! 落差申报（相对围堰）：
//! 1. sqlite 索引腿未实装（依赖面零新增约束，rusqlite 不在引擎许可依赖集，承 A6
//!    零新增申报）：append 只落 ndjson 权威腿 + 投影腿两腿；rebuild 子命令未实装，
//!    调用即 stderr 报未实装、退出码 2；reconcile 的 db_set 恒按空处理（calls.db
//!    在盘亦不读，db_only 恒空、ndjson_only 即全量权威腿 id）。DB_SCHEMA_VERSION
//!    常数保留在册。
//! 2. import 子命令与围堰同形：暂不实现，stderr 占位、退出码 2。
//! 3. append 入参校验失败（tool/occasion/commands 空）围堰为未捕获 ValueError 栈
//!    回退退出 1，本件 stderr 单行报文退出 1，报文形不对齐、退出码对齐。
//! 4. event_id 随机源取 uuid v4 前 4 字节大端（围堰 os.urandom(4)），分布等价。
//! 5. flock 可重入计数表（同线程同路径重入只计数）未移植——bin 为单线程单锁窗，
//!    无重入路径。
//! 6. Python json 浮点最短表示边缘形未对齐（serde_json vs repr）。

use serde_json::{json, Map, Value};
use std::fs;
use std::io::Write;
use std::os::unix::io::RawFd;
use std::path::{Path, PathBuf};
use std::process::exit;

use uuid::Uuid;

/// 围堰版本锚：sih-tools/calllog/src/calllog/__init__.py __version__。
#[allow(dead_code)]
const VERSION: &str = "0.1.0";
const CALLS_NDJSON_NAME: &str = "calls.ndjson";
const CALLS_DB_NAME: &str = "calls.db";
/// sqlite 索引腿 schema 版本在册（索引腿未实装，见头注落差一）。
#[allow(dead_code)]
const DB_SCHEMA_VERSION: i64 = 1;
const EVENT_ID_PREFIX: &str = "clog";

/// 19 工具投影面（core.py PROJECTION_PATHS 逐字对表）。
const PROJECTION_PATHS: [&str; 19] = [
    "sih-tools/cascade/CALL-LOG.md",
    "sih-tools/elicit/CALL-LOG.md",
    "sih-tools/facet/CALL-LOG.md",
    "sih-tools/formatter/CALL-LOG.md",
    "sih-tools/gauge/CALL-LOG.md",
    "sih-tools/identity/CALL-LOG.md",
    "sih-tools/latex-helper/CALL-LOG.md",
    "sih-tools/lease/CALL-LOG.md",
    "sih-tools/locator/CALL-LOG.md",
    "sih-tools/locks/CALL-LOG.md",
    "sih-tools/meter/CALL-LOG.md",
    "sih-tools/nomenclator/CALL-LOG.md",
    "sih-tools/parser/CALL-LOG.md",
    "sih-tools/scribe/CALL-LOG.md",
    "sih-tools/scrutinator/CALL-LOG.md",
    "sih-tools/selector/CALL-LOG.md",
    "sih-tools/tally/CALL-LOG.md",
    "sih-tools/watchcheck/CALL-LOG.md",
    "sih-tools/wikirecall/CALL-LOG.md",
];

// ---------- Python json 规范形（见头注落差六） ----------

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

fn write_compact_sorted(v: &Value, out: &mut String) {
    match v {
        Value::Object(m) => {
            out.push('{');
            let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
            kvs.sort_by(|a, b| a.0.cmp(b.0));
            for (i, (k, val)) in kvs.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str(&py_escape(k));
                out.push(':');
                write_compact_sorted(val, out);
            }
            out.push('}');
        }
        Value::Array(a) => {
            out.push('[');
            for (i, val) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_compact_sorted(val, out);
            }
            out.push(']');
        }
        other => out.push_str(&py_scalar(other)),
    }
}

/// json.dumps(v, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
fn py_compact_sorted(v: &Value) -> String {
    let mut s = String::new();
    write_compact_sorted(v, &mut s);
    s
}

// ---------- 写点互斥：flock（lease.ledgerwrite 同源形） ----------

struct FlockGuard {
    fd: RawFd,
}

fn flock_acquire(target: &Path) -> Result<FlockGuard, String> {
    let mut lock_os = target.as_os_str().to_owned();
    lock_os.push(".lock");
    let lock_path = PathBuf::from(lock_os);
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("lock 目录创建失败: {e}"))?;
    }
    let cpath = std::ffi::CString::new(lock_path.to_string_lossy().as_bytes())
        .map_err(|_| "lock 路径含 NUL".to_string())?;
    let fd = unsafe { libc::open(cpath.as_ptr(), libc::O_RDWR | libc::O_CREAT, 0o644) };
    if fd < 0 {
        return Err(format!("lock 打开失败: {}", lock_path.display()));
    }
    if unsafe { libc::flock(fd, libc::LOCK_EX) } != 0 {
        unsafe { libc::close(fd) };
        return Err(format!("lock 取锁失败: {}", lock_path.display()));
    }
    Ok(FlockGuard { fd })
}

impl Drop for FlockGuard {
    fn drop(&mut self) {
        unsafe {
            libc::flock(self.fd, libc::LOCK_UN);
            libc::close(self.fd);
        }
    }
}

// ---------- 权威腿与投影腿 ----------

/// 已持锁窗内的 O_APPEND 原子整行写（append 单锁窗承载多腿用）。
fn write_row_unlocked(ndjson_path: &Path, event: &Value) -> Result<usize, String> {
    let payload = format!("{}\n", py_compact_sorted(event));
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(ndjson_path)
        .map_err(|e| format!("ndjson 打开失败: {e}"))?;
    f.write_all(payload.as_bytes()).map_err(|e| format!("ndjson 写入失败: {e}"))?;
    Ok(payload.len())
}

fn projection_path(root: &Path, tool: &str) -> PathBuf {
    root.join("sih-tools").join(tool).join("CALL-LOG.md")
}

fn default_ndjson(root: &Path) -> PathBuf {
    root.join("sih-tools").join("calllog").join(CALLS_NDJSON_NAME)
}

/// 投影腿再生：该 tool 的账本行按序取 verbatim 字段重组（projfix-solo T-1）。
fn render_projection_for_tool(root: &Path, tool: &str) -> Result<(), String> {
    let ndjson = default_ndjson(root);
    let target = projection_path(root, tool);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("投影目录创建失败: {e}"))?;
    }
    let mut rows: Vec<Value> = vec![];
    if ndjson.exists() {
        let text = fs::read_to_string(&ndjson).map_err(|e| format!("ndjson 读取失败: {e}"))?;
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let obj: Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if obj.get("tool").and_then(|v| v.as_str()) == Some(tool) {
                rows.push(obj);
            }
        }
    }
    let mut text = rows
        .iter()
        .map(|r| r.get("verbatim").and_then(|v| v.as_str()).unwrap_or("").to_string())
        .collect::<Vec<String>>()
        .join("\n");
    if !text.is_empty() {
        text.push('\n');
    }
    fs::write(&target, text).map_err(|e| format!("投影写入失败: {e}"))
}

/// 权威腿读全：逐行 JSON 解析，错误行抛 ValueError（退出码 1 路径）。
fn read_ndjson(ndjson_path: &Path) -> Result<Vec<Value>, String> {
    if !ndjson_path.exists() {
        return Ok(vec![]);
    }
    let text = fs::read_to_string(ndjson_path).map_err(|e| format!("{e}"))?;
    let mut rows = vec![];
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<Value>(line) {
            Ok(v) => rows.push(v),
            Err(e) => return Err(format!("ndjson line {} invalid: {e}", number + 1)),
        }
    }
    Ok(rows)
}

fn new_event_id() -> String {
    let bytes = *Uuid::new_v4().as_bytes();
    let n = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    format!("{EVENT_ID_PREFIX}-{n:08x}")
}

fn now_iso_seconds() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S+00:00").to_string()
}

/// 三腿齐落（本件两腿：sqlite 索引腿未实装，见头注落差一）。
fn append_call(
    tool: &str,
    occasion: &str,
    commands: &str,
    exit_code: &str,
    session: Option<&str>,
    note: Option<&str>,
    root: &Path,
) -> Result<Value, String> {
    if tool.is_empty() {
        return Err("tool 必填字符串".to_string());
    }
    if occasion.is_empty() {
        return Err("occasion 必填字符串".to_string());
    }
    if commands.is_empty() {
        return Err("commands 必填字符串".to_string());
    }
    let ndjson_path = default_ndjson(root);
    let at_iso = now_iso_seconds();
    let date_part = at_iso.get(..10).unwrap_or("").to_string();
    let verbatim_row = format!(
        "| {date_part} | {occasion} | {commands} | {exit_code} | {} | {} |",
        session.unwrap_or(""),
        note.unwrap_or("")
    );
    let mut event = Map::new();
    event.insert("at".into(), json!(at_iso));
    event.insert("commands".into(), json!(commands));
    event.insert("event_id".into(), json!(new_event_id()));
    event.insert("exit".into(), json!(exit_code));
    event.insert("note".into(), json!(note.unwrap_or("")));
    event.insert("occasion".into(), json!(occasion));
    event.insert("session".into(), json!(session.unwrap_or("")));
    event.insert("tool".into(), json!(tool));
    event.insert("verbatim".into(), json!(verbatim_row));
    let event = Value::Object(event);
    // 单锁窗包两腿（围堰三腿同窗，本件 sqlite 索引腿未实装）
    let _guard = flock_acquire(&ndjson_path)?;
    write_row_unlocked(&ndjson_path, &event)?;
    render_projection_for_tool(root, tool)?;
    Ok(event)
}

// ---------- reconcile ----------

fn reconcile(root: &Path) -> Result<Value, String> {
    let ndjson = default_ndjson(root);
    // sqlite 索引腿未实装：db_set 恒空（见头注落差一）
    let nd_rows = read_ndjson(&ndjson)?;
    let mut nd_set: Vec<String> = nd_rows
        .iter()
        .filter_map(|r| r.get("event_id").and_then(|v| v.as_str()).map(|s| s.to_string()))
        .collect();
    nd_set.sort();
    nd_set.dedup();
    let mut per_tool = Map::new();
    for proj in PROJECTION_PATHS {
        let tool = proj.split('/').rev().nth(1).unwrap_or("");
        let proj_path = root.join(proj);
        let mut proj_set: Vec<String> = vec![];
        if proj_path.exists() {
            let text = fs::read_to_string(&proj_path).map_err(|e| format!("{e}"))?;
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Ok(obj) = serde_json::from_str::<Value>(line) {
                    if let Some(eid) = obj.get("event_id").and_then(|v| v.as_str()) {
                        proj_set.push(eid.to_string());
                    }
                }
            }
        }
        proj_set.sort();
        proj_set.dedup();
        per_tool.insert(
            tool.to_string(),
            json!({
                "missing_in_projection": [],
                "ndjson_ids_for_tool": nd_set,
                "projection_ids": proj_set,
            }),
        );
    }
    Ok(json!({
        "ndjson_ids": nd_set,
        "db_only": [],
        "ndjson_only": nd_set,
        "per_tool": per_tool,
    }))
}

// ---------- CLI ----------

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: calllogtool --root <工作区根> <append|render|reconcile|rebuild|import> [子命令参数]");
    exit(2);
}

/// 围堰未捕获 ValueError 路径近似（见头注落差三）：stderr 单行、退出 1。
fn die_value_error(msg: &str) -> ! {
    eprintln!("ValueError: {msg}");
    exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0usize;
    let mut root: Option<String> = None;
    while i < args.len() {
        if args[i] == "--root" {
            i += 1;
            if i >= args.len() {
                usage_fail("--root 缺值");
            }
            root = Some(args[i].clone());
            i += 1;
        } else if let Some(v) = args[i].strip_prefix("--root=") {
            root = Some(v.to_string());
            i += 1;
        } else if args[i].starts_with("--") {
            usage_fail(&format!("未知旗标: {}", args[i]));
        } else {
            break;
        }
    }
    let root_s = match root {
        Some(r) => r,
        None => usage_fail("缺 --root（必填）"),
    };
    if i >= args.len() {
        usage_fail("缺子命令");
    }
    let root = PathBuf::from(&root_s);
    let sub = args[i].clone();
    let rest: Vec<String> = args[i + 1..].to_vec();

    match sub.as_str() {
        "append" => {
            let mut tool: Option<String> = None;
            let mut occasion: Option<String> = None;
            let mut commands: Option<String> = None;
            let mut exit_code: Option<String> = None;
            let mut session: Option<String> = None;
            let mut note: Option<String> = None;
            let mut j = 0usize;
            while j < rest.len() {
                let take = |name: &str, slot: &mut Option<String>, j: &mut usize| {
                    *j += 1;
                    if *j >= rest.len() {
                        usage_fail(&format!("{name} 缺值"));
                    }
                    *slot = Some(rest[*j].clone());
                };
                match rest[j].as_str() {
                    "--tool" => take("--tool", &mut tool, &mut j),
                    "--occasion" => take("--occasion", &mut occasion, &mut j),
                    "--commands" => take("--commands", &mut commands, &mut j),
                    "--exit" => take("--exit", &mut exit_code, &mut j),
                    "--session" => take("--session", &mut session, &mut j),
                    "--note" => take("--note", &mut note, &mut j),
                    a if a.starts_with("--") => usage_fail(&format!("未知旗标: {a}")),
                    a => usage_fail(&format!("意外位置参数: {a}")),
                }
                j += 1;
            }
            let tool = tool.unwrap_or_else(|| usage_fail("缺 --tool（必填）"));
            let occasion = occasion.unwrap_or_else(|| usage_fail("缺 --occasion（必填）"));
            let commands = commands.unwrap_or_else(|| usage_fail("缺 --commands（必填）"));
            let exit_code = exit_code.unwrap_or_else(|| usage_fail("缺 --exit（必填）"));
            match append_call(
                &tool,
                &occasion,
                &commands,
                &exit_code,
                session.as_deref(),
                note.as_deref(),
                &root,
            ) {
                Ok(event) => {
                    println!("{}", py_compact_sorted(&event));
                    exit(0);
                }
                Err(msg) => die_value_error(&msg),
            }
        }
        "render" => {
            let mut tool: Option<String> = None;
            let mut j = 0usize;
            while j < rest.len() {
                if rest[j] == "--tool" {
                    j += 1;
                    if j >= rest.len() {
                        usage_fail("--tool 缺值");
                    }
                    tool = Some(rest[j].clone());
                } else if rest[j].starts_with("--") {
                    usage_fail(&format!("未知旗标: {}", rest[j]));
                } else {
                    usage_fail(&format!("意外位置参数: {}", rest[j]));
                }
                j += 1;
            }
            let tool = tool.unwrap_or_else(|| usage_fail("缺 --tool（必填）"));
            if let Err(e) = render_projection_for_tool(&root, &tool) {
                die_value_error(&e);
            }
            exit(0);
        }
        "reconcile" => {
            if !rest.is_empty() {
                usage_fail("reconcile 不接受参数");
            }
            match reconcile(&root) {
                Ok(report) => {
                    println!("{}", py_compact_sorted(&report));
                    exit(0);
                }
                Err(msg) => die_value_error(&msg),
            }
        }
        "rebuild" => {
            // 落差申报一：sqlite 索引腿未实装。
            eprintln!(
                "calllogtool rebuild: 未实装（sqlite 索引腿未移植，依赖面零新增约束，落差见 bin 头注；权威腿 {CALLS_NDJSON_NAME} 与索引腿 {CALLS_DB_NAME} 语义在册 DB_SCHEMA_VERSION={DB_SCHEMA_VERSION}）"
            );
            exit(2);
        }
        "import" => {
            eprintln!("calllogtool import: 暂不实现，留作后继");
            exit(2);
        }
        other => usage_fail(&format!("未知子命令: {other}")),
    }
}
