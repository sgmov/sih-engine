//! lease-mergeleg6-parallel 簇I T6：locksview 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/locks 0.2.0（cli.py、core.py，basefix-solo 修后形）。
//! 三例金向量：正常（acquire 幂等与 status 投影与 release）、拒绝（五验与锁态
//! 各拦截理由码）、边界（乐观链假级联双路与单遍配对互斥不绕穿）。fixture 全部
//! temp 自建；bin 名 locksview 避让引擎 lease 内 locks 语义位（bin 头注申报）。

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_locksview")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn run_json(args: &[&str]) -> (i32, Value, String) {
    let (code, out, err) = run(args);
    (code, serde_json::from_str(&out).unwrap_or(Value::Null), err)
}

struct Fx {
    root: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    let root = guard.path().to_path_buf();
    fs::create_dir_all(root.join("worktrees/.bindings")).unwrap();
    Fx { root, _guard: guard }
}

impl Fx {
    fn p(&self, rel: &str) -> String {
        self.root.join(rel).display().to_string()
    }

    /// 会话档 issued 行入工地台账；绑定侧档三稳定子集同写入。
    fn issue_session(&self, sid: &str, allow: &[&str]) {
        let ledger = self.p("sessions.ndjson");
        let line = serde_json::json!({
            "event": "issued",
            "session_id": sid,
            "allow": allow,
        });
        let mut text = fs::read_to_string(&ledger).unwrap_or_default();
        text.push_str(&format!("{}\n", line));
        fs::write(&ledger, text).unwrap();
        let binding = self.root.join("worktrees/.bindings").join(format!("{sid}.json"));
        fs::write(
            &binding,
            serde_json::json!({
                "hostname": "testhost",
                "user": "testuser",
                "boottime": "2026-09-13T00:00:00+00:00",
            })
            .to_string(),
        )
        .unwrap();
    }

    fn identity(&self) -> String {
        let path = self.root.join("identity.json");
        fs::write(
            &path,
            serde_json::json!({
                "anomalies": [],
                "observed": {
                    "hostname": "testhost",
                    "user": "testuser",
                    "boottime": "2026-09-13T00:00:00+00:00",
                },
                "identity": {"hash": "a".repeat(64)},
            })
            .to_string(),
        )
        .unwrap();
        path.display().to_string()
    }

    fn acquire(&self, sid: &str, path: &str, at: &str) -> Vec<String> {
        vec![
            "acquire".into(),
            "--path".into(), path.into(),
            "--identity".into(), self.identity(),
            "--session".into(), sid.into(),
            "--at".into(), at.into(),
            "--root".into(), self.root.display().to_string(),
            "--locks".into(), self.p("locks.ndjson"),
            "--ledger".into(), self.p("sessions.ndjson"),
        ]
    }

    fn release(&self, sid: &str, path: &str, at: &str) -> Vec<String> {
        vec![
            "release".into(),
            "--path".into(), path.into(),
            "--identity".into(), self.identity(),
            "--session".into(), sid.into(),
            "--at".into(), at.into(),
            "--root".into(), self.root.display().to_string(),
            "--locks".into(), self.p("locks.ndjson"),
            "--ledger".into(), self.p("sessions.ndjson"),
        ]
    }

    fn locks_lines(&self) -> Vec<String> {
        fs::read_to_string(self.root.join("locks.ndjson"))
            .unwrap_or_default()
            .lines()
            .map(|s| s.to_string())
            .collect()
    }
}

// ---------- 正常形：五验全过取锁、同会话幂等、status 投影、放锁 ----------

#[test]
fn t1_normal_acquire_status_release() {
    let fx = build_fx();
    fx.issue_session("s1", &["sih-engine/doc/", "sih-tools/"]);

    // acquire 全链过：acquired 行落地
    let args = fx.acquire("s1", "sih-engine/doc/a.md", "2026-09-13T08:00:00+00:00");
    let (code, v, err) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "acquire 须 0，err={err}");
    assert_eq!(v["duplicate"], false);
    assert_eq!(v["line"]["event"], "acquired");
    assert_eq!(v["line"]["session_id"], "s1");
    assert_eq!(v["line"]["path"], "sih-engine/doc/a.md");
    assert_eq!(v["line"]["tool"]["name"], "locks");
    assert_eq!(v["line"]["tool"]["version"], "0.2.0");

    // 锁台账行：sort_keys 紧凑形逐字节金向量（--at 复演注入即确定性）
    let lines = fx.locks_lines();
    assert_eq!(lines.len(), 1);
    assert_eq!(
        lines[0],
        format!(
            "{{\"acquired_at\":\"2026-09-13T08:00:00+00:00\",\"event\":\"acquired\",\"path\":\"sih-engine/doc/a.md\",\"session_id\":\"s1\",\"tool\":{{\"name\":\"locks\",\"version\":\"0.2.0\"}}}}"
        ),
        "锁台账行紧凑金向量，got={}",
        lines[0]
    );

    // 同会话重复 acquire：记行不拦即幂等留痕 duplicate=true
    let args = fx.acquire("s1", "sih-engine/doc/a.md", "2026-09-13T08:01:00+00:00");
    let (code, v, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    assert_eq!(v["duplicate"], true);
    assert_eq!(fx.locks_lines().len(), 2);

    // status：在锁两行按 (path, acquired_at) 序，summary 计数
    let (code, v, _) = run_json(&[
        "status", "--root", &fx.root.display().to_string(), "--locks", &fx.p("locks.ndjson"),
    ]);
    assert_eq!(code, 0);
    assert_eq!(v["summary"]["held"], 2);
    assert_eq!(v["held"].as_array().unwrap().len(), 2);
    assert_eq!(v["header"]["tool"]["version"], "0.2.0");

    // release：同五验加持锁验，released 行落地
    let args = fx.release("s1", "sih-engine/doc/a.md", "2026-09-13T09:00:00+00:00");
    let (code, v, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    assert_eq!(v["line"]["event"], "released");

    // 放锁后 status 归零
    let (code, v, _) = run_json(&[
        "status", "--root", &fx.root.display().to_string(), "--locks", &fx.p("locks.ndjson"),
    ]);
    assert_eq!(code, 0);
    assert_eq!(v["summary"]["held"], 0);
}

// ---------- 拒绝形：五验理由码与锁态拦截、工具异常 ----------

#[test]
fn t2_reject_forms() {
    let fx = build_fx();
    fx.issue_session("s1", &["sih-engine/doc/"]);
    fx.issue_session("s2", &["sih-engine/doc/"]);
    let id = fx.identity();

    // s1 取锁
    let args = fx.acquire("s1", "sih-engine/doc/a.md", "2026-09-13T08:00:00+00:00");
    let (code, _, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);

    // 他会话持锁：locked_elsewhere 拦（退出码一）
    let args = fx.acquire("s2", "sih-engine/doc/a.md", "2026-09-13T08:01:00+00:00");
    let (code, v, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert_eq!(v["error"], "locked_elsewhere");
    assert_eq!(v["detail"]["holder"], "s1");

    // 范围外：scope_violation 拦
    let args = fx.acquire("s2", "sih-visual/logo.svg", "2026-09-13T08:02:00+00:00");
    let (code, v, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert_eq!(v["error"], "scope_violation");

    // 绑定侧档缺席：binding_absent 拦
    fs::remove_file(fx.root.join("worktrees/.bindings/s2.json")).unwrap();
    let args = fx.acquire("s2", "sih-engine/doc/b.md", "2026-09-13T08:03:00+00:00");
    let (code, v, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert_eq!(v["error"], "binding_absent");
    fx.issue_session("s2", &["sih-engine/doc/"]);

    // 身份异常点：identity_anomaly 拦
    let bad_id = fx.root.join("bad-identity.json");
    fs::write(
        &bad_id,
        serde_json::json!({
            "anomalies": ["hash_mismatch"],
            "observed": {
                "hostname": "testhost",
                "user": "testuser",
                "boottime": "2026-09-13T00:00:00+00:00",
            },
            "identity": {"hash": "a".repeat(64)},
        })
        .to_string(),
    )
    .unwrap();
    let (code, v, _) = run_json(&[
        "acquire", "--path", "sih-engine/doc/b.md", "--identity", bad_id.display().to_string().as_str(),
        "--session", "s2", "--root", &fx.root.display().to_string(),
        "--locks", &fx.p("locks.ndjson"), "--ledger", &fx.p("sessions.ndjson"),
    ]);
    assert_eq!(code, 1);
    assert_eq!(v["error"], "identity_anomaly");

    // 绑定漂移：hostname_drift 拦
    let drift_id = fx.root.join("drift-identity.json");
    fs::write(
        &drift_id,
        serde_json::json!({
            "anomalies": [],
            "observed": {
                "hostname": "otherhost",
                "user": "testuser",
                "boottime": "2026-09-13T00:00:00+00:00",
            },
            "identity": {"hash": "a".repeat(64)},
        })
        .to_string(),
    )
    .unwrap();
    let (code, v, _) = run_json(&[
        "acquire", "--path", "sih-engine/doc/b.md", "--identity", drift_id.display().to_string().as_str(),
        "--session", "s2", "--root", &fx.root.display().to_string(),
        "--locks", &fx.p("locks.ndjson"), "--ledger", &fx.p("sessions.ndjson"),
    ]);
    assert_eq!(code, 1);
    assert_eq!(v["error"], "hostname_drift");

    // 零在册：no_active_session 拦
    let empty_ledger = fx.p("empty-sessions.ndjson");
    let (code, v, _) = run_json(&[
        "acquire", "--path", "sih-engine/doc/b.md", "--identity", &id,
        "--root", &fx.root.display().to_string(),
        "--locks", &fx.p("locks.ndjson"), "--ledger", &empty_ledger,
    ]);
    assert_eq!(code, 1);
    assert_eq!(v["error"], "no_active_session");

    // 非持锁者放锁：not_holder 拦；无锁放锁：not_locked 拦
    let args = fx.release("s2", "sih-engine/doc/a.md", "2026-09-13T08:04:00+00:00");
    let (code, v, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert_eq!(v["error"], "not_holder");
    let args = fx.release("s2", "sih-engine/doc/zz.md", "2026-09-13T08:05:00+00:00");
    let (code, v, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert_eq!(v["error"], "not_locked");

    // 工具异常：会话档不可解析（多在册无参 → 2；坏 json → 2）
    let multi = fx.p("multi.ndjson");
    fs::write(&multi, format!(
        "{{\"event\":\"issued\",\"session_id\":\"x1\",\"allow\":[]}}\n{{\"event\":\"issued\",\"session_id\":\"x2\",\"allow\":[]}}\n"
    )).unwrap();
    let (code, _, _) = run_json(&[
        "acquire", "--path", "sih-engine/doc/b.md", "--identity", &id,
        "--root", &fx.root.display().to_string(),
        "--locks", &fx.p("locks.ndjson"), "--ledger", &multi,
    ]);
    assert_eq!(code, 2, "多在册无参须工具异常 2");
    let broken = fx.p("broken.ndjson");
    fs::write(&broken, "{not json}\n").unwrap();
    let (code, v, _) = run_json(&[
        "acquire", "--path", "sih-engine/doc/b.md", "--identity", &id,
        "--root", &fx.root.display().to_string(),
        "--locks", &fx.p("locks.ndjson"), "--ledger", &broken,
    ]);
    assert_eq!(code, 2);
    assert!(v["error"].as_str().unwrap().contains("invalid"));

    // 用法错：缺 --path → 2
    let (code, _, err) = run(&[
        "acquire", "--identity", &id, "--root", &fx.root.display().to_string(),
    ]);
    assert_eq!(code, 2);
    assert!(err.contains("用法错"));
}

// ---------- 边界形：乐观链假级联双路与单遍配对互斥不绕穿 ----------

#[test]
fn t3_edge_optimistic_and_pairing() {
    let fx = build_fx();
    fx.issue_session("s1", &["sih-engine/doc/"]);
    let root_s = fx.root.display().to_string();
    let locks_s = fx.p("locks.ndjson");
    let common: [&str; 4] = ["--root", root_s.as_str(), "--locks", locks_s.as_str()];

    // 假 cascade 可执行 shim：经 CASCADE_BIN_OVERRIDE 覆盖位注入（locksview
    // 乐观链已直调引擎 cascade bin，不再经 uv），按 $FAKE_CASCADE 吐三样报告
    // （未知目标 → 净 → 脏）
    let fakebin = fx.root.join("fakebin");
    fs::create_dir_all(&fakebin).unwrap();
    let cascade = fakebin.join("cascade");
    fs::write(
        &cascade,
        "#!/bin/sh\ncase \"$FAKE_CASCADE\" in\n\
         unknown) echo '{\"targets\": {}}' ;;\n\
         dirty) echo '{\"targets\": {\"b.md\": {\"dirty\": [\"cascade/old.md\"]}}}' ;;\n\
         *) echo '{\"targets\": {\"a.md\": {\"dirty\": []}}}' ;;\nesac\n",
    )
    .unwrap();
    fs::set_permissions(&cascade, fs::Permissions::from_mode(0o755)).unwrap();
    let run_with_fake = |fake: &str, args: &[&str]| {
        Command::new(bin())
            .args(args)
            .env("CASCADE_BIN_OVERRIDE", cascade.display().to_string())
            .env("FAKE_CASCADE", fake)
            .output()
            .unwrap()
    };

    // 语料前缀外 → 工具异常 2（写面封闭，未触子进程）
    let (code, v, _) = run_json(&[
        "check", "--path", "sih-visual/logo.svg", "--cascade-dir", &fakebin.display().to_string(),
        common[0], common[1], common[2], common[3],
    ]);
    assert_eq!(code, 2);
    assert!(v["error"].as_str().unwrap().contains("sih-engine/doc/"));
    assert_eq!(fx.locks_lines().len(), 0, "前缀外不落 checked 行");

    // unknown_target：不在册过注记（退出码零，checked 行留痕）
    let out = run_with_fake("unknown", &[
        "check", "--path", "sih-engine/doc/ghost.md", "--cascade-dir", &fakebin.display().to_string(),
        common[0], common[1], common[2], common[3],
    ]);
    assert_eq!(out.status.code(), Some(0), "stdout={}", String::from_utf8_lossy(&out.stdout));
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["verdict"], "unknown_target");
    assert_eq!(fx.locks_lines().len(), 1);
    assert!(fx.locks_lines()[0].contains("\"event\":\"checked\""));

    // writable：净上游过
    let out = run_with_fake("clean", &[
        "check", "--path", "sih-engine/doc/a.md", "--cascade-dir", &fakebin.display().to_string(),
        common[0], common[1], common[2], common[3],
    ]);
    assert_eq!(out.status.code(), Some(0));
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["verdict"], "writable");
    assert_eq!(v["dirty"].as_array().unwrap().len(), 0);

    // blocked：脏上游拦（退出码一，dirty 清单入报告，checked 行留痕）
    let out = run_with_fake("dirty", &[
        "check", "--path", "sih-engine/doc/b.md", "--cascade-dir", &fakebin.display().to_string(),
        common[0], common[1], common[2], common[3],
    ]);
    assert_eq!(out.status.code(), Some(1));
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["verdict"], "blocked");
    assert_eq!(v["error"], "dirty_upstream");
    assert_eq!(v["dirty"][0], "cascade/old.md");
    assert_eq!(fx.locks_lines().len(), 3);

    // 单遍事件序配对（basefix-solo 修后形）：放后重取，锁面读出持位且他会被拦
    let args = fx.acquire("s1", "sih-engine/doc/c.md", "2026-09-13T10:00:00+00:00");
    let (code, _, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    let args = fx.release("s1", "sih-engine/doc/c.md", "2026-09-13T10:01:00+00:00");
    let (code, _, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    let args = fx.acquire("s1", "sih-engine/doc/c.md", "2026-09-13T10:02:00+00:00");
    let (code, _, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);

    // status 行面投影：围堰 lock_status 按 acquired 行逐行对表 held 映射，
    // 同会话放后重取形两行均持位（不去重，行面投影语义）；互斥判定在
    // active_locks 单遍配对，见下方 locked_elsewhere 断言。
    let (code, v, _) = run_json(&[
        "status", "--path", "sih-engine/doc/c.md",
        "--root", &fx.root.display().to_string(), "--locks", &fx.p("locks.ndjson"),
    ]);
    assert_eq!(code, 0);
    let held = v["held"].as_array().unwrap();
    assert_eq!(held.len(), 2, "行面投影含重取前后两行，got={v}");
    assert_eq!(held[0]["acquired_at"], "2026-09-13T10:00:00+00:00");
    assert_eq!(held[1]["acquired_at"], "2026-09-13T10:02:00+00:00");
    assert_eq!(v["summary"]["held"], 2);

    // 他会话再取被拦：互斥不绕穿
    fx.issue_session("s2", &["sih-engine/doc/"]);
    let args = fx.acquire("s2", "sih-engine/doc/c.md", "2026-09-13T10:03:00+00:00");
    let (code, v, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert_eq!(v["error"], "locked_elsewhere");
}
