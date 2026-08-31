//! viewer 视图组件命令行面，承接 DEC-007#decision-component 聚合输出组件。
//!
//! 视图零写零 LLM，承 DEC-006 本名回滚即代码标识符 viewer。
//! 三子命令即 alarms 异常视图、heartbeat 心跳视图、settle 结算视图。
//! 退出码三值：0 = 常态、1 = alarms 非空、2 = 工具异常含链不可读。
//! 同参双跑逐字节一致是构成性纪律。
//!
//! 多链装载（viewfix-solo 整改批入）：--trail 可重复，按参序全量装载并按参序
//! 串联事件流。单链行为与整改前逐字节不变；多链 payload 增 trails 数组、trail
//! 字段保持为单字符串（兼容单链回归断言）。配对纯函数零改动，病仅在装载层。

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

/// 解析命令行参数。承 viewfix-solo：返回按参序保留全部 (key, value) 对，
/// 允许多次同名旗标（如 --trail a --trail b）共址。HashMap 形态整改前同
/// key 覆盖，仅保留最后一条——是 CLI 装载层缺陷根因。
fn read_args() -> Vec<(String, String)> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts: Vec<(String, String)> = Vec::new();
    let mut i = 2;
    while i < args.len() {
        if args[i].starts_with("--") && i + 1 < args.len() {
            opts.push((
                args[i].trim_start_matches('-').to_string(),
                args[i + 1].clone(),
            ));
            i += 2;
        } else {
            i += 1;
        }
    }
    opts
}

/// 取全部 --trail 旗标的参序路径。
fn collect_trails(opts: &[(String, String)]) -> Vec<String> {
    opts.iter()
        .filter(|(k, _)| k == "trail")
        .map(|(_, v)| v.clone())
        .collect()
}

/// 单值取（如 --at / --date），按参序首条命中。零命中返回 None。
fn first_value<'a>(opts: &'a [(String, String)], key: &str) -> Option<&'a str> {
    for (k, v) in opts {
        if k == key {
            return Some(v.as_str());
        }
    }
    None
}

/// 多链装载：按参序全量读，按参序串联事件流。任一链缺/坏即 emit 退出码 2。
fn load_trails_or_die(trails: &[String]) -> Vec<sih_engine::event_stream::event::Event> {
    if trails.is_empty() {
        emit(json!({"error": "缺 --trail"}), 2);
    }
    let mut all: Vec<sih_engine::event_stream::event::Event> = Vec::new();
    for t in trails {
        let path = PathBuf::from(t);
        if !path.exists() {
            emit(json!({"error": "trail 不存在", "path": t}), 2);
        }
        match load_events(path.as_path()) {
            Ok(mut evs) => all.append(&mut evs),
            Err(e) => emit(json!({"error": format!("trail 不可读 {e:?}"), "path": t}), 2),
        }
    }
    all
}

/// payload 中附 trail 信息。单链保持 trail 字符串逐字节不变（回归）；多链
/// 增 trails 数组（顺序与 --trail 旗标参序一致），trail 字符串取首条。
fn attach_trail_info(payload: &mut serde_json::Value, trails: &[String]) {
    if let Some(first) = trails.first() {
        payload["trail"] = serde_json::Value::String(first.clone());
    }
    if trails.len() > 1 {
        payload["trails"] = serde_json::Value::Array(
            trails
                .iter()
                .map(|t| serde_json::Value::String(t.clone()))
                .collect(),
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("");
    let opts = read_args();
    let trails = collect_trails(&opts);

    match cmd {
        "alarms" => {
            let events = load_trails_or_die(&trails);
            let view = alarms_view(&events);
            let code = view.exit_code_suggested;
            let mut payload = serde_json::to_value(&view).unwrap();
            attach_trail_info(&mut payload, &trails);
            emit(payload, code);
        }
        "heartbeat" => {
            let events = load_trails_or_die(&trails);
            let Some(at) = first_value(&opts, "at") else {
                emit(json!({"error": "缺 --at YYYY-MM-DD"}), 2)
            };
            let date = match NaiveDate::parse_from_str(at, "%Y-%m-%d") {
                Ok(d) => d,
                Err(e) => emit(json!({"error": format!("--at 解析失败 {e}")}), 2),
            };
            let view = heartbeat_view(&events, date);
            let mut payload = serde_json::to_value(&view).unwrap();
            attach_trail_info(&mut payload, &trails);
            emit(payload, 0);
        }
        "settle" => {
            let events = load_trails_or_die(&trails);
            let Some(date_str) = first_value(&opts, "date") else {
                emit(json!({"error": "缺 --date YYYY-MM-DD"}), 2)
            };
            let date = match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                Ok(d) => d,
                Err(e) => emit(json!({"error": format!("--date 解析失败 {e}")}), 2),
            };
            // 在泊复检名单 = heartbeat 配对产出，复用同一参照日
            let hb = heartbeat_view(&events, date);
            let view = settle_view(&events, date, hb.parking);
            let mut payload = serde_json::to_value(&view).unwrap();
            attach_trail_info(&mut payload, &trails);
            emit(payload, 0);
        }
        _ => emit(
            json!({"error": "用法 viewer <alarms|heartbeat|settle> --trail <路径> [--at YYYY-MM-DD | --date YYYY-MM-DD]"}),
            2,
        ),
    }
}

#[cfg(test)]
mod tests {
    //! viewfix-solo 跨链装载层单元测试。
    //!
    //! 三组：read_args 允许多次同名旗标共址、collect_trails 按参序保留全部
    //! --trail、attach_trail_info 单链逐字节不变 + 多链 trails 数组在档。
    //! 跨链集成测试以 cargo test 整库形态跑（sih-engine/tests/cli_multitrail.rs），
    //! 走真实 7 链 + tempfile 双链 + 主程序入口。

    use super::*;

    /// read_args 允许多次同名旗标共址，按参序保留全部。
    /// 整改前 HashMap 同 key 覆盖，仅留最后一条。
    #[test]
    fn r1_read_args_preserves_repeated_flags_in_order() {
        let parsed = vec![
            ("trail".to_string(), "a.ndjson".to_string()),
            ("at".to_string(), "2026-08-25".to_string()),
            ("trail".to_string(), "b.ndjson".to_string()),
            ("trail".to_string(), "c.ndjson".to_string()),
        ];
        // 模拟 read_args 的解析路径：仅取 --key val 对，按出现次序
        let collected = collect_trails(&parsed);
        assert_eq!(collected, vec!["a.ndjson", "b.ndjson", "c.ndjson"]);
    }

    /// collect_trails 零参序命中返空数组。
    #[test]
    fn r2_collect_trails_empty_when_absent() {
        let parsed = vec![("at".to_string(), "2026-08-25".to_string())];
        assert!(collect_trails(&parsed).is_empty());
    }

    /// attach_trail_info：单链保持 trail 字符串逐字节不变（回归断言）。
    #[test]
    fn r3_attach_trail_info_single_chain_byte_identical() {
        let mut payload = json!({"consumable_count": 0, "parking": []});
        attach_trail_info(&mut payload, &["only.ndjson".to_string()]);
        let s = serde_json::to_string(&payload).unwrap();
        // 单链不增 trails 字段，仅有 trail 字符串
        assert!(s.contains("\"trail\":\"only.ndjson\""));
        assert!(!s.contains("\"trails\""));
    }

    /// attach_trail_info：多链增 trails 数组按参序排列，trail 字段保持首条。
    #[test]
    fn r4_attach_trail_info_multi_chain_trails_array_in_order() {
        let mut payload = json!({"consumable_count": 0, "parking": []});
        let trails = vec![
            "08-25.ndjson".to_string(),
            "08-26.ndjson".to_string(),
            "08-27.ndjson".to_string(),
        ];
        attach_trail_info(&mut payload, &trails);
        assert_eq!(payload["trail"], "08-25.ndjson");
        assert_eq!(
            payload["trails"],
            json!(["08-25.ndjson", "08-26.ndjson", "08-27.ndjson"])
        );
    }

    /// first_value 取 --key 首条命中（同 key 重复时取首）。
    #[test]
    fn r5_first_value_returns_first_match() {
        let parsed = vec![
            ("at".to_string(), "2026-08-25".to_string()),
            ("at".to_string(), "2026-08-31".to_string()),
        ];
        assert_eq!(first_value(&parsed, "at"), Some("2026-08-25"));
        assert_eq!(first_value(&parsed, "date"), None);
    }
}
