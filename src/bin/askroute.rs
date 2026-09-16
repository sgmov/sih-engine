//! 判定路由（askroute）命令行面 —— gap-askroute-port 引擎位移植件。
//!
//! 正典指针：围堰契约 sih-tools/askroute/CONTRACT.md（intents-v0 包 schema）、
//! DEC-006 三问载体、SPEC-025 腿六长尾收口。围堰 askroute 为纯数据包形
//! （判定包 + test_pack.py drift 守卫，无可执行件），本 bin 为 MCP 强制触发协议
//! （AGENTS.md § sih 强制触发协议）的引擎侧承载位：缩写→意图→动作链解析全部
//! 由包数据机械承载，不经模型生成；表外缩写唯一出口是 unknown_action 三步兜底
//! （查册 → 语料召回 → 问人），禁即兴解释后执行。
//!
//! CLI：`askroute route <缩写> [--pack <包.json>]`；`askroute check [--pack <包.json>]`。
//! 缺省包为编译期内嵌引擎位 intents-v0（src/askroute/packs/intents-v0.json，
//! 与围堰包逐字节随迁）；`--pack` 可指围堰包或其他 intents 包路径做同参双跑。
//!
//! 退出码三值：
//! - route：0 命中在册意图（按 chain 固定拉起动作链）；1 表外缩写落 unknown_action
//!   出口（按 steps 三步兜底）；2 包不可载（schema 破）或用法错。
//! - check：0 drift 守卫全绿；1 有违例（判词逐项在 findings）；2 包不可载或用法错。
//!
//! 本 bin 纯判定零写入：不触链、不落账本、零网络零 LLM。
//! 双跑对表由 tests/mergeall_t6_askroute.rs 承载（T0 包逐字节随迁 +
//! 全缩写表同参双跑逐字节一致 + unknown 出口行为对表）。

use std::process::exit;

use serde_json::{json, Value};

use sih_engine::askroute::{check_pack, parse_pack, route, RouteOutcome, DEFAULT_PACK, PACK_KIND};

const USAGE: &str = "用法：askroute route <缩写> [--pack <包.json>] | askroute check [--pack <包.json>]";

fn emit(payload: &Value, code: i32) -> ! {
    println!("{}", payload);
    exit(code)
}

fn fail(message: String) -> ! {
    emit(&json!({ "error": message }), 2)
}

/// 载包：缺省内嵌引擎位包，--pack 指文件路径。载入失败即退出码 2。
fn load_pack(pack_flag: Option<&str>) -> sih_engine::askroute::Pack {
    let text = match pack_flag {
        None => DEFAULT_PACK.to_string(),
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(err) => fail(format!("包不可读 {err}：{path}")),
        },
    };
    match parse_pack(&text) {
        Ok(pack) => pack,
        Err(err) => fail(err),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let sub = match args.next() {
        Some(sub) => sub,
        None => fail(USAGE.to_string()),
    };
    let mut pack_flag: Option<String> = None;
    let mut positionals: Vec<String> = Vec::new();
    let rest: Vec<String> = args.collect();
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--pack" => {
                let value = match rest.get(i + 1) {
                    Some(value) => value.clone(),
                    None => fail("--pack 需要包文件路径参数".to_string()),
                };
                pack_flag = Some(value);
                i += 2;
            }
            other if other.starts_with("--") => fail(format!("未知旗标 {other}")),
            other => {
                positionals.push(other.to_string());
                i += 1;
            }
        }
    }

    match sub.as_str() {
        // route <缩写>：缩写→意图→动作链，表外落 unknown_action 出口
        "route" => {
            if positionals.len() != 1 {
                fail(format!("route 须恰一位置参数 <缩写>；{USAGE}"));
            }
            let pack = load_pack(pack_flag.as_deref());
            match route(&pack, &positionals[0]) {
                RouteOutcome::Hit { intent, chain, manual } => emit(
                    &json!({
                        "matched": true,
                        "intent": intent,
                        "chain": chain,
                        "manual": manual,
                        "pack": PACK_KIND,
                        "version": pack.version,
                    }),
                    0,
                ),
                RouteOutcome::Unknown(unknown) => emit(
                    &json!({
                        "matched": false,
                        "intent": unknown.id,
                        "steps": unknown.steps,
                        "pack": PACK_KIND,
                        "version": pack.version,
                    }),
                    1,
                ),
            }
        }
        // check：drift 守卫（test_pack.py 守卫面移植），违例逐项判词
        "check" => {
            if !positionals.is_empty() {
                fail(format!("check 不收位置参数；{USAGE}"));
            }
            let pack = load_pack(pack_flag.as_deref());
            let findings = check_pack(&pack);
            if findings.is_empty() {
                let abbreviations: usize = pack.intents.iter().map(|i| i.abbreviations.len()).sum();
                emit(
                    &json!({
                        "check": "ok",
                        "pack": PACK_KIND,
                        "version": pack.version,
                        "intents": pack.intents.len(),
                        "abbreviations": abbreviations,
                    }),
                    0,
                );
            } else {
                emit(
                    &json!({
                        "check": "violations",
                        "pack": PACK_KIND,
                        "version": pack.version,
                        "findings": findings,
                    }),
                    1,
                );
            }
        }
        other => fail(format!("未知子命令 {other}；{USAGE}")),
    }
}
