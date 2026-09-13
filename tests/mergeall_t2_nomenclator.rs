//! lease-mergeall-parallel 簇B T2：nomenclator 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/nomenclator 0.3.0（cli.py 三值退出码、pack.py 六 body
//! 包校验、check.py 两规则、matching.py 匹配三粒度、query.py 六态、map.py 四段）。
//! fixture 全部 temp 自建最小包，围堰真实包零触碰。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin_nomenclator() -> &'static str {
    env!("CARGO_BIN_EXE_nomenclator")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_nomenclator())
        .args(args)
        .output()
        .unwrap();
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

/// temp 自建最小术语包：envelope 加 manifest 加 terms 加 lazy 加 dead 加 candidates
/// 加 anchors，四状态体与围堰 pack.py 校验序对表。
fn build_ws() -> Ws {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let pack = dir.join("pack-t2");
    fs::create_dir_all(&pack).unwrap();
    fs::write(
        pack.join("envelope.json"),
        r#"{"envelope_version":1,"id":"t2core","family":"nomenclator","body_type":"config","bodies":["manifest.json","terms.json","lazy.json","dead.json","candidates.json","anchors.json"]}"#,
    )
    .unwrap();
    fs::write(
        pack.join("manifest.json"),
        r#"{"name":"t2core","version":"0.0.1","domain":{"include":["**/*.md"],"exclude":["**/skip/**"]}}"#,
    )
    .unwrap();
    fs::write(
        pack.join("terms.json"),
        r#"{"terms":[
            {"zh":"化格","en":"Formatter","code":"formatter","definition":"化其格式，笔在核前","signed":"2026-09-13","source":"T2"},
            {"zh":"核阅","en":"Scrutinator","code":"scrutinator","definition":"对材料逐条核验阅览","signed":"2026-09-13","source":"T2"}
        ]}"#,
    )
    .unwrap();
    fs::write(
        pack.join("lazy.json"),
        r#"{"lazy":[
            {"word":"懒波词","since":"2026-09-13","source":"T2"},
            {"word":"Format","since":"2026-09-13","source":"T2"}
        ]}"#,
    )
    .unwrap();
    fs::write(
        pack.join("dead.json"),
        r#"{"dead":[
            {"word":"筛分","class":"strict","cause":"测试死因一","source":"T2"},
            {"word":"分拣","class":"name_only","cause":"测试死因二","source":"T2"}
        ]}"#,
    )
    .unwrap();
    fs::write(
        pack.join("candidates.json"),
        r#"{"candidates":[{"word":"候补词","note":"备而不荐","source":"T2"}]}"#,
    )
    .unwrap();
    fs::write(pack.join("anchors.json"), r#"{"anchors":[]}"#).unwrap();
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

// ---------- 正常形：check 两规则、query 六态、map 四段 ----------

#[test]
fn t1_normal_check_query_map() {
    let ws = build_ws();
    let pa = pack_arg(&ws);

    // check：禁字级死档（strict）加懒波词，同键去重，退出码 1
    let doc = target(&ws, "doc.md");
    fs::write(&doc, "本件用筛分一词。\n懒波词出现两处懒波词。\n").unwrap();
    let (code, out, _) = run(&["check", "--pack", &pa, &doc]);
    assert_eq!(code, 1, "有违例须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["header"]["tool"]["name"], "nomenclator");
    assert_eq!(rep["header"]["pack"]["name"], "t2core");
    assert_eq!(rep["summary"], serde_json::json!({"checked":1,"skipped":0,"findings":2}));
    let findings = rep["findings"].as_array().unwrap();
    assert_eq!(findings[0]["rule"], "dead_ban");
    assert_eq!(findings[0]["severity"], "high");
    assert_eq!(findings[0]["word"], "筛分");
    assert_eq!(findings[0]["class"], "strict");
    assert_eq!(findings[0]["line"], 1);
    assert_eq!(findings[0]["cause"], "测试死因一");
    assert_eq!(findings[1]["rule"], "lazy_in_doc");
    assert_eq!(findings[1]["severity"], "medium");
    assert_eq!(findings[1]["class"], "lazy");
    assert_eq!(findings[1]["line"], 2, "同行两命中须去重为一");

    // check 英文整词粒度：Format 命中 Format，不命中 Formatter 与 formatx
    let doc2 = target(&ws, "en.md");
    fs::write(&doc2, "Format 与 Formatter 与 formatx\n").unwrap();
    let (code, out, _) = run(&["check", "--pack", &pa, &doc2]);
    assert_eq!(code, 1, "out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    let findings = rep["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 1, "整词边界外零命中，out={out}");
    assert_eq!(findings[0]["word"], "Format");
    assert_eq!(findings[0]["line"], 1);

    // query：已立六态出参（established 为首态）
    let (code, out, _) = run(&["query", "--pack", &pa, "--word", "化格"]);
    assert_eq!(code, 0, "out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["word"], "化格");
    assert_eq!(rep["state"], "established");
    assert_eq!(rep["states"], serde_json::json!(["established"]));
    assert_eq!(rep["hits"][0]["definition"], "化其格式，笔在核前");
    assert_eq!(rep["hits"][0]["code"], "formatter");

    // query：死档两态标签 dead/strict 与 dead/name_only
    let (code, out, _) = run(&["query", "--pack", &pa, "--word", "筛分"]);
    assert_eq!(code, 0, "out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["state"], "dead/strict");
    assert_eq!(rep["hits"][0]["cause"], "测试死因一");
    let (code, out, _) = run(&["query", "--pack", &pa, "--word", "分拣"]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["state"], "dead/name_only");

    // map：既裁 code 形加六态透传加出泊指针，只报不判恒零
    let (code, out, _) = run(&["map", "--pack", &pa, "--concept", "化格"]);
    assert_eq!(code, 0, "map 恒零，out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["six_states"]["state"], "established");
    assert_eq!(rep["code_forms"].as_array().unwrap().len(), 1);
    assert_eq!(rep["code_forms"][0]["code"], "formatter");
    assert_eq!(rep["neighbors"].as_array().unwrap().len(), 0);
    assert_eq!(rep["pointers"].as_array().unwrap().len(), 4);
    assert!(
        rep["disclaimer"]
            .as_str()
            .unwrap()
            .starts_with("只报不判"),
        "免责声明须在报"
    );

    // map 近邻：概念「筛」跨态子串命中死档筛分（dead/strict）
    let (code, out, _) = run(&["map", "--pack", &pa, "--concept", "筛"]);
    assert_eq!(code, 0, "out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["six_states"]["state"], "unknown");
    let neighbors = rep["neighbors"].as_array().unwrap();
    assert_eq!(neighbors.len(), 1, "out={out}");
    assert_eq!(neighbors[0]["state"], "dead/strict");
    assert_eq!(neighbors[0]["name"], "筛分");
}

// ---------- 拒绝形：包缺席、包非法、目标缺席、缺参、register 未实装 ----------

#[test]
fn t2_reject_forms() {
    let ws = build_ws();
    let pa = pack_arg(&ws);

    // 包目录缺席：错误 JSON 退出码 2
    let (code, out, _) = run(&["check", "--pack", "/tmp/t2-nonexistent-nom-pack", "x.md"]);
    assert_eq!(code, 2, "包缺席须 2");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert!(rep["error"].as_str().unwrap().contains("envelope missing"));

    // dead class 非法：包校验拒，退出码 2
    let bad = ws.dir.join("bad-pack");
    fs::create_dir_all(&bad).unwrap();
    fs::write(bad.join("envelope.json"), r#"{"envelope_version":1,"id":"b","family":"nomenclator","body_type":"config","bodies":["manifest.json","terms.json","lazy.json","dead.json","candidates.json"]}"#).unwrap();
    fs::write(bad.join("manifest.json"), r#"{"name":"b","version":"0","domain":{}}"#).unwrap();
    fs::write(bad.join("terms.json"), r#"{"terms":[]}"#).unwrap();
    fs::write(bad.join("lazy.json"), r#"{"lazy":[]}"#).unwrap();
    fs::write(bad.join("dead.json"), r#"{"dead":[{"word":"坏档","class":"wild"}]}"#).unwrap();
    fs::write(bad.join("candidates.json"), r#"{"candidates":[]}"#).unwrap();
    let doc = target(&ws, "doc.md");
    fs::write(&doc, "x\n").unwrap();
    let (code, out, _) = run(&["check", "--pack", &bad.to_string_lossy(), &doc]);
    assert_eq!(code, 2, "class 非法须 2，out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert!(rep["error"].as_str().unwrap().contains("死档条目 class 非法"));

    // 清单缺字段：退出码 2
    let nofield = ws.dir.join("nofield-pack");
    fs::create_dir_all(&nofield).unwrap();
    fs::write(nofield.join("envelope.json"), r#"{"envelope_version":1,"id":"n","family":"nomenclator","body_type":"config","bodies":["manifest.json","terms.json","lazy.json","dead.json","candidates.json"]}"#).unwrap();
    fs::write(nofield.join("manifest.json"), r#"{"name":"n"}"#).unwrap();
    fs::write(nofield.join("terms.json"), r#"{"terms":[]}"#).unwrap();
    fs::write(nofield.join("lazy.json"), r#"{"lazy":[]}"#).unwrap();
    fs::write(nofield.join("dead.json"), r#"{"dead":[]}"#).unwrap();
    fs::write(nofield.join("candidates.json"), r#"{"candidates":[]}"#).unwrap();
    let (code, out, _) = run(&["check", "--pack", &nofield.to_string_lossy(), &doc]);
    assert_eq!(code, 2, "清单缺字段须 2，out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert!(rep["error"].as_str().unwrap().contains("术语包清单缺字段"));

    // 目标缺席：退出码 2
    let (code, out, _) = run(&["check", "--pack", &pa, &target(&ws, "nope.md")]);
    assert_eq!(code, 2, "目标缺席须 2");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert!(rep["error"].as_str().unwrap().contains("目标不存在"));

    // query 缺 --word：用法错退出码 2
    let (code, _, err) = run(&["query", "--pack", &pa]);
    assert_eq!(code, 2, "缺 --word 须 2，err={err}");

    // check 缺 --pack：用法错退出码 2
    let (code, _, _) = run(&["check", &doc]);
    assert_eq!(code, 2, "缺 --pack 须 2");

    // 缺子命令：用法错退出码 2
    let (code, _, _) = run(&[]);
    assert_eq!(code, 2, "缺子命令须 2");

    // register 写面：申报未实装，退出码 2 错误 JSON
    let (code, out, _) = run(&["register", "--pack", &pa]);
    assert_eq!(code, 2, "register 未实装须 2");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert!(
        rep["error"].as_str().unwrap().contains("未实装"),
        "error 须申报未实装，out={out}"
    );
}

// ---------- 边界形：空文档零命中、unknown 态、排除跳过、quiet 紧凑行 ----------

#[test]
fn t3_edge_zero_hit_excluded_quiet() {
    let ws = build_ws();
    let pa = pack_arg(&ws);

    // 空文档：零命中零违例，退出码 0
    let empty = target(&ws, "empty.md");
    fs::write(&empty, "").unwrap();
    let (code, out, _) = run(&["check", "--pack", &pa, &empty]);
    assert_eq!(code, 0, "空文档零命中须 0，out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"], serde_json::json!({"checked":1,"skipped":0,"findings":0}));
    assert_eq!(rep["findings"].as_array().unwrap().len(), 0);

    // query unknown 态：查无出 unknown，退出码 0
    let (code, out, _) = run(&["query", "--pack", &pa, "--word", "查无此词"]);
    assert_eq!(code, 0, "out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["state"], "unknown");
    assert_eq!(rep["hits"], serde_json::json!([]));

    // 排除域跳过：**/skip/** 模式命中即 skipped 不核查，退出码 0
    let skip_dir = ws.dir.join("skip");
    fs::create_dir_all(&skip_dir).unwrap();
    let skipped_doc = skip_dir.join("s.md");
    fs::write(&skipped_doc, "筛分与懒波词俱在\n").unwrap();
    let (code, out, _) = run(&["check", "--pack", &pa, &skipped_doc.to_string_lossy()]);
    assert_eq!(code, 0, "排除域零违例须 0，out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["summary"], serde_json::json!({"checked":0,"skipped":1,"findings":0}));
    assert_eq!(rep["skipped"].as_array().unwrap().len(), 1);

    // --quiet 紧凑行：violation 与 clean 两态
    let doc = target(&ws, "q.md");
    fs::write(&doc, "筛分在此\n").unwrap();
    let (code, out, _) = run(&["check", "--pack", &pa, "--quiet", &doc]);
    assert_eq!(code, 1);
    let raw = out.trim();
    assert!(!raw.contains('\n'), "quiet 须单行，got={raw}");
    let line: Value = serde_json::from_str(raw).unwrap();
    assert_eq!(line["tool"], "nomenclator");
    assert_eq!(line["command"], "check");
    assert_eq!(line["code"], 1);
    assert_eq!(line["summary"]["status"], "violation");
    assert_eq!(line["summary"]["findings"], 1);

    fs::write(&doc, "干净行\n").unwrap();
    let (code, out, _) = run(&["check", "--pack", &pa, "--quiet", &doc]);
    assert_eq!(code, 0);
    let line: Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(line["code"], 0);
    assert_eq!(line["summary"]["status"], "clean");

    // map 空串概念：零近邻零裁决恒零
    let (code, out, _) = run(&["map", "--pack", &pa, "--concept", "   "]);
    assert_eq!(code, 0, "out={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["concept"], "");
    assert_eq!(rep["six_states"]["state"], "unknown");
    assert_eq!(rep["neighbors"].as_array().unwrap().len(), 0);
}
