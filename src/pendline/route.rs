//! route 面（pl-03）：判词三态路由与落链。
//!
//! 承 DES-017 编排三段之三「处置路由」与泊位通道：读回填评分材料（或
//! attractor 判词材料形），判词三态机械判据直读 verdict/gate_verdict 字段
//!（stable_clear=过、boundary=boundary、其余=未过），零新判定语义——
//! 编排层只读值路由，不计算不重测。
//!
//! 三态处置（全路由留痕可对账，写链一律经 scribe 二进制子进程，零直接写
//! trail 文件）：
//! - 过者（stable_clear）：执行位标记——建议通道字段 channel（batch|
//!   direct_edit），链上留痕走 scribe direct 直改笔（subject=execution_
//!   marked，pen_kind=agent 强制 --identity-report 正身件，无会话形
//!   --no-session-reason，对表 idenlane 直改笔形）。
//! - boundary 者：人重写标记（scribe direct，subject=human_rewrite_marked），
//!   禁自动重测禁自动入泊——代码结构保证 boundary 分支不触 park 不触重测。
//! - 未过者：scribe park 停泊事件落链（title/exit_condition/ttl_days/
//!   context 照录判词要害；park 无会话闸，对表 scribe 源码）。
//!
//! 落链事件类型注记：scribe 二进制事件型固定（direct 形即
//! direct_edit_completed），编排标记名承载于直改笔 details.subject 字段，
//! 链上断言面即 event_type + subject 双键。

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

use super::{read_json, sanitize, write_json, Args, PendError};

pub const REPORT_KIND: &str = "pendline-route-report";
pub const REPORT_VERSION: i64 = 1;
pub const DEFAULT_TTL_DAYS: i64 = 30;

/// 路由三态。
#[derive(Debug, PartialEq)]
enum Disposition {
    Passed,
    Boundary,
    Failed,
}

/// 子命令入口。
pub fn run(args: &Args) -> Result<i32, PendError> {
    let materials = args.list("materials");
    if materials.is_empty() {
        return Err(PendError::usage("route 缺 --materials <回填评分材料.json>..."));
    }
    let scribe_binary = args.get("scribe-binary")?;
    let trail = args.get("trail")?;
    let out_path = args.opt("out");
    let ttl_default: i64 = match args.opt("ttl-days") {
        Some(v) => v
            .parse()
            .map_err(|_| PendError::usage(format!("--ttl-days 须为整数：{}", v)))?,
        None => DEFAULT_TTL_DAYS,
    };
    let dry_run = args.flag("dry-run");
    let identity_report = args.opt("identity-report");
    let session = args.opt("session");
    let sessions = args.opt("sessions");
    let locks = args.opt("locks");
    let no_session_reason = args.opt("no-session-reason");

    if session.is_some() && sessions.is_none() {
        return Err(PendError::usage("--session 须配 --sessions <会话台账>（scribe 会话在册验）"));
    }

    // 预读全材料：判词直读与前置校验（零写入先行，避免半途环境错）。
    let mut parsed: Vec<(String, Value)> = Vec::new();
    for m in &materials {
        let v = read_json(Path::new(m))?;
        read_verdict(&v).ok_or_else(|| {
            PendError::usage(format!("评分材料缺 verdict/gate_verdict 字段：{}", m))
        })?;
        parsed.push((m.clone(), v));
    }
    // 前置校验：执行位与 boundary 标记走直改笔，须正身件与会话席（会话或
    // 无会话事由二择一）。未过入泊走 scribe park 无会话闸，不前置强求。
    let needs_mark = parsed
        .iter()
        .any(|(_, v)| matches!(read_verdict(v), Some("stable_clear") | Some("boundary")));
    if needs_mark && !dry_run {
        if identity_report.is_none() {
            return Err(PendError::usage(
                "route 有过者或 boundary 者须 --identity-report <正身报告.json>（直改笔 agent 形强制）",
            ));
        }
        if session.is_none() && no_session_reason.is_none() {
            return Err(PendError::usage(
                "route 有过者或 boundary 者须 --session <会话号> 或 --no-session-reason <事由> 二择一（scribe 会话闸）",
            ));
        }
    }

    let mut results: Vec<Value> = Vec::new();
    let mut failures = 0usize;
    for (mpath, material) in &parsed {
        let verdict = read_verdict(&material).unwrap_or_default().to_string();
        let disposition = classify(&verdict);
        let gid = material
            .get("gid")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let result = match disposition {
            Disposition::Passed => mark_and_report(
                &material, &mpath, &gid, "execution_marked", &scribe_binary, &trail,
                &identity_report, &session, &sessions, &locks, &no_session_reason, dry_run,
            ),
            Disposition::Boundary => mark_and_report(
                &material, &mpath, &gid, "human_rewrite_marked", &scribe_binary, &trail,
                &identity_report, &session, &sessions, &locks, &no_session_reason, dry_run,
            ),
            Disposition::Failed => park_and_report(
                &material, &mpath, &gid, &verdict, &scribe_binary, &trail, &locks, ttl_default,
                dry_run,
            ),
        };
        match result {
            Ok(mut r) => {
                r["verdict"] = json!(verdict);
                results.push(r);
            }
            Err(e) => {
                failures += 1;
                results.push(json!({
                    "gid": gid,
                    "verdict": verdict,
                    "disposition": disposition_label(&disposition),
                    "ok": false,
                    "error": e.message,
                }));
            }
        }
    }

    let report = json!({
        "kind": REPORT_KIND,
        "version": REPORT_VERSION,
        "trail": trail,
        "dry_run": dry_run,
        "routed": results.len(),
        "failures": failures,
        "results": results,
    });
    if let Some(op) = &out_path {
        write_json(Path::new(op), &report)?;
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&report).unwrap_or_default()
    );
    if failures > 0 {
        Ok(1)
    } else {
        Ok(0)
    }
}

/// 判词直读：gate_verdict 优先（三值闸词表），回落 verdict 字段；均缺席
/// 即 None。零计算零判定。
fn read_verdict(material: &Value) -> Option<&str> {
    material
        .get("gate_verdict")
        .or_else(|| material.get("verdict"))
        .and_then(|v| v.as_str())
}

fn classify(verdict: &str) -> Disposition {
    match verdict {
        "stable_clear" => Disposition::Passed,
        "boundary" => Disposition::Boundary,
        _ => Disposition::Failed,
    }
}

fn disposition_label(d: &Disposition) -> &'static str {
    match d {
        Disposition::Passed => "passed",
        Disposition::Boundary => "boundary",
        Disposition::Failed => "failed",
    }
}

/// 执行位标记与 boundary 人重写标记：scribe direct 直改笔落链。
#[allow(clippy::too_many_arguments)]
fn mark_and_report(
    material: &Value,
    material_path: &str,
    gid: &str,
    subject: &str,
    scribe_binary: &str,
    trail: &str,
    identity_report: &Option<String>,
    session: &Option<String>,
    sessions: &Option<String>,
    locks: &Option<String>,
    no_session_reason: &Option<String>,
    dry_run: bool,
) -> Result<Value, PendError> {
    let contract_path = material.get("contract_path").and_then(|v| v.as_str()).unwrap_or("");
    let responses_path = material.get("responses_path").and_then(|v| v.as_str()).unwrap_or("");
    let channel = match material.get("channel").and_then(|v| v.as_str()) {
        Some(c) if c == "batch" || c == "direct_edit" => c.to_string(),
        _ => "batch".to_string(),
    };
    let is_boundary = subject == "human_rewrite_marked";
    let mut files: Vec<String> = Vec::new();
    for p in [contract_path, responses_path, material_path] {
        if !p.is_empty() {
            files.push(p.to_string());
        }
    }
    if dry_run {
        return Ok(json!({
            "gid": gid,
            "disposition": if is_boundary { "boundary" } else { "passed" },
            "action": subject,
            "channel": channel,
            "dry_run": true,
            "scribe_subcommand": "direct",
            "chain_event_type": "direct_edit_completed",
            "chain_subject": subject,
            "ok": true,
        }));
    }
    // 直改笔记录件：pen_kind=agent（编排者为 agent），标记名在 subject，
    // 对表 scribe direct 既有调用面（pen_form 必为 direct）。
    let record = json!({
        "pen_form": "direct",
        "pen_kind": "agent",
        "actor_id": "pendline",
        "files": files,
        "subject": subject,
    });
    let scratch = mark_record_path(gid, subject);
    write_json(&scratch, &record)?;
    let mut cmd = Command::new(scribe_binary);
    cmd.args([
        "direct",
        "--record",
        &scratch.display().to_string(),
        "--trail",
        trail,
        "--identity-report",
        identity_report.as_deref().unwrap_or_default(),
    ]);
    if let Some(s) = session {
        cmd.args(["--session", s]);
        if let Some(sp) = sessions {
            cmd.args(["--sessions", sp]);
        }
    } else if let Some(r) = no_session_reason {
        cmd.args(["--no-session-reason", r]);
    }
    if let Some(l) = locks {
        cmd.args(["--locks", l]);
    }
    let out = run_capture(cmd)?;
    let rc = out.status.code().unwrap_or(-1);
    if rc != 0 {
        // scribe 错误信封出 stdout（emit 形），stderr 一并附录不静默。
        return Err(PendError::violate(format!(
            "scribe direct 非零退出 {}：stdout={} stderr={}",
            rc,
            String::from_utf8_lossy(&out.stdout).trim(),
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    let envelope = parse_envelope(&out.stdout);
    Ok(json!({
        "gid": gid,
        "disposition": if is_boundary { "boundary" } else { "passed" },
        "action": subject,
        "channel": channel,
        "dry_run": false,
        "scribe_subcommand": "direct",
        "scribe_exit": rc,
        "chain_event_type": "direct_edit_completed",
        "chain_subject": subject,
        "event_id": envelope.get("event_id").cloned().unwrap_or(Value::Null),
        "event_hash": envelope.get("event_hash").cloned().unwrap_or(Value::Null),
        "ok": true,
    }))
}

/// 未过入泊：scribe park 停泊事件落链，title/exit_condition/ttl_days/
/// context 照录判词要害（材料可覆写，缺省机械派生）。
#[allow(clippy::too_many_arguments)]
fn park_and_report(
    material: &Value,
    material_path: &str,
    gid: &str,
    verdict: &str,
    scribe_binary: &str,
    trail: &str,
    locks: &Option<String>,
    ttl_default: i64,
    dry_run: bool,
) -> Result<Value, PendError> {
    let entry_id = material
        .get("entry_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("pendline-{}", sanitize(gid)));
    let title = material
        .get("title")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| gid.to_string());
    let exit_condition = material
        .get("exit_condition")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("用户裁定 {} 处置（复测通过或消项）后出泊", gid));
    let ttl_days = material.get("ttl_days").and_then(|v| v.as_i64()).unwrap_or(ttl_default);
    if ttl_days <= 0 {
        return Err(PendError::usage(format!("ttl_days 须为正整数，得到 {}", ttl_days)));
    }
    let context = material.get("context").cloned().unwrap_or_else(|| {
        json!({
            "verdict": verdict,
            "gid": gid,
            "material": material_path,
            "source": "pendline-route",
        })
    });
    if dry_run {
        return Ok(json!({
            "gid": gid,
            "disposition": "failed",
            "action": "parking_entered",
            "entry_id": entry_id,
            "dry_run": true,
            "scribe_subcommand": "park",
            "chain_event_type": "parking_entered",
            "ok": true,
        }));
    }
    let record = json!({
        "action": "enter",
        "entry_id": entry_id,
        "title": title,
        "exit_condition": exit_condition,
        "ttl_days": ttl_days,
        "context": context,
    });
    let scratch = mark_record_path(gid, "park");
    write_json(&scratch, &record)?;
    let mut cmd = Command::new(scribe_binary);
    cmd.args([
        "park",
        "--record",
        &scratch.display().to_string(),
        "--trail",
        trail,
    ]);
    if let Some(l) = locks {
        cmd.args(["--locks", l]);
    }
    let out = run_capture(cmd)?;
    let rc = out.status.code().unwrap_or(-1);
    if rc != 0 {
        return Err(PendError::violate(format!(
            "scribe park 非零退出 {}：stdout={} stderr={}",
            rc,
            String::from_utf8_lossy(&out.stdout).trim(),
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    let envelope = parse_envelope(&out.stdout);
    Ok(json!({
        "gid": gid,
        "disposition": "failed",
        "action": "parking_entered",
        "entry_id": entry_id,
        "ttl_days": ttl_days,
        "dry_run": false,
        "scribe_subcommand": "park",
        "scribe_exit": rc,
        "chain_event_type": "parking_entered",
        "event_id": envelope.get("event_id").cloned().unwrap_or(Value::Null),
        "event_hash": envelope.get("event_hash").cloned().unwrap_or(Value::Null),
        "ok": true,
    }))
}

fn run_capture(mut cmd: Command) -> Result<std::process::Output, PendError> {
    cmd.output()
        .map_err(|e| PendError::usage(format!("scribe 子进程不可启：{}", e)))
}

/// scribe 写回执解析：stdout 单个 JSON 信封（status/event_id/event_hash）。
fn parse_envelope(stdout: &[u8]) -> Value {
    let text = String::from_utf8_lossy(stdout);
    serde_json::from_str(text.trim()).unwrap_or(Value::Null)
}

/// 标记与停泊记录暂存位：系统临时目录，进程号加原子序隔离（测试进程内
/// 多并发 route 各得独立记录件）。
fn mark_record_path(gid: &str, kind: &str) -> PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static SEQ: AtomicUsize = AtomicUsize::new(0);
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("pendline-route-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    dir.join(format!("{}-{}-{}.json", sanitize(gid), kind, n))
}
