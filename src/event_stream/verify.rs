//! 哈希链校验入口，承接 DES-007#verify-interface 与 SPEC-004#verify-entry
//!
//! 接收校验范围（全量或区间），验证哈希链完整性，返回校验结果。

use crate::event_stream::event::Event;


/// 校验范围
#[derive(Debug, Clone)]
pub enum VerifyRange {
    /// 全量校验
    Full,
    /// 区间校验：起始事件 ID 到结束事件 ID（含首尾）
    Range {
        start_event_id: String,
        end_event_id: String,
    },
}

/// 校验结果（成功）
#[derive(Debug, Clone)]
pub struct VerifySuccess {
    /// 校验的事件数
    pub event_count: usize,
    /// 首事件哈希
    pub first_hash: String,
    /// 末事件哈希
    pub last_hash: String,
    /// 校验时间戳
    pub verified_at: chrono::DateTime<chrono::Utc>,
}

/// 校验错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    /// 哈希链断裂：某事件的 prev_hash 与前一事件的 event_hash 不一致
    HashChainBroken {
        /// 断裂处的事件 ID
        broken_event_id: String,
    },
    /// 事件 ID 缺失：无法定位校验起点
    EventIdNotFound(String),
    /// 事件列表为空
    EmptyStream,
    /// 区间起始 > 区间结束
    InvalidRange,
}

/// 哈希链校验入口。
///
/// 校验范围支持全量与区间。
/// 错误类型覆盖哈希链断裂与事件 ID 缺失两类。
///
/// 载体引用（承接 ORD-019 版本偏序与外化状态存储）：verify 即全链自最小元
/// 起的传递性复算，逐事件核对 prev_hash 是否等于前事件 event_hash，构成
/// 版本偏序的可审计性机械实现。复算位即 hash::verify_chain，推导见
/// sih-math/docs/scriwire-scribe-derivation-2026-09-03.md。
pub fn verify(events: &[Event], range: VerifyRange) -> Result<VerifySuccess, VerifyError> {
    if events.is_empty() {
        return Err(VerifyError::EmptyStream);
    }

    let (start_idx, end_idx) = match range {
        VerifyRange::Full => (0, events.len()),
        VerifyRange::Range {
            start_event_id,
            end_event_id,
        } => {
            let start_idx = events
                .iter()
                .position(|e| e.event_id == start_event_id)
                .ok_or_else(|| VerifyError::EventIdNotFound(start_event_id))?;
            let end_idx = events
                .iter()
                .rposition(|e| e.event_id == end_event_id)
                .ok_or_else(|| VerifyError::EventIdNotFound(end_event_id.clone()))?;
            if start_idx > end_idx {
                return Err(VerifyError::InvalidRange);
            }
            // 含尾：end_idx + 1
            (start_idx, end_idx + 1)
        }
    };

    let slice = &events[start_idx..end_idx];
    let (ok, first_hash, last_hash, err_event_id) = crate::event_stream::hash::verify_chain(slice);

    if !ok {
        return Err(VerifyError::HashChainBroken {
            broken_event_id: err_event_id.unwrap_or_else(|| slice[0].event_id.clone()),
        });
    }

    Ok(VerifySuccess {
        event_count: slice.len(),
        first_hash,
        last_hash,
        verified_at: chrono::Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::{Actor, ActorType, Event};
    use crate::event_stream::hash::GENESIS_PREV_HASH;
    use chrono::Utc;

    fn make_event(
        idx: usize,
        prev_hash: &str,
        event_hash: &str,
    ) -> Event {
        Event {
            event_id: format!("{:04}", idx),
            event_type: "task_completion".into(),
            timestamp: chrono::DateTime::parse_from_rfc3339(&format!(
                "2026-07-27T{:02}:00:00.000000+00:00",
                idx
            ))
            .unwrap()
            .with_timezone(&Utc),
            actor: Actor {
                actor_id: "test".into(),
                actor_type: ActorType::Agent,
                invoked_via: "test".into(),
            },
            details: Some(serde_json::json!({"change_summary": "test"})),
            doc_id: "TEST".into(),
            prev_hash: prev_hash.into(),
            event_hash: event_hash.into(),
            event_class: None,
            verification_result: None,
        }
    }

    fn build_correct_chain(n: usize) -> Vec<Event> {
        let mut events = Vec::new();
        let mut prev_hash = GENESIS_PREV_HASH.to_string();
        for i in 0..n {
            let e_hash = format!("{:064x}", i);
            events.push(make_event(i, &prev_hash, &e_hash));
            prev_hash = e_hash;
        }
        events
    }

    #[test]
    fn test_full_verify_success() {
        let events = build_correct_chain(3);
        let result = verify(&events, VerifyRange::Full);
        assert!(result.is_ok());
        let succ = result.unwrap();
        assert_eq!(succ.event_count, 3);
        assert_eq!(succ.first_hash, "0000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(succ.last_hash, format!("{:064x}", 2));
    }

    #[test]
    fn test_range_verify_success() {
        let events = build_correct_chain(4);
        let result = verify(
            &events,
            VerifyRange::Range {
                start_event_id: "0001".into(),
                end_event_id: "0002".into(),
            },
        );
        assert!(result.is_ok());
        let succ = result.unwrap();
        assert_eq!(succ.event_count, 2);
    }

    #[test]
    fn test_hash_chain_broken() {
        let mut events = build_correct_chain(3);
        // 篡改第二个事件的 prev_hash
        events[1].prev_hash = "wrong_hash_wrong_hash_wrong_hash_wrong_hash_wrong_hash_wrong_hash".into();

        let result = verify(&events, VerifyRange::Full);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, VerifyError::HashChainBroken { .. }));
    }

    #[test]
    fn test_event_id_not_found() {
        let events = build_correct_chain(2);
        let result = verify(
            &events,
            VerifyRange::Range {
                start_event_id: "nonexistent".into(),
                end_event_id: "0001".into(),
            },
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, VerifyError::EventIdNotFound(id) if id == "nonexistent"));
    }

    #[test]
    fn test_empty_stream_error() {
        let result = verify(&[], VerifyRange::Full);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyError::EmptyStream);
    }

    #[test]
    fn test_invalid_range() {
        let events = build_correct_chain(3);
        let result = verify(
            &events,
            VerifyRange::Range {
                start_event_id: "0002".into(),
                end_event_id: "0000".into(),
            },
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyError::InvalidRange);
    }
}
