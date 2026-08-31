//! viewer 视图组件命令行面，承接 DEC-007#decision-component 聚合输出组件。
//!
//! 视图零写零 LLM，承 DEC-006 本名回滚即代码标识符 viewer。
//! 三子命令即 alarms 异常视图、heartbeat 心跳视图、settle 结算视图。
//! 退出码三值：0 = 常态、1 = alarms 非空、2 = 工具异常含链不可读。
//! 同参双跑逐字节一致是构成性纪律。

use chrono::NaiveDate;
use serde_json::json;
use sih_engine::event_stream::load_events;
use sih_engine::view::{alarms::alarms_view, heartbeat::heartbeat_view, settle::settle_view};
use std::path::PathBuf;
use std::process::exit;

fn emit(value: serde_json::Value, code: i32) -> ! {
    println!("{}", serde_json::to_string_pretty(&value).unwrap());
    exit(code)
}

fn read_args() -> std::collections::HashMap<String, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = std::collections::HashMap::new();
    let mut i = 2;
    while i < args.len() {
        if args[i].starts_with("--") && i + 1 < args.len() {
            opts.insert(args[i].trim_start_matches('-').to_string(), args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    opts
}

fn load_or_die(trail: &str) -> Vec<sih_engine::event_stream::event::Event> {
    let path = PathBuf::from(trail);
    if !path.exists() {
        emit(json!({"error": "trail 不存在", "path": trail}), 2);
    }
    match load_events(path.as_path()) {
        Ok(events) => events,
        Err(e) => emit(json!({"error": format!("trail 不可读 {e:?}"), "path": trail}), 2),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("");
    let opts = read_args();
    let opt = |k: &str| opts.get(k).cloned();

    match cmd {
        "alarms" => {
            let Some(trail) = opt("trail") else {
                emit(json!({"error": "缺 --trail"}), 2)
            };
            let events = load_or_die(&trail);
            let view = alarms_view(&events);
            let code = view.exit_code_suggested;
            let mut payload = serde_json::to_value(&view).unwrap();
            payload["trail"] = serde_json::Value::String(trail);
            emit(payload, code);
        }
        "heartbeat" => {
            let Some(trail) = opt("trail") else {
                emit(json!({"error": "缺 --trail"}), 2)
            };
            let events = load_or_die(&trail);
            let Some(at) = opt("at") else {
                emit(json!({"error": "缺 --at YYYY-MM-DD"}), 2)
            };
            let date = match NaiveDate::parse_from_str(&at, "%Y-%m-%d") {
                Ok(d) => d,
                Err(e) => emit(json!({"error": format!("--at 解析失败 {e}")}), 2),
            };
            let view = heartbeat_view(&events, date);
            let mut payload = serde_json::to_value(&view).unwrap();
            payload["trail"] = serde_json::Value::String(trail);
            emit(payload, 0);
        }
        "settle" => {
            let Some(trail) = opt("trail") else {
                emit(json!({"error": "缺 --trail"}), 2)
            };
            let events = load_or_die(&trail);
            let Some(date_str) = opt("date") else {
                emit(json!({"error": "缺 --date YYYY-MM-DD"}), 2)
            };
            let date = match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                Ok(d) => d,
                Err(e) => emit(json!({"error": format!("--date 解析失败 {e}")}), 2),
            };
            // 在泊复检名单 = heartbeat 配对产出，复用同一参照日
            let hb = heartbeat_view(&events, date);
            let view = settle_view(&events, date, hb.parking);
            let mut payload = serde_json::to_value(&view).unwrap();
            payload["trail"] = serde_json::Value::String(trail);
            emit(payload, 0);
        }
        _ => emit(
            json!({"error": "用法 viewer <alarms|heartbeat|settle> --trail <路径> [--at YYYY-MM-DD | --date YYYY-MM-DD]"}),
            2,
        ),
    }
}
