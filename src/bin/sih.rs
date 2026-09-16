//! sih 命令薄壳：`sih init` 开域单步窄口（adoptface 批）。
//!
//! 职能窄口：调 mcpserver::bootstrap 的 init_precheck（六项预检，逐项判词）
//! 加 open_domain（落地五步加开域毕验域），stdout 出结构化 JSON 单对象；
//! 域根缺省取 cwd，--root 可传。零签发零镜像零客户端注册——全链五段位归
//! sihmcp bootstrap（bootstrap_domain 单一 canonical 路径），本壳零重实现。
//!
//! 正典指针：SPEC-025（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md，
//! 引擎 bin 位融回线）；DEC-023（sih-engine/doc/decision/023-mcp-rust-carrier.md，
//! MCP 线 Rust 载体定约）；SPEC-026（sih-engine/doc/spec/
//! SPEC-026-engine-test-design-v1.md，配套 tests/sih_init.rs 黑箱测试）；
//! 行为对等基准 sih-tools/mcpline/src/mcpline/init.py（开域单步窄口正典）；
//! 承载模块自带正典 DES-015 与 DES-014 与 SPEC-023（bootstrap.rs
//! CANON_POINTERS 同源照录）。
//!
//! 退出码对齐引擎惯例（bootstrap run_cli 与 BootstrapError 同形）：
//! 0 成功；1 前置拒或验红（教学 JSON 出 stdout）；2 工具自身异常与用法错
//! （usage 出 stderr，教学 JSON 出 stdout）。

use std::path::PathBuf;

use serde_json::{json, Value};

use sih_engine::mcpserver::bootstrap::{
    central_registry_path, init_precheck, open_domain, BootstrapError, DEFAULT_OPENED_BY,
};
use sih_engine::mcpserver::runtime::{resolve_root, today_str};

const COMMAND: &str = "sih init";
const USAGE: &str = "usage: sih init [--root <域根>]（域根缺省=当前目录；开域窄口详见域自述卡 sih/README.md 与 DES-015）";

/// 六项预检名录与拒语锚。名录序与锚词同 bootstrap::init_precheck（序固定
/// 「一」至「六」拒语）逐字对表——锚取各拒句唯一子串（「中央登记册」一锚兼容
/// tool_error 的「工具自身异常：」前缀形）；锚漂移即失配，判词回落 unverified
/// 如实显形零假判（本表是出参整形元数据，预检行为本体仍在 bootstrap）。
const PRECHECK_ITEMS: [(&str, &[&str]); 6] = [
    ("域根存在且为目录", &["域根缺席：", "域根非目录："]),
    ("域根下 .git 在位", &["域根下 .git 缺席："]),
    (
        "中央登记册本域 active 标识牌行在档且同路",
        &["中央登记册"],
    ),
    (
        "域根不等于中央根（第一域映射形不开域）",
        &["域根等于中央根："],
    ),
    ("sih/domain.json 缺席（幂等守卫）", &["域已开域"]),
    ("半态拒：sih/ 非空但 domain.json 缺席", &["半态："]),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = tokio::runtime::Runtime::new()
        .expect("sih：tokio 运行时建立失败（工具自身异常）")
        .block_on(run(args));
    std::process::exit(code);
}

/// 薄壳主体：argv 解析 → init_precheck 逐项判词 → open_domain → 结构化出参。
/// 退出码即 BootstrapError.exit_code（1 前置拒或验红、2 工具自身异常），
/// 用法错 2（usage 出 stderr）。
async fn run(args: Vec<String>) -> i32 {
    let mut it = args.iter();
    match it.next().map(String::as_str) {
        None => {
            eprintln!("{USAGE}");
            return 2;
        }
        Some("-h") | Some("--help") => {
            println!("{USAGE}");
            return 0;
        }
        Some("init") => {}
        Some(other) => {
            eprintln!("sih：未知子命令：{other}");
            eprintln!("{USAGE}");
            return 2;
        }
    }
    let mut root: Option<String> = None;
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--root" => match it.next() {
                Some(v) => root = Some(v.clone()),
                None => {
                    eprintln!("sih：--root 缺值");
                    eprintln!("{USAGE}");
                    return 2;
                }
            },
            other => {
                eprintln!("sih：未知参数：{other}");
                eprintln!("{USAGE}");
                return 2;
            }
        }
    }
    let dom = root.map(PathBuf::from).unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });
    let central_root = resolve_root();
    let registry = central_registry_path(&central_root);
    let date = today_str();

    match init_precheck(&dom, &central_root, &registry) {
        Ok((dom2, row)) => match open_domain(&dom2, &row, DEFAULT_OPENED_BY, &date).await {
            Ok(opened) => {
                emit(&json!({
                    "ok": true,
                    "command": COMMAND,
                    "central_root": central_root.display().to_string(),
                    "registry": registry.display().to_string(),
                    "domain_root": dom2.display().to_string(),
                    "opened_by": DEFAULT_OPENED_BY,
                    "date": date,
                    "precheck": {
                        "verdict": "pass",
                        "items": items_pass(),
                    },
                    "domain": opened,
                }));
                0
            }
            Err(e) => {
                let mut out = base_rejection(
                    "open_domain",
                    &dom2,
                    &central_root,
                    &registry,
                    &e,
                );
                out["precheck"] = json!({"verdict": "pass", "items": items_pass()});
                emit(&out);
                e.exit_code
            }
        },
        Err(e) => {
            let err = payload_str(&e, "error").unwrap_or_default();
            let mut out = base_rejection("precheck", &dom, &central_root, &registry, &e);
            out["precheck"] = precheck_rejected(&err);
            emit(&out);
            e.exit_code
        }
    }
}

/// 拒形基底：ok 假、阶段、面位三径与 error 加 fix（fix 在位才带）加退出码。
fn base_rejection(
    stage: &str,
    dom: &std::path::Path,
    central_root: &std::path::Path,
    registry: &std::path::Path,
    e: &BootstrapError,
) -> Value {
    let mut out = json!({
        "ok": false,
        "command": COMMAND,
        "stage": stage,
        "central_root": central_root.display().to_string(),
        "registry": registry.display().to_string(),
        "domain_root": dom.display().to_string(),
        "error": payload_str(e, "error").unwrap_or_default(),
    });
    if let Some(fix) = payload_str(e, "fix") {
        out["fix"] = json!(fix);
    }
    out
}

/// 预检全过判词（六项逐项 pass）。
fn items_pass() -> Vec<Value> {
    PRECHECK_ITEMS
        .iter()
        .map(|(name, _)| json!({"item": name, "verdict": "pass"}))
        .collect()
}

/// 预检拒形逐项判词：init_precheck 序固定 fail-fast——命中锚即第 k 项 fail
/// （项内带原拒语），k 前序项机械 pass，k 后序项未及即 unverified；锚零命中
/// 即六项全 unverified（如实，零假判）。
fn precheck_rejected(err: &str) -> Value {
    let hit = PRECHECK_ITEMS
        .iter()
        .position(|(_, anchors)| anchors.iter().any(|a| err.contains(a)));
    let items: Vec<Value> = PRECHECK_ITEMS
        .iter()
        .enumerate()
        .map(|(i, (name, _))| {
            let verdict = match hit {
                Some(k) if i < k => "pass",
                Some(k) if i == k => "fail",
                _ => "unverified",
            };
            let mut item = json!({"item": name, "verdict": verdict});
            if Some(i) == hit {
                item["detail"] = json!(err);
            }
            item
        })
        .collect();
    json!({
        "verdict": "rejected",
        "failed_item": hit.map(|k| json!(PRECHECK_ITEMS[k].0)).unwrap_or(Value::Null),
        "items": items,
    })
}

/// 引擎错误载荷取字符串字段（error 与 fix 均可）。
fn payload_str(e: &BootstrapError, key: &str) -> Option<String> {
    e.payload.get(key).and_then(Value::as_str).map(str::to_string)
}

/// 结构化出参单口：stdout 缩进 JSON 单对象。
fn emit(v: &Value) {
    match serde_json::to_string_pretty(v) {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("sih：结果序列化失败（工具自身异常）：{e}"),
    }
}
