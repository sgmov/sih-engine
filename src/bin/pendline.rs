//! pendline 命令行面——候裁处置范式编排件的五子命令入口（pl-04）。
//!
//! 正典指针：DES-017 候裁处置范式编排设计 v1
//! （doc/design/DES-017-pending-adjudication-orchestration-v1.md）；
//! 任务包 sih/state/plan/pendline.md；人工活证
//! sih/event/plan/mcpdual-parallel-results.md 第九节
//!（「待裁过得一，裁一过一执行一，未过入泊」人工执行实录的机械化）。
//!
//! 子命令五件：collect（三源候选汇总）与 dispatch（档位采样合同生成）与
//! backfill（外部回填件接评分路由，插拔腿）与 route（判词三态路由与
//! scribe 落链）与 config（档位配置件读写两面）。
//! 退出码对齐引擎惯例：0 成功、1 违规或落链失败、2 用法或环境错误。
//!
//! 编排层零新执法位：候选发现归 critsweep、机器裁定归 attractor（判词
//! 上游直读）、未过入泊归 scribe park，本件只收集、分派、路由、留痕。
//! 模块树经 #[path] 引入（bin 私有形，lib 导出面零改动）。

use std::process::ExitCode;

#[path = "../pendline/mod.rs"]
mod pendline;

use pendline::{parse_args, PendError, USAGE};

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if argv.is_empty() {
        eprintln!("{}", USAGE);
        return ExitCode::from(2);
    }
    let sub = argv[0].as_str();
    let rest = &argv[1..];
    let result: Result<i32, PendError> = match sub {
        "collect" => match parse_args(rest, &[]) {
            Ok(a) => pendline::collect::run(&a),
            Err(e) => Err(e),
        },
        "dispatch" => match parse_args(rest, &[]) {
            Ok(a) => pendline::dispatch::run(&a),
            Err(e) => Err(e),
        },
        "backfill" => match parse_args(rest, &[]) {
            Ok(a) => pendline::dispatch::run_backfill(&a),
            Err(e) => Err(e),
        },
        "route" => match parse_args(rest, &["dry-run"]) {
            Ok(a) => pendline::route::run(&a),
            Err(e) => Err(e),
        },
        "config" => match parse_args(rest, &["get"]) {
            Ok(a) => pendline::config::run(&a),
            Err(e) => Err(e),
        },
        "help" | "--help" | "-h" => {
            println!("{}", USAGE);
            return ExitCode::from(0);
        }
        _ => {
            eprintln!("未知子命令 {}\n{}", sub, USAGE);
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!(
                "{}",
                serde_json::to_string(&serde_json::json!({"error": e.message}))
                    .unwrap_or_default()
            );
            ExitCode::from(e.code as u8)
        }
    }
}
