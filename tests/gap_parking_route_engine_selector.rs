//! gap-parking-route-engine-selector：critsweep 泊界路由腿经引擎 selector bin 正路径回归。
//!
//! 承 critsweep.rs 泊界腿承载位变更（自 gap-parking-route-engine-selector 起泊界
//! 路由腿直调引擎 selector bin：本 bin 同目录兄弟位 + 引擎仓 packs/selector/parking）。
//! 既有 mergeall_t1_critsweep 只断言无材料降级形，本件补正路径：
//! 一例三路路由（mainline/siding/scrap_track 各一）、一例空目录绿态，
//! 另附围堰 Python selector 同参对表抽查一例（对比判路与 summary 计数，
//! 报文格式差异不比：引擎 py_compact 紧凑形对围堰 indent=2 已知偏差如实不比）。
//! fixture 全 temp 自建，零真实账本写入。

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin_critsweep() -> &'static str {
    env!("CARGO_BIN_EXE_critsweep")
}

fn run(args: &[String]) -> (i32, Value) {
    let out = Command::new(bin_critsweep())
        .args(args)
        .env_remove("CRITSWEEP_REGISTRY")
        .output()
        .unwrap();
    let rc = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let v: Value = serde_json::from_str(&stdout).expect("critsweep 出参须为严格 JSON 单对象");
    (rc, v)
}

struct Ws {
    root: PathBuf,
    registry: PathBuf,
    _guard: tempfile::TempDir,
}

const MIN_REGISTRY: &str =
    r#"{"criteria":[{"id":"P-ACT","title":"占位活动判据","status_kind":"activity","token_scope":["parkingsweep"]}]}"#;

/// first_domain 布局沙箱：双仓标记件 + 两线泊界材料目录 + 显式 registry。
fn build_ws(tag: &str, materials: Option<&[(&str, String)]>) -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let root = guard.path().join(format!("ws-{tag}"));
    // first_domain 标记件（critsweep detect_layout 判定位）
    fs::create_dir_all(root.join("sih-engine")).unwrap();
    fs::create_dir_all(root.join("sih-tools")).unwrap();
    fs::write(root.join("sih-engine/Cargo.toml"), "[package]\nname=\"stub\"\n").unwrap();
    fs::write(root.join("sih-tools/pyproject.toml"), "# stub\n").unwrap();
    // 两线泊界材料目录（first_domain parking_jobs 相对 parking_cwd=root/sih-tools）
    let engine_mats = root.join("sih-engine/sih/state/parking/materials");
    let tools_mats = root.join("sih-tools/parking/materials");
    fs::create_dir_all(&engine_mats).unwrap();
    fs::create_dir_all(&tools_mats).unwrap();
    if let Some(mats) = materials {
        write_files(&engine_mats, mats);
        write_files(&tools_mats, mats);
    }
    let registry = root.join("registry.json");
    fs::write(&registry, MIN_REGISTRY).unwrap();
    Ws { root, registry, _guard: guard }
}

fn write_files(dir: &Path, mats: &[(&str, String)]) {
    for (name, body) in mats {
        fs::write(dir.join(name), body).unwrap();
    }
}

/// 三路材料组：mainline 一（TTL 未到）、siding 一（缺 parking 块 P102 败）、
/// scrap_track 一（缺 state 字段 P101 败）。
fn three_route_materials() -> Vec<(&'static str, String)> {
    vec![
        (
            "pk-main.json",
            "{\"id\":\"pk-main\",\"path\":\"sih-engine/doc/a.md\",\"state\":\"parked\",\"parking\":{\"entered_at\":\"2026-09-01\",\"ttl_days\":30}}\n".into(),
        ),
        (
            "pk-side.json",
            "{\"id\":\"pk-side\",\"path\":\"sih-engine/doc/b.md\",\"state\":\"parked\"}\n".into(),
        ),
        (
            "pk-scrap.json",
            "{\"id\":\"pk-scrap\",\"path\":\"sih-engine/doc/c.md\"}\n".into(),
        ),
    ]
}

fn sweep_args(ws: &Ws) -> Vec<String> {
    vec![
        "--at".into(),
        "2026-09-13".into(),
        "--root".into(),
        ws.root.display().to_string(),
        "--registry".into(),
        ws.registry.display().to_string(),
    ]
}

/// 正路径：两线泊界材料各三件经引擎 selector bin 真路由，三路各归其位。
#[test]
fn t1_parking_face_routes_via_engine_selector_bin() {
    let ws = build_ws("three", Some(&three_route_materials()));
    let (rc, out) = run(&sweep_args(&ws));
    assert_eq!(rc, 0, "泊界正路径整体退出码零：{}", out);

    for job in ["engine", "tools"] {
        let line = &out["parking"][job];
        assert_eq!(line["route"], "routed", "job {job} 须真路由非降级：{line}");
        assert!(
            line["exit_code"] == 0 || line["exit_code"] == 1,
            "路由退出码须 0/1 两值内：{line}"
        );
        assert_eq!(line["summary"]["total"], 3, "job {job} 三件全路由：{line}");
        assert_eq!(line["summary"]["mainline"], 1, "job {job} mainline 一件：{line}");
        assert_eq!(line["summary"]["siding"], 1, "job {job} siding 一件：{line}");
        assert_eq!(line["summary"]["scrap_track"], 1, "job {job} scrap_track 一件：{line}");
    }
    // 泊界面自身零降级：degradations 无泊界路由字样
    let degr = out["degradations"].as_array().unwrap();
    assert!(
        !degr.iter().any(|d| d.as_str().unwrap_or("").contains("泊界")),
        "泊界腿不得降级：{degr:?}"
    );
}

/// 空目录绿态：两线材料目录在而空，路由绿态 total 0 非降级。
#[test]
fn t2_parking_face_empty_dirs_green_routed() {
    let ws = build_ws("empty", None);
    let (rc, out) = run(&sweep_args(&ws));
    assert_eq!(rc, 0, "空目录绿态退出码零：{}", out);
    for job in ["engine", "tools"] {
        let line = &out["parking"][job];
        assert_eq!(line["route"], "routed", "空批即 routed 绿态非降级：{line}");
        assert_eq!(line["summary"]["total"], 0, "空批 total 0：{line}");
    }
}

/// 围堰 Python selector 同参对表抽查：同包同参同材料，逐材料判路与 summary 计数一致
/// （报文格式差异——引擎 py_compact 紧凑形对围堰 indent=2——不比，已知偏差）。
#[test]
fn t3_engine_selector_matches_frozen_python_same_params() {
    let ws = build_ws("xcheck", Some(&three_route_materials()));
    let pack = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("packs/selector/parking");
    assert!(pack.is_dir(), "引擎 parking 包须在位：{}", pack.display());

    let engine_out = Command::new(env!("CARGO_BIN_EXE_selector"))
        .args([
            "route",
            "--pack",
            &pack.display().to_string(),
            "--reference-time",
            "2026-09-13",
        ])
        .arg(ws.root.join("sih-engine/sih/state/parking/materials"))
        .output()
        .unwrap();
    assert_eq!(engine_out.status.code(), Some(0), "引擎 selector 退出码零");

    let tools_src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("sih-tools/selector/src");
    let py_script = format!(
        "import sys; sys.path.insert(0, {src:?}); from selector.cli import main; sys.exit(main(sys.argv[1:]))",
        src = tools_src.display().to_string(),
    );
    let py_out = Command::new("python3")
        .arg("-c")
        .arg(&py_script)
        .args([
            "route",
            "--pack",
            &pack.display().to_string(),
            "--reference-time",
            "2026-09-13",
        ])
        .arg(ws.root.join("sih-engine/sih/state/parking/materials"))
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("python3 不可用即本对表腿不可跑，环境缺 python3 属环境事实");
    let py_ok = py_out.status.code() == Some(0);
    let py_stdout = String::from_utf8_lossy(&py_out.stdout).to_string();
    if !py_ok {
        // 围堰件跑不起来（如 python 环境缺依赖）即如实申报跳过，不硬凑一致
        eprintln!(
            "围堰 python selector 不可跑（退出码 {:?}），对表腿如实跳过：{}",
            py_out.status.code(),
            String::from_utf8_lossy(&py_out.stderr),
        );
        return;
    }
    let eng: Value = serde_json::from_str(&String::from_utf8_lossy(&engine_out.stdout)).unwrap();
    let py: Value = serde_json::from_str(py_stdout.trim()).unwrap();
    // 逐材料判路与首败谓词一致
    let eng_by_id: Value = eng["routed"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| (e["id"].as_str().unwrap().to_string(), e.clone()))
        .collect::<serde_json::Map<String, Value>>()
        .into();
    for e in py["routed"].as_array().unwrap() {
        let id = e["id"].as_str().unwrap();
        assert_eq!(
            eng_by_id[id]["route"], e["route"],
            "材料 {id} 判路须与围堰一致"
        );
        assert_eq!(
            eng_by_id[id]["failed_predicate"], e["failed_predicate"],
            "材料 {id} 首败谓词须与围堰一致"
        );
    }
    // summary 计数一致（alarms 数组序不比，计数比）
    assert_eq!(eng["summary"]["total"], py["summary"]["total"]);
    assert_eq!(eng["summary"]["mainline"], py["summary"]["mainline"]);
    assert_eq!(eng["summary"]["siding"], py["summary"]["siding"]);
    assert_eq!(eng["summary"]["scrap_track"], py["summary"]["scrap_track"]);
    assert_eq!(eng["summary"]["alarms"].as_array().map(|a| a.len()), py["summary"]["alarms"].as_array().map(|a| a.len()));
}
