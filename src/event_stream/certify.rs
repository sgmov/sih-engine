//! 报告消费入口，承接 SPEC-006#append-entry。
//!
//! 消费工具报告 JSON 加显式退出码，产 certification_completed 事件输入。
//! 负载八项即报告路径、报告哈希、规约包版本清单、目标内容哈希清单、
//! 发现计数、退出码、工具版本、golden 基线。重复消费不拒可审计。

use crate::event_stream::event::{Actor, EventInput};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Debug)]
pub enum CertifyError {
    ReportNotJson,
    ReportNotObject,
}

pub(crate) fn sha256_hex(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hex::encode(hasher.finalize())
}

/// 消费工具报告产 certification_completed 事件输入，负载八项承 SPEC-006。
pub fn certification_event(
    report_path: &Path,
    report_text: &str,
    exit_code: i32,
    actor: Actor,
    timestamp: DateTime<Utc>,
) -> Result<EventInput, CertifyError> {
    let report: Value = serde_json::from_str(report_text).map_err(|_| CertifyError::ReportNotJson)?;
    let obj = report.as_object().ok_or(CertifyError::ReportNotObject)?;

    let pack_versions: Vec<String> = obj
        .get("packs")
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|p| {
                    let name = p.get("name")?.as_str()?;
                    let version = p.get("version")?.as_str()?;
                    Some(format!("{name}/{version}"))
                })
                .collect()
        })
        .unwrap_or_default();

    let finding_count = obj
        .get("findings")
        .and_then(|f| f.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    let tool_version = obj
        .get("tool")
        .and_then(|t| t.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let golden_baseline = obj
        .get("golden_baseline")
        .and_then(|g| g.as_str())
        .unwrap_or("")
        .to_string();

    let doc_id = report_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let details = json!({
        "report_path": report_path.to_string_lossy(),
        "report_hash": sha256_hex(report_text),
        "pack_versions": pack_versions,
        "content_hashes": obj.get("content_hashes").cloned().unwrap_or(json!({})),
        "finding_count": finding_count,
        "exit_code": exit_code,
        "tool_version": tool_version,
        "golden_baseline": golden_baseline,
    });

    let verification_result = json!({
        "exit_code": exit_code,
        "findings": finding_count,
        "total": finding_count,
    });

    Ok(EventInput {
        event_id: uuid::Uuid::new_v4().to_string(),
        event_type: "certification_completed".to_string(),
        timestamp,
        actor,
        details: Some(details),
        doc_id,
        prev_hash: None,
        event_class: Some("record_only".to_string()),
        verification_result: Some(verification_result),
    })
}
