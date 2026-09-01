//! 报告结构与渲染：与工具件 Python 输出逐字段对表（除 engine.version 外）。
//!
//! 承 SPEC-013 § 验收判据 A1 + § 验收判据 #acceptance 第三节 A1。

use serde::Serialize;
use serde_json::{Map, Number, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct EngineHeader {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackHeader {
    pub name: String,
    pub version: String,
    pub domain: DomainOutput,
}

#[derive(Debug, Clone, Serialize)]
pub struct DomainOutput {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

/// finding 嵌套形：与工具件 Python 嵌套形逐字段对表。
///
/// 工具件 finding 形：`{"pack", "rule_id", "path", "location": {"line"}|{"path"}, "message"}`
/// 键序固定，序列化顺序与工具件一致。
#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub pack: String,
    pub rule_id: String,
    pub path: String,
    pub location: Location,
    pub message: String,
}

/// finding 位置：text 材料用 line，json 材料用 path，二选一嵌套。
/// 工具件两种形态：
///   - text: `{"line": <行号>}` —— 行号从 1 起
///   - json: `{"path": "<点分路径>"}` —— 解析路径串
///
/// serde untagged 序列化：单一键输出，键名依变体而异。
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Location {
    Line { line: usize },
    Path { path: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub total: usize,
    /// 保留发现顺序（与工具件 Python dict 插入序对表）
    pub by_rule: Map<String, Value>,
    pub by_pack: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EngineReport {
    pub engine: EngineHeader,
    pub packs: Vec<PackHeader>,
    pub targets: Vec<String>,
    /// 保留 targets 顺序（与工具件 Python dict 插入序对表）
    pub content_hashes: Map<String, Value>,
    pub findings: Vec<Finding>,
    pub domain_mismatches: Vec<String>,
    pub summary: Summary,
}

pub fn compute_sha256(text: &str) -> String {
    let mut h = Sha256::new();
    h.update(text.as_bytes());
    hex::encode(h.finalize())
}

pub fn compute_sha256_file(path: &Path) -> std::io::Result<String> {
    let text = std::fs::read_to_string(path)?;
    Ok(compute_sha256(&text))
}

pub fn build_summary(findings: &[Finding]) -> Summary {
    let mut by_rule: Map<String, Value> = Map::new();
    let mut by_pack: Map<String, Value> = Map::new();
    for f in findings {
        let n = by_rule
            .get(f.rule_id.as_str())
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            + 1;
        by_rule.insert(f.rule_id.clone(), Value::Number(Number::from(n)));
        let n = by_pack
            .get(f.pack.as_str())
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            + 1;
        by_pack.insert(f.pack.clone(), Value::Number(Number::from(n)));
    }
    Summary {
        total: findings.len(),
        by_rule,
        by_pack,
    }
}
