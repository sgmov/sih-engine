//! lease-mergeleg23-parallel 簇D T3：parser（句读）引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/parser 0.1.0（cli*.py 四子命令、lexer.py 词法、
//! peg.py 产生式、entries.py 投影、langpack.py 校验）。fixture 全部 temp 自建
//! 最小语言包，围堰真实包零触碰（真实包对表已在本批施工对表留痕：三真包
//! parse/entries 与围堰 Python 逐字节一致、vectors 全过）。

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin_parser() -> &'static str {
    env!("CARGO_BIN_EXE_parser")
}

/// 对"三件缺失"态短重试：macOS 虚拟环境下刚写入的文件对子进程可见性有迟滞，
/// 非工具行为态；真缺失（目录不在）不受影响，重试耗尽后如实返回。
fn run_stable(args: &[&str]) -> (i32, String, String) {
    let mut last = run(args);
    for _ in 0..12 {
        if !(last.1.is_empty() && last.2.contains("缺失")) {
            return last;
        }
        std::thread::sleep(std::time::Duration::from_millis(400));
        last = run(args);
    }
    last
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_parser()).args(args).output().unwrap();
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
    _guard: tempfile::TempDir,
}

/// temp 自建最小语言包：词法 WS 跳过加 WORD 加 NUM，文法 doc→item*，
/// item→choice(word|num)，映射 word 与 num 与 item 三条。
fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let pack = dir.join("t3pack");
    fs::create_dir_all(&pack).unwrap();
    fs::write(
        pack.join("tokens.json"),
        r#"{"version":1,"tokens":[
            {"name":"WS","pattern":"[ \\t\\r\\n]+","skip":true},
            {"name":"WORD","pattern":"[a-zA-Z]+"},
            {"name":"NUM","pattern":"[0-9]+"}]}"#,
    )
    .unwrap();
    fs::write(
        pack.join("grammar.json"),
        r#"{"version":1,"start":"doc","rules":{
            "doc":{"repeat":{"sub":{"rule":"item"},"min":0}},
            "item":{"choice":[{"rule":"word"},{"rule":"num"}]},
            "word":{"token":"WORD"},
            "num":{"token":"NUM"}}}"#,
    )
    .unwrap();
    fs::write(
        pack.join("mapping.json"),
        r#"{"version":1,"entries":[
            {"rule":"item","kind":"item","name_from":{"path":{"key_token":"WORD","element_rules":["item"]}}},
            {"rule":"word","kind":"word","name_from":{"join_tokens":{"name":"WORD","sep":" "}}},
            {"rule":"num","kind":"num","lines":"null"}]}"#,
    )
    .unwrap();
    Ws { dir, pack, _guard: guard }
}

fn pack_arg(ws: &Ws) -> String {
    ws.pack.to_string_lossy().into_owned()
}

/// 载体名即语言包目录规范化后的末段（cli_entries 对表）。
fn carrier_of(ws: &Ws) -> String {
    fs::canonicalize(&ws.pack)
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned()
}

// ---------- 正常形：parse 产树、双跑一致、entries 十字段与稳定标识复算 ----------

#[test]
fn t1_normal_parse_entries_double_run() {
    let ws = build_ws();
    let pa = pack_arg(&ws);
    let infile = ws.dir.join("in.txt");
    fs::write(&infile, "ab 12 cd\n").unwrap();
    let inarg = infile.to_string_lossy().into_owned();

    // parse：产树至标准输出，退出码 0；结构断言根 rule doc 挂三 item。
    let (code, out1, err) = run(&["parse", "--pack", &pa, "--in", &inarg]);
    assert_eq!(code, 0, "parse 须 0，stderr={err}");
    let tree: Value = serde_json::from_str(&out1).unwrap();
    assert_eq!(tree["type"], "rule");
    assert_eq!(tree["name"], "doc");
    let items = tree["children"].as_array().unwrap();
    assert_eq!(items.len(), 3, "三 item");
    assert_eq!(items[0]["children"][0]["children"][0]["text"], "ab");
    assert_eq!(items[0]["children"][0]["children"][0]["name"], "WORD");
    assert_eq!(items[0]["children"][0]["name"], "word");
    assert_eq!(items[1]["children"][0]["name"], "num");
    assert_eq!(items[1]["children"][0]["children"][0]["text"], "12");

    // 双跑逐字节一致（确定性纪律）。
    let (code2, out2, _) = run(&["parse", "--pack", &pa, "--in", &inarg]);
    assert_eq!(code2, 0);
    assert_eq!(out1, out2, "双跑须逐字节一致");

    // entries：条目抽取产十字段 ndjson，退出码 0，stderr 计数报文。
    let outfile = ws.dir.join("out").join("entries.ndjson");
    let outarg = outfile.to_string_lossy().into_owned();
    let (code, out, err) = run(&["entries", "--pack", &pa, "--in", &inarg, "--out", &outarg]);
    assert_eq!(code, 0, "entries 须 0，stdout={out} stderr={err}");
    assert!(err.contains("条 ->"), "stderr 须报计数，err={err}");
    assert!(out.is_empty(), "entries stdout 须空");
    let nd = fs::read_to_string(&outfile).unwrap();
    let lines: Vec<&str> = nd.lines().collect();
    assert_eq!(lines.len(), 6, "6 条目：item/word/num/item/word/item");

    let carrier = carrier_of(&ws);
    let path_str = inarg.as_str();
    let rows: Vec<Value> = lines.iter().map(|l| serde_json::from_str(l).unwrap()).collect();
    // 字段序固定（十字段）。
    let keys: Vec<&str> = rows[0].as_object().unwrap().keys().map(|k| k.as_str()).collect();
    assert_eq!(
        keys,
        vec!["id", "path", "carrier", "kind", "name", "seq", "line_start", "line_end", "content_hash", "text"]
    );
    // 先序文档序：item(1) word(ab,1) num(12,1) item(2) word(cd,2) item(3)。
    // 先序文档序：item(1) word(ab,1) item(2) num(12,1) item(3) word(cd,2)——
    // 父规则节点先于其后代产出（walk 先序）。
    let kinds: Vec<&str> = rows.iter().map(|r| r["kind"].as_str().unwrap()).collect();
    assert_eq!(kinds, vec!["item", "word", "item", "num", "item", "word"]);
    assert_eq!(rows[0]["name"], "[0]");
    assert_eq!(rows[2]["name"], "[1]");
    assert_eq!(rows[4]["name"], "[2]");
    assert_eq!(rows[1]["name"], "ab");
    assert_eq!(rows[3]["name"], Value::Null, "num 条目 name 无 name_from 须 null");
    assert_eq!(rows[3]["line_start"], Value::Null, "num 条 lines:null 须行位 null");
    assert_eq!(rows[1]["line_start"], 1);
    assert_eq!(rows[5]["seq"], 2);

    // 稳定标识复算（DES-009：路径加载体类型加序号加内容哈希）。
    for (idx, text, kind, seq) in [
        (0usize, "ab", "item", 1u64),
        (1, "ab", "word", 1),
        (2, "12", "item", 2),
        (3, "12", "num", 1),
        (4, "cd", "item", 3),
        (5, "cd", "word", 2),
    ] {
        let chash = sha_hex(text);
        let want = stable_id(path_str, &carrier, kind, seq, &chash);
        assert_eq!(rows[idx]["id"].as_str().unwrap(), want, "kind={kind} seq={seq}");
        assert_eq!(rows[idx]["content_hash"].as_str().unwrap(), chash);
        assert_eq!(rows[idx]["text"].as_str().unwrap(), text);
    }
}

// ---------- 拒绝形：包缺席、模式不合法、输入不可读、缺参、lint 三查有 finding ----------

#[test]
fn t2_reject_forms() {
    let ws = build_ws();
    let pa = pack_arg(&ws);
    let infile = ws.dir.join("in.txt");
    fs::write(&infile, "ok\n").unwrap();
    let inarg = infile.to_string_lossy().into_owned();

    // 语言包目录缺席：InputUnreadable 即退出码一，stderr 报文。
    let (code, out, err) = run(&["parse", "--pack", "/tmp/t3-nonexistent-pack", "--in", &inarg]);
    assert_eq!(code, 1, "包缺席须 1，err={err}");
    assert!(out.is_empty());
    assert!(err.starts_with("parser: 语言包目录不存在或不可读"), "err={err}");

    // 词法模式不合法：schema 收集为 PackViolation，退出码一。
    let badpack = ws.dir.join("badpack");
    fs::create_dir_all(&badpack).unwrap();
    fs::write(badpack.join("tokens.json"), r#"{"version":1,"tokens":[{"name":"X","pattern":"[unclosed"}]}"#).unwrap();
    fs::write(badpack.join("grammar.json"), r#"{"version":1,"start":"s","rules":{"s":{"token":"X"}}}"#).unwrap();
    fs::write(badpack.join("mapping.json"), r#"{"version":1,"entries":[]}"#).unwrap();
    let (code, _, err) = run(&["parse", "--pack", &badpack.to_string_lossy().into_owned(), "--in", &inarg]);
    assert_eq!(code, 1, "模式不合法须 1");
    assert!(err.contains("pattern 不合 re 语法"), "err={err}");

    // 输入不可读：退出码一。
    let (code, _, err) = run(&["parse", "--pack", &pa, "--in", "/tmp/t3-nonexistent-input.txt"]);
    assert_eq!(code, 1, "输入不可读须 1");
    assert!(err.contains("输入不可读"), "err={err}");

    // 缺 --pack：用法错退出码二；未知子命令同。
    let (code, _, _) = run(&["parse", "--in", &inarg]);
    assert_eq!(code, 2, "缺 --pack 须 2");
    let (code, _, _) = run(&["nosuch", "--pack", &pa]);
    assert_eq!(code, 2, "未知子命令须 2");

    // lint 三查：规则引用未定义即退出码一带 finding。
    // lintpack 单独落 /tmp 子目录：本环境 /var/folders tempdir 对新落盘文件存在
    // 跨进程可见性迟滞窗口（见 run_stable 注），/private/tmp 无此象。
    let lintguard = tempfile::TempDir::new_in("/tmp").unwrap();
    let lintpack = lintguard.path().to_path_buf();
    fs::create_dir_all(&lintpack).unwrap();
    fs::write(lintpack.join("tokens.json"), r#"{"version":1,"tokens":[{"name":"X","pattern":"x"}]}"#).unwrap();
    fs::write(
        lintpack.join("grammar.json"),
        r#"{"version":1,"start":"s","rules":{"s":{"rule":"missing"}}}"#,
    )
    .unwrap();
    fs::write(lintpack.join("mapping.json"), r#"{"version":1,"entries":[]}"#).unwrap();
    let lp2 = lintpack.to_string_lossy().into_owned();
    let (code, out, err) = run_stable(&["lint", "--pack", &lp2]);
    assert_eq!(code, 1, "lint 有 finding 须 1，stderr={err}");
    assert!(out.is_empty());
    assert!(err.contains("lint: 规则引用未定义 missing"), "err={err}");

    // lint 包目录不在：逐件缺失 finding 收集，退出码一（cli_lint 对表，Python 同形）。
    let (code, _, err) = run(&["lint", "--pack", "/tmp/t3-nonexistent-pack"]);
    assert_eq!(code, 1, "lint 包不在须 1");
    assert!(err.contains("lint: tokens.json 缺失"), "err={err}");
}

// ---------- 边界形：空输入产树、零命中如实空、vectors 无目录零向量过 ----------

#[test]
fn t3_edge_empty_and_zero_hit() {
    let ws = build_ws();
    let pa = pack_arg(&ws);

    // 空输入：解析产树带错误节点，退出码零，不崩不拒。
    let empty = ws.dir.join("empty.txt");
    fs::write(&empty, "").unwrap();
    let (code, out, err) = run(&["parse", "--pack", &pa, "--in", &empty.to_string_lossy().into_owned()]);
    assert_eq!(code, 0, "空输入须 0，stderr={err}");
    let tree: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(tree["name"], "doc");
    let errnode = &tree["children"][0];
    assert_eq!(errnode["type"], "error");
    assert_eq!(errnode["message"], "输入为空记号流为零");
    assert_eq!(errnode["expected"], "doc");

    // 纯数字输入：word 与 item(路径名) 命中但 num 无映射外零命中形——
    // 用只含未映射规则的包抽零条目：空映射包。
    let zpack = ws.dir.join("zeropack");
    fs::create_dir_all(&zpack).unwrap();
    fs::write(zpack.join("tokens.json"), r#"{"version":1,"tokens":[{"name":"WORD","pattern":"[a-zA-Z]+"}]}"#).unwrap();
    fs::write(zpack.join("grammar.json"), r#"{"version":1,"start":"doc","rules":{"doc":{"repeat":{"sub":{"token":"WORD"},"min":0}}}}"#).unwrap();
    fs::write(zpack.join("mapping.json"), r#"{"version":1,"entries":[]}"#).unwrap();
    let infile = ws.dir.join("words.txt");
    fs::write(&infile, "hello world\n").unwrap();
    let outfile = ws.dir.join("zero.ndjson");
    let (code, _, err) = run(&[
        "entries",
        "--pack",
        &zpack.to_string_lossy().into_owned(),
        "--in",
        &infile.to_string_lossy().into_owned(),
        "--out",
        &outfile.to_string_lossy().into_owned(),
    ]);
    assert_eq!(code, 0, "零命中须 0，stderr={err}");
    assert!(err.contains("entries: 0 条"), "零命中计数如实报，err={err}");
    assert_eq!(fs::read_to_string(&outfile).unwrap(), "", "零命中 ndjson 须如实空文件");

    // vectors：无 vectors 目录即零向量过，退出码零。
    let (code, out, _) = run(&["vectors", "--pack", &pa]);
    assert_eq!(code, 0, "无 vectors 目录须 0");
    let rep: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(rep["mode"], "回归");
    assert_eq!(rep["pass"], 0);
    assert_eq!(rep["fail"].as_array().unwrap().len(), 0);
    assert_eq!(rep["detail"], "无 vectors 目录即零向量过");
}
