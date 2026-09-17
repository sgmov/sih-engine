//! wengumcp-parallel 批 canonical 新城域检索集成测试：布局判别、城档五映射、
//! 内嵌记忆包索引、两根分离 locator 祖先回溯、--root 显式拒面。
//! 夹具落真工作区 target/ 下（gitignored），借祖先 sih-tools/locator 码根跑真 locator。

use std::fs;
use std::path::{Path, PathBuf};

use sih_engine::retriever::{derive_root, layout_form, recall, Layout, RecallArgs};

/// canonical 城夹具：真工作区 target/ 下建 sih/ 树，四档各一件材料加链事件一宗。
fn canonical_fixture() -> PathBuf {
    let root = derive_root(Path::new(env!("CARGO_MANIFEST_DIR"))).expect("真工作区根");
    let dir = root
        .join("target")
        .join(format!("wengumcp-canonical-fixture-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("sih/ledger")).unwrap();
    fs::create_dir_all(dir.join("sih/event/trail")).unwrap();
    fs::create_dir_all(dir.join("sih/event/plan")).unwrap();
    fs::create_dir_all(dir.join("sih/state/plan")).unwrap();
    fs::create_dir_all(dir.join("sih/state/parking/materials")).unwrap();
    fs::write(
        dir.join("sih/event/trail/2026-09-11.ndjson"),
        concat!(
            r#"{"event_id":"ev-city-0000-0000-000000000001","event_type":"certification_completed","timestamp":"2026-09-11T00:00:00Z","actor":{"actor_id":"fixture","actor_type":"agent","invoked_via":"test-fixture"},"details":{"package":"cityfix-solo","session_id":"city0001"},"doc_id":"doc-cityfix","prev_hash":"0000000000000000000000000000000000000000000000000000000000000000","event_hash":"a1b2c3d4"}"#,
            "\n",
        ),
    )
    .unwrap();
    fs::write(
        dir.join("sih/event/plan/cityfix-solo-results.md"),
        "- 城档检索结论：canonical 夹具结论档命中行，锚词 wengucity\n",
    )
    .unwrap();
    fs::write(
        dir.join("sih/state/plan/cityfix-solo.md"),
        "# cityfix-solo 任务包\n意图载体行：城档检索锚词 wengucity\n",
    )
    .unwrap();
    fs::write(
        dir.join("sih/state/parking/materials/pk-901.json"),
        r#"{"entry_id":"pk-901","title":"城档检索锚词 wengucity 停泊件"}"#,
    )
    .unwrap();
    dir
}

/// T-1 布局判别：canonical 夹具 layout_form Canonical 且 derive_root 自子目录命中城根。
#[test]
fn t1_layout_form_and_derive() {
    let city = canonical_fixture();
    assert_eq!(layout_form(&city), Some(Layout::Canonical));
    assert_eq!(
        derive_root(&city.join("sih/state/plan")),
        Some(city.clone())
    );
    // 真中央根仍判 first_domain 零回归
    let central = derive_root(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    assert_eq!(layout_form(&central), Some(Layout::FirstDomain));
}

/// T-2 城档索引与映射：word 轴检索命中结论档与意图档与悬置档三档。
#[test]
fn t2_city_archives_indexed_and_classified() {
    let city = canonical_fixture();
    let args = RecallArgs {
        root: city.clone(),
        topics: Vec::new(),
        events: Vec::new(),
        since: None,
        until: None,
        words: vec!["wengucity".to_string()],
        miss_log: None,
        semantic: None,
        archives: Vec::new(),
        at: "2026-09-11".to_string(),
    };
    let rows = recall(&args).expect("canonical recall 成功（locator 祖先回溯）");
    let mut conclusion = false;
    let mut intent = false;
    let mut parked = false;
    for r in &rows {
        match r.archive {
            sih_engine::retriever::Archive::Conclusion => conclusion = true,
            sih_engine::retriever::Archive::Intent => intent = true,
            sih_engine::retriever::Archive::Parked => parked = true,
            _ => {}
        }
    }
    assert!(conclusion, "结论档未命中，rows={rows:?}");
    assert!(intent, "意图档未命中，rows={rows:?}");
    assert!(parked, "悬置档未命中，rows={rows:?}");
}

/// T-3 城链加载：canonical 形 trail 目录取本域 sih/event/trail，事件轴命中归事实档。
#[test]
fn t3_city_chain_event_axis() {
    let city = canonical_fixture();
    let args = RecallArgs {
        root: city,
        topics: Vec::new(),
        events: vec!["certification_completed".to_string()],
        since: None,
        until: None,
        words: Vec::new(),
        miss_log: None,
        semantic: None,
        archives: Vec::new(),
        at: "2026-09-11".to_string(),
    };
    let rows = recall(&args).expect("事件轴 recall 成功");
    assert!(
        rows.iter().any(|r| r.archive == sih_engine::retriever::Archive::Fact),
        "事实档事件未命中，rows={rows:?}"
    );
}

/// T-4 降级承袭：非布局两形根 lib recall 回落 first_domain 路径由既有缺席报错
/// 如实显形（MissingBase locator，F-8 语义零变）；严判只在 bin --root 显式面。
#[test]
fn t4_non_layout_root_legacy_degradation() {
    let tmp = std::env::temp_dir().join(format!("wengumcp-bare-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(tmp.join("ai-ex")).unwrap();
    let args = RecallArgs {
        root: tmp,
        topics: vec!["x".to_string()],
        events: Vec::new(),
        since: None,
        until: None,
        words: Vec::new(),
        miss_log: None,
        semantic: None,
        archives: Vec::new(),
        at: "2026-09-11".to_string(),
    };
    let err = recall(&args).expect_err("缺 locator 底座须报缺席");
    assert!(matches!(
        err,
        sih_engine::retriever::RecallError::MissingBase(_)
    ));
}
