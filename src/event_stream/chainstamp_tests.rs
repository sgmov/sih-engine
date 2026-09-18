//! chainstamp-solo 四族测试：链铸时戳、旧行零动、confirm 三态、单调拒收不可达。
//! 先红后绿——红相即旧码对迟到 hint 抛 TimestampNotMonotonic 且 confirm 命令缺席。
//!
//! testhard 批件二（2026-09-18）：scribe 二进制寻址统一切 crate::testbin 两级
//! 解析——旧形 `CARGO_MANIFEST_DIR/target/debug/scribe` 在 worktree 工地形
//! （共享 CARGO_TARGET_DIR）必红，修前红证 confirm_three_states 留档。

#[cfg(test)]
mod chainstamp {
    use chrono::{Duration, Utc};
    use crate::event_stream::{
        append, compute_event_hash, load_events, verify, Actor, ActorType, Event, EventInput,
        VerifyRange,
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
        let dir = std::env::temp_dir().join(format!("chainstamp-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("trail.ndjson")
    }

    fn ev(id: &str, ts: chrono::DateTime<Utc>) -> EventInput {
        EventInput {
            event_id: id.to_string(),
            event_type: "chainstamp_test_event".to_string(),
            timestamp: ts,
            actor: actor(),
            details: None,
            doc_id: "chainstamp-test".to_string(),
            prev_hash: None,
            event_class: None,
            verification_result: None,
            session_id: None,
            identity_hash: None,
        }
    }

    fn scribe_bin() -> PathBuf {
        crate::testbin::bin("scribe")
    }

    // ── 族一：链铸时戳——迟到 hint 铸值抬升、未来 hint 原样直通、链序严格单调 ──

    #[test]
    fn mint_raises_late_hint_future_hint_passes_through() {
        let trail = temp_trail("mint");
        let _ = std::fs::remove_file(&trail);
        let t0 = Utc::now();
        let mut store: Vec<Event> = Vec::new();

        // e1：首事件，hint 即铸值
        append(ev("e1", t0), &mut store, Some(&trail)).unwrap();
        // e2：迟到一小时——旧码此处 Err(TimestampNotMonotonic)，新码铸值抬升
        let late = t0 - Duration::hours(1);
        append(ev("e2", late), &mut store, Some(&trail)).unwrap();
        // e3：未来 hint——高于铸线即原样直通
        let future = t0 + Duration::days(365);
        append(ev("e3", future), &mut store, Some(&trail)).unwrap();

        let events = load_events(&trail).unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].timestamp, t0);
        // e2 铸值 = e1 + 1ms
        assert_eq!(events[1].timestamp, events[0].timestamp + Duration::milliseconds(1));
        // e2 的迟到 hint 降级 declared_ts 元数据
        let det = events[1].details.as_ref().expect("迟到 hint 必留 declared_ts");
        assert_eq!(det["declared_ts"], late.to_rfc3339());
        // e3 未来 hint 原样直通，零 declared_ts
        assert_eq!(events[2].timestamp, future);
        assert!(events[2].details.is_none(), "hint 高于铸线不注入元数据");
        // 链 valid 且全序严格单调
        let v = verify(&events, VerifyRange::Full).unwrap();
        assert_eq!(v.event_count, 3);
        for w in events.windows(2) {
            assert!(w[1].timestamp > w[0].timestamp, "时戳序严格单调");
        }
    }

    // ── 族二：旧行零动——前缀字节不变、旧形哈希复算一致、无调整零元数据 ──

    #[test]
    fn old_rows_byte_identical_and_hash_stable() {
        let trail = temp_trail("oldrows");
        let _ = std::fs::remove_file(&trail);
        let t0 = Utc::now();
        let mut store: Vec<Event> = Vec::new();
        // e1：hint 高于铸线，语义上与旧码产出逐字节同（时戳原样、零 declared_ts）
        append(ev("e1", t0), &mut store, Some(&trail)).unwrap();
        let before = std::fs::read(&trail).unwrap();
        // e2：迟到 hint 触发铸值——旧行必须零动
        append(ev("e2", t0 - Duration::seconds(5)), &mut store, Some(&trail)).unwrap();
        let after = std::fs::read(&trail).unwrap();
        assert!(
            after.starts_with(&before),
            "追加后既有链行前缀字节零改变"
        );
        // 旧形事件哈希复算一致（哈希公式零动的构造性证据）
        let events = load_events(&trail).unwrap();
        assert_eq!(compute_event_hash(&events[0]), events[0].event_hash);
        // 铸值行哈希同样复算一致（新语义内部自洽）
        assert_eq!(compute_event_hash(&events[1]), events[1].event_hash);
    }

    // ── 族三：confirm——hash 前缀形、event-id 形、未在档、用法错 ──

    #[test]
    fn confirm_three_states() {
        let trail = temp_trail("confirm");
        let _ = std::fs::remove_file(&trail);
        let t0 = Utc::now();
        let mut store: Vec<Event> = Vec::new();
        append(ev("confirm-alpha-0001", t0), &mut store, Some(&trail)).unwrap();
        append(ev("confirm-beta-0002", t0 + Duration::milliseconds(1)), &mut store, Some(&trail)).unwrap();
        let events = load_events(&trail).unwrap();
        let prefix8 = events[1].event_hash[..8].to_string();
        let bin = scribe_bin();
        let t = trail.to_str().unwrap();

        // hash 前缀命中
        let out = Command::new(&bin)
            .args(["confirm", "--trail", t, "--hash", &prefix8])
            .output().unwrap();
        assert_eq!(out.status.code(), Some(0), "hash 前缀在档即零");
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(v["found"], true);
        assert_eq!(v["index"], 1);
        assert_eq!(v["event_id"], "confirm-beta-0002");

        // event-id 命中
        let out = Command::new(&bin)
            .args(["confirm", "--trail", t, "--event-id", "confirm-alpha-0001"])
            .output().unwrap();
        assert_eq!(out.status.code(), Some(0));
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(v["found"], true);
        assert_eq!(v["index"], 0);

        // 未在档
        let out = Command::new(&bin)
            .args(["confirm", "--trail", t, "--hash", "ffffffff"])
            .output().unwrap();
        assert_eq!(out.status.code(), Some(1), "未在档即一");
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(v["found"], false);

        // 用法错：前缀过短
        let out = Command::new(&bin)
            .args(["confirm", "--trail", t, "--hash", "abc"])
            .output().unwrap();
        assert_eq!(out.status.code(), Some(2), "前缀不足八位即用法错二");

        // 用法错：缺 --trail
        let out = Command::new(&bin)
            .args(["confirm", "--hash", &prefix8])
            .output().unwrap();
        assert_eq!(out.status.code(), Some(2));

        // 用法错：hash 与 event-id 须二择一
        let out = Command::new(&bin)
            .args(["confirm", "--trail", t, "--hash", &prefix8, "--event-id", "x"])
            .output().unwrap();
        assert_eq!(out.status.code(), Some(2));
    }

    // ── 族四：单调拒收不可达——病态 hint 俱成，链恒 valid ──

    #[test]
    fn monotonic_rejection_unreachable_for_any_hint() {
        let trail = temp_trail("unreach");
        let _ = std::fs::remove_file(&trail);
        let t0 = Utc::now();
        let mut store: Vec<Event> = Vec::new();
        append(ev("u0", t0), &mut store, Some(&trail)).unwrap();
        let pathological = [
            t0,                               // 恰等链尾
            t0 - Duration::milliseconds(1),   // 差一毫秒
            t0 - Duration::days(3650),        // 十年前
            chrono::DateTime::<Utc>::MIN_UTC, // 纪元底
        ];
        for (i, h) in pathological.iter().enumerate() {
            let id = format!("u{}", i + 1);
            let r = append(ev(&id, *h), &mut store, Some(&trail));
            assert!(r.is_ok(), "病态 hint {} 竟被拒：{:?}", i, r.err());
        }
        let events = load_events(&trail).unwrap();
        assert_eq!(events.len(), 5);
        let v = verify(&events, VerifyRange::Full).unwrap();
        assert_eq!(v.event_count, 5);
        for w in events.windows(2) {
            assert!(w[1].timestamp > w[0].timestamp);
        }
    }
}
