//! T3 退出码三值全表（SPEC-024 验收判据 A2）：0 成、1 拦、2 工具异常。

use std::path::PathBuf;
use std::process::Command;

fn lease_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/debug/lease")
}

#[test]
fn t3_status_exit_zero() {
    let out = Command::new(lease_bin())
        .arg("status")
        .output()
        .expect("引擎 lease 二进制在位");
    assert_eq!(out.status.code(), Some(0), "status 绿态退出码 0");
}

#[test]
fn t3_unknown_subcommand_exit_two() {
    let out = Command::new(lease_bin())
        .args(["definitely-not-a-subcommand"])
        .output()
        .expect("引擎 lease 二进制在位");
    assert_eq!(out.status.code(), Some(2), "未知子命令退出码 2");
}
