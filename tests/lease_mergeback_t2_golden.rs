//! T2 金向量逐字节一致（SPEC-024 验收判据 A1）。
//! 绿态判据：引擎 lease 对金向量五Receipt在归一 session_id 与域根路径两域后逐字节一致。
//! 归一依据见 src/lease/fixtures/golden/NORMALIZATION.md。

use std::path::PathBuf;
use std::process::Command;

fn manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn lease_bin() -> PathBuf {
    manifest().join("target/debug/lease")
}
fn golden_dir() -> PathBuf {
    manifest().join("src/lease/fixtures/golden")
}
const GOLDEN_ROOT: &str = "/Users/moc/workspaces/SiHankor/work/lease-fixtures/domain1";
const GOLDEN_SID: &str = "29869ed8bd9e35ac";

fn git(dir: &PathBuf, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git 可用");
    assert!(out.status.success(), "git {:?} 失败: {}", args, String::from_utf8_lossy(&out.stderr));
}

fn make_domain(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("lease-t2-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    let eng = d.join("sih-engine");
    std::fs::create_dir_all(eng.join("sih/state/plan")).unwrap();
    git(&eng, &["init"]);
    git(&eng, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "--allow-empty", "-m", "init"]);
    for stem in ["golden-a", "golden-b"] {
        let pkg = format!(
            "# {}: fixture 净目标基形\n\n## 请求写入 {{#requested-writes}}\n\n- sih/ledger/sessions.ndjson\n- sih/ledger/locks.ndjson\n- sih/ledger/bills.ndjson\n",
            stem
        );
        std::fs::write(eng.join(format!("sih/state/plan/{}.md", stem)), pkg).unwrap();
    }
    git(&eng, &["add", "-A"]);
    git(&eng, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-m", "pkg"]);
    d
}

fn run_lease(root: &PathBuf, args: &[&str]) -> (i32, String, String) {
    let led = root.join("sih/ledger");
    let out = Command::new(lease_bin())
        .args([
            "--root", root.to_str().unwrap(),
            "--ledger", led.join("sessions.ndjson").to_str().unwrap(),
            "--locks", led.join("locks.ndjson").to_str().unwrap(),
            "--bills", led.join("bills.ndjson").to_str().unwrap(),
            "--at", "2026-09-13T00:00:00+00:00",
        ])
        .args(args)
        .output()
        .expect("引擎 lease 可执行");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

fn normalize(text: &str, root: &PathBuf, sid: &str) -> String {
    text.replace(root.to_str().unwrap(), "<ROOT>")
        .replace(sid, "<SID>")
}

fn golden(name: &str) -> String {
    normalize(
        &std::fs::read_to_string(golden_dir().join(name)).unwrap(),
        &PathBuf::from(GOLDEN_ROOT),
        GOLDEN_SID,
    )
}

fn open_args() -> [String; 4] {
    [
        "--identity".into(),
        golden_dir().join("inputs/identity.json").to_str().unwrap().into(),
        "--intent".into(),
        golden_dir().join("inputs/intent.json").to_str().unwrap().into(),
    ]
}

fn sid_of(out: &str) -> String {
    serde_json::from_str::<serde_json::Value>(out).unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string()
}

#[test]
fn t2_golden_receipts_in_place() {
    for name in [
        "open-receipt.json",
        "lock-receipt.json",
        "unlock-receipt.json",
        "close-receipt.json",
        "close-receipt-idle.json",
        "NORMALIZATION.md",
        "inputs/identity.json",
        "inputs/intent.json",
    ] {
        assert!(golden_dir().join(name).is_file(), "金向量 {} 在档", name);
    }
}

#[test]
fn t2_open_receipt_byte_identical() {
    let d = make_domain("open");
    let ia = open_args();
    let (rc, out, err) = run_lease(&d, &["open", "--package", "golden-a", &ia[0], &ia[1], &ia[2], &ia[3]]);
    assert_eq!(rc, 0, "open 退出码: {}", err);
    let sid = sid_of(&out);
    let got = normalize(&out, &d, &sid);
    assert_eq!(got, golden("open-receipt.json"), "open 回执归一后逐字节比对");
}

#[test]
fn t2_lock_unlock_receipts_byte_identical() {
    let d = make_domain("lockunlock");
    let ia = open_args();
    let (_, open_out, err) = run_lease(&d, &["open", "--package", "golden-a", &ia[0], &ia[1], &ia[2], &ia[3]]);
    assert_eq!(err, "", "open 零 stderr");
    let sid = sid_of(&open_out);
    let (rc, lock_out, _) = run_lease(&d, &["lock", "--path", "sih/ledger/sessions.ndjson", "--session", &sid]);
    assert_eq!(rc, 0);
    let (rc, unlock_out, _) = run_lease(&d, &["unlock", "--path", "sih/ledger/sessions.ndjson", "--session", &sid]);
    assert_eq!(rc, 0);
    for (got_raw, name) in [(lock_out, "lock-receipt.json"), (unlock_out, "unlock-receipt.json")] {
        let got = normalize(&got_raw, &d, &sid);
        assert_eq!(got, golden(name), "{} 归一后逐字节比对", name);
    }
}

#[test]
fn t2_close_receipt_byte_identical() {
    let d = make_domain("close");
    let ia = open_args();
    let (_, open_out, _) = run_lease(&d, &["open", "--package", "golden-a", &ia[0], &ia[1], &ia[2], &ia[3]]);
    let sid = sid_of(&open_out);
    run_lease(&d, &["lock", "--path", "sih/ledger/sessions.ndjson", "--session", &sid]);
    run_lease(&d, &["unlock", "--path", "sih/ledger/sessions.ndjson", "--session", &sid]);
    let (rc, close_out, err) = run_lease(&d, &["close", "--package", "golden-a"]);
    assert_eq!(rc, 0, "close 退出码: {}", err);
    let got = normalize(&close_out, &d, &sid);
    assert_eq!(got, golden("close-receipt.json"), "close 回执归一后逐字节比对");
}
