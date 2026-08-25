//! 意图事件构建，承接 SPEC-005#persistence。
//!
//! 输出记录经验收后构建 intent_refined 事件的写入输入，
//! event_class 固定 record_only，doc_id 取会话标识，
//! 写入经 event_stream append 即书简融回位。

use chrono::Utc;
use uuid::Uuid;

use crate::event_stream::event::{Actor, EventInput};

use super::record::OutputRecord;

/// intent_refined 事件类型名，SPEC-004 事件类型层级治理环层登记在案。
pub const INTENT_EVENT_TYPE: &str = "intent_refined";
/// 意图提炼事件的固定分类，仅记录不进人类视图。
pub const INTENT_EVENT_CLASS: &str = "record_only";

/// 由输出记录构建意图事件的写入输入。
///
/// 调用方须先经 [`super::validate::validate`] 验收通过，本函数不复验。
pub fn intent_event_input(record: &OutputRecord, actor: Actor) -> Result<EventInput, serde_json::Error> {
    Ok(EventInput {
        event_id: Uuid::new_v4().to_string(),
        event_type: INTENT_EVENT_TYPE.into(),
        timestamp: Utc::now(),
        actor,
        details: Some(serde_json::to_value(record)?),
        doc_id: record.session_id.clone(),
        prev_hash: None,
        event_class: Some(INTENT_EVENT_CLASS.into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ask3repeater::record::{
        Anchor, Domain, DomainContract, DomainTag, InquiryStage, IntentContract, PhilosophyRef,
    };
    use crate::event_stream::event::ActorType;

    fn record() -> OutputRecord {
        OutputRecord {
            session_id: "sess-gate-test".into(),
            raw_input: "三问要建".into(),
            round: 1,
            intent_contract: IntentContract {
                goal: "建三问组件".into(),
                exclusions: vec![],
                output_format: "事件构建测试".into(),
                injected_constraints: vec![],
            },
            domain_contract: DomainContract {
                target_domain: Domain {
                    scope: "门构建测试".into(),
                    max_depth: 1,
                },
                support_domain: Domain {
                    scope: "背景".into(),
                    max_depth: 1,
                },
            },
            anchors: vec![Anchor {
                anchor_seq: 0,
                text: "测试锚点".into(),
                inquiry_stage: InquiryStage::First,
                domain_tag: DomainTag::Target,
                depth: 0,
                philosophy_ref: PhilosophyRef {
                    source: "philosophy.md".into(),
                    quote: "引文".into(),
                },
                rationale: "测试".into(),
                evidence: "raw_input:1".into(),
                confidence: 0.9,
            }],
            calls_in: 1,
            calls_out: 1,
        }
    }

    #[test]
    fn test_intent_event_fields() {
        let actor = Actor {
            actor_id: "ask3repeater".into(),
            actor_type: ActorType::Agent,
            invoked_via: "engine-src".into(),
        };
        let input = intent_event_input(&record(), actor).unwrap();
        assert_eq!(input.event_type, "intent_refined");
        assert_eq!(input.event_class.as_deref(), Some("record_only"));
        assert_eq!(input.doc_id, "sess-gate-test");
        let details = input.details.unwrap();
        assert_eq!(details["session_id"], "sess-gate-test");
        assert_eq!(details["anchors"].as_array().unwrap().len(), 1);
    }
}
