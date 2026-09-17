//! retrieverline 批（rl-04）同参双跑对表：引擎语义通道与工具壳 wikirecall 判词一致。
//! 正典指针：retrieverline 批立项包（sih-engine/sih/state/plan/retrieverline.md）。
//!
//! 形制承 mcpdual-parallel 对表先例：同一语料（dualrun/corpus 十二件）与同一查询
//! （八件，覆盖词面命中型三、语义近邻型三、零命中型二）双跑两实现。引擎侧即本测
//! 试（#[ignore]，实跑 cargo test --test retrline_dualrun -- --ignored 出
//! engine-results.json）；工具壳侧即同目录 run_semantic_side.py（import
//! sih-tools/wikirecall/semantic 跑同语料同参）；判词口径机械可查即 dualrun-summary.json：
//! top-k 集合全等且逐位得分差 ≤ 1e-9 方判一致。本件只产引擎侧读数与核对在档读数，
//! 零裁决；不一致如实入 summary 禁美化。
//!
//! 引擎语料面说明：对表语料取整档文本为文档面（每 md 件一文档），键即文件 stem；
//! 引擎生产路径（recall 内 entry.text 为面）由 lib 测试承载，对表位只验算法一致。

use std::collections::BTreeMap;
use std::path::PathBuf;

use sih_engine::retriever::semantic;

const K: usize = 3;

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("sih/event/plan/retrline-materials/dualrun/corpus")
}

fn dualrun_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("sih/event/plan/retrline-materials/dualrun")
}

fn queries() -> Vec<(&'static str, &'static str)> {
    vec![
        // 词面命中型三：查询词与目标文档共享鉴别词元，直击即中。
        ("租约 生命周期", "lexical-hit"),
        ("句读 解析", "lexical-hit"),
        ("哈希 链 认证", "lexical-hit"),
        // 语义近邻型三：无标题词直击，凭共现词元正分近邻排序。
        ("并发 控制 死锁", "semantic-near"),
        ("留痕 追溯 不可改", "semantic-near"),
        ("三路 路由 判定", "semantic-near"),
        // 零命中型二：全词元与语料零交集，双实现俱空手。
        ("量子 色动力学 夸克", "zero-hit"),
        ("巧克力 火锅 配方", "zero-hit"),
    ]
}

fn load_docs() -> BTreeMap<String, String> {
    let mut docs = BTreeMap::new();
    let mut entries: Vec<_> = std::fs::read_dir(corpus_dir())
        .expect("corpus 目录可开")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |x| x == "md"))
        .collect();
    entries.sort();
    for path in entries {
        let stem = path
            .file_stem()
            .expect("md 件有 stem")
            .to_string_lossy()
            .into_owned();
        let text = std::fs::read_to_string(&path).expect("语料件可读");
        docs.insert(stem, text);
    }
    docs
}

/// rl-04 引擎侧出数：同语料同参跑引擎语义通道写 engine-results.json 供工具壳
/// 侧 harness 对表。实跑：cargo test --test retrline_dualrun -- --ignored --nocapture
#[test]
#[ignore]
fn dualrun_engine_semantic_side_writes_results() {
    let docs = load_docs();
    assert_eq!(docs.len(), 12, "语料十二件");
    let mut rows: Vec<serde_json::Value> = Vec::new();
    for (q, kind) in queries() {
        let hits = semantic::semantic_channel(&docs, &[q.to_string()], K);
        let hit_rows: Vec<serde_json::Value> = hits
            .iter()
            .map(|(id, score)| serde_json::json!({ "id": id, "score": score }))
            .collect();
        rows.push(serde_json::json!({ "query": q, "hit_type": kind, "hits": hit_rows }));
    }
    let report = serde_json::json!({
        "schema": "retrline-dualrun-engine/1",
        "date": "2026-09-18",
        "engine_side": "sih-engine src/retriever/semantic.rs semantic_channel",
        "corpus": "sih/event/plan/retrline-materials/dualrun/corpus",
        "k": K,
        "queries": rows,
    });
    let out = dualrun_dir().join("engine-results.json");
    std::fs::write(&out, serde_json::to_string_pretty(&report).unwrap() + "\n")
        .expect("engine-results.json 可写");
    println!("engine side written: {}", out.display());
}

/// rl-04 在档读数核对（随例行测试跑，零外部依赖）：在档 engine-results.json 与
/// 现行语料与现行查询集与现行引擎读数逐位一致，防语料与查询漂移后记录档失效。
#[test]
fn dualrun_recorded_artifacts_match_current() {
    let recorded = std::fs::read_to_string(dualrun_dir().join("engine-results.json"))
        .expect("在档 engine-results.json 可读（随批入版控）");
    let v: serde_json::Value = serde_json::from_str(&recorded).expect("在档件可解析");
    assert_eq!(v["schema"], "retrline-dualrun-engine/1");
    assert_eq!(v["k"], K);
    let docs = load_docs();
    let qs = v["queries"].as_array().expect("queries 数组").clone();
    assert_eq!(qs.len(), queries().len(), "查询数零漂移");
    for (row, (q, _kind)) in qs.iter().zip(queries()) {
        assert_eq!(row["query"], q, "查询序零漂移");
        let hits = semantic::semantic_channel(&docs, &[q.to_string()], K);
        let got: Vec<serde_json::Value> = hits
            .iter()
            .map(|(id, score)| serde_json::json!({ "id": id, "score": score }))
            .collect();
        assert_eq!(
            row["hits"].as_array().map(|a| a.as_slice()).unwrap_or(&[]),
            got.as_slice(),
            "查询 {q} 引擎读数与在档不一致"
        );
    }
}
