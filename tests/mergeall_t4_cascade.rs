//! lease-mergeleg23-parallel 簇E T4：cascade 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/cascade 0.4.0（cli.py 三子命令三值退出码、core.py
//! 引用即边建册、孤儿边侦查、链上最近认证哈希对表上游洁净判定）。fixture 全部
//! temp 自建最小语料与册，围堰真实语料零触碰。三例：正常形（build 边册形加
//! 零孤儿绿态）、拒绝形（孤儿边退出码一加册非法退出码二）、边界形（洁净
//! clean 与无认证史 unverified 与偏离 dirty 三态加空边册）。

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use sha2::{Digest, Sha256};

fn bin_cascade() -> &'static str {
    env!("CARGO_BIN_EXE_cascade")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_cascade()).args(args).output().unwrap();
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

fn canonical(p: &Path) -> String {
    fs::canonicalize(p).unwrap().to_string_lossy().into_owned()
}

struct Ws {
    _guard: tempfile::TempDir,
    dir: PathBuf,
    root: PathBuf,
}

fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let root = dir.join("root");
    fs::create_dir_all(root.join("nested")).unwrap();
    fs::write(
        root.join("DEC-001-alpha.md"),
        "Root decision, consults PRO-002-beta and GOV-009-missing.\n",
    )
    .unwrap();
    fs::write(root.join("PRO-002-beta.md"), "Provider doc.\n").unwrap();
    fs::write(root.join("nested/PRO-004-gamma.md"), "Nested doc refs DEC-001.\n").unwrap();
    Ws {
        _guard: guard,
        dir,
        root,
    }
}

fn root_str(ws: &Ws) -> String {
    ws.root.to_string_lossy().into_owned()
}

// ---------- 正常形：引用即边建册、注记分诊、零孤儿绿态 ----------

#[test]
fn t1_normal_build_edges_and_zero_orphans() {
    let ws = build_ws();
    let reg = ws.dir.join("reg.json");

    // build --out：出册落盘，stdout 报 header 加 written。
    let (code, out, err) = run(&[
        "build",
        "--root",
        &root_str(&ws),
        "--out",
        reg.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "build 须 0，stdout={out} stderr={err}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["header"]["tool"]["name"], "cascade");
    assert_eq!(rep["header"]["tool"]["version"], "0.4.0");
    assert_eq!(
        rep["header"]["root"],
        canonical(&ws.root),
        "header.root 即解析后语料根"
    );
    assert_eq!(rep["written"], reg.to_str().unwrap());
    assert!(reg.is_file(), "册文件已落盘");

    // 册形：前缀族精确匹配建边、自引排除、无主词入注记、零值边账。
    let reg_data: Value =
        serde_json::from_str(&fs::read_to_string(&reg).unwrap()).unwrap();
    assert_eq!(
        reg_data["edges"]["DEC-001-alpha.md"],
        serde_json::json!(["PRO-002-beta.md"]),
        "DEC-001 引用 PRO-002 建边"
    );
    assert_eq!(
        reg_data["edges"]["nested/PRO-004-gamma.md"],
        serde_json::json!(["DEC-001-alpha.md"]),
        "嵌套相对路径即 id"
    );
    assert_eq!(
        reg_data["notes"]["unmapped"],
        serde_json::json!(["GOV-009"]),
        "无主引用词入注记"
    );
    assert_eq!(
        reg_data["notes"]["ambiguous"],
        serde_json::json!([]),
        "唯一解析零多义"
    );
    let ledger = &reg_data["ledger"]["DEC-001-alpha.md"]["PRO-002-beta.md"];
    assert_eq!(ledger["upstream_changes"], 0);
    assert_eq!(ledger["consulted"], 0);
    assert_eq!(ledger["blocked"], 0);
    assert_eq!(ledger["hits"], 0);

    // orphans：上游全在，零孤儿退出码零。
    let (code, out, _) = run(&["orphans", "--registry", reg.to_str().unwrap(), "--root", &root_str(&ws)]);
    assert_eq!(code, 0, "零孤儿须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"]["orphans"], 0);
    assert_eq!(rep["orphans"].as_array().unwrap().len(), 0);
}

// ---------- 拒绝形：孤儿边即边悬空报退出码一；册非法报退出码二 ----------

#[test]
fn t2_reject_orphans_exit_one_and_registry_invalid_exit_two() {
    let ws = build_ws();
    let reg = ws.dir.join("reg.json");
    let (code, _, _) = run(&["build", "--root", &root_str(&ws), "--out", reg.to_str().unwrap()]);
    assert_eq!(code, 0);

    // 上游路径缺席即孤儿：不改名即删除新建规则的机械兜底。
    fs::remove_file(ws.root.join("PRO-002-beta.md")).unwrap();
    let (code, out, _) = run(&["orphans", "--registry", reg.to_str().unwrap(), "--root", &root_str(&ws)]);
    assert_eq!(code, 1, "有孤儿须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    let orphans = rep["orphans"].as_array().unwrap();
    assert_eq!(orphans.len(), 1);
    assert_eq!(orphans[0]["edge"], "DEC-001-alpha.md -> PRO-002-beta.md");
    assert_eq!(orphans[0]["missing"], "PRO-002-beta.md");
    assert_eq!(rep["summary"]["orphans"], 1);

    // 册文件缺席：退出码二。
    let (code, out, _) = run(&["orphans", "--registry", "/nonexistent-t4.json", "--root", &root_str(&ws)]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        err["error"],
        "registry unreadable: /nonexistent-t4.json"
    );

    // 册缺 edges 表：退出码二。
    let bad_reg = ws.dir.join("bad-reg.json");
    fs::write(&bad_reg, r#"{"header":{}}"#).unwrap();
    let (code, out, _) = run(&["orphans", "--registry", bad_reg.to_str().unwrap(), "--root", &root_str(&ws)]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(err["error"], "registry missing edges table");

    // 册 JSON 非法：退出码二。
    let junk = ws.dir.join("junk.json");
    fs::write(&junk, "{not json").unwrap();
    let (code, out, _) = run(&["orphans", "--registry", junk.to_str().unwrap(), "--root", &root_str(&ws)]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert!(
        err["error"]
            .as_str()
            .unwrap()
            .starts_with("registry json invalid:"),
        "error={}",
        err["error"]
    );

    // 语料根缺席 build：退出码二。
    let (code, out, _) = run(&["build", "--root", "/nonexistent-t4-root"]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert!(
        err["error"]
            .as_str()
            .unwrap()
            .starts_with("corpus root missing:"),
        "error={}",
        err["error"]
    );
}

// ---------- 边界形：clean/unverified/dirty 三态洁净判定、空边册绿态、trail 不可读 ----------

#[test]
fn t3_boundary_check_tri_state_and_empty_edges_and_trail_unreadable() {
    let ws = build_ws();
    let reg = ws.dir.join("reg.json");
    let (code, _, _) = run(&["build", "--root", &root_str(&ws), "--out", reg.to_str().unwrap()]);
    assert_eq!(code, 0);

    // 链面 fixture：PRO-002-beta.md 已认证，DEC-001-alpha.md 无认证史。
    let beta_hash = sha256_hex(b"Provider doc.\n");
    let beta_abs = canonical(&ws.root.join("PRO-002-beta.md"));
    let reports = ws.dir.join("reports");
    fs::create_dir_all(&reports).unwrap();
    fs::write(
        reports.join("rep1.json"),
        format!(r#"{{"content_hashes":{{"{}":"{}"}}}}"#, beta_abs, beta_hash),
    )
    .unwrap();
    let trail = ws.dir.join("trail.ndjson");
    fs::write(
        &trail,
        r#"{"event_type":"certification_completed","report_type":"scrutinator","doc_id":"d1","report":"rep1.json"}"#,
    )
    .unwrap();

    // clean + unverified：无脏上游即全 writable，退出码零。
    let (code, out, _) = run(&[
        "check",
        "--registry",
        reg.to_str().unwrap(),
        "--root",
        &root_str(&ws),
        "--trail",
        trail.to_str().unwrap(),
        "--reports-root",
        reports.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "无脏上游须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"]["blocked"], 0);
    assert_eq!(rep["summary"]["targets"], 2);
    assert_eq!(rep["summary"]["dirty_edges"], 0);
    assert_eq!(rep["summary"]["unverified_edges"], 1);
    assert_eq!(rep["summary"]["writable"], 2);
    assert_eq!(rep["header"]["trails"][0], trail.to_str().unwrap());
    let targets = &rep["targets"];
    assert_eq!(
        targets["DEC-001-alpha.md"]["upstreams"]["PRO-002-beta.md"],
        "clean",
        "当前哈希等于链上最近认证哈希即 clean"
    );
    assert_eq!(
        targets["nested/PRO-004-gamma.md"]["upstreams"]["DEC-001-alpha.md"],
        "unverified",
        "无认证史即 unverified 不拦"
    );
    assert_eq!(targets["DEC-001-alpha.md"]["verdict"], "writable");

    // dirty：上游偏离链上哈希即拒写目标 blocked，退出码一。
    fs::write(ws.root.join("PRO-002-beta.md"), "Provider doc changed.\n").unwrap();
    let (code, out, _) = run(&[
        "check",
        "--registry",
        reg.to_str().unwrap(),
        "--root",
        &root_str(&ws),
        "--trail",
        trail.to_str().unwrap(),
        "--reports-root",
        reports.to_str().unwrap(),
    ]);
    assert_eq!(code, 1, "有 blocked 须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"]["blocked"], 1);
    assert_eq!(rep["summary"]["dirty_edges"], 1);
    assert_eq!(rep["summary"]["writable"], 1);
    assert_eq!(rep["targets"]["DEC-001-alpha.md"]["verdict"], "blocked");
    assert_eq!(
        rep["targets"]["DEC-001-alpha.md"]["dirty"],
        serde_json::json!(["PRO-002-beta.md"])
    );

    // 空边册绿态：check 与 orphans 全零退出码。
    let empty_root = ws.dir.join("empty-root");
    fs::create_dir_all(&empty_root).unwrap();
    let empty_reg = ws.dir.join("empty-reg.json");
    let (code, _, _) = run(&[
        "build",
        "--root",
        empty_root.to_str().unwrap(),
        "--out",
        empty_reg.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "空语料建册须 0");
    let (code, out, _) = run(&[
        "check",
        "--registry",
        empty_reg.to_str().unwrap(),
        "--root",
        empty_root.to_str().unwrap(),
        "--trail",
        trail.to_str().unwrap(),
        "--reports-root",
        reports.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "空边册 check 须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"]["targets"], 0);
    let (code, _, _) = run(&[
        "orphans",
        "--registry",
        empty_reg.to_str().unwrap(),
        "--root",
        empty_root.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "空边册 orphans 须 0");

    // trail 不可读：退出码二。
    let (code, out, _) = run(&[
        "check",
        "--registry",
        reg.to_str().unwrap(),
        "--root",
        &root_str(&ws),
        "--trail",
        "/nonexistent-t4.ndjson",
        "--reports-root",
        reports.to_str().unwrap(),
    ]);
    assert_eq!(code, 2);
    let err: Value = serde_json::from_str(&out).unwrap();
    assert!(
        err["error"]
            .as_str()
            .unwrap()
            .starts_with("trail unreadable:"),
        "error={}",
        err["error"]
    );
}
