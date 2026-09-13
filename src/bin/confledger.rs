//! 引擎侧冲突账本（confledger）命令行面 —— lease-mergeleg6-parallel 簇I 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/confledger（cli.py、ledger.py、constants.py、preempt.py），
//! 只读对表移植，围堰源码零改动。规格基准 confmodel-derivation v3：D1 余额守恒、
//! D2 下限零、D3 支付 fail-closed、D4 铸币纯机械、D10 人席位无强制算子、D11 grandfather。
//! 休眠门（结算罚金 D6 递延 I3）全部零触发。
//!
//! CLI：`confledger <open|activate|mint|pay|balance|verify|replay|bills|preempt> ...`
//! 退出码三值：0 = 过、1 = 业务拒绝（含 verify invalid 与休眠门）、2 = 工具异常。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. preempt 环五步纯函数（preempt_ring）在围堰即不可达：constants.PREEMPT_PRICE
//!    槽位态休眠（None）使 CLI 恒走休眠门退出码一。本移植只承载休眠门同形，环体
//!    未移植（同参下环体不可达，移植死码零行为增益）；休眠门激活属围堰后继批。
//! 2. 事件时间戳取 UTC 微秒 ISO 形（%Y-%m-%dT%H:%M:%S%.6f+00:00），与围堰
//!    datetime.now(timezone.utc).isoformat() 微秒形同构；非复演注入位（围堰无 --at）。
//! 3. 账本追加以 flock(LOCK_EX) + write_all 承围堰 flock 加单次 os.write 写点纪律
//!    （pk057fix）；整行小于缓冲时与单调用观测等价，write_all 循环语义为 Rust 面。
//! 4. 文件缺席类错误（正身件、trail、账单）退出码二与围堰一致，error 报文内文为
//!    Rust 形，不逐字对齐 Python OSError 字符串。
//! 5. argparse 用法错（缺参、未知子命令、--seat 选择字越界）为自足解析，stderr
//!    短句回报退出码 2，报文形不对齐。
//! 6. 台账行 JSON 为 Python json.dumps(sort_keys=True) 缺省分隔形（", "/": "，
//!    ensure_ascii=False 原 UTF-8）逐字节同构；stdout emit 同形。
//! 7. balance/verify 对缺字段事件行按空串零值容错；围堰对缺 ts/ref/account 键
//!    以 KeyError 崩溃退出，移植不复制崩溃路径。
//! 8. verify 的未配对支付 findings 遍历按 ref 首现序（与围堰 dict 插入序同形）。

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::process::exit;

/// 推导档 v3 已定值（constants.py 对表）：m_clean = 1 归一通道（单位锚）、
/// m_active = 1 推导通道；其余全槽位态休眠（PREEMPT_PRICE 等 None 即零值不在此出现）。
const M_CLEAN: i64 = 1;
const M_ACTIVE: i64 = 1;
const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const KINDS: [&str; 8] = [
    "activation",
    "account_open",
    "mint_clean_close",
    "mint_active_pen",
    "mint_repair_foreign",
    "mint_self_repair",
    "pay_out",
    "pay_in",
];

fn mint_amount(kind: &str) -> Option<i64> {
    match kind {
        "mint_clean_close" => Some(M_CLEAN),
        "mint_active_pen" => Some(M_ACTIVE),
        _ => None,
    }
}

// ---------- Python json.dumps 同构序列化（sort_keys / ensure_ascii=False） ----------

fn json_quote(s: &str) -> String {
    serde_json::to_string(s).unwrap()
}

fn py_json(v: &Value, indent: Option<usize>, compact: bool, sort: bool) -> String {
    let mut out = String::new();
    py_rec(v, indent, compact, sort, 0, &mut out);
    out
}

fn py_rec(
    v: &Value,
    indent: Option<usize>,
    compact: bool,
    sort: bool,
    depth: usize,
    out: &mut String,
) {
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
                    match indent {
                        None => {
                            if !compact {
                                out.push(' ');
                            }
                        }
                        Some(n) => {
                            out.push('\n');
                            out.push_str(&" ".repeat((depth + 1) * n));
                        }
                    }
                } else if let Some(n) = indent {
                    out.push('\n');
                    out.push_str(&" ".repeat((depth + 1) * n));
                }
                out.push_str(&json_quote(k));
                out.push_str(if compact { ":" } else { ": " });
                py_rec(val, indent, compact, sort, depth + 1, out);
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
                    match indent {
                        None => {
                            if !compact {
                                out.push(' ');
                            }
                        }
                        Some(n) => {
                            out.push('\n');
                            out.push_str(&" ".repeat((depth + 1) * n));
                        }
                    }
                } else if let Some(n) = indent {
                    out.push('\n');
                    out.push_str(&" ".repeat((depth + 1) * n));
                }
                py_rec(val, indent, compact, sort, depth + 1, out);
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

fn ts_now() -> String {
    chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.6f+00:00")
        .to_string()
}

// ---------- 台账本体（ledger.py 对表） ----------

struct Ledger {
    path: PathBuf,
}

impl Ledger {
    fn events(&self) -> Result<Vec<Value>, String> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let text =
            fs::read_to_string(&self.path).map_err(|e| format!("ledger unreadable: {}", e))?;
        let mut out = vec![];
        for (i, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let v: Value = serde_json::from_str(line)
                .map_err(|e| format!("ledger line {} invalid: {}", i + 1, e))?;
            out.push(v);
        }
        Ok(out)
    }

    /// 追加唯一写点：flock 串行 + 单行落笔（pk057fix 纪律，头注落差三）。
    fn append(&self, event: &Value) -> Result<(), String> {
        if let Some(p) = self.path.parent() {
            if !p.as_os_str().is_empty() {
                fs::create_dir_all(p).map_err(|e| format!("ledger dir unwritable: {}", e))?;
            }
        }
        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| format!("ledger unwritable: {}", e))?;
        unsafe {
            libc::flock(f.as_raw_fd(), libc::LOCK_EX);
        }
        let line = py_json(event, None, false, true) + "\n";
        let mut handle = &f;
        handle
            .write_all(line.as_bytes())
            .map_err(|e| format!("ledger write failed: {}", e))?;
        Ok(()) // close 即放 flock
    }

    fn activation(&self) -> Result<Option<Value>, String> {
        for e in self.events()? {
            if e.get("kind").and_then(|k| k.as_str()) == Some("activation") {
                return Ok(Some(e));
            }
        }
        Ok(None)
    }
}

fn s_field<'a>(e: &'a Value, k: &str) -> &'a str {
    e.get(k).and_then(|v| v.as_str()).unwrap_or("")
}

fn i_field(e: &Value, k: &str) -> i64 {
    e.get(k).and_then(|v| v.as_i64()).unwrap_or(0)
}

fn accounts_of(events: &[Value]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for e in events {
        if s_field(e, "kind") == "account_open" {
            out.insert(
                s_field(e, "account").to_string(),
                e.get("seat").and_then(|v| v.as_str()).unwrap_or("agent").to_string(),
            );
        }
    }
    out
}

fn grandfather_cut(led: &Ledger) -> Result<Option<String>, String> {
    Ok(led
        .activation()?
        .map(|a| s_field(&a, "ts").to_string()))
}

/// 纯求和（cmd_pay 用，无 I1 中断）。
fn balance_sum(events: &[Value], account: &str, cut: Option<&str>) -> i64 {
    let mut total = 0;
    for e in events {
        if s_field(e, "account") != account {
            continue;
        }
        if let Some(c) = cut {
            if s_field(e, "ts") < c {
                continue;
            }
        }
        total += i_field(e, "amount");
    }
    total
}

fn emit(obj: Value, code: i32) -> ! {
    println!("{}", py_json(&obj, None, false, true));
    exit(code)
}

fn err2(msg: String) -> ! {
    emit(json!({ "error": msg }), 2)
}

// ---------- 命令面（cli.py 对表） ----------

fn load_identity_report(path: &str) -> Result<(String, String, String), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("正身件不可读: {}", e))?;
    let d: Value = serde_json::from_str(&text).map_err(|e| format!("正身件不可解析: {}", e))?;
    let core = d
        .get("core_hash")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .or_else(|| {
            d.get("identity")?
                .get("core_hash")?
                .as_str()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        })
        .unwrap_or_default();
    let ident = d
        .get("identity")
        .and_then(|i| i.get("hash"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if core.is_empty() || ident.is_empty() {
        return Err("正身件缺 core_hash 或 identity.hash".to_string());
    }
    let seat = d
        .get("seat")
        .and_then(|v| v.as_str())
        .unwrap_or("agent")
        .to_string();
    Ok((core, ident, seat))
}

fn cmd_open(map: &HashMap<String, String>) -> ! {
    let led = Ledger { path: PathBuf::from(req(map, "ledger")) };
    let report = req(map, "identity-report");
    let (core, ident, mut seat) = match load_identity_report(report) {
        Ok(v) => v,
        Err(e) => err2(e),
    };
    let events = match led.events() {
        Ok(v) => v,
        Err(e) => err2(e),
    };
    let accounts = accounts_of(&events);
    if accounts.contains_key(&core) {
        emit(json!({"error": "账户已开户", "account": core}), 1);
    }
    if let Some(arg_seat) = map.get("seat") {
        if arg_seat != &seat {
            seat = arg_seat.clone();
        }
    }
    let bytes = match fs::read(report) {
        Ok(b) => b,
        Err(e) => err2(format!("正身件不可读: {}", e)),
    };
    let ref_ = format!("identity-report:{}", sha256_hex(&bytes));
    let ev = json!({
        "ts": ts_now(), "kind": "account_open", "account": core,
        "amount": 0, "ref": ref_, "identity_hash": ident, "seat": seat,
    });
    if let Err(e) = led.append(&ev) {
        err2(e);
    }
    emit(json!({"status": "opened", "account": core, "seat": seat}), 0)
}

fn cmd_activate(map: &HashMap<String, String>) -> ! {
    let led = Ledger { path: PathBuf::from(req(map, "ledger")) };
    let events = match led.events() {
        Ok(v) => v,
        Err(e) => err2(e),
    };
    if !events.is_empty() {
        emit(json!({"error": "账本非空，activation 只能是首事件"}), 1);
    }
    let (_core, ident, _seat) = match load_identity_report(req(map, "identity-report")) {
        Ok(v) => v,
        Err(e) => err2(e),
    };
    let ref_ = req(map, "ref").to_string();
    let ts = req(map, "ts").to_string();
    let ev = json!({
        "ts": ts, "kind": "activation", "account": "",
        "amount": 0, "ref": ref_, "identity_hash": ident,
    });
    if let Err(e) = led.append(&ev) {
        err2(e);
    }
    emit(
        json!({"status": "activated", "t0_ts": req(map, "ts"), "t0_ref": req(map, "ref")}),
        0,
    )
}

fn cmd_mint(map: &HashMap<String, String>) -> ! {
    let led = Ledger { path: PathBuf::from(req(map, "ledger")) };
    let kind = req(map, "kind").to_string();
    if !KINDS.contains(&kind.as_str()) {
        emit(json!({"error": format!("未知 kind {kind}")}), 1);
    }
    if kind == "mint_repair_foreign" {
        emit(
            json!({"error": "m_repair 槽位态休眠（D_acc 待实证）", "kind": kind, "state": "dormant"}),
            1,
        );
    }
    if kind == "mint_self_repair" {
        emit(json!({"error": "自产自修不铸币（A2）", "kind": kind}), 1);
    }
    let amount = match mint_amount(&kind) {
        Some(a) => a,
        None => emit(json!({"error": format!("{kind} 无已定铸币额")}), 1),
    };
    let events = match led.events() {
        Ok(v) => v,
        Err(e) => err2(e),
    };
    let account = req(map, "account").to_string();
    if !accounts_of(&events).contains_key(&account) {
        emit(json!({"error": "账户未开户", "account": account}), 1);
    }
    let witness = match map.get("trail") {
        Some(t) => match fs::read_to_string(t) {
            Ok(s) => s,
            Err(e) => err2(format!("trail 不可读: {}", e)),
        },
        None => String::new(),
    };
    let ref_ = req(map, "ref").to_string();
    if ref_.is_empty() || !witness.contains(&ref_) {
        emit(json!({"error": "ref 须为链上事件哈希且在 --trail 见证", "ref": ref_}), 1);
    }
    let ev = json!({
        "ts": ts_now(), "kind": kind, "account": account,
        "amount": amount, "ref": ref_, "identity_hash": ZERO_HASH,
    });
    if let Err(e) = led.append(&ev) {
        err2(e);
    }
    emit(
        json!({"status": "minted", "kind": kind, "amount": amount, "account": account}),
        0,
    )
}

fn cmd_pay(map: &HashMap<String, String>) -> ! {
    let led = Ledger { path: PathBuf::from(req(map, "ledger")) };
    let events = match led.events() {
        Ok(v) => v,
        Err(e) => err2(e),
    };
    let accounts = accounts_of(&events);
    let from = req(map, "account").to_string();
    let to = req(map, "to").to_string();
    let amount: i64 = match req(map, "amount").parse() {
        Ok(v) => v,
        Err(_) => usage_fail("--amount 须整数（argparse type=int 同形）"),
    };
    if !accounts.contains_key(&from) {
        emit(json!({"error": "付款账户未开户", "account": from}), 1);
    }
    if !accounts.contains_key(&to) {
        emit(json!({"error": "收款账户未开户", "account": to}), 1);
    }
    if accounts.get(&from).map(|s| s.as_str()) == Some("human") {
        emit(json!({"error": "人席位无强制算子（D10）"}), 1);
    }
    if amount <= 0 {
        emit(json!({"error": "amount 须正整数"}), 1);
    }
    let cut = match grandfather_cut(&led) {
        Ok(c) => c,
        Err(e) => err2(e),
    };
    let bal = balance_sum(&events, &from, cut.as_deref());
    if bal < amount {
        emit(
            json!({"error": "余额不足，fail-closed 拒零部分支付", "balance": bal, "requested": amount}),
            1,
        );
    }
    let ts = ts_now();
    let ref_ = format!(
        "pay-{}",
        sha256_hex(format!("{from}{to}{amount}{ts}").as_bytes())
    );
    let out_ev = json!({
        "ts": ts, "kind": "pay_out", "account": from,
        "amount": -amount, "ref": ref_, "identity_hash": ZERO_HASH,
    });
    let in_ev = json!({
        "ts": ts, "kind": "pay_in", "account": to,
        "amount": amount, "ref": ref_, "identity_hash": ZERO_HASH,
    });
    if let Err(e) = led.append(&out_ev).and_then(|_| led.append(&in_ev)) {
        err2(e);
    }
    emit(
        json!({"status": "paid", "from": from, "to": to, "amount": amount, "ref": ref_,
               "note": "即付不退（A4），退款算子不在词汇面"}),
        0,
    )
}

fn cmd_balance(map: &HashMap<String, String>) -> ! {
    let led = Ledger { path: PathBuf::from(req(map, "ledger")) };
    let events = match led.events() {
        Ok(v) => v,
        Err(e) => err2(e),
    };
    let cut = match grandfather_cut(&led) {
        Ok(c) => c,
        Err(e) => err2(e),
    };
    if let Some(account) = map.get("account") {
        let mut total = 0i64;
        let mut excluded = 0i64;
        for e in &events {
            if s_field(e, "account") != account {
                continue;
            }
            if let Some(c) = &cut {
                if s_field(e, "ts") < c.as_str() {
                    excluded += 1;
                    continue;
                }
            }
            total += i_field(e, "amount");
            if total < 0 {
                emit(json!({"error": "I1 破：回放出现负余额", "account": account}), 1);
            }
        }
        emit(
            json!({"account": account, "balance": total, "grandfather_excluded": excluded}),
            0,
        );
    }
    let mut out = Map::new();
    for acct in accounts_of(&events).keys() {
        let mut total = 0i64;
        for e in &events {
            if s_field(e, "account") != acct {
                continue;
            }
            if let Some(c) = &cut {
                if s_field(e, "ts") < c.as_str() {
                    continue;
                }
            }
            total += i_field(e, "amount");
            if total < 0 {
                emit(json!({"error": "I1 破：回放出现负余额", "account": acct}), 1);
            }
        }
        let mut entry = Map::new();
        entry.insert("balance".to_string(), json!(total));
        out.insert(acct.clone(), Value::Object(entry));
    }
    emit(json!({"accounts": out}), 0)
}

fn ref16(e: &Value) -> String {
    let r = s_field(e, "ref");
    r.chars().take(16).collect()
}

fn acct8(account: &str) -> String {
    account.chars().take(8).collect()
}

fn cmd_verify(map: &HashMap<String, String>) -> ! {
    let led = Ledger { path: PathBuf::from(req(map, "ledger")) };
    let events = match led.events() {
        Ok(v) => v,
        Err(e) => err2(e),
    };
    let mut findings: Vec<String> = vec![];
    if events.is_empty() || s_field(&events[0], "kind") != "activation" {
        findings.push("首事件非 activation（T₀ 未定义即未激活）".to_string());
    }
    let act = led.activation().unwrap_or(None);
    let cut = act.as_ref().map(|a| s_field(a, "ts").to_string());
    for e in &events {
        let kind = s_field(e, "kind");
        // 围堰机械形：PREEMPT_KINDS additive 独立于 KINDS，verify 对其照报未知 kind
        // （与 preempt.py 注释"verify 不受扰"的语义宣称有差，机械行为以此为准）。
        if !KINDS.contains(&kind) {
            findings.push(format!("未知 kind {kind}"));
        }
        if let Some(c) = &cut {
            if s_field(e, "ts") < c.as_str() && kind != "activation" {
                findings.push(format!("I7 破：T₀ 前事件 {}", ref16(e)));
            }
        }
    }
    let mut bal: BTreeMap<String, i64> = BTreeMap::new();
    for e in &events {
        let kind = s_field(e, "kind");
        if let Some(c) = &cut {
            if s_field(e, "ts") < c.as_str() && kind != "activation" {
                continue;
            }
        }
        if kind == "mint_repair_foreign" || kind == "mint_self_repair" {
            findings.push(format!("休眠/禁用铸币出现在账 {}", ref16(e)));
        }
        let account = s_field(e, "account").to_string();
        let entry = bal.entry(account).or_insert(0);
        *entry += i_field(e, "amount");
        if *entry < 0 {
            findings.push(format!("I1 破：{} 回放负余额", acct8(s_field(e, "account"))));
        }
    }
    let mut pairs: Vec<(String, Vec<String>)> = vec![];
    for e in &events {
        let kind = s_field(e, "kind");
        if kind == "pay_out" || kind == "pay_in" {
            let ref_ = s_field(e, "ref").to_string();
            match pairs.iter_mut().find(|(r, _)| r == &ref_) {
                Some((_, kinds)) => kinds.push(kind.to_string()),
                None => pairs.push((ref_, vec![kind.to_string()])),
            }
        }
    }
    for (ref_, kinds) in &pairs {
        let mut sorted = kinds.clone();
        sorted.sort();
        if sorted != ["pay_in".to_string(), "pay_out".to_string()] {
            findings.push(format!(
                "I2 破：支付 {} 未配对",
                ref_.chars().take(16).collect::<String>()
            ));
        }
    }
    let mut opened: Vec<String> = vec![];
    for e in &events {
        let kind = s_field(e, "kind");
        let account = s_field(e, "account");
        if kind == "account_open" {
            if opened.iter().any(|a| a == account) {
                findings.push(format!("重复开户 {}", acct8(account)));
            }
            opened.push(account.to_string());
        } else if !account.is_empty() && kind != "activation" && !opened.iter().any(|a| a == account)
        {
            findings.push(format!("未开户账户有账 {}", acct8(account)));
        }
    }
    let code = if findings.is_empty() { 0 } else { 1 };
    emit(
        json!({
            "status": if code == 0 { "valid" } else { "invalid" },
            "findings": findings,
            "events": events.len(),
        }),
        code,
    )
}

fn cmd_replay(map: &HashMap<String, String>) -> ! {
    let led = Ledger { path: PathBuf::from(req(map, "ledger")) };
    let events = match led.events() {
        Ok(v) => v,
        Err(e) => err2(e),
    };
    let cut = match grandfather_cut(&led) {
        Ok(c) => c,
        Err(e) => err2(e),
    };
    let mut accounts: Map<String, Value> = Map::new();
    for e in &events {
        if s_field(e, "kind") == "activation" {
            continue;
        }
        if let Some(c) = &cut {
            if s_field(e, "ts") < c.as_str() {
                continue;
            }
        }
        let account = s_field(e, "account").to_string();
        let entry = accounts.entry(account).or_insert_with(|| {
            json!({"balance": 0, "seat": e.get("seat").cloned().unwrap_or(Value::Null)})
        });
        entry["balance"] = json!(entry["balance"].as_i64().unwrap_or(0) + i_field(e, "amount"));
        if let Some(seat) = e.get("seat") {
            if !seat.is_null() {
                entry["seat"] = seat.clone();
            }
        }
    }
    emit(json!({"t0": cut.map(Value::String).unwrap_or(Value::Null), "accounts": accounts}), 0)
}

fn cmd_bills(map: &HashMap<String, String>) -> ! {
    let path = PathBuf::from(req(map, "bills"));
    if !path.exists() {
        emit(json!({"error": format!("账单文件缺席 {}", path.display())}), 2);
    }
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => emit(json!({"error": format!("账单文件不可读: {}", e)}), 2),
    };
    let mut counts: BTreeMap<String, i64> = BTreeMap::new();
    let mut total = 0i64;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let e: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(err) => emit(json!({"error": format!("账单行不可解析: {}", err)}), 2),
        };
        let et = e
            .get("event_type")
            .and_then(|v| v.as_str())
            .unwrap_or("?")
            .to_string();
        *counts.entry(et).or_insert(0) += 1;
        total += e.get("bill_points").and_then(|v| v.as_i64()).unwrap_or(0);
    }
    emit(
        json!({
            "status": "readonly-summary",
            "by_event_type": counts,
            "bill_points_total": total,
            "note": "阶段一账单零追溯（A5），不入台账余额，标定数据面自此攒量",
        }),
        0,
    )
}

fn cmd_preempt(_map: &HashMap<String, String>) -> ! {
    // 抢占价格槽位态休眠（PREEMPT_PRICE = None）：休眠门恒触发，与围堰同形。
    // 环体不可达申报见头注落差一。
    emit(
        json!({
            "error": "抢占价格槽位态休眠（dormant）：三通道无出生源，标定通道触发后激活",
            "state": "dormant",
        }),
        1,
    )
}

// ---------- CLI 解析 ----------

fn req<'a>(map: &'a HashMap<String, String>, name: &str) -> &'a str {
    map.get(name)
        .map(|s| s.as_str())
        .unwrap_or_else(|| usage_fail(&format!("--{name} 必填")))
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!(
        "用法: confledger <open|activate|mint|pay|balance|verify|replay|bills|preempt> [旗标...]"
    );
    exit(2);
}

fn parse_flags(args: &[String]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].clone();
        let name = match a.strip_prefix("--") {
            Some(n) if !n.is_empty() => n.to_string(),
            _ => usage_fail(&format!("未知位置参数: {a}")),
        };
        i += 1;
        if i >= args.len() {
            usage_fail(&format!("--{name} 缺值"));
        }
        map.insert(name, args[i].clone());
        i += 1;
    }
    map
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage_fail("缺子命令");
    }
    let sub = args[0].clone();
    let map = parse_flags(&args[1..]);
    match sub.as_str() {
        "open" => {
            if let Some(s) = map.get("seat") {
                if s != "agent" && s != "human" {
                    usage_fail("--seat 只接受 agent 或 human");
                }
            }
            cmd_open(&map)
        }
        "activate" => cmd_activate(&map),
        "mint" => cmd_mint(&map),
        "pay" => cmd_pay(&map),
        "balance" => cmd_balance(&map),
        "verify" => cmd_verify(&map),
        "replay" => cmd_replay(&map),
        "bills" => cmd_bills(&map),
        "preempt" => cmd_preempt(&map),
        other => usage_fail(&format!("未知子命令: {other}")),
    }
}

fn main() {
    exit(run());
}
