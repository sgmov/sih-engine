//! pk-055 cascadeclose-solo：级联边册投影 close 前重建接线集成测试。
//! 承接 2026-09-17 用户裁定形态（close 前重建挂收约流程，承 pk-052 择案先例）。
//! 六面：
//! - stale 投影 fixture：close 触发重建、工地落盘自成一笔提交、归并带回主树、
//!   主树投影与 cascade build 正典产物逐字节一致、回执 engage 键与吊销行注记在档。
//! - fresh 投影：in_sync 零噪零笔，主树历史无重建笔。
//! - 无级联基建（无语料根与投影件）：跳过注记不炸（吊销行 reason 入档，回执不增键）。
//! - 新城域形（root 下无 sih-tools/lease/ledger，承 ledger_surface 分流先例）：
//!   跳过注记，主树投影零触碰。
//! - 无工地载体（会话仓无 sih-engine）：跳过注记，主树零直写（投影只经归并路径）。
//! - 语料毒件（非 UTF-8 md）：重建失败 fail-visible 拒收约退出码一，会话保持活跃
//!   零归并工地不拆。

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

const STEM: &str = "cascadeclose";

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

fn init_repo(dir: &Path) {
    let _ = Command::new("git").arg("init").arg("-b").arg("master").arg(dir).output();
    git(dir, &["config", "user.email", "t@t"]);
    git(dir, &["config", "user.name", "t"]);
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-m", "base"]);
}

fn cascade_bin_run(args: &[&str]) -> String {
    let o = Command::new(env!("CARGO_BIN_EXE_cascade"))
        .args(args)
        .output()
        .expect("cascade spawn");
    assert!(
        o.status.success(),
        "cascade {:?}: {} {}",
        args,
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    String::from_utf8(o.stdout).unwrap()
}

#[derive(Clone, Copy, PartialEq)]
enum Corpus {
    None,
    StaleProjection,
    FreshProjection,
    PoisonUtf8,
}

#[derive(Clone, Copy, PartialEq)]
enum LedgerForm {
    Central,
    Domain,
}

struct Domain {
    root: PathBuf,
    engine: PathBuf,
    ledger: PathBuf,
    locks: PathBuf,
    bills: PathBuf,
    trail: PathBuf,
}

fn make_domain(tag: &str, corpus: Corpus, form: LedgerForm) -> Domain {
    let root = std::env::temp_dir().join(format!("cascadeclose-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let engine = root.join("sih-engine");
    std::fs::create_dir_all(engine.join("sih/state/plan")).unwrap();
    std::fs::create_dir_all(engine.join("sih/event/trail")).unwrap();
    std::fs::write(engine.join("README.md"), "base\n").unwrap();
    std::fs::write(
        engine.join(format!("sih/state/plan/{}.md", STEM)),
        "# cascadeclose：pk-055 fixture 任务包\n\n## 请求写入 {#requested-writes}\n\n（本批零声明）\n",
    )
    .unwrap();
    let doc = engine.join("doc");
    if corpus != Corpus::None {
        std::fs::create_dir_all(&doc).unwrap();
        std::fs::write(
            doc.join("DEC-001-alpha.md"),
            "Root decision, consults PRO-002-beta and GOV-009-missing.\n",
        )
        .unwrap();
        std::fs::write(doc.join("PRO-002-beta.md"), "Provider doc.\n").unwrap();
        let projection = doc.join("CASCADE.json");
        match corpus {
            Corpus::FreshProjection => {
                cascade_bin_run(&[
                    "build",
                    "--root",
                    doc.to_str().unwrap(),
                    "--out",
                    projection.to_str().unwrap(),
                ]);
            }
            _ => {
                // stale 种子：任意在盘字节，只须在位触发重建路径
                std::fs::write(&projection, "{\"edges\": {}, \"stale\": true}\n").unwrap();
            }
        }
    }
    if corpus == Corpus::PoisonUtf8 {
        // 毒件：非 UTF-8 语料 md（扫描 panic 形，对表 bin 落差五）
        std::fs::write(doc.join("poison.md"), b"\xff\xfe not utf8 \x00\n").unwrap();
    }
    init_repo(&engine);
    let tools = root.join("sih-tools");
    std::fs::create_dir_all(&tools).unwrap();
    std::fs::write(tools.join("placeholder.txt"), "base\n").unwrap();
    init_repo(&tools);
    let (ledger, locks, bills) = match form {
        LedgerForm::Central => {
            let dir = root.join("sih-tools/lease/ledger");
            std::fs::create_dir_all(&dir).unwrap();
            (
                dir.join("sessions.ndjson"),
                dir.join("locks.ndjson"),
                dir.join("bills.ndjson"),
            )
        }
        LedgerForm::Domain => {
            let dir = root.join("sih/ledger");
            std::fs::create_dir_all(&dir).unwrap();
            (
                dir.join("sessions.ndjson"),
                dir.join("locks.ndjson"),
                dir.join("bills.ndjson"),
            )
        }
    };
    for p in [&ledger, &locks, &bills] {
        std::fs::write(p, "").unwrap();
    }
    Domain {
        trail: engine.join("sih/event/trail"),
        ledger,
        locks,
        bills,
        engine,
        root,
    }
}

fn run_lease(domain: &Domain, args: &[String]) -> (i32, String, String) {
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
        .args(&base)
        .output()
        .expect("lease spawn");
    (
        o.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
    )
}

/// 开约并落 intent 与 cert 两笔链笔（SDDG-1 规格申报词形开），返回会话号。
fn open_with_chain(domain: &Domain, tag: &str, repos: &[&str]) -> String {
    let spec = "本批规格消费面：SPEC-024 腿二正典";
    let record = json!({
        "session_id": format!("sess-{}", tag),
        "round": 1,
        "calls_in": 0,
        "calls_out": 0,
        "raw_input": spec,
        "intent_contract": {"goal": spec},
        "domain_contract": {"repos": repos, "writes": []}
    });
    let intent_path = domain.root.join(format!("intent-{}.json", tag));
    std::fs::write(&intent_path, record.to_string()).unwrap();
    let identity_path = domain.root.join("identity.json");
    std::fs::write(
        &identity_path,
        json!({
            "anomalies": [],
            "identity": {
                "core_hash": "fixturecorehash0000000000000000000000000000000000000000000000000000000000",
                "file_sha256": format!("fixturefsha{}", tag),
                "identity_hash": "fixtureidentityhash00000000000000000000000000000000000000000000000000000"
            }
        })
        .to_string(),
    )
    .unwrap();
    let mut args: Vec<String> = vec![
        "open".into(),
        "--package".into(),
        STEM.into(),
        "--at".into(),
        "2026-09-13T00:00:00+00:00".into(),
        "--identity".into(),
        identity_path.display().to_string(),
        "--intent".into(),
        intent_path.display().to_string(),
    ];
    for r in repos {
        args.push("--repo".into());
        args.push((*r).into());
    }
    let (code, out, err) = run_lease(domain, &args);
    assert_eq!(code, 0, "open: {} {}", out, err);
    let sid = serde_json::from_str::<Value>(&out).unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();

    let trail = domain.trail.join("2026-09-13.ndjson");
    let o = Command::new(env!("CARGO_BIN_EXE_scribe"))
        .args([
            "intent",
            "--record",
            intent_path.to_str().unwrap(),
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
    assert!(
        o.status.success(),
        "scribe intent: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    let report = domain.root.join(format!("cert-{}.json", tag));
    std::fs::write(&report, json!({"note": tag}).to_string()).unwrap();
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
    assert!(
        o.status.success(),
        "scribe append: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    sid
}

fn close_args(domain: &Domain, extra: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = vec![
        "close".into(),
        "--package".into(),
        STEM.into(),
        "--trail".into(),
        domain.trail.join("2026-09-13.ndjson").display().to_string(),
    ];
    v.extend(extra.iter().map(|s| s.to_string()));
    v
}

fn revoked_detail(domain: &Domain) -> Value {
    let rows: Vec<Value> = std::fs::read_to_string(&domain.ledger)
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    rows.iter()
        .rev()
        .find(|r| r.get("event").and_then(|v| v.as_str()) == Some("revoked"))
        .map(|r| r.get("detail").cloned().unwrap_or(Value::Null))
        .unwrap_or(Value::Null)
}

fn session_last_event(domain: &Domain, sid: &str) -> String {
    let rows: Vec<Value> = std::fs::read_to_string(&domain.ledger)
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    rows.iter()
        .rev()
        .find(|r| r.get("session_id").and_then(|v| v.as_str()) == Some(sid))
        .and_then(|r| r.get("event").and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_string()
}

#[test]
fn cascadeclose_stale_projection_rebuilt_and_carried_by_merge() {
    let d = make_domain("stale", Corpus::StaleProjection, LedgerForm::Central);
    let _sid = open_with_chain(&d, "stale", &["sih-engine"]);
    let main_projection = d.engine.join("doc/CASCADE.json");
    let stale_bytes = std::fs::read_to_string(&main_projection).unwrap();

    let args = close_args(&d, &[]);
    let (code, out, err) = run_lease(&d, &args);
    assert_eq!(code, 0, "stale 投影收约须过：{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(r["revoked"], json!(true));
    assert_eq!(r["cascade_gate"]["checked"], json!(true), "engage 形上回执：{r}");
    assert_eq!(r["cascade_gate"]["result"], json!("updated"));
    assert_eq!(r["cascade_gate"]["diff"], json!(true));
    assert!(
        !r["cascade_gate"]["commit"].as_str().unwrap_or("").is_empty(),
        "工地重建笔短 sha 在档：{r}"
    );

    // 主树投影经归并更新，与 cascade build 正典产物逐字节一致（含册记主树语料根）。
    let expected = cascade_bin_run(&["build", "--root", d.engine.join("doc").to_str().unwrap()]);
    let after = std::fs::read_to_string(&main_projection).unwrap();
    assert_eq!(after, expected, "归并后主树投影即正典建册产物");
    assert_ne!(after, stale_bytes, "stale 种子确被重建替换");
    let parsed: Value = serde_json::from_str(&after).unwrap();
    assert_eq!(
        parsed["edges"]["DEC-001-alpha.md"],
        json!(["PRO-002-beta.md"]),
        "引用即边语义对表 t4 金向量"
    );
    assert_eq!(parsed["notes"]["unmapped"], json!(["GOV-009"]));

    // 主树历史含工地重建笔与归并笔，工地已拆。
    let (_, subjects, _) = git(&d.engine, &["log", "--format=%s"]);
    assert!(
        subjects.contains(&format!(
            "cascadeclose-solo: pre-close cascade projection rebuild ({})",
            STEM
        )),
        "重建笔随归并入主树历史：{}",
        subjects
    );
    assert!(subjects.contains(&format!("merge: {} 副本归并", STEM)));
    assert!(!d.root.join(format!("worktrees/sih-engine/{}", STEM)).exists());

    // 吊销行注记在档。
    assert_eq!(revoked_detail(&d)["cascade_gate"]["result"], json!("updated"));
}

#[test]
fn cascadeclose_fresh_projection_in_sync_zero_noise() {
    let d = make_domain("sync", Corpus::FreshProjection, LedgerForm::Central);
    let _sid = open_with_chain(&d, "sync", &["sih-engine"]);
    let main_projection = d.engine.join("doc/CASCADE.json");
    let seed = std::fs::read_to_string(&main_projection).unwrap();

    let args = close_args(&d, &[]);
    let (code, out, err) = run_lease(&d, &args);
    assert_eq!(code, 0, "fresh 投影收约须过：{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(r["cascade_gate"]["result"], json!("in_sync"));
    assert_eq!(r["cascade_gate"]["diff"], json!(false));

    // 零噪零笔：主树历史无重建笔，投影字节不动。
    let (_, subjects, _) = git(&d.engine, &["log", "--format=%s"]);
    assert!(!subjects.contains("projection rebuild"), "in_sync 不得落笔：{}", subjects);
    assert_eq!(std::fs::read_to_string(&main_projection).unwrap(), seed);
}

#[test]
fn cascadeclose_no_infrastructure_skip_noted_not_crashing() {
    let d = make_domain("noinfra", Corpus::None, LedgerForm::Central);
    let _sid = open_with_chain(&d, "noinfra", &["sih-engine"]);
    let args = close_args(&d, &[]);
    let (code, out, err) = run_lease(&d, &args);
    assert_eq!(code, 0, "无级联基建跳过不炸：{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert!(
        r.get("cascade_gate").is_none(),
        "跳过形回执不增键（承 T2 金向量冻结）：{r}"
    );
    assert_eq!(
        revoked_detail(&d)["cascade_gate"]["reason"],
        json!("no_cascade_infrastructure"),
        "跳过注记入吊销行"
    );
}

#[test]
fn cascadeclose_domain_form_skip_noted_and_projection_untouched() {
    let d = make_domain("domain", Corpus::StaleProjection, LedgerForm::Domain);
    let _sid = open_with_chain(&d, "domain", &["sih-engine"]);
    // open 骨架无条件建 root/sih-tools/lease/ledger/checks（与围堰对表行为），
    // 移除后即新城域形 root（真域根本无此目录），承 ledger_surface 分流先例。
    let skeleton = d.root.join("sih-tools/lease/ledger");
    assert!(skeleton.is_dir(), "open 骨架在位前提：{}", skeleton.display());
    std::fs::remove_dir_all(&skeleton).unwrap();

    let main_projection = d.engine.join("doc/CASCADE.json");
    let stale_bytes = std::fs::read_to_string(&main_projection).unwrap();
    let args = close_args(&d, &[]);
    let (code, out, err) = run_lease(&d, &args);
    assert_eq!(code, 0, "新城域形跳过不炸：{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert!(r.get("cascade_gate").is_none(), "跳过形回执不增键：{r}");
    assert_eq!(
        revoked_detail(&d)["cascade_gate"]["reason"],
        json!("domain_form"),
        "新城域形注记入吊销行"
    );
    assert_eq!(
        std::fs::read_to_string(&main_projection).unwrap(),
        stale_bytes,
        "新城域形主树投影零触碰"
    );
}

#[test]
fn cascadeclose_no_worktree_carrier_skip_and_no_main_write() {
    let d = make_domain("nocarrier", Corpus::StaleProjection, LedgerForm::Central);
    let _sid = open_with_chain(&d, "nocarrier", &["sih-tools"]);
    // defectwave 批缺陷四：pk-104 openface 推导形后 open 声明 ["sih-tools"]
    // 仍得引擎仓（声明面全无仓前缀回落缺省 sih-engine 与旗标面并集），
    // skip 前提不可达即恒红。载体判定（closegate cascade_projection_rebuild）：
    // 会话 sih-engine 工地在席且其 doc/CASCADE.json 在案即载体。故 open 后、
    // close 前移走引擎工地目录模拟载体灭失（工程真实形：载体中途灭失），
    // close 应报 cascade_gate reason no_worktree_carrier 且主树投影零直写。
    let engine_worktree = d.root.join(format!("worktrees/sih-engine/{}", STEM));
    assert!(engine_worktree.is_dir(), "引擎工地在席前提：{}", engine_worktree.display());
    // git worktree remove 正形拆除（清主仓 worktree 元数据；裸 rm 会让
    // branch -d 因「used by worktree」拒删）。拆后载体灭失。
    let (ok, _, werr) = git(
        &d.engine,
        &["worktree", "remove", engine_worktree.to_str().unwrap()],
    );
    assert!(ok, "引擎工地拆除: {}", werr);
    let main_projection = d.engine.join("doc/CASCADE.json");
    let stale_bytes = std::fs::read_to_string(&main_projection).unwrap();

    let args = close_args(&d, &[]);
    let (code, out, err) = run_lease(&d, &args);
    assert_eq!(code, 0, "无载体跳过不炸：{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert!(r.get("cascade_gate").is_none(), "跳过形回执不增键：{r}");
    assert_eq!(
        revoked_detail(&d)["cascade_gate"]["reason"],
        json!("no_worktree_carrier"),
        "无载体注记入吊销行"
    );
    assert_eq!(
        std::fs::read_to_string(&main_projection).unwrap(),
        stale_bytes,
        "无载体即无归并载体：主树零直写，stale 保持待 sih-engine 批自愈"
    );
}

#[test]
fn cascadeclose_poison_corpus_fail_visible_reject() {
    let d = make_domain("poison", Corpus::PoisonUtf8, LedgerForm::Central);
    let sid = open_with_chain(&d, "poison", &["sih-engine"]);
    let main_projection = d.engine.join("doc/CASCADE.json");
    let stale_bytes = std::fs::read_to_string(&main_projection).unwrap();

    let args = close_args(&d, &[]);
    let (code, out, err) = run_lease(&d, &args);
    assert_eq!(code, 1, "语料毒件须 fail-visible 拒收约：{} {}", out, err);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(e["reason"], json!("cascade_rebuild_failed"));
    assert!(
        e["error"].as_str().unwrap().contains("级联投影重建失败"),
        "拒收词带原因：{e}"
    );

    // 会话保持活跃、零归并、工地不拆、主树零触碰。
    assert_eq!(session_last_event(&d, &sid), "issued", "会话保持活跃");
    assert_eq!(
        std::fs::read_to_string(&main_projection).unwrap(),
        stale_bytes,
        "主树投影零触碰"
    );
    assert!(
        d.root.join(format!("worktrees/sih-engine/{}", STEM)).is_dir(),
        "工地未拆（拒在归并前）"
    );
    let (_, subjects, _) = git(&d.engine, &["log", "--format=%s"]);
    assert!(!subjects.contains("merge:"), "零归并：{}", subjects);
}
