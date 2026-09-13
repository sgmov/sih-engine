//! lease-mergeleg23-parallel 簇E T4：selector 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/selector 0.4.0（cli.py 三值退出码、pack.py 包校验、
//! predicates.py 机械求值、route.py 首败定路与批级告警）。fixture 全部 temp
//! 自建最小包，围堰真实包零触碰。三例：正常形（全过走 mainline）、拒绝形
//! （首败走 siding 加告警加输入非法退出码二）、边界形（空批绿态、密度恰达
//! 阈值、到期日当日即败）。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin_selector() -> &'static str {
    env!("CARGO_BIN_EXE_selector")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_selector()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

struct Ws {
    _guard: tempfile::TempDir,
    dir: PathBuf,
    pack: PathBuf,     // 主包：schema + whitelist + density
    time_pack: PathBuf, // 时间包：time_deadline 单谓词
}

fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let pack = dir.join("pack-t4sel");
    fs::create_dir_all(&pack).unwrap();
    fs::write(
        pack.join("envelope.json"),
        r#"{"envelope_version":1,"id":"t4sel","family":"attractor","body_type":"config","bodies":["manifest.toml","routes.toml"]}"#,
    )
    .unwrap();
    fs::write(
        pack.join("manifest.toml"),
        "name = \"t4sel\"\nversion = \"0.0.1\"\n\n[domain]\ninclude = [\"**/*.json\"]\nexclude = []\n",
    )
    .unwrap();
    // 密度谓词 resolve_root 即本 temp 语料根，锚存在性在本目录内判定。
    let routes = format!(
        "[[predicates]]\nid = \"R1\"\nkind = \"schema_required\"\nroute_on_fail = \"scrap_track\"\nfields = [\"id\", \"path\"]\n\n[[predicates]]\nid = \"W1\"\nkind = \"anchor_whitelist\"\nroute_on_fail = \"siding\"\nwhitelist = [\"sih-engine/*\", \"sih-tools/*\"]\n\n[[predicates]]\nid = \"D1\"\nkind = \"anchor_density\"\ndensity_threshold = 0.5\nresolve_root = \"{}\"\n\n[defaults]\npass_route = \"mainline\"\n\n[alarms]\nsiding_surplus_threshold = 1\nmainline_starvation_threshold = 0\n",
        dir.display()
    );
    fs::write(pack.join("routes.toml"), routes).unwrap();

    let time_pack = dir.join("pack-t4time");
    fs::create_dir_all(&time_pack).unwrap();
    fs::write(
        time_pack.join("envelope.json"),
        r#"{"envelope_version":1,"id":"t4time","family":"attractor","body_type":"config","bodies":["manifest.toml","routes.toml"]}"#,
    )
    .unwrap();
    fs::write(
        time_pack.join("manifest.toml"),
        "name = \"t4time\"\nversion = \"0.0.1\"\n\n[domain]\ninclude = [\"**/*.json\"]\nexclude = []\n",
    )
    .unwrap();
    fs::write(
        time_pack.join("routes.toml"),
        "[[predicates]]\nid = \"T1\"\nkind = \"time_deadline\"\nroute_on_fail = \"siding\"\n\n[defaults]\npass_route = \"mainline\"\n\n[alarms]\nsiding_surplus_threshold = 9\nmainline_starvation_threshold = 0\n",
    )
    .unwrap();

    // 语料锚：密度谓词的存在性判定目标。
    fs::write(dir.join("seed.md"), "seed anchor file\n").unwrap();

    Ws {
        _guard: guard,
        dir,
        pack,
        time_pack,
    }
}

fn material(ws: &Ws, name: &str, body: &str) -> String {
    let p = ws.dir.join(name);
    fs::write(&p, body).unwrap();
    p.to_string_lossy().into_owned()
}

// ---------- 正常形：谓词按声明序全过走 pass_route，零告警绿态 ----------

#[test]
fn t1_normal_all_pass_mainline() {
    let ws = build_ws();
    let pa = ws.pack.to_string_lossy().into_owned();
    let good = material(
        &ws,
        "good.json",
        r#"{"id":"m1","path":"doc-a.md","anchors":["sih-engine/doc/spec/SPEC-007.md"],"round":{"assertions":[{"evidence":["seed.md:1"]}]}}"#,
    );
    let (code, out, err) = run(&["route", "--pack", &pa, &good]);
    assert_eq!(code, 0, "全过零告警须 0，stdout={out} stderr={err}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["status"], "routed");
    assert_eq!(rep["header"]["tool"]["name"], "selector");
    assert_eq!(rep["header"]["tool"]["version"], "0.4.0");
    assert_eq!(rep["header"]["pack"]["name"], "t4sel");
    assert_eq!(rep["header"]["domain"]["include"][0], "**/*.json");
    let entry = &rep["routed"][0];
    assert_eq!(entry["id"], "m1");
    assert_eq!(entry["route"], "mainline");
    assert!(entry["failed_predicate"].is_null());
    let checks = entry["checks"].as_array().unwrap();
    assert_eq!(checks.len(), 3, "三谓词全评估：schema+whitelist+density");
    assert_eq!(checks[0]["id"], "R1");
    assert_eq!(checks[0]["pass"], true);
    assert_eq!(checks[1]["id"], "W1");
    assert_eq!(checks[2]["id"], "D1");
    assert_eq!(entry["round"], true, "轮记录携带 round 真");
    assert!(entry.get("parking").is_none(), "非停泊件不携带 parking 键");
    assert_eq!(rep["summary"]["total"], 1);
    assert_eq!(rep["summary"]["mainline"], 1);
    assert_eq!(rep["summary"]["alarms"].as_array().unwrap().len(), 0);
}

// ---------- 拒绝形：首败定路 siding 加批级告警；非法输入退出码二 ----------

#[test]
fn t2_reject_first_fail_siding_alarm_and_input_invalid() {
    let ws = build_ws();
    let pa = ws.pack.to_string_lossy().into_owned();
    let good = material(
        &ws,
        "good.json",
        r#"{"id":"m1","path":"doc-a.md","anchors":["sih-engine/doc/x.md"],"round":{"assertions":[{"evidence":["seed.md"]}]}}"#,
    );
    let bad = material(
        &ws,
        "bad.json",
        r#"{"id":"m2","path":"doc-b.md","anchors":["outside/evil.md"]}"#,
    );
    let (code, out, _) = run(&["route", "--pack", &pa, &good, &bad]);
    assert_eq!(code, 1, "有告警须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    let bad_entry = &rep["routed"][1];
    assert_eq!(bad_entry["route"], "siding", "首败 W1 定路 siding");
    assert_eq!(bad_entry["failed_predicate"], "W1");
    // scrap_track 谓词顺位评估但首败已定路：schema 过、白名单败、密度败（无 round）。
    let checks = bad_entry["checks"].as_array().unwrap();
    assert_eq!(checks[1]["pass"], false);
    assert_eq!(checks[2]["pass"], false);
    assert_eq!(rep["summary"]["siding"], 1);
    assert_eq!(rep["summary"]["mainline"], 1);
    let alarms = rep["summary"]["alarms"].as_array().unwrap();
    assert_eq!(alarms.len(), 1, "siding 1 >= 阈值 1 产有余告警");
    assert_eq!(alarms[0]["kind"], "siding_surplus");
    assert_eq!(alarms[0]["count"], 1);
    assert_eq!(alarms[0]["threshold"], 1);

    // 非法输入面：包缺席、材料 JSON 非法、材料缺席、参照时间非法，全退出码二。
    let (code, out, _) = run(&["route", "--pack", "/nonexistent-t4"]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert!(
        err["error"]
            .as_str()
            .unwrap()
            .starts_with("pack invalid: pack directory missing:"),
        "error={}"
        , err["error"]
    );
    let badjson = material(&ws, "badjson.json", "{not json");
    let (code, out, _) = run(&["route", "--pack", &pa, &badjson]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert!(
        err["error"].as_str().unwrap().starts_with("material json invalid:"),
        "error={}",
        err["error"]
    );
    let (code, out, _) = run(&["route", "--pack", &pa, "/nonexistent-t4.json"]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert!(
        err["error"].as_str().unwrap().starts_with("material file missing:"),
        "error={}",
        err["error"]
    );
    let (code, out, _) = run(&["route", "--pack", &pa, "--reference-time", "notadate", &good]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        err["error"],
        "reference_time invalid ISO date: notadate"
    );
}

// ---------- 边界形：空批绿态、密度恰达阈值、到期日当日即败、缺参照即 fail-closed ----------

#[test]
fn t3_boundary_empty_batch_density_edge_and_deadline_day() {
    let ws = build_ws();
    let pa = ws.pack.to_string_lossy().into_owned();
    let tp = ws.time_pack.to_string_lossy().into_owned();

    // 空批绿态：零材料合法，退出码零，无饥饿误报。
    let (code, out, _) = run(&["route", "--pack", &pa]);
    assert_eq!(code, 0, "空批须绿态 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"]["total"], 0);
    assert_eq!(rep["summary"]["alarms"].as_array().unwrap().len(), 0);

    // 密度恰达阈值：1/2 = 0.5 >= 0.5 方过，且「路径:行」条目取路径前缀解析。
    let edge = material(
        &ws,
        "edge.json",
        r#"{"id":"m3","path":"x.md","anchors":[],"round":{"assertions":[{"evidence":["seed.md:3"]},{"evidence":[]}]}}"#,
    );
    let (code, out, _) = run(&["route", "--pack", &pa, &edge]);
    assert_eq!(code, 0, "恰达阈值须过，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["routed"][0]["route"], "mainline");
    assert!(rep["routed"][0]["failed_predicate"].is_null());

    // 到期日当日即败：entered 2026-09-01 + ttl 5 = 2026-09-06，参照 09-06 判败；
    // 同批 ttl 6 件到期 09-07 判过。主线一及格即零饥饿告警，退出码零。
    let due = material(
        &ws,
        "p-due.json",
        r#"{"id":"p1","path":"p.md","parking":{"entered_at":"2026-09-01","ttl_days":5}}"#,
    );
    let ok = material(
        &ws,
        "p-ok.json",
        r#"{"id":"p2","path":"q.md","parking":{"entered_at":"2026-09-01","ttl_days":6}}"#,
    );
    let (code, out, _) = run(&[
        "route", "--pack", &tp, "--reference-time", "2026-09-06", &due, &ok,
    ]);
    assert_eq!(code, 0, "到期与未到期各一、主线非零零告警须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["routed"][0]["route"], "siding", "到期日当日即败");
    assert_eq!(rep["routed"][0]["failed_predicate"], "T1");
    assert_eq!(rep["routed"][0]["parking"], true, "停泊材料携带 parking 真");
    assert_eq!(rep["routed"][1]["route"], "mainline", "到期前判过");
    assert_eq!(rep["header"]["reference_time"], "2026-09-06");
    assert_eq!(rep["summary"]["mainline"], 1);
    assert_eq!(rep["summary"]["siding"], 1);
    assert_eq!(rep["summary"]["alarms"].as_array().unwrap().len(), 0);

    // 到期前一日参照：两件全过走主线，退出码零。
    let (code, out, _) = run(&[
        "route", "--pack", &tp, "--reference-time", "2026-09-05", &due, &ok,
    ]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["routed"][0]["route"], "mainline", "到期前判过");
    assert_eq!(rep["routed"][1]["route"], "mainline", "到期前判过");
    assert_eq!(rep["summary"]["alarms"].as_array().unwrap().len(), 0);

    // 缺参照时间即 fail-closed：两件全判败走 siding，主线归零触发饥饿告警。
    let (code, out, _) = run(&["route", "--pack", &tp, &due, &ok]);
    assert_eq!(code, 1, "零主线触发饥饿告警须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["routed"][0]["route"], "siding", "缺参照时间判败");
    assert_eq!(rep["routed"][1]["failed_predicate"], "T1");
    let alarms = rep["summary"]["alarms"].as_array().unwrap();
    assert_eq!(alarms.len(), 1);
    assert_eq!(alarms[0]["kind"], "mainline_starvation");
    assert_eq!(alarms[0]["count"], 0);
}
