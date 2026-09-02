//! model_utils——模型家族与认知框架提取（融回自围堰 facet/src/model_utils.py，
//! csnx 压缩前置工具，compiler 依赖件）。
//!
//! actor_id 形：`group+model_id[+suffix]`。

use std::sync::OnceLock;

/// 模型家族匹配模式（按优先级排序）。
fn family_patterns() -> &'static [( &'static str, regex::Regex)] {
    static PATTERNS: OnceLock<Vec<(&'static str, regex::Regex)>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        vec![
            ("DeepSeek", regex::Regex::new(r"^[Dd]eep\s*[Ss]eek").unwrap()),
            ("MiniMax", regex::Regex::new(r"(?i)^MiniMax").unwrap()),
            ("Doubao", regex::Regex::new(r"(?i)^Doubao").unwrap()),
            ("Qwen", regex::Regex::new(r"(?i)^Qwen").unwrap()),
            ("GLM", regex::Regex::new(r"(?i)^GLM").unwrap()),
            ("Kimi", regex::Regex::new(r"(?i)^Kimi").unwrap()),
        ]
    })
}

/// 非默认认知框架（从实验 suffix 提取）。
const NON_DEFAULT_FRAMEWORKS: &[&str] = &[
    "sensitivity",
    "metacognition",
    "find_critical",
    "minor_only",
    "redteam",
    "redteam_attack",
    "quantification",
    "assume_wrong",
    "decompose",
    "baseline_review",
];

/// 从 actor_id 提取模型家族名。
pub fn extract_family(actor_id: &str) -> String {
    let parts: Vec<&str> = actor_id.split('+').collect();
    let model_id = if parts.len() >= 2 { parts[1] } else { actor_id };
    for (family, pattern) in family_patterns() {
        if pattern.is_match(model_id) {
            return family.to_string();
        }
    }
    "Unknown".to_string()
}

/// 从 actor_id 提取认知框架：`default` 或具体框架名。
pub fn extract_framework(actor_id: &str) -> String {
    let parts: Vec<&str> = actor_id.split('+').collect();
    let suffix = if parts.len() >= 3 { parts[2] } else { "" };
    if NON_DEFAULT_FRAMEWORKS.contains(&suffix) {
        return suffix.to_string();
    }
    "default".to_string()
}

/// 判断该 actor 是否使用了非默认认知框架。
pub fn is_non_default_framework(actor_id: &str) -> bool {
    extract_framework(actor_id) != "default"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doctest_anchors() {
        assert_eq!(extract_family("AISTORE+DeepSeek-V4-Flash-A"), "DeepSeek");
        assert_eq!(extract_family("AISTORE+Qwen3.5-397B-A17B-Pro"), "Qwen");
        assert_eq!(extract_family("MINIMAX+MiniMax-M3"), "MiniMax");
        assert_eq!(extract_family("AISTORE+DeepSeek-V4-Flash-A+sensitivity"), "DeepSeek");
        assert_eq!(extract_framework("AISTORE+DeepSeek-V4-Flash-A"), "default");
        assert_eq!(extract_framework("AISTORE+DeepSeek-V4-Flash-A+sensitivity"), "sensitivity");
        assert_eq!(extract_framework("AISTORE+DeepSeek-V4-Flash-A+a"), "default");
        assert!(is_non_default_framework("AISTORE+DeepSeek-V4-Flash-A+sensitivity"));
        assert!(!is_non_default_framework("AISTORE+DeepSeek-V4-Flash-A"));
    }
}
