//! beta 写面授权矩阵：路由层静态裁剪表（DES-014 第三节正典）。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/writeface/matrix.py。矩阵只决
//! 定工具是否向调用方透传，透传后的判定全归既有程序；takeover 与 bypass 对任
//! 何来源拒透传（无行即无注册即无调用面）。

pub const AGENT_CLASS_ENV: &str = "SIH_MCPLINE_AGENT_CLASS";
pub const AGENT_CLASS_LOCAL: &str = "local";
pub const AGENT_CLASS_EXTERNAL: &str = "external";

/// 矩阵行：beta 写面工具名全集（record_direct 与 lease_unclaim 仅本地可信行）。
pub const MATRIX_ROWS: [&str; 12] = [
    "lease_open",
    "record_intent",
    "record_append",
    "record_park",
    "record_direct",
    "lease_lock",
    "lease_unlock",
    "lease_wait_turn",
    "lease_claim",
    "lease_unclaim",
    "lease_commit",
    "lease_close",
];

/// 外部冷 agent 最小写集（DES-014 第三节判词照录，即此十行）。
pub const EXTERNAL_ALLOWED: [&str; 10] = [
    "lease_open",
    "record_intent",
    "record_append",
    "record_park",
    "lease_lock",
    "lease_unlock",
    "lease_wait_turn",
    "lease_claim",
    "lease_commit",
    "lease_close",
];

/// 调用方分级解析：静态部署配置，缺省 external（最小特权），非法值按 external。
pub fn resolve_agent_class() -> &'static str {
    match std::env::var(AGENT_CLASS_ENV) {
        Ok(v) if v.trim().eq_ignore_ascii_case(AGENT_CLASS_LOCAL) => AGENT_CLASS_LOCAL,
        _ => AGENT_CLASS_EXTERNAL,
    }
}

/// 矩阵行裁决：该分级是否透传该工具。拒透传即不发起调用（路由层裁剪）。
pub fn is_tool_exposed(tool_name: &str, agent_class: &str) -> bool {
    if !MATRIX_ROWS.contains(&tool_name) {
        return false;
    }
    if agent_class == AGENT_CLASS_LOCAL {
        return true;
    }
    EXTERNAL_ALLOWED.contains(&tool_name)
}
