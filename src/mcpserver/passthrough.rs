//! beta 写面 CLI 透传层：既有执法的薄投影（DES-014 第四节正典）。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/writeface/passthrough.py。零新
//! 增判定：参数透传、结果透传、退出码透传。禁令三条：不内嵌 allow/deny 判定
//! 替代执法；不缓存执法结果跨调用复用；不绕过 CLI 直写台账、链文件或 git 对
//! 象。本层零文件写盘（唯一落盘位是正身报告暂存，session 层）。
//!
//! 段2 范围声明：stdio 缺省分支即第一域映射形（layout_for(r, r) first 形），
//! 域布局两形投影随段3 HTTP 面扩展。

use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};

use super::runtime::{code_root, run_readonly, scribe_bin, today_str, RunOutcome};

pub const LEASE_BIN_TIMEOUT: f64 = 600.0;
pub const SCRIBE_TIMEOUT: f64 = 120.0;

fn secs(t: f64) -> std::time::Duration {
    std::time::Duration::from_secs_f64(t)
}

/// 第一域正典台账位（缺省分支形）。
pub fn ledger_paths(root: &Path) -> Map<String, Value> {
    let base = root.join("sih-tools/lease/ledger");
    Map::from_iter([
        ("ledger".to_string(), json!(base.join("sessions.ndjson").display().to_string())),
        ("locks".to_string(), json!(base.join("locks.ndjson").display().to_string())),
        ("claims".to_string(), json!(base.join("claims.ndjson").display().to_string())),
        ("bills".to_string(), json!(base.join("lockface-bills.ndjson").display().to_string())),
    ])
}

/// 段2 缺省布局：数据根自锚即第一域映射形；trail 即引擎域链路径。
pub fn default_trail(root: &Path, date: &str) -> PathBuf {
    root.join("sih-engine/sih/event/trail").join(format!("{date}.ndjson"))
}

/// 路径参数根锚定归一：相对形按域根解析（子进程 cwd 无关，静态归一）。
pub fn anchored(root: &Path, p: &str) -> String {
    if p.is_empty() {
        return p.to_string();
    }
    if Path::new(p).is_absolute() {
        p.to_string()
    } else {
        root.join(p).display().to_string()
    }
}

/// 子进程透传：cwd 承 "/"（与现行形一致），捕获 stdout/stderr，退出码原样。
pub async fn run_cli(argv: &[String], timeout: f64) -> RunOutcome {
    run_readonly(argv, Some(Path::new("/")), None, secs(timeout)).await
}

fn lease_base(root: &Path, sub: &str) -> Vec<String> {
    vec![
        "uv".to_string(),
        "run".to_string(),
        "--project".to_string(),
        code_root().join("sih-tools/lease").display().to_string(),
        "lease".to_string(),
        sub.to_string(),
        "--root".to_string(),
        root.display().to_string(),
    ]
}

fn led_str(root: &Path, key: &str) -> String {
    ledger_paths(root).get(key).and_then(|v| v.as_str()).unwrap_or_default().to_string()
}

// ------------------------------------------------------------------ scribe 面

pub fn scribe_intent_argv(root: &Path, session_id: &str, record: &str,
                          validation: Option<&str>, date: Option<&str>) -> Vec<String> {
    let trail = default_trail(root, &date.map(str::to_string).unwrap_or_else(today_str));
    let mut argv = vec![
        scribe_bin().display().to_string(),
        "intent".to_string(),
        "--record".to_string(),
        anchored(root, record),
    ];
    if let Some(v) = validation {
        argv.push("--validation".to_string());
        argv.push(anchored(root, v));
    }
    argv.extend([
        "--trail".to_string(), trail.display().to_string(),
        "--locks".to_string(), led_str(root, "locks"),
        "--sessions".to_string(), led_str(root, "ledger"),
        "--session".to_string(), session_id.to_string(),
    ]);
    argv
}

pub fn scribe_append_argv(root: &Path, session_id: &str, report: &str,
                          exit_code: i64, date: Option<&str>) -> Vec<String> {
    let trail = default_trail(root, &date.map(str::to_string).unwrap_or_else(today_str));
    vec![
        scribe_bin().display().to_string(),
        "append".to_string(),
        "--report".to_string(), anchored(root, report),
        "--exit-code".to_string(), exit_code.to_string(),
        "--trail".to_string(), trail.display().to_string(),
        "--locks".to_string(), led_str(root, "locks"),
        "--sessions".to_string(), led_str(root, "ledger"),
        "--session".to_string(), session_id.to_string(),
    ]
}

pub fn scribe_park_argv(root: &Path, session_id: &str, record: &str,
                        date: Option<&str>) -> Vec<String> {
    let trail = default_trail(root, &date.map(str::to_string).unwrap_or_else(today_str));
    vec![
        scribe_bin().display().to_string(),
        "park".to_string(),
        "--record".to_string(), anchored(root, record),
        "--trail".to_string(), trail.display().to_string(),
        "--locks".to_string(), led_str(root, "locks"),
        "--session".to_string(), session_id.to_string(),
    ]
}

pub fn scribe_direct_argv(root: &Path, session_id: &str, identity_report: &Path,
                          record: &str, date: Option<&str>) -> Vec<String> {
    let trail = default_trail(root, &date.map(str::to_string).unwrap_or_else(today_str));
    vec![
        scribe_bin().display().to_string(),
        "direct".to_string(),
        "--record".to_string(), anchored(root, record),
        "--trail".to_string(), trail.display().to_string(),
        "--identity-report".to_string(), identity_report.display().to_string(),
        "--sessions".to_string(), led_str(root, "ledger"),
        "--session".to_string(), session_id.to_string(),
    ]
}

// ------------------------------------------------------------------- lease 面

pub fn lease_open_argv(root: &Path, identity_report: &Path, package: &str,
                       intent: &str, repos: &[String], allow: &[String]) -> Vec<String> {
    let mut argv = lease_base(root, "open");
    argv.extend([
        "--package".to_string(), package.to_string(),
        "--identity".to_string(), identity_report.display().to_string(),
        "--intent".to_string(), anchored(root, intent),
        "--ledger".to_string(), led_str(root, "ledger"),
        "--claims".to_string(), led_str(root, "claims"),
        "--locks".to_string(), led_str(root, "locks"),
    ]);
    for r in repos {
        argv.extend(["--repo".to_string(), r.clone()]);
    }
    for a in allow {
        argv.extend(["--allow".to_string(), a.clone()]);
    }
    argv
}

pub fn lease_lock_argv(root: &Path, identity_report: &Path, session_id: &str,
                       path: &str, mode: Option<&str>, wait: bool) -> Vec<String> {
    let mut argv = lease_base(root, "lock");
    argv.extend([
        "--path".to_string(), path.to_string(),
        "--identity".to_string(), identity_report.display().to_string(),
        "--session".to_string(), session_id.to_string(),
        "--ledger".to_string(), led_str(root, "ledger"),
        "--locks".to_string(), led_str(root, "locks"),
    ]);
    if let Some(m) = mode {
        argv.extend(["--mode".to_string(), m.to_string()]);
    }
    if wait {
        argv.push("--wait".to_string());
    }
    argv
}

pub fn lease_unlock_argv(root: &Path, identity_report: &Path, session_id: &str,
                         path: &str) -> Vec<String> {
    let mut argv = lease_base(root, "unlock");
    argv.extend([
        "--path".to_string(), path.to_string(),
        "--identity".to_string(), identity_report.display().to_string(),
        "--session".to_string(), session_id.to_string(),
        "--ledger".to_string(), led_str(root, "ledger"),
        "--locks".to_string(), led_str(root, "locks"),
    ]);
    argv
}

pub fn lease_wait_turn_argv(root: &Path, identity_report: &Path, session_id: &str,
                            path: &str, mode: Option<&str>, timeout_seconds: Option<f64>,
                            interval_seconds: Option<f64>) -> Vec<String> {
    let mut argv = lease_base(root, "wait-turn");
    argv.extend([
        "--path".to_string(), path.to_string(),
        "--identity".to_string(), identity_report.display().to_string(),
        "--session".to_string(), session_id.to_string(),
        "--ledger".to_string(), led_str(root, "ledger"),
        "--locks".to_string(), led_str(root, "locks"),
    ]);
    if let Some(m) = mode {
        argv.extend(["--mode".to_string(), m.to_string()]);
    }
    if let Some(t) = timeout_seconds {
        argv.extend(["--timeout".to_string(), format!("{t}")]);
    }
    if let Some(i) = interval_seconds {
        argv.extend(["--interval".to_string(), format!("{i}")]);
    }
    argv
}

pub fn lease_claim_argv(root: &Path, package: &str, ttl: i64,
                        claimant: Option<&str>) -> Vec<String> {
    let mut argv = lease_base(root, "claim");
    argv.extend([
        "--package".to_string(), package.to_string(),
        "--ttl".to_string(), ttl.to_string(),
        "--claims".to_string(), led_str(root, "claims"),
    ]);
    if let Some(c) = claimant {
        argv.extend(["--claimant".to_string(), c.to_string()]);
    }
    argv
}

pub fn lease_unclaim_argv(root: &Path, package: &str, claimant: Option<&str>) -> Vec<String> {
    let mut argv = lease_base(root, "unclaim");
    argv.extend([
        "--package".to_string(), package.to_string(),
        "--claims".to_string(), led_str(root, "claims"),
    ]);
    if let Some(c) = claimant {
        argv.extend(["--claimant".to_string(), c.to_string()]);
    }
    argv
}

/// 提交 argv：trail 逐条 --trail 透传（相对形按域根锚定归一），root 覆写透传
/// （缺席即域根），force 与 bypass 旗标永不透传。
#[allow(clippy::too_many_arguments)]
pub fn lease_commit_argv(root: &Path, session_id: &str, repo: &str, stage: &str,
                         subject: &str, seq: Option<i64>, cert: Option<&str>,
                         note: Option<&str>, trail: &[String], root_param: Option<&str>) -> Vec<String> {
    let mut argv = lease_base(root, "commit");
    argv.extend([
        "--repo".to_string(), anchored(root, repo),
        "--session".to_string(), session_id.to_string(),
        "--stage".to_string(), stage.to_string(),
        "--subject".to_string(), subject.to_string(),
        "--ledger".to_string(), led_str(root, "ledger"),
        "--root".to_string(), match root_param {
            Some(r) => anchored(root, r),
            None => root.display().to_string(),
        },
    ]);
    for t in trail {
        argv.extend(["--trail".to_string(), anchored(root, t)]);
    }
    if let Some(s) = seq {
        argv.extend(["--seq".to_string(), s.to_string()]);
    }
    if let Some(c) = cert {
        argv.extend(["--cert".to_string(), c.to_string()]);
    }
    if let Some(n) = note {
        argv.extend(["--note".to_string(), n.to_string()]);
    }
    argv
}

/// 收约 argv：light_trail 非 None 即零写连接轻收约形；--force 与 bypass 与
/// ack 旗标永不透传（server 不强拆，人节点裁决位不代行）。
pub fn lease_close_argv(root: &Path, package: &str, reason: Option<&str>,
                        light_trail: Option<&Path>) -> Vec<String> {
    let mut argv = lease_base(root, "close");
    argv.extend([
        "--package".to_string(), package.to_string(),
        "--ledger".to_string(), led_str(root, "ledger"),
        "--locks".to_string(), led_str(root, "locks"),
    ]);
    if let Some(r) = reason {
        argv.extend(["--reason".to_string(), r.to_string()]);
    }
    if let Some(t) = light_trail {
        argv.extend(["--trail".to_string(), t.display().to_string()]);
    }
    argv
}

/// 结果透传载荷：退出码与出参原样载出，出参可解析 JSON 时并载解析形。
pub fn passthrough_result(tool: &str, session_id: Option<&str>, rc: i32,
                          stdout: &str, stderr: &str) -> Value {
    let strip = stdout.trim();
    let parsed: Value = if strip.starts_with('{') || strip.starts_with('[') {
        serde_json::from_str(strip).unwrap_or(Value::Null)
    } else {
        Value::Null
    };
    json!({
        "tool": tool,
        "session_id": session_id,
        "exit_code": rc,
        "stdout": stdout,
        "stderr": stderr,
        "result": parsed,
    })
}

/// lease open 出参会话号提取（既有出参投影，零判定）。
pub fn parse_session_id(stdout: &str) -> Option<String> {
    let strip = stdout.trim();
    let parsed: Value = serde_json::from_str(strip).ok()?;
    parsed.get("session_id").and_then(|s| s.as_str()).map(str::to_owned)
}
