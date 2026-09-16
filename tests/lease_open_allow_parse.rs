//! gap-lease-allow-parse 端到端回归（recognize-solo 双跑偏差第二处清偿验收钉）：
//! 含散文 allow 行的包档经引擎 lease open 后，签发会话 allow 面为围堰
//! declguard-solo 冻结启发族抽取结果（core.py parse_requested_writes 对表：
//! 「——」截断取路径部加剥首尾反引号，理由散文不入锁范围；显式 allow 键
//! 同经抽取），锁范围验不再必拒。
//!
//! 对表基准：sih-tools/lease/src/lease/core.py（围堰冻结，只读参照）；
//! 同参双跑活体验收记录见批结果档（沙箱临时目录账本，真实账本零写）。

use std::path::{Path, PathBuf};
use std::process::Command;

fn manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git 可用");
    assert!(
        out.status.success(),
        "git {:?} 失败: {}",
        args,
        String::from_utf8_lossy(&out.stderr)
    );
}

/// 沙箱域：git 仓加含散文 allow 行的包档（recognize-solo 病形：
/// 反引号路径——理由散文）。
fn make_domain(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("lease-allowparse-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    let eng = d.join("sih-engine");
    std::fs::create_dir_all(eng.join("sih/state/plan")).unwrap();
    std::fs::create_dir_all(eng.join("src/mcpserver")).unwrap();
    git(&eng, &["init"]);
    git(&eng, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "--allow-empty", "-m", "init"]);
    let pkg = "# allowparse fixture\n\n## 九、请求写入 {#requested-writes}\n\n\
               - `sih-engine/src/mcpserver/httpface.rs`——识别域隔离三闸与教学载荷\n\
               - sih-engine/src/bin/lease.rs——锁范围解析位修复\n";
    std::fs::write(eng.join("sih/state/plan/allowparse-prose.md"), pkg).unwrap();
    std::fs::write(eng.join("src/mcpserver/httpface.rs"), "// fixture\n").unwrap();
    git(&eng, &["add", "-A"]);
    git(&eng, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-m", "pkg"]);
    d
}

fn run_open(root: &Path, extra_allow: &[&str]) -> (i32, serde_json::Value, String) {
    let inputs = manifest().join("src/lease/fixtures/golden/inputs");
    let led = root.join("sih/ledger");
    let mut args: Vec<String> = [
        "--root", root.to_str().unwrap(),
        "--ledger", led.join("sessions.ndjson").to_str().unwrap(),
        "--locks", led.join("locks.ndjson").to_str().unwrap(),
        "--bills", led.join("bills.ndjson").to_str().unwrap(),
        "--at", "2026-09-16T00:00:00+00:00",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    args.extend([
        "open".to_string(),
        "--package".to_string(),
        "allowparse-prose".to_string(),
        "--identity".to_string(),
        inputs.join("identity.json").display().to_string(),
        "--intent".to_string(),
        inputs.join("intent.json").display().to_string(),
    ]);
    for a in extra_allow {
        args.extend(["--allow".to_string(), a.to_string()]);
    }
    let out = Command::new(manifest().join("target/debug/lease"))
        .args(&args)
        .output()
        .expect("引擎 lease 可执行");
    (
        out.status.code().unwrap_or(-1),
        serde_json::from_str(&String::from_utf8_lossy(&out.stdout))
            .unwrap_or(serde_json::Value::Null),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

#[test]
fn open_prose_allow_lines_extract_paths_not_prose() {
    let d = make_domain("receipt");
    let (rc, receipt, err) = run_open(&d, &["sih-engine/sih/state/plan/allowparse-prose-results.md——显式补面理由散文"]);
    assert_eq!(rc, 0, "open 退出码: {}", err);
    assert_eq!(receipt["event"], "issued");
    assert_eq!(
        receipt["allow"],
        serde_json::json!([
            "sih-engine/src/mcpserver/httpface.rs",
            "sih-engine/src/bin/lease.rs",
            "sih-engine/sih/state/plan/allowparse-prose-results.md",
        ]),
        "allow 面须为路径抽取形：反引号剥净、「——」后散文不入、显式键同经抽取"
    );
    let _ = std::fs::remove_dir_all(&d);
}

#[test]
fn open_explicit_duplicate_of_package_line_dedups() {
    // 组合形对表围堰 core.py open_preflight：显式键不在 requested 者并入，
    // 与包档重复者不再入（面不重账）。
    let d = make_domain("dedup");
    let (rc, receipt, err) = run_open(&d, &["sih-engine/src/bin/lease.rs"]);
    assert_eq!(rc, 0, "open 退出码: {}", err);
    assert_eq!(
        receipt["allow"],
        serde_json::json!([
            "sih-engine/src/mcpserver/httpface.rs",
            "sih-engine/src/bin/lease.rs",
        ]),
        "显式键与包档重复即去重"
    );
    let _ = std::fs::remove_dir_all(&d);
}
