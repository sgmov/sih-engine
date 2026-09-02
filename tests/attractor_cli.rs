//! deyimerge-tdd-solo T3+T5：退出码全表与机械腿不变量。
//!
//! T3：check 三值、sign 三态（refused 1 / scribe 失败透传 / signed 0）、
//!     verify 两值、watch 两值，与围堰实测全表对齐。
//! T5：机械腿零网络零 LLM 零 key 读取（源码扫描断言 + Cargo 依赖断言 +
//!     离线可跑）。

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use sih_engine::attractor::tally;

fn golden() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/attractor/fixtures/golden")
}

fn bin_attractor() -> &'static str {
    env!("CARGO_BIN_EXE_attractor")
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

fn run_bin(args: &[&str]) -> (i32, Vec<u8>, Vec<u8>) {
    let out = Command::new(bin_attractor()).args(args).output().unwrap();
    (out.status.code().unwrap_or(-1), out.stdout, out.stderr)
}

// ---------- T3 退出码全表 ----------

#[test]
fn t3_check_exit_codes() {
    // 合规 0
    let ws = build_ws("adisp-net");
    let (code, out, _) = run_bin(&["check", "--material", &ws.dir.join("material.json").to_string_lossy()]);
    assert_eq!(code, 0, "合规核对须 0，stdout={}", String::from_utf8_lossy(&out));
    let report: Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(report["disposition"], "裁决通过");
    // 单违规 1
    let ws = build_ws("dirty-return-r2");
    let (code, out, _) = run_bin(&["check", "--material", &ws.dir.join("material.json").to_string_lossy()]);
    assert_eq!(code, 1, "R2 失败核对须 1");
    let report: Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(report["disposition"], "材料退回");
    // 材料缺文件（环境错误）2
    let (code, _, err) = run_bin(&["check", "--material", "/tmp/deyimerge-tdd-nonexistent-material.json"]);
    assert_eq!(code, 2, "缺材料须环境错误 2，stderr={}", String::from_utf8_lossy(&err));
    // kind 不符（用法域）2
    let ws = build_ws("adisp-net");
    let bad = ws.dir.join("bad-kind.json");
    fs::write(&bad, "{\"kind\": \"other\"}\n").unwrap();
    let (code, _, err) = run_bin(&["check", "--material", &bad.to_string_lossy()]);
    assert_eq!(code, 2, "kind 不符须 2，stderr={}", String::from_utf8_lossy(&err));
    assert!(String::from_utf8_lossy(&err).contains("ValueError"), "错误信封须 ValueError 形");
}

#[test]
fn t3_sign_tri_state() {
    // signed 0：fake scribe 成功
    let ws = build_ws("adisp-net");
    let out_dir = ws.dir.join("sign-out");
    let fake = ws.dir.join("fake-scribe.sh");
    fs::write(&fake, "#!/bin/sh\nexit 0\n").unwrap();
    make_executable(&fake);
    let (code, out, _) = run_bin(&["sign",
        "--material", &ws.dir.join("material.json").to_string_lossy(),
        "--out", &out_dir.to_string_lossy(),
        "--trail", "t.ndjson", "--scribe-binary", &fake.to_string_lossy(),
        "--session", "sess-t", "--locks", "locks.ndjson"]);
    assert_eq!(code, 0, "裁决通过签署须 0，out={}", String::from_utf8_lossy(&out));
    let signed: Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(signed["signed"], "adisp-guard-1");
    let signcheck = out_dir.join("adisp-guard-1-signcheck.json");
    assert!(signcheck.is_file(), "签署须落 signcheck");
    let recorded: Value = serde_json::from_str(&fs::read_to_string(&signcheck).unwrap()).unwrap();
    assert_eq!(recorded["disposition"], "裁决通过");
    // refused 1：挂起拒签，不落据
    let ws = build_ws("dirty-suspend-near");
    let out2 = ws.dir.join("sign-out2");
    let (code, out, _) = run_bin(&["sign",
        "--material", &ws.dir.join("material.json").to_string_lossy(),
        "--out", &out2.to_string_lossy(),
        "--trail", "t.ndjson", "--scribe-binary", &fake.to_string_lossy(),
        "--session", "sess-t", "--locks", "locks.ndjson"]);
    assert_eq!(code, 1, "挂起拒签须 1");
    let refused: Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(refused["sign"], "refused");
    assert!(!out2.join("m-x-signcheck.json").exists());
    assert!(!out2.join("adisp-guard-1-signcheck.json").exists(), "拒签不得落据");
    // scribe 失败打 sign failed 透传退出码（3），不出 signed
    let ws = build_ws("adisp-net");
    let out3 = ws.dir.join("sign-out3");
    let fake3 = ws.dir.join("fake-scribe3.sh");
    fs::write(&fake3, "#!/bin/sh\nexit 3\n").unwrap();
    make_executable(&fake3);
    let (code, out, _) = run_bin(&["sign",
        "--material", &ws.dir.join("material.json").to_string_lossy(),
        "--out", &out3.to_string_lossy(),
        "--trail", "t.ndjson", "--scribe-binary", &fake3.to_string_lossy(),
        "--session", "sess-t", "--locks", "locks.ndjson"]);
    assert_eq!(code, 3, "scribe 失败须透传其退出码 3");
    let failed: Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(failed["sign"], "failed");
    assert_eq!(failed["scribe_exit"], 3);
}

#[test]
fn t3_verify_two_values() {
    let ws = build_ws("adisp-net");
    let mat = ws.dir.join("material.json");
    let report = tally::check_material(&mat).unwrap();
    let rp = ws.dir.join("report.json");
    fs::write(&rp, format!("{}\n", sih_engine::attractor::jsonc::canonical_json(&report))).unwrap();
    let (code, out, _) = run_bin(&["verify", "--material", &mat.to_string_lossy(), "--report", &rp.to_string_lossy()]);
    assert_eq!(code, 0, "一致重放须 0，out={}", String::from_utf8_lossy(&out));
    assert!(String::from_utf8_lossy(&out).contains("identical"));
    let mut bad = report.clone();
    bad["disposition"] = Value::String("裁决通过X".into());
    let rp2 = ws.dir.join("bad.json");
    fs::write(&rp2, sih_engine::attractor::jsonc::canonical_json(&bad)).unwrap();
    let (code, out, err) = run_bin(&["verify", "--material", &mat.to_string_lossy(), "--report", &rp2.to_string_lossy()]);
    assert_eq!(code, 1, "分歧重放须 1");
    assert!(String::from_utf8_lossy(&err).contains("divergent"), "divergent 报文出 stderr（对齐围堰）");
    let _ = out;
}

#[test]
fn t3_watch_two_values() {
    // 空目录 0
    let ws = build_ws("adisp-net");
    let empty = ws.dir.join("empty-reports");
    fs::create_dir_all(&empty).unwrap();
    let (code, out, _) = run_bin(&["watch", "--reports", &empty.to_string_lossy()]);
    assert_eq!(code, 0);
    assert!(String::from_utf8_lossy(&out).contains("empty"));
    // 无异常 0 / 边界变更 1
    let reports = ws.dir.join("reports");
    fs::create_dir_all(&reports).unwrap();
    let mat = ws.dir.join("material.json");
    let report = tally::check_material(&mat).unwrap();
    fs::write(reports.join("adisp-guard-1-signcheck.json"),
              format!("{}\n", sih_engine::attractor::jsonc::canonical_json(&report))).unwrap();
    let (code, _, _) = run_bin(&["watch", "--reports", &reports.to_string_lossy()]);
    assert_eq!(code, 0, "签署后无变更加重放须 0");
    // 篡改 topic → 复算不符 → 1
    let topic = ws.dir.join("input/topic.md");
    fs::write(&topic, fs::read_to_string(&topic).unwrap() + "\n篡改\n").unwrap();
    let (code, out, _) = run_bin(&["watch", "--reports", &reports.to_string_lossy()]);
    assert_eq!(code, 1, "边界变更须 1");
    assert!(String::from_utf8_lossy(&out).contains("anomalies"));
}

fn make_executable(p: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perm = fs::metadata(p).unwrap().permissions();
    perm.set_mode(0o755);
    fs::set_permissions(p, perm).unwrap();
}

// ---------- T5 机械腿不变量 ----------

#[test]
fn t5_zero_network_zero_llm_source_scan() {
    // src/attractor/ 与 src/bin/attractor.rs 源码零网络零 LLM 零 key 痕迹
    let forbidden = [
        "llm_client", "AsyncOpenAI", "openai", "reqwest", "hyper", "ureq",
        "curl", "TcpStream", "UdpSocket", "tokio::net", "api_key", "apiKey",
        "env_loader", "HTTP", "https://", "http://",
    ];
    let mut sources = Vec::new();
    collect_rs(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/attractor"), &mut sources);
    sources.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/bin/attractor.rs"));
    assert!(!sources.is_empty());
    for src in &sources {
        let text = fs::read_to_string(src).unwrap();
        for f in forbidden {
            assert!(!text.contains(f), "{} 含禁词 {}（机械腿零网络零 LLM 红线）", src.display(), f);
        }
    }
}

#[test]
fn t5_zero_network_deps_in_cargo() {
    let cargo = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")).unwrap();
    for dep in ["reqwest", "hyper", "curl", "ureq", "tokio", "surf", "attohttpc", "openai"] {
        assert!(!cargo.contains(dep), "Cargo.toml 含网络或 LLM 依赖 {dep}");
    }
}

#[test]
fn t5_offline_runnable() {
    // 离线可跑：check 全流程零网络零外部进程（除 sign 的书简子进程）
    let ws = build_ws("p3xcarr-net");
    let mat = ws.dir.join("material.json");
    let (code, out, _) = run_bin(&["check", "--material", &mat.to_string_lossy()]);
    assert_eq!(code, 0, "离线 check 须全过：{}", String::from_utf8_lossy(&out));
    // 目标仓零写入：check 后工作区除既有文件外零新增
    let before: Vec<_> = walk(&ws.dir);
    let _ = run_bin(&["check", "--material", &mat.to_string_lossy()]);
    let after: Vec<_> = walk(&ws.dir);
    assert_eq!(before, after, "check 不得写任何文件");
}

fn walk(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for f in fs::read_dir(dir).unwrap() {
        let f = f.unwrap();
        let name = f.file_name().to_string_lossy().to_string();
        if f.path().is_dir() {
            for sub in walk(&f.path()) {
                out.push(format!("{name}/{sub}"));
            }
        } else {
            out.push(name);
        }
    }
    out.sort();
    out
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    for f in fs::read_dir(dir).unwrap() {
        let f = f.unwrap();
        if f.path().is_dir() {
            collect_rs(&f.path(), out);
        } else if f.path().extension().map(|e| e == "rs").unwrap_or(false) {
            out.push(f.path());
        }
    }
}
