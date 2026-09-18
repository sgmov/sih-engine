//! defectwave 批缺陷一回归：lease open 正身/意图件缺席零校验即空哈希签发。
//! 病灶申报出处：sih/event/plan/usedpaths-materials/usedpaths-results.md 偏差
//! 申报节（2026-09-18 usedpaths 批首会话 b2a65416071e1f06：identity 与 intent
//! 文件路径已删后 open 仍成功签发，回执 record_sha256 即空串 sha
//! e3b0c442...、identity_hash 空）。
//! 修法面：cmd_open 一切副作用（目录骨架、worktree add、台账写、账单）之前
//! 加正身与意图件闸——存在、非空、JSON 可解析、identity_hash 非空（同义键
//! /identity/identity_hash 与 /identity/hash 对齐既有取哈希面）；任一不满足
//! die(2) 零残留。承 openhyg 闸序纪律（前五位只读化在前副作用在后）。

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

const STEM: &str = "defectwave";

fn git(dir: &Path, args: &[&str]) {
    let o = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git spawn");
    assert!(
        o.status.success(),
        "git {:?}: {}",
        args,
        String::from_utf8_lossy(&o.stderr)
    );
}

fn init_repo(dir: &Path) {
    let _ = Command::new("git").arg("init").arg("-b").arg("master").arg(dir).output();
    git(dir, &["config", "user.email", "t@t"]);
    git(dir, &["config", "user.name", "t"]);
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-m", "base"]);
}

struct Domain {
    root: PathBuf,
    ledger: PathBuf,
    locks: PathBuf,
    bills: PathBuf,
}

fn make_domain(tag: &str) -> Domain {
    let root = std::env::temp_dir().join(format!("dwopengate-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let engine = root.join("sih-engine");
    std::fs::create_dir_all(engine.join("sih/state/plan")).unwrap();
    std::fs::write(engine.join("README.md"), "base\n").unwrap();
    std::fs::write(
        engine.join(format!("sih/state/plan/{}.md", STEM)),
        "# defectwave：open 闸 fixture 任务包\n\n## 请求写入 {#requested-writes}\n\n（本批零声明）\n",
    )
    .unwrap();
    init_repo(&engine);
    let tools = root.join("sih-tools");
    std::fs::create_dir_all(&tools).unwrap();
    std::fs::write(tools.join("placeholder.txt"), "base\n").unwrap();
    init_repo(&tools);
    let dir = root.join("sih-tools/lease/ledger");
    std::fs::create_dir_all(&dir).unwrap();
    let ledger = dir.join("sessions.ndjson");
    let locks = dir.join("locks.ndjson");
    let bills = dir.join("bills.ndjson");
    for p in [&ledger, &locks, &bills] {
        std::fs::write(p, "").unwrap();
    }
    Domain { root, ledger, locks, bills }
}

/// open 调用形（三台账显式全传），identity 与 intent 路径由参传入（可指向
/// 缺席路径模拟件已被删形）。
fn run_open(d: &Domain, identity: &Path, intent: &Path) -> (i32, String, String) {
    let o = Command::new(env!("CARGO_BIN_EXE_lease"))
        .args([
            "open".to_string(),
            "--package".to_string(),
            STEM.to_string(),
            "--at".to_string(),
            "2026-09-18T00:00:00+00:00".to_string(),
            "--identity".to_string(),
            identity.display().to_string(),
            "--intent".to_string(),
            intent.display().to_string(),
            "--repo".to_string(),
            "sih-engine".to_string(),
            "--root".to_string(),
            d.root.display().to_string(),
            "--ledger".to_string(),
            d.ledger.display().to_string(),
            "--locks".to_string(),
            d.locks.display().to_string(),
            "--bills".to_string(),
            d.bills.display().to_string(),
        ])
        .output()
        .expect("lease spawn");
    (
        o.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
    )
}

fn valid_identity_bytes() -> Vec<u8> {
    json!({
        "anomalies": [],
        "identity": {
            "core_hash": "fixturecorehash0000000000000000000000000000000000000000000000000000000000",
            "identity_hash": "fixtureidentityhash00000000000000000000000000000000000000000000000000000"
        }
    })
    .to_string()
    .into_bytes()
}

fn write_input(d: &Domain, name: &str, bytes: &[u8]) -> PathBuf {
    let p = d.root.join(name);
    std::fs::write(&p, bytes).unwrap();
    p
}

/// 残留核算：sessions 台账零新行（文件保持空），worktrees 零新建（目录不存）。
fn assert_zero_residue(d: &Domain) {
    let ledger_text = std::fs::read_to_string(&d.ledger).unwrap();
    assert!(
        ledger_text.trim().is_empty(),
        "sessions 台账须零新行：{}",
        ledger_text
    );
    assert!(
        !d.root.join("worktrees").exists(),
        "worktrees 零新建：{}",
        d.root.join("worktrees").display()
    );
}

/// 缺正身件（路径已删形）→ exit 2 零残留。
#[test]
fn open_absent_identity_rejected_exit2_zero_residue() {
    let d = make_domain("absentid");
    let intent = write_input(&d, "intent.json", b"{\"anchors\": []}");
    let (code, out, err) = run_open(&d, &d.root.join("absent-identity.json"), &intent);
    assert_eq!(code, 2, "缺席正身件须 exit 2：out={} err={}", out, err);
    assert!(!err.trim().is_empty(), "拒收报文须在案（stderr）");
    assert_zero_residue(&d);
    let _ = std::fs::remove_dir_all(&d.root);
}

/// 缺意图件（路径已删形）→ exit 2 零残留。
#[test]
fn open_absent_intent_rejected_exit2_zero_residue() {
    let d = make_domain("absentintent");
    let identity = write_input(&d, "identity.json", &valid_identity_bytes());
    let (code, out, err) = run_open(&d, &identity, &d.root.join("absent-intent.json"));
    assert_eq!(code, 2, "缺席意图件须 exit 2：out={} err={}", out, err);
    assert!(!err.trim().is_empty(), "拒收报文须在案（stderr）");
    assert_zero_residue(&d);
    let _ = std::fs::remove_dir_all(&d.root);
}

/// 正身件 JSON 可解析但 identity_hash 缺席（同义键俱无）→ exit 2 零残留。
#[test]
fn open_identity_without_hash_rejected_exit2() {
    let d = make_domain("nohash");
    let identity = write_input(&d, "identity-nohash.json", b"{\"identity\": {\"core_hash\": \"x\"}}");
    let intent = write_input(&d, "intent.json", b"{\"anchors\": []}");
    let (code, out, err) = run_open(&d, &identity, &intent);
    assert_eq!(code, 2, "identity_hash 缺席须 exit 2：out={} err={}", out, err);
    assert_zero_residue(&d);
    let _ = std::fs::remove_dir_all(&d.root);
}

/// 意图件非 JSON → exit 2 零残留。
#[test]
fn open_intent_malformed_json_rejected_exit2() {
    let d = make_domain("badintent");
    let identity = write_input(&d, "identity.json", &valid_identity_bytes());
    let intent = write_input(&d, "intent-bad.json", b"not-json{{{");
    let (code, out, err) = run_open(&d, &identity, &intent);
    assert_eq!(code, 2, "意图件不可解析须 exit 2：out={} err={}", out, err);
    assert_zero_residue(&d);
    let _ = std::fs::remove_dir_all(&d.root);
}

/// 对照组（防过杀）：正身与意图件俱在且合规，open 照常签发，同义键 /identity/hash
/// 形亦放行（对齐既有取哈希面）。
#[test]
fn open_valid_inputs_still_issued_control() {
    let d = make_domain("control");
    let identity = write_input(&d, "identity.json", &valid_identity_bytes());
    let intent = write_input(&d, "intent.json", b"{\"anchors\": []}");
    let (code, out, err) = run_open(&d, &identity, &intent);
    assert_eq!(code, 0, "合规输入须照常签发：out={} err={}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(r["event"], "issued");
    assert!(
        !r["intent"]["record_sha256"].as_str().unwrap_or("").is_empty(),
        "record_sha256 非空（缺陷病灶面）"
    );
    let ledger_rows = std::fs::read_to_string(&d.ledger).unwrap().lines().count();
    assert_eq!(ledger_rows, 1, "台账恰一签发行");
    let _ = std::fs::remove_dir_all(&d.root);
}
