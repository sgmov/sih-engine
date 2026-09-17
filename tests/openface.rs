//! pk-104 openface-solo 端到端回归：lease open repos 面单一源化。
//! repos 推导自任务包声明面仓前缀（对表 closegate 声明差集闸 repo_entries
//! 前缀解析形 token.starts_with("<repo>/")）：声明路径以 sih-engine/ 开头
//! 即挂引擎仓、sih-tools/ 开头即挂工具仓，域外件（AGENTS.md 与
//! 远期备忘录.md 形）不推导仓；--repo 旗标保留为显式扩写位与推导面并集
//!（组合序对表 compose_allow：推导面原序保留，旗标不在面者按序并入）；
//! 推导面空（声明全无仓前缀）回落缺省 sih-engine（腿一金向量兼容形）。

use std::path::{Path, PathBuf};
use std::process::Command;

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

fn init_repo(dir: &Path) {
    let _ = Command::new("git")
        .arg("init")
        .arg("-b")
        .arg("master")
        .arg(dir)
        .output();
    git(dir, &["config", "user.email", "t@t"]);
    git(dir, &["config", "user.name", "t"]);
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-m", "base"]);
}

/// 沙箱域：双 git 仓（sih-engine 与 sih-tools，工地挂载面前置）加包档
///（requested 面由参传入）。返回 canonical 域根（对表回执内 py_resolve 面）。
fn make_domain(tag: &str, stem: &str, requested: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("openface-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    let eng = d.join("sih-engine");
    std::fs::create_dir_all(eng.join("sih/state/plan")).unwrap();
    std::fs::write(eng.join("README.md"), "base\n").unwrap();
    init_repo(&eng);
    let tools = d.join("sih-tools");
    std::fs::create_dir_all(tools.join("lease/src/lease")).unwrap();
    std::fs::write(tools.join("core.py"), "# base\n").unwrap();
    init_repo(&tools);
    let pkg = format!(
        "# {}：openface repos 面单一源化 fixture\n\n## 请求写入 {{#requested-writes}}\n\n{}\n",
        stem, requested
    );
    std::fs::write(eng.join(format!("sih/state/plan/{}.md", stem)), pkg).unwrap();
    git(&eng, &["add", "-A"]);
    git(&eng, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-m", "pkg"]);
    std::fs::canonicalize(&d).unwrap()
}

fn run_open(root: &Path, stem: &str, repo_flags: &[&str]) -> (i32, serde_json::Value, String) {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let inputs = manifest.join("src/lease/fixtures/golden/inputs");
    let led = root.join("sih/ledger");
    let mut args: Vec<String> = [
        "--root", root.to_str().unwrap(),
        "--ledger", &led.join("sessions.ndjson").display().to_string(),
        "--locks", &led.join("locks.ndjson").display().to_string(),
        "--bills", &led.join("bills.ndjson").display().to_string(),
        "--at", "2026-09-17T00:00:00+00:00",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    args.extend([
        "open".to_string(),
        "--package".to_string(),
        stem.to_string(),
        "--identity".to_string(),
        inputs.join("identity.json").display().to_string(),
        "--intent".to_string(),
        inputs.join("intent.json").display().to_string(),
    ]);
    for r in repo_flags {
        args.extend(["--repo".to_string(), (*r).to_string()]);
    }
    let out = Command::new(env!("CARGO_BIN_EXE_lease"))
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

/// 回执 repos 面仓名抽取（repo 字段 file_name 位）。
fn repo_names(receipt: &serde_json::Value) -> Vec<String> {
    receipt["repos"]
        .as_array()
        .expect("repos 面在回执")
        .iter()
        .map(|e| {
            PathBuf::from(e["repo"].as_str().unwrap_or(""))
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default()
        })
        .collect()
}

/// A 双仓推导（红测主证）：包档声明 sih-engine/ 与 sih-tools/ 双前缀加两件
/// 域外件，不传 --repo，open 后回执 repos 应含双仓且域外件不增仓。
#[test]
fn open_derives_dual_repos_from_package_declarations() {
    let stem = "openface-dual";
    let d = make_domain(
        "dual",
        stem,
        concat!(
            "- sih-engine/src/bin/lease.rs\n",
            "- sih-tools/lease/src/lease/core.py\n",
            "- AGENTS.md——域外件不推导仓\n",
            "- 远期备忘录.md\n",
        ),
    );
    let (rc, receipt, err) = run_open(&d, stem, &[]);
    assert_eq!(rc, 0, "open 退出码: {}", err);
    assert_eq!(receipt["event"], "issued");
    assert_eq!(
        receipt["allow"],
        serde_json::json!([
            "sih-engine/src/bin/lease.rs",
            "sih-tools/lease/src/lease/core.py",
            "AGENTS.md",
            "远期备忘录.md",
        ]),
        "声明面四件全入 allow（域外件在内，散文不入）"
    );
    assert_eq!(
        repo_names(&receipt),
        vec!["sih-engine".to_string(), "sih-tools".to_string()],
        "repos 须为声明面仓前缀推导的双仓（域外件不推导仓）"
    );
    for e in receipt["repos"].as_array().unwrap() {
        let rn = PathBuf::from(e["repo"].as_str().unwrap())
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        assert_eq!(
            e["branch"],
            serde_json::json!(format!("msh/{}", stem)),
            "branch 沿用 msh/<stem> 形"
        );
        let wt = e["worktree"].as_str().unwrap();
        assert_eq!(
            wt,
            d.join("worktrees").join(&rn).join(stem).display().to_string(),
            "worktree 位沿现形 worktrees/<仓>/<stem>"
        );
        assert!(Path::new(wt).join(".git").exists(), "git worktree 实建: {}", wt);
    }
    let _ = std::fs::remove_dir_all(&d);
}

/// B 旗标并集（红测副证）：包档只声明引擎仓件，--repo sih-tools 显式扩写
/// 工具仓，repos 应为推导面与旗标面并集（旧形纯旗标会丢引擎仓）。
#[test]
fn open_repo_flag_unions_with_derived_face() {
    let stem = "openface-union";
    let d = make_domain("union", stem, "- sih-engine/src/lib_u.rs\n");
    let (rc, receipt, err) = run_open(&d, stem, &["sih-tools"]);
    assert_eq!(rc, 0, "open 退出码: {}", err);
    let mut names = repo_names(&receipt);
    names.sort();
    assert_eq!(
        names,
        vec!["sih-engine".to_string(), "sih-tools".to_string()],
        "--repo 旗标与推导面并集（非替换）"
    );
    let _ = std::fs::remove_dir_all(&d);
}

/// B' 旗标与推导面重合去重：双仓已推导，重复旗标不重账（组合序对表
/// compose_allow 面不重账）。
#[test]
fn open_repo_flag_duplicate_of_derived_dedups() {
    let stem = "openface-dedup";
    let d = make_domain(
        "dedup",
        stem,
        "- sih-engine/src/lib_d.rs\n- sih-tools/lease/src/x.py\n",
    );
    let (rc, receipt, err) = run_open(&d, stem, &["sih-engine", "sih-engine"]);
    assert_eq!(rc, 0, "open 退出码: {}", err);
    assert_eq!(
        repo_names(&receipt),
        vec!["sih-engine".to_string(), "sih-tools".to_string()],
        "重复旗标与推导面去重，面不重账"
    );
    let _ = std::fs::remove_dir_all(&d);
}

/// C 缺省回落（兼容钉）：声明面全无仓前缀（裸 sih 面加域外件），推导面空
/// 回落缺省 sih-engine（腿一金向量 open-receipt 兼容形，t2 逐字节比对不破）。
#[test]
fn open_no_repo_prefix_declarations_fall_back_to_engine_default() {
    let stem = "openface-fallback";
    let d = make_domain(
        "fallback",
        stem,
        concat!(
            "- sih/ledger/sessions.ndjson\n",
            "- AGENTS.md\n",
            "- 远期备忘录.md\n",
        ),
    );
    let (rc, receipt, err) = run_open(&d, stem, &[]);
    assert_eq!(rc, 0, "open 退出码: {}", err);
    assert_eq!(
        repo_names(&receipt),
        vec!["sih-engine".to_string()],
        "推导面空回落缺省引擎仓（金向量兼容形）"
    );
    let _ = std::fs::remove_dir_all(&d);
}
