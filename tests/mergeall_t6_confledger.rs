//! lease-mergeleg6-parallel 簇I T6：confledger 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/confledger（cli.py、ledger.py、constants.py）。
//! 三例金向量：正常（activate+open+mint+pay 全链守恒）、拒绝（休眠门与
//! fail-closed 各拒绝腿）、边界（grandfather 截断与 verify 破检）。fixture
//! 全部 temp 自建。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_confledger")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn run_json(args: &[&str]) -> (i32, Value, String) {
    let (code, out, err) = run(args);
    (code, serde_json::from_str(&out).unwrap_or(Value::Null), err)
}

struct Fx {
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    Fx { dir: guard.path().to_path_buf(), _guard: guard }
}

impl Fx {
    fn ledger(&self, name: &str) -> String {
        self.dir.join(name).display().to_string()
    }

    /// 正身件：core_hash 与 identity.hash 双哈希形。
    fn identity_report(&self, name: &str, core: &str) -> String {
        let path = self.dir.join(name);
        fs::write(
            &path,
            serde_json::json!({
                "core_hash": core,
                "identity": {"hash": format!("id-{core}")},
                "seat": "agent",
            })
            .to_string(),
        )
        .unwrap();
        path.display().to_string()
    }
}

const A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

// ---------- 正常形：全链 open→mint→pay 守恒与 verify/replay 投影 ----------

#[test]
fn t1_normal_full_chain() {
    let fx = build_fx();
    let led = fx.ledger("led.ndjson");
    let ra = fx.identity_report("a.json", A);
    let rb = fx.identity_report("b.json", B);
    let trail = fx.dir.join("trail.ndjson");
    let mint_ref = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    fs::write(&trail, format!("noise line\nprior {mint_ref} in witness\n")).unwrap();

    // activate：空账本首事件
    let (code, v, err) = run_json(&[
        "activate", "--ledger", &led, "--ref", "r0-anchor",
        "--ts", "2026-09-01T00:00:00+00:00", "--identity-report", &ra,
    ]);
    assert_eq!(code, 0, "activate 须 0，err={err}");
    assert_eq!(v["status"], "activated");
    assert_eq!(v["t0_ts"], "2026-09-01T00:00:00+00:00");
    assert_eq!(v["t0_ref"], "r0-anchor");

    // open 双户
    let (code, v, _) = run_json(&["open", "--ledger", &led, "--identity-report", &ra]);
    assert_eq!(code, 0);
    assert_eq!(v["status"], "opened");
    assert_eq!(v["account"], A);
    assert_eq!(v["seat"], "agent");
    let (code, v, _) = run_json(&["open", "--ledger", &led, "--identity-report", &rb]);
    assert_eq!(code, 0);
    assert_eq!(v["account"], B);

    // mint：ref 须在 trail 见证
    let (code, v, _) = run_json(&[
        "mint", "--ledger", &led, "--account", A, "--kind", "mint_clean_close",
        "--ref", mint_ref, "--trail", trail.display().to_string().as_str(),
    ]);
    assert_eq!(code, 0, "mint 须 0");
    assert_eq!(v["status"], "minted");
    assert_eq!(v["kind"], "mint_clean_close");
    assert_eq!(v["amount"], 1, "m_clean = 1 单位锚");
    assert_eq!(v["account"], A);

    // pay：A → B 1 点，即付不退注记
    let (code, v, _) = run_json(&[
        "pay", "--ledger", &led, "--account", A, "--to", B, "--amount", "1",
    ]);
    assert_eq!(code, 0);
    assert_eq!(v["status"], "paid");
    assert_eq!(v["amount"], 1);
    assert_eq!(v["from"], A);
    assert_eq!(v["to"], B);
    assert!(v["ref"].as_str().unwrap().starts_with("pay-"), "pay ref 形");
    assert!(v["note"].as_str().unwrap().contains("A4"), "即付不退注记");

    // balance 单户与全户
    let (code, v, _) = run_json(&["balance", "--ledger", &led, "--account", A]);
    assert_eq!(code, 0);
    assert_eq!(v["balance"], 0, "mint 1 - pay 1 = 0");
    assert_eq!(v["grandfather_excluded"], 0);
    let (code, v, _) = run_json(&["balance", "--ledger", &led]);
    assert_eq!(code, 0);
    assert_eq!(v["accounts"][A]["balance"], 0);
    assert_eq!(v["accounts"][B]["balance"], 1);

    // verify：六事件全绿
    let (code, v, _) = run_json(&["verify", "--ledger", &led]);
    assert_eq!(code, 0, "verify 须 valid");
    assert_eq!(v["status"], "valid");
    assert_eq!(v["events"], 6, "activation+2open+mint+pay_out+pay_in");
    assert_eq!(v["findings"].as_array().unwrap().len(), 0);

    // replay：t0 与余额投影
    let (code, v, _) = run_json(&["replay", "--ledger", &led]);
    assert_eq!(code, 0);
    assert_eq!(v["t0"], "2026-09-01T00:00:00+00:00");
    assert_eq!(v["accounts"][A]["balance"], 0);
    assert_eq!(v["accounts"][A]["seat"], "agent");
    assert_eq!(v["accounts"][B]["balance"], 1);

    // 台账行形：sort_keys 缺省分隔形（", "/": "），account 键排首
    let text = fs::read_to_string(&led).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 6);
    let first: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(first["kind"], "activation");
    assert!(lines[0].starts_with("{\"account\":"), "sort_keys 首键 account");
    assert!(lines[0].contains("\": "), "缺省分隔含空格形");
    for line in &lines {
        let row: Value = serde_json::from_str(line).unwrap();
        let mut keys: Vec<&String> = row.as_object().unwrap().keys().collect();
        let sorted = {
            keys.sort();
            keys.clone()
        };
        assert_eq!(row.as_object().unwrap().keys().collect::<Vec<_>>(), sorted);
    }
}

// ---------- 拒绝形：休眠门、fail-closed、D10 与工具异常 ----------

#[test]
fn t2_reject_forms() {
    let fx = build_fx();
    let led = fx.ledger("led.ndjson");
    let ra = fx.identity_report("a.json", A);
    let rb = fx.identity_report("b.json", B);
    let rc_ = fx.identity_report("c.json", C);

    // 激活与双户（A 有 1 点余额）
    run(&["activate", "--ledger", &led, "--ref", "r0", "--ts", "2026-09-01T00:00:00+00:00", "--identity-report", &ra]);
    run(&["open", "--ledger", &led, "--identity-report", &ra]);
    run(&["open", "--ledger", &led, "--identity-report", &rb]);

    // 重复开户 → 1
    let (code, v, _) = run_json(&["open", "--ledger", &led, "--identity-report", &ra]);
    assert_eq!(code, 1);
    assert_eq!(v["error"], "账户已开户");

    // 余额不足 fail-closed（D3）→ 1
    let (code, v, _) = run_json(&[
        "pay", "--ledger", &led, "--account", A, "--to", B, "--amount", "5",
    ]);
    assert_eq!(code, 1);
    assert!(v["error"].as_str().unwrap().contains("余额不足"));
    assert_eq!(v["balance"], 0);
    assert_eq!(v["requested"], 5);

    // 人席位无强制算子（D10）→ 1
    run(&["open", "--ledger", &led, "--identity-report", &rc_, "--seat", "human"]);
    let (code, v, _) = run_json(&[
        "pay", "--ledger", &led, "--account", C, "--to", A, "--amount", "1",
    ]);
    assert_eq!(code, 1);
    assert!(v["error"].as_str().unwrap().contains("D10"));

    // mint 未知 kind → 1
    let (code, v, _) = run_json(&[
        "mint", "--ledger", &led, "--account", A, "--kind", "mint_no_such", "--ref", "x",
    ]);
    assert_eq!(code, 1);
    assert!(v["error"].as_str().unwrap().contains("未知 kind"));

    // mint 休眠槽位（m_repair）与禁用（自产自修）→ 1
    for kind in ["mint_repair_foreign", "mint_self_repair"] {
        let (code, v, _) = run_json(&[
            "mint", "--ledger", &led, "--account", A, "--kind", kind, "--ref", "x",
        ]);
        assert_eq!(code, 1, "{kind} 须拒");
        if kind == "mint_repair_foreign" {
            assert_eq!(v["state"], "dormant");
        }
    }

    // mint 未开户 → 1；ref 无 trail 见证 → 1
    let (code, _, _) = run_json(&[
        "mint", "--ledger", &led, "--account", "dddd", "--kind", "mint_clean_close", "--ref", "x",
    ]);
    assert_eq!(code, 1);
    let (code, v, _) = run_json(&[
        "mint", "--ledger", &led, "--account", A, "--kind", "mint_clean_close", "--ref", "absent",
    ]);
    assert_eq!(code, 1);
    assert!(v["error"].as_str().unwrap().contains("见证"));

    // activate 复激活 → 1
    let (code, v, _) = run_json(&[
        "activate", "--ledger", &led, "--ref", "r9", "--ts", "2026-09-02T00:00:00+00:00",
        "--identity-report", &ra,
    ]);
    assert_eq!(code, 1);
    assert!(v["error"].as_str().unwrap().contains("activation 只能是首事件"));

    // verify 破检：孤立 pay_out（I2 未配对 + 未开户 + 首事件非 activation）
    let bad = fx.ledger("bad.ndjson");
    fs::write(&bad, format!(
        "{{\"ts\": \"2026-01-01T00:00:00+00:00\", \"kind\": \"pay_out\", \"account\": \"{A}\", \"amount\": -3, \"ref\": \"p1\", \"identity_hash\": \"0\"}}\n"
    )).unwrap();
    let (code, v, _) = run_json(&["verify", "--ledger", &bad]);
    assert_eq!(code, 1, "verify invalid 须 1");
    assert_eq!(v["status"], "invalid");
    let findings: Vec<&str> = v["findings"].as_array().unwrap().iter().map(|f| f.as_str().unwrap()).collect();
    assert!(findings.iter().any(|f| f.contains("首事件非 activation")));
    assert!(findings.iter().any(|f| f.contains("I2 破")));
    assert!(findings.iter().any(|f| f.contains("未开户账户有账")));

    // 工具异常腿：正身件缺席 → 2；bills 缺席 → 2
    let (code, _, _) = run_json(&["open", "--ledger", &led, "--identity-report", "/no/such/report.json"]);
    assert_eq!(code, 2);
    let (code, v, _) = run_json(&["bills", "--bills", "/no/such/bills.ndjson"]);
    assert_eq!(code, 2);
    assert!(v["error"].as_str().unwrap().contains("账单文件缺席"));

    // preempt 休眠门恒触发（PREEMPT_PRICE 槽位态 None）→ 1
    let (code, v, _) = run_json(&[
        "preempt", "--ledger", &led, "--account", A, "--target", B, "--ref", "x",
    ]);
    assert_eq!(code, 1);
    assert_eq!(v["state"], "dormant");

    // 用法错：未知子命令与缺必填 → 2
    let (code, _, err) = run(&["no-such-sub"]);
    assert_eq!(code, 2);
    assert!(err.contains("用法错"));
    let (code, _, _) = run(&["pay", "--ledger", &led]);
    assert_eq!(code, 2);
}

// ---------- 边界形：grandfather 截断、零额支付、空账投影、账单只读消费 ----------

#[test]
fn t3_edge_grandfather_and_bills() {
    let fx = build_fx();

    // 手写账本：T₀ 前铸币被 grandfather 排除（D11）
    let led = fx.ledger("gf.ndjson");
    fs::write(&led, format!(
        "{{\"account\": \"\", \"amount\": 0, \"identity_hash\": \"0\", \"kind\": \"activation\", \"ref\": \"r0\", \"ts\": \"2026-01-01T00:00:00+00:00\"}}\n\
         {{\"account\": \"{A}\", \"amount\": 100, \"identity_hash\": \"0\", \"kind\": \"mint_clean_close\", \"ref\": \"old\", \"ts\": \"2025-06-01T00:00:00+00:00\"}}\n\
         {{\"account\": \"{A}\", \"amount\": 0, \"identity_hash\": \"idA\", \"kind\": \"account_open\", \"ref\": \"oa\", \"seat\": \"agent\", \"ts\": \"2026-02-01T00:00:00+00:00\"}}\n\
         {{\"account\": \"{A}\", \"amount\": 7, \"identity_hash\": \"0\", \"kind\": \"mint_active_pen\", \"ref\": \"m7\", \"ts\": \"2026-03-01T00:00:00+00:00\"}}\n"
    )).unwrap();
    let (code, v, _) = run_json(&["balance", "--ledger", &led, "--account", A]);
    assert_eq!(code, 0);
    assert_eq!(v["balance"], 7, "T₀ 前的 100 点被 grandfather 截断");
    assert_eq!(v["grandfather_excluded"], 1);

    // verify 同账本：I7（T₀ 前事件）与休眠铸币出现照报
    let (code, v, _) = run_json(&["verify", "--ledger", &led]);
    assert_eq!(code, 1);
    let findings: Vec<String> = v["findings"].as_array().unwrap().iter().map(|f| f.as_str().unwrap().to_string()).collect();
    assert!(findings.iter().any(|f| f.contains("I7 破")), "findings={findings:?}");

    // 零额与负额支付拒绝
    let led2 = fx.ledger("pay2.ndjson");
    let ra = fx.identity_report("a.json", A);
    let rb = fx.identity_report("b.json", B);
    run(&["activate", "--ledger", &led2, "--ref", "r0", "--ts", "2026-09-01T00:00:00+00:00", "--identity-report", &ra]);
    run(&["open", "--ledger", &led2, "--identity-report", &ra]);
    run(&["open", "--ledger", &led2, "--identity-report", &rb]);
    for amount in ["0", "-1"] {
        let (code, v, _) = run_json(&[
            "pay", "--ledger", &led2, "--account", A, "--to", B, "--amount", amount,
        ]);
        assert_eq!(code, 1, "amount {amount} 须拒");
        assert!(v["error"].as_str().unwrap().contains("正整数"));
    }

    // 未开户账户 balance 恒零不拒（围堰同形）
    let (code, v, _) = run_json(&["balance", "--ledger", &led2, "--account", "nope"]);
    assert_eq!(code, 0);
    assert_eq!(v["balance"], 0);
    assert_eq!(v["grandfather_excluded"], 0);

    // 空账本 replay：t0 null 零账户
    let empty = fx.ledger("empty.ndjson");
    let (code, v, _) = run_json(&["replay", "--ledger", &empty]);
    assert_eq!(code, 0);
    assert!(v["t0"].is_null());
    assert_eq!(v["accounts"].as_object().unwrap().len(), 0);

    // 空账本 verify：首事件非 activation 即 invalid
    let (code, v, _) = run_json(&["verify", "--ledger", &empty]);
    assert_eq!(code, 1);
    assert!(v["findings"].as_array().unwrap()[0]
        .as_str()
        .unwrap()
        .contains("首事件非 activation"));

    // bills 只读消费：空行跳过、缺 bill_points 记零、逐型计数
    let bills = fx.dir.join("bills.ndjson");
    fs::write(&bills, concat!(
        "{\"event_type\": \"lease_open\", \"bill_points\": 2}\n",
        "\n",
        "{\"event_type\": \"lease_open\"}\n",
        "{\"event_type\": \"lease_commit\", \"bill_points\": 5}\n",
    )).unwrap();
    let (code, v, _) = run_json(&["bills", "--bills", bills.display().to_string().as_str()]);
    assert_eq!(code, 0);
    assert_eq!(v["status"], "readonly-summary");
    assert_eq!(v["by_event_type"]["lease_open"], 2);
    assert_eq!(v["by_event_type"]["lease_commit"], 1);
    assert_eq!(v["bill_points_total"], 7);
}
