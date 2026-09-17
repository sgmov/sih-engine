//! retriever 即温故宿主命令面，承接 SPEC-007#interface-signature 与 SPEC-008#boundary 与修订六。
//!
//! 子命令单件 recall，参数十件即 topic 与 word 与 event 与 since 与 until 与 archive 与
//! root 与 miss-log 与 at 与 out，退出码三值即零成功一拦二异常。根定位 --root 显式给参
//! 优先（须为布局两形根即 first_domain 双仓目录或 canonical sih/ledger，俱缺 exit 2），
//! 缺省从当前目录上溯，不读系统钟。wengumcp-parallel 批增 --root 与 canonical 两形。
//! retrieverline 批（rl-03）承 pk-050 切换形增两旗标：缺省即语义通道 K=3（确定性统计
//! 向量），--lexical 词面回退位即切前词面行为，--semantic K 显式给参即其值（0 仍即关）；
//! 既有旗标全部向后兼容，既有参数形与输出 NDJSON 行形零变。

use std::path::PathBuf;

use sih_engine::retriever::{
    derive_root, exit_code, layout_form, recall, rows_to_ndjson, semantic, write_output_envelope,
    RecallArgs,
};

fn fail(message: &str, code: i32) -> ! {
    eprintln!("{{\"error\": \"{message}\"}}");
    std::process::exit(code)
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if argv.first().map(|s| s.as_str()) != Some("recall") {
        fail("用法 retriever recall --topic 词 --word 词 --event 记号 --since 日期 --until 日期 --archive 档名 --root 路径 --miss-log 路径 --at 日期 --out 路径 --lexical --semantic K", 2);
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
    let mut explicit_root: Option<PathBuf> = None;
    let mut lexical = false;
    let mut semantic_flag: Option<usize> = None;

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
            "--root" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                explicit_root = Some(PathBuf::from(v.clone()));
                i += 2;
            }
            "--lexical" => {
                lexical = true;
                i += 1;
            }
            "--semantic" => {
                let Some(v) = argv.get(i + 1) else { fail("参数缺值", 2); };
                let Ok(k) = v.parse::<usize>() else {
                    fail("参数非法：--semantic 须非负整数（0 即关）", 2);
                };
                semantic_flag = Some(k);
                i += 2;
            }
            other => fail(&format!("未知参数 {other}"), 2),
        }
    }

    // 缺省位裁决承 wikirecall resolve_semantic_k 对表：--lexical 显式回退即词面
    //（None）；--semantic 显式给参即其值（0 仍即关）；无旗标缺省即语义层 K=3。
    let semantic = if lexical {
        None
    } else {
        match semantic_flag {
            Some(0) => None,
            Some(k) => Some(k),
            None => Some(semantic::DEFAULT_SEMANTIC_K),
        }
    };

    let root = match explicit_root {
        Some(path) => {
            if layout_form(&path).is_none() {
                fail("显式根非布局两形：first_domain 双仓目录与 canonical sih/ledger 俱缺，拒零动作", 2);
            }
            path
        }
        None => {
            let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            let Some(root) = derive_root(&cwd) else {
                fail("工作区根不可定位即上溯无 sih-engine 与 sih-tools 并存层亦无 canonical sih/ledger 形", 2);
            };
            root
        }
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
        semantic,
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
