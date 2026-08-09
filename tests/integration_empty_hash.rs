//! 集成测试：哈希链重建后完整性校验
//!
//! 加载 fixtures/2026-08-03.ndjson，验证哈希链重建后通过完整性校验。
//!
//! 该文件原为空哈希过渡态即 event_id 与 event_hash 与 prev_hash 字段全空，
//! 经确定性程序 rebuild_hash_chain 重建后哈希链完整。
//!
//! 预期：校验成功，哈希链从首事件到末事件连续无断裂。
//!
//! 承接 DES-007#integration-empty-hash 与 OQ-01 哈希链重建。

use sih_engine::{load_events, verify, VerifyRange};

#[test]
fn test_rebuilt_hash_chain_passes_verification() {
    let fixtures_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/2026-08-03.ndjson");
    let events = load_events(std::path::Path::new(fixtures_path))
        .expect("failed to load fixtures/2026-08-03.ndjson");

    // 2026-08-03.ndjson 含 7 条事件
    assert_eq!(
        events.len(), 7,
        "expected exactly 7 events in 2026-08-03.ndjson"
    );

    // 验证：重建后所有事件的哈希字段非空
    for (i, e) in events.iter().enumerate() {
        assert!(
            !e.event_hash.is_empty(),
            "事件 {} event_hash 重建后须非空，实际为空",
            i
        );
        assert!(
            !e.prev_hash.is_empty(),
            "事件 {} prev_hash 重建后须非空，实际为空",
            i
        );
        assert!(
            !e.event_id.is_empty(),
            "事件 {} event_id 重建后须非空，实际为空",
            i
        );
    }

    let result = verify(&events, VerifyRange::Full);

    assert!(
        result.is_ok(),
        "重建后的哈希链须通过完整性校验，实际失败: {:?}",
        result.err()
    );

    let success = result.unwrap();
    assert_eq!(
        success.event_count, 7,
        "校验事件数须为 7"
    );
}
