//! 直改链笔入口，承接 [pen-solo] 直改申报声明。
//!
//! 直改链笔 = 显式申报受治理但未持租约锁面的改档动作。事件类型
//! `direct_edit_completed` 落在同一条 trail 哈希链上，事件分类
//! `record_only`（声明型事件不被消费回看）。守卫严校：JSON 必为对象、
//! 路径非空、事由非空、申报人身份可解析（即 actor_id 非空），缺即拒
//! 不入流。这是这个仓的纪律——直改申报必留下不可篡改的留痕。

use crate::event_stream::event::{Actor, EventInput};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};

/// 直改链笔守卫拒绝类型，逐字段定位不静默。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PenError {
    /// JSON 解析失败
    RecordNotJson,
    /// 解析成功但非 JSON 对象（数组或标量）
    RecordNotObject,
    /// 必填字段缺失或为空串
    MissingField(&'static str),
}

impl std::fmt::Display for PenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PenError::RecordNotJson => write!(f, "记录非 JSON"),
            PenError::RecordNotObject => write!(f, "记录非 JSON 对象"),
            PenError::MissingField(field) => write!(f, "缺字段 {field}"),
        }
    }
}

/// 直改链笔事件构建：路径 + 事由 + 申报人三必填，doc_id 由路径派生。
///
/// 严校四道门按序：
/// 1. JSON 解析（RecordNotJson）
/// 2. 对象形态（RecordNotObject）
/// 3. 三必填字段非空即 path / subject / actor_id 缺一即拒（MissingField）
/// 4. 字段值非空串（同一 MissingField 变体，区分调用方可读性）
///
/// 拒绝时不静默、不伪造默认值（这是这个仓的纪律：直改申报必留下不可
/// 篡改的留痕，伪造 actor_id 或空事由等于污染审计面）。可选字段
/// `rationale` 缺省即空串保留。
///
/// doc_id 派生策略：`direct-edit-<path 末段>`。同路径多次直改申报落在
/// 不同事件（event_id 不同）共享同一 doc_id，便于守卫按路径聚合核验。
/// 路径以 `/` 结尾或无末段时退化为原路径字符串以保留可读性。
pub fn pen_event(
    record_text: &str,
    actor: Actor,
    timestamp: DateTime<Utc>,
) -> Result<EventInput, PenError> {
    let record: Value = serde_json::from_str(record_text).map_err(|_| PenError::RecordNotJson)?;
    let obj = record.as_object().ok_or(PenError::RecordNotObject)?;

    let path = obj
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or(PenError::MissingField("path"))?;
    if path.is_empty() {
        return Err(PenError::MissingField("path"));
    }
    let subject = obj
        .get("subject")
        .and_then(|v| v.as_str())
        .ok_or(PenError::MissingField("subject"))?;
    if subject.is_empty() {
        return Err(PenError::MissingField("subject"));
    }
    let actor_id = obj
        .get("actor_id")
        .and_then(|v| v.as_str())
        .ok_or(PenError::MissingField("actor_id"))?;
    if actor_id.is_empty() {
        return Err(PenError::MissingField("actor_id"));
    }

    let rationale = obj
        .get("rationale")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let path_tail = std::path::Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path);
    let event_id = uuid::Uuid::new_v4().to_string();
    let doc_id = format!("direct-edit-{path_tail}");

    // details schema 与 direct 笔（idenlane-envelope-solo 批）同构：files 复数
    // 数组是守卫第四面（guardcore._read_pen_face）唯一读取的键位。pen 笔在此
    // 不另立形态——两入口同事件类型若分两套 schema，守卫只认其中一套，另一套
    // 写的申报在守卫面完全不可见。故单路径申报归一为单元素 files 数组。
    let details = json!({
        "pen_form": "direct",
        "pen_kind": "human",
        "actor_id": actor_id,
        "files": [path],
        "subject": subject,
        "rationale": rationale,
        "timestamp": timestamp.to_rfc3339(),
    });

    Ok(EventInput {
        event_id,
        event_type: "direct_edit_completed".to_string(),
        timestamp,
        actor,
        details: Some(details),
        doc_id,
        prev_hash: None,
        event_class: Some("record_only".to_string()),
        verification_result: None,
        session_id: None,
        identity_hash: None,
    })
}

#[cfg(test)]
mod pen_tests {
    use super::*;
    use crate::event_stream::event::ActorType;
    use crate::event_stream::{append, verify, VerifyRange};

    fn actor() -> Actor {
        Actor {
            actor_id: "scribe".to_string(),
            actor_type: ActorType::System,
            invoked_via: "cli".to_string(),
        }
    }

    fn ts() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-09-29T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    #[test]
    fn p1_guard_accepts_well_formed() {
        let rec = r#"{"path":"sih-engine/src/foo.rs","subject":"修复直改链笔","actor_id":"dev-1"}"#;
        let out = pen_event(rec, actor(), ts()).unwrap();
        assert_eq!(out.event_type, "direct_edit_completed");
        assert_eq!(out.event_class.as_deref(), Some("record_only"));
        assert!(out.doc_id.starts_with("direct-edit-"));
        let d = out.details.expect("details 在场");
        // schema 与 direct 笔同构：files 复数数组是守卫第四面唯一读取的键位
        assert_eq!(d["files"][0], "sih-engine/src/foo.rs");
        assert_eq!(d["pen_form"], "direct");
        assert_eq!(d["subject"], "修复直改链笔");
        assert_eq!(d["actor_id"], "dev-1");
        assert_eq!(d["rationale"], "");
    }

    #[test]
    fn p2_guard_rejects_missing_required_fields() {
        let no_path = r#"{"subject":"事由","actor_id":"dev-1"}"#;
        assert!(matches!(
            pen_event(no_path, actor(), ts()),
            Err(PenError::MissingField("path"))
        ));
        let no_subj = r#"{"path":"x","actor_id":"dev-1"}"#;
        assert!(matches!(
            pen_event(no_subj, actor(), ts()),
            Err(PenError::MissingField("subject"))
        ));
        let no_actor = r#"{"path":"x","subject":"事由"}"#;
        assert!(matches!(
            pen_event(no_actor, actor(), ts()),
            Err(PenError::MissingField("actor_id"))
        ));
    }

    #[test]
    fn p3_guard_rejects_empty_required_fields() {
        let empty_path = r#"{"path":"","subject":"事由","actor_id":"dev-1"}"#;
        assert!(matches!(
            pen_event(empty_path, actor(), ts()),
            Err(PenError::MissingField("path"))
        ));
        let empty_subj = r#"{"path":"x","subject":"","actor_id":"dev-1"}"#;
        assert!(matches!(
            pen_event(empty_subj, actor(), ts()),
            Err(PenError::MissingField("subject"))
        ));
        let empty_actor = r#"{"path":"x","subject":"事由","actor_id":""}"#;
        assert!(matches!(
            pen_event(empty_actor, actor(), ts()),
            Err(PenError::MissingField("actor_id"))
        ));
    }

    #[test]
    fn p4_guard_rejects_non_json_and_non_object() {
        let not_json = "not json at all";
        assert!(matches!(
            pen_event(not_json, actor(), ts()),
            Err(PenError::RecordNotJson)
        ));
        let arr = r#"[1, 2, 3]"#;
        assert!(matches!(
            pen_event(arr, actor(), ts()),
            Err(PenError::RecordNotObject)
        ));
        let scalar = r#""just a string""#;
        assert!(matches!(
            pen_event(scalar, actor(), ts()),
            Err(PenError::RecordNotObject)
        ));
    }

    #[test]
    fn p5_doc_id_uses_path_tail() {
        let rec = r#"{"path":"a/b/c/file.rs","subject":"x","actor_id":"y"}"#;
        let out = pen_event(rec, actor(), ts()).unwrap();
        assert_eq!(out.doc_id, "direct-edit-file.rs");
    }

    #[test]
    fn p6_rationale_optional_default_empty() {
        let no_rat = r#"{"path":"x","subject":"s","actor_id":"a"}"#;
        let out = pen_event(no_rat, actor(), ts()).unwrap();
        let d = out.details.unwrap();
        assert_eq!(d["rationale"], "");
    }

    #[test]
    fn p7_pen_event_appends_and_chain_verifies() {
        let dir = std::env::temp_dir().join(format!("pen-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let trail = dir.join("trail.ndjson");
        let _ = std::fs::remove_file(&trail);
        let mut store = Vec::new();
        let rec = r#"{"path":"sih-engine/src/foo.rs","subject":"单测","actor_id":"dev-1","rationale":"端到端"}"#;
        let input = pen_event(rec, actor(), ts()).unwrap();
        append(input, &mut store, Some(&trail)).unwrap();
        let events = crate::event_stream::load_events(&trail).unwrap();
        assert_eq!(events[0].event_type, "direct_edit_completed");
        assert_eq!(events[0].event_class.as_deref(), Some("record_only"));
        assert_eq!(events[0].doc_id, "direct-edit-foo.rs");
        assert!(verify(&events, VerifyRange::Full).is_ok());
        let _ = std::fs::remove_file(&trail);
    }
}