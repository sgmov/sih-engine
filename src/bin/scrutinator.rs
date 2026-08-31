//! 引擎侧核阅组件命令行面，承接 DEC-007#decision-component 与 SPEC-013。
//!
//! 视图零写零 LLM，承 DEC-006 本名回滚即代码标识符 scrutinator。
//! 退出码三值：0 = 零违规、1 = 存在违规、2 = 工具自身异常含链不可读与域外。
//! 规则包装载方式：编译期内嵌（include_str!），运行时不传 --pack 走默认三包。

use serde::Serialize;
use sih_engine::scrutiny::{
    asset::PACK_NAMES, load_packs, render_report, run_rules_on_json, run_rules_on_text,
};
use std::path::PathBuf;
use std::process::exit;

#[derive(Serialize)]
struct CliOutput {
    error: String,
}

fn emit(value: serde_json::Value, code: i32) -> ! {
    println!("{}", serde_json::to_string_pretty(&value).unwrap());
    exit(code)
}

fn read_text(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts: Vec<(String, String)> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        if args[i].starts_with("--") && i + 1 < args.len() {
            opts.push((args[i].trim_start_matches('-').to_string(), args[i + 1].clone()));
            i += 2;
        } else {
            i += 1;
        }
    }
    let opt = |k: &str| -> Option<String> {
        opts.iter().find(|(n, _)| n == k).map(|(_, v)| v.clone())
    };

    // 装载：--pack 可重复，未传走默认三包全量
    let pack_names: Vec<&str> = if opts.iter().any(|(n, _)| n == "pack") {
        opts.iter()
            .filter(|(n, _)| n == "pack")
            .map(|(_, v)| v.as_str())
            .collect()
    } else {
        PACK_NAMES.to_vec()
    };

    // 域判定：任何 --target 不在包域内则退出码 2
    let target_paths: Vec<String> = opts
        .iter()
        .filter(|(n, _)| n == "target")
        .map(|(_, v)| v.clone())
        .collect();
    if target_paths.is_empty() {
        emit(
            serde_json::to_value(&CliOutput {
                error: "用法 scrutinator --pack <包> --target <目标> [--pack <包>...]".to_string(),
            })
            .unwrap(),
            2,
        );
    }

    // 缺包检查：未知名包产退出码 2
    let packs_data = match load_packs(&pack_names) {
        Ok(v) => v,
        Err(e) => {
            emit(
                serde_json::to_value(&CliOutput { error: e }).unwrap(),
                2,
            );
        }
    };

    // 域判定：每个 target 必须命中至少一个包域；都未命中则产 domain_mismatch 并退出码 2
    let mut all_in_domain = true;
    let mut domain_mismatches: Vec<String> = Vec::new();
    for path in &target_paths {
        let in_any = packs_data.iter().any(|(_, dom, _)| {
            let abs = std::fs::canonicalize(path)
                .ok()
                .and_then(|p| p.to_str().map(String::from))
                .unwrap_or_else(|| path.clone());
            sih_engine::scrutiny::rule::domain_match(dom, &abs)
                || sih_engine::scrutiny::rule::domain_match(dom, path)
        });
        if !in_any {
            all_in_domain = false;
            domain_mismatches.push(path.clone());
        }
    }
    if !all_in_domain {
        let mut out = serde_json::json!({
            "error": "目标不在任何已加载规则包声明的治理域内",
            "unmatched_targets": domain_mismatches,
        });
        for (name, dom, _) in &packs_data {
            out["packs"]
                .as_object_mut()
                .unwrap()
                .insert(name.clone(), serde_json::json!({"include": dom.include, "exclude": dom.exclude}));
        }
        emit(out, 2);
    }

    // 加载目标文本/JSON
    let mut targets: Vec<(String, Option<String>)> = Vec::new();
    for path in &target_paths {
        let text = read_text(path);
        targets.push((path.clone(), text));
    }

    // 跑规则
    let mut findings = Vec::new();
    for (pack_name, _dom, rules) in &packs_data {
        for (path, text_opt) in &targets {
            if let Some(text) = text_opt {
                // 判断材料轴：manifest 标 json 走 json 规则，否则 text
                let manifest = sih_engine::scrutiny::asset::manifest(pack_name);
                let is_json_pack = manifest.contains("material = \"json\"")
                    || manifest.contains("material=\"json\"");
                if is_json_pack {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
                        findings.extend(run_rules_on_json(pack_name, rules, &v));
                    }
                } else {
                    findings.extend(run_rules_on_text(pack_name, rules, text));
                }
                let _ = path;
            }
        }
    }

    let report = render_report(&packs_data, &targets, &findings);
    let code = if report.findings.is_empty() { 0 } else { 1 };
    emit(serde_json::to_value(&report).unwrap(), code);
}
