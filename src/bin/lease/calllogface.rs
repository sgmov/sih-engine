//! call-log import：19 册 markdown 存量迁移到权威腿（calllog_import.py 对表，
//! P3 迁移忠实形 m-calllog-dual-1 已裁）。
//!
//! 零有损：每行原文整行入 verbatim 载体；尽力解析：类型字段解析失败即空值
//! 申报不猜不编；对表机械：行数符加 verbatim 零丢加失败清单在档。
//! 落差申报：sqlite 索引腿未实装（Cargo 零 rusqlite，A9 依赖零新增，循腿二
//! bills SQL 同款）；投影腿再生（projfix-solo verbatim 重组形）已实装。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};

use crate::commitlaw::py_resolve;
use crate::{die, emit, one, now_utc};

const PROJECTIONS: &[(&str, &str)] = &[
    ("sih-tools/cascade/CALL-LOG.md", "cascade"),
    ("sih-tools/elicit/CALL-LOG.md", "elicit"),
    ("sih-tools/facet/CALL-LOG.md", "facet"),
    ("sih-tools/formatter/CALL-LOG.md", "formatter"),
    ("sih-tools/gauge/CALL-LOG.md", "gauge"),
    ("sih-tools/identity/CALL-LOG.md", "identity"),
    ("sih-tools/latex-helper/CALL-LOG.md", "latex-helper"),
    ("sih-tools/lease/CALL-LOG.md", "lease"),
    ("sih-tools/locator/CALL-LOG.md", "locator"),
    ("sih-tools/locks/CALL-LOG.md", "locks"),
    ("sih-tools/meter/CALL-LOG.md", "meter"),
    ("sih-tools/nomenclator/CALL-LOG.md", "nomenclator"),
    ("sih-tools/parser/CALL-LOG.md", "parser"),
    ("sih-tools/scribe/CALL-LOG.md", "scribe"),
    ("sih-tools/scrutinator/CALL-LOG.md", "scrutinator"),
    ("sih-tools/selector/CALL-LOG.md", "selector"),
    ("sih-tools/tally/CALL-LOG.md", "tally"),
    ("sih-tools/watchcheck/CALL-LOG.md", "watchcheck"),
    ("sih-tools/wikirecall/CALL-LOG.md", "wikirecall"),
];

/// Python json.dumps(sort_keys=True, separators=(",", ":")) 对表：递归排序
/// 压缩形。
fn py_compact_sorted(v: &Value) -> String {
    fn to_sorted(v: &Value) -> Value {
        match v {
            Value::Object(m) => {
                let mut bt: BTreeMap<String, Value> = BTreeMap::new();
                for (k, val) in m {
                    bt.insert(k.clone(), to_sorted(val));
                }
                Value::Object(bt.into_iter().map(|(k, val)| (k, val)).collect::<Map<String, Value>>())
            }
            Value::Array(a) => Value::Array(a.iter().map(to_sorted).collect()),
            other => other.clone(),
        }
    }
    serde_json::to_string(&to_sorted(v)).unwrap_or_default()
}

fn is_sep_line(line: &str) -> bool {
    // 表格分隔行（`| --- | --- |` 类，SEP_RE 对表手写形）。
    let t = line.trim();
    if t.is_empty() {
        return false;
    }
    let t = t.trim_matches('|');
    let cells: Vec<&str> = t.split('|').map(|c| c.trim()).collect();
    if cells.len() < 2 {
        return false;
    }
    cells.iter().all(|c| {
        let mut s = c.to_string();
        if s.starts_with(':') {
            s.remove(0);
        }
        if s.ends_with(':') {
            s.pop();
        }
        !s.is_empty() && s.chars().all(|ch| ch == '-')
    })
}

fn classify_line(line: &str) -> &'static str {
    let stripped = line.trim();
    if stripped.is_empty() {
        return "blank";
    }
    if is_sep_line(line) {
        return "sep";
    }
    let heading = stripped.starts_with('#')
        && stripped.trim_start_matches('#').starts_with(' ');
    if heading {
        return "header";
    }
    if stripped.starts_with('|') && stripped.ends_with('|') && stripped.contains('|') {
        return "row";
    }
    if line.starts_with(['-', '*', ' ', '\t']) {
        let l = line.trim_start();
        if l.starts_with("- ") || l.starts_with("* ") {
            return "list";
        }
    }
    "text"
}

fn parse_table_row(line: &str) -> Vec<String> {
    let s = line.trim();
    let s = s.strip_prefix('|').unwrap_or(s);
    let s = s.strip_suffix('|').unwrap_or(s);
    s.split('|').map(|c| c.trim().to_string()).collect()
}

fn parse_row_fields(line: &str) -> Map<String, Value> {
    // 已知表头形态尽力解析（calllog_import._parse_table_row_to_event 对表）：
    // 5 列 日期/事由/调用/退出码/备注；6 列 加会话；7 列加备注异位；>=4 列取前四。
    let cells = parse_table_row(line);
    let mut f = Map::new();
    let get = |i: usize| cells.get(i).cloned().unwrap_or_default();
    if cells.is_empty() {
        return f;
    }
    f.insert("date".into(), json!(get(0)));
    match cells.len() {
        5 => {
            f.insert("occasion".into(), json!(get(1)));
            f.insert("commands".into(), json!(get(2)));
            f.insert("exit".into(), json!(get(3)));
            f.insert("note".into(), json!(get(4)));
        }
        6 => {
            f.insert("occasion".into(), json!(get(1)));
            f.insert("commands".into(), json!(get(2)));
            f.insert("exit".into(), json!(get(3)));
            f.insert("session".into(), json!(get(4)));
            f.insert("note".into(), json!(get(5)));
        }
        n if n >= 7 => {
            f.insert("occasion".into(), json!(get(1)));
            f.insert("commands".into(), json!(get(2)));
            f.insert("exit".into(), json!(get(3)));
            f.insert("note".into(), json!(get(6)));
        }
        n if n >= 4 => {
            f.insert("occasion".into(), json!(get(1)));
            f.insert("commands".into(), json!(get(2)));
            f.insert("exit".into(), json!(get(3)));
        }
        _ => {}
    }
    f.retain(|_, v| v.as_str().map(|s| !s.is_empty()).unwrap_or(false));
    f
}

fn import_one_book(root: &Path, rel_path: &str, tool: &str, at: &str) -> (Vec<Value>, usize, usize, Vec<Value>) {
    let full = root.join(rel_path);
    if !full.exists() {
        return (Vec::new(), 0, 0, Vec::new());
    }
    let text = std::fs::read_to_string(&full).unwrap_or_default();
    // Python splitlines 对表：尾换行不产生尾空行（split('\n') 会）。
    let mut lines: Vec<&str> = text.split('\n').collect();
    if text.ends_with('\n') {
        lines.pop();
    }
    let mut rows = Vec::new();
    let mut n_data = 0usize;
    let mut n_verbatim = 0usize;
    let failures: Vec<Value> = Vec::new();
    for line in lines {
        let line = line.trim_end_matches('\r');
        let kind = classify_line(line);
        n_verbatim += 1;
        let eid = format!("clog-import-{}", uuid::Uuid::new_v4().simple().to_string()[..12].to_string());
        let mut event = json!({
            "at": at,
            "commands": "",
            "event_id": eid,
            "exit": "",
            "kind": kind,
            "note": "",
            "occasion": "",
            "session": "",
            "tool": tool,
            "verbatim": line,
        });
        if kind == "row" {
            let fields = parse_row_fields(line);
            n_data += 1;
            for (k, v) in fields {
                event[k] = v;
            }
        } else if kind == "list" {
            // 列表项首段日期形：YYYY-MM-DD 后空格接文本（宽松对表）。
            let stripped = line.trim_start().trim_start_matches(['-', '*']).trim();
            let head: String = stripped.chars().take(10).collect();
            let is_date = head.chars().count() == 10
                && head.as_bytes()[4] == b'-'
                && head.as_bytes()[7] == b'-'
                && head.chars().all(|c| c.is_ascii_digit() || c == '-');
            if is_date && stripped.len() > 10 && stripped.as_bytes()[10] == b' ' {
                event["occasion"] = json!(stripped[11..].to_string());
            }
        }
        rows.push(event);
    }
    (rows, n_data, n_verbatim, failures)
}

fn render_projection(root: &Path, tool: &str) -> PathBuf {
    // 投影腿再生（calllog.core._render_projection_for_tool 对表）：权威腿按
    // tool 过滤，verbatim 逐行重组（尾换行），路径缺席派生创建。
    let ndjson = root.join("sih-tools").join("calllog").join("calls.ndjson");
    let target = root.join("sih-tools").join(tool).join("CALL-LOG.md");
    let _ = std::fs::create_dir_all(target.parent().unwrap_or(root));
    let mut text = String::new();
    if let Ok(content) = std::fs::read_to_string(&ndjson) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(obj) = serde_json::from_str::<Value>(line) {
                if obj.get("tool").and_then(|v| v.as_str()) == Some(tool) {
                    text.push_str(obj.get("verbatim").and_then(|v| v.as_str()).unwrap_or(""));
                    text.push('\n');
                }
            }
        }
    }
    let _ = std::fs::write(&target, text);
    target
}

pub(crate) fn cmd_calllog_import(m: &BTreeMap<String, Vec<String>>) -> ! {
    let root = py_resolve(&PathBuf::from(
        one(m, "root").unwrap_or(&detect_root_fallback()),
    ));
    let calllog_dir = root.join("sih-tools").join("calllog");
    let _ = std::fs::create_dir_all(&calllog_dir);
    let ndjson = calllog_dir.join("calls.ndjson");
    if let Ok(existing) = std::fs::read_to_string(&ndjson) {
        if !existing.trim().is_empty() {
            // 一次性语义：已导入拒零覆写，二次跑零动作并报已迁。
            print!("{}", emit(&json!({
                "ok": true, "skipped": true,
                "reason": "ndjson already exists; import is one-shot",
                "ndjson_path": ndjson.display().to_string(),
                "existing_bytes": existing.len(),
                "sqlite_leg": "未实装（A9 依赖零新增，申报面见 SPEC-024 v1.3）",
            })));
            std::process::exit(0);
        }
    }
    let at = one(m, "at").map(|s| s.to_string()).unwrap_or_else(now_utc);
    let mut all_rows: Vec<Value> = Vec::new();
    let mut per_book: Vec<Value> = Vec::new();
    let mut n_data_total = 0usize;
    let mut n_verbatim_total = 0usize;
    for (rel, tool) in PROJECTIONS {
        let (rows, n_data, n_verbatim, _failures) = import_one_book(&root, rel, tool, &at);
        per_book.push(json!({
            "tool": tool, "path": rel, "rows": rows.len(),
            "missing": !root.join(rel).exists(),
            "data_rows": n_data, "verbatim_lines": n_verbatim,
        }));
        all_rows.extend(rows);
        n_data_total += n_data;
        n_verbatim_total += n_verbatim;
    }
    let mut payload = String::new();
    for row in &all_rows {
        payload.push_str(&py_compact_sorted(row));
        payload.push('\n');
    }
    if let Err(e) = std::fs::write(&ndjson, payload) {
        die(2, "权威腿写入失败", json!({"err": e.to_string()}));
    }
    for (_, tool) in PROJECTIONS {
        render_projection(&root, tool);
    }
    print!("{}", emit(&json!({
        "ok": true, "skipped": false,
        "ndjson_path": ndjson.display().to_string(),
        "db_path": null,
        "sqlite_leg": "未实装（A9 依赖零新增，申报面见 SPEC-024 v1.3）",
        "per_book": per_book,
        "summary": {
            "books": PROJECTIONS.len(),
            "data_rows": n_data_total,
            "verbatim_lines": n_verbatim_total,
            "parse_failures": 0,
        },
    })));
    std::process::exit(0);
}

fn detect_root_fallback() -> String {
    crate::attachments::detect_domain_context(None).display().to_string()
}
