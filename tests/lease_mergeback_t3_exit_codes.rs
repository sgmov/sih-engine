//! T3 退出码三值全表（SPEC-024 验收判据 A2）：0 成、1 拦、2 工具异常。
//! 红态：引擎 lease 二进制未建；绿态即三值全表对齐金向量与围堰实测。

use std::path::PathBuf;
use std::process::Command;

fn engine_lease() -> Command {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("target/debug/lease");
    Command::new(p)
}

#[test]
fn t3_status_exit_zero() {
    let out = engine_lease()
        .arg("status")
        .output()
        .expect("TDD 红态：引擎 lease 二进制未建（src/bin/lease.rs 缺席）");
    assert_eq!(out.status.code(), Some(0), "status 绿态退出码 0");
}
