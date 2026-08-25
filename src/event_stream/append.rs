//! 追加写入入口，承接 DES-007#append-interface 与 SPEC-004#append-entry
//!
//! 追加写入是事件进入事件流的唯一合法通道。
//! 执行写入前确定性校验四项，校验通过则追加到事件流末尾。

use crate::event_stream::event::{ActorType, Event, EventInput};
use crate::event_stream::hash::{compute_event_hash, GENESIS_PREV_HASH};
use std::collections::HashSet;
use std::path::Path;

/// 追加写入结果（成功）
#[derive(Debug, Clone)]
pub struct AppendSuccess {
    pub event_id: String,
    pub event_hash: String,
    pub written_at: chrono::DateTime<chrono::Utc>,
}

/// 追加写入错误类型（失败）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppendError {
    /// event_id 在事件流中已存在
    DuplicateEventId(String),
    /// 时间戳小于等于上一事件的 timestamp
    TimestampNotMonotonic {
        event_timestamp: String,
        last_timestamp: String,
    },
    /// prev_hash 不等于前一事件的 event_hash
    PrevHashMismatch {
        provided: String,
        expected: String,
    },
    /// 操作者标识非法（actor_type = Agent 但 invoked_via 指向 LLM）
    IllegalActor {
        actor_id: String,
        actor_type: String,
        reason: String,
    },
    /// 事件类型要求 details 但未提供
    MissingRequiredDetails(String),
    /// 内部错误
    Internal(String),
}

/// 追加写入入口。
///
/// 校验项（承接 SPEC-004#acceptance-criteria 写入前校验）：
/// 1. event_id 全局唯一性
/// 2. 时间戳单调递增
/// 3. prev_hash 匹配前事件哈希
/// 4. 操作者合法性（actor_type = Agent 时须对应确定性程序）
///
/// # Arguments
/// * `input` — 待写入事件输入
/// * `store` — 当前事件存储（内存中的事件列表）
/// * `path` — NDJSON 文件路径（用于持久化）
///
/// # Panics
/// 文件写入失败时 panic（事件流不可部分写入）
pub fn append(
    input: EventInput,
    store: &mut Vec<Event>,
    path: Option<&Path>,
) -> Result<AppendSuccess, AppendError> {
    // ── 前置：校验 details 必须性 ─────────────────────────────────────────
    let requires_details = [
        "task_completion",
        "intent_anchored",
        "intent_revised",
        "symbol_generated",
        "crosscheck_completed",
    ];
    if requires_details.contains(&input.event_type.as_str()) && input.details.is_none() {
        return Err(AppendError::MissingRequiredDetails(input.event_type.clone()));
    }

    // ── 第一项校验：event_id 全局唯一性 ─────────────────────────────────
    {
        let seen_ids: HashSet<_> = store.iter().map(|e| e.event_id.clone()).collect();
        if seen_ids.contains(&input.event_id) {
            return Err(AppendError::DuplicateEventId(input.event_id));
        }
    }

    // ── 第二项校验：时间戳单调递增 ───────────────────────────────────────
    let prev_hash = if let Some(ref ph) = input.prev_hash {
        ph.clone()
    } else if let Some(last) = store.last() {
        last.event_hash.clone()
    } else {
        GENESIS_PREV_HASH.to_string()
    };

    if let Some(last) = store.last() {
        if input.timestamp <= last.timestamp {
            return Err(AppendError::TimestampNotMonotonic {
                event_timestamp: input.timestamp.to_rfc3339(),
                last_timestamp: last.timestamp.to_rfc3339(),
            });
        }
        // ── 第三项校验：prev_hash 匹配前事件哈希 ───────────────────────
        if input.prev_hash.is_some() && input.prev_hash.as_ref().unwrap() != &last.event_hash {
            return Err(AppendError::PrevHashMismatch {
                provided: input.prev_hash.as_ref().unwrap().clone(),
                expected: last.event_hash.clone(),
            });
        }
    } else if input.prev_hash.is_some() && input.prev_hash.as_ref().unwrap() != GENESIS_PREV_HASH {
        // 首个事件但提供了非 genesis prev_hash
        return Err(AppendError::PrevHashMismatch {
            provided: input.prev_hash.as_ref().unwrap().clone(),
            expected: GENESIS_PREV_HASH.to_string(),
        });
    }

    // ── 第四项校验：操作者合法性 ──────────────────────────────────────────
    validate_actor(&input.actor)?;

    // ── 构建完整事件 ─────────────────────────────────────────────────────
    let now = chrono::Utc::now();
    let event = Event {
        event_id: input.event_id.clone(),
        event_type: input.event_type.clone(),
        timestamp: input.timestamp,
        actor: input.actor.clone(),
        details: input.details.clone(),
        doc_id: input.doc_id.clone(),
        prev_hash: prev_hash.clone(),
        event_hash: String::new(), // 先占位，计算完再填
        event_class: input.event_class.clone(),
        verification_result: None,
    };

    let event_hash = compute_event_hash(&event);
    let mut final_event = event;
    final_event.event_hash = event_hash.clone();

    // ── 持久化到 NDJSON ─────────────────────────────────────────────────
    if let Some(p) = path {
        let line = final_event
            .to_json_line()
            .map_err(|e| AppendError::Internal(format!("JSON serialization failed: {e}")))?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(p)
            .map_err(|e| AppendError::Internal(format!("file open failed: {e}")))?;
        use std::io::Write;
        writeln!(file, "{line}").map_err(|e| AppendError::Internal(format!("write failed: {e}")))?;
    }

    // ── 更新内存存储 ─────────────────────────────────────────────────────
    store.push(final_event);

    Ok(AppendSuccess {
        event_id: input.event_id,
        event_hash,
        written_at: now,
    })
}

/// 第四项校验：操作者合法性。
/// actor_type = Agent 时，该代理须是确定性程序调用的执行者，不是 LLM 直接调用。
fn validate_actor(actor: &crate::event_stream::event::Actor) -> Result<(), AppendError> {
    if actor.actor_type == ActorType::Agent {
        // invoked_via 若含 LLM 标识，视为非法
        let llm_patterns = ["llm", "gpt", "claude", "gemini", "chatgpt", "o1-", "o3-"];
        let via_lower = actor.invoked_via.to_lowercase();
        for pattern in llm_patterns {
            if via_lower.contains(pattern) {
                return Err(AppendError::IllegalActor {
                    actor_id: actor.actor_id.clone(),
                    actor_type: format!("{:?}", actor.actor_type),
                    reason: format!(
                        "actor_type=Agent but invoked_via contains LLM identifier: {}",
                        actor.invoked_via
                    ),
                });
            }
        }
    }
    Ok(())
}

/// 从 NDJSON 文件加载全部事件到内存。
pub fn load_events(path: &Path) -> Result<Vec<Event>, AppendError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| AppendError::Internal(format!("read file failed: {e}")))?;
    let mut events = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let event: Event = serde_json::from_str(trimmed)
            .map_err(|e| AppendError::Internal(format!("parse line failed: {e}")))?;
        events.push(event);
    }
    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::{Actor, ActorType};
    use chrono::{TimeZone, Utc};

    fn make_input(event_id: &str, timestamp: &str) -> EventInput {
        EventInput {
            event_id: event_id.into(),
            event_type: "task_completion".into(),
            timestamp: Utc.timestamp_opt(
                chrono::DateTime::parse_from_rfc3339(timestamp).unwrap().timestamp(),
                0,
            )
            .unwrap(),
            actor: Actor {
                actor_id: "deterministic-proc".into(),
                actor_type: ActorType::Agent,
                invoked_via: "deterministic-cli".into(),
            },
            details: Some(serde_json::json!({"change_summary": "test"})),
            doc_id: "TEST-DOC".into(),
            prev_hash: None,
            event_class: None,
        }
    }

    #[test]
    fn test_append_first_event_success() {
        let mut store = Vec::new();
        let input = make_input("evt-001", "2026-07-27T13:00:00.000000+00:00");
        let result = append(input, &mut store, None);
        assert!(result.is_ok());
        let succ = result.unwrap();
        assert_eq!(succ.event_id, "evt-001");
        assert!(!succ.event_hash.is_empty());
        assert_eq!(store.len(), 1);
        assert_eq!(
            store[0].prev_hash,
            GENESIS_PREV_HASH
        );
    }

    #[test]
    fn test_append_second_event_success() {
        let mut store = Vec::new();
        let input1 = make_input("evt-001", "2026-07-27T13:00:00.000000+00:00");
        append(input1, &mut store, None).unwrap();

        let input2 = make_input("evt-002", "2026-07-27T14:00:00.000000+00:00");
        let result2 = append(input2, &mut store, None);
        assert!(result2.is_ok());
        let succ2 = result2.unwrap();
        // prev_hash 应等于 evt-001 的 event_hash
        assert_eq!(succ2.event_hash, store[1].event_hash);
        assert_eq!(store[1].prev_hash, store[0].event_hash);
    }

    #[test]
    fn test_reject_duplicate_event_id() {
        let mut store = Vec::new();
        let input1 = make_input("evt-dup", "2026-07-27T13:00:00.000000+00:00");
        append(input1, &mut store, None).unwrap();

        let input2 = make_input("evt-dup", "2026-07-27T14:00:00.000000+00:00");
        let result = append(input2, &mut store, None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppendError::DuplicateEventId(id) if id == "evt-dup"));
    }

    #[test]
    fn test_reject_non_monotonic_timestamp() {
        let mut store = Vec::new();
        let input1 = make_input("evt-001", "2026-07-27T14:00:00.000000+00:00");
        append(input1, &mut store, None).unwrap();

        let input2 = make_input("evt-002", "2026-07-27T13:00:00.000000+00:00");
        let result = append(input2, &mut store, None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppendError::TimestampNotMonotonic { .. }));
    }

    #[test]
    fn test_reject_prev_hash_mismatch() {
        let mut store = Vec::new();
        let input1 = make_input("evt-001", "2026-07-27T13:00:00.000000+00:00");
        append(input1, &mut store, None).unwrap();

        // 错误地提供错误的 prev_hash
        let mut input2 = make_input("evt-002", "2026-07-27T14:00:00.000000+00:00");
        input2.prev_hash = Some("wrong_hash_wrong_hash_wrong_hash".into());
        let result = append(input2, &mut store, None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppendError::PrevHashMismatch { .. }));
    }

    #[test]
    fn test_reject_illegal_actor_llm_invoked_via() {
        let mut store = Vec::new();
        let mut input = make_input("evt-001", "2026-07-27T13:00:00.000000+00:00");
        input.actor.invoked_via = "llm-direct-call".into();
        let result = append(input, &mut store, None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppendError::IllegalActor { .. }));
    }

    #[test]
    fn test_reject_missing_details_for_task_completion() {
        let mut store = Vec::new();
        let mut input = make_input("evt-001", "2026-07-27T13:00:00.000000+00:00");
        input.details = None;
        let result = append(input, &mut store, None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            AppendError::MissingRequiredDetails(t) if t == "task_completion"
        ));
    }
}
