//! 意图入口，承接 SPEC-006#intent-entry。
//!
//! 双件消费即意图记录 JSON 加核验报告 JSON，核验报告须为 ask3 类零发现，
//! 有发现即拒不入流。负载承 SPEC-006 十二项，doc_id 取会话标识。

use crate::event_stream::certify::sha256_hex;
use crate::event_stream::event::{Actor, Event, EventInput};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::path::Path;

#[derive(Debug)]
pub enum IntentError {
    RecordNotJson,
    RecordMissingField(&'static str),
    ValidationNotJson,
    FindingsPresent(usize),
    ValidationNotOk(String),
    LineageMismatch,
    IntentRecordUsed,
}

/// 重意图拒闸：同 record 路径已有 intent_refined 在链即拒，--allow-reintent 放行。
///
/// 承 guardrail2-solo 闸二：scope 即链目录全量重放面（承 park 的
/// load_parking_scope 形），按事件 details.record_path 与当次 record_path 比对。
/// allow_reintent 显式开启即放行，供 facepark 丢事件重追加的合法通道。
pub fn check_intent_reused(scope: &[Event], record_path: &str, allow_reintent: bool) -> Result<(), IntentError> {
    if allow_reintent {
        return Ok(());
    }
    for event in scope {
        if event.event_type != "intent_refined" {
            continue;
        }
        let Some(details) = &event.details else { continue };
        let Some(path) = details.get("record_path").and_then(|v| v.as_str()) else {
            continue;
        };
        if path == record_path {
            return Err(IntentError::IntentRecordUsed);
        }
    }
    Ok(())
}

/// 双件消费即意图记录加核验报告，核验零发现放行产 intent_refined 事件输入。
pub fn intent_event(
    record_path: &Path,
    record_text: &str,
    validation_path: &Path,
    validation_text: &str,
    actor: Actor,
    timestamp: DateTime<Utc>,
) -> Result<EventInput, IntentError> {
    let record: Value =
        serde_json::from_str(record_text).map_err(|_| IntentError::RecordNotJson)?;
    let validation: Value = serde_json::from_str(validation_text)
        .map_err(|_| IntentError::ValidationNotJson)?;

    let status = validation.get("status").and_then(|s| s.as_str());
    if let Some(s) = status {
        if s != "ok" {
            return Err(IntentError::ValidationNotOk(s.to_string()));
        }
    }
    let v_session = validation.get("session_id").and_then(|s| s.as_str());
    let r_session = record.get("session_id").and_then(|s| s.as_str());
    if let (Some(vs), Some(rs)) = (v_session, r_session) {
        if vs != rs {
            return Err(IntentError::LineageMismatch);
        }
    }

    let findings = validation.get("findings").and_then(|f| f.as_array());
    if let Some(arr) = findings {
        if !arr.is_empty() {
            return Err(IntentError::FindingsPresent(arr.len()));
        }
    }

    let session_id = record
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or(IntentError::RecordMissingField("session_id"))?
        .to_string();
    let round = record
        .get("round")
        .and_then(|v| v.as_i64())
        .ok_or(IntentError::RecordMissingField("round"))?;
    let anchor_count = record
        .get("anchors")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let calls_in = record
        .get("calls_in")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let calls_out = record
        .get("calls_out")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let goal = record
        .pointer("/intent_contract/goal")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let pack_versions: Vec<String> = validation
        .get("packs")
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|p| {
                    let name = p.get("name")?.as_str()?;
                    let version = p.get("version")?.as_str()?;
                    Some(format!("{name}/{version}"))
                })
                .collect()
        })
        .unwrap_or_default();

    let tool_version = validation
        .get("tool")
        .and_then(|t| t.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let golden_baseline = validation
        .get("golden_baseline")
        .and_then(|g| g.as_str())
        .unwrap_or("")
        .to_string();

    let details = json!({
        "record_path": record_path.to_string_lossy(),
        "record_hash": sha256_hex(record_text),
        "validation_report_path": validation_path.to_string_lossy(),
        "validation_report_hash": sha256_hex(validation_text),
        "pack_versions": pack_versions,
        "session_id": session_id,
        "round": round,
        "anchor_count": anchor_count,
        "calls_in": calls_in,
        "calls_out": calls_out,
        "goal": goal,
        "tool_version": tool_version,
        "golden_baseline": golden_baseline,
    });

    Ok(EventInput {
        event_id: uuid::Uuid::new_v4().to_string(),
        event_type: "intent_refined".to_string(),
        timestamp,
        actor,
        details: Some(details),
        doc_id: session_id,
        prev_hash: None,
        event_class: Some("record_only".to_string()),
        verification_result: None,
        session_id: None,
        identity_hash: None,
    })
}
#[cfg(test)]
mod intent_status_tests {
    use super::*;

    fn actor() -> Actor {
        serde_json::from_str(
            r#"{"actor_id":"scribe","actor_type":"system","invoked_via":"cli"}"#,
        )
        .expect("actor")
    }

    #[test]
    fn rejected_status_refused() {
        let rec = r#"{"session_id":"s1","round":1}"#;
        let val = r#"{"status":"rejected","findings":[]}"#;
        let err = intent_event(
            Path::new("r.json"), rec, Path::new("v.json"), val,
            actor(), Utc::now(),
        );
        match err {
            Err(IntentError::ValidationNotOk(s)) => assert_eq!(s, "rejected"),
            other => panic!("expected ValidationNotOk, got {other:?}"),
        }
    }

    #[test]
    fn session_mismatch_refused() {
        let rec = r#"{"session_id":"s1","round":1}"#;
        let val = r#"{"status":"ok","session_id":"s2","findings":[]}"#;
        let err = intent_event(
            Path::new("r.json"), rec, Path::new("v.json"), val,
            actor(), Utc::now(),
        );
        assert!(matches!(err, Err(IntentError::LineageMismatch)));
    }

    #[test]
    fn ok_status_passes_gate() {
        let rec = r#"{"session_id":"s1","round":1,"anchors":[],"calls_in":0,"calls_out":0,"intent_contract":{"goal":"g"}}"#;
        let val = r#"{"status":"ok","findings":[]}"#;
        let out = intent_event(
            Path::new("r.json"), rec, Path::new("v.json"), val,
            actor(), Utc::now(),
        );
        assert!(out.is_ok(), "unexpected error: {out:?}");
    }

    fn reused_event(record_path: &str) -> Event {
        Event {
            event_id: "id".to_string(),
            event_type: "intent_refined".to_string(),
            timestamp: Utc::now(),
            actor: actor(),
            details: Some(json!({"record_path": record_path})),
            doc_id: "s1".to_string(),
            prev_hash: String::new(),
            event_hash: String::new(),
            event_class: None,
            verification_result: None,
            session_id: None,
            identity_hash: None,
        }
    }

    // 闸二 R1 同 record 路径已有 intent_refined 即拒。
    #[test]
    fn reintent_repeated_record_rejected() {
        let scope = vec![reused_event("/tmp/r.json")];
        assert!(matches!(
            check_intent_reused(&scope, "/tmp/r.json", false),
            Err(IntentError::IntentRecordUsed)
        ));
    }

    // 闸二 R2 链上无同 record 即放行。
    #[test]
    fn reintent_fresh_record_allows() {
        let scope = vec![reused_event("/tmp/other.json")];
        assert!(check_intent_reused(&scope, "/tmp/r.json", false).is_ok());
    }

    // 闸二 R3 空链即放行。
    #[test]
    fn reintent_empty_scope_allows() {
        assert!(check_intent_reused(&[], "/tmp/r.json", false).is_ok());
    }

    // 闸二 R4 --allow-reintent 显式开启即放行同 record 重追加合法通道。
    #[test]
    fn reintent_allow_reintent_allows() {
        let scope = vec![reused_event("/tmp/r.json")];
        assert!(check_intent_reused(&scope, "/tmp/r.json", true).is_ok());
    }
}
