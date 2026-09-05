//! 事件数据结构，承接 SPEC-004#data-contract 与 DES-007#data-structure
//!
//! 事件核心结构含八个必须字段与两个可选字段。
//! 操作者子结构三字段：actor_id、actor_type、invoked_via。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// 操作者子结构，标识事件的发起主体。
/// 承接 SPEC-004#actor-structure。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Actor {
    /// 操作者标识，须为合法的人类标识或确定性程序标识。
    pub actor_id: String,
    /// 操作者类型：human、agent、system。
    /// agent 须对应确定性程序调用的执行者，非 LLM 直接调用。
    #[serde(deserialize_with = "deserialize_actor_type")]
    pub actor_type: ActorType,
    /// 调用途径，标识操作者通过何种方式触发此事件。
    pub invoked_via: String,
}

/// actor_type 枚举，承接 SPEC-004#actor-structure 合法性约束。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActorType {
    Human,
    Agent,
    System,
}

/// 反序列化辅助：接受大小写不敏感的 actor_type。
fn deserialize_actor_type<'de, D>(deserializer: D) -> Result<ActorType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "human" => Ok(ActorType::Human),
        "agent" => Ok(ActorType::Agent),
        "system" => Ok(ActorType::System),
        _ => Err(serde::de::Error::custom(format!(
            "invalid actor_type: {s}, expected human/agent/system"
        ))),
    }
}

/// 事件核心结构，承接 SPEC-004#event-core 与 DES-007#event-core-structure。
///
/// 必须字段八个：event_id、event_type、timestamp、actor、details、
/// doc_id、prev_hash、event_hash。
/// 可选字段两个：event_class、verification_result。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Event {
    // 必须字段
    /// 事件全局唯一标识，UUID v4 格式。
    pub event_id: String,
    /// 事件类型标识，取值见 SPEC-004#event-type-hierarchy。
    pub event_type: String,
    /// ISO 8601 格式时间戳，事件流内单调递增。
    pub timestamp: DateTime<Utc>,
    /// 操作者子结构。
    pub actor: Actor,
    /// 事件负载，JSON 对象。部分事件类型允许省略。
    #[serde(default)]
    pub details: Option<JsonValue>,
    /// 关联文档标识，跨事件串联同一治理对象的生命周期。
    pub doc_id: String,
    /// 前事件哈希，首事件取全零值。
    pub prev_hash: String,
    /// 本事件哈希，由本事件除 event_hash 外的全部字段计算得出。
    pub event_hash: String,
    // 可选字段
    /// 事件分类：consumable 或 record_only，承接 SPEC-004#event-classification。
    #[serde(default)]
    pub event_class: Option<String>,
    /// 校验结果，部分事件类型承载。
    #[serde(default)]
    pub verification_result: Option<JsonValue>,
    /// 链上正身绑定（idenlane-solo iden-01）：事件信封所载会话标识。
    /// 租约笔承载会话号；直改笔为空或直改标记。旧事件行缺省即 None，
    /// 零数据迁移零改动，verify 重算兼容（新增字段只出现在新行）。
    #[serde(default)]
    pub session_id: Option<String>,
    /// 链上正身绑定（idenlane-solo iden-01）：生产者身份哈希。
    /// 租约笔按会话号从台账查得；直改笔按身份件查得。旧事件行缺省即 None。
    #[serde(default)]
    pub identity_hash: Option<String>,
}

/// 构建 Event 时的输入结构，不含 computed 字段（event_hash、prev_hash）。
/// 用于追加写入入口的输入。
#[derive(Debug, Clone)]
pub struct EventInput {
    pub event_id: String,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub actor: Actor,
    pub details: Option<JsonValue>,
    pub doc_id: String,
    /// 可选，调用方若已计算则传入；否则由 append 内部填充。
    pub prev_hash: Option<String>,
    /// 可选事件分类，缺省 None 即不入分类字段，三问意图事件固定 record_only。
    pub event_class: Option<String>,
    /// 可选校验结果，承 SPEC-006 报告消费入口透传，缺省 None。
    pub verification_result: Option<JsonValue>,
    /// 链上正身绑定写入（idenlane-solo iden-01）：会话号与生产者身份
    /// 哈希，追加时由调用方按会话台账或直改身份件解析后填入，None 即旧
    /// 行为零改动。
    pub session_id: Option<String>,
    pub identity_hash: Option<String>,
}

impl Event {
    /// 将 Event 反序列化为 JSON 行（单行 NDJSON 格式）。
    pub fn to_json_line(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// 从 JSON 行解析 Event。
    pub fn from_json_line(line: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_type_case_insensitive() {
        // 大写
        let json = r#"{"actor_id":"test","actor_type":"Agent","invoked_via":"cli"}"#;
        let actor: Actor = serde_json::from_str(json).unwrap();
        assert_eq!(actor.actor_type, ActorType::Agent);

        // 小写
        let json2 = r#"{"actor_id":"test","actor_type":"agent","invoked_via":"cli"}"#;
        let actor2: Actor = serde_json::from_str(json2).unwrap();
        assert_eq!(actor2.actor_type, ActorType::Agent);

        // 混合
        let json3 = r#"{"actor_id":"test","actor_type":"HUMAN","invoked_via":"cli"}"#;
        let actor3: Actor = serde_json::from_str(json3).unwrap();
        assert_eq!(actor3.actor_type, ActorType::Human);
    }

    #[test]
    fn test_event_roundtrip() {
        let event = Event {
            event_id: "f9a88582-fd53-4843-a849-f1c42b2d651c".into(),
            event_type: "task_completion".into(),
            timestamp: chrono::DateTime::parse_from_rfc3339("2026-07-27T13:19:47.234923+00:00")
                .unwrap()
                .with_timezone(&Utc),
            actor: Actor {
                actor_id: "zcode-main".into(),
                actor_type: ActorType::Agent,
                invoked_via: "zcode-mcp".into(),
            },
            details: Some(serde_json::json!({"change_summary": "test"})),
            doc_id: "TEST-DOC".into(),
            prev_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            event_hash: "abc123".into(),
            event_class: Some("consumable".into()),
            verification_result: None,
            session_id: None,
            identity_hash: None,
        };

        let line = event.to_json_line().unwrap();
        let parsed: Event = Event::from_json_line(&line).unwrap();
        assert_eq!(parsed.event_id, event.event_id);
        assert_eq!(parsed.event_hash, event.event_hash);
    }
}
