//! identity-fix 批端到端回归：无盐 verify 必须限时完成。
//! 修前 fs::read("/dev/urandom") 无限读致挂死被杀（退出码 137，证据
//! evidence-137.md），本测试以 15 秒超时闸承载回归位：旧病回流即红，
//! 不挂死套件。

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// 跑一次无盐 verify，15 秒超时闸到即杀子进程并红（回归显式暴露不静默）。
fn run_verify() -> (i32, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_identity"))
        .args(["verify", "--no-net"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn identity");
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        match child.try_wait().expect("wait identity") {
            Some(status) => {
                let out = child.wait_with_output().expect("output");
                return (
                    status.code().unwrap_or(-1),
                    String::from_utf8_lossy(&out.stdout).to_string(),
                );
            }
            None => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    panic!("identity verify 无盐径超时未返：137 回归");
                }
                std::thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

#[test]
fn t1_no_salt_verify_completes_with_valid_salt() {
    let (code, stdout) = run_verify();
    assert_eq!(code, 0, "verify 退出码非零: {stdout}");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("报告非 JSON");
    let salt = v["identity"]["salt"].as_str().expect("盐缺席");
    assert_eq!(salt.len(), 64);
    assert!(salt.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn t2_consecutive_runs_produce_different_salts() {
    let (_, a) = run_verify();
    let (_, b) = run_verify();
    let sa: String = serde_json::from_str::<serde_json::Value>(&a).unwrap()["identity"]["salt"]
        .as_str()
        .unwrap()
        .to_string();
    let sb: String = serde_json::from_str::<serde_json::Value>(&b).unwrap()["identity"]["salt"]
        .as_str()
        .unwrap()
        .to_string();
    assert_ne!(sa, sb, "连续两跑盐相同，生产盐随机性丢失");
}
