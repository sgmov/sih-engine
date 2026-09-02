//! deyimerge-tdd-solo T1+T2：金向量冻结与四类工件逐字节一致。
//!
//! 金向量由围堰 Python 原件跑出冻结（freeze_golden.py，唯一基准禁自造），
//! 本测试以引擎件同参重建工作区，token 反填后逐字节 cmp。
//! 归一形：仅 @ROOT@（工作区根）与 @MATERIAL@（材料实参串）与 @TOPIC_ENTRY@
//! （合同 meta 路径位）三 token，双跑活体 cmp 零归一另证（探针证据）。

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use sih_engine::attractor::contract_mode;
use sih_engine::attractor::jsonc::canonical_json;
use sih_engine::attractor::tally;

fn golden() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/attractor/fixtures/golden")
}

fn read_expected(rel: &str) -> Vec<u8> {
    fs::read(golden().join(rel)).unwrap_or_else(|e| panic!("金向量缺席 {rel}: {e}"))
}

struct Ws {
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_ws(scenario: &str) -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let src = golden().join(scenario).join("input");
    if src.exists() {
        fs::create_dir_all(dir.join("input")).unwrap();
        for f in fs::read_dir(&src).unwrap() {
            let f = f.unwrap();
            fs::copy(f.path(), dir.join("input").join(f.file_name())).unwrap();
        }
    }
    let mat_src = src.join("material.json");
    if mat_src.exists() {
        let text = fs::read_to_string(&mat_src).unwrap().replace("@ROOT@", &dir.to_string_lossy());
        fs::write(dir.join("material.json"), text).unwrap();
    }
    Ws { dir, _guard: guard }
}

fn detok_expect(scenario: &str, ws: &Ws, mat: &std::path::Path) -> String {
    let want_raw = read_expected(&format!("{scenario}/expected/check-report.json"));
    String::from_utf8(want_raw)
        .unwrap()
        .replace("@MATERIAL@", &mat.to_string_lossy())
        .replace("@ROOT@", &ws.dir.to_string_lossy())
}

// ---------- T1 金向量冻结 ----------

#[test]
fn t1_golden_frozen_with_dirty_targets() {
    let g = golden();
    // 目录在位（红态判据：fixtures 目录不存在）
    assert!(g.is_dir(), "金向量目录未冻结");
    // 净二
    for s in ["adisp-net", "p3xcarr-net"] {
        assert!(g.join(s).join("expected/check-report.json").is_file(), "{s} 缺核对报告金向量");
        assert!(g.join(s).join("expected/signcheck.json").is_file(), "{s} 缺终签锚金向量");
        assert!(g.join(s).join("expected/check-exit.txt").is_file(), "{s} 缺退出码");
        assert!(g.join(s).join("input/material.json").is_file(), "{s} 缺输入材料");
    }
    // 脏三：挂起形、退回形、告警形
    let suspend = serde_json::from_str::<Value>(
        &String::from_utf8(read_expected("dirty-suspend-near/expected/check-report.json")).unwrap())
        .unwrap();
    assert_eq!(suspend["disposition"], "挂起", "脏形一须挂起");
    let ret = serde_json::from_str::<Value>(
        &String::from_utf8(read_expected("dirty-return-r2/expected/check-report.json")).unwrap())
        .unwrap();
    assert_eq!(ret["disposition"], "材料退回", "脏形二须材料退回");
    assert!(ret["failed"].as_array().unwrap().iter().any(|f| f["rule"] == "R2"), "退回形须 R2 失败在案");
    let alarm = serde_json::from_str::<Value>(
        &String::from_utf8(read_expected("dirty-alarm-r7/expected/check-report.json")).unwrap())
        .unwrap();
    assert_eq!(alarm["disposition"], "裁决通过");
    assert_eq!(alarm["alarms"].as_array().unwrap().len(), 1, "脏形三须 R7 超三次改写告警");
    assert!(alarm["alarms"][0].as_str().unwrap().contains("钓邻域"));
    // 合同类拒收基线四形
    for s in ["reject-missing-shot", "reject-key-mismatch", "reject-shot-misalign", "reject-raw-empty"] {
        let e = read_expected(&format!("{s}/expected/reject.json"));
        let v: Value = serde_json::from_slice(&e).unwrap();
        assert!(v["error"].as_str().unwrap().starts_with("ValueError: "), "{s} 拒收基线缺 ValueError 信封");
    }
    // 金向量内含合同哈希与响应哈希与终签锚哈希
    let net: Value = serde_json::from_str(
        &String::from_utf8(read_expected("adisp-net/expected/check-report.json")).unwrap()).unwrap();
    assert_eq!(net["gid"], "adisp-guard-1");
    assert_eq!(net["verdict"], "pass");
    assert!(net["passed"].as_array().unwrap().iter().any(|p| p.as_str().unwrap().contains("contract_sha256 复算一致")));
    assert!(net["passed"].as_array().unwrap().iter().any(|p| p.as_str().unwrap().contains("responses_sha256 复算一致")));
    // 统计参照值在册
    let stats: Value = serde_json::from_slice(&read_expected("stats-values.json")).unwrap();
    assert!(stats["hellinger"].as_array().unwrap().len() >= 4);
    assert!(stats["anova"].as_array().unwrap().len() >= 1);
}

#[test]
fn t1_contract_and_score_golden_frozen() {
    let g = golden();
    assert!(g.join("adisp-net/expected/contract.json").is_file(), "合同金向量缺席");
    for s in ["adisp-guard-1-net", "m-p3xcarr-net"] {
        let m: Value = serde_json::from_str(&String::from_utf8(
            read_expected(&format!("{s}/expected/score-material.json"))).unwrap()).unwrap();
        assert_eq!(m["kind"], "facet-contract-score");
        assert_eq!(m["score_version"], 2);
        assert!(m["contract_sha256"].as_str().unwrap().len() == 64, "{s} 合同哈希缺席");
        assert!(m["responses_sha256"].as_str().unwrap().len() == 64, "{s} 响应哈希缺席");
        assert_eq!(m["gate_verdict"], "stable_clear");
    }
}

// ---------- T2 四类工件逐字节一致 ----------

#[test]
fn t2_check_reports_byte_identical_all_scenarios() {
    for scenario in [
        "adisp-net", "p3xcarr-net",
        "dirty-suspend-near", "dirty-return-r2", "dirty-alarm-r7",
    ] {
        let ws = build_ws(scenario);
        let mat = ws.dir.join("material.json");
        let report = tally::check_material(&mat).unwrap_or_else(|e| panic!("{scenario}: {e}"));
        let exit: u8 = if report["verdict"] == "pass" { 0 } else { 1 };
        let mut got = canonical_json(&report).into_bytes();
        got.push(b'\n');
        assert_eq!(
            String::from_utf8_lossy(&got),
            detok_expect(scenario, &ws, &mat),
            "{scenario} 核对报告漂移"
        );
        let want_exit: u8 = read_expected(&format!("{scenario}/expected/check-exit.txt"))[0] - b'0';
        assert_eq!(exit, want_exit, "{scenario} 退出码漂移");
    }
}

#[test]
fn t2_signcheck_bytes_identical() {
    for scenario in ["adisp-net", "p3xcarr-net", "dirty-suspend-near", "dirty-return-r2", "dirty-alarm-r7"] {
        let ws = build_ws(scenario);
        let mat = ws.dir.join("material.json");
        let report = tally::check_material(&mat).unwrap();
        let mut got = canonical_json(&report).into_bytes();
        got.push(b'\n');
        let want_raw = read_expected(&format!("{scenario}/expected/signcheck.json"));
        let want = String::from_utf8(want_raw).unwrap()
            .replace("@MATERIAL@", &mat.to_string_lossy())
            .replace("@ROOT@", &ws.dir.to_string_lossy());
        assert_eq!(String::from_utf8_lossy(&got), want, "{scenario} 终签锚漂移");
    }
}

#[test]
fn t2_score_material_byte_identical() {
    for (gid, scenario) in [("adisp-guard-1", "adisp-net"), ("m-p3xcarr", "p3xcarr-net")] {
        let ws = build_ws(scenario);
        // 计分工作区：合同与响应在工作区根，trail 缺席（新鲜飞轮）
        fs::copy(ws.dir.join("input/contract.json"), ws.dir.join("contract.json")).unwrap();
        fs::copy(ws.dir.join("input/responses.jsonl"), ws.dir.join("responses.jsonl")).unwrap();
        let expected_raw = read_expected(&format!("{gid}-net/expected/score-material.json"));
        let expected: Value = serde_json::from_slice(&expected_raw).unwrap();
        let gate = expected["gate_verdict"].as_str().unwrap().to_string();
        let baseline: Value = serde_json::from_str(
            &fs::read_to_string(ws.dir.join("input/seat-baseline.json")).unwrap()).unwrap();
        let ih = baseline["identity_hash"].as_str().unwrap().to_string();
        let out_path = ws.dir.join("score-material.json");
        contract_mode::score_pipeline(
            &ws.dir.join("contract.json"),
            &ws.dir.join("responses.jsonl"),
            &ws.dir.join("flywheel-trail.jsonl"),
            &out_path,
            &ih,
            &gate,
        )
        .unwrap();
        let got = fs::read_to_string(&out_path).unwrap();
        let want = String::from_utf8(expected_raw).unwrap()
            .replace("@ROOT@", &ws.dir.to_string_lossy());
        assert_eq!(got, want, "{gid} 计分材料漂移");
    }
}

#[test]
fn t2_contract_emit_byte_identical() {
    let g = golden();
    let ws = build_ws("adisp-net");
    let atoms = sih_engine::attractor::paradigm_loader::load_atoms(
        &g.join("contract-emit/atom.yaml")).unwrap();
    let ng_raw = fs::read_to_string(g.join("contract-emit/ng-medium.txt")).unwrap();
    let ng = contract_mode::scheme_clipped_ng(&ng_raw);
    assert_eq!(
        contract_mode::sha256_bytes(ng.as_bytes()),
        "d4f847016697d5be0701ce7630e49b1ec328a2ace64169a3cdd1bd49c99af32c"
    );
    let topic_path = ws.dir.join("input/topic.md");
    let out_path = ws.dir.join("contract.json");
    let topic_md_text = fs::read_to_string(&topic_path).unwrap();
    let (system_prompt, user_prompt) =
        contract_mode::build_integrator_prompts(&atoms, &topic_md_text, &ng);
    let shots = contract_mode::make_shots(&system_prompt, &user_prompt, "adisp-guard-1", 9, 1);
    contract_mode::emit_contract(
        &out_path,
        &serde_json::json!({"framework": "ZCode", "model_id": "GLM-5.3-Flash", "version": "self-reported"}),
        &serde_json::json!({"paradigm_id": "normative_convergence", "atom": "integrator",
                            "direction": "judge", "ng_label": "medium",
                            "ng_text_sha256": contract_mode::sha256_bytes(ng.as_bytes())}),
        &serde_json::json!({"gid": "adisp-guard-1",
                            "title": "拒直提守卫命题：lease 应增 pre-commit 守卫拦无模板 plain git commit，--no-verify 显式留痕放行",
                            "topic_sha256": contract_mode::sha256_file(&topic_path).unwrap()}),
        &shots,
        &serde_json::json!({"measurement_entry": topic_path.to_string_lossy(), "n_declared": 9}),
    )
    .unwrap();
    let got = fs::read_to_string(&out_path).unwrap();
    let want = String::from_utf8(read_expected("adisp-net/expected/contract.json")).unwrap()
        .replace("@TOPIC_ENTRY@", &topic_path.to_string_lossy());
    assert_eq!(got, want, "合同 emit 漂移");
}

#[test]
fn t2_rejection_envelopes_byte_identical() {
    for scenario in ["reject-missing-shot", "reject-key-mismatch", "reject-shot-misalign", "reject-raw-empty"] {
        let ws = build_ws(scenario);
        let contract = contract_mode::load_contract(&ws.dir.join("input/contract.json")).unwrap();
        let err = contract_mode::load_responses(&ws.dir.join("input/responses.jsonl"), &contract)
            .err()
            .unwrap_or_else(|| panic!("{scenario} 围堰拒收形未被引擎拒收"));
        let mut got = format!("{{\"error\": \"ValueError: {}\"}}", err.message()).into_bytes();
        got.push(b'\n');
        assert_eq!(
            String::from_utf8_lossy(&got),
            String::from_utf8_lossy(&read_expected(&format!("{scenario}/expected/reject.json"))),
            "{scenario} 拒收消息漂移"
        );
    }
}

#[test]
fn t2_frozen_golden_zero_drift_after_freeze() {
    // 金向量冻结后零漂移（红线）：期望件哈希重算对表
    let pairs = [
        ("adisp-net/expected/check-report.json", "740f0f45961bc9ab"),
        ("p3xcarr-net/expected/check-report.json", "0b8897b704ce52fe"),
        ("dirty-suspend-near/expected/check-report.json", "e4eebf46a8c01538"),
        ("dirty-return-r2/expected/check-report.json", "a29ccb95626b6ef7"),
        ("dirty-alarm-r7/expected/check-report.json", "b2a97bcc8456c7c6"),
    ];
    use sha2::{Digest, Sha256};
    for (rel, want) in pairs {
        let bytes = read_expected(rel);
        let got = format!("{:x}", Sha256::digest(&bytes));
        assert_eq!(&got[..16], want, "{rel} 冻结后漂移");
    }
}

#[test]
fn t2_stats_conclusion_equivalence() {
    // 统计面判据（SPEC-014 腿切分清单边界声明）：统计结论等值 + 容差比对，
    // 不取字节级（浮点文本形显式范畴排除）。参照值由围堰原件跑出冻结。
    use serde_json::json;
    use sih_engine::attractor::stats;
    let v: Value = serde_json::from_slice(&read_expected("stats-values.json")).unwrap();
    let f64s = |node: &Value, idx: usize| -> f64 {
        node.as_array().unwrap()[idx].as_f64().unwrap()
    };
    let flat = |node: &Value, idx: usize| -> f64 { node.as_array().unwrap()[idx].as_f64().unwrap() };
    let close = |a: f64, b: f64| (a - b).abs() <= 1e-9 * f64::max(1.0, b.abs());
    let jv = |xs: &[f64]| xs.iter().map(|x| json!(x)).collect::<Vec<_>>();

    let hp: &[&[f64]] = &[&[0.25, 0.75], &[1.0, 0.0], &[60.0, 0.0, 0.0], &[3.0, 7.0]];
    let hq: &[&[f64]] = &[&[0.25, 0.75], &[0.0, 1.0], &[58.0, 2.0, 0.0], &[4.0, 6.0]];
    for i in 0..hp.len() {
        let got = stats::hellinger_distance(&jv(hp[i]), &jv(hq[i])).unwrap();
        assert!(close(got, flat(&v["hellinger"], i)), "hellinger[{i}] {got} vs {}", flat(&v["hellinger"], i));
    }
    let tp: &[&[f64]] = &[&[3.0, 7.0], &[1.0, 0.0], &[60.0, 0.0, 0.0], &[1.0, 0.0]];
    let tq: &[&[f64]] = &[&[3.0, 7.0], &[0.0, 1.0], &[58.0, 2.0, 0.0], &[0.5, 0.5]];
    for i in 0..tp.len() {
        let got = stats::tv_distance(&jv(tp[i]), &jv(tq[i])).unwrap();
        assert!(close(got, flat(&v["tv"], i)), "tv[{i}] {got} vs {}", flat(&v["tv"], i));
    }
    let jp: &[&[f64]] = &[&[0.2, 0.8], &[1.0, 0.0], &[1.0, 0.0], &[10.0, 20.0, 30.0]];
    let jq: &[&[f64]] = &[&[0.2, 0.8], &[0.0, 1.0], &[0.5, 0.5], &[30.0, 20.0, 10.0]];
    for i in 0..jp.len() {
        let got = stats::js_divergence(&jv(jp[i]), &jv(jq[i])).unwrap();
        assert!(close(got, flat(&v["js"], i)), "js[{i}] {got} vs {}", flat(&v["js"], i));
    }
    let cp: &[&[f64]] = &[&[3.0, 5.0, 7.0], &[3.0, 0.0], &[60.0, 0.0, 0.0], &[1.0, 1.0]];
    let cq: &[&[f64]] = &[&[3.0, 5.0, 7.0], &[0.0, 5.0], &[58.0, 2.0, 0.0], &[1.0, 0.0]];
    for i in 0..cp.len() {
        let got = stats::generalized_jaccard(&jv(cp[i]), &jv(cq[i])).unwrap();
        assert!(close(got, flat(&v["jaccard"], i)), "jaccard[{i}] {got} vs {}", flat(&v["jaccard"], i));
    }
    // 二项检验锚点
    let binom = &v["binom"];
    assert!(close(stats::binomial_test_pvalue(60, 60, 0.5).unwrap(), f64s(binom, 0)));
    assert!(close(stats::binomial_test_pvalue(0, 60, 0.5).unwrap(), f64s(binom, 1)));
    assert!(close(stats::binomial_test_pvalue(3, 10, 0.5).unwrap(), f64s(binom, 2)));
    assert!(close(stats::boundary_test_pvalue(2, 100, 0.01).unwrap(), f64s(binom, 3)));
    // 校正族锚点
    let bonf0 = v["bonferroni"].as_array().unwrap()[0].as_array().unwrap().clone();
    let got_bonf0 = stats::bonferroni_correction(&[json!(0.01), json!(0.04)], 0.05).unwrap();
    for (i, g) in got_bonf0.iter().enumerate() {
        assert!(close(*g, bonf0[i].as_f64().unwrap()));
    }
    let bh0 = v["bh"].as_array().unwrap()[0].as_array().unwrap().clone();
    let got_bh0 = stats::bh_fdr(&[json!(0.001), json!(0.008), json!(0.039), json!(0.041), json!(0.042)], 0.05).unwrap();
    for (i, g) in got_bh0.iter().enumerate() {
        assert!(close(*g, bh0[i].as_f64().unwrap()));
    }
    // 功效锚点
    let power = &v["power"];
    assert!(close(stats::statistical_power(10, 0.5, 0.9, 0.05).unwrap(), f64s(power, 1)));
    assert!(stats::statistical_power(60, 0.5, 0.8, 0.05).unwrap() >= 0.99, "功效分析结论等值");
    // ANOVA 结构与结论
    let anova = &v["anova"][0];
    let got = stats::mixed_effects_anova(
        &[
            (("a1".into(), "b1".into()), vec![1.0, 2.0, 3.0]),
            (("a1".into(), "b2".into()), vec![4.0, 5.0, 6.0]),
            (("a2".into(), "b1".into()), vec![7.0, 8.0, 9.0]),
            (("a2".into(), "b2".into()), vec![10.0, 11.0, 12.0]),
        ],
        ("factor_a", "factor_b"),
    ).unwrap();
    assert!(close(got["grand_mean"].as_f64().unwrap(), anova["grand_mean"].as_f64().unwrap()));
    assert_eq!(got["n_obs"], anova["n_obs"]);
    let want_pa = anova["factors"]["factor_a"]["p"].as_f64().unwrap();
    assert!(close(got["factors"]["factor_a"]["p"].as_f64().unwrap(), want_pa), "主效应 p 值等值");
    assert!(want_pa < 1e-4, "围堰参照主效应显著");
    // 置换检验：结论等值（强重叠 p < 0.01、零交集 p = 1.0、同 seed 可复现）
    let concl = &v["conclusion"];
    let universe: Vec<String> = (0..10).map(|i| format!("p{i}")).collect();
    let factories: Vec<(String, Vec<String>)> = (0..3)
        .map(|i| (format!("f{i}"), (0..8).map(|j| format!("p{j}")).collect()))
        .collect();
    let p1 = stats::permutation_test_shared_convergence(&factories, Some(&universe), 2000, 7).unwrap();
    assert!(p1 < 0.01, "强重叠结论等值：{p1} vs 围堰 {}", concl["perm_strong_overlap_p_lt_0.01"]);
    let p2 = stats::permutation_test_shared_convergence(
        &[("a".into(), vec!["x".into()])],
        Some(&["x".into(), "y".into()]), 500, 1).unwrap();
    assert_eq!(p2, 1.0, "零交集恒一");
    let p3 = stats::permutation_test_shared_convergence(&factories, Some(&universe), 2000, 7).unwrap();
    assert_eq!(p1, p3, "同 seed 双跑逐字节一致");
}
