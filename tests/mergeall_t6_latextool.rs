//! lease-mergeleg6-parallel 簇G T6：latextool 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/latex-helper 0.1.0（validate / autofix / block-create /
//! suggest / knowledge 五子命令）。compute 未移植（latextool.rs 头注落差申报一），
//! 本件对其申报错误面。fixture 全部 temp 自建，金向量三形：正常形 / 拒绝形 / 边界形。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin_latextool() -> &'static str {
    env!("CARGO_BIN_EXE_latextool")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_latextool()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn run_json(args: &[&str]) -> (i32, Value) {
    let (code, stdout, stderr) = run(args);
    assert!(
        stderr.is_empty() || code == 2,
        "unexpected stderr on exit {code}: {stderr}"
    );
    let v: Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("stdout not json (exit {code}): {e}\n---\n{stdout}")
    });
    (code, v)
}

struct Fx {
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    Fx { dir, _guard: guard }
}

// ============ 正常形：validate / block-create / suggest / knowledge 主道 ============

#[test]
fn t01_validate_clean_text_exit0() {
    let (code, v) = run_json(&["validate", "--text", r"\frac{1}{2}"]);
    assert_eq!(code, 0);
    assert_eq!(v["summary"]["errors"].as_i64().unwrap(), 0);
    assert_eq!(v["summary"]["warnings"].as_i64().unwrap(), 0);
    assert!(v.get("file").is_none(), "--text 路径出参不带 file 键");
}

#[test]
fn t02_validate_file_clean_exit0() {
    let fx = build_fx();
    let f = fx.dir.join("ok.md");
    fs::write(&f, "```latex\n\\frac{1}{2}\n```\n").unwrap();
    let (code, v) = run_json(&["validate", "--file", f.to_str().unwrap()]);
    assert_eq!(code, 0);
    assert_eq!(v["summary"]["errors"].as_i64().unwrap(), 0);
    assert_eq!(v["file"].as_str().unwrap(), f.to_str().unwrap());
}

#[test]
fn t03_validate_five_error_classes_exit1() {
    // 一文聚齐 5 类：UNCLOSED_BRACKET + REQUIRED_PARAM + ENV_MISMATCH + MISSING_ENV
    // 四错误 + UNDEFINED_REF 一警告。
    let text = "\\begin{align}\n\\frac{b} (\n\\ref{eq:nope}\n\\end{gather}\n\\begin{matrix}\n";
    let (code, v) = run_json(&["validate", "--text", text]);
    assert_eq!(code, 1);
    assert_eq!(v["summary"]["errors"].as_i64().unwrap(), 4);
    assert_eq!(v["summary"]["warnings"].as_i64().unwrap(), 1);
    let rules: Vec<&str> = v["errors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["rule_id"].as_str().unwrap())
        .collect();
    assert!(rules.contains(&"UNCLOSED_BRACKET"));
    assert!(rules.contains(&"REQUIRED_PARAM"));
    assert!(rules.contains(&"ENV_MISMATCH"));
    assert!(rules.contains(&"MISSING_ENV"));
    let wrules: Vec<&str> = v["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w["rule_id"].as_str().unwrap())
        .collect();
    assert_eq!(wrules, vec!["UNDEFINED_REF"]);
    // Issue 出参形五键齐全
    let e0 = &v["errors"][0];
    for k in ["rule_id", "message", "line", "column", "severity", "context"] {
        assert!(e0.get(k).is_some(), "issue 缺键 {k}");
    }
}

#[test]
fn t04_block_create_integral_golden() {
    let (code, v) = run_json(&[
        "block-create",
        "--template",
        "integral",
        "--params",
        r"lower=0,upper=\infty,integrand=e^{-x^2},var=x",
    ]);
    assert_eq!(code, 0);
    assert_eq!(v["template"].as_str().unwrap(), "integral");
    let block = v["block"].as_str().unwrap();
    assert!(block.starts_with("```latex\n"));
    assert!(block.ends_with("\n```"));
    assert!(block.contains(r"\int_{0}^{\infty}"));
    assert!(block.contains("e^{-x^2}"));
    assert!(block.contains(r"\, dx"));
}

#[test]
fn t05_block_create_list_and_defaults() {
    let (code, v) = run_json(&["block-create", "--list"]);
    assert_eq!(code, 0);
    let names: Vec<&str> = v["templates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, vec!["align", "equation", "frac", "integral", "lim", "matrix", "pmatrix", "sqrt", "sum"]);
    // 缺省参数：frac 默认 a/b
    let (code, v) = run_json(&["block-create", "--template", "frac"]);
    assert_eq!(code, 0);
    assert!(v["block"].as_str().unwrap().contains(r"\frac{a}{b}"));
    assert_eq!(v["params"].as_str().unwrap(), "");
}

#[test]
fn t06_suggest_english_and_chinese() {
    let (code, v) = run_json(&["suggest", "integral"]);
    assert_eq!(code, 0);
    assert_eq!(v["count"].as_i64().unwrap(), 1);
    let m = &v["matches"][0];
    assert_eq!(m["key"].as_str().unwrap(), "integral_definite");
    assert_eq!(m["latex"].as_str().unwrap(), r"\int_{a}^{b} f(x) \, dx");
    assert_eq!(m["category"].as_str().unwrap(), "operator");
    for k in ["key", "latex", "category", "description", "params", "optional"] {
        assert!(m.get(k).is_some(), "suggest 条目缺键 {k}");
    }
    // 中文关键词同链
    let (code, v) = run_json(&["suggest", "积分"]);
    assert_eq!(code, 0);
    assert!(v["count"].as_i64().unwrap() >= 1);
    assert_eq!(v["matches"][0]["key"].as_str().unwrap(), "integral_definite");
}

#[test]
fn t07_knowledge_inventory() {
    let (code, v) = run_json(&["knowledge"]);
    assert_eq!(code, 0);
    // 知识库 84 词条全量移植（围堰 grep "latex": 计 85 含文档行，词条实数 84）
    assert_eq!(v["count"].as_i64().unwrap(), 84);
    let keys = v["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 84);
    assert!(keys.iter().any(|k| k.as_str().unwrap() == "integral_definite"));
    let cats: Vec<&str> = v["categories"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c.as_str().unwrap())
        .collect();
    assert!(cats.contains(&"operator"));
    assert!(cats.contains(&"env_matrix"));
    assert!(cats.contains(&"greek"));
}

// ============ 拒绝形：参数缺 / 文件缺 / 模板缺 / 未移植子命令 ============

#[test]
fn t08_validate_missing_arg_exit2() {
    let (code, v) = run_json(&["validate"]);
    assert_eq!(code, 2);
    assert_eq!(v["error"].as_str().unwrap(), "must provide --file or --text");
}

#[test]
fn t09_validate_file_not_found_exit2() {
    let fx = build_fx();
    let f = fx.dir.join("nope.md");
    let (code, v) = run_json(&["validate", "--file", f.to_str().unwrap()]);
    assert_eq!(code, 2);
    assert!(v["error"].as_str().unwrap().starts_with("file not found: "));
}

#[test]
fn t10_autofix_file_not_found_exit2() {
    let fx = build_fx();
    let f = fx.dir.join("nope.md");
    let (code, v) = run_json(&["autofix", "--file", f.to_str().unwrap()]);
    assert_eq!(code, 2);
    assert_eq!(v["error"].as_str().unwrap(), "file not found");
    assert_eq!(v["summary"]["added"].as_i64().unwrap(), 0);
}

#[test]
fn t11_block_create_unknown_template_exit2() {
    let (code, v) = run_json(&["block-create", "--template", "no_such_template_xyz"]);
    assert_eq!(code, 2);
    let err = v["error"].as_str().unwrap();
    assert!(
        err.starts_with("unknown template: 'no_such_template_xyz' (available: "),
        "报文形不对: {err}"
    );
    assert!(err.ends_with("align, equation, frac, integral, lim, matrix, pmatrix, sqrt, sum)"));
}

#[test]
fn t12_block_create_missing_required_exit2() {
    // arg 必填，显式空串触发 required 判（对表 Python falsy 语义）
    let (code, v) = run_json(&["block-create", "--template", "sqrt", "--params", "arg="]);
    assert_eq!(code, 2);
    assert_eq!(v["error"].as_str().unwrap(), "missing required param: arg");
}

#[test]
fn t13_cli_usage_rejections_exit2() {
    // 无子命令
    let (code, _out, stderr) = run(&[]);
    assert_eq!(code, 2);
    assert!(!stderr.is_empty());
    // 未知子命令
    let (code, _out, stderr) = run(&["frobnicate"]);
    assert_eq!(code, 2);
    assert!(!stderr.is_empty());
    // 坏 --format 值
    let (code, _out, stderr) = run(&["validate", "--text", "x", "--format", "yaml"]);
    assert_eq!(code, 2);
    assert!(!stderr.is_empty());
}

#[test]
fn t14_compute_unported_exit2() {
    // 落差申报一：参数面保留，调用即申报未移植，退出码 2
    let (code, v) = run_json(&["compute", "simplify", "x^2 + 2*x + 1"]);
    assert_eq!(code, 2);
    let err = v["error"].as_str().unwrap();
    assert!(err.contains("未移植"), "报文应申报未移植: {err}");
    assert_eq!(v["operation"].as_str().unwrap(), "simplify");
    // 缺参报文与围堰对齐
    let (code, v) = run_json(&["compute", "limit"]);
    assert_eq!(code, 2);
    assert_eq!(
        v["error"].as_str().unwrap(),
        "operation and expression required (positional or --operation/--expression)"
    );
}

// ============ 边界形：半开区间豁免 / 转义括号 / 修复双模式 / 空查询 / limit ============

#[test]
fn t15_half_open_intervals_exempt() {
    // 标准数学半开区间 [a, b) 与 (a, b] 白名单豁免
    let (code, v) = run_json(&["validate", "--text", "q ∈ [0, 1)"]);
    assert_eq!(code, 0);
    assert_eq!(v["summary"]["errors"].as_i64().unwrap(), 0);
    let (code, v) = run_json(&["validate", "--text", "x ∈ (-∞, 1]"]);
    assert_eq!(code, 0);
    assert_eq!(v["summary"]["errors"].as_i64().unwrap(), 0);
    // 未闭合的 [0, 1（缺右括号）仍要报
    let (code, v) = run_json(&["validate", "--text", "x ∈ [0, 1"]);
    assert_eq!(code, 1);
    let rules: Vec<&str> = v["errors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["rule_id"].as_str().unwrap())
        .collect();
    assert!(rules.contains(&"UNCLOSED_BRACKET"));
}

#[test]
fn t16_escaped_brackets_exempt() {
    // \{ \} 转义括号不入栈匹配
    let (code, v) = run_json(&["validate", "--text", r"set \{a, b\} has size 2"]);
    assert_eq!(code, 0);
    assert_eq!(v["summary"]["errors"].as_i64().unwrap(), 0);
}

#[test]
fn t17_autofix_fixes_brace_and_env() {
    // 未闭合 { 行尾补 }
    let (code, v) = run_json(&["autofix", "--text", r"\frac{1}{2"]);
    assert_eq!(code, 1);
    assert_eq!(v["changed"].as_bool().unwrap(), true);
    assert!(v["fixed_text"].as_str().unwrap().contains(r"\frac{1}{2}"));
    assert!(v["summary"]["added"].as_i64().unwrap() >= 1);
    assert!(v["original_text"].as_str().unwrap().contains(r"\frac{1}{2"));
    // 缺 \end 末尾补
    let (code, v) = run_json(&["autofix", "--text", r"\begin{matrix} a & b \\ c & d"]);
    assert_eq!(code, 1);
    let fixed = v["fixed_text"].as_str().unwrap();
    assert!(fixed.contains(r"\end{matrix}"));
    assert!(fixed.ends_with('\n'));
    // \left( 缺 \right 补 \right) 形
    let (code, v) = run_json(&["autofix", "--text", r"\left( x + 1"]);
    assert_eq!(code, 1);
    assert!(v["fixed_text"].as_str().unwrap().contains(r"\left( x + 1\right)"));
    // 干净文本不动，退出码 0
    let (code, v) = run_json(&["autofix", "--text", r"\frac{1}{2}"]);
    assert_eq!(code, 0);
    assert_eq!(v["changed"].as_bool().unwrap(), false);
    assert_eq!(v["summary"]["added"].as_i64().unwrap(), 0);
}

#[test]
fn t18_autofix_file_check_then_write() {
    let fx = build_fx();
    let f = fx.dir.join("broken.md");
    fs::write(&f, r"\frac{1}{2").unwrap();
    // check 模式：报修不落盘，退出码 1
    let (code, v) = run_json(&["autofix", "--file", f.to_str().unwrap()]);
    assert_eq!(code, 1);
    assert_eq!(v["written"].as_bool().unwrap(), false);
    assert_eq!(fs::read_to_string(&f).unwrap(), r"\frac{1}{2");
    // --write 模式：落盘修复文本（退出码仍 1：修了 > 0 处）
    let (code, v) = run_json(&["autofix", "--file", f.to_str().unwrap(), "--write"]);
    assert_eq!(code, 1);
    assert_eq!(v["written"].as_bool().unwrap(), true);
    assert_eq!(fs::read_to_string(&f).unwrap(), r"\frac{1}{2}");
}

#[test]
fn t19_suggest_empty_query_and_limit() {
    // 空查询：报 error 键但退出码 0（对表围堰 cmd_suggest）
    let (code, v) = run_json(&["suggest", ""]);
    assert_eq!(code, 0);
    assert_eq!(v["error"].as_str().unwrap(), "empty query");
    assert_eq!(v["count"].as_i64().unwrap(), 0);
    assert_eq!(v["query"].as_str().unwrap(), "");
    // 无匹配查询：count 0 无 error
    let (code, v) = run_json(&["suggest", "this is a very specific nonsense query xyzzy"]);
    assert_eq!(code, 0);
    assert_eq!(v["count"].as_i64().unwrap(), 0);
    assert!(v.get("error").is_none());
    // limit 截断生效
    let (code, v) = run_json(&["suggest", "alpha beta gamma delta epsilon", "--limit", "2"]);
    assert_eq!(code, 0);
    assert_eq!(v["count"].as_i64().unwrap(), 2);
}

#[test]
fn t20_defined_ref_no_warning() {
    let text = r"\begin{equation} a=b \label{eq:foo} \end{equation} 引用 \ref{eq:foo}";
    let (code, v) = run_json(&["validate", "--text", text]);
    assert_eq!(code, 0);
    assert_eq!(v["summary"]["warnings"].as_i64().unwrap(), 0);
    // 未声明引用报警但不改退出码（errors == 0 即 0）
    let (code, v) = run_json(&["validate", "--text", r"看 \ref{eq:foo}，其中 \begin{equation} a=b \end{equation}"]);
    assert_eq!(code, 0);
    assert_eq!(v["summary"]["warnings"].as_i64().unwrap(), 1);
    assert_eq!(v["warnings"][0]["rule_id"].as_str().unwrap(), "UNDEFINED_REF");
}

#[test]
fn t21_version_flag() {
    let (code, out, _stderr) = run(&["--version"]);
    assert_eq!(code, 0);
    assert!(out.contains("latextool"));
    assert!(out.contains(VERSION_ANCHOR));
}

const VERSION_ANCHOR: &str = "0.1.0";
