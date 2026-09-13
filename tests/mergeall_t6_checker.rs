//! lease-mergeleg6-parallel 簇I T6：checker 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/checker 0.1.0（cli.py、engine.py，空腹判定机）。
//! 三例金向量：正常（词汇表多操作全绿与报告双版本戳）、拒绝（fail 三态与
//! 包非法/目标缺席退出码二）、边界（空节、min 恰达、reference_closure 闭包）。
//! fixture 全部 temp 自建；包 JSON 以字符串字面量承载（避开 json! 宏对深嵌套
//! 对象数组的解析局限）。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_checker")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn run_json(args: &[&str]) -> (i32, Value, String) {
    let (code, out, err) = run(args);
    (code, serde_json::from_str(&out).unwrap_or(Value::Null), err)
}

struct Fx {
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    Fx { dir: guard.path().to_path_buf(), _guard: guard }
}

impl Fx {
    fn write(&self, name: &str, content: &str) -> String {
        let path = self.dir.join(name);
        fs::write(&path, content).unwrap();
        path.display().to_string()
    }

    /// 包 JSON 以字符串字面量写入（完整包形：pack/version/doc_class/rules）。
    fn pack_raw(&self, name: &str, rules_json: &str) -> String {
        self.write(
            name,
            &format!(
                r#"{{"pack": "t6pack", "version": "1.0.0", "doc_class": "t6", "rules": {rules_json}}}"#
            ),
        )
    }
}

// ---------- 正常形：多操作全绿与报告形 ----------

#[test]
fn t1_normal_all_pass() {
    let fx = build_fx();
    let pack = fx.pack_raw(
        "pack.json",
        r#"[
        {"id": "R1", "check": {"ops": [{"op": "heading_present", "level": 1}]}},
        {"id": "R2", "check": {"ops": [{"op": "sections_present", "level": 2, "names": ["A", "B"], "ordered": true}]}},
        {"id": "R3", "check": {"ops": [{"op": "section_nonempty", "level": 2, "name": "A"}]}},
        {"id": "R4", "check": {"ops": [{"op": "list_items_present", "marker": "-", "min": 2}]}},
        {"id": "R5", "check": {"ops": [{"op": "list_items_match", "marker": "-", "token": "T6-\\d+"}]}},
        {"id": "R6", "check": {"ops": [{"op": "pattern_unique", "pattern": "T6-\\d+", "format": "T6-\\d+"}]}},
        {"id": "R7", "check": {"ops": [{"op": "exists_readable"}]}},
        {"id": "R8", "check": {"ops": [{"op": "child_line_matches", "child_pattern": "^K-\\d+\\b.*$", "line_prefix": "状态：", "must_match": ["三态"]}]}}
    ]"#,
    );
    let target = fx.write(
        "target.md",
        concat!(
            "# 标题\n",
            "\n",
            "## A\n",
            "- T6-1 第一项\n",
            "- T6-2 第二项\n",
            "\n",
            "K-1 子块\n",
            "状态：三态承载\n",
            "\n",
            "## B\n",
            "正文在位\n",
        ),
    );
    let (code, out, err) = run(&["--pack", &pack, &target]);
    assert_eq!(code, 0, "全绿须 0，err={err}");
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["engine"], "checker");
    assert_eq!(v["engine_version"], "0.1.0");
    assert_eq!(v["pack"], "t6pack");
    assert_eq!(v["pack_version"], "1.0.0");
    assert_eq!(v["doc_class"], "t6");
    assert_eq!(v["verdict"], "pass");
    assert_eq!(v["results"].as_array().unwrap().len(), 8);
    assert!(v["results"].as_array().unwrap().iter().all(|r| r["state"] == "pass"));
    assert_eq!(v["findings"].as_array().unwrap().len(), 0);

    // any_of 组合子：一支过即过
    let pack2 = fx.pack_raw(
        "pack2.json",
        r#"[{"id": "ANY", "check": {"ops": [{"op": "any_of", "branches": [
            {"op": "heading_present", "level": 6},
            {"op": "heading_present", "level": 1}
        ]}]}}]"#,
    );
    let (code, out, _) = run(&["--pack", &pack2, &target]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["verdict"], "pass");

    // stdout 为 Python json.dumps(indent=1) 同构形（层进单空格、尾换行）
    assert!(out.starts_with("{\n \"doc_class\": \"t6\",\n"), "indent=1 金向量形，got={out}");
    assert!(out.ends_with("}\n"));
}

// ---------- 拒绝形：fail 三态与工具异常退出码二 ----------

#[test]
fn t2_reject_forms() {
    let fx = build_fx();
    let target = fx.write(
        "target.md",
        concat!(
            "# 标题\n",
            "## A\n",
            "- T6-1 第一项\n",
            "- T6-1 重号项\n",
            "## B\n",
        ),
    );

    // 缺节 + 空节 + 重号 + 缺 token：四态一次触发
    let pack = fx.pack_raw(
        "pack.json",
        r#"[
        {"id": "MISS", "check": {"ops": [{"op": "sections_present", "level": 2, "names": ["A", "X"]}]}},
        {"id": "EMPTY", "check": {"ops": [{"op": "section_nonempty", "level": 2, "name": "B"}]}},
        {"id": "DUP", "check": {"ops": [{"op": "pattern_unique", "pattern": "T6-\\d+", "format": "T6-\\d+"}]}},
        {"id": "TOK", "check": {"ops": [{"op": "list_items_match", "marker": "-", "token": "有标记"}]}}
    ]"#,
    );
    let (code, out, _) = run(&["--pack", &pack, &target]);
    assert_eq!(code, 1, "违规须 1");
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["verdict"], "fail");
    let findings = v["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 4);
    assert_eq!(findings[0]["state"], "missing", "缺节 missing");
    assert!(findings[0]["detail"].as_str().unwrap().contains("缺节位"));
    assert_eq!(findings[1]["state"], "violation", "空节 violation");
    assert!(findings[1]["detail"].as_str().unwrap().contains("空节"));
    assert_eq!(findings[2]["state"], "violation", "重号 violation");
    assert!(findings[2]["detail"].as_str().unwrap().contains("重号"));
    assert_eq!(findings[3]["state"], "broken_ref", "缺 token broken_ref");
    // hint 通道透传（failure_location 缺席即空串）
    assert_eq!(findings[0]["hint"], "");

    // 节序错位：ordered 落 violation
    let pack2 = fx.pack_raw(
        "pack2.json",
        r#"[{"id": "ORD", "check": {"ops": [{"op": "sections_present", "level": 2, "names": ["B", "A"], "ordered": true}]}}]"#,
    );
    let (code, out, _) = run(&["--pack", &pack2, &target]);
    assert_eq!(code, 1);
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["findings"][0]["state"], "violation");
    assert_eq!(v["findings"][0]["detail"], "节序错位");

    // 词汇表外操作：包非法退出码二（stderr）
    let bad_pack = fx.pack_raw(
        "bad.json",
        r#"[{"id": "B", "check": {"ops": [{"op": "no_such_op"}]}}]"#,
    );
    let (code, _, err) = run(&["--pack", &bad_pack, &target]);
    assert_eq!(code, 2);
    assert!(err.contains("包非法"), "err={err}");

    // 包文件缺席 → 2；目标缺席 → 2；包缺键 → 2
    let (code, _, err) = run(&["--pack", "/no/such/pack.json", &target]);
    assert_eq!(code, 2);
    assert!(err.contains("包文件缺席"));
    let (code, _, err) = run(&["--pack", &pack, "/no/such/target.md"]);
    assert_eq!(code, 2);
    assert!(err.contains("目标缺席"));
    let missing_key = fx.write("mk.json", r#"{"pack": "x"}"#);
    let (code, _, err) = run(&["--pack", &missing_key, &target]);
    assert_eq!(code, 2);
    assert!(err.contains("包缺键"));

    // 用法错：缺 --pack → 2
    let (code, _, err) = run(&[&target]);
    assert_eq!(code, 2);
    assert!(err.contains("用法错"));
}

// ---------- 边界形：min 恰达、闭包悬空、条款正文与引用缺席 ----------

#[test]
fn t3_edge_min_and_closure() {
    let fx = build_fx();

    // section_has_matches：恰达 min 过，低于即 missing
    let target = fx.write(
        "t.md",
        concat!(
            "# 标题\n",
            "## 校验\n",
            "锚甲 锚乙 锚丙\n",
        ),
    );
    let pack_ok = fx.pack_raw(
        "ok.json",
        r#"[{"id": "M", "check": {"ops": [{"op": "section_has_matches", "section": "校验", "pattern": "锚", "min": 3}]}}]"#,
    );
    let (code, _, _) = run(&["--pack", &pack_ok, &target]);
    assert_eq!(code, 0, "恰达 min 须过");
    let pack_low = fx.pack_raw(
        "low.json",
        r#"[{"id": "M", "check": {"ops": [{"op": "section_has_matches", "section": "校验", "pattern": "锚", "min": 4}]}}]"#,
    );
    let (code, out, _) = run(&["--pack", &pack_low, &target]);
    assert_eq!(code, 1);
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["findings"][0]["state"], "missing");

    // 条款正文 token：块正文含 REQUIRED 过，缺即 violation
    let clause_doc = fx.write(
        "clauses.md",
        concat!(
            "# 条款面\n",
            "C-1 强制条款甲\n",
            "正文含 REQUIRED 词\n",
            "C-2 强制条款乙\n",
            "正文缺词\n",
        ),
    );
    let pack_c = fx.pack_raw(
        "c.json",
        r#"[{"id": "C", "check": {"ops": [{"op": "clause_body_has", "clause_pattern": "^C-\\d+.*$", "token": "REQUIRED"}]}}]"#,
    );
    let (code, out, _) = run(&["--pack", &pack_c, &clause_doc]);
    assert_eq!(code, 1);
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["findings"][0]["state"], "violation");
    assert!(v["findings"][0]["detail"].as_str().unwrap().contains("C-2"));

    // reference_closure：@source 全闭合过；悬空即 broken_ref；--reference 缺席即 2
    let target = fx.write(
        "refs.md",
        concat!(
            "# 引用面\n",
            "REF X-1 引用甲\n",
            "REF X-2 引用乙\n",
        ),
    );
    let ref_doc = fx.write("defs.md", "# 定义面\nX-1 即甲\nX-2 即乙\n");
    let pack_r = fx.pack_raw(
        "r.json",
        r#"[{"id": "R", "check": {"ops": [{"op": "reference_closure", "items_marker": "REF", "ref_pattern": "X-\\d+", "source": "@reference", "defined_pattern": "(X-\\d+)"}]}}]"#,
    );
    let (code, _, _) = run(&["--pack", &pack_r, &target, "--reference", &ref_doc]);
    assert_eq!(code, 0, "闭包全闭合须过");

    let partial = fx.write("partial.md", "# 定义面\nX-1 即甲\n");
    let (code, out, _) = run(&["--pack", &pack_r, &target, "--reference", &partial]);
    assert_eq!(code, 1);
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["findings"][0]["state"], "broken_ref");
    assert!(v["findings"][0]["detail"].as_str().unwrap().contains("X-2"));

    let (code, _, err) = run(&["--pack", &pack_r, &target]);
    assert_eq!(code, 2, "--reference 缺席须 2");
    assert!(err.contains("引用目标异常"));

    // source 非 @reference：包非法 2
    let pack_s = fx.pack_raw(
        "s.json",
        r#"[{"id": "S", "check": {"ops": [{"op": "reference_closure", "items_marker": "REF", "ref_pattern": "X-\\d+", "source": "@elsewhere", "defined_pattern": "(X-\\d+)"}]}}]"#,
    );
    let (code, _, _) = run(&["--pack", &pack_s, &target, "--reference", &ref_doc]);
    assert_eq!(code, 2);
}
