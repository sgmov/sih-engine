//! m-gatesplit T 系：判据 v4 旗语分立（信心旗判定用、超纲旗记账用）。
//!
//! 四件规定单测（任务包 §三）：
//! - 信心旗触发 boundary（单测一）
//! - 超纲旗不触发仅记账（单测二）
//! - 旧四键响应兼容（单测三）
//! - 缺键 false（单测四）
//!
//! 加集成三件：引擎模板资产缺省出题（emit-contract 无 --atoms）、score 无
//! 外部判词时 v4 自判闸与缺口记账、tally 校验面认 v4 材料（八字段 trail 加
//! coverage 单列计数）。历史 trail 与金向量零改写（向前生效）。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::{json, Value};

use sih_engine::attractor::compiler;
use sih_engine::attractor::contract_mode;
use sih_engine::attractor::paradigm_loader;
use sih_engine::attractor::tally;

const IH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

struct Ws {
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    Ws { dir, _guard: guard }
}

/// 五键裁判响应原文（可指定双旗）。
fn raw_answer(boundary: bool, coverage: bool) -> String {
    json!({
        "decision": "comply",
        "basis_regulation": "baseline_4",
        "reason": if coverage { "题材超出规约直接覆盖：命题涉及的领域规约未直接言及" } else { "依据基线四可验证性要求" },
        "boundary_flag": boundary,
        "coverage_flag": coverage,
    })
    .to_string()
}

/// 旧四键裁判响应原文（v3 历史形，无 coverage_flag 键）。
fn raw_answer_v3(boundary: bool) -> String {
    json!({
        "decision": "comply",
        "basis_regulation": "baseline_4",
        "reason": "依据基线四可验证性要求",
        "boundary_flag": boundary,
    })
    .to_string()
}

// ---------- 单测三：旧四键响应兼容（解析面五键化不破旧形） ----------

#[test]
fn unit3_legacy_four_key_response_compatible() {
    // parse_raw_answer：旧四键 JSON → coverage_flag 缺席即 false
    let d = contract_mode::parse_raw_answer(&raw_answer_v3(false));
    assert_eq!(d["decision"], "comply");
    assert_eq!(d["boundary_flag"], false);
    assert_eq!(d["coverage_flag"], false, "旧四键响应 coverage_flag 须缺省 false");
    assert_eq!(d["void"], false);
    let d2 = contract_mode::parse_raw_answer(&raw_answer_v3(true));
    assert_eq!(d2["boundary_flag"], true);
    assert_eq!(d2["coverage_flag"], false);
    // extract_decision_fields：旧四键 → Some 且 coverage_flag false
    let e = compiler::extract_decision_fields(&raw_answer_v3(false)).unwrap();
    assert_eq!(e["coverage_flag"], false);
    assert_eq!(e["decision"], "comply");
    // 五键 → coverage_flag 如实
    let e2 = compiler::extract_decision_fields(&raw_answer(false, true)).unwrap();
    assert_eq!(e2["coverage_flag"], true);
}

// ---------- 单测四：缺键 false（schema 内缺 coverage_flag 键） ----------

#[test]
fn unit4_missing_key_defaults_false() {
    // JSON 对象内缺 coverage_flag 键（也缺 boundary_flag 键：两旗缺席均 false）
    let raw = "{\"decision\": \"violate\", \"basis_regulation\": \"baseline_1\", \"reason\": \"r\"}";
    let d = contract_mode::parse_raw_answer(raw);
    assert_eq!(d["decision"], "violate");
    assert_eq!(d["boundary_flag"], false, "boundary_flag 缺席须 false");
    assert_eq!(d["coverage_flag"], false, "coverage_flag 缺席须 false");
    // 非 JSON：解析失败仍 no_answer + boundary_flag=true + coverage_flag=false
    let bad = contract_mode::parse_raw_answer("这不是 JSON");
    assert_eq!(bad["decision"], "no_answer");
    assert_eq!(bad["boundary_flag"], true);
    assert_eq!(bad["coverage_flag"], false, "解析失败不是超纲陈述，coverage 落 false");
    assert_eq!(bad["void"], true);
    // extract_decision_fields：boundary_flag 仍是 required 面（缺席 → None）
    let no_bflag = "{\"decision\": \"comply\", \"basis_regulation\": \"baseline_4\", \"reason\": \"r\"}";
    assert!(compiler::extract_decision_fields(no_bflag).is_none());
}

// ---------- 单测一与单测二：双旗分立下的闸语义（走 score 全链） ----------

/// 建 cell：引擎模板出题 → n 发响应 → score（v4 自判闸）→ 返回 (ws, 计分材料, gid)。
fn build_score(boundary_flags: &[bool], coverage_flags: &[bool]) -> (Ws, Value) {
    let w = ws();
    let topic = w.dir.join("topic.md");
    fs::write(&topic, "命题正文：闸门修复责任转移给固定程序\n").unwrap();
    let atoms = paradigm_loader::load_engine_template_atoms().unwrap();
    let topic_text = fs::read_to_string(&topic).unwrap();
    let (system, user) = contract_mode::build_integrator_prompts(&atoms, &topic_text, "规约正文\n");
    let shots = contract_mode::make_shots(&system, &user, "m-gatesplit-t", boundary_flags.len() as i64, 1);
    let contract_path = w.dir.join("contract.json");
    contract_mode::emit_contract(
        &contract_path,
        &json!({"framework": "ZCode", "model_id": "GLM-Test", "version": "self-reported"}),
        &json!({"paradigm_id": "normative_convergence", "atom": "integrator", "direction": "judge",
                "ng_label": "medium", "ng_text_sha256": contract_mode::sha256_bytes(b"ng")}),
        &json!({"gid": "m-gatesplit-t", "title": "旗语分立测试",
                "topic_sha256": contract_mode::sha256_file(&topic).unwrap()}),
        &shots,
        &json!({"measurement_entry": topic.to_string_lossy(), "n_declared": boundary_flags.len()}),
    )
    .unwrap();
    // 引擎模板产出的 system 提示词须为 v4 五键形
    assert!(system.contains("coverage_flag"), "引擎模板须含 coverage_flag 字段说明");
    assert!(system.contains("你对本判定是否拿不准"), "boundary_flag 须为信心信号定义");
    let responses_path = w.dir.join("responses.jsonl");
    let mut lines = Vec::new();
    for (k, (b, c)) in boundary_flags.iter().zip(coverage_flags.iter()).enumerate() {
        lines.push(json!({"key": format!("m-gatesplit-t#r{}", k + 1), "shot": k + 1,
                          "raw": raw_answer(*b, *c)}).to_string());
    }
    fs::write(&responses_path, lines.join("\n") + "\n").unwrap();
    let out = w.dir.join("score-material.json");
    let material = contract_mode::score_pipeline(
        &contract_path, &responses_path, &w.dir.join("trail.jsonl"), &out, IH, None,
    )
    .unwrap();
    (w, material)
}

#[test]
fn unit1_confidence_flag_trips_boundary() {
    // 单测一：信心旗 4/9 ≈ 0.4444 ≥ 0.34 → boundary；超纲旗再多也不影响此判
    let bflags = [false, false, false, false, false, true, true, true, true];
    let cflags = [true, true, true, true, true, true, true, true, true];
    let (w, m) = build_score(&bflags, &cflags);
    assert_eq!(m["gate_verdict"], "boundary", "信心旗率超阈须打回重作态");
    assert_eq!(m["gate_source"], "engine-maturation-v4");
    assert_eq!(m["criteria_version"], "v4");
    assert_eq!(m["gate"]["criteria"]["boundary_low"], false);
    assert_eq!(m["gate"]["metrics"]["boundary_rate"], 0.4444);
    // tally 面认 v4 结果：boundary → 打回重作
    let report = check_of(&w, &m);
    assert_eq!(report["disposition"], "打回重作");
    assert!(report["passed"].as_array().unwrap().iter()
        .any(|p| p.as_str().unwrap().starts_with("R3: coverage 单列记账 9/9")),
        "coverage 计数须单列在案：{}", report["passed"]);
}

#[test]
fn unit2_coverage_flag_books_without_tripping() {
    // 单测二：超纲旗 9/9 全举、信心旗零举 → stable_clear，coverage 只记账
    let bflags = [false; 9];
    let cflags = [true; 9];
    let (w, m) = build_score(&bflags, &cflags);
    assert_eq!(m["gate_verdict"], "stable_clear", "超纲描述信号不得触发三态");
    assert_eq!(m["gate"]["criteria"]["boundary_low"], true);
    assert_eq!(m["coverage_flags"], 9, "缺口记账计数须在册");
    assert_eq!(m["coverage_rate"], 1.0);
    let reasons = m["coverage_reasons"].as_array().unwrap();
    assert_eq!(reasons.len(), 9, "举旗 reason 摘引须逐发在册");
    assert_eq!(reasons[0]["key"], "m-gatesplit-t#r1");
    assert!(reasons[0]["reason"].as_str().unwrap().contains("超出规约"));
    // tally 面照常裁决通过（描述信号不在线扰人）
    let report = check_of(&w, &m);
    assert_eq!(report["disposition"], "裁决通过");
    assert_eq!(report["direction"], "comply");
}

/// score 材料 → cell 装配 → check 全量核对（v4 面）。
fn check_of(w: &Ws, score: &Value) -> Value {
    let cell = w.dir.join("cell/m-gatesplit-t");
    fs::create_dir_all(&cell).unwrap();
    fs::copy(w.dir.join("contract.json"), cell.join("contract.json")).unwrap();
    fs::copy(w.dir.join("responses.jsonl"), cell.join("responses.jsonl")).unwrap();
    fs::copy(w.dir.join("topic.md"), cell.join("topic.md")).unwrap();
    fs::copy(w.dir.join("trail.jsonl"), cell.join("flywheel-trail.jsonl")).unwrap();
    fs::write(cell.join("contract-n9-score-material.json"),
              serde_json::to_string(score).unwrap()).unwrap();
    let baseline = w.dir.join("seat-baseline.json");
    fs::write(&baseline, json!({"verdict": "可用", "identity_hash": IH}).to_string()).unwrap();
    let out = w.dir.join("assembled.json");
    let assembled = tally::assemble_material(
        "m-gatesplit-t", &w.dir.join("cell"), &[], Some(&baseline),
        Some("2026-09-24"), &out, None,
    ).unwrap();
    assert_eq!(assembled["criteria_version"], "v4", "v4 计分材料装配须随件 v4");
    assert_eq!(assembled["coverage_flags"], score["coverage_flags"], "coverage 记账须透传装配面");
    tally::check_material(&out).unwrap()
}

// ---------- 集成：外部判词兼容形保持旧四键面（金向量对表线） ----------

#[test]
fn integration_legacy_gate_param_keeps_legacy_shape() {
    let bflags = [false; 3];
    let cflags = [true; 3];
    let (w, _m) = build_score(&bflags, &cflags);
    // 以同输入走外部判词形：材料面不带任何 v4 扩展键（围堰兼容逐字节面）
    let out = w.dir.join("score-legacy.json");
    let responses_path = w.dir.join("responses.jsonl");
    let contract_path = w.dir.join("contract.json");
    // trail 已有同 run_id 行：幂等跳过，runs_written 0 不影响断言面
    let legacy = contract_mode::score_pipeline(
        &contract_path, &responses_path, &w.dir.join("trail.jsonl"), &out, IH,
        Some("stable_clear"),
    ).unwrap();
    assert_eq!(legacy["gate_verdict"], "stable_clear");
    assert!(legacy.get("gate_source").is_none(), "兼容形不得携带 gate_source");
    assert!(legacy.get("coverage_flags").is_none(), "兼容形不得携带 coverage 记账");
    assert!(legacy.get("criteria_version").is_none(), "兼容形不得携带 criteria_version");
    assert!(legacy.get("gate").is_none(), "兼容形不得携带判据明细");
}

// ---------- 集成：emit-contract CLI 无 --atoms 走引擎资产缺省 ----------

#[test]
fn integration_emit_contract_engine_template_default() {
    let w = ws();
    let topic = w.dir.join("topic.md");
    fs::write(&topic, "命题正文\n").unwrap();
    let ng = w.dir.join("ng.txt");
    fs::write(&ng, "规约正文\n").unwrap();
    let out = w.dir.join("contract.json");
    let res = Command::new(env!("CARGO_BIN_EXE_attractor"))
        .args(["emit-contract",
            "--topic", &topic.to_string_lossy(),
            "--ng-file", &ng.to_string_lossy(),
            "--seat", "ZCode:GLM-Test",
            "--gid", "m-gatesplit-cli",
            "--title", "模板缺省测试",
            "--shots", "3",
            "--ng-label", "medium",
            "--out", &out.to_string_lossy()])
        .output().unwrap();
    assert_eq!(res.status.code(), Some(0),
        "引擎资产缺省出题须 0：stderr={}", String::from_utf8_lossy(&res.stderr));
    let contract: Value = serde_json::from_str(&fs::read_to_string(&out).unwrap()).unwrap();
    let sys = contract["shots"][0]["system_prompt"].as_str().unwrap();
    assert!(sys.contains("coverage_flag"), "缺省模板须为 v4 五键形");
    assert!(sys.contains("描述信号，不影响判定"));
    assert_eq!(contract["shots"].as_array().unwrap().len(), 3);
}

// ---------- R5 读档注入形：mint 退役后配对降为留档信号（m-gatesplit 范围扩展腿） ----------

/// R5 专用装配：复用 build_score 的 stable_clear 计分材料，指定基线件与正身
/// 报告内容做装配后全量核对。返回 check 报告。
fn r5_check(baseline: &Value, identity_core: Option<&str>) -> Value {
    let (w, score) = build_score(&[false; 3], &[false; 3]);
    let cell = w.dir.join("cell/m-gatesplit-t");
    fs::create_dir_all(&cell).unwrap();
    fs::copy(w.dir.join("contract.json"), cell.join("contract.json")).unwrap();
    fs::copy(w.dir.join("responses.jsonl"), cell.join("responses.jsonl")).unwrap();
    fs::copy(w.dir.join("topic.md"), cell.join("topic.md")).unwrap();
    fs::copy(w.dir.join("trail.jsonl"), cell.join("flywheel-trail.jsonl")).unwrap();
    fs::write(cell.join("contract-n3-score-material.json"),
              serde_json::to_string(&score).unwrap()).unwrap();
    let baseline_path = w.dir.join("r5-baseline.json");
    fs::write(&baseline_path, serde_json::to_string(baseline).unwrap()).unwrap();
    let report_path = w.dir.join("r5-identity.json");
    let identity = match identity_core {
        Some(c) => json!({"identity": {"core_hash": c}}),
        None => json!({"identity": {}}),
    };
    fs::write(&report_path, identity.to_string()).unwrap();
    let out = w.dir.join("r5-assembled.json");
    tally::assemble_material(
        "m-gatesplit-t", &w.dir.join("cell"), &[], Some(&baseline_path),
        Some("2026-09-24"), &out, Some(&report_path),
    ).unwrap();
    tally::check_material(&out).unwrap()
}

#[test]
fn r5_core_equal_pairs_and_passes() {
    // 核等值快径不动：核哈希一致且判定可用 → 裁决通过、无核漂移告警
    let report = r5_check(
        &json!({"verdict": "可用", "identity_hash": "bbbb", "core_hash": "CCC"}),
        Some("CCC"),
    );
    assert_eq!(report["disposition"], "裁决通过");
    assert!(report["passed"].as_array().unwrap().iter()
        .any(|p| p.as_str().unwrap().contains("身份核哈希一致")));
    assert!(!report["alarms"].as_array().unwrap().iter()
        .any(|a| a.as_str().unwrap().contains("核哈希与冻结基线不一致")),
        "核等值不得出漂移告警：{}", report["alarms"]);
}

#[test]
fn r5_core_mismatch_postmint_signal_not_blocker() {
    // 读档注入形核心：核不一致 + 判定可用 → 裁决通过 + 漂移信号留档告警
    let report = r5_check(
        &json!({"verdict": "可用", "identity_hash": "bbbb", "core_hash": "DDD"}),
        Some("CCC"),
    );
    assert_eq!(report["disposition"], "裁决通过",
        "pk-044 读档注入形下核漂移不得拦判定：{}", report["disposition"]);
    assert!(report["passed"].as_array().unwrap().iter()
        .any(|p| p.as_str().unwrap().contains("环境漂移信号留档")));
    assert!(report["alarms"].as_array().unwrap().iter()
        .any(|a| a.as_str().unwrap().contains("核哈希与冻结基线不一致")),
        "核漂移告警须在档：{}", report["alarms"]);
}

#[test]
fn r5_core_mismatch_with_drift_verdict_still_suspends() {
    // 基线判定漂移告警仍是阻断位：核配对结果不豁免基线异常三态
    let report = r5_check(
        &json!({"verdict": "漂移告警", "identity_hash": "bbbb", "core_hash": "CCC"}),
        Some("CCC"),
    );
    assert_eq!(report["disposition"], "挂起");
}

#[test]
fn r5_identity_mismatch_postmint_signal_not_blocker() {
    // 旧身份哈希路径同读档注入形：双带不一致 + 判定可用 → 裁决通过 + 告警留档
    let report = r5_check(
        &json!({"verdict": "可用", "identity_hash": "bbbb"}),
        None,
    );
    assert_eq!(report["disposition"], "裁决通过");
    assert!(report["alarms"].as_array().unwrap().iter()
        .any(|a| a.as_str().unwrap().contains("身份哈希与冻结基线不一致")),
        "身份哈希漂移告警须在档：{}", report["alarms"]);
}

#[test]
fn r5_identity_report_without_core_falls_back() {
    // 旧正身报告无 core_hash：材料空缺不判败，回退现行为（判定三态直判）
    let report = r5_check(
        &json!({"verdict": "可用"}),
        None,
    );
    assert_eq!(report["disposition"], "裁决通过");
    assert!(report["passed"].as_array().unwrap().iter()
        .any(|p| p.as_str().unwrap().contains("席位当日基线判定可用")));
}
