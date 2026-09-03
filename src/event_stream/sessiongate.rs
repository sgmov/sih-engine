//! 会话在册验模块，承 guardrail2-solo 闸三即书简写路径会话登记守卫。
//!
//! 台账行形承 sih-tools/lease/ledger/sessions.ndjson 即每行一 JSON 对象，
//! 行 event 值为 issued 置在册，revoked 弹在册同号，兼前向兼容跳过其余行。
//! 语义承租约 active_sessions 单遍事件序：同号重开不被旧 revoked 误抹。
//!
//! 无 --session 即拒 SessionNotActive；有会话号但台账无在册同号即已吊销或
//! 从未签发亦拒 SessionNotActive。台账缺席即零在册即会话号无从在册即拒；
//! JSON 非法或行缺键即工具异常不静默。--no-session-reason 主会处置位默认关，
//! 显式开启即无 --session 亦放行，行事由随事件落链供审计。

use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug)]
pub enum SessionGateError {
    /// 无会话号且未予主会处置位，或会话号不在册活跃即拒写。
    NotActive { session_id: Option<String> },
    /// 台账不可读或行结构非法，工具异常。
    Unreadable(String),
}

/// 会话在册前查。sessions 即会话台账路径，session 即调用方会话号，
/// no_session_reason 即主会处置事由（缺省关）。
///
/// 返回 Ok(事由) 即放行，事由为 Some 即主会处置位放行；NotActive 即会话
/// 不在册或未给会话号即拒写；Unreadable 即台账 JSON 非法或缺键。
pub fn check_session(
    sessions: &Path,
    session: Option<&str>,
    no_session_reason: Option<&str>,
) -> Result<Option<String>, SessionGateError> {
    match (session, no_session_reason) {
        (None, Some(reason)) => {
            // 主会处置位：无会话号但显式给事由即放行，事由供审计。
            return Ok(Some(reason.to_string()));
        }
        (None, None) => return Err(SessionGateError::NotActive { session_id: None }),
        (Some(_), _) => {}
    }
    let text = match std::fs::read_to_string(sessions) {
        Ok(t) => t,
        Err(_) => {
            // 台账缺席即零在册，会话号无从活跃即拒。
            return Err(SessionGateError::NotActive {
                session_id: session.map(|s| s.to_string()),
            });
        }
    };
    let mut active: BTreeSet<String> = BTreeSet::new();
    for (lineno, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let row: serde_json::Value = serde_json::from_str(line)
            .map_err(|e| SessionGateError::Unreadable(format!("行 {} 非法 {e}", lineno + 1)))?;
        let Some(kind) = row.get("event").and_then(|v| v.as_str()) else {
            return Err(SessionGateError::Unreadable(format!("行 {} 缺 event", lineno + 1)));
        };
        let Some(sid) = row.get("session_id").and_then(|v| v.as_str()) else {
            return Err(SessionGateError::Unreadable(format!("行 {} 缺 session_id", lineno + 1)));
        };
        match kind {
            "issued" => {
                active.insert(sid.to_string());
            }
            "revoked" => {
                active.remove(sid);
            }
            _ => {}
        }
    }
    let sid = session.unwrap();
    if active.contains(sid) {
        Ok(None)
    } else {
        Err(SessionGateError::NotActive {
            session_id: Some(sid.to_string()),
        })
    }
}

#[cfg(test)]
mod sessiongate_tests {
    use super::{check_session, SessionGateError};
    use std::path::PathBuf;

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sessiongate-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    // S1 无会话号且无处置位即拒 SessionNotActive。
    #[test]
    fn s1_no_session_rejected() {
        let dir = temp_dir("s1");
        let sessions = dir.join("sessions.ndjson");
        std::fs::write(&sessions, "{\"event\":\"issued\",\"session_id\":\"sessA\"}\n").unwrap();
        assert!(matches!(
            check_session(&sessions, None, None),
            Err(SessionGateError::NotActive { session_id: None })
        ));
    }

    // S2 有会话号在册活跃即放行。
    #[test]
    fn s2_active_session_allows() {
        let dir = temp_dir("s2");
        let sessions = dir.join("sessions.ndjson");
        std::fs::write(&sessions, "{\"event\":\"issued\",\"session_id\":\"sessA\"}\n").unwrap();
        assert!(check_session(&sessions, Some("sessA"), None).is_ok());
    }

    // S3 会话号已吊销即拒。
    #[test]
    fn s3_revoked_rejected() {
        let dir = temp_dir("s3");
        let sessions = dir.join("sessions.ndjson");
        std::fs::write(
            &sessions,
            "{\"event\":\"issued\",\"session_id\":\"sessA\"}\n{\"event\":\"revoked\",\"session_id\":\"sessA\"}\n",
        )
        .unwrap();
        assert!(matches!(
            check_session(&sessions, Some("sessA"), None),
            Err(SessionGateError::NotActive { session_id: Some(ref s) }) if s == "sessA"
        ));
    }

    // S4 会话号从未签发即拒。
    #[test]
    fn s4_never_issued_rejected() {
        let dir = temp_dir("s4");
        let sessions = dir.join("sessions.ndjson");
        std::fs::write(&sessions, "{\"event\":\"issued\",\"session_id\":\"sessA\"}\n").unwrap();
        assert!(matches!(
            check_session(&sessions, Some("sessB"), None),
            Err(SessionGateError::NotActive { .. })
        ));
    }

    // S5 台账缺席即零在册拒会话号。
    #[test]
    fn s5_missing_ledger_rejects() {
        let dir = temp_dir("s5");
        let sessions = dir.join("absent.ndjson");
        assert!(matches!(
            check_session(&sessions, Some("sessA"), None),
            Err(SessionGateError::NotActive { .. })
        ));
    }

    // S6 台账 JSON 非法即工具异常不静默。
    #[test]
    fn s6_malformed_unreadable() {
        let dir = temp_dir("s6");
        let sessions = dir.join("sessions.ndjson");
        std::fs::write(&sessions, "not-json\n").unwrap();
        assert!(matches!(
            check_session(&sessions, Some("sessA"), None),
            Err(SessionGateError::Unreadable(_))
        ));
    }

    // S7 无会话号但主会处置位放行并载事由。
    #[test]
    fn s7_no_session_reason_allows() {
        let dir = temp_dir("s7");
        let sessions = dir.join("absent.ndjson");
        assert_eq!(
            check_session(&sessions, None, Some("主会处置")).unwrap(),
            Some("主会处置".to_string())
        );
    }

    // S8 同号重开不被旧 revoked 误抹（单遍事件序语义）。
    #[test]
    fn s8_reissue_not_clobbered() {
        let dir = temp_dir("s8");
        let sessions = dir.join("sessions.ndjson");
        std::fs::write(
            &sessions,
            "{\"event\":\"issued\",\"session_id\":\"sessA\"}\n{\"event\":\"revoked\",\"session_id\":\"sessA\"}\n{\"event\":\"issued\",\"session_id\":\"sessA\"}\n",
        )
        .unwrap();
        assert!(check_session(&sessions, Some("sessA"), None).is_ok());
    }
}