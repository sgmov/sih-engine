//! 编译期内嵌规则包：承 DEC-013 融回门机制，规则包随件迁引擎侧
//! 编译时 include_str! 嵌入 manifest.toml + rules.toml，
//! 运行时不读不写规则包文件，承工程基线第一条确定性程序。
//!
//! 详见 SPEC-013 § 规则包装载方式。

/// 编译期内嵌的规则包三件，按包名查
pub fn manifest(name: &str) -> &'static str {
    match name {
        "des-001" => include_str!("packs/des-001/manifest.toml"),
        "des-001-mathe" => include_str!("packs/des-001-mathe/manifest.toml"),
        "ask3" => include_str!("packs/ask3/manifest.toml"),
        _ => "",
    }
}

/// 编译期内嵌的规则包三件 rules.toml，按包名查
pub fn rules(name: &str) -> &'static str {
    match name {
        "des-001" => include_str!("packs/des-001/rules.toml"),
        "des-001-mathe" => include_str!("packs/des-001-mathe/rules.toml"),
        "ask3" => include_str!("packs/ask3/rules.toml"),
        _ => "",
    }
}

/// 三包名清单（按规格三包即 des-001 + des-001-mathe + ask3）
pub const PACK_NAMES: &[&str] = &["des-001", "des-001-mathe", "ask3"];
