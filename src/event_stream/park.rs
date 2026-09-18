//! 停泊入口，承接 SPEC-006#park-entry。
//!
//! 双动作 enter 与 exit，配对不变量两道机械门即重入拒与无主出拒，
//! 在泊判定按既有事件链序重放停泊事件，拒绝零留痕即不产事件。

use crate::event_stream::append::{load_events, AppendError};
use crate::event_stream::event::{Actor, Event, EventInput};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::path::Path;

#[derive(Debug)]
pub enum ParkError {
    RecordNotJson,
    ActionMissing,
    EnterMissingField(&'static str),
    ExitMissingField(&'static str),
    InvalidTtl,
    InvalidDisposition(String),
    ReEnterRejected(String),
    OrphanExitRejected(String),
    EntryIdUsedRejected(String),
}

fn entry_parked(existing: &[Event], entry_id: &str) -> bool {
    let mut parked = false;
    for e in existing {
        let Some(d) = e.details.as_ref().and_then(|d| d.as_object()) else {
            continue;
        };
        if d.get("entry_id").and_then(|v| v.as_str()) != Some(entry_id) {
            continue;
        }
        match e.event_type.as_str() {
            "parking_entered" => parked = true,
            "parking_exited" => parked = false,
            _ => {}
        }
    }
    parked
}

/// 号源唯一判据即重放面内该 entry_id 出现过任何 parking_entered 即已用，承 entryunique-solo 修订二。
fn entry_id_used(existing: &[Event], entry_id: &str) -> bool {
    existing.iter().any(|e| {
        e.event_type == "parking_entered"
            && e.details
                .as_ref()
                .and_then(|d| d.as_object())
                .and_then(|d| d.get("entry_id"))
                .and_then(|v| v.as_str())
                == Some(entry_id)
    })
}

/// 双动作 enter 与 exit，配对不变量三道门按既有事件链序重放判定。
pub fn park_event(
    record_text: &str,
    existing: &[Event],
    actor: Actor,
    timestamp: DateTime<Utc>,
) -> Result<EventInput, ParkError> {
    let record: Value = serde_json::from_str(record_text).map_err(|_| ParkError::RecordNotJson)?;
    let obj = record.as_object().ok_or(ParkError::RecordNotJson)?;
    let action = obj
        .get("action")
        .and_then(|a| a.as_str())
        .ok_or(ParkError::ActionMissing)?;

    match action {
        "enter" => {
            let entry_id = obj
                .get("entry_id")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::EnterMissingField("entry_id"))?
                .to_string();
            if entry_parked(existing, &entry_id) {
                return Err(ParkError::ReEnterRejected(entry_id));
            }
            if entry_id_used(existing, &entry_id) {
                return Err(ParkError::EntryIdUsedRejected(entry_id));
            }
            let title = obj
                .get("title")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::EnterMissingField("title"))?;
            let exit_condition = obj
                .get("exit_condition")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::EnterMissingField("exit_condition"))?;
            let ttl_days = obj
                .get("ttl_days")
                .and_then(|v| v.as_i64())
                .ok_or(ParkError::EnterMissingField("ttl_days"))?;
            if ttl_days <= 0 {
                return Err(ParkError::InvalidTtl);
            }
            let mut details = json!({
                "entry_id": entry_id,
                "title": title,
                "exit_condition": exit_condition,
                "ttl_days": ttl_days,
            });
            if let Some(ctx) = obj.get("context") {
                details["context"] = ctx.clone();
            }
            let doc_id = entry_id;
            Ok(EventInput {
                event_id: uuid::Uuid::new_v4().to_string(),
                event_type: "parking_entered".to_string(),
                timestamp,
                actor,
                details: Some(details),
                doc_id,
                prev_hash: None,
                event_class: Some("record_only".to_string()),
                verification_result: None,
                session_id: None,
                identity_hash: None,
            })
        }
        "exit" => {
            let entry_id = obj
                .get("entry_id")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::ExitMissingField("entry_id"))?
                .to_string();
            if !entry_parked(existing, &entry_id) {
                return Err(ParkError::OrphanExitRejected(entry_id));
            }
            let disposition = obj
                .get("disposition")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::ExitMissingField("disposition"))?;
            if disposition != "promoted" && disposition != "discarded" {
                return Err(ParkError::InvalidDisposition(disposition.to_string()));
            }
            let ruling = obj
                .get("ruling")
                .and_then(|v| v.as_str())
                .ok_or(ParkError::ExitMissingField("ruling"))?;
            let details = json!({
                "entry_id": entry_id,
                "disposition": disposition,
                "ruling": ruling,
            });
            let doc_id = entry_id;
            Ok(EventInput {
                event_id: uuid::Uuid::new_v4().to_string(),
                event_type: "parking_exited".to_string(),
                timestamp,
                actor,
                details: Some(details),
                doc_id,
                prev_hash: None,
                event_class: Some("record_only".to_string()),
                verification_result: None,
                session_id: None,
                identity_hash: None,
            })
        }
        _ => Err(ParkError::ActionMissing),
    }
}

/// 泊界账本重放面收窄即规范链文件面（testhard 批件三）。
///
/// park 配对门的重放面从 --trail 单链文件扩为同目录规范链文件集按名序
/// （parkreplay-solo 修订一原形），本批收窄其成员判：日期形
/// `<YYYY-MM-DD>.ndjson` 链文件，加日期前缀 splinter 分叉保全件形
/// `<YYYY-MM-DD>-*-splinter.ndjson`（如 2026-08-28-pk023spec-pre-replay-
/// splinter.ndjson，重复入泊笔去重语义由名序后真链覆盖承载，零破坏），加
/// --trail 显式指定文件本身（链文件身份由调用方声明承载，测试形
/// trail.ndjson 兼容）。其余 ndjson（随手笔记等非链件）一律忽略——病灶
/// 申报出处：sih/event/plan/pendline-materials/pendline-results.md 测试发现
/// 处置节（scribe park 泊界重放面解析同目录全部 ndjson）。跨天出泊可达性
/// 零回退（日期形全在面）；文件名序加载零变。
///
/// 载体引用（承接 ORD-019 版本偏序与外化状态存储）：重放面遍历是版本偏序
/// 持久性语义的机械实例，按既有事件链序重放判定在泊状态。推导见
/// sih-math/docs/scriwire-scribe-derivation-2026-09-03.md。
pub fn load_parking_scope(trail: &Path) -> Result<Vec<Event>, AppendError> {
    let target = trail.to_path_buf();
    let dir = match target.parent() {
        Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
        _ => return load_events(&target),
    };
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
        .map_err(|e| AppendError::Internal(format!("read dir failed: {e}")))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|p| is_canonical_chain_name(p))
        .collect();
    // --trail 显式指定文件即链文件（非日期形兼容）；首次落笔前文件可尚未
    // 在盘（旧形 read_dir 亦不见），缺席不推入。同名判同件（候选同出一目录）。
    if target.is_file() && !files.iter().any(|f| f.file_name() == target.file_name()) {
        files.push(target);
    }
    files.sort();
    let mut scope = Vec::new();
    for f in &files {
        scope.extend(load_events(f)?);
    }
    Ok(scope)
}

/// 规范链文件名判：日期形 `<YYYY-MM-DD>.ndjson` 与日期前缀 splinter 保全件形
/// `<YYYY-MM-DD>-*-splinter.ndjson`（testhard 批件三）。
fn is_canonical_chain_name(p: &Path) -> bool {
    let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    let Some(stem) = name.strip_suffix(".ndjson") else {
        return false;
    };
    if is_date_form(stem) {
        return true;
    }
    stem.len() > 11
        && stem.as_bytes()[10] == b'-'
        && stem.ends_with("-splinter")
        && is_date_form(&stem[..10])
}

/// `YYYY-MM-DD` 十位日期形判。
fn is_date_form(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && b.iter().enumerate().all(|(i, c)| {
            i == 4 || i == 7 || c.is_ascii_digit()
        })
}

#[cfg(test)]
mod scope_tests {
    use super::*;
    use crate::event_stream::append::append;
    use crate::event_stream::event::ActorType;
    use crate::event_stream::hash::GENESIS_PREV_HASH;

    fn actor() -> Actor {
        Actor {
            actor_id: "scribe".to_string(),
            actor_type: ActorType::System,
            invoked_via: "cli".to_string(),
        }
    }

    fn park_to(dir: &Path, file: &str, record: &str, stamp: chrono::DateTime<chrono::Utc>) {
        let path = dir.join(file);
        let mut store = load_events(&path).unwrap_or_default();
        let scope = load_parking_scope(&path).unwrap();
        let input = park_event(record, &scope, actor(), stamp).unwrap();
        append(input, &mut store, Some(&path)).unwrap();
    }

    #[test]
    fn cross_day_exit_reaches_over_full_scope() {
        let dir = tempfile::tempdir().unwrap();
        let enter = r#"{"action":"enter","entry_id":"pk-rr-1","title":"跨天出泊","exit_condition":"测试即弃","ttl_days":7}"#;
        let exit = r#"{"action":"exit","entry_id":"pk-rr-1","disposition":"discarded","ruling":"测试即弃"}"#;
        let t1 = chrono::Utc::now();
        park_to(dir.path(), "2026-09-01.ndjson", enter, t1);

        let today = dir.path().join("2026-09-03.ndjson");
        let single = load_events(&today).unwrap_or_default();
        assert!(matches!(
            park_event(exit, &single, actor(), t1 + chrono::Duration::seconds(10)),
            Err(ParkError::OrphanExitRejected(_))
        ));

        let t2 = t1 + chrono::Duration::seconds(20);
        park_to(dir.path(), "2026-09-03.ndjson", exit, t2);
        let scope = load_parking_scope(&today).unwrap();
        assert!(!scope.is_empty());
        let again = r#"{"action":"exit","entry_id":"pk-rr-1","disposition":"discarded","ruling":"复出即无主出拒"}"#;
        assert!(matches!(
            park_event(again, &scope, actor(), t2 + chrono::Duration::seconds(30)),
            Err(ParkError::OrphanExitRejected(_))
        ));
    }

    #[test]
    fn cross_file_reenter_rejected_over_full_scope() {
        let dir = tempfile::tempdir().unwrap();
        let enter = r#"{"action":"enter","entry_id":"pk-rr-2","title":"跨文件重入拒","exit_condition":"测试即弃","ttl_days":7}"#;
        let t1 = chrono::Utc::now();
        park_to(dir.path(), "2026-09-01.ndjson", enter, t1);
        let same_file = load_events(&dir.path().join("2026-09-01.ndjson")).unwrap();
        assert!(matches!(
            park_event(enter, &same_file, actor(), t1 + chrono::Duration::seconds(10)),
            Err(ParkError::ReEnterRejected(_))
        ));
        let today = dir.path().join("2026-09-03.ndjson");
        let scope = load_parking_scope(&today).unwrap();
        assert!(matches!(
            park_event(enter, &scope, actor(), t1 + chrono::Duration::seconds(20)),
            Err(ParkError::ReEnterRejected(_))
        ));
    }

    #[test]
    fn append_face_stays_single_chain() {
        let dir = tempfile::tempdir().unwrap();
        let enter = r#"{"action":"enter","entry_id":"pk-rr-3","title":"追加面单链","exit_condition":"测试即弃","ttl_days":7}"#;
        let exit = r#"{"action":"exit","entry_id":"pk-rr-3","disposition":"promoted","ruling":"测试即弃"}"#;
        let t1 = chrono::Utc::now();
        park_to(dir.path(), "2026-09-01.ndjson", enter, t1);
        park_to(dir.path(), "2026-09-03.ndjson", exit, t1 + chrono::Duration::seconds(10));
        let day_a = load_events(&dir.path().join("2026-09-01.ndjson")).unwrap();
        let day_b = load_events(&dir.path().join("2026-09-03.ndjson")).unwrap();
        assert_eq!(day_a.len(), 1);
        assert_eq!(day_b.len(), 1);
        assert_eq!(day_b[0].prev_hash, GENESIS_PREV_HASH);
        assert_eq!(day_b[0].event_type, "parking_exited");
    }

    #[test]
    fn new_entry_passes() {
        let dir = tempfile::tempdir().unwrap();
        let enter = r#"{"action":"enter","entry_id":"pk-eu-1","title":"新号过","exit_condition":"测试即弃","ttl_days":7}"#;
        let t1 = chrono::Utc::now();
        park_to(dir.path(), "2026-09-03.ndjson", enter, t1);
        let events = load_events(&dir.path().join("2026-09-03.ndjson")).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "parking_entered");
    }

    #[test]
    fn in_parking_reenter_rejected_regression() {
        let dir = tempfile::tempdir().unwrap();
        let enter = r#"{"action":"enter","entry_id":"pk-eu-2","title":"在泊重入拒","exit_condition":"测试即弃","ttl_days":7}"#;
        let t1 = chrono::Utc::now();
        park_to(dir.path(), "2026-09-03.ndjson", enter, t1);
        let scope = load_parking_scope(&dir.path().join("2026-09-03.ndjson")).unwrap();
        assert!(matches!(
            park_event(enter, &scope, actor(), t1 + chrono::Duration::seconds(10)),
            Err(ParkError::ReEnterRejected(_))
        ));
    }

    #[test]
    fn exited_number_reuse_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let enter = r#"{"action":"enter","entry_id":"pk-eu-3","title":"已出泊号复用拒","exit_condition":"测试即弃","ttl_days":7}"#;
        let exit = r#"{"action":"exit","entry_id":"pk-eu-3","disposition":"discarded","ruling":"测试即弃"}"#;
        let t1 = chrono::Utc::now();
        park_to(dir.path(), "2026-09-03.ndjson", enter, t1);
        park_to(dir.path(), "2026-09-03.ndjson", exit, t1 + chrono::Duration::seconds(10));
        let scope = load_parking_scope(&dir.path().join("2026-09-03.ndjson")).unwrap();
        assert!(matches!(
            park_event(enter, &scope, actor(), t1 + chrono::Duration::seconds(20)),
            Err(ParkError::EntryIdUsedRejected(_))
        ));
    }

    #[test]
    fn cross_day_used_number_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let enter = r#"{"action":"enter","entry_id":"pk-eu-4","title":"跨天已用号拒","exit_condition":"测试即弃","ttl_days":7}"#;
        let exit = r#"{"action":"exit","entry_id":"pk-eu-4","disposition":"discarded","ruling":"测试即弃"}"#;
        let t1 = chrono::Utc::now();
        park_to(dir.path(), "2026-09-01.ndjson", enter, t1);
        park_to(dir.path(), "2026-09-01.ndjson", exit, t1 + chrono::Duration::seconds(10));
        let today = dir.path().join("2026-09-03.ndjson");
        let scope = load_parking_scope(&today).unwrap();
        assert!(matches!(
            park_event(enter, &scope, actor(), t1 + chrono::Duration::seconds(20)),
            Err(ParkError::EntryIdUsedRejected(_))
        ));
    }

    /// testhard 批件三红转绿钉：泊界重放面只认规范链文件，同目录非日期形
    /// ndjson（随手笔记）忽略，规范链行数零变且号源判定零污染。
    /// 病灶申报出处：sih/event/plan/pendline-materials/pendline-results.md
    /// 测试发现处置节（scribe park 泊界重放面解析同目录全部 ndjson）。
    /// 红相即旧码 read_dir 全量 ndjson 面把垃圾件 parking_entered 假行吃进
    /// 泊界账：scope 多一行且同号新入泊被 EntryIdUsed 误拒。
    #[test]
    fn non_chain_ndjson_ignored_in_parking_scope() {
        let dir = tempfile::tempdir().unwrap();
        let enter = r#"{"action":"enter","entry_id":"pk-th-3","title":"规范链入泊","exit_condition":"测试即弃","ttl_days":7}"#;
        let t1 = chrono::Utc::now();
        park_to(dir.path(), "2026-09-18.ndjson", enter, t1);
        // 垃圾笔记件：非日期形 ndjson，内含 parking_entered 假行（合法事件形）。
        let notes = r#"{"action":"enter","entry_id":"pk-th-garbage","title":"随手笔记假行","exit_condition":"c","ttl_days":3}"#;
        park_to(dir.path(), "notes.ndjson", notes, t1);
        let today = dir.path().join("2026-09-18.ndjson");
        let scope = load_parking_scope(&today).unwrap();
        assert_eq!(scope.len(), 1, "重放面只认规范链，垃圾 ndjson 假行忽略");
        assert_eq!(scope[0].details.as_ref().unwrap()["entry_id"], "pk-th-3");
        // 号源判定零污染：与假行同 entry_id 的新入泊应放行。
        let again = r#"{"action":"enter","entry_id":"pk-th-garbage","title":"假号真用","exit_condition":"c","ttl_days":3}"#;
        let verdict = park_event(again, &scope, actor(), t1 + chrono::Duration::seconds(10));
        assert!(
            verdict.is_ok(),
            "垃圾 ndjson 假行不得污染号源判定：{:?}",
            verdict.err()
        );
    }

    /// testhard 批件三白名单钉：日期前缀 splinter 分叉保全件形仍在重放面
    ///（2026-08-28-pk023spec-pre-replay-splinter.ndjson 先例），重复入泊笔
    /// 去重语义零破坏；垃圾件同场仍忽略。
    #[test]
    fn splinter_form_stays_in_scope() {
        let dir = tempfile::tempdir().unwrap();
        let t1 = chrono::Utc::now();
        let enter_a = r#"{"action":"enter","entry_id":"pk-th-sa","title":"真链入泊","exit_condition":"c","ttl_days":7}"#;
        let enter_b = r#"{"action":"enter","entry_id":"pk-th-sb","title":"保全件入泊","exit_condition":"c","ttl_days":7}"#;
        park_to(dir.path(), "2026-08-28.ndjson", enter_a, t1);
        park_to(dir.path(), "2026-08-28-pk023spec-pre-replay-splinter.ndjson", enter_b, t1);
        let notes = r#"{"action":"enter","entry_id":"pk-th-garbage","title":"随手笔记假行","exit_condition":"c","ttl_days":3}"#;
        park_to(dir.path(), "notes.ndjson", notes, t1);
        let scope = load_parking_scope(&dir.path().join("2026-08-28.ndjson")).unwrap();
        let ids: Vec<&str> = scope
            .iter()
            .filter_map(|e| e.details.as_ref())
            .filter_map(|d| d.get("entry_id").and_then(|v| v.as_str()))
            .collect();
        assert_eq!(ids, vec!["pk-th-sb", "pk-th-sa"], "splinter 保全件在面且名序居真链前，垃圾件忽略");
    }

    /// testhard 批件三声明面钉：--trail 显式指定文件即链文件（非日期形
    /// 兼容），自含停泊笔在其重放面内，在泊重入拒零回退。
    #[test]
    fn non_date_trail_self_in_scope() {
        let dir = tempfile::tempdir().unwrap();
        let enter = r#"{"action":"enter","entry_id":"pk-th-nd","title":"非日期链自含","exit_condition":"c","ttl_days":7}"#;
        let t1 = chrono::Utc::now();
        park_to(dir.path(), "trail.ndjson", enter, t1);
        let scope = load_parking_scope(&dir.path().join("trail.ndjson")).unwrap();
        assert_eq!(scope.len(), 1, "--trail 显式指定文件在其重放面内");
        assert!(matches!(
            park_event(enter, &scope, actor(), t1 + chrono::Duration::seconds(10)),
            Err(ParkError::ReEnterRejected(_))
        ));
    }
}
