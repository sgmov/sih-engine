//! T4 五验与闸族语义（SPEC-024 验收判据 A3 之一）：撞锁形 locked_elsewhere 拒。

use std::path::PathBuf;
use std::process::Command;

fn manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn git(dir: &PathBuf, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().unwrap();
    assert!(out.status.success());
}

fn make_domain(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("lease-t4-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    let eng = d.join("sih-engine");
    std::fs::create_dir_all(eng.join("sih/state/plan")).unwrap();
    git(&eng, &["init"]);
    git(&eng, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "--allow-empty", "-m", "init"]);
    for stem in ["golden-a", "golden-b"] {
        let pkg = format!(
            "# {}: fixture\n\n## 请求写入 {{#requested-writes}}\n\n- sih/ledger/sessions.ndjson\n- sih/ledger/locks.ndjson\n- sih/ledger/bills.ndjson\n",
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
    let out = Command::new(manifest().join("target/debug/lease"))
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

#[test]
fn t4_collision_locked_elsewhere() {
    let d = make_domain("collide");
    let inputs = manifest().join("src/lease/fixtures/golden/inputs");
    let (_, a_out, _) = run_lease(&d, &[
        "open", "--package", "golden-a",
        "--identity", inputs.join("identity.json").to_str().unwrap(),
        "--intent", inputs.join("intent.json").to_str().unwrap(),
    ]);
    let (_, b_out, _) = run_lease(&d, &[
        "open", "--package", "golden-b",
        "--identity", inputs.join("identity.json").to_str().unwrap(),
        "--intent", inputs.join("intent.json").to_str().unwrap(),
    ]);
    let sid_a = serde_json::from_str::<serde_json::Value>(&a_out).unwrap()["session_id"]
        .as_str().unwrap().to_string();
    let sid_b = serde_json::from_str::<serde_json::Value>(&b_out).unwrap()["session_id"]
        .as_str().unwrap().to_string();
    let (rc, _, _) = run_lease(&d, &["lock", "--path", "sih/ledger/sessions.ndjson", "--session", &sid_a]);
    assert_eq!(rc, 0, "先至者持锁");
    let (rc2, _, err_b) = run_lease(&d, &["lock", "--path", "sih/ledger/sessions.ndjson", "--session", &sid_b]);
    assert_eq!(rc2, 1, "后至者被拒");
    assert!(err_b.contains("locked_elsewhere"), "撞锁理由码字面: {}", err_b);
}
