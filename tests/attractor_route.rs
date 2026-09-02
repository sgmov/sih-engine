//! autoflow2-solo T1+T2+T3+T5+T6：路由谓词融回金向量与退出码全表与机械腿
//! 不变量与截流装配位。
//!
//! T1：金向量冻结（围堰 selector 实测输出，八场景）。
//! T2：引擎 route 输出对金向量逐字节 cmp + 退出码一致（同参形：包数据
//!     逐字节随迁、材料同源、参照时间同值、绝对路径运行）。
//! T3：route 退出码三值全表（0 净、1 告警、2 包非法或材料缺失或解析失败）。
//! T5：机械腿零网络零 LLM 零 key 读取（源码扫描 + Cargo 依赖断言）。
//! T6：截流装配位两公共函数走谓词机。
//!
//! T4（谓词机语义）在 src/attractor/route.rs 模块内单测承载。

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use sih_engine::attractor::route;
use sih_engine::ask3repeater::intercept;

fn golden() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/attractor/fixtures/route/golden")
}

fn engine_packs() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/attractor/packs")
}

fn bin_attractor() -> &'static str {
    env!("CARGO_BIN_EXE_attractor")
}

fn pack_worktree() -> PathBuf {
    // 围堰原包绝对路径：仅用于跨对表（同参形条款的包路径差异是调用形差异，
    // 包数据逐字节随迁由 T0 承载）
    PathBuf::from("/Users/moc/workspaces/SiHankor/sih-tools/selector/packs")
}

fn run_bin(args: &[&str]) -> (i32, Vec<u8>, Vec<u8>) {
    let out = Command::new(bin_attractor()).args(args).output().unwrap();
    (out.status.code().unwrap_or(-1), out.stdout, out.stderr)
}

// ---------- T0 包数据逐字节随迁（同参形构成条件） ----------

#[test]
fn t0_migrated_packs_byte_identical() {
    for pack in ["core", "parking"] {
        for file in ["manifest.toml", "routes.toml"] {
            let src = pack_worktree().join(pack).join(file);
            let dst = engine_packs().join(pack).join(file);
            if !src.exists() {
                // 围堰不在位的机器（如纯引擎 CI）跳过跨对表，随迁件自校验由 T2 承载
                continue;
            }
            assert_eq!(
                fs::read(&src).unwrap(),
                fs::read(&dst).unwrap(),
                "随迁包 {pack}/{file} 与围堰原包须逐字节一致"
            );
        }
    }
}

// ---------- T1 金向量冻结 ----------

#[test]
fn t1_golden_frozen() {
    for scenario in [
        "net-core",
        "net-parking",
        "dirty-schema",
        "dirty-anchor-domain",
        "dirty-expired",
        "badpack-kind",
        "attribution-core",
        "attribution-parking",
    ] {
        let dir = golden().join(scenario);
        assert!(dir.join("expected.json").exists(), "金向量缺席：{scenario}/expected.json");
        assert!(dir.join("input").is_dir(), "金向量输入缺席：{scenario}/input");
    }
}

fn golden_scenarios() -> Vec<(&'static str, Option<&'static str>, &'static str, i32)> {
    // (场景, 参照时间, 包, 围堰冻结退出码)
    vec![
        ("net-core", None, "core", 0),
        ("net-parking", Some("2026-09-02"), "parking", 0),
        ("dirty-schema", None, "core", 1),
        ("dirty-anchor-domain", None, "core", 1),
        ("dirty-expired", Some("2026-09-02"), "parking", 1),
        ("attribution-core", None, "core", 0),
        ("attribution-parking", Some("2026-09-02"), "parking", 1),
    ]
}

// ---------- T2 金向量逐字节一致 ----------

#[test]
fn t2_golden_byte_identical() {
    for (scenario, reference_time, pack_name, expected_code) in golden_scenarios() {
        let dir = golden().join(scenario);
        let input = dir.join("input");
        let pack = engine_packs().join(pack_name);
        let mut args = vec!["route".to_string(), "--pack".to_string(), pack.to_string_lossy().to_string()];
        if let Some(rt) = reference_time {
            args.push("--reference-time".to_string());
            args.push(rt.to_string());
        }
        args.push(input.to_string_lossy().to_string());
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let (code, out, err) = run_bin(&arg_refs);
        let expected = fs::read(dir.join("expected.json")).unwrap();
        assert_eq!(code, expected_code, "{scenario} 退出码须对齐围堰冻结值，stderr={}", String::from_utf8_lossy(&err));
        assert_eq!(
            out, expected,
            "{scenario} 引擎 route 输出须与围堰冻结金向量逐字节一致"
        );
    }
}

#[test]
fn t2_badpack_envelope_byte_identical() {
    let dir = golden().join("badpack-kind");
    let pack = dir.join("input").join("pack");
    let mat = dir.join("input").join("m1.json");
    let (code, out, _err) = run_bin(&[
        "route",
        "--pack",
        &pack.to_string_lossy(),
        &mat.to_string_lossy(),
    ]);
    let expected_text =
        fs::read_to_string(dir.join("expected.json")).unwrap().replace("@PACKDIR@", &pack.to_string_lossy());
    assert_eq!(code, 2, "未知谓词 kind 须拒包 exit 2");
    assert_eq!(String::from_utf8_lossy(&out), expected_text, "拒包错误信封须逐字节一致");
}

// ---------- T3 退出码全表 ----------

#[test]
fn t3_exit_code_table() {
    let base = golden();
    // 0：净目标无告警
    let (code, _, _) = run_bin(&[
        "route",
        "--pack",
        &engine_packs().join("core").to_string_lossy(),
        &base.join("net-core").join("input").to_string_lossy(),
    ]);
    assert_eq!(code, 0);
    // 1：路由毕有告警（饥饿告警）
    let (code, out, _) = run_bin(&[
        "route",
        "--pack",
        &engine_packs().join("core").to_string_lossy(),
        &base.join("dirty-schema").join("input").to_string_lossy(),
    ]);
    assert_eq!(code, 1, "有告警须 1，stdout={}", String::from_utf8_lossy(&out));
    // 2：包非法（未知 kind）
    let (code, _, _) = run_bin(&[
        "route",
        "--pack",
        &base.join("badpack-kind").join("input").join("pack").to_string_lossy(),
        &base.join("badpack-kind").join("input").join("m1.json").to_string_lossy(),
    ]);
    assert_eq!(code, 2);
    // 2：包目录缺席
    let (code, out, _) = run_bin(&[
        "route",
        "--pack",
        "/nonexistent/autoflow2-pack",
        &base.join("net-core").join("input").to_string_lossy(),
    ]);
    assert_eq!(code, 2, "包缺席须 2，stdout={}", String::from_utf8_lossy(&out));
    assert!(String::from_utf8_lossy(&out).contains("pack directory missing"));
    // 2：材料缺席
    let (code, _, _) = run_bin(&[
        "route",
        "--pack",
        &engine_packs().join("core").to_string_lossy(),
        "/nonexistent/autoflow2-material.json",
    ]);
    assert_eq!(code, 2);
    // 2：材料 json 非法
    let tmp = tempfile::tempdir().unwrap();
    let bad = tmp.path().join("bad.json");
    fs::write(&bad, "{not json").unwrap();
    let (code, _, _) = run_bin(&[
        "route",
        "--pack",
        &engine_packs().join("core").to_string_lossy(),
        &bad.to_string_lossy(),
    ]);
    assert_eq!(code, 2);
    // 2：材料顶层非对象
    let arr = tmp.path().join("arr.json");
    fs::write(&arr, "[1,2]\n").unwrap();
    let (code, out, _) = run_bin(&[
        "route",
        "--pack",
        &engine_packs().join("core").to_string_lossy(),
        &arr.to_string_lossy(),
    ]);
    assert_eq!(code, 2);
    assert!(String::from_utf8_lossy(&out).contains("top level must be an object"));
    // 2：reference-time 非法
    let (code, _, _) = run_bin(&[
        "route",
        "--pack",
        &engine_packs().join("parking").to_string_lossy(),
        "--reference-time",
        "not-a-date",
        &base.join("net-parking").join("input").to_string_lossy(),
    ]);
    assert_eq!(code, 2);
    // 0：空批绿态（零材料零告警）
    let empty = tempfile::tempdir().unwrap();
    let (code, out, _) = run_bin(&[
        "route",
        "--pack",
        &engine_packs().join("core").to_string_lossy(),
        &empty.path().to_string_lossy(),
    ]);
    assert_eq!(code, 0, "空批绿态须 0");
    let report: Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(report["summary"]["total"], 0);
    assert_eq!(report["status"], "routed");
}

use serde_json::Value;

// ---------- T5 机械腿不变量 ----------

#[test]
fn t5_zero_network_zero_llm_source_scan() {
    let sources = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/attractor/route.rs"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ask3repeater/intercept.rs"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/bin/attractor.rs"),
    ];
    let forbidden = [
        "reqwest", "hyper", "tokio::net", "TcpStream", "openai", "OpenAI", "llm_client",
        "api_key", "API_KEY", "http://", "https://",
    ];
    for src in sources {
        let text = fs::read_to_string(&src).unwrap();
        for f in forbidden {
            assert!(!text.contains(f), "{} 含禁词 {}（机械腿零网络零 LLM 红线）", src.display(), f);
        }
    }
}

#[test]
fn t5_zero_network_deps_in_cargo() {
    let cargo = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")).unwrap();
    for dep in ["reqwest", "hyper", "tokio", "openai"] {
        assert!(!cargo.contains(dep), "Cargo.toml 含网络或 LLM 依赖 {dep}");
    }
}

// ---------- T6 截流装配位 ----------

#[test]
fn t6_intercept_load_and_round() {
    // 装配位：读 core 包装载入三问
    let pack = intercept::load_intercept_pack(&engine_packs().join("core")).unwrap();
    assert_eq!(pack.name, "core");
    assert_eq!(pack.predicates.len(), 4);
    // 按轮判定：常设约束缺回执判败（fail-closed 语义经谓词机）
    let material: Value = serde_json::json!({
        "id": "round-m1",
        "path": "task-packages/round-m1.md",
        "state": "process",
        "anchors": ["doc/a.md"],
        "requested_writes": ["sih-engine/src/x.rs"],
        "round": {
            "assertions": [{"claim": "x", "evidence": ["doc/a.md"]}],
            "conclusions": [],
            "receipts": [{"tool": "formatter"}]
        }
    });
    let results = intercept::round_interception(&material, &pack, None);
    assert_eq!(results.len(), 4);
    let by_id = |id: &str| results.iter().find(|r| r["id"] == id).unwrap()["pass"]
        == Value::Bool(true);
    assert!(by_id("R001") && by_id("R002") && by_id("R003"), "单件族四件查材料层须过");
    // parking 包经装配位装载即时间族可用
    let parking = intercept::load_intercept_pack(&engine_packs().join("parking")).unwrap();
    assert_eq!(parking.predicates.len(), 3);
    // 包非法即装配位拒
    let bad = golden().join("badpack-kind").join("input").join("pack");
    assert!(intercept::load_intercept_pack(&bad).is_err(), "未知 kind 装配位须拒包");
}

#[test]
fn t6_ask3_existing_validate_zero_change() {
    // 三问既有验收行为零改：最小合法输出记录经验收函数过
    use sih_engine::ask3repeater::{
        validate, Anchor, Domain, DomainContract, DomainTag, InquiryStage, IntentContract,
        OutputRecord, PhilosophyRef,
    };
    let record = OutputRecord {
        session_id: "sess-t6-zero-change".to_string(),
        raw_input: "测试输入".to_string(),
        round: 1,
        intent_contract: IntentContract {
            goal: "目标".to_string(),
            exclusions: vec![],
            output_format: "格式".to_string(),
            injected_constraints: vec![],
        },
        domain_contract: DomainContract {
            target_domain: Domain { scope: "scope".to_string(), max_depth: 1 },
            support_domain: Domain { scope: "scope".to_string(), max_depth: 1 },
        },
        anchors: vec![Anchor {
            anchor_seq: 1,
            text: "锚文本".to_string(),
            inquiry_stage: InquiryStage::First,
            domain_tag: DomainTag::Target,
            depth: 0,
            philosophy_ref: PhilosophyRef {
                source: "sih-philosophy/emanation/proodos/08-on-settle.md".to_string(),
                quote: "留痕".to_string(),
            },
            rationale: "理据".to_string(),
            evidence: "sih-philosophy/emanation/proodos/08-on-settle.md:114".to_string(),
            confidence: 0.95,
        }],
        calls_in: 0,
        calls_out: 0,
    };
    let root = PathBuf::from("/Users/moc/workspaces/SiHankor");
    assert!(validate(&record, &root).is_ok(), "三问既有验收须零改全通：{:?}", validate(&record, &root));
}

// ---------- lib 面活体双跑探针（同参形） ----------

#[test]
fn t2_live_lib_route_matches_coferdam_output() {
    // lib 面 route_batch 直接对金向量：同材料同参照时间输出逐字节
    for (scenario, reference_time, pack_name, _) in golden_scenarios() {
        let dir = golden().join(scenario);
        let pack_dir = engine_packs().join(pack_name);
        let pack = route::load_pack(&pack_dir).unwrap();
        let mut materials = Vec::new();
        let mut paths: Vec<PathBuf> = fs::read_dir(dir.join("input"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().map(|e| e == "json").unwrap_or(false))
            .collect();
        paths.sort();
        for p in paths {
            let text = fs::read_to_string(&p).unwrap();
            materials.push(route::parse_material(&text, &p.to_string_lossy()).unwrap());
        }
        let report = route::build_report(&pack, &route::route_batch(&materials, &pack, reference_time), reference_time);
        let expected = fs::read_to_string(dir.join("expected.json")).unwrap();
        assert_eq!(
            format!("{}\n", route::report_json(&report)),
            expected,
            "{scenario} lib 面输出须与金向量逐字节一致"
        );
    }
}
