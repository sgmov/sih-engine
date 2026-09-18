//! collect 面（pl-01）：三源候选汇总。
//!
//! 承 DES-017 编排三段之首「候选收集」：判据扫出参（--critsweep-json，
//! v1 只吃文件不内调 critsweep 进程）、泊界在泊候选（--parking-json 或
//! --parking-trail 只读扫 parking_entered 未出泊事件）、批候裁单
//!（--covenant 可多条，md/json 自由文本候裁项）。输出统一候选清单
//! candidates.json（kind=pendline-candidates，每项带 source/id/title/summary/
//! origin）。零判定语义：汇总即候选，不筛选不排序裁决。

use serde_json::{json, Map, Value};
use std::path::Path;

use super::{read_json, read_text, write_json, Args, PendError};

pub const CANDIDATES_KIND: &str = "pendline-candidates";
pub const CANDIDATES_VERSION: i64 = 1;

#[derive(Debug, Clone)]
pub struct Candidate {
    pub source: String,
    pub id: String,
    pub title: String,
    pub summary: String,
    pub origin: Value,
}

impl Candidate {
    pub fn to_json(&self) -> Value {
        json!({
            "source": self.source,
            "id": self.id,
            "title": self.title,
            "summary": self.summary,
            "origin": self.origin,
        })
    }
}

/// 子命令入口：汇总三源出候选清单，退出码 0；输入不可读或非法即 2。
pub fn run(args: &Args) -> Result<i32, PendError> {
    let out_path = args.get("out")?;
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut source_counts = Map::new();
    let mut notes: Vec<String> = Vec::new();

    // 源一：判据扫出参（判据沉底机械召回，status=sunk 入候选）。
    let mut critsweep_n = 0i64;
    for p in args.list("critsweep-json") {
        let v = read_json(Path::new(&p))?;
        let criteria = v
            .get("criteria")
            .and_then(|c| c.as_array())
            .ok_or_else(|| {
                PendError::usage(format!(
                    "--critsweep-json {} 非判据扫出参形（缺 criteria 数组）",
                    p
                ))
            })?;
        for crit in criteria {
            if crit.get("status").and_then(|s| s.as_str()) != Some("sunk") {
                continue;
            }
            let id = crit.get("id").map(value_plain).unwrap_or_default();
            let title = crit.get("title").map(value_plain).unwrap_or_default();
            candidates.push(Candidate {
                source: "critsweep".to_string(),
                summary: title.clone(),
                id,
                title,
                origin: json!({
                    "file": p,
                    "status": crit.get("status").cloned().unwrap_or(Value::Null),
                    "gap_days": crit.get("gap_days").cloned().unwrap_or(Value::Null),
                    "last_hit": crit.get("last_hit").cloned().unwrap_or(Value::Null),
                }),
            });
            critsweep_n += 1;
        }
    }
    source_counts.insert("critsweep".to_string(), json!(critsweep_n));

    // 源二：泊界在泊候选（--parking-json 直给形或 --parking-trail 只读扫形）。
    let mut parking_n = 0i64;
    for p in args.list("parking-json") {
        let v = read_json(Path::new(&p))?;
        for entry in parking_entries_of(&v) {
            candidates.push(parking_candidate(&entry, &p));
            parking_n += 1;
        }
    }
    for p in args.list("parking-trail") {
        let (entries, skipped) = scan_parking_trail(Path::new(&p))?;
        if skipped > 0 {
            notes.push(format!("{} 跳过 {} 行非 JSON 行", p, skipped));
        }
        for entry in entries {
            candidates.push(parking_candidate(&entry, &p));
            parking_n += 1;
        }
    }
    source_counts.insert("parking".to_string(), json!(parking_n));

    // 源三：批候裁单（md 取列表项；json 取数组或 items/covenant/candidates 键）。
    let mut covenant_n = 0i64;
    for p in args.list("covenant") {
        let path = Path::new(&p);
        let lower = p.to_ascii_lowercase();
        if lower.ends_with(".json") {
            let v = read_json(path)?;
            for item in covenant_items_of(&v) {
                let n = covenant_n + 1;
                let (id, title, summary) = match item {
                    Value::String(s) => (format!("covenant-{}", n), s.clone(), s),
                    Value::Object(_) => {
                        let id = item
                            .get("id")
                            .map(value_plain)
                            .filter(|s| !s.is_empty())
                            .unwrap_or_else(|| format!("covenant-{}", n));
                        let title = item.get("title").map(value_plain).unwrap_or_else(|| id.clone());
                        let summary = item
                            .get("summary")
                            .map(value_plain)
                            .filter(|s| !s.is_empty())
                            .unwrap_or_else(|| title.clone());
                        (id, title, summary)
                    }
                    _ => continue,
                };
                candidates.push(Candidate {
                    source: "covenant".to_string(),
                    summary,
                    origin: json!({"file": p}),
                    id,
                    title,
                });
                covenant_n += 1;
            }
        } else {
            let text = read_text(path)?;
            for line in text.lines() {
                let t = line.trim();
                let body = t
                    .strip_prefix("- ")
                    .or_else(|| t.strip_prefix("* "))
                    .map(|b| b.trim().to_string());
                let body = match body {
                    Some(b) => Some(b),
                    None => {
                        // 有序列表项形「3. 」
                        let mut it = t.splitn(2, ". ");
                        match (it.next(), it.next()) {
                            (Some(head), Some(rest))
                                if !head.is_empty() && head.chars().all(|c| c.is_ascii_digit()) =>
                            {
                                Some(rest.trim().to_string())
                            }
                            _ => None,
                        }
                    }
                };
                let body = match body {
                    Some(b) if !b.is_empty() => b,
                    _ => continue,
                };
                let n = covenant_n + 1;
                candidates.push(Candidate {
                    source: "covenant".to_string(),
                    id: format!("covenant-{}", n),
                    title: body.clone(),
                    summary: body,
                    origin: json!({"file": p}),
                });
                covenant_n += 1;
            }
        }
    }
    source_counts.insert("covenant".to_string(), json!(covenant_n));

    let out = json!({
        "kind": CANDIDATES_KIND,
        "version": CANDIDATES_VERSION,
        "collected_at": super::now_iso(),
        "source_counts": Value::Object(source_counts),
        "notes": notes,
        "candidates": candidates.iter().map(|c| c.to_json()).collect::<Vec<Value>>(),
    });
    write_json(Path::new(&out_path), &out)?;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "collected": candidates.len(),
            "out": out_path,
        }))
        .unwrap_or_default()
    );
    Ok(0)
}

fn value_plain(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// parking-json 形：顶层数组或对象 entries/candidates/items 键数组。
fn parking_entries_of(v: &Value) -> Vec<Value> {
    match v {
        Value::Array(a) => a.clone(),
        Value::Object(_) => ["entries", "candidates", "items"]
            .iter()
            .find_map(|k| v.get(*k).and_then(|x| x.as_array()).cloned())
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn parking_candidate(entry: &Value, file: &str) -> Candidate {
    let id = entry
        .get("entry_id")
        .or_else(|| entry.get("id"))
        .map(value_plain)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "parking-unknown".to_string());
    let title = entry
        .get("title")
        .map(value_plain)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| id.clone());
    let exit_condition = entry
        .get("exit_condition")
        .map(value_plain)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| title.clone());
    Candidate {
        source: "parking".to_string(),
        summary: exit_condition.clone(),
        id,
        title,
        origin: json!({
            "file": file,
            "exit_condition": exit_condition,
            "ttl_days": entry.get("ttl_days").cloned().unwrap_or(Value::Null),
        }),
    }
}

/// trail 只读扫：parking_entered/parking_exited 逐行重放，末态在泊者出候选。
/// 返回（在泊候选明细，跳过的非 JSON 行数）。
fn scan_parking_trail(path: &Path) -> Result<(Vec<Value>, usize), PendError> {
    let text = read_text(path)?;
    let mut parked: Map<String, Value> = Map::new();
    let mut skipped = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let event: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => {
                skipped += 1;
                continue;
            }
        };
        let et = event.get("event_type").and_then(|v| v.as_str()).unwrap_or("");
        let details = event.get("details").cloned().unwrap_or_else(|| json!({}));
        let entry_id = details
            .get("entry_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if entry_id.is_empty() {
            continue;
        }
        match et {
            "parking_entered" => {
                parked.insert(entry_id, details);
            }
            "parking_exited" => {
                parked.remove(&entry_id);
            }
            _ => {}
        }
    }
    Ok((parked.values().cloned().collect(), skipped))
}

/// covenant-json 形：顶层数组或对象 items/covenant/candidates 键数组。
fn covenant_items_of(v: &Value) -> Vec<Value> {
    match v {
        Value::Array(a) => a.clone(),
        Value::Object(_) => ["items", "covenant", "candidates"]
            .iter()
            .find_map(|k| v.get(*k).and_then(|x| x.as_array()).cloned())
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}
