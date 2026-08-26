//! TDD 六组测试承 SPEC-006 测试计划，先红后绿。

#[cfg(test)]
mod tdd {
    use chrono::Utc;
    use serde_json::json;
    use crate::event_stream::{
        append, certification_event, compute_event_hash, intent_event, load_events, park_event,
        verify, Actor, ActorType, Event, EventInput, VerifyRange,
    };
    use std::path::PathBuf;
    use std::process::Command;



    fn actor() -> Actor {
        Actor {
            actor_id: "scribegate".to_string(),
            actor_type: ActorType::System,
            invoked_via: "cli".to_string(),
        }
    }

    fn temp_trail(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("scribegate-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("trail.ndjson")
    }

    // T1 golden 向量集：五类边界冻结，复算一致。
    #[test]
    fn t1_golden_vectors_recompute() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/golden/event-stream-vectors.json");
        let text = std::fs::read_to_string(&path).expect("向量集存在");
        let vectors: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
        assert!(vectors.len() >= 5, "五类边界至少五向量");
        let mut classes = std::collections::HashSet::new();
        for v in &vectors {
            let event: Event = serde_json::from_value(v["event"].clone()).unwrap();
            classes.insert(v["class"].as_str().unwrap().to_string());
            assert_eq!(
                compute_event_hash(&event),
                v["expected_hash"].as_str().unwrap(),
                "向量 {} 复算不一致",
                v["name"].as_str().unwrap()
            );
        }
        for need in ["genesis", "details_null", "optional_absent", "non_ascii", "timestamp_edge"] {
            assert!(classes.contains(need), "缺边界类 {need}");
        }
    }

    // T2 生产复验：四文件全量 valid。
    #[test]
    fn t2_production_trails_verify() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dir = ["../sih-tools/scribe/trail", "../../../sih-tools/scribe/trail"]
            .iter()
            .map(|c| manifest.join(c))
            .find(|d| d.exists())
            .expect("生产 trail 目录存在");
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .expect("生产 trail 目录存在")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map_or(false, |x| x == "ndjson"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4, "生产 trail 恰四文件");
        for f in &files {
            let events = load_events(f).unwrap();
            let ok = verify(&events, VerifyRange::Full)
                .unwrap_or_else(|e| panic!("{} 校验失败 {e:?}", f.display()));
            assert!(ok.event_count > 0);
        }
    }

    // T3 intent：零发现放行、有发现拒。
    #[test]
    fn t3_intent_zero_findings_passes() {
        let record = json!({
            "session_id": "sess-test-1", "round": 1,
            "anchors": [{"a": 1}, {"a": 2}, {"a": 3}],
            "calls_in": 2, "calls_out": 5,
            "intent_contract": {"goal": "测试意图"}
        }).to_string();
        let validation = json!({
            "findings": [], "packs": [{"name": "ask3", "version": "0.1.0"}],
            "golden_baseline": "0.1.0",
            "tool": {"name": "scrutinator", "version": "0.1.0"}
        }).to_string();
        let input = intent_event(
            PathBuf::from("r.json").as_path(), &record,
            PathBuf::from("v.json").as_path(), &validation,
            actor(), Utc::now(),
        ).expect("零发现放行");
        assert_eq!(input.event_type, "intent_refined");
        assert_eq!(input.doc_id, "sess-test-1");
        let d = input.details.clone().unwrap();
        assert_eq!(d["session_id"], "sess-test-1");
        assert_eq!(d["round"], 1);
        assert_eq!(d["anchor_count"], 3);
        assert_eq!(d["calls_in"], 2);
        assert_eq!(d["calls_out"], 5);
        assert!(d["record_hash"].is_string());
        assert!(d["validation_report_hash"].is_string());
        assert!(d["pack_versions"].is_array());
        assert!(d["goal"].is_string());
    }

    #[test]
    fn t3_intent_findings_rejected() {
        let record = json!({"session_id": "s", "round": 1, "anchors": [], "calls_in": 0, "calls_out": 0, "intent_contract": {"goal": "g"}}).to_string();
        let bad = json!({"findings": [{"rule_id": "A01"}], "packs": [], "tool": {"name": "x", "version": "0"}}).to_string();
        let out = intent_event(PathBuf::from("r").as_path(), &record, PathBuf::from("v").as_path(), &bad, actor(), Utc::now());
        assert!(out.is_err(), "有发现即拒");
    }

    // T4 park：重入拒、无主出拒、出泊后可再入。
    #[test]
    fn t4_park_invariants() {
        let enter = json!({"action": "enter", "entry_id": "pk-1", "title": "t", "exit_condition": "c", "ttl_days": 30}).to_string();
        let e1 = park_event(&enter, &[], actor(), Utc::now()).expect("首次入泊放行");
        let events = vec![as_event(e1)];
        let reenter = park_event(&enter, &events, actor(), Utc::now());
        assert!(reenter.is_err(), "在泊重入拒");
        let exit = json!({"action": "exit", "entry_id": "pk-1", "disposition": "promoted", "ruling": "裁定转正"}).to_string();
        let x = park_event(&exit, &events, actor(), Utc::now()).expect("在泊出泊放行");
        let events2 = vec![events[0].clone(), as_event(x)];
        let orphan = park_event(&exit, &events2, actor(), Utc::now());
        assert!(orphan.is_err(), "无主出拒");
        let again = park_event(&enter, &events2, actor(), Utc::now());
        assert!(again.is_ok(), "出泊后可再入");
    }

    fn as_event(input: EventInput) -> Event {
        // 走 append 真链落盘再读回，复用生产路径。
        let trail = temp_trail("park");
        let _ = std::fs::remove_file(&trail);
        let mut store: Vec<Event> = Vec::new();
        append(input, &mut store, Some(&trail)).unwrap();
        store.remove(0)
    }

    // T5 报告消费：八项负载、重复消费两事件同报告哈希。
    #[test]
    fn t5_certify_payload_and_duplicate() {
        let report = json!({
            "engine": {"name": "scrutinator", "version": "0.1.0"},
            "packs": [{"name": "des-001", "version": "1.0"}],
            "content_hashes": {"doc.md": "abc123"},
            "findings": [],
            "golden_baseline": "0.1.0",
            "tool": {"name": "scrutinator", "version": "0.1.0"}
        }).to_string();
        let i1 = certification_event(PathBuf::from("rep-1.json").as_path(), &report, 0, actor(), Utc::now()).unwrap();
        let i2 = certification_event(PathBuf::from("rep-1.json").as_path(), &report, 0, actor(), Utc::now()).unwrap();
        assert_eq!(i1.event_type, "certification_completed");
        assert_eq!(i1.doc_id, "rep-1");
        let d = i1.details.clone().unwrap();
        for key in ["report_path", "report_hash", "pack_versions", "content_hashes", "finding_count", "exit_code", "tool_version", "golden_baseline"] {
            assert!(d.get(key).is_some(), "负载缺 {key}");
        }
        assert_eq!(d["finding_count"], 0);
        assert_eq!(d["exit_code"], 0);
        let h1 = i2.details.as_ref().unwrap()["report_hash"].clone();
        assert_eq!(d["report_hash"], h1, "重复消费同报告哈希可审计");
        let trail = temp_trail("dup");
        let _ = std::fs::remove_file(&trail);
        let mut store: Vec<Event> = Vec::new();
        append(i1, &mut store, Some(&trail)).unwrap();
        append(i2, &mut store, Some(&trail)).unwrap();
        assert_eq!(store.len(), 2, "重复消费不拒即两独立事件");
        drop(load_events(&trail).unwrap());
    }

    // T6 scribegate 退出码三值。
    #[test]
    fn t6_scribegate_exit_codes() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bin = manifest.join("target/debug/scribegate");
        if !bin.exists() {
            panic!("scribegate 未构建");
        }
        let trail = temp_trail("cli");
        let _ = std::fs::remove_file(&trail);
        let ok = Command::new(&bin).args(["verify", "--trail", "/nonexistent.ndjson"]).output().unwrap();
        assert_eq!(ok.status.code(), Some(2), "trail 不存在即工具异常二");
        let report = json!({"engine": {"name": "f", "version": "0"}, "packs": [], "content_hashes": {}, "findings": [], "golden_baseline": "0", "tool": {"name": "f", "version": "0"}}).to_string();
        let rpath = std::env::temp_dir().join(format!("scribegate-rep-{}.json", std::process::id()));
        std::fs::write(&rpath, &report).unwrap();
        let good = Command::new(&bin).args(["append", "--report", rpath.to_str().unwrap(), "--exit-code", "0", "--trail", trail.to_str().unwrap()]).output().unwrap();
        assert_eq!(good.status.code(), Some(0), "写入成功零");
        let v = Command::new(&bin).args(["verify", "--trail", trail.to_str().unwrap()]).output().unwrap();
        assert_eq!(v.status.code(), Some(0), "链校验通过零");
        let q = Command::new(&bin).args(["query", "--trail", trail.to_str().unwrap(), "--doc-id", "no-such"]).output().unwrap();
        assert_eq!(q.status.code(), Some(1), "查无一");
    }
}
