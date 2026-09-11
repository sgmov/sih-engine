#![allow(dead_code)]

//! sihmcp 单元基线（sihmcp-solo 段1）：日期形、布局判别、链路径、错误载荷
//! 四字段、注册面九具。

use super::runtime::{detect_layout_form, error_payload, trail_path, valid_date};
use super::server::{schemas_nonempty, tool_names};

#[test]
fn valid_date_shape() {
    assert!(valid_date("2026-09-11"));
    assert!(!valid_date("2026-9-1"));
    assert!(!valid_date(""));
    assert!(!valid_date("2026/09/11"));
    assert!(!valid_date("20260911"));
}

#[test]
fn layout_forms() {
    // 布局判别是纯逻辑：临时目录造标记验形（first_domain 双仓标记俱在；
    // canonical 域内 sih/ledger 面在场）
    let base = std::env::temp_dir().join(format!("sihmcp-layout-{}", std::process::id()));
    std::fs::create_dir_all(base.join("sih-engine")).unwrap();
    std::fs::create_dir_all(base.join("sih-tools")).unwrap();
    std::fs::write(base.join("sih-engine/Cargo.toml"), "[package]").unwrap();
    std::fs::write(base.join("sih-tools/pyproject.toml"), "").unwrap();
    assert_eq!(detect_layout_form(&base), Some("first_domain"));
    assert_eq!(
        trail_path(&base, "2026-09-11"),
        base.join("sih-engine/sih/event/trail/2026-09-11.ndjson")
    );
    let base2 = std::env::temp_dir().join(format!("sihmcp-layout-c-{}", std::process::id()));
    std::fs::create_dir_all(base2.join("sih/ledger")).unwrap();
    assert_eq!(detect_layout_form(&base2), Some("canonical"));
    assert_eq!(
        trail_path(&base2, "2026-09-11"),
        base2.join("sih/event/trail/2026-09-11.ndjson")
    );
    let _ = std::fs::remove_dir_all(&base);
    let _ = std::fs::remove_dir_all(&base2);
}

#[test]
fn error_payload_four_fields() {
    let v = error_payload("t", "做什么", &["a: 参"], "坏了");
    assert_eq!(v["error"], "坏了");
    assert_eq!(v["what_this_tool_does"], "做什么");
    assert_eq!(v["valid_params"], serde_json::json!(["a: 参"]));
    assert_eq!(
        v["canonical_pointers"],
        serde_json::json!([
            "sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md",
            "sih-engine/sih/state/plan/mcpline-line-v1.md",
        ])
    );
}

#[test]
fn registry_nine_tools_with_schemas() {
    assert_eq!(tool_names().len(), 9);
    assert!(schemas_nonempty());
}

#[tokio::test]
async fn naming_guide_static_shape() {
    let v = super::alpha::naming_guide().await;
    let sections = v["sections"].as_object().unwrap();
    assert_eq!(sections.len(), 5);
    assert_eq!(sections["naming_five_step_form"].as_array().unwrap().len(), 5);
    assert!(v["disclaimer"].as_str().unwrap().contains("零裁决"));
}

// 红线守卫（sihmcp-solo 段1）：MCP 只读面源码 grep 写动词零命中
// （对等 mcpline tests/test_zero_write.py 禁词表，Rust 源形适配）。

use std::path::PathBuf;

const FORBIDDEN_PATTERNS: &[(&str, &str)] = &[
    (r"\bappend\b", "scribe append / 文件追加动词"),
    (r"\bwrite_text\b|\bwrite_bytes\b", "写盘动词"),
    (r"\bfs::write\b|\bFile::create\b", "std 写盘动词"),
    (r"\bcreate_dir\b|\bcreate_dir_all\b", "建目录动词"),
    (r"\bremove_file\b|\bremove_dir_all\b|\bfs::rename\b", "删改文件动词"),
    (r"\bOpenOptions\b", "写模式文件打开"),
    (r"\bshutil\b", "shutil 文件操作"),
];

const SOURCE_FILES: &[&str] = &[
    "src/mcpserver/runtime.rs",
    "src/mcpserver/alpha.rs",
    "src/mcpserver/server.rs",
    "src/bin/sihmcp.rs",
];

fn source_of(rel: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    std::fs::read_to_string(path).unwrap_or_default()
}

#[test]
fn zero_write_guard_alpha_readonly_source() {
    let mut hits = vec![];
    for rel in SOURCE_FILES {
        let src = source_of(rel);
        assert!(!src.is_empty(), "{rel} 不可读");
        for (pattern, reason) in FORBIDDEN_PATTERNS {
            let re = regex::Regex::new(pattern).unwrap();
            if re.is_match(&src) {
                hits.push(format!("{rel}: {reason} ({pattern})"));
            }
        }
    }
    assert!(hits.is_empty(), "写动词命中：\n{}", hits.join("\n"));
}

#[test]
fn zero_write_guard_no_write_subcommand_literals() {
    // scribe 写动词与 lease 写动词子命令字面量（独立引号形）禁入；
    // Python 禁词表独立引号形对等。
    let standalone = [
        "\"append\"", "\"intent\"", "\"park\"", "\"record\"", "\"direct\"", "\"claim\"",
        "\"lock\"", "\"unlock\"", "\"close\"", "\"commit\"",
    ];
    for rel in SOURCE_FILES {
        let src = source_of(rel);
        for lit in standalone {
            assert!(!src.contains(lit), "{rel} 命中写动词子命令字面量 {lit}");
        }
    }
}
