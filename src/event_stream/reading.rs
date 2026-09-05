//! 读数事件守卫与构建，承接 SPEC-011#data-contract 与 SPEC-011#persistence。
//!
//! reading_recorded 事件是秤星 record 动作的落链形态。守卫只校字段形态
//! 不评读数高低，即只报不判的写入侧对应：七必填字段缺即拒、扩展字段
//! 形态违例即拒、维度三枚举、值零到一或 insufficient 标记、公式版本 ga
//! 数字形、摘要六十四位十六进制。ga-2 增量扩展即七必填字段只增不删，
//! confidence_band 与 posterior_mean 两扩展字段仅 adoption 维出且可选，
//! ga-1 七字段读数回放兼容。建议排序自动处置三字段经多字段拒显式拦，
//! 零 LLM 只记不判。

use crate::event_stream::event::{Actor, EventInput};
use serde_json::Value;

/// 读数字段七件逐一必填，随 SPEC-011 接口冻结面冻结。
pub const READING_FIELDS: [&str; 7] = [
    "dimension",
    "subject",
    "value",
    "window",
    "formula_version",
    "computed_at",
    "inputs_digest",
];

/// ga-2 增量扩展字段，可选且仅 adoption 维出，随 SPEC-011 修订记录扩展。
pub const READING_EXT_FIELDS: [&str; 2] = ["confidence_band", "posterior_mean"];

/// 维度轴值三枚举，随 SPEC-011 三维定义冻结。
pub const READING_DIMENSIONS: [&str; 3] = ["convergence", "adoption", "mergeback"];

/// 守卫拒绝类型，逐字段定位不静默。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadingError {
    /// JSON 解析失败或非对象
    Unparsable(String),
    /// 字段缺失，含缺哪些
    MissingField(String),
    /// 字段多出，含多哪些；建议排序自动处置类字段在此显式露形
    ExtraField(String),
    /// 字段形态违例，含字段名与事由
    BadField { field: String, reason: String },
}

impl std::fmt::Display for ReadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadingError::Unparsable(e) => write!(f, "读数不可解析 {e}"),
            ReadingError::MissingField(fld) => write!(f, "缺字段 {fld}"),
            ReadingError::ExtraField(fld) => write!(f, "多字段 {fld}"),
            ReadingError::BadField { field, reason } => {
                write!(f, "字段 {field} 形态违例 {reason}")
            }
        }
    }
}

/// 六十四位小写十六进制。
fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// 参照时间 YYYY-MM-DD 形，逐字回显不转时区。
fn is_iso_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[0..4].iter().all(|c| c.is_ascii_digit())
        && b[4] == b'-'
        && b[5..7].iter().all(|c| c.is_ascii_digit())
        && b[7] == b'-'
        && b[8..10].iter().all(|c| c.is_ascii_digit())
        && chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok()
}

/// 公式版本 ga 数字形，公式增改走版本管理即 ga-1 与后继。
fn is_formula_version(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("ga-") else {
        return false;
    };
    !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit())
}

/// 读数守卫：机械校验七字段形态，通过即返回规范化读数对象。
/// 校形不判值，value 数值零到一或 insufficient 标记两可。
pub fn guard_reading(text: &str) -> Result<Value, ReadingError> {
    let parsed: Value =
        serde_json::from_str(text).map_err(|e| ReadingError::Unparsable(e.to_string()))?;
    let obj = parsed
        .as_object()
        .ok_or_else(|| ReadingError::Unparsable("读数非 JSON 对象".into()))?;

    let mut missing = Vec::new();
    for field in READING_FIELDS {
        if !obj.contains_key(field) {
            missing.push(field.to_string());
        }
    }
    if !missing.is_empty() {
        return Err(ReadingError::MissingField(missing.join(",")));
    }

    let mut extra = Vec::new();
    for key in obj.keys() {
        if !READING_FIELDS.contains(&key.as_str()) && !READING_EXT_FIELDS.contains(&key.as_str()) {
            extra.push(key.clone());
        }
    }
    if !extra.is_empty() {
        return Err(ReadingError::ExtraField(extra.join(",")));
    }

    let dim = obj["dimension"].as_str().unwrap_or("");
    if !READING_DIMENSIONS.contains(&dim) {
        return Err(ReadingError::BadField {
            field: "dimension".into(),
            reason: format!("非三枚举 {dim}"),
        });
    }

    let subject = obj["subject"].as_str().unwrap_or("");
    if subject.is_empty() {
        return Err(ReadingError::BadField {
            field: "subject".into(),
            reason: "主体标识空".into(),
        });
    }

    match &obj["value"] {
        Value::Number(n) => {
            let v = n
                .as_f64()
                .ok_or_else(|| ReadingError::BadField {
                    field: "value".into(),
                    reason: "数值不可解析".into(),
                })?;
            if !(0.0..=1.0).contains(&v) {
                return Err(ReadingError::BadField {
                    field: "value".into(),
                    reason: format!("越零一到界 {v}"),
                });
            }
        }
        Value::String(s) if s == "insufficient" => {}
        _ => {
            return Err(ReadingError::BadField {
                field: "value".into(),
                reason: "非零到一数值亦非 insufficient 标记".into(),
            })
        }
    }

    // ga-2 扩展字段形态校验：confidence_band 为 {lower, upper} 零到一且 lower<=upper，
    // posterior_mean 为零到一数值；扩展字段仅 adoption 维出，他维出现即拒。
    if let Some(cb) = obj.get("confidence_band") {
        if dim != "adoption" {
            return Err(ReadingError::BadField {
                field: "confidence_band".into(),
                reason: format!("非 adoption 维出现 {dim}"),
            });
        }
        let lo = cb.get("lower").and_then(Value::as_f64);
        let up = cb.get("upper").and_then(Value::as_f64);
        let (Some(lo), Some(up)) = (lo, up) else {
            return Err(ReadingError::BadField {
                field: "confidence_band".into(),
                reason: "lower 与 upper 缺一或非数值".into(),
            });
        };
        if !(0.0..=1.0).contains(&lo) || !(0.0..=1.0).contains(&up) || lo > up {
            return Err(ReadingError::BadField {
                field: "confidence_band".into(),
                reason: format!("越零一到界或 lower 大于 upper {lo} {up}"),
            });
        }
    }
    if let Some(pm) = obj.get("posterior_mean") {
        if dim != "adoption" {
            return Err(ReadingError::BadField {
                field: "posterior_mean".into(),
                reason: format!("非 adoption 维出现 {dim}"),
            });
        }
        let v = pm
            .as_f64()
            .ok_or_else(|| ReadingError::BadField {
                field: "posterior_mean".into(),
                reason: "非数值".into(),
            })?;
        if !(0.0..=1.0).contains(&v) {
            return Err(ReadingError::BadField {
                field: "posterior_mean".into(),
                reason: format!("越零一到界 {v}"),
            });
        }
    }

    let str_ok = |field: &str, ok: bool, reason: &str| -> Result<(), ReadingError> {
        if ok {
            Ok(())
        } else {
            Err(ReadingError::BadField {
                field: field.to_string(),
                reason: reason.to_string(),
            })
        }
    };
    str_ok(
        "window",
        obj["window"].as_str().is_some_and(|s| !s.is_empty()),
        "窗口描记空",
    )?;
    str_ok(
        "formula_version",
        obj["formula_version"]
            .as_str()
            .is_some_and(|s| is_formula_version(s)),
        "非 ga 数字形",
    )?;
    str_ok(
        "computed_at",
        obj["computed_at"].as_str().is_some_and(|s| is_iso_date(s)),
        "非 YYYY-MM-DD 形",
    )?;
    str_ok(
        "inputs_digest",
        obj["inputs_digest"]
            .as_str()
            .is_some_and(|s| is_sha256_hex(s)),
        "非六十四位十六进制",
    )?;

    Ok(parsed)
}

/// 读数事件构建：守卫通过后拼 EventInput，doc_id 即读数序列标识
/// gauge-reading-维度-主体，事件分类 consumable 即读数历史可被 read 回看。
pub fn reading_event(text: &str, actor: Actor) -> Result<EventInput, ReadingError> {
    let reading = guard_reading(text)?;
    let doc_id = format!(
        "gauge-reading-{}-{}",
        reading["dimension"].as_str().unwrap_or(""),
        reading["subject"].as_str().unwrap_or("")
    );
    Ok(EventInput {
        event_id: uuid::Uuid::new_v4().to_string(),
        event_type: "reading_recorded".to_string(),
        timestamp: chrono::Utc::now(),
        actor,
        details: Some(reading),
        doc_id,
        prev_hash: None,
        event_class: Some("consumable".to_string()),
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
            actor_id: "gauge".to_string(),
            actor_type: ActorType::System,
            invoked_via: "cli".to_string(),
        }
    }

    fn good() -> String {
        r#"{"dimension":"convergence","subject":"sih-engine","value":0.5,"window":"2026-08-01/2026-08-30","formula_version":"ga-1","computed_at":"2026-08-30","inputs_digest":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#.into()
    }

    #[test]
    fn r1_guard_accepts_well_formed() {
        let g = guard_reading(&good());
        assert!(g.is_ok(), "{g:?}");
    }

    #[test]
    fn r2_guard_rejects_missing_and_extra() {
        let v: Value = serde_json::from_str(&good()).unwrap();
        let mut less = v.clone();
        less.as_object_mut().unwrap().remove("window");
        assert!(matches!(
            guard_reading(&less.to_string()),
            Err(ReadingError::MissingField(_))
        ));
        let mut more = v;
        more.as_object_mut()
            .unwrap()
            .insert("suggestion".into(), Value::String("加码".into()));
        assert!(matches!(
            guard_reading(&more.to_string()),
            Err(ReadingError::ExtraField(e)) if e.contains("suggestion")
        ));
    }

    #[test]
    fn r3_guard_rejects_enum_value_digest_shapes() {
        let bad_dim = good().replace("convergence", "mood");
        assert!(matches!(
            guard_reading(&bad_dim),
            Err(ReadingError::BadField { field, .. }) if field == "dimension"
        ));
        let bad_val = good().replace("0.5", "1.5");
        assert!(matches!(
            guard_reading(&bad_val),
            Err(ReadingError::BadField { field, .. }) if field == "value"
        ));
        let bad_hex = good().replace(&"a".repeat(64), &"z".repeat(64));
        assert!(matches!(
            guard_reading(&bad_hex),
            Err(ReadingError::BadField { field, .. }) if field == "inputs_digest"
        ));
        let bad_formula = good().replace("ga-1", "v9");
        assert!(matches!(
            guard_reading(&bad_formula),
            Err(ReadingError::BadField { field, .. }) if field == "formula_version"
        ));
        let bad_date = good().replace("2026-08-30", "2026-13-99");
        assert!(matches!(
            guard_reading(&bad_date),
            Err(ReadingError::BadField { field, .. }) if field == "computed_at"
        ));
    }

    #[test]
    fn r4_insufficient_marker_and_bounds_ok() {
        let ins = good().replace("0.5", "\"insufficient\"");
        assert!(guard_reading(&ins).is_ok());
        assert!(guard_reading(&good().replace("0.5", "0.0")).is_ok());
        assert!(guard_reading(&good().replace("0.5", "1.0")).is_ok());
    }

    fn good_ga2() -> String {
        r#"{"dimension":"adoption","subject":"agents","value":0.5,"window":"2026-08-01/2026-08-30","formula_version":"ga-2","computed_at":"2026-08-30","inputs_digest":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","confidence_band":{"lower":0.0,"upper":1.0},"posterior_mean":0.5}"#.into()
    }

    #[test]
    fn r6_guard_accepts_ga2_ext_fields() {
        assert!(guard_reading(&good_ga2()).is_ok());
    }

    #[test]
    fn r7_guard_rejects_malformed_ga2_ext() {
        // confidence_band 越界
        let bad_lo = good_ga2().replace("\"lower\":0.0", "\"lower\":-0.1");
        assert!(matches!(
            guard_reading(&bad_lo),
            Err(ReadingError::BadField { field, .. }) if field == "confidence_band"
        ));
        // confidence_band lower 大于 upper
        let bad_ord = good_ga2().replace("\"lower\":0.0,\"upper\":1.0", "\"lower\":0.9,\"upper\":0.1");
        assert!(matches!(
            guard_reading(&bad_ord),
            Err(ReadingError::BadField { field, .. }) if field == "confidence_band"
        ));
        // posterior_mean 越界
        let bad_pm = good_ga2().replace("\"posterior_mean\":0.5", "\"posterior_mean\":1.5");
        assert!(matches!(
            guard_reading(&bad_pm),
            Err(ReadingError::BadField { field, .. }) if field == "posterior_mean"
        ));
        // 扩展字段出现在非 adoption 维
        let bad_dim = good_ga2().replace("\"dimension\":\"adoption\"", "\"dimension\":\"convergence\"");
        assert!(matches!(
            guard_reading(&bad_dim),
            Err(ReadingError::BadField { field, .. }) if field == "confidence_band"
        ));
        // 未知扩展字段仍拒
        let bad_extra = good_ga2().replace("\"posterior_mean\":0.5", "\"posterior_mean\":0.5,\"suggestion\":\"加码\"");
        assert!(matches!(
            guard_reading(&bad_extra),
            Err(ReadingError::ExtraField(e)) if e.contains("suggestion")
        ));
    }

    #[test]
    fn r8_ga1_reading_still_accepted() {
        // ga-1 七字段读数在 ga-2 守卫下回放兼容
        assert!(guard_reading(&good()).is_ok());
    }

    #[test]
    fn r5_reading_event_appends_and_chain_verifies() {
        let dir = std::env::temp_dir().join(format!("gauge-reading-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let trail = dir.join("trail.ndjson");
        let _ = std::fs::remove_file(&trail);
        let mut store = Vec::new();
        let input = reading_event(&good(), actor()).unwrap();
        assert_eq!(input.event_type, "reading_recorded");
        assert_eq!(input.doc_id, "gauge-reading-convergence-sih-engine");
        append(input, &mut store, Some(&trail)).unwrap();
        let events = crate::event_stream::load_events(&trail).unwrap();
        assert_eq!(events[0].event_type, "reading_recorded");
        assert!(verify(&events, VerifyRange::Full).is_ok());
        let _ = std::fs::remove_file(&trail);
    }
}
