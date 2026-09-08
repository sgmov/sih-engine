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

/// 编译期内嵌的信封三件（SPEC-022 信封统一），按包名查。
/// bodies 存在性由编译期 include_str! 保证，运行时只校字段。
pub fn envelope(name: &str) -> &'static str {
    match name {
        "des-001" => include_str!("packs/des-001/envelope.json"),
        "des-001-mathe" => include_str!("packs/des-001-mathe/envelope.json"),
        "ask3" => include_str!("packs/ask3/envelope.json"),
        _ => "",
    }
}

/// 信封硬切换校验（SPEC-022）：包名无信封或字段非法即拒。
pub fn check_envelope(name: &str) -> Result<(), String> {
    let text = envelope(name);
    if text.is_empty() {
        return Err(format!("envelope missing: {name}"));
    }
    let v: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| format!("envelope invalid: {name}: {e}"))?;
    if v.get("envelope_version").and_then(|x| x.as_i64()) != Some(1) {
        return Err(format!("envelope invalid: {name}: envelope_version must be 1"));
    }
    let id_ok = v.get("id").and_then(|x| x.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
    if !id_ok {
        return Err(format!("envelope invalid: {name}: id must be non-empty string"));
    }
    let family_ok = v.get("family").and_then(|x| x.as_str())
        .map(|s| matches!(s, "scrutinator" | "attractor" | "formatter" | "nomenclator"))
        .unwrap_or(false);
    if !family_ok {
        return Err(format!("envelope invalid: {name}: family illegal"));
    }
    let bt_ok = v.get("body_type").and_then(|x| x.as_str())
        .map(|s| matches!(s, "config" | "doc_spec"))
        .unwrap_or(false);
    if !bt_ok {
        return Err(format!("envelope invalid: {name}: body_type illegal"));
    }
    match v.get("bodies").and_then(|x| x.as_array()) {
        Some(a) if !a.is_empty() && a.iter().all(|x| x.is_string()) => Ok(()),
        _ => Err(format!("envelope invalid: {name}: bodies must be non-empty string list")),
    }
}
