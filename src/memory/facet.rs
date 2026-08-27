//! 切面产出即摘录截断与事件出处与排序，承接 SPEC-008 确定性与排序节。

use crate::event_stream::Event;

use super::FacetRow;

/// 摘录上限二百五十六字符，超长截断附三句点。
pub fn excerpt_of_text(text: &str) -> String {
    let mut chars = text.chars();
    let head: String = chars.by_ref().take(256).collect();
    if chars.next().is_none() {
        head
    } else {
        format!("{head}...")
    }
}

/// 事件摘录即紧凑序列化逐字再承截断规则。
pub fn excerpt_of_event(event: &Event) -> String {
    let text = serde_json::to_string(event).unwrap_or_default();
    excerpt_of_text(&text)
}

/// 事件出处即 event_id 加事件哈希前缀八位。
pub fn event_reference(event: &Event) -> String {
    let head_len = event.event_hash.len().min(8);
    format!("{}/{}", event.event_id, &event.event_hash[..head_len])
}

fn sort_key(row: &FacetRow) -> (u8, u8, String, u32, String, u8, String) {
    let is_event = if row.carrier == "event" { 1 } else { 0 };
    let path_part = if is_event == 1 {
        row.reference.clone()
    } else {
        let cut = row
            .reference
            .find('@')
            .unwrap_or(row.reference.len());
        row.reference[..cut].to_string()
    };
    let line = if row.carrier == "md" {
        row.reference
            .rsplit_once(':')
            .and_then(|(_, range)| range.split_once('-'))
            .and_then(|(ls, _)| ls.parse().ok())
            .unwrap_or(0)
    } else {
        0
    };
    (
        row.archive as u8,
        is_event,
        path_part,
        line,
        row.reference.clone(),
        row.axis as u8,
        row.matched.clone(),
    )
}

/// 排序机械即档序固定、档内文件载体先事件载体后、路径字典序加行序、轴序末位决胜。
pub fn sort_rows(rows: &mut [FacetRow]) {
    rows.sort_by_key(sort_key);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::{Actor, ActorType};
    use crate::memory::{Archive, Axis, RecallError};
    use serde_json::json;

    /// F-9 截断即超限截断附三句点与限内原样。
    #[test]
    fn f9_excerpt_truncation() {
        let short = "短文本";
        assert_eq!(excerpt_of_text(short), "短文本");
        let long: String = "词".repeat(300);
        let cut = excerpt_of_text(&long);
        assert!(cut.ends_with("..."));
        assert_eq!(cut.trim_end_matches('.').chars().count(), 256);
        let exact: String = "词".repeat(256);
        assert_eq!(excerpt_of_text(&exact), exact);
    }

    /// F-9 排序即档序加档内文件先事件后加行序加轴序。
    #[test]
    fn f9_sort_rows_order() {
        let mk = |archive: Archive, carrier: &'static str, r: &str, axis: Axis| FacetRow {
            archive,
            carrier,
            reference: r.to_string(),
            axis,
            excerpt: String::new(),
            matched: String::new(),
            at: "t".into(),
        };
        let mut rows = vec![
            mk(Archive::Conclusion, "md", "sih-engine/sih/event/plan/b-results.md@9-9", Axis::Topic),
            mk(Archive::Fact, "event", "e2/aaaaaaaa", Axis::Time),
            mk(Archive::Conclusion, "md", "sih-engine/sih/event/plan/a-results.md@3-3", Axis::Topic),
            mk(Archive::Fact, "event", "e1/bbbbbbbb", Axis::Event),
            mk(Archive::Conclusion, "md", "sih-engine/sih/event/plan/a-results.md@1-1", Axis::Topic),
            mk(Archive::Fact, "event", "e1/bbbbbbbb", Axis::Time),
        ];
        sort_rows(&mut rows);
        let refs: Vec<&str> = rows.iter().map(|r| r.reference.as_str()).collect();
        assert_eq!(
            refs,
            vec![
                "e1/bbbbbbbb",
                "e1/bbbbbbbb",
                "e2/aaaaaaaa",
                "sih-engine/sih/event/plan/a-results.md@1-1",
                "sih-engine/sih/event/plan/a-results.md@3-3",
                "sih-engine/sih/event/plan/b-results.md@9-9",
            ]
        );
        assert_eq!(rows[0].axis, Axis::Event);
        assert_eq!(rows[1].axis, Axis::Time);
    }

    /// F-9 事件出处即标识加哈希八位前缀。
    #[test]
    fn f9_event_reference() {
        let e = Event {
            event_id: "id-1".into(),
            event_type: "t".into(),
            timestamp: chrono::Utc::now(),
            actor: Actor {
                actor_id: "a".into(),
                actor_type: ActorType::System,
                invoked_via: "test".into(),
            },
            details: Some(json!({})),
            doc_id: "d".into(),
            prev_hash: "0".into(),
            event_hash: "abcdef1234567890".into(),
            event_class: None,
            verification_result: None,
        };
        assert_eq!(event_reference(&e), "id-1/abcdef12");
    }

    /// 退出码映射即拦一异常二。
    #[test]
    fn f9_exit_code_mapping() {
        use crate::memory::exit_code;
        assert_eq!(exit_code(&RecallError::Blocked("x".into())), 1);
        assert_eq!(exit_code(&RecallError::MissingBase("locator".into())), 2);
        assert_eq!(exit_code(&RecallError::TargetUnreadable("t".into())), 2);
        assert_eq!(exit_code(&RecallError::OutUnwritable("o".into())), 2);
        assert_eq!(exit_code(&RecallError::Internal("i".into())), 2);
    }
}
