//! 心跳视图：在泊配对与到期三态与最近读数。
//!
//! 承 PARKING-v1#heartbeat 心跳报数与结算单必经栏归视图族。
//!
//! 在泊事件按 entry_id 配对 parking_entered 与 parking_exited。
//! 每项按 entered_at + ttl_days 对参照日出三态：
//!   0 = 在泊未到期，1 = 在泊到期，2 = 已出泊。
//! 最近三维读数即 convergence / adoption / mergeback 按 doc_id 倒序取最近一条，
//! 缺席字段如实标 unknown 不虚构趋势，承秤星纪律。
//! 视图零写：纯函数，不写任何文件。

use crate::event_stream::event::Event;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use std::collections::HashMap;

/// 在泊单条状态。
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ParkingState {
    pub entry_id: String,
    pub title: String,
    pub entered_at: String,
    pub ttl_days: i64,
    pub reference_date: String,
    /// 0 = 在泊未到期
    /// 1 = 在泊到期
    /// 2 = 已出泊
    pub state: i32,
    pub state_label: &'static str,
    pub exited_at: Option<String>,
    pub disposition: Option<String>,
}

/// 单维最近读数。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RecentReading {
    pub dimension: String,
    pub value: serde_json::Value,
    pub computed_at: String,
    pub window: String,
    pub subject: String,
    pub present: bool,
}

/// 心跳视图聚合。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HeartbeatView {
    pub reference_date: String,
    pub parking: Vec<ParkingState>,
    pub consumable_count: usize,
    pub recent_readings: Vec<RecentReading>,
}

/// 在泊三态机：按 entry_id 顺序重放停泊事件。
pub fn parking_states(events: &[Event], reference_date: NaiveDate) -> Vec<ParkingState> {
    let mut parked: HashMap<String, ParkingState> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for e in events {
        let Some(d) = e.details.as_ref().and_then(|v| v.as_object()) else { continue };
        let Some(entry_id) = d.get("entry_id").and_then(|v| v.as_str()) else { continue };
        match e.event_type.as_str() {
            "parking_entered" => {
                let title = d.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let ttl_days = d.get("ttl_days").and_then(|v| v.as_i64()).unwrap_or(0);
                let entered_at_date = e.timestamp.with_timezone(&Utc).date_naive();
                let state = if entered_at_date
                    .checked_add_signed(chrono::Duration::days(ttl_days))
                    .map(|d| d < reference_date)
                    .unwrap_or(false)
                {
                    1
                } else {
                    0
                };
                let state_label = match state {
                    0 => "在泊未到期",
                    1 => "在泊到期",
                    _ => "在泊未到期",
                };
                let p = ParkingState {
                    entry_id: entry_id.to_string(),
                    title,
                    entered_at: e.timestamp.to_rfc3339(),
                    ttl_days,
                    reference_date: reference_date.to_string(),
                    state,
                    state_label,
                    exited_at: None,
                    disposition: None,
                };
                if !parked.contains_key(entry_id) {
                    order.push(entry_id.to_string());
                }
                parked.insert(entry_id.to_string(), p);
            }
            "parking_exited" => {
                let disposition = d.get("disposition").and_then(|v| v.as_str()).map(String::from);
                if let Some(p) = parked.get_mut(entry_id) {
                    p.state = 2;
                    p.state_label = "已出泊";
                    p.exited_at = Some(e.timestamp.to_rfc3339());
                    p.disposition = disposition;
                }
            }
            _ => {}
        }
    }
    order
        .into_iter()
        .filter_map(|k| parked.remove(&k))
        .collect()
}

/// 可消费事件计数：event_class == "consumable" 的事件总数。
pub fn consumable_count(events: &[Event]) -> usize {
    events
        .iter()
        .filter(|e| e.event_class.as_deref() == Some("consumable"))
        .count()
}

/// 最近读数：每维度按 doc_id 字典序取最近一条（按 timestamp 倒序），
/// 缺席维度如实标 present=false。
pub fn recent_readings(events: &[Event]) -> Vec<RecentReading> {
    let dims = ["convergence", "adoption", "mergeback"];
    let mut out = Vec::new();
    for dim in dims {
        let candidates: Vec<&Event> = events
            .iter()
            .filter(|e| e.event_type == "reading_recorded")
            .filter(|e| {
                e.details
                    .as_ref()
                    .and_then(|d| d.get("dimension"))
                    .and_then(|v| v.as_str())
                    == Some(dim)
            })
            .collect();
        if let Some(latest) = candidates.iter().max_by_key(|e| e.timestamp) {
            let dim_v = latest
                .details
                .as_ref()
                .and_then(|d| d.get("dimension"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let subject = latest
                .details
                .as_ref()
                .and_then(|d| d.get("subject"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let computed_at = latest
                .details
                .as_ref()
                .and_then(|d| d.get("computed_at"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let window = latest
                .details
                .as_ref()
                .and_then(|d| d.get("window"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let value = latest
                .details
                .as_ref()
                .and_then(|d| d.get("value"))
                .cloned()
                .unwrap_or(serde_json::Value::String("insufficient".into()));
            out.push(RecentReading {
                dimension: dim_v,
                value,
                computed_at,
                window,
                subject,
                present: true,
            });
        } else {
            out.push(RecentReading {
                dimension: dim.to_string(),
                value: serde_json::Value::String("insufficient".into()),
                computed_at: "unknown".into(),
                window: "unknown".into(),
                subject: "unknown".into(),
                present: false,
            });
        }
    }
    out
}

/// 心跳视图聚合。
pub fn heartbeat_view(events: &[Event], reference_date: NaiveDate) -> HeartbeatView {
    HeartbeatView {
        reference_date: reference_date.to_string(),
        parking: parking_states(events, reference_date),
        consumable_count: consumable_count(events),
        recent_readings: recent_readings(events),
    }
}

/// 工具：把 RFC3339 时间戳转 NaiveDate
pub fn parse_date_from_timestamp(ts: &DateTime<Utc>) -> NaiveDate {
    ts.with_timezone(&Utc).date_naive()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::{Actor, ActorType, Event};
    use chrono::TimeZone;
    use serde_json::json;

    fn ev(event_id: &str, event_type: &str, ts_secs: i64, details: serde_json::Value, doc_id: &str, class: Option<&str>) -> Event {
        Event {
            event_id: event_id.into(),
            event_type: event_type.into(),
            timestamp: Utc.timestamp_opt(ts_secs, 0).unwrap(),
            actor: Actor {
                actor_id: "test".into(),
                actor_type: ActorType::System,
                invoked_via: "cli".into(),
            },
            details: Some(details),
            doc_id: doc_id.into(),
            prev_hash: "0".into(),
            event_hash: format!("hash-{event_id}"),
            event_class: class.map(String::from),
            verification_result: None,
        }
    }

    fn reading(dim: &str, value: f64, ts_secs: i64, doc: &str) -> Event {
        ev(
            doc,
            "reading_recorded",
            ts_secs,
            json!({
                "dimension": dim,
                "subject": "sih-engine",
                "value": value,
                "window": "2026-08-01/2026-08-30",
                "formula_version": "ga-1",
                "computed_at": "2026-08-30",
                "inputs_digest": "a".repeat(64)
            }),
            doc,
            Some("consumable"),
        )
    }

    #[test]
    fn r1_parking_pairing_active() {
        // entered 5 天前，ttl_days=7，参照 2026-01-08 仍在泊未到期
        let entered = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
        let events = vec![
            ev("e1", "parking_entered", entered, json!({"entry_id":"pk-001","title":"t","ttl_days":7}), "pk-001", Some("record_only")),
        ];
        let view = heartbeat_view(&events, NaiveDate::from_ymd_opt(2026, 1, 8).unwrap());
        assert_eq!(view.parking.len(), 1);
        assert_eq!(view.parking[0].state, 0);
        assert_eq!(view.parking[0].state_label, "在泊未到期");
    }

    #[test]
    fn r2_parking_expired_state() {
        let events = vec![
            ev("e1", "parking_entered", 100, json!({"entry_id":"pk-001","title":"t","ttl_days":3}), "pk-001", Some("record_only")),
        ];
        let view = heartbeat_view(&events, NaiveDate::from_ymd_opt(2026, 1, 8).unwrap());
        assert_eq!(view.parking[0].state, 1);
        assert_eq!(view.parking[0].state_label, "在泊到期");
    }

    #[test]
    fn r3_parking_exited_state() {
        let events = vec![
            ev("e1", "parking_entered", 100, json!({"entry_id":"pk-001","title":"t","ttl_days":30}), "pk-001", Some("record_only")),
            ev("e2", "parking_exited", 200, json!({"entry_id":"pk-001","disposition":"promoted","ruling":"ok"}), "pk-001", Some("record_only")),
        ];
        let view = heartbeat_view(&events, NaiveDate::from_ymd_opt(2026, 1, 8).unwrap());
        assert_eq!(view.parking[0].state, 2);
        assert_eq!(view.parking[0].state_label, "已出泊");
        assert_eq!(view.parking[0].disposition.as_deref(), Some("promoted"));
    }

    #[test]
    fn r4_recent_readings_three_dims_or_insufficient() {
        let events = vec![reading("convergence", 0.5, 300, "g-c")];
        let view = heartbeat_view(&events, NaiveDate::from_ymd_opt(2026, 1, 8).unwrap());
        assert_eq!(view.recent_readings.len(), 3);
        let conv = view.recent_readings.iter().find(|r| r.dimension == "convergence").unwrap();
        assert!(conv.present);
        let adopt = view.recent_readings.iter().find(|r| r.dimension == "adoption").unwrap();
        assert!(!adopt.present);
    }

    #[test]
    fn r5_consumable_count() {
        let events = vec![
            ev("e1", "x", 100, json!({}), "d1", Some("consumable")),
            ev("e2", "y", 200, json!({}), "d2", Some("record_only")),
            ev("e3", "z", 300, json!({}), "d3", Some("consumable")),
        ];
        let view = heartbeat_view(&events, NaiveDate::from_ymd_opt(2026, 1, 8).unwrap());
        assert_eq!(view.consumable_count, 2);
    }

    #[test]
    fn r6_byte_identical_repeated_call() {
        let events = vec![
            ev("e1", "parking_entered", 100, json!({"entry_id":"pk-001","title":"t","ttl_days":7}), "pk-001", Some("record_only")),
            reading("convergence", 0.5, 300, "g-c"),
        ];
        let a = serde_json::to_string(&heartbeat_view(&events, NaiveDate::from_ymd_opt(2026, 1, 8).unwrap())).unwrap();
        let b = serde_json::to_string(&heartbeat_view(&events, NaiveDate::from_ymd_opt(2026, 1, 8).unwrap())).unwrap();
        assert_eq!(a, b);
    }
}
