//! 错误四类验收，承接 SPEC-005#error-categories。
//!
//! 形态即必填与类型，契约即锚点与深度双检与轮次，
//! 血统即引文逐字与出处与证据行号，计量即 calls 单调。零 LLM。

use std::path::Path;

use super::record::{DomainTag, OutputRecord};

/// 验收错误四类，退出码一的承载。
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Ask3Error {
    #[error("形态错误：{0}")]
    Shape(String),
    #[error("契约违例：{0}")]
    Contract(String),
    #[error("血统不可核验：{0}")]
    Lineage(String),
    #[error("调用计量异常：{0}")]
    Metering(String),
}

/// 全量验收，返回全部错误不短路，错误空即通过。
pub fn validate(record: &OutputRecord, root: &Path) -> Result<(), Vec<Ask3Error>> {
    let mut errors = Vec::new();
    errors.extend(shape_errors(record));
    errors.extend(contract_errors(record));
    errors.extend(lineage_errors(record, root));
    errors.extend(metering_errors(record));
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn shape_errors(record: &OutputRecord) -> Vec<Ask3Error> {
    let mut errors = Vec::new();
    if record.session_id.trim().is_empty() {
        errors.push(Ask3Error::Shape("session_id 为空".into()));
    }
    if record.raw_input.trim().is_empty() {
        errors.push(Ask3Error::Shape("raw_input 为空".into()));
    }
    if record.intent_contract.goal.trim().is_empty() {
        errors.push(Ask3Error::Shape("intent_contract.goal 为空".into()));
    }
    if record.intent_contract.output_format.trim().is_empty() {
        errors.push(Ask3Error::Shape("intent_contract.output_format 为空".into()));
    }
    errors
}

fn contract_errors(record: &OutputRecord) -> Vec<Ask3Error> {
    let mut errors = Vec::new();
    if record.anchors.is_empty() {
        errors.push(Ask3Error::Contract("锚点数组为空，一问至少析出一条显层命题".into()));
    }
    if record.round == 0 {
        errors.push(Ask3Error::Contract("round 从 1 起，0 非法".into()));
    }
    let declared = record.domain_contract.support_domain.max_depth
        > record.domain_contract.target_domain.max_depth;
    if declared {
        errors.push(Ask3Error::Contract(
            "深度双检第一检失败，支撑域声明上限超过目标域".into(),
        ));
    }
    let support_max = anchors_max_depth(record, DomainTag::Support);
    let target_max = anchors_max_depth(record, DomainTag::Target);
    if support_max > target_max {
        errors.push(Ask3Error::Contract(
            "深度双检第二检失败，支撑域锚点实际深度超过目标域".into(),
        ));
    }
    errors
}

fn anchors_max_depth(record: &OutputRecord, tag: DomainTag) -> u32 {
    record
        .anchors
        .iter()
        .filter(|anchor| anchor.domain_tag == tag)
        .map(|anchor| anchor.depth)
        .max()
        .unwrap_or(0)
}

fn lineage_errors(record: &OutputRecord, root: &Path) -> Vec<Ask3Error> {
    let mut errors = Vec::new();
    let raw_lines = record.raw_input.lines().count().max(1);
    for anchor in &record.anchors {
        let source = root.join(&anchor.philosophy_ref.source);
        if !source.is_file() {
            errors.push(Ask3Error::Lineage(format!(
                "锚点 {} 哲学引文出处不存在：{}",
                anchor.anchor_seq, anchor.philosophy_ref.source
            )));
            continue;
        }
        let text = match std::fs::read_to_string(&source) {
            Ok(text) => text,
            Err(err) => {
                errors.push(Ask3Error::Lineage(format!(
                    "锚点 {} 哲学引文出处不可读：{err}",
                    anchor.anchor_seq
                )));
                continue;
            }
        };
        if !text.contains(&anchor.philosophy_ref.quote) {
            errors.push(Ask3Error::Lineage(format!(
                "锚点 {} 哲学引文摘录与原文不一致",
                anchor.anchor_seq
            )));
        }
        if !evidence_in_range(&anchor.evidence, root, raw_lines) {
            errors.push(Ask3Error::Lineage(format!(
                "锚点 {} 证据行号不可定位：{}",
                anchor.anchor_seq, anchor.evidence
            )));
        }
    }
    errors
}

/// 证据行号两形态：raw_input:N 或 仓内路径:行号。
fn evidence_in_range(evidence: &str, root: &Path, raw_lines: usize) -> bool {
    let (path_part, line_part) = match evidence.rsplit_once(':') {
        Some(parts) => parts,
        None => return false,
    };
    let line: usize = match line_part.parse() {
        Ok(value) => value,
        Err(_) => return false,
    };
    if line == 0 {
        return false;
    }
    if path_part == "raw_input" {
        return line <= raw_lines;
    }
    let target = root.join(path_part);
    if !target.is_file() {
        return false;
    }
    match std::fs::read_to_string(&target) {
        Ok(text) => line <= text.lines().count().max(1),
        Err(_) => false,
    }
}

fn metering_errors(record: &OutputRecord) -> Vec<Ask3Error> {
    let mut errors = Vec::new();
    if record.calls_out < record.calls_in {
        errors.push(Ask3Error::Metering(format!(
            "calls_out {} 小于 calls_in {}",
            record.calls_out, record.calls_in
        )));
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ask3repeater::record::{
        Anchor, Domain, DomainContract, DomainTag, InquiryStage, IntentContract, OutputRecord,
        PhilosophyRef,
    };
    use std::path::PathBuf;

    fn fixture_root() -> PathBuf {
        let root = std::env::temp_dir().join("ask3-validate-test");
        let _ = std::fs::create_dir_all(root.join("philosophy"));
        std::fs::write(root.join("philography.md"), "应而不藏即回应而不隐藏。\n").unwrap();
        root
    }

    fn valid_record(_root: &Path) -> OutputRecord {
        OutputRecord {
            session_id: "sess-test".into(),
            raw_input: "三问要建".into(),
            round: 1,
            intent_contract: IntentContract {
                goal: "建三问组件".into(),
                exclusions: vec![],
                output_format: "cargo test 全绿".into(),
                injected_constraints: vec![],
            },
            domain_contract: DomainContract {
                target_domain: Domain {
                    scope: "三问批".into(),
                    max_depth: 1,
                },
                support_domain: Domain {
                    scope: "背景".into(),
                    max_depth: 1,
                },
            },
            anchors: vec![Anchor {
                anchor_seq: 0,
                text: "用户开工令三问要建".into(),
                inquiry_stage: InquiryStage::First,
                domain_tag: DomainTag::Target,
                depth: 0,
                philosophy_ref: PhilosophyRef {
                    source: "philography.md".into(),
                    quote: "应而不藏即回应而不隐藏".into(),
                },
                rationale: "开工令即批次起点".into(),
                evidence: "raw_input:1".into(),
                confidence: 0.9,
            }],
            calls_in: 1,
            calls_out: 1,
        }
    }

    #[test]
    fn test_valid_record_passes() {
        let root = fixture_root();
        assert!(validate(&valid_record(&root), &root).is_ok());
    }

    #[test]
    fn test_shape_error_on_empty_goal() {
        let root = fixture_root();
        let mut record = valid_record(&root);
        record.intent_contract.goal = " ".into();
        let errors = validate(&record, &root).unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, Ask3Error::Shape(_))));
    }

    #[test]
    fn test_contract_error_empty_anchors() {
        let root = fixture_root();
        let mut record = valid_record(&root);
        record.anchors.clear();
        let errors = validate(&record, &root).unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, Ask3Error::Contract(_))));
    }

    #[test]
    fn test_contract_error_depth_double_check() {
        let root = fixture_root();
        let mut record = valid_record(&root);
        record.domain_contract.support_domain.max_depth = 2;
        let errors = validate(&record, &root).unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, Ask3Error::Contract(_))));
    }

    #[test]
    fn test_lineage_error_quote_mismatch() {
        let root = fixture_root();
        let mut record = valid_record(&root);
        record.anchors[0].philosophy_ref.quote = "原文里没有这句话".into();
        let errors = validate(&record, &root).unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, Ask3Error::Lineage(_))));
    }

    #[test]
    fn test_lineage_error_source_missing() {
        let root = fixture_root();
        let mut record = valid_record(&root);
        record.anchors[0].philosophy_ref.source = "ghost.md".into();
        let errors = validate(&record, &root).unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, Ask3Error::Lineage(_))));
    }

    #[test]
    fn test_metering_error_reversed_calls() {
        let root = fixture_root();
        let mut record = valid_record(&root);
        record.calls_out = 0;
        record.calls_in = 1;
        let errors = validate(&record, &root).unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, Ask3Error::Metering(_))));
    }

    #[test]
    fn test_evidence_file_path_form() {
        let root = fixture_root();
        let mut record = valid_record(&root);
        record.anchors[0].evidence = "philography.md:1".into();
        assert!(validate(&record, &root).is_ok());
        record.anchors[0].evidence = "philography.md:99".into();
        let errors = validate(&record, &root).unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, Ask3Error::Lineage(_))));
    }
}
