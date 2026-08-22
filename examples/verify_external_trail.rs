//! 外部 trail 链校验器，承接 scribe/CONTRACT.md 验收判据六即链互认。
//!
//! 用引擎侧链校验语义验证外部 ndjson 事件链：prev_hash 逐环衔接、
//! event_hash 与复算一致、时间戳单调递增。工具线 scribe 所写 trail
//! 须过本校验，即融回非重建的可证伪预演。
//!
//! 用法：cargo run --example verify_external_trail -- <trail_file>
//! 退出码：0 链完整；1 链断裂或校验失败；2 读取或解析失败。

use sih_engine::event_stream::event::Event;
use sih_engine::event_stream::hash::{compute_event_hash, GENESIS_PREV_HASH};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("用法: cargo run --example verify_external_trail -- <trail_file>");
        std::process::exit(2);
    }
    let path = &args[1];

    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("读取失败 {}: {}", path, e);
        std::process::exit(2);
    });

    let mut events: Vec<Event> = Vec::new();
    for (i, line) in content.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        let event = Event::from_json_line(line).unwrap_or_else(|e| {
            eprintln!("第 {} 行解析失败: {}", i + 1, e);
            std::process::exit(2);
        });
        events.push(event);
    }
    if events.is_empty() {
        eprintln!("空链: {}", path);
        std::process::exit(1);
    }

    let mut violations: Vec<String> = Vec::new();
    let mut prev_hash = GENESIS_PREV_HASH.to_string();
    let mut prev_ts = None;

    for (i, event) in events.iter().enumerate() {
        if event.prev_hash != prev_hash {
            violations.push(format!(
                "行 {}: prev_hash 不衔接，期望 {}... 实为 {}...",
                i + 1,
                &prev_hash[..16.min(prev_hash.len())],
                &event.prev_hash[..16.min(event.prev_hash.len())]
            ));
        }
        let recomputed = compute_event_hash(event);
        if event.event_hash != recomputed {
            violations.push(format!(
                "行 {}: event_hash 与复算不一致，存 {}... 算 {}...",
                i + 1,
                &event.event_hash[..16.min(event.event_hash.len())],
                &recomputed[..16.min(recomputed.len())]
            ));
        }
        if let Some(last) = prev_ts {
            if event.timestamp <= last {
                violations.push(format!("行 {}: 时间戳非单调递增", i + 1));
            }
        }
        prev_hash = event.event_hash.clone();
        prev_ts = Some(event.timestamp);
    }

    if violations.is_empty() {
        println!(
            "链完整: {} 共 {} 事件，首 {}... 末 {}...",
            path,
            events.len(),
            &events[0].event_hash[..16],
            &events.last().unwrap().event_hash[..16]
        );
        std::process::exit(0);
    } else {
        for v in &violations {
            eprintln!("{}", v);
        }
        eprintln!("链断裂: {} 共 {} 处违规", path, violations.len());
        std::process::exit(1);
    }
}
