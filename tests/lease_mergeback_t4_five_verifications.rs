//! T4 五验与闸族语义（SPEC-024 验收判据 A3 之一）：撞锁形 locked_elsewhere 拒。
//! 红态：引擎 lease 二进制未建；绿态即二会话争一路径后至者拒、理由码字面对表围堰。

use std::path::PathBuf;
use std::process::Command;

fn engine_lease() -> Command {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("target/debug/lease");
    Command::new(p)
}

#[test]
fn t4_collision_locked_elsewhere() {
    let out = engine_lease()
        .args(["lock", "--path", "fixture/path"])
        .output()
        .expect("TDD 红态：引擎 lease 二进制未建（src/bin/lease.rs 缺席）");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("locked_elsewhere"),
        "撞锁形理由码 locked_elsewhere 字面对表围堰"
    );
}
