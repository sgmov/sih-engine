//! scribegate 即书简融回命令行面，承接 SPEC-006#boundary 与 T6。
//!
//! 子命令五件即 append、verify、query、intent、park，另携 vectors 冻结向量集。
//! 退出码三值即零成功、一校验或检索异常、二工具自身异常。

use chrono::Utc;
use serde_json::json;
use sih_engine::event_stream::event::{Actor, ActorType};
use sih_engine::event_stream::{
    append, certification_event, compute_event_hash, intent_event, load_events, park_event,
    query, verify, Event, EventFilter, VerifyRange, GENESIS_PREV_HASH,
};
use std::path::PathBuf;
use std::process::exit;

fn gate_actor() -> Actor {
    Actor {
        actor_id: "scribegate".to_string(),
        actor_type: ActorType::System,
        invoked_via: "cli".to_string(),
    }
}

fn read_text(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn emit(value: serde_json::Value, code: i32) -> ! {
    println!("{}", serde_json::to_string_pretty(&value).unwrap());
    exit(code)
}

fn load_store(trail: &str) -> Vec<Event> {
    let path = PathBuf::from(trail);
    if !path.exists() {
        return Vec::new();
    }
    match load_events(path.as_path()) {
        Ok(events) => events,
        Err(e) => emit(json!({"error": format!("trail 不可读 {e:?}")}), 2),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("");
    let mut opts = std::collections::HashMap::new();
    let mut i = 2;
    while i + 1 < args.len() + 1 && i < args.len() {
        if args[i].starts_with("--") && i + 1 < args.len() {
            opts.insert(args[i].trim_start_matches('-').to_string(), args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    let opt = |k: &str| opts.get(k).cloned();

    match cmd {
        "verify" => {
            let Some(trail) = opt("trail") else { emit(json!({"error": "缺 --trail"}), 2) };
            let Some(text) = read_text(&trail) else {
                emit(json!({"error": "trail 不存在"}), 2)
            };
            let _ = text;
            let events = load_store(&trail);
            match verify(&events, VerifyRange::Full) {
                Ok(ok) => emit(
                    json!({
                        "status": "valid",
                        "events": ok.event_count,
                        "first_hash": ok.first_hash,
                        "last_hash": ok.last_hash,
                    }),
                    0,
                ),
                Err(e) => emit(json!({"status": "broken", "error": format!("{e:?}")}), 1),
            }
        }
        "query" => {
            let Some(trail) = opt("trail") else { emit(json!({"error": "缺 --trail"}), 2) };
            let events = load_store(&trail);
            if events.is_empty() && !PathBuf::from(&trail).exists() {
                emit(json!({"error": "trail 不存在"}), 2);
            }
            let filter = EventFilter {
                event_type: opt("event-type"),
                doc_id: opt("doc-id"),
                ..Default::default()
            };
            let list = query(&events, Some(filter), None);
            if list.events.is_empty() {
                emit(json!({"matches": 0}), 1);
            }
            emit(json!({"matches": list.events.len(), "events": list.events}), 0);
        }
        "append" => {
            let (Some(report), Some(exit_code), Some(trail)) =
                (opt("report"), opt("exit-code"), opt("trail"))
            else {
                emit(json!({"error": "缺参"}), 2)
            };
            let Ok(exit_code) = exit_code.parse::<i32>() else {
                emit(json!({"error": "退出码非数"}), 2)
            };
            let Some(text) = read_text(&report) else { emit(json!({"error": "报告不存在"}), 2) };
            let input = match certification_event(
                PathBuf::from(&report).as_path(),
                &text,
                exit_code,
                gate_actor(),
                Utc::now(),
            ) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("报告不识别 {e:?}")}), 2),
            };
            let mut store = load_store(&trail);
            match append(input, &mut store, Some(&PathBuf::from(&trail))) {
                Ok(ok) => emit(
                    json!({"status": "appended", "event_id": ok.event_id, "event_hash": ok.event_hash}),
                    0,
                ),
                Err(e) => emit(json!({"error": format!("写入拒 {e:?}")}), 1),
            }
        }
        "intent" => {
            let (Some(record), Some(validation), Some(trail)) =
                (opt("record"), opt("validation"), opt("trail"))
            else {
                emit(json!({"error": "缺参"}), 2)
            };
            let (Some(rtext), Some(vtext)) = (read_text(&record), read_text(&validation)) else {
                emit(json!({"error": "双件缺失"}), 2)
            };
            let input = match intent_event(
                PathBuf::from(&record).as_path(),
                &rtext,
                PathBuf::from(&validation).as_path(),
                &vtext,
                gate_actor(),
                Utc::now(),
            ) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("意图拒 {e:?}")}), 1),
            };
            let mut store = load_store(&trail);
            match append(input, &mut store, Some(&PathBuf::from(&trail))) {
                Ok(ok) => emit(
                    json!({"status": "appended", "event_id": ok.event_id, "event_hash": ok.event_hash}),
                    0,
                ),
                Err(e) => emit(json!({"error": format!("写入拒 {e:?}")}), 1),
            }
        }
        "park" => {
            let (Some(record), Some(trail)) = (opt("record"), opt("trail")) else {
                emit(json!({"error": "缺参"}), 2)
            };
            let Some(text) = read_text(&record) else { emit(json!({"error": "记录不存在"}), 2) };
            let store = load_store(&trail);
            let input = match park_event(&text, &store, gate_actor(), Utc::now()) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("停泊拒 {e:?}")}), 1),
            };
            let mut store = store;
            match append(input, &mut store, Some(&PathBuf::from(&trail))) {
                Ok(ok) => emit(
                    json!({"status": "appended", "event_id": ok.event_id, "event_hash": ok.event_hash}),
                    0,
                ),
                Err(e) => emit(json!({"error": format!("写入拒 {e:?}")}), 1),
            }
        }
        "vectors" => {
            let Some(out) = opt("write") else { emit(json!({"error": "缺 --write"}), 2) };
            let vectors = golden_vectors();
            let text = serde_json::to_string_pretty(&vectors).unwrap();
            std::fs::write(&out, text).expect("向量集写入");
            emit(json!({"status": "written", "count": vectors.as_array().map(|a| a.len()).unwrap_or(0)}), 0);
        }
        _ => emit(
            json!({"error": "用法 scribegate <append|verify|query|intent|park|vectors> --trail <路径>"}),
            2,
        ),
    }
}

fn golden_vectors() -> serde_json::Value {
    use chrono::{DateTime, Utc};
    use sih_engine::event_stream::event::{Actor, ActorType, Event};
    let base_actor = || Actor {
        actor_id: "scribegate".to_string(),
        actor_type: ActorType::System,
        invoked_via: "cli".to_string(),
    };
    let ts = |s: &str| DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc);
    let mut out = Vec::new();
    let mut build = |class: &str, event: Event| {
        let expected = compute_event_hash(&event);
        out.push(json!({
            "class": class,
            "name": event.doc_id.clone(),
            "event": serde_json::to_value(&event).unwrap(),
            "expected_hash": expected,
        }));
    };
    let mk = |event_id: &str, event_type: &str, timestamp: chrono::DateTime<Utc>,
              details: Option<serde_json::Value>, doc_id: &str,
              event_class: Option<String>, verification_result: Option<serde_json::Value>| {
        let mut e = Event {
            event_id: event_id.to_string(),
            event_type: event_type.to_string(),
            timestamp,
            actor: base_actor(),
            details,
            doc_id: doc_id.to_string(),
            prev_hash: GENESIS_PREV_HASH.to_string(),
            event_hash: String::new(),
            event_class,
            verification_result,
        };
        e.event_hash = compute_event_hash(&e);
        e
    };
    build("genesis", mk("00000000-0000-4000-8000-000000000001", "session_open",
        ts("2026-08-27T00:00:00Z"), Some(json!({"note": "genesis"})), "vec-genesis", None, None));
    build("details_null", mk("00000000-0000-4000-8000-000000000002", "session_close",
        ts("2026-08-27T00:00:01Z"), None, "vec-details-null", None, None));
    build("optional_absent", mk("00000000-0000-4000-8000-000000000003", "task_completion",
        ts("2026-08-27T00:00:02.500Z"), Some(json!({"task": "t"})), "vec-optional", None, None));
    build("non_ascii", mk("00000000-0000-4000-8000-000000000004", "certification_completed",
        ts("2026-08-27T00:00:03Z"), Some(json!({"词": "书简融回", "路径": "副本/核阅"})),
        "vec-非ASCII", Some("record_only".to_string()), None));
    build("timestamp_edge", mk("00000000-0000-4000-8000-000000000005", "parking_entered",
        ts("2026-08-27T23:59:59.999999+00:00"), Some(json!({"entry_id": "vec-5"})),
        "vec-ts-edge", Some("record_only".to_string()),
        Some(json!({"exit_code": 0, "findings": 0, "total": 0}))));
    json!(out)
}
