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

// alpha 只读面写动词禁入扫描面：runtime 加 alpha 加 bin。server.rs 是注册面
// （段2 起 beta 工具名与参数名合法在列——Python 同款守卫也只扫 runtime.py 不
// 扫 server.py），注册面只扫直写盘原语。
const SOURCE_FILES: &[&str] = &[
    "src/mcpserver/runtime.rs",
    "src/mcpserver/alpha.rs",
    "src/bin/sihmcp.rs",
];

const REGISTRY_SOURCE_FILES: &[&str] = &["src/mcpserver/server.rs"];

const REGISTRY_FORBIDDEN_PATTERNS: &[(&str, &str)] = &[
    (r"\bfs::write\b|\bFile::create\b|\bOpenOptions\b", "写盘原语"),
    (r"\bcreate_dir\b|\bcreate_dir_all\b", "建目录动词"),
    (r"\bremove_file\b|\bremove_dir_all\b|\bfs::rename\b", "删改文件动词"),
];

// beta 写面直写盘禁入模式（对等 Python WRITEFACE_FORBIDDEN_PATTERNS）：写面
// 封闭即子进程调用仅既有通道；唯一落盘位是正身报告 tempfile 暂存（session.rs
// 单点白名单，注释声明同 Python 形）。writeface 源合法含 CLI 子命令字面量，
// 故 standalone 字面量守卫不扫 writeface 面。
const WRITEFACE_SOURCE_FILES: &[&str] = &[
    "src/mcpserver/matrix.rs",
    "src/mcpserver/errors.rs",
    "src/mcpserver/passthrough.rs",
    "src/mcpserver/tools.rs",
];

const WRITEFACE_FORBIDDEN_PATTERNS: &[(&str, &str)] = &[
    (r"\bwrite_text\b|\bwrite_bytes\b", "Path 写盘动词"),
    (r"\bfs::write\b|\bFile::create\b", "std 写盘动词"),
    (r"\bcreate_dir\b|\bcreate_dir_all\b", "建目录动词"),
    (r"\bremove_file\b|\bremove_dir_all\b|\bfs::rename\b", "删改文件动词"),
    (r"\bOpenOptions\b", "写模式文件打开"),
    (r"\bshutil\b", "shutil 文件操作"),
    (r"ledgerwrite|append_row", "台账写路径（写面只经 CLI 透传）"),
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
    // 注册面只扫直写盘原语。
    let mut reg_hits = vec![];
    for rel in REGISTRY_SOURCE_FILES {
        let src = source_of(rel);
        assert!(!src.is_empty(), "{rel} 不可读");
        for (pattern, reason) in REGISTRY_FORBIDDEN_PATTERNS {
            let re = regex::Regex::new(pattern).unwrap();
            if re.is_match(&src) {
                reg_hits.push(format!("{rel}: {reason} ({pattern})"));
            }
        }
    }
    assert!(reg_hits.is_empty(), "注册面直写盘命中：\n{}", reg_hits.join("\n"));
}

#[test]
fn zero_write_guard_no_write_subcommand_literals() {
    // scribe 写动词与 lease 写动词子命令字面量（独立引号形）禁入 alpha 面；
    // Python 禁词表独立引号形对等。writeface 面合法含 CLI 子命令字面量
    // （argv 构造本体），不扫。
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

#[test]
fn zero_write_guard_writeface_no_direct_writes() {
    let mut hits = vec![];
    for rel in WRITEFACE_SOURCE_FILES {
        let src = source_of(rel);
        assert!(!src.is_empty(), "{rel} 不可读");
        for (pattern, reason) in WRITEFACE_FORBIDDEN_PATTERNS {
            let re = regex::Regex::new(pattern).unwrap();
            if re.is_match(&src) {
                hits.push(format!("{rel}: {reason} ({pattern})"));
            }
        }
    }
    // 白名单单点：session.rs 正身报告 tempfile 暂存（本包唯一文件写位）。
    let session_src = source_of("src/mcpserver/session.rs");
    for pattern in [r"\bfs::write\b|\bFile::create\b", r"\bremove_file\b"] {
        let re = regex::Regex::new(pattern).unwrap();
        assert!(re.is_match(&session_src) == false || session_src.contains("唯一文件写位"),
            "session.rs 写位越出正身暂存白名单 ({pattern})");
    }
    assert!(hits.is_empty(), "writeface 直写盘命中：\n{}", hits.join("\n"));
}

// ------------------------------------------------------------- beta 单元基线

#[test]
fn matrix_exposure_by_class() {
    use crate::mcpserver::matrix::{is_tool_exposed, resolve_agent_class};
    // 缺省 external（最小特权）：最小写集十行透传，直改与领取释放拒。
    assert_eq!(resolve_agent_class(), "external"); // 测试进程未设 env
    assert!(is_tool_exposed("lease_open", "external"));
    assert!(is_tool_exposed("lease_commit", "external"));
    assert!(!is_tool_exposed("record_direct", "external"));
    assert!(!is_tool_exposed("lease_unclaim", "external"));
    // local 全矩阵。
    assert!(is_tool_exposed("record_direct", "local"));
    assert!(is_tool_exposed("lease_unclaim", "local"));
    // takeover 与 bypass 无行即无注册即无调用面。
    assert!(!is_tool_exposed("takeover", "local"));
    assert!(!is_tool_exposed("bypass", "local"));
}

#[test]
fn reason_table_first_match_wins() {
    use crate::mcpserver::errors::match_reason;
    let (code, gate, _, _) = match_reason("lease lock 退出码 1: locked_elsewhere 已持锁").unwrap();
    assert_eq!(code, "locked_elsewhere");
    assert_eq!(gate, "lease lock 锁冲突拒");
    assert!(match_reason("").is_none());
    assert!(match_reason("平平无奇报文").is_none());
}

#[test]
fn error_text_sanitization() {
    use crate::mcpserver::errors::{redact_other_sessions, relativize_paths, sanitize_error_text};
    let root = "/Users/x/SiHankor";
    assert_eq!(
        relativize_paths(&format!("{root}/sih-engine/x.rs 爆了"), root),
        "sih-engine/x.rs 爆了" // 根前缀剥除形（Python relativize_paths 对等）
    );
    let (out, n) = redact_other_sessions("会话 0123456789abcdef 与 0123456789abcdef 持锁", Some("0123456789abcdef"));
    assert_eq!(n, 0);
    assert!(out.contains("0123456789abcdef"));
    let (out2, n2) = redact_other_sessions("会话 fedcba9876543210 持锁", Some("0123456789abcdef"));
    assert_eq!(n2, 1);
    assert!(out2.contains("[他会话号已裁剪]"));
    // external 级裁剪串接形。
    let s = sanitize_error_text(&format!("{root}/a 会话 fedcba9876543210 拒"), root, "external", None);
    assert!(!s.contains(root));
    assert!(s.contains("[他会话号已裁剪]"));
}

#[test]
fn passthrough_shapes() {
    use crate::mcpserver::passthrough::{anchored, passthrough_result, parse_session_id};
    let root = std::path::Path::new("/tmp/fakeroot");
    assert_eq!(anchored(root, "work/a.json"), "/tmp/fakeroot/work/a.json");
    assert_eq!(anchored(root, "/abs/a.json"), "/abs/a.json");
    let v = passthrough_result("t", Some("s"), 0, "{\"session_id\": \"abc\"}\n", "");
    assert_eq!(v["exit_code"], 0);
    assert_eq!(v["result"]["session_id"], "abc");
    assert_eq!(parse_session_id("{\"session_id\": \"0123456789abcdef\"}").unwrap(), "0123456789abcdef");
    assert!(parse_session_id("not json").is_none());
}

#[tokio::test]
async fn precheck_unbound_teaches_open() {
    use crate::mcpserver::session::ConnectionSession;
    let mut conn = ConnectionSession::new("external");
    let v = crate::mcpserver::tools::tool_record_park(&mut conn, Some("work/park.json".to_string()), None).await;
    assert_eq!(v["reason_code"], "session_not_bound");
    assert_eq!(v["gate"], "连接与会话一对一");
    assert_eq!(v["exit_code_semantics"].as_str().unwrap().len() > 10, true);
}
