//! 三问确定性外壳的校验腿，承接 SPEC-005#deterministic-shell。
//!
//! 三值退出码：零即记录验收通过，一即错误四类任一命中，
//! 二即工具自身异常含记录缺席与 JSON 不可解析。
//! 本外壳不执行诘察、不生成记录、不裁决内容真伪、不落盘，
//! 写入腿归书简与 event_stream。

use std::path::PathBuf;
use std::process::exit;

use sih_engine::ask3repeater::{validate, Ask3Error, OutputRecord};

fn emit(payload: &serde_json::Value, code: i32) -> ! {
    println!("{}", payload);
    exit(code)
}

fn main() {
    let mut record_path: Option<PathBuf> = None;
    let mut root: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => match args.next() {
                Some(value) => root = Some(PathBuf::from(value)),
                None => emit(&serde_json::json!({"error": "--root 需要路径参数"}), 2),
            },
            other => record_path = Some(PathBuf::from(other)),
        }
    }
    let record_path = match record_path {
        Some(path) => path,
        None => emit(&serde_json::json!({"error": "用法：ask3repeatergate <记录.json> --root <工作区根>"}), 2),
    };
    let root = match root {
        Some(path) => path,
        None => emit(&serde_json::json!({"error": "缺少 --root 工作区根"}), 2),
    };
    let text = match std::fs::read_to_string(&record_path) {
        Ok(text) => text,
        Err(err) => emit(
            &serde_json::json!({"error": format!("记录不可读 {err}"), "path": record_path.display().to_string()}),
            2,
        ),
    };
    let record: OutputRecord = match serde_json::from_str(&text) {
        Ok(record) => record,
        Err(err) => emit(
            &serde_json::json!({"error": format!("记录形态不可解析 {err}"), "class": "shape"}),
            2,
        ),
    };
    match validate(&record, &root) {
        Ok(()) => emit(
            &serde_json::json!({
                "status": "ok",
                "session_id": record.session_id,
                "anchor_count": record.anchors.len(),
            }),
            0,
        ),
        Err(errors) => {
            let findings: Vec<serde_json::Value> = errors
                .iter()
                .map(|error| {
                    let class = match error {
                        Ask3Error::Shape(_) => "shape",
                        Ask3Error::Contract(_) => "contract",
                        Ask3Error::Lineage(_) => "lineage",
                        Ask3Error::Metering(_) => "metering",
                    };
                    serde_json::json!({"class": class, "message": error.to_string()})
                })
                .collect();
            emit(
                &serde_json::json!({"status": "rejected", "findings": findings}),
                1,
            )
        }
    }
}
