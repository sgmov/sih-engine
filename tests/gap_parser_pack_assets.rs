//! gap-parser-pack-assets：parser 引擎仓包资产缺省包根解析回归。
//!
//! 承 parser.rs 引擎位默认包根（SPEC-025 融回缺口 gap-packs-assets）：
//! 相对形包名经引擎位候选序补解析，首位 exe 派生（引擎仓根/packs/parser），
//! cwd 无关。既有 mergeall_t3_parser 全走 temp 自建包显式 --pack，本件补
//! 引擎仓真实资产（packs/parser/json 与 packs/parser/markdown，含 vectors
//! 金向量在册资产）的缺省解析闭环：裸包名 vectors 双包全过、parse/entries
//! 产出对表在册期望件、裸包名不存在的报错形。fixture 全 temp 自建，
//! 围堰真实包零触碰。

use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

fn bin_parser() -> &'static str {
    env!("CARGO_BIN_EXE_parser")
}

fn engine_pack(pack: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("packs/parser").join(pack)
}

fn run_in(dir: &PathBuf, args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_parser())
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn temp_cwd(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("gap-parser-pack-{tag}-{}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// 裸包名 vectors：json 与 markdown 双包经 exe 派生缺省包根解析，金向量全过。
#[test]
fn t1_bare_pack_name_resolves_engine_assets_vectors_pass() {
    let cwd = temp_cwd("vectors");
    for pack in ["json", "markdown"] {
        assert!(
            engine_pack(pack).is_dir(),
            "引擎 {pack} 包资产须在位（gap-packs-assets 落位）：{}",
            engine_pack(pack).display()
        );
        let (code, out, err) = run_in(&cwd, &["vectors", "--pack", pack]);
        assert_eq!(code, 0, "vectors {pack} 须全过：{out} {err}");
        let v: Value = serde_json::from_str(out.trim()).expect("vectors 出参 JSON");
        assert_eq!(v["fail"].as_array().map(|a| a.len()), Some(0), "{pack} 零败：{v}");
        let passed = v["pass"].as_i64().unwrap_or(0);
        assert!(passed > 0, "{pack} 须有在跑向量（pass 计数）：{v}");
    }
}

/// parse/entries 经裸包名解析产出与在册期望件逐字节一致（cwd 与引擎仓无关）。
#[test]
fn t2_parse_and_entries_match_checked_in_expected() {
    let cwd = temp_cwd("parse");
    let sample = engine_pack("json").join("vectors/in/sample.json");

    let (code, out, err) = run_in(&cwd, &["parse", "--pack", "json", "--in", &sample.display().to_string()]);
    assert_eq!(code, 0, "parse 败：{err}");
    let expected = std::fs::read_to_string(engine_pack("json").join("vectors/expected/sample.json.parse.json")).unwrap();
    assert_eq!(out, expected, "parse 产出须与在册期望件逐字节一致");

    // entries：出参落盘与在册 entries 期望 ndjson 逐字节一致。
    // entries 条目 path 字段按传入形记录（期望件即相对形 vectors/in/sample.json），
    // 故与冻结形同参：cwd 置包目录内，--in 用相对形。
    let out_path = cwd.join("entries.ndjson");
    let pack_dir = engine_pack("json");
    let (code, _, err) = run_in(
        &pack_dir,
        &[
            "entries",
            "--pack",
            "json",
            "--in",
            "vectors/in/sample.json",
            "--out",
            &out_path.display().to_string(),
        ],
    );
    assert_eq!(code, 0, "entries 败：{err}");
    let got = std::fs::read_to_string(&out_path).unwrap();
    let expected_entries =
        std::fs::read_to_string(engine_pack("json").join("vectors/expected/sample.json.entries.ndjson")).unwrap();
    assert_eq!(got, expected_entries, "entries 产出须与在册期望件逐字节一致");
}

/// 全不中（裸名两侧皆无）报既有「语言包目录不存在」路径退出码一，
/// 不静默不降级（vectors 对缺席包是零向量绿态，包缺席报错由 parse 的
/// 立即 load_pack 路径承载）；t1/t2 自无关 cwd 成功本身即证解析走 exe 派生引擎位。
#[test]
fn t3_bare_pack_missing_reports_pack_missing_exit_one() {
    let cwd = temp_cwd("missing");
    let (code, _, err) = run_in(
        &cwd,
        &["parse", "--pack", "ghost-pack-nowhere", "--in", "whatever.json"],
    );
    assert_eq!(code, 1, "裸名不存在退出码一：{err}");
    assert!(
        err.contains("语言包目录不存在或不可读"),
        "报错形须对表既有包缺席路径：{err}"
    );
}
