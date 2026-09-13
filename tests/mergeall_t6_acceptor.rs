//! lease-mergeleg6-parallel 簇J T6：acceptor 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/acceptor 0.2.0（engine.py 四查解释执行、三态失败定位、
//! validate_pack 词汇表封闭；cli.py 三值退出码）。fixture 全部 temp 自建最小判定包，
//! 被检命令用 /bin/sh -c 确定论形，围堰真实判定包零触碰。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn bin_acceptor() -> &'static str {
    env!("CARGO_BIN_EXE_acceptor")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_acceptor()).args(args).output().unwrap();
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

struct Ws {
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    Ws { dir, _guard: guard }
}

/// 写判定包（json! 建 Value 后序列化，路径占位逐字段预展开）。
fn write_pack(ws: &Ws, name: &str, pack: &Value) -> String {
    let p = ws.dir.join(name);
    fs::write(&p, serde_json::to_string_pretty(pack).unwrap()).unwrap();
    p.to_string_lossy().into_owned()
}

fn root_arg(ws: &Ws) -> String {
    ws.dir.to_string_lossy().into_owned()
}

// ---------- 正常形：双跑与红转绿与冻结比对全过、双 token 占位展开 ----------

#[test]
fn t1_normal_all_pass() {
    let ws = build_ws();
    // 红证件与冻结期望件与 {ROOT} 载体文件
    fs::write(ws.dir.join("red.txt"), "红证件内容\n").unwrap();
    let red_hash = sha256_hex("红证件内容\n".as_bytes());
    fs::write(ws.dir.join("hello.txt"), "hello\n").unwrap();
    fs::write(ws.dir.join("frozen.bin"), "hello\n").unwrap();

    let pack = json!({
        "pack": "t6-normal",
        "version": "0.1.0",
        "doc_class": "TDD 判定包",
        "checks": [
            {"id": "TC-001", "check_name": "double_run",
             "command": ["/bin/sh", "-c", "printf 'hi\\n'"],
             "hint": "同参双跑逐字节一致"},
            {"id": "TC-002", "check_name": "red_then_green",
             "command": ["/bin/sh", "-c", "exit 0"],
             "red_evidence": "{PACK_DIR}/red.txt", "red_hash": red_hash,
             "expect_exit": 0},
            {"id": "TC-003", "check_name": "baseline_freeze",
             "command": ["/bin/sh", "-c", "cat {ROOT}/hello.txt"],
             "frozen": "{PACK_DIR}/frozen.bin"}
        ],
        "declaration": {"claim": "unchanged-only"}
    });
    let pa = write_pack(&ws, "pack.json", &pack);

    let (code, out, err) = run(&["--pack", &pa, "--root", &root_arg(&ws)]);
    assert_eq!(code, 0, "全过须 0，stderr={err}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["verdict"], "pass");
    assert_eq!(rep["engine"], "acceptor");
    // 报告面版本戳按围堰 engine.py 硬编码 0.1.0 对表（头注落差一）
    assert_eq!(rep["engine_version"], "0.1.0");
    assert_eq!(rep["pack"], "t6-normal");
    assert_eq!(rep["pack_version"], "0.1.0");
    assert_eq!(rep["declaration"]["claim"], "unchanged-only");
    let results = rep["results"].as_array().unwrap();
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r["state"] == "pass"));
    assert_eq!(rep["findings"].as_array().unwrap().len(), 0);

    // stdout 缩进形对表 Python json.dumps(indent=1, sort_keys=True)：一级一空格
    let first_line = out.lines().next().unwrap();
    assert_eq!(first_line, "{");
    assert!(out.contains("\n \"declaration\": {"), "缩进一形，got 首行后={}", &out[..120.min(out.len())]);

    // 缺 --root：{ROOT} 缺省调用 cwd——用相对路径包与 cwd 建件重验一例
    let cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&ws.dir).unwrap();
    let (code, out, _) = run(&["--pack", &pa]);
    std::env::set_current_dir(cwd).unwrap();
    assert_eq!(code, 0, "缺 --root 须缺省 cwd 展开，err={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["verdict"], "pass");
}

// ---------- 拒绝形：包缺席、包不可解析、词汇表外、claim 非法、命令不可执行 ----------

#[test]
fn t2_reject_forms() {
    let ws = build_ws();
    let ra = root_arg(&ws);

    // 包缺席：退出码 2，stderr 判定包缺席
    let (code, out, err) = run(&["--pack", "/tmp/t6-no-such-pack.json", "--root", &ra]);
    assert_eq!(code, 2);
    assert!(out.is_empty());
    assert!(err.contains("判定包缺席"), "err={err}");

    // 包 JSON 不可解析：退出码 2
    let bad = ws.dir.join("bad.json");
    fs::write(&bad, "{not json").unwrap();
    let (code, _, err) = run(&["--pack", &bad.to_string_lossy(), "--root", &ra]);
    assert_eq!(code, 2);
    assert!(err.contains("判定包 JSON 不可解析"), "err={err}");

    // 词汇表外操作：退出码 2，报词汇表外
    let pack = json!({
        "pack": "t6-bad", "version": "0.1.0", "doc_class": "TDD 判定包",
        "checks": [{"id": "TC-901", "check_name": "no_such_op", "command": ["/bin/true"]}],
        "declaration": {"claim": "unchanged-only"}
    });
    let pa = write_pack(&ws, "badvocab.json", &pack);
    let (code, _, err) = run(&["--pack", &pa, "--root", &ra]);
    assert_eq!(code, 2);
    assert!(err.contains("词汇表外操作"), "err={err}");
    assert!(err.contains("no_such_op"), "err 须报操作名，err={err}");

    // 包缺键：退出码 2
    let pack = json!({
        "pack": "t6-miss", "doc_class": "TDD 判定包",
        "checks": [], "declaration": {"claim": "unchanged-only"}
    });
    let pa = write_pack(&ws, "misskey.json", &pack);
    let (code, _, err) = run(&["--pack", &pa, "--root", &ra]);
    assert_eq!(code, 2);
    assert!(err.contains("包缺键"), "err={err}");

    // claim 非法（甲乙红线）：退出码 2
    let pack = json!({
        "pack": "t6-claim", "version": "0.1.0", "doc_class": "TDD 判定包",
        "checks": [],
        "declaration": {"claim": "changed-too"}
    });
    let pa = write_pack(&ws, "claim.json", &pack);
    let (code, _, err) = run(&["--pack", &pa, "--root", &ra]);
    assert_eq!(code, 2);
    assert!(err.contains("声明范围 claim 非法"), "err={err}");

    // 命令不可执行：退出码 2，执行环境异常
    let pack = json!({
        "pack": "t6-exec", "version": "0.1.0", "doc_class": "TDD 判定包",
        "checks": [{"id": "TC-902", "check_name": "double_run",
                    "command": ["/bin/t6-definitely-not-a-real-bin"]}],
        "declaration": {"claim": "unchanged-only"}
    });
    let pa = write_pack(&ws, "exec.json", &pack);
    let (code, _, err) = run(&["--pack", &pa, "--root", &ra]);
    assert_eq!(code, 2);
    assert!(err.contains("执行环境异常"), "err={err}");

    // 缺 --pack：退出码 2
    let (code, _, _) = run(&["--root", &ra]);
    assert_eq!(code, 2, "缺 --pack 须 2");
}

// ---------- 边界形：三态失败定位（violation 双跑/漂移/覆盖差集与 missing 三形） ----------

#[test]
fn t3_edge_three_state_findings() {
    let ws = build_ws();
    let ra = root_arg(&ws);

    // violation：双跑不一致（$$ 即进程号，两跑必异）
    let pack = json!({
        "pack": "t6-drift", "version": "0.1.0", "doc_class": "TDD 判定包",
        "checks": [{"id": "TC-101", "check_name": "double_run",
                    "command": ["/bin/sh", "-c", "echo $$"]}],
        "declaration": {"claim": "unchanged-only"}
    });
    let pa = write_pack(&ws, "drift.json", &pack);
    let (code, out, _) = run(&["--pack", &pa, "--root", &ra]);
    assert_eq!(code, 1, "有查失败须 1");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["verdict"], "fail");
    let f = &rep["findings"][0];
    assert_eq!(f["state"], "violation");
    assert_eq!(f["id"], "TC-101");
    assert!(f["detail"].as_str().unwrap().contains("双跑不一致"), "{f}");

    // violation：冻结漂移报原始事实不归因（首差异偏移 0、字节摘要）
    fs::write(ws.dir.join("frozen.bin"), "expect\n").unwrap();
    let pack = json!({
        "pack": "t6-freeze", "version": "0.1.0", "doc_class": "TDD 判定包",
        "checks": [{"id": "TC-102", "check_name": "baseline_freeze",
                    "command": ["/bin/sh", "-c", "printf 'drifted\\n'"],
                    "frozen": "{PACK_DIR}/frozen.bin"}],
        "declaration": {"claim": "unchanged-only"}
    });
    let pa = write_pack(&ws, "freeze.json", &pack);
    let (code, out, _) = run(&["--pack", &pa, "--root", &ra]);
    assert_eq!(code, 1);
    let rep: Value = serde_json::from_str(&out).unwrap();
    let f = &rep["findings"][0];
    assert_eq!(f["state"], "violation");
    let detail = f["detail"].as_str().unwrap();
    assert!(detail.contains("漂移原始事实"), "{detail}");
    assert!(detail.contains("首差异偏移 0"), "{detail}");
    assert!(detail.contains("输出 8 字节对期望 7 字节"), "{detail}");
    assert!(detail.contains("v0 不归因"), "{detail}");

    // missing：冻结期望件缺席与红证件缺席
    let pack = json!({
        "pack": "t6-miss", "version": "0.1.0", "doc_class": "TDD 判定包",
        "checks": [
            {"id": "TC-103", "check_name": "baseline_freeze",
             "command": ["/bin/sh", "-c", "true"], "frozen": "{PACK_DIR}/absent.bin"},
            {"id": "TC-104", "check_name": "red_then_green",
             "command": ["/bin/sh", "-c", "exit 0"],
             "red_evidence": "{PACK_DIR}/absent-red.txt", "red_hash": "00"}
        ],
        "declaration": {"claim": "unchanged-only"}
    });
    let pa = write_pack(&ws, "missing.json", &pack);
    let (code, out, _) = run(&["--pack", &pa, "--root", &ra]);
    assert_eq!(code, 1);
    let rep: Value = serde_json::from_str(&out).unwrap();
    let fs_ = rep["findings"].as_array().unwrap();
    assert_eq!(fs_.len(), 2);
    assert_eq!(fs_[0]["state"], "missing");
    assert!(fs_[0]["detail"].as_str().unwrap().contains("冻结期望件缺席"));
    assert_eq!(fs_[1]["state"], "missing");
    assert!(fs_[1]["detail"]
        .as_str()
        .unwrap()
        .contains("红证件缺席或哈希不符即缺件态：红证件缺席"));

    // violation：场景覆盖差集非零（beta 得 2 期望 0）；修后全过
    fs::write(
        ws.dir.join("scenarios.md"),
        "# 场景\n\n#### S1: SDD-001\n- 判据: prog=alpha expect=0\n#### S2: SDD-002\n- 判据: prog=beta expect=0\n",
    )
    .unwrap();
    let sc = ws.dir.join("scenarios.md").to_string_lossy().into_owned();
    let pack = json!({
        "pack": "t6-scen", "version": "0.1.0", "doc_class": "TDD 判定包",
        "checks": [{"id": "TC-105", "check_name": "scenario_coverage",
                    "scenarios": sc, "line_prefix": "- 判据:",
                    "program_field_regex": "prog=(\\S+)",
                    "expect_regex": "expect=(\\d+)",
                    "judge_map": {
                        "alpha": ["/bin/sh", "-c", "exit 0"],
                        "beta": ["/bin/sh", "-c", "exit 2"]
                    }}],
        "declaration": {"claim": "unchanged-only"}
    });
    let pa = write_pack(&ws, "scen.json", &pack);
    let (code, out, _) = run(&["--pack", &pa, "--root", &ra]);
    assert_eq!(code, 1);
    let rep: Value = serde_json::from_str(&out).unwrap();
    let f = &rep["findings"][0];
    assert_eq!(f["state"], "violation");
    let detail = f["detail"].as_str().unwrap();
    assert!(detail.starts_with("覆盖差集非零："), "{detail}");
    assert!(detail.contains("SDD-002（得 2 期望 0）"), "{detail}");

    // 修 beta 判据为绿：覆盖差集归零全过
    let pack2 = json!({
        "pack": "t6-scen", "version": "0.1.0", "doc_class": "TDD 判定包",
        "checks": [{"id": "TC-105", "check_name": "scenario_coverage",
                    "scenarios": sc, "line_prefix": "- 判据:",
                    "program_field_regex": "prog=(\\S+)",
                    "expect_regex": "expect=(\\d+)",
                    "judge_map": {
                        "alpha": ["/bin/sh", "-c", "exit 0"],
                        "beta": ["/bin/sh", "-c", "exit 0"]
                    }}],
        "declaration": {"claim": "unchanged-only"}
    });
    let pa2 = write_pack(&ws, "scen2.json", &pack2);
    let (code, out, _) = run(&["--pack", &pa2, "--root", &ra]);
    assert_eq!(code, 0, "覆盖差集零须 0");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["verdict"], "pass");

    // missing：场景清单无编号条目
    fs::write(ws.dir.join("empty-scen.md"), "零编号条目\n").unwrap();
    let sc2 = ws.dir.join("empty-scen.md").to_string_lossy().into_owned();
    let pack3 = json!({
        "pack": "t6-scen2", "version": "0.1.0", "doc_class": "TDD 判定包",
        "checks": [{"id": "TC-106", "check_name": "scenario_coverage",
                    "scenarios": sc2, "line_prefix": "- 判据:",
                    "program_field_regex": "prog=(\\S+)",
                    "expect_regex": "expect=(\\d+)",
                    "judge_map": {"alpha": ["/bin/sh", "-c", "exit 0"]}}],
        "declaration": {"claim": "unchanged-only"}
    });
    let pa3 = write_pack(&ws, "scen3.json", &pack3);
    let (code, out, _) = run(&["--pack", &pa3, "--root", &ra]);
    assert_eq!(code, 1);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["findings"][0]["state"], "missing");
    assert!(rep["findings"][0]["detail"]
        .as_str()
        .unwrap()
        .contains("场景清单无编号条目"));
}
