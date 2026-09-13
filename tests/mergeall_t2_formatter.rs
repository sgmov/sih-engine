//! lease-mergeall-parallel 簇B T2：formatter 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/formatter 0.2.0（cli.py 三值退出码、packs.py 包校验、
//! format.py 行级与 json 操作、report.py 报告形）。fixture 全部 temp 自建最小包，
//! 围堰真实包零触碰。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin_formatter() -> &'static str {
    env!("CARGO_BIN_EXE_formatter")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_formatter()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

struct Ws {
    dir: PathBuf,
    pack: PathBuf,
    _guard: tempfile::TempDir,
}

/// temp 自建最小格式包：envelope + manifest.toml + operations.toml
/// 两操作：L001 行级（general-v1 同参）、J001 json 规范化。
fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let pack = dir.join("pack-t2");
    fs::create_dir_all(&pack).unwrap();
    fs::write(
        pack.join("envelope.json"),
        r#"{"envelope_version":1,"id":"t2","family":"formatter","body_type":"config","bodies":["manifest.toml","operations.toml"]}"#,
    )
    .unwrap();
    fs::write(
        pack.join("manifest.toml"),
        "name = \"t2\"\nversion = \"0.0.1\"\n\n[domain]\ninclude = [\"**/*.md\", \"**/*.json\"]\nexclude = []\n",
    )
    .unwrap();
    fs::write(
        pack.join("operations.toml"),
        "[[ops]]\nid = \"L001\"\nkind = \"line_ops\"\nparams = { trailing_whitespace = true, eol = \"lf\", final_newline = true }\n\n[[ops]]\nid = \"J001\"\nkind = \"json_canonicalize\"\nparams = { sort_keys = true, indent = 2, ensure_ascii = false }\n",
    )
    .unwrap();
    Ws {
        dir,
        pack,
        _guard: guard,
    }
}

fn pack_arg(ws: &Ws) -> String {
    ws.pack.to_string_lossy().into_owned()
}

fn target(ws: &Ws, name: &str) -> String {
    ws.dir.join(name).to_string_lossy().into_owned()
}

// ---------- 正常形：check 报改、write 落写、幂等重跑落零、围栏豁免 ----------

#[test]
fn t1_normal_check_write_idempotent_fence() {
    let ws = build_ws();
    let pa = pack_arg(&ws);

    // check 只读：报改不落写，退出码 1
    let doc = target(&ws, "doc.md");
    fs::write(&doc, "你好  \n世界\t\n").unwrap();
    let (code, out, _) = run(&["--pack", &pa, &doc]);
    assert_eq!(code, 1, "有改须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["engine"]["name"], "formatter");
    assert_eq!(rep["packs"][0]["name"], "t2");
    assert_eq!(rep["summary"]["targets_changed"], 1);
    assert_eq!(rep["summary"]["total_changed_lines"], 2);
    assert_eq!(rep["changes"][0]["path"], doc);
    assert!(rep["content_hashes"][&doc].is_string(), "内容哈希须在报");
    assert_eq!(rep["domain_mismatches"].as_array().unwrap().len(), 0);
    assert_eq!(
        fs::read_to_string(&doc).unwrap(),
        "你好  \n世界\t\n",
        "check 只读不落写"
    );

    // write 落写：行尾空白清、末行换行补
    let (code, out, _) = run(&["--pack", &pa, "--write", &doc]);
    assert_eq!(code, 1, "首遍 write 有改须 1，stdout={out}");
    assert_eq!(fs::read_to_string(&doc).unwrap(), "你好\n世界\n");

    // 幂等重跑落零：二遍 write 输出恒同，退出码 0
    let (code, out, _) = run(&["--pack", &pa, "--write", &doc]);
    assert_eq!(code, 0, "二遍 write 须落零，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"]["targets_changed"], 0);

    // md 围栏内豁免：围栏内行尾空白保留，围栏外归一
    let doc2 = target(&ws, "fence.md");
    fs::write(&doc2, "```text\nkeep  \n```\n尾部  \n").unwrap();
    let (code, _, _) = run(&["--pack", &pa, "--write", &doc2]);
    assert_eq!(code, 1);
    assert_eq!(
        fs::read_to_string(&doc2).unwrap(),
        "```text\nkeep  \n```\n尾部\n",
        "围栏内豁免、围栏外归一"
    );
}

// ---------- 正常形（json 规范化）：键排序加末行换行 ----------

#[test]
fn t2_normal_json_canonicalize() {
    let ws = build_ws();
    let pa = pack_arg(&ws);
    let doc = target(&ws, "data.json");
    fs::write(&doc, "{\"b\": 1,\n  \"a\": 2}").unwrap();
    let (code, out, _) = run(&["--pack", &pa, "--write", &doc]);
    assert_eq!(code, 1, "json 有改须 1，stdout={out}");
    assert_eq!(fs::read_to_string(&doc).unwrap(), "{\n  \"a\": 2,\n  \"b\": 1\n}\n");
    // 幂等：二遍落零
    let (code, _, _) = run(&["--pack", &pa, "--write", &doc]);
    assert_eq!(code, 0, "json 二遍须落零");
}

// ---------- 拒绝形：包缺席、目标缺席、缺参、域外 ----------

#[test]
fn t3_reject_forms() {
    let ws = build_ws();
    let pa = pack_arg(&ws);

    // 包目录缺席：格式包加载失败，退出码 2，stderr 报文
    let (code, out, err) = run(&["--pack", "/tmp/t2-nonexistent-pack-dir", "x.md"]);
    assert_eq!(code, 2, "包缺席须 2");
    assert!(out.is_empty(), "包缺席 stdout 须空");
    assert!(err.contains("格式包加载失败"), "stderr 须报加载失败，err={err}");

    // envelope family 非法：包无效退出码 2
    let bad_pack = ws.dir.join("bad-pack");
    fs::create_dir_all(&bad_pack).unwrap();
    fs::write(bad_pack.join("envelope.json"), r#"{"envelope_version":1,"id":"b","family":"nope","body_type":"config","bodies":["manifest.toml"]}"#).unwrap();
    fs::write(bad_pack.join("manifest.toml"), "name = \"b\"\nversion = \"0\"\n[domain]\ninclude = [\"**/*.md\"]\n").unwrap();
    let (code, _, err) = run(&["--pack", &bad_pack.to_string_lossy(), "x.md"]);
    assert_eq!(code, 2, "family 非法须 2，err={err}");
    assert!(err.contains("family illegal"));

    // 目标缺席：退出码 2，mismatch reason 目标不存在或非文件
    let (code, out, _) = run(&["--pack", &pa, &target(&ws, "nope.md")]);
    assert_eq!(code, 2, "目标缺席须 2");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["domain_mismatches"][0]["reason"], "目标不存在或非文件");

    // 域外（.txt 不在 include）：退出码 2
    let txt = target(&ws, "note.txt");
    fs::write(&txt, "hello  \n").unwrap();
    let (code, out, _) = run(&["--pack", &pa, &txt]);
    assert_eq!(code, 2, "域外须 2");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        rep["domain_mismatches"][0]["reason"],
        "目标不在任何已加载格式包声明的扩展域内"
    );
    assert_eq!(rep["domain_mismatches"][0]["pack_domains"][0], "t2");

    // 缺 --pack：用法错退出码 2
    let doc = target(&ws, "doc.md");
    fs::write(&doc, "x\n").unwrap();
    let (code, _, _) = run(&[&doc]);
    assert_eq!(code, 2, "缺 --pack 须 2");

    // 缺目标：用法错退出码 2
    let (code, _, _) = run(&["--pack", &pa]);
    assert_eq!(code, 2, "缺目标须 2");

    // 未知操作 kind：表外请求拒绝，退出码 2
    let bad_ops = ws.dir.join("bad-ops-pack");
    fs::create_dir_all(&bad_ops).unwrap();
    fs::write(bad_ops.join("envelope.json"), r#"{"envelope_version":1,"id":"bo","family":"formatter","body_type":"config","bodies":["manifest.toml","operations.toml"]}"#).unwrap();
    fs::write(bad_ops.join("manifest.toml"), "name = \"bo\"\nversion = \"0\"\n[domain]\ninclude = [\"**/*.md\"]\n").unwrap();
    fs::write(bad_ops.join("operations.toml"), "[[ops]]\nid = \"X\"\nkind = \"no_such_kind\"\n").unwrap();
    let (code, out, _) = run(&["--pack", &bad_ops.to_string_lossy(), &doc]);
    assert_eq!(code, 2, "未知 kind 须 2");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert!(
        rep["domain_mismatches"][0]["reason"]
            .as_str()
            .unwrap()
            .contains("未知操作 kind"),
        "reason 须报未知操作"
    );
}

// ---------- 边界形：空文档、--quiet 紧凑行 ----------

#[test]
fn t4_edge_empty_and_quiet() {
    let ws = build_ws();
    let pa = pack_arg(&ws);

    // 空文档：零改，退出码 0（final_newline 不给空文补尾）
    let empty = target(&ws, "empty.md");
    fs::write(&empty, "").unwrap();
    let (code, out, _) = run(&["--pack", &pa, "--write", &empty]);
    assert_eq!(code, 0, "空文档须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"]["targets_changed"], 0);
    assert_eq!(fs::read_to_string(&empty).unwrap(), "", "空文零写");

    // --quiet 紧凑行：单行 JSON，code 与 summary 三键
    let doc = target(&ws, "q.md");
    fs::write(&doc, "脏行  \n").unwrap();
    let (code, out, _) = run(&["--pack", &pa, "--quiet", &doc]);
    assert_eq!(code, 1);
    let line: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(line["tool"], "formatter");
    assert_eq!(line["command"], "check");
    assert_eq!(line["code"], 1);
    assert_eq!(line["summary"]["targets_changed"], 1);
    assert_eq!(line["summary"]["domain_mismatches"], 0);
    let raw = out.trim();
    assert!(!raw.contains('\n'), "quiet 须单行，got={raw}");

    // --quiet 落零形：干净文档 code 0
    fs::write(&doc, "净行\n").unwrap();
    let (code, out, _) = run(&["--pack", &pa, "--quiet", &doc]);
    assert_eq!(code, 0);
    let line: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(line["code"], 0);
    assert_eq!(line["summary"]["total_changed_lines"], 0);
}
