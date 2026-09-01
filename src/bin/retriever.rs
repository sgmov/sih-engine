//! retriever 即温故宿主命令面，承接 SPEC-007#interface-signature 与 SPEC-008#boundary 与修订六。
//!
//! 子命令单件 recall，参数九件即 topic 与 word 与 event 与 since 与 until 与 archive 与 miss-log 与 at 与 out，
//! 退出码三值即零成功一拦二异常。根定位从当前目录上溯，不读系统钟。

use std::path::PathBuf;

use sih_engine::retriever::{derive_root, exit_code, recall, rows_to_ndjson, write_output_envelope, RecallArgs};

fn fail(message: &str, code: i32) -> ! {
    eprintln!("{{\"error\": \"{message}\"}}");
    std::process::exit(code)
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if argv.first().map(|s| s.as_str()) != Some("recall") {
        fail("用法 retriever recall --topic 词 --word 词 --event 记号 --since 日期 --until 日期 --archive 档名 --miss-log 路径 --at 日期 --out 路径", 2);
    }
    let mut topics: Vec<String> = Vec::new();
    let mut words: Vec<String> = Vec::new();
    let mut events: Vec<String> = Vec::new();
    let mut archives: Vec<String> = Vec::new();
    let mut since: Option<String> = None;
    let mut until: Option<String> = None;
    let mut at: Option<String> = None;
    let mut out: Option<String> = None;
    let mut miss_log: Option<String> = None;

    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--topic" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                topics.push(v.clone());
                i += 2;
            }
            "--word" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                words.push(v.clone());
                i += 2;
            }
            "--event" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                events.push(v.clone());
                i += 2;
            }
            "--archive" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                archives.push(v.clone());
                i += 2;
            }
            "--since" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                since = Some(v.clone());
                i += 2;
            }
            "--until" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                until = Some(v.clone());
                i += 2;
            }
            "--at" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                at = Some(v.clone());
                i += 2;
            }
            "--out" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                out = Some(v.clone());
                i += 2;
            }
            "--miss-log" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                miss_log = Some(v.clone());
                i += 2;
            }
            other => fail(&format!("未知参数 {other}"), 2),
        }
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let Some(root) = derive_root(&cwd) else {
        fail("工作区根不可定位即上溯无 sih-engine 与 sih-tools 并存层", 2);
    };

    let args = RecallArgs {
        root,
        topics,
        words,
        events,
        since,
        until,
        archives,
        at: at.unwrap_or_default(),
        miss_log: miss_log.map(PathBuf::from),
    };

    match recall(&args) {
        Ok(rows) => {
            if let Some(path) = args_out(&out) {
                match write_output_envelope(&path, &rows, &args.topics) {
                    Ok(()) => std::process::exit(0),
                    Err(e) => fail(&format!("{e:?}"), exit_code(&e)),
                }
            }
            print!("{}", rows_to_ndjson(&rows));
            std::process::exit(0);
        }
        Err(e) => fail(&format!("{e:?}"), exit_code(&e)),
    }
}

fn args_out(out: &Option<String>) -> Option<PathBuf> {
    out.as_ref().map(PathBuf::from)
}
