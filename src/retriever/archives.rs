//! 五档路径集分类与事件档属判定，承接 SPEC-008 五档映射落差节。
//!
//! 分类规则单源在本模块，窄域包仅预筛，一致性对表由测试 F-10 承载。
//! wengumcp-parallel 批增布局两形：first_domain 即中央双仓形（现表逐字节零变），
//! canonical 即新城正典 sih/ 形（城档映射表随批钉死，见 classify_path_canonical）。

use super::Archive;

/// 布局两形：first_domain 即中央双仓形，canonical 即新城正典域 sih/ 形。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    FirstDomain,
    Canonical,
}

/// 按工作区根相对路径机械判定档属，未入五档路径集返回 None。
pub fn classify_path(rel: &str) -> Option<Archive> {
    if rel.starts_with("ai-ex/") && rel.ends_with(".md") {
        return Some(Archive::Experience);
    }
    if rel.starts_with("sih-engine/sih/event/plan/") && rel.ends_with("-results.md") {
        return Some(Archive::Conclusion);
    }
    if rel == "sih-engine/doc/governance/PARKING-v1.md" || rel == "sih-tools/PARKING-v1.md" {
        return Some(Archive::Parked);
    }
    if rel.starts_with("sih-tools/parking/records/") && rel.ends_with(".json") {
        return Some(Archive::Parked);
    }
    if rel.starts_with("sih-tools/scribe/reports/") && rel.ends_with(".json") {
        let name = rel.rsplit('/').next().unwrap_or("");
        if name.contains("ask3") && name.contains("record") {
            return Some(Archive::Intent);
        }
        return None;
    }
    // retrieverline 批（rl-02）档案面扩容两域：检索可见面非引用权威面，SETSP 名录与
    // 旧仓 doc 只作盘点源不作引用源（工程基线与资产回锚登记先例），来源域标注
    // 经 source_domain_of 随件出行零伪装正典；档属归经验档承 ai-ex 外部经验先例。
    if rel.starts_with("sihankor/doc/") && rel.ends_with(".md") {
        return Some(Archive::Experience);
    }
    if rel.starts_with(".tmp/SihEngineeringTechnologySelectionPrecedent/") && rel.ends_with(".md")
    {
        return Some(Archive::Experience);
    }
    None
}

/// retrieverline 批（rl-02）：扩容两域来源域标注，未入两域返回 None（既有域条目
/// 零标注，输出行零新键）。检索可见面非引用权威面：标注是出处显形，非正典地位授予。
pub fn source_domain_of(rel: &str) -> Option<&'static str> {
    if rel.starts_with("sihankor/doc/") {
        return Some("legacy-sihankor");
    }
    if rel.starts_with(".tmp/SihEngineeringTechnologySelectionPrecedent/") {
        return Some("setsp");
    }
    None
}

/// 分形判档：按布局两形取各自映射表（wengumcp-parallel 批件二）。
pub fn classify_path_in(layout: Layout, rel: &str) -> Option<Archive> {
    match layout {
        Layout::FirstDomain => classify_path(rel),
        Layout::Canonical => classify_path_canonical(rel),
    }
}

/// canonical 新城正典域城档映射表：任务包归意图档、结果档归结论档、泊界材料归
/// 悬置档；Experience 城无对应档案即零命中如实。事件档属 classify_event 现表
/// 两形共用零变。
fn classify_path_canonical(rel: &str) -> Option<Archive> {
    if rel.starts_with("sih/event/plan/") && rel.ends_with("-results.md") {
        return Some(Archive::Conclusion);
    }
    if rel.starts_with("sih/state/plan/") && rel.ends_with(".md") {
        return Some(Archive::Intent);
    }
    if rel.starts_with("sih/state/parking/materials/") && rel.ends_with(".json") {
        return Some(Archive::Parked);
    }
    if rel == "sih/state/parking/PARKING-v1.md" {
        return Some(Archive::Parked);
    }
    None
}


/// 按事件类型判定档属即停泊两型归悬置、意图型归意图、其余归事实。
pub fn classify_event(event_type: &str) -> Archive {
    match event_type {
        "parking_entered" | "parking_exited" => Archive::Parked,
        "intent_refined" => Archive::Intent,
        _ => Archive::Fact,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// F-9 档属归位即路径分类表与事件档属表。
    #[test]
    fn f9_classify_path_table() {
        assert_eq!(
            classify_path("sih-engine/sih/event/plan/mem-init-t6d-results.md"),
            Some(Archive::Conclusion)
        );
        assert_eq!(classify_path("ai-ex/INDEX.md"), Some(Archive::Experience));
        assert_eq!(
            classify_path("ai-ex/hermes-research/A-official-docs.md"),
            Some(Archive::Experience)
        );
        assert_eq!(
            classify_path("sih-engine/doc/governance/PARKING-v1.md"),
            Some(Archive::Parked)
        );
        assert_eq!(classify_path("sih-tools/PARKING-v1.md"), Some(Archive::Parked));
        assert_eq!(
            classify_path("sih-tools/parking/records/pk-024-enter.json"),
            Some(Archive::Parked)
        );
        assert_eq!(
            classify_path("sih-tools/scribe/reports/2026-08-27-ask3-memimpl-record.json"),
            Some(Archive::Intent)
        );
        assert_eq!(
            classify_path("sih-tools/scribe/reports/2026-08-27-ask3-memimpl-record-r2.json"),
            Some(Archive::Intent)
        );
        assert_eq!(classify_path("sih-engine/doc/spec/SPEC-007-project-memory-component.md"), None);
        assert_eq!(classify_path("sih-engine/sih/event/plan/mem-init-t6d.md"), None);
        assert_eq!(classify_path("sih-tools/scribe/reports/2026-08-27-memimpl-identity.json"), None);
    }

    /// retrieverline rl-02：扩容两域归经验档且来源域标注随件；既有域零标注零串线。
    #[test]
    fn retrline_extension_domains_classify_and_annotate() {
        assert_eq!(
            classify_path("sihankor/doc/decision/DEC-019-trust-decay.md"),
            Some(Archive::Experience)
        );
        assert_eq!(
            classify_path(
                ".tmp/SihEngineeringTechnologySelectionPrecedent/engineering-bridge/meta/05-meta-defenses.md"
            ),
            Some(Archive::Experience)
        );
        assert_eq!(
            source_domain_of("sihankor/doc/design/DES-016-trust-scoring-implementation.md"),
            Some("legacy-sihankor")
        );
        assert_eq!(
            source_domain_of(".tmp/SihEngineeringTechnologySelectionPrecedent/README.md"),
            Some("setsp")
        );
        // 两域非 md 载体不入档案面（yaml 关系边集在册未入索引），标注判定与载体验独立。
        assert_eq!(classify_path("sihankor/doc/other.txt"), None);
        assert_eq!(
            classify_path(".tmp/SihEngineeringTechnologySelectionPrecedent/decisions/rust.yaml"),
            None
        );
        // 既有域零标注：ai-ex 属本域经验档不落 legacy 标注。
        assert_eq!(source_domain_of("ai-ex/INDEX.md"), None);
        assert_eq!(source_domain_of("sih-engine/doc/governance/PARKING-v1.md"), None);
    }

    #[test]
    fn f9_classify_event_table() {
        assert_eq!(classify_event("parking_entered"), Archive::Parked);
        assert_eq!(classify_event("parking_exited"), Archive::Parked);
        assert_eq!(classify_event("intent_refined"), Archive::Intent);
        assert_eq!(classify_event("certification_completed"), Archive::Fact);
        assert_eq!(classify_event("session_open"), Archive::Fact);
    }
}
