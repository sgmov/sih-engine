//! 跨方核毕事件守卫与构建，承接 DES-011 输出契约与 SPEC-004 事件词表。
//!
//! crosscheck_completed 事件是执契 sign 动作的落链形态。守卫校执契核对
//! 报告十二字段冻结面，另从报告所指材料拈三字段即 topic_sha256 与
//! dc_fingerprint 与 n_shots 承 DES-011 载荷六扩展字段，报告本体不动
//! 即执契 watch 既有重放零破。材料缺席拒。event_class 按处置机械分即
//! 裁决通过取仅记录余取可消费，只记不判零 LLM。

use crate::event_stream::event::{Actor, EventInput};
use serde_json::Value;

/// 执契核对报告十二字段逐一必填，随 tally 1.0 报告冻结面冻结。
pub const CROSSCHECK_REPORT_FIELDS: [&str; 12] = [
    "gid",
    "disposition",
    "direction",
    "verdict",
    "gate_verdict",
    "rules_version",
    "material",
    "tool",
    "version",
    "passed",
    "failed",
    "alarms",
];

/// 从所指材料拈三字段承 DES-011 载荷扩展，事件侧冻结合计十五字段。
pub const CROSSCHECK_LIFT_FIELDS: [&str; 3] = ["topic_sha256", "dc_fingerprint", "n_shots"];

/// 守卫拒绝类型，逐字段定位不静默。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrosscheckError {
    /// JSON 解析失败或非对象
    Unparsable(String),
    /// 字段缺失，含缺哪些
    MissingField(String),
    /// 字段多出，含多哪些
    ExtraField(String),
    /// 字段形态违例，含字段名与事由
    BadField { field: String, reason: String },
    /// 报告所指材料缺席或不可读
    MaterialAbsent(String),
}

impl std::fmt::Display for CrosscheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrosscheckError::Unparsable(e) => write!(f, "报告不可解析 {e}"),
            CrosscheckError::MissingField(fld) => write!(f, "缺字段 {fld}"),
            CrosscheckError::ExtraField(fld) => write!(f, "多字段 {fld}"),
            CrosscheckError::BadField { field, reason } => {
                write!(f, "字段 {field} 形态违例 {reason}")
            }
            CrosscheckError::MaterialAbsent(p) => write!(f, "所指材料缺席 {p}"),
        }
    }
}

/// 六十四位小写十六进制。
fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// 规则版本 des 三位号加 r 数字形，规则增改走版本管理。
fn is_rules_version(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("des-") else {
        return false;
    };
    let Some((num, tail)) = rest.split_once("-r") else {
        return false;
    };
    !num.is_empty()
        && num.len() == 3
        && num.bytes().all(|b| b.is_ascii_digit())
        && !tail.is_empty()
        && tail.bytes().all(|b| b.is_ascii_digit())
}

/// 程序版本三点数字形。
fn is_semver(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    parts.len() == 3
        && parts.iter().all(|p| {
            !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit())
        })
}

/// gid 小写字母数字与短横线形，首字符非短横线。
fn is_gid(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let b = s.as_bytes();
    let head_ok = b[0].is_ascii_lowercase() || b[0].is_ascii_digit();
    head_ok
        && b.iter().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-'
        })
}

/// 材料引用相对路径 json 形，绝对路径拒。
fn is_material_ref(s: &str) -> bool {
    !s.is_empty() && !s.starts_with('/') && s.ends_with(".json")
}

fn bad(field: &str, reason: &str) -> CrosscheckError {
    CrosscheckError::BadField {
        field: field.to_string(),
        reason: reason.to_string(),
    }
}

/// 跨方核毕守卫：机械校报告十二字段形态加材料拈三，通过即返回十五字段事件负载。
/// material_text 即报告所指核对输入材料原文，缺席即拒。
pub fn guard_crosscheck(
    report_text: &str,
    material_text: Option<&str>,
) -> Result<Value, CrosscheckError> {
    let parsed: Value = serde_json::from_str(report_text)
        .map_err(|e| CrosscheckError::Unparsable(e.to_string()))?;
    let obj = parsed
        .as_object()
        .ok_or_else(|| CrosscheckError::Unparsable("报告非 JSON 对象".into()))?;

    let mut missing = Vec::new();
    for field in CROSSCHECK_REPORT_FIELDS {
        if !obj.contains_key(field) {
            missing.push(field.to_string());
        }
    }
    if !missing.is_empty() {
        return Err(CrosscheckError::MissingField(missing.join(",")));
    }

    let mut extra = Vec::new();
    for key in obj.keys() {
        if !CROSSCHECK_REPORT_FIELDS.contains(&key.as_str()) {
            extra.push(key.clone());
        }
    }
    if !extra.is_empty() {
        return Err(CrosscheckError::ExtraField(extra.join(",")));
    }

    if !obj["gid"]
        .as_str()
        .is_some_and(|s| is_gid(s))
    {
        return Err(bad("gid", "非小写字母数字短横线形"));
    }
    if !obj["disposition"].as_str().is_some_and(|s| !s.is_empty()) {
        return Err(bad("disposition", "处置空"));
    }
    if !obj["direction"].as_str().is_some_and(|s| !s.is_empty()) {
        return Err(bad("direction", "方向空"));
    }
    if !obj["verdict"]
        .as_str()
        .is_some_and(|s| s == "pass" || s == "fail")
    {
        return Err(bad("verdict", "非 pass 或 fail"));
    }
    if !obj["gate_verdict"].as_str().is_some_and(|s| !s.is_empty()) {
        return Err(bad("gate_verdict", "闸裁决空"));
    }
    if !obj["rules_version"]
        .as_str()
        .is_some_and(|s| is_rules_version(s))
    {
        return Err(bad("rules_version", "非 des 三位号 r 数字形"));
    }
    if !obj["material"]
        .as_str()
        .is_some_and(|s| is_material_ref(s))
    {
        return Err(bad("material", "非相对 json 路径"));
    }
    if !obj["tool"].as_str().is_some_and(|s| s == "tally") {
        return Err(bad("tool", "非 tally"));
    }
    if !obj["version"]
        .as_str()
        .is_some_and(|s| is_semver(s))
    {
        return Err(bad("version", "非三点数字形"));
    }
    if !obj["passed"].is_array() || !obj["failed"].is_array() || !obj["alarms"].is_array() {
        return Err(bad("passed/failed/alarms", "非数组"));
    }

    // 拈三：从所指核对输入材料取 DES-011 载荷扩展字段
    let material_ref = obj["material"].as_str().unwrap_or("");
    let material = match material_text {
        Some(text) => serde_json::from_str::<Value>(text).map_err(|e| {
            CrosscheckError::MaterialAbsent(format!("{material_ref} 不可解析 {e}"))
        })?,
        None => return Err(CrosscheckError::MaterialAbsent(material_ref.to_string())),
    };
    if !material
        .get("topic_sha256")
        .and_then(|v| v.as_str())
        .is_some_and(|s| is_sha256_hex(s))
    {
        return Err(bad("topic_sha256", "材料缺或非六十四位十六进制"));
    }
    if !material
        .get("dc_fingerprint")
        .and_then(|v| v.as_str())
        .is_some_and(|s| !s.is_empty())
    {
        return Err(bad("dc_fingerprint", "材料缺或指纹空"));
    }
    let n_shots = material.get("n_shots").and_then(|v| v.as_u64());
    if !n_shots.is_some_and(|n| n >= 1) {
        return Err(bad("n_shots", "材料缺或采样发数小于一"));
    }

    let mut payload = parsed;
    let map = payload.as_object_mut().unwrap();
    map.insert(
        "topic_sha256".into(),
        material["topic_sha256"].clone(),
    );
    map.insert(
        "dc_fingerprint".into(),
        material["dc_fingerprint"].clone(),
    );
    map.insert("n_shots".into(), material["n_shots"].clone());
    Ok(payload)
}

/// 跨方核毕事件构建：守卫通过后拼 EventInput，doc_id 即 crosscheck-gid，
/// 事件分类按处置机械分即裁决通过取仅记录余取可消费承 DES-011。
pub fn crosscheck_event(
    report_text: &str,
    material_text: &str,
    actor: Actor,
) -> Result<EventInput, CrosscheckError> {
    let payload = guard_crosscheck(report_text, Some(material_text))?;
    let gid = payload["gid"].as_str().unwrap_or("").to_string();
    let disposition = payload["disposition"].as_str().unwrap_or("").to_string();
    let event_class = if disposition == "裁决通过" {
        "record_only"
    } else {
        "consumable"
    }
    .to_string();
    Ok(EventInput {
        event_id: uuid::Uuid::new_v4().to_string(),
        event_type: "crosscheck_completed".to_string(),
        timestamp: chrono::Utc::now(),
        actor,
        details: Some(payload),
        doc_id: format!("crosscheck-{gid}"),
        prev_hash: None,
        event_class: Some(event_class),
        verification_result: None,
        session_id: None,
        identity_hash: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::event::ActorType;
    use crate::event_stream::{append, verify, VerifyRange};

    fn actor() -> Actor {
        Actor {
            actor_id: "tally".to_string(),
            actor_type: ActorType::System,
            invoked_via: "cli".to_string(),
        }
    }

    fn good_report() -> String {
        r#"{"alarms":[],"direction":"comply","disposition":"裁决通过","failed":[],"gate_verdict":"stable_clear","gid":"m-namefit","material":"reports/m-namefit-check-input.json","passed":["R1: 核对程序版本署名 tally/1.0.0"],"rules_version":"des-011-r1","tool":"tally","verdict":"pass","version":"1.0.0"}"#.into()
    }

    fn good_material() -> String {
        r#"{"kind":"tally-check-input","gid":"m-namefit","topic_path":"t.md","topic_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","dc_fingerprint":"fp-1","n_shots":3}"#.into()
    }

    #[test]
    fn c1_guard_accepts_well_formed() {
        let g = guard_crosscheck(&good_report(), Some(&good_material()));
        assert!(g.is_ok(), "{g:?}");
        let payload = g.unwrap();
        assert_eq!(payload["topic_sha256"].as_str().unwrap().len(), 64);
        assert_eq!(payload["n_shots"].as_u64(), Some(3));
    }

    #[test]
    fn c2_missing_and_extra_rejected() {
        let mut obj: Value = serde_json::from_str(&good_report()).unwrap();
        obj.as_object_mut().unwrap().remove("gate_verdict");
        let e = guard_crosscheck(&obj.to_string(), Some(&good_material()));
        assert!(matches!(e, Err(CrosscheckError::MissingField(_))));

        let mut obj2: Value = serde_json::from_str(&good_report()).unwrap();
        obj2.as_object_mut()
            .unwrap()
            .insert("suggestion".into(), Value::String("x".into()));
        let e2 = guard_crosscheck(&obj2.to_string(), Some(&good_material()));
        assert!(matches!(e2, Err(CrosscheckError::ExtraField(_))));
    }

    #[test]
    fn c3_material_absent_rejected() {
        let e = guard_crosscheck(&good_report(), None);
        assert!(matches!(e, Err(CrosscheckError::MaterialAbsent(_))));
        let bad_mat = good_material().replace(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "short",
        );
        let e2 = guard_crosscheck(&good_report(), Some(&bad_mat));
        assert!(matches!(e2, Err(CrosscheckError::BadField { field, .. }) if field == "topic_sha256"));
    }

    #[test]
    fn c4_shape_violations_rejected() {
        let cases: [(&str, Value); 6] = [
            ("verdict", Value::String("maybe".into())),
            ("tool", Value::String("gauge".into())),
            ("rules_version", Value::String("r1".into())),
            ("material", Value::String("/abs/x.json".into())),
            ("version", Value::String("1".into())),
            ("gid", Value::String("-bad".into())),
        ];
        for (field, value) in cases {
            let obj: Value = serde_json::from_str(&good_report()).unwrap();
            let mut o = obj.as_object().unwrap().clone();
            o.insert(field.to_string(), value);
            let e = guard_crosscheck(&serde_json::to_string(&o).unwrap(), Some(&good_material()));
            assert!(
                matches!(&e, Err(CrosscheckError::BadField { field: f, .. }) if f == field),
                "{field} 应形态拒，实得 {e:?}"
            );
        }
    }

    #[test]
    fn c5_event_class_by_disposition() {
        let ev = crosscheck_event(&good_report(), &good_material(), actor()).unwrap();
        assert_eq!(ev.event_type, "crosscheck_completed");
        assert_eq!(ev.doc_id, "crosscheck-m-namefit");
        assert_eq!(ev.event_class.as_deref(), Some("record_only"));

        let suspended = good_report()
            .replace("裁决通过", "挂起")
            .replace("\"verdict\":\"pass\"", "\"verdict\":\"fail\"");
        let ev2 = crosscheck_event(&suspended, &good_material(), actor()).unwrap();
        assert_eq!(ev2.event_class.as_deref(), Some("consumable"));
    }

    #[test]
    fn c6_append_and_verify_roundtrip() {
        let ev = crosscheck_event(&good_report(), &good_material(), actor()).unwrap();
        let mut store: Vec<crate::event_stream::event::Event> = Vec::new();
        assert!(append(ev, &mut store, None).is_ok());
        let verdict = verify(&store, VerifyRange::Full);
        assert!(verdict.is_ok(), "{verdict:?}");
    }
}
