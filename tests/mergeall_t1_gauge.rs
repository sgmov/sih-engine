//! lease-mergeall-parallel 簇A T1：gauge 引擎 bin 金向量三例。
//! 正常形：read 三维读数（convergence/adoption/mergeback）ga-2 值对表
//! 与 record 落链回执组装；拒绝形：维度违例 exit 1 与缺旗标 exit 2 与
//! 落链拒 exit 1；边界形：空会话账本零虚构形。scribe 用 fixture 假件承接。

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

fn write_file(p: &Path, s: &str) {
    if let Some(d) = p.parent() {
        std::fs::create_dir_all(d).unwrap();
    }
    std::fs::write(p, s).unwrap();
}

fn mkdir(p: &Path) {
    std::fs::create_dir_all(p).unwrap();
}

struct Ws {
    root: PathBuf,
    trail: PathBuf,
    sessions: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_ws(sessions_content: &str) -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let root = guard.path().to_path_buf();
    // convergence 组件面：src 下两目录，链事件命中其一
    mkdir(&root.join("src/alpha"));
    mkdir(&root.join("src/beta"));
    // mergeback 工具面：三目录
    mkdir(&root.join("tools/attnanchor"));
    mkdir(&root.join("tools/critsweep"));
    mkdir(&root.join("tools/gauge"));
    let trail = root.join("trail.ndjson");
    write_file(
        &trail,
        "{\"event_hash\":\"aaaabbbb\",\"event_type\":\"certification_completed\",\"timestamp\":\"2026-09-13T10:00:00\",\"details\":{\"record_path\":\"plan/mergeall/src/alpha/x.md\"}}\n",
    );
    let sessions = root.join("sessions.ndjson");
    write_file(&sessions, sessions_content);
    Ws { root, trail, sessions, _guard: guard }
}

const TWO_ISSUED: &str = "{\"session_id\":\"s1\",\"event\":\"issued\",\"issued_at\":\"2026-09-13T09:00:00\",\"identity\":{\"identity_hash\":\"h1\"}}\n\
                          {\"session_id\":\"s2\",\"event\":\"issued\",\"issued_at\":\"2026-09-13T10:00:00\"}\n";

fn write_script(dir: &Path, name: &str, body: &str) -> PathBuf {
    let p = dir.join(name);
    std::fs::write(&p, body).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    p
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_gauge"))
        .args(args)
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

fn read_args(ws: &Ws, dim: &str) -> Vec<String> {
    vec![
        "read".into(),
        "--dimension".into(),
        dim.into(),
        "--at".into(),
        "2026-09-13".into(),
        "--trail".into(),
        ws.trail.to_string_lossy().to_string(),
        "--sessions-ledger".into(),
        ws.sessions.to_string_lossy().to_string(),
        "--src-root".into(),
        ws.root.to_string_lossy().to_string(),
        "--tools-root".into(),
        ws.root.join("tools").to_string_lossy().to_string(),
    ]
}

fn run_read(ws: &Ws, dim: &str) -> (i32, Value, String) {
    let a = read_args(ws, dim);
    let refs: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
    let (rc, so, se) = run(&refs);
    let v: Value = serde_json::from_str(&so).expect("出参须为严格 JSON 单对象");
    (rc, v, se)
}

#[test]
fn t1_normal_three_dimensions_and_record() {
    let ws = build_ws(TWO_ISSUED);

    // convergence：组件命中一半，inputs 取命中事件哈希
    let (rc, v, se) = run_read(&ws, "convergence");
    assert_eq!(rc, 0, "stderr: {}", se);
    assert_eq!(jv(&v["reading"]["value"]), serde_json::json!(0.5));
    assert_eq!(jv(&v["reading"]["subject"]), "sih-engine");
    assert_eq!(jv(&v["reading"]["formula_version"]), "ga-2");
    assert_eq!(jv(&v["reading"]["window"]), "2026-09-13/2026-09-13");
    assert_eq!(
        v["reading"]["inputs_digest"].as_str().unwrap().len(),
        64,
        "inputs_digest 六十四位十六进制"
    );
    assert_eq!(v["inputs"]["events"], serde_json::json!(["aaaabbbb"]));
    assert_eq!(jv(&v["sequence"]), "insufficient", "零 history 如实");
    assert!(v.get("evidence_basis").is_none(), "底座全在域内出参零增");

    // adoption：k=1 n=2，置信带与后验均值随行
    let (rc, v, se) = run_read(&ws, "adoption");
    assert_eq!(rc, 0, "stderr: {}", se);
    assert_eq!(jv(&v["reading"]["value"]), serde_json::json!(0.5));
    assert_eq!(jv(&v["reading"]["subject"]), "agents");
    assert_eq!(jv(&v["reading"]["posterior_mean"]), serde_json::json!(0.5));
    assert_eq!(jv(&v["reading"]["confidence_band"]["lower"]), serde_json::json!(0.0));
    assert_eq!(jv(&v["reading"]["confidence_band"]["upper"]), serde_json::json!(1.0));

    // mergeback：三工具两融回
    let mut a = read_args(&ws, "mergeback");
    a.push("--merged".into());
    a.push("gauge,scribe".into());
    let refs: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
    let (rc, so, se) = run(&refs);
    assert_eq!(rc, 0, "stderr: {}", se);
    let v: Value = serde_json::from_str(&so).unwrap();
    assert_eq!(jv(&v["reading"]["value"]), serde_json::json!(0.666667));
    assert_eq!(jv(&v["reading"]["subject"]), "tools-wheel");

    // record 落链：fixture 假书简承接回执组装
    let scribe = write_script(
        &ws.root,
        "fake-scribe-ok.sh",
        "#!/bin/sh\necho '{\"event_hash\": \"cafe12\"}'\nexit 0\n",
    );
    let record_trail = ws.root.join("out.ndjson");
    let mut a = read_args(&ws, "convergence");
    a[0] = "record".into();
    a.push("--scribe".into());
    a.push(scribe.to_string_lossy().to_string());
    a.push("--record-trail".into());
    a.push(record_trail.to_string_lossy().to_string());
    let refs: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
    let (rc, so, se) = run(&refs);
    assert_eq!(rc, 0, "stderr: {}", se);
    let v: Value = serde_json::from_str(&so).unwrap();
    assert_eq!(jv(&v["formula_version"]), "ga-2");
    assert_eq!(jv(&v["record_trail"]), record_trail.to_string_lossy().to_string());
    assert_eq!(jv(&v["recorded"][0]["dimension"]), "convergence");
    assert_eq!(jv(&v["recorded"][0]["event_hash"]), "cafe12");
    assert_eq!(jv(&v["recorded"][0]["subject"]), "sih-engine");
}

fn jv(v: &Value) -> Value {
    v.clone()
}

#[test]
fn t2_reject_dimension_and_flags_and_chain_refuse() {
    let ws = build_ws(TWO_ISSUED);

    // 维度违例：read 未知维 exit 1
    let a = read_args(&ws, "bogus");
    let refs: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
    let (rc, so, se) = run(&refs);
    assert_eq!(rc, 1, "维度违例退出码一");
    assert!(se.contains("维度违例"), "error 键在 stderr：{}", se);
    assert!(so.is_empty());

    // 缺必填旗标：record 缺 --scribe exit 2
    let mut a = read_args(&ws, "convergence");
    a[0] = "record".into();
    let refs: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
    let (rc, so, se) = run(&refs);
    assert_eq!(rc, 2, "缺旗标退出码二");
    assert!(se.contains("error"), "stderr 载 error：{}", se);
    assert!(so.is_empty(), "拒形不出 stdout 读数");

    // 落链拒：假书简执法位非零退出透传
    let scribe = write_script(
        &ws.root,
        "fake-scribe-deny.sh",
        "#!/bin/sh\necho denied >&2\nexit 3\n",
    );
    let mut a = read_args(&ws, "convergence");
    a[0] = "record".into();
    a.push("--scribe".into());
    a.push(scribe.to_string_lossy().to_string());
    a.push("--record-trail".into());
    a.push(ws.root.join("out2.ndjson").to_string_lossy().to_string());
    let refs: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
    let (rc, so, se) = run(&refs);
    assert_eq!(rc, 1, "落链拒退出码一");
    assert!(se.contains("落链拒"), "error 键在 stderr：{}", se);
    assert!(se.contains("denied"), "书简 stderr 截段透传：{}", se);
    assert!(so.is_empty());
}

#[test]
fn t3_edge_empty_sessions_zero_fiction() {
    let ws = build_ws("");
    let (rc, v, se) = run_read(&ws, "adoption");
    assert_eq!(rc, 0, "stderr: {}", se);
    assert_eq!(jv(&v["reading"]["value"]), serde_json::json!(0.0), "n=0 即 0.0 零虚构");
    assert!(
        v["reading"].get("confidence_band").is_none(),
        "n=0 不出置信带"
    );
    assert!(
        v["reading"].get("posterior_mean").is_none(),
        "n=0 不出后验均值"
    );
    assert_eq!(jv(&v["sequence"]), "insufficient");
}
