//! gap-mergeback-uncommitted-landing：声明差集闸落位面回归（SPEC-024/SPEC-020）。
//!
//! 既有 lease_mergeback_t6_close_gates 已对表差集闸拦截形与认领放行形与错认领拒形，
//! 本件补落位（landing）面其余分支：
//! t1 已提交落位形——声明路径已在会话分支落提交，close 无认领即过，闸四面全空；
//! t2 目录形态声明——目录 token 未落位整批拒，token 带斜杠原样留痕；
//! t3 豁免三面——unparsed（仓外 token）与 shared_surface_exempt（共享活面）与
//! exempt_conditional（条件申报）不拦不放行认领计数。
//! fixture 全 temp 自建（git 沙箱仓与工地），零真实账本写入。

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

fn git(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let o = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git spawn");
    (
        o.status.success(),
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
    )
}

struct Domain {
    root: PathBuf,
    ledger: PathBuf,
    locks: PathBuf,
    bills: PathBuf,
    trail: PathBuf,
    pkg: PathBuf,
}

fn init_repo(dir: &Path) {
    let _ = Command::new("git").arg("init").arg("-b").arg("master").arg(dir).output();
    git(dir, &["config", "user.email", "t@t"]);
    git(dir, &["config", "user.name", "t"]);
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-m", "base"]);
}

fn make_domain(tag: &str, requested_writes: &str) -> Domain {
    let root = std::env::temp_dir().join(format!("gap-landing-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let engine = root.join("sih-engine");
    std::fs::create_dir_all(&engine).unwrap();
    std::fs::write(engine.join("README.md"), "base\n").unwrap();
    init_repo(&engine);
    let tools = root.join("sih-tools");
    std::fs::create_dir_all(&tools).unwrap();
    std::fs::write(tools.join("placeholder.txt"), "base\n").unwrap();
    init_repo(&tools);

    std::fs::create_dir_all(root.join("sih/ledger")).unwrap();
    std::fs::create_dir_all(engine.join("sih/event/trail")).unwrap();
    let pkg = root.join("sih/state/plan").join(format!("{}.md", tag));
    std::fs::create_dir_all(pkg.parent().unwrap()).unwrap();
    std::fs::write(
        &pkg,
        format!(
            "# {tag}：未提交落位面任务包\n\n## 请求写入 {{#requested-writes}}\n\n{requested_writes}\n"
        ),
    )
    .unwrap();
    std::fs::write(root.join("sih/ledger/sessions.ndjson"), "").unwrap();
    std::fs::write(root.join("sih/ledger/locks.ndjson"), "").unwrap();
    std::fs::write(root.join("sih/ledger/bills.ndjson"), "").unwrap();
    Domain {
        trail: engine.join("sih/event/trail"),
        ledger: root.join("sih/ledger/sessions.ndjson"),
        locks: root.join("sih/ledger/locks.ndjson"),
        bills: root.join("sih/ledger/bills.ndjson"),
        root,
        pkg,
    }
}

fn run_lease(domain: &Domain, stem: &str, args: &[&str]) -> (i32, String, String) {
    let base = [
        "--root".to_string(),
        domain.root.display().to_string(),
        "--ledger".to_string(),
        domain.ledger.display().to_string(),
        "--locks".to_string(),
        domain.locks.display().to_string(),
        "--bills".to_string(),
        domain.bills.display().to_string(),
    ];
    let o = Command::new(env!("CARGO_BIN_EXE_lease"))
        .args(args)
        .arg("--package")
        .arg(stem)
        .args(&base)
        .output()
        .expect("lease spawn");
    (
        o.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
    )
}

/// 开约全链：spec 申报句入 intent（SDDG-1 过）+ 链上 intent 与 cert 两笔。
fn open_with_chain(domain: &Domain, stem: &str) -> String {
    let spec = "本批规格消费面：SPEC-024 落位面正典";
    let intent_path = domain.root.join(format!("intent-{}.json", stem));
    std::fs::write(
        &intent_path,
        json!({
            "session_id": format!("sess-{}", stem),
            "round": 1,
            "calls_in": 0,
            "calls_out": 0,
            "raw_input": spec,
            "intent_contract": {"goal": spec},
            "domain_contract": {"repos": ["sih-engine"], "writes": []}
        })
        .to_string(),
    )
    .unwrap();
    let identity_path = domain.root.join("identity.json");
    std::fs::write(
        &identity_path,
        json!({
            "anomalies": [],
            "identity": {
                "core_hash": "fixturecorehash0000000000000000000000000000000000000000000000000000000000",
                "file_sha256": format!("fixturefsha{}", stem),
                "identity_hash": "fixtureidentityhash00000000000000000000000000000000000000000000000000000000"
            }
        })
        .to_string(),
    )
    .unwrap();
    let (code, out, err) = run_lease(
        domain,
        stem,
        &[
            "open",
            "--at",
            "2026-09-13T00:00:00+00:00",
            "--identity",
            identity_path.to_str().unwrap(),
            "--intent",
            intent_path.to_str().unwrap(),
        ],
    );
    assert_eq!(code, 0, "open: {} {}", out, err);
    let sid = serde_json::from_str::<Value>(&out).unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();

    let trail = domain.trail.join("2026-09-13.ndjson");
    let record = domain.root.join(format!("record-{}.json", stem));
    std::fs::write(
        &record,
        json!({
            "session_id": format!("sess-{}", stem),
            "round": 1,
            "calls_in": 0,
            "calls_out": 0,
            "raw_input": spec,
            "intent_contract": {"goal": spec},
            "domain_contract": {"repos": ["sih-engine"], "writes": []}
        })
        .to_string(),
    )
    .unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_scribe"))
        .args([
            "intent",
            "--record",
            record.to_str().unwrap(),
            "--trail",
            trail.to_str().unwrap(),
            "--locks",
            domain.locks.to_str().unwrap(),
            "--sessions",
            domain.ledger.to_str().unwrap(),
            "--session",
            &sid,
        ])
        .output()
        .expect("scribe intent");
    assert!(o.status.success(), "scribe intent: {}", String::from_utf8_lossy(&o.stderr));

    let report = domain.root.join(format!("cert-{}.json", stem));
    std::fs::write(&report, json!({"note": stem}).to_string()).unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_scribe"))
        .args([
            "append",
            "--report",
            report.to_str().unwrap(),
            "--exit-code",
            "0",
            "--trail",
            trail.to_str().unwrap(),
            "--locks",
            domain.locks.to_str().unwrap(),
            "--sessions",
            domain.ledger.to_str().unwrap(),
            "--session",
            &sid,
        ])
        .output()
        .expect("scribe append");
    assert!(o.status.success(), "scribe append: {}", String::from_utf8_lossy(&o.stderr));
    sid
}

fn close_args(domain: &Domain, stem: &str, extra: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = vec![
        "close".into(),
        "--trail".into(),
        domain.trail.join("2026-09-13.ndjson").display().to_string(),
    ];
    v.extend(extra.iter().map(|s| s.to_string()));
    let _ = stem;
    v
}

/// 已提交落位形：声明路径已在会话分支（工地提交）落位。真实提交令 SDDG-2/3/4
/// 进入实判（差分非空即新源码件与结果档与测试件判据全亮），走 DEC-024 显式
/// 绕行通道 close --bypass-sddgate <事由>（落 bypass 台账留痕），落位闸本体
/// 无认领即过：闸四面全空、绕行判词与台账行在档。
#[test]
fn t1_committed_landing_passes_without_ack() {
    let d = make_domain("landok", "- sih-engine/src/landed.rs\n");
    let _sid = open_with_chain(&d, "landok");
    // 工地（worktree）内提交声明路径至会话分支 msh/landok
    let wt = d.root.join("worktrees/sih-engine/landok");
    assert!(wt.is_dir(), "开约须建工地：{}", wt.display());
    std::fs::create_dir_all(wt.join("src")).unwrap();
    std::fs::write(wt.join("src/landed.rs"), "pub fn landed() {}\n").unwrap();
    let (ok, _, err) = git(&wt, &["add", "-A"]);
    assert!(ok, "git add: {err}");
    let (ok, _, err) = git(&wt, &["commit", "-m", "land declared write"]);
    assert!(ok, "git commit: {err}");

    let bypass_reason = "落位面沙箱验证：声明已提交落位全链，SDDG 实判面越出本件靶面";
    let args = close_args(
        &d,
        "landok",
        &["--bypass-sddgate", bypass_reason],
    );
    let (code, out, err) = run_lease(&d, "landok", &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "已提交落位须收约过：{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(r["declaration_gate"]["uncommitted"], json!([]), "落位闸无认领即过：{r}");
    assert_eq!(r["declaration_gate"]["acks"], json!([]));
    assert_eq!(r["declaration_gate"]["unparsed"], json!([]));
    assert_eq!(r["declaration_gate"]["shared_surface_exempt"], json!([]));
    assert_eq!(r["gates_skipped"]["count"], json!(0), "绕行不计入 gates_skipped：{r}");
    assert_eq!(r["sddgate_gate"]["verdict"], json!("bypass"), "绕行判词如实回显：{r}");
    assert_eq!(r["sddgate_gate"]["reason"], json!(bypass_reason));
    assert_eq!(r["revoked"], json!(true));
    // 绕行台账留痕在档（缺省 bypass 位）
    let bypass_text = std::fs::read_to_string(d.root.join("sih-tools/lease/ledger/bypass.ndjson")).unwrap();
    let rows: Vec<Value> = bypass_text
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert!(
        rows.iter().any(|row| row["event"] == json!("bypassed_sddgate")
            && row["reason"] == json!(bypass_reason)
            && row["package"] == json!("landok")),
        "bypass 台账行在档：{rows:?}"
    );
}

/// 目录形态声明：目录 token 未落位整批拒，token 带斜杠原样入未提交清单。
#[test]
fn t2_dir_form_declared_uncommitted_reject() {
    let d = make_domain("landdir", "- sih-engine/src/newdir/\n");
    let _sid = open_with_chain(&d, "landdir");
    let args = close_args(&d, "landdir", &[]);
    let (code, out, _) = run_lease(&d, "landdir", &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1, "目录未落位须整批拒：{out}");
    let e: Value = serde_json::from_str(&out).unwrap();
    let msg = e["error"].as_str().unwrap();
    assert!(msg.starts_with("差集闸拦截：声明未提交"), "{}", msg);
    assert!(msg.contains("sih-engine/src/newdir/"), "目录 token 带斜杠留痕：{msg}");
}

/// 豁免三面：unparsed 与 shared_surface_exempt 与 exempt_conditional 不拦不放认领。
#[test]
fn t3_unparsed_shared_and_conditional_exempt_faces() {
    let d = make_domain(
        "landex",
        "- external/doc/x.md\n- sih-engine/sih/event/trail/2026-09-13.ndjson\n- sih-engine/doc/late.md 若批准后补写\n",
    );
    let _sid = open_with_chain(&d, "landex");
    let args = close_args(&d, "landex", &[]);
    let (code, out, err) = run_lease(&d, "landex", &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "豁免三面须收约过：{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    let gate = &r["declaration_gate"];
    assert_eq!(gate["uncommitted"], json!([]), "三豁免面不得入未提交清单：{gate}");
    assert_eq!(gate["acks"], json!([]));
    assert_eq!(
        gate["unparsed"],
        json!(["external/doc/x.md"]),
        "仓外 token 入 unparsed 面：{gate}"
    );
    assert!(
        gate["shared_surface_exempt"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "sih-engine/sih/event/trail/2026-09-13.ndjson"),
        "共享活面 token 入豁免面：{gate}"
    );
    assert_eq!(gate["exempt_conditional"].as_array().unwrap().len(), 1, "条件申报入豁免面：{gate}");
    assert_eq!(r["gates_skipped"]["count"], json!(0));
}
