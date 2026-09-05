//! golden 向量集生成器，承接 scribe/CONTRACT.md 机器形态第六件即 golden 前置。
//!
//! 构造覆盖边界情形的事件样本，用与 hash.rs 完全相同的算法计算 event_hash，
//! 输出 JSON 向量集供工具线 scribe 复算对拍。哈希权威是 hash.rs 的
//! compute_event_hash，本文件中 payload 的重复构建仅为诊断材料。
//!
//! 用法：cargo run --example golden_vectors -- <out.json>

use chrono::{TimeZone, Utc};
use serde_json::json;
use sih_engine::event_stream::event::{Actor, ActorType, Event};
use sih_engine::event_stream::hash::{compute_event_hash, GENESIS_PREV_HASH};
use std::env;
use std::fs;

fn make_event(
    event_id: &str,
    event_type: &str,
    ts_nanos: u32,
    actor_id: &str,
    invoked_via: &str,
    details: Option<serde_json::Value>,
    doc_id: &str,
    prev_hash: &str,
    event_class: Option<&str>,
    verification_result: Option<serde_json::Value>,
) -> Event {
    let base = Utc
        .with_ymd_and_hms(2026, 8, 22, 12, 34, 56)
        .unwrap();
    let timestamp = base + chrono::Duration::nanoseconds(ts_nanos as i64);
    Event {
        event_id: event_id.to_string(),
        event_type: event_type.to_string(),
        timestamp,
        actor: Actor {
            actor_id: actor_id.to_string(),
            actor_type: ActorType::System,
            invoked_via: invoked_via.to_string(),
        },
        details,
        doc_id: doc_id.to_string(),
        prev_hash: prev_hash.to_string(),
        event_hash: String::new(),
        event_class: event_class.map(|s| s.to_string()),
        verification_result,
        session_id: None,
        identity_hash: None,
    }
}

/// 与 hash.rs 相同的 payload 构建，仅作诊断输出，权威在 compute_event_hash。
fn diagnostic_payload(event: &Event) -> String {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    map.insert(
        "actor",
        serde_json::to_value(&event.actor).expect("actor serializable"),
    );
    map.insert(
        "details",
        event.details.clone().unwrap_or(serde_json::Value::Null),
    );
    map.insert("doc_id", serde_json::json!(event.doc_id));
    map.insert(
        "event_class",
        event
            .event_class
            .as_ref()
            .map(|s| serde_json::json!(s))
            .unwrap_or(serde_json::Value::Null),
    );
    map.insert("event_id", serde_json::json!(event.event_id));
    map.insert("event_type", serde_json::json!(event.event_type));
    map.insert("prev_hash", serde_json::json!(event.prev_hash));
    map.insert(
        "timestamp",
        serde_json::json!(event.timestamp.to_rfc3339()),
    );
    map.insert(
        "verification_result",
        event
            .verification_result
            .clone()
            .unwrap_or(serde_json::Value::Null),
    );
    serde_json::to_string(&map).expect("BTreeMap serializable")
}

fn vector(id: &str, event: &mut Event) -> serde_json::Value {
    let payload = {
        event.event_hash = String::new();
        diagnostic_payload(event)
    };
    let hash = compute_event_hash(event);
    event.event_hash = hash.clone();
    json!({
        "id": id,
        "event": serde_json::to_value(event).expect("event serializable"),
        "payload": payload,
        "event_hash": hash,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("用法: cargo run --example golden_vectors -- <out.json>");
        std::process::exit(2);
    }
    let out_path = &args[1];

    let mut vectors: Vec<serde_json::Value> = Vec::new();

    // 一 创世全字段：details 含中文、嵌套对象、数组、整数；两可选字段在场
    let mut v1 = make_event(
        "11111111-1111-4111-8111-111111111111",
        "certification_completed",
        234_923_000,
        "scribe",
        "cli",
        Some(json!({
            "report_path": "scrutinator/reports/PRO-007.json",
            "report_hash": "a1b2c3d4e5f6",
            "pack_versions": ["des-001/0.1.0"],
            "content_hashes": {"doc/proposal/PRO-007.md": "3a2ee8211ea92276"},
            "finding_count": 0,
            "exit_code": 0,
            "tool_version": "0.1.0",
            "golden_baseline": "0.1.0"
        })),
        "PRO-007",
        GENESIS_PREV_HASH,
        Some("record_only"),
        Some(json!({"exit_code": 0, "total": 0})),
    );
    let v1_hash = {
        vectors.push(vector("genesis_full", &mut v1));
        vectors.last().unwrap()["event_hash"].as_str().unwrap().to_string()
    };

    // 二 空值与可选字段缺省：details 与 event_class 与 verification_result 全缺省
    let mut v2 = make_event(
        "22222222-2222-4222-8222-222222222222",
        "session_open",
        0,
        "scribe",
        "cli",
        None,
        "SESSION",
        &v1_hash,
        None,
        None,
    );
    vectors.push(vector("nulls_and_omitted", &mut v2));

    // 三 毫秒时间戳：纳秒为整千组即三位小数
    let mut v3 = make_event(
        "33333333-3333-4333-8333-333333333333",
        "certification_completed",
        123_000_000,
        "scribe",
        "cli",
        Some(json!({"note": "millis"})),
        "MILLIS",
        &v1_hash,
        Some("record_only"),
        None,
    );
    vectors.push(vector("timestamp_millis", &mut v3));

    // 四 纳秒时间戳：九位小数
    let mut v4 = make_event(
        "44444444-4444-4444-8444-444444444444",
        "certification_completed",
        123_456_789,
        "scribe",
        "cli",
        Some(json!({"note": "nanos"})),
        "NANOS",
        &v1_hash,
        None,
        Some(json!({"ok": true})),
    );
    vectors.push(vector("timestamp_nanos", &mut v4));

    // 五 零小数时间戳：无小数位
    let mut v5 = make_event(
        "55555555-5555-4555-8555-555555555555",
        "certification_completed",
        0,
        "scribe",
        "cli",
        Some(json!({"note": "zero"})),
        "ZERO",
        &v1_hash,
        Some("record_only"),
        None,
    );
    vectors.push(vector("timestamp_zero_frac", &mut v5));

    // 六 转义与非 ASCII：引号、反斜杠、换行、制表、控制字符、中文
    let mut v6 = make_event(
        "66666666-6666-4666-8666-666666666666",
        "certification_completed",
        500_000_000,
        "scribe-司衡",
        "cli",
        Some(json!({
            "quote": "含\"引号\"",
            "backslash": "反\\斜杠",
            "newline": "第\n二行",
            "tab": "制\t表",
            "control": "控\u{0001}字符",
            "chinese": "书简外切名随物走"
        })),
        "文档-司衡/书简",
        &v1_hash,
        Some("record_only"),
        None,
    );
    vectors.push(vector("escapes_and_unicode", &mut v6));

    // 七 链式：prev_hash 取向量六的计算哈希，演示链延伸
    let v6_hash = vectors.last().unwrap()["event_hash"].as_str().unwrap().to_string();
    let mut v7 = make_event(
        "77777777-7777-4777-8777-777777777777",
        "certification_completed",
        750_000_123,
        "scribe",
        "cli",
        Some(json!({"chained": true, "depth": 2})),
        "CHAIN",
        &v6_hash,
        Some("record_only"),
        Some(json!({"exit_code": 1, "total": 3})),
    );
    vectors.push(vector("chained", &mut v7));

    let doc = json!({
        "generator": "sih-engine/examples/golden_vectors.rs",
        "engine_version": env!("CARGO_PKG_VERSION"),
        "hash_authority": "sih-engine/src/event_stream/hash.rs compute_event_hash",
        "note": "工具线复算 event_hash 与 payload 须与向量逐字节一致，不一致即禁写",
        "vectors": vectors,
    });

    let pretty = serde_json::to_string_pretty(&doc).expect("doc serializable");
    fs::write(out_path, pretty + "\n").unwrap_or_else(|e| {
        eprintln!("写入失败 {}: {}", out_path, e);
        std::process::exit(2);
    });
    println!("golden 向量 {} 条已写入 {}", doc["vectors"].as_array().unwrap().len(), out_path);
}
