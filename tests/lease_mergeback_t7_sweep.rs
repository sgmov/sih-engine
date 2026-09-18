//! 腿三 T7：sweep 五类普查与 call-log import 与 hooks 与 stem 闸全查集成测试
//! （lease-attachments-solo 批，SPEC-024 A9 加 T7 判据）。fixture 域自足构建。

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

fn lease_bin() -> &'static str {
    env!("CARGO_BIN_EXE_lease")
}

fn run_lease(args: &[String]) -> (i32, String, String) {
    let out = Command::new(lease_bin()).args(args).output().expect("引擎 lease 可执行");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

fn lease_args(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|s| s.to_string()).collect()
}

fn git(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let o = Command::new("git").arg("-C").arg(dir).args(args).output().expect("git spawn");
    (
        o.status.success(),
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
    )
}

struct SweeDomain {
    root: PathBuf,
    ledger: PathBuf,
    locks: PathBuf,
}

fn make_sweep_domain(tag: &str) -> SweeDomain {
    let root = std::env::temp_dir().join(format!("leg3-t7-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    // 双仓标记俱在（first_domain 判定形）＋双 git 仓（无主工地扫描面）。
    let eng = root.join("sih-engine");
    let tools = root.join("sih-tools");
    std::fs::create_dir_all(eng.join("sih/state/plan")).unwrap();
    std::fs::create_dir_all(tools.join("lease/ledger/checks")).unwrap();
    for repo in [&eng, &tools] {
        let _ = Command::new("git").arg("init").arg("-b").arg("main").arg(repo).output();
        git(repo, &["config", "user.email", "t@t"]);
        git(repo, &["config", "user.name", "t"]);
        std::fs::write(repo.join(".gitkeep"), "").unwrap();
        git(repo, &["add", "-A"]);
        git(repo, &["commit", "-m", "base"]);
    }
    SweeDomain {
        ledger: tools.join("lease/ledger/sessions.ndjson"),
        locks: tools.join("lease/ledger/locks.ndjson"),
        root,
    }
}

fn sweep_args(d: &SweeDomain) -> [String; 6] {
    [
        "--root".into(),
        d.root.to_str().unwrap().into(),
        "--ledger".into(),
        d.ledger.to_str().unwrap().into(),
        "--locks".into(),
        d.locks.to_str().unwrap().into(),
    ]
}

// --------------------------------------------- F1 sweep 净态与幻影修复
#[test]
fn t7_sweep_clean_exit_zero() {
    let d = make_sweep_domain("clean");
    let a = sweep_args(&d).to_vec();
    let (rc, out, _err) = run_lease(&lease_args(&["sweep"]).into_iter().chain(a.clone()).collect::<Vec<String>>());
    assert_eq!(rc, 0, "净态退出码零：{out}");
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["verdict"], "clean");
    assert_eq!(v["summary"]["auto_cleanable"], 0);
    assert_eq!(v["summary"]["awaiting"], 0);
}

#[test]
fn t7_sweep_phantom_fix_repairs_ledger() {
    let d = make_sweep_domain("phantom");
    // 幻影活跃会话：issued 在册、锁面零持有、worktree 与分支零存、结果档归档
    // ——四证取三（超阈值二）即 auto_clean。
    let sid = "phantom00000001";
    let row = json!({
        "event": "issued",
        "session_id": sid,
        "package": "gone-batch",
        "repos": [{"repo": d.root.join("sih-engine").display().to_string(),
                   "worktree": "/nonexistent/wt", "branch": "msh/gone-batch"}],
    });
    std::fs::write(&d.ledger, serde_json::to_string(&row).unwrap() + "\n").unwrap();
    std::fs::write(
        d.root.join("sih-engine/sih/state/plan/gone-batch-results.md"),
        "# 归档\n",
    )
    .unwrap();
    let a = sweep_args(&d).to_vec();
    let (rc, out, _err) = run_lease(&lease_args(&["sweep"]).into_iter().chain(a.clone()).collect::<Vec<String>>());
    assert_eq!(rc, 1, "残留退出码一：{out}");
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["summary"]["auto_cleanable"], 1);
    assert_eq!(v["classes"]["phantom_sessions"][0]["object"], sid);

    // --fix：修正性 revoked 终笔落台账，复普查 verdict clean。
    let (rc2, out2, _err2) = run_lease(&lease_args(&["sweep", "--fix"]).into_iter().chain(a.clone()).collect::<Vec<String>>());
    assert_eq!(rc2, 0, "--fix 后净：{out2}");
    let v2: Value = serde_json::from_str(&out2).unwrap();
    assert_eq!(v2["post_fix"]["verdict"], "clean");
    let ledger_text = std::fs::read_to_string(&d.ledger).unwrap();
    assert!(ledger_text.contains("\"revoked\""), "修正性 revoked 终笔在台账");
    assert!(ledger_text.contains("sweep:phantom-session"), "repair 标记显式");
}

#[test]
fn t7_sweep_stale_check_awaiting_guard() {
    let d = make_sweep_domain("stale");
    // 停滞检验文件：pid 死而窗会话在册活跃即候裁不清（防搁浅守卫）。
    let sid = "alive000000001";
    let issued = json!({
        "event": "issued",
        "session_id": sid,
        "package": "live-batch",
        "repos": [{"repo": d.root.join("sih-engine").display().to_string(),
                   "worktree": "/nonexistent/wt", "branch": "msh/live-batch"}],
    });
    std::fs::write(&d.ledger, serde_json::to_string(&issued).unwrap() + "\n").unwrap();
    let check = json!({
        "package": "stale-live",
        "pid": 2_000_000_000u64,
        "pid_started": "no-such-start-time",
        "session_id": sid,
        "heartbeat_at": "2026-09-01T00:00:00+00:00",
    });
    std::fs::write(
        d.root.join("sih-tools/lease/ledger/checks/stale-live.json"),
        serde_json::to_string(&check).unwrap(),
    )
    .unwrap();
    let a = sweep_args(&d).to_vec();
    let (rc, out, _err) = run_lease(&lease_args(&["sweep"]).into_iter().chain(a.clone()).collect::<Vec<String>>());
    let v: Value = serde_json::from_str(&out).unwrap();
    let items = v["classes"]["stale_check_files"].as_array().unwrap();
    assert_eq!(items.len(), 1, "检验文件在列：{out}");
    assert_eq!(items[0]["state"], "awaiting", "防搁浅守卫候裁：{out}");
    assert_eq!(items[0]["channel"], "human-takeover");
    assert_eq!(rc, 1);
    // --fix 不清候裁件。
    let (_rc2, out2, _e2) = run_lease(&lease_args(&["sweep", "--fix"]).into_iter().chain(a.clone()).collect::<Vec<String>>());
    let v2: Value = serde_json::from_str(&out2).unwrap();
    assert!(v2["classes"]["stale_check_files"].as_array().unwrap().len() >= 1);
    assert!(
        d.root.join("sih-tools/lease/ledger/checks/stale-live.json").is_file(),
        "候裁件零触碰"
    );
}

// --------------------------------------------- F2 call-log import 零有损
#[test]
fn t7_calllog_import_verbatim_lossless() {
    let d = make_sweep_domain("calllog");
    let book_lease = "| 日期 | 事由 | 调用 | 退出码 | 会话号 | 备注 |\n| --- | --- | --- | --- | --- | --- |\n| 2026-09-01 | 测试 | lease lock | 0 | sid1 | 备注一 |\n\n尾行文本\n";
    let book_gauge = "- 2026-09-02 gauge record 0\n## 表头\n| 日期 | 场景 | 包 | 目标 | 结果 | 核阅交叉 |\n";
    std::fs::create_dir_all(d.root.join("sih-tools/gauge")).unwrap();
    std::fs::write(d.root.join("sih-tools/lease/CALL-LOG.md"), book_lease).unwrap();
    std::fs::write(d.root.join("sih-tools/gauge/CALL-LOG.md"), book_gauge).unwrap();

    let (rc, out, err) = run_lease(&lease_args(&["call-log", "--root", d.root.to_str().unwrap()]));
    assert_eq!(rc, 0, "{err}");
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["skipped"], false);
    assert_eq!(v["summary"]["books"], 19);

    // verbatim 零丢：重组即逐字节还原原册（projfix-solo F-2 判据）。
    let rebuilt_lease = std::fs::read_to_string(d.root.join("sih-tools/lease/CALL-LOG.md")).unwrap();
    assert_eq!(rebuilt_lease, book_lease);
    let rebuilt_gauge = std::fs::read_to_string(d.root.join("sih-tools/gauge/CALL-LOG.md")).unwrap();
    assert_eq!(rebuilt_gauge, book_gauge);

    // 权威腿行数符：两册行数和（lease 5 行加 gauge 3 行）；PROJECTIONS 序
    // gauge(idx4) 先于 lease(idx7)，首行即 gauge 册。
    let ndjson = std::fs::read_to_string(d.root.join("sih-tools/calllog/calls.ndjson")).unwrap();
    assert_eq!(ndjson.lines().count(), 8);
    let first: Value = serde_json::from_str(ndjson.lines().next().unwrap()).unwrap();
    assert_eq!(first["tool"], "gauge");
    let lease_data: Value = ndjson
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .find(|r| r["tool"] == "lease" && r["session"] == "sid1")
        .expect("lease 数据行在档");
    assert_eq!(lease_data["kind"], "row");
    assert_eq!(lease_data["commands"], "lease lock");

    // 一次性语义：二次跑 skipped。
    let (rc2, out2, _e2) = run_lease(&lease_args(&["call-log", "--root", d.root.to_str().unwrap()]));
    assert_eq!(rc2, 0);
    let v2: Value = serde_json::from_str(&out2).unwrap();
    assert_eq!(v2["skipped"], true);
}

// --------------------------------------------- F3 hooks 安装拆卸回环
#[test]
fn t7_hooks_install_uninstall_roundtrip() {
    let d = make_sweep_domain("hooks");
    let eng = d.root.join("sih-engine");
    let (rc, out, err) = run_lease(&lease_args(&["install-hooks", "--repo", eng.to_str().unwrap()]));
    assert_eq!(rc, 0, "{out}{err}");
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["hooks"][0]["installed"], true);
    assert!(v["hooks"][0]["hooks_path"].as_str().unwrap().ends_with("lease/hooks"));

    let (rc2, out2, cfg, _) = {
        let o = Command::new("git")
            .arg("-C")
            .arg(&eng)
            .args(["config", "core.hooksPath"])
            .output()
            .unwrap();
        (
            o.status.code().unwrap_or(-1),
            String::new(),
            String::from_utf8_lossy(&o.stdout).to_string(),
            (),
        )
    };
    assert_eq!(rc2, 0, "config 写入可回读");
    assert!(cfg.trim().ends_with("lease/hooks"));

    let (rc3, out3, _err3) = run_lease(&lease_args(&["uninstall-hooks", "--repo", eng.to_str().unwrap()]));
    assert_eq!(rc3, 0, "{out3}");
    let v3: Value = serde_json::from_str(&out3).unwrap();
    assert_eq!(v3["hooks"][0]["state"], "unset");

    // 再拆卸：键缺席 rc5 计 already_absent 照常收敛。
    let (rc4, out4, _e4) = run_lease(&lease_args(&["uninstall-hooks", "--repo", eng.to_str().unwrap()]));
    assert_eq!(rc4, 0);
    let v4: Value = serde_json::from_str(&out4).unwrap();
    assert_eq!(v4["hooks"][0]["state"], "already_absent");
}

// --------------------------------------------- F4 stem 闸全查与回归
/// defectwave 批缺陷一闸适配夹具：open 正身与意图件校验前置后，stem 闸教学
/// 验证须以合规两件入参抵达 stem 段（/nonexistent 松形不再可达 stem 闸）。
fn write_open_input_fixtures(root: &Path) {
    std::fs::write(
        root.join("identity.json"),
        json!({
            "anomalies": [],
            "identity": {
                "core_hash": "t7fixturecorehash",
                "identity_hash": "t7fixtureidentityhash",
            },
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(root.join("intent.json"), json!({"anchors": []}).to_string()).unwrap();
}

#[test]
fn t7_stem_gate_full_query_rejects_unknown() {
    // fixture 域预置最小真词典包（envelope 加六 body，词表空）：spawn uv
    // nomenclator query 走真全查，未知词裸开拒教学信封零落盘（金向量 stem
    // 拒形对表；包解析先于 stem 闸——任务包文件须在，围堰同序）。
    let d = make_sweep_domain("stemfull");
    let pack = d.root.join("sih-tools/nomenclator/packs/core");
    std::fs::create_dir_all(&pack).unwrap();
    std::fs::write(
        pack.join("envelope.json"),
        r#"{"envelope_version":1,"id":"core","family":"nomenclator","body_type":"config","bodies":["manifest.json","terms.json","lazy.json","dead.json","candidates.json","anchors.json"]}"#,
    )
    .unwrap();
    std::fs::write(
        pack.join("manifest.json"),
        r#"{"name":"t7","version":"0","domain":{"include":[],"exclude":[]}}"#,
    )
    .unwrap();
    std::fs::write(pack.join("terms.json"), r#"{"terms":[]}"#).unwrap();
    std::fs::write(pack.join("lazy.json"), r#"{"lazy":[]}"#).unwrap();
    std::fs::write(pack.join("dead.json"), r#"{"dead":[]}"#).unwrap();
    std::fs::write(pack.join("candidates.json"), r#"{"candidates":[]}"#).unwrap();
    std::fs::write(pack.join("anchors.json"), r#"{"anchors":[]}"#).unwrap();
    std::fs::create_dir_all(d.root.join("sih/state/plan")).unwrap();
    std::fs::write(
        d.root.join("sih/state/plan/uncoined-word-xyz.md"),
        "# uncoined-word-xyz\n\n## 请求写入 {#requested-writes}\n\n- sih-engine/src/x.rs\n",
    )
    .unwrap();
    write_open_input_fixtures(&d.root);

    let (rc, _out, err) = run_lease(&lease_args(&[
        "open",
        "--package",
        "uncoined-word-xyz",
        "--root",
        d.root.to_str().unwrap(),
        "--identity",
        d.root.join("identity.json").to_str().unwrap(),
        "--intent",
        d.root.join("intent.json").to_str().unwrap(),
        "--ledger",
        d.ledger.to_str().unwrap(),
        "--locks",
        d.locks.to_str().unwrap(),
        "--bills",
        d.root.join("sih-tools/lease/ledger/bills.ndjson").to_str().unwrap(),
    ]));
    assert_eq!(rc, 2);
    let v: Value = serde_json::from_str(&err).unwrap();
    let msg = v["error"].as_str().unwrap_or("");
    assert!(msg.contains("未立名新词"), "全查路拒教学：{msg}");
    assert!(msg.contains("--new-stem"), "认领指引在案：{msg}");
}

#[test]
fn t7_stem_gate_pack_absent_skip_regression() {
    // fixture 域（first 判定目录在而词典包缺席）走 pack_absent_skip（腿一形
    // 回归不破，leg2 T2 金向量同款）。
    let d = make_sweep_domain("stemskip");
    // first 判定在引擎 stem_check_full 用目录级（sih-engine 与 sih-tools 目录
    // 在即真）。
    write_open_input_fixtures(&d.root);
    let (_rc, out, err) = run_lease(&lease_args(&[
        "open",
        "--package",
        "goldc",
        "--root",
        d.root.to_str().unwrap(),
        "--identity",
        d.root.join("identity.json").to_str().unwrap(),
        "--intent",
        d.root.join("intent.json").to_str().unwrap(),
        "--ledger",
        d.ledger.to_str().unwrap(),
        "--locks",
        d.locks.to_str().unwrap(),
        "--bills",
        d.root.join("sih-tools/lease/ledger/bills.ndjson").to_str().unwrap(),
    ]));
    // open 后续身份件缺席拒（exit 2）——但 stem 段须已走 skip 而非闸拒。
    let blob = format!("{out}{err}");
    assert!(
        !blob.contains("未立名新词") && !blob.contains("相撞"),
        "包缺席域零全查：{blob}"
    );
}
