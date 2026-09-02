//! deyimerge-tdd-solo T4+T6：跨腿契约兼容、多包归因、des-011 规约随迁、
//! 腿切分清单逐行核对、存量感知幂等续跑。
//!
//! sign 经既有 guard_crosscheck 零改通过即 T4 终签锚判据：引擎 sign 以真
//! scribe 二进制落临时链，链 verify valid 且事件类型 crosscheck_completed。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

use sih_engine::attractor::contract_mode;
use sih_engine::attractor::jsonc::canonical_json;
use sih_engine::attractor::tally;

fn golden() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/attractor/fixtures/golden")
}

struct Ws {
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_ws(scenario: &str) -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let src = golden().join(scenario).join("input");
    fs::create_dir_all(dir.join("input")).unwrap();
    for f in fs::read_dir(&src).unwrap() {
        let f = f.unwrap();
        fs::copy(f.path(), dir.join("input").join(f.file_name())).unwrap();
    }
    let mat_src = src.join("material.json");
    if mat_src.exists() {
        let text = fs::read_to_string(&mat_src).unwrap().replace("@ROOT@", &dir.to_string_lossy());
        fs::write(dir.join("material.json"), text).unwrap();
    }
    Ws { dir, _guard: guard }
}

// ---------- T4 跨腿契约兼容 ----------

#[test]
fn t4_assemble_field_compat_then_check_passes() {
    // 计分材料经 assemble 至 tally-check-input 逐字段兼容（SPEC-014 跨腿字段对表）
    let ws = build_ws("adisp-net");
    let cell = ws.dir.join("cell/adisp-guard-1");
    fs::create_dir_all(&cell).unwrap();
    // 合同补 proposition.topic_sha256（assemble 的 topic 发现依据）
    let mut contract: Value = serde_json::from_str(
        &fs::read_to_string(ws.dir.join("input/contract.json")).unwrap()).unwrap();
    let topic_hash = contract_mode::sha256_file(&ws.dir.join("input/topic.md")).unwrap();
    contract["proposition"]["topic_sha256"] = Value::String(topic_hash.clone());
    fs::write(cell.join("contract.json"), serde_json::to_string(&contract).unwrap()).unwrap();
    // 响应原样、trail 与 topic 就位
    fs::copy(ws.dir.join("input/responses.jsonl"), cell.join("responses.jsonl")).unwrap();
    fs::copy(ws.dir.join("input/flywheel-trail.jsonl"), cell.join("flywheel-trail.jsonl")).unwrap();
    fs::copy(ws.dir.join("input/topic.md"), cell.join("topic.md")).unwrap();
    let contract_sha = contract_mode::sha256_file(&cell.join("contract.json")).unwrap();
    let responses_sha = contract_mode::sha256_file(&cell.join("responses.jsonl")).unwrap();
    let baseline: Value = serde_json::from_str(
        &fs::read_to_string(ws.dir.join("input/seat-baseline.json")).unwrap()).unwrap();
    let score = serde_json::json!({
        "gid": "adisp-guard-1",
        "contract_path": cell.join("contract.json").to_string_lossy(),
        "contract_sha256": contract_sha,
        "responses_path": cell.join("responses.jsonl").to_string_lossy(),
        "responses_sha256": responses_sha,
        "trail_path": cell.join("flywheel-trail.jsonl").to_string_lossy(),
        "n_shots": 9, "voids": [],
        "gate_verdict": "stable_clear",
        "identity_hash": baseline["identity_hash"],
    });
    fs::write(cell.join("contract-n9-score-material.json"), serde_json::to_string(&score).unwrap()).unwrap();
    let baseline_path = cell.join("baseline.json");
    fs::write(&baseline_path, serde_json::to_string(&baseline).unwrap()).unwrap();
    let out = ws.dir.join("assembled.json");
    let material = tally::assemble_material(
        "adisp-guard-1", &ws.dir.join("cell"), &[], Some(&baseline_path),
        Some("2026-09-02"), &out).unwrap();
    // 跨腿契约字段逐字段对表
    for field in ["kind", "gid", "topic_path", "topic_sha256", "trail_path", "dc_fingerprint",
                  "gate_verdict", "criteria_version", "contract_path", "contract_sha256",
                  "responses_path", "responses_sha256", "n_shots", "voids", "rules_version"] {
        assert!(material.get(field).is_some(), "assemble 缺跨腿字段 {field}");
    }
    assert_eq!(material["kind"], "tally-check-input");
    assert_eq!(material["criteria_version"], "v3");
    assert_eq!(material["rules_version"], "des-011-r1");
    assert_eq!(material["topic_sha256"], Value::String(topic_hash));
    assert_eq!(material["identity_hash"], baseline["identity_hash"]);
    // 装配件可被 check 全过（跨腿零语义漂移）
    let report = tally::check_material(&out).unwrap();
    assert_eq!(report["verdict"], "pass");
    assert_eq!(report["disposition"], "裁决通过");
}

#[test]
fn t4_engine_signcheck_passes_guard_crosscheck_unchanged() {
    // 引擎 sign 产 signcheck 经既有 scribe crosscheck 落临时链，链 verify valid。
    // 材料用相对路径形（guard_crosscheck 十五字段面拒绝对路径材料引用，
    // 承 SPEC-014 crosscheck 载荷对表），内部路径相对进程 CWD 解。
    let ws = build_ws("adisp-net");
    let mut material: Value = serde_json::from_str(&fs::read_to_string(ws.dir.join("material.json")).unwrap()).unwrap();
    {
        let o = material.as_object_mut().unwrap();
        for key in ["topic_path", "trail_path", "contract_path", "responses_path", "seat_baseline_path"] {
            if let Some(v) = o.get(key).and_then(|x| x.as_str()).map(|s| s.to_string()) {
                let abs = std::path::PathBuf::from(&v);
                let rel = abs.strip_prefix(&ws.dir).map(|p| p.to_string_lossy().into_owned()).unwrap_or(v);
                o.insert(key.into(), Value::String(rel));
            }
        }
    }
    fs::write(ws.dir.join("material-rel.json"), format!("{}\n", canonical_json(&material))).unwrap();
    let out_dir = ws.dir.join("sign-out");
    let scribe = env!("CARGO_BIN_EXE_scribe");
    let out = Command::new(env!("CARGO_BIN_EXE_attractor"))
        .current_dir(&ws.dir)
        .args(["sign",
            "--material", "material-rel.json",
            "--out", "sign-out",
            "--trail", "trail.ndjson",
            "--scribe-binary", scribe,
            "--session", "sess-deyitdd-test",
            "--locks", "absent-locks.ndjson"])
        .output().unwrap();
    assert_eq!(out.status.code(), Some(0), "sign stdout={} stderr={}",
               String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    let signcheck = out_dir.join("adisp-guard-1-signcheck.json");
    assert!(signcheck.is_file());
    // 链上事件：crosscheck_completed 且 verify valid
    let v = Command::new(scribe).current_dir(&ws.dir).args(["verify", "--trail", "trail.ndjson"]).output().unwrap();
    let vout: Value = serde_json::from_slice(&v.stdout).unwrap();
    assert_eq!(vout["status"], "valid", "临时链须 valid：{}", String::from_utf8_lossy(&v.stdout));
    let chain = fs::read_to_string(ws.dir.join("trail.ndjson")).unwrap();
    assert!(chain.contains("crosscheck_completed"), "链上须有 crosscheck_completed 事件");
    assert!(chain.contains("crosscheck-adisp-guard-1"), "doc_id 须 crosscheck-gid 形");
}

#[test]
fn t4_multi_gid_no_cross_contamination() {
    // 多 gid 归因：谱系与预算按 gid 独立累计互不串扰（A3）
    let ws = build_ws("adisp-net");
    // gid A 的 trail（九发 m-a）+ 材料 A：R6 未超、R7 同 gid 一致
    let mat_a = ws.dir.join("material.json");
    let report_a = tally::check_material(&mat_a).unwrap();
    assert_eq!(report_a["verdict"], "pass");
    // 材料 B（异 gid）指向同一 trail：R7 须浮异 gid 判定行，不静默串扰
    let mut material_b: Value = serde_json::from_str(&fs::read_to_string(&mat_a).unwrap()).unwrap();
    material_b["gid"] = Value::String("m-other".into());
    let mat_b = ws.dir.join("material-b.json");
    fs::write(&mat_b, format!("{}\n", canonical_json(&material_b))).unwrap();
    let report_b = tally::check_material(&mat_b).unwrap();
    assert_eq!(report_b["disposition"], "材料退回");
    assert!(report_b["failed"].as_array().unwrap().iter()
        .any(|f| f["rule"] == "R7" && f["where"].as_str().unwrap().contains("异 gid")),
        "异 gid 须 R7 失败在案：{}", canonical_json(&report_b));
}

// ---------- T4 腿切分清单逐行核对 ----------

#[test]
fn t4_leg_split_manifest() {
    // 融回十一行 → 引擎侧对应物在位；留堰件标识符零出现
    let manifest: &[(&str, &str)] = &[
        // (围堰腿切分行, 引擎对应物路径，tests 行对应集成测试组)
        ("contract_mode.py", "src/attractor/contract_mode.rs"),
        ("facet_stats.py", "src/attractor/stats/mod.rs"),
        ("facet_stats_metrics.py", "src/attractor/stats/metrics.rs"),
        ("facet_stats_inf.py", "src/attractor/stats/inf.rs"),
        ("facet_stats_conv.py", "src/attractor/stats/conv.rs"),
        ("compiler.py", "src/attractor/compiler.rs"),
        ("validators.py", "src/attractor/validators.rs"),
        ("anchors.py", "src/attractor/anchors.rs"),
        ("model_utils.py", "src/attractor/model_utils.rs"),
        ("paradigm_loader.py", "src/attractor/paradigm_loader.rs"),
        ("tally/src/tally/cli.py", "src/attractor/tally.rs"),
        ("tests/test_tally.py（测试面随迁作 TDD 基线）", "tests/attractor_contract.rs"),
    ];
    let attractor_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/attractor");
    let bin_rs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/bin/attractor.rs")).unwrap();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for (line, module_file) in manifest {
        let path = root.join(module_file);
        assert!(path.is_file(), "融回行 {line} 无引擎对应物 {module_file}");
    }
    // 留堰件名零出现于引擎融回件。注：trail 行 schema 的 JSON 字面时间戳
    // 字段（append_contract_runs 用标准时钟生成）是围堰行形契约字段，非留
    // 堰件引用，不在禁词列。
    let stay = ["llm_client", "env_loader", "req.py", "engine.py", "runner.py",
                "concurrency", "persistence", "audit.py", "thinking_resolver",
                "measure.py", "singleseat", "dose_driver", "config.py"];
    let mut all = String::new();
    for entry in fs::read_dir(&attractor_dir).unwrap() {
        let entry = entry.unwrap();
        let p = entry.path();
        if p.is_dir() {
            for sub in fs::read_dir(&p).unwrap() {
                let sub = sub.unwrap();
                all.push_str(&fs::read_to_string(sub.path()).unwrap_or_default());
            }
        } else {
            all.push_str(&fs::read_to_string(p).unwrap_or_default());
        }
    }
    all.push_str(&bin_rs);
    for s in stay {
        assert!(!all.contains(s), "融回件含留堰标识 {s}（腿切分红线）");
    }
}

// ---------- T6 des-011 规约与存活性 ----------

#[test]
fn t6_rules_version_shape() {
    // 与引擎既有 is_rules_version 同形：des 三位号 r 数字形
    assert!(tally::is_rules_version("des-011-r1"));
    assert!(tally::is_rules_version("des-042-r9"));
    assert!(!tally::is_rules_version("des-011"));
    assert!(!tally::is_rules_version("des-11-r1"));
    assert!(!tally::is_rules_version("des-011-rx"));
    assert!(!tally::is_rules_version(""));
    // 缺省承载 des-011-r1
    let ws = build_ws("adisp-net");
    let mut material: Value = serde_json::from_str(&fs::read_to_string(ws.dir.join("material.json")).unwrap()).unwrap();
    material.as_object_mut().unwrap().remove("rules_version");
    let mat = ws.dir.join("material-norv.json");
    fs::write(&mat, format!("{}\n", canonical_json(&material))).unwrap();
    let report = tally::check_material(&mat).unwrap();
    assert_eq!(report["rules_version"], "des-011-r1", "缺省须回落 des-011-r1");
}

#[test]
fn t6_tri_state_mapping_four_dispositions() {
    // 三态映射四值处置零漂移（对照 tests/test_tally.py 原判据）
    let ws = build_ws("adisp-net");
    let mat = ws.dir.join("material.json");
    let base: Value = serde_json::from_str(&fs::read_to_string(&mat).unwrap()).unwrap();
    let write = |ws: &Ws, name: &str, verdict: &str| -> PathBuf {
        let mut m = base.clone();
        m["gate_verdict"] = Value::String(verdict.into());
        let p = ws.dir.join(name);
        fs::write(&p, format!("{}\n", canonical_json(&m))).unwrap();
        p
    };
    assert_eq!(tally::check_material(&write(&ws, "v-clear.json", "stable_clear")).unwrap()["disposition"], "裁决通过");
    assert_eq!(tally::check_material(&write(&ws, "v-near.json", "near_threshold")).unwrap()["disposition"], "挂起");
    assert_eq!(tally::check_material(&write(&ws, "v-boundary.json", "boundary")).unwrap()["disposition"], "打回重作");
    // R5 漂移告警 → 挂起（席位异常先于闸三态）
    let mut m = base.clone();
    let drift_baseline = ws.dir.join("drift-baseline.json");
    let mut b = baseline_of(&ws);
    b["verdict"] = Value::String("漂移告警".into());
    fs::write(&drift_baseline, serde_json::to_string(&b).unwrap()).unwrap();
    m["seat_baseline_path"] = Value::String(drift_baseline.to_string_lossy().into());
    let p = ws.dir.join("v-drift.json");
    fs::write(&p, format!("{}\n", canonical_json(&m))).unwrap();
    assert_eq!(tally::check_material(&p).unwrap()["disposition"], "挂起");
}

fn baseline_of(ws: &Ws) -> Value {
    serde_json::from_str(&fs::read_to_string(ws.dir.join("input/seat-baseline.json")).unwrap()).unwrap()
}

#[test]
fn t6_idempotent_replay_no_history_rewrite() {
    // 存量感知幂等续跑：run_id 已在 trail 即跳过，重复调用不重写历史行（R4）
    let ws = build_ws("adisp-net");
    let trail = ws.dir.join("fresh-trail.jsonl");
    let contract = contract_mode::load_contract(&ws.dir.join("input/contract.json")).unwrap();
    let responses = contract_mode::load_responses(&ws.dir.join("input/responses.jsonl"), &contract).unwrap();
    let (per_actor, voids) = contract_mode::dc_from_responses(&responses, &contract["seat"]).unwrap();
    assert!(voids.is_empty());
    let written1 = contract_mode::append_contract_runs(
        &trail, "adisp-guard-1", &responses, &per_actor, &contract["seat"], "medium").unwrap();
    assert_eq!(written1.len(), 9);
    let snapshot = fs::read_to_string(&trail).unwrap();
    let written2 = contract_mode::append_contract_runs(
        &trail, "adisp-guard-1", &responses, &per_actor, &contract["seat"], "medium").unwrap();
    assert!(written2.is_empty(), "幂等续跑须零实写");
    assert_eq!(fs::read_to_string(&trail).unwrap(), snapshot, "历史行不得重写");
}
