//! lease-mergeleg6-parallel 簇J T6：projsnap 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/projsnap 0.1.0（cli.py：build 再生、check stale 三形、
//! diff 差量、手写节守卫、同参双跑幂等，DES-018 测试计划 T1 至 T4 同参对表）。
//! fixture 全部 temp 自建最小工作区，围堰真实工作区零触碰。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin_projsnap() -> &'static str {
    env!("CARGO_BIN_EXE_projsnap")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_projsnap()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

const TEMPLATE: &str = "# fixture 工作区\n\n手写节：项目身份（宪法性手写，零触碰）。\n\n<!-- projsnap:begin file-index -->\n<!-- projsnap:end file-index -->\n\n<!-- projsnap:begin tool-versions -->\n<!-- projsnap:end tool-versions -->\n\n<!-- projsnap:begin parked-count -->\n<!-- projsnap:end parked-count -->\n\n手写尾节：产出前自检（零触碰）。\n";

struct Ws {
    dir: PathBuf,
    target: PathBuf,
    _guard: tempfile::TempDir,
}

/// temp 自建最小工作区：三登记目录加泊材料两件加组件两件加 AGENTS.md 空节模板。
fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    fs::create_dir_all(dir.join("sih-engine/doc/decision")).unwrap();
    fs::create_dir_all(dir.join("sih-engine/doc/design")).unwrap();
    fs::create_dir_all(dir.join("sih-engine/doc/spec")).unwrap();
    fs::create_dir_all(dir.join("sih-engine/sih/state/parking/materials")).unwrap();
    fs::write(dir.join("sih-engine/doc/decision/000-first.md"), "# 000\n").unwrap();
    fs::write(dir.join("sih-engine/doc/design/DES-001-y.md"), "# DES-001\n").unwrap();
    fs::write(dir.join("sih-engine/doc/spec/SPEC-001-x.md"), "# SPEC-001\n").unwrap();
    for (name, ver) in [("alpha", "0.1.0"), ("beta", "0.2.0")] {
        let d = dir.join(format!("sih-tools/{name}"));
        fs::create_dir_all(&d).unwrap();
        fs::write(
            d.join("pyproject.toml"),
            format!("[project]\nname = \"{name}\"\nversion = \"{ver}\"\n"),
        )
        .unwrap();
    }
    fs::write(
        dir.join("sih-engine/sih/state/parking/materials/pk-001.json"),
        r#"{"state": "parked"}"#,
    )
    .unwrap();
    fs::write(
        dir.join("sih-engine/sih/state/parking/materials/pk-002.json"),
        r#"{"state": "exited"}"#,
    )
    .unwrap();
    let target = dir.join("AGENTS.md");
    fs::write(&target, TEMPLATE).unwrap();
    Ws { dir, target, _guard: guard }
}

fn ws_arg(ws: &Ws) -> String {
    ws.dir.to_string_lossy().into_owned()
}

fn target_arg(ws: &Ws) -> String {
    ws.target.to_string_lossy().into_owned()
}

// ---------- 正常形：build 再生、手写节零触碰、重跑落零、check 回鲜 ----------

#[test]
fn t1_normal_build_guarded_idempotent() {
    let ws = build_ws();
    let ra = ws_arg(&ws);
    let ta = target_arg(&ws);

    // build --json（首遍）：changed 真、三节 created、节哈希与 sources_hash 在报
    let (code, out, _) = run(&["build", "--root", &ra, "--target", &ta, "--json"]);
    assert_eq!(code, 0, "build 恒 0");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["changed"], true);
    assert_eq!(rep["tool"], "projsnap");
    assert_eq!(rep["version"], "0.1.0");
    assert_eq!(rep["status"], "ok");
    let written = rep["written"].as_array().unwrap();
    assert_eq!(written.len(), 3, "首遍三节全 updated（模板空节已在场），got={written:?}");
    assert!(written.iter().all(|w| w["action"] == "updated"));
    assert_eq!(rep["sections"].as_object().unwrap().len(), 3);
    for (_, h) in rep["sections"].as_object().unwrap() {
        assert_eq!(h.as_str().unwrap().len(), 64, "节哈希须 sha256 hex");
    }
    assert_eq!(rep["sources_hash"].as_str().unwrap().len(), 64);

    // build（无 --json）：stdout 静默，退出码 0
    let (code, out, _) = run(&["build", "--root", &ra, "--target", &ta]);
    assert_eq!(code, 0);
    assert!(out.is_empty(), "无 --json stdout 须空，got={out}");

    // 二遍 --json 落零：changed 假、written 空
    let (code, out, _) = run(&["build", "--root", &ra, "--target", &ta, "--json"]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["changed"], false, "二遍 build 须落零");
    assert_eq!(rep["written"].as_array().unwrap().len(), 0);

    // 投影内容：三登记面逐行、组件版本面、泊界计数面
    let text = fs::read_to_string(&ws.target).unwrap();
    assert!(text.contains("- decision: 000-first.md"), "{text}");
    assert!(text.contains("- design: DES-001-y.md"));
    assert!(text.contains("- spec: SPEC-001-x.md"));
    assert!(text.contains("- alpha 0.1.0"));
    assert!(text.contains("- beta 0.2.0"));
    assert!(text.contains("- 泊界计数面: 在泊 1 项（投影，真相在链）"));
    // 手写节零触碰（DES-018 A2）：头节与尾节原样在场
    assert!(text.contains("手写节：项目身份（宪法性手写，零触碰）。"));
    assert!(text.contains("手写尾节：产出前自检（零触碰）。"));

    // 重跑落零：changed 假、文本恒同（ORD-023 幂等）
    let before = fs::read_to_string(&ws.target).unwrap();
    let (code, out, _) = run(&["build", "--root", &ra, "--target", &ta, "--json"]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["changed"], false, "重跑须落零");
    assert_eq!(fs::read_to_string(&ws.target).unwrap(), before);

    // check 回鲜：stale 空、fresh 三节、退出码 0
    let (code, out, _) = run(&["check", "--root", &ra, "--target", &ta, "--json"]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["stale"].as_array().unwrap().len(), 0);
    assert_eq!(rep["fresh"].as_array().unwrap().len(), 3);
}

// ---------- 拒绝形：未知子命令、无子命令、stale 三形逐位定位 ----------

#[test]
fn t2_reject_and_stale_forms() {
    let ws = build_ws();
    let ra = ws_arg(&ws);
    let ta = target_arg(&ws);
    run(&["build", "--root", &ra, "--target", &ta]);

    // 未知子命令：退出码 2，stderr JSON 用法报文
    let (code, out, err) = run(&["frobnicate", "--root", &ra]);
    assert_eq!(code, 2, "未知子命令须 2");
    assert!(out.is_empty());
    assert!(err.contains("用法 projsnap <build|check|diff>"), "err={err}");

    // 无子命令：退出码 2
    let (code, _, err) = run(&[]);
    assert_eq!(code, 2, "无子命令须 2");
    assert!(err.contains("用法"));

    // 源变一：新决策档 → file-index stale（source_newer）
    fs::write(ws.dir.join("sih-engine/doc/decision/001-second.md"), "# 001\n").unwrap();
    let (code, out, _) = run(&["check", "--root", &ra, "--target", &ta, "--json"]);
    assert_eq!(code, 1, "有 stale 须 1");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        rep["stale"],
        serde_json::json!([{"reason": "source_newer", "section": "file-index"}]),
        "stale 逐位定位，got={:?}",
        rep["stale"]
    );

    // 源变二：新泊件 → parked-count stale
    run(&["build", "--root", &ra, "--target", &ta]);
    fs::write(
        ws.dir.join("sih-engine/sih/state/parking/materials/pk-003.json"),
        r#"{"state": "parked"}"#,
    )
    .unwrap();
    let (code, out, _) = run(&["check", "--root", &ra, "--target", &ta, "--json"]);
    assert_eq!(code, 1);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["stale"][0]["section"], "parked-count");
    assert_eq!(rep["stale"][0]["reason"], "source_newer");

    // 源变三：组件版本变更 → tool-versions stale
    run(&["build", "--root", &ra, "--target", &ta]);
    fs::write(
        ws.dir.join("sih-tools/alpha/pyproject.toml"),
        "[project]\nname = \"alpha\"\nversion = \"0.9.0\"\n",
    )
    .unwrap();
    let (code, out, _) = run(&["check", "--root", &ra, "--target", &ta, "--json"]);
    assert_eq!(code, 1);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["stale"][0]["section"], "tool-versions");

    // 再生后回鲜
    run(&["build", "--root", &ra, "--target", &ta]);
    let (code, out, _) = run(&["check", "--root", &ra, "--target", &ta, "--json"]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["stale"].as_array().unwrap().len(), 0);
    assert!(fs::read_to_string(&ws.target).unwrap().contains("- alpha 0.9.0"));
}

// ---------- 边界形：diff 有差/无差、缺目标自建、absent 三节、未知旗标静默跳过 ----------

#[test]
fn t3_edge_diff_and_missing_target() {
    let ws = build_ws();
    let ra = ws_arg(&ws);
    let ta = target_arg(&ws);

    // diff（build 前）：有差退出码 1，unified 差量头两行与增行在场
    let (code, out, _) = run(&["diff", "--root", &ra, "--target", &ta]);
    assert_eq!(code, 1, "有差须 1");
    assert!(out.starts_with("--- current\n+++ regenerated\n"), "diff 头形，got={out}");
    assert!(out.contains("@@ "));
    assert!(out.contains("+- decision: 000-first.md"));
    assert!(out.contains(" - 手写节") || out.contains("手写节"), "上下文行含手写内容");

    // build 后 diff：无差退出码 0、stdout 空
    run(&["build", "--root", &ra, "--target", &ta]);
    let (code, out, _) = run(&["diff", "--root", &ra, "--target", &ta]);
    assert_eq!(code, 0);
    assert!(out.is_empty(), "无差 stdout 须空");

    // 缺目标 check：三节全 absent，退出码 1
    let missing = ws.dir.join("nope-AGENTS.md");
    let (code, out, _) = run(&["check", "--root", &ra, "--target", &missing.to_string_lossy(), "--json"]);
    assert_eq!(code, 1);
    let rep: Value = serde_json::from_str(&out).unwrap();
    let stale = rep["stale"].as_array().unwrap();
    assert_eq!(stale.len(), 3);
    assert!(stale.iter().all(|s| s["reason"] == "absent"));

    // 缺目标 build：文件自建、created 三条、退出码 0
    let (code, out, _) = run(&["build", "--root", &ra, "--target", &missing.to_string_lossy(), "--json"]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["changed"], true);
    let written = rep["written"].as_array().unwrap();
    assert_eq!(written.len(), 3);
    assert!(written.iter().all(|w| w["action"] == "created"));
    assert!(missing.is_file(), "缺目标 build 须自建");
    let (code, _, _) = run(&["check", "--root", &ra, "--target", &missing.to_string_lossy()]);
    assert_eq!(code, 0, "自建后回鲜");

    // 未知旗标静默跳过（围堰缺省形对表）：不影响 build 结果
    let (code, _, _) = run(&["build", "--root", &ra, "--target", &ta, "--bogus"]);
    assert_eq!(code, 0, "未知旗标须静默跳过");
}
