//! gap-identity-timeout：identity 采集子进程超时闸回归。
//!
//! 行为面对表围堰 sih-tools/identity 0.5.0 core.py _run（subprocess
//! timeout=2，TimeoutExpired 捕获返空串）：PATH 前插挂起假件（exec sleep 300
//! 的 ifconfig/ps，sysctl 不遮蔽走真件），live verify（--no-net）须在闸时限
//! 内完成、退出码三值内（非信号死即 137 形），采集链挂起不再拖垮全形；正常
//! 环境全形退出码 0/2。fixture 全 temp 自建，零真实账本写入。

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::time::Instant;

fn bin_identity() -> &'static str {
    env!("CARGO_BIN_EXE_identity")
}

/// 返（退出码 None 即信号死、stdout、stderr、墙钟秒）。
fn run_with_path(args: &[&str], path_env: Option<&str>) -> (Option<i32>, String, String, f64) {
    let start = Instant::now();
    let mut cmd = Command::new(bin_identity());
    cmd.args(args);
    if let Some(p) = path_env {
        cmd.env("PATH", p);
    }
    let out = cmd.output().unwrap();
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        start.elapsed().as_secs_f64(),
    )
}

const SALT: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

/// PATH 前插挂起假件目录：ifconfig 与 ps 均 exec /bin/sleep 300（挂起不产出），
/// sysctl 不遮蔽走真件。返（新 PATH，假件目录 guard 延寿）。
fn prepend_hanging_fake_bin() -> (String, tempfile::TempDir) {
    let guard = tempfile::TempDir::new().unwrap();
    for name in ["ifconfig", "ps"] {
        let p = guard.path().join(name);
        fs::write(&p, "#!/bin/sh\nexec /bin/sleep 300\n").unwrap();
        fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let orig = std::env::var("PATH").unwrap_or_default();
    (format!("{}:{orig}", guard.path().display()), guard)
}

/// 挂起件被闸杀：live 全形在闸时限内（mac/parent_start/ancestry 三处挂起
/// ≈3×2s）以正常退出码收束，非 137 信号死形，采集链挂起不拖垮全形。
#[test]
fn t1_hanging_collectors_killed_at_gate_live_verify_bounded() {
    let (path_env, _guard) = prepend_hanging_fake_bin();
    let (code, out, err, elapsed) = run_with_path(
        &["verify", "--no-net", "--salt", SALT, "--quiet"],
        Some(&path_env),
    );
    assert!(
        code == Some(0) || code == Some(2),
        "挂起采集须被闸杀后正常退出 0/2，实得 code={code:?}（None 即信号死 137 形）stdout={out} stderr={err}"
    );
    assert!(
        elapsed < 15.0,
        "挂起件 sleep 300 须在闸 2s×三处采集 ≈6s 内收束，实得 {elapsed}s"
    );
}

/// 正常环境全形：真 PATH 真 sysctl/ps/ifconfig，瞬回且退出码 0/2 而非 137。
#[test]
fn t2_normal_environment_full_form_exit_zero_or_two() {
    let (code, out, err, elapsed) = run_with_path(&["verify", "--no-net", "--salt", SALT], None);
    assert!(
        code == Some(0) || code == Some(2),
        "正常环境全形须 0/2 而非 137 形，实得 code={code:?} stdout={out} stderr={err}"
    );
    assert!(elapsed < 15.0, "正常全形须瞬回，实得 {elapsed}s");
}
