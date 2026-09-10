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

/// 意图记录消费产 intent_refined 事件输入（DES-016 意图形分派）：ask3 形
/// 带验证件走核验零发现放行；plain 形零验证件零哲学引文放行。
pub fn intent_event(
    record_path: &Path,
    record_text: &str,
    validation: Option<(&Path, &str)>,
    actor: Actor,
    timestamp: DateTime<Utc>,
) -> Result<EventInput, IntentError> {
    let record: Value =
        serde_json::from_str(record_text).map_err(|_| IntentError::RecordNotJson)?;
    let anchor_count = record
        .get("anchors")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    // DES-016 意图形分派：载 anchors 者为 ask3 形（验证件必填），缺省者为
    // plain 形（验证件豁免，零哲学引文）；闸验结构不验哲学，零语义裁决。
    let (validation_value, validation_path_str, validation_hash, intent_form): (
        Option<serde_json::Value>,
        String,
        String,
        &'static str,
    ) = match validation {
        None => {
            if anchor_count > 0 {
                return Err(IntentError::RecordMissingField("validation"));
            }
            (None, String::new(), String::new(), "plain")
        }
        Some((vp, vtext)) => {
            let v: Value =
                serde_json::from_str(vtext).map_err(|_| IntentError::ValidationNotJson)?;
            let status = v.get("status").and_then(|s| s.as_str());
            if let Some(s) = status {
                if s != "ok" {
                    return Err(IntentError::ValidationNotOk(s.to_string()));
                }
            }
            let v_session = v.get("session_id").and_then(|s| s.as_str());
            let r_session = record.get("session_id").and_then(|s| s.as_str());
            if let (Some(vs), Some(rs)) = (v_session, r_session) {
                if vs != rs {
                    return Err(IntentError::LineageMismatch);
                }
            }
            let findings = v.get("findings").and_then(|f| f.as_array());
            if let Some(arr) = findings {
                if !arr.is_empty() {
                    return Err(IntentError::FindingsPresent(arr.len()));
                }
            }
            (Some(v), vp.to_string_lossy().to_string(), sha256_hex(vtext), "ask3")
        }
    };

    let session_id = record
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or(IntentError::RecordMissingField("session_id"))?
        .to_string();
    let round = record
        .get("round")
        .and_then(|v| v.as_i64())
        .ok_or(IntentError::RecordMissingField("round"))?;
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

    let pack_versions: Vec<String> = validation_value
        .as_ref()
        .and_then(|v| v.get("packs"))
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

    let tool_version = validation_value
        .as_ref()
        .and_then(|v| v.get("tool"))
        .and_then(|t| t.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let golden_baseline = validation_value
        .as_ref()
        .and_then(|v| v.get("golden_baseline"))
        .and_then(|g| g.as_str())
        .unwrap_or("")
        .to_string();

    let details = json!({
        "record_path": record_path.to_string_lossy(),
        "record_hash": sha256_hex(record_text),
        "validation_report_path": validation_path_str,
        "validation_report_hash": validation_hash,
        "intent_form": intent_form,
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
            Path::new("r.json"), rec, Some((Path::new("v.json"), val)),
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
            Path::new("r.json"), rec, Some((Path::new("v.json"), val)),
            actor(), Utc::now(),
        );
        assert!(matches!(err, Err(IntentError::LineageMismatch)));
    }

    #[test]
    fn ok_status_passes_gate() {
        let rec = r#"{"session_id":"s1","round":1,"anchors":[],"calls_in":0,"calls_out":0,"intent_contract":{"goal":"g"}}"#;
        let val = r#"{"status":"ok","findings":[]}"#;
        let out = intent_event(
            Path::new("r.json"), rec, Some((Path::new("v.json"), val)),
            actor(), Utc::now(),
        );
        assert!(out.is_ok(), "unexpected error: {out:?}");
    }

    #[test]
    fn plain_form_without_validation_passes() {
        let rec = r#"{"session_id":"s1","round":1,"calls_in":0,"calls_out":0,"intent_contract":{"goal":"g"},"domain_contract":{}}"#;
        let out = intent_event(Path::new("r.json"), rec, None, actor(), Utc::now());
        let input = out.expect("plain 形零验证件放行");
        let d = input.details.expect("details");
        assert_eq!(d["anchor_count"], 0);
        assert_eq!(d["intent_form"], "plain");
        assert_eq!(d["validation_report_path"], "");
    }

    #[test]
    fn ask3_form_without_validation_refused() {
        let rec = r#"{"session_id":"s1","round":1,"anchors":[{"a":1}],"intent_contract":{"goal":"g"}}"#;
        let out = intent_event(Path::new("r.json"), rec, None, actor(), Utc::now());
        assert!(matches!(out, Err(IntentError::RecordMissingField("validation"))));
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
