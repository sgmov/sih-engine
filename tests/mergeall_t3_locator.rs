//! lease-mergeleg23-parallel 簇D T3：locator（寻址）引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/locator 0.1.0（pack.py 域包与遍历、builder.py 建索引、
//! entry.py 与 identity.py 条目与稳定标识、storage.py 文件后端、query.py 查询、
//! stale.py 陈旧三态）。fixture 全部 temp 自建最小域包与语料，围堰真实包零触碰
//! （真实语料对表已在本批施工对表留痕：围堰 vectors 语料 md 与 json 与 code 条目
//! 同 Python 冻结期望逐字节一致、双跑一致、stale 全绿）。
//! 落差项见 src/bin/locator.rs 头注（toml 键序、yaml 行位、vectors 子命令未移植），
//! fixture 相应避开非确定面。

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn bin_locator() -> &'static str {
    env!("CARGO_BIN_EXE_locator")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_locator()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn sha_hex(s: &str) -> String {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    hex::encode(h.finalize())
}

fn stable_id(path: &str, carrier: &str, kind: &str, seq: u64, chash: &str) -> String {
    sha_hex(&format!("{path}\u{0}{carrier}:{kind}\u{0}{seq}\u{0}{chash}"))
}

struct Ws {
    dir: PathBuf,
    pack: PathBuf,
    corpus: PathBuf,
    _guard: tempfile::TempDir,
}

/// temp 自建最小域包与语料：a.md（标题加段落加列表项）加 b.json（两键），
/// include 仅 md 与 json，避开 toml 键序与 yaml 行位两申报落差面。
fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new_in("/tmp").unwrap();
    let dir = guard.path().to_path_buf();
    let pack = dir.join("pack.json");
    fs::write(
        &pack,
        r#"{"name":"t3loc","version":"1","include":["**/*.md","**/*.json"],"exclude":[],"stale_threshold":0,"max_file_bytes":100000}"#,
    )
    .unwrap();
    let corpus = dir.join("corpus");
    fs::create_dir_all(&corpus).unwrap();
    fs::write(corpus.join("a.md"), "# 标题\n\n段落 body。\n- 项甲\n").unwrap();
    fs::write(corpus.join("b.json"), r#"{"k": "v", "n": 2}"#).unwrap();
    Ws {
        dir,
        pack,
        corpus,
        _guard: guard,
    }
}

// ---------- 正常形：建索引、稳定标识复算、双跑逐字节一致、term/ref 查得、stale 三态 ----------

#[test]
fn t1_normal_build_query_stale() {
    let ws = build_ws();
    let pa = ws.pack.to_string_lossy().into_owned();
    let root = ws.corpus.to_string_lossy().into_owned();
    let idx = ws.dir.join("index.ndjson");
    let ia = idx.to_string_lossy().into_owned();

    // build：退出码零，摘要两文件五条目（md 标题加段落加列表项三条加 json 两键）。
    let (code, out, err) = run(&["build", "--pack", &pa, "--root", &root, "--out", &ia]);
    assert_eq!(code, 0, "build 须 0，stderr={err}");
    let summary: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(summary["files"], 2);
    assert_eq!(summary["entries"], 5);
    assert_eq!(summary["parse_errors"].as_array().unwrap().len(), 0);

    // 索引件形：header 行与 file 行与 entry 行，键序固定。
    let nd = fs::read_to_string(&idx).unwrap();
    let lines: Vec<&str> = nd.lines().collect();
    assert_eq!(lines.len(), 1 + 2 + 5, "header 加两 file 加五 entry");
    let header: Value = serde_json::from_str(lines[0]).unwrap();
    let hkeys: Vec<&str> = header.as_object().unwrap().keys().map(|k| k.as_str()).collect();
    assert_eq!(
        hkeys,
        vec!["type", "tool", "tool_version", "pack", "carriers", "file_count", "entry_count", "parse_errors"]
    );
    assert_eq!(header["tool"], "locator");
    assert_eq!(header["tool_version"], "0.1.0");
    assert_eq!(header["pack"]["name"], "t3loc");
    assert_eq!(header["file_count"], 2);
    let frow: Value = serde_json::from_str(lines[1]).unwrap();
    let fkeys: Vec<&str> = frow.as_object().unwrap().keys().map(|k| k.as_str()).collect();
    assert_eq!(fkeys, vec!["type", "path", "content_hash", "entries"]);

    // 条目序：a.md 三条（heading/para/list_item）后 b.json 两条（key k、key n）。
    let rows: Vec<Value> = lines[1..]
        .iter()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|r| r["type"] == "entry")
        .collect();
    let ekeys: Vec<&str> = rows[0].as_object().unwrap().keys().map(|k| k.as_str()).collect();
    assert_eq!(
        ekeys,
        vec!["type", "id", "path", "carrier", "kind", "name", "seq", "line_start", "line_end", "content_hash", "text"]
    );
    assert_eq!(rows[0]["kind"], "heading");
    assert_eq!(rows[0]["name"], "标题");
    assert_eq!(rows[0]["line_start"], 1);
    assert_eq!(rows[1]["kind"], "para");
    assert_eq!(rows[1]["text"], "段落 body。");
    assert_eq!(rows[2]["kind"], "list_item");
    assert_eq!(rows[2]["text"], "项甲");
    assert_eq!(rows[3]["path"], "b.json");
    assert_eq!(rows[3]["carrier"], "json");
    assert_eq!(rows[3]["kind"], "key");
    assert_eq!(rows[3]["name"], "k");
    assert_eq!(rows[3]["text"], "v");
    assert_eq!(rows[4]["name"], "n");
    assert_eq!(rows[4]["text"], "2");

    // 稳定标识复算（DES-009：路径加载体类型加序号加内容哈希）。
    let want_k = stable_id("b.json", "json", "key", 1, &sha_hex("v"));
    assert_eq!(rows[3]["id"].as_str().unwrap(), want_k, "json k 键标识须复算一致");
    let want_h = stable_id("a.md", "markdown", "heading", 1, &sha_hex("标题"));
    assert_eq!(rows[0]["id"].as_str().unwrap(), want_h);

    // 双跑逐字节一致（快照复现纪律）。
    let idx2 = ws.dir.join("index2.ndjson");
    let ia2 = idx2.to_string_lossy().into_owned();
    let (code, _, _) = run(&["build", "--pack", &pa, "--root", &root, "--out", &ia2]);
    assert_eq!(code, 0);
    assert_eq!(fs::read(&idx).unwrap(), fs::read(&idx2).unwrap(), "双跑须逐字节一致");

    // query term：按符号名查得标题条目，退出码零。
    let (code, out, _) = run(&["query", "--index", &ia, "--term", "标题"]);
    assert_eq!(code, 0, "term 查得须 0");
    let rep: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(rep["mode"], "term");
    assert_eq!(rep["entries"].as_array().unwrap().len(), 1);
    assert_eq!(rep["entries"][0]["id"].as_str().unwrap(), want_h);

    // query ref：条目反查加出现层无状态扫描，列位按码点计。
    let (code, out, _) = run(&["query", "--index", &ia, "--ref", "body", "--pack", &pa, "--root", &root]);
    assert_eq!(code, 0, "ref 查得须 0");
    let rep: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(rep["entries"].as_array().unwrap().len(), 1, "para 条目 name/text 反查命中");
    let occ = rep["occurrences"].as_array().unwrap();
    assert_eq!(occ.len(), 1);
    assert_eq!(occ[0]["path"], "a.md");
    assert_eq!(occ[0]["line"], 3);
    assert_eq!(occ[0]["col"], 4, "段落 body。 中 body 起于第四码点");

    // stale：新建索引对现语料全 fresh，退出码零不须重建。
    let (code, out, _) = run(&["stale", "--pack", &pa, "--root", &root, "--index", &ia]);
    assert_eq!(code, 0, "fresh 须 0");
    let rep: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(rep["rebuild_required"], false);
    assert_eq!(rep["over_threshold"], false);
    assert_eq!(rep["reason"], Value::Null);
    assert_eq!(rep["files"]["fresh"].as_array().unwrap().len(), 2);

    // stale：语料变更即 stale 三态对表，退出码一。
    fs::write(ws.corpus.join("a.md"), "# 改\n").unwrap();
    fs::write(ws.corpus.join("new.md"), "# 新\n").unwrap();
    fs::remove_file(ws.corpus.join("b.json")).unwrap();
    let (code, out, _) = run(&["stale", "--pack", &pa, "--root", &root, "--index", &ia]);
    assert_eq!(code, 1, "有 stale 须 1");
    let rep: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(rep["files"]["stale"], serde_json::json!(["a.md"]));
    assert_eq!(rep["files"]["missing"], serde_json::json!(["b.json", "new.md"]));
    assert_eq!(rep["rebuild_required"], true);
    assert_eq!(rep["over_threshold"], true);
}

// ---------- 拒绝形：索引缺席、语料根缺席、查无所指、缺参、域包坏 ----------

#[test]
fn t2_reject_forms() {
    let ws = build_ws();
    let pa = ws.pack.to_string_lossy().into_owned();
    let root = ws.corpus.to_string_lossy().into_owned();
    let idx = ws.dir.join("index.ndjson");
    let ia = idx.to_string_lossy().into_owned();
    let (code, _, _) = run(&["build", "--pack", &pa, "--root", &root, "--out", &ia]);
    assert_eq!(code, 0);

    // 索引件缺席：工具异常退出码二，stderr error 键。
    let (code, out, err) = run(&["query", "--index", "/tmp/t3loc-no-such-index.ndjson", "--id", "x"]);
    assert_eq!(code, 2, "索引缺席须 2，err={err}");
    assert!(out.is_empty());
    let e: Value = serde_json::from_str(err.trim()).unwrap();
    assert!(e["error"].is_string(), "stderr 须 error 键 JSON，err={err}");

    // 语料根缺席：build 与 stale 同退出码二。
    let (code, _, err) = run(&["build", "--pack", &pa, "--root", "/tmp/t3loc-no-such-root", "--out", "/tmp/t3loc-x.ndjson"]);
    assert_eq!(code, 2, "根缺席须 2，err={err}");
    let e: Value = serde_json::from_str(err.trim()).unwrap();
    assert!(e["error"].is_string());
    let (code, _, _) = run(&["stale", "--pack", &pa, "--root", "/tmp/t3loc-no-such-root", "--index", &ia]);
    assert_eq!(code, 2);

    // 查无所指：退出码一，entries 与 occurrences 如实空。
    let (code, out, _) = run(&["query", "--index", &ia, "--id", "deadbeef"]);
    assert_eq!(code, 1, "查无所指须 1");
    let rep: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(rep["entries"].as_array().unwrap().len(), 0);
    assert_eq!(rep["occurrences"].as_array().unwrap().len(), 0);

    // ref 缺 --pack 与 --root：退出码二 error 键。
    let (code, _, err) = run(&["query", "--index", &ia, "--ref", "body"]);
    assert_eq!(code, 2, "ref 缺参须 2，err={err}");
    let e: Value = serde_json::from_str(err.trim()).unwrap();
    assert_eq!(e["error"], "ref 查询需 --pack 与 --root");

    // 域包坏：pack.json 缺 name 键即工具异常退出码二。
    let badpack = ws.dir.join("badpack.json");
    fs::write(&badpack, r#"{"version":"1","include":["**/*.md"]}"#).unwrap();
    let ba = badpack.to_string_lossy().into_owned();
    let (code, _, err) = run(&["build", "--pack", &ba, "--root", &root, "--out", "/tmp/t3loc-bad.ndjson"]);
    assert_eq!(code, 2, "域包坏须 2，err={err}");
    let e: Value = serde_json::from_str(err.trim()).unwrap();
    assert!(e["error"].is_string());

    // 缺参：build 缺 --out 即退出码二。
    let (code, _, _) = run(&["build", "--pack", &pa, "--root", &root]);
    assert_eq!(code, 2, "缺 --out 须 2");
}

// ---------- 边界形：空语料零文件、空文件入 parse_errors、零命中如实空 ----------

#[test]
fn t3_edge_empty_corpus_and_zero_hit() {
    let ws = build_ws();
    let pa = ws.pack.to_string_lossy().into_owned();

    // 空语料目录：建索引零文件零条目，退出码零，索引件仅头部行。
    let empty = ws.dir.join("emptycorpus");
    fs::create_dir_all(&empty).unwrap();
    let ea = empty.to_string_lossy().into_owned();
    let idx = ws.dir.join("empty.ndjson");
    let ia = idx.to_string_lossy().into_owned();
    let (code, out, err) = run(&["build", "--pack", &pa, "--root", &ea, "--out", &ia]);
    assert_eq!(code, 0, "空语料须 0，stderr={err}");
    let summary: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(summary["files"], 0);
    assert_eq!(summary["entries"], 0);
    let nd = fs::read_to_string(&idx).unwrap();
    assert_eq!(nd.lines().count(), 1, "仅头部行");
    let header: Value = serde_json::from_str(nd.trim()).unwrap();
    assert_eq!(header["entry_count"], 0);

    // 空文件：json 空文件解析败即记入 parse_errors 不致命，md 空文件零单元。
    let corpus2 = ws.dir.join("corpus2");
    fs::create_dir_all(&corpus2).unwrap();
    fs::write(corpus2.join("broken.json"), "").unwrap();
    fs::write(corpus2.join("blank.md"), "\n\n").unwrap();
    let c2 = corpus2.to_string_lossy().into_owned();
    let idx2 = ws.dir.join("index3.ndjson");
    let ia2 = idx2.to_string_lossy().into_owned();
    let (code, out, _) = run(&["build", "--pack", &pa, "--root", &c2, "--out", &ia2]);
    assert_eq!(code, 0, "解析败不致命须 0");
    let summary: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(summary["files"], 1, "仅 blank.md 成档");
    assert_eq!(summary["entries"], 0, "空 md 零单元");
    assert_eq!(summary["parse_errors"], serde_json::json!(["broken.json"]), "空 json 如实入解析错");

    // 零命中：term 无所指退出码一，entries 如实空形。
    let (code, out, _) = run(&["query", "--index", &ia2, "--term", "不存在词"]);
    assert_eq!(code, 1, "零命中须 1");
    let rep: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(rep["entries"], serde_json::json!([]));
    assert_eq!(rep["occurrences"], serde_json::json!([]));
}
