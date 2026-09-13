//! lease-mergeleg6-parallel 簇H T6：incubation 契约校验引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/incubation/checker（cli.py 三值退出码封套、
//! engine.py 封闭词汇表 v0 谓词求值、pack.py 包装载与包族目录配对）。
//! fixture 全部 temp 自建最小探针包与目标件，围堰真实包族零触碰。
//! 金向量三例：正常（合规通过与配对与双跑一致）、拒绝（三态发现定位）、
//! 边界（包非法与目标缺席与 D-4 必携位与不可解码目标）。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn bin_incubation() -> &'static str {
    env!("CARGO_BIN_EXE_incubation")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_incubation()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn run_env(args: &[&str], key: &str, val: &str) -> (i32, String, String) {
    let out = Command::new(bin_incubation())
        .args(args)
        .env(key, val)
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

const GOOD: &str = "# 探针标题\n\n## 范围\n\n正文内容。\n\n## 裁决\n\n1. 引 S-001 证据：退出码零\nX-001\nX-002\n";
const BAD: &str = "# 探针标题\n\n## 范围\n\n1. 引 S-999 证据：无\nX-001\nX-001\n";
const REF: &str = "### Requirement: S-001\n\n#### Scenario: R-001\n";

/// 探针包：五规则五操作面（exists_readable、heading_present、sections_present
/// ordered、pattern_unique、reference_closure），D-4 必携位 required true。
const PACK_P1: &str = r#"{
  "pack": "t6-probe",
  "version": "0.1.0",
  "doc_class": "探针文档",
  "provenance": {"d4_anchor_slot": {"required": true, "vectors_home": "sih-tools/basemgr/vectors/checker-golden/"}},
  "rules": [
    {"id": "PB-001", "failure_location": {"state": "missing", "hint": "文档缺席或不可读"}, "check": {"ops": [{"op": "exists_readable"}]}},
    {"id": "PB-002", "failure_location": {"state": "missing", "hint": "一级标题缺席"}, "check": {"ops": [{"op": "heading_present", "level": 1}]}},
    {"id": "PB-003", "failure_location": {"state": "missing", "hint": "节缺席"}, "check": {"ops": [{"op": "sections_present", "level": 2, "names": ["范围", "裁决"], "ordered": true}]}},
    {"id": "PB-004", "failure_location": {"state": "violation", "hint": "编号重号或缺形"}, "check": {"ops": [{"op": "pattern_unique", "pattern": "^(X-\\d+)", "format": "X-\\d+"}]}},
    {"id": "PB-005", "failure_location": {"state": "broken_ref", "hint": "引用悬空"}, "check": {"ops": [{"op": "reference_closure", "items_marker": "^\\d+\\. ", "ref_pattern": "[SR]-\\d+", "source": "@reference", "defined_pattern": "^(?:### Requirement|#### Scenario):\\s*([SR]-\\d+)"}]}}
  ]
}"#;

struct Ws {
    dir: PathBuf,
    p1: PathBuf,
    good: PathBuf,
    bad: PathBuf,
    rf: PathBuf,
    fam: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let p1 = dir.join("p1.json");
    fs::write(&p1, PACK_P1).unwrap();
    let good = dir.join("good.md");
    fs::write(&good, GOOD).unwrap();
    let bad = dir.join("bad.md");
    fs::write(&bad, BAD).unwrap();
    let rf = dir.join("ref.md");
    fs::write(&rf, REF).unwrap();
    // 包族目录形：manifest 基名配对（good.md → p1.json）。
    let fam = dir.join("fam");
    fs::create_dir_all(&fam).unwrap();
    fs::write(
        fam.join("manifest.json"),
        r#"{"packs": ["p1.json"], "fixtures": ["good.md"]}"#,
    )
    .unwrap();
    fs::write(fam.join("p1.json"), PACK_P1).unwrap();
    Ws { dir, p1, good, bad, rf, fam, _guard: guard }
}

/// 探针包装备：单规则单操作，D-4 可选（required false 即不拦）。
fn probe_pack(name: &str, with_d4: bool, rules: Value) -> String {
    let mut m = serde_json::Map::new();
    m.insert("pack".into(), json!(name));
    m.insert("version".into(), json!("0.1.0"));
    if with_d4 {
        m.insert("provenance".into(), json!({"d4_anchor_slot": {"required": false}}));
    }
    m.insert("rules".into(), rules);
    serde_json::to_string(&Value::Object(m)).unwrap()
}

fn one_rule(id: &str, state: &str, hint: &str, ops: Value) -> Value {
    json!({"id": id, "failure_location": {"state": state, "hint": hint}, "check": {"ops": ops}})
}

// ---------- 金向量一（正常）：合规通过、引用闭包、目录配对、双跑一致 ----------

#[test]
fn t1_normal_pass_reference_closure_dir_form_determinism() {
    let ws = build_ws();
    let p1 = ws.p1.to_str().unwrap();
    let good = ws.good.to_str().unwrap();
    let rf = ws.rf.to_str().unwrap();

    let (code, out, err) = run(&["--pack", p1, "--reference", rf, good]);
    assert_eq!(code, 0, "合规须 0: {out}{err}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["verdict"], "pass");
    assert_eq!(rep["findings"].as_array().unwrap().len(), 0);
    assert_eq!(rep["packs"][0]["name"], "t6-probe");
    assert_eq!(rep["packs"][0]["version"], "0.1.0");
    assert_eq!(rep["packs"][0]["doc_class"], "探针文档");
    assert_eq!(rep["targets"][0]["path"], good);
    let want_sha = sha256_hex(&fs::read(good).unwrap());
    assert_eq!(rep["targets"][0]["sha256"], want_sha, "内容摘要入报告作凭据");
    assert_eq!(rep["targets"][0]["results"][0]["verdict"], "pass");

    // 双跑逐字节一致（决定论）。
    let (c2, out2, _) = run(&["--pack", p1, "--reference", rf, good]);
    assert_eq!(c2, 0);
    assert_eq!(out2, out, "同参双跑 stdout 逐字节一致");

    // 引擎版本戳走环境变量。
    let (c3, out3, _) = run_env(
        &["--pack", p1, "--reference", rf, good],
        "SIH_CHECKER_ENGINE_VERSION",
        "9.9.9.t6",
    );
    assert_eq!(c3, 0);
    let rep3: Value = serde_json::from_str(&out3).unwrap();
    assert_eq!(rep3["engine_version"], "9.9.9.t6");

    // 包族目录形：manifest 基名配对落一包施检。
    let fam = ws.fam.to_str().unwrap();
    let (code, out, _) = run(&["--pack", fam, "--reference", rf, good]);
    assert_eq!(code, 0, "{out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    let results = rep["targets"][0]["results"].as_array().unwrap();
    assert_eq!(results.len(), 1, "基名配对施一包");
    assert_eq!(results[0]["pack"], "t6-probe");
}

// ---------- 金向量二（拒绝）：三态发现带行号与编号引用与规则归属 ----------

#[test]
fn t2_reject_violations_three_states_location() {
    let ws = build_ws();
    let p1 = ws.p1.to_str().unwrap();
    let bad = ws.bad.to_str().unwrap();

    // 无 --reference：引用闭包语料空，悬空按 broken_ref 落笔，不静默。
    let (code, out, _) = run(&["--pack", p1, bad]);
    assert_eq!(code, 1, "违规须 1: {out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["verdict"], "fail");
    let findings = rep["findings"].as_array().unwrap();

    let got: Vec<(String, String)> = findings
        .iter()
        .map(|f| {
            (
                f["rule"].as_str().unwrap().to_string(),
                f["state"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert!(got.contains(&("PB-003".into(), "missing".into())), "{got:?}");
    assert!(got.contains(&("PB-004".into(), "violation".into())), "{got:?}");
    assert!(got.contains(&("PB-005".into(), "broken_ref".into())), "{got:?}");

    for f in findings {
        let state = f["state"].as_str().unwrap();
        assert!(
            ["missing", "violation", "broken_ref"].contains(&state),
            "三态纪律: {state}"
        );
        assert!(f["hint"].is_string(), "三态定位带失败提示");
        assert_eq!(f["pack"], "t6-probe");
        assert_eq!(f["pack_version"], "0.1.0");
    }

    // 定位：pattern_unique 重号报重号行与编号；引用悬空报编号；节缺席报名。
    let dup = findings.iter().find(|f| f["rule"] == "PB-004").unwrap();
    assert_eq!(dup["line"], 7, "第二个 X-001 落第七行");
    assert_eq!(dup["ref"], "X-001");
    let dangling = findings.iter().find(|f| f["rule"] == "PB-005").unwrap();
    assert_eq!(dangling["line"], 5);
    assert_eq!(dangling["ref"], "S-999");
    let missing_sec = findings.iter().find(|f| f["rule"] == "PB-003").unwrap();
    assert_eq!(missing_sec["ref"], "裁决");
    assert!(missing_sec["line"].is_null());
}

// ---------- 金向量三（边界）：包非法、目标缺席、D-4 必携位、不可解码目标 ----------

#[test]
fn t3_boundary_illegal_pack_missing_target_d4_undecodable() {
    let ws = build_ws();
    let p1 = ws.p1.to_str().unwrap();
    let good = ws.good.to_str().unwrap();

    // (a) 词汇表外操作 → 二。
    let pk = ws.dir.join("pk-unknown.json");
    fs::write(
        &pk,
        probe_pack("probe", true, json!([one_rule("ZZ-001", "missing", "探测", json!([{"op": "no_such_op"}]))])),
    )
    .unwrap();
    let (code, out, _) = run(&["--pack", pk.to_str().unwrap(), good]);
    assert_eq!(code, 2);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert!(rep["error"].is_string());

    // (a) 操作缺必填参数 → 二。
    let pk = ws.dir.join("pk-noparam.json");
    fs::write(
        &pk,
        probe_pack("probe", true, json!([one_rule("ZZ-001", "missing", "探测", json!([{"op": "heading_present"}]))])),
    )
    .unwrap();
    let (code, _, _) = run(&["--pack", pk.to_str().unwrap(), good]);
    assert_eq!(code, 2);

    // (a) 规则 id 重号 → 二。
    let pk = ws.dir.join("pk-dupid.json");
    fs::write(
        &pk,
        probe_pack(
            "probe",
            true,
            json!([
                one_rule("ZZ-001", "missing", "探测", json!([{"op": "exists_readable"}])),
                one_rule("ZZ-001", "missing", "探测", json!([{"op": "exists_readable"}]))
            ]),
        ),
    )
    .unwrap();
    let (code, _, _) = run(&["--pack", pk.to_str().unwrap(), good]);
    assert_eq!(code, 2);

    // (a) 正则非法 → 二。
    let pk = ws.dir.join("pk-badregex.json");
    fs::write(
        &pk,
        probe_pack(
            "probe",
            true,
            json!([one_rule("ZZ-001", "violation", "探测", json!([{"op": "pattern_unique", "pattern": "^X-(\\d+", "format": "X-\\d+"}]))]),
        ),
    )
    .unwrap();
    let (code, _, _) = run(&["--pack", pk.to_str().unwrap(), good]);
    assert_eq!(code, 2);

    // (a) pattern_unique 零捕获组 → 二。
    let pk = ws.dir.join("pk-nogroup.json");
    fs::write(
        &pk,
        probe_pack(
            "probe",
            true,
            json!([one_rule("ZZ-001", "violation", "探测", json!([{"op": "pattern_unique", "pattern": "^X-\\d+", "format": "X-\\d+"}]))]),
        ),
    )
    .unwrap();
    let (code, _, _) = run(&["--pack", pk.to_str().unwrap(), good]);
    assert_eq!(code, 2);

    // (a) 包文件缺席与包族目录缺 manifest → 二。
    let (code, _, _) = run(&["--pack", ws.dir.join("absent.json").to_str().unwrap(), good]);
    assert_eq!(code, 2);
    let emptydir = ws.dir.join("emptyfam");
    fs::create_dir_all(&emptydir).unwrap();
    let (code, _, _) = run(&["--pack", emptydir.to_str().unwrap(), good]);
    assert_eq!(code, 2);

    // (b) 目标缺席 → 二。
    let no_such = ws.dir.join("no-such-target.md");
    let (code, out, _) = run(&["--pack", p1, no_such.to_str().unwrap()]);
    assert_eq!(code, 2);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["error"], "目标缺席或非常规文件");

    // (c) D-4 必携位：包缺出处面挂锚位即违规标记。
    let pk = ws.dir.join("pk-nod4.json");
    fs::write(
        &pk,
        probe_pack(
            "probe",
            false,
            json!([one_rule("ZZ-001", "missing", "探测", json!([{"op": "exists_readable"}]))]),
        ),
    )
    .unwrap();
    let (code, out, _) = run(&["--pack", pk.to_str().unwrap(), good]);
    assert_eq!(code, 1);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert!(
        rep["findings"].as_array().unwrap().iter().any(|f| f["rule"] == "D4-ANCHOR" && f["state"] == "violation"),
        "D4-ANCHOR 违规须在"
    );
    // required false 即不拦：合规落零。
    let pk = ws.dir.join("pk-d4off.json");
    fs::write(
        &pk,
        probe_pack(
            "probe",
            true,
            json!([one_rule("ZZ-001", "missing", "探测", json!([{"op": "exists_readable"}]))]),
        ),
    )
    .unwrap();
    let (code, _, _) = run(&["--pack", pk.to_str().unwrap(), good]);
    assert_eq!(code, 0);

    // (d) 不可解码目标：仅存在可读类规则落缺件一笔。
    let binpath = ws.dir.join("bin.md");
    fs::write(&binpath, [0xFFu8, 0xFE, 0x80]).unwrap();
    let pk = ws.dir.join("pk-er.json");
    fs::write(
        &pk,
        probe_pack(
            "probe",
            true,
            json!([one_rule("ZZ-001", "missing", "探测", json!([{"op": "exists_readable"}]))]),
        ),
    )
    .unwrap();
    let (code, out, _) = run(&["--pack", pk.to_str().unwrap(), binpath.to_str().unwrap()]);
    assert_eq!(code, 1);
    let rep: Value = serde_json::from_str(&out).unwrap();
    let findings = rep["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0]["state"], "missing");
    assert!(findings[0]["line"].is_null());
    // 非存在类规则无文本可求值 → 零发现合规落零。
    let pk = ws.dir.join("pk-hp.json");
    fs::write(
        &pk,
        probe_pack(
            "probe",
            true,
            json!([one_rule("ZZ-001", "missing", "探测", json!([{"op": "heading_present", "level": 1}]))]),
        ),
    )
    .unwrap();
    let (code, _, _) = run(&["--pack", pk.to_str().unwrap(), binpath.to_str().unwrap()]);
    assert_eq!(code, 0);
}
