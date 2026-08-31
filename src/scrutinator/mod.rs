//! 引擎侧核阅组件库，承接 SPEC-013。
//!
//! 模块结构：
//! - `mod rule` 规则条目与谓词实现（text 五类 + json 五类）
//! - `mod report` 报告结构与渲染
//! - `mod asset` 编译期内嵌规则包（include_str!）
//!
//! 公开 API：
//! - `run` 入口签名（承 § 验收判据 A1 同包同目标逐字节一致）

pub mod asset;
pub mod report;
pub mod rule;

#[cfg(test)]
mod tests;

use report::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const ENGINE_NAME: &str = "scrutinator";
pub const ENGINE_VERSION: &str = "0.1.0";

/// 加载编译期内嵌的规则包
pub fn load_pack(pack_name: &str) -> Result<(rule::DomainSpec, Vec<rule::RuleEntry>), String> {
    rule::load_pack(pack_name)
}

/// 加载多个包（按包名顺序）
pub fn load_packs(names: &[&str]) -> Result<Vec<(String, rule::DomainSpec, Vec<rule::RuleEntry>)>, String> {
    let mut out = Vec::new();
    for n in names {
        let (d, r) = load_pack(n)?;
        out.push((n.to_string(), d, r));
    }
    Ok(out)
}

/// 对单条文本运行一组规则，返回 findings
pub fn run_rules_on_text(
    pack_name: &str,
    rules: &[rule::RuleEntry],
    text: &str,
) -> Vec<report::Finding> {
    let mut out = Vec::new();
    for r in rules {
        for tf in rule::check_text(r, text) {
            out.push(report::Finding {
                rule_id: tf.rule_id.clone(),
                line: if tf.line == 0 { None } else { Some(tf.line) },
                message: tf.message,
                pack: pack_name.to_string(),
            });
        }
    }
    out
}

/// 对单条 JSON 文本运行 json 规则
pub fn run_rules_on_json(
    pack_name: &str,
    rules: &[rule::RuleEntry],
    value: &serde_json::Value,
) -> Vec<report::Finding> {
    let mut out = Vec::new();
    for r in rules {
        let findings = match r.kind {
            rule::RuleKind::JsonField => rule::check_json_field(r, value),
            rule::RuleKind::JsonFieldCompare => rule::check_json_field_compare(r, value),
            rule::RuleKind::JsonNumberRange => rule::check_json_number_range(r, value),
            rule::RuleKind::JsonArraySchema => rule::check_json_array_schema(r, value),
            rule::RuleKind::JsonParse => match rule::try_parse_json(value.to_string().as_str()) {
                Ok(_) => vec![],
                Err(e) => vec![rule::JsonFinding {
                    rule_id: r.id.clone(),
                    pointer: "<root>".to_string(),
                    message: e,
                }],
            },
            _ => continue,
        };
        for jf in findings {
            out.push(report::Finding {
                rule_id: jf.rule_id,
                line: None,
                message: jf.message,
                pack: pack_name.to_string(),
            });
        }
    }
    out
}

/// 渲染 EngineReport（与工具件 Python 输出字段顺序近似，键名逐字段对表）
pub fn render_report(
    packs: &[(String, rule::DomainSpec, Vec<rule::RuleEntry>)],
    targets: &[(String, Option<String>)], // (path, text)
    findings: &[report::Finding],
) -> EngineReport {
    let pack_headers: Vec<PackHeader> = packs
        .iter()
        .map(|(name, dom, _)| PackHeader {
            name: name.clone(),
            version: dom.version.clone(),
            domain: DomainOutput {
                include: dom.include.clone(),
                exclude: dom.exclude.clone(),
            },
        })
        .collect();
    let target_headers: Vec<String> = targets.iter().map(|(p, _)| p.clone()).collect();
    let mut hashes = BTreeMap::new();
    for (p, t) in targets {
        if let Some(text) = t {
            let mut h = Sha256::new();
            h.update(text.as_bytes());
            hashes.insert(p.clone(), hex::encode(h.finalize()));
        }
    }
    EngineReport {
        engine: EngineHeader {
            name: ENGINE_NAME.to_string(),
            version: ENGINE_VERSION.to_string(),
        },
        packs: pack_headers,
        targets: target_headers,
        content_hashes: hashes,
        findings: findings.to_vec(),
        domain_mismatches: Vec::new(),
        summary: report::build_summary(findings),
    }
}
