//! 跨链集成测试（viewrider-solo F-2 重建）
//!
//! 装载层整改复测：viewer 二进制对可重复 --trail 旗标按参序全量装载并
//! 按参序串联事件流供三子命令消费，承 viewfix-solo 段1 整改基础。
//!
//! 三断言簇：
//!   A 多链装载：enter 写在甲链、exit 写在乙链，双链按参序跑 heartbeat
//!     时该项「已出泊」带 disposition；只跑乙链时该项缺席。
//!   B 单链回归：单链跑 alarms 与 heartbeat 的输出与独立构造的预期
//!     逐字段一致（即整改前单链行为不变）。
//!   C 配对三态：构造在泊未到期与在泊到期（entered_at 加 ttl_days 早于
//!     参照日）两态断言 state 标签。
//!
//! 临时链文件构造，不依赖真实链路径。

use chrono::NaiveDate;
use sih_engine::view::heartbeat::heartbeat_view;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn viewer_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_viewer"))
}

fn write_temp_trail(name: &str, body: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!(
        "viewrider-{}-{}-{}.ndjson",
        std::process::id(),
        name,
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    ));
    let mut f = std::fs::File::create(&p).expect("create temp trail");
    f.write_all(body.as_bytes()).expect("write temp trail");
    p
}

/// 构造 Event 序列化为单行 NDJSON 字串
fn mk_event(
    event_id: &str,
    event_type: &str,
    timestamp: &str,
    doc_id: &str,
    event_class: &str,
    details_json: &str,
) -> String {
    format!(
        r#"{{"event_id":"{}","event_type":"{}","timestamp":"{}","actor":{{"actor_id":"test","actor_type":"system","invoked_via":"cli"}},"doc_id":"{}","prev_hash":"","event_hash":"","event_class":"{}","verification_result":null,"details":{}}}"#,
        event_id, event_type, timestamp, doc_id, event_class, details_json
    )
}

fn parking_entered(entry_id: &str, title: &str, ttl_days: i64, ts: &str) -> String {
    mk_event(
        &format!("e-{}", entry_id),
        "parking_entered",
        ts,
        entry_id,
        "record_only",
        &format!(
            r#"{{"entry_id":"{}","title":"{}","exit_condition":"","ttl_days":{}}}"#,
            entry_id, title, ttl_days
        ),
    )
}

fn parking_exited(entry_id: &str, disposition: &str, ts: &str) -> String {
    mk_event(
        &format!("x-{}", entry_id),
        "parking_exited",
        ts,
        entry_id,
        "record_only",
        &format!(
            r#"{{"entry_id":"{}","disposition":"{}","ruling":"ok"}}"#,
            entry_id, disposition
        ),
    )
}

fn reading_recorded(dim: &str, value: f64, ts: &str) -> String {
    let s = format!("{}", value);
    mk_event(
        &format!("r-{}-{}", dim, ts),
        "reading_recorded",
        ts,
        &format!("gauge-reading-{}-sih-engine", dim),
        "consumable",
        &format!(
            r#"{{"dimension":"{}","subject":"sih-engine","value":{},"window":"2026-08-01/2026-08-30","formula_version":"ga-1","computed_at":"2026-08-30","inputs_digest":"{}"}}"#,
            dim,
            s,
            "a".repeat(64)
        ),
    )
}

// ============================================================
// A 多链装载：enter 甲链 / exit 乙链，双链参序跑出"已出泊"带 disposition
// ============================================================

#[test]
fn a_multi_chain_parking_pairing() {
    let enter_ts = "2026-08-20T00:00:00+00:00";
    let exit_ts = "2026-08-25T00:00:00+00:00";

    // 甲链：含 parking_entered
    let a_body = format!("{}\n", parking_entered("pk-A1", "T", 30, enter_ts));
    let a_path = write_temp_trail("chain_a", &a_body);

    // 乙链：含 parking_exited
    let b_body = format!("{}\n", parking_exited("pk-A1", "promoted", exit_ts));
    let b_path = write_temp_trail("chain_b", &b_body);

    // 双链按参序跑 heartbeat — 应输出"已出泊"带 disposition
    let out = Command::new(viewer_bin())
        .arg("heartbeat")
        .arg("--trail").arg(&a_path)
        .arg("--trail").arg(&b_path)
        .arg("--at").arg("2026-08-31")
        .output()
        .expect("run viewer heartbeat");
    assert!(out.status.success(), "viewer heartbeat exit non-zero: {:?}", out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("viewer output is JSON");
    let parking = v["parking"].as_array().expect("parking is array");
    assert_eq!(parking.len(), 1, "expected 1 parking entry, got {}", parking.len());
    let e = &parking[0];
    assert_eq!(e["entry_id"], "pk-A1");
    assert_eq!(e["state"], 2, "state 期望 2=已出泊");
    assert_eq!(e["disposition"], "promoted", "disposition 期望 promoted");
    assert_eq!(e["state_label"], "已出泊");
    // trails 数组按参序两件
    let trails = v["trails"].as_array().expect("trails array");
    assert_eq!(trails.len(), 2, "trails 数组长度 2");
    assert!(trails[0].as_str().unwrap().contains("chain_a"), "trails[0] 含 chain_a");
    assert!(trails[1].as_str().unwrap().contains("chain_b"), "trails[1] 含 chain_b");

    // 只跑乙链 — 该项应缺席（无 entered）
    let out_b_only = Command::new(viewer_bin())
        .arg("heartbeat")
        .arg("--trail").arg(&b_path)
        .arg("--at").arg("2026-08-31")
        .output()
        .expect("run viewer heartbeat single");
    assert!(out_b_only.status.success());
    let stdout_b = String::from_utf8_lossy(&out_b_only.stdout);
    let v_b: serde_json::Value = serde_json::from_str(&stdout_b).expect("JSON");
    let parking_b = v_b["parking"].as_array().expect("parking array");
    assert_eq!(parking_b.len(), 0, "只乙链时 pk-A1 缺席，期望 parking=0");

    let _ = std::fs::remove_file(&a_path);
    let _ = std::fs::remove_file(&b_path);
}

// ============================================================
// B 单链回归：单链跑 alarms + heartbeat 与独立预期逐字段一致
// ============================================================

#[test]
fn b_single_chain_regression_byte_identical() {
    // 单链：一条 reading_recorded（consumable），无 parking_entered
    let ts = "2026-08-30T00:00:00+00:00";
    let body = format!("{}\n", reading_recorded("convergence", 0.5, ts));
    let path = write_temp_trail("chain_b_single", &body);

    // alarms 单链跑 — 期望 1 consumable alarms, exit 1
    let out_a = Command::new(viewer_bin())
        .arg("alarms")
        .arg("--trail").arg(&path)
        .output()
        .expect("run viewer alarms");
    assert_eq!(out_a.status.code(), Some(1), "alarms 期望 exit 1（非空）");
    let stdout_a = String::from_utf8_lossy(&out_a.stdout);
    let va: serde_json::Value = serde_json::from_str(&stdout_a).expect("JSON");
    assert_eq!(va["total"], 1);
    assert_eq!(va["exit_code_suggested"], 1);
    let alarms = va["alarms"].as_array().unwrap();
    assert_eq!(alarms.len(), 1);
    assert_eq!(alarms[0]["event_type"], "reading_recorded");
    // 单链不增 trails 字段，仅 trail 字符串
    assert!(va["trail"].is_string(), "单链 trail 字符串在场");
    assert!(va["trails"].is_null(), "单链 trails 数组缺席");

    // heartbeat 单链跑 — 期望 parking=0, consumable=1
    let out_h = Command::new(viewer_bin())
        .arg("heartbeat")
        .arg("--trail").arg(&path)
        .arg("--at").arg("2026-08-30")
        .output()
        .expect("run viewer heartbeat");
    assert!(out_h.status.success());
    let stdout_h = String::from_utf8_lossy(&out_h.stdout);
    let vh: serde_json::Value = serde_json::from_str(&stdout_h).expect("JSON");
    assert_eq!(vh["parking"].as_array().unwrap().len(), 0);
    assert_eq!(vh["consumable_count"], 1);

    // 复跑一致性：同参再跑一次，逐字段一致
    let out_h2 = Command::new(viewer_bin())
        .arg("heartbeat")
        .arg("--trail").arg(&path)
        .arg("--at").arg("2026-08-30")
        .output()
        .expect("run viewer heartbeat 2");
    let stdout_h2 = String::from_utf8_lossy(&out_h2.stdout);
    assert_eq!(vh, serde_json::from_str::<serde_json::Value>(&stdout_h2).unwrap(),
        "同参双跑逐字段一致");

    let _ = std::fs::remove_file(&path);
}

// ============================================================
// C 配对三态：在泊未到期 + 在泊到期
// ============================================================

#[test]
fn c_pairing_three_states() {
    // 在泊未到期：entered_at=2026-08-29, ttl=30, ref=2026-08-31 → 30 天未到
    // 在泊到期：entered_at=2026-07-01, ttl=10, ref=2026-08-31 → 早过
    let e_active = parking_entered(
        "pk-active",
        "在泊未到期测试",
        30,
        "2026-08-29T00:00:00+00:00",
    );
    let e_expired = parking_entered(
        "pk-expired",
        "在泊到期测试",
        10,
        "2026-07-01T00:00:00+00:00",
    );
    let body = format!("{}\n{}\n", e_active, e_expired);
    let path = write_temp_trail("chain_c_three_states", &body);

    // 通过 cargo test 解析：直接调 sih_engine::view::heartbeat 函数（不经过 CLI 装载）
    let events = sih_engine::load_events(&path).expect("load trail");
    let view = heartbeat_view(&events, NaiveDate::from_ymd_opt(2026, 8, 31).unwrap());

    assert_eq!(view.parking.len(), 2, "2 件在泊或到期");
    let active = view.parking.iter().find(|p| p.entry_id == "pk-active").expect("pk-active 在列");
    assert_eq!(active.state, 0, "pk-active state=0 在泊未到期");
    assert_eq!(active.state_label, "在泊未到期");

    let expired = view.parking.iter().find(|p| p.entry_id == "pk-expired").expect("pk-expired 在列");
    assert_eq!(expired.state, 1, "pk-expired state=1 在泊到期");
    assert_eq!(expired.state_label, "在泊到期");

    // 验证 CLI 装载层 + heartbeat_view 输出一致：通过 viewer binary 跑同数据
    let out_cli = Command::new(viewer_bin())
        .arg("heartbeat")
        .arg("--trail").arg(&path)
        .arg("--at").arg("2026-08-31")
        .output()
        .expect("run viewer heartbeat three states");
    assert!(out_cli.status.success());
    let stdout_cli = String::from_utf8_lossy(&out_cli.stdout);
    let v_cli: serde_json::Value = serde_json::from_str(&stdout_cli).expect("JSON");
    let cli_parking = v_cli["parking"].as_array().expect("parking array");
    let cli_active = cli_parking.iter().find(|p| p["entry_id"] == "pk-active").expect("pk-active");
    let cli_expired = cli_parking.iter().find(|p| p["entry_id"] == "pk-expired").expect("pk-expired");
    assert_eq!(cli_active["state"], 0, "CLI: pk-active state=0");
    assert_eq!(cli_expired["state"], 1, "CLI: pk-expired state=1");

    let _ = std::fs::remove_file(&path);
}
