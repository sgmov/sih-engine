//! 引擎侧正身（Identity）命令行面 —— lease-mergeleg23-parallel 簇E 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/identity 0.5.0（src/identity/：cli.py、core.py），
//! 只读对表移植，围堰源码零改动。零信任身份验证：采集组件十二件、身份串 v3
//! 规范形、盐哈希与无盐核哈希、声明对表、human seat 五闸验真。留痕不签即
//! 结果只进报告不阻断调用方动作。
//!
//! CLI：`identity verify [--claims <声明JSON>] [--salt <64位hex>] [--at <ISO8601>]
//!       [--inject <复演JSON>] [--human-seat <验真件JSON>] [--no-net] [--quiet]`
//! 退出码三值：0 = attest/verified 无异常、1 = 有异常或失配或 human seat 拒、
//! 2 = 工具异常（盐非法/声明越域/复演件缺键/文件不可读）。输出 json 报告
//! sort_keys + indent 2（--quiet 即单行紧凑形），错误即 stdout {"error": ...}。
//!
//! 落差申报（相对围堰）：
//! 1. 外部钟探针以 curl 子进程承载（围堰 urllib.request），--max-time 3 对齐
//!    NET_TIMEOUT，重定向跟随对齐；curl 缺席或失败即缺席空串与围堰超时形一致。
//! 2. 子进程采集（sysctl/ps/ifconfig）无超时闸（围堰 subprocess timeout=2；
//!    std Command 无超时，实践命令瞬回；curl 自带 --max-time）。
//! 3. MAC 采集以 ifconfig -a 首 ether/lladdr 行承载（围堰 uuid.getnode 多策略
//!    探测近似），组播位判随机回退语义保留，无址即空串。
//! 4. 非字符串型 claims/inject 组件值（数值与布尔与 null）的渲染以 JSON 形承载
//!    （true/false/null），围堰 Python str() 形（True/False/None）不对齐；
//!    字符串值（实践形）两边逐字节一致。
//! 5. 时间戳解析认 RFC3339 与空格分隔基形；Python fromisoformat 的更宽紧凑形
//!    未对齐；解析失败两边同为无异常可算即零异常。
//! 6. 用法错（缺子命令、未知旗标）为自足解析 stderr 回报，退出码 2 与围堰
//!    argparse 对齐，报文形不对齐；帮助面未移植。
//! 7. 时钟偏移与开机时刻比较按整秒承载（围堰 .timestamp() 浮点微差未对齐）。

use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat, TimeZone, Utc};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::net::TcpStream;
use std::process::{Command, exit};
use std::time::Duration;

/// 围堰版本锚：sih-tools/identity/src/identity/__init__.py __version__。
const ENGINE_VERSION: &str = "0.5.0";

/// 组件序归一位（ALG-002 商集隔离，core.py COMPONENT_ORDER 对表）。
const COMPONENT_ORDER: [&str; 12] = [
    "pid",
    "ppid",
    "hostname",
    "mac",
    "user",
    "sandbox_id",
    "session_id",
    "boottime",
    "parent_start",
    "ancestry",
    "net_time",
    "timestamp",
];
const COMPARABLE: [&str; 11] = [
    "pid",
    "ppid",
    "hostname",
    "mac",
    "user",
    "sandbox_id",
    "session_id",
    "boottime",
    "parent_start",
    "ancestry",
    "net_time",
];
const CORE_PREFIX: &str = "core-v1";
const CORE_KEY_ORDER: [&str; 6] = [
    "mac",
    "hostname",
    "user",
    "boottime",
    "sandbox_id",
    "lineage",
];
const GENERIC_WRAPPER_SEGMENTS: [&str; 7] = [
    "python3",
    "uv",
    "zsh",
    "bash",
    "sh",
    "launchd",
    "agent-tool-host",
];
const SALT_BYTES: usize = 32;
const SALT_HEX_LEN: usize = 64;
const PROC_CAP: usize = 32;
const SKEW_MAX_SECONDS: i64 = 300;
const NET_PROBE_HOST: &str = "1.1.1.1";
const NET_PROBE_PORT: u16 = 443;
const NET_TIME_URL: &str = "https://www.cloudflare.com/cdn-cgi/trace";
const HUMAN_SEAT: &str = "human";
const HUMAN_SEAT_REQUIRED: [&str; 3] = ["seat", "name", "attest_at"];
const FINGERPRINT_FORMS: [&str; 3] = ["keyboard_layout", "boottime", "link_code"];

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

/// 对象键递归排序（json.dumps sort_keys=True 语义）。
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

fn emit_error(msg: &str) -> ! {
    println!("{}", json!({"error": msg}));
    exit(2);
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!(
        "用法: identity verify [--claims <声明JSON>] [--salt <64位hex>] [--at <ISO8601>] \
         [--inject <复演JSON>] [--human-seat <验真件JSON>] [--no-net] [--quiet]"
    );
    exit(2);
}

fn read_text(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|_| format!("input file unreadable: {path}"))
}

/// 非字符串组件值的 JSON 形渲染（落差四）。
fn json_str_value(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

// ---------- 盐（core.py validate_salt/new_salt 对表） ----------

fn validate_salt(salt_hex: &str) -> Result<String, String> {
    let lowered = salt_hex.to_lowercase();
    if lowered.len() != SALT_HEX_LEN || !lowered.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("salt must be 64 hex chars".to_string());
    }
    Ok(lowered)
}

fn new_salt() -> String {
    if let Ok(bytes) = fs::read("/dev/urandom") {
        // 只取前 32 字节；/dev/urandom 是无限流，read 返回请求量。
        let take: Vec<u8> = bytes.into_iter().take(SALT_BYTES).collect();
        if take.len() == SALT_BYTES {
            return hex::encode(&take);
        }
    }
    // 兜底：双 v4 UUID 合成 32 字节（落地机器 /dev/urandom 恒可用，此径不达）。
    let mut buf = Vec::with_capacity(SALT_BYTES);
    while buf.len() < SALT_BYTES {
        buf.extend_from_slice(uuid::Uuid::new_v4().as_bytes());
    }
    buf.truncate(SALT_BYTES);
    hex::encode(&buf)
}

// ---------- 组件采集（core.py collect_components 对表） ----------

fn run_cmd(cmd: &[&str]) -> String {
    let Ok(out) = Command::new(cmd[0]).args(&cmd[1..]).env("LC_ALL", "C").output()
    else {
        return String::new();
    };
    if out.status.success() {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        String::new()
    }
}

/// 网卡硬件址：ifconfig -a 首 ether/lladdr 行，组播位置位即随机回退判缺席（落差三）。
fn collect_mac() -> String {
    let out = run_cmd(&["ifconfig", "-a"]);
    for line in out.lines() {
        let t = line.trim();
        let rest = if let Some(r) = t.strip_prefix("ether ") {
            r
        } else if let Some(r) = t.strip_prefix("lladdr ") {
            r
        } else {
            continue;
        };
        let Some(addr) = rest.split_whitespace().next() else {
            continue;
        };
        let groups: Vec<&str> = addr.split(':').collect();
        if groups.len() == 6
            && groups
                .iter()
                .all(|g| g.len() == 2 && g.chars().all(|c| c.is_ascii_hexdigit()))
        {
            let node = groups.join("").to_lowercase();
            // uuid.getnode: (node >> 40) & 0x01 组播位判随机回退。
            let first = u8::from_str_radix(&node[0..2], 16).unwrap_or(0xff);
            if first & 0x01 == 1 {
                return String::new();
            }
            return node;
        }
    }
    String::new()
}

fn collect_hostname() -> String {
    let mut buf = [0u8; 256];
    unsafe {
        if libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) == 0 {
            let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            return String::from_utf8_lossy(&buf[..end]).into_owned();
        }
    }
    String::new()
}

/// getpass.getuser 序：LOGNAME、USER、LNAME、USERNAME，末位 getpwuid。
fn collect_user() -> String {
    for k in ["LOGNAME", "USER", "LNAME", "USERNAME"] {
        if let Ok(v) = std::env::var(k) {
            if !v.is_empty() {
                return v;
            }
        }
    }
    unsafe {
        let pw = libc::getpwuid(libc::getuid());
        if !pw.is_null() {
            let name = (*pw).pw_name;
            if !name.is_null() {
                return std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned();
            }
        }
    }
    String::new()
}

/// 开机时刻即内核 kern.boottime，时钟回拨到开机前的穿帮件。
fn collect_boottime() -> String {
    let out = run_cmd(&["sysctl", "-n", "kern.boottime"]);
    let re = regex::Regex::new(r"sec\s*=\s*(\d+)").unwrap();
    if let Some(m) = re.captures(&out) {
        return m[1].to_string();
    }
    if let Ok(text) = fs::read_to_string("/proc/stat") {
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("btime ") {
                if let Some(tok) = rest.split_whitespace().next() {
                    return tok.to_string();
                }
            }
        }
    }
    String::new()
}

/// 父进程出生时刻，pid 复用冒充的穿帮件。
fn collect_parent_start() -> String {
    let ppid = unsafe { libc::getppid() };
    let out = run_cmd(&["ps", "-p", &ppid.to_string(), "-o", "lstart="]);
    if out.is_empty() {
        return String::new();
    }
    let norm = out.split_whitespace().collect::<Vec<_>>().join(" ");
    NaiveDateTime::parse_from_str(&norm, "%a %b %d %H:%M:%S %Y")
        .ok()
        .and_then(|naive| Local.from_local_datetime(&naive).single())
        .map(|dt| {
            dt.with_timezone(&Utc)
                .to_rfc3339_opts(SecondsFormat::Secs, false)
        })
        .unwrap_or_default()
}

fn path_basename(p: &str) -> &str {
    match p.rsplit_once('/') {
        Some((_, base)) => base,
        None => p,
    }
}

/// 完整父链进程名序列，自本进程上溯至顶，深度封顶。
fn collect_ancestry() -> String {
    let mut names: Vec<String> = Vec::new();
    let mut pid: i32 = unsafe { libc::getpid() };
    let mut seen: HashSet<i32> = HashSet::new();
    for _ in 0..PROC_CAP {
        if pid <= 0 || seen.contains(&pid) {
            break;
        }
        seen.insert(pid);
        let out = run_cmd(&["ps", "-p", &pid.to_string(), "-o", "ppid=,comm="]);
        let trimmed = out.trim_start();
        let Some(first_ws) = trimmed.find(char::is_whitespace) else {
            break;
        };
        let first = &trimmed[..first_ws];
        let rest = trimmed[first_ws..].trim();
        if first.is_empty() || rest.is_empty() {
            break;
        }
        names.push(path_basename(rest).to_string());
        match first.parse::<i32>() {
            Ok(p) => pid = p,
            Err(_) => break,
        }
    }
    names.join(">")
}

/// 联网可达探针即直连 TCP，通即真可达。
fn probe_net_reachable() -> bool {
    let addr = format!("{NET_PROBE_HOST}:{NET_PROBE_PORT}");
    if let Ok(addrs) = std::net::ToSocketAddrs::to_socket_addrs(addr.as_str()) {
        for a in addrs {
            if TcpStream::connect_timeout(&a, Duration::from_millis(1500)).is_ok() {
                return true;
            }
        }
    }
    false
}

/// 外部权威时间即 trace 接口，curl 子进程承载（落差一），超时即缺席。
fn fetch_net_time() -> String {
    let Ok(out) = Command::new("curl")
        .args(["-s", "-L", "--max-time", "3", NET_TIME_URL])
        .output()
    else {
        return String::new();
    };
    if !out.status.success() {
        return String::new();
    }
    let body = String::from_utf8_lossy(&out.stdout[..out.stdout.len().min(4096)]);
    for line in body.lines() {
        if let Some(v) = line.strip_prefix("ts=") {
            if let Ok(f) = v.trim().parse::<f64>() {
                return (f as i64).to_string();
            }
        }
    }
    String::new()
}

fn collect_components(
    inject: Option<&Value>,
    at: Option<&str>,
    no_net: bool,
) -> Result<Map<String, Value>, String> {
    let mut out = Map::new();
    if let Some(inj) = inject {
        let mut missing: Vec<&str> = Vec::new();
        for key in COMPONENT_ORDER {
            match inj.get(key) {
                Some(v) => {
                    out.insert(key.to_string(), Value::String(json_str_value(v)));
                }
                None => missing.push(key),
            }
        }
        if !missing.is_empty() {
            return Err(format!("inject missing components: {}", missing.join(", ")));
        }
        return Ok(out);
    }
    let timestamp = at
        .map(|s| s.to_string())
        .unwrap_or_else(|| Utc::now().to_rfc3339_opts(SecondsFormat::Secs, false));
    let mut vals: Vec<(&str, String)> = Vec::new();
    vals.push(("pid", unsafe { libc::getpid() }.to_string()));
    vals.push(("ppid", unsafe { libc::getppid() }.to_string()));
    vals.push(("hostname", collect_hostname()));
    vals.push(("mac", collect_mac()));
    vals.push(("user", collect_user()));
    vals.push((
        "sandbox_id",
        std::env::var("SIH_SANDBOX_ID").unwrap_or_default(),
    ));
    vals.push((
        "session_id",
        std::env::var("SIH_SESSION_ID").unwrap_or_default(),
    ));
    vals.push(("boottime", collect_boottime()));
    vals.push(("parent_start", collect_parent_start()));
    vals.push(("ancestry", collect_ancestry()));
    vals.push(("net_time", if no_net { String::new() } else { fetch_net_time() }));
    vals.push(("timestamp", timestamp));
    for (k, v) in vals {
        out.insert(k.to_string(), Value::String(v));
    }
    Ok(out)
}

// ---------- 身份串与哈希（core.py 对表，ALG-002 序归一与哈希合成位） ----------

fn identity_string(components: &Map<String, Value>) -> String {
    let mut parts: Vec<String> = Vec::new();
    for key in COMPONENT_ORDER {
        let v = components
            .get(key)
            .and_then(|x| x.as_str())
            .unwrap_or_default();
        parts.push(format!("{key}={v}"));
    }
    format!("v3|{}", parts.join("|"))
}

fn identity_hash(salt_hex: &str, components: &Map<String, Value>) -> String {
    sha256_hex(format!("{salt_hex}|{}", identity_string(components)).as_bytes())
}

fn normalize_lineage(ancestry: &str) -> String {
    if ancestry.is_empty() {
        return String::new();
    }
    let re = regex::Regex::new(r"^(zcode-host-local)-\d+$").unwrap();
    let mut tokens: Vec<String> = Vec::new();
    for seg0 in ancestry.split('>') {
        let seg = seg0.trim();
        if seg.is_empty() || GENERIC_WRAPPER_SEGMENTS.contains(&seg) {
            continue;
        }
        let mut seg_s = seg.to_string();
        if let Some(m) = re.captures(seg) {
            seg_s = m[1].to_string();
        }
        tokens.push(seg_s);
    }
    tokens.join(">")
}

fn core_components(components: &Map<String, Value>) -> Map<String, Value> {
    let mut out = Map::new();
    let get = |k: &str| {
        components
            .get(k)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };
    out.insert("mac".into(), json!(get("mac")));
    out.insert("hostname".into(), json!(get("hostname")));
    out.insert("user".into(), json!(get("user")));
    out.insert("boottime".into(), json!(get("boottime")));
    out.insert("sandbox_id".into(), json!(get("sandbox_id")));
    out.insert(
        "lineage".into(),
        json!(normalize_lineage(&get("ancestry"))),
    );
    out
}

fn core_string(components: &Map<String, Value>) -> String {
    let core = core_components(components);
    let parts: Vec<String> = CORE_KEY_ORDER
        .iter()
        .map(|k| {
            let v = core[*k].as_str().unwrap_or_default();
            format!("{k}={v}")
        })
        .collect();
    format!("{CORE_PREFIX}|{}", parts.join("|"))
}

fn core_hash(components: &Map<String, Value>) -> String {
    sha256_hex(core_string(components).as_bytes())
}

// ---------- 声明对表（core.py compare_claims 对表，PROB-013 漂移比对位） ----------

fn compare_claims(
    claims: &Value,
    observed: &Map<String, Value>,
) -> Result<(Map<String, Value>, usize, usize), String> {
    let obj = claims.as_object().unwrap();
    let mut keys: Vec<&String> = obj.keys().collect();
    keys.sort();
    let mut results = Map::new();
    let mut matched = 0usize;
    let claimed_total = keys.len();
    for key in keys.iter() {
        let key: &String = key;
        if !COMPARABLE.contains(&key.as_str()) {
            return Err(format!("claim key outside comparable set: {key}"));
        }
        let claimed = json_str_value(&obj[key.as_str()]);
        let observed_value = observed
            .get(key.as_str())
            .map(json_str_value)
            .unwrap_or_default();
        let result = if claimed == observed_value {
            matched += 1;
            "match"
        } else {
            "mismatch"
        };
        results.insert(
            key.clone(),
            json!({"claimed": claimed, "observed": observed_value, "result": result}),
        );
    }
    let mismatched = claimed_total - matched;
    Ok((results, matched, mismatched))
}

// ---------- 异常点（core.py detect_anomalies 对表，观察事实非动作） ----------

fn parse_timestamp_epoch(s: &str) -> Option<f64> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.timestamp() as f64);
    }
    for fmt in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%d %H:%M:%S%.f"] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(s, fmt) {
            if let Some(dt) = Local.from_local_datetime(&naive).single() {
                return Some(dt.timestamp() as f64);
            }
        }
    }
    None
}

fn detect_anomalies(observed: &Map<String, Value>, now_epoch: f64, live: bool) -> Vec<Value> {
    let mut anomalies = Vec::new();
    let net_time = observed
        .get("net_time")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let reachable = !net_time.is_empty() || (live && probe_net_reachable());
    if net_time.is_empty() && reachable {
        anomalies.push(json!({
            "detail": "tcp reachable but external time absent",
            "kind": "net_time_missing_while_reachable",
        }));
    }
    if !net_time.is_empty() {
        if let Ok(nt) = net_time.trim().parse::<i64>() {
            if (now_epoch - nt as f64).abs() > SKEW_MAX_SECONDS as f64 {
                anomalies.push(json!({
                    "detail": format!("local vs net skew over {SKEW_MAX_SECONDS}s"),
                    "kind": "clock_skew",
                }));
            }
        }
    }
    let boottime = observed
        .get("boottime")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if !boottime.is_empty() {
        if let Ok(bt) = boottime.trim().parse::<i64>() {
            if now_epoch < bt as f64 {
                anomalies.push(json!({
                    "detail": "local timestamp precedes boot",
                    "kind": "timestamp_before_boottime",
                }));
            }
        }
    }
    anomalies
}

// ---------- human seat 五闸验真（core.py verify_human_seat 对表，留痕不签） ----------

fn filled(v: Option<&Value>) -> bool {
    v.and_then(|x| x.as_str())
        .map(|s| !s.is_empty())
        .unwrap_or(false)
}

fn fingerprint_registered(fp: Option<&Value>) -> bool {
    match fp {
        Some(Value::Object(m)) => FINGERPRINT_FORMS.iter().any(|form| {
            m.get(*form)
                .map(|v| !v.is_null() && v.as_str() != Some(""))
                .unwrap_or(false)
        }),
        Some(other) => filled(Some(other)),
        None => false,
    }
}

fn verify_human_seat(record: &Value) -> Value {
    let obj = record.as_object();
    let gate1 = obj.is_some();
    let gate2 = obj
        .map(|o| o.get("seat").and_then(|v| v.as_str()) == Some(HUMAN_SEAT))
        .unwrap_or(false);
    let gate3 = obj
        .map(|o| {
            HUMAN_SEAT_REQUIRED
                .iter()
                .filter(|k| **k != "seat")
                .all(|k| filled(o.get(*k)))
        })
        .unwrap_or(false);
    let (gate4, gate5) = if gate1 {
        let o = obj.unwrap();
        (
            filled(o.get("guardian")),
            fingerprint_registered(o.get("fingerprint")),
        )
    } else {
        (false, false)
    };
    let verdict = if gate1 && gate2 && gate3 {
        "attest"
    } else {
        "reject"
    };
    json!({
        "gates": {
            "gate1_object": gate1,
            "gate2_seat": gate2,
            "gate3_required_fields": gate3,
            "gate4_guardian_signed": gate4,
            "gate5_fingerprint_registered": gate5,
        },
        "verdict": verdict,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage_fail("缺子命令（verify）");
    }
    if args[0] != "verify" {
        usage_fail(&format!("未知子命令: {}", args[0]));
    }
    let mut claims_path: Option<String> = None;
    let mut salt_arg: Option<String> = None;
    let mut at: Option<String> = None;
    let mut inject_path: Option<String> = None;
    let mut human_seat_path: Option<String> = None;
    let mut no_net = false;
    let mut quiet = false;
    let mut i = 1usize;
    while i < args.len() {
        let a = args[i].clone();
        match a.as_str() {
            "--claims" => {
                i += 1;
                if i >= args.len() {
                    usage_fail("--claims 缺值");
                }
                claims_path = Some(args[i].clone());
            }
            "--salt" => {
                i += 1;
                if i >= args.len() {
                    usage_fail("--salt 缺值");
                }
                salt_arg = Some(args[i].clone());
            }
            "--at" => {
                i += 1;
                if i >= args.len() {
                    usage_fail("--at 缺值");
                }
                at = Some(args[i].clone());
            }
            "--inject" => {
                i += 1;
                if i >= args.len() {
                    usage_fail("--inject 缺值");
                }
                inject_path = Some(args[i].clone());
            }
            "--human-seat" => {
                i += 1;
                if i >= args.len() {
                    usage_fail("--human-seat 缺值");
                }
                human_seat_path = Some(args[i].clone());
            }
            "--no-net" => no_net = true,
            "--quiet" => quiet = true,
            _ if a.starts_with("--") => usage_fail(&format!("未知旗标: {a}")),
            _ => usage_fail(&format!("多余位置参数: {a}")),
        }
        i += 1;
    }

    let inject: Option<Value> = match &inject_path {
        Some(p) => {
            let text = match read_text(p) {
                Ok(t) => t,
                Err(e) => emit_error(&e),
            };
            let v: Value = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(e) => emit_error(&format!("input json invalid: {e}")),
            };
            if !v.is_object() {
                emit_error("inject top level must be an object");
            }
            Some(v)
        }
        None => None,
    };

    let observed = match collect_components(inject.as_ref(), at.as_deref(), no_net) {
        Ok(o) => o,
        Err(e) => emit_error(&e),
    };
    let salt = match &salt_arg {
        Some(s) if !s.is_empty() => match validate_salt(s) {
            Ok(v) => v,
            Err(e) => emit_error(&e),
        },
        _ => new_salt(),
    };

    let now_epoch = parse_timestamp_epoch(
        observed.get("timestamp").and_then(|v| v.as_str()).unwrap_or(""),
    );
    let anomalies = match now_epoch {
        Some(epoch) => detect_anomalies(&observed, epoch, inject.is_none() && !no_net),
        None => vec![],
    };
    let net_time_empty = observed
        .get("net_time")
        .and_then(|v| v.as_str())
        .map(|s| s.is_empty())
        .unwrap_or(true);
    let net_probe = if !net_time_empty {
        "external"
    } else if no_net {
        "disabled"
    } else {
        "absent"
    };
    let mode = if claims_path.is_some() {
        "verify"
    } else {
        "attest"
    };
    let identity_hash_value = identity_hash(&salt, &observed);
    let mut report = json!({
        "anomalies": anomalies,
        "claims": {},
        "header": {
            "mode": mode,
            "net_probe": net_probe,
            "tool": {"name": "identity", "version": ENGINE_VERSION},
        },
        "identity": {
            "hash": identity_hash_value,
            "core_hash": core_hash(&observed),
            "core_components": core_components(&observed),
            "salt": salt,
            "string": identity_string(&observed),
        },
        "observed": observed,
        "summary": {
            "anomalies": anomalies.len(),
            "claimed": 0,
            "matched": 0,
            "mismatched": 0,
            "verdict": "attest",
        },
    });
    let mut exit_code: i32 = if !anomalies.is_empty() { 1 } else { 0 };

    if let Some(cp) = &claims_path {
        let text = match read_text(cp) {
            Ok(t) => t,
            Err(e) => emit_error(&e),
        };
        let claims: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => emit_error(&format!("input json invalid: {e}")),
        };
        if !claims.is_object() {
            emit_error("claims top level must be an object");
        }
        let (results, matched, mismatched) = match compare_claims(&claims, report["observed"].as_object().unwrap())
        {
            Ok(r) => r,
            Err(e) => emit_error(&e),
        };
        let verdict = if mismatched == 0 && anomalies.is_empty() {
            "verified"
        } else {
            "mismatch"
        };
        report["claims"] = Value::Object(results);
        report["summary"] = json!({
            "anomalies": anomalies.len(),
            "claimed": matched + mismatched,
            "matched": matched,
            "mismatched": mismatched,
            "verdict": verdict,
        });
        exit_code = if mismatched > 0 || !anomalies.is_empty() {
            1
        } else {
            0
        };
    }

    if let Some(hp) = &human_seat_path {
        let text = match read_text(hp) {
            Ok(t) => t,
            Err(e) => emit_error(&e),
        };
        let seat_record: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => emit_error(&format!("human seat json invalid: {e}")),
        };
        let seat_result = verify_human_seat(&seat_record);
        let verdict = seat_result["verdict"].as_str().unwrap_or_default().to_string();
        report["human_seat"] = json!({
            "gates": seat_result["gates"],
            "record": seat_record,
            "source": hp,
            "verdict": verdict,
        });
        report["summary"]["human_seat_verdict"] = json!(verdict);
        if verdict == "reject" {
            exit_code = 1;
        }
    }

    if quiet {
        let mut line = json!({
            "tool": "identity",
            "command": "verify",
            "code": exit_code,
            "summary": {
                "status": report["summary"]["verdict"],
                "anomalies": anomalies.len(),
                "claimed": report["summary"]["claimed"],
                "identity_hash": identity_hash_value,
            },
        });
        if human_seat_path.is_some() {
            line["summary"]["human_seat_verdict"] = report["summary"]["human_seat_verdict"].clone();
        }
        println!("{}", sort_value(line));
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&sort_value(report)).unwrap()
        );
    }
    exit(exit_code);
}
