//! beta 写面失败语义：分级裁剪静态映射表（DES-014 第六节正典）。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/writeface/errors.py。报错即教
//! 学的边界按调用方来源分级；零动态拼接不可信数据进错误文本，对 CLI 出错的转
//! 述只做静态重写（路径相对化与会话号裁剪）。

use serde_json::{json, Value};

use super::matrix::AGENT_CLASS_EXTERNAL;

pub const EXIT_CODE_SEMANTICS: &str =
    "0=成；1=拦（既有执法拒，理由码原样透出）；2=工具异常（CLI 自身问题非调用拦截）";

pub const CANON_DES_014: &str = "sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md";
pub const CANON_SPEC_023: &str = "sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md";

/// 理由码静态映射表：(匹配子串, 理由码, 闸位名称, 释义, 修复动作指引)；表序
/// 即匹配序，首中即裁。
pub const REASON_TABLE: [(&str, &str, &str, &str, &str); 16] = [
    (
        "IntentRecordUsedRejected",
        "IntentRecordUsedRejected",
        "scribe 闸二重意图拒",
        "同 record 路径已有意图笔在链即拒",
        "同 record 路径唯一意图笔，重放即拒；更换 record 路径，或候人节点处置（MCP 面不透传重放通道旗标，通道缺省关）",
    ),
    (
        "SessionNotActive",
        "SessionNotActive",
        "scribe 闸三会话在册验",
        "会话须在会话台账活跃，无或已吊销即拒",
        "先调 lease_open 立会话；会话已被 close 则沿用在册事实，勿复写",
    ),
    (
        "locked_elsewhere",
        "locked_elsewhere",
        "lease lock 锁冲突拒",
        "目标路径现势锁他持",
        "调 lease_wait_turn 排队候轮，或候持锁方放锁后重试",
    ),
    (
        "queued_not_your_turn",
        "queued_not_your_turn",
        "lease 排队保序公平拒",
        "队列未轮到本会话",
        "调 lease_wait_turn 阻塞候轮（timeout 到即如实返退出码与出队事实）",
    ),
    (
        "scope_violation",
        "scope_violation",
        "lease lock 范围验",
        "路径不在会话 allow 面",
        "按会话立约 allow 面内取锁，或候会话重开调整写入面",
    ),
    (
        "open_precheck_conflict",
        "open_precheck_conflict",
        "lease open 施工面交集预检",
        "写入面与活跃会话交集冲突",
        "候在先会话收约后重开；外部位只载冲突计数，持有者清单系协作信息不对外部位透出",
    ),
    (
        "PackageAlreadyClaimed",
        "PackageAlreadyClaimed",
        "lease claims 领取账本",
        "同包未过期在领即拒",
        "候在领过期或换包；领取是声明位非执法位",
    ),
    (
        "PackageSessionActive",
        "PackageSessionActive",
        "lease open 同包活跃闸",
        "同包已有活跃会话",
        "先 close 旧会话再重开",
    ),
    (
        "active session exists",
        "PackageSessionActive",
        "lease open 同包活跃闸",
        "同包已有活跃会话",
        "先 close 旧会话再重开",
    ),
    (
        "identity_anomaly",
        "identity_anomaly",
        "lease 五验身份验",
        "正身报告异常非空",
        "候 server 重签正身件后重试",
    ),
    (
        "hostname_drift",
        "hostname_drift",
        "lease 绑定验漂移拦",
        "hostname 与绑定侧档失配",
        "会话与主机绑定失配，候会话重开",
    ),
    (
        "user_drift",
        "user_drift",
        "lease 绑定验漂移拦",
        "user 与绑定侧档失配",
        "会话与用户绑定失配，候会话重开",
    ),
    (
        "boottime_drift",
        "boottime_drift",
        "lease 绑定验漂移拦",
        "boottime 与绑定侧档失配",
        "主机经历重启，候会话重开",
    ),
    (
        "binding_absent",
        "binding_absent",
        "lease 绑定验侧档",
        "会话绑定侧档缺席",
        "会话档不完整，候会话重开",
    ),
    (
        "链证守门拦截",
        "chain_gate_missing",
        "lease close 链闸",
        "本会话正典链缺笔（意图笔与认证笔不全）",
        "本会话链上有意图笔与认证笔方可收约；补链笔后收约，或候人节点处置",
    ),
    (
        "收约前置态被阻",
        "close_precheck_blocked",
        "lease closeguard 收约前置检查",
        "目标仓合并态或共享面脏",
        "按报文处置脏面后重试；server 不强拆",
    ),
];

/// 绝对路径面裁剪：根前缀重写为工作区相对形（两级通用，静态重写）。
pub fn relativize_paths(text: &str, root_str: &str) -> String {
    if text.is_empty() || root_str.is_empty() {
        return text.to_string();
    }
    let root = root_str.strip_suffix('/').unwrap_or(root_str);
    let out = text.replace(&format!("{root}/"), "");
    let out = out.replace(&format!("{root}'"), "'");
    let out = out.replace(&format!("{root}\""), "\"");
    out.replace(root, ".")
}

/// 他会话身份裁剪（外部位）：非本会话的十六位会话号替换为裁剪标记；返回
/// （裁剪后文本，裁剪计数）。
pub fn redact_other_sessions(text: &str, own_session_id: Option<&str>) -> (String, usize) {
    use std::sync::OnceLock;
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| regex::Regex::new(r"\b[0-9a-f]{16}\b").unwrap());
    let mut count = 0usize;
    let out = re.replace_all(text, |caps: &regex::Captures| {
        let token = caps.get(0).map(|m| m.as_str()).unwrap_or("");
        if own_session_id == Some(token) {
            token.to_string()
        } else {
            count += 1;
            "[他会话号已裁剪]".to_string()
        }
    });
    (out.to_string(), count)
}

/// 错误文本静态裁剪：路径相对化（两级通用）＋ 外部位他会话号裁剪。
pub fn sanitize_error_text(text: &str, root_str: &str, agent_class: &str, own_session_id: Option<&str>) -> String {
    let out = relativize_paths(text, root_str);
    if agent_class == AGENT_CLASS_EXTERNAL {
        redact_other_sessions(&out, own_session_id).0
    } else {
        out
    }
}

/// 理由码静态匹配：首中即裁。返回（理由码，闸位，释义，动作）。
pub fn match_reason(text: &str) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    if text.is_empty() {
        return None;
    }
    for (needle, code, gate, meaning, action) in REASON_TABLE {
        if text.contains(needle) {
            return Some((code, gate, meaning, action));
        }
    }
    None
}

/// 施工面交集预检的外部位裁剪（DES-014 第六节）：只载冲突计数与建议动作。
pub fn external_precheck_count_only(message: &str, _sanitized: &str) -> Option<String> {
    if !message.contains("open_precheck_conflict") {
        return None;
    }
    let count = message.matches("\"holder\"").count()
        + if message.contains("\"path\"") && !message.contains("\"holder\"") {
            message.matches("\"path\"").count()
        } else {
            0
        };
    Some(format!(
        "施工面交集预检冲突 {count} 处（外部位只载计数，持有者清单不透出）；建议候在先会话收约后重开，或与持有批协调后重试"
    ))
}

/// 错误载荷：承 SPEC-023 四字段契约形，beta 扩展教学与裁剪位九字段。
#[allow(clippy::too_many_arguments)]
pub fn error_payload(
    _tool: &str,
    what: &str,
    params: &[String],
    message: &str,
    root_str: &str,
    agent_class: &str,
    own_session_id: Option<&str>,
    exit_code: i64,
    reason_code: Option<&str>,
    gate: Option<&str>,
    suggested_action: Option<&str>,
) -> Value {
    let mut sanitized = sanitize_error_text(message, root_str, agent_class, own_session_id);
    let mut reason_code = reason_code.map(str::to_string);
    let mut gate = gate.map(str::to_string);
    let mut suggested_action = suggested_action.map(str::to_string);
    if reason_code.is_none() {
        if let Some((code, g, _, action)) = match_reason(message).or_else(|| match_reason(&sanitized)) {
            reason_code = Some(code.to_string());
            gate = Some(g.to_string());
            suggested_action = Some(action.to_string());
        }
    }
    if agent_class == AGENT_CLASS_EXTERNAL && reason_code.as_deref() == Some("open_precheck_conflict") {
        if let Some(replaced) = external_precheck_count_only(message, &sanitized) {
            sanitized = replaced;
        }
    }
    json!({
        "error": sanitized,
        "reason_code": reason_code,
        "gate": gate,
        "what_this_tool_does": what,
        "valid_params": params,
        "exit_code_semantics": EXIT_CODE_SEMANTICS,
        "exit_code": exit_code,
        "suggested_action": suggested_action,
        "canonical_pointers": [CANON_DES_014, CANON_SPEC_023],
    })
}
