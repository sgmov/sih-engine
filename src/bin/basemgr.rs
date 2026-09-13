//! 引擎侧基线向量管理工具（basemgr）命令行面 —— lease-mergeleg6-parallel 簇I 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/basemgr 0.1.0（cli.py、engine.py），只读对表移植，
//! 围堰源码零改动。基线向量与规约向量双种共享家位：freeze、replay、refreeze
//! 三操作（登记件冻结形），重放只读零写入；冻结与重冻落笔仅限显式指名向量件。
//! 零项目知识零命令字面：命令串、路径、映射全部在调用参数与向量件数据。
//!
//! CLI：
//! `basemgr freeze  --vector <件.json> --command <token...>|--command-json <JSON>
//!                  --kind baseline|spec --program-version <串> --pack-version <串>
//!                  [--impact-set a,b] [--target-hash <sha>] [--cwd <目录>] [--root <根>]`
//! `basemgr replay  --vector <件.json> --command ... [--cwd] [--root]`
//! `basemgr refreeze --vector <件.json> --command ... [--red-evidence <件>]
//!                  [--six-now <JSON>] [--cwd] [--root]`
//! 退出码三值：0 = 过或重录完成、1 = 漂移或 refuse、2 = 工具自身异常。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. 环境指纹白名单检查：围堰 engine.py 未 import os，白名单非空时 os.environ
//!    即 NameError 崩溃（traceback 退出码一）；移植按意图承载（键缺席环境即
//!    指纹前置拦截退出码二），白名单空集（v0 起步形）行为逐字节同构。
//! 2. 向量件 / index.json / --six-now JSON 不可解析：围堰 JSONDecodeError 未接
//!    （traceback 退出码一），移植按向量件非法／执行环境异常退出码二。
//! 3. --root 缺席：围堰 fingerprint 与 run 的 cwd 取 str(None)="None" 崩溃或
//!    TypeError；移植按空根即当前目录对表（指纹比对自然失配拦），语义见
//!    前置探针；测试与现役调用均显式传 --root。
//! 4. 命令超时（围堰 subprocess timeout=600）未实装，挂起形为无限等待。
//! 5. --command 变参在 argparse nargs="*" 语义下遇 "-" 起始 token 即截断，
//!    移植同形（含 "-" 起始 token 的命令须经 --command-json 承载）。
//! 6. 报告 stdout 为 Python json.dumps(sort_keys=True, indent=1) 同构形
//!    （自足序列化器逐字节对齐）；stderr 错误为插入序单行形。

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

const ROOT_TOKEN: &str = "{ROOT}";
const KINDS: [&str; 2] = ["baseline", "spec"];
const SIX_OBJECT_KEYS: [&str; 6] = [
    "program",
    "normalizer",
    "mutations",
    "classes",
    "representative",
    "probes",
];

/// 引擎错误：向量件非法 / 执行环境异常 / 指纹前置拦截，均退出码二。
enum BErr {
    Vector(String),
    Exec(String),
    Fingerprint(String),
}

// ---------- Python json.dumps 同构序列化 ----------

fn json_quote(s: &str) -> String {
    serde_json::to_string(s).unwrap()
}

fn py_json(v: &Value, indent: Option<usize>, sort: bool) -> String {
    let mut out = String::new();
    py_rec(v, indent, sort, 0, &mut out);
    out
}

fn py_rec(v: &Value, indent: Option<usize>, sort: bool, depth: usize, out: &mut String) {
    match v {
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
            if sort {
                kvs.sort_by(|a, b| a.0.cmp(b.0));
            }
            out.push('{');
            for (i, (k, val)) in kvs.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                if let Some(n) = indent {
                    out.push('\n');
                    out.push_str(&" ".repeat((depth + 1) * n));
                }
                out.push_str(&json_quote(k));
                out.push_str(": ");
                py_rec(val, indent, sort, depth + 1, out);
            }
            if let Some(n) = indent {
                out.push('\n');
                out.push_str(&" ".repeat(depth * n));
            }
            out.push('}');
        }
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, val) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                if let Some(n) = indent {
                    out.push('\n');
                    out.push_str(&" ".repeat((depth + 1) * n));
                }
                py_rec(val, indent, sort, depth + 1, out);
            }
            if let Some(n) = indent {
                out.push('\n');
                out.push_str(&" ".repeat(depth * n));
            }
            out.push(']');
        }
        Value::String(s) => out.push_str(&json_quote(s)),
        Value::Number(n) => out.push_str(&n.to_string()),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Null => out.push_str("null"),
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

/// Python Path.resolve() 非严格形：canonicalize 优先，失败退绝对化拼接。
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

fn py_list_repr(items: &[String]) -> String {
    let inner: Vec<String> = items.iter().map(|s| format!("'{s}'")).collect();
    format!("[{}]", inner.join(", "))
}

fn first_diff(out: &[u8], want: &[u8]) -> usize {
    let m = out.len().min(want.len());
    for i in 0..m {
        if out[i] != want[i] {
            return i;
        }
    }
    m
}

// ---------- 引擎（engine.py 对表） ----------

fn expand(tokens: &[String], root: Option<&str>) -> Vec<String> {
    tokens
        .iter()
        .map(|t| match root {
            Some(r) => t.replace(ROOT_TOKEN, r),
            None => t.clone(),
        })
        .collect()
}

/// 归一命令串：仅工作区根前缀替换为 {ROOT} 占位（vecfix-solo 对表）。
fn normalize_command(command: &[String], root: Option<&str>) -> Vec<String> {
    match root {
        None => command.to_vec(),
        Some(r) => {
            let root_str = resolve_abs(Path::new(r));
            command
                .iter()
                .map(|t| {
                    if t.starts_with(&root_str) {
                        format!("{ROOT_TOKEN}{}", &t[root_str.len()..])
                    } else {
                        t.clone()
                    }
                })
                .collect()
        }
    }
}

fn normalize_cwd(cwd: Option<&str>, root: Option<&str>) -> Option<String> {
    match (cwd, root) {
        (Some(c), Some(r)) => {
            let root_str = resolve_abs(Path::new(r));
            if c.starts_with(&root_str) {
                Some(format!("{ROOT_TOKEN}{}", &c[root_str.len()..]))
            } else {
                Some(c.to_string())
            }
        }
        (Some(c), None) => Some(c.to_string()),
        _ => None,
    }
}

fn run_cmd(
    cmd_tokens: &[String],
    root: Option<&str>,
    cwd: Option<&str>,
) -> Result<(i32, Vec<u8>), BErr> {
    let cmd = expand(cmd_tokens, root);
    if cmd.is_empty() {
        return Err(BErr::Exec("命令不可执行：".to_string()));
    }
    let workdir = match cwd {
        Some(c) => expand(&[c.to_string()], root)[0].clone(),
        None => root.unwrap_or("None").to_string(),
    };
    let mut c = Command::new(&cmd[0]);
    c.args(&cmd[1..]).current_dir(&workdir);
    let out = c
        .output()
        .map_err(|_| BErr::Exec(format!("命令不可执行：{}", cmd[0])))?;
    Ok((out.status.code().unwrap_or(-1), out.stdout))
}

fn load_vector(path: &str) -> Result<(Value, PathBuf), BErr> {
    let p = PathBuf::from(path);
    if !p.exists() {
        return Err(BErr::Vector(format!("向量件缺席：{}", p.display())));
    }
    let text = fs::read_to_string(&p)
        .map_err(|e| BErr::Vector(format!("向量件不可读：{e}")))?;
    let d: Value = serde_json::from_str(&text)
        .map_err(|e| BErr::Vector(format!("向量件 JSON 不可解析：{e}")))?;
    for key in ["vector", "kind", "quadruple", "env_fingerprint", "version_triple", "six_objects"] {
        if d.get(key).is_none() {
            return Err(BErr::Vector(format!("向量件缺键：{key}")));
        }
    }
    let kind = d["kind"].as_str().unwrap_or("");
    if !KINDS.contains(&kind) {
        return Err(BErr::Vector(format!("种非法：{kind}")));
    }
    Ok((d, resolve_abs(&p).into()))
}

fn fingerprint_now(root: Option<&str>) -> String {
    resolve_abs(Path::new(root.unwrap_or("")))
}

fn check_fingerprint(meta: &Value, root: Option<&str>) -> Result<(), BErr> {
    let fp = &meta["env_fingerprint"];
    let now = fingerprint_now(root);
    let frozen = fp.get("cwd_root").and_then(|v| v.as_str()).unwrap_or("");
    if frozen != now {
        return Err(BErr::Fingerprint(format!(
            "指纹前置拦截：冻结 cwd {frozen} 对当前 {now} 不符"
        )));
    }
    // 白名单检查按意图承载（头注落差一：围堰 os 未 import，非空即崩溃）。
    if let Some(wl) = fp.get("env_whitelist").and_then(|v| v.as_object()) {
        for k in wl.keys() {
            if std::env::var_os(k).is_none() {
                return Err(BErr::Fingerprint(format!(
                    "指纹前置拦截：白名单环境变量缺席 {k}"
                )));
            }
        }
    }
    Ok(())
}

fn expected_path(vp: &Path, meta: &Value) -> PathBuf {
    let name = meta["quadruple"]
        .get("expected_file")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    vp.parent().unwrap_or(Path::new(".")).join(name)
}

fn write_meta(vp: &Path, meta: &Value) -> Result<(), BErr> {
    fs::write(vp, py_json(meta, Some(1), true) + "\n")
        .map_err(|e| BErr::Exec(format!("向量件不可写：{e}")))
}

fn update_index(vp: &Path, meta: &Value) -> Result<(), BErr> {
    let index = vp.parent().unwrap_or(Path::new(".")).join("index.json");
    let mut idx: Value = if index.exists() {
        let text = fs::read_to_string(&index)
            .map_err(|e| BErr::Vector(format!("index 不可读：{e}")))?;
        serde_json::from_str(&text)
            .map_err(|e| BErr::Vector(format!("index JSON 不可解析：{e}")))?
    } else {
        json!({"vectors": []})
    };
    let stem = meta["vector"].as_str().unwrap_or("");
    let mut vectors: Vec<Value> = idx["vectors"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|v| v.get("vector").and_then(|x| x.as_str()) != Some(stem))
        .collect();
    vectors.push(json!({
        "vector": stem,
        "kind": meta["kind"],
        "expected_sha256": meta["expected_sha256"],
    }));
    vectors.sort_by(|a, b| {
        a.get("vector")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .cmp(b.get("vector").and_then(|v| v.as_str()).unwrap_or(""))
    });
    idx["vectors"] = Value::Array(vectors);
    fs::write(&index, py_json(&idx, Some(1), true) + "\n")
        .map_err(|e| BErr::Exec(format!("index 不可写：{e}")))
}

fn bump_index_sha(vp: &Path, meta: &Value) -> Result<(), BErr> {
    let index = vp.parent().unwrap_or(Path::new(".")).join("index.json");
    if !index.exists() {
        return Ok(());
    }
    let text = fs::read_to_string(&index)
        .map_err(|e| BErr::Vector(format!("index 不可读：{e}")))?;
    let mut idx: Value = serde_json::from_str(&text)
        .map_err(|e| BErr::Vector(format!("index JSON 不可解析：{e}")))?;
    let stem = meta["vector"].as_str().unwrap_or("");
    if let Some(vectors) = idx["vectors"].as_array_mut() {
        for v in vectors.iter_mut() {
            if v.get("vector").and_then(|x| x.as_str()) == Some(stem) {
                v["expected_sha256"] = meta["expected_sha256"].clone();
            }
        }
    }
    fs::write(&index, py_json(&idx, Some(1), true) + "\n")
        .map_err(|e| BErr::Exec(format!("index 不可写：{e}")))
}

fn op_freeze(
    vector_path: &str,
    command: &[String],
    kind: &str,
    program_version: &str,
    pack_version: &str,
    impact_set: Option<Vec<String>>,
    target_hash: Option<&str>,
    root: Option<&str>,
    cwd: Option<&str>,
) -> Result<Value, BErr> {
    if !KINDS.contains(&kind) {
        return Err(BErr::Vector(format!("种非法：{kind}")));
    }
    let vp = PathBuf::from(resolve_abs(Path::new(vector_path)));
    if let Some(parent) = vp.parent() {
        fs::create_dir_all(parent).map_err(|e| BErr::Exec(format!("向量家位不可建：{e}")))?;
    }
    let (rc, out) = run_cmd(command, root, cwd)?;
    if kind == "spec" {
        if !(rc == 0 || rc == 1) || out.is_empty() {
            return Err(BErr::Exec(format!(
                "冻结拒收：规约种红输出允许（exit 1），其余拒绝——退出码 {rc}、输出 {} 字节",
                out.len()
            )));
        }
    } else if rc != 0 || out.is_empty() {
        return Err(BErr::Exec(format!(
            "冻结拒收：被检命令退出码 {rc}、输出 {} 字节——期望输出必须来自成功运行",
            out.len()
        )));
    }
    let stem = vp.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
    let expected = vp
        .parent()
        .unwrap_or(Path::new("."))
        .join(format!("{stem}.bin"));
    fs::write(&expected, &out).map_err(|e| BErr::Exec(format!("期望输出件不可写：{e}")))?;
    let norm_command = normalize_command(command, root);
    let norm_cwd = normalize_cwd(cwd, root);
    let sha = sha256_hex(&out);
    let cwd_root = fingerprint_now(root);
    let meta = json!({
        "vector": stem,
        "kind": kind,
        "vector_version": "0.1.0",
        "created": chrono::Local::now().format("%Y-%m-%d").to_string(),
        "command": norm_command,
        "quadruple": {
            "input_desc": norm_command.join("|"),
            "context_desc": format!(
                "cwd={} env_whitelist={{}} clock=declared-fixed",
                norm_cwd.clone().unwrap_or_else(|| "None".to_string())
            ),
            "pathing": "cli-cwd-relative",
            "command_cwd": norm_cwd,
            "expected_file": expected.file_name().and_then(|s| s.to_str()).unwrap_or(""),
        },
        "env_fingerprint": {
            "cwd_root": cwd_root,
            "path_resolution": "cli-cwd",
            "env_whitelist": {},
            "clock": "declared-fixed",
            "note": "T11 不完备声明：白名单外环境变量不在保证域",
        },
        "version_triple": {
            "program": program_version,
            "pack": pack_version,
            "target": target_hash.map(|s| s.to_string()).unwrap_or_else(|| sha.clone()),
        },
        "six_objects": {
            "program": Value::Null,
            "normalizer": Value::Null,
            "mutations": Value::Null,
            "classes": Value::Null,
            "representative": Value::Null,
            "probes": Value::Null,
        },
        "impact_set": impact_set.map(|v| json!(v)).unwrap_or(Value::Null),
        "expected_sha256": sha,
        "declaration": {"claim": "unchanged-only"},
    });
    write_meta(&vp, &meta)?;
    update_index(&vp, &meta)?;
    Ok(json!({
        "op": "freeze",
        "vector": stem,
        "expected_sha256": meta["expected_sha256"],
        "exit_code": rc,
        "verdict": "pass",
        "declaration": meta["declaration"],
    }))
}

fn replay_cwd(meta: &Value, cwd: Option<&str>) -> Option<String> {
    cwd.map(|s| s.to_string()).or_else(|| {
        meta["quadruple"]
            .get("command_cwd")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    })
}

fn op_replay(
    vector_path: &str,
    command: &[String],
    root: Option<&str>,
    cwd: Option<&str>,
) -> Result<Value, BErr> {
    let (meta, vp) = load_vector(vector_path)?;
    check_fingerprint(&meta, root)?;
    let cwd = replay_cwd(&meta, cwd);
    let expected = expected_path(&vp, &meta);
    if !expected.exists() {
        return Err(BErr::Vector(format!("期望输出件缺席：{}", expected.display())));
    }
    let (rc, out) = run_cmd(command, root, cwd.as_deref())?;
    let want = fs::read(&expected).unwrap_or_default();
    if out == want && rc == 0 {
        return Ok(json!({
            "op": "replay",
            "vector": meta["vector"],
            "kind": meta["kind"],
            "verdict": "pass",
            "claim": "unchanged-only",
            "exit_code": rc,
        }));
    }
    let first = first_diff(&out, &want);
    Ok(json!({
        "op": "replay",
        "vector": meta["vector"],
        "kind": meta["kind"],
        "verdict": "fail",
        "claim": "unchanged-only",
        "drift_fact": format!(
            "退出码 {rc}（期望 0），输出 {} 字节对期望 {} 字节，首差异偏移 {first}",
            out.len(),
            want.len()
        ),
    }))
}

fn op_refreeze(
    vector_path: &str,
    command: &[String],
    red_evidence: Option<&str>,
    six_now: Option<&str>,
    root: Option<&str>,
    cwd: Option<&str>,
) -> Result<Value, BErr> {
    let (mut meta, vp) = load_vector(vector_path)?;
    check_fingerprint(&meta, root)?;
    let cwd = replay_cwd(&meta, cwd);
    if let Some(six) = six_now {
        let now: Value = serde_json::from_str(six)
            .map_err(|e| BErr::Exec(format!("six-now JSON 不可解析：{e}")))?;
        let mut changed: Vec<String> = vec![];
        if let (Some(now_obj), Some(six_obj)) = (now.as_object(), meta["six_objects"].as_object()) {
            for k in SIX_OBJECT_KEYS {
                if let Some(v) = now_obj.get(k) {
                    if v != six_obj.get(k).unwrap_or(&Value::Null) {
                        changed.push(k.to_string());
                    }
                }
            }
        }
        if !changed.is_empty() {
            return Ok(json!({
                "op": "refreeze",
                "vector": meta["vector"],
                "verdict": "refuse",
                "reason": format!("六冻结对象变更零豁免：{}——须全量重算", changed.join("、")),
            }));
        }
    }
    let expected = expected_path(&vp, &meta);
    let (rc, out) = run_cmd(command, root, cwd.as_deref())?;
    let want = if expected.exists() {
        fs::read(&expected).unwrap_or_default()
    } else {
        vec![]
    };
    if out == want && rc == 0 {
        return Ok(json!({
            "op": "refreeze",
            "vector": meta["vector"],
            "verdict": "pass",
            "note": "零漂移无需重冻",
        }));
    }
    let kind = meta["kind"].as_str().unwrap_or("");
    let impact_declared = meta
        .get("impact_set")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    if kind == "spec" {
        let red_ok = red_evidence
            .map(|p| Path::new(p).exists())
            .unwrap_or(false);
        if !red_ok {
            return Ok(json!({
                "op": "refreeze",
                "vector": meta["vector"],
                "verdict": "refuse",
                "reason": "规约种先红前置：新鲜红证缺席（缺件态）",
            }));
        }
    } else if !impact_declared {
        return Ok(json!({
            "op": "refreeze",
            "vector": meta["vector"],
            "verdict": "refuse",
            "reason": "未分类漂移：无申报影响集，refuse 重冻（v0 不归因）",
        }));
    }
    let first = first_diff(&out, &want);
    fs::write(&expected, &out).map_err(|e| BErr::Exec(format!("期望输出件不可写：{e}")))?;
    let sha = sha256_hex(&out);
    meta["expected_sha256"] = json!(sha);
    meta["quadruple"]["input_desc"] = json!(command.join("|"));
    write_meta(&vp, &meta)?;
    bump_index_sha(&vp, &meta)?;
    let impact_list: Vec<String> = meta
        .get("impact_set")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .map(|x| x.as_str().unwrap_or("").to_string())
                .collect()
        })
        .unwrap_or_default();
    Ok(json!({
        "op": "refreeze",
        "vector": meta["vector"],
        "verdict": "refroze",
        "attribution": format!("纯期望过期（申报影响集 {}）", py_list_repr(&impact_list)),
        "drift_fact": format!("退出码 {rc}，首差异偏移 {first}"),
        "new_sha256": sha,
    }))
}

// ---------- CLI ----------

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!(
        "用法: basemgr <freeze|replay|refreeze> --vector <件.json> --command <token...>|--command-json <JSON> [旗标...]"
    );
    exit(2);
}

fn fail2(err: &BErr) -> ! {
    let payload = match err {
        BErr::Vector(d) => json!({"error": "向量件非法", "detail": d}),
        BErr::Exec(d) => json!({"error": "执行环境异常", "detail": d}),
        BErr::Fingerprint(d) => json!({"error": "指纹前置拦截", "detail": d}),
    };
    eprintln!("{}", py_json(&payload, None, false));
    exit(2)
}

struct Args {
    op: String,
    vector: Option<String>,
    command: Option<Vec<String>>,
    command_json: Option<String>,
    cwd: Option<String>,
    root: Option<String>,
    kind: Option<String>,
    program_version: Option<String>,
    pack_version: Option<String>,
    impact_set: Option<String>,
    target_hash: Option<String>,
    red_evidence: Option<String>,
    six_now: Option<String>,
}

fn parse_args() -> Args {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut it = raw.iter().peekable();
    let mut a = Args {
        op: String::new(),
        vector: None,
        command: None,
        command_json: None,
        cwd: None,
        root: None,
        kind: None,
        program_version: None,
        pack_version: None,
        impact_set: None,
        target_hash: None,
        red_evidence: None,
        six_now: None,
    };
    // 子命令与顶层 --root（argparse 顶层旗标在子命令前的对表）。
    loop {
        match it.peek().map(|s| s.as_str()) {
            Some("--root") => {
                it.next();
                match it.next() {
                    Some(v) => a.root = Some(v.clone()),
                    None => usage_fail("--root 缺值"),
                }
            }
            Some(s) if s.starts_with("--") => usage_fail(&format!("未知顶层旗标: {s}")),
            Some(s) if s == "freeze" || s == "replay" || s == "refreeze" => {
                a.op = s.to_string();
                it.next();
                break;
            }
            _ => usage_fail("缺子命令（freeze|replay|refreeze）"),
        }
    }
    // 子命令旗标域（argparse 子解析器同形：越域旗标即用法错退出码二）。
    let allowed: &[&str] = match a.op.as_str() {
        "freeze" => &[
            "vector",
            "command",
            "command-json",
            "cwd",
            "root",
            "kind",
            "program-version",
            "pack-version",
            "impact-set",
            "target-hash",
        ],
        "replay" => &["vector", "command", "command-json", "cwd", "root"],
        "refreeze" => &["vector", "command", "command-json", "cwd", "root", "red-evidence", "six-now"],
        other => usage_fail(&format!("未知子命令: {other}")),
    };
    while let Some(arg) = it.next() {
        let name = arg.strip_prefix("--").unwrap_or_else(|| usage_fail(&format!("未知位置参数: {arg}")));
        if !allowed.contains(&name) {
            usage_fail(&format!("子命令 {} 不接受 --{name}", a.op));
        }
        let value = match it.next() {
            Some(v) => v.clone(),
            None => usage_fail(&format!("--{name} 缺值")),
        };
        match name {
            "vector" => a.vector = Some(value),
            "command-json" => a.command_json = Some(value),
            "cwd" => a.cwd = Some(value),
            "root" => a.root = Some(value),
            "kind" => a.kind = Some(value),
            "program-version" => a.program_version = Some(value),
            "pack-version" => a.pack_version = Some(value),
            "impact-set" => a.impact_set = Some(value),
            "target-hash" => a.target_hash = Some(value),
            "red-evidence" => a.red_evidence = Some(value),
            "six-now" => a.six_now = Some(value),
            "command" => {
                // argparse nargs="*" 同形：遇 "-" 起始 token 即截断（头注落差五）。
                let mut tokens = vec![value];
                while let Some(nxt) = it.peek() {
                    if nxt.starts_with('-') {
                        break;
                    }
                    tokens.push(nxt.to_string());
                    it.next();
                }
                a.command = Some(tokens);
            }
            other => usage_fail(&format!("未知旗标: --{other}")),
        }
    }
    a
}

fn run() -> i32 {
    let mut a = parse_args();
    if a.op.is_empty() {
        usage_fail("缺子命令");
    }
    if a.command_json.is_some() {
        let text = a.command_json.clone().unwrap();
        match serde_json::from_str::<Vec<String>>(&text) {
            Ok(v) => a.command = Some(v),
            Err(e) => {
                eprintln!(
                    "{}",
                    py_json(&json!({"error": "command-json 不可解析", "detail": e.to_string()}), None, false)
                );
                return 2;
            }
        }
    }
    let command = match a.command.clone() {
        Some(c) if !c.is_empty() => c,
        _ => {
            eprintln!("用法错: --command 或 --command-json 必填");
            return 2;
        }
    };
    let vector = match a.vector.clone() {
        Some(v) => v,
        None => usage_fail("--vector 必填"),
    };
    let root = a.root.as_deref();
    let cwd = a.cwd.as_deref();

    let report = match a.op.as_str() {
        "freeze" => {
            let kind = match a.kind.clone() {
                Some(k) => k,
                None => usage_fail("freeze 缺 --kind（必填）"),
            };
            let pv = match a.program_version.clone() {
                Some(k) => k,
                None => usage_fail("freeze 缺 --program-version（必填）"),
            };
            let packv = match a.pack_version.clone() {
                Some(k) => k,
                None => usage_fail("freeze 缺 --pack-version（必填）"),
            };
            let impact: Option<Vec<String>> = a
                .impact_set
                .as_deref()
                .map(|s| {
                    let items: Vec<String> = s
                        .split(',')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.to_string())
                        .collect();
                    items
                })
                .and_then(|v| if v.is_empty() { None } else { Some(v) });
            op_freeze(
                &vector,
                &command,
                &kind,
                &pv,
                &packv,
                impact,
                a.target_hash.as_deref(),
                root,
                cwd,
            )
        }
        "replay" => op_replay(&vector, &command, root, cwd),
        "refreeze" => op_refreeze(
            &vector,
            &command,
            a.red_evidence.as_deref(),
            a.six_now.as_deref(),
            root,
            cwd,
        ),
        other => usage_fail(&format!("未知子命令: {other}")),
    };

    match report {
        Ok(rep) => {
            println!("{}", py_json(&rep, Some(1), true));
            let refused = rep.get("verdict") == Some(&json!("refuse"))
                || rep.get("verdict") == Some(&json!("fail"));
            if refused {
                1
            } else {
                0
            }
        }
        Err(e) => fail2(&e),
    }
}

fn main() {
    exit(run());
}
