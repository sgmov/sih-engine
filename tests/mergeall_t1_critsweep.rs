//! lease-mergeall-parallel 簇A T1：critsweep 引擎 bin 金向量三例。
//! 正常形：活动判据三态判定与证据判据 achieved 与在飞面与散文对照；
//! 拒绝形：裁判面封印（非缺省阈值缺事由即拒，exit 1）；
//! 边界形：零命中 unknown 形与 trail 目录缺席降级与越窗事由登记。

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

fn write_file(p: &Path, s: &str) {
    if let Some(d) = p.parent() {
        std::fs::create_dir_all(d).unwrap();
    }
    std::fs::write(p, s).unwrap();
}

struct Ws {
    root: PathBuf,
    registry: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_ws(with_trail: bool) -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let root = guard.path().to_path_buf();
    let registry = root.join("registry.json");
    if with_trail {
        let trail_dir = root.join("sih-engine/sih/event/trail");
        write_file(
            &trail_dir.join("2026-09-13.ndjson"),
            "{\"event_id\":\"e1\",\"event_type\":\"certification_completed\",\"details\":{\"record_path\":\"plan/mergeall-parallel/report.md\"}}\n\
             {\"event_id\":\"e2\",\"event_type\":\"note\",\"details\":{\"note\":\"散文提及 mergeall 属设计引用非程序活动\"}}\n",
        );
        write_file(
            &root.join("sih-tools/lease/ledger/sessions.ndjson"),
            "{\"session_id\":\"s1\",\"event\":\"issued\"}\n",
        );
        write_file(
            &root.join("sih-tools/lease/ledger/locks.ndjson"),
            "{\"session_id\":\"s1\",\"path\":\"p\",\"event\":\"acquired\"}\n",
        );
        write_file(&root.join("evidence.md"), "构造性对齐 达标\n");
        write_file(
            &registry,
            r#"{"criteria":[
              {"id":"T-ACT","title":"活动判据","status_kind":"activity","token_scope":["mergeall"],"registered_at":"2026-09-10"},
              {"id":"T-EV","title":"证据判据","status_kind":"evidence","token_scope":["evtoken"],"registered_at":"2026-09-01",
               "evidence":{"files":[{"path":"evidence.md","contains":"达标"}],"chain":[]}}
            ]}"#,
        );
    } else {
        write_file(
            &registry,
            r#"{"criteria":[{"id":"T-UNK","title":"无登记日判据","status_kind":"activity","token_scope":["neverhit"]}]}"#,
        );
    }
    Ws { root, registry, _guard: guard }
}

fn run(args: &[&str]) -> (i32, Value) {
    let out = Command::new(env!("CARGO_BIN_EXE_critsweep"))
        .args(args)
        .output()
        .unwrap();
    let rc = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let v: Value = serde_json::from_str(&stdout).expect("出参须为严格 JSON 单对象");
    (rc, v)
}

fn jv(v: &Value) -> Value {
    v.clone()
}

#[test]
fn t1_normal_criteria_and_faces() {
    let ws = build_ws(true);
    let (rc, out) = run(&[
        "--at",
        "2026-09-13",
        "--root",
        ws.root.to_str().unwrap(),
        "--registry",
        ws.registry.to_str().unwrap(),
    ]);
    assert_eq!(rc, 0, "回算成功退出码零：{}", out);
    assert_eq!(jv(&out["tool"]), "critsweep");
    assert_eq!(jv(&out["at"]), "2026-09-13");
    assert_eq!(jv(&out["root_form"]), "unknown");
    assert_eq!(
        out["namespace_fields"],
        serde_json::json!(["record_path", "report_path", "package"])
    );
    assert_eq!(out["window"], serde_json::json!(["2026-09-04", "2026-09-13"]));

    // 活动判据：命名空间命中在 at 当日即 gap=0，阈值三日内在飞
    let c0 = &out["criteria"][0];
    assert_eq!(jv(&c0["id"]), "T-ACT");
    assert_eq!(jv(&c0["status"]), "in_flight");
    assert_eq!(jv(&c0["status_kind"]), "activity");
    assert_eq!(jv(&c0["gap_days"]), 0);
    assert_eq!(jv(&c0["gap_basis"]), "last_hit");
    assert_eq!(jv(&c0["last_hit"]), "2026-09-13");
    assert_eq!(jv(&c0["namespace_hits"]), 1);
    assert_eq!(
        c0["hit_days"]["mergeall"],
        serde_json::json!(["2026-09-13"])
    );
    // 散文对照面：e2 全文含 mergeall 而命名空间零命中，列对照不计数
    assert_eq!(c0["prose_only_control"][0].as_array().unwrap().len(), 1);

    // 证据判据：文件指纹在档即 achieved
    let c1 = &out["criteria"][1];
    assert_eq!(jv(&c1["status"]), "achieved");
    assert_eq!(jv(&c1["status_kind"]), "evidence");
    assert_eq!(jv(&c1["expected"]), "achieved");
    assert_eq!(jv(&c1["evidence"][0]["ok"]), true);

    assert_eq!(jv(&out["trail"]["events_scanned"]), 2, "e1 与 e2 两事件入扫");
    assert_eq!(jv(&out["inflight"]["active_sessions"]), 1);
    assert_eq!(jv(&out["inflight"]["held_locks"]), 1);
    // fixture 无 selector 工地：泊界降级如实，整体 degraded 可见
    assert_eq!(jv(&out["parking"]["engine"]["route"]), "degraded");
    assert_eq!(jv(&out["degraded"]), true);
    assert_eq!(jv(&out["threshold_registered"]), false);
    assert_eq!(jv(&out["threshold_days"]), 3);
}

#[test]
fn t2_reject_threshold_seal() {
    let ws = build_ws(true);
    let out = Command::new(env!("CARGO_BIN_EXE_critsweep"))
        .args([
            "--at",
            "2026-09-13",
            "--root",
            ws.root.to_str().unwrap(),
            "--registry",
            ws.registry.to_str().unwrap(),
            "--threshold",
            "7",
        ])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1), "裁判面封印即执法拦截退出码一");
    let v: Value = serde_json::from_str(&String::from_utf8_lossy(&out.stdout)).unwrap();
    assert!(
        v.get("error").is_some(),
        "error 键在档：{}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert_eq!(jv(&v["reason_code"]), "threshold_reason_missing");
    assert_eq!(jv(&v["tool"]), "critsweep");
    // 封印拒发生在登记前：referee 台账零写入
    assert!(!ws.root.join("referee.ndjson").exists());
}

#[test]
fn t3_edge_zero_hit_and_degradation() {
    let ws = build_ws(false);
    let referee = ws.root.join("referee.ndjson");
    let (rc, out) = run(&[
        "--at",
        "2026-09-13",
        "--root",
        ws.root.to_str().unwrap(),
        "--registry",
        ws.registry.to_str().unwrap(),
        "--criterion",
        "T-UNK",
        "--threshold",
        "5",
        "--threshold-reason",
        "边界重放显式给参",
        "--referee-ledger",
        referee.to_str().unwrap(),
    ]);
    assert_eq!(rc, 0, "边界形退出码仍零：{}", out);
    // 零命中且无 registered_at：unknown 如实零虚构
    let c0 = &out["criteria"][0];
    assert_eq!(jv(&c0["id"]), "T-UNK");
    assert_eq!(jv(&c0["status"]), "unknown");
    assert!(c0["gap_days"].is_null(), "gap 不可估即 null 零虚构");
    assert_eq!(jv(&c0["gap_basis"]), "unavailable");
    assert_eq!(jv(&c0["namespace_hits"]), 0);
    // trail 目录缺席：全窗缺文件日与降级可见
    assert_eq!(jv(&out["trail"]["events_scanned"]), 0);
    assert_eq!(out["trail"]["missing_days"].as_array().unwrap().len(), 10);
    assert_eq!(jv(&out["degraded"]), true);
    // 越窗事由登记：referee 台账一行在档且 threshold_registered 出参明示
    assert_eq!(jv(&out["threshold_registered"]), true);
    let row_text = std::fs::read_to_string(&referee).unwrap();
    let rows: Vec<Value> = row_text
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(jv(&rows[0]["threshold"]), 5);
    assert_eq!(jv(&rows[0]["reason"]), "边界重放显式给参");
}
