//! 结算视图：按日计数与认证意图计数与在泊复检名单。
//!
//! 承 PARKING-v1#heartbeat 心跳节拍为主线每段结算时在泊事项复检是结算单必经栏，
//! 结算前置即泊界清算完成，在泊归零方开结算。
//!
//! 视图零写：纯函数。
//! 缺席字段如实标 unknown，不虚构趋势与数值承秤星纪律。

use crate::event_stream::event::Event;
use crate::view::heartbeat::ParkingState;
use chrono::NaiveDate;
use serde::Serialize;
use std::collections::BTreeMap;

/// 结算单聚合。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SettleView {
    pub date: String,
    /// 事件类型 → 当日计数
    pub event_type_counts: Vec<(String, usize)>,
    pub certification_count: usize,
    pub intent_count: usize,
    pub consumable_count: usize,
    pub total_events: usize,
    /// 在泊复检名单（与 heartbeat 配对同源）
    pub parking_review: Vec<ParkingState>,
}

/// 按日过滤事件：timestamp 落在 [date 00:00 UTC, next_date 00:00 UTC) 区间内。
pub fn events_on_date(events: &[Event], date: NaiveDate) -> Vec<Event> {
    let start = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
    let next = date
        .succ_opt()
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|d| d.and_utc());
    events
        .iter()
        .filter(|e| e.timestamp >= start && next.map(|n| e.timestamp < n).unwrap_or(true))
        .cloned()
        .collect()
}

/// 事件类型计数（按字典序排序输出）。
pub fn event_type_counts(day_events: &[Event]) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for e in day_events {
        *counts.entry(e.event_type.clone()).or_insert(0) += 1;
    }
    counts.into_iter().collect()
}

/// 认证计数：event_type == "certification_completed"。
pub fn certification_count(day_events: &[Event]) -> usize {
    day_events
        .iter()
        .filter(|e| e.event_type == "certification_completed")
        .count()
}

/// 意图计数：event_type == "intent_refined"。
pub fn intent_count(day_events: &[Event]) -> usize {
    day_events
        .iter()
        .filter(|e| e.event_type == "intent_refined")
        .count()
}

/// 当日可消费事件计数。
pub fn consumable_count(day_events: &[Event]) -> usize {
    day_events
        .iter()
        .filter(|e| e.event_class.as_deref() == Some("consumable"))
        .count()
}

/// 结算视图聚合。
pub fn settle_view(
    events: &[Event],
    date: NaiveDate,
    parking: Vec<ParkingState>,
) -> SettleView {
    let day = events_on_date(events, date);
    SettleView {
        date: date.to_string(),
        total_events: day.len(),
        event_type_counts: event_type_counts(&day),
        certification_count: certification_count(&day),
        intent_count: intent_count(&day),
        consumable_count: consumable_count(&day),
        parking_review: parking,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::{Actor, ActorType, Event};
    use crate::view::heartbeat::heartbeat_view;
    use chrono::{NaiveDate, TimeZone, Utc};
    use serde_json::json;

    fn ev(event_id: &str, event_type: &str, ts_secs: i64, class: Option<&str>) -> Event {
        Event {
            event_id: event_id.into(),
            event_type: event_type.into(),
            timestamp: Utc.timestamp_opt(ts_secs, 0).unwrap(),
            actor: Actor {
                actor_id: "test".into(),
                actor_type: ActorType::System,
                invoked_via: "cli".into(),
            },
            details: None,
            doc_id: "D".into(),
            prev_hash: "0".into(),
            event_hash: format!("h-{event_id}"),
            event_class: class.map(String::from),
            verification_result: None,
            session_id: None,
            identity_hash: None,
        }
    }

    #[test]
    fn r1_event_type_counts_match_input() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 8).unwrap();
        // 2026-01-08 00:00 UTC = 1767830400
        let day_start = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
        let next = date.succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
        let events = vec![
            ev("e1", "task_completion", day_start + 100, Some("consumable")),
            ev("e2", "task_completion", day_start + 200, Some("consumable")),
            ev("e3", "certification_completed", day_start + 300, Some("record_only")),
            ev("e4", "intent_refined", day_start + 400, Some("record_only")),
            ev("e5", "session_open", day_start + 500, None),
            ev("e6", "task_completion", next + 100, Some("consumable")), // next day excluded
        ];
        let view = settle_view(&events, date, vec![]);
        assert_eq!(view.total_events, 5);
        let counts: std::collections::HashMap<String, usize> = view.event_type_counts.into_iter().collect();
        assert_eq!(counts.get("task_completion"), Some(&2));
        assert_eq!(counts.get("certification_completed"), Some(&1));
        assert_eq!(counts.get("intent_refined"), Some(&1));
        assert_eq!(counts.get("session_open"), Some(&1));
    }

    #[test]
    fn r2_certification_and_intent_counts() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 8).unwrap();
        let day_start = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
        let events = vec![
            ev("e1", "certification_completed", day_start + 1, Some("record_only")),
            ev("e2", "certification_completed", day_start + 2, Some("record_only")),
            ev("e3", "intent_refined", day_start + 3, Some("record_only")),
        ];
        let view = settle_view(&events, date, vec![]);
        assert_eq!(view.certification_count, 2);
        assert_eq!(view.intent_count, 1);
    }

    #[test]
    fn r3_parking_review_carries_heartbeat_state() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 8).unwrap();
        let day_start = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
        // 用含 details.entry_id 与 ttl_days 的真实停泊事件
        let mut e1 = ev("e1", "parking_entered", day_start + 1, Some("record_only"));
        e1.details = Some(json!({"entry_id":"pk-100","title":"settle-test","ttl_days":7}));
        let events = vec![e1];
        let hb = heartbeat_view(&events, date);
        let view = settle_view(&events, date, hb.parking.clone());
        assert_eq!(view.parking_review.len(), 1);
    }

    #[test]
    fn r4_byte_identical_repeated_call() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 8).unwrap();
        let events = vec![ev("e1", "task_completion", date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() + 1, Some("consumable"))];
        let a = serde_json::to_string(&settle_view(&events, date, vec![])).unwrap();
        let b = serde_json::to_string(&settle_view(&events, date, vec![])).unwrap();
        assert_eq!(a, b);
    }
}
