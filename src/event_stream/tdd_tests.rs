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
            actor_id: "scribe".to_string(),
            actor_type: ActorType::System,
            invoked_via: "cli".to_string(),
        }
    }

    fn temp_trail(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("scribe-{tag}-{}", std::process::id()));
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

    // T2 生产复验：在场文件全量 valid 且目录非空。
    // 恰数断言已除即工具书简退役后日文件数随退役批迁移浮动，死数与本意无关，
    // 存量红定性见 lockguard-solo 结果档即主线未改源同红。
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
        assert!(!files.is_empty(), "生产 trail 至少一件");
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
        assert!(again.is_err(), "出泊后号源唯一拒不可再入，承 entryunique-solo 修订二");
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

    // T6 scribe 退出码三值即本名回滚后。
    #[test]
    fn t6_scribe_exit_codes() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bin = manifest.join("target/debug/scribe");
        if !bin.exists() {
            panic!("scribe 未构建");
        }
        let trail = temp_trail("cli");
        let _ = std::fs::remove_file(&trail);
        let ok = Command::new(&bin).args(["verify", "--trail", "/nonexistent.ndjson"]).output().unwrap();
        assert_eq!(ok.status.code(), Some(2), "trail 不存在即工具异常二");
        let report = json!({"engine": {"name": "f", "version": "0"}, "packs": [], "content_hashes": {}, "findings": [], "golden_baseline": "0", "tool": {"name": "f", "version": "0"}}).to_string();
        let rpath = std::env::temp_dir().join(format!("scribe-rep-{}.json", std::process::id()));
        std::fs::write(&rpath, &report).unwrap();
        // 闸三会话在册验：append 认证需活跃会话，先建会话台账在册再验证写成功零。
        let sessions = temp_trail("sesscli");
        let _ = std::fs::remove_file(&sessions);
        let sid = format!("sess-t6-{}", std::process::id());
        std::fs::write(&sessions, format!("{{\"event\":\"issued\",\"session_id\":\"{sid}\"}}\n")).unwrap();
        let good = Command::new(&bin).args([
            "append", "--report", rpath.to_str().unwrap(), "--exit-code", "0",
            "--trail", trail.to_str().unwrap(),
            "--session", &sid, "--sessions", sessions.to_str().unwrap(),
        ]).output().unwrap();
        assert_eq!(good.status.code(), Some(0), "写入成功零");
        let v = Command::new(&bin).args(["verify", "--trail", trail.to_str().unwrap()]).output().unwrap();
        assert_eq!(v.status.code(), Some(0), "链校验通过零");
        let q = Command::new(&bin).args(["query", "--trail", trail.to_str().unwrap(), "--doc-id", "no-such"]).output().unwrap();
        assert_eq!(q.status.code(), Some(1), "查无一");
    }

    // ---- 防分叉护栏（guardrail-solo 批）----
    // 四写入入口 --trail 路径含 worktrees/ 段即拒退出码二载错文，
    // 显式覆写旗标 --allow-worktree-trail 默认关，主树路径行为零回退。
    // 构造工地装饰链路径与合法报告件，逐入口实测拒证与覆写放行。

    fn worktree_trail() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "worktrees/sih-engine/{}-trail",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("2026-09-03.ndjson")
    }

    fn cert_report() -> PathBuf {
        let p = std::env::temp_dir().join(format!("guard-cert-{}.json", std::process::id()));
        let report = json!({"engine": {"name": "f", "version": "0"}, "packs": [], "content_hashes": {}, "findings": [], "golden_baseline": "0", "tool": {"name": "f", "version": "0"}});
        if !p.exists() {
            std::fs::write(&p, report.to_string()).unwrap();
        }
        p
    }

    fn intent_record() -> PathBuf {
        let p = std::env::temp_dir().join(format!("guard-record-{}.json", std::process::id()));
        if !p.exists() {
            let rec = json!({
                "session_id": "sess-guard-test", "round": 1,
                "intent_contract": {"goal": "x", "exclusions": [], "output_format": "x", "injected_constraints": []},
                "domain_contract": {"target_domain": {"scope": "x", "max_depth": 1}, "support_domain": {"scope": "x", "max_depth": 1}},
                "anchors": [
                    {"anchor_seq": 1, "text": "a", "inquiry_stage": "first", "domain_tag": "target", "depth": 0,
                     "philosophy_ref": {"source": "sih-philosophy/emanation/proodos/07-on-assay.md", "quote": "q"}, "rationale": "r", "evidence": "e", "confidence": 0.9}
                ],
                "calls_in": 0, "calls_out": 0, "elicitation": {"signals_file": "x", "signals": 0, "disposition": []}
            });
            std::fs::write(&p, rec.to_string()).unwrap();
        }
        p
    }

    fn valid_rec() -> PathBuf {
        let p = std::env::temp_dir().join(format!("guard-valid-{}.json", std::process::id()));
        if !p.exists() {
            std::fs::write(&p, r#"{"status":"ok","anchor_count":1}"#).unwrap();
        }
        p
    }

    // G-A1 append 工地链副本拒退出码二载错文。
    #[test]
    fn ga1_append_worktree_trail_rejected() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bin = manifest.join("target/debug/scribe");
        let wt = worktree_trail();
        let out = Command::new(&bin)
            .args(["append", "--report", cert_report().to_str().unwrap(), "--exit-code", "0", "--trail", wt.to_str().unwrap()])
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(2), "工地链副本 append 应拒退出码二");
        let body = String::from_utf8_lossy(&out.stdout);
        assert!(body.contains("工地链副本禁追加即认证先落主链"), "错文应载明护栏：{body}");
    }

    // G-A2 record 工地链副本拒退出码二。
    #[test]
    fn ga2_record_worktree_trail_rejected() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bin = manifest.join("target/debug/scribe");
        let wt = worktree_trail();
        let reading = std::env::temp_dir().join(format!("guard-reading-{}.json", std::process::id()));
        let rd = r#"{"dimension":"convergence","subject":"sih-engine","value":0.5,"window":"2026-08-01/2026-08-30","formula_version":"ga-1","computed_at":"2026-08-30","inputs_digest":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#;
        let _ = std::fs::remove_file(&reading);
        std::fs::write(&reading, rd).unwrap();
        let out = Command::new(&bin)
            .args(["record", "--reading", reading.to_str().unwrap(), "--trail", wt.to_str().unwrap()])
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(2), "工地链副本 record 应拒退出码二");
        let body = String::from_utf8_lossy(&out.stdout);
        assert!(body.contains("工地链副本禁追加即认证先落主链"), "错文应载明护栏：{body}");
        // 覆写旗标放行：默认关，显式开即过护栏。
        let allow = Command::new(&bin)
            .args(["record", "--reading", reading.to_str().unwrap(), "--trail", wt.to_str().unwrap(), "--allow-worktree-trail", "1"])
            .output()
            .unwrap();
        assert_eq!(allow.status.code(), Some(0), "显式覆写旗标应放行");
        let _ = std::fs::remove_file(&reading);
    }

    // G-A3 intent 工地链副本拒退出码二。
    #[test]
    fn ga3_intent_worktree_trail_rejected() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bin = manifest.join("target/debug/scribe");
        let wt = worktree_trail();
        let out = Command::new(&bin)
            .args(["intent", "--record", intent_record().to_str().unwrap(), "--validation", valid_rec().to_str().unwrap(), "--trail", wt.to_str().unwrap()])
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(2), "工地链副本 intent 应拒退出码二");
        let body = String::from_utf8_lossy(&out.stdout);
        assert!(body.contains("工地链副本禁追加即认证先落主链"), "错文应载明护栏：{body}");
    }

    // G-A4 park 工地链副本拒退出码二。
    #[test]
    fn ga4_park_worktree_trail_rejected() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bin = manifest.join("target/debug/scribe");
        let wt = worktree_trail();
        let rec = std::env::temp_dir().join(format!("guard-park-{}.json", std::process::id()));
        let rp = json!({"action": "enter", "entry_id": "g-test", "title": "t", "exit_condition": "c", "ttl_days": 3});
        let _ = std::fs::remove_file(&rec);
        std::fs::write(&rec, rp.to_string()).unwrap();
        let out = Command::new(&bin)
            .args(["park", "--record", rec.to_str().unwrap(), "--trail", wt.to_str().unwrap()])
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(2), "工地链副本 park 应拒退出码二");
        let body = String::from_utf8_lossy(&out.stdout);
        assert!(body.contains("工地链副本禁追加即认证先落主链"), "错文应载明护栏：{body}");
        let _ = std::fs::remove_file(&rec);
    }

    // T7 双进程并发追加竞测（basefix-solo 批 F-2，先红后绿）。
    // 两进程对同一链文件各追 N 笔，修后两进程事件全数在链且 verify valid。
    // 修复前读算追加无锁窗口：两进程同尾算 prev_hash 双写即链分叉 verify 破。
    #[test]
    fn t7_concurrent_append_race_two_processes() {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bin = manifest.join("target/debug/scribe");
        if !bin.exists() {
            panic!("scribe 未构建");
        }
        let trail = temp_trail("race2");
        let _ = std::fs::remove_file(&trail);
        let sessions = temp_trail("race-sess");
        let _ = std::fs::remove_file(&sessions);
        let sid = format!("sess-t7-{}", std::process::id());
        std::fs::write(&sessions, format!("{{\"event\":\"issued\",\"session_id\":\"{sid}\"}}\n")).unwrap();
        let dir = temp_trail("race-reports");
        std::fs::create_dir_all(&dir).unwrap();
        let n = 6;
        for i in 0..n {
            let report = json!({
                "engine": {"name": "f", "version": "0"}, "packs": [], "content_hashes": {},
                "findings": [], "golden_baseline": "0", "tool": {"name": "f", "version": "0"}
            });
            std::fs::write(dir.join(format!("rep-{i}.json")), report.to_string()).unwrap();
        }
        let script = format!(
            r#"for i in $(seq 0 {}); do "$1" append --report "$2/rep-$i.json" --exit-code 0 --trail "$3" --session "$4" --sessions "$5" || exit 9; sleep 0.01; done"#,
            n - 1
        );
        let mut handles = Vec::new();
        for _p in 0..2 {
            let bin_c = bin.clone();
            let dir_c = dir.clone();
            let trail_c = trail.clone();
            let sess_c = sessions.clone();
            let sid_c = sid.clone();
            let script_c = script.clone();
            handles.push(std::thread::spawn(move || {
                Command::new("sh")
                    .arg("-c")
                    .arg(&script_c)
                    .arg("probe")
                    .arg(bin_c)
                    .arg(dir_c)
                    .arg(trail_c)
                    .arg(sid_c)
                    .arg(sess_c)
                    .output()
                    .unwrap()
            }));
        }
        for h in handles {
            let out = h.join().unwrap();
            assert!(
                out.status.success(),
                "子进程败: {} {}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
        }
        let events = load_events(&trail).unwrap();
        assert_eq!(events.len(), 2 * n, "两进程事件全数在链");
        verify(&events, VerifyRange::Full).unwrap_or_else(|e| panic!("链校验败 {e:?}"));
    }
}
