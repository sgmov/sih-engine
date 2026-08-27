//! 三轴语义即时间日界与事件记号匹配，承接 SPEC-008 轴语义落差节。

use chrono::{Datelike, DateTime, FixedOffset, NaiveDate, TimeZone, Utc};

use crate::event_stream::Event;

use super::RecallError;

/// 东八区固定偏移，项目运转时区，无夏令时。
fn cst() -> FixedOffset {
    FixedOffset::east_opt(8 * 3600).expect("固定偏移恒合法")
}

/// 日期日界即东八区当日零时至当日末纳秒，边界日含即含承 SPEC-007 时间轴条款。
pub fn day_bounds(date: &str) -> Result<(DateTime<Utc>, DateTime<Utc>), RecallError> {
    let day = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| RecallError::Blocked(format!("日期非法 {date}")))?;
    let tz = cst();
    let start = tz
        .with_ymd_and_hms(day.year(), day.month(), day.day(), 0, 0, 0)
        .single()
        .ok_or_else(|| RecallError::Internal(format!("日界起不可定 {date}")))?;
    let end = tz
        .with_ymd_and_hms(day.year(), day.month(), day.day(), 23, 59, 59)
        .single()
        .and_then(|t| t.checked_add_signed(chrono::Duration::nanoseconds(999_999_999)))
        .ok_or_else(|| RecallError::Internal(format!("日界止不可定 {date}")))?;
    Ok((start.with_timezone(&Utc), end.with_timezone(&Utc)))
}

/// 事件时间戳是否落入 since 与 until 窗口，None 侧不设界。
pub fn time_hit(
    ts: DateTime<Utc>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<bool, RecallError> {
    if let Some(s) = since {
        if ts < day_bounds(s)?.0 {
            return Ok(false);
        }
    }
    if let Some(u) = until {
        if ts > day_bounds(u)?.1 {
            return Ok(false);
        }
    }
    Ok(true)
}

/// 事件记号三路精确匹配即事件类型或 details.entry_id 或 details.session_id。
pub fn event_token_hit(event: &Event, token: &str) -> bool {
    if event.event_type == token {
        return true;
    }
    let Some(obj) = event.details.as_ref().and_then(|d| d.as_object()) else {
        return false;
    };
    obj.get("entry_id").and_then(|v| v.as_str()) == Some(token)
        || obj.get("session_id").and_then(|v| v.as_str()) == Some(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::{Actor, ActorType};
    use serde_json::json;

    fn mk_event(ts: DateTime<Utc>) -> Event {
        Event {
            event_id: "e-1".into(),
            event_type: "certification_completed".into(),
            timestamp: ts,
            actor: Actor {
                actor_id: "t".into(),
                actor_type: ActorType::System,
                invoked_via: "test".into(),
            },
            details: Some(json!({"entry_id": "pk-024", "session_id": "sess-x"})),
            doc_id: "d".into(),
            prev_hash: "0".into(),
            event_hash: "0".into(),
            event_class: None,
            verification_result: None,
        }
    }

    /// F-5 合成边界即首末瞬皆收、窗外一纳秒即排除。
    #[test]
    fn f5_day_bounds_inclusive() {
        let (start, end) = day_bounds("2026-08-27").unwrap();
        let tz = cst();
        let expect_start = tz
            .with_ymd_and_hms(2026, 8, 27, 0, 0, 0)
            .single()
            .unwrap()
            .with_timezone(&Utc);
        let expect_end = tz
            .with_ymd_and_hms(2026, 8, 27, 23, 59, 59)
            .single()
            .unwrap()
            .checked_add_signed(chrono::Duration::nanoseconds(999_999_999))
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(start, expect_start);
        assert_eq!(end, expect_end);
        assert!(time_hit(start, Some("2026-08-27"), Some("2026-08-27")).unwrap());
        assert!(time_hit(end, Some("2026-08-27"), Some("2026-08-27")).unwrap());
        let before = start - chrono::Duration::nanoseconds(1);
        let after = end + chrono::Duration::nanoseconds(1);
        assert!(!time_hit(before, Some("2026-08-27"), Some("2026-08-27")).unwrap());
        assert!(!time_hit(after, Some("2026-08-27"), Some("2026-08-27")).unwrap());
        assert!(day_bounds("2026-13-99").is_err());
    }

    /// F-9 记号三路即类型与 entry_id 与 session_id 各中一路。
    #[test]
    fn f9_event_token_hit_three_ways() {
        let e = mk_event(Utc::now());
        assert!(event_token_hit(&e, "certification_completed"));
        assert!(event_token_hit(&e, "pk-024"));
        assert!(event_token_hit(&e, "sess-x"));
        assert!(!event_token_hit(&e, "no-such-token"));
    }
}
