//! pathfix 批（pk-089 承接 + bypass 台账误位修复）集成测试。
//! 承载：lease bypass 缺参拒、工作区根上溯发现落正册、非工作区 fail-closed。
//! 域形基座 ledger_surface 双分支由 src/bin/lease/commitlaw.rs 内单测承载。

use std::path::{Path, PathBuf};
use std::process::Command;

fn lease() -> Command {
    Command::new(env!("CARGO_BIN_EXE_lease"))
}

fn make_ws(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pathfix-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("sih-engine/src/bin/lease")).unwrap();
    std::fs::create_dir_all(root.join("sih-tools/lease/ledger")).unwrap();
    root
}

fn row_count(path: &Path) -> usize {
    std::fs::read_to_string(path)
        .map(|s| s.lines().filter(|l| !l.trim().is_empty()).count())
        .unwrap_or(0)
}

#[test]
fn t1_bypass_missing_args_rejected() {
    // 零参与缺 sha：俱退出码二拒，零台账落笔（防空行污染）。
    let ws = make_ws("t1");
    for args in [vec!["bypass"], vec!["bypass", "--repo", "x", "--reason", "r"]] {
        let out = lease()
            .args(&args)
            .current_dir(ws.join("sih-engine"))
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(2),
            "缺参须退出码二：{:?} -> {}",
            args,
            String::from_utf8_lossy(&out.stdout)
        );
    }
    assert!(!ws.join("sih-tools/lease/ledger/bypass.ndjson").exists());
    let _ = std::fs::remove_dir_all(&ws);
}

#[test]
fn t2_bypass_root_discovery_lands_central() {
    // 不传 --root，自 cwd 上溯发现工作区根，行落中央正册位。
    let ws = make_ws("t2");
    let out = lease()
        .args([
            "bypass",
            "--repo",
            ws.join("sih-engine").to_str().unwrap(),
            "--sha",
            "deadbeef",
            "--reason",
            "pathfix-t2 根发现验证",
        ])
        .current_dir(ws.join("sih-engine/src/bin/lease"))
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "bypass 须成功：{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let ledger = ws.join("sih-tools/lease/ledger/bypass.ndjson");
    assert_eq!(row_count(&ledger), 1, "正册位恰一行");
    assert!(
        !ws.join("sih-engine/sih-tools").exists(),
        "cwd 侧不得再沉积迷路树"
    );
    let _ = std::fs::remove_dir_all(&ws);
}

#[test]
fn t3_bypass_no_workspace_fail_closed() {
    // 自非工作区目录跑且不传 --root：fail-closed 退出码二，不落 cwd 缺省。
    let bare = std::env::temp_dir().join(format!("pathfix-t3-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&bare);
    std::fs::create_dir_all(&bare).unwrap();
    let out = lease()
        .args([
            "bypass",
            "--repo",
            "/tmp/whatever",
            "--sha",
            "deadbeef",
            "--reason",
            "pathfix-t3 fail-closed 验证",
        ])
        .current_dir(&bare)
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(2),
        "非工作区须 fail-closed：{}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(!bare.join("sih-tools").exists(), "bare 位零沉积");
    let _ = std::fs::remove_dir_all(&bare);
}
