//! lease-mergeleg6-parallel 簇J T6：elicit 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/elicit 0.2.0（cli.py：check 三扳机信号生成、digest 消化
//! 闸孤悬判定、suspend 挂起记录；CONTRACT.md 修订一 --quiet 紧凑形）。fixture 全部
//! temp 自建最小面件，围堰真实登记面零触碰。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin_elicit() -> &'static str {
    env!("CARGO_BIN_EXE_elicit")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_elicit()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

struct Ws {
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    Ws { dir, _guard: guard }
}

/// 登记面：terms.json 两词（词甲/词乙三形在册），词丙不在册。
fn make_packs(ws: &Ws) -> String {
    let packs = ws.dir.join("packs");
    fs::create_dir_all(&packs).unwrap();
    fs::write(
        packs.join("terms.json"),
        r#"{"terms": [{"zh": "词甲", "en": "word-a", "code": "word_a"}, {"zh": "词乙", "en": "word-b", "code": "word_b"}]}"#,
    )
    .unwrap();
    packs.to_string_lossy().into_owned()
}

fn p(ws: &Ws, name: &str) -> String {
    ws.dir.join(name).to_string_lossy().into_owned()
}

// ---------- 正常形：check 出信号件、digest 全处置过闸、suspend 落记录 ----------

#[test]
fn t1_normal_check_digest_suspend() {
    let ws = build_ws();
    let packs = make_packs(&ws);
    let sig = p(&ws, "signals.ndjson");

    // check：词甲在册零信号、词丙未登记出轻信号，退出码 1
    let (code, out, _) = run(&[
        "check", "--packs", &packs, "--words", "词甲", "--words", "词丙",
        "--out", &sig, "--at", "2026-09-13",
    ]);
    assert_eq!(code, 1, "有信号须 1，stdout={out}");
    assert!(out.is_empty(), "有 --out 时 stdout 静默，got={out}");
    let text = fs::read_to_string(&sig).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 1, "只在册外一词一条信号，got={text}");
    let s: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(s["signal_type"], "unregistered");
    assert_eq!(s["subject"], "词丙");
    assert_eq!(s["weight"], "轻");
    assert!(s["source_ref"].as_str().unwrap().ends_with("terms.json"));
    assert_eq!(s["at"], "2026-09-13");
    // 五字段紧凑形（separators=(",", ":")）逐字对表
    let terms_disp = format!("{packs}/terms.json");
    assert_eq!(
        lines[0],
        format!(
            "{{\"signal_type\":\"unregistered\",\"subject\":\"词丙\",\"weight\":\"轻\",\"source_ref\":\"{terms_disp}\",\"at\":\"2026-09-13\"}}",
        ),
        "紧凑形逐字对表，got={}",
        lines[0]
    );

    // digest：契约含处置标记 → passed，空图全图可达
    let contract = p(&ws, "contract.md");
    fs::write(&contract, "## 处置\n\n叩问处置[词丙] 已升级人工。\n").unwrap();
    let (code, out, _) = run(&["digest", "--signals", &sig, "--contract", &contract]);
    assert_eq!(code, 0, "全处置须 0");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["digest"], "passed");
    assert_eq!(rep["covered"], 1);

    // suspend：落挂起记录，退出码 0
    let log = p(&ws, "suspend.ndjson");
    let (code, out, _) = run(&["suspend", "--signals", &sig, "--log", &log, "--at", "2026-09-13"]);
    assert_eq!(code, 0);
    assert!(out.contains("\"suspend\": \"recorded\""), "缺省形报文，got={out}");
    let rec: Value = serde_json::from_str(&fs::read_to_string(&log).unwrap().trim()).unwrap();
    assert_eq!(rec["state"], "suspended");
    assert_eq!(rec["signals"], 1);
    assert_eq!(rec["subjects"], serde_json::json!(["词丙"]));
    assert_eq!(rec["logged_at"], "2026-09-13");

    // --quiet 紧凑形：check 单行摘要，完整信号仍写 --out
    let sig2 = p(&ws, "signals2.ndjson");
    let (code, out, _) = run(&[
        "check", "--packs", &packs, "--words", "词乙", "--words", "词丙",
        "--out", &sig2, "--quiet",
    ]);
    assert_eq!(code, 1);
    let line: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(line["tool"], "elicit");
    assert_eq!(line["command"], "check");
    assert_eq!(line["code"], 1);
    assert_eq!(line["summary"]["signals"], 1);
    assert_eq!(line["summary"]["out"], sig2);
    assert!(!out.trim().contains('\n'), "quiet 须单行，got={out}");
    assert_eq!(fs::read_to_string(&sig2).unwrap().lines().count(), 1);
}

// ---------- 拒绝形：登记面缺席、信号件缺席、契约缺席、孤悬 blocked 拦截 ----------

#[test]
fn t2_reject_forms() {
    let ws = build_ws();
    let empty_dir = p(&ws, "no-terms-dir");
    fs::create_dir_all(&empty_dir).unwrap();

    // check 登记面缺席：退出码 2，stdout 报目标不存在
    let (code, out, _) = run(&["check", "--packs", &empty_dir, "--words", "词甲"]);
    assert_eq!(code, 2);
    assert!(out.contains("目标不存在"), "out={out}");
    assert!(out.contains("terms.json"));

    // digest 信号件缺席：退出码 2
    let (code, out, _) = run(&["digest", "--signals", &p(&ws, "nope.ndjson"), "--contract", "x.md"]);
    assert_eq!(code, 2);
    assert!(out.contains("目标不存在"), "out={out}");

    // digest 契约缺席：退出码 2
    let sig = p(&ws, "sig.ndjson");
    fs::write(&sig, "{\"subject\": \"词丙\"}\n").unwrap();
    let (code, out, _) = run(&["digest", "--signals", &sig, "--contract", &p(&ws, "nope.md")]);
    assert_eq!(code, 2);
    assert!(out.contains("目标不存在"), "out={out}");

    // digest 孤悬 blocked：缺处置标记，退出码 1，missing 逐项
    let contract = p(&ws, "contract.md");
    fs::write(&contract, "无任何处置标记的契约\n").unwrap();
    let (code, out, _) = run(&["digest", "--signals", &sig, "--contract", &contract]);
    assert_eq!(code, 1, "孤悬在场须 1");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["digest"], "blocked");
    assert_eq!(rep["missing"], serde_json::json!(["词丙"]));

    // suspend 信号件缺席：退出码 2
    let (code, out, _) = run(&["suspend", "--signals", &p(&ws, "nope.ndjson"), "--log", "l.ndjson"]);
    assert_eq!(code, 2);
    assert!(out.contains("目标不存在"), "out={out}");

    // 缺必填旗标：check 缺 --packs 退出码 2
    let (code, _, _) = run(&["check", "--words", "词甲"]);
    assert_eq!(code, 2, "check 缺 --packs 须 2");
    let (code, _, _) = run(&["digest", "--signals", &sig]);
    assert_eq!(code, 2, "digest 缺 --contract 须 2");
    let (code, _, _) = run(&["frobnicate"]);
    assert_eq!(code, 2, "未知子命令须 2");
}

// ---------- 边界形：零信号空图平凡可达、矛盾候选重信号与 @index 寻址、排序 ----------

#[test]
fn t3_edge_zero_signals_conflicts_order() {
    let ws = build_ws();
    let packs = make_packs(&ws);

    // 零信号：全在册，退出码 0，信号件空（0 字节）
    let sig = p(&ws, "zero.ndjson");
    let (code, out, _) = run(&[
        "check", "--packs", &packs, "--words", "词甲", "--words", "词乙",
        "--out", &sig,
    ]);
    assert_eq!(code, 0, "零信号须 0");
    assert!(out.is_empty());
    assert_eq!(fs::read(&sig).unwrap().len(), 0, "零信号信号件须空");

    // 空图平凡可达：零信号 digest 直过，covered 0
    let contract = p(&ws, "contract.md");
    fs::write(&contract, "任意契约\n").unwrap();
    let (code, out, _) = run(&["digest", "--signals", &sig, "--contract", &contract, "--quiet"]);
    assert_eq!(code, 0, "空图平凡可达须 0");
    let line: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(line["summary"]["digest"], "passed");
    assert_eq!(line["summary"]["covered"], 0);

    // 矛盾候选：recall_conflict 重信号 @1 寻址 + ruling_conflict 对历史面
    let recall = p(&ws, "recall.ndjson");
    fs::write(&recall, "{\"matched\": \"甲\"}\n{\"matched\": \"乙\"}\n").unwrap();
    let history = p(&ws, "history.ndjson");
    fs::write(&history, "{\"note\": \"裁定甲归乙\"}\n").unwrap();
    let sig2 = p(&ws, "conflict.ndjson");
    let (code, out, _) = run(&[
        "check", "--packs", &packs,
        "--words", "词外",
        "--conflict-words", "甲",
        "--recall-face", &recall,
        "--history-face", &history,
        "--out", &sig2,
    ]);
    assert_eq!(code, 1, "有信号须 1，stdout={out}");
    let text = fs::read_to_string(&sig2).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 3, "两矛盾候选加一未登记，got={text}");
    let parsed: Vec<Value> = lines.iter().map(|l| serde_json::from_str(l).unwrap()).collect();
    // 排序对表：(signal_type, subject) 字典序：recall_conflict < ruling_conflict < unregistered
    assert_eq!(parsed[0]["signal_type"], "recall_conflict");
    assert_eq!(parsed[0]["subject"], "甲");
    assert_eq!(parsed[0]["weight"], "重");
    assert_eq!(
        parsed[0]["source_ref"],
        format!("{recall}@1"),
        "@index 寻址对表首行命中"
    );
    assert_eq!(parsed[1]["signal_type"], "ruling_conflict");
    assert_eq!(parsed[1]["weight"], "重");
    assert_eq!(parsed[1]["source_ref"], format!("{history}@1"));
    assert_eq!(parsed[2]["signal_type"], "unregistered");
    assert_eq!(parsed[2]["subject"], "词外");

    // recall_miss：主题对召回面零命中出轻信号
    let sig3 = p(&ws, "miss.ndjson");
    let (code, _, _) = run(&[
        "check", "--packs", &packs, "--topics", "无此主题",
        "--recall-face", &recall, "--out", &sig3,
    ]);
    assert_eq!(code, 1);
    let s: Value = serde_json::from_str(&fs::read_to_string(&sig3).unwrap().trim()).unwrap();
    assert_eq!(s["signal_type"], "recall_miss");
    assert_eq!(s["weight"], "轻");
}
