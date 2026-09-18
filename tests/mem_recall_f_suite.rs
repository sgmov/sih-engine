//! 项目记忆实装批 F 锚定集成测试，承接 SPEC-007 验收判据与 SPEC-008 测试计划。
//!
//! 真实工作区材料跑 F-1 至 F-8 与 F-10，红态即桩入口 todo 宏，绿态即实装完成。
//!
//! testhard 批（2026-09-18）：F-7 双跑输入快照冻结加固（件一），病灶申报出处
//! defectwave 波后全量回归记录——全量跑窗口内当日链被并发会话合法追加，
//! 活 trail 背靠背双跑假红；详见 f7 测试体注记。

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_json::Value;

use sih_engine::event_stream::{query as chain_query, EventFilter};
use sih_engine::retriever::{
    derive_root, load_chain, locator_bridge, recall, rows_to_ndjson, Archive, RecallArgs,
    RecallError,
};

fn real_root() -> PathBuf {
    derive_root(Path::new(env!("CARGO_MANIFEST_DIR"))).expect("工作区根定位")
}

fn args_real(topics: &[&str], events: &[&str], since: Option<&str>, until: Option<&str>) -> RecallArgs {
    RecallArgs {
        root: real_root(),
        topics: topics.iter().map(|s| s.to_string()).collect(),
        events: events.iter().map(|s| s.to_string()).collect(),
        since: since.map(|s| s.to_string()),
        until: until.map(|s| s.to_string()),
        words: Vec::new(),
        miss_log: None,
        semantic: None,
        archives: Vec::new(),
        at: "2026-08-27T19:30:00+08:00".to_string(),
    }
}

/// F-1 五档覆盖即五档各至少一条真实材料命中。
#[test]
fn f1_five_archive_coverage() {
    let rows = recall(&args_real(&["pk-024", "recall"], &["certification_completed"], None, None))
        .expect("recall 成功");
    let seen: BTreeSet<Archive> = rows.iter().map(|r| r.archive).collect();
    for want in [
        Archive::Fact,
        Archive::Conclusion,
        Archive::Experience,
        Archive::Parked,
        Archive::Intent,
    ] {
        assert!(seen.contains(&want), "缺档 {want:?}，实见 {seen:?}");
    }
}

/// F-2 出处可机械回验即 md 行区间重开比对、json 标识重建直取、事件哈希在链。
#[test]
fn f2_refs_machine_verifiable() {
    let root = real_root();
    let rows = recall(&args_real(&["pk-024", "recall"], &["certification_completed"], None, None))
        .expect("recall 成功");
    assert!(!rows.is_empty());
    let index = locator_bridge::build_index(&root).expect("建索引");
    let chain = load_chain(&root).expect("链加载");
    let mut md_n = 0;
    let mut json_n = 0;
    let mut event_n = 0;
    for row in &rows {
        match row.carrier {
            "md" => {
                let (path_part, range) = row.reference.rsplit_once('@').expect("md ref 形态");
                let (ls, le) = range.split_once('-').expect("行区间形态");
                let ls: usize = ls.parse().unwrap();
                let le: usize = le.parse().unwrap();
                let text = std::fs::read_to_string(root.join(path_part)).expect("md 开档");
                let lines: Vec<&str> = text.lines().collect();
                assert!(le >= ls && ls >= 1 && le <= lines.len(), "行区间越界 {}", row.reference);
                let window = lines[ls - 1..le].join("\n");
                if row.excerpt.ends_with("...") {
                    let head = row.excerpt.trim_end_matches('.');
                    assert!(window.contains(head), "摘录截断前段不在行区间 {}", row.reference);
                } else {
                    assert!(window.contains(&row.excerpt), "摘录不在行区间 {}", row.reference);
                }
                md_n += 1;
            }
            "json" => {
                let (path_part, id) = row.reference.split_once('@').expect("json ref 形态");
                let hit = locator_bridge::query_by_id(&root, &index, id).expect("标识直取");
                let hit = hit.expect("json 条目可复原");
                assert_eq!(hit.path, path_part);
                assert_eq!(hit.id, id);
                json_n += 1;
            }
            "event" => {
                let (event_id, hash8) = row.reference.split_once('/').expect("事件 ref 形态");
                assert_eq!(hash8.len(), 8);
                let hit = chain.iter().find(|e| e.event_id == event_id).expect("事件在链");
                assert!(hit.event_hash.starts_with(hash8), "哈希前缀不符 {}", row.reference);
                event_n += 1;
            }
            other => panic!("未知载体 {other}"),
        }
    }
    assert!(md_n > 0 && json_n > 0 && event_n > 0, "三载体各须有覆盖 md={md_n} json={json_n} event={event_n}");
}

/// F-3 主题轴跨档即同一主题词跨至少三档返回切面。
#[test]
fn f3_topic_cross_archive() {
    let rows = recall(&args_real(&["pk-024"], &[], None, None)).expect("recall 成功");
    let file_archives: BTreeSet<Archive> = rows
        .iter()
        .filter(|r| r.carrier != "event")
        .map(|r| r.archive)
        .collect();
    assert!(
        file_archives.len() >= 3,
        "主题词跨档不足三档，实见 {file_archives:?}"
    );
}

/// F-4 事件轴正确即按事件类型过滤与 query 位直查一致，事实与意图两路。
#[test]
fn f4_event_axis_matches_query() {
    let root = real_root();
    let store = load_chain(&root).expect("链加载");
    for (token, want_archive) in [("intent_refined", Archive::Intent), ("certification_completed", Archive::Fact)] {
        let rows = recall(&args_real(&[], &[token], None, None)).expect("recall 成功");
        let got: BTreeSet<String> = rows
            .iter()
            .filter(|r| r.axis.as_str() == "event")
            .map(|r| r.reference.split('/').next().unwrap().to_string())
            .collect();
        assert!(rows.iter().all(|r| r.archive == want_archive), "{token} 档属须全为 {want_archive:?}");
        let direct = chain_query(
            &store,
            Some(EventFilter {
                event_type: Some(token.to_string()),
                ..Default::default()
            }),
            None,
        );
        let expect: BTreeSet<String> = direct.events.iter().map(|e| e.event_id.clone()).collect();
        assert_eq!(got, expect, "{token} 与 query 位直查不一致");
    }
}

/// F-5 时间轴边界即真实链单日过滤全落当日，边界含即含由单测合成事件承载。
#[test]
fn f5_time_axis_day_filter() {
    let rows = recall(&args_real(&[], &[], Some("2026-08-27"), Some("2026-08-27"))).expect("recall 成功");
    let time_rows: Vec<_> = rows.iter().filter(|r| r.axis.as_str() == "time").collect();
    assert!(!time_rows.is_empty(), "当日链有时间轴命中");
    for row in &time_rows {
        let ts = chrono::DateTime::parse_from_rfc3339(&row.matched).expect("时间戳原文可解析");
        let local = ts.with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap());
        assert_eq!(local.format("%Y-%m-%d").to_string(), "2026-08-27", "越日事件 {}", row.reference);
    }
}

/// F-6 只报不判即输出行键集恰七字段且键序以 archive 起，值域小写枚举。
#[test]
fn f6_no_judgment_fields() {
    let rows = recall(&args_real(&["pk-024"], &["certification_completed"], None, None)).expect("recall 成功");
    let ndjson = rows_to_ndjson(&rows);
    assert!(!ndjson.is_empty());
    for line in ndjson.lines() {
        assert!(line.starts_with(r#"{"archive":"#), "键序须以 archive 起：{line}");
        let v: Value = serde_json::from_str(line).expect("行可解析");
        let keys: BTreeSet<&str> = v.as_object().unwrap().keys().map(|k| k.as_str()).collect();
        assert_eq!(
            keys,
            BTreeSet::from(["archive", "carrier", "ref", "axis", "excerpt", "matched", "at"]),
            "键集须恰七字段"
        );
        let archive = v["archive"].as_str().unwrap();
        assert!(
            ["fact", "conclusion", "experience", "parked", "intent"].contains(&archive),
            "档名值域小写枚举，实见 {archive}"
        );
        let axis = v["axis"].as_str().unwrap();
        assert!(
            ["topic", "event", "time"].contains(&axis),
            "轴名值域小写枚举，实见 {axis}"
        );
        let carrier = v["carrier"].as_str().unwrap();
        assert!(["md", "json", "event"].contains(&carrier), "载体值域，实见 {carrier}");
    }
}

/// F-7 确定性即同参双跑逐字节一致。
///
/// testhard 批件一加固。病灶申报出处：defectwave 波后全量回归记录（2026-09-18）
/// ——全量跑 349 秒窗口内当日链被并发会话合法追加，旧形活 trail 背靠背双跑
/// 之间链面变行即假红（单跑必绿，2026-09-18 实证 301.95 秒绿）。修法把双跑
/// 输入冻结：trail（事件轴竞态面）与 canonical 内嵌记忆包 include 语料面
///（主题轴输入）快照拷贝进临时 canonical 城形根，两跑同指快照断言逐字节
/// 一致；recall 真代码路径零冻结（locator 子进程、链加载、排序、序列化全实
/// 跑）。真 trail 冒烟形弃留申报：排序行集等断言在跑间追加场景仍假红（追加即增行、
/// 集合必变），冻结形是唯一对竞态干净的双跑形。红证机制复现：活链双跑跑间
/// 追一行即逐字节断言红、同场景快照形绿（testhard 批 scratch 实测留汇报）。
#[test]
fn f7_double_run_identical() {
    let snap = f7_snapshot_root().expect("快照根构建");
    let args_at_snap = || RecallArgs {
        root: snap.path().to_path_buf(),
        topics: vec!["pk-024".to_string(), "recall".to_string()],
        events: vec!["certification_completed".to_string()],
        since: None,
        until: None,
        words: Vec::new(),
        miss_log: None,
        semantic: None,
        archives: Vec::new(),
        at: "2026-08-27T19:30:00+08:00".to_string(),
    };
    let a = rows_to_ndjson(&recall(&args_at_snap()).expect("首跑"));
    let b = rows_to_ndjson(&recall(&args_at_snap()).expect("再跑"));
    assert!(!a.is_empty(), "快照双跑输出非空");
    assert_eq!(a, b, "同参双跑（冻结输入）须逐字节一致");
    // 双轴皆在面：事件轴证 trail 快照承载真链材料，主题轴证语料快照在位。
    let axes: BTreeSet<String> = a
        .lines()
        .filter_map(|l| {
            serde_json::from_str::<Value>(l)
                .ok()
                .and_then(|v| v["axis"].as_str().map(|s| s.to_string()))
        })
        .collect();
    assert!(
        axes.contains("event") && axes.contains("topic"),
        "事件轴与主题轴双在面，实见 {axes:?}"
    );
}

/// F-7 快照根：canonical 城形临时根，收容 recall 全部只读输入。
///
/// 形制择定申报：快照根取 canonical 城形（sih/ledger 标记），非 first_domain
/// 镜像——first_domain 形的 locator 码根=root/sih-tools/locator 须整仓拷贝
/// 或 symlink，而 locator pyproject 带 "../parser" 相对路径依赖，symlink 形
/// uv 解析必败（status=2 实证）；canonical 形 locator 码根经祖先上溯定位真
/// sih-tools/locator（零 symlink 零拷贝，uv 子进程照实跑）。落位居
/// CARGO_MANIFEST_DIR/target/ 下（祖先链含 sih-tools，构建产物位零工作区
/// 污染），TempDir 随测自清。
///
/// 快照面＝canonical 内嵌记忆包 include 全域（locator_bridge::
/// CANONICAL_MEMORY_PACK）：trail 全量 ndjson（事件轴竞态病灶面）加
/// sih/event/plan/*-results.md 加 sih/state/plan/*.md 加
/// sih/state/parking/materials/*.json 加 sih/state/parking/PARKING-v1.md，
/// 源自 real_root 引擎仓 sih 树。源缺席域静默跳过（快照面可裁，双跑一致性
/// 不受缺席影响）。
fn f7_snapshot_root() -> std::io::Result<tempfile::TempDir> {
    let ws = real_root();
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/f7-snap");
    std::fs::create_dir_all(&base)?;
    let snap = tempfile::Builder::new().prefix("run-").tempdir_in(&base)?;

    // canonical 城形标记与 sih 树骨架（layout_form 判据 root/sih/ledger）。
    std::fs::create_dir_all(snap.path().join("sih/ledger"))?;

    // trail：事件轴输入，竞态病灶面，全量 ndjson 冻结。
    f7_copy_flat(
        &ws.join("sih-engine/sih/event/trail"),
        &snap.path().join("sih/event/trail"),
        &|_| true,
    )?;
    // canonical 内嵌记忆包 include 面：主题轴语料。
    f7_copy_flat(
        &ws.join("sih-engine/sih/event/plan"),
        &snap.path().join("sih/event/plan"),
        &|name| name.ends_with("-results.md"),
    )?;
    f7_copy_flat(
        &ws.join("sih-engine/sih/state/plan"),
        &snap.path().join("sih/state/plan"),
        &|name| name.ends_with(".md"),
    )?;
    f7_copy_flat(
        &ws.join("sih-engine/sih/state/parking/materials"),
        &snap.path().join("sih/state/parking/materials"),
        &|name| name.ends_with(".json"),
    )?;
    let parking_doc = ws.join("sih-engine/doc/governance/PARKING-v1.md");
    if parking_doc.is_file() {
        std::fs::copy(
            &parking_doc,
            snap.path().join("sih/state/parking/PARKING-v1.md"),
        )?;
    }
    Ok(snap)
}

/// 单层按名滤拷贝（pack include 单层 glob 形），源缺席静默跳过。
fn f7_copy_flat(src: &Path, dst: &Path, keep: &dyn Fn(&str) -> bool) -> std::io::Result<()> {
    if !src.is_dir() {
        return Ok(());
    }
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if keep(name) {
            std::fs::copy(entry.path(), dst.join(&name))?;
        }
    }
    Ok(())
}

/// F-8 退化不崩即底座件缺席报缺席件名退出码二，五档载体原样无损。
#[test]
fn f8_degradation_names_missing_base() {
    let tmp = tempfile::tempdir().expect("临时根");
    let root_a = tmp.path().join("no-locator");
    let carriers = root_a.join("ai-ex");
    std::fs::create_dir_all(&carriers).unwrap();
    let sentinel = carriers.join("A.md");
    std::fs::write(&sentinel, "经验档哨兵\n").unwrap();
    let before = std::fs::read(&sentinel).unwrap();

    let err = recall(&RecallArgs {
        root: root_a.clone(),
        topics: vec!["哨兵".to_string()],
        events: vec![],
        since: None,
        until: None,
        words: Vec::new(),
        miss_log: None,
        semantic: None,
        archives: vec![],
        at: "t".to_string(),
    })
    .unwrap_err();
    assert!(matches!(&err, RecallError::MissingBase(name) if name.contains("locator")), "缺席件名 {err:?}");
    assert_eq!(sih_engine::retriever::exit_code(&err), 2);

    let root_b = tmp.path().join("no-trail");
    std::fs::create_dir_all(root_b.join("sih-tools/locator")).unwrap();
    let err = recall(&RecallArgs {
        root: root_b.clone(),
        topics: vec![],
        events: vec!["intent_refined".to_string()],
        since: None,
        until: None,
        words: Vec::new(),
        miss_log: None,
        semantic: None,
        archives: vec![],
        at: "t".to_string(),
    })
    .unwrap_err();
    assert!(matches!(&err, RecallError::TargetUnreadable(p) if p.contains("trail")), "缺席件名 {err:?}");
    assert_eq!(sih_engine::retriever::exit_code(&err), 2);

    assert_eq!(std::fs::read(&sentinel).unwrap(), before, "五档载体原样无损");
}

/// F-10 包档一致即窄域包内文件路径全可归档恰一档。
#[test]
fn f10_pack_archive_consistency() {
    let root = real_root();
    let index = locator_bridge::build_index(&root).expect("建索引");
    let text = std::fs::read_to_string(&index).expect("索引件可读");
    let mut file_count = 0;
    for line in text.lines() {
        let v: Value = serde_json::from_str(line).expect("索引行可解析");
        if v.get("type").and_then(|t| t.as_str()) == Some("file") {
            let path = v.get("path").and_then(|p| p.as_str()).expect("文件记录带路径");
            assert!(
                sih_engine::retriever::archives::classify_path(path).is_some(),
                "包内路径不可归档：{path}"
            );
            file_count += 1;
        }
    }
    assert!(file_count > 150, "窄域包文件数异常 {file_count}");
}
