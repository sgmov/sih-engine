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
    reading_event, sessiongate::SessionGateError, verify, Event, EventFilter, VerifyRange,
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
                let input = match certification_event(
                    PathBuf::from(&report).as_path(),
                    &text,
                    exit_code,
                    gate_actor(),
                    Utc::now(),
                ) {
                    Ok(i) => i,
                    Err(e) => emit(json!({"error": format!("报告不识别 {e:?}")}), 2),
                };
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
            let input = match intent_event(
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
            let input = match park_event(&text, &scope, gate_actor(), Utc::now()) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("停泊拒 {e:?}")}), 1),
            };
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
            let input = match reading_event(&text, gate_actor()) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("读数守卫拒 {e}")}), 1),
            };
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
            let input = match crosscheck_event(&text, &mat_text, gate_actor()) {
                Ok(i) => i,
                Err(e) => emit(json!({"error": format!("跨方核毕守卫拒 {e}")}), 1),
            };
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
        _ => emit(
            json!({"error": "用法 scribe <append|verify|query|intent|park|record|crosscheck|vectors> --trail <路径>"}),
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
