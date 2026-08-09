//! 集成测试：哈希链完整性
//!
//! 加载 fixtures/2026-07-27.ndjson，验证事件流哈希链完整性。
//!
//! 预期：校验成功，返回校验事件数与末事件哈希。
//!
//! 承接 DES-007#integration-hash-chain 与 SPEC-004#acceptance-criteria 不可篡改性。

use sih_engine::{load_events, verify, VerifyRange};

#[test]
fn test_hash_chain_integrity() {
    let fixtures_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/2026-07-27.ndjson");
    let events = load_events(std::path::Path::new(fixtures_path))
        .expect("failed to load fixtures/2026-07-27.ndjson");

    // 2026-07-27.ndjson 含 6 条事件
    assert!(
        events.len() >= 6,
        "expected at least 6 events, got {}",
        events.len()
    );

    let result = verify(&events, VerifyRange::Full);

    assert!(
        result.is_ok(),
        "哈希链完整性校验应通过，但得到错误: {:?}",
        result.err()
    );

    let succ = result.unwrap();
    assert_eq!(
        succ.event_count, events.len(),
        "校验事件数应等于总事件数"
    );
    assert!(
        !succ.first_hash.is_empty(),
        "首事件哈希不应为空"
    );
    assert!(
        !succ.last_hash.is_empty(),
        "末事件哈希不应为空"
    );

    // 验证首事件的 prev_hash 为全零（创世块）
    assert_eq!(
        events[0].prev_hash,
        "0000000000000000000000000000000000000000000000000000000000000000",
        "首事件 prev_hash 应为全零（创世块）"
    );

    // 验证哈希链逐链接续
    for i in 1..events.len() {
        assert_eq!(
            events[i].prev_hash, events[i - 1].event_hash,
            "事件 {} prev_hash 应等于事件 {} 的 event_hash",
            i,
            i - 1
        );
    }
}
