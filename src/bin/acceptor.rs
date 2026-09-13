//! 引擎侧空腹判定包执行机（acceptor）命令行面 —— lease-mergeleg6-parallel 簇J 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/acceptor 0.2.0（src/acceptor/engine.py 与 cli.py），
//! 只读对表移植，围堰源码零改动。空腹构成性：零项目知识零命令字面，命令串、路径、
//! 映射、语法参数全部在判定包数据；{ROOT}/{PACK_DIR} 占位展开是引擎通用能力。
//! 四查操作封闭词汇表：double_run、baseline_freeze、red_then_green、scenario_coverage；
//! 三态失败定位 missing、violation、broken_ref。
//!
//! CLI：`acceptor --pack <判定包.json> [--root <工作区根>]`
//! 退出码三值：0 全过、1 有查失败、2 工具自身异常（包非法、词汇表外、命令不可执行）。
//! 报告 JSON 落 stdout（sort_keys 缩进一），错误 JSON 落 stderr。硬失败不静默。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. 报告 engine_version 按 engine.py 硬编码 "0.1.0" 对表（包 __version__ 0.2.0
//!    不同步是围堰事实，移植按报告面原样保留）。
//! 2. checks 非数组、条目非对象、declaration 非对象、命令 token 数组为空、
//!    expect 捕获非整数、冻结件读盘失败等形状，围堰为未捕 traceback（退出码 1），
//!    移植统一按 PackError / ExecError 退出码 2 硬失败不静默。
//! 3. 正则引擎为 Rust regex：Python 独有的环视等构造编译失败按 PackError 退出码 2。
//! 4. 报告缩进形按 Python json.dumps(indent=1, sort_keys=True) 自写序列化对表；
//!    浮点最短表示边缘形未逐字对齐。
//! 5. 子进程超时 600 秒对表；被检命令被信号杀死（returncode None）按 -1 记
//!    （Python 记 -N）；spawn 非 NotFound 错误（如权限拒绝）按 ExecError 退出码 2
//!    （围堰仅捕 FileNotFoundError）。
//! 6. 非字符串命令 token 按 %s 形字符串化执行（围堰 subprocess 对非 str 抛
//!    TypeError traceback 退出码 1）。

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::time::{Duration, Instant};

/// 报告面版本戳：engine.py 硬编码 "0.1.0" 对表（见头注落差一）。
const ENGINE_VERSION: &str = "0.1.0";
const ROOT_TOKEN: &str = "{ROOT}";
const PACK_TOKEN: &str = "{PACK_DIR}";
const RED_HINT: &str = "红证件缺席或哈希不符即缺件态";
const VOCAB: [&str; 4] = ["double_run", "baseline_freeze", "red_then_green", "scenario_coverage"];
const TIMEOUT_SECS: u64 = 600;

/// 包非法（PackError）与执行环境异常（ExecError）双轨。
enum AErr {
    Pack(String),
    Exec(String),
}

// ---- Python 形 JSON 序列化（json.dumps 对表：sort_keys / separators / indent） ----

fn py_dumps(v: &Value, sort_keys: bool, tight: bool, indent: Option<usize>) -> String {
    let mut out = String::new();
    write_py(v, sort_keys, tight, indent, 0, &mut out);
    out
}

fn write_py(v: &Value, sort_keys: bool, tight: bool, indent: Option<usize>, level: usize, out: &mut String) {
    match v {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            out.push_str(&v.to_string())
        }
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            match indent {
                Some(w) => {
                    let inner = " ".repeat(w * (level + 1));
                    let close = " ".repeat(w * level);
                    out.push_str("[\n");
                    for (i, item) in a.iter().enumerate() {
                        if i > 0 {
                            out.push_str(",\n");
                        }
                        out.push_str(&inner);
                        write_py(item, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push('\n');
                    out.push_str(&close);
                    out.push(']');
                }
                None => {
                    let sep = if tight { "," } else { ", " };
                    out.push('[');
                    for (i, item) in a.iter().enumerate() {
                        if i > 0 {
                            out.push_str(sep);
                        }
                        write_py(item, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push(']');
                }
            }
        }
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
            if sort_keys {
                kvs.sort_by(|a, b| a.0.cmp(b.0));
            }
            match indent {
                Some(w) => {
                    let inner = " ".repeat(w * (level + 1));
                    let close = " ".repeat(w * level);
                    out.push_str("{\n");
                    for (i, (k, val)) in kvs.iter().enumerate() {
                        if i > 0 {
                            out.push_str(",\n");
                        }
                        out.push_str(&inner);
                        out.push_str(&Value::String((*k).clone()).to_string());
                        out.push_str(": ");
                        write_py(val, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push('\n');
                    out.push_str(&close);
                    out.push('}');
                }
                None => {
                    let (isep, ksep) = if tight { (",", ":") } else { (", ", ": ") };
                    out.push('{');
                    for (i, (k, val)) in kvs.iter().enumerate() {
                        if i > 0 {
                            out.push_str(isep);
                        }
                        out.push_str(&Value::String((*k).clone()).to_string());
                        out.push_str(ksep);
                        write_py(val, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push('}');
                }
            }
        }
    }
}

/// Python %s 形字符串化（None/True/False 与数字面对表，容器形浅对表）。
fn py_str(v: &Value) -> String {
    match v {
        Value::Null => "None".to_string(),
        Value::Bool(true) => "True".to_string(),
        Value::Bool(false) => "False".to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn expand_token(t: &str, root: &str, pack_dir: &str) -> String {
    t.replace(ROOT_TOKEN, root).replace(PACK_TOKEN, pack_dir)
}

/// 占位展开路径（_p 对表）。
fn expand_path(v: &Value, root: &Path, pack_dir: &Path) -> PathBuf {
    let root_s = root.to_string_lossy().into_owned();
    let pack_s = pack_dir.to_string_lossy().into_owned();
    PathBuf::from(expand_token(&py_str(v), &root_s, &pack_s))
}

/// cwd 参数（params.get("cwd") 真值判对表：缺席或空串即 None）。
fn opt_cwd(c: &Value, root: &Path, pack_dir: &Path) -> Option<PathBuf> {
    c.get("cwd")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| expand_path(&Value::String(s.to_string()), root, pack_dir))
}

/// 命令 token 数组取形（已过 validate 的 list 校验，逐 token 字符串化）。
fn command_tokens(c: &Value) -> Vec<String> {
    c.get("command")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().map(py_str).collect())
        .unwrap_or_default()
}

/// 被检命令执行（_run 对表：占位展开、cwd 缺省 {ROOT}、stdout 捕获、超时 600s）。
fn run_cmd(
    tokens: &[String],
    root: &Path,
    pack_dir: &Path,
    cwd: Option<&Path>,
) -> Result<(i64, Vec<u8>), AErr> {
    let root_s = root.to_string_lossy().into_owned();
    let pack_s = pack_dir.to_string_lossy().into_owned();
    let cmd: Vec<String> = tokens
        .iter()
        .map(|t| expand_token(t, &root_s, &pack_s))
        .collect();
    if cmd.is_empty() {
        return Err(AErr::Exec("命令不可执行：".to_string()));
    }
    let workdir = cwd.unwrap_or(root);
    let mut command = Command::new(&cmd[0]);
    command
        .args(&cmd[1..])
        .current_dir(workdir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = match command.spawn() {
        Ok(ch) => ch,
        Err(_) => return Err(AErr::Exec(format!("命令不可执行：{}", cmd[0]))),
    };
    let mut out_pipe = child.stdout.take().unwrap();
    let t_out = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = out_pipe.read_to_end(&mut buf);
        buf
    });
    let mut err_pipe = child.stderr.take().unwrap();
    let t_err = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = err_pipe.read_to_end(&mut buf);
        buf
    });
    let deadline = Instant::now() + Duration::from_secs(TIMEOUT_SECS);
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break Some(s),
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    break None;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(_) => return Err(AErr::Exec("命令不可执行".to_string())),
        }
    };
    let out = t_out.join().unwrap_or_default();
    let _ = t_err.join();
    match status {
        None => Err(AErr::Exec("命令超时".to_string())),
        Some(s) => Ok((s.code().unwrap_or(-1) as i64, out)),
    }
}

// ---- 四查操作（engine.py _op_* 逐字对表） ----

fn op_double_run(c: &Value, root: &Path, pack_dir: &Path) -> Result<(String, String), AErr> {
    let tokens = command_tokens(c);
    let cwd = opt_cwd(c, root, pack_dir);
    let (rc1, out1) = run_cmd(&tokens, root, pack_dir, cwd.as_deref())?;
    let (rc2, out2) = run_cmd(&tokens, root, pack_dir, cwd.as_deref())?;
    if rc1 == rc2 && out1 == out2 {
        Ok(("pass".to_string(), String::new()))
    } else {
        Ok((
            "violation".to_string(),
            format!("双跑不一致：退出码 {rc1}/{rc2}，输出等 {}", out1 == out2),
        ))
    }
}

fn op_baseline_freeze(c: &Value, root: &Path, pack_dir: &Path) -> Result<(String, String), AErr> {
    let frozen = expand_path(c.get("frozen").unwrap_or(&Value::Null), root, pack_dir);
    if !frozen.exists() {
        return Ok(("missing".to_string(), "冻结期望件缺席".to_string()));
    }
    let cwd = opt_cwd(c, root, pack_dir);
    let tokens = command_tokens(c);
    let (rc, out) = run_cmd(&tokens, root, pack_dir, cwd.as_deref())?;
    let want = fs::read(&frozen).map_err(|e| AErr::Exec(format!("冻结期望件读取失败：{e}")))?;
    if out == want && rc == 0 {
        return Ok(("pass".to_string(), String::new()));
    }
    let m = out.len().min(want.len());
    let first = (0..m).find(|&i| out[i] != want[i]).unwrap_or(m);
    Ok((
        "violation".to_string(),
        format!(
            "漂移原始事实：退出码 {rc}（期望 0），输出 {} 字节对期望 {} 字节，首差异偏移 {first}；v0 不归因",
            out.len(),
            want.len()
        ),
    ))
}

fn op_red_then_green(c: &Value, root: &Path, pack_dir: &Path) -> Result<(String, String), AErr> {
    let ev = expand_path(c.get("red_evidence").unwrap_or(&Value::Null), root, pack_dir);
    if !ev.exists() {
        return Ok(("missing".to_string(), format!("{RED_HINT}：红证件缺席")));
    }
    let ev_bytes = fs::read(&ev).map_err(|e| AErr::Exec(format!("红证件读取失败：{e}")))?;
    let want_hash = c.get("red_hash").and_then(|v| v.as_str()).unwrap_or("");
    if sha256_hex(&ev_bytes) != want_hash {
        return Ok(("missing".to_string(), format!("{RED_HINT}：哈希不符")));
    }
    let expect = c.get("expect_exit").and_then(|v| v.as_i64()).unwrap_or(0);
    let cwd = opt_cwd(c, root, pack_dir);
    let tokens = command_tokens(c);
    let (rc, _) = run_cmd(&tokens, root, pack_dir, cwd.as_deref())?;
    if rc == expect {
        Ok(("pass".to_string(), String::new()))
    } else {
        Ok(("violation".to_string(), format!("当前不绿：退出码 {rc}（期望 {expect}）")))
    }
}

/// 场景编号提取（engine.py 硬编码正则 ^#### \w+:\s*(\S+-\d+) 对表）。
const SCENARIO_ID_RE: &str = r"(?m)^#### \w+:\s*(\S+-\d+)";

fn op_scenario_coverage(c: &Value, root: &Path, pack_dir: &Path) -> Result<(String, String), AErr> {
    let sc = expand_path(c.get("scenarios").unwrap_or(&Value::Null), root, pack_dir);
    if !sc.exists() {
        return Ok(("missing".to_string(), "场景清单缺席".to_string()));
    }
    let sc_bytes = fs::read(&sc).map_err(|e| AErr::Exec(format!("场景清单读取失败：{e}")))?;
    let text = String::from_utf8(sc_bytes)
        .map_err(|e| AErr::Exec(format!("场景清单非 UTF-8：{e}")))?;
    let prefix = c
        .get("line_prefix")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let prog_re = regex::Regex::new(
        c.get("program_field_regex").and_then(|v| v.as_str()).unwrap_or(""),
    )
    .map_err(|e| AErr::Pack(format!("正则编译失败：program_field_regex：{e}")))?;
    let exp_re = regex::Regex::new(c.get("expect_regex").and_then(|v| v.as_str()).unwrap_or(""))
        .map_err(|e| AErr::Pack(format!("正则编译失败：expect_regex：{e}")))?;
    let judge_map: HashMap<String, Vec<String>> = c
        .get("judge_map")
        .and_then(|v| v.as_object())
        .map(|m| {
            m.iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.as_array()
                            .map(|a| a.iter().map(py_str).collect())
                            .unwrap_or_default(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    let id_re = regex::Regex::new(SCENARIO_ID_RE).unwrap();
    let s_ids: Vec<String> = id_re
        .captures_iter(&text)
        .filter_map(|cc| cc.get(1).map(|m| m.as_str().to_string()))
        .collect();
    if s_ids.is_empty() {
        return Ok(("missing".to_string(), "场景清单无编号条目".to_string()));
    }
    let lines: Vec<&str> = text
        .lines()
        .filter(|ln| ln.trim().starts_with(prefix.as_str()))
        .collect();
    if lines.len() < s_ids.len() {
        return Ok((
            "missing".to_string(),
            format!("判据行缺席：{} 条场景对 {} 行", s_ids.len(), lines.len()),
        ));
    }
    let cwd = opt_cwd(c, root, pack_dir);
    let prefix_chars = prefix.chars().count();
    let mut fails: Vec<String> = vec![];
    for (sid, ln) in s_ids.iter().zip(lines.iter()) {
        // Python 切片按字符计：body = ln.strip()[len(prefix):].strip()。
        let trimmed = ln.trim();
        let body: String = trimmed.chars().skip(prefix_chars).collect::<String>().trim().to_string();
        let (pm, em) = match (prog_re.captures(&body), exp_re.captures(&body)) {
            (Some(p), Some(e)) => (p, e),
            _ => return Ok(("missing".to_string(), format!("判据行要素缺项：{sid}"))),
        };
        let prog = pm.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let cmd_tokens = match judge_map.get(&prog) {
            Some(t) => t.clone(),
            None => return Err(AErr::Pack(format!("judge_map 无程序映射：{prog}"))),
        };
        if cmd_tokens.is_empty() {
            return Err(AErr::Pack(format!("judge_map 程序命令非数组：{prog}")));
        }
        let expect_raw = em.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let expect = expect_raw
            .parse::<i64>()
            .map_err(|_| AErr::Pack(format!("期望值非整数：{expect_raw}")))?;
        let (rc, _) = run_cmd(&cmd_tokens, root, pack_dir, cwd.as_deref())?;
        if rc != expect {
            fails.push(format!("{sid}（得 {rc} 期望 {expect_raw}）"));
        }
    }
    if !fails.is_empty() {
        return Ok(("violation".to_string(), format!("覆盖差集非零：{}", fails.join("、"))));
    }
    Ok(("pass".to_string(), String::new()))
}

// ---- 包校验与求值（engine.py validate_pack / evaluate 逐字对表） ----

fn validate_pack(pack: &Value) -> Result<(), AErr> {
    let obj = match pack.as_object() {
        Some(o) => o,
        None => return Err(AErr::Pack("包非对象".to_string())),
    };
    for key in ["pack", "version", "doc_class", "checks", "declaration"] {
        if !obj.contains_key(key) {
            return Err(AErr::Pack(format!("包缺键：{key}")));
        }
    }
    let checks = match obj["checks"].as_array() {
        Some(a) => a,
        None => return Err(AErr::Pack("checks 非数组".to_string())),
    };
    for c in checks {
        let entry = match c.as_object() {
            Some(e) => e,
            None => return Err(AErr::Pack("检查条目非对象".to_string())),
        };
        let name = entry.get("check_name").unwrap_or(&Value::Null);
        if !name.as_str().map(|s| VOCAB.contains(&s)).unwrap_or(false) {
            return Err(AErr::Pack(format!(
                "词汇表外操作：{}（{}）",
                py_str(name),
                py_str(entry.get("id").unwrap_or(&Value::Null))
            )));
        }
        let name = name.as_str().unwrap_or("");
        if name == "scenario_coverage" {
            if !entry.get("judge_map").map(|v| v.is_object()).unwrap_or(false) {
                return Err(AErr::Pack(format!(
                    "judge_map 缺席或非映射：{}",
                    py_str(entry.get("id").unwrap_or(&Value::Null))
                )));
            }
        } else if !entry.get("command").map(|v| v.is_array()).unwrap_or(false) {
            return Err(AErr::Pack(format!(
                "命令非 token 数组：{}",
                py_str(entry.get("id").unwrap_or(&Value::Null))
            )));
        }
    }
    let claim_ok = obj["declaration"]
        .as_object()
        .and_then(|d| d.get("claim"))
        .and_then(|v| v.as_str())
        == Some("unchanged-only");
    if !claim_ok {
        return Err(AErr::Pack("声明范围 claim 非法（甲乙红线）".to_string()));
    }
    Ok(())
}

fn evaluate(pack: &Value, root: &Path, pack_dir: &Path) -> Result<Value, AErr> {
    validate_pack(pack)?;
    let checks = pack["checks"].as_array().unwrap();
    let mut results: Vec<Value> = vec![];
    let mut findings: Vec<Value> = vec![];
    for c in checks {
        let name = c["check_name"].as_str().unwrap_or("");
        let (state, detail) = match name {
            "double_run" => op_double_run(c, root, pack_dir)?,
            "baseline_freeze" => op_baseline_freeze(c, root, pack_dir)?,
            "red_then_green" => op_red_then_green(c, root, pack_dir)?,
            "scenario_coverage" => op_scenario_coverage(c, root, pack_dir)?,
            _ => unreachable!("validate_pack 已拒词汇表外操作"),
        };
        let id = c.get("id").cloned().unwrap_or(Value::Null);
        results.push(json!({"id": id, "check_name": c["check_name"], "state": state}));
        if state != "pass" {
            let hint = c.get("hint").cloned().unwrap_or(json!(""));
            findings.push(json!({
                "id": id,
                "check_name": c["check_name"],
                "state": state,
                "detail": detail,
                "hint": hint,
            }));
        }
    }
    Ok(json!({
        "engine": "acceptor",
        "engine_version": ENGINE_VERSION,
        "pack": pack["pack"],
        "pack_version": pack["version"],
        "verdict": if findings.is_empty() { "pass" } else { "fail" },
        "declaration": pack["declaration"],
        "results": results,
        "findings": findings,
    }))
}

// ---- 入口 ----

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: acceptor --pack <判定包.json> [--root <工作区根>]");
    exit(2);
}

fn err_stderr(error: &str, detail: &str) {
    eprintln!("{}", py_dumps(&json!({"error": error, "detail": detail}), false, false, None));
}

/// 非严格 resolve（Path.resolve() 对表：canonicalize 优先，失败退绝对化拼接）。
fn resolve_path(p: &Path) -> PathBuf {
    if let Ok(c) = fs::canonicalize(p) {
        return c;
    }
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_default().join(p)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut pack_arg: Option<String> = None;
    let mut root_arg: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].clone();
        if a == "--pack" {
            i += 1;
            if i >= args.len() {
                usage_fail("--pack 缺值");
            }
            pack_arg = Some(args[i].clone());
        } else if let Some(v) = a.strip_prefix("--pack=") {
            pack_arg = Some(v.to_string());
        } else if a == "--root" {
            i += 1;
            if i >= args.len() {
                usage_fail("--root 缺值");
            }
            root_arg = Some(args[i].clone());
        } else if let Some(v) = a.strip_prefix("--root=") {
            root_arg = Some(v.to_string());
        } else if a.starts_with('-') {
            usage_fail(&format!("未知旗标: {a}"));
        } else {
            usage_fail(&format!("多余位置参数: {a}"));
        }
        i += 1;
    }
    let pack_arg = match pack_arg {
        Some(p) => p,
        None => usage_fail("缺 --pack（必填）"),
    };
    let root = match root_arg {
        Some(r) => resolve_path(Path::new(&r)),
        None => std::env::current_dir().unwrap_or_default(),
    };
    let pack_path = PathBuf::from(&pack_arg);
    let pack_text = match fs::read_to_string(&pack_path) {
        Ok(t) => t,
        Err(_) => {
            eprintln!(
                "{}",
                py_dumps(
                    &json!({"error": "判定包缺席", "pack": pack_arg}),
                    false,
                    false,
                    None
                )
            );
            exit(2);
        }
    };
    let pack: Value = match serde_json::from_str(&pack_text) {
        Ok(p) => p,
        Err(e) => {
            err_stderr("判定包 JSON 不可解析", &e.to_string());
            exit(2);
        }
    };
    let pack_dir = resolve_path(&pack_path).parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let report = match evaluate(&pack, &root, &pack_dir) {
        Ok(r) => r,
        Err(AErr::Pack(e)) => {
            err_stderr("判定包非法", &e);
            exit(2);
        }
        Err(AErr::Exec(e)) => {
            err_stderr("执行环境异常", &e);
            exit(2);
        }
    };
    println!("{}", py_dumps(&report, true, false, Some(1)));
    exit(if report["verdict"] == "pass" { 0 } else { 1 });
}
