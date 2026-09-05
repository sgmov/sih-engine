//! 异常视图：仅可消费类事件进视图，仅记录类事件不进，承 SPEC-004 第 80-84 行即 FM-05 事件分类。
//!
//! 视图零写：纯函数，从给定事件集合过滤 event_class == "consumable" 的事件，
//! 呈报含事件哈希与 doc_id 指针与事件类型与时间戳。
//! 空即零、非空即一、异常即二。

use crate::event_stream::event::Event;
use serde::Serialize;

/// 单条异常事件指针呈报。
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AlarmEntry {
    /// 事件唯一 ID
    pub event_id: String,
    /// 事件类型
    pub event_type: String,
    /// 关联文档 ID
    pub doc_id: String,
    /// ISO 8601 时间戳
    pub timestamp: String,
    /// 事件哈希
    pub event_hash: String,
}

/// 异常视图聚合结果。
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AlarmsView {
    /// 异常事件集合，按事件时间戳升序
    pub alarms: Vec<AlarmEntry>,
    /// 异常事件总数
    pub total: usize,
    /// 退出码建议：0 = 空，1 = 非空
    pub exit_code_suggested: i32,
}

/// 过滤 event_class == "consumable" 的事件。event_class 缺失或非 "consumable" 视为非可消费。
pub fn consumable_events(events: &[Event]) -> Vec<AlarmEntry> {
    let mut out: Vec<AlarmEntry> = events
        .iter()
        .filter(|e| e.event_class.as_deref() == Some("consumable"))
        .map(|e| AlarmEntry {
            event_id: e.event_id.clone(),
            event_type: e.event_type.clone(),
            doc_id: e.doc_id.clone(),
            timestamp: e.timestamp.to_rfc3339(),
            event_hash: e.event_hash.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    out
}

/// 构造异常视图聚合。
pub fn alarms_view(events: &[Event]) -> AlarmsView {
    let alarms = consumable_events(events);
    let total = alarms.len();
    let exit_code_suggested = if total == 0 { 0 } else { 1 };
    AlarmsView { alarms, total, exit_code_suggested }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::{Actor, ActorType, Event};
    use chrono::{TimeZone, Utc};

    fn ev(event_id: &str, event_type: &str, event_class: Option<&str>, hour: i32) -> Event {
        Event {
            event_id: event_id.into(),
            event_type: event_type.into(),
            timestamp: Utc.timestamp_opt(1_000_000 + hour as i64 * 3600, 0).unwrap(),
            actor: Actor {
                actor_id: "test".into(),
                actor_type: ActorType::System,
                invoked_via: "cli".into(),
            },
            details: None,
            doc_id: "TEST".into(),
            prev_hash: "0".into(),
            event_hash: format!("hash-{event_id}"),
            event_class: event_class.map(String::from),
            verification_result: None,
            session_id: None,
            identity_hash: None,
        }
    }

    #[test]
    fn r1_only_consumable_classes_pass_through() {
        let events = vec![
            ev("e1", "crosscheck_completed", Some("consumable"), 10),
            ev("e2", "intent_refined", Some("record_only"), 11),
            ev("e3", "task_completion", Some("consumable"), 12),
            ev("e4", "session_open", None, 13),
        ];
        let view = alarms_view(&events);
        assert_eq!(view.total, 2);
        let ids: Vec<&str> = view.alarms.iter().map(|a| a.event_id.as_str()).collect();
        assert_eq!(ids, vec!["e1", "e3"]);
    }

    #[test]
    fn r2_record_only_never_enters_view() {
        let events = vec![
            ev("e1", "intent_refined", Some("record_only"), 10),
            ev("e2", "parking_entered", Some("record_only"), 11),
            ev("e3", "certification_completed", Some("record_only"), 12),
        ];
        let view = alarms_view(&events);
        assert_eq!(view.total, 0);
        assert_eq!(view.exit_code_suggested, 0);
        assert!(view.alarms.is_empty());
    }

    #[test]
    fn r3_empty_input_yields_empty_view_with_zero_exit() {
        let view = alarms_view(&[]);
        assert_eq!(view.total, 0);
        assert_eq!(view.exit_code_suggested, 0);
    }

    #[test]
    fn r4_nonempty_consumable_yields_exit_code_one() {
        let events = vec![ev("e1", "x", Some("consumable"), 10)];
        let view = alarms_view(&events);
        assert_eq!(view.exit_code_suggested, 1);
    }

    #[test]
    fn r5_temporal_ordering_is_stable_ascending() {
        let events = vec![
            ev("e2", "x", Some("consumable"), 20),
            ev("e1", "x", Some("consumable"), 10),
            ev("e3", "x", Some("consumable"), 15),
        ];
        let view = alarms_view(&events);
        let ids: Vec<&str> = view.alarms.iter().map(|a| a.event_id.as_str()).collect();
        assert_eq!(ids, vec!["e1", "e3", "e2"]);
    }

    #[test]
    fn r6_byte_identical_under_repeated_call() {
        let events = vec![
            ev("e1", "x", Some("consumable"), 10),
            ev("e2", "y", Some("record_only"), 11),
        ];
        let a = serde_json::to_string(&alarms_view(&events)).unwrap();
        let b = serde_json::to_string(&alarms_view(&events)).unwrap();
        assert_eq!(a, b);
    }
}
