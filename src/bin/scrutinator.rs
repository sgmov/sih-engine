//! 引擎侧核阅组件命令行面，承接 DEC-007#decision-component 与 SPEC-013。
//!
//! 视图零写零 LLM，承 DEC-006 本名回滚即代码标识符 scrutinator。
//! 退出码三值：0 = 零违规、1 = 存在违规、2 = 工具自身异常含链不可读与域外。
//! 规则包装载方式：编译期内嵌（include_str!），运行时不传 --pack 走默认三包。
//!
//! CLI 双形（承 SPEC-013 修订二）：
//! - 位置参数形（正典工具兼容形）：`scrutinator --pack <包> <目标> [<目标>...]`
//! - --target 旗标形（引擎别名）：`scrutinator --pack <包> --target <目标> [--target <目标>...]`
//! 两形并存期间同传即按出现顺序取并集。
//!
//! 空载形：未传 --pack 即 packs=[] 报告，零发现，退出码 0。

use serde::Serialize;
use sih_engine::scrutinator::{
    asset::PACK_NAMES, load_packs, render_report, run_rules_on_json, run_rules_on_text,
};
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

/// CLI 解析结果。
/// opts: 旗标位（不含 --pack 后立即的位置参数）
/// positional_targets: 位置参数按出现顺序收集的目标
struct Parsed {
    /// 每个 --pack 的值，按出现顺序
    pack_names: Vec<String>,
    /// 每个 --target 的值（旗标别名），按出现顺序
    flag_targets: Vec<String>,
    /// 位置参数目标，按出现顺序（--pack 旗标值之后、非 -- 开头的参数）
    positional_targets: Vec<String>,
}

/// 解析 CLI 参数。
///
/// 关键约定：
/// - `--pack <值>`：可重复，<值> 必传
/// - `--target <值>`：可重复，<值> 必传（旗标别名形）
/// - 位置参数：跟在某个 --pack 之后、不是 -- 开头、不是上一旗标值的参数都视为 target
///   直到下一个 -- 开头的旗标
/// - 旗标值 = 旗标后第一个非自身 token
/// - 同形并存：旗标 target 与位置参数 target 按出现顺序取并集
fn parse_args(args: &[String]) -> Parsed {
    let mut pack_names: Vec<String> = Vec::new();
    let mut flag_targets: Vec<String> = Vec::new();
    let mut positional_targets: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--pack" {
            if i + 1 < args.len() {
                pack_names.push(args[i + 1].clone());
                i += 2;
            } else {
                i += 1;
            }
        } else if args[i] == "--target" {
            if i + 1 < args.len() {
                flag_targets.push(args[i + 1].clone());
                i += 2;
            } else {
                i += 1;
            }
        } else if args[i].starts_with("--") {
            // 未知旗标：跳过旗标与其值（若 value 不以 -- 开头则认为是旗标值）
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                i += 2;
            } else {
                i += 1;
            }
        } else {
            // 位置参数
            positional_targets.push(args[i].clone());
            i += 1;
        }
    }
    Parsed {
        pack_names,
        flag_targets,
        positional_targets,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // 跳过 argv[0]（binary 名）
    let parsed = parse_args(&args[1..]);

    // 装载：--pack 可重复，未传走默认三包全量
    let pack_names: Vec<String> = if !parsed.pack_names.is_empty() {
        parsed.pack_names.clone()
    } else {
        // 未传 --pack = 走默认三包（决定空载形与否：传 --pack 但取空值才是空载）
        PACK_NAMES.iter().map(|s| s.to_string()).collect()
    };

    // 缺包检查：未知名包产退出码 2
    let pack_refs: Vec<&str> = pack_names.iter().map(|s| s.as_str()).collect();
    let packs_data = match load_packs(&pack_refs) {
        Ok(v) => v,
        Err(e) => {
            emit(
                serde_json::to_value(&CliOutput { error: e }).unwrap(),
                2,
            );
        }
    };

    // 目标收集：旗标 target + 位置参数 target 按出现顺序取并集
    // 这里为了语义最简：合并去重但保持首次出现顺序
    let mut target_paths: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for t in &parsed.flag_targets {
        if seen.insert(t.clone()) {
            target_paths.push(t.clone());
        }
    }
    for t in &parsed.positional_targets {
        if seen.insert(t.clone()) {
            target_paths.push(t.clone());
        }
    }

    // 空载形：未传任何 target（包括位置参数），产 packs=[] 报告、findings=[]、退出码 0
    if target_paths.is_empty() {
        let report = render_report(&packs_data, &[], &[]);
        emit(serde_json::to_value(&report).unwrap(), 0);
    }

    // 域判定：每个 target 必须命中至少一个包域；都未命中则产 domain_mismatch 并退出码 2
    let mut all_in_domain = true;
    let mut domain_mismatches: Vec<String> = Vec::new();
    for path in &target_paths {
        let in_any = packs_data.iter().any(|(_, dom, _)| {
            let abs = std::fs::canonicalize(path)
                .ok()
                .and_then(|p| p.to_str().map(String::from))
                .unwrap_or_else(|| path.clone());
            sih_engine::scrutinator::rule::domain_match(dom, &abs)
                || sih_engine::scrutinator::rule::domain_match(dom, path)
        });
        if !in_any {
            all_in_domain = false;
            domain_mismatches.push(path.clone());
        }
    }
    if !all_in_domain {
        let mut packs_map = serde_json::Map::new();
        for (name, dom, _) in &packs_data {
            packs_map.insert(
                name.clone(),
                serde_json::json!({"include": dom.include, "exclude": dom.exclude}),
            );
        }
        let out = serde_json::json!({
            "error": "目标不在任何已加载规则包声明的治理域内",
            "unmatched_targets": domain_mismatches,
            "packs": serde_json::Value::Object(packs_map),
        });
        emit(out, 2);
    }

    // 加载目标文本/JSON
    let mut targets: Vec<(String, Option<String>)> = Vec::new();
    for path in &target_paths {
        let text = read_text(path);
        targets.push((path.clone(), text));
    }

    // 跑规则：每个 pack 只跑在它域内的 target
    let mut findings = Vec::new();
    for (pack_name, dom, rules) in &packs_data {
        for (path, text_opt) in &targets {
            if let Some(text) = text_opt {
                // 域内判定：target 在 pack 域内才跑
                let abs = std::fs::canonicalize(path)
                    .ok()
                    .and_then(|p| p.to_str().map(String::from))
                    .unwrap_or_else(|| path.clone());
                let in_domain = sih_engine::scrutinator::rule::domain_match(dom, &abs)
                    || sih_engine::scrutinator::rule::domain_match(dom, path);
                if !in_domain {
                    continue;
                }
                // 判断材料轴：manifest 标 json 走 json 规则，否则 text
                let manifest = sih_engine::scrutinator::asset::manifest(pack_name);
                let is_json_pack = manifest.contains("material = \"json\"")
                    || manifest.contains("material=\"json\"");
                if is_json_pack {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
                        findings.extend(run_rules_on_json(pack_name, rules, &v));
                    }
                } else {
                    findings.extend(run_rules_on_text(pack_name, rules, text));
                }
            }
        }
    }

    let report = render_report(&packs_data, &targets, &findings);
    let code = if report.findings.is_empty() { 0 } else { 1 };
    emit(serde_json::to_value(&report).unwrap(), code);
}
