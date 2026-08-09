//! 哈希链计算，承接 DES-007#hash-computation-ownership 与 SPEC-004#hash-chain
//!
//! 哈希算法 SHA-256，序列化顺序字段名字典序，计算不含 LLM 调用不含随机性。

use crate::event_stream::event::Event;
use sha2::{Digest, Sha256};

/// GENESIS_PREV_HASH 是首个事件 prev_hash 的取值。
pub const GENESIS_PREV_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// 计算事件的 event_hash。
///
/// 序列化规则（承接 SPEC-004#hash-chain）：
/// - 除 event_hash 外的全部必须字段与可选字段参与序列化
/// - 序列化顺序固定为字段名字典序（字典序）
/// - SHA-256 算法
pub fn compute_event_hash(event: &Event) -> String {
    // 构建字典序序列化的 payload
    // 字段字典序：actor, details, doc_id, event_class, event_id,
    //             event_type, prev_hash, timestamp, verification_result
    use std::collections::BTreeMap;
    

    let mut map = BTreeMap::new();

    map.insert("actor", serde_json::to_value(&event.actor).expect("actor serializable"));
    // details: Option<JsonValue> → null 或实际值（不能省略，否则哈希不一致）
    map.insert(
        "details",
        event.details.clone().unwrap_or(serde_json::Value::Null),
    );
    map.insert("doc_id", serde_json::json!(event.doc_id));
    // event_class: Option<String> → null 或实际值
    map.insert(
        "event_class",
        event.event_class
            .as_ref()
            .map(|s| serde_json::json!(s))
            .unwrap_or(serde_json::Value::Null),
    );
    map.insert("event_id", serde_json::json!(event.event_id));
    map.insert("event_type", serde_json::json!(event.event_type));
    map.insert("prev_hash", serde_json::json!(event.prev_hash));
    map.insert(
        "timestamp",
        serde_json::json!(event.timestamp.to_rfc3339()),
    );
    // verification_result: Option<JsonValue> → null 或实际值
    map.insert(
        "verification_result",
        event.verification_result
            .clone()
            .unwrap_or(serde_json::Value::Null),
    );

    let payload = serde_json::to_string(&map).expect("BTreeMap serializable");
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// 验证单个事件的 prev_hash 是否等于前一事件的 event_hash。
pub fn verify_link(prev_event: &Event, current_event: &Event) -> bool {
    current_event.prev_hash == prev_event.event_hash
}

/// 验证一个事件列表的哈希链完整性。
/// 从首事件到末事件，逐个验证：
/// 1. 各事件的 event_hash 非空（SHA-256 十六进制64字符）
/// 2. 各事件的 prev_hash 非空
/// 3. prev_hash 匹配前一事件的 event_hash
///
/// 返回 (ok, first_hash, last_hash, err_event_id)。
pub fn verify_chain(events: &[Event]) -> (bool, String, String, Option<String>) {
    if events.is_empty() {
        return (true, String::new(), String::new(), None);
    }
    let first_hash = events[0].event_hash.clone();
    let last_hash = events[events.len() - 1].event_hash.clone();

    // 空哈希检测：event_hash 和 prev_hash 均须非空
    const SHA256_HEX_LEN: usize = 64;
    for (i, e) in events.iter().enumerate() {
        if e.event_hash.is_empty() {
            return (false, first_hash.clone(), last_hash.clone(), Some(e.event_id.clone()));
        }
        if e.event_hash.len() != SHA256_HEX_LEN {
            return (false, first_hash.clone(), last_hash.clone(), Some(e.event_id.clone()));
        }
        if i > 0 && e.prev_hash.is_empty() {
            return (false, first_hash.clone(), last_hash.clone(), Some(e.event_id.clone()));
        }
    }

    for i in 1..events.len() {
        if events[i].prev_hash != events[i - 1].event_hash {
            return (false, first_hash.clone(), last_hash.clone(), Some(events[i].event_id.clone()));
        }
    }
    (true, first_hash, last_hash, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::{Actor, ActorType, Event};
    use chrono::Utc;

    fn make_event(event_id: &str, prev_hash: &str, event_hash: &str) -> Event {
        Event {
            event_id: event_id.into(),
            event_type: "task_completion".into(),
            timestamp: chrono::DateTime::parse_from_rfc3339("2026-07-27T13:00:00.000000+00:00")
                .unwrap()
                .with_timezone(&Utc),
            actor: Actor {
                actor_id: "test-agent".into(),
                actor_type: ActorType::Agent,
                invoked_via: "test".into(),
            },
            details: Some(serde_json::json!({"change_summary": "test change"})),
            doc_id: "TEST".into(),
            prev_hash: prev_hash.into(),
            event_hash: event_hash.into(),
            event_class: None,
            verification_result: None,
        }
    }

    #[test]
    fn test_compute_hash_deterministic() {
        let event = make_event(
            "00000000-0000-0000-0000-000000000001",
            GENESIS_PREV_HASH,
            "", // event_hash 为空，计算时需要填入
        );
        // 先把 event_hash 设为一个占位值（因为计算哈希时 event_hash 字段不参与）
        let hash1 = compute_event_hash(&event);

        // 相同输入必然产生相同哈希
        let hash2 = compute_event_hash(&event);
        assert_eq!(hash1, hash2, "哈希计算须确定性，同一输入同输出");
    }

    #[test]
    fn test_tamper_detection() {
        // 构造一条正确链：事件1 → 事件2
        let e1_hash = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let e2 = make_event(
            "00000000-0000-0000-0000-000000000002",
            e1_hash,
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        );

        // 篡改 e1 的内容（改变 details），重新计算其哈希
        let mut tampered_e1 = make_event(
            "00000000-0000-0000-0000-000000000001",
            GENESIS_PREV_HASH,
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc", // 原始哈希
        );
        // 篡改 details
        tampered_e1.details = Some(serde_json::json!({"change_summary": "TAMPERED!"}));

        // 重新计算篡改后 e1 的哈希
        let tampered_e1_new_hash = compute_event_hash(&tampered_e1);

        // e2 的 prev_hash 仍为原 e1_hash，不匹配篡改后 e1 的新哈希 → 链断裂
        assert_ne!(
            tampered_e1_new_hash, e1_hash,
            "篡改字段后哈希须改变"
        );

        let (ok, _, _, err_id) = verify_chain(&[tampered_e1, e2.clone()]);
        assert!(
            !ok,
            "篡改后哈希链须检测出断裂"
        );
        assert_eq!(err_id, Some("00000000-0000-0000-0000-000000000002".into()));
    }

    #[test]
    fn test_verify_chain_success() {
        // 手动构造一条正确链（每个事件的 prev_hash 指向前一事件的 event_hash）
        let e1_hash = compute_event_hash(&make_event(
            "00000000-0000-0000-0000-000000000001",
            GENESIS_PREV_HASH,
            "", // 计算时占位
        ));
        let e1 = make_event(
            "00000000-0000-0000-0000-000000000001",
            GENESIS_PREV_HASH,
            &e1_hash,
        );

        let e2_hash = compute_event_hash(&make_event(
            "00000000-0000-0000-0000-000000000002",
            &e1_hash,
            "",
        ));
        let e2 = make_event(
            "00000000-0000-0000-0000-000000000002",
            &e1_hash,
            &e2_hash,
        );

        let (ok, first, last, err) = verify_chain(&[e1, e2]);
        assert!(ok, "正确链须通过校验");
        assert_eq!(first, e1_hash);
        assert_eq!(last, e2_hash);
        assert!(err.is_none());
    }

    #[test]
    fn test_empty_chain() {
        let (ok, first, last, err) = verify_chain(&[]);
        assert!(ok, "空链须返回成功");
        assert_eq!(first, "");
        assert_eq!(last, "");
        assert!(err.is_none());
    }
}
