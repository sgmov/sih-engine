//! 事件检索入口，承接 DES-007#query-interface 与 SPEC-004#query-entry
//!
//! 接收检索条件，返回匹配的事件集合。只读操作，不修改事件流。

use crate::event_stream::event::Event;
use chrono::{DateTime, Utc};

/// 检索过滤条件，支持按多个维度组合过滤。
#[derive(Debug, Clone, Default)]
pub struct EventFilter {
    /// 按事件类型过滤（精确匹配）
    pub event_type: Option<String>,
    /// 按操作者 ID 过滤（精确匹配）
    pub actor_id: Option<String>,
    /// 起始时间戳（含）
    pub timestamp_start: Option<DateTime<Utc>>,
    /// 结束时间戳（含）
    pub timestamp_end: Option<DateTime<Utc>>,
    /// 按关联文档 ID 过滤（精确匹配）
    pub doc_id: Option<String>,
}

/// 聚合选项。
#[derive(Debug, Clone, Default)]
pub struct EventAggregate {
    /// 按事件类型计数
    pub count_by_type: bool,
    /// 按操作者 ID 计数
    pub count_by_actor: bool,
    /// 时间窗口分桶计数（小时粒度）
    pub bucket_by_hour: bool,
}

/// 聚合结果
#[derive(Debug, Clone)]
pub struct AggregationResult {
    /// 按事件类型计数：类型 → 数量
    pub by_event_type: Vec<(String, usize)>,
    /// 按操作者 ID 计数：actor_id → 数量
    pub by_actor_id: Vec<(String, usize)>,
    /// 时间窗口分桶：小时 → 数量
    pub by_hour: Vec<(String, usize)>,
}

/// 检索结果列表。
#[derive(Debug, Clone)]
pub struct EventList {
    /// 匹配的事件集合
    pub events: Vec<Event>,
    /// 匹配总数
    pub total: usize,
    /// 聚合结果（若有）
    pub aggregation: Option<AggregationResult>,
}

/// 事件检索入口。
///
/// 接收 filter 检索条件与 aggregate 聚合选项，返回匹配事件集合与聚合结果。
///
/// filter 与 aggregate 均为可选；两者均不提供时返回全部事件。
pub fn query(
    events: &[Event],
    filter: Option<EventFilter>,
    aggregate: Option<EventAggregate>,
) -> EventList {
    // 应用过滤
    let filtered: Vec<_> = match filter {
        Some(f) => events
            .iter()
            .filter(|e| {
                f.event_type
                    .as_ref()
                    .map_or(true, |t| &e.event_type == t)
                    && f.actor_id
                        .as_ref()
                        .map_or(true, |a| &e.actor.actor_id == a)
                    && f.timestamp_start
                        .map_or(true, |start| e.timestamp >= start)
                    && f.timestamp_end
                        .map_or(true, |end| e.timestamp <= end)
                    && f.doc_id.as_ref().map_or(true, |d| &e.doc_id == d)
            })
            .cloned()
            .collect(),
        None => events.iter().cloned().collect(),
    };

    let total = filtered.len();

    // 应用聚合
    let aggregation = aggregate.map(|agg| {
        let mut by_type: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        let mut by_actor: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        let mut by_hour: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();

        for e in &filtered {
            if agg.count_by_type {
                *by_type.entry(e.event_type.clone()).or_insert(0) += 1;
            }
            if agg.count_by_actor {
                *by_actor.entry(e.actor.actor_id.clone()).or_insert(0) += 1;
            }
            if agg.bucket_by_hour {
                let hour = e.timestamp.format("%Y-%m-%d %H:00").to_string();
                *by_hour.entry(hour).or_insert(0) += 1;
            }
        }

        AggregationResult {
            by_event_type: by_type.into_iter().collect(),
            by_actor_id: by_actor.into_iter().collect(),
            by_hour: by_hour.into_iter().collect(),
        }
    });

    EventList {
        events: filtered,
        total,
        aggregation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::{Actor, ActorType, Event};
    use chrono::TimeZone;

    fn make_event(event_id: &str, event_type: &str, actor_id: &str, hour: i32) -> Event {
        Event {
            event_id: event_id.into(),
            event_type: event_type.into(),
            timestamp: Utc.timestamp_opt(1000000 + hour as i64 * 3600, 0).unwrap(),
            actor: Actor {
                actor_id: actor_id.into(),
                actor_type: ActorType::Agent,
                invoked_via: "test".into(),
            },
            details: Some(serde_json::json!({"change_summary": "test"})),
            doc_id: "TEST-DOC".into(),
            prev_hash: "0".into(),
            event_hash: "0".into(),
            event_class: None,
            verification_result: None,
        }
    }

    fn store() -> Vec<Event> {
        vec![
            make_event("e1", "task_completion", "agent-1", 10),
            make_event("e2", "session_open", "agent-1", 11),
            make_event("e3", "task_completion", "agent-2", 12),
            make_event("e4", "crosscheck_completed", "agent-1", 13),
        ]
    }

    #[test]
    fn test_query_all() {
        let events = store();
        let result = query(&events, None, None);
        assert_eq!(result.total, 4);
    }

    #[test]
    fn test_filter_by_event_type() {
        let events = store();
        let filter = EventFilter {
            event_type: Some("task_completion".into()),
            ..Default::default()
        };
        let result = query(&events, Some(filter), None);
        assert_eq!(result.total, 2);
        assert!(result.events.iter().all(|e| e.event_type == "task_completion"));
    }

    #[test]
    fn test_filter_by_actor_id() {
        let events = store();
        let filter = EventFilter {
            actor_id: Some("agent-1".into()),
            ..Default::default()
        };
        let result = query(&events, Some(filter), None);
        assert_eq!(result.total, 3);
        assert!(result.events.iter().all(|e| e.actor.actor_id == "agent-1"));
    }

    #[test]
    fn test_filter_by_time_range() {
        let events = store();
        // store() creates events with hours 10, 11, 12, 13.
        // Filter for hours 10-11 inclusive: should match e1 (hour 10) and e2 (hour 11).
        let start_hour = 10_i32;
        let end_hour = 11_i32;
        let base_ts = 1_000_000_i64;
        let filter = EventFilter {
            timestamp_start: Some(Utc.timestamp_opt(base_ts + start_hour as i64 * 3600, 0).unwrap()),
            timestamp_end: Some(Utc.timestamp_opt(base_ts + end_hour as i64 * 3600, 0).unwrap()),
            ..Default::default()
        };
        let result = query(&events, Some(filter), None);
        // e1 (hour=10) and e2 (hour=11) match; e3 (hour=12) and e4 (hour=13) are excluded
        assert_eq!(result.total, 2);
        assert!(result.events.iter().all(|e| {
            e.event_id == "e1" || e.event_id == "e2"
        }));
    }

    #[test]
    fn test_aggregate_by_type() {
        let events = store();
        let agg = EventAggregate {
            count_by_type: true,
            ..Default::default()
        };
        let result = query(&events, None, Some(agg));
        assert!(result.aggregation.is_some());
        let ag = result.aggregation.unwrap();
        let type_counts: Vec<_> = ag.by_event_type;
        assert!(type_counts.iter().any(|(t, c)| t == "task_completion" && *c == 2));
        assert!(type_counts.iter().any(|(t, c)| t == "session_open" && *c == 1));
    }

    #[test]
    fn test_aggregate_by_actor() {
        let events = store();
        let agg = EventAggregate {
            count_by_actor: true,
            ..Default::default()
        };
        let result = query(&events, None, Some(agg));
        let ag = result.aggregation.unwrap();
        assert!(ag.by_actor_id.iter().any(|(a, c)| a == "agent-1" && *c == 3));
        assert!(ag.by_actor_id.iter().any(|(a, c)| a == "agent-2" && *c == 1));
    }

    #[test]
    fn test_aggregate_by_hour() {
        let events = store();
        let agg = EventAggregate {
            bucket_by_hour: true,
            ..Default::default()
        };
        let result = query(&events, None, Some(agg));
        let ag = result.aggregation.unwrap();
        assert_eq!(ag.by_hour.len(), 4);
    }

    #[test]
    fn test_combined_filter_and_aggregate() {
        let events = store();
        let filter = EventFilter {
            event_type: Some("task_completion".into()),
            ..Default::default()
        };
        let agg = EventAggregate {
            count_by_actor: true,
            ..Default::default()
        };
        let result = query(&events, Some(filter), Some(agg));
        assert_eq!(result.total, 2);
        let ag = result.aggregation.unwrap();
        assert!(ag.by_actor_id.iter().any(|(a, c)| a == "agent-1" && *c == 1));
        assert!(ag.by_actor_id.iter().any(|(a, c)| a == "agent-2" && *c == 1));
    }
}
