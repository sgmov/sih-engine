//! 直提守卫纯函数：提交信息形态判定与 staged 面三面覆盖判定与活跃锁投影（SPEC-024
//! 腿二，lease-commitlaw-parallel 批）。融回基准权威 sih-tools/lease/src/lease/
//! guardcore.py，只拦信息形态不判内容；钩子薄壳装配位归腿三。

use serde_json::{json, Value};
use std::path::Path;
use std::sync::OnceLock;
use regex::Regex;

pub(crate) const GUARD_VERSION: &str = "1.17.0";

pub(crate) const STAGED_REJECT_GUIDANCE: &str = "pre-commit 守卫拦截：staged 文件未在活跃租约锁面 ∪ 共享追加面 ∪ 轻车道白名单内。\n三通道：\n  1) 开租约持锁（lease open + lease lock --path <路径>）\n  2) 走直改链笔申报（scribe pen 落 direct_edit_completed 声明）\n  3) 显式绕行：git commit --no-verify 后 lease bypass --repo <仓> --sha <提交号> --reason <事由> 登记留痕\n白名单与共享面扩面走 CONTRACT 修订，禁裸奔增删。";

pub(crate) const DIRECT_LANE_FILE_WHITELIST: [(&str, &str); 34] = [
    ("path", "sih-tools/lease/ledger/"),
    ("path", "sih-tools/lease/CALL-LOG.md"),
    ("path", "sih-tools/scribe/CALL-LOG.md"),
    ("path", "sih-tools/scribe/reports/"),
    ("path", "sih-tools/identity/reports/"),
    ("path", "sih-tools/tally/reports/"),
    ("path", "sih-engine/sih/state/parking/materials/"),
    ("path", "sih-engine/doc/governance/PARKING-v1.md"),
    ("path", "sih-tools/PARKING-v1.md"),
    ("path", "sih-tools/calllog/calls.ndjson"),
    ("path", "sih-tools/acceptor/CALL-LOG.md"),
    ("path", "sih-tools/attnanchor/CALL-LOG.md"),
    ("path", "sih-tools/attractor/CALL-LOG.md"),
    ("path", "sih-tools/cascade/CALL-LOG.md"),
    ("path", "sih-tools/critsweep/CALL-LOG.md"),
    ("path", "sih-tools/elicit/CALL-LOG.md"),
    ("path", "sih-tools/facet/CALL-LOG.md"),
    ("path", "sih-tools/formatter/CALL-LOG.md"),
    ("path", "sih-tools/gauge/CALL-LOG.md"),
    ("path", "sih-tools/identity/CALL-LOG.md"),
    ("path", "sih-tools/latex-helper/CALL-LOG.md"),
    ("path", "sih-tools/lease/CALL-LOG.md"),
    ("path", "sih-tools/locator/CALL-LOG.md"),
    ("path", "sih-tools/locks/CALL-LOG.md"),
    ("path", "sih-tools/meter/CALL-LOG.md"),
    ("path", "sih-tools/nomenclator/CALL-LOG.md"),
    ("path", "sih-tools/parser/CALL-LOG.md"),
    ("path", "sih-tools/scribe/CALL-LOG.md"),
    ("path", "sih-tools/scrutinator/CALL-LOG.md"),
    ("path", "sih-tools/selector/CALL-LOG.md"),
    ("path", "sih-tools/tally/CALL-LOG.md"),
    ("path", "sih-tools/watchcheck/CALL-LOG.md"),
    ("path", "sih-tools/wikirecall/CALL-LOG.md"),
    ("class", "untracked-disposal"),
];

pub(crate) const SCOPE_SHARED_SURFACE: [&str; 27] = [
    "sih-engine/sih/event/trail",
    "sih-tools/scribe/reports",
    "sih-tools/identity/reports",
    "sih-tools/tally/reports",
    "sih-tools/meter/counts",
    "sih-tools/lease/ledger",
    "sih-tools/lease/ledger/receipts",
    "sih-tools/calllog",
    "sih-tools/cascade/CALL-LOG.md",
    "sih-tools/elicit/CALL-LOG.md",
    "sih-tools/facet/CALL-LOG.md",
    "sih-tools/formatter/CALL-LOG.md",
    "sih-tools/gauge/CALL-LOG.md",
    "sih-tools/identity/CALL-LOG.md",
    "sih-tools/latex-helper/CALL-LOG.md",
    "sih-tools/lease/CALL-LOG.md",
    "sih-tools/locator/CALL-LOG.md",
    "sih-tools/locks/CALL-LOG.md",
    "sih-tools/meter/CALL-LOG.md",
    "sih-tools/nomenclator/CALL-LOG.md",
    "sih-tools/parser/CALL-LOG.md",
    "sih-tools/scribe/CALL-LOG.md",
    "sih-tools/scrutinator/CALL-LOG.md",
    "sih-tools/selector/CALL-LOG.md",
    "sih-tools/tally/CALL-LOG.md",
    "sih-tools/watchcheck/CALL-LOG.md",
    "sih-tools/wikirecall/CALL-LOG.md",
];

pub(crate) fn session_line_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?m)^session: ([0-9a-f]{16})\b").unwrap())
}

pub(crate) fn cert_line_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\bcert: ([0-9a-f]+)").unwrap())
}

fn merge_line_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^merge: \S+ 副本归并$").unwrap())
}

pub(crate) fn direct_pen_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\b直改链笔[：:]\s*([0-9a-f]{8})\b").unwrap())
}

/// BATCH_PREFIX 前瞻断言形（Rust regex crate 不支持前瞻）手写等价：
/// ^[a-z][a-z0-9]*(?:-[a-z0-9]+)+ 后随 [\s(、。一-龥]
pub(crate) fn batch_prefix_match(first: &str) -> bool {
    let chars: Vec<char> = first.chars().collect();
    let mut i = 0usize;
    if i >= chars.len() || !chars[i].is_ascii_lowercase() {
        return false;
    }
    i += 1;
    while i < chars.len() && (chars[i].is_ascii_lowercase() || chars[i].is_ascii_digit()) {
        i += 1;
    }
    let mut groups = 0usize;
    loop {
        if i < chars.len() && chars[i] == '-' {
            let save = i;
            i += 1;
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_lowercase() || chars[i].is_ascii_digit()) {
                i += 1;
            }
            if i == start {
                i = save;
                break;
            }
            groups += 1;
        } else {
            break;
        }
    }
    if groups == 0 {
        return false;
    }
    match chars.get(i) {
        Some(c) => {
            c.is_whitespace()
                || *c == '('
                || *c == '、'
                || *c == '。'
                || ('\u{4e00}'..='\u{9fff}').contains(c)
        }
        None => false,
    }
}

pub(crate) fn validate_commit_message(text: &str) -> (bool, &'static str) {
    let first = text.lines().next().unwrap_or("").trim();
    if direct_pen_re().is_match(text) {
        return (true, "ok_direct");
    }
    let has_session = session_line_re().is_match(text);
    let has_cert = cert_line_re().is_match(text);
    if has_session && has_cert {
        return (true, "ok");
    }
    if merge_line_re().is_match(first) {
        return (true, "ok");
    }
    if first.starts_with("merge:") {
        return (false, "no_merge_hanger");
    }
    if has_session {
        if first.contains(" wip ") {
            return (true, "ok");
        }
        return (false, "no_cert");
    }
    if batch_prefix_match(first) {
        return (false, "batch_prefix_no_session");
    }
    (false, "no_session")
}

fn staged_covered(ws_path: &str, active_locks: &[Value]) -> bool {
    for lock in active_locks {
        let lp = lock.get("path").and_then(|v| v.as_str()).unwrap_or("");
        if !lp.is_empty()
            && (ws_path == lp || ws_path.starts_with(&format!("{}/", lp.trim_end_matches('/'))))
        {
            return true;
        }
    }
    if SCOPE_SHARED_SURFACE.iter().any(|face| ws_path.starts_with(face)) {
        return true;
    }
    for (kind, pat) in DIRECT_LANE_FILE_WHITELIST {
        if kind == "path" && ws_path.starts_with(pat) {
            return true;
        }
    }
    false
}

pub(crate) fn staged_enforce(staged_files: &[String], active_locks: &[Value], allow_no_verify: bool) -> Value {
    if allow_no_verify {
        return json!({
            "allow_no_verify": true,
            "guidance": "",
            "ok": true,
            "rejected": [],
        });
    }
    let rejected: Vec<&String> = staged_files
        .iter()
        .filter(|p| !staged_covered(p, active_locks))
        .collect();
    json!({
        "allow_no_verify": false,
        "guidance": STAGED_REJECT_GUIDANCE,
        "ok": rejected.is_empty(),
        "rejected": rejected,
    })
}

pub(crate) fn parse_active_locks(ledger_path: &Path) -> Vec<Value> {
    let mut active: Vec<Value> = Vec::new();
    let text = match std::fs::read_to_string(ledger_path) {
        Ok(t) => t,
        Err(_) => return active,
    };
    if ledger_path.extension().map(|e| e != "ndjson").unwrap_or(true) {
        return Vec::new();
    }
    let mut held: Vec<(String, String, String)> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let ev: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let p = ev.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let sid = ev
            .get("session_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        match ev.get("event").and_then(|v| v.as_str()) {
            Some("acquired") => {
                let mode = ev
                    .get("mode")
                    .and_then(|v| v.as_str())
                    .unwrap_or("exclusive")
                    .to_string();
                if let Some(slot) = held.iter_mut().find(|(hp, hs, _)| *hp == p && *hs == sid) {
                    slot.2 = mode;
                } else {
                    held.push((p, sid, mode));
                }
            }
            Some("released") => {
                held.retain(|(hp, hs, _)| !(hp == &p && hs == &sid));
            }
            _ => {}
        }
    }
    for (p, _sid, mode) in held {
        if !p.is_empty() {
            active.push(json!({"mode": mode, "path": p}));
        }
    }
    active
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t7_message_forms() {
        let wip = "goldc wip 初次落笔\n\nsession: 0123456789abcdef\n";
        assert_eq!(validate_commit_message(wip), (true, "ok"));
        let settle = "goldc 段1 收约\n\nsession: 0123456789abcdef cert: 31447900 base: master@abcd123\n";
        assert_eq!(validate_commit_message(settle), (true, "ok"));
        let merge = "merge: goldc 副本归并\n";
        assert_eq!(validate_commit_message(merge), (true, "ok"));
        assert_eq!(validate_commit_message("merge: 缺挂接行\n"), (false, "no_merge_hanger"));
        assert_eq!(
            validate_commit_message("无证段头\n\nsession: 0123456789abcdef\n"),
            (false, "no_cert")
        );
        assert_eq!(
            validate_commit_message("mathpipe-a3-solo 冒头批名无会话\n"),
            (false, "batch_prefix_no_session")
        );
        assert_eq!(validate_commit_message("随手一写\n"), (false, "no_session"));
    }

    #[test]
    fn t7_direct_pen_first() {
        let text = "直改链笔: 31447900\n任意形态\n";
        assert_eq!(validate_commit_message(text), (true, "ok_direct"));
        let text_fullwidth = "直改链笔：31447900\n";
        assert_eq!(validate_commit_message(text_fullwidth), (true, "ok_direct"));
    }

    #[test]
    fn t7_batch_prefix_edges() {
        assert!(batch_prefix_match("abc-def x"));
        assert!(batch_prefix_match("mathpipe-a3-solo 段头"));
        assert!(!batch_prefix_match("mathpipe-a3-solo（全角括不在前瞻类）"));
        assert!(!batch_prefix_match("abc x"), "单段无连字符组不成批名前缀");
        assert!(!batch_prefix_match("Abc-def x"), "首字符大写不入");
        assert!(!batch_prefix_match("abc-def"), "行尾无前瞻字符不命中");
    }

    #[test]
    fn t7_staged_enforce_three_faces() {
        let locks = vec![json!({"mode": "exclusive", "path": "sih-engine/src/lib.rs"})];
        assert_eq!(staged_enforce(&["sih-engine/src/lib.rs".into()], &locks, false)["ok"], json!(true));
        assert_eq!(
            staged_enforce(&["sih-tools/nomenclator/packs/core/x.json".into()], &[], false)["ok"],
            json!(false),
            "共享面外白名单外即拒"
        );
        assert_eq!(
            staged_enforce(&["sih-engine/sih/event/trail/2026-09-13.ndjson".into()], &[], false)["ok"],
            json!(true),
            "共享追加面放行"
        );
        assert_eq!(
            staged_enforce(&["sih-tools/lease/CALL-LOG.md".into()], &[], false)["ok"],
            json!(true),
            "轻车道白名单放行"
        );
        let v = staged_enforce(&["a.txt".into()], &[], false);
        assert_eq!(v["ok"], json!(false));
        assert_eq!(v["guidance"], json!(STAGED_REJECT_GUIDANCE));
        assert_eq!(staged_enforce(&["a.txt".into()], &[], true)["ok"], json!(true));
    }

    #[test]
    fn t7_active_locks_pairing() {
        let dir = std::env::temp_dir().join(format!("leg2-guard-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("locks.ndjson");
        std::fs::write(
            &p,
            concat!(
                "{\"event\":\"acquired\",\"path\":\"a\",\"session_id\":\"s1\",\"mode\":\"exclusive\"}\n",
                "{\"event\":\"acquired\",\"path\":\"b\",\"session_id\":\"s1\",\"mode\":\"append\"}\n",
                "{\"event\":\"released\",\"path\":\"a\",\"session_id\":\"s1\"}\n",
            ),
        )
        .unwrap();
        let active = parse_active_locks(&p);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0]["path"], json!("b"));
        assert_eq!(active[0]["mode"], json!("append"));
        std::fs::remove_dir_all(&dir).ok();
    }
}
