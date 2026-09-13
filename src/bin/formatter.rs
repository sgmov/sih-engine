//! 引擎侧化格（Formatter）命令行面 —— lease-mergeall-parallel 簇B 移植件。
//!
//! 行为面移植自围堰 sih-tools/formatter 0.2.0（src/formatter/：cli.py、packs.py、
//! format.py、report.py；域判定借 sih-tools/scrutinator/src/scrutinator/domain.py 的
//! in_domain），只读对表移植，围堰源码零改动。
//!
//! CLI：`formatter --pack <格式包目录> [--pack ...] [--write] [--quiet] <目标>...`
//! 退出码三值：0 = 无需改、1 = 已有改（--write 即已落写）、2 = 工具异常。
//! 幂等构成性：同输入二遍格式化输出恒同，重跑落零（R 重写收敛不动点）。
//!
//! 落差申报（相对围堰）：
//! 1. json_canonicalize 的 indent 参数仅支持 2（serde_json pretty 固定两空格缩进），
//!    非 2 值按 2 出；默认 2 与围堰一致。
//! 2. json_canonicalize 的 ensure_ascii=true 未实装（恒按 false 出原 UTF-8）；
//!    默认 false 与围堰一致。
//! 3. Python json 的 NaN/Infinity 字面量接受性与浮点最短表示边缘形未对齐
//!    （serde_json 拒 NaN/Infinity）。
//! 4. 用法错（缺 --pack、缺目标、未知旗标）为自足解析，stderr 短句回报，
//!    退出码 2 与围堰 argparse 对齐，报文形不对齐。
//! 5. 读取失败与解码失败的 mismatch reason 内文不对齐（退出码与结构对齐）。
//! 6. envelope_version 判等按 JSON 整数 1 严格判（Python 1.0 == 1 宽判未对齐）。

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::process::exit;

/// 围堰版本锚：sih-tools/formatter/src/formatter/__init__.py __version__。
const ENGINE_VERSION: &str = "0.2.0";

/// 包允许的 family 全集（envelope 共享形，packs.py 逐字对表）。
const FAMILIES: [&str; 4] = ["scrutinator", "attractor", "formatter", "nomenclator"];

struct Op {
    #[allow(dead_code)]
    id: String,
    kind: String,
    params: toml::value::Table,
}

struct FPack {
    name: String,
    version: String,
    include_raw: Vec<String>,
    exclude_raw: Vec<String>,
    include: Vec<regex::Regex>,
    exclude: Vec<regex::Regex>,
    ops: Vec<Op>,
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: formatter --pack <格式包目录> [--pack <格式包目录>...] [--write] [--quiet] <目标>...");
    exit(2);
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

/// 域 glob 编译（scrutinator/domain.py `_compile` 逐字对表）：
/// 双星跨目录段，单星段内任意，问号单字，模式可自任一路径边界锚定。
fn glob_to_regex(pattern: &str) -> String {
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::from("(?:^|/)");
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i..].starts_with(&['*', '*', '/']) {
            out.push_str("(?:[^/]+/)*");
            i += 3;
        } else if chars[i] == '*' {
            out.push_str("[^/]*");
            i += 1;
        } else if chars[i] == '?' {
            out.push_str("[^/]");
            i += 1;
        } else {
            out.push_str(&regex::escape(&chars[i].to_string()));
            i += 1;
        }
    }
    out
}

/// 先查 exclude 后查 include，exclude 一票否决（domain.in_domain 语义）。
fn in_domain(path: &str, pack: &FPack) -> bool {
    let p = path.replace('\\', "/");
    for re in &pack.exclude {
        if re.is_match(&p) {
            return false;
        }
    }
    for re in &pack.include {
        if re.is_match(&p) {
            return true;
        }
    }
    false
}

/// Python path.resolve() 非严格形：canonicalize 优先，失败退绝对化拼接。
fn resolve_abs(p: &Path) -> String {
    if let Ok(c) = fs::canonicalize(p) {
        return c.to_string_lossy().into_owned();
    }
    if p.is_absolute() {
        p.to_string_lossy().into_owned()
    } else {
        std::env::current_dir()
            .unwrap_or_default()
            .join(p)
            .to_string_lossy()
            .into_owned()
    }
}

/// envelope 校验（packs.py load_pack 前半逐字对表）。
fn validate_envelope(dir: &Path) -> Result<(), String> {
    let env_path = dir.join("envelope.json");
    if !env_path.is_file() {
        return Err(format!("envelope missing: {}", dir.display()));
    }
    let text = fs::read_to_string(&env_path)
        .map_err(|e| format!("envelope invalid: {}: {}", dir.display(), e))?;
    let env: Value = serde_json::from_str(&text)
        .map_err(|e| format!("envelope invalid: {}: {}", dir.display(), e))?;
    if env.get("envelope_version").and_then(|v| v.as_i64()) != Some(1) {
        return Err(format!(
            "envelope invalid: {}: envelope_version must be 1",
            dir.display()
        ));
    }
    let id_ok = env
        .get("id")
        .and_then(|v| v.as_str())
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    if !id_ok {
        return Err(format!(
            "envelope invalid: {}: id must be non-empty string",
            dir.display()
        ));
    }
    let family = env.get("family").and_then(|v| v.as_str()).unwrap_or("");
    if !FAMILIES.contains(&family) {
        return Err(format!("envelope invalid: {}: family illegal", dir.display()));
    }
    let body_type = env.get("body_type").and_then(|v| v.as_str()).unwrap_or("");
    if body_type != "config" && body_type != "doc_spec" {
        return Err(format!(
            "envelope invalid: {}: body_type illegal",
            dir.display()
        ));
    }
    let bodies: Vec<&str> = env
        .get("bodies")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|b| b.as_str()).collect())
        .unwrap_or_default();
    let bodies_all_str = env
        .get("bodies")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty() && a.iter().all(|b| b.is_string()))
        .unwrap_or(false);
    if !bodies_all_str {
        return Err(format!(
            "envelope invalid: {}: bodies must be non-empty string list",
            dir.display()
        ));
    }
    for b in bodies {
        if !dir.join(b).is_file() {
            return Err(format!("envelope body missing: {}: {}", dir.display(), b));
        }
    }
    Ok(())
}

/// 格式包装载（packs.py load_pack 后半对表）：manifest 三键，ops 每条至少 id、kind。
fn load_pack(dir: &Path) -> Result<FPack, String> {
    validate_envelope(dir)?;
    let manifest_path = dir.join("manifest.toml");
    let ops_path = dir.join("operations.toml");
    for p in [&manifest_path, &ops_path] {
        if !p.is_file() {
            return Err(format!("格式包缺文件: {}", p.display()));
        }
    }
    let manifest_text = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("TOML 解析失败: {e}"))?;
    let manifest: toml::Value =
        toml::from_str(&manifest_text).map_err(|e| format!("TOML 解析失败: {e}"))?;
    let ops_text = fs::read_to_string(&ops_path).map_err(|e| format!("TOML 解析失败: {e}"))?;
    let raw: toml::Value = toml::from_str(&ops_text).map_err(|e| format!("TOML 解析失败: {e}"))?;

    let name = match manifest.get("name").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => return Err("manifest.name 必须为非空字符串".to_string()),
    };
    let version = match manifest.get("version").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => return Err("manifest.version 必须为非空字符串".to_string()),
    };
    let domain = match manifest.get("domain") {
        Some(d) if d.is_table() => d.clone(),
        _ => return Err("manifest.domain 必须为表".to_string()),
    };
    let include_raw: Vec<String> = match domain.get("include").and_then(|v| v.as_array()) {
        Some(a) if !a.is_empty() && a.iter().all(|x| x.is_str()) => {
            a.iter().map(|x| x.as_str().unwrap().to_string()).collect()
        }
        _ => return Err("manifest.domain.include 必须为非空字符串数组".to_string()),
    };
    let exclude_raw: Vec<String> = match domain.get("exclude") {
        None => vec![],
        Some(v) => match v.as_array() {
            Some(a) if a.iter().all(|x| x.is_str()) => a
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
            _ => return Err("manifest.domain.exclude 必须为字符串数组".to_string()),
        },
    };
    let include = include_raw
        .iter()
        .map(|p| regex::Regex::new(&glob_to_regex(p)).unwrap())
        .collect();
    let exclude = exclude_raw
        .iter()
        .map(|p| regex::Regex::new(&glob_to_regex(p)).unwrap())
        .collect();

    let ops_raw = match raw.get("ops").and_then(|v| v.as_array()) {
        Some(a) if !a.is_empty() => a.clone(),
        _ => return Err("operations.toml 的 ops 必须为非空数组".to_string()),
    };
    let mut ops = vec![];
    for o in &ops_raw {
        let table = match o.as_table() {
            Some(t) => t,
            None => return Err("操作必须为表".to_string()),
        };
        let oid = match table.get("id").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return Err("操作缺 id".to_string()),
        };
        let kind = match table.get("kind").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return Err(format!("操作 {oid} 缺 kind")),
        };
        let params = match table.get("params") {
            Some(p) if p.is_table() => p.as_table().unwrap().clone(),
            _ => toml::value::Table::new(),
        };
        ops.push(Op { id: oid, kind, params });
    }
    Ok(FPack {
        name,
        version,
        include_raw,
        exclude_raw,
        include,
        exclude,
        ops,
    })
}

fn fence_line(line: &str) -> bool {
    // 围堰 FENCE_RE = ^\s*```，re.match 语义即前缀判。
    line.trim_start().starts_with("```")
}

/// 行级操作：行尾空白、行尾符统一、末行换行；md 时围栏内豁免（format.py 对表）。
fn apply_line_ops(text: &str, params: &toml::value::Table, fence_aware: bool) -> String {
    let eol_lf = params.get("eol").and_then(|v| v.as_str()).unwrap_or("lf") == "lf";
    let trailing = params
        .get("trailing_whitespace")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let final_nl = params
        .get("final_newline")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let mut out: Vec<String> = vec![];
    let mut in_fence = false;
    for line in text.split('\n') {
        if fence_aware && fence_line(line) {
            in_fence = !in_fence;
            out.push(if eol_lf {
                line.trim_end_matches('\r').to_string()
            } else {
                line.to_string()
            });
            continue;
        }
        if in_fence {
            out.push(if eol_lf {
                line.trim_end_matches('\r').to_string()
            } else {
                line.to_string()
            });
            continue;
        }
        let mut s = line;
        if eol_lf {
            s = s.trim_end_matches('\r');
        }
        if trailing {
            s = s.trim_end_matches([' ', '\t']);
        }
        out.push(s.to_string());
    }
    let mut new = out.join("\n");
    if final_nl && !new.is_empty() && !new.ends_with('\n') {
        new.push('\n');
    }
    new
}

/// 对象键递归排序（json.dumps sort_keys=True 语义，serde_json preserve_order 下重建）。
fn sort_value(v: Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut kvs: Vec<(String, Value)> = m.into_iter().collect();
            kvs.sort_by(|a, b| a.0.cmp(&b.0));
            let mut out = Map::new();
            for (k, val) in kvs {
                out.insert(k, sort_value(val));
            }
            Value::Object(out)
        }
        Value::Array(a) => Value::Array(a.into_iter().map(sort_value).collect()),
        other => other,
    }
}

/// json 规范化：解析后按包参数重排；解析失败即 FormatError（退出码二路径）。
fn apply_json_canonicalize(text: &str, params: &toml::value::Table) -> Result<String, String> {
    let data: Value = serde_json::from_str(text).map_err(|e| format!("json 解析失败: {e}"))?;
    let sort_keys = params
        .get("sort_keys")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let data = if sort_keys { sort_value(data) } else { data };
    // 落差：indent 恒按 2；ensure_ascii 恒按 false（见头注申报一、二）。
    let mut s = serde_json::to_string_pretty(&data).map_err(|e| format!("json 解析失败: {e}"))?;
    s.push('\n');
    Ok(s)
}

/// 按序执行命中包的操作（format.py apply_ops 对表）：
/// json 先规范化后行级，其余仅行级；未知 kind 即表外请求拒绝。
fn apply_ops(text: &str, ops: &[(&str, &Op)], is_md: bool, is_json: bool) -> Result<String, String> {
    let mut result = text.to_string();
    for (pack_name, op) in ops {
        if op.kind == "json_canonicalize" {
            if !is_json {
                continue;
            }
            result = apply_json_canonicalize(&result, &op.params)?;
        } else if op.kind == "line_ops" {
            result = apply_line_ops(&result, &op.params, is_md);
        } else {
            return Err(format!("未知操作 kind: {} (包 {})", op.kind, pack_name));
        }
    }
    Ok(result)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut pack_dirs: Vec<String> = vec![];
    let mut write_mode = false;
    let mut quiet = false;
    let mut targets: Vec<String> = vec![];
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].clone();
        if a == "--pack" {
            i += 1;
            if i >= args.len() {
                usage_fail("--pack 缺值");
            }
            pack_dirs.push(args[i].clone());
            i += 1;
        } else if let Some(v) = a.strip_prefix("--pack=") {
            pack_dirs.push(v.to_string());
            i += 1;
        } else if a == "--write" {
            write_mode = true;
            i += 1;
        } else if a == "--quiet" {
            quiet = true;
            i += 1;
        } else if a == "--version" {
            println!("{ENGINE_VERSION}");
            exit(0);
        } else if a.starts_with("--") {
            usage_fail(&format!("未知旗标: {a}"));
        } else {
            targets.push(a);
            i += 1;
        }
    }
    if pack_dirs.is_empty() {
        usage_fail("缺 --pack（必填，可重复）");
    }
    if targets.is_empty() {
        usage_fail("缺目标文件");
    }

    let mut packs: Vec<FPack> = vec![];
    for d in &pack_dirs {
        match load_pack(Path::new(d)) {
            Ok(p) => packs.push(p),
            Err(e) => {
                eprintln!("格式包加载失败: {e}");
                exit(2);
            }
        }
    }

    let mut changes: Vec<Value> = vec![];
    let mut mismatches: Vec<Value> = vec![];
    let mut hashes = Map::new();
    let mut anomaly = false;

    for target in &targets {
        let path = Path::new(target);
        if !path.is_file() {
            anomaly = true;
            mismatches.push(json!({"target": target, "reason": "目标不存在或非文件"}));
            continue;
        }
        let resolved = resolve_abs(path);
        let mut governing: Vec<(&FPack, &Op)> = vec![];
        for p in &packs {
            if in_domain(&resolved, p) {
                for op in &p.ops {
                    governing.push((p, op));
                }
            }
        }
        if governing.is_empty() {
            anomaly = true;
            let names: Vec<&str> = packs.iter().map(|p| p.name.as_str()).collect();
            mismatches.push(json!({
                "target": target,
                "reason": "目标不在任何已加载格式包声明的扩展域内",
                "pack_domains": names,
            }));
            continue;
        }
        let raw = match fs::read(path) {
            Ok(r) => r,
            Err(e) => {
                anomaly = true;
                mismatches.push(json!({"target": target, "reason": format!("读取失败: {e}")}));
                continue;
            }
        };
        hashes.insert(target.clone(), Value::String(sha256_hex(&raw)));
        let text = match String::from_utf8(raw) {
            Ok(t) => t,
            Err(e) => {
                anomaly = true;
                mismatches.push(json!({"target": target, "reason": format!("读取失败: {e}")}));
                continue;
            }
        };
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let is_md = ext == "md";
        let is_json = ext == "json";
        let ops_ref: Vec<(&str, &Op)> = governing.iter().map(|(p, o)| (p.name.as_str(), *o)).collect();
        let new_text = match apply_ops(&text, &ops_ref, is_md, is_json) {
            Ok(t) => t,
            Err(e) => {
                anomaly = true;
                mismatches.push(json!({"target": target, "reason": e}));
                continue;
            }
        };
        if new_text != text {
            let old_lines: Vec<&str> = text.split('\n').collect();
            let new_lines: Vec<&str> = new_text.split('\n').collect();
            let mut changed = old_lines
                .iter()
                .zip(new_lines.iter())
                .filter(|(a, b)| a != b)
                .count();
            changed += old_lines.len().abs_diff(new_lines.len());
            changes.push(json!({"path": target, "changed_lines": changed}));
            if write_mode {
                match fs::write(path, new_text.as_bytes()) {
                    Ok(()) => {
                        if let Ok(nb) = fs::read(path) {
                            hashes.insert(target.clone(), Value::String(sha256_hex(&nb)));
                        }
                    }
                    Err(e) => {
                        anomaly = true;
                        mismatches
                            .push(json!({"target": target, "reason": format!("写入失败: {e}")}));
                        continue;
                    }
                }
            }
        }
    }

    let total_changed: u64 = changes
        .iter()
        .filter_map(|c| c["changed_lines"].as_u64())
        .sum();
    let code: i32 = if anomaly {
        2
    } else if !changes.is_empty() {
        1
    } else {
        0
    };

    if quiet {
        let mut line = Map::new();
        line.insert("tool".to_string(), json!("formatter"));
        line.insert(
            "command".to_string(),
            json!(if write_mode { "write" } else { "check" }),
        );
        line.insert("code".to_string(), json!(code));
        line.insert(
            "summary".to_string(),
            json!({
                "targets_changed": changes.len(),
                "total_changed_lines": total_changed,
                "domain_mismatches": mismatches.len(),
            }),
        );
        println!("{}", sort_value(Value::Object(line)).to_string());
    } else {
        let packs_json: Vec<Value> = packs
            .iter()
            .map(|p| {
                json!({
                    "name": p.name,
                    "version": p.version,
                    "domain": {"include": p.include_raw, "exclude": p.exclude_raw},
                })
            })
            .collect();
        let mut rep = Map::new();
        rep.insert(
            "engine".to_string(),
            json!({"name": "formatter", "version": ENGINE_VERSION}),
        );
        let targets_changed = changes.len();
        rep.insert("packs".to_string(), Value::Array(packs_json));
        rep.insert("targets".to_string(), json!(targets));
        rep.insert("content_hashes".to_string(), Value::Object(hashes));
        rep.insert("changes".to_string(), Value::Array(changes));
        rep.insert("domain_mismatches".to_string(), Value::Array(mismatches));
        rep.insert(
            "summary".to_string(),
            json!({"targets_changed": targets_changed, "total_changed_lines": total_changed}),
        );
        println!(
            "{}",
            serde_json::to_string_pretty(&Value::Object(rep)).unwrap()
        );
    }
    exit(code);
}
