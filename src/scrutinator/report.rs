//! 报告结构与渲染：与工具件 Python 输出逐字段对表（除 engine.version 外）。
//!
//! 承 SPEC-013 § 验收判据 A1 + § 验收判据 #acceptance 第三节 A1。

use serde::Serialize;
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

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub line: Option<usize>,
    pub message: String,
    pub pack: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub total: usize,
    pub by_rule: BTreeMap<String, usize>,
    pub by_pack: BTreeMap<String, usize>,
}

use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct EngineReport {
    pub engine: EngineHeader,
    pub packs: Vec<PackHeader>,
    pub targets: Vec<String>,
    pub content_hashes: BTreeMap<String, String>,
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
    let mut by_rule: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_pack: BTreeMap<String, usize> = BTreeMap::new();
    for f in findings {
        *by_rule.entry(f.rule_id.clone()).or_insert(0) += 1;
        *by_pack.entry(f.pack.clone()).or_insert(0) += 1;
    }
    Summary {
        total: findings.len(),
        by_rule,
        by_pack,
    }
}
