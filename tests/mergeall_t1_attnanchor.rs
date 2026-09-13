//! lease-mergeall-parallel 簇A T1：attnanchor 引擎 bin 金向量三例。
//! 拒绝形对表围堰契约改形申报：围堰退出码恒零（注入链路永不阻断会话），
//! 坏输入不产生非零退出，降级行在 additionalContext 内可见不静默——
//! 故拒绝形断言「缺场降级可见 + 退出码仍零」，非通用模板的非零退出断言。

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

fn write_file(p: &Path, s: &str) {
    if let Some(d) = p.parent() {
        std::fs::create_dir_all(d).unwrap();
    }
    std::fs::write(p, s).unwrap();
}

struct Ws {
    root: PathBuf,
    _guard: tempfile::TempDir,
}

fn empty_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let root = guard.path().to_path_buf();
    Ws { root, _guard: guard }
}

fn run(root: &Path, extra: &[&str]) -> (i32, Value) {
    let out = Command::new(env!("CARGO_BIN_EXE_attnanchor"))
        .arg("--root")
        .arg(root)
        .args(extra)
        .output()
        .unwrap();
    let rc = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let v: Value =
        serde_json::from_str(&stdout).expect("出参须为严格 JSON 单对象");
    (rc, v)
}

fn ctx_lines(v: &Value) -> Vec<String> {
    v["additionalContext"]
        .as_str()
        .expect("单键 additionalContext")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

#[test]
fn t1_normal_five_lines_assembled() {
    let ws = empty_ws();
    write_file(&ws.root.join(".session-anchor.md"), "簇A施工：三件移植\n");
    write_file(
        &ws.root.join("sih-tools/lease/ledger/sessions.ndjson"),
        "{\"session_id\":\"s1\",\"event\":\"issued\"}\n{\"session_id\":\"s2\",\"event\":\"closed\"}\n",
    );
    // acquired 后 released 同键覆盖：末态非持锁
    write_file(
        &ws.root.join("sih-tools/lease/ledger/locks.ndjson"),
        "{\"session_id\":\"s1\",\"path\":\"a\",\"event\":\"acquired\"}\n{\"session_id\":\"s1\",\"path\":\"a\",\"event\":\"released\"}\n",
    );
    let (rc, v) = run(&ws.root, &["--at", "2026-09-13"]);
    assert_eq!(rc, 0, "退出码恒零契约");
    assert_eq!(v.as_object().unwrap().len(), 1, "严格 JSON 单键 additionalContext");
    let lines = ctx_lines(&v);
    assert_eq!(lines.len(), 5, "五行锚读数：{:?}", lines);
    assert_eq!(lines[0], "[锚] 簇A施工：三件移植");
    assert_eq!(lines[1], "[在飞] 会话 1 在册 持锁 0 条");
    // fixture 无 selector 工地：泊界降级如实
    assert_eq!(lines[2], "[泊界] 路由不可用（降级）");
    // fixture 无当日 trail：链面降级如实
    assert_eq!(lines[3], "[链] 当日链不可读（降级）");
    assert!(lines[4].starts_with("[令] 新主题先三岔"), "纪律令在第五行：{}", lines[4]);
}

#[test]
fn t2_reject_form_degrade_visible_constant_zero() {
    let ws = empty_ws();
    let (rc, v) = run(&ws.root, &["--at", "2026-09-13"]);
    assert_eq!(rc, 0, "围堰契约：退出码恒零，坏输入不阻断会话");
    let lines = ctx_lines(&v);
    assert_eq!(lines.len(), 5);
    assert!(
        lines[0].contains("缺场告警"),
        "缺锚降级行可见：{}",
        lines[0]
    );
    assert_eq!(lines[1], "[在飞] 账本不可用（降级）");
}

#[test]
fn t3_edge_empty_forms() {
    let ws = empty_ws();
    // 首行空形：文件在场但首行为空
    write_file(&ws.root.join(".session-anchor.md"), "\n");
    // 空账本形：读得动但零行
    write_file(&ws.root.join("sih-tools/lease/ledger/sessions.ndjson"), "");
    write_file(&ws.root.join("sih-tools/lease/ledger/locks.ndjson"), "");
    // 空链形：当日 trail 在场但零行
    write_file(
        &ws.root.join("sih-engine/sih/event/trail/2026-09-13.ndjson"),
        "",
    );
    let (rc, v) = run(&ws.root, &["--at", "2026-09-13"]);
    assert_eq!(rc, 0);
    let lines = ctx_lines(&v);
    assert_eq!(lines[0], "[锚] 任务锚文件首行为空，先立锚再动工");
    assert_eq!(lines[1], "[在飞] 会话 0 在册 持锁 0 条");
    assert_eq!(lines[3], "[链] 今日 0 笔 末笔  ", "空链零行如实：{:?}", lines[3]);
}
