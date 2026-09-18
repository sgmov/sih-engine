//! pendline 批测试族（pl-05）：夹具回填件驱动的三态全链回归。
//! 正典指针：DES-017 候裁处置范式编排设计 v1
//!（doc/design/DES-017-pending-adjudication-orchestration-v1.md）与任务包
//! sih/state/plan/pendline.md 与人工活证
//! sih/event/plan/mcpdual-parallel-results.md 第九节（待裁过得一，裁一过一
//! 执行一，未过入泊）。
//! 覆盖面：collect 三源汇总；dispatch 档位 off/3/6/9 与缺省 3 与非法值
//! exit 2 与配置件读写两面；backfill 夹具回填件；route 三态路由全链
//!（collect→dispatch 出合同→注入夹具 responses→route 落链到临时 trail→
//! 断言链上事件类型与字段）与链 verify valid；零触碰断言（critsweep 与
//! attractor 与 lease 三件生产码 git 零 diff）。
//! 测试纪律：全部落临时目录临时链，零真实 trail 写，零模型调用，git 只读
//! 查询（status），零 git 写动作。

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

// ---------------------------------------------------------------- 底座

fn pendline_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_pendline"))
}

fn scribe_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_scribe"))
}

fn run(cmd: &mut Command) -> (i32, String, String) {
    let out = cmd.output().expect("子进程可启");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

fn write_file(p: &Path, s: &str) {
    if let Some(d) = p.parent() {
        std::fs::create_dir_all(d).unwrap();
    }
    std::fs::write(p, s).unwrap();
}

fn read_json(p: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
}

struct Ws {
    #[allow(dead_code)]
    root: PathBuf,
    guard: tempfile::TempDir,
}

fn temp_ws(tag: &str) -> Ws {
    let guard = tempfile::TempDir::new_in(std::env::temp_dir()).unwrap();
    let root = guard.path().join(tag);
    std::fs::create_dir_all(&root).unwrap();
    Ws { root, guard }
}

// ---------------------------------------------------------------- 夹具

/// 判据扫出参夹具：一件 sunk 沉底入候选，一件 achieved 不入。
fn critsweep_fixture(p: &Path) {
    write_file(
        p,
        r#"{
  "tool": "critsweep",
  "version": "1.2.0",
  "criteria": [
    {"id": "GOV-002-SDDG", "title": "SDDG 完备度判据", "status": "sunk", "gap_days": 12, "last_hit": null},
    {"id": "HEALTHY-1", "title": "已达成判据", "status": "achieved", "gap_days": 0}
  ]
}"#,
    );
}

/// 泊界 trail 夹具：pk-alpha 在泊，pk-beta 入后已出泊（不应入候选）。
fn parking_trail_fixture(p: &Path) {
    let enter_a = r#"{"event_id":"e1","event_type":"parking_entered","details":{"entry_id":"pk-alpha","title":"发现一 matches 字段两形","exit_condition":"用户裁统一或文档注明","ttl_days":30}}"#;
    let enter_b = r#"{"event_id":"e2","event_type":"parking_entered","details":{"entry_id":"pk-beta","title":"已出泊项","exit_condition":"已裁","ttl_days":30}}"#;
    let exit_b = r#"{"event_id":"e3","event_type":"parking_exited","details":{"entry_id":"pk-beta","disposition":"promoted","ruling":"已裁出"}}"#;
    write_file(p, &format!("{}\n{}\n{}\n", enter_a, enter_b, exit_b));
}

/// 批候裁单 md 夹具：两件自由文本候裁项。
fn covenant_md_fixture(p: &Path) {
    write_file(
        p,
        "# 批候裁单\n\n- 复核 AGENTS.md 工具层静态审计节调用形\n2. 清点 packs 目录并归位\n",
    );
}

/// 正身报告夹具（scribe direct agent 笔强制 --identity-report）。
fn identity_report_fixture(p: &Path) {
    let hash = "a".repeat(64);
    write_file(
        p,
        &format!("{{\"identity\": {{\"hash\": \"{}\"}}}}", hash),
    );
}

/// 夹具回填件：读合同 shots 逐发生成响应行（facet 纪律三键 key/shot/raw）。
fn responses_fixture(contract_path: &Path, out: &Path) {
    let contract = read_json(contract_path);
    let mut lines: Vec<String> = Vec::new();
    for shot in contract["shots"].as_array().unwrap() {
        let key = shot["key"].as_str().unwrap();
        let shot_n = shot["shot"].as_i64().unwrap();
        lines.push(format!(
            "{{\"key\":\"{}\",\"shot\":{},\"raw\":\"{{\\\"decision\\\":\\\"comply\\\",\\\"basis_regulation\\\":\\\"baseline\\\",\\\"boundary_flag\\\":false,\\\"reason\\\":\\\"fixture\\\"}}\"}}",
            key, shot_n
        ));
    }
    write_file(out, &format!("{}\n", lines.join("\n")));
}

/// 全链铺设：collect→dispatch→backfill，返回（工地目录，回填评分材料路径）。
fn full_chain(ws: &Ws, gears: &str, verdict: &str) -> PathBuf {
    let root = &ws.root;
    let crit = root.join("critsweep.json");
    let ptrail = root.join("parking.ndjson");
    let covenant = root.join("covenant.md");
    critsweep_fixture(&crit);
    parking_trail_fixture(&ptrail);
    covenant_md_fixture(&covenant);
    let candidates = root.join("candidates.json");
    let (rc, _, stderr) = run(
        pendline_bin().args([
            "collect",
            "--critsweep-json",
            crit.to_str().unwrap(),
            "--parking-trail",
            ptrail.to_str().unwrap(),
            "--covenant",
            covenant.to_str().unwrap(),
            "--out",
            candidates.to_str().unwrap(),
        ]),
    );
    assert_eq!(rc, 0, "collect 退出码非零：{}", stderr);

    let dispatch_dir = root.join("contracts");
    let (rc, _, stderr) = run(pendline_bin().args([
        "dispatch",
        "--candidates",
        candidates.to_str().unwrap(),
        "--out",
        dispatch_dir.to_str().unwrap(),
        "--gears",
        gears,
    ]));
    assert_eq!(rc, 0, "dispatch 退出码非零：{}", stderr);

    // 取候裁单第一件的合同注入夹具回填件。
    let contract_path = dispatch_dir.join("covenant-1.contract.json");
    assert!(contract_path.is_file(), "合同应落盘：{}", contract_path.display());
    let responses = root.join("responses.jsonl");
    responses_fixture(&contract_path, &responses);
    let material = root.join("backfill.json");
    let (rc, _, stderr) = run(pendline_bin().args([
        "backfill",
        "--contract",
        contract_path.to_str().unwrap(),
        "--responses",
        responses.to_str().unwrap(),
        "--verdict",
        verdict,
        "--out",
        material.to_str().unwrap(),
    ]));
    assert_eq!(rc, 0, "backfill 退出码非零：{}", stderr);
    material
}

/// 链事件读取：全部事件解析为 Value 数组。
fn trail_events(trail: &Path) -> Vec<Value> {
    std::fs::read_to_string(trail)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

/// 临时链路径：chains 子目录承载（与夹具 parking.ndjson 隔离——scribe 泊界
/// 重放面会解析 trail 同目录全部 ndjson），目录预建（scribe append 不建父目录）。
fn chain_trail(ws: &Ws, name: &str) -> PathBuf {
    let trail = ws.root.join("chains").join(format!("{}.ndjson", name));
    std::fs::create_dir_all(trail.parent().unwrap()).unwrap();
    trail
}

/// 链 verify valid 断言。
fn assert_trail_valid(trail: &Path) {
    let (rc, stdout, stderr) = run(&mut scribe_bin().args(["verify", "--trail", trail.to_str().unwrap()]));
    assert_eq!(rc, 0, "链 verify 应 valid：{} {}", stdout, stderr);
    assert!(stdout.contains("\"valid\""), "verify 判词应 valid：{}", stdout);
}

fn route_cmd<'a>(ws: &'a Ws, material: &Path, trail: &Path, report: &Path) -> Command {
    let ir = ws.root.join("identity.json");
    identity_report_fixture(&ir);
    let mut c = pendline_bin();
    c.args([
        "route",
        "--materials",
        material.to_str().unwrap(),
        "--scribe-binary",
        env!("CARGO_BIN_EXE_scribe"),
        "--trail",
        trail.to_str().unwrap(),
        "--identity-report",
        ir.to_str().unwrap(),
        "--no-session-reason",
        "pendline 无会话编排处置位（测试临时链）",
        "--out",
        report.to_str().unwrap(),
    ]);
    c
}

// ------------------------------------------------- 三态全链（pl-05 主线）

/// 过者全链：stable_clear → 执行位标记落链（直改笔 subject=execution_marked）
/// 且建议通道字段在报。
#[test]
fn route_passed_execution_mark_full_chain() {
    let ws = temp_ws("pendline-passed");
    let material = full_chain(&ws, "3", "stable_clear");
    let trail = chain_trail(&ws, "trail-a");
    let report = ws.root.join("route-report.json");
    let (rc, stdout, stderr) = run(&mut route_cmd(&ws, &material, &trail, &report));
    assert_eq!(rc, 0, "route 应绿：{} {}", stdout, stderr);

    let rep = read_json(&report);
    let r0 = &rep["results"][0];
    assert_eq!(r0["disposition"], "passed");
    assert_eq!(r0["action"], "execution_marked");
    assert_eq!(r0["channel"], "batch", "缺省建议通道应为 batch");
    assert_eq!(r0["chain_event_type"], "direct_edit_completed");

    let events = trail_events(&trail);
    let mark = events
        .iter()
        .find(|e| e["event_type"] == "direct_edit_completed")
        .expect("链上应有执行位标记事件");
    assert_eq!(
        mark["details"]["subject"], "execution_marked",
        "直改笔 subject 应承载执行位标记名"
    );
    assert_eq!(mark["details"]["pen_kind"], "agent");
    assert_eq!(mark["details"]["pen_form"], "direct");
    let files = mark["details"]["files"].as_array().unwrap();
    assert!(
        files.iter().any(|f| f.as_str().unwrap().contains("backfill.json")),
        "标记 files 应携带评分材料指针"
    );
    assert!(events.iter().all(|e| e["event_type"] != "parking_entered"));
    assert_trail_valid(&trail);
}

/// 未过全链：near_threshold（其余=未过）→ scribe park 入泊，泊位四要害
/// 照录判词与链 valid。
#[test]
fn route_failed_park_full_chain() {
    let ws = temp_ws("pendline-failed");
    let material = full_chain(&ws, "3", "near_threshold");
    let trail = chain_trail(&ws, "trail-b");
    let report = ws.root.join("route-report.json");
    let (rc, stdout, stderr) = run(&mut route_cmd(&ws, &material, &trail, &report));
    assert_eq!(rc, 0, "route 应绿：{} {}", stdout, stderr);

    let rep = read_json(&report);
    let r0 = &rep["results"][0];
    assert_eq!(r0["disposition"], "failed");
    assert_eq!(r0["action"], "parking_entered");
    assert_eq!(r0["chain_event_type"], "parking_entered");
    assert_eq!(r0["ttl_days"], 30, "缺省 ttl 应 30");

    let events = trail_events(&trail);
    let park = events
        .iter()
        .find(|e| e["event_type"] == "parking_entered")
        .expect("链上应有停泊事件");
    assert_eq!(park["details"]["entry_id"], "pendline-covenant-1");
    assert_eq!(park["details"]["ttl_days"], 30);
    assert!(
        park["details"]["exit_condition"]
            .as_str()
            .unwrap()
            .contains("covenant-1"),
        "出泊条件应照录判词要害"
    );
    assert_eq!(park["details"]["context"]["verdict"], "near_threshold");
    assert_trail_valid(&trail);
}

/// boundary 全链：人重写标记落链，禁自动入泊禁自动重测（链上零停泊事件、
/// 零新合同产生——重测面在编排层结构性缺席）。
#[test]
fn route_boundary_human_rewrite_full_chain() {
    let ws = temp_ws("pendline-boundary");
    let material = full_chain(&ws, "3", "boundary");
    let trail = chain_trail(&ws, "trail-c");
    let report = ws.root.join("route-report.json");
    let (rc, stdout, stderr) = run(&mut route_cmd(&ws, &material, &trail, &report));
    assert_eq!(rc, 0, "route 应绿：{} {}", stdout, stderr);

    let rep = read_json(&report);
    let r0 = &rep["results"][0];
    assert_eq!(r0["disposition"], "boundary");
    assert_eq!(r0["action"], "human_rewrite_marked");

    let events = trail_events(&trail);
    let mark = events
        .iter()
        .find(|e| e["event_type"] == "direct_edit_completed")
        .expect("链上应有人重写标记事件");
    assert_eq!(
        mark["details"]["subject"], "human_rewrite_marked",
        "直改笔 subject 应承载人重写标记名"
    );
    // 禁自动入泊：boundary 分支链上零停泊事件。
    assert!(
        events.iter().all(|e| e["event_type"] != "parking_entered"),
        "boundary 禁自动入泊"
    );
    assert_trail_valid(&trail);
}

/// 混批路由：三态同批各一件，链上事件齐备且 verify valid。
#[test]
fn route_mixed_three_states_on_chain() {
    let ws = temp_ws("pendline-mixed");
    let m_pass = full_chain(&ws, "3", "stable_clear");
    let m_fail = {
        let m = ws.root.join("backfill-fail.json");
        let v = read_json(&m_pass);
        let mut v2 = v.clone();
        v2["verdict"] = serde_json::json!("unstable");
        v2["gate_verdict"] = serde_json::json!("unstable");
        write_file(&m, &serde_json::to_string_pretty(&v2).unwrap());
        m
    };
    let m_bdy = {
        let m = ws.root.join("backfill-bdy.json");
        let v = read_json(&m_pass);
        let mut v2 = v.clone();
        v2["entry_id"] = serde_json::json!("bdy-entry");
        v2["verdict"] = serde_json::json!("boundary");
        v2["gate_verdict"] = serde_json::json!("boundary");
        write_file(&m, &serde_json::to_string_pretty(&v2).unwrap());
        m
    };
    let trail = chain_trail(&ws, "trail-d");
    let ir = ws.root.join("identity.json");
    identity_report_fixture(&ir);
    let (rc, stdout, stderr) = run(
        pendline_bin()
            .arg("route")
            .args([
                "--materials",
                m_pass.to_str().unwrap(),
                "--materials",
                m_fail.to_str().unwrap(),
                "--materials",
                m_bdy.to_str().unwrap(),
                "--scribe-binary",
                env!("CARGO_BIN_EXE_scribe"),
                "--trail",
                trail.to_str().unwrap(),
                "--identity-report",
                ir.to_str().unwrap(),
                "--no-session-reason",
                "pendline 混批测试",
                "--ttl-days",
                "7",
            ]),
    );
    assert_eq!(rc, 0, "混批 route 应绿：{} {}", stdout, stderr);

    let events = trail_events(&trail);
    let types: Vec<&str> = events.iter().filter_map(|e| e["event_type"].as_str()).collect();
    assert_eq!(types.iter().filter(|t| **t == "parking_entered").count(), 1);
    assert_eq!(types.iter().filter(|t| **t == "direct_edit_completed").count(), 2);
    let parked = events
        .iter()
        .find(|e| e["event_type"] == "parking_entered")
        .unwrap();
    assert_eq!(parked["details"]["ttl_days"], 7, "--ttl-days 应透传缺省 ttl");
    assert_trail_valid(&trail);
}

// ------------------------------------------------- 档位面（pl-02/pl-04）

fn dispatch_only(ws: &Ws, gears: &[&str]) -> (i32, String, PathBuf) {
    let candidates = ws.root.join("candidates.json");
    if !candidates.is_file() {
        let crit = ws.root.join("critsweep.json");
        let covenant = ws.root.join("covenant.md");
        critsweep_fixture(&crit);
        covenant_md_fixture(&covenant);
        let (rc, _, stderr) = run(
            pendline_bin().args([
                "collect",
                "--critsweep-json",
                crit.to_str().unwrap(),
                "--covenant",
                covenant.to_str().unwrap(),
                "--out",
                candidates.to_str().unwrap(),
            ]),
        );
        assert_eq!(rc, 0, "{}", stderr);
    }
    let dir = ws.root.join(format!(
        "out-{}",
        gears.join("-").replace('/', "_")
    ));
    let mut c = pendline_bin();
    c.args([
        "dispatch",
        "--candidates",
        candidates.to_str().unwrap(),
        "--out",
        dir.to_str().unwrap(),
    ]);
    if !gears.is_empty() {
        c.args(["--gears", gears[0]]);
    }
    let (rc, stdout, _) = run(&mut c);
    (rc, stdout, dir)
}

#[test]
fn dispatch_gear_shots_contract_shape() {
    for (gear, shots) in [("3", 3), ("6", 6), ("9", 9)] {
        let ws = temp_ws("pendline-gear");
        let (rc, stdout, dir) = dispatch_only(&ws, &[gear]);
        assert_eq!(rc, 0, "gear {} dispatch 应绿：{}", gear, stdout);
        let manifest = read_json(&dir.join("manifest.json"));
        assert_eq!(manifest["mode"], "sampling");
        assert_eq!(manifest["gears"], shots);
        let contracts = manifest["contracts"].as_array().unwrap();
        assert_eq!(contracts.len(), 3, "三候选应各一份合同（沉底一加候裁单二）");
        for c in contracts {
            let cp = PathBuf::from(c["path"].as_str().unwrap());
            let contract = read_json(&cp);
            assert_eq!(contract["kind"], "facet-sampling-contract", "合同形对齐 facet");
            assert_eq!(contract["shots"].as_array().unwrap().len(), shots as usize);
            assert_eq!(contract["meta"]["gear"], shots, "meta.gear 应记档位");
            assert!(
                contract["proposition"]["text"].as_str().unwrap().len() > 0,
                "合同应含命题文本"
            );
            assert!(contract["seat"]["framework"].is_string(), "合同应含席位框架");
        }
    }
}

#[test]
fn dispatch_default_gear_is_3() {
    let ws = temp_ws("pendline-default");
    let (rc, _, dir) = dispatch_only(&ws, &[]);
    assert_eq!(rc, 0);
    let manifest = read_json(&dir.join("manifest.json"));
    assert_eq!(manifest["gears"], 3, "缺省档位应 3 低档（DES-017 令源原字）");
}

#[test]
fn dispatch_off_zero_sampling_manual_list() {
    let ws = temp_ws("pendline-off");
    let (rc, stdout, dir) = dispatch_only(&ws, &["off"]);
    assert_eq!(rc, 0, "{}", stdout);
    let manifest = read_json(&dir.join("manifest.json"));
    assert_eq!(manifest["mode"], "manual", "off 即零采样直呈全人工");
    assert!(manifest["contracts"].as_array().unwrap().is_empty());
    let entries = std::fs::read_dir(&dir).unwrap().count();
    assert_eq!(entries, 1, "off 档目录应仅 manifest.json 零采样合同");
    assert!(manifest["candidates"].as_array().unwrap().len() == 3);
}

#[test]
fn dispatch_illegal_gear_exit_2() {
    let ws = temp_ws("pendline-illegal");
    let (rc, stdout, _) = dispatch_only(&ws, &["4"]);
    assert_eq!(rc, 2, "档位值域外应 exit 2：{}", stdout);
    let (rc2, _, _) = dispatch_only(&ws, &["0"]);
    assert_eq!(rc2, 2);
}

#[test]
fn config_read_write_both_faces() {
    let ws = temp_ws("pendline-config");
    let cfg = ws.root.join("pendline-config.json");

    // 读面：缺文件如实报缺省 3。
    let (rc, stdout, _) = run(
        pendline_bin()
            .arg("config")
            .arg("--config")
            .arg(cfg.to_str().unwrap())
            .arg("--get"),
    );
    assert_eq!(rc, 0);
    assert!(stdout.contains("\"default\""), "缺文件应报缺省来源：{}", stdout);

    // 写面：set 6 落盘。
    let (rc, _, stderr) = run(
        pendline_bin()
            .arg("config")
            .arg("--config")
            .arg(cfg.to_str().unwrap())
            .arg("--set")
            .arg("6"),
    );
    assert_eq!(rc, 0, "{}", stderr);
    let cfgv = read_json(&cfg);
    assert_eq!(cfgv["gears"], 6, "配置件应持存 gears=6");

    // 读面：回读 6。
    let (rc, stdout, _) = run(
        pendline_bin()
            .arg("config")
            .arg("--config")
            .arg(cfg.to_str().unwrap())
            .arg("--get"),
    );
    assert_eq!(rc, 0);
    assert!(stdout.contains("6"), "get 应回读 6：{}", stdout);

    // dispatch 无旗标走配置件 6。
    let candidates = ws.root.join("candidates.json");
    let crit = ws.root.join("critsweep.json");
    let covenant = ws.root.join("covenant.md");
    critsweep_fixture(&crit);
    covenant_md_fixture(&covenant);
    let (rc, _, stderr) = run(
        pendline_bin().args([
            "collect",
            "--critsweep-json",
            crit.to_str().unwrap(),
            "--covenant",
            covenant.to_str().unwrap(),
            "--out",
            candidates.to_str().unwrap(),
        ]),
    );
    assert_eq!(rc, 0, "{}", stderr);
    let dir = ws.root.join("out-cfg");
    let (rc, _, stderr) = run(
        pendline_bin().args([
            "dispatch",
            "--candidates",
            candidates.to_str().unwrap(),
            "--out",
            dir.to_str().unwrap(),
            "--config",
            cfg.to_str().unwrap(),
        ]),
    );
    assert_eq!(rc, 0, "{}", stderr);
    let contract = read_json(&dir.join("covenant-1.contract.json"));
    assert_eq!(contract["shots"].as_array().unwrap().len(), 6, "配置件档位应在役");

    // CLI 旗标压过配置件。
    let dir2 = ws.root.join("out-cli");
    let (rc, _, _) = run(
        pendline_bin().args([
            "dispatch",
            "--candidates",
            candidates.to_str().unwrap(),
            "--out",
            dir2.to_str().unwrap(),
            "--config",
            cfg.to_str().unwrap(),
            "--gears",
            "3",
        ]),
    );
    assert_eq!(rc, 0);
    let contract2 = read_json(&dir2.join("covenant-1.contract.json"));
    assert_eq!(contract2["shots"].as_array().unwrap().len(), 3, "CLI 旗标优先");

    // 配置件值域外即 exit 2（读写两面各一）。
    write_file(&cfg, "{\"gears\": 7}\n");
    let (rc, _, _) = run(
        pendline_bin()
            .arg("config")
            .arg("--config")
            .arg(cfg.to_str().unwrap())
            .arg("--get"),
    );
    assert_eq!(rc, 2, "配置件值域外 get 应 exit 2");
    let (rc, _, _) = run(
        pendline_bin().args([
            "dispatch",
            "--candidates",
            candidates.to_str().unwrap(),
            "--out",
            ws.root.join("out-bad").to_str().unwrap(),
            "--config",
            cfg.to_str().unwrap(),
        ]),
    );
    assert_eq!(rc, 2, "配置件值域外 dispatch 应 exit 2");
    let (rc, _, _) = run(
        pendline_bin()
            .arg("config")
            .arg("--config")
            .arg(cfg.to_str().unwrap())
            .arg("--set")
            .arg("12"),
    );
    assert_eq!(rc, 2, "set 值域外应 exit 2");
}

// ------------------------------------------------- collect 面（pl-01）

#[test]
fn collect_three_sources_unified() {
    let ws = temp_ws("pendline-collect");
    let crit = ws.root.join("critsweep.json");
    let ptrail = ws.root.join("parking.ndjson");
    let covenant = ws.root.join("covenant.md");
    critsweep_fixture(&crit);
    parking_trail_fixture(&ptrail);
    covenant_md_fixture(&covenant);
    let out = ws.root.join("candidates.json");
    let (rc, stdout, stderr) = run(
        pendline_bin().args([
            "collect",
            "--critsweep-json",
            crit.to_str().unwrap(),
            "--parking-trail",
            ptrail.to_str().unwrap(),
            "--covenant",
            covenant.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ]),
    );
    assert_eq!(rc, 0, "{}{}", stdout, stderr);
    let cv = read_json(&out);
    let cands = cv["candidates"].as_array().unwrap();
    // 沉底一（achieved 不入）+ 在泊一（已出泊不入）+ 候裁单二。
    assert_eq!(cands.len(), 4, "四件候选：{:?}", cands);
    let by_source = |s: &str| cands.iter().filter(|c| c["source"] == s).count();
    assert_eq!(by_source("critsweep"), 1);
    assert_eq!(by_source("parking"), 1);
    assert_eq!(by_source("covenant"), 2);
    let parked = cands.iter().find(|c| c["source"] == "parking").unwrap();
    assert_eq!(parked["id"], "pk-alpha");
    assert_eq!(parked["summary"], "用户裁统一或文档注明", "泊位候选应带出泊条件");
    let sunk = cands.iter().find(|c| c["source"] == "critsweep").unwrap();
    assert_eq!(sunk["id"], "GOV-002-SDDG");
    let cov = cands.iter().find(|c| c["id"] == "covenant-2").unwrap();
    assert_eq!(cov["title"], "清点 packs 目录并归位", "有序列表项应解析");
}

#[test]
fn collect_missing_critsweep_shape_exit_2() {
    let ws = temp_ws("pendline-collect-bad");
    let bad = ws.root.join("bad.json");
    write_file(&bad, "{\"unrelated\": true}\n");
    let (rc, _, _) = run(
        pendline_bin().args([
            "collect",
            "--critsweep-json",
            bad.to_str().unwrap(),
            "--out",
            ws.root.join("c.json").to_str().unwrap(),
        ]),
    );
    assert_eq!(rc, 2, "非判据扫出参形应 exit 2");
}

// ------------------------------------------------- backfill 面（pl-02）

#[test]
fn backfill_rejects_mismatched_responses() {
    let ws = temp_ws("pendline-backfill");
    let material = full_chain(&ws, "3", "stable_clear");
    let _ = material;
    // 缺一发响应即拒收（facet 严格对表纪律）。
    let contract = ws.root.join("contracts").join("covenant-1.contract.json");
    let responses = ws.root.join("responses-short.jsonl");
    let full = ws.root.join("responses.jsonl");
    let text = std::fs::read_to_string(&full).unwrap();
    let lines: Vec<&str> = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    assert!(lines.len() >= 2, "夹具应有多发");
    write_file(&responses, &format!("{}\n", lines[..lines.len() - 1].join("\n")));
    let (rc, _, _) = run(
        pendline_bin().args([
            "backfill",
            "--contract",
            contract.to_str().unwrap(),
            "--responses",
            responses.to_str().unwrap(),
            "--verdict",
            "stable_clear",
            "--out",
            ws.root.join("m2.json").to_str().unwrap(),
        ]),
    );
    assert_eq!(rc, 1, "缺发回填件应拒收 exit 1");
}

// ------------------------------------------------- 落链纪律面（pl-03）

#[test]
fn route_marks_require_identity_report_exit_2() {
    let ws = temp_ws("pendline-noir");
    let material = full_chain(&ws, "3", "stable_clear");
    let trail = chain_trail(&ws, "trail-e");
    // 过者无正身件：前置校验 exit 2 且零写入。
    let (rc, _, _) = run(
        pendline_bin()
            .arg("route")
            .args([
                "--materials",
                material.to_str().unwrap(),
                "--scribe-binary",
                env!("CARGO_BIN_EXE_scribe"),
                "--trail",
                trail.to_str().unwrap(),
                "--no-session-reason",
                "缺正身件形",
            ]),
    );
    assert_eq!(rc, 2, "过者缺正身件应 exit 2");
    assert!(!trail.exists(), "前置校验拒收应零链写入");

    // 过者会话席二择一缺席同拒。
    let ir = ws.root.join("identity.json");
    identity_report_fixture(&ir);
    let (rc, _, _) = run(
        pendline_bin()
            .arg("route")
            .args([
                "--materials",
                material.to_str().unwrap(),
                "--scribe-binary",
                env!("CARGO_BIN_EXE_scribe"),
                "--trail",
                trail.to_str().unwrap(),
                "--identity-report",
                ir.to_str().unwrap(),
            ]),
    );
    assert_eq!(rc, 2, "会话与无会话事由俱缺应 exit 2");
    assert!(!trail.exists());
}

#[test]
fn route_dry_run_zero_chain_write() {
    let ws = temp_ws("pendline-dry");
    let material = full_chain(&ws, "3", "boundary");
    let trail = chain_trail(&ws, "trail-f");
    let ir = ws.root.join("identity.json");
    identity_report_fixture(&ir);
    let (rc, stdout, _) = run(
        pendline_bin()
            .arg("route")
            .args([
                "--materials",
                material.to_str().unwrap(),
                "--scribe-binary",
                env!("CARGO_BIN_EXE_scribe"),
                "--trail",
                trail.to_str().unwrap(),
                "--identity-report",
                ir.to_str().unwrap(),
                "--no-session-reason",
                "dry run",
                "--dry-run",
            ]),
    );
    assert_eq!(rc, 0);
    assert!(stdout.contains("\"dry_run\": true") || stdout.contains("\"dry_run\":true"));
    assert!(!trail.exists(), "dry-run 应零链写入");
}

// ------------------------------------------------- 零触碰断言

/// 集成零触碰：critsweep 与 attractor 与 lease 三件生产码 git 零 diff
///（porcelain 查询只读，工件清单外零改动）。
#[test]
fn zero_touch_critsweep_attractor_lease() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for p in [
        "src/bin/critsweep.rs",
        "src/attractor/mod.rs",
        "src/attractor/contract_mode.rs",
        "src/bin/lease.rs",
        "src/bin/lease/closegate.rs",
    ] {
        assert!(root.join(p).is_file(), "被断言生产码应存在：{}", p);
    }
    let out = Command::new("git")
        .arg("-C")
        .arg(&root)
        .args([
            "status",
            "--porcelain",
            "--",
            "src/bin/critsweep.rs",
            "src/attractor",
            "src/bin/lease.rs",
            "src/bin/lease",
            "src/retriever",
            "src/mcpserver",
        ])
        .output()
        .expect("git 可用");
    assert!(
        out.status.success(),
        "git status 应成功：{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.trim().is_empty(),
        "critsweep 与 attractor 与 lease 与 retriever 与 mcpserver 应零 diff，实得：\n{}",
        stdout
    );
}
