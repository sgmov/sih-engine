//! attractor 命令行面——得一机械核对腿的六子命令入口。
//!
//! 子命令承围堰形：emit-contract（合同出题半）与 score（计分半）与
//! check（全量核对）与 verify（重放比对）与 sign（机器终签）与 watch
//! （边界变更监视）。退出码对齐围堰：0 合规、1 违规或拒收、2 用法或
//! 环境错误；sign 三态承 listzero-solo 五修后现文即 refused 1、scribe
//! 失败打 sign failed 透传其退出码、signed 0。
//!
//! 机械腿不变量：全程零网络、零 LLM、零 key 读取、零目标仓写入；采样
//! 动作只存在于围堰子进程（经文件合同交接）。

use std::path::PathBuf;
use std::process::ExitCode;

use serde_json::{json, Value};

use sih_engine::attractor::contract_mode;
use sih_engine::attractor::jsonc::PyError;
use sih_engine::attractor::paradigm_loader;
use sih_engine::attractor::tally;

const USAGE: &str = "用法 attractor <emit-contract|score|check|verify|sign|watch> <子命令参数>\n\
emit-contract: --topic <topic.md> --ng-file <ng文本> --seat <框架:模型> --gid <gid> --title <题名> \\\n\
               --shots <n> --atoms <atom.yaml> --ng-label <档> --out <合同.json> \\\n\
               [--paradigm-id normative_convergence] [--atom integrator] [--direction judge] \\\n\
               [--n-declared <n>] [--version-label <版本自报>]\n\
score: --contract <合同.json> --responses <响应.jsonl> --trail <飞轮链.jsonl> \\\n\
       --identity-hash <hex64> --gate-verdict <闸三态> --out <计分材料.json>\n\
check: --material <裁决材料.json>\n\
verify: --material <裁决材料.json> --report <核对报告.json>\n\
sign: --material <裁决材料.json> --out <落盘目录> --trail <引擎链> \\\n\
      --scribe-binary <引擎scribe> --session <会话号> --locks <锁册>\n\
watch: --reports <签署报告目录>";

struct Args {
    flags: std::collections::HashMap<String, String>,
}

impl Args {
    fn parse(argv: &[String]) -> Result<Args, String> {
        let mut flags = std::collections::HashMap::new();
        let mut i = 0;
        while i < argv.len() {
            let a = &argv[i];
            if let Some(name) = a.strip_prefix("--") {
                let value = argv.get(i + 1).ok_or_else(|| format!("旗标 {} 缺值", a))?;
                flags.insert(name.to_string(), value.clone());
                i += 2;
            } else {
                return Err(format!("未知位置参数 {}", a));
            }
        }
        Ok(Args { flags })
    }
    fn get(&self, name: &str) -> Result<String, String> {
        self.flags
            .get(name)
            .cloned()
            .ok_or_else(|| format!("缺 --{}", name))
    }
    fn opt(&self, name: &str) -> Option<String> {
        self.flags.get(name).cloned()
    }
}

fn emit_args(args: &Args) -> Result<(), PyError> {
    let topic_path = PathBuf::from(args.get("topic").map_err(eusage)?);
    let out_path = PathBuf::from(args.get("out").map_err(eusage)?);
    let atoms_path = PathBuf::from(args.get("atoms").map_err(eusage)?);
    let seat_raw = args.get("seat").map_err(eusage)?;
    let gid = args.get("gid").map_err(eusage)?;
    let title = args.get("title").map_err(eusage)?;
    let shots: i64 = args.get("shots").map_err(eusage)?.parse().map_err(|_| eusage("--shots 须为整数"))?;
    let ng_label = args.get("ng-label").map_err(eusage)?;
    let ng_text_raw = match args.opt("ng-file") {
        Some(p) => std::fs::read_to_string(&p).map_err(|e| super_eio(&e))?,
        None => args.opt("ng-text").unwrap_or_default(),
    };
    let n_declared: i64 = match args.opt("n-declared") {
        Some(v) => v.parse().map_err(|_| eusage("--n-declared 须为整数"))?,
        None => shots,
    };
    let paradigm_id = args.opt("paradigm-id").unwrap_or_else(|| "normative_convergence".into());
    let atom_name = args.opt("atom").unwrap_or_else(|| "integrator".into());
    let direction_label = args.opt("direction").unwrap_or_else(|| "judge".into());
    let version_label = args.opt("version-label").unwrap_or_else(|| "self-reported".into());

    if shots < 1 {
        return Err(sih_engine::attractor::jsonc::verr(format!("--shots 须 ≥1，得到 {}", shots)));
    }
    let seat_parts: Vec<&str> = seat_raw.split(':').collect();
    if seat_parts.len() != 2 || seat_parts[0].is_empty() || seat_parts[1].is_empty() {
        return Err(sih_engine::attractor::jsonc::verr(format!(
            "--seat 须为 framework:model 形：{}",
            seat_raw
        )));
    }

    let atoms = paradigm_loader::load_atoms(&atoms_path)?;
    let ng_seen = contract_mode::scheme_clipped_ng(&ng_text_raw);
    let topic_md_text = std::fs::read_to_string(&topic_path).map_err(|e| super_eio(&e))?;
    let (system_prompt, user_prompt) =
        contract_mode::build_integrator_prompts(&atoms, &topic_md_text, &ng_seen);
    let shots_list = contract_mode::make_shots(&system_prompt, &user_prompt, &gid, shots, 1);
    // 方向名的集成面校验（integrator 第一方向缺位即用法错）
    if atoms.get(&atom_name).is_none() {
        return Err(sih_engine::attractor::jsonc::verr(format!(
            "atom '{}' 不在 atoms 中",
            atom_name
        )));
    }
    contract_mode::emit_contract(
        &out_path,
        &json!({"framework": seat_parts[0], "model_id": seat_parts[1], "version": version_label}),
        &json!({
            "paradigm_id": paradigm_id,
            "atom": atom_name,
            "direction": direction_label,
            "ng_label": ng_label,
            "ng_text_sha256": contract_mode::sha256_bytes(ng_seen.as_bytes()),
        }),
        &json!({
            "gid": gid,
            "title": title,
            "topic_sha256": contract_mode::sha256_file(&topic_path)?,
        }),
        &shots_list,
        &json!({"measurement_entry": topic_path.to_string_lossy(), "n_declared": n_declared}),
    )?;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "emitted": out_path.to_string_lossy(),
            "gid": gid,
            "n_shots": shots,
        }))
        .unwrap_or_default()
    );
    Ok(())
}

fn score_args(args: &Args) -> Result<(), PyError> {
    let contract_path = PathBuf::from(args.get("contract").map_err(eusage)?);
    let responses_path = PathBuf::from(args.get("responses").map_err(eusage)?);
    let trail_path = PathBuf::from(args.get("trail").map_err(eusage)?);
    let out_path = PathBuf::from(args.get("out").map_err(eusage)?);
    let identity_hash = args.get("identity-hash").map_err(eusage)?;
    let gate_verdict = args.get("gate-verdict").map_err(eusage)?;
    let material = contract_mode::score_pipeline(
        &contract_path,
        &responses_path,
        &trail_path,
        &out_path,
        &identity_hash,
        &gate_verdict,
    )?;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "score_material": out_path.to_string_lossy(),
            "gid": material["gid"],
            "n_shots": material["n_shots"],
            "voids": material["voids"].as_array().map(|a| a.len()).unwrap_or(0),
            "runs_written": material["runs_written"],
            "gate_verdict": material["gate_verdict"],
        }))
        .unwrap_or_default()
    );
    Ok(())
}

fn eusage(msg: impl std::fmt::Display) -> PyError {
    PyError { kind: "UsageError", msg: msg.to_string() }
}

fn super_eio(e: &std::io::Error) -> PyError {
    PyError { kind: "OSError", msg: format!("{}", e) }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if argv.is_empty() {
        eprintln!("{}", USAGE);
        return ExitCode::from(2);
    }
    let sub = argv[0].as_str();
    let rest = &argv[1..];
    let result: Result<i32, PyError> = match sub {
        "emit-contract" => match Args::parse(rest) {
            Ok(a) => emit_args(&a).map(|_| 0),
            Err(msg) => {
                eprintln!("{}\n{}", msg, USAGE);
                return ExitCode::from(2);
            }
        },
        "score" => match Args::parse(rest) {
            Ok(a) => score_args(&a).map(|_| 0),
            Err(msg) => {
                eprintln!("{}\n{}", msg, USAGE);
                return ExitCode::from(2);
            }
        },
        "check" => match Args::parse(rest) {
            Ok(a) => match a.get("material") {
                Ok(m) => match tally::check_material(&PathBuf::from(m)) {
                    Ok(report) => {
                        println!("{}", sih_engine::attractor::jsonc::canonical_json(&report));
                        Ok(if report["verdict"] == "pass" { 0 } else { 1 })
                    }
                    Err(e) => Err(e),
                },
                Err(msg) => {
                    eprintln!("{}\n{}", msg, USAGE);
                    return ExitCode::from(2);
                }
            },
            Err(msg) => {
                eprintln!("{}\n{}", msg, USAGE);
                return ExitCode::from(2);
            }
        },
        "verify" => match Args::parse(rest) {
            Ok(a) => match (a.get("material"), a.get("report")) {
                (Ok(m), Ok(r)) => tally::verify_material(&PathBuf::from(m), &PathBuf::from(r)),
                _ => {
                    eprintln!("{}\n{}", "缺 --material 或 --report", USAGE);
                    return ExitCode::from(2);
                }
            },
            Err(msg) => {
                eprintln!("{}\n{}", msg, USAGE);
                return ExitCode::from(2);
            }
        },
        "sign" => match Args::parse(rest) {
            Ok(a) => match (
                a.get("material"),
                a.get("out"),
                a.get("trail"),
                a.get("scribe-binary"),
                a.get("session"),
                a.get("locks"),
            ) {
                (Ok(m), Ok(o), Ok(t), Ok(s), Ok(sess), Ok(l)) => {
                    tally::sign_material(&PathBuf::from(m), &PathBuf::from(o), &t, &s, &sess, &l)
                }
                _ => {
                    eprintln!("{}\n{}", "sign 缺必需旗标", USAGE);
                    return ExitCode::from(2);
                }
            },
            Err(msg) => {
                eprintln!("{}\n{}", msg, USAGE);
                return ExitCode::from(2);
            }
        },
        "watch" => match Args::parse(rest) {
            Ok(a) => match a.get("reports") {
                Ok(r) => tally::watch_reports(&PathBuf::from(r)),
                Err(msg) => {
                    eprintln!("{}\n{}", msg, USAGE);
                    return ExitCode::from(2);
                }
            },
            Err(msg) => {
                eprintln!("{}\n{}", msg, USAGE);
                return ExitCode::from(2);
            }
        },
        _ => {
            eprintln!("未知子命令 {}\n{}", sub, USAGE);
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            // 错误信封出 stderr；退出码按子命令分域：score 与 emit-contract
            // 的拒收（ValueError）出 1，其余域错误一律环境错误出 2（对齐围
            // 堰 tally main 的 ValueError→2 语义）
            eprintln!("{{\"error\": \"{}\"}}", e);
            let rejectable = matches!(sub, "score" | "emit-contract");
            let code = if rejectable && e.kind == "ValueError" { 1 } else { 2 };
            ExitCode::from(code)
        }
    }
}

// Value 引用保持（score 输出经 json! 构造）。
#[allow(dead_code)]
fn _value_ref(_v: &Value) {}
