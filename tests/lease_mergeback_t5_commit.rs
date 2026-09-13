//! 腿二 T5：commit 四验与 message 三形态与退出码集成测试（lease-commitlaw-parallel 批）。
//! fixture 域自足构建，回执键面与报文形态与拒绝信封逐字节对表围堰语义（SPEC-024 A2）。

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

fn git(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let o = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git spawn");
    (
        o.status.success(),
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
    )
}

struct Domain {
    root: PathBuf,
    ledger: PathBuf,
    locks: PathBuf,
    bills: PathBuf,
    trail: PathBuf,
}

fn make_domain(tag: &str) -> Domain {
    let root = std::env::temp_dir().join(format!("leg2-t5-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let repo = root.join("sih-engine");
    std::fs::create_dir_all(&repo).unwrap();
    let _ = Command::new("git")
        .arg("init")
        .arg("-b")
        .arg("master")
        .arg(&repo)
        .output();
    git(&repo, &["config", "user.email", "t@t"]);
    git(&repo, &["config", "user.name", "t"]);
    std::fs::write(repo.join("README.md"), "base\n").unwrap();
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-m", "base"]);

    std::fs::create_dir_all(root.join("sih/state/plan")).unwrap();
    std::fs::create_dir_all(root.join("sih/ledger")).unwrap();
    std::fs::create_dir_all(root.join("sih/event/trail")).unwrap();
    std::fs::write(
        root.join("sih/state/plan/goldc.md"),
        concat!(
            "# goldc：腿二金向量任务包\n\n",
            "## 请求写入 {#requested-writes}\n\n",
            "- sih-engine/src/lib_x.rs\n",
            "- sih-engine/sih/state/plan/goldc.md\n",
            "- sih-engine/sih/event/trail/\n",
        ),
    )
    .unwrap();
    std::fs::write(root.join("sih/ledger/sessions.ndjson"), "").unwrap();
    std::fs::write(root.join("sih/ledger/locks.ndjson"), "").unwrap();
    std::fs::write(root.join("sih/ledger/bills.ndjson"), "").unwrap();
    Domain {
        trail: root.join("sih/event/trail"),
        ledger: root.join("sih/ledger/sessions.ndjson"),
        locks: root.join("sih/ledger/locks.ndjson"),
        bills: root.join("sih/ledger/bills.ndjson"),
        root,
    }
}

fn run_lease(domain: &Domain, args: &[&str]) -> (i32, String, String) {
    let base = [
        "--root".to_string(),
        domain.root.display().to_string(),
        "--ledger".to_string(),
        domain.ledger.display().to_string(),
        "--locks".to_string(),
        domain.locks.display().to_string(),
        "--bills".to_string(),
        domain.bills.display().to_string(),
    ];
    let o = Command::new(env!("CARGO_BIN_EXE_lease"))
        .args(args)
        .args(&base)
        .output()
        .expect("lease spawn");
    (
        o.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
    )
}

fn open_session(domain: &Domain) -> (String, PathBuf) {
    let (code, out, err) = run_lease(domain, &["open", "--package", "goldc", "--at", "2026-09-13"]);
    assert_eq!(code, 0, "open failed: {} {}", out, err);
    let receipt: Value = serde_json::from_str(&out).unwrap();
    let sid = receipt["session_id"].as_str().unwrap().to_string();
    let worktree = PathBuf::from(
        receipt["repos"][0]["worktree"]
            .as_str()
            .unwrap()
            .to_string(),
    );
    (sid, worktree)
}

fn stage_file(worktree: &Path, rel: &str, text: &str) {
    let full = worktree.join(rel);
    std::fs::create_dir_all(full.parent().unwrap()).unwrap();
    std::fs::write(full, text).unwrap();
    let (ok, _, err) = git(worktree, &["add", rel]);
    assert!(ok, "git add: {}", err);
}

#[test]
fn t5_wip_message_form_and_receipt() {
    let d = make_domain("wip");
    let (sid, wt) = open_session(&d);
    stage_file(&wt, "src/lib_x.rs", "// SPEC-024 腿二\n");
    let (code, out, err) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            wt.to_str().unwrap(),
            "--stage",
            "wip",
            "--subject",
            "初次落笔",
            "--session",
            &sid,
        ],
    );
    assert_eq!(code, 0, "{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(r["stage"], json!("wip"));
    assert_eq!(r["package"], json!("goldc"));
    assert_eq!(r["session_id"], json!(sid));
    assert_eq!(
        r["checks"],
        json!(["session_active", "staged_in_scope"])
    );
    assert_eq!(
        r["message"],
        json!(format!("goldc wip 初次落笔\n\nsession: {}\n", sid))
    );
    assert!(r["commit"].as_str().unwrap().len() >= 7);
    assert!(r["base"].as_str().unwrap().starts_with("master@"));
}

#[test]
fn t5_wip_dedup_subject_prefix() {
    let d = make_domain("dedup");
    let (sid, wt) = open_session(&d);
    stage_file(&wt, "src/lib_x.rs", "x\n");
    let (code, out, err) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            wt.to_str().unwrap(),
            "--stage",
            "wip",
            "--subject",
            "goldc wip 重复前缀落笔",
            "--session",
            &sid,
        ],
    );
    assert_eq!(code, 0, "{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        r["message"],
        json!(format!("goldc wip 重复前缀落笔\n\nsession: {}\n", sid)),
        "subject 同字样前缀须剥去防批名双现（pk-097 其二）"
    );
}

#[test]
fn t5_nothing_staged_and_scope_and_worktree_gate() {
    let d = make_domain("gates");
    let (sid, wt) = open_session(&d);
    // 空暂存
    let (code, out, _) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            wt.to_str().unwrap(),
            "--stage",
            "wip",
            "--subject",
            "空",
            "--session",
            &sid,
        ],
    );
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(e["error"], json!("nothing_staged"));
    assert!(e.get("detail").map(|v| v.is_null()).unwrap_or(false));

    // 范围外暂存
    stage_file(&wt, "src/other.rs", "y\n");
    let (code, out, _) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            wt.to_str().unwrap(),
            "--stage",
            "wip",
            "--subject",
            "越界",
            "--session",
            &sid,
        ],
    );
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(e["error"], json!("staged_out_of_scope"));
    assert_eq!(
        e["detail"]["outside"],
        json!(["sih-engine/src/other.rs"])
    );

    // 主检出直提拒（commit 须指围堰，pk-031 证据一）
    let main_repo = d.root.join("sih-engine");
    stage_file(&main_repo, "src/lib_x.rs", "z\n");
    let (code, out, _) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            main_repo.to_str().unwrap(),
            "--stage",
            "wip",
            "--subject",
            "主检出",
            "--session",
            &sid,
        ],
    );
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(e["error"], json!("commit_must_target_worktree"));
    assert_eq!(e["detail"]["worktree"].as_str().unwrap(), wt.to_str().unwrap());

    // 会话不在册
    let (code, out, _) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            wt.to_str().unwrap(),
            "--stage",
            "wip",
            "--subject",
            "无会话",
            "--session",
            "ffffffffffffffff",
        ],
    );
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(e["error"], json!("session_not_active"));
    assert_eq!(e["detail"]["wanted"], json!("ffffffffffffffff"));
}

#[test]
fn t5_settle_usage_and_cert_gate() {
    let d = make_domain("settle");
    let (sid, wt) = open_session(&d);
    stage_file(&wt, "src/lib_x.rs", "// SPEC-024 腿二\n");

    // settle 缺 --seq → exit 2
    let (code, out, _) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            wt.to_str().unwrap(),
            "--stage",
            "settle",
            "--subject",
            "缺seq",
            "--session",
            &sid,
        ],
    );
    assert_eq!(code, 2);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(e["error"], json!("settle requires --seq"));

    // settle 缺 --cert → exit 2
    let (code, out, _) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            wt.to_str().unwrap(),
            "--stage",
            "settle",
            "--subject",
            "缺cert",
            "--seq",
            "1",
            "--session",
            &sid,
        ],
    );
    assert_eq!(code, 2);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(e["error"], json!("settle requires --cert"));

    // cert 不在链 → exit 1
    let (code, out, _) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            wt.to_str().unwrap(),
            "--stage",
            "settle",
            "--subject",
            "假证",
            "--seq",
            "1",
            "--cert",
            "deadbeef",
            "--session",
            &sid,
        ],
    );
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(e["error"], json!("cert_not_on_chain"));
    assert_eq!(e["detail"]["cert"], json!("deadbeef"));
}

#[test]
fn t5_settle_success_with_on_chain_cert() {
    let d = make_domain("settle-ok");
    let (sid, wt) = open_session(&d);
    stage_file(&wt, "src/lib_x.rs", "// SPEC-024 腿二\n");

    // 链上落认证笔取真 cert
    let report = d.root.join("cert-report.json");
    std::fs::write(&report, "{\"note\": \"t5 settle cert\"}").unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_scribe"))
        .args([
            "append",
            "--report",
            report.to_str().unwrap(),
            "--exit-code",
            "0",
            "--trail",
            d.trail.join("2026-09-13.ndjson").to_str().unwrap(),
            "--locks",
            d.locks.to_str().unwrap(),
            "--sessions",
            d.ledger.to_str().unwrap(),
            "--session",
            &sid,
        ])
        .output()
        .expect("scribe spawn");
    assert!(o.status.success(), "scribe append: {}", String::from_utf8_lossy(&o.stderr));
    let scribe_out: Value = serde_json::from_str(&String::from_utf8_lossy(&o.stdout)).unwrap();
    let full_hash = scribe_out["event_hash"].as_str().unwrap().to_string();
    let cert = &full_hash[..8];

    let (code, out, err) = run_lease(
        &d,
        &[
            "commit",
            "--repo",
            wt.to_str().unwrap(),
            "--stage",
            "settle",
            "--subject",
            "段结算落笔",
            "--seq",
            "1",
            "--cert",
            cert,
            "--session",
            &sid,
        ],
    );
    assert_eq!(code, 0, "{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(r["stage"], json!("settle"));
    assert_eq!(
        r["checks"],
        json!(["session_active", "staged_in_scope", "cert_on_chain"])
    );
    assert!(r["message"].as_str().unwrap().contains(&format!("goldc 段1 段结算落笔\n\nsession: {} cert: {} base: master@", sid, cert)));
    // settle 回执附 gauge 读数，fixture 域秤星缺席即 skipped 注记
    assert_eq!(r["gauge"]["skipped"], json!(true));
    assert_eq!(r["gauge"]["reason"], json!("missing"));
}
