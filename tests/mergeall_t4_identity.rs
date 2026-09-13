//! lease-mergeleg23-parallel 簇E T4：identity 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/identity 0.5.0（cli.py 单子命令 verify 双模式三值
//! 退出码、core.py 组件序归一身份串 v3、盐哈希与无盐核哈希、声明对表、
//! human seat 五闸）。fixture 全部 temp 自建，采集走 --inject 复演通道不采
//! 实时，哈希期望值由测试侧以同语义 SHA-256 独立复算对表。三例：正常形
//! （attest 加 verified 全 match）、拒绝形（mismatch 与 human seat 拒与越域
//! 键退出码二）、边界形（时钟偏移阈值两侧、禁网缺席、早于开机穿帮件）。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;
use sha2::{Digest, Sha256};

fn bin_identity() -> &'static str {
    env!("CARGO_BIN_EXE_identity")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_identity()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

const SALT: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
/// 2026-09-13T00:00:00Z。
const TS: &str = "2026-09-13T00:00:00+00:00";
const TS_EPOCH: i64 = 1789257600;

struct Ws {
    _guard: tempfile::TempDir,
    dir: PathBuf,
}

fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    Ws { _guard: guard, dir }
}

fn write_json(ws: &Ws, name: &str, body: &str) -> String {
    let p = ws.dir.join(name);
    fs::write(&p, body).unwrap();
    p.to_string_lossy().into_owned()
}

/// 全键齐备复演件：net_time 与 timestamp 同刻即零偏移，boottime 远古。
fn inject_json(net_time: i64, boottime: &str) -> String {
    format!(
        r#"{{"pid":"101","ppid":"10","hostname":"h1","mac":"aabbccddeeff","user":"alice","sandbox_id":"sb1","session_id":"se1","boottime":"{boottime}","parent_start":"2026-09-12T00:00:00+00:00","ancestry":"zcode>python3>myagent","net_time":"{net_time}","timestamp":"{TS}"}}"#
    )
}

fn expected_identity_string() -> String {
    "v3|pid=101|ppid=10|hostname=h1|mac=aabbccddeeff|user=alice|sandbox_id=sb1|session_id=se1|boottime=1000000|parent_start=2026-09-12T00:00:00+00:00|ancestry=zcode>python3>myagent|net_time=1789257600|timestamp=2026-09-13T00:00:00+00:00".to_string()
}

fn expected_core_string() -> String {
    "core-v1|mac=aabbccddeeff|hostname=h1|user=alice|boottime=1000000|sandbox_id=sb1|lineage=zcode>myagent".to_string()
}

// ---------- 正常形：attest 零异常、盐哈希与核哈希独立复算对表、claims 全 match verified ----------

#[test]
fn t1_normal_attest_and_claims_verified() {
    let ws = build_ws();
    let inject = write_json(&ws, "inject.json", &inject_json(TS_EPOCH, "1000000"));

    // attest 模式：复演件零异常，退出码零。
    let (code, out, err) = run(&["verify", "--inject", &inject, "--salt", SALT]);
    assert_eq!(code, 0, "attest 须 0，stdout={out} stderr={err}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["header"]["mode"], "attest");
    assert_eq!(rep["header"]["net_probe"], "external", "net_time 在场即 external");
    assert_eq!(rep["header"]["tool"]["name"], "identity");
    assert_eq!(rep["header"]["tool"]["version"], "0.5.0");
    assert_eq!(rep["anomalies"].as_array().unwrap().len(), 0);
    assert_eq!(rep["summary"]["verdict"], "attest");

    // 哈希金向量：测试侧独立复算对表（盐哈希盐串竖线身份串，核哈希无盐核串）。
    assert_eq!(rep["identity"]["string"], expected_identity_string());
    assert_eq!(
        rep["identity"]["hash"],
        sha256_hex(format!("{SALT}|{}", expected_identity_string()).as_bytes()),
        "盐哈希逐字节对表"
    );
    assert_eq!(rep["identity"]["core_components"]["lineage"], "zcode>myagent",
        "血统归一剔通用包装段 python3");
    assert_eq!(rep["identity"]["core_hash"], sha256_hex(expected_core_string().as_bytes()),
        "核哈希逐字节对表");
    assert_eq!(rep["observed"]["pid"], "101");
    assert_eq!(rep["observed"]["mac"], "aabbccddeeff");

    // verify 模式：claims 全 match 即 verified，退出码零。
    let claims = write_json(
        &ws,
        "ok.json",
        r#"{"user":"alice","hostname":"h1","mac":"aabbccddeeff"}"#,
    );
    let (code, out, _) = run(&["verify", "--inject", &inject, "--salt", SALT, "--claims", &claims]);
    assert_eq!(code, 0, "全 match 须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["header"]["mode"], "verify");
    assert_eq!(rep["summary"]["verdict"], "verified");
    assert_eq!(rep["summary"]["claimed"], 3);
    assert_eq!(rep["summary"]["matched"], 3);
    assert_eq!(rep["summary"]["mismatched"], 0);
    assert_eq!(rep["claims"]["user"]["result"], "match");
    assert_eq!(rep["claims"]["user"]["claimed"], "alice");
    assert_eq!(rep["claims"]["user"]["observed"], "alice");
}

// ---------- 拒绝形：mismatch 退出码一、human seat 拒退出码一、越域键与坏盐与缺键退出码二 ----------

#[test]
fn t2_reject_mismatch_human_seat_reject_and_input_invalid() {
    let ws = build_ws();
    let inject = write_json(&ws, "inject.json", &inject_json(TS_EPOCH, "1000000"));

    // claims 失配：在键位指认漂移变点，verdict mismatch，退出码一。
    let claims = write_json(&ws, "bad.json", r#"{"user":"bob"}"#);
    let (code, out, _) = run(&["verify", "--inject", &inject, "--salt", SALT, "--claims", &claims]);
    assert_eq!(code, 1, "失配须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"]["verdict"], "mismatch");
    assert_eq!(rep["summary"]["matched"], 0);
    assert_eq!(rep["summary"]["mismatched"], 1);
    assert_eq!(rep["claims"]["user"]["result"], "mismatch");
    assert_eq!(rep["claims"]["user"]["claimed"], "bob");

    // human seat 拒：seat 非 human 过闸二败，退出码一。
    let seat = write_json(&ws, "reject.json", r#"{"seat":"agent","name":"X","attest_at":"2026-09-13"}"#);
    let (code, out, _) = run(&[
        "verify", "--inject", &inject, "--salt", SALT, "--human-seat", &seat,
    ]);
    assert_eq!(code, 1, "human seat 拒须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["human_seat"]["verdict"], "reject");
    assert_eq!(rep["human_seat"]["gates"]["gate1_object"], true);
    assert_eq!(rep["human_seat"]["gates"]["gate2_seat"], false);
    assert_eq!(rep["summary"]["human_seat_verdict"], "reject");

    // 越域键：timestamp 不入对表域，退出码二。
    let claims = write_json(&ws, "bogus.json", r#"{"timestamp":"2026-09-13T00:00:00+00:00"}"#);
    let (code, out, _) = run(&["verify", "--inject", &inject, "--salt", SALT, "--claims", &claims]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert!(
        err["error"]
            .as_str()
            .unwrap()
            .starts_with("claim key outside comparable set:"),
        "error={}",
        err["error"]
    );

    // 盐非法：非六十四位十六进制，退出码二。
    let (code, out, _) = run(&["verify", "--inject", &inject, "--salt", "xyz"]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(err["error"], "salt must be 64 hex chars");

    // 复演件缺组件：闸外缺一即拒，退出码二。
    let partial = write_json(&ws, "partial.json", r#"{"pid":"1"}"#);
    let (code, out, _) = run(&["verify", "--inject", &partial, "--salt", SALT]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert!(
        err["error"]
            .as_str()
            .unwrap()
            .starts_with("inject missing components:"),
        "error={}",
        err["error"]
    );

    // 声明文件不可读：退出码二。
    let (code, out, _) = run(&[
        "verify",
        "--inject",
        &inject,
        "--salt",
        SALT,
        "--claims",
        "/nonexistent-t4.json",
    ]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(err["error"], "input file unreadable: /nonexistent-t4.json");
}

// ---------- 边界形：时钟偏移阈值两侧、禁网缺席、早于开机穿帮件、human seat 全闸 attest ----------

#[test]
fn t3_boundary_clock_skew_threshold_no_net_and_before_boot() {
    let ws = build_ws();

    // 偏移恰 300 秒不越阈：零异常退出码零。
    let inject = write_json(&ws, "in-edge.json", &inject_json(TS_EPOCH - 300, "1000000"));
    let (code, out, _) = run(&["verify", "--inject", &inject, "--salt", SALT]);
    assert_eq!(code, 0, "恰 300s 不越阈须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["anomalies"].as_array().unwrap().len(), 0);

    // 偏移 301 秒越阈：clock_skew 异常，退出码一。
    let inject = write_json(&ws, "in-skew.json", &inject_json(TS_EPOCH - 301, "1000000"));
    let (code, out, _) = run(&["verify", "--inject", &inject, "--salt", SALT]);
    assert_eq!(code, 1, "越阈须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    let anomalies = rep["anomalies"].as_array().unwrap();
    assert_eq!(anomalies.len(), 1);
    assert_eq!(anomalies[0]["kind"], "clock_skew");

    // 早于开机穿帮件：timestamp 早于 boottime 即 timestamp_before_boottime。
    let inject = write_json(&ws, "in-boot.json", &inject_json(TS_EPOCH, "1789257900"));
    let (code, out, _) = run(&["verify", "--inject", &inject, "--salt", SALT]);
    assert_eq!(code, 1, "穿帮件须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    let anomalies = rep["anomalies"].as_array().unwrap();
    assert_eq!(anomalies.len(), 1);
    assert_eq!(anomalies[0]["kind"], "timestamp_before_boottime");

    // 禁网缺席：live 采集跳过外部钟探针，net_probe 记 disabled，core_hash 不受影响。
    let (code, out, _) = run(&["verify", "--no-net", "--salt", SALT, "--quiet"]);
    assert_eq!(code, 0, "禁网零异常须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["tool"], "identity");
    assert_eq!(rep["command"], "verify");
    assert_eq!(rep["code"], 0);
    assert_eq!(rep["summary"]["status"], "attest");
    assert!(rep["summary"]["identity_hash"].is_string());

    // 禁网全形态报告：net_probe disabled 与核哈希在场。
    let (code, out, _) = run(&["verify", "--no-net", "--salt", SALT]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["header"]["net_probe"], "disabled");
    assert!(rep["identity"]["core_hash"].is_string());
    assert!(rep["identity"]["hash"].is_string());

    // human seat 全闸：闸一对象闸二 human 闸三必填全过，闸四签字闸五指纹只记状态。
    let inject = write_json(&ws, "in-seat.json", &inject_json(TS_EPOCH, "1000000"));
    let seat = write_json(
        &ws,
        "seat.json",
        r#"{"seat":"human","name":"操作人","attest_at":"2026-09-13","guardian":"G","fingerprint":{"keyboard_layout":"colemak"}}"#,
    );
    let (code, out, _) = run(&[
        "verify", "--inject", &inject, "--salt", SALT, "--human-seat", &seat, "--quiet",
    ]);
    assert_eq!(code, 0, "human seat 全闸须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"]["human_seat_verdict"], "attest");
    let (code, out, _) = run(&[
        "verify", "--inject", &inject, "--salt", SALT, "--human-seat", &seat,
    ]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    let gates = &rep["human_seat"]["gates"];
    assert_eq!(gates["gate1_object"], true);
    assert_eq!(gates["gate2_seat"], true);
    assert_eq!(gates["gate3_required_fields"], true);
    assert_eq!(gates["gate4_guardian_signed"], true);
    assert_eq!(gates["gate5_fingerprint_registered"], true);
    assert_eq!(rep["human_seat"]["verdict"], "attest");
}
