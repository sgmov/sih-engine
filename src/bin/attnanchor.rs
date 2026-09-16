//! 回锚引擎件（attnanchor）：五行锚读数组装器，注入式回锚 v1 的引擎 bin 形。
//! 移植源：sih-tools/attnanchor/anchor.py（lease-mergeall-parallel 簇A施工）。
//! 行为链：任务锚（.session-anchor.md 首行）→ 在飞（sessions/locks 两账本回算）
//! → 泊界（selector parking 包路由引擎线与工具线）→ 链（当日 trail 末笔读数
//! + 中央登记册 active 域近 7 日覆盖核对告警）→ 纪律令。出参严格 JSON 单键
//! additionalContext；退出码恒零即注入链路永不阻断会话；失效降级可见不静默。
//!
//! 落差申报（对表围堰 anchor.py，零粉饰）：
//! 1. 根判别：围堰自脚本位逐级上溯；引擎 bin 无脚本位，改自 cwd 逐级上溯找
//!    sih-tools/lease/ledger/sessions.ndjson，env（ZCODE_PROJECT_DIR／
//!    CLAUDE_PROJECT_DIR）次之，末位回退 cwd（围堰回退脚本上溯位）。
//! 2. 新增 --root 与 --at 显式参（围堰无参形）：--root 供测试缝与显式覆盖；
//!    --at 定链面日与覆盖窗（缺省仍取实日，读钟例外条款不变，critsweep --at 同先例）。
//! 3. selector 路由子进程超时以 try_wait 轮询十秒实现，等价 subprocess timeout=10；
//!    自 gap-parking-route-engine-selector 起泊界路由腿直调引擎 selector bin
//!    （本 bin 同目录兄弟位 + 引擎仓 packs/selector/parking），消对围堰
//!    uv run 的运行时依赖（iso-04 locks_read 直调先例同形）；materials 相对形
//!    归一绝对后传参，语义同围堰 cwd 锚定形。
//! 4. 账本与 trail 非 UTF-8 读失败：围堰个别裸崩位（except 只接 OSError），
//!    移植一律降级行如实。
//! 5. 出参 pretty 化（围堰 compact 单行）：JSON 语义等价，注入面解析不受影响。
//! 6. 域覆盖核对依赖 git 在 PATH（git log --pretty=%cs），与围堰同。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。

use serde_json::{json, Map, Value};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const DISCIPLINE: &str =
    "[令] 新主题先三岔：属当下吸收／成形入队（泊界或待办）／未成形缓议；越界读须一句申报";
/// 域覆盖核对窗口：近 7 日，确定性常数（围堰 COVERAGE_WINDOW_DAYS）
const COVERAGE_WINDOW_DAYS: i64 = 7;
const ROUTE_TIMEOUT_S: u64 = 10;

fn sort_json(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let bt: std::collections::BTreeMap<&String, &Value> = m.iter().collect();
            Value::Object(
                bt.into_iter()
                    .map(|(k, val)| (k.clone(), sort_json(val)))
                    .collect(),
            )
        }
        Value::Array(a) => Value::Array(a.iter().map(sort_json).collect()),
        other => other.clone(),
    }
}

fn emit(v: &Value) -> String {
    let mut s = serde_json::to_string_pretty(&sort_json(v)).unwrap();
    s.push('\n');
    s
}

fn today() -> chrono::NaiveDate {
    chrono::Local::now().date_naive()
}

fn parse_at(s: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
}

fn resolve_root(explicit: Option<&str>) -> PathBuf {
    if let Some(r) = explicit {
        if !r.is_empty() {
            return PathBuf::from(r);
        }
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut cur = Some(cwd.as_path());
    while let Some(c) = cur {
        if c.join("sih-tools/lease/ledger/sessions.ndjson").is_file() {
            return c.to_path_buf();
        }
        cur = c.parent();
    }
    for k in ["ZCODE_PROJECT_DIR", "CLAUDE_PROJECT_DIR"] {
        if let Ok(v) = std::env::var(k) {
            if !v.is_empty() {
                return PathBuf::from(v);
            }
        }
    }
    cwd
}

fn anchor_line(base: &Path) -> String {
    let f = base.join(".session-anchor.md");
    match std::fs::read_to_string(&f) {
        Ok(text) => match text.lines().next() {
            Some(first) => {
                let t = first.trim();
                if !t.is_empty() {
                    format!("[锚] {}", t)
                } else {
                    "[锚] 任务锚文件首行为空，先立锚再动工".to_string()
                }
            }
            None => "[锚] 缺场告警：无任务锚文件 .session-anchor.md，先立锚再动工".to_string(),
        },
        Err(_) => "[锚] 缺场告警：无任务锚文件 .session-anchor.md，先立锚再动工".to_string(),
    }
}

/// JSON 值转 Python str() 风字符串（路径与事件字段主形态是字符串）
fn py_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "None".to_string(),
        Value::Bool(b) => {
            if *b {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        other => other.to_string(),
    }
}

fn inflight_line(base: &Path) -> String {
    let sessions_path = base.join("sih-tools/lease/ledger/sessions.ndjson");
    let locks_path = base.join("sih-tools/lease/ledger/locks.ndjson");
    let run = || -> Result<(usize, usize), ()> {
        let mut state: std::collections::BTreeMap<String, Value> = Default::default();
        let text = std::fs::read_to_string(&sessions_path).map_err(|_| ())?;
        for line in text.lines() {
            let e: Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if let Some(sid) = e.get("session_id").and_then(|v| v.as_str()) {
                let ev = e.get("event").cloned().unwrap_or(Value::Null);
                state.insert(sid.to_string(), ev);
            }
        }
        let active = state
            .values()
            .filter(|ev| {
                let s = ev.as_str().unwrap_or("");
                s != "closed" && s != "revoked"
            })
            .count();
        let mut lock_state: std::collections::BTreeMap<String, Value> = Default::default();
        let text = std::fs::read_to_string(&locks_path).map_err(|_| ())?;
        for line in text.lines() {
            let e: Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if let Some(sid) = e.get("session_id").and_then(|v| v.as_str()) {
                let p = match e.get("path") {
                    Some(v) => py_str(v),
                    None => "None".to_string(),
                };
                let ev = e.get("event").cloned().unwrap_or(Value::Null);
                lock_state.insert(format!("{}|{}", sid, p), ev);
            }
        }
        let held = lock_state
            .values()
            .filter(|ev| ev.as_str() == Some("acquired"))
            .count();
        Ok((active, held))
    };
    match run() {
        Ok((active, held)) => format!("[在飞] 会话 {} 在册 持锁 {} 条", active, held),
        Err(_) => "[在飞] 账本不可用（降级）".to_string(),
    }
}

enum ProcErr {
    Timeout,
    Fail,
}

struct ProcOut {
    _code: Option<i32>,
    stdout: String,
    _stderr: String,
}

fn run_with_timeout(mut cmd: Command, timeout: Duration) -> Result<ProcOut, ProcErr> {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => return Err(ProcErr::Fail),
    };
    let deadline = Instant::now() + timeout;
    let code;
    loop {
        match child.try_wait() {
            Ok(Some(st)) => {
                code = st.code();
                break;
            }
            Ok(None) => {}
            Err(_) => return Err(ProcErr::Fail),
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(ProcErr::Timeout);
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    let mut so = String::new();
    if let Some(mut h) = child.stdout.take() {
        let _ = h.read_to_string(&mut so);
    }
    let mut se = String::new();
    if let Some(mut h) = child.stderr.take() {
        let _ = h.read_to_string(&mut se);
    }
    Ok(ProcOut {
        _code: code,
        stdout: so,
        _stderr: se,
    })
}

/// 引擎 selector 二进制位（gap-parking-route-engine-selector）：本 bin 同目录
/// 兄弟位（cargo 同树 bins 同居 target/debug 或 target/release），缺位回落
/// PATH 裸名，spawn 失败即降级如实（iso-04 locks_read 直调先例同形，
/// mcpserver runtime lease_bin 兜底精神）。
fn selector_bin() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sib = dir.join("selector");
            if sib.is_file() {
                return sib;
            }
        }
    }
    PathBuf::from("selector")
}

/// 泊界包位：exe 派生引擎仓根 packs/selector/parking（gap-packs-assets 落位，
/// 与围堰 sih-tools/selector/packs/parking 内容同基），绝对形存在即用；
/// 缺位回落相对形 "parking" 交 selector resolve_pack_input 引擎位候选序自解析，
/// 全不中即 selector 既有 pack directory missing 路径如实报错（exit 2 降级可见）。
fn parking_pack_arg() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(root) = exe
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
        {
            let p = root.join("packs").join("selector").join("parking");
            if p.is_dir() {
                return p.display().to_string();
            }
        }
    }
    "parking".to_string()
}

fn route_one(base: &Path, materials: &str, at: chrono::NaiveDate) -> Result<Value, ()> {
    // 泛在与围堰同参：materials 相对形原以 <base>/sih-tools 为 cwd 基准，
    // 归一绝对后传引擎 selector bin，子进程 cwd 不再锚定 sih-tools
    //（对围堰 uv run 的最后运行时依赖随之消除）。
    let mats = base.join("sih-tools").join(materials);
    let mut cmd = Command::new(selector_bin());
    cmd.args([
        "route",
        "--pack",
        &parking_pack_arg(),
        "--reference-time",
        &at.to_string(),
    ]);
    cmd.arg(&mats);
    let out = run_with_timeout(cmd, Duration::from_secs(ROUTE_TIMEOUT_S)).map_err(|_| ())?;
    let parsed: Value = serde_json::from_str(out.stdout.trim()).map_err(|_| ())?;
    match parsed.get("summary") {
        Some(s) if s.is_object() => Ok(s.clone()),
        _ => Err(()),
    }
}

fn parking_line(base: &Path, at: chrono::NaiveDate) -> String {
    let run = || -> Result<String, ()> {
        let eng = route_one(base, "../sih-engine/sih/state/parking/materials", at)?;
        let tls = route_one(base, "parking/materials", at)?;
        let fmt = |s: &Value| -> String {
            let g = |k: &str| -> String {
                match s.get(k) {
                    Some(Value::String(x)) => x.clone(),
                    Some(v) if !v.is_null() => v.to_string(),
                    _ => "-".to_string(),
                }
            };
            let alarms = s
                .get("alarms")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            format!(
                "主线{} 侧{} 废{} 告警{}",
                g("mainline"),
                g("siding"),
                g("scrap_track"),
                alarms
            )
        };
        Ok(format!(
            "[泊界] 引擎线 {} · 工具线 {}",
            fmt(&eng),
            fmt(&tls)
        ))
    };
    run().unwrap_or_else(|_| "[泊界] 路由不可用（降级）".to_string())
}

fn registry_domains(base: &Path) -> Vec<String> {
    let mut domains: Vec<String> = Vec::new();
    let text = match std::fs::read_to_string(base.join("sih-tools/mcpline/ledger/tokens.ndjson")) {
        Ok(t) => t,
        Err(_) => return domains,
    };
    for line in text.lines() {
        let e: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if e.get("status").and_then(|v| v.as_str()) != Some("active") {
            continue;
        }
        if let Some(d) = e.get("domain_root").and_then(|v| v.as_str()) {
            if !d.is_empty() && !domains.iter().any(|x| x == d) {
                domains.push(d.to_string());
            }
        }
    }
    domains
}

fn commit_days(domain: &str) -> std::collections::BTreeMap<String, u64> {
    let out = match Command::new("git")
        .args(["-C", domain, "log", "--pretty=%cs"])
        .output()
    {
        Ok(o) => o,
        Err(_) => return Default::default(),
    };
    if !out.status.success() {
        return Default::default();
    }
    let mut days: std::collections::BTreeMap<String, u64> = Default::default();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let d = line.trim();
        let b = d.as_bytes();
        if b.len() == 10 && b[4] == b'-' && b[7] == b'-' {
            *days.entry(d.to_string()).or_insert(0) += 1;
        }
    }
    days
}

fn covered(domain: &str, day: &str) -> bool {
    match std::fs::read_to_string(
        Path::new(domain)
            .join("sih/event/trail")
            .join(format!("{}.ndjson", day)),
    ) {
        Ok(text) => text.lines().any(|s| !s.trim().is_empty()),
        Err(_) => false,
    }
}

fn coverage_alarms(base: &Path, today_d: chrono::NaiveDate) -> Vec<String> {
    let mut alarms = Vec::new();
    for domain in registry_domains(base) {
        let counts = commit_days(&domain);
        if counts.is_empty() {
            continue;
        }
        let floor = today_d - chrono::Duration::days(COVERAGE_WINDOW_DAYS - 1);
        let floor_s = floor.to_string();
        let today_s = today_d.to_string();
        let in_window: Vec<String> = counts
            .keys()
            .filter(|d| d.as_str() >= floor_s.as_str() && d.as_str() <= today_s.as_str())
            .cloned()
            .collect();
        for d in in_window {
            if !covered(&domain, &d) {
                let name = Path::new(&domain)
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| domain.clone());
                alarms.push(format!(
                    "越限告警：{} {} {}笔提交查无此人",
                    name, d, counts[&d]
                ));
            }
        }
    }
    alarms
}

fn chain_line(base: &Path, today_d: chrono::NaiveDate) -> String {
    let reading = (|| -> Result<String, ()> {
        let f = base
            .join("sih-engine/sih/event/trail")
            .join(format!("{}.ndjson", today_d.to_string()));
        let text = std::fs::read_to_string(&f).map_err(|_| ())?;
        let lines: Vec<&str> = text.lines().collect();
        let n = lines.len();
        let last: Value = if n > 0 {
            serde_json::from_str(lines[n - 1]).map_err(|_| ())?
        } else {
            json!({})
        };
        let h8: String = match last.get("event_hash") {
            Some(Value::String(s)) => s.chars().take(8).collect(),
            Some(v) => py_str(v).chars().take(8).collect(),
            None => String::new(),
        };
        let et = match last.get("event_type") {
            Some(v) => py_str(v),
            None => String::new(),
        };
        Ok(format!("[链] 今日 {} 笔 末笔 {} {}", n, h8, et))
    })()
    .unwrap_or_else(|_| "[链] 当日链不可读（降级）".to_string());
    let alarms = coverage_alarms(base, today_d);
    if alarms.is_empty() {
        reading
    } else {
        format!("{} · {}", reading, alarms.join(" · "))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut root_arg: Option<String> = None;
    let mut at_arg: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                i += 1;
                root_arg = args.get(i).cloned();
            }
            "--at" => {
                i += 1;
                at_arg = args.get(i).cloned();
            }
            _ => {}
        }
        i += 1;
    }
    let base = resolve_root(root_arg.as_deref());
    let at = at_arg
        .as_deref()
        .and_then(parse_at)
        .unwrap_or_else(today);
    let text = [
        anchor_line(&base),
        inflight_line(&base),
        parking_line(&base, at),
        chain_line(&base, at),
        DISCIPLINE.to_string(),
    ]
    .join("\n");
    let mut m = Map::new();
    m.insert("additionalContext".to_string(), json!(text));
    print!("{}", emit(&Value::Object(m)));
    // 退出码恒零承围堰契约：注入链路永不阻断会话，失效降级可见不静默
}
