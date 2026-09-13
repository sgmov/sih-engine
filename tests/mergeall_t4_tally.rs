//! lease-mergeleg23-parallel 簇F T4：tally 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/tally 1.0.0（cli.py：check R1-R9 核对与 DES-011 四值
//! 处置、verify 重放比对、sign 机器终签外调、watch 批量重放）。fixture 全部
//! temp 自建最小材料件，围堰真实命题区零触碰。

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn bin_tally() -> &'static str {
    env!("CARGO_BIN_EXE_tally")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_tally()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn sha256_hex(b: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b);
    hex::encode(h.finalize())
}

/// 与 bin 同形的 Python json.dumps(sort_keys=True) 规范行（fingerprint 复算用）。
fn py_escape(s: &str) -> String {
    let mut out = String::new();
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn py_line_sorted(v: &Value) -> String {
    fn rec(v: &Value, out: &mut String) {
        match v {
            Value::Object(m) => {
                let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
                kvs.sort_by(|a, b| a.0.cmp(b.0));
                out.push('{');
                for (i, (k, val)) in kvs.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&py_escape(k));
                    out.push_str(": ");
                    rec(val, out);
                }
                out.push('}');
            }
            Value::Array(a) => {
                out.push('[');
                for (i, val) in a.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    rec(val, out);
                }
                out.push(']');
            }
            Value::Null => out.push_str("null"),
            Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Value::Number(n) => out.push_str(&n.to_string()),
            Value::String(s) => out.push_str(&py_escape(s)),
        }
    }
    let mut s = String::new();
    rec(v, &mut s);
    s
}

struct Fx {
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn per_actor_dc() -> Value {
    json!({
        "per_actor": [
            {
                "actor_id": "a1",
                "model_id": "m1",
                "family": "f1",
                "decision": "comply",
                "basis_regulation": "BASE-1",
                "boundary_flag": false,
                "reason": "r1"
            }
        ]
    })
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();

    let topic = dir.join("topic.md");
    fs::write(&topic, "命题正文 topic-t4\n").unwrap();

    let dc = per_actor_dc();
    let trail_line = json!({
        "trail_type": "flywheel_run",
        "guidance_id": "gid-t4",
        "decision_convergence": dc,
    });
    let trail = dir.join("flywheel-trail.jsonl");
    fs::write(&trail, format!("{trail_line}\n")).unwrap();

    let baseline = dir.join("seat-baseline.json");
    fs::write(&baseline, r#"{"verdict": "可用"}"#).unwrap();

    Fx { dir, _guard: guard }
}

fn material_json(fx: &Fx, gate_verdict: &str, topic_sha: &str) -> Value {
    let dc_list = json!([per_actor_dc()]);
    let fingerprint = sha256_hex(py_line_sorted(&dc_list).as_bytes())[..16].to_string();
    json!({
        "kind": "tally-check-input",
        "gid": "gid-t4",
        "topic_path": fx.dir.join("topic.md"),
        "topic_sha256": topic_sha,
        "trail_path": fx.dir.join("flywheel-trail.jsonl"),
        "dc_fingerprint": fingerprint,
        "gate_verdict": gate_verdict,
        "criteria_version": "v3",
        "seat_baseline_path": fx.dir.join("seat-baseline.json"),
    })
}

fn write_material(fx: &Fx, name: &str, m: &Value) -> String {
    let p = fx.dir.join(name);
    fs::write(&p, serde_json::to_string_pretty(m).unwrap()).unwrap();
    p.to_string_lossy().into_owned()
}

fn good_material_path(fx: &Fx) -> String {
    let topic_sha = sha256_hex(&fs::read(fx.dir.join("topic.md")).unwrap());
    write_material(fx, "material.json", &material_json(fx, "stable_clear", &topic_sha))
}

// ---------- 正常形：全过核对、verify 重放一致、sign 机器终签 ----------

#[test]
fn t1_normal_check_verify_sign() {
    let fx = build_fx();
    let mpath = good_material_path(&fx);

    let (code, out, err) = run(&["check", "--material", &mpath]);
    assert_eq!(code, 0, "全过核对须 0，stdout={out} stderr={err}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["tool"], "tally");
    assert_eq!(rep["version"], "1.0.0");
    assert_eq!(rep["disposition"], "裁决通过");
    assert_eq!(rep["direction"], "comply");
    assert_eq!(rep["verdict"], "pass");
    let passed = rep["passed"].as_array().unwrap();
    assert!(passed.iter().any(|p| p.as_str().unwrap().starts_with("R1: ")));
    assert!(passed.iter().any(|p| p.as_str().unwrap() == "R2: topic_sha256 复算一致"));
    assert!(passed.iter().any(|p| p.as_str().unwrap() == "R3: dc_fingerprint 复算一致"));
    assert!(passed.iter().any(|p| p.as_str().unwrap() == "R5: 席位当日基线判定可用"));
    assert!(passed.iter().any(|p| p.as_str().unwrap() == "R6: 同 gid 累计 1 发未超预算"));
    assert_eq!(rep["failed"].as_array().unwrap().len(), 0);

    // verify：同输入重放与既报逐字节一致
    let rpath = fx.dir.join("signcheck").join("gid-t4-signcheck.json");
    fs::create_dir_all(rpath.parent().unwrap()).unwrap();
    fs::write(&rpath, out.as_bytes()).unwrap();
    let (code, vout, _) = run(&["verify", "--material", &mpath, "--report", &rpath.to_string_lossy()]);
    assert_eq!(code, 0, "重放一致须 0");
    let vrep: Value = serde_json::from_str(&vout).unwrap();
    assert_eq!(vrep["verify"], "identical");
    assert_eq!(vrep["gid"], "gid-t4");

    // sign：仅裁决通过可落据；scribe-binary 用 /bin/echo 桩（退出 0）
    let out_dir = fx.dir.join("signout");
    let (code, sout, _) = run(&[
        "sign",
        "--material",
        &mpath,
        "--out",
        &out_dir.to_string_lossy(),
        "--trail",
        "t.ndjson",
        "--scribe-binary",
        "/bin/echo",
        "--session",
        "s1",
        "--locks",
        "locks.ndjson",
    ]);
    assert_eq!(code, 0, "终签须 0，stdout={sout}");
    // scribe-binary 用 /bin/echo 桩：其输出透传混入 stdout，取末行 JSON
    let srep: Value = serde_json::from_str(sout.trim().lines().last().unwrap()).unwrap();
    assert_eq!(srep["signed"], "gid-t4");
    assert!(out_dir.join("gid-t4-signcheck.json").is_file(), "核对报告须落盘");

    // watch：签署报告批量重放，无异常
    let (code, wout, _) = run(&["watch", "--reports", &out_dir.to_string_lossy()]);
    assert_eq!(code, 0, "watch 无异常须 0，stdout={wout}");
    let wrep: Value = serde_json::from_str(&wout).unwrap();
    assert_eq!(wrep["watch"], "done");
    assert_eq!(wrep["ok"], 1);
    assert_eq!(wrep["anomalies"].as_array().unwrap().len(), 0);
}

// ---------- 拒绝形：kind 不符、哈希篡改、终签拒绝、watch 异常视图 ----------

#[test]
fn t2_reject_forms() {
    let fx = build_fx();
    let topic_sha = sha256_hex(&fs::read(fx.dir.join("topic.md")).unwrap());

    // kind 不符：环境错退出码 2，stderr JSON error
    let bad_kind = fx.dir.join("bad-kind.json");
    let mut m = material_json(&fx, "stable_clear", &topic_sha);
    m["kind"] = json!("other-kind");
    fs::write(&bad_kind, serde_json::to_string_pretty(&m).unwrap()).unwrap();
    let (code, out, err) = run(&["check", "--material", &bad_kind.to_string_lossy()]);
    assert_eq!(code, 2, "kind 不符须 2");
    assert!(out.is_empty());
    assert!(err.contains("kind 不符"), "stderr 须报 kind 不符，err={err}");

    // topic_sha256 篡改：R2 失败，材料退回，退出码 1
    let mpath = write_material(&fx, "material.json", &material_json(&fx, "stable_clear", "deadbeef"));
    let (code, out, _) = run(&["check", "--material", &mpath]);
    assert_eq!(code, 1, "规则失败须 1");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["verdict"], "fail");
    assert_eq!(rep["disposition"], "材料退回");
    let failed = rep["failed"].as_array().unwrap();
    assert_eq!(failed[0]["rule"], "R2");
    assert_eq!(failed[0]["where"], "topic_sha256 与 topic.md 复算不符");

    // sign 拒绝：处置非裁决通过即拒，退出码 1
    let out_dir = fx.dir.join("signout2");
    let (code, sout, _) = run(&[
        "sign",
        "--material",
        &mpath,
        "--out",
        &out_dir.to_string_lossy(),
        "--trail",
        "t.ndjson",
        "--scribe-binary",
        "/bin/echo",
        "--session",
        "s1",
        "--locks",
        "l.ndjson",
    ]);
    assert_eq!(code, 1, "非裁决通过终签须拒");
    let srep: Value = serde_json::from_str(&sout).unwrap();
    assert_eq!(srep["sign"], "refused");
    assert!(!out_dir.exists(), "拒绝形不落任何盘");

    // watch：篡改既报即异常视图，退出码 1
    let reports_dir = fx.dir.join("watchreports");
    fs::create_dir_all(&reports_dir).unwrap();
    let good = good_material_path(&fx);
    let (c0, good_out, _) = run(&["check", "--material", &good]);
    assert_eq!(c0, 0);
    let mut tampered: Value = serde_json::from_str(&good_out).unwrap();
    tampered["disposition"] = json!("挂起");
    let rp = reports_dir.join("gid-t4-signcheck.json");
    fs::write(&rp, serde_json::to_string_pretty(&tampered).unwrap()).unwrap();
    let (code, wout, _) = run(&["watch", "--reports", &reports_dir.to_string_lossy()]);
    assert_eq!(code, 1, "watch 有异常须 1，stdout={wout}");
    let wrep: Value = serde_json::from_str(&wout).unwrap();
    assert_eq!(wrep["anomalies"].as_array().unwrap().len(), 1);

    // 缺 --material：用法错退出码 2
    let (code, _, _) = run(&["check"]);
    assert_eq!(code, 2, "缺 --material 须 2");
}

// ---------- 边界形：boundary 打回重作、R8 挂起、verify 分叉、材料缺席 ----------

#[test]
fn t3_edge_boundary_r8_divergent_missing() {
    let fx = build_fx();
    let topic_sha = sha256_hex(&fs::read(fx.dir.join("topic.md")).unwrap());

    // boundary 且核对全过：打回重作、verdict pass、退出码 0（pass 只指核对全过）
    let mpath = write_material(&fx, "material.json", &material_json(&fx, "boundary", &topic_sha));
    let (code, out, _) = run(&["check", "--material", &mpath]);
    assert_eq!(code, 0, "boundary 无规则失败须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["disposition"], "打回重作");
    assert_eq!(rep["verdict"], "pass");

    // R8 推导档缺席：rules_version des-011-r2 → 挂起，退出码 0
    let mut m8 = material_json(&fx, "stable_clear", &topic_sha);
    m8["rules_version"] = json!("des-011-r2");
    let m8path = write_material(&fx, "material-r8.json", &m8);
    let (code, out, _) = run(&["check", "--material", &m8path]);
    assert_eq!(code, 0, "挂起无 failed 须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["disposition"], "挂起");
    let passed = rep["passed"].as_array().unwrap();
    assert!(passed.iter().any(|p| p.as_str().unwrap() == "R8: 推导档指针缺席（处置走优先级映射）"));

    // verify 分叉：复算与既报不符 → 退出码 1，stderr divergent
    let good_mpath = good_material_path(&fx);
    let (c0, good_out, _) = run(&["check", "--material", &good_mpath]);
    assert_eq!(c0, 0);
    let rpath = fx.dir.join("recorded.json");
    fs::write(&rpath, good_out.as_bytes()).unwrap();
    let tampered_m = write_material(&fx, "material.json", &material_json(&fx, "stable_clear", "deadbeef"));
    let (code, _, verr) = run(&["verify", "--material", &tampered_m, "--report", &rpath.to_string_lossy()]);
    assert_eq!(code, 1, "复算分叉须 1");
    assert!(verr.contains("divergent"), "stderr 须报 divergent，err={verr}");

    // 材料文件缺席：环境错退出码 2
    let (code, _, err) = run(&["check", "--material", &fx.dir.join("nope.json").to_string_lossy()]);
    assert_eq!(code, 2, "材料缺席须 2");
    assert!(err.contains("error"), "stderr 须报 error，err={err}");
}
