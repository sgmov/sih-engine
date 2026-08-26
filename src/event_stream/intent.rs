//! 意图入口，承接 SPEC-006#intent-entry。
//!
//! 双件消费即意图记录 JSON 加核验报告 JSON，核验报告须为 ask3 类零发现，
//! 有发现即拒不入流。负载承 SPEC-006 十二项，doc_id 取会话标识。

use crate::event_stream::certify::sha256_hex;
use crate::event_stream::event::{Actor, EventInput};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::path::Path;

#[derive(Debug)]
pub enum IntentError {
    RecordNotJson,
    RecordMissingField(&'static str),
    ValidationNotJson,
    FindingsPresent(usize),
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
    })
}
