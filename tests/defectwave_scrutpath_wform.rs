//! defectwave 批缺陷三 CLI 级回归：scrutinator 工地路径域匹配缺口（w/ 布局）。
//! 病灶申报出处：sih/event/plan/des017-materials/des017-results.md 偏差申报节
//! 与 sih/event/plan/parksync-materials/parksync-results.md 偏差申报节
//!（2026-09-18 两文档批核阅首跑 exit-2，工地路径未归一，两法绕行确证真审）。
//! 修法：src/scrutinator/rule.rs normalize_worktree_rel 补 w/<名>/ 剥前缀回
//! sih-engine/（既有 worktrees/<仓>/<批>/ 形回归钉在 src/scrutinator/tests.rs
//! defectwave_worktrees_form_regression_pin）。
//! 注：src/scrutinator/tests.rs 的 CLI 级用例经 CARGO_MANIFEST_DIR/target 回退
//! 位取 binary，在共享 CARGO_TARGET_DIR 工地形属已知环境假红（基点同红），
//! 本件以集成测试 CARGO_BIN_EXE 位承载 CLI 级真绿证。

use std::process::Command;

/// CLI 级全链（假工作区）：w/ 工地 doc 路径应判域内退出 0（修复前 exit 2 即
/// des017 与 parksync 两批申报病灶形）。
#[test]
fn defectwave_w_form_worktree_doc_exit_zero() {
    let ws = tempfile::tempdir().expect("tempdir");
    let marker = ws.path().join("sih-tools/lease/ledger/sessions.ndjson");
    std::fs::create_dir_all(marker.parent().unwrap()).expect("marker 目录创建");
    std::fs::write(&marker, "").expect("marker 写");
    let doc = ws.path().join("w/defectwave/doc/design/DEFECTWAVE-TEST.md");
    std::fs::create_dir_all(doc.parent().unwrap()).expect("doc 目录创建");
    std::fs::write(&doc, "# 测试标题\n\n零违规正文。\n").expect("doc 写");
    let out = Command::new(env!("CARGO_BIN_EXE_scrutinator"))
        .args(["--pack", "des-001", "--target", doc.to_str().unwrap()])
        .output()
        .expect("scrutinator binary 启动失败");
    let code = out.status.code().unwrap_or(-1);
    assert_eq!(
        code, 0,
        "w/ 工地 doc 形应判域内退出 0，实际 {}: {} {}",
        code,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// CLI 级回归钉：既有 worktrees/<仓>/<批>/ 形退出 0 零回归。
#[test]
fn defectwave_worktrees_form_exit_zero_regression_pin() {
    let ws = tempfile::tempdir().expect("tempdir");
    let marker = ws.path().join("sih-tools/lease/ledger/sessions.ndjson");
    std::fs::create_dir_all(marker.parent().unwrap()).expect("marker 目录创建");
    std::fs::write(&marker, "").expect("marker 写");
    let doc = ws
        .path()
        .join("worktrees/sih-engine/defectwave/doc/design/DEFECTWAVE-TEST.md");
    std::fs::create_dir_all(doc.parent().unwrap()).expect("doc 目录创建");
    std::fs::write(&doc, "# 测试标题\n\n零违规正文。\n").expect("doc 写");
    let out = Command::new(env!("CARGO_BIN_EXE_scrutinator"))
        .args(["--pack", "des-001", "--target", doc.to_str().unwrap()])
        .output()
        .expect("scrutinator binary 启动失败");
    assert_eq!(
        out.status.code().unwrap_or(-1),
        0,
        "worktrees 工地 doc 形既有归一形零回归: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// 域清单零扩面：w/ 形域外路径（非 doc 面）仍 exit 2。
#[test]
fn defectwave_w_form_out_of_domain_still_exit_two() {
    let ws = tempfile::tempdir().expect("tempdir");
    let marker = ws.path().join("sih-tools/lease/ledger/sessions.ndjson");
    std::fs::create_dir_all(marker.parent().unwrap()).expect("marker 目录创建");
    std::fs::write(&marker, "").expect("marker 写");
    let src = ws.path().join("w/defectwave/src/lib.rs");
    std::fs::create_dir_all(src.parent().unwrap()).expect("src 目录创建");
    std::fs::write(&src, "// 非文档面\n").expect("src 写");
    let out = Command::new(env!("CARGO_BIN_EXE_scrutinator"))
        .args(["--pack", "des-001", "--target", src.to_str().unwrap()])
        .output()
        .expect("scrutinator binary 启动失败");
    assert_eq!(
        out.status.code().unwrap_or(-1),
        2,
        "w/ 形非 doc 面仍域外 exit 2（域清单零扩面）"
    );
}
