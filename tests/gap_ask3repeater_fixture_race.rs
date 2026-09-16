//! gap-ask3repeater-fixture-race：三问确定性外壳校验腿（ask3repeater bin）黑箱回归。
//!
//! 承 SPEC-005#deterministic-shell 校验腿：既有覆盖仅 src/ask3repeater/validate.rs
//! 单元测试（其夹具已按 tag+pid per-test 唯一路径消并行竞态）与 attractor_route.rs
//! 对 intercept 模块的进程内调用，bin CLI 面无任何黑箱测试。本件补：
//! 验收通过形与错误四类（形态/契约/血统/计量）经 bin 逐类触发，用法错与记录
//! 缺席与记录不可解析三值退出码对表。夹具竞态纪律：每测试独立 TempDir（唯一
//! 路径并行不互踩），哲学原文件与记录件同域落地，记录件名逐测试唯一；
//! 零共享路径零跨测试文件写入。

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_ask3repeater")
}

fn run(args: &[String]) -> (i32, Value, String) {
    let out = Command::new(bin()).args(args).output().unwrap();
    let code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let v: Value = serde_json::from_str(stdout.trim()).unwrap_or(Value::Null);
    (code, v, stdout)
}

/// 每测试独立沙箱域：philography.md 哲学原文与记录件同域，路径唯一不互踩。
struct Fx {
    dir: tempfile::TempDir,
}

const QUOTE: &str = "应而不藏即回应而不隐藏";

fn build_fx() -> Fx {
    let dir = tempfile::TempDir::new().unwrap();
    fs::write(dir.path().join("philography.md"), format!("{QUOTE}。\n")).unwrap();
    Fx { dir }
}

impl Fx {
    fn root(&self) -> String {
        self.dir.path().display().to_string()
    }

    fn write_record(&self, name: &str, record: &Value) -> String {
        let p = self.dir.path().join(name);
        fs::write(&p, record.to_string()).unwrap();
        p.display().to_string()
    }
}

fn valid_record() -> Value {
    json!({
        "session_id": "sess-gap-ask3",
        "raw_input": "三问要建\n",
        "round": 1,
        "intent_contract": {
            "goal": "建三问组件",
            "exclusions": [],
            "output_format": "cargo test 全绿",
            "injected_constraints": []
        },
        "domain_contract": {
            "target_domain": {"scope": "三问批", "max_depth": 1},
            "support_domain": {"scope": "背景", "max_depth": 1}
        },
        "anchors": [{
            "anchor_seq": 0,
            "text": "用户开工令三问要建",
            "inquiry_stage": "first",
            "domain_tag": "target",
            "depth": 0,
            "philosophy_ref": {"source": "philography.md", "quote": QUOTE},
            "rationale": "开工令即批次起点",
            "evidence": "raw_input:1",
            "confidence": 0.9
        }],
        "calls_in": 1,
        "calls_out": 1
    })
}

/// 验收通过形：status ok，会话号与锚点数如实回显，退出码零。
#[test]
fn t1_valid_record_accepted_exit_zero() {
    let fx = build_fx();
    let rec = fx.write_record("rec-ok.json", &valid_record());
    let (code, v, _) = run(&["--root".into(), fx.root(), rec]);
    assert_eq!(code, 0, "验收通过形退出码零：{v}");
    assert_eq!(v["status"], "ok");
    assert_eq!(v["session_id"], "sess-gap-ask3");
    assert_eq!(v["anchor_count"], 1);
}

/// 契约违例：锚点空数组与 round 0 与深度双检，逐形退出码一 findings 带类名。
#[test]
fn t2_contract_violations_rejected_per_form() {
    let fx = build_fx();

    let mut empty_anchors = valid_record();
    empty_anchors["anchors"] = json!([]);
    let rec = fx.write_record("rec-anchor.json", &empty_anchors);
    let (code, v, _) = run(&["--root".into(), fx.root(), rec]);
    assert_eq!(code, 1);
    assert_eq!(v["status"], "rejected");
    assert!(
        v["findings"].as_array().unwrap().iter().any(|f| f["class"] == "contract"),
        "锚点空数组须契约违例：{v}"
    );

    let mut zero_round = valid_record();
    zero_round["round"] = json!(0);
    let rec = fx.write_record("rec-round.json", &zero_round);
    let (code, v, _) = run(&["--root".into(), fx.root(), rec]);
    assert_eq!(code, 1);
    assert!(v["findings"].as_array().unwrap().iter().any(|f| f["class"] == "contract"));

    let mut deep = valid_record();
    deep["domain_contract"]["support_domain"]["max_depth"] = json!(2);
    let rec = fx.write_record("rec-depth.json", &deep);
    let (code, v, _) = run(&["--root".into(), fx.root(), rec]);
    assert_eq!(code, 1);
    assert!(v["findings"].as_array().unwrap().iter().any(|f| f["class"] == "contract"));
}

/// 血统不可核验：引文与原文不一致、出处缺席、证据行号越界，类名 lineage。
#[test]
fn t3_lineage_violations_rejected() {
    let fx = build_fx();

    let mut quote = valid_record();
    quote["anchors"][0]["philosophy_ref"]["quote"] = json!("原文里没有这句话");
    let rec = fx.write_record("rec-quote.json", &quote);
    let (code, v, _) = run(&["--root".into(), fx.root(), rec]);
    assert_eq!(code, 1);
    assert!(v["findings"].as_array().unwrap().iter().any(|f| f["class"] == "lineage"));

    let mut ghost = valid_record();
    ghost["anchors"][0]["philosophy_ref"]["source"] = json!("ghost.md");
    let rec = fx.write_record("rec-ghost.json", &ghost);
    let (code, v, _) = run(&["--root".into(), fx.root(), rec]);
    assert_eq!(code, 1);
    assert!(v["findings"].as_array().unwrap().iter().any(|f| f["class"] == "lineage"));

    let mut ev = valid_record();
    ev["anchors"][0]["evidence"] = json!("raw_input:99");
    let rec = fx.write_record("rec-ev.json", &ev);
    let (code, v, _) = run(&["--root".into(), fx.root(), rec]);
    assert_eq!(code, 1);
    assert!(v["findings"].as_array().unwrap().iter().any(|f| f["class"] == "lineage"));
}

/// 计量异常（calls_out < calls_in）与形态错误（goal 空）两类各一。
#[test]
fn t4_metering_and_shape_violations_rejected() {
    let fx = build_fx();

    let mut metering = valid_record();
    metering["calls_in"] = json!(3);
    metering["calls_out"] = json!(1);
    let rec = fx.write_record("rec-metering.json", &metering);
    let (code, v, _) = run(&["--root".into(), fx.root(), rec]);
    assert_eq!(code, 1);
    assert!(v["findings"].as_array().unwrap().iter().any(|f| f["class"] == "metering"));

    let mut shape = valid_record();
    shape["intent_contract"]["goal"] = json!(" ");
    let rec = fx.write_record("rec-shape.json", &shape);
    let (code, v, _) = run(&["--root".into(), fx.root(), rec]);
    assert_eq!(code, 1);
    assert!(v["findings"].as_array().unwrap().iter().any(|f| f["class"] == "shape"));
}

/// 工具异常三值面：记录缺席与记录不可解析与用法错（缺点名/缺 --root）退出码二。
#[test]
fn t5_tool_error_exit_two_forms() {
    let fx = build_fx();
    let missing = fx.dir.path().join("absent.json").display().to_string();
    let (code, v, _) = run(&["--root".into(), fx.root(), missing]);
    assert_eq!(code, 2);
    assert!(v["error"].as_str().unwrap().contains("记录不可读"), "{v}");

    let broken = fx.dir.path().join("broken.json");
    fs::write(&broken, "{not json}").unwrap();
    let (code, v, _) = run(&["--root".into(), fx.root(), broken.display().to_string()]);
    assert_eq!(code, 2);
    assert!(v["error"].as_str().unwrap().contains("记录形态不可解析"), "{v}");
    assert_eq!(v["class"], "shape");

    let (code, v, _) = run(&[]);
    assert_eq!(code, 2);
    assert!(v["error"].as_str().unwrap().contains("用法"), "{v}");

    let rec = fx.write_record("rec-noroot.json", &valid_record());
    let (code, v, _) = run(&[rec]);
    assert_eq!(code, 2);
    assert!(v["error"].as_str().unwrap().contains("--root"), "{v}");

    let (code, v, _) = run(&["--root".into()]);
    assert_eq!(code, 2);
    assert!(v["error"].as_str().unwrap().contains("--root"), "{v}");
}

/// 并行竞态面：同域两记录件并发校验互不串扰（bin 零共享可变态，逐件独立读验）。
#[test]
fn t6_concurrent_checks_on_separate_fixtures_no_interference() {
    let fx_a = build_fx();
    let fx_b = build_fx();
    let rec_a = fx_a.write_record("rec-a.json", &valid_record());
    let mut bad = valid_record();
    bad["calls_in"] = json!(5);
    bad["calls_out"] = json!(1);
    let rec_b = fx_b.write_record("rec-b.json", &bad);

    let h_a = std::thread::spawn(move || run(&["--root".into(), fx_a.root(), rec_a]));
    let h_b = std::thread::spawn(move || run(&["--root".into(), fx_b.root(), rec_b]));
    let (code_a, v_a, _) = h_a.join().unwrap();
    let (code_b, v_b, _) = h_b.join().unwrap();
    assert_eq!(code_a, 0, "甲域通过形不受乙域影响：{v_a}");
    assert_eq!(code_b, 1, "乙域计量违例不受甲域影响：{v_b}");
    assert!(v_b["findings"].as_array().unwrap().iter().any(|f| f["class"] == "metering"));
}
