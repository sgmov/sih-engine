//! 引擎件核阅组件 cargo 测试入口
//!
//! 承 SPEC-013 § 验收判据 A1-A6 与任务包 F-2 测试守卫要求：
//! - 金向量六件逐字节断言（des-001 域 GOV-002/GOV-003、des-001-mathe 域 LIM-001/MUL-001、ask3 域 ask3-scrutmerge-sdd/ask3-viewrider）
//! - 退出码五场景（合规、空载、单违规、域外、缺包）
//! - 多包归因（双包同载、发现逐条携包名）
//! - 位置参数形（与工具件同形；与 --target 旗标形逐字节一致）
//!
//! 资源位：src/scrutinator/fixtures/golden/ 即六件 JSON
//! 目标位：六个真实目标，路径在金向量文件中固定

use std::path::Path;
use std::process::Command;

/// 引擎件 binary 路径。
///
/// 优先用 cargo 集成测试的 CARGO_BIN_EXE_scrutinator；lib test 时 fallback 到
/// `target/debug/scrutinator` 绝对路径（前提：cargo build --bins 已 build）。
fn bin() -> String {
    if let Ok(p) = std::env::var("CARGO_BIN_EXE_scrutinator") {
        return p;
    }
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR 未设置");
    let p = std::path::Path::new(&manifest).join("target/debug/scrutinator");
    if p.exists() {
        return p.to_str().unwrap().to_string();
    }
    panic!(
        "scrutinator binary 未找到：CARGO_BIN_EXE_scrutinator 未设且 {} 不存在。请先 cargo build --bins 再 cargo test --lib",
        p.display()
    );
}

/// 跑引擎件 CLI，传 --pack + --target（旗标形），返回 (exit_code, stdout_string)
fn run_flag(packs: &[&str], targets: &[&str]) -> (i32, String) {
    let mut cmd = Command::new(bin());
    for p in packs {
        cmd.arg("--pack").arg(p);
    }
    for t in targets {
        cmd.arg("--target").arg(t);
    }
    let out = cmd.output().expect("scrutinator binary 启动失败");
    let code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    (code, stdout)
}

/// 跑引擎件 CLI，传 --pack + 位置参数目标（位置形），返回 (exit_code, stdout_string)
fn run_positional(packs: &[&str], targets: &[&str]) -> (i32, String) {
    let mut cmd = Command::new(bin());
    for p in packs {
        cmd.arg("--pack").arg(p);
    }
    for t in targets {
        cmd.arg(t);
    }
    let out = cmd.output().expect("scrutinator binary 启动失败");
    let code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    (code, stdout)
}

/// 从金向量 JSON 提取目标路径（金向量 targets 数组第 0 项绝对路径）
fn golden_target(golden_path: &Path) -> String {
    let bytes = std::fs::read(golden_path).expect("金向量读失败");
    let v: serde_json::Value = serde_json::from_slice(&bytes).expect("金向量 JSON 解析失败");
    let t = v.get("targets").and_then(|t| t.as_array()).and_then(|a| a.first()).and_then(|s| s.as_str());
    t.expect("金向量无 targets[0]").to_string()
}

/// 路径：CARGO_MANIFEST_DIR 即 crate 根
fn crate_root() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

fn golden_path(name: &str) -> std::path::PathBuf {
    Path::new(crate_root()).join("src/scrutinator/fixtures/golden").join(name)
}

// ============================================================================
// 金向量六件（逐字节断言）
// ============================================================================

#[test]
fn golden_ask3_scrutmerge_sdd() {
    let golden_file = golden_path("ask3-scrutmerge-sdd.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["ask3"], &[&target]);
    assert_eq!(code, 0, "ask3-scrutmerge-sdd 金向量场景应退出 0，实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "ask3-scrutmerge-sdd 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_ask3_viewrider() {
    let golden_file = golden_path("ask3-viewrider.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["ask3"], &[&target]);
    assert_eq!(code, 0, "ask3-viewrider 金向量场景应退出 0，实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "ask3-viewrider 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_des001_gov002() {
    let golden_file = golden_path("des-001-gov002.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001"], &[&target]);
    assert_eq!(code, 0, "des-001-gov002 金向量场景应退出 0，实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "des-001-gov002 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_des001_gov003() {
    let golden_file = golden_path("des-001-gov003.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001"], &[&target]);
    assert_eq!(code, 0, "des-001-gov003 金向量场景应退出 0，实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "des-001-gov003 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_des001mathe_lim001() {
    let golden_file = golden_path("des-001-mathe-lim001.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001-mathe"], &[&target]);
    assert_eq!(code, 0, "des-001-mathe-lim001 金向量场景应退出 0，实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "des-001-mathe-lim001 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_des001mathe_mul001() {
    let golden_file = golden_path("des-001-mathe-mul001.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001-mathe"], &[&target]);
    assert_eq!(code, 0, "des-001-mathe-mul001 金向量场景应退出 0，实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "des-001-mathe-mul001 引擎件输出与金向量逐字节不一致");
}

// ============================================================================
// goldfix-solo 整改批新增 6 件金向量（脏目标覆盖 12 码 + 真目标 DEC-020）
//
// 冻结基础：工具件 Python 逐字节输出即基准（承任务包 § 二 关键设计 一）
// 覆盖：C001 C002 C006 S002 S004 S005 S006 F000 F002 F003 F005 N002
// ============================================================================

#[test]
fn golden_des001_goldfix_001_multiflag() {
    let golden_file = golden_path("des-001-goldfix-001-multiflag.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001"], &[&target]);
    assert_eq!(code, 1, "goldfix-001-multiflag 应退出 1（违规），实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "goldfix-001-multiflag 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_des001_goldfix_002_header() {
    let golden_file = golden_path("des-001-goldfix-002-header.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001"], &[&target]);
    assert_eq!(code, 1, "goldfix-002-header 应退出 1（违规），实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "goldfix-002-header 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_des001_goldfix_003_fence() {
    let golden_file = golden_path("des-001-goldfix-003-fence.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001"], &[&target]);
    assert_eq!(code, 1, "goldfix-003-fence 应退出 1（违规），实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "goldfix-003-fence 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_des001_goldfix_004_nav() {
    let golden_file = golden_path("des-001-goldfix-004-nav.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001"], &[&target]);
    assert_eq!(code, 1, "goldfix-004-nav 应退出 1（违规），实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "goldfix-004-nav 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_des001_goldfix_005_skip() {
    let golden_file = golden_path("des-001-goldfix-005-skip.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001"], &[&target]);
    assert_eq!(code, 1, "goldfix-005-skip 应退出 1（违规），实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "goldfix-005-skip 引擎件输出与金向量逐字节不一致");
}

#[test]
fn golden_des001_dec020() {
    let golden_file = golden_path("des-001-dec020.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_flag(&["des-001"], &[&target]);
    assert_eq!(code, 1, "DEC-020 应退出 1（违规），实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "DEC-020 引擎件输出与金向量逐字节不一致");
}

// ============================================================================
// 退出码五场景用例
// ============================================================================

#[test]
fn exit_compliant_zero() {
    // 合规场景：GOV-002 在 des-001 域内，零违规，退出码 0
    let target = golden_target(&golden_path("des-001-gov002.json"));
    let (code, _stdout) = run_flag(&["des-001"], &[&target]);
    assert_eq!(code, 0, "合规场景应退出 0，实际 {}", code);
}

#[test]
fn exit_empty_load_zero() {
    // 空载形：不传任何 target，产 packs=[] findings=[] 报告，退出码 0
    let (code, _stdout) = run_flag(&["des-001"], &[]);
    assert_eq!(code, 0, "空载场景应退出 0，实际 {}", code);
}

#[test]
fn exit_single_violation_one() {
    // 单违规场景：tmp 文件在 des-001 域（**/scrutinator/fixtures/corpus/**/*.md），
    // 内容触发 C001 破折号
    let tmp = std::path::Path::new(crate_root())
        .join("src/scrutinator/fixtures/corpus/.scrut-test-single-violation.md");
    std::fs::create_dir_all(tmp.parent().unwrap()).expect("corpus 目录创建失败");
    std::fs::write(&tmp, "# 测试标题\n\n内容包含破折号——这会触发 C001。\n").expect("tmp 写失败");
    let target = tmp.to_str().unwrap().to_string();
    let (code, _stdout) = run_flag(&["des-001"], &[&target]);
    let _ = std::fs::remove_file(&tmp);
    let _ = std::fs::remove_dir(tmp.parent().unwrap());
    assert_eq!(code, 1, "单违规场景应退出 1，实际 {}", code);
}

#[test]
fn exit_out_of_domain_two() {
    // 域外场景：用一个不在任何已加载包域内的目标
    let target = "/tmp/this-is-definitely-out-of-domain-xyz123.md";
    // 先创建空文件确保读不到的内容是域外而非文件不存在
    std::fs::write(target, "").ok();
    let (code, _stdout) = run_flag(&["des-001"], &[target]);
    let _ = std::fs::remove_file(target);
    assert_eq!(code, 2, "域外场景应退出 2，实际 {}", code);
}

#[test]
fn exit_missing_pack_two() {
    // 缺包场景：未知名包产退出码 2
    let target = golden_target(&golden_path("des-001-gov002.json"));
    let (code, _stdout) = run_flag(&["unknown-pack-xyz-12345"], &[&target]);
    assert_eq!(code, 2, "缺包场景应退出 2，实际 {}", code);
}

// ============================================================================
// 多包归因用例
// ============================================================================

#[test]
fn multipack_attribution_pack_names() {
    // 双包同载：des-001 + ask3，每个包用自己的域
    // 实际：取 GOV-002 + ask3-scrutmerge-sdd 双目标
    let target_gov = golden_target(&golden_path("des-001-gov002.json"));
    let target_ask3 = golden_target(&golden_path("ask3-scrutmerge-sdd.json"));
    let (code, stdout) = run_flag(&["des-001", "ask3"], &[&target_gov, &target_ask3]);
    assert_eq!(code, 0, "多包同载合规场景应退出 0，实际 {}", code);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("多包输出解析失败");
    let packs = v.get("packs").and_then(|p| p.as_array()).expect("packs 字段缺失");
    assert_eq!(packs.len(), 2, "应加载两个包，实际 {}", packs.len());
    let pack_names: Vec<String> = packs.iter()
        .filter_map(|p| p.get("name").and_then(|n| n.as_str()).map(String::from))
        .collect();
    assert!(pack_names.contains(&"des-001".to_string()), "packs 应含 des-001");
    assert!(pack_names.contains(&"ask3".to_string()), "packs 应含 ask3");
}

// ============================================================================
// CLI 双形（位置参数形 + --target 旗标形）
// ============================================================================

#[test]
fn cli_positional_form_matches_golden() {
    // 位置参数形：与金向量逐字节一致（与旗标形同输出）
    let golden_file = golden_path("des-001-gov002.json");
    let target = golden_target(&golden_file);
    let (code, stdout) = run_positional(&["des-001"], &[&target]);
    assert_eq!(code, 0, "位置参数形应识别目标并退出 0，实际 {}", code);
    let expected = std::fs::read_to_string(&golden_file).expect("金向量文件读失败");
    assert_eq!(stdout, expected, "位置参数形引擎件输出与金向量逐字节不一致");
}

#[test]
fn cli_positional_and_flag_forms_byte_identical() {
    // 位置形与旗标形同包同目标同路径串输出逐字节一致
    let target = golden_target(&golden_path("des-001-gov002.json"));
    let (code_flag, stdout_flag) = run_flag(&["des-001"], &[&target]);
    let (code_pos, stdout_pos) = run_positional(&["des-001"], &[&target]);
    assert_eq!(code_flag, 0, "旗标形应退出 0");
    assert_eq!(code_pos, 0, "位置形应退出 0");
    assert_eq!(code_flag, code_pos, "两形退出码不一致：flag={} pos={}", code_flag, code_pos);
    assert_eq!(stdout_flag, stdout_pos, "两形输出不一致");
}

// ============================================================================
// C009 判定性常数邻近级地板（gatemath-solo 批增，SDD 承 SPEC-019）
// 文档级谓词 doc_constant_gate：声明形行围栏豁免与行内代码剥离，
// 邻近在场即载体词面与推导档词面同文档双在场，缺一即每条声明行各一笔。
// ============================================================================

const C9_RULE_TOML: &str = r#"
[[rules]]
id = "C009"
kind = "doc_constant_gate"
params = { declaration = '(阈值|判据|裁决规则|水平|窗口|预算)\s*[=＝为：:]?\s*[0-9]', carrier = '\b(TOP|PROB|ORD|ALG|CALC|LIM|DIFF|INT|APP)-[0-9]+\b', derivation = 'sih-math/docs/[^ \t`)）】，。]*derivation[^ \t`)）】，。]*' }
message = "判定性常数裸奔邻近级"
"#;

fn c9_rule() -> crate::scrutinator::rule::RuleEntry {
    let rules = crate::scrutinator::rule::parse_rules(C9_RULE_TOML)
        .expect("doc_constant_gate 规则应可解析");
    assert_eq!(rules.len(), 1);
    rules.into_iter().next().unwrap()
}

fn c9_lines(rule: &crate::scrutinator::rule::RuleEntry, text: &str) -> Vec<usize> {
    crate::scrutinator::rule::check_text(rule, text)
        .into_iter()
        .map(|f| f.line)
        .collect()
}

#[test]
fn c9_parse_accepts_doc_constant_gate() {
    let rule = c9_rule();
    assert_eq!(rule.id, "C009");
    assert!(matches!(
        rule.kind,
        crate::scrutinator::rule::RuleKind::DocConstantGate
    ));
}

#[test]
fn c9_compliant_nearby_presence_zero() {
    let rule = c9_rule();
    let text = "# 样文\n\n置信水平 0.05 为检验判据。\n\n载体承 PROB-010 排队论与推导档 sih-math/docs/gatemath-derivation-2026-09-04.md。\n";
    assert!(c9_lines(&rule, text).is_empty());
}

#[test]
fn c9_naked_document_fires_per_declaration_line() {
    let rule = c9_rule();
    let text = "# 样文\n\n重试上限窗口 30 天。\n预算 9 发封顶。\n";
    assert_eq!(c9_lines(&rule, text), vec![3, 4]);
}

#[test]
fn c9_missing_derivation_only_fires() {
    let rule = c9_rule();
    let text = "# 样文\n\n窗口 7 天起算。\n载体承 ORD-019 版本偏序。\n";
    assert_eq!(c9_lines(&rule, text), vec![3]);
}

#[test]
fn c9_missing_carrier_only_fires() {
    let rule = c9_rule();
    let text = "# 样文\n\n阈值 0.9。\n推导档 sih-math/docs/gatemath-derivation-2026-09-04.md 在档。\n";
    assert_eq!(c9_lines(&rule, text), vec![3]);
}

#[test]
fn c9_fence_declaration_exempt_and_pointer_in_fence_counts() {
    let rule = c9_rule();
    let text = "# 样文\n\n```text\n窗口 30 天示例\n```\n\n载体承 TOP-008 与推导档 sih-math/docs/x-derivation-1.md。\n";
    assert!(c9_lines(&rule, text).is_empty());
}

#[test]
fn c9_inline_code_declaration_stripped() {
    let rule = c9_rule();
    let text = "# 样文\n\n写法 `窗口 30 天` 仅为示例词面。\n\n载体承 ALG-002 与推导档 sih-math/docs/y-derivation-2.md。\n";
    assert!(c9_lines(&rule, text).is_empty());
}

#[test]
fn c9_extended_trigger_words_only_with_digit_adjacency() {
    let rule = c9_rule();
    let text = "# 样文\n\n达到可工程化水平。承接五段结构。\n窗口期天数另定。\n\n载体承 PROB-011 与推导档 sih-math/docs/z-derivation-3.md。\n";
    assert!(c9_lines(&rule, text).is_empty());
}

#[test]
fn c9_bridge_chars_fire() {
    let rule = c9_rule();
    let text = "# 样文\n\n窗口: 12 天一轮。\n预算=3 发。\n显著性水平为 0.05。\n";
    assert_eq!(c9_lines(&rule, text), vec![3, 4, 5]);
}

// ============================================================================
// scrutpath-solo 批增：域判定路径基根锚定（工地路径域外误判修复）
//
// 病灶：des-001 include 按根相对 glob（sih-engine/doc/**），引擎件按
// canonicalize 绝对路径匹配，工地路径形 worktrees/<仓>/<批>/doc/... 不命中
// 即域外 exit 2（acceptclose 与 constclear2c 与 adjudicate2 三窗申报同款，
// 本批修复前红证 materials/prefix-red-worktree-exit2.json 在档）。
// 修法：anchor.py 同款上溯判根（标记件 sih-tools/lease/ledger/sessions.ndjson）
// 加 worktree 路径归一化（剥 worktrees/<仓>/<批> 前缀回仓相对形）。
// 红线：域清单零扩面（des-001 仍只盖 sih-engine/doc）、退出码三值语义零变。
// ============================================================================

fn des001_spec() -> crate::scrutinator::rule::DomainSpec {
    let (dom, _) = crate::scrutinator::rule::load_pack("des-001").expect("des-001 包应可加载");
    dom
}

#[test]
fn scrutpath_rooted_worktree_doc_in_domain() {
    // 工地路径形经根锚定归一后应判域内（修复前 false 即三窗申报病灶）
    let dom = des001_spec();
    let root = "/fake/ws-root";
    let path = "/fake/ws-root/worktrees/sih-engine/scrutpath-solo/doc/GOV-900.md";
    assert!(
        crate::scrutinator::rule::domain_match_rooted(&dom, root, path),
        "工地 doc 路径形经根锚定归一后应判域内"
    );
}

#[test]
fn scrutpath_rooted_maintree_rel_zero_regression() {
    // 主树根相对形既有判定零回归
    let dom = des001_spec();
    let root = "/fake/ws-root";
    let path = "/fake/ws-root/sih-engine/doc/GOV-900.md";
    assert!(
        crate::scrutinator::rule::domain_match_rooted(&dom, root, path),
        "主树根相对 doc 形应域内"
    );
}

#[test]
fn scrutpath_rooted_out_of_domain_unchanged() {
    // 域清单零扩面与 exclude 语义零变：域外路径不得因根锚定入域，skills 排除仍生效
    let dom = des001_spec();
    let root = "/fake/ws-root";
    assert!(
        !crate::scrutinator::rule::domain_match_rooted(&dom, root, "/fake/ws-root/other/doc/x.md"),
        "域外路径不得因根锚定扩面入域"
    );
    assert!(
        !crate::scrutinator::rule::domain_match_rooted(
            &dom,
            root,
            "/fake/ws-root/worktrees/sih-engine/b/doc/skills/x.md"
        ),
        "exclude 语义零变：worktree 形下 skills 排除仍生效"
    );
}

#[test]
fn scrutpath_normalize_worktree_rel() {
    // 归一化单元：剥 worktrees/<仓>/<批> 前缀回仓相对形；非 worktree 形返回 None
    assert_eq!(
        crate::scrutinator::rule::normalize_worktree_rel(
            "worktrees/sih-engine/scrutpath-solo/doc/x.md"
        )
        .as_deref(),
        Some("sih-engine/doc/x.md")
    );
    assert_eq!(crate::scrutinator::rule::normalize_worktree_rel("sih-engine/doc/x.md"), None);
    assert_eq!(crate::scrutinator::rule::normalize_worktree_rel("worktrees/sih-engine"), None);
}

#[test]
fn scrutpath_discover_workspace_root_finds_marker() {
    // 上溯判根（anchor.py 同款标记件）：从目标路径逐级上溯找 sih-tools/lease/ledger/sessions.ndjson
    let ws = tempfile::tempdir().expect("tempdir");
    let marker = ws.path().join("sih-tools/lease/ledger/sessions.ndjson");
    std::fs::create_dir_all(marker.parent().unwrap()).expect("marker 目录创建");
    std::fs::write(&marker, "").expect("marker 写");
    let target = ws.path().join("worktrees/sih-engine/b/doc/x.md");
    std::fs::create_dir_all(target.parent().unwrap()).expect("target 目录创建");
    let root = crate::scrutinator::rule::discover_workspace_root(target.to_str().unwrap())
        .expect("应发现工作区根");
    assert_eq!(std::path::Path::new(&root), ws.path(), "判根应落在标记件所在目录");
}

#[test]
fn scrutpath_discover_workspace_root_none_without_marker() {
    // 无标记件面（域外纵深守卫）：/tmp 上溯无根应返回 None，退出码三值语义零变承此
    assert!(crate::scrutinator::rule::discover_workspace_root("/tmp").is_none());
}

#[test]
fn scrutpath_worktree_path_form_exit_zero() {
    // CLI 级（假工作区全链）：工地 doc 路径应判域内（修复前 exit 2 即三窗申报形）
    let ws = tempfile::tempdir().expect("tempdir");
    let marker = ws.path().join("sih-tools/lease/ledger/sessions.ndjson");
    std::fs::create_dir_all(marker.parent().unwrap()).expect("marker 目录创建");
    std::fs::write(&marker, "").expect("marker 写");
    let doc = ws.path().join("worktrees/sih-engine/scrutpath-solo/doc/SCRUTPATH-TEST.md");
    std::fs::create_dir_all(doc.parent().unwrap()).expect("doc 目录创建");
    std::fs::write(&doc, "# 测试标题\n\n零违规正文。\n").expect("doc 写");
    let (code, _stdout) = run_flag(&["des-001"], &[doc.to_str().unwrap()]);
    assert_eq!(code, 0, "工地 doc 路径形应判域内退出 0（域外即三窗病灶形），实际 {}", code);
}

