//! 锁位前查模块，承 lockguard-solo 批即书简写路径守卫。
//!
//! 台账行形承 sih-tools/lease/ledger/locks.ndjson 即每行一 JSON 对象。
//! 守卫只认锁态迁移两值即 event 为 acquired 与 released 的行，其余行如
//! check 位的 checked 审计行跳过即前向兼容。语义：按 session_id 加 path
//! 二元组取最新迁移，最新为 acquired 即 active 锁。路径对表即台账相对
//! 路径按台账文件上四级为工作区根解析后与 trail 规范绝对路径比对，台账
//! 绝对路径径直比对。他会话持锁即拒，本会话持锁或零锁或台账文件缺席
//! 放行，JSON 非法或锁态行缺键即工具异常不静默。
//!
//! 追加态锁承 locksplit-solo 批：acquired 行 mode 字段为 append 即追加态
//! 持位，trail 追加操作认 append 持位即他会话 append 持位不拦（追加本
//! 无害并发），exclusive 持位照旧互斥拦。mode 缺席即 exclusive 向后兼容。

use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug)]
pub enum LockGateError {
    /// trail 路径在他会话 active 锁下，holders 即持锁会话号表。
    Held { holders: Vec<String> },
    /// 台账不可读或行结构非法，工具异常。
    Unreadable(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LockEvent {
    Acquired,
    Released,
}

fn parse_event(value: &str) -> Option<LockEvent> {
    match value {
        "acquired" => Some(LockEvent::Acquired),
        "released" => Some(LockEvent::Released),
        _ => None,
    }
}

/// 规范路径形即能 canonicalize 则 canonicalize，否则原样。
fn norm(path: &Path) -> std::path::PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// 锁位前查。locks 即台账路径，trail 即待写链路径，session 即调用方会话号。
///
/// 返回 Ok 即放行；Held 即他会话持锁拒写；Unreadable 即台账缺席以外的
/// JSON 非法或锁态行缺键。台账文件缺席视为零锁放行。
pub fn check_locks(locks: &Path, trail: &Path, session: Option<&str>) -> Result<(), LockGateError> {
    let text = match std::fs::read_to_string(locks) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(LockGateError::Unreadable(format!("{e}"))),
    };
    // 工作区根即台账上四级，ROOT/sih-tools/lease/ledger/locks.ndjson。
    let root = locks.ancestors().nth(4).map(|p| p.to_path_buf());
    let trail_norm = norm(trail);
    // 按 (session_id, path) 取最新锁态迁移，非迁移行跳过。
    let mut latest: BTreeMap<(String, String), (LockEvent, String)> = BTreeMap::new();
    for (lineno, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let row: serde_json::Value = serde_json::from_str(line)
            .map_err(|e| LockGateError::Unreadable(format!("行 {} 非法 {e}", lineno + 1)))?;
        let Some(kind) = row.get("event").and_then(|v| v.as_str()) else {
            return Err(LockGateError::Unreadable(format!("行 {} 缺 event", lineno + 1)));
        };
        let Some(kind) = parse_event(kind) else {
            continue;
        };
        let Some(session_id) = row.get("session_id").and_then(|v| v.as_str()) else {
            return Err(LockGateError::Unreadable(format!("行 {} 缺 session_id", lineno + 1)));
        };
        let Some(path) = row.get("path").and_then(|v| v.as_str()) else {
            return Err(LockGateError::Unreadable(format!("行 {} 缺 path", lineno + 1)));
        };
        let mode = row.get("mode").and_then(|v| v.as_str()).unwrap_or("exclusive");
        latest.insert((session_id.to_string(), path.to_string()), (kind, mode.to_string()));
    }
    let mut holders = Vec::new();
    for ((session_id, path), (kind, mode)) in &latest {
        if *kind != LockEvent::Acquired {
            continue;
        }
        let locked = match Path::new(path).is_absolute() {
            true => norm(Path::new(path)),
            false => match &root {
                Some(root) => norm(&root.join(path)),
                None => continue,
            },
        };
        if locked == trail_norm {
            if session.map(|s| s == session_id).unwrap_or(false) {
                continue;
            }
            if mode == "exclusive" {
                holders.push(session_id.clone());
            }
        }
    }
    if holders.is_empty() {
        Ok(())
    } else {
        holders.sort();
        Err(LockGateError::Held { holders })
    }
}

#[cfg(test)]
mod lockgate_tests {
    use super::{check_locks, LockGateError};
    use std::path::PathBuf;

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("lockgate-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    // G1 台账缺席即零锁放行。
    #[test]
    fn g1_missing_ledger_allows() {
        let locks = temp_dir("g1").join("absent.ndjson");
        let trail = temp_dir("g1").join("trail.ndjson");
        assert!(check_locks(&locks, &trail, None).is_ok());
    }

    // G2 他会话持锁即拒并报持锁方。
    #[test]
    fn g2_other_session_held() {
        let dir = temp_dir("g2");
        let locks = dir.join("ROOT/sih-tools/lease/ledger/locks.ndjson");
        std::fs::create_dir_all(locks.parent().unwrap()).unwrap();
        std::fs::write(
            &locks,
            "{\"event\":\"acquired\",\"session_id\":\"sessA\",\"path\":\"sih-engine/sih/event/trail/2026-08-28.ndjson\"}\n",
        )
        .unwrap();
        let trail = dir.join("ROOT/sih-engine/sih/event/trail/2026-08-28.ndjson");
        std::fs::create_dir_all(trail.parent().unwrap()).unwrap();
        std::fs::write(&trail, "").unwrap();
        match check_locks(&locks, &trail, Some("sessB")) {
            Err(LockGateError::Held { holders }) => assert_eq!(holders, vec!["sessA".to_string()]),
            other => panic!("应 Held 实得 {other:?}"),
        }
        // 无会话号同样拒。
        assert!(matches!(
            check_locks(&locks, &trail, None),
            Err(LockGateError::Held { .. })
        ));
    }

    // G3 本会话持锁放行。
    #[test]
    fn g3_self_session_allows() {
        let dir = temp_dir("g3");
        let locks = dir.join("ROOT/sih-tools/lease/ledger/locks.ndjson");
        std::fs::create_dir_all(locks.parent().unwrap()).unwrap();
        std::fs::write(
            &locks,
            "{\"event\":\"acquired\",\"session_id\":\"sessA\",\"path\":\"sih-engine/sih/event/trail/2026-08-28.ndjson\"}\n",
        )
        .unwrap();
        let trail = dir.join("ROOT/sih-engine/sih/event/trail/2026-08-28.ndjson");
        std::fs::create_dir_all(trail.parent().unwrap()).unwrap();
        std::fs::write(&trail, "").unwrap();
        assert!(check_locks(&locks, &trail, Some("sessA")).is_ok());
    }

    // G4 先取后释即放行。
    #[test]
    fn g4_released_allows() {
        let dir = temp_dir("g4");
        let locks = dir.join("ROOT/sih-tools/lease/ledger/locks.ndjson");
        std::fs::create_dir_all(locks.parent().unwrap()).unwrap();
        std::fs::write(
            &locks,
            "{\"event\":\"acquired\",\"session_id\":\"sessA\",\"path\":\"t.ndjson\"}\n{\"event\":\"released\",\"session_id\":\"sessA\",\"path\":\"t.ndjson\"}\n",
        )
        .unwrap();
        let trail = dir.join("ROOT/t.ndjson");
        std::fs::write(&trail, "").unwrap();
        assert!(check_locks(&locks, &trail, None).is_ok());
    }

    // G5 台账 JSON 非法即工具异常不静默。
    #[test]
    fn g5_malformed_is_unreadable() {
        let dir = temp_dir("g5");
        let locks = dir.join("locks.ndjson");
        std::fs::write(&locks, "not-json\n").unwrap();
        let trail = dir.join("t.ndjson");
        std::fs::write(&trail, "").unwrap();
        assert!(matches!(
            check_locks(&locks, &trail, None),
            Err(LockGateError::Unreadable(_))
        ));
    }

    // G7 非锁态行跳过即 checked 审计行前向兼容。
    #[test]
    fn g7_non_transition_rows_skipped() {
        let dir = temp_dir("g7");
        let locks = dir.join("ROOT/sih-tools/lease/ledger/locks.ndjson");
        std::fs::create_dir_all(locks.parent().unwrap()).unwrap();
        std::fs::write(
            &locks,
            "{\"event\":\"checked\",\"path\":\"x.md\",\"verdict\":\"unknown_target\",\"tool\":{\"name\":\"locks\",\"version\":\"0.1.0\"}}\n{\"event\":\"acquired\",\"session_id\":\"sessA\",\"path\":\"t.ndjson\"}\n",
        )
        .unwrap();
        let trail = dir.join("ROOT/t.ndjson");
        std::fs::write(&trail, "").unwrap();
        assert!(matches!(
            check_locks(&locks, &trail, None),
            Err(LockGateError::Held { .. })
        ));
    }

    // G6 另一会话的他路径锁不波及本 trail。
    #[test]
    fn g6_unrelated_path_allows() {
        let dir = temp_dir("g6");
        let locks = dir.join("ROOT/sih-tools/lease/ledger/locks.ndjson");
        std::fs::create_dir_all(locks.parent().unwrap()).unwrap();
        std::fs::write(
            &locks,
            "{\"event\":\"acquired\",\"session_id\":\"sessA\",\"path\":\"sih-tools/facet/x.md\"}\n",
        )
        .unwrap();
        let trail = dir.join("ROOT/sih-engine/sih/event/trail/2026-08-28.ndjson");
        std::fs::create_dir_all(trail.parent().unwrap()).unwrap();
        std::fs::write(&trail, "").unwrap();
        assert!(check_locks(&locks, &trail, None).is_ok());
    }

    // G8 他会话 append 持位不拦本会话 trail 追加（追加态锁共存）。
    #[test]
    fn g8_append_holder_allows() {
        let dir = temp_dir("g8");
        let locks = dir.join("ROOT/sih-tools/lease/ledger/locks.ndjson");
        std::fs::create_dir_all(locks.parent().unwrap()).unwrap();
        std::fs::write(
            &locks,
            "{\"event\":\"acquired\",\"session_id\":\"sessA\",\"path\":\"sih-engine/sih/event/trail/2026-08-28.ndjson\",\"mode\":\"append\"}\n",
        )
        .unwrap();
        let trail = dir.join("ROOT/sih-engine/sih/event/trail/2026-08-28.ndjson");
        std::fs::create_dir_all(trail.parent().unwrap()).unwrap();
        std::fs::write(&trail, "").unwrap();
        assert!(check_locks(&locks, &trail, Some("sessB")).is_ok());
        assert!(check_locks(&locks, &trail, None).is_ok());
    }

    // G9 他会话 exclusive 持位照旧拦，mode 缺席即 exclusive 向后兼容。
    #[test]
    fn g9_exclusive_holder_blocks() {
        let dir = temp_dir("g9");
        let locks = dir.join("ROOT/sih-tools/lease/ledger/locks.ndjson");
        std::fs::create_dir_all(locks.parent().unwrap()).unwrap();
        std::fs::write(
            &locks,
            "{\"event\":\"acquired\",\"session_id\":\"sessA\",\"path\":\"sih-engine/sih/event/trail/2026-08-28.ndjson\",\"mode\":\"exclusive\"}\n",
        )
        .unwrap();
        let trail = dir.join("ROOT/sih-engine/sih/event/trail/2026-08-28.ndjson");
        std::fs::create_dir_all(trail.parent().unwrap()).unwrap();
        std::fs::write(&trail, "").unwrap();
        match check_locks(&locks, &trail, Some("sessB")) {
            Err(LockGateError::Held { holders }) => assert_eq!(holders, vec!["sessA".to_string()]),
            other => panic!("应 Held 实得 {other:?}"),
        }
    }

    // G10 append 与 exclusive 混持即 exclusive 拦（互斥语义）。
    #[test]
    fn g10_append_plus_exclusive_blocks() {
        let dir = temp_dir("g10");
        let locks = dir.join("ROOT/sih-tools/lease/ledger/locks.ndjson");
        std::fs::create_dir_all(locks.parent().unwrap()).unwrap();
        std::fs::write(
            &locks,
            "{\"event\":\"acquired\",\"session_id\":\"sessA\",\"path\":\"sih-engine/sih/event/trail/2026-08-28.ndjson\",\"mode\":\"append\"}\n{\"event\":\"acquired\",\"session_id\":\"sessB\",\"path\":\"sih-engine/sih/event/trail/2026-08-28.ndjson\",\"mode\":\"exclusive\"}\n",
        )
        .unwrap();
        let trail = dir.join("ROOT/sih-engine/sih/event/trail/2026-08-28.ndjson");
        std::fs::create_dir_all(trail.parent().unwrap()).unwrap();
        std::fs::write(&trail, "").unwrap();
        match check_locks(&locks, &trail, Some("sessC")) {
            Err(LockGateError::Held { holders }) => assert_eq!(holders, vec!["sessB".to_string()]),
            other => panic!("应 Held 实得 {other:?}"),
        }
    }
}
