//! gap-askroute-port T6：askroute 引擎位移植金向量与双跑对表。
//!
//! 围堰 askroute 为纯数据包形（sih-tools/askroute：CONTRACT.md + packs/intents-v0.json
//! + tests/test_pack.py drift 守卫，无可执行件），故对表形按 SPEC-024 同参双跑条款
//! 的包位变体执行：同一引擎 bin 分别对引擎位包（src/askroute/packs/intents-v0.json）
//! 与围堰包（sih-tools/askroute/packs/intents-v0.json）同参双跑，判出的意图 id 与
//! chain 逐项一致，输出逐字节一致（报文不含包路径，包位差异不落报文）。
//!
//! - T0：引擎位包与围堰包逐字节随迁对表（围堰不在位即跳过跨对表，随迁件自校验
//!   由 T1/T2 承载）。
//! - T1：全缩写表双跑对表 + 金向量（围堰现行文意图 id 与 chain 逐项）。
//! - T2：unknown_action 出口行为对表（matched=false、id、三步序，退出码 1）。
//! - T3：check 守卫双跑对表 + 篡改包红形 + 退出码三值全表（沙箱 temp 自建，
//!   真实治理账本与链零触碰）。

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

fn bin_askroute() -> &'static str {
    env!("CARGO_BIN_EXE_askroute")
}

fn engine_pack() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/askroute/packs/intents-v0.json")
}

/// 围堰原包：只读对表基准（围堰全目录冻结零改动）。
fn worktree_pack() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("sih-tools/askroute/packs/intents-v0.json")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_askroute()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn pack_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

// ---------- T0 包数据逐字节随迁（同参双跑构成条件） ----------

#[test]
fn t0_migrated_pack_byte_identical() {
    let worktree = worktree_pack();
    if !worktree.exists() {
        // 围堰不在位的机器（纯引擎 CI）跳过跨对表
        return;
    }
    assert_eq!(
        fs::read(&worktree).unwrap(),
        fs::read(engine_pack()).unwrap(),
        "引擎位包与围堰原包须逐字节一致"
    );
}

// ---------- T1 全缩写表同参双跑 + 金向量 ----------

#[test]
fn t1_dualrun_all_abbreviations_golden() {
    let engine = engine_pack();
    let worktree = worktree_pack();
    let pack = pack_json(&engine);

    // 金向量取自围堰现行文（intents-v0.json），非引擎生成
    let golden: Vec<(String, &str, Value)> = vec![
        (
            "体检".into(),
            "coldstart-check",
            serde_json::json!(["locks_read", "critsweep", "heartbeat", "chain_verify 今日"]),
        ),
        ("冷启动".into(), "coldstart-check", serde_json::json!(["locks_read", "critsweep", "heartbeat", "chain_verify 今日"])),
        ("查册".into(), "naming", serde_json::json!(["naming_guide", "nomenclator_query <候选词>", "nomenclator map --concept <概念词>"])),
        ("温故".into(), "recall", serde_json::json!(["retriever_recall topic=<主题> 或 word=<词>"])),
        ("泊一下".into(), "park", serde_json::json!(["record_park"])),
        ("排队".into(), "lock-wait", serde_json::json!(["lease_wait_turn"])),
        ("开租约".into(), "write-chain", serde_json::json!(["lease_open", "lease_lock", "record_intent", "record_append", "lease_commit", "lease_unlock", "lease_close"])),
    ];
    for (abbr, intent, chain) in &golden {
        let (code, out, err) = run(&["route", abbr, "--pack", engine.to_str().unwrap()]);
        assert_eq!(code, 0, "route {abbr} 须命中，out={out} err={err}");
        let parsed: Value = serde_json::from_str(out.trim()).unwrap();
        assert_eq!(parsed["matched"], true);
        assert_eq!(&parsed["intent"], intent, "意图 id 对表：{abbr}");
        assert_eq!(&parsed["chain"], chain, "chain 逐项对表：{abbr}");
    }

    // 全缩写表逐条双跑：引擎位包 vs 围堰包，同参逐字节一致
    let intents = pack["intents"].as_array().unwrap();
    for intent in intents {
        let id = intent["id"].as_str().unwrap();
        let chain = &intent["chain"];
        for abbr in intent["abbreviations"].as_array().unwrap() {
            let abbr = abbr.as_str().unwrap();
            let (code_a, out_a, _) = run(&["route", abbr, "--pack", engine.to_str().unwrap()]);
            let (code_b, out_b, _) = run(&["route", abbr, "--pack", worktree.to_str().unwrap()]);
            assert_eq!(code_a, code_b, "双跑退出码一致：{abbr}");
            assert_eq!(out_a, out_b, "双跑报文逐字节一致：{abbr}");
            let parsed: Value = serde_json::from_str(out_a.trim()).unwrap();
            assert_eq!(parsed["matched"], true, "{abbr} 在册须命中");
            assert_eq!(parsed["intent"], id, "双跑意图 id 对表：{abbr}");
            assert_eq!(&parsed["chain"], chain, "双跑 chain 逐项对表：{abbr}");
        }
    }
}

// ---------- T2 unknown_action 出口行为对表 ----------

#[test]
fn t2_unknown_exit_behavior() {
    let engine = engine_pack();
    let worktree = worktree_pack();

    for pack_path in [&engine, &worktree] {
        if !pack_path.exists() {
            continue;
        }
        let (code, out, err) = run(&["route", "不在册的缩写", "--pack", pack_path.to_str().unwrap()]);
        assert_eq!(code, 1, "表外缩写出口须 1，out={out} err={err}");
        let parsed: Value = serde_json::from_str(out.trim()).unwrap();
        assert_eq!(parsed["matched"], false);
        assert_eq!(parsed["intent"], "askroute-unknown");
        // 三步序即纪律：查册 → 语料召回 → 问人，与围堰包 unknown_action 逐项一致
        let expected = &pack_json(pack_path)["unknown_action"]["steps"];
        assert_eq!(&parsed["steps"], expected, "steps 与包数据逐项一致");
        assert_eq!(expected.as_array().unwrap().len(), 3);
        assert!(expected[0].as_str().unwrap().starts_with("nomenclator_query"));
        assert!(expected[1].as_str().unwrap().starts_with("retriever_recall"));
        assert!(expected[2].as_str().unwrap().starts_with("ask_human"));
    }

    // 双包位同参双跑逐字节一致
    if worktree.exists() {
        let (_, out_a, _) = run(&["route", "不在册的缩写", "--pack", engine.to_str().unwrap()]);
        let (_, out_b, _) = run(&["route", "不在册的缩写", "--pack", worktree.to_str().unwrap()]);
        assert_eq!(out_a, out_b, "unknown 出口双跑报文逐字节一致");
    }
}

// ---------- T3 check 守卫对表 + 红形 + 退出码全表 ----------

#[test]
fn t3_check_guard_dualrun_and_exit_table() {
    let engine = engine_pack();
    let worktree = worktree_pack();

    // 守卫全绿：退出码 0，且双包位同参双跑逐字节一致
    let (code_a, out_a, err_a) = run(&["check", "--pack", engine.to_str().unwrap()]);
    assert_eq!(code_a, 0, "引擎位包守卫须绿，out={out_a} err={err_a}");
    let green: Value = serde_json::from_str(out_a.trim()).unwrap();
    assert_eq!(green["check"], "ok");
    assert_eq!(green["intents"], 7);
    assert_eq!(green["abbreviations"], 14);
    if worktree.exists() {
        let (code_b, out_b, _) = run(&["check", "--pack", worktree.to_str().unwrap()]);
        assert_eq!(code_b, 0);
        assert_eq!(out_a, out_b, "check 双跑报文逐字节一致");
    }

    // 篡改红形（沙箱 temp，围堰与引擎树零触碰）：
    // 删 unknown_action → 载入层红（退出码 2，CONTRACT：本键不可删除）
    let guard = tempfile::TempDir::new().unwrap();
    let tampered = guard.path().join("tampered.json");
    let mut pack = pack_json(&engine);
    pack.as_object_mut().unwrap().remove("unknown_action");
    fs::write(&tampered, serde_json::to_string(&pack).unwrap()).unwrap();
    let (code, out, _) = run(&["check", "--pack", tampered.to_str().unwrap()]);
    assert_eq!(code, 2, "删 unknown_action 须载入层红，out={out}");
    assert!(out.contains("unknown_action"), "out={out}");

    // 缩写跨意图冲突 → 守卫层红（退出码 1，判词逐项）
    let mut conflicted = pack_json(&engine);
    let borrowed_abbr = conflicted["intents"][1]["abbreviations"][0].clone();
    conflicted["intents"][0]["abbreviations"]
        .as_array_mut()
        .unwrap()
        .push(borrowed_abbr);
    fs::write(&tampered, serde_json::to_string(&conflicted).unwrap()).unwrap();
    let (code, out, _) = run(&["check", "--pack", tampered.to_str().unwrap()]);
    assert_eq!(code, 1, "跨意图冲突须守卫层红，out={out}");
    let parsed: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(parsed["check"], "violations");
    let findings: Vec<&str> = parsed["findings"].as_array().unwrap().iter().map(|f| f.as_str().unwrap()).collect();
    assert!(findings.iter().any(|f| f.contains("缩写跨意图冲突")), "findings={findings:?}");

    // 包 JSON 破 → 退出码 2
    fs::write(&tampered, "not json").unwrap();
    let (code, out, _) = run(&["route", "体检", "--pack", tampered.to_str().unwrap()]);
    assert_eq!(code, 2, "包破 route 须 2，out={out}");
    let (code, _, _) = run(&["check", "--pack", tampered.to_str().unwrap()]);
    assert_eq!(code, 2, "包破 check 须 2");

    // 包文件缺席 → 退出码 2
    let (code, out, _) = run(&["route", "体检", "--pack", guard.path().join("nope.json").to_str().unwrap()]);
    assert_eq!(code, 2, "包缺席须 2，out={out}");
    assert!(out.contains("包不可读"), "out={out}");

    // 用法错全表 → 退出码 2
    let (code, _, _) = run(&[]);
    assert_eq!(code, 2, "零参数须 2");
    let (code, _, _) = run(&["route"]);
    assert_eq!(code, 2, "route 缺缩写须 2");
    let (code, _, _) = run(&["route", "体检", "多一参"]);
    assert_eq!(code, 2, "route 多位置参数须 2");
    let (code, _, _) = run(&["frobnicate"]);
    assert_eq!(code, 2, "未知子命令须 2");
    let (code, _, _) = run(&["route", "体检", "--bogus"]);
    assert_eq!(code, 2, "未知旗标须 2");
    let (code, _, _) = run(&["route", "体检", "--pack"]);
    assert_eq!(code, 2, "--pack 缺值须 2");
    let (code, _, _) = run(&["check", "多一参"]);
    assert_eq!(code, 2, "check 收位置参数须 2");

    // 缺省包（内嵌引擎位）与 --pack 引擎位包同参逐字节一致
    let (code_default, out_default, _) = run(&["route", "体检"]);
    assert_eq!(code_default, 0);
    let (code_flag, out_flag, _) = run(&["route", "体检", "--pack", engine.to_str().unwrap()]);
    assert_eq!(code_flag, 0);
    assert_eq!(out_default, out_flag, "内嵌包与引擎位包文件同参逐字节一致");
}
