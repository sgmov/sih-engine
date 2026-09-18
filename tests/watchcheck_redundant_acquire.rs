//! hygwave 批件二：watchcheck 冗余重复 acquire 台账卫生读数（红转绿）。
//!
//! 正典指针：
//! - hygwave 批令（2026-09-18，工作工地 w/hygwave）；
//! - sih-engine/sih/event/plan/faceprecise-solo-materials/faceprecise-analysis.md §1.4
//!   （idwire-solo 会话 28d326f924916cd0 原始流 123 笔 acquired 仅 11 个不同路径，
//!   同路径重复 acquire 至 17 次，朴素时间序回放读出 112 并发）。
//!
//! 语义定义（与 §1.4 口径对齐）：
//! - max_repeat_acquires：同 (路径, 会话) 最多 acquired 笔数（faceprecise「重复 acquire 至 17 次」即 17）；
//! - redundant_total：会话内 Σ(单路径 acquired 笔数 − 1)（§1.4 朴素回放虚增 112 即此数）；
//! - redundant_total > REDUNDANT_ACQUIRE_THRESHOLD（3）才呈报，形承 LOCKFACE_WIDE_THRESHOLD 先例，只报不裁。
//!
//! 机读面：`check --json` 增可选键 `redundant_acquires`，零冗余时不出键（向后兼容）；
//! 缺省文本面零改动（mergeall_t4_watchcheck 金向量为证）。

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin_watchcheck() -> &'static str {
    env!("CARGO_BIN_EXE_watchcheck")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_watchcheck()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .expect("git 可用");
    assert!(
        out.status.success(),
        "git {args:?} 失败: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

struct Fx {
    root: PathBuf,
    locks: PathBuf,
    sessions: PathBuf,
    trail: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    let root = guard.path().to_path_buf();
    for name in ["sih-tools", "sih-engine"] {
        let repo = root.join(name);
        fs::create_dir_all(&repo).unwrap();
        git(&repo, &["init"]);
    }
    let ledger = root.join("ledger");
    fs::create_dir_all(&ledger).unwrap();
    let locks = ledger.join("locks.ndjson");
    let sessions = ledger.join("sessions.ndjson");
    let trail = root.join("trail.ndjson");
    fs::write(&trail, "").unwrap();
    Fx { root, locks, sessions, trail, _guard: guard }
}

fn acquired_line(path: &str, sid: &str) -> String {
    format!(
        "{{\"acquired_at\": \"2026-09-03T10:31:11+00:00\", \"event\": \"acquired\", \"path\": \"{path}\", \"session_id\": \"{sid}\", \"tool\": {{\"name\": \"locks\", \"version\": \"0.1.0\"}}}}\n"
    )
}

fn issued_line(sid: &str, package: &str) -> String {
    format!(
        "{{\"event\": \"issued\", \"package\": \"{package}\", \"session_id\": \"{sid}\", \"tool\": {{\"name\": \"lease\", \"version\": \"1.13.0\"}}}}\n"
    )
}

const AT: &str = "2026-09-18";

fn run_json(fx: &Fx) -> (i32, serde_json::Value, String) {
    let r = fx.root.to_string_lossy().into_owned();
    let l = fx.locks.to_string_lossy().into_owned();
    let t = fx.trail.to_string_lossy().into_owned();
    let (code, out, err) = run(&[
        "check", "--at", AT, "--root", &r, "--locks", &l, "--trail", &t, "--json",
    ]);
    let v = serde_json::from_str(&out).unwrap_or_else(|e| {
        panic!("stdout 须为 JSON 对象（exit={code}，err={err}，out={out}）：{e}")
    });
    (code, v, err)
}

// ---------- 绿形一：冗余会话入清单（红：改造前 --json 面不存在即失败） ----------

#[test]
fn t1_redundant_session_listed_in_json() {
    let fx = build_fx();
    // 会话 aaaa：p1 五笔（冗余 4 超阈）+ p2 一笔；会话 bbbb：十二路径各一笔零冗余
    let mut locks = String::new();
    for _ in 0..5 {
        locks += &acquired_line("p1", "aaaa");
    }
    locks += &acquired_line("p2", "aaaa");
    for i in 0..12 {
        locks += &acquired_line(&format!("q{i}"), "bbbb");
    }
    fs::write(&fx.locks, locks).unwrap();
    fs::write(&fx.sessions, issued_line("aaaa", "hyg-test-batch")).unwrap();

    let (code, v, _err) = run_json(&fx);
    assert_eq!(code, 0, "净态根退出码零");
    let key = v
        .get("redundant_acquires")
        .expect("冗余在册时 JSON 须出 redundant_acquires 键");
    let list = key.as_array().expect("redundant_acquires 须为数组");
    assert_eq!(list.len(), 1, "仅 aaaa 超阈，{list:?}");
    let e = &list[0];
    assert_eq!(e["session"], "aaaa");
    assert_eq!(e["package"], "hyg-test-batch", "批名 package 自 sessions 台账反查");
    assert_eq!(e["max_repeat_acquires"], 5, "单路径最多 acquired 笔数（§1.4 口径）");
    assert_eq!(e["redundant_total"], 4, "Σ(笔数−1)");
}

// ---------- 绿形二：零冗余对照组——键不出（向后兼容） ----------

#[test]
fn t2_zero_redundancy_key_absent() {
    let fx = build_fx();
    let mut locks = String::new();
    for i in 0..10 {
        locks += &acquired_line(&format!("p{i}"), "cccc");
    }
    locks += &acquired_line("p0", "dddd");
    fs::write(&fx.locks, locks).unwrap();

    let r = fx.root.to_string_lossy().into_owned();
    let l = fx.locks.to_string_lossy().into_owned();
    let t = fx.trail.to_string_lossy().into_owned();
    // JSON 面：零冗余时不出键
    let (code, out, err) = run(&[
        "check", "--at", AT, "--root", &r, "--locks", &l, "--trail", &t, "--json",
    ]);
    assert_eq!(code, 0, "净态根退出码零，err={err}");
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(
        v.get("redundant_acquires").is_none(),
        "零冗余时不出 redundant_acquires 键（向后兼容），{v:?}"
    );
    // 文本面：零冗余时不呈报卫生节
    let (code2, out2, _err2) = run(&["check", "--at", AT, "--root", &r, "--locks", &l, "--trail", &t]);
    assert_eq!(code2, 0);
    assert!(!out2.contains("冗余"), "零冗余文本面零提及，out2={out2}");
}

// ---------- 绿形三：阈值边界——冗余恰 3 不呈报，4 才呈报 ----------

#[test]
fn t3_threshold_boundary() {
    // 恰 4 笔（冗余 3，不超阈）→ 不出键
    let fx = build_fx();
    let mut locks = String::new();
    for _ in 0..4 {
        locks += &acquired_line("p1", "eeee");
    }
    fs::write(&fx.locks, locks).unwrap();
    let (_, v, _) = run_json(&fx);
    assert!(v.get("redundant_acquires").is_none(), "冗余恰 3 不得呈报，{v:?}");

    // 5 笔（冗余 4，超阈）→ 出键
    let fx = build_fx();
    let mut locks = String::new();
    for _ in 0..5 {
        locks += &acquired_line("p1", "ffff");
    }
    fs::write(&fx.locks, locks).unwrap();
    let (_, v, _) = run_json(&fx);
    let list = v["redundant_acquires"].as_array().expect("冗余 4 须呈报");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0]["redundant_total"], 4);
}

// ---------- 绿形四：文本面呈报形与 package 缺席兜底 ----------

#[test]
fn t4_text_face_reports_and_package_fallback() {
    let fx = build_fx();
    let mut locks = String::new();
    for _ in 0..6 {
        locks += &acquired_line("p1", "gggg");
    }
    fs::write(&fx.locks, locks).unwrap();
    // sessions 台账缺席：package 兜底「未知」，不异常
    let r = fx.root.to_string_lossy().into_owned();
    let l = fx.locks.to_string_lossy().into_owned();
    let t = fx.trail.to_string_lossy().into_owned();
    let (code, out, err) = run(&["check", "--at", AT, "--root", &r, "--locks", &l, "--trail", &t]);
    assert_eq!(code, 0, "净态根退出码零（卫生读数只报不裁），err={err}");
    assert!(
        out.contains("冗余重复加锁台账卫生读数"),
        "文本面须呈报卫生节，out={out}"
    );
    assert!(out.contains("session=gggg"), "会话号在列，out={out}");
    assert!(out.contains("package=未知"), "package 缺席兜底，out={out}");
    assert!(out.contains("只报不裁"), "呈报不代裁语义在文面");

    // 有 sessions 台账时 package 反查
    fs::write(&fx.sessions, issued_line("gggg", "pkg-lookup")).unwrap();
    let (_, out, _) = run(&["check", "--at", AT, "--root", &r, "--locks", &l, "--trail", &t]);
    assert!(out.contains("package=pkg-lookup"), "package 反查，out={out}");
}
