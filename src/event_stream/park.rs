//! 停泊入口，承接 SPEC-006#park-entry。
//!
//! 双动作 enter 与 exit，配对不变量两道机械门即重入拒与无主出拒，
//! 在泊判定按既有事件链序重放停泊事件，拒绝零留痕即不产事件。

use crate::event_stream::event::{Actor, Event, EventInput};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};

#[derive(Debug)]
pub enum ParkError {
    RecordNotJson,
    ActionMissing,
    EnterMissingField(&'static str),
    ExitMissingField(&'static str),
    InvalidTtl,
    InvalidDisposition(String),
    ReEnterRejected(String),
    OrphanExitRejected(String),
}

fn entry_parked(existing: &[Event], entry_id: &str) -> bool {
    let mut parked = false;
    for e in existing {
        let Some(d) = e.details.as_ref().and_then(|d| d.as_object()) else {
            continue;
        };
        if d.get("entry_id").and_then(|v| v.as_str()) != Some(entry_id) {
            continue;
        }
        match e.event_type.as_str() {
            "parking_entered" => parked = true,
            "parking_exited" => parked = false,
            _ => {}
        }
    }
    parked
}

/// 双动作 enter 与 exit，配对不变量两道门按既有事件链序重放判定。
pub fn park_event(
    record_text: &str,
    existing: &[Event],
    actor: Actor,
    timestamp: DateTime<Utc>,
) -> Result<EventInput, ParkError> {
    let record: Value = serde_json::from_str(record_text).map_err(|_| ParkError::RecordNotJson)?;
    let obj = record.as_object().ok_or(ParkError::RecordNotJson)?;
    let action = obj
        .get("action")
        .and_then(|a| a.as_str())
        .ok_or(ParkError::ActionMissing)?;

    match action {
        "enter" => {
            let entry_id = obj
                .get("entry_id")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::EnterMissingField("entry_id"))?
                .to_string();
            if entry_parked(existing, &entry_id) {
                return Err(ParkError::ReEnterRejected(entry_id));
            }
            let title = obj
                .get("title")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::EnterMissingField("title"))?;
            let exit_condition = obj
                .get("exit_condition")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::EnterMissingField("exit_condition"))?;
            let ttl_days = obj
                .get("ttl_days")
                .and_then(|v| v.as_i64())
                .ok_or(ParkError::EnterMissingField("ttl_days"))?;
            if ttl_days <= 0 {
                return Err(ParkError::InvalidTtl);
            }
            let mut details = json!({
                "entry_id": entry_id,
                "title": title,
                "exit_condition": exit_condition,
                "ttl_days": ttl_days,
            });
            if let Some(ctx) = obj.get("context") {
                details["context"] = ctx.clone();
            }
            let doc_id = entry_id;
            Ok(EventInput {
                event_id: uuid::Uuid::new_v4().to_string(),
                event_type: "parking_entered".to_string(),
                timestamp,
                actor,
                details: Some(details),
                doc_id,
                prev_hash: None,
                event_class: Some("record_only".to_string()),
                verification_result: None,
            })
        }
        "exit" => {
            let entry_id = obj
                .get("entry_id")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::ExitMissingField("entry_id"))?
                .to_string();
            if !entry_parked(existing, &entry_id) {
                return Err(ParkError::OrphanExitRejected(entry_id));
            }
            let disposition = obj
                .get("disposition")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::ExitMissingField("disposition"))?;
            if disposition != "promoted" && disposition != "discarded" {
                return Err(ParkError::InvalidDisposition(disposition.to_string()));
            }
            let ruling = obj
                .get("ruling")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::ExitMissingField("ruling"))?;
            let details = json!({
                "entry_id": entry_id,
                "disposition": disposition,
                "ruling": ruling,
            });
            let doc_id = entry_id;
            Ok(EventInput {
                event_id: uuid::Uuid::new_v4().to_string(),
                event_type: "parking_exited".to_string(),
                timestamp,
                actor,
                details: Some(details),
                doc_id,
                prev_hash: None,
                event_class: Some("record_only".to_string()),
                verification_result: None,
            })
        }
        _ => Err(ParkError::ActionMissing),
    }
}
