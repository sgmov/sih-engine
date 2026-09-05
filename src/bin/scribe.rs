//! scribe 即书简融回命令行面即本名回滚承 DEC-017 修订二，承接 SPEC-006#boundary 与 T6。
//!
//! 子命令六件即 append、verify、query、intent、park、record，另携 vectors 冻结向量集。
//! record 即秤星读数落链入口，七字段守卫承 SPEC-011，只校形不判值。
//! 退出码三值即零成功、一校验或检索异常、二工具自身异常。

use chrono::Utc;
use serde_json::json;
use sih_engine::event_stream::event::{Actor, ActorType};
use sih_engine::event_stream::park::load_parking_scope;
use sih_engine::event_stream::{
    append, certification_event, check_intent_reused, check_locks, check_session,
    compute_event_hash, intent_event, load_events, AppendError,
    crosscheck_event, lockgate::LockGateError, park_event, query,
    reading_event, sessiongate::SessionGateError, verify, Event, EventFilter, EventInput, VerifyRange,
    GENESIS_PREV_HASH,
};
use std::path::{Path, PathBuf};
use std::process::exit;

fn gate_actor() -> Actor {
    Actor {
        actor_id: "scribe".to_string(),
        actor_type: ActorType::System,
        invoked_via: "cli".to_string(),
    }
}

fn read_text(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn emit(value: serde_json::Value, code: i32) -> ! {
    println!("{}", serde_json::to_string_pretty(&value).unwrap());
    exit(code)
}

fn load_store(trail: &str) -> Vec<Event> {
    let path = PathBuf::from(trail);
    if !path.exists() {
        return Vec::new();
    }
    match load_events(path.as_path()) {
        Ok(events) => events,
        Err(e) => emit(json!({"error": format!("trail 不可读 {e:?}")}), 2),
    }
}

/// 锁位前查即他会话持锁拒写，承 lockguard-solo 批。
///
/// --locks 缺席即零查旧行为逐字节不变；在场即对写命令 trail 路径前查，
/// 他会话 active 锁拒写退出码一并报持锁方，本会话持锁或零锁放行，
/// 台账不可读退出码二不静默。--session 即调用方会话号用于本会话放行。
fn lockgate_guard(opts: &std::collections::HashMap<String, String>, trail: &str) {
    let Some(locks) = opts.get("locks") else { return };
    match check_locks(
        Path::new(locks),
        Path::new(trail),
        opts.get("session").map(|s| s.as_str()),
    ) {
        Ok(()) => {}
        Err(LockGateError::Held { holders }) => {
            emit(json!({"error": "链路径在他会话锁下", "holders": holders}), 1)
        }
        Err(LockGateError::Unreadable(e)) => {
            emit(json!({"error": format!("锁台账不可读 {e}")}), 2)
        }
    }
}

/// 工地链副本禁追加护栏：--trail 物理路径含 worktrees/ 段即拒退出码二。
///
/// 认证必须落主树活链，链文件 settle 前一次性拷工地，严禁工地链副本追加
/// 致主链同位双 valid 续链分叉（entryunique 与 mathpipe-a3 教训）。判定按
/// 规范路径组件扫描 worktrees 段，worker 目录即绝对路径态。显式覆写旗标
/// --allow-worktree-trail 默认关，仅应急位显式开启。
fn worktree_trail_guard(opts: &std::collections::HashMap<String, String>, trail: &str) {
    let path = PathBuf::from(trail);
    let contains_worktree = path
        .components()
        .any(|c| c.as_os_str() == "worktrees");
    if !contains_worktree {
        return;
    }
    if opts.get("allow-worktree-trail").map(|s| s == "1" || s == "true").unwrap_or(false) {
        return;
    }
    emit(json!({"error": "工地链副本禁追加即认证先落主链"}), 2);
}

/// 会话在册前查：--session 须在会话台账为活跃，无或已吊销即拒 SessionNotActive。
///
/// 承 guardrail2-solo 闸三，intent 与 append 认证两写命令共用。--sessions 即
/// 会话台账路径，--session 即调用方会话号，--no-session-reason 主会处置位缺省关。
/// 处置位开启且无会话号即放行行事由落链；有会话号须台账在册活跃否则拒。
fn session_guard(opts: &std::collections::HashMap<String, String>) -> Option<String> {
    let session = opts.get("session").map(|s| s.as_str());
    let reason = opts.get("no-session-reason").cloned();
    if session.is_none() && reason.is_some() {
        return reason; // 主会处置位：无会话号但显式给事由即放行，由写分支落链。
    }
    if session.is_none() {
        emit(
            json!({"error": "会话不在册即拒 SessionNotActive：缺 --session，主会处置位 --no-session-reason <事由> 默认关"}),
            1,
        );
    }
    let Some(sessions) = opts.get("sessions") else {
        emit(json!({"error": "会话在册验需 --sessions <会话台账路径>"}), 2);
    };
    match check_session(Path::new(sessions), session, None) {
        Ok(_) => None,
        Err(SessionGateError::NotActive { session_id }) => {
            emit(
                json!({"error": "会话不在册即拒 SessionNotActive", "session_id": session_id}),
                1,
            )
        }
        Err(SessionGateError::Unreadable(e)) => {
            emit(json!({"error": format!("会话台账不可读 {e}")}), 2)
        }
    }
}

/// 链上正身绑定解析（idenlane-solo iden-01）：按会话号从会话台账查得
/// 生产者身份哈希落事件信封，零新增必填参数（--session 与 --sessions
/// 即闸三既有参）。单遍事件序语义同 check_session：issued 行载该会话
/// 当前身份（最新 issued 在册覆盖），revoked 即不在册。会话号或台账
/// 缺席即 None（直改笔位另以身份件解析）；台账不可读即工具异常不静默。
fn resolve_envelope(
    sessions: Option<&str>,
    session: Option<&str>,
) -> Result<Option<(String, Option<String>)>, String> {
    let Some(sid) = session else { return Ok(None) };
    let Some(sp) = sessions else { return Ok(None) };
    let text = std::fs::read_to_string(sp).map_err(|e| format!("会话台账不可读 {e}"))?;
    let mut identity: Option<String> = None;
    let mut active = false;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let row: serde_json::Value = serde_json::from_str(line).map_err(|_| "会话台账行非法".to_string())?;
        if row.get("session_id").and_then(|v| v.as_str()) != Some(sid) {
            continue;
        }
        match row.get("event").and_then(|v| v.as_str()) {
            Some("issued") => {
                active = true;
                identity = row
                    .get("identity")
                    .and_then(|v| v.get("identity_hash"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
            Some("revoked") => active = false,
            _ => {}
        }
    }
    if active {
        Ok(Some((sid.to_string(), identity)))
    } else {
        Ok(None)
    }
}

/// 链上正身绑定写入（idenlane-solo iden-01）：会话在场即在册即把
/// session_id 与 identity_hash 落进事件输入信封；缺省即不改动旧行为。
fn bind_envelope(opts: &std::collections::HashMap<String, String>, input: &mut EventInput) {
    let sessions = opts.get("sessions").map(|s| s.as_str());
    let session = opts.get("session").map(|s| s.as_str());
    match resolve_envelope(sessions, session) {
        Ok(Some((sid, id_hash))) => {
            input.session_id = Some(sid);
            input.identity_hash = id_hash;
        }
        Ok(None) => {}
        Err(e) => emit(json!({"error": e}), 2),
    }
}

const USAGE: &str = r#"scribe 书简：引擎事件链写入与校验命令行

子命令：
  append    追加认证事件（写）
    必填：--report <报告文件> --exit-code <退出码> --trail <链文件>
    可选：--locks <锁册路径> --session <会话号> --sessions <会话台账路径> --allow-worktree-trail <1|true> --no-session-reason <事由>
    示例：scribe append --report report.json --exit-code 0 --trail trail.ndjson
    闸三会话在册验：--session 须在会话台账为活跃，无或已吊销即拒 SessionNotActive；
      --no-session-reason <事由> 主会处置位默认关，无会话号时显式开启即放行。

  intent    追加意图精炼事件（写）
    必填：--record <ask3 记录> --validation <验证件> --trail <链文件>
    可选：--locks <锁册路径> --session <会话号> --sessions <会话台账路径> --allow-worktree-trail <1|true> --allow-reintent <1|true> --no-session-reason <事由>
    示例：scribe intent --record ask3.json --validation valid.json --trail trail.ndjson
    闸二重意图拒：同 record 路径已有 intent_refined 在链即拒 IntentRecordUsedRejected；
      --allow-reintent 默认关留 facepark 丢事件重追加合法通道。
    闸三会话在册验：同 append，--session 须活跃，--no-session-reason 默认关。

  park      追加停泊事件（写）
    必填：--record <停泊记录 JSON> --trail <链文件>
    可选：--locks <锁册路径> --session <会话号> --allow-worktree-trail <1|true>

  verify    校验链完整性（读）
    必填：--trail <链文件>
    示例：scribe verify --trail trail.ndjson

  query     检索链事件（读）
    必填：--trail <链文件>
    可选：--event-type <类型> --doc-id <文档ID>

  record    秤星读数落链（写）
    七字段守卫，详见 SPEC-011；可选 --allow-worktree-trail <1|true>

  vectors   冻结向量集（读）

退出码：0=成功 1=校验/检索异常 2=工具自身异常
"#;

fn print_usage() {
    println!("{}", USAGE);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("");
    if cmd == "--help" || cmd == "-h" || cmd == "help" || cmd.is_empty() {
        print_usage();
        if cmd.is_empty() {
            exit(2);
        }
        return;
    }
    let mut opts = std::collections::HashMap::new();
    let mut i = 2;
    while i + 1 < args.len() + 1 && i < args.len() {
        if args[i].starts_with("--") && i + 1 < args.len() {
            opts.insert(args[i].trim_start_matches('-').to_string(), args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    let opt = |k: &str| opts.get(k).cloned();

    match cmd {
        "verify" => {
            let Some(trail) = opt("trail") else { emit(json!({"error": "verify 缺 --trail，用法：scribe verify --trail <链文件>"}), 2) };
            let Some(text) = read_text(&trail) else {
                emit(json!({"error": "trail 不存在"}), 2)
            };
            let _ = text;
            let events = load_store(&trail);
            match verify(&events, VerifyRange::Full) {
                Ok(ok) => emit(
                    json!({
                        "status": "valid",
                        "events": ok.event_count,
                        "first_hash": ok.first_hash,
                        "last_hash": ok.last_hash,
                    }),
                    0,
                ),
                Err(e) => emit(json!({"status": "broken", "error": format!("{e:?}")}), 1),
            }
        }
        "query" => {
            let Some(trail) = opt("trail") else { emit(json!({"error": "query 缺少 --trail，用法：scribe query --trail <链文件>"}), 2) };
            let events = load_store(&trail);
            if events.is_empty() && !PathBuf::from(&trail).exists() {
                emit(json!({"error": "trail 不存在"}), 2);
            }
            let filter = EventFilter {
                event_type: opt("event-type"),
                doc_id: opt("doc-id"),
                ..Default::default()
            };
            let list = query(&events, Some(filter), None);
            if list.events.is_empty() {
                emit(json!({"matches": 0}), 1);
            }
            emit(json!({"matches": list.events.len(), "events": list.events}), 0);
        }
        "append" => {
            let (Some(report), Some(exit_code), Some(trail)) =
                (opt("report"), opt("exit-code"), opt("trail"))
            else {
                emit(json!({"error": "append 缺少必填参数，需 --report <报告文件> --exit-code <退出码> --trail <链文件>"}), 2)
            };
            lockgate_guard(&opts, &trail);
            worktree_trail_guard(&opts, &trail);
            let Ok(exit_code) = exit_code.parse::<i32>() else {
                emit(json!({"error": "退出码非数"}), 2)
            };
            session_guard(&opts);
            let Some(text) = read_text(&report) else { emit(json!({"error": "报告不存在"}), 2) };
            let mut store = load_store(&trail);
            // 追加原子化重试（basefix-solo 批 F-2）：并发下他进程先行落链使
            // 本进程预取时间戳不越新链尾即拒 TimestampNotMonotonic，取新鲜
            // 时间戳有界重试三次，事件内容仍由报告件确定性派生零伪造。
            let mut attempt = 0u8;
            loop {
                let mut input = match certification_event(
                    PathBuf::from(&report).as_path(),
                    &text,
                    exit_code,
                    gate_actor(),
                    Utc::now(),
                ) {
                    Ok(i) => i,
                    Err(e) => emit(json!({"error": format!("报告不识别 {e:?}")}), 2),
                };
                bind_envelope(&opts, &mut input);
                match append(input, &mut store, Some(&PathBuf::from(&trail))) {
                    Ok(ok) => emit(
                        json!({"status": "appended", "event_id": ok.event_id, "event_hash": ok.event_hash}),
                        0,
                    ),
                    Err(AppendError::TimestampNotMonotonic { .. }) if attempt < 3 => {
                        attempt += 1;
                        continue;
                    }
                    Err(e) => emit(json!({"error": format!("写入拒 {e:?}")}), 1),
                }
            }
        }
        "intent" => {
            let (Some(record), Some(validation), Some(trail)) =
                (opt("record"), opt("validation"), opt("trail"))
            else {
                emit(json!({"error": "intent 缺少必填参数，需 --record <ask3 记录> --validation <验证件> --trail <链文件>"}), 2)
            };
            lockgate_guard(&opts, &trail);
            worktree_trail_guard(&opts, &trail);
            session_guard(&opts);
            let (Some(rtext), Some(vtext)) = (read_text(&record), read_text(&validation)) else {
                emit(json!({"error": "双件缺失"}), 2)
            };
            let scope = match load_parking_scope(Path::new(&trail)) {
                Ok(s) => s,
                Err(e) => emit(json!({"error": format!("重放面不可读 {e:?}")}), 2),
            };
            let allow_reintent = opts.get("allow-reintent").map(|s| s == "1" || s == "true").unwrap_or(false);
            match check_intent_reused(&scope, &record, allow_reintent) {
                Ok(()) => {}
                Err(sih_engine::event_stream::intent::IntentError::IntentRecordUsed) => {
                    emit(json!({"error": "意图记录已用即拒 IntentRecordUsedRejected，主会处置位 --allow-reintent 默认关"}), 1)
                }
                Err(e) => emit(json!({"error": format!("重意图拒异常 {e:?}")}), 2),
            }
            let mut input = match intent_event(
                PathBuf::from(&record).as_path(),
                &rtext,
                PathBuf::from(&validation).as_path(),
                &vtext,
                gate_actor(),
                Utc::now(),
            ) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("意图拒 {e:?}")}), 1),
            };
            bind_envelope(&opts, &mut input);
            let mut store = load_store(&trail);
            match append(input, &mut store, Some(&PathBuf::from(&trail))) {
                Ok(ok) => emit(
                    json!({"status": "appended", "event_id": ok.event_id, "event_hash": ok.event_hash}),
                    0,
                ),
                Err(e) => emit(json!({"error": format!("写入拒 {e:?}")}), 1),
            }
        }
        "park" => {
            let (Some(record), Some(trail)) = (opt("record"), opt("trail")) else {
                emit(json!({"error": "park 缺少必填参数，需 --record <停泊记录 JSON> --trail <链文件>"}), 2)
            };
            lockgate_guard(&opts, &trail);
            worktree_trail_guard(&opts, &trail);
            let Some(text) = read_text(&record) else { emit(json!({"error": "记录不存在"}), 2) };
            let store = load_store(&trail);
            let scope = match load_parking_scope(Path::new(&trail)) {
                Ok(s) => s,
                Err(e) => emit(json!({"error": format!("重放面不可读 {e:?}")}), 2),
            };
            let mut input = match park_event(&text, &scope, gate_actor(), Utc::now()) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("停泊拒 {e:?}")}), 1),
            };
            bind_envelope(&opts, &mut input);
            let mut store = store;
            match append(input, &mut store, Some(&PathBuf::from(&trail))) {
                Ok(ok) => emit(
                    json!({"status": "appended", "event_id": ok.event_id, "event_hash": ok.event_hash}),
                    0,
                ),
                Err(e) => emit(json!({"error": format!("写入拒 {e:?}")}), 1),
            }
        }
        "record" => {
            let (Some(reading), Some(trail)) = (opt("reading"), opt("trail")) else {
                emit(json!({"error": "record 缺少必填参数，需 --reading <读数件> --trail <链文件>"}), 2)
            };
            lockgate_guard(&opts, &trail);
            worktree_trail_guard(&opts, &trail);
            let Some(text) = read_text(&reading) else { emit(json!({"error": "读数件不存在"}), 2) };
            let mut input = match reading_event(&text, gate_actor()) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("读数守卫拒 {e}")}), 1),
            };
            bind_envelope(&opts, &mut input);
            let doc_id = input.doc_id.clone();
            let mut store = load_store(&trail);
            match append(input, &mut store, Some(&PathBuf::from(&trail))) {
                Ok(ok) => emit(
                    json!({
                        "status": "appended",
                        "event_id": ok.event_id,
                        "event_hash": ok.event_hash,
                        "doc_id": doc_id,
                    }),
                    0,
                ),
                Err(e) => emit(json!({"error": format!("写入拒 {e:?}")}), 1),
            }
        }
        "crosscheck" => {
            let (Some(report), Some(material), Some(trail)) =
                (opt("report"), opt("material"), opt("trail"))
            else {
                emit(json!({"error": "record 缺少必填参数，运行 scribe --help 查看用法"}), 2)
            };
            lockgate_guard(&opts, &trail);
            let Some(text) = read_text(&report) else { emit(json!({"error": "报告不存在"}), 2) };
            let Some(mat_text) = read_text(&material) else {
                emit(json!({"error": format!("所指材料不存在 {material}")}), 2)
            };
            let mut input = match crosscheck_event(&text, &mat_text, gate_actor()) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("跨方核毕守卫拒 {e}")}), 1),
            };
            bind_envelope(&opts, &mut input);
            let doc_id = input.doc_id.clone();
            let mut store = load_store(&trail);
            match append(input, &mut store, Some(&PathBuf::from(&trail))) {
                Ok(ok) => emit(
                    json!({
                        "status": "appended",
                        "event_id": ok.event_id,
                        "event_hash": ok.event_hash,
                        "doc_id": doc_id,
                    }),
                    0,
                ),
                Err(e) => emit(json!({"error": format!("写入拒 {e:?}")}), 1),
            }
        }
        "vectors" => {
            let Some(out) = opt("write") else { emit(json!({"error": "缺 --write"}), 2) };
            let vectors = golden_vectors();
            let text = serde_json::to_string_pretty(&vectors).unwrap();
            std::fs::write(&out, text).expect("向量集写入");
            emit(json!({"status": "written", "count": vectors.as_array().map(|a| a.len()).unwrap_or(0)}), 0);
        }
        "direct" => {
            // 直改笔形（idenlane-envelope-solo 批）：无租约轻量链笔，agent
            // 笔强制挂 --identity-report，human 笔接口位（human seat 件在
            // 批 B，批 A 留接口位校验形 + 测试跳过形）。
            let (Some(record), Some(trail)) = (opt("record"), opt("trail")) else {
                emit(json!({"error": "direct 缺少必填参数，需 --record <直改链笔 JSON> --trail <链文件>"}), 2)
            };
            lockgate_guard(&opts, &trail);
            worktree_trail_guard(&opts, &trail);
            session_guard(&opts);
            let Some(text) = read_text(&record) else { emit(json!({"error": "记录不存在"}), 2) };
            let record_value: serde_json::Value = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(e) => emit(json!({"error": format!("记录非法 JSON {e}")}), 2),
            };
            let pen_form = record_value.get("pen_form").and_then(|v| v.as_str()).unwrap_or("");
            let pen_kind = record_value.get("pen_kind").and_then(|v| v.as_str()).unwrap_or("");
            if pen_form != "direct" {
                emit(json!({"error": "pen_form 必为 direct"}), 1);
            }
            let actor_id = record_value.get("actor_id").and_then(|v| v.as_str()).unwrap_or("unknown");
            let files = record_value.get("files").and_then(|v| v.as_array()).cloned().unwrap_or_default();
            let subject = record_value.get("subject").and_then(|v| v.as_str()).unwrap_or("");
            let timestamp_str = record_value.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
            let timestamp: chrono::DateTime<chrono::Utc> = if timestamp_str.is_empty() {
                chrono::Utc::now()
            } else {
                match chrono::DateTime::parse_from_rfc3339(timestamp_str) {
                    Ok(t) => t.with_timezone(&chrono::Utc),
                    Err(_) => emit(json!({"error": "timestamp 非 ISO8601"}), 1),
                }
            };
            // 笔形分类判定语义一裁（idenlane-envelope-solo 批 F-5）：
            // agent 笔强制挂 --identity-report 拒未挂；human 笔接口位留
            // 批 B（human seat 件）；其他笔形拒。
            let id_report_path = opt("identity-report");
            let human_seat_path = opt("human-seat");
            let session_opt = opts.get("session").map(|s| s.as_str());
            let mut actor = Actor {
                actor_id: actor_id.to_string(),
                actor_type: ActorType::Human,
                invoked_via: "direct-pen".to_string(),
            };
            let mut identity_hash: Option<String> = None;
            let mut session_id_to_bind: Option<String> = None;
            match pen_kind {
                "agent" => {
                    let Some(irp) = id_report_path else {
                        emit(json!({"error": "agent 笔强制 --identity-report 正身报告，拒未挂"}), 1);
                    };
                    let Some(ir_text) = read_text(&irp) else {
                        emit(json!({"error": format!("正身报告不可读 {irp}")}), 2);
                    };
                    let ir: serde_json::Value = match serde_json::from_str(&ir_text) {
                        Ok(v) => v,
                        Err(e) => emit(json!({"error": format!("正身报告非法 {e}")}), 2),
                    };
                    identity_hash = ir
                        .get("identity")
                        .and_then(|v| v.get("hash"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    if identity_hash.is_none() {
                        emit(json!({"error": "正身报告缺 identity.hash"}), 1);
                    }
                    actor.actor_type = ActorType::Agent;
                    if let Some(sid) = session_opt {
                        session_id_to_bind = Some(sid.to_string());
                    }
                }
                "human" => {
                    // human 笔接口位：批 B 实装 human seat 件前，--human-seat
                    // 必挂且为合法 JSON 对象含 seat=="human"，否则拒。
                    // 测试跳过形即 --allow-human-stub 旗标供 TDD 跳过此验。
                    let allow_stub = opts.get("allow-human-stub").map(|s| s == "1" || s == "true").unwrap_or(false);
                    if !allow_stub {
                        let Some(hsp) = human_seat_path else {
                            emit(json!({"error": "human 笔强制 --human-seat 验真件（批 B 承载），或 --allow-human-stub 1 跳过（测试形）"}), 1);
                        };
                        let Some(hs_text) = read_text(&hsp) else {
                            emit(json!({"error": format!("人席位验真件不可读 {hsp}")}), 2);
                        };
                        let hs: serde_json::Value = match serde_json::from_str(&hs_text) {
                            Ok(v) => v,
                            Err(e) => emit(json!({"error": format!("人席位验真件非法 {e}")}), 2),
                        };
                        if hs.get("seat").and_then(|v| v.as_str()) != Some("human") {
                            emit(json!({"error": "人席位验真件 seat 字段必为 human"}), 1);
                        }
                        if hs.get("name").and_then(|v| v.as_str()).map_or(true, |s| s.is_empty()) {
                            emit(json!({"error": "人席位验真件缺 name 字段"}), 1);
                        }
                        if hs.get("attest_at").and_then(|v| v.as_str()).map_or(true, |s| s.is_empty()) {
                            emit(json!({"error": "人席位验真件缺 attest_at 字段"}), 1);
                        }
                    }
                    actor.actor_type = ActorType::Human;
                }
                _ => emit(json!({"error": "pen_kind 必为 agent 或 human"}), 1),
            }
            // 直改链笔事件构造：event_type = direct_edit_completed，
            // event_class = record_only，details 载笔体（pen_form、pen_kind、
            // actor_id、files、subject、timestamp）；envelope 字段即
            // session_id + identity_hash 由 bind_envelope 与本批手动注入。
            let event_id = uuid::Uuid::new_v4().to_string();
            let doc_id = format!("direct-{pen_kind}-{}", &event_id[..8]);
            let details = json!({
                "pen_form": pen_form,
                "pen_kind": pen_kind,
                "actor_id": actor_id,
                "files": files,
                "subject": subject,
                "timestamp": timestamp.to_rfc3339(),
            });
            let mut input = EventInput {
                event_id: event_id.clone(),
                event_type: "direct_edit_completed".to_string(),
                timestamp,
                actor,
                details: Some(details),
                doc_id: doc_id.clone(),
                prev_hash: None,
                event_class: Some("record_only".to_string()),
                verification_result: None,
                session_id: None,
                identity_hash: None,
            };
            // 直改笔形信封：session_id 与 identity_hash 注入
            if let Some(sid) = session_id_to_bind {
                input.session_id = Some(sid);
            }
            input.identity_hash = identity_hash;
            // human 笔无 identity_hash 走 None（受保护：None 跳过 hash 入）
            let mut store = load_store(&trail);
            match append(input, &mut store, Some(&PathBuf::from(&trail))) {
                Ok(ok) => emit(
                    json!({
                        "status": "appended",
                        "event_id": ok.event_id,
                        "event_hash": ok.event_hash,
                        "doc_id": doc_id,
                    }),
                    0,
                ),
                Err(e) => emit(json!({"error": format!("写入拒 {e:?}")}), 1),
            }
        }
        _ => emit(
            json!({"error": "用法 scribe <append|verify|query|intent|park|record|crosscheck|direct|vectors> --trail <路径>"}),
            2,
        ),
    }
}

fn golden_vectors() -> serde_json::Value {
    use chrono::{DateTime, Utc};
    use sih_engine::event_stream::event::{Actor, ActorType, Event};
    let base_actor = || Actor {
        actor_id: "scribe".to_string(),
        actor_type: ActorType::System,
        invoked_via: "cli".to_string(),
    };
    let ts = |s: &str| DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc);
    let mut out = Vec::new();
    let mut build = |class: &str, event: Event| {
        let expected = compute_event_hash(&event);
        out.push(json!({
            "class": class,
            "name": event.doc_id.clone(),
            "event": serde_json::to_value(&event).unwrap(),
            "expected_hash": expected,
        }));
    };
    let mk = |event_id: &str, event_type: &str, timestamp: chrono::DateTime<Utc>,
              details: Option<serde_json::Value>, doc_id: &str,
              event_class: Option<String>, verification_result: Option<serde_json::Value>| {
        let mut e = Event {
            event_id: event_id.to_string(),
            event_type: event_type.to_string(),
            timestamp,
            actor: base_actor(),
            details,
            doc_id: doc_id.to_string(),
            prev_hash: GENESIS_PREV_HASH.to_string(),
            event_hash: String::new(),
            event_class,
            verification_result,
            session_id: None,
            identity_hash: None,
        };
        e.event_hash = compute_event_hash(&e);
        e
    };
    build("genesis", mk("00000000-0000-4000-8000-000000000001", "session_open",
        ts("2026-08-27T00:00:00Z"), Some(json!({"note": "genesis"})), "vec-genesis", None, None));
    build("details_null", mk("00000000-0000-4000-8000-000000000002", "session_close",
        ts("2026-08-27T00:00:01Z"), None, "vec-details-null", None, None));
    build("optional_absent", mk("00000000-0000-4000-8000-000000000003", "task_completion",
        ts("2026-08-27T00:00:02.500Z"), Some(json!({"task": "t"})), "vec-optional", None, None));
    build("non_ascii", mk("00000000-0000-4000-8000-000000000004", "certification_completed",
        ts("2026-08-27T00:00:03Z"), Some(json!({"词": "书简融回", "路径": "副本/核阅"})),
        "vec-非ASCII", Some("record_only".to_string()), None));
    build("timestamp_edge", mk("00000000-0000-4000-8000-000000000005", "parking_entered",
        ts("2026-08-27T23:59:59.999999+00:00"), Some(json!({"entry_id": "vec-5"})),
        "vec-ts-edge", Some("record_only".to_string()),
        Some(json!({"exit_code": 0, "findings": 0, "total": 0}))));
    json!(out)
}
