//! 腿二 T6：chained close 闸序集成测试（lease-commitlaw-parallel 批）。
//! 链证守门、SDDG 拒绝教学、无主闸、声明差集闸、认领放行五场景对表围堰捕获形
//! （work/leg2-materials/golden-capture c22/c27/c28/c29/c30）。

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
}

fn init_repo(dir: &Path) {
    let _ = Command::new("git").arg("init").arg("-b").arg("master").arg(dir).output();
    git(dir, &["config", "user.email", "t@t"]);
    git(dir, &["config", "user.name", "t"]);
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-m", "base"]);
}

fn make_domain(tag: &str) -> Domain {
    let root = std::env::temp_dir().join(format!("leg2-t6-{}-{}", tag, std::process::id()));
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
    std::fs::create_dir_all(engine.join("sih/state/plan")).unwrap();
    std::fs::write(
        engine.join("sih/state/plan/goldd.md"),
        concat!(
            "# goldd：腿二金向量任务包\n\n",
            "## 请求写入 {#requested-writes}\n\n",
            "- sih-engine/src/lib_g.rs\n",
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
    }
}

fn run_lease(domain: &Domain, args: &[&str]) -> (i32, String, String) {
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

/// intent 文本带规格申报词形开关，链落 intent 与 cert 两笔
fn open_with_chain(domain: &Domain, tag: &str, declare_spec: bool) -> String {
    let spec_sentence = if declare_spec {
        "本批规格消费面：SPEC-024 腿二正典"
    } else {
        "普通任务描述文本"
    };
    let intent_path = domain.root.join(format!("intent-{}.json", tag));
    std::fs::write(
        &intent_path,
        json!({
            "session_id": format!("sess-{}", tag),
            "round": 1,
            "calls_in": 0,
            "calls_out": 0,
            "raw_input": spec_sentence,
            "intent_contract": {"goal": spec_sentence},
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
                "file_sha256": format!("fixturefsha{}", tag),
                "identity_hash": "fixtureidentityhash00000000000000000000000000000000000000000000000000000000"
            }
        })
        .to_string(),
    )
    .unwrap();
    let (code, out, err) = run_lease(
        domain,
        &[
            "open",
            "--package",
            "goldd",
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
    let intent_record = domain.root.join(format!("record-{}.json", tag));
    std::fs::write(
        &intent_record,
        json!({
            "session_id": format!("sess-{}", tag),
            "round": 1,
            "calls_in": 0,
            "calls_out": 0,
            "raw_input": spec_sentence,
            "intent_contract": {"goal": spec_sentence},
            "domain_contract": {"repos": ["sih-engine"], "writes": []}
        })
        .to_string(),
    )
    .unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_scribe"))
        .args([
            "intent",
            "--record",
            intent_record.to_str().unwrap(),
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
    assert!(o.status.success(), "scribe append: {}", String::from_utf8_lossy(&o.stderr));
    sid
}

fn close_args(domain: &Domain, extra: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = vec![
        "close".into(),
        "--package".into(),
        "goldd".into(),
        "--trail".into(),
        domain.trail.join("2026-09-13.ndjson").display().to_string(),
    ];
    v.extend(extra.iter().map(|s| s.to_string()));
    v
}

#[test]
fn t6_chain_gate_missing_cert_pen() {
    let d = make_domain("chain");
    // 只落 intent 不落 cert：链证守门缺笔整批拒
    let sid = {
        let spec = "本批规格消费面：SPEC-024 腿二正典";
        let intent_path = d.root.join("intent-only.json");
        std::fs::write(
            &intent_path,
            json!({"session_id": "sess-only", "round": 1, "calls_in": 0, "calls_out": 0,
                   "raw_input": spec, "intent_contract": {"goal": spec},
                   "domain_contract": {"repos": ["sih-engine"], "writes": []}})
            .to_string(),
        )
        .unwrap();
        let identity_path = d.root.join("identity.json");
        std::fs::write(
            &identity_path,
            json!({"anomalies": [], "identity": {
                "core_hash": "fixturecorehash0000000000000000000000000000000000000000000000000000000000",
                "file_sha256": "fixturefsha-only",
                "identity_hash": "fixtureidentityhash00000000000000000000000000000000000000000000000000000000"
            }})
            .to_string(),
        )
        .unwrap();
        let (code, out, err) = run_lease(
            &d,
            &[
                "open",
                "--package",
                "goldd",
                "--at",
                "2026-09-13T00:00:00+00:00",
                "--identity",
                identity_path.to_str().unwrap(),
                "--intent",
                intent_path.to_str().unwrap(),
            ],
        );
        assert_eq!(code, 0, "{} {}", out, err);
        let sid = serde_json::from_str::<Value>(&out).unwrap()["session_id"]
            .as_str()
            .unwrap()
            .to_string();
        let record = d.root.join("record-only.json");
        std::fs::write(
            &record,
            json!({"session_id": "sess-only", "round": 1, "calls_in": 0, "calls_out": 0,
                   "raw_input": spec, "intent_contract": {"goal": spec},
                   "domain_contract": {"repos": ["sih-engine"], "writes": []}})
            .to_string(),
        )
        .unwrap();
        let o = Command::new(env!("CARGO_BIN_EXE_scribe"))
            .args([
                "intent",
                "--record",
                record.to_str().unwrap(),
                "--trail",
                d.trail.join("2026-09-13.ndjson").to_str().unwrap(),
                "--locks",
                d.locks.to_str().unwrap(),
                "--sessions",
                d.ledger.to_str().unwrap(),
                "--session",
                &sid,
            ])
            .output()
            .unwrap();
        assert!(o.status.success());
        sid
    };
    let args = close_args(&d, &[]);
    let (code, out, _) = run_lease(&d, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        e["error"],
        json!(format!(
            "链证守门拦截：本会话正典链缺笔，补链笔后收约{{\"session_id\": \"{}\", \"missing\": [\"certification_completed\"]}}",
            sid
        ))
    );
}

#[test]
fn t6_sddg1_reject_teaching_embed() {
    let d = make_domain("sddg1");
    let _sid = open_with_chain(&d, "sddg1", false);
    let args = close_args(&d, &[]);
    let (code, out, _) = run_lease(&d, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    let msg = e["error"].as_str().unwrap();
    assert!(msg.starts_with(
        "SDDG 门拦截：SDD/TDD 完备度判据未过，整批拒零动作（DEC-024 B 层逐批执法，判定命令形机械执法零 LLM 判词位）。\n{\n  \"reason_code\": \"sddgate_rejected\",\n  \"gate\": [\n    \"SDDG-1\"\n  ],"
    ), "teaching 内嵌插入序与判定命令形须逐字节对表：{}", msg);
    assert!(msg.contains("\"intent_spec_declared\": false"));
    assert!(msg.contains("criteria_commands"));
}

#[test]
fn t6_orphan_gate_reject_and_bypass() {
    let d = make_domain("orphan");
    let _sid = open_with_chain(&d, "orphan", true);
    // 主树无主改件（tracked 修改，不在锁面/声明面/豁免面）
    std::fs::write(d.root.join("sih-engine/README.md"), "dirty\n").unwrap();
    let args = close_args(&d, &[]);
    let (code, out, _) = run_lease(&d, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    let msg = e["error"].as_str().unwrap();
    assert!(msg.starts_with(
        "无主闸拦截：主树存在无主 tracked 改件（脏文件集 − 锁面 ∪ 链笔声明面 ∪ 豁免面 = 无主清单，共 1 件），整批拒零动作。"
    ), "{}", msg);
    assert!(msg.contains("\"path\": \"sih-engine/README.md\""));
    assert!(msg.contains("\"xy\": \" M\""));
    assert!(msg.contains("清单：[\n  {\n    \"path\": \"sih-engine/README.md\",\n    \"xy\": \" M\",\n    \"mtime\":"));
}

#[test]
fn t6_declaration_gate_reject_and_ack_mismatch() {
    let d = make_domain("decl");
    let _sid = open_with_chain(&d, "decl", true);
    // 认领差集外路径先于差集闸拒（unknown_acks 零粉饰）
    let args = close_args(
        &d,
        &["--ack-uncommitted", "sih-engine/elsewhere.txt=错认领"],
    );
    let (code, out, _) = run_lease(&d, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        e["error"],
        json!("差集闸认领对表失败：认领路径不在声明未提交差集内（与实态不符零粉饰）{\"unknown_acks\": [\"sih-engine/elsewhere.txt\"]}")
    );

    // 差集闸拒：声明未提交清单
    let d2 = make_domain("decl2");
    let _sid2 = open_with_chain(&d2, "decl2", true);
    let args2 = close_args(&d2, &[]);
    let (code, out, _) = run_lease(&d2, &args2.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        e["error"],
        json!("差集闸拦截：声明未提交（请求写入节对分支提交树差集非空，整批拒零动作；补提交或 close --ack-uncommitted 逐路径带事由放行）{\"gate\": \"declared_uncommitted\", \"uncommitted\": [\"sih-engine/src/lib_g.rs\"], \"exempt_conditional\": [], \"unparsed\": [], \"acks\": [], \"shared_surface_exempt\": []}")
    );

    // 认领形非法（等号缺）
    let args3 = close_args(&d2, &["--ack-uncommitted", "no-equal-form"]);
    let (code, out, _) = run_lease(&d2, &args3.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    let e: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        e["error"],
        json!("差集闸认领形非法：须 路径=事由（事由零粉饰留档）")
    );
    assert_eq!(e["reason"], json!("ack_uncommitted_invalid"));
}

#[test]
fn t6_close_success_with_ack() {
    let d = make_domain("ok");
    let sid = open_with_chain(&d, "ok", true);
    let args = close_args(
        &d,
        &[
            "--ack-uncommitted",
            "sih-engine/src/lib_g.rs=fixture 捕获：声明未提交认领放行形（零粉饰事由留档）",
        ],
    );
    let (code, out, err) = run_lease(&d, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "{} {}", out, err);
    let r: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(r["revoked"], json!(true));
    assert_eq!(r["failed"], json!([]));
    assert_eq!(r["session_id"], json!(sid));
    assert_eq!(r["chain_gate"], json!({"checked": true, "trails": 1}));
    assert_eq!(
        r["declaration_gate"]["acks"],
        json!([{"path": "sih-engine/src/lib_g.rs",
                "reason": "fixture 捕获：声明未提交认领放行形（零粉饰事由留档）"}])
    );
    assert_eq!(r["declaration_gate"]["uncommitted"], json!([]));
    assert_eq!(
        r["gates_skipped"]["ack_uncommitted"],
        json!(["sih-engine/src/lib_g.rs"])
    );
    assert_eq!(r["gates_skipped"]["count"], json!(1));
    assert_eq!(r["sddgate_gate"]["verdict"], json!("pass"));
    assert_eq!(r["removed"][0]["result"], json!("removed"));
    assert_eq!(r["removed"][0]["branch_result"], json!("deleted"));
    assert_eq!(r["removed"][0]["worktree_result"], json!("removed"));
    // 台账收口：revoked 行在册
    let rows: Vec<Value> = std::fs::read_to_string(&d.ledger)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert!(rows
        .iter()
        .any(|row| row.get("event").and_then(|v| v.as_str()) == Some("revoked")));
}
