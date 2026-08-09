//! 哈希链重建工具，承接 OQ-01 trail 断裂修复。
//!
//! 读取指定 trail 文件，为 event_id 与哈希字段缺失的行补齐，
//! 使用与 hash.rs 完全相同的算法重建哈希链，写回原文件。
//!
//! 用法：cargo run --example rebuild_hash_chain <trail_file>

use sih_engine::event_stream::event::Event;
use sih_engine::event_stream::hash::{compute_event_hash, GENESIS_PREV_HASH};
use std::env;
use std::fs;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("用法: cargo run --example rebuild_hash_chain <trail_file>");
        std::process::exit(1);
    }
    let path = &args[1];

    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("读取文件失败 {}: {}", path, e);
        std::process::exit(1);
    });

    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        println!("文件为空，无操作");
        return;
    }

    // 反序列化全部事件
    let mut events: Vec<Event> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let mut event = Event::from_json_line(line).unwrap_or_else(|e| {
            eprintln!("第 {} 行反序列化失败: {}", i + 1, e);
            std::process::exit(1);
        });

        // 补齐缺失的 event_id
        if event.event_id.is_empty() {
            event.event_id = Uuid::new_v4().to_string();
            println!("行 {}: 补 event_id = {}", i + 1, event.event_id);
        }

        events.push(event);
    }

    // 重建哈希链
    let mut prev_hash = GENESIS_PREV_HASH.to_string();
    for (i, event) in events.iter_mut().enumerate() {
        // 设置 prev_hash
        let old_prev = event.prev_hash.clone();
        if event.prev_hash.is_empty() || i == 0 {
            event.prev_hash = prev_hash.clone();
            if old_prev != event.prev_hash {
                println!(
                    "行 {}: 设 prev_hash = {}... (原: {})",
                    i + 1,
                    &event.prev_hash[..16],
                    if old_prev.is_empty() { "空" } else { &old_prev[..16.min(old_prev.len())] }
                );
            }
        }

        // 计算 event_hash
        let new_hash = compute_event_hash(event);
        let old_hash = event.event_hash.clone();
        if old_hash != new_hash {
            println!(
                "行 {}: 算 event_hash = {}... (原: {})",
                i + 1,
                &new_hash[..16],
                if old_hash.is_empty() { "空" } else { &old_hash[..16.min(old_hash.len())] }
            );
            event.event_hash = new_hash;
        }

        // 更新 prev_hash 供下一行使用
        prev_hash = event.event_hash.clone();
    }

    // 序列化写回
    let output: Vec<String> = events
        .iter()
        .map(|e| e.to_json_line().unwrap())
        .collect();
    let output_content = output.join("\n") + "\n";

    fs::write(path, output_content).unwrap_or_else(|e| {
        eprintln!("写回文件失败 {}: {}", path, e);
        std::process::exit(1);
    });

    println!("\n完成: {} 共 {} 行哈希链已重建", path, events.len());
}
