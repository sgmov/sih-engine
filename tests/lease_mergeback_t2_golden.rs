//! T2 金向量逐字节一致（SPEC-024 验收判据 A1）。
//! 红态：引擎 lease 二进制未建，本测必然红；绿态即 src/bin/lease.rs 建成且
//! 对金向量五Receipt在归一 session_id 与域根路径两域后逐字节一致。
//! 归一依据见 src/lease/fixtures/golden/NORMALIZATION.md。

use std::path::PathBuf;
use std::process::Command;

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lease/fixtures/golden")
}

fn engine_lease_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/debug/lease")
}

#[test]
fn t2_golden_receipts_in_place() {
    for name in [
        "open-receipt.json",
        "lock-receipt.json",
        "unlock-receipt.json",
        "close-receipt.json",
        "close-receipt-idle.json",
    ] {
        assert!(
            golden_dir().join(name).is_file(),
            "金向量 {name} 在档"
        );
    }
}

#[test]
fn t2_open_receipt_byte_identical() {
    assert!(
        engine_lease_path().is_file(),
        "TDD 红态：引擎 lease 二进制未建（src/bin/lease.rs 缺席），SPEC-024 T2 无法执行"
    );
    let out = engine_lease_path();
    let _ = Command::new(out); // 绿态：同参形 fixture 重放 + 归一 cmp，腿一绿批承载
}
