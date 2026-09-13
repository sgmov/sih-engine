//! lease-mergeleg6-parallel 簇H T6：wikirecall 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/wikirecall（recall.py 三通道召回主链、semantic.py
//! 语义层、selftest.py 夹具仓构造）。fixture 全部 temp 自建最小夹具仓，
//! 围堰真实数学仓零触碰。金向量三例：正常（三通道并集与书单）、拒绝（用法错
//! 退出码二）、边界（零查询与词面回退逐字节一致与别名展开与空仓）。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::{json, Value};

fn bin_wikirecall() -> &'static str {
    env!("CARGO_BIN_EXE_wikirecall")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_wikirecall()).args(args).output().unwrap();
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

impl Ws {
    fn repo_str(&self) -> String {
        self.dir.to_string_lossy().into_owned()
    }
}

const FIX_A: &str = "# TOP-901 测试不动点\n\n状态：测试夹具。\n\n## 触发问题 {#triggers}\n\n- iteration 什么时候可以停下来？\n- 迭代收敛怎么验证？\n- fixed point 的条件是什么？\n- 这个算子稳不稳定？\n- 收敛判据有哪些？\n\n## 哲学桥接 {#philosophy-bridge}\n\n- 关系：见 TOP-902 与 ORD-905\n";

const FIX_B: &str = "# TOP-902 测试邻域\n\n状态：测试夹具。\n\n## 触发问题 {#triggers}\n\n- separation 的工程含义是什么？\n- 隔离怎么做才够？\n- neighborhood 半径怎么定？\n- 两态能不能判明？\n- 距离怎么选？\n\n## 哲学桥接 {#philosophy-bridge}\n\n- 关系：见 ORD-905\n";

const FIX_C: &str = "# ORD-905 测试全序\n\n状态：测试夹具。\n\n## 触发问题 {#triggers}\n\n- deadlock 怎么从根上杜绝？\n- 锁序怎么定？\n- 全序申请为什么防死锁？\n- 资源排序的规矩是什么？\n- ordering 怎么机械检查？\n";

/// temp 自建最小夹具仓（selftest.make_fixture 同构）。
fn build_repo() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    fs::create_dir_all(dir.join("topology/entries")).unwrap();
    fs::create_dir_all(dir.join("order/entries")).unwrap();
    fs::create_dir_all(dir.join("llm-friendly-build")).unwrap();
    fs::write(dir.join("topology/entries/TOP-901-test-a.md"), FIX_A).unwrap();
    fs::write(dir.join("topology/entries/TOP-902-test-b.md"), FIX_B).unwrap();
    fs::write(dir.join("order/entries/ORD-905-test-c.md"), FIX_C).unwrap();
    fs::write(dir.join("llm-friendly-build/mapping.md"), "# 测试映射\n").unwrap();
    fs::write(dir.join("topology/INDEX.md"), "| TOP-901 | 测试不动点 | 已建 |\n").unwrap();
    Ws { dir, _guard: guard }
}

// ---------- 金向量一（正常）：三通道并集出应读书单与出处与落档 ----------

#[test]
fn t1_normal_three_channels_union_reading_list() {
    let ws = build_repo();
    let repo = ws.repo_str();

    let (code, out, err) = run(&["--repo", &repo, "--query", "fixed point"]);
    assert_eq!(code, 0, "召回恒成功: {out}{err}");
    let plan: Value = serde_json::from_str(&out).unwrap();

    // 词面通道：查询子串命中触发集。
    assert_eq!(plan["channels"]["word"], json!(["TOP-901"]));
    // 语义通道（缺省 K=3）：正分命中前 K。
    assert_eq!(plan["channels"]["semantic"], json!(["TOP-901"]));
    let score = plan["channels"]["semantic_scores"]["TOP-901"].as_f64().unwrap();
    assert!(score > 0.0 && score <= 1.0, "余弦落 (0,1]: {score}");
    // 图通道：关系一跳双向扩展。
    assert_eq!(plan["channels"]["graph"], json!(["ORD-905", "TOP-902"]));
    // 并集书单：ids 排序、paths 对齐。
    assert_eq!(
        plan["reading_list"]["entry_ids"],
        json!(["ORD-905", "TOP-901", "TOP-902"])
    );
    let paths = plan["reading_list"]["entry_paths"].as_array().unwrap();
    assert_eq!(paths.len(), 3);
    assert!(paths[1].as_str().unwrap().ends_with("TOP-901-test-a.md"), "{paths:?}");
    // 出处：词面加语义并图。
    assert_eq!(plan["provenance"]["TOP-901"], json!(["word", "semantic"]));
    assert_eq!(plan["provenance"]["TOP-902"], json!(["graph"]));
    assert_eq!(plan["provenance"]["ORD-905"], json!(["graph"]));
    // 骨架：在册两件，channels 按 SKELETON 序、reading_list 按排序。
    assert_eq!(
        plan["channels"]["skeleton"],
        json!(["llm-friendly-build/mapping.md", "topology/INDEX.md"])
    );
    assert_eq!(
        plan["reading_list"]["skeleton_files"],
        json!(["llm-friendly-build/mapping.md", "topology/INDEX.md"])
    );
    assert_eq!(plan["query"], json!(["fixed point"]));
    assert_eq!(plan["expanded"], json!(["fixed point"]));

    // --out 落档与 stdout 同文（含末行换行）。
    let outp = ws.dir.join("plan.json");
    let (code2, out2, _) = run(&[
        "--repo", &repo,
        "--query", "fixed point",
        "--out", outp.to_str().unwrap(),
    ]);
    assert_eq!(code2, 0);
    let file_text = fs::read_to_string(&outp).unwrap();
    assert!(file_text.ends_with('\n'));
    assert_eq!(file_text, out2, "--out 落档须与 stdout 同文（均含末行换行）");
}

// ---------- 金向量二（拒绝）：用法错退出码二 ----------

#[test]
fn t2_reject_usage_errors_exit2() {
    let ws = build_repo();
    let repo = ws.repo_str();

    // 缺必填 --repo。
    let (code, out, err) = run(&["--query", "x"]);
    assert_eq!(code, 2, "stdout={out}");
    assert!(out.is_empty(), "拒绝面无报告落 stdout");
    assert!(!err.is_empty(), "stderr 须有用法报文");

    // 未知旗标。
    let (code, _, err) = run(&["--repo", &repo, "--query", "x", "--bogus"]);
    assert_eq!(code, 2);
    assert!(!err.is_empty());

    // --semantic 非整数。
    let (code, _, _) = run(&["--repo", &repo, "--query", "x", "--semantic", "abc"]);
    assert_eq!(code, 2);

    // --repo 缺值。
    let (code, _, _) = run(&["--repo"]);
    assert_eq!(code, 2);

    // --word 带显式值即拒。
    let (code, _, _) = run(&["--repo", &repo, "--query", "x", "--word=1"]);
    assert_eq!(code, 2);
}

// ---------- 金向量三（边界）：零查询、词面回退逐字节一致、别名展开、空仓 ----------

#[test]
fn t3_boundary_zero_query_word_fallback_aliases_empty_repo() {
    let ws = build_repo();
    let repo = ws.repo_str();

    // 零查询词面回退：书单空、骨架仍在、无 semantic 键。
    let (code, out, _) = run(&["--repo", &repo, "--word"]);
    assert_eq!(code, 0);
    let plan: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(plan["channels"]["word"], json!([]));
    assert_eq!(plan["reading_list"]["entry_ids"], json!([]));
    assert!(
        !plan["reading_list"]["skeleton_files"].as_array().unwrap().is_empty(),
        "骨架不得为空"
    );
    assert!(
        plan["channels"].get("semantic").is_none(),
        "词面回退面 channels 无 semantic 键"
    );

    // --semantic 0 与 --word 输出逐字节一致（回退位行为零改）。
    let (c0, out0, _) = run(&["--repo", &repo, "--semantic", "0"]);
    assert_eq!(c0, 0);
    assert_eq!(out0, out, "--semantic 0 须与 --word 逐字节一致");

    // 双跑逐字节一致（确定性）。
    let (_, out_again, _) = run(&["--repo", &repo, "--word"]);
    assert_eq!(out_again, out);

    // 别名展开：卡点 → TOP-901，expanded 排序去重。
    let ap = ws.dir.join("aliases.json");
    fs::write(&ap, r#"{"卡点": "TOP-901"}"#).unwrap();
    let (code, out, _) = run(&[
        "--repo", &repo,
        "--query", "卡点",
        "--aliases", ap.to_str().unwrap(),
        "--word",
    ]);
    assert_eq!(code, 0);
    let plan: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(plan["expanded"], json!(["TOP-901", "卡点"]));
    assert_eq!(plan["channels"]["word"], json!(["TOP-901"]));

    // 空仓边界：零命中零骨架仍出计划退出码零。
    let empty = ws.dir.join("no-such-repo");
    let (code, out, _) = run(&["--repo", empty.to_str().unwrap(), "--query", "x"]);
    assert_eq!(code, 0);
    let plan: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(plan["reading_list"]["entry_ids"], json!([]));
    assert_eq!(plan["channels"]["skeleton"], json!([]));
}
